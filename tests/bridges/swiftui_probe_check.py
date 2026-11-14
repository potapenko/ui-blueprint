#!/usr/bin/env python3
"""Check SDK-produced synthetic records with the real canonical Rust consumers.

Run Swift tests with UIB_SWIFTUI_TEST_OUTPUT set to an owned temporary directory
first. This is a codec/geometry regression, never actual-app or visual acceptance.
"""
import argparse
import json
import pathlib
import subprocess
import tempfile


def run(command):
    result = subprocess.run(command, text=True, capture_output=True, timeout=10)
    if result.returncode:
        raise RuntimeError(f"bounded consumer refused: {result.stderr.strip() or result.stdout.strip()}")
    return json.loads(result.stdout)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cli", type=pathlib.Path, required=True)
    parser.add_argument("--validator", type=pathlib.Path, required=True)
    parser.add_argument("--records", type=pathlib.Path, required=True)
    args = parser.parse_args()
    space = dict(id="swiftui-root-pt", kind="local", units="pt", origin="top_left")
    with tempfile.TemporaryDirectory(prefix="uib-swiftui-query-") as directory:
        query_path = pathlib.Path(directory) / "query.json"
        for filename, expected, selected in [
            ("before.json", 8, [("icon", 1.0), ("text", 0.0)]),
            ("after.json", 18, [("icon", 1.0), ("text", 0.0)]),
            ("mounted-subtree.json", 4, [("panel", 0.0), ("panel-row-0", 0.0)]),
        ]:
            path = args.records / filename
            result = run([str(args.validator), "--max-bytes", "1048576", str(path)])
            assert result["valid"] is True
            snapshot = json.loads(path.read_text())["artifact"]["data"]
            nodes = {
                next(d["state"]["value"]["value"] for d in node["source_declarations"]
                     if d["name"] == "marker_id"): node for node in snapshot["nodes"]
            }
            anchors = [dict(element=nodes[name]["key"], frame_kind="layout_bounds",
                            coordinate_space=space, fraction=fraction, axis="x")
                       for name, fraction in selected]
            query_path.write_text(json.dumps(dict(schema_version="0.2.0", artifact=dict(
                kind="geometry_query", data=dict(id="gap", scope_id=snapshot["context"]["scope_id"],
                targets=[a["element"] for a in anchors], operation="gap", anchors=anchors,
                quantity_kind="length", units="pt",
                applies_when=dict(platform=None, text_scale=None, input_mode=None))))))
            measured = run([str(args.cli), "measure", "--snapshot", str(path), "--query", str(query_path),
                            "--space", "swiftui-root-pt", "--max-input-bytes", "1048576",
                            "--max-output-bytes", "1048576", "--json"])["artifact"]["data"]["result"]
            assert measured["status"] == "known"
            assert measured["measurement"]["value"]["value"]["amount"] == expected
    assert not pathlib.Path(directory).exists()
    print("SwiftUI SDK synthetic records: canonical validation, gaps 8/18 pt and optional-row inset 4 pt passed")


if __name__ == "__main__":
    main()
