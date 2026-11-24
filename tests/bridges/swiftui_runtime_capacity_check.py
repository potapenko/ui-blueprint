#!/usr/bin/env python3
"""Full large-tree admission through the owned SubtreeProbe fixture and Rust."""
import argparse
import concurrent.futures
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("caller", "bundle", "identity", "control", "evidence-root"):
        parser.add_argument("--" + name, required=True)
    args = parser.parse_args()
    spec = importlib.util.spec_from_file_location("capacity_caller", args.caller)
    caller = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(caller)
    identity = json.loads(Path(args.identity).read_bytes())
    root, control, bundle = Path(args.evidence_root), Path(args.control), Path(args.bundle)
    assert root.is_dir()
    windows, _ = caller.request(identity["session"], "windows")
    window, = windows["windows"]
    binding = ["--session", identity["session"], "--window", str(window["window"]),
               "--generation", window["generation"]]
    base = [sys.executable, "-B", args.caller]
    summary = {}

    def capture(name, selector=None):
        output = root / (name + ".json")
        command = base + ["capture", *binding, "--out", str(output)]
        if selector:
            command += ["--subtree-type", selector]
        start = time.monotonic()
        result = subprocess.run(command, capture_output=True, timeout=35)
        assert result.returncode == 0, result.stderr
        receipt = json.loads(result.stdout)
        raw = output.read_bytes()
        assert b"UIB_RUNTIME_MODEL_CANARY_317cce" not in raw
        return output, raw, dict(nodes=receipt["nodes"], bytes=len(raw),
            seconds=time.monotonic() - start, observation=receipt["observation_id"])

    (control / "phase").write_text("5")
    time.sleep(.5)
    output, raw, summary["full"] = capture("full")
    assert summary["full"]["nodes"] > 16384 and len(raw) > 16 * 1024 * 1024
    start = time.monotonic()
    subprocess.run([str(bundle / "uiblueprint-validate"), "--max-bytes", str(caller.MAX_BYTES), str(output)],
                   check=True, capture_output=True, timeout=30)
    summary["validate_seconds"] = time.monotonic() - start
    doc = json.loads(raw)["artifact"]["data"]
    nodes = {node["key"]["key"]: node for node in doc["nodes"]}
    selected, = [n for n in nodes.values() if n["native_role"].get("value", {}).get("value") == "SwiftUI.SpacingLayout"]
    pending, descendants = [selected], []
    while pending:
        node = pending.pop(); descendants.append(node)
        pending += [nodes[child["key"]] for child in node["children"]]
    color, = [n for n in descendants if n["native_role"].get("value", {}).get("value") == "SwiftUI.Color"]
    geometry = color["properties"][0]["state"]["value"]["value"]
    space = geometry["coordinate_space"]
    query = dict(id="capacity-width", scope_id=doc["context"]["scope_id"], targets=[color["key"]],
        applies_when=dict(platform=None, input_mode=None, text_scale=None), operation="width",
        anchors=[dict(element=color["key"], frame_kind="layout_bounds", coordinate_space=space, fraction=0, axis="x")],
        quantity_kind="length", units="pt")
    query_path = root / "width-query.json"
    query_path.write_bytes(caller.encoded(dict(schema_version="0.2.0", artifact=dict(kind="geometry_query", data=query))))
    start = time.monotonic()
    result = subprocess.run([str(bundle / "uiblueprint"), "measure", "--snapshot", str(output),
        "--query", str(query_path), "--space", space["id"], "--max-input-bytes", str(caller.MAX_BYTES + 16384),
        "--max-output-bytes", "1048576", "--json", "--result-only"], check=True, capture_output=True, timeout=30)
    measured = json.loads(result.stdout)
    assert measured["result"]["measurement"]["value"]["value"]["amount"] == 120
    assert measured["snapshot"]["observations"] == doc["observations"]
    assert measured["snapshot"]["coverage"] == doc["coverage"]
    summary["full_saved_measure"] = dict(width=120, seconds=time.monotonic() - start, bytes=len(result.stdout))
    del raw, doc, nodes, descendants
    _, _, summary["scoped"] = capture("scoped", "SwiftUI.SpacingLayout")
    # Same owned process changes layout; actual Rust uses the fresh scoped observation.
    summary["fresh_widths"] = []
    for phase, width in [(1, 180), (5, 120)]:
        (control / "phase").write_text(str(phase)); time.sleep(.5)
        start = time.monotonic()
        result = subprocess.run(base + ["measure", *binding, "--subtree-type", "SwiftUI.SpacingLayout",
            "--type", "SwiftUI.Color", "--relation", "width", "--bundle", str(bundle)],
            check=True, capture_output=True, timeout=35)
        reply = json.loads(result.stdout)
        values = [m["result"]["result"]["measurement"]["value"]["value"]["amount"] for m in reply["measurements"]]
        assert values == [width]
        summary["fresh_widths"].append(dict(width=width, observation=reply["observation_id"], seconds=time.monotonic()-start))
    start = time.monotonic()
    result = subprocess.run(base + ["measure", *binding, "--type", "SwiftUI.SpacingLayout",
        "--relation", "width", "--bundle", str(bundle)], check=True, capture_output=True, timeout=35)
    reply = json.loads(result.stdout)
    assert len(reply["measurements"]) == 1 and reply["measurements"][0]["result"]["result"]["status"] == "known"
    summary["full_live_measure"] = dict(seconds=time.monotonic()-start, bytes=len(result.stdout))
    with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
        receipts = list(pool.map(lambda n: capture("concurrent-" + str(n))[2], range(8)))
    assert len({r["observation"] for r in receipts}) == 8
    assert all(r["nodes"] > 16384 and r["bytes"] > 16 * 1024 * 1024 for r in receipts)
    summary["concurrent"] = receipts
    # Import the existing pre-dispatch cancellation/input contract check.
    spec = importlib.util.spec_from_file_location("subtree_checks", Path(__file__).with_name("swiftui_runtime_subtree_check.py"))
    checks = importlib.util.module_from_spec(spec); spec.loader.exec_module(checks)
    summary["cancel_and_input"] = checks.endpoint_refusals(args.caller, identity["session"], window)
    summary["canary_absent"] = True
    (root / "summary.json").write_text(json.dumps(summary, indent=2))
    print(json.dumps(summary))


if __name__ == "__main__":
    main()
