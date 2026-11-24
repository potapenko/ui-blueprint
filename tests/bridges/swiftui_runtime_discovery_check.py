#!/usr/bin/env python3
"""Bounded discovery qualification on the owned SubtreeProbe fixture only."""
import argparse
from collections import Counter
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import time
import uuid


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("caller", "bundle", "identity", "control", "evidence-root"):
        parser.add_argument("--" + name, required=True)
    args = parser.parse_args()
    identity = json.loads(Path(args.identity).read_bytes())
    control, evidence = Path(args.control), Path(args.evidence_root)
    base = [sys.executable, "-B", args.caller]
    windows = json.loads(subprocess.check_output(base + ["windows", "--session", identity["session"]]))
    window, = windows["windows"]
    binding = ["--session", identity["session"], "--window", str(window["window"]),
               "--generation", window["generation"], "--timeout", "10"]
    summary = {"discoveries": [], "refusals": []}

    def phase(value):
        (control / "phase").write_text(str(value))
        time.sleep(.5)

    def call(name, operation, selector=None, expected=None):
        output = evidence / (name + ".json")
        command = base + [operation, *binding, "--out", str(output)]
        if selector:
            command += ["--subtree-type", selector]
        completed = subprocess.run(command, capture_output=True, timeout=15)
        if expected:
            assert completed.returncode == 4 and not output.exists()
            assert json.loads(completed.stderr)["reason"] == expected, completed.stderr
            summary["refusals"].append(expected)
            return
        assert completed.returncode == 0, completed.stderr
        raw = output.read_bytes()
        assert len(raw) <= 1024 * 1024 * 1024
        assert b"UIB_RUNTIME_MODEL_CANARY_317cce" not in raw
        receipt, document = json.loads(completed.stdout), json.loads(raw)
        if operation == "discover":
            validate_discovery(document)
            for name in ("request_id", "observation_id", "window", "window_generation"):
                assert document[name] == receipt[name]
            assert document["session_id"] == receipt["run_id"]
            assert receipt["nodes"] == len(document["nodes"])
            assert len(completed.stdout) < 16384, "stdout must stay a compact receipt"
            summary["discoveries"].append(dict(nodes=len(document["nodes"]), bytes=len(raw),
                types=len(document["types"]), observation=document["observation_id"]))
        else:
            subprocess.run([str(Path(args.bundle) / "uiblueprint-validate"), "--max-bytes", "1073741824", str(output)],
                           check=True, capture_output=True, timeout=10)
        return document

    phase(0)
    call("large-whole", "capture")
    discovery = call("large-discovery", "discover")
    by_name = {t["name"]: t for t in discovery["types"]}
    # This qualified fixture's observed source container encloses its button parts.
    selector = by_name["SwiftUI.SpacingLayout"]["name"]
    assert by_name[selector]["count"] == 1
    assert by_name["SwiftUI.Button"]["count"] == 2
    assert "SubtreeProbe.ScopeDialog" not in by_name
    capture = call("discovered-component", "capture", selector)
    assert capture["artifact"]["data"]["id"] != discovery["observation_id"]
    names = {n["native_role"].get("value", {}).get("value") for n in capture["artifact"]["data"]["nodes"]}
    assert {"SwiftUI.Button", "SwiftUI.Image", "SwiftUI.Text"} <= names
    phase(2)
    call("removed-after-discovery", "capture", selector, "subtreeNotFound")
    absent = call("removed-discovery", "discover")
    assert selector not in {t["name"] for t in absent["types"]}
    phase(3)
    call("duplicated-after-discovery", "capture", selector, "ambiguousSubtree")
    duplicated = call("duplicated-discovery", "discover")
    assert next(t["count"] for t in duplicated["types"] if t["name"] == selector) == 2
    phase(5)
    expanded = call("expanded-source", "discover")
    assert len(expanded["nodes"]) > 16384
    phase(4)
    small = call("small-discovery", "discover")
    full = call("small-whole", "capture")["artifact"]["data"]
    assert len(full["nodes"]) == len(small["nodes"])
    roles = Counter(n["native_role"].get("value", {}).get("value") for n in full["nodes"])
    assert {t["name"]: t["count"] for t in small["types"]} == dict(roles)
    assert len({d["observation"] for d in summary["discoveries"]}) == len(summary["discoveries"])
    summary["cancelled"] = cancel_discovery(args.caller, identity["session"], window)
    binding[binding.index("--generation") + 1] = "stale"
    call("stale-discovery", "discover", expected="staleWindow")
    summary["selector"] = selector
    summary["canary_absent"] = True
    (evidence / "summary.json").write_text(json.dumps(summary, indent=2))
    print(json.dumps(summary))


def validate_discovery(document):
    assert document["discovery_version"] == "1.0.0"
    assert document["kind"] == "swiftui_runtime_discovery" and "artifact" not in document
    types, nodes = document["types"], document["nodes"]
    assert [t["name"] for t in types] == sorted({t["name"] for t in types})
    assert all(set(t) == {"name", "count"} and "<" not in t["name"] for t in types)
    seen, counts = {}, Counter()
    for node in nodes:
        assert set(node) == {"index", "parent", "hosting_root", "type_index"}
        assert node["index"] not in seen
        if node["parent"] is not None:
            assert node["parent"] in seen
            assert seen[node["parent"]]["hosting_root"] == node["hosting_root"]
        assert 0 <= node["hosting_root"] < document["coverage"]["hosting_roots"]
        if node["type_index"] is not None:
            assert 0 <= node["type_index"] < len(types)
        counts[node["type_index"]] += 1
        seen[node["index"]] = node
    assert all(t["count"] == counts[i] for i, t in enumerate(types))
    assert document["coverage"]["source_nodes"] == len(nodes)
    assert document["coverage"]["nodes_without_type"] == counts[None]


def cancel_discovery(caller_path, session, window):
    spec = importlib.util.spec_from_file_location("discovery_caller", caller_path)
    caller = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(caller)
    root, descriptor = caller.endpoint(session)
    request_id = str(uuid.uuid4())
    request = dict(version="1.0.0", id=request_id, run=descriptor["run_id"], capability=descriptor["capability"],
                   operation="discover", window=window["window"], windowGeneration=window["generation"])
    cancel_path, response_path = root / (request_id + ".cancel"), root / (request_id + ".response.json")
    caller.publish(cancel_path, b"cancel\n")
    caller.publish(root / (request_id + ".request.json"), caller.encoded(request))
    deadline = time.monotonic() + 5
    while not response_path.exists():
        assert time.monotonic() < deadline
        time.sleep(.025)
    assert caller.read_json(response_path)["reason"] == "cancelled"
    assert not (root / (request_id + ".discovery.json")).exists()
    assert not (root / (request_id + ".request.json")).exists()
    response_path.unlink(); cancel_path.unlink()
    return True


if __name__ == "__main__":
    main()
