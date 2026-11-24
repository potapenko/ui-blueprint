#!/usr/bin/env python3
"""Qualify an owned SwiftUIRuntimeSubtreeFixture through the normal CLI.

Build the fixture as module SubtreeProbe. On the qualified source profile,
GroupBox exposes two nominal GroupBox nodes; its unique SpacingLayout descendant
is the observed root containing the button/image/text parts (not an app alias). Identity/control are private, owned
internal runtime inputs; evidence-root is an existing external task directory.
This test never accesses a consumer application's window or private descriptor.
"""
import argparse
import concurrent.futures
import json
import importlib.util
import uuid
from pathlib import Path
import subprocess
import sys
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("caller", "bundle", "identity", "control", "evidence-root"):
        parser.add_argument("--" + name, required=True)
    parser.add_argument("--legacy-caller")
    args = parser.parse_args()
    identity = json.loads(Path(args.identity).read_bytes())
    control, evidence, bundle = Path(args.control), Path(args.evidence_root), Path(args.bundle)
    assert evidence.is_dir() and control.is_dir()
    base = [sys.executable, "-B", args.caller]
    binding = ["--session", identity["session"]]
    windows = subprocess.run(base + ["windows", *binding], check=True, capture_output=True)
    window, = json.loads(windows.stdout)["windows"]
    binding += ["--window", str(window["window"]), "--generation", window["generation"], "--timeout", "10"]
    summary = {"phases": [], "pid": identity["pid"]}

    def invoke(name, selector=None, failure=None, extra=()):
        output = evidence / (name + ".json")
        command = base + ["capture", *binding, "--out", str(output), *extra]
        if selector is not None:
            command += ["--subtree-type", selector]
        start = time.monotonic()
        completed = subprocess.run(command, capture_output=True, timeout=15)
        elapsed = time.monotonic() - start
        if failure:
            assert completed.returncode == 4, (name, completed.returncode)
            assert json.loads(completed.stderr)["reason"] == failure, completed.stderr
            assert not output.exists(), "refused capture must not publish"
            return {"reason": failure, "seconds": elapsed}
        assert completed.returncode == 0, completed.stderr
        raw = output.read_bytes()
        assert len(raw) <= 1024 * 1024 * 1024
        assert b"UIB_RUNTIME_MODEL_CANARY_317cce" not in raw
        subprocess.run([str(bundle / "uiblueprint-validate"), "--max-bytes", "1073741824", str(output)],
                       check=True, capture_output=True, timeout=10)
        doc = json.loads(raw)["artifact"]["data"]
        assert_closure(doc)
        result = json.loads(completed.stdout)
        result.update(bytes=len(raw), seconds=elapsed)
        return result, doc

    for phase in (0, 1, 2, 3, 4, 5, 0):
        (control / "phase").write_text(str(phase))
        time.sleep(.5)  # fixture-owned state input, no production UI automation
        name = "phase-" + str(len(summary["phases"]))
        if phase in (2, 3):
            result = invoke(name, "SwiftUI.SpacingLayout", "subtreeNotFound" if phase == 2 else "ambiguousSubtree")
        else:
            result, doc = invoke(name, "SwiftUI.SpacingLayout")
            types = [node["native_role"].get("value", {}).get("value") for node in doc["nodes"]]
            assert {"SwiftUI.SpacingLayout", "SwiftUI.Button", "SwiftUI.Image", "SwiftUI.Text"} <= set(types)
            assert doc["coverage"]["omitted_count"] == 0
            assert doc["coverage"]["unknown_count"] == 0
            assert result["selection"]["source_nodes"] > len(doc["nodes"])
            measure = subprocess.run(base + ["measure", *binding, "--subtree-type", "SwiftUI.SpacingLayout",
                "--type", "SwiftUI.Color", "--relation", "width", "--bundle", str(bundle)],
                check=True, capture_output=True, timeout=15)
            measurements = json.loads(measure.stdout)["measurements"]
            amounts = [m["result"]["result"]["measurement"]["value"]["value"]["amount"]
                       for m in measurements if m.get("result", {}).get("result", {}).get("status") == "known"]
            assert (180 if phase == 1 else 120) in amounts, measurements
            result["rust_color_widths"] = amounts
            if phase == 0:
                full_result, full_doc = invoke(name + "-whole")
                assert full_result["bytes"] > 16 * 1024 * 1024
                summary["whole_large"] = dict(nodes=len(full_doc["nodes"]), bytes=full_result["bytes"])
                summary["ambiguous_common_type"] = invoke(name + "-buttons", "SwiftUI.Button", "ambiguousSubtree")
                summary["not_source_exposed"] = invoke(name + "-custom", "SubtreeProbe.ScopeDialog", "subtreeNotFound")
            if phase == 4:
                full, full_doc = invoke(name + "-whole")
                summary["whole_small"] = dict(nodes=len(full_doc["nodes"]), bytes=full["bytes"])
                if args.legacy_caller:
                    legacy_output = evidence / "legacy-whole.json"
                    legacy = subprocess.run([sys.executable, "-B", args.legacy_caller, "capture", *binding,
                        "--type", "SwiftUI.Color", "--out", str(legacy_output)], check=True, capture_output=True, timeout=15)
                    legacy_doc = json.loads(legacy_output.read_bytes())["artifact"]["data"]
                    subprocess.run([str(bundle / "uiblueprint-validate"), "--max-bytes", "1073741824", str(legacy_output)],
                                   check=True, capture_output=True, timeout=10)
                    assert len(legacy_doc["nodes"]) == len(full_doc["nodes"]), "legacy --type must not change capture scope"
                    assert "selection" not in json.loads(legacy.stdout)
                    summary["legacy_whole_nodes"] = len(legacy_doc["nodes"])
                    legacy_help = subprocess.check_output([sys.executable, "-B", args.legacy_caller,
                                                           "capture", "--help"], timeout=5)
                    if b"--subtree-type" in legacy_help:  # 0.8 callers only support full capture.
                        legacy_scoped_output = evidence / "legacy-subtree.json"
                        legacy_scoped = subprocess.run([sys.executable, "-B", args.legacy_caller, "capture", *binding,
                            "--subtree-type", "SwiftUI.SpacingLayout", "--out", str(legacy_scoped_output)],
                            check=True, capture_output=True, timeout=15)
                        legacy_scoped_doc = json.loads(legacy_scoped_output.read_bytes())["artifact"]["data"]
                        assert len(legacy_scoped_doc["nodes"]) == len(doc["nodes"])
                        assert_closure(legacy_scoped_doc)
                        summary["legacy_subtree_nodes"] = json.loads(legacy_scoped.stdout)["nodes"]
        summary["phases"].append(dict(phase=phase, result=result))

    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        captured = list(pool.map(lambda i: invoke(f"concurrent-{i}", "SwiftUI.SpacingLayout"), range(4)))
    ids = [result["observation_id"] for result, _ in captured]
    assert len(set(ids)) == 4
    summary["concurrent_distinct"] = 4
    assert len({phase["result"]["observation_id"] for phase in summary["phases"] if "observation_id" in phase["result"]}) == 5
    summary["canary_absent"] = True
    summary["cancel_and_input"] = endpoint_refusals(args.caller, identity["session"], window)
    # Stale bindings remain authoritative even with a valid selector.
    binding[binding.index("--generation") + 1] = "stale-generation"
    summary["stale"] = invoke("stale", "SwiftUI.SpacingLayout", "staleWindow")
    (evidence / "summary.json").write_text(json.dumps(summary, indent=2))
    print(json.dumps(summary))


def endpoint_refusals(caller_path, session, window):
    spec = importlib.util.spec_from_file_location("subtree_caller", caller_path)
    caller = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(caller)
    root, descriptor = caller.endpoint(session)
    results = []
    for value, cancel, expected in [("SwiftUI.SpacingLayout", True, "cancelled"),
                                    (None, False, "invalidRequest"),
                                    ("bad\nselector", False, "invalidRequest")]:
        request_id = str(uuid.uuid4())
        request = dict(version="1.0.0", id=request_id, run=descriptor["run_id"], capability=descriptor["capability"],
                       operation="capture", window=window["window"], windowGeneration=window["generation"], subtreeType=value)
        cancel_path = root / (request_id + ".cancel")
        response_path = root / (request_id + ".response.json")
        if cancel:
            caller.publish(cancel_path, b"cancel\n")
        caller.publish(root / (request_id + ".request.json"), caller.encoded(request))
        deadline = time.monotonic() + 5
        while not response_path.exists():
            assert time.monotonic() < deadline, "fixture request did not complete"
            time.sleep(.025)
        response = caller.read_json(response_path)
        assert response["reason"] == expected
        assert not (root / (request_id + ".snapshot.json")).exists()
        assert not (root / (request_id + ".request.json")).exists()
        response_path.unlink()
        cancel_path.unlink(missing_ok=True)
        results.append(expected)
    return results


def assert_closure(doc):
    nodes = {node["key"]["key"]: node for node in doc["nodes"]}
    parents = {child["key"]: node["key"]["key"] for node in nodes.values() for child in node["children"]}
    for key, node in nodes.items():
        assert all(child["key"] in nodes for child in node["children"])
        declarations = {d["name"]: d["state"]["value"]["value"] for d in node["source_declarations"]}
        for name in ("layout_position_source", "layout_size_source"):
            if name not in declarations:
                continue
            ancestor = parents.get(key)
            while ancestor is not None and ancestor != declarations[name]:
                ancestor = parents.get(ancestor)
            assert ancestor is not None, "inherited source must remain on the ancestor chain"
        if declarations.get("selection_role") == "ancestor_context":
            assert declarations["excluded_direct_children"] >= 0
        elif "selection_role" in declarations:
            assert declarations["excluded_direct_children"] == 0, "subtree must keep every child"


if __name__ == "__main__":
    main()
