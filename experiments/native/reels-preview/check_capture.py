"""Verify saved R11 cells. Geometry selects no runtime node and grants no identity.

All 21 nodes are selected by the qualified owner/grid/child structure first.
An unordered one-to-one match with independent public anchors is a fixture test,
never semantic row ordering or correspondence in a product snapshot.
"""
import argparse
import json
import math
from pathlib import Path
import subprocess

OWNER = "R11PreviewFixture.FeatureGateSubscribePreview"


def require(condition, reason):
    if not condition: raise ValueError(reason)


def role(node):
    return node.get("native_role", {}).get("value", {}).get("value")


def select_cells(document, manifest):
    require(manifest["status"] == "ready" and manifest["cleanup"] == "detached_restored", "capture incomplete")
    provenance = manifest.get("selection_provenance", {})
    require(provenance.get("source_type") == OWNER and provenance.get("selection_source") == "body_owner", "wrong owner")
    audit = manifest["cache_audit"]
    require(audit["model_range_reads"] == 0 and audit["evaluated_swiftui_or_graph"] is False, "privacy/evaluation violation")
    snapshot = document["artifact"]["data"]
    nodes = snapshot["nodes"]
    by = {n["key"]["key"]: n for n in nodes}
    require(len(by) == len(nodes), "duplicate node key")
    grids = [n for n in nodes if role(n) == "LazyVGrid"]
    require(len(grids) == 1, "grid not unique")
    node = grids[0]
    for expected in ("ResettableLazyLayoutRoot", "Tree"):
        require(len(node["children"]) == 1, "unexpected grid structure")
        node = by[node["children"][0]["key"]]
        require(role(node) == expected, "unexpected grid structure")
    keys = [key["key"] for key in node["children"]]
    require(len(keys) == len(set(keys)) == 21, "not 21 distinct cells")
    cells = [by[k] for k in keys]
    require(all(role(n) == "HiddenForReuseEffect" for n in cells), "wrong cell boundary")
    require({n["key"]["key"] for n in nodes if role(n) == "HiddenForReuseEffect"} == set(keys), "extra cell boundary")
    return snapshot, cells


def window_rect(snapshot, node):
    field = next(p for p in node["properties"] if p["field"] == "layout_bounds")
    require(field["state"]["availability"] == "known", "layout unknown")
    require(field["evidence"]["provenance"] == "reported", "layout not reported")
    geometry = field["state"]["value"]["value"]
    require(geometry["frame_kind"] == "layout_bounds" and geometry["shape"]["shape"] == "rect", "not layout rect")
    mapping = geometry["transform"]
    require(mapping["status"] == "known", "window transform unknown")
    transform = mapping["transform"]
    context = snapshot["context"]
    require(transform["from"] == geometry["coordinate_space"] and transform["to"]["kind"] == "window"
            and transform["to"]["units"] == "pt" and transform["to"]["origin"] == "top_left", "wrong coordinate space")
    require(transform["target"] == context["target"] and transform["surface"] == node["surface"]
            and transform["environment_revision"] == context["environment_revision"], "stale mapping")
    require(transform["evidence"]["method"].endswith(".NSView.convertPoint"), "no native mapping evidence")
    a, b, c, d, tx, ty = transform["affine"]
    require([a,b,c,d] == [1,0,0,1], "unsupported fixture transform")
    rect = geometry["shape"]["value"]
    values = [rect[k] for k in ("x", "y", "width", "height")]
    require(all(math.isfinite(v) for v in values+[tx,ty]) and values[2] >= 0 and values[3] >= 0, "invalid rect")
    # Test oracle comparison, not replacement production geometry arithmetic.
    return geometry, [values[0]+tx, values[1]+ty, values[2], values[3]]


def compare(document, manifest, oracle, request):
    snapshot, cells = select_cells(document, manifest)
    require(oracle["pid"] == request["pid"] and oracle["window_id"] == request["window_id"], "oracle target mismatch")
    require(snapshot["context"]["target"]["id"] == "macos-pid-"+str(request["pid"]), "snapshot target mismatch")
    require(len(oracle["cells"]) == 21, "oracle incomplete")
    # Public anchors are snapped to device pixels; raw cache values are not.
    # One physical pixel bounds coordinate/extent rounding in THIS fixture test.
    # This is not a product tolerance or a claim of exact raw==anchor equality.
    tolerance = 1/oracle["display_scale"]
    unused, rows = set(oracle["cells"]), []
    for node in cells:
        geometry, actual = window_rect(snapshot, node)
        candidates = [(key, max(abs(x-y) for x,y in zip(actual, value["window"])))
                      for key,value in oracle["cells"].items()]
        matches = [(key, error) for key,error in candidates if error <= tolerance+1e-9]
        require(len(matches) == 1 and matches[0][0] in unused, "no unique oracle bijection")
        key, error = matches[0]; unused.remove(key)
        rows.append(dict(oracle_id=int(key), source_key=node["key"],
            source_layout=geometry, window_rect=actual, anchor_window_rect=oracle["cells"][key]["window"],
            max_abs_difference_pt=error))
    require(not unused, "oracle coverage missing")
    return dict(status="fixture-pass", cells=sorted(rows,key=lambda r:r["oracle_id"]),
        max_abs_difference_pt=max(r["max_abs_difference_pt"] for r in rows),
        comparison_tolerance_pt=tolerance, exact_equality=all(r["max_abs_difference_pt"] < 1e-9 for r in rows),
        semantic_order="unknown; oracle correspondence is comparison-only", donor_admission=False)


def run(case, bundle=None):
    request = json.loads((case/"request.json").read_text())
    manifest = json.loads((case/("manifest-"+request["request_id"]+".json")).read_text())
    require(manifest.get("snapshot") is not None, "capture has no snapshot")
    source = case/manifest["snapshot"]
    before = (case/"oracle-before.json").read_bytes()
    after = case/"oracle-after.json"
    require(after.exists() and after.read_bytes() == before, "oracle not stable across capture")
    document = json.loads(source.read_text()); oracle = json.loads(before)
    report = compare(document,manifest,oracle,request)
    if bundle:
        valid = subprocess.run([str(bundle/"uiblueprint-validate"), "--max-bytes", "1048576", str(source)],capture_output=True,text=True)
        require(valid.returncode == 0, "canonical validation: "+valid.stdout+valid.stderr)
        snapshot = document["artifact"]["data"]
        results = []
        query_file = case/"query.json"
        try:
            for row in report["cells"]:
                g = row["source_layout"]
                for operation,axis,index in (("width","x",2),("height","y",3)):
                    query = dict(schema_version="0.2.0",artifact=dict(kind="geometry_query",data=dict(
                        id="r11-"+operation,scope_id=snapshot["context"]["scope_id"],targets=[row["source_key"]],
                        operation=operation,anchors=[dict(element=row["source_key"],frame_kind="layout_bounds",
                            coordinate_space=g["coordinate_space"],fraction=0,axis=axis)],
                        quantity_kind="length",units="pt",applies_when=dict(platform=None,input_mode=None,text_scale=None))))
                    query_file.write_text(json.dumps(query))
                    p = subprocess.run([str(bundle/"uiblueprint"),"measure","--snapshot",str(source),"--query",str(query_file),
                        "--space",g["transform"]["transform"]["to"]["id"],"--max-input-bytes","1048576",
                        "--max-output-bytes","1048576","--json"],capture_output=True,text=True)
                    require(p.returncode == 0,"Rust measurement failed: "+p.stderr+p.stdout[:120])
                    result=json.loads(p.stdout)["artifact"]["data"]
                    measured=result["result"]
                    require(measured["status"] == "known" and
                        measured["measurement"]["space"] == g["transform"]["transform"]["to"], "Rust returned wrong space")
                    require(abs(measured["measurement"]["value"]["value"]["amount"]-row["window_rect"][index]) < 1e-9,
                        "Rust/oracle comparison arithmetic disagrees")
                    results.append(dict(oracle_id=row["oracle_id"],operation=operation,
                        result={key:value for key,value in result.items() if key not in ("snapshot","evaluation")}))
        finally:
            query_file.unlink(missing_ok=True)
        report["rust_measurements"] = results
        report["canonical_validation"] = "passed"
    return report


if __name__ == "__main__":
    p=argparse.ArgumentParser(description=__doc__);p.add_argument("case",type=Path);p.add_argument("--bundle",type=Path)
    p.add_argument("--out",type=Path);a=p.parse_args();result=run(a.case,a.bundle)
    if a.out:
        with a.out.open("x") as output: json.dump(result,output,indent=2)
    print(json.dumps({k:v for k,v in result.items() if k not in ("cells","rust_measurements")}))
