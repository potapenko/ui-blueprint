"""Translate independent authored facts into canonical wire fixtures, not analytics.

Geometry result amounts below are declared oracle records; G01 must compute them.
Expected exit statuses come only from golden-oracles/expected.json.
"""
import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
ORACLES = ROOT / "fixtures/golden-oracles"
FIELDS = ["role", "name", "enabled", "checked"]


def clone(value):
    return copy.deepcopy(value)


def key(name="check-1", namespace="macos.ax"):
    return {"namespace": namespace, "key": name}


def context(fields=None):
    return dict(schema_version="0.1.0", session_id="a1", target=dict(id="native-app", generation="g1"),
                surfaces=[dict(id="window-1", generation="w1")], scope_id="form-1", projection="interaction",
                fields=fields or FIELDS[:], plugin=dict(id="macos", version="0.1.0"), environment_revision="display-1")


def coverage(c, status="complete"):
    return dict(status=status, scope_id=c["scope_id"], fields=c["fields"][:], omitted_count=0 if status == "complete" else None, unknown_count=0 if status == "complete" else None)


def observation(oid="O10", c=None, namespace="macos.ax"):
    c = c or context()
    end = 110 if oid == "O10" else 130
    return dict(id=oid, source_namespace=namespace, channel="external_semantics", start=end-10, end=end,
                clock_domain="fixture-parent-monotonic", time_unit="milliseconds", freshness_basis="live_read",
                consistency_reason=None, answer_source="live", freshness="current", last_verified=end,
                consistency="stable", coverage=coverage(c))


def evidence(oid="O10", namespace="macos.ax"):
    return dict(observation_id=oid, source_namespace=namespace, provenance="reported", method="authored_fixture", uncertainty=None)


def value(kind, val):
    return dict(type=kind, value=val)


def prop(field, val=None, oid="O10", namespace="macos.ax", status="known", sensitivity="public"):
    state = dict(availability=status)
    if status == "known":
        state["value"] = val
    elif status != "redacted":
        state["reason"] = "not_exposed"
    return dict(selection="requested", field=field, sensitivity=sensitivity, evidence=evidence(oid, namespace), state=state)


def unknown_focus():
    return dict(keyboard=dict(status="unknown", reason="not_observed"), accessibility=dict(status="unknown", reason="not_observed"),
                active_descendant=dict(status="unknown", reason="not_observed"), text_selection=None,
                composition_state=dict(selection="not_requested", field="value"))


def snapshot(revision=10):
    c = context(); oid = f"O{revision}"
    node = dict(key=key(), surface=clone(c["surfaces"][0]), native_role=dict(availability="known",value=value("text","AXCheckBox")), children=[], extensions=[], source_declarations=[],
                properties=[prop("role", value("role", "checkbox"), oid), prop("name", value("text", "Пример"), oid),
                            prop("enabled", value("flag", True), oid), prop("checked", value("flag", revision == 11), oid)])
    return dict(surface_records=[], id=f"S{revision}", revision=revision, source_state=f"fixture-checkpoint-{revision}", context=c,
                observations=[observation(oid, c)], nodes=[node], relations=[], components=[], focus=unknown_focus(), captures=[], coverage=coverage(c))


def delta():
    s = snapshot(11)
    return {"base_revision": 10, "revision": 11, "source_state": s["source_state"], "context": clone(s["context"]),
            "observations": clone(s["observations"]), "upsert": clone(s["nodes"]), "removed": [],
            "relations": [], "focus": clone(s["focus"]), "coverage": clone(s["coverage"])}


def action():
    c = context()
    return dict(id="set-checked", context=c, backend_ref=dict(session_id="a1", key=key(), snapshot_id="S10", observation_id="O10", target=clone(c["target"]), surface=clone(c["surfaces"][0])),
                intent=dict(intent="set_checked", value=True), modality="setter", input_space=None, required_enabled=True, authorized_scope="form-1", unique_match=True,
                resolution=dict(evidence=evidence(), writable=dict(availability="known", value=value("flag", True)), value_allowed=dict(availability="known", value=value("flag", True)), available_intents=["set_checked"]))


def request():
    return dict(clock_domain="fixture-parent-monotonic", request_id="request-1", context=context(), limits=dict(max_elements=32, max_depth=8, max_output_bytes=65536, deadline_ms=250), freshness_policy="current_required", operation=dict(operation="observe", channels=["external_semantics"]))


def session():
    c = context()
    return dict(allowed_scopes=["form-1"], session_id="a1", plugin=c["plugin"], supported_versions=["0.1.0"], target=c["target"], surfaces=c["surfaces"],
                capabilities=[dict(channel="external_semantics", operation="observe", status="supported", reason=None), dict(channel="rendered_capture", operation="observe", status="permission_required", reason="capture_permission_absent")])


def transition(verified=True):
    sid = "verify" if verified else "dispatch"
    return dict(id="transition-1", context=context(), steps=[dict(id=sid, action_id="set-checked", delivery="confirmed", outcome="succeeded" if verified else "pending_verification", before_snapshot="S10", after_snapshot="S11" if verified else None, verification_observation="O11" if verified else None)], completed_steps=[sid], stopped_at=None, stop_on_error=True)


def expectation():
    return dict(id="checked-true", scope_id="form-1", targets=[key()], rule=dict(relation="property_equals", field="checked", expected=value("flag", True)), applies_when=dict(platform=None, input_mode=None, text_scale=None), expected_from="user_scenario_GOLDEN01")


def finding(status="pass"):
    return dict(id="checked-finding", expectation_id="checked-true", snapshot_id="S11", observation_id="O11", status=status, measured=value("flag", True) if status != "unknown" else None, reason=None if status != "unknown" else "required_value_unknown")


def issue(code="permission_required"):
    return dict(code=code, scope_id="form-1", failed_step="capture", recovery_class="reobserve")


def document(kind, data):
    return dict(schema_version="0.1.0", artifact=dict(kind=kind, data=data))


def chain():
    return dict(before=snapshot(), action=action(), delivery=transition(False), verification=transition(True), after=snapshot(11), delta=delta(), expectation=expectation(), finding=finding(), export=dict(purpose="compare", before="S10", after="S11", changed=[key()], include_geometry=False, include_pixels=False, description="Checkbox changed false to true."))


def delta_case():
    return dict(base=snapshot(), update=delta(), source_snapshot=snapshot(11))


def space(units="css_px", name="local-form"):
    return dict(id=name, kind="local", units=units, origin="top_left")


def geometry(rect=None, units="css_px", frame="layout_bounds"):
    rect = rect or [10, 20, 30, 10]
    return dict(frame_kind=frame, coordinate_space=space(units), shape=dict(shape="rect", value=dict(zip(["x", "y", "width", "height"], rect))), transform=dict(status="local_only"))


def geometry_case():
    c = context(["layout_bounds"])
    return dict(context=c, geometry=geometry(), observation=observation("OG1", c, "fixture.authored_layout"))


def geometry_finding(operation="gap", measured=8, expected=8, status="pass", comparison="equal", quantity_kind="length", tolerance=0.01, rects=None):
    rects = rects or {"A": [10,20,30,10], "B": [48,20,20,10], "C": [76,20,20,10], "container": [0,0,120,60]}
    s = snapshot(11); c = context(["layout_bounds"]); s["context"] = c; s["coverage"] = coverage(c); s["observations"] = [observation("OG1",c,"fixture.authored_layout")]; s["nodes"] = []
    for name, rect in rects.items():
        s["nodes"].append(dict(key=key(name,"fixture.authored_layout"), surface=clone(c["surfaces"][0]), native_role=dict(availability="unsupported",reason="not_exposed"), children=[], extensions=[], source_declarations=[], properties=[prop("layout_bounds",value("geometry",geometry(rect)),"OG1","fixture.authored_layout")]))
    targets = [n["key"] for n in s["nodes"] if n["key"]["key"] in (list(rects)[:2])]
    e = dict(id="geometry-check",scope_id="form-1",targets=targets,rule=dict(relation="geometry",operation=operation,anchors=[dict(element=k,frame_kind="layout_bounds",coordinate_space=space(),fraction=0.5,axis="x") for k in targets],expected=expected,comparison=comparison,quantity_kind=quantity_kind,units="css_px",tolerance=tolerance),applies_when=dict(platform=None,input_mode=None,text_scale=None),expected_from="fixture_literal")
    f = dict(id="geometry-finding",expectation_id=e["id"],snapshot_id=s["id"],observation_id="OG1",status=status,measured=value("quantity",dict(amount=measured,kind=quantity_kind,source_units="css_px")) if status != "unknown" else None,reason="required_measurement_unknown" if status == "unknown" else None)
    return dict(snapshot=s,expectation=e,finding=f)


def remove_child():
    b = snapshot(); parent = b["nodes"][0]; parent["key"] = key("parent"); parent["children"] = [key("child")]
    child = clone(parent); child["key"] = key("child"); child["children"] = []; b["nodes"].append(child)
    b["relations"] = [{"kind":"labelled_by","from":key("parent"),"to":key("child"),"evidence":evidence()}]
    b["focus"]["keyboard"] = dict(status="known",target=key("child"),evidence=evidence())
    d = delta(); d["upsert"][0]["key"] = key("parent"); d["removed"] = [dict(key=key("child"),evidence=evidence("O11"))]
    return dict(base=b,update=d,source_snapshot=None)


def forms():
    s = snapshot(); c=context(["visible_text","accessibility_name","value"]); s["context"]=c;s["coverage"]=coverage(c);s["observations"]=[observation(c=c)];s["nodes"]=[]
    for name, text in [("textbox-1","A😀B"),("button-1","Apply"),("option-1","Option"),("draft-1","draft")]:
        s["nodes"].append(dict(key=key(name),surface=clone(c["surfaces"][0]),native_role=dict(availability="unsupported",reason="not_exposed"),children=[],extensions=[],source_declarations=[],properties=[prop("visible_text",value("text",text)),prop("accessibility_name",value("text","Confirm filters" if name=="button-1" else text)),prop("value",value("text",text))]))
    s["nodes"][-1]["extensions"]=[dict(namespace="fixture.app",name="applied_value",property=prop("value",status="unknown"))]
    s["focus"]=dict(keyboard=dict(status="known",target=key("textbox-1"),evidence=evidence()),accessibility=dict(status="known",target=key("button-1"),evidence=evidence()),active_descendant=dict(status="known",target=key("option-1"),evidence=evidence()),text_selection=dict(anchor=1,focus=3,units="utf16_code_units",evidence=evidence()),composition_state=prop("value",status="unknown"))
    return s


def translate(cid):
    """Yield suffix, actual document, structural acceptance (independently chosen)."""
    d = None; structural = True
    if cid == "GOLDEN01": d=document("golden_chain",chain())
    elif cid in ["G01-READONLY","G01-UNSUPPORTED","G01-GENERATION","G01-TARGET_GONE","G01-SAME_LABELS","IDENTITY-WRONG-TARGET","IDENTITY-REQUERY","IDENTITY-UNRESOLVED"]:
        s=snapshot(); a=action(); code="stale_target"; state="current"
        if cid in ["G01-READONLY","G01-UNSUPPORTED"]:
            a["resolution"]["available_intents"]=[];a["resolution"]["writable"]=dict(availability="known",value=value("flag",False));code="unsupported"
            if cid=="G01-UNSUPPORTED":s["nodes"][0]["properties"][-1]=prop("checked",status="unsupported")
        elif cid=="G01-GENERATION":
            s["context"]["surfaces"][0]["generation"]="w2";s["nodes"][0]["surface"]["generation"]="w2";a["context"]=clone(s["context"])
        elif cid=="G01-TARGET_GONE":state="gone"
        elif cid=="G01-SAME_LABELS":
            other=clone(s["nodes"][0]);other["key"]=key("check-2");s["nodes"].append(other);a["unique_match"]=False;code="ambiguous_target"
        elif cid=="IDENTITY-WRONG-TARGET":a["backend_ref"]["target"]["id"]="other-app"
        elif cid=="IDENTITY-REQUERY":
            s=snapshot(11);s["context"]["plugin"]["id"]="web";s["context"]["surfaces"][0]=dict(id="document",generation="d2");s["nodes"][0]["surface"]=clone(s["context"]["surfaces"][0]);s["nodes"][0]["key"]=key("e2","web.dom");s["nodes"][0]["native_role"]=dict(availability="known",value=value("text","input[type=checkbox]"));s["observations"][0]["source_namespace"]="web.dom"
            for p in s["nodes"][0]["properties"]:p["evidence"]["source_namespace"]="web.dom"
            a["context"]=clone(s["context"]);a["backend_ref"]["key"]=key("e1","web.dom");a["backend_ref"]["surface"]=dict(id="document",generation="d1")
            old=document("action_result",dict(snapshot=s,action=a,target_state="current",issue=issue("stale_target"),dispatched=False));yield "old_ref",old,True
            fresh=clone(a);fresh["backend_ref"].update(key=key("e2","web.dom"),snapshot_id="S11",observation_id="O11",surface=clone(s["context"]["surfaces"][0]));fresh["resolution"]["evidence"]=evidence("O11","web.dom")
            yield "new_ref",document("action",dict(snapshot=s,action=fresh)),True;return
        else:
            d=document("resolution_refusal",dict(requested=context(),candidates=[dict(process_id=42,window_id=7,title="Example",bounds=dict(x=0,y=0,width=100,height=100),coordinate_space=dict(id="screen",kind="screen",units="pt",origin="top_left"),process_continuity=dict(availability="unknown",reason="unknown_incarnation"),owner_binding=dict(availability="unknown",reason="no_mapping"))],issue=issue("target_unresolved"),dispatched=False));yield "",d,True;return
        d=document("action_result",dict(snapshot=s,action=a,target_state=state,issue=issue(code),dispatched=False))
    elif cid in ["G01-CANCEL_AFTER_DELIVERY","G01-VERIFY_MISSING","G01-VERIFY_TARGET_CHANGED"]:
        t=transition(False);t["steps"][0]["outcome"]="action_outcome_unknown";t["stopped_at"]="dispatch";after=snapshot(11)
        if cid=="G01-VERIFY_MISSING":after["nodes"][0]["properties"][-1]=prop("checked",oid="O11",status="unknown")
        if cid=="G01-VERIFY_TARGET_CHANGED":after["context"]["target"]["generation"]="g2"
        d=document("transition_context",dict(transition=t,before=snapshot(),after=after))
    elif cid in ["G01-LOST_BASE","G01-SKIPPED_REVISION"]:
        delta_value=delta()
        if cid=="G01-SKIPPED_REVISION":delta_value.update(base_revision=11,revision=12)
        d=document("delta_result",dict(base=None if cid=="G01-LOST_BASE" else snapshot(),update=delta_value,issue=issue("resync_required"),published=False))
    elif cid=="G01-REDACTED":
        s=snapshot(11);s["nodes"][0]["properties"][-1]=prop("checked",oid="O11",status="redacted");d=document("finding",dict(snapshot=s,expectation=expectation(),finding=finding("unknown")))
    elif cid.startswith("ENV-"):
        family=cid.split('-')[1].lower();bad=cid.endswith('INVALID')
        bases={"request":lambda:document("request",request()),"capability":lambda:document("session",session()),"session":lambda:document("session_context",dict(session=session(),request=request(),backend_ref=action()["backend_ref"])),"property":lambda:document("property",dict(property=prop("checked",value("flag",False)),observation=observation())),"observation":lambda:document("observation",observation()),"snapshot":lambda:document("snapshot",snapshot()),"delta":lambda:document("delta",delta_case()),"action":lambda:document("action",dict(snapshot=snapshot(),action=action())),"transition":lambda:document("transition_context",dict(transition=transition(),before=snapshot(),after=snapshot(11))),"expectation":lambda:document("expectation",geometry_finding("width",30,30,rects={"A":[10,20,30,10]})["expectation"]),"finding":lambda:document("finding",geometry_finding("width",30,30,rects={"A":[10,20,30,10]})),"error":lambda:document("error",issue())}
        d=bases[family]();x=d["artifact"]["data"]
        if bad:
            if family=="request":del x["limits"]["deadline_ms"];structural=False
            elif family=="capability":x["supported_versions"]=["0.2.0"];structural=False
            elif family=="session":x["request"]["context"]["session_id"]="a2"
            elif family=="property":del x["property"]["state"]["value"];structural=False
            elif family=="observation":del x["clock_domain"];structural=False
            elif family=="snapshot":x["nodes"].append(clone(x["nodes"][0]))
            elif family=="delta":x["update"]["upsert"][0]["properties"]=x["update"]["upsert"][0]["properties"][-1:]
            elif family=="action":del x["action"]["backend_ref"];structural=False
            elif family=="transition":x["after"]=None;x["transition"]["steps"][0]["verification_observation"]=None
            elif family=="expectation":del x["expected_from"];structural=False
            elif family=="finding":x["finding"]["measured"]=None
            else:del x["scope_id"];structural=False
    elif cid.startswith("PROP-"):
        p=prop("value",value("text",""));d=document("property",dict(property=p,observation=observation()))
        if cid in ["PROP-UNKNOWN","PROP-UNSUPPORTED","PROP-REDACTED"]:d["artifact"]["data"]["property"]=prop("value",status=cid.split('-')[1].lower())
        elif cid=="PROP-UNAVAILABLE-WITH-VALUE":
            for status in ["unknown","unsupported","redacted"]:
                variant=clone(d);p2=prop("value",status=status);p2["state"]["value"]=value("flag",False);variant["artifact"]["data"]["property"]=p2;yield status,variant,False
            return
        elif cid=="PROP-NOT-REQUESTED":
            s=snapshot(11);s["context"]["fields"]=["role"];s["coverage"]=coverage(s["context"]);s["observations"][0]["coverage"]=coverage(s["context"]);s["nodes"][0]["properties"]=[s["nodes"][0]["properties"][0],dict(selection="not_requested",field="checked")];d=document("snapshot",s)
        elif cid=="PROP-NOT-REQUESTED-AS-AVAILABILITY":p["state"]=dict(availability="not_requested");structural=False
        elif cid=="PROP-NULL":p["state"]["value"]=None;structural=False
    elif cid=="SNAP-MISSING-PROPERTY":
        s=snapshot();s["nodes"][0]["properties"].pop();d=document("snapshot",s)
    elif cid in ["DELTA-UNKNOWN-REPLACES-KNOWN","DELTA-STALE-CONTAMINATION"]:
        x=delta_case();x["source_snapshot"]["nodes"][0]["properties"][-1]=prop("checked",oid="O11",status="unknown")
        if cid=="DELTA-UNKNOWN-REPLACES-KNOWN":x["update"]["upsert"]=clone(x["source_snapshot"]["nodes"])
        else:x["update"]["upsert"][0]["properties"][-1]["state"]["value"]=value("flag",False)
        d=document("delta",x)
    elif cid.startswith("DELTA-CONTEXT-"):
        x=delta_case();dim=cid.removeprefix("DELTA-CONTEXT-");c=x["update"]["context"]
        if dim=="TARGET_GENERATION":c["target"]["generation"]="g2"
        elif dim=="SURFACE_GENERATION":c["surfaces"][0]["generation"]="w2"
        elif dim=="SCOPE":c["scope_id"]="other-form"
        elif dim=="PROJECTION":c["projection"]="design"
        elif dim=="FIELDS":c["fields"]=["role"]
        elif dim=="SCHEMA_VERSION":c["schema_version"]="0.2.0";structural=False
        elif dim=="PLUGIN_VERSION":c["plugin"]["version"]="0.2.0"
        elif dim=="BASE_REVISION":x["update"]["base_revision"]=9
        else:c["environment_revision"]="display-2"
        d=document("delta",x)
    elif cid in ["DELTA-ATOMIC-RELATIONS","DELTA-JUSTIFIED-REMOVAL"]:
        x=remove_child()
        if cid=="DELTA-ATOMIC-RELATIONS":x["update"]["upsert"][0]["children"]=[key("child")];x["update"]["relations"]=clone(x["base"]["relations"]);x["update"]["relations"][0]["evidence"]=evidence("O11");x["update"]["focus"]["keyboard"]=dict(status="known",target=key("child"),evidence=evidence("O11"))
        d=document("delta",x)
    elif cid=="COVERAGE-EMPTY":
        s=snapshot(11);s["nodes"]=[];s["context"].update(scope_id="empty-container",fields=["role"]);s["coverage"]=coverage(s["context"]);s["observations"][0]["coverage"]=coverage(s["context"]);d=document("snapshot",s)
    elif cid=="COVERAGE-NOT-DELETED":
        for variant in ["node_cap","extraction_failed","projection","scope","virtualized","base_evicted"]:
            x=remove_child();shape=True
            if variant=="projection":x["update"]["context"]["projection"]="design"
            elif variant=="scope":x["update"]["context"]["scope_id"]="other"
            elif variant=="base_evicted":x["base"]=None;shape=False
            else:x["update"]["observations"][0]["coverage"]["status"]="partial" if variant!="extraction_failed" else "unknown"
            yield variant,document("delta",x),shape
        return
    elif cid in ["OBS-INDEPENDENT-AXES","OBS-TTL","OBS-REVERSED-INTERVAL"]:
        o=observation("O11")
        if cid=="OBS-REVERSED-INTERVAL":o.update(start=110,end=100)
        else:
            o.update(answer_source="cache",freshness_basis="revalidated",consistency="unknown",consistency_reason="source_mismatch");o["coverage"]["status"]="partial"
            if cid=="OBS-TTL":o.update(last_verified=110,freshness_basis="ttl_only")
        d=document("observation",o)
    elif cid in ["OBS-CROSS-CLOCK","OBS-CROSS-CLOCK-FALSE-PROOF"]:
        a=observation();b=observation("O11");a.update(clock_domain="helper-monotonic",start=100,end=110,last_verified=110);b.update(clock_domain="parent-monotonic",start=105,end=115,last_verified=115)
        d=document("temporal_comparison",dict(observations=[a,b],transform=None,overlap=5 if cid.endswith('FALSE-PROOF') else None,atomic_claim=False))
    elif cid in ["GRAPH-MANY-TO-MANY","GRAPH-HEURISTIC-NOT-ACTION"]:
        s=snapshot(11);s["nodes"][0]["key"]=key("button");s["nodes"][0]["properties"][0]["state"]["value"]=value("role","button")
        probe_obs=observation("OP",s["context"],"probe");probe_obs["channel"]="opt_in_layout_probe";s["observations"].append(probe_obs)
        for name in (["container","icon","text"] if cid=="GRAPH-MANY-TO-MANY" else ["icon"]):
            n=clone(s["nodes"][0]);n["key"]=key(name,"probe");n["native_role"]=dict(availability="unsupported",reason="not_exposed")
            n["properties"]=[prop(f,oid="OP",namespace="probe",status="unknown") for f in FIELDS]
            s["nodes"].append(n);e=evidence("OP","probe");e["provenance"]="reported" if cid=="GRAPH-MANY-TO-MANY" else "estimated"
            s["relations"].append({"kind":"represents","from":key("button"),"to":key(name,"probe"),"evidence":e})
        if cid=="GRAPH-MANY-TO-MANY":s["components"]=[dict(logical_component_key="fixture-component",members=[n["key"] for n in s["nodes"]],declaration_source="fixture_app_mapping",provenance="reported")]
        d=document("snapshot",s)
    elif cid in ["FORMS-SEPARATE","FORMS-MISSING-OFFSET-UNIT"]:
        s=forms()
        if cid.endswith('UNIT'):del s["focus"]["text_selection"]["units"];structural=False
        d=document("snapshot",s)
    elif cid=="VERSION-UNKNOWN-CORE":d=document("request",request());d["artifact"]["data"]["made_up_core_field"]=True;structural=False
    elif cid=="RAW-ROLES":
        for namespace,role in [("macos.ax","AXCheckBox"),("web.dom","input[type=checkbox]")]:
            s=snapshot();s["nodes"][0]["key"]["namespace"]=namespace;s["nodes"][0]["native_role"]=dict(availability="known",value=value("text",role));s["observations"][0]["source_namespace"]=namespace
            for p in s["nodes"][0]["properties"]:p["evidence"]["source_namespace"]=namespace
            yield namespace,document("snapshot",s),True
        return
    elif cid=="ERROR-PRIVATE-PAYLOAD":d=document("error",issue("timeout"));d["artifact"]["data"]["raw_backend_message"]="CANARY_ORACLE_SECRET";structural=False
    elif cid.startswith('GEO-'):yield from geometry_cases(cid);return
    elif cid in ["REQUEST-MISSING-REQUIRED","SNAPSHOT-MISSING-CONTEXT"]:
        base=document("request",request()) if cid.startswith('REQUEST') else document("snapshot",snapshot())
        paths=[["artifact","data","request_id"],["schema_version"],["artifact","data","operation"],["artifact","data","context","session_id"],["artifact","data","context","scope_id"],["artifact","data","context","fields"],["artifact","data","limits","max_depth"],["artifact","data","limits","max_elements"],["artifact","data","limits","max_output_bytes"],["artifact","data","limits","deadline_ms"]] if cid.startswith('REQUEST') else [["artifact","data","revision"],["artifact","data","context","target","generation"],["artifact","data","context","surfaces",0,"generation"],["artifact","data","context","scope_id"],["artifact","data","context","projection"],["artifact","data","context","fields"],["artifact","data","context","schema_version"],["artifact","data","context","plugin","version"],["artifact","data","nodes",0,"properties",0,"evidence","observation_id"],["artifact","data","coverage","scope_id"]]
        for index,path in enumerate(paths):
            x=clone(base);at=x
            for segment in path[:-1]:at=at[segment]
            del at[path[-1]];yield str(index),x,False
        return
    if d is None:raise ValueError('Untranslated oracle case: '+cid)
    yield "",d,structural


def geometry_cases(cid):
    positive={"GEO-GAP":("gap",8,8,"pass","equal","length",0.01),"GEO-GAP-FAIL":("gap",8,10,"fail","equal","length",0.01),"GEO-CENTERS":("distance",33,33,"pass","equal","length",0.01),"GEO-ALIGNED":("aligned",0,0,"pass","equal","length",0.01),"GEO-INSIDE":("inside",10,10,"pass","at_least","length",0),"GEO-INTERSECTS":("intersects",25,0,"pass","greater_than","area",0),"GEO-OVERFLOW":("overflow",10,0,"fail","at_most","length",0),"GEO-EQUAL-SPACING":("equal_spacing",0,0,"pass","equal","length",0.01)}
    if cid in positive:
        rects=None
        if cid=="GEO-INSIDE":rects={"A":[10,20,30,10],"container":[0,0,120,60]}
        elif cid=="GEO-INTERSECTS":rects={"A":[10,20,30,10],"D":[35,25,20,10]}
        elif cid=="GEO-OVERFLOW":rects={"inner":[110,20,20,10],"container":[0,0,120,60]}
        x=geometry_finding(*positive[cid],rects=rects)
        if cid in ["GEO-GAP","GEO-GAP-FAIL"]:
            for anchor,fraction in zip(x["expectation"]["rule"]["anchors"],[1,0]):anchor["fraction"]=fraction
        if cid=="GEO-CENTERS":
            for anchor in x["expectation"]["rule"]["anchors"]:anchor["axis"]="xy"
        if cid in ["GEO-ALIGNED","GEO-EQUAL-SPACING"]:
            anchors=x["expectation"]["rule"]["anchors"];third=clone(anchors[-1]);third["element"]=key("C","fixture.authored_layout");anchors.append(third);x["expectation"]["targets"].append(third["element"])
            if cid=="GEO-ALIGNED":
                for anchor in anchors:anchor.update(axis="y",fraction=0)
        yield "",document("finding",x),True;return
    if cid=="GEO-SIZE-RATIO":
        for op,n,kind,tol in [("width",30,"length",0.01),("height",10,"length",0.01),("ratio",3,"ratio",0.001)]:
            x=geometry_finding(op,n,n,quantity_kind=kind,tolerance=tol,rects={"A":[10,20,30,10]});yield op,document("finding",x),True
        return
    if cid in ["GEO-BASELINE","GEO-BASELINE-UNKNOWN"]:
        x=geometry_finding("baseline",0.2,0,rects={"text-1":[0,0,0,0],"text-2":[0,0,0,0]},status="unknown" if cid.endswith('UNKNOWN') else "pass",tolerance=100 if cid.endswith('UNKNOWN') else 0.25)
        s=x["snapshot"];s["context"]["fields"]=["baseline"];s["coverage"]=coverage(s["context"]);s["observations"][0]["coverage"]=coverage(s["context"])
        for index,n in enumerate(s["nodes"]):n["properties"]=[prop("baseline",value("baseline",dict(coordinate=27 if index==0 else 27.2,space=space())),"OG1","fixture.authored_layout",status="unknown" if index==1 and cid.endswith('UNKNOWN') else "known")]
        yield "",document("finding",x),True;return
    if cid in ["GEO-MISSING-TRANSFORM","GEO-FRAME-KIND"]:
        x=geometry_finding("width",30,30,status="unknown",rects={"A":[10,20,30,10]}) if cid=="GEO-FRAME-KIND" else geometry_finding(status="unknown",tolerance=999,rects={"A":[10,20,30,10],"B":[48,20,20,10]});s=x["snapshot"];s["context"]["fields"]=["layout_bounds","accessibility_bounds"];s["coverage"]=coverage(s["context"]);s["observations"][0]["coverage"]=coverage(s["context"])
        for index,n in enumerate(s["nodes"]):
            g=geometry(frame="accessibility_bounds");n["properties"].append(prop("accessibility_bounds",value("geometry",g),"OG1","fixture.authored_layout"))
            if index==0:n["properties"][0]["state"]=dict(availability="unknown",reason="not_exposed")
        if cid=="GEO-MISSING-TRANSFORM":
            ga=s["nodes"][0]["properties"][1]["state"]["value"]["value"];ga["coordinate_space"]=dict(id="screen",kind="screen",units="pt",origin="top_left");ga["transform"]=dict(status="unknown",reason="missing_transform")
            gb=s["nodes"][1]["properties"][0]["state"]["value"]["value"];gb["coordinate_space"]=space("px","frame");gb["transform"]=dict(status="unknown",reason="missing_transform")
            anchors=x["expectation"]["rule"]["anchors"];anchors[0].update(frame_kind="accessibility_bounds",coordinate_space=clone(ga["coordinate_space"]));anchors[1]["coordinate_space"]=clone(gb["coordinate_space"]);x["expectation"]["rule"]["units"]="px";x["finding"]["reason"]="missing_transform"
        yield "",document("finding",x),True;return
    if cid in ["GEO-RULE-MISSING-TOLERANCE","GEO-RULE-EXECUTABLE"]:
        e=geometry_finding()["expectation"]
        if cid.endswith('TOLERANCE'):del e["rule"]["tolerance"]
        else:e["rule"]=dict(relation="javascript",code="return true")
        yield "",document("expectation",e),False;return
    x=geometry_case();shape=True
    if cid=="GEO-INVALID-UNIT":x["geometry"]["coordinate_space"]["units"]="em";shape=False
    elif cid=="GEO-INVALID-SHAPE":x["geometry"]["shape"]["value"]["width"]=-30
    elif cid=="GEO-NONFINITE":x["geometry"]["shape"]["value"]["x"]=float("inf");shape=False
    elif cid in ["GEO-TRANSFORM","GEO-TRANSFORM-CONTEXT"]:
        g=x["geometry"];g["coordinate_space"]=space("pt","window-content");g["transform"]=dict(status="known",transform=dict(from_=None))
        g["transform"]["transform"]={"from":space("pt","window-content"),"to":space("px","frame"),"affine":[2,0,0,2,6,8],"evidence":evidence("OG1","fixture.authored_layout"),"target":clone(x["context"]["target"]),"surface":clone(x["context"]["surfaces"][0]),"environment_revision":"display-1"}
        if cid.endswith('CONTEXT'):x["context"]["environment_revision"]="display-2"
    else:raise ValueError(cid)
    yield "",document("geometry",x),shape


def main():
    inputs=json.loads((ORACLES/'inputs.json').read_text())["cases"]
    expected={c["case_id"]:c for c in json.loads((ORACLES/'expected.json').read_text())["cases"]}
    manifest=[]
    for case in inputs:
        cid=case["case_id"]
        for suffix,doc,structural in translate(cid):
            filename=cid+('__'+suffix if suffix else '')+'.json'
            (OUT/filename).write_text(json.dumps(doc,ensure_ascii=False,indent=2).replace(': Infinity', ': 1e999')+'\n')
            manifest.append(dict(case_id=cid,variant=suffix,path=filename,structural_valid=structural,validator_exit=expected[cid]["validator_exit"]))
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print(f'{len(inputs)} independent cases -> {len(manifest)} wire fixtures')


if __name__ == '__main__':main()
