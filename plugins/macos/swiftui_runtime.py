#!/usr/bin/env python3
"""Process-owned SwiftUI runtime requests; geometry arithmetic stays in Rust."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import stat
import subprocess
import sys
import tempfile
import time
import uuid

MAX_BYTES = 1024 * 1024 * 1024


class Failure(Exception):
    def __init__(self, reason, source_limit=None):
        super().__init__(reason)
        self.source_limit = source_limit


def source_limit_detail(value):
    limits = {"native_nodes": (16384, 1048576), "native_depth": (512,), "source_nodes": (16384, 1048576),
              "source_depth": (512,), "type_name_bytes": (65536,), "type_name_controls": (0,),
              "snapshot_sequence": (18446744073709551615,)}
    if (not isinstance(value, dict) or set(value) != {"kind", "limit", "observed"}
            or not isinstance(value["kind"], str) or value["kind"] not in limits
            or type(value["limit"]) is not int or value["limit"] not in limits[value["kind"]]
            or type(value["observed"]) is not int or not 0 <= value["observed"] <= 18446744073709551615):
        raise Failure("invalid_source_limit_detail")
    kind, observed = value["kind"], value["observed"]
    if ((kind == "snapshot_sequence" and observed != value["limit"])
            or (kind == "type_name_controls" and observed != 1)
            or (kind not in ("snapshot_sequence", "type_name_controls") and observed <= value["limit"])):
        raise Failure("invalid_source_limit_detail")
    return value


def unique_object(pairs):
    value = {}
    for key, item in pairs:
        if key in value:
            raise Failure("duplicate_json_key")
        value[key] = item
    return value


def read_json(path, cap=MAX_BYTES):
    path = Path(path)
    info = path.lstat()
    if not stat.S_ISREG(info.st_mode) or info.st_uid != os.getuid() or info.st_size > cap:
        raise Failure("invalid_input_file")
    with path.open("rb") as handle:
        raw = handle.read(cap + 1)
    if len(raw) > cap:
        raise Failure("input_limit")
    return json.loads(raw, object_pairs_hook=unique_object)


def publish(path, data):
    path = Path(path)
    if not path.parent.is_dir() or path.parent.is_symlink():
        raise Failure("invalid_output_directory")
    fd, temporary = tempfile.mkstemp(prefix=".uib-", dir=path.parent)
    try:
        with os.fdopen(fd, "wb") as handle:
            handle.write(data)
        os.link(temporary, path)
    finally:
        os.unlink(temporary)


def encoded(value):
    return (json.dumps(value, separators=(",", ":"), allow_nan=False) + "\n").encode()


def project_recipe(path):
    recipe = read_json(path, 16_384)
    required = {"recipe_version", "project_root", "project", "scheme", "product", "bundle"}
    if (not isinstance(recipe, dict) or not required <= recipe.keys()
            or set(recipe) - required - {"environment", "arguments"}
            or recipe.get("recipe_version") != "1.0.0"
            or any(not isinstance(recipe[k], str) or not recipe[k] for k in required)):
        raise Failure("invalid_project_recipe")
    env, arguments = recipe.get("environment", {}), recipe.get("arguments", [])
    if (not isinstance(env, dict) or any(not re.fullmatch(r"[A-Z_][A-Z0-9_]*", k)
            or not isinstance(v, str) or "\0" in v for k, v in env.items())
            or not isinstance(arguments, list) or any(not isinstance(a, str) or "\0" in a for a in arguments)):
        raise Failure("invalid_project_recipe")
    if set(env) & {"HOME", "CODEX_HOME", "TMPDIR", "TMP", "TEMP", "UIBLUEPRINT_RUNTIME", "SWIFTUI_VIEW_DEBUG"}:
        raise Failure("reserved_environment_key")
    return recipe


def source_identity(root):
    """Fingerprint Git-visible working inputs, including unstaged/untracked edits.

    External local packages and ignored generated inputs retain their owner's
    separate provenance; this does not claim an atomic filesystem snapshot.
    """
    def git(*args):
        return subprocess.check_output(["git", *args], cwd=root, timeout=15)
    revision = git("rev-parse", "HEAD").decode().strip()
    names = sorted(set(git("ls-files", "--cached", "--others", "--exclude-standard", "-z").split(b"\0")) - {b""})
    digest = hashlib.sha256()
    for name in names:
        digest.update(len(name).to_bytes(8, "big")); digest.update(name)
        path = Path(root) / os.fsdecode(name)
        try:
            info = path.lstat()
        except FileNotFoundError:
            digest.update(b"missing"); continue
        digest.update(str(stat.S_IFMT(info.st_mode) | (info.st_mode & 0o111)).encode())
        if stat.S_ISLNK(info.st_mode):
            digest.update(os.fsencode(os.readlink(path)))
        elif stat.S_ISREG(info.st_mode):
            content = hashlib.sha256()
            with path.open("rb") as stream:
                for chunk in iter(lambda: stream.read(1_048_576), b""):
                    content.update(chunk)
            digest.update(content.digest())
        elif stat.S_ISDIR(info.st_mode):
            # A submodule has its own working tree; include it recursively.
            if (path / ".git").exists():
                digest.update(encoded(source_identity(path)))
            else:
                raise Failure("unsupported_source_entry")
        else:
            raise Failure("unsupported_source_entry")
    return dict(revision=revision, git_visible_inputs_sha256=digest.hexdigest(),
                dirty=bool(git("status", "--porcelain=v1", "--untracked-files=all")))


def endpoint(path):
    path = Path(path).absolute()
    info = path.parent.lstat()
    if not stat.S_ISDIR(info.st_mode) or info.st_uid != os.getuid() or info.st_mode & 0o077:
        raise Failure("endpoint_not_private")
    value = read_json(path, 16_384)
    if (value.get("protocol") != "1.0.0" or value.get("source") != "swiftui.runtime.layout"
            or Path(value.get("directory", "")).resolve() != path.parent.resolve()
            or type(value.get("pid")) is not int or value["pid"] <= 0):
        raise Failure("incompatible_endpoint")
    try:
        os.kill(value["pid"], 0)
    except ProcessLookupError:
        raise Failure("target_unavailable") from None
    return path.parent, value


def request(session, operation, window=None, generation=None, timeout=30, subtree_type=None, *, deadline=None):
    if deadline is None:
        deadline = time.monotonic() + timeout
    root, descriptor = endpoint(session)
    if subtree_type is not None:
        if (operation != "capture" or not isinstance(subtree_type, str) or not subtree_type
                or len(subtree_type.encode("utf-8")) > 1024
                or any(ord(c) < 32 or 127 <= ord(c) <= 159 for c in subtree_type)):
            raise Failure("invalid_subtree_type")
        if "capture-subtree-type" not in descriptor.get("capabilities", []):
            raise Failure("subtree_selection_unsupported_by_endpoint")
    if operation == "discover" and "discover-types" not in descriptor.get("capabilities", []):
        raise Failure("discovery_unsupported_by_endpoint")
    request_id = str(uuid.uuid4())
    input_path = root / (request_id + ".request.json")
    response_path = root / (request_id + ".response.json")
    cancel_path = root / (request_id + ".cancel")
    artifact_path = None
    payload = dict(version="1.0.0", id=request_id, run=descriptor["run_id"],
                   capability=descriptor["capability"], operation=operation,
                   window=window, windowGeneration=generation)
    if subtree_type is not None:
        payload["subtreeType"] = subtree_type
    if time.monotonic() >= deadline:
        raise Failure("request_deadline_before_dispatch")
    publish(input_path, encoded(payload))
    try:
        while not response_path.exists():
            if time.monotonic() >= deadline:
                publish(cancel_path, b"cancel\n")
                raise Failure("request_deadline_source_outcome_unknown")
            time.sleep(0.025)
        response = read_json(response_path, 16_384)
        if (response.get("protocol") != "1.0.0" or response.get("request_id") != request_id
                or response.get("run_id") != descriptor["run_id"]):
            raise Failure("response_binding_mismatch")
        if response.get("status") != "ok":
            reason = response.get("reason", "source_refused")
            detail = None
            if reason == "sourceLimit" and "source_limit" in response:
                detail = source_limit_detail(response["source_limit"])
            raise Failure(reason, detail)
        if operation in ("capture", "discover"):
            artifact = "snapshot" if operation == "capture" else "discovery"
            if response.get(artifact) != request_id + "." + artifact + ".json":
                raise Failure("invalid_" + artifact + "_binding")
            artifact_path = root / response[artifact]
            info = artifact_path.lstat()
            if not stat.S_ISREG(info.st_mode) or info.st_size > MAX_BYTES:
                raise Failure("invalid_" + artifact + "_file")
            raw = artifact_path.read_bytes()
            if hashlib.sha256(raw).hexdigest() != response.get("sha256"):
                raise Failure(artifact + "_hash_mismatch")
            return response, raw
        return response, None
    finally:
        # Only this call's already completed products are acknowledged here.
        # A timed-out call leaves cancellation and any late response for its owner.
        if response_path.exists():
            for path in (input_path, response_path, artifact_path, cancel_path):
                if path is not None:
                    try:
                        path.unlink()
                    except FileNotFoundError:
                        pass


def launch(app, run_root, environment=None, arguments=None, expected_bundle=None):
    app = Path(app).resolve()
    if app.suffix != ".app" or not app.is_dir():
        raise Failure("app_not_found")
    if app.stat().st_dev != Path(tempfile.gettempdir()).stat().st_dev:
        raise Failure("app_runtime_must_be_internal")
    root = Path(run_root).resolve()
    if not root.is_dir() or root.stat().st_dev != Path(tempfile.gettempdir()).stat().st_dev:
        raise Failure("runtime_root_must_be_existing_internal_directory")
    run = Path(tempfile.mkdtemp(prefix="uib-launch-", dir=root))
    (run / "OWNERSHIP.md").write_text("Owner: invoking Blueprint task\nState: active\nPurpose: owned application launch. Retain app and session until owner releases them.\n")
    output = run / "launch.stdout"
    error = run / "launch.stderr"
    env = dict(environment or {})
    if any(k in env for k in ("HOME", "CODEX_HOME", "TMPDIR", "TMP", "TEMP")):
        raise Failure("reserved_environment_key")
    env.update(UIBLUEPRINT_RUNTIME="1", SWIFTUI_VIEW_DEBUG="29", TMPDIR=str(run), TMP=str(run), TEMP=str(run))
    command = ["/usr/bin/open", "-n", "-g", "--stdout", str(output), "--stderr", str(error)]
    for key, value in env.items():
        if not re.fullmatch(r"[A-Z_][A-Z0-9_]*", key) or not isinstance(value, str):
            raise Failure("invalid_environment")
        command += ["--env", key + "=" + value]
    command.append(str(app))
    if arguments:
        command += ["--args", *arguments]
    subprocess.run(command, check=True, timeout=15, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    deadline = time.monotonic() + 20
    while time.monotonic() < deadline:
        if error.exists():
            # Do not print unrelated application diagnostics or environment values.
            with error.open("rb") as stream:
                text = stream.read(1_048_576).decode("utf-8", "replace")
            paths = [line.removeprefix("UIBLUEPRINT_SESSION ") for line in text.splitlines()
                     if line.startswith("UIBLUEPRINT_SESSION ")]
            if paths:
                _, descriptor = endpoint(paths[-1])
                if expected_bundle and descriptor.get("bundle") != expected_bundle:
                    raise Failure("unexpected_bundle")
                return dict(session=paths[-1], pid=descriptor["pid"], bundle=descriptor["bundle"],
                            app=str(app), run_directory=str(run))
        time.sleep(0.05)
    raise Failure("runtime_hook_not_ready; owned_launch=" + str(run))


def prepare(args):
    recipe_path = Path(args.recipe).resolve()
    recipe = project_recipe(recipe_path)
    project_root = (recipe_path.parent / recipe["project_root"]).resolve()
    project = (project_root / recipe["project"]).resolve()
    if not project.is_relative_to(project_root) or project.suffix != ".xcodeproj":
        raise Failure("invalid_project")
    product = recipe["product"]
    if Path(product).name != product or not product.endswith(".app"):
        raise Failure("invalid_product")
    build_root = Path(args.build_root).resolve()
    run_root = Path(args.run_root).resolve()
    if not build_root.is_dir() or not run_root.is_dir():
        raise Failure("prepare_roots_must_exist")
    identity = source_identity(project_root)
    with tempfile.TemporaryDirectory(prefix="uib-prepare-", dir=build_root) as task:
        task = Path(task)
        env = dict(os.environ, TMPDIR=str(task), TMP=str(task), TEMP=str(task))
        log = task / "build.log"
        command = ["xcodebuild", "-project", str(project), "-scheme", recipe["scheme"],
                   "-configuration", "Debug", "-derivedDataPath", str(task / "DerivedData"), "build"]
        with log.open("wb") as stream:
            completed = subprocess.run(command, cwd=project_root, env=env, stdout=stream, stderr=subprocess.STDOUT, timeout=2400)
        if completed.returncode:
            raise Failure("project_build_failed")
        if source_identity(project_root) != identity or project_recipe(recipe_path) != recipe:
            raise Failure("source_inputs_changed_during_build")
        source = task / "DerivedData/Build/Products/Debug" / product
        if not source.is_dir():
            raise Failure("built_product_missing")
        artifact = Path(tempfile.mkdtemp(prefix="uib-app-", dir=run_root))
        shutil.copytree(source, artifact / product, symlinks=False)
        (artifact / "OWNERSHIP.md").write_text(f"Owner: invoking Blueprint task\nState: active\nPurpose: retained diagnostic app at {identity['revision']}\nRetain until instance and consumers release it.\n")
        build_identity = dict(source=identity, recipe_sha256=hashlib.sha256(encoded(recipe)).hexdigest())
        publish(artifact / "build-identity.json", encoded(build_identity))
    result = launch(artifact / product, run_root, recipe.get("environment"), recipe.get("arguments"), recipe["bundle"])
    result["build_identity"] = build_identity
    return result


def measured_nodes(snapshot, wanted_type):
    nodes = snapshot["artifact"]["data"]["nodes"]
    selected = []
    for node in nodes:
        role = node["native_role"]
        name = role.get("value", {}).get("value", "") if role.get("availability") == "known" else ""
        if wanted_type and name.split("<", 1)[0] != wanted_type:
            continue
        state = next(p["state"] for p in node["properties"] if p["field"] == "layout_bounds")
        selected.append((node, name, state))
    return selected


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    p = sub.add_parser("launch")
    p.add_argument("--app", required=True)
    p.add_argument("--run-root", required=True)
    p.add_argument("--recipe", help="Reuse the project's declared launch environment and arguments")
    p = sub.add_parser("prepare")
    p.add_argument("--recipe", required=True)
    p.add_argument("--build-root", required=True)
    p.add_argument("--run-root", required=True)
    for operation in ("windows", "capture", "tree", "measure", "discover"):
        p = sub.add_parser(operation)
        p.add_argument("--session", required=True)
        p.add_argument("--window", type=int)
        p.add_argument("--generation")
        p.add_argument("--timeout", type=float, default=30)
        if operation != "windows":
            p.add_argument("--out", required=operation in ("capture", "discover"))
        if operation not in ("windows", "discover"):
            p.add_argument("--type", help="Filter returned nodes after capture (unchanged)")
            p.add_argument("--subtree-type", help="Select one exact nominal type and descendants before export")
        if operation == "measure":
            p.add_argument("--relation", choices=("width", "height"), required=True)
            p.add_argument("--bundle", required=True)
    args = parser.parse_args()
    if args.command == "launch":
        recipe = project_recipe(args.recipe) if args.recipe else {}
        result = launch(args.app, args.run_root, recipe.get("environment"), recipe.get("arguments"), recipe.get("bundle"))
    elif args.command == "prepare":
        result = prepare(args)
    else:
        if not 0 < args.timeout <= 120:
            raise Failure("invalid_deadline")
        args.deadline = time.monotonic() + args.timeout
        if args.command == "measure":
            require_result_only(args.bundle, args.timeout)
        remaining = args.deadline - time.monotonic()
        if remaining <= 0:
            raise Failure("measurement_deadline" if args.command == "measure" else "request_deadline_before_dispatch")
        result, raw = request(args.session, args.command if args.command in ("windows", "discover") else "capture",
                              args.window, args.generation, remaining, getattr(args, "subtree_type", None),
                              deadline=args.deadline)
        if raw is not None:
            artifact = "discovery" if args.command == "discover" else "snapshot"
            result.pop(artifact, None)
            if args.out:
                publish(args.out, raw)
                result["saved_" + artifact] = str(Path(args.out).absolute())
            if args.command != "discover":
                doc = json.loads(raw, object_pairs_hook=unique_object)
                selected = measured_nodes(doc, args.type)
                if args.command == "tree":
                    result["nodes"] = [dict(key=n["key"], type=t, children=n["children"], layout=s)
                                       for n, t, s in selected]
                elif args.command == "measure":
                    result["measurements"] = measure(args, doc, raw, selected)
    print(json.dumps(result, ensure_ascii=False, separators=(",", ":")))


def require_result_only(bundle, timeout):
    result = subprocess.run([str(Path(bundle).resolve() / "uiblueprint"), "--help"],
                            capture_output=True, timeout=min(timeout, 5))
    if result.returncode != 0 or b"measure --json --result-only" not in result.stdout.lower():
        raise Failure("result_only_measurement_unsupported_by_bundle")


def measure(args, doc, raw, selected):
    bundle = Path(args.bundle).resolve()
    results = []
    with tempfile.TemporaryDirectory(prefix="uib-query-") as task:
        task = Path(task)
        snapshot = task / "snapshot.json"
        snapshot.write_bytes(raw)
        def remaining():
            value = args.deadline - time.monotonic()
            if value <= 0:
                raise Failure("measurement_deadline")
            return value
        # Rust measure loads the original Snapshot with Document::from_json,
        # including semantic validation. Avoid parsing a large window twice
        # before its first useful query. An all-unknown/empty selection still
        # needs the independent validator because no measurement will run.
        if not any(state["availability"] == "known" for _, _, state in selected):
            subprocess.run([str(bundle / "uiblueprint-validate"), "--max-bytes", str(MAX_BYTES), str(snapshot)],
                           check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=remaining())
        for node, name, state in selected:
            if state["availability"] != "known":
                results.append(dict(key=node["key"], type=name, status="unknown", reason=state.get("reason")))
                continue
            geometry = state["value"]["value"]
            space = geometry["coordinate_space"]
            query = dict(id="runtime-measure", scope_id=doc["artifact"]["data"]["context"]["scope_id"],
                         targets=[node["key"]], applies_when=dict(platform=None, input_mode=None, text_scale=None),
                         operation=args.relation, anchors=[dict(element=node["key"], frame_kind="layout_bounds",
                             coordinate_space=space, fraction=0, axis="x" if args.relation == "width" else "y")],
                         quantity_kind="length", units="pt")
            query_path = task / "query.json"
            query_path.write_bytes(encoded(dict(schema_version="0.2.0", artifact=dict(kind="geometry_query", data=query))))
            completed = subprocess.run([str(bundle / "uiblueprint"), "measure", "--snapshot", str(snapshot),
                "--query", str(query_path), "--space", space["id"], "--max-input-bytes", str(MAX_BYTES + 16384),
                "--max-output-bytes", str(MAX_BYTES * 2), "--json", "--result-only"], capture_output=True, timeout=remaining())
            if completed.returncode not in (0, 4):
                raise Failure("rust_measure_failed")
            answer = json.loads(completed.stdout)
            if (answer.get("output_version") != "1.0.0" or answer.get("kind") != "measurement_result"
                    or answer.get("snapshot", {}).get("id") != doc["artifact"]["data"]["id"]):
                raise Failure("invalid_measurement_result")
            compact = {"result": answer["result"]}
            results.append(dict(key=node["key"], type=name, result=compact))
    return results


if __name__ == "__main__":
    try:
        main()
    except (Failure, OSError, ValueError, KeyError, subprocess.SubprocessError) as error:
        code = str(error) if isinstance(error, Failure) else type(error).__name__
        failure = dict(status="failed", reason=code)
        if isinstance(error, Failure) and error.source_limit is not None:
            failure["source_limit"] = error.source_limit
        print(json.dumps(failure), file=sys.stderr)
        sys.exit(4)
