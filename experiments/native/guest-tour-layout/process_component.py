"""Normalize and run the installed Rust validator/measure after owner recovery."""
import json
import os
import subprocess
import sys
from pathlib import Path

from normalize import normalize, unique_object


def save(path, data):
    with os.fdopen(os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600), "w") as stream:
        json.dump(data, stream, allow_nan=False)


def process(receipt, bundle):
    with receipt.open("rb") as stream:
        raw = stream.read(16385)
    if len(raw) > 16384:
        raise ValueError("receipt_too_large")
    source = normalize(json.loads(raw, object_pairs_hook=unique_object))
    output = receipt.with_name(receipt.stem + "-snapshot.json")
    save(output, source)

    def run(args):
        result = subprocess.run(args, capture_output=True, text=True, timeout=10)
        if result.returncode:
            raise ValueError("canonical_tool_refused")
        return result.stdout

    run([str(bundle / "uiblueprint-validate"), "--max-bytes", "65536", str(output)])
    snapshot = source["artifact"]["data"]
    measurements = []
    for node in snapshot["nodes"]:
        if not node["properties"] or node["properties"][0]["state"]["availability"] != "known":
            continue
        geometry = node["properties"][0]["state"]["value"]["value"]
        for operation, axis in (("width", "x"), ("height", "y")):
            name = node["key"]["key"] + "." + operation
            query = {"schema_version": "0.2.0", "artifact": {"kind": "geometry_query", "data": {
                "id": name, "scope_id": snapshot["context"]["scope_id"], "targets": [node["key"]],
                "applies_when": {"platform": None, "input_mode": None, "text_scale": None},
                "operation": operation, "anchors": [{"element": node["key"], "frame_kind": "layout_bounds",
                    "coordinate_space": geometry["coordinate_space"], "fraction": 0, "axis": axis}],
                "quantity_kind": "length", "units": "pt"}}}
            query_path = receipt.parent / (name + ".query.json")
            save(query_path, query)
            measured = json.loads(run([str(bundle / "uiblueprint"), "measure", "--snapshot", str(output), "--query", str(query_path),
                "--space", geometry["coordinate_space"]["id"], "--max-input-bytes", "65536", "--max-output-bytes", "65536", "--json"]))
            save(receipt.parent / (name + ".measurement.json"), measured)
            measurements.append({"id": name, "result": measured["artifact"]["data"]["result"]})
    print(json.dumps({"snapshot": str(output), "validated": True, "measurements": measurements}, allow_nan=False))


if __name__ == "__main__":
    try:
        if len(sys.argv) != 3:
            raise ValueError("arguments")
        process(Path(sys.argv[1]), Path(sys.argv[2]))
    except (OSError, ValueError, KeyError, TypeError, subprocess.TimeoutExpired):
        print("component_processing_refused", file=sys.stderr)
        sys.exit(2)
