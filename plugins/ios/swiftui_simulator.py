#!/usr/bin/env python3
"""Launch an explicitly owned Simulator app and capture source/PNG pairs.

Build using the app's recipe first. This command does not acquire a device,
change its configuration, or terminate another owner's application.
"""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import plistlib
import re
import struct
import subprocess
import sys
import tempfile
import time


def runtime_caller():
    here = Path(__file__).resolve().parent
    path = here / "swiftui_runtime.py"
    if not path.is_file():
        path = here.parent / "macos/swiftui_runtime.py"
    spec = importlib.util.spec_from_file_location("blueprint_runtime", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def simctl(*args, timeout=30, env=None):
    result = subprocess.run(["xcrun", "simctl", *args], capture_output=True, timeout=timeout, env=env)
    if result.returncode:
        raise ValueError("simulator_command_failed:" + args[0])
    return result.stdout.decode().strip()


def launch(args, caller):
    app = Path(args.app).resolve()
    root = Path(args.run_root).resolve()
    if (not app.is_dir() or not root.is_dir()
            or app.stat().st_dev != Path("/private/tmp").stat().st_dev
            or root.stat().st_dev != Path("/private/tmp").stat().st_dev):
        raise ValueError("internal_app_and_existing_runtime_required")
    info = plistlib.loads((app / "Info.plist").read_bytes())
    bundle = info["CFBundleIdentifier"]
    recipe = caller.project_recipe(args.recipe) if args.recipe else {}
    if recipe and recipe["bundle"] != bundle:
        raise ValueError("recipe_bundle_mismatch")
    run = Path(tempfile.mkdtemp(prefix="uib-simulator-launch-", dir=root))
    (run / "OWNERSHIP.md").write_text("Owner: invoking Simulator capture task\nPurpose: owned Simulator launch\nState: active\nRetain while app/consumers need descriptor; stop/uninstall only this owned app before cleanup.\n")
    simctl("install", args.simulator, str(app), timeout=60)
    env = dict(os.environ)
    for key in list(env):
        if key.startswith("SIMCTL_CHILD_"):
            del env[key]
    options = dict(recipe.get("environment", {}), UIBLUEPRINT_RUNTIME="1", SWIFTUI_VIEW_DEBUG="29")
    env.update({"SIMCTL_CHILD_" + key: value for key, value in options.items()})
    container = Path(simctl("get_app_container", args.simulator, bundle, "data")).resolve()
    previous = set((container / "tmp").glob("uiblueprint-runtime-*/session.json"))
    # simctl's stdout/stderr destinations are interpreted by the simulated OS.
    # Bind only a newly created descriptor in this app's exact data container to
    # the PID returned by LaunchServices; never scan other apps or reuse old runs.
    launched = simctl("launch", args.simulator, bundle, *recipe.get("arguments", []), env=env)
    match = re.fullmatch(re.escape(bundle) + r": (\d+)", launched)
    if not match:
        raise ValueError("unresolved_launched_pid")
    pid = int(match.group(1))
    deadline = time.monotonic() + 20
    while time.monotonic() < deadline:
        for path in set((container / "tmp").glob("uiblueprint-runtime-*/session.json")) - previous:
            _, descriptor = caller.endpoint(path)
            if descriptor["bundle"] == bundle and descriptor["pid"] == pid:
                return dict(session=str(path), pid=pid, bundle=bundle,
                            simulator=args.simulator, app=str(app), run_directory=str(run))
        time.sleep(.05)
    raise ValueError("runtime_hook_not_ready; owned_launch=" + str(run))


def pair(args, caller):
    if getattr(args, "label_style", False) and args.source != "uikit":
        raise ValueError("invalid_label_style_selection")
    if getattr(args, "font_resolution", False) and args.source != "uikit":
        raise ValueError("invalid_font_resolution_selection")
    output = Path(args.out).absolute()
    if output.exists():
        raise ValueError("output_must_not_exist")
    endpoint_root, descriptor = caller.endpoint(args.session)
    # The descriptor must belong to this exact Simulator's app data container.
    container = Path(simctl("get_app_container", args.simulator, descriptor["bundle"], "data")).resolve()
    if not endpoint_root.resolve().is_relative_to(container):
        raise ValueError("simulator_session_mismatch")
    windows, _ = caller.request(args.session, "windows", timeout=args.timeout)
    candidates = [w for w in windows["windows"] if args.window is None or w["window"] == args.window]
    if len(candidates) != 1:
        raise ValueError("ambiguous_or_unavailable_window")
    window = candidates[0]
    if window.get("platform") not in ("ios-simulator", "tvos-simulator"):
        raise ValueError("simulator_metadata_required")
    if window.get("scene_state") != 0:
        raise ValueError("foreground_scene_required_for_png_pair")
    response, raw = caller.request(args.session, "capture", window["window"], window["generation"], args.timeout, source=args.source, label_style=getattr(args, "label_style", False), font_resolution=getattr(args, "font_resolution", False))
    with tempfile.TemporaryDirectory(prefix="uib-simulator-pair-", dir="/private/tmp") as temporary:
        png = Path(temporary) / "screen.png"
        start = time.monotonic()
        simctl("io", args.simulator, "screenshot", "--type=png", str(png), timeout=20)
        end = time.monotonic()
        image = png.read_bytes()
        if image[:8] != b"\x89PNG\r\n\x1a\n":
            raise ValueError("lossless_png_required")
        width, height = struct.unpack(">II", image[16:24])
        scale = window["screen_scale"]
        if (width, height) != (round(window["screen_width"] * scale), round(window["screen_height"] * scale)):
            raise ValueError("screen_pixel_mapping_unqualified")
        latest, _ = caller.request(args.session, "windows", timeout=args.timeout)
        if window not in latest["windows"]:
            raise ValueError("window_metadata_changed_during_pair")
        output.mkdir(mode=0o700)
        (output / "OWNERSHIP.md").write_text("Owner: invoking Simulator capture task\nState: retained\nPurpose: source geometry/native PNG pair\nRelease: consuming reference owner accepts/supersedes pair.\n")
        caller.publish(output / "snapshot.json", raw)
        caller.publish(output / "screen.png", image)
        metadata = dict(pair_version="1.0.0", simulator=args.simulator, pid=descriptor["pid"],
                        bundle=descriptor["bundle"], window=window, observation_id=response["observation_id"],
                        snapshot_sha256=hashlib.sha256(raw).hexdigest(), png_sha256=hashlib.sha256(image).hexdigest(),
                        pixel_width=width, pixel_height=height, screenshot_start=start, screenshot_end=end,
                        screenshot_clock="host-monotonic", consistency="sequential-not-atomic",
                        screen_pt_to_png=[scale, 0, 0, scale, 0, 0],
                        swiftui_host_mapping="not-applicable" if args.source == "uikit" else "requires-qualified-source-profile")
        if args.source == "uikit":
            metadata["source_namespace"] = "uikit.runtime.layout"
            metadata["uikit_mapping"] = "model-local-to-window; screen-pair-requires-qualification"
        caller.publish(output / "pair.json", caller.encoded(metadata))
    return dict(status="ok", directory=str(output), nodes=response["nodes"],
                observation_id=response["observation_id"], pixel_width=width, pixel_height=height)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    p = commands.add_parser("launch")
    p.add_argument("--app", required=True)
    p.add_argument("--simulator", required=True)
    p.add_argument("--run-root", required=True)
    p.add_argument("--recipe")
    p = commands.add_parser("pair")
    p.add_argument("--session", required=True)
    p.add_argument("--simulator", required=True)
    p.add_argument("--window", type=int)
    p.add_argument("--source", choices=("swiftui", "uikit"), default="swiftui")
    p.add_argument("--font-resolution", action="store_true", help="Resolve an integrated SwiftUI font probe; UIKit only")
    p.add_argument("--label-style", action="store_true", help="Opt-in configured UILabel font/colors; UIKit only")
    p.add_argument("--out", required=True)
    p.add_argument("--timeout", type=float, default=30)
    args = parser.parse_args()
    if args.command == "pair" and not 0 < args.timeout <= 120:
        raise ValueError("invalid_deadline")
    caller = runtime_caller()
    result = launch(args, caller) if args.command == "launch" else pair(args, caller)
    print(json.dumps(result, separators=(",", ":")))


if __name__ == "__main__":
    try:
        main()
    except Exception as error:
        # Fixed codes only; no source payload/launch environment in exceptions.
        is_source_refusal = type(error).__name__ == "Failure" and type(error).__module__ == "blueprint_runtime"
        reason = str(error) if type(error) is ValueError or is_source_refusal else type(error).__name__
        failure = dict(status="failed", reason=reason)
        if is_source_refusal and error.source_limit is not None:
            failure["source_limit"] = error.source_limit
        print(json.dumps(failure), file=sys.stderr)
        sys.exit(4)
