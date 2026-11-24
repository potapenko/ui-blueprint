#!/usr/bin/env python3
"""Owned SwiftUIRuntimeAffineFixture qualification; never an application collector.

The tiny affine oracle below compares source normalization with public SwiftUI
GeometryProxy readings. User measurements are separately checked through Rust.
"""
import argparse
import concurrent.futures
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import time


def main():
    p = argparse.ArgumentParser(description=__doc__)
    for name in ("caller", "app", "bundle", "run-root", "evidence-root"):
        p.add_argument("--" + name, required=True)
    args = p.parse_args()
    bundle = Path(args.bundle)
    assert all((bundle / name).is_file() for name in ("uiblueprint", "uiblueprint-validate")), "matching bundle missing"
    assert Path(args.app).is_dir() and Path(args.caller).is_file()
    spec = importlib.util.spec_from_file_location("runtime_caller", args.caller)
    caller = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(caller)
    control = Path(tempfile.mkdtemp(prefix="uib-affine-control-", dir=args.run_root))
    evidence = Path(tempfile.mkdtemp(prefix="uib-affine-evidence-", dir=args.evidence_root))
    note = "Owner: invoking Blueprint fixture task\nState: active\nRetain final qualification until release owner accepts it.\n"
    for root in (control, evidence):
        (root / "OWNERSHIP.md").write_text(note)
    launched = caller.launch(args.app, args.run_root, {"UIB_FIXTURE_CONTROL": str(control)})
    summary = {"phases": [], "pid": launched["pid"]}
    try:
        for sequence, phase in enumerate((0, 1, 2, 0)):
            (control / "phase").write_text(str(phase))
            time.sleep(0.4)
            response, raw = caller.request(launched["session"], "capture")
            assert b"UIB_RUNTIME_MODEL_CANARY_317cce" not in raw
            snapshot_path = evidence / f"phase-{sequence}.json"
            caller.publish(snapshot_path, raw)
            subprocess.run([str(bundle / "uiblueprint-validate"), "--max-bytes", "16777216", str(snapshot_path)],
                           check=True, capture_output=True, timeout=5)
            document = json.loads(raw)["artifact"]["data"]
            oracle_path = control / "oracle.json"
            oracle_path.unlink(missing_ok=True)
            (control / "oracle.request").touch()
            deadline = time.monotonic() + 2
            while not oracle_path.exists():
                assert time.monotonic() < deadline, "public oracle not ready"
                time.sleep(0.025)
            oracle = json.loads(oracle_path.read_bytes())
            assert oracle["phase"] == phase
            caller.publish(evidence / f"oracle-{sequence}.json", caller.encoded(oracle))
            selected = compare_public_oracles(document, oracle["rectangles"])
            assert document["coverage"]["unknown_count"] == 0
            if phase == 2:
                width = rust_measure(bundle, snapshot_path, document, [selected["color"]], "width")
                gap = rust_measure(bundle, snapshot_path, document, [selected["color"], selected["text"]], "gap")
                assert abs(width - oracle["rectangles"]["color"][2]) < 1e-8
                color, text = oracle["rectangles"]["color"], oracle["rectangles"]["text"]
                assert abs(gap - (text[0] - color[0] - color[2])) < 1e-8
                summary["rust_width"] = width
                summary["rust_gap"] = gap
            summary["phases"].append(dict(phase=phase, nodes=len(document["nodes"]), bytes=len(raw),
                                          observation=response["observation_id"]))
        with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
            captures = list(pool.map(lambda _: caller.request(launched["session"], "capture"), range(8)))
        assert len({r["observation_id"] for r, _ in captures}) == 8
        assert all(b"UIB_RUNTIME_MODEL_CANARY_317cce" not in raw for _, raw in captures)
        summary["concurrent_distinct"] = 8
        summary["canary_absent"] = True
        caller.publish(evidence / "summary.json", caller.encoded(summary))
        print(json.dumps(dict(summary=summary, evidence=str(evidence))))
    finally:
        (control / "quit").touch()


def compare_public_oracles(document, expected):
    nodes = document["nodes"]
    by_key = {n["key"]["key"]: n for n in nodes}
    parents = {child["key"]: n["key"]["key"] for n in nodes for child in n["children"]}
    hosts = set()
    candidates = []
    for node in nodes:
        declarations = {v["name"]: v["state"]["value"]["value"] for v in node["source_declarations"]}
        hosts.add(declarations["hosting_root"])
        for name in ("layout_position_source", "layout_size_source"):
            if name not in declarations:
                continue
            ancestor = parents.get(node["key"]["key"])
            while ancestor is not None and ancestor != declarations[name]:
                ancestor = parents.get(ancestor)
            assert ancestor is not None and ancestor in by_key, "cross-tree input inheritance"
        role = node["native_role"].get("value", {}).get("value", "")
        assert "<" not in role
        if role not in ("SwiftUI.Color", "SwiftUI.Text"):
            continue
        state = node["properties"][0]["state"]
        assert state["availability"] == "known"
        geometry = state["value"]["value"]
        mapping = geometry["transform"]
        assert mapping["status"] == "known"
        a, b, c, d, tx, ty = mapping["transform"]["affine"]
        rect = geometry["shape"]["value"]
        x, y, w, h = (rect[k] for k in ("x", "y", "width", "height"))
        points = [(a*px+c*py+tx, b*px+d*py+ty) for px, py in ((x,y),(x+w,y),(x+w,y+h),(x,y+h))]
        xs, ys = zip(*points)
        candidates.append((role, [min(xs), min(ys), max(xs)-min(xs), max(ys)-min(ys)], node))
    assert len(hosts) == 2, "content and toolbar roots must both be captured"
    selected = {}
    for name, rect in expected.items():
        role = "SwiftUI.Color" if name == "color" else "SwiftUI.Text"
        matches = [node for kind, actual, node in candidates if kind == role and
                   max(abs(a-b) for a, b in zip(actual, rect)) < 1e-8]
        assert len(matches) == 1, f"public {name} oracle has {len(matches)} matching source leaves"
        selected[name] = matches[0]
    assert set(selected) == {"color", "text", "button", "toolbar"}
    return selected


def rust_measure(bundle, snapshot, document, nodes, relation):
    geometries = [n["properties"][0]["state"]["value"]["value"] for n in nodes]
    anchors = [dict(element=n["key"], frame_kind="layout_bounds", coordinate_space=g["coordinate_space"],
                    axis="x", fraction=1 if relation == "gap" and i == 0 else 0)
               for i, (n, g) in enumerate(zip(nodes, geometries))]
    query = dict(id="fixture-public-oracle", scope_id=document["context"]["scope_id"],
                 targets=[n["key"] for n in nodes], applies_when=dict(platform=None, input_mode=None, text_scale=None),
                 operation=relation, anchors=anchors, quantity_kind="length", units="pt")
    path = snapshot.with_name(snapshot.stem + "-" + relation + ".query.json")
    path.write_text(json.dumps(dict(schema_version="0.2.0", artifact=dict(kind="geometry_query", data=query))))
    result = subprocess.run([str(bundle / "uiblueprint"), "measure", "--snapshot", str(snapshot), "--query", str(path),
        "--space", geometries[0]["transform"]["transform"]["to"]["id"],
        "--max-input-bytes", "16777216", "--max-output-bytes", "33554432", "--json"],
        check=True, capture_output=True, timeout=5)
    data = json.loads(result.stdout)["artifact"]["data"]["result"]
    assert data["status"] == "known"
    value = data["measurement"]["value"]
    assert value["type"] == "quantity"
    return value["value"]["amount"]


if __name__ == "__main__":
    main()
