"""Authored analysis vectors. Never calls the candidate engine or changes goldens."""
import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
cases = []
C = copy.deepcopy


def old(name):
    return json.loads((ROOT / "fixtures/golden" / (name + ".json")).read_text())["artifact"]["data"]


def measurement(name="GEO-GAP", details=None):
    original = old(name)
    s, e = C(original["snapshot"]), original["expectation"]
    rule = e["rule"]
    q = {k: C(e[k]) for k in ("id", "scope_id", "targets", "applies_when")}
    q.update({k: C(rule[k]) for k in ("operation", "anchors", "quantity_kind", "units")})
    space = C(q["anchors"][0]["coordinate_space"])
    evidence = s["nodes"][0]["properties"][0]["evidence"]
    return dict(snapshot=s, query=q, evaluation=dict(snapshot_id=s["id"], revision=s["revision"],
        context=C(s["context"]), result_space=space, transforms=[], conditions=None),
        result=dict(status="known", measurement=dict(value=C(original["finding"]["measured"]),
            space=C(space), details=details or dict(kind="scalar"), evidence=[C(evidence)])))


def save(name, data, kind="measurement", valid=True, structural=True, engine="not_checked"):
    path = OUT / (name + ".json")
    text = json.dumps(dict(schema_version="0.2.0", artifact=dict(kind=kind, data=data)),
        ensure_ascii=False, indent=2) + "\n"
    if not path.exists() or path.read_text() != text:
        path.write_text(text)
    cases.append(dict(path=name + ".json", contract_valid=valid, structural_valid=structural,
        engine_verification=engine))


def unknown(data, reason, evidence=None):
    data["result"] = dict(status="unknown", reason=reason, evidence=[] if evidence is None else C(evidence))
    return data


def geom(data, i=0):
    return data["snapshot"]["nodes"][i]["properties"][0]["state"]["value"]["value"]


def observation(data, name, method):
    o = C(data["snapshot"]["observations"][0]); o["id"] = name
    data["snapshot"]["observations"].append(o)
    e = C(data["snapshot"]["nodes"][0]["properties"][0]["evidence"])
    e.update(observation_id=name, method=method)
    return e


def transform(data, source, destination, affine, evidence):
    ctx = data["snapshot"]["context"]
    return {"from":C(source),"to":C(destination),"affine":affine,
        "evidence":C(evidence),"target":C(ctx["target"]),"surface":C(ctx["surfaces"][0]),
        "environment_revision":ctx["environment_revision"]}


gap = measurement()
save("query-gap", gap["query"], "geometry_query")
save("evaluation-local", gap["evaluation"], "evaluation_input")
save("measurement-gap", gap, engine="match")  # Independent GEO literal8.
for name, x, amount in [("known-zero", 40, 0), ("signed-gap", 30, -10)]:
    d = C(gap); geom(d, 1)["shape"]["value"]["x"] = x
    d["result"]["measurement"]["value"]["value"]["amount"] = amount
    save(name, d, engine="match")  # A.right40; B.left40/30.
for source, name, details in [
    ("GEO-SIZE-RATIO__width", "width", None),
    ("GEO-SIZE-RATIO__height", "height", None),
    ("GEO-SIZE-RATIO__ratio", "ratio", None),
    ("GEO-INSIDE", "insets", dict(kind="insets",left=10,top=20,right=80,bottom=30)),
    ("GEO-OVERFLOW", "overflow", dict(kind="insets",left=110,top=20,right=-10,bottom=30)),
    ("GEO-INTERSECTS", "intersection", dict(kind="intersection",rect=dict(x=35,y=25,width=5,height=5))),
    ("GEO-EQUAL-SPACING", "ordered-gaps", dict(kind="gaps",values=[8,8])),
]:
    save(name, measurement(source, details), engine="match")
d = measurement("GEO-INTERSECTS", dict(kind="intersection",rect=None))
geom(d, 1)["shape"]["value"]["x"] = 40
d["result"]["measurement"]["value"]["value"]["amount"] = 0
save("empty-intersection", d, engine="match")

d = C(gap); p = d["snapshot"]["nodes"][0]["properties"][0]
p["state"] = dict(availability="unknown",reason="authored_unknown")
save("unknown-property", unknown(d,"unknown_property",[p["evidence"]]), engine="match")
d = C(gap); p = d["snapshot"]["nodes"][0]["properties"][0]
p["state"] = dict(availability="unsupported",reason="authored_not_exposed")
save("unsupported-property", unknown(d,"unsupported_property",[p["evidence"]]), engine="match")
d = C(gap)
for context in [d["snapshot"]["context"],d["snapshot"]["coverage"],d["snapshot"]["observations"][0]["coverage"]]:
    context["fields"] = ["role"]
for node in d["snapshot"]["nodes"]:
    role = C(node["properties"][0]);role.update(field="role",state=dict(availability="known",value=dict(type="role",value="button")))
    node["properties"] = [dict(selection="not_requested",field="layout_bounds"),role]
d["evaluation"]["context"] = C(d["snapshot"]["context"])
save("not-requested", unknown(d,"not_requested"), engine="match")
d = C(gap); p = d["snapshot"]["nodes"][0]["properties"][0]
p["sensitivity"] = "sensitive"; p["state"] = dict(availability="redacted")
save("redacted", unknown(d,"redacted_property",[p["evidence"]]), engine="match")
d = C(gap); d["query"]["targets"].append(dict(namespace="fixture.authored_layout",key="absent"))
save("missing-target", unknown(d,"target_unresolved"), engine="match")
d = C(gap); geom(d)["shape"] = dict(shape="polygon",value=[dict(x=10,y=20),dict(x=40,y=20),dict(x=40,y=30)])
save("unsupported-shape", unknown(d,"unsupported_shape",gap["result"]["measurement"]["evidence"]), engine="match")
d = C(gap); geom(d,1)["coordinate_space"]["id"] = "other-local"
d["query"]["anchors"][1]["coordinate_space"]["id"] = "other-local"
save("missing-transform", unknown(d,"missing_transform",gap["result"]["measurement"]["evidence"]), engine="match")
d = C(gap); d["snapshot"]["observations"][0].update(consistency="unknown",consistency_reason="authored_unstable")
save("unstable-source", unknown(d,"unstable_state",gap["result"]["measurement"]["evidence"]), engine="match")

d = C(gap); second = observation(d,"OG2","authored_second_geometry")
d["snapshot"]["nodes"][1]["properties"][0]["evidence"] = second
d["result"]["measurement"]["evidence"].append(C(second))
save("distinct-observations", d, engine="match")

multi = C(gap); source = multi["evaluation"]["result_space"]
middle = dict(id="intermediate-css",kind="local",units="css_px",origin="top_left")
destination = dict(id="screen-px",kind="screen",units="px",origin="top_left")
e1 = observation(multi,"OT1","authored_translation"); e2 = observation(multi,"OT2","authored_scale")
unused = observation(multi,"OU","authored_unused_mapping")
multi["evaluation"]["transforms"] = [transform(multi,source,middle,[1,0,0,1,5,0],e1),
    transform(multi,middle,destination,[2,0,0,2,100,200],e2),
    transform(multi,source,dict(id="unused",kind="local",units="css_px",origin="top_left"),[1,0,0,1,0,0],unused)]
multi["evaluation"]["result_space"] = C(destination); multi["query"]["units"] = "px"
multi["result"]["measurement"].update(space=C(destination),evidence=[C(gap["result"]["measurement"]["evidence"][0]),e1,e2])
multi["result"]["measurement"]["value"]["value"].update(amount=16,source_units="px")
save("multi-hop-conversion", multi, engine="match")  # Translation cancels,8css_px*2=16px.

d = measurement("GEO-BASELINE")
d["snapshot"]["nodes"][1]["properties"][0]["state"]["value"]["value"]["coordinate"] = 30
source = d["evaluation"]["result_space"]
destination = dict(id="flipped-px",kind="local",units="px",origin="bottom_left")
ev = observation(d,"OT-baseline","authored_baseline_flip")
d["evaluation"].update(result_space=C(destination),transforms=[transform(d,source,destination,[2,0,0,-2,0,100],ev)])
d["query"]["units"] = "px"; m = d["result"]["measurement"]
m.update(space=C(destination));m["value"]["value"].update(amount=6,source_units="px");m["evidence"].append(ev)
save("baseline-origin-conversion", d, engine="match")  # 27->46,30->40; spread6.

conditional = C(gap); conditional["query"]["applies_when"]["platform"] = "macos"
ce = observation(conditional,"OC1","authored_observed_conditions")
conditional["evaluation"]["conditions"] = dict(values=dict(platform="macos",input_mode=None,text_scale=None),evidence=ce)
conditional["result"]["measurement"]["evidence"].insert(0,C(ce))
save("observed-conditions", conditional, engine="match")
d = C(conditional); d["evaluation"]["conditions"] = None
save("missing-conditions", unknown(d,"applicability_unknown"), engine="match")
d = C(conditional); d["evaluation"]["conditions"]["values"]["platform"] = "web"
save("not-applicable", unknown(d,"not_applicable",[ce]), engine="match")
d = C(conditional); d["snapshot"]["observations"][-1].update(consistency="unknown",consistency_reason="authored_unstable")
save("unstable-conditions", unknown(d,"unstable_state",[ce]), engine="match")

original = old("GEO-GAP")
check = dict(snapshot=C(gap["snapshot"]),expectation=C(original["expectation"]),evaluation=C(gap["evaluation"]),
    measurement=C(gap["result"]),finding=C(original["finding"]))
check["finding"]["reason"] = "expectation_satisfied"
save("check-pass", check, "geometry_check", engine="match")
converted = C(check);converted.update(snapshot=C(multi["snapshot"]),evaluation=C(multi["evaluation"]),measurement=C(multi["result"]))
converted["expectation"]["rule"].update(units="px",expected=16)
converted["finding"]["measured"] = C(multi["result"]["measurement"]["value"])
save("check-converted", converted, "geometry_check", engine="match")

for name, mutation in [
    ("invalid-revision", lambda d: d["evaluation"].update(revision=999)),
    ("invalid-result-space", lambda d: d["result"]["measurement"]["space"].update(id="unrelated")),
    ("invalid-units", lambda d: d["result"]["measurement"]["value"]["value"].update(source_units="pt")),
    ("invalid-details", lambda d: d["result"]["measurement"].update(details=dict(kind="gaps",values=[8]))),
    ("invalid-empty-evidence", lambda d: d["result"]["measurement"].update(evidence=[])),
    ("invalid-evidence-source", lambda d: d["result"]["measurement"]["evidence"][0].update(method="not_supplied")),
    ("invalid-known-missing-target", lambda d: d["query"]["targets"].append(dict(namespace="fixture.authored_layout",key="absent"))),
    ("invalid-kind", lambda d: d["result"]["measurement"]["value"]["value"].update(kind="area")),
]:
    d=C(gap);mutation(d);save(name,d,valid=False)
d=C(gap);d["query"]["expected"]=8;save("invalid-query-normative-field",d,valid=False,structural=False)
d=C(gap);d["result"]=dict(status="unknown",reason="unknown_property",evidence=[],value=0)
save("invalid-unknown-value",d,valid=False,structural=False)
d=C(gap);d["result"]["measurement"]["details"]["value"]=0
save("invalid-scalar-payload",d,valid=False,structural=False)
d=C(check);d["finding"]["status"]="fail";save("invalid-check-outcome",d,"geometry_check",valid=False)
d=C(multi);d["evaluation"]["transforms"][0]["target"]["generation"]="wrong"
save("invalid-transform-generation",d,valid=False)
d=C(multi);d["evaluation"]["transforms"][0]["affine"]=[0,0,0,0,0,0]
save("invalid-singular-transform",d,valid=False)
d=C(conditional);d["evaluation"]["conditions"]["evidence"]["observation_id"]="absent"
save("invalid-condition-reference",d,valid=False)

# These are deliberately contract-valid declarations, NOT verified calculations.
d=C(gap);d["result"]["measurement"]["value"]["value"]["amount"]=9
save("tampered-quantity-contract-only",d,engine="mismatch")
d=C(check);d["measurement"]["measurement"]["value"]["value"]["amount"]=9
d["finding"].update(status="fail",reason="expectation_mismatch",measured=C(d["measurement"]["measurement"]["value"]))
save("tampered-check-contract-only",d,"geometry_check",engine="mismatch")
save("tampered-unknown-contract-only",unknown(C(gap),"unknown_property"),engine="mismatch")

# R1: positional forms violate the existing object-only schema at each enum boundary.
for name, source_name, replacement in [
    ("invalid-details-array-scalar", "measurement-gap", ["scalar"]),
    ("invalid-details-array-insets", "insets", ["insets", 10, 20, 80, 30]),
    ("invalid-details-array-intersection", "intersection", ["intersection", dict(x=35,y=25,width=5,height=5)]),
    ("invalid-details-array-gaps", "ordered-gaps", ["gaps", [8,8]]),
]:
    d=C(json.loads((OUT / (source_name + ".json")).read_text())["artifact"]["data"])
    d["result"]["measurement"]["details"] = replacement
    save(name,d,valid=False,structural=False)
d=C(gap);d["result"]=["known",C(gap["result"]["measurement"])]
save("invalid-result-array-known",d,valid=False,structural=False)
d=C(gap);d["result"]=["unknown","unknown_property",[]]
save("invalid-result-array-unknown",d,valid=False,structural=False)
d=C(check);d["measurement"]=["known",C(check["measurement"]["measurement"])]
save("invalid-check-result-array",d,"geometry_check",valid=False,structural=False)
for kind, payload in [("geometry_query",gap["query"]),("evaluation_input",gap["evaluation"]),
                      ("measurement",gap),("geometry_check",check)]:
    name="invalid-artifact-array-" + kind.replace("_", "-")
    path=OUT / (name + ".json")
    text=json.dumps(dict(schema_version="0.2.0",artifact=[kind,C(payload)]),ensure_ascii=False,indent=2)+"\n"
    if not path.exists() or path.read_text()!=text:
        path.write_text(text)
    cases.append(dict(path=name+".json",contract_valid=False,structural_valid=False,engine_verification="not_checked"))

(OUT / "manifest.json").write_text(json.dumps(cases,indent=2)+"\n")
print(f"Authored {len(cases)} analysis cases; no engine invoked.")
