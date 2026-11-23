"""Synthetic selected-component/transform tests; invoke with bundle and task temp."""
import copy
import json
import subprocess
import sys
from pathlib import Path

from normalize import BUNDLE, IDENTITY, MODULE, normalize
from group_value import decode_group


def main():
    bundle, output = map(Path, sys.argv[1:])
    identity = {**IDENTITY, "bundle": BUNDLE, "window_screen_pt": [510, 330, 900, 572]}
    receipt = {"format": "playphraseme.guest-tour.component-1", "card": "context", "module_uuid": MODULE,
               "source_revision": "e19b20c474afe6462786ca0dd3dd5ad9ae37dbce", "records": [],
               "units": "pt", "time_unit": "ns", "clock_domain": "owner-lldb-" + "f" * 32,
               "detached_verified": True, "stationary_window": True, "before": identity,
               "after": copy.deepcopy(identity), "fixture": False,
               "consistency": "sequential-source-reads-not-presented-frame", "groups": "not_collected"}
    for i, (kind, values) in enumerate([
        ("map", {"origin": [5, 7], "viewport": [10, 20, 600, 400], "scale": [1]}),
        ("origin", {"origin": [15, 17]}),
        ("card", {"target": [220, 200, 40, 20], "rect": [10, 200, 100, 50], "start": [240, 210]}),
    ]):
        receipt["records"].append({"kind": kind, "values": values, "observation_id": str(i + 1) * 32,
                                   "read_start": 1000 + i * 1000, "read_end": 1500 + i * 1000})
    source = normalize(receipt)
    snapshot = source["artifact"]["data"]
    assert len(snapshot["observations"]) == 3
    assert all(o["consistency"] == "unknown" for o in snapshot["observations"])
    first, _, viewport = snapshot["nodes"]
    space = first["properties"][0]["state"]["value"]["value"]["transform"]["transform"]["to"]
    query = {"schema_version": "0.2.0", "artifact": {"kind": "geometry_query", "data": {
        "id": "different-local-origins", "scope_id": snapshot["context"]["scope_id"],
        "targets": [first["key"], viewport["key"]], "operation": "gap", "quantity_kind": "length", "units": "pt",
        "applies_when": {"platform": None, "input_mode": None, "text_scale": None},
        "anchors": [{"element": n["key"], "frame_kind": "layout_bounds", "fraction": 0, "axis": "x",
                     "coordinate_space": n["properties"][0]["state"]["value"]["value"]["coordinate_space"]}
                    for n in (first, viewport)]}}}
    source_path, query_path = output / "component-check-snapshot.json", output / "component-check-query.json"
    source_path.write_text(json.dumps(source))
    query_path.write_text(json.dumps(query))
    subprocess.run([str(bundle / "uiblueprint-validate"), "--max-bytes", "65536", str(source_path)], check=True, timeout=10)
    result = subprocess.run([str(bundle / "uiblueprint"), "measure", "--snapshot", str(source_path), "--query", str(query_path),
                             "--space", space["id"], "--max-input-bytes", "65536", "--max-output-bytes", "65536", "--json"],
                            check=True, capture_output=True, text=True, timeout=10)
    measurement = json.loads(result.stdout)["artifact"]["data"]["result"]["measurement"]
    assert measurement["value"]["value"]["amount"] == -10
    assert len(measurement["evidence"]) == 4
    for mutate in [lambda x: x.update(fixture=True),
                   lambda x: x["after"].update(window_screen_pt=[0, 0, 1, 1]),
                   lambda x: x["records"][1].update(observation_id=x["records"][0]["observation_id"]),
                   lambda x: x["records"][2].update(read_start=0),
                   lambda x: x["records"][0]["values"].update(viewport=[0, 0, -1, 1])]:
        bad = copy.deepcopy(receipt)
        mutate(bad)
        try:
            normalize(bad)
        except ValueError:
            pass
        else:
            raise AssertionError("invalid component receipt admitted")
    for tag, raw in [("nil", b"\x00"), ("some", None), ("some", b"\x04"), ("some", b"\x00\x00"), ("unknown", None)]:
        try:
            decode_group(tag, raw)
        except ValueError:
            pass
        else:
            raise AssertionError("invalid group admitted")
    print("Synthetic component: 5 receipt and 5 group refusals; Rust transformed gap -10 pt, 4 contributing evidence records.")


if __name__ == "__main__":
    main()
