"""Synthetic normalization and real CLI checks; no product process is selected."""
import copy
import json
import subprocess
import sys
from pathlib import Path

from normalize import BUNDLE, IDENTITY, MODULE, NAMESPACE, SOURCE, normalize, unique_object


def main():
    bundle, output = map(Path, sys.argv[1:])
    assert output.is_dir()
    identity = {**IDENTITY, "bundle": BUNDLE, "window_screen_pt": [0, 30, 1920, 1050]}
    record = {
        "source_site": SOURCE, "module_uuid": MODULE, "card": "language",
        "space": "guest-tour GeometryReader local points", "target_xywh": [11.25, 22.5, 33, 44],
        "rect_xywh": [55.5, 66.25, 77.5, 88.75], "start_xy": [27.75, 44.5],
        "identity_before_after": IDENTITY, "detached_verified": True,
        "observation_id": "0123456789abcdef0123456789abcdef", "clock_domain": "owner-lldb-process.monotonic",
        "time_unit": "ns", "read_start": 123456789, "read_end": 234567890, "units": "pt",
        "before": identity, "after": identity, "runtime_state": "synthetic test only",
    }
    snapshot = normalize(record)
    data = snapshot["artifact"]["data"]
    assert data["coverage"]["status"] == "partial" and not data["components"] and not data["captures"]
    assert data["observations"][0]["start"] == 123.456789
    first = data["nodes"][0]
    geometry = first["properties"][0]["state"]["value"]["value"]
    assert geometry["shape"]["value"] == {"x": 55.5, "y": 66.25, "width": 77.5, "height": 88.75}
    assert geometry["transform"] == {"status": "local_only"}
    rejected = 0
    for field, value in [("detached_verified", False), ("read_start", -1), ("read_end", 0),
                         ("read_start", True), ("card", "unknown"), ("module_uuid", "other"),
                         ("target_xywh", [1, 2, 3]), ("rect_xywh", [0, 0, -1, 3]),
                         ("rect_xywh", [0, 0, float("nan"), 3]), ("start_xy", [0, 0]),
                         ("observation_id", "not-an-observation"), ("before", {**identity, "window_id": 1061})]:
        bad = copy.deepcopy(record)
        bad[field] = value
        try:
            normalize(bad)
        except ValueError:
            rejected += 1
        else:
            raise AssertionError(field)
    try:
        json.loads('{"card":"language","card":"actor"}', object_pairs_hook=unique_object)
    except ValueError:
        rejected += 1
    else:
        raise AssertionError("duplicate accepted")
    source = output / "synthetic-snapshot.json"
    source.write_text(json.dumps(snapshot))
    def run(args):
        result = subprocess.run(args, text=True, capture_output=True, timeout=10)
        assert result.returncode == 0, (args[0], result.returncode, result.stderr)
        return result.stdout
    run([str(bundle / "uiblueprint-validate"), "--max-bytes", "65536", str(source)])
    run([str(bundle / "uiblueprint"), "inspect", "--snapshot", str(source), "--ref", json.dumps(first["key"]),
         "--view", "design", "--max-input-bytes", "65536", "--max-output-bytes", "65536"])
    space = geometry["coordinate_space"]
    query = {"schema_version": "0.2.0", "artifact": {"kind": "geometry_query", "data": {
        "id": "synthetic-card-width", "scope_id": data["context"]["scope_id"], "targets": [first["key"]],
        "applies_when": {"platform": None, "input_mode": None, "text_scale": None}, "operation": "width",
        "anchors": [{"element": first["key"], "frame_kind": "layout_bounds", "coordinate_space": space, "fraction": 0, "axis": "x"}],
        "quantity_kind": "length", "units": "pt"}}}
    query_path = output / "synthetic-query.json"
    query_path.write_text(json.dumps(query))
    measured = json.loads(run([str(bundle / "uiblueprint"), "measure", "--snapshot", str(source), "--query", str(query_path),
                              "--space", space["id"], "--max-input-bytes", "65536", "--max-output-bytes", "65536", "--json"]))
    result = measured["artifact"]["data"]["result"]
    assert result["status"] == "known"
    assert result["measurement"]["value"]["value"]["amount"] == 77.5
    assert result["measurement"]["evidence"][0]["source_namespace"] == NAMESPACE
    print(f"Synthetic only: {rejected} refusals; canonical validate, inspect and width 77.5 pt passed.")


if __name__ == "__main__":
    main()
