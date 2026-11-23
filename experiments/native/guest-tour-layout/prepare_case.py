"""Prepare one immutable G2 owner packet, without querying or attaching the app."""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import sys
import tempfile

from normalize import CARDS


def sha(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--template", type=Path, required=True, help="Previously verified fixed G2 profile")
    parser.add_argument("--card", choices=sorted(CARDS), required=True)
    parser.add_argument("--frame", nargs=4, type=float, required=True, metavar=("X", "Y", "W", "H"))
    parser.add_argument("--kind", choices=("geometry", "group"), default="geometry")
    args = parser.parse_args()
    if not all(math.isfinite(x) for x in args.frame) or min(args.frame[2:]) <= 0:
        raise ValueError("invalid_frame")
    volume = Path("/Volumes/Develop-2TB")
    scratch = volume / "Develop/tmp"
    if not volume.is_mount() or not os.access(scratch, os.W_OK):
        raise ValueError("external_volume_unavailable")
    profile = json.loads(args.template.read_text())
    for path, expected in profile["pins"].items():
        if sha(Path(path)) != expected:
            raise ValueError("template_artifact_changed")
    artifacts = {}
    for path in profile["pins"]:
        if Path(path).name in ("Control", "GroupControl", "debugserver", "identity"):
            if Path(path).name in artifacts:
                raise ValueError("ambiguous_template_artifact")
            artifacts[Path(path).name] = Path(path)
    if set(artifacts) != {"Control", "GroupControl", "debugserver", "identity"}:
        raise ValueError("incomplete_template")
    if sha(artifacts["debugserver"]) != "830f585db3fcf5c5e15f580065137c04924e95551d961a8a3cac63113c2cebdd":
        raise ValueError("backend_not_qualified")
    source = Path(__file__).resolve().parent
    runtime = Path(tempfile.mkdtemp(prefix="uib-s14-case-", dir="/private/tmp"))
    evidence = Path(tempfile.mkdtemp(prefix="uib-s14-case-", dir=scratch))
    if runtime.stat().st_dev == evidence.stat().st_dev:
        raise ValueError("runtime_storage_not_separate")
    for directory in (runtime, evidence):
        (directory / "OWNERSHIP.md").write_text(
            "Owner: S14 chat 01a20922-fc3b-7d30-935a-652c6db5f3dc. Active single-case packet. "
            "Retain package, reviewed pins and evidence until consumer acceptance or explicit retirement. "
            "Build/module-cache is disposable only after all package consumers exit. No ownership of the consumer app/stores.\n")
    (runtime / "tmp").mkdir()
    (evidence / "build").mkdir()
    for name, path in artifacts.items():
        if name == "identity":
            continue
        shutil.copy2(path, runtime / name)
        if sha(runtime / name) != profile["pins"][str(path)]:
            raise ValueError("copy_changed")
        symbols = path.with_name(path.name + ".dSYM")
        if symbols.exists():
            shutil.copytree(symbols, runtime / symbols.name)
    for name in ("component_capture.py", "normalize.py", "group_value.py", "process_component.py"):
        shutil.copy2(source / name, runtime / name)
    env = dict(os.environ, TMPDIR=str(evidence / "build") + "/", TMP=str(evidence / "build"), TEMP=str(evidence / "build"))
    subprocess.run(["xcrun", "swiftc", "-module-cache-path", str(evidence / "build/module-cache"),
                    str(source / "identity.swift"), "-o", str(runtime / "identity")], env=env, check=True, timeout=60)
    for path in artifacts.values():
        del profile["pins"][str(path)]
    for name in artifacts:
        profile["pins"][str(runtime / name)] = sha(runtime / name)
    profile.update(evidence=str(evidence), card=args.card, window_frame=args.frame)
    (runtime / "profile.json").write_text(json.dumps(profile, indent=2))
    modes = ("offline", "owner", "owner-group", "fixture", "fixture-group", "fixture-nohit", "fixture-missing", "fixture-absent")
    for mode in modes:
        (runtime / (mode + ".lldb")).write_text("script RUN_KIND=" + repr(mode) + "; __file__=" + repr(str(runtime / "component_capture.py")) +
                                               "; exec(compile(open(__file__).read(),__file__,'exec'))\n")
    # A single wrapper admits exactly the selected operation, not both owner modes.
    mode = "owner-group" if args.kind == "group" else "owner"
    receipt = "group.json" if args.kind == "group" else "component.json"
    bundle_paths = [Path(p).parent for p in profile["pins"] if Path(p).name == "uiblueprint"]
    if len(bundle_paths) != 1:
        raise ValueError("cli_bundle_unresolved")
    quote = shlex.quote
    wrapper = "\n".join([
        "#!/bin/sh", "set -eu", '[ "$#" -eq 0 ] && [ -t 0 ] && [ -t 1 ] || exit 2',
        "cd " + quote(str(runtime)), "export TMPDIR=" + quote(str(runtime / "tmp") + "/") + " TMP=" + quote(str(runtime / "tmp")) + " TEMP=" + quote(str(runtime / "tmp")),
        "/usr/bin/shasum -a 256 -c SHA256SUMS >/dev/null",
        "/Applications/Xcode.app/Contents/Developer/usr/bin/lldb --no-lldbinit -s " + quote(str(runtime / (mode + ".lldb"))),
        "if [ -f " + quote(str(evidence / "outcome.json")) + " ]; then cat " + quote(str(evidence / "outcome.json")) + "; exit 4; fi",
        quote(sys.executable) + " -B " + quote(str(runtime / "process_component.py")) + " " + quote(str(evidence / receipt)) + " " + quote(str(bundle_paths[0])), "",
    ])
    (runtime / "capture").write_text(wrapper)
    (runtime / "capture").chmod(0o700)
    files = sorted(p for p in runtime.iterdir() if p.is_file() and p.name != "OWNERSHIP.md")
    (runtime / "SHA256SUMS").write_text("".join(sha(p) + "  " + p.name + "\n" for p in files))
    offline = subprocess.run(["/Applications/Xcode.app/Contents/Developer/usr/bin/lldb", "--no-lldbinit", "-b", "-s",
                              str(runtime / "offline.lldb"), "-o", "quit"], env=dict(os.environ, TMPDIR=str(runtime / "tmp") + "/"),
                             capture_output=True, text=True, timeout=30)
    (evidence / "offline.txt").write_text(offline.stdout + offline.stderr)
    if offline.returncode or "OFFLINE_FOUR_SOURCE_SITES_VERIFIED" not in offline.stdout:
        raise ValueError("offline_site_verification_failed")
    handoff = {"runtime": str(runtime), "evidence": str(evidence), "manifest_sha256": sha(runtime / "SHA256SUMS"),
               "command": str(runtime / "capture"), "card": args.card, "kind": args.kind, "frame": args.frame,
               "consumer_attempted": False, "offline_sites_verified": True}
    (evidence / "packet.json").write_text(json.dumps(handoff, indent=2))
    print(json.dumps(handoff))


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError, subprocess.SubprocessError):
        print("case_preparation_refused", file=sys.stderr)
        sys.exit(2)
