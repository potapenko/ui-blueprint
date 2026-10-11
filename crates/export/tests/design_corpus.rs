//! CE1 public export API adversarial vectors, not image or real UI evidence.
use serde_json::{Value as Json, json};
use uiblueprint_export::*;

fn limits() -> ExportLimits {
    ExportLimits {
        max_input_bytes: 2_000_000,
        max_output_bytes: 4_000_000,
        max_components: 256,
        max_views: 8,
        components_per_detail: 12,
    }
}
fn fixture(widths: &[f64]) -> (Json, Vec<Json>, Json, Json) {
    let doc: Json = serde_json::from_str(include_str!(
        "../../../fixtures/golden/GEO-SIZE-RATIO__width.json"
    ))
    .unwrap();
    let mut s = doc["artifact"]["data"]["snapshot"].clone();
    let expectation = doc["artifact"]["data"]["expectation"].clone();
    let base = s["nodes"][0].clone();
    let mut nodes = vec![];
    let mut qs = vec![];
    for (i, w) in widths.iter().enumerate() {
        let mut n = base.clone();
        n["key"]["key"] = json!(format!("private-node-{i}"));
        n["properties"][0]["state"]["value"]["value"]["shape"]["value"]["width"] = json!(w);
        let mut a = expectation["rule"]["anchors"].clone();
        a[0]["element"] = n["key"].clone();
        qs.push(json!({"id":format!("private-query-{i}"),"scope_id":s["context"]["scope_id"],"targets":[n["key"]],"operation":"width","anchors":a,"quantity_kind":"length","units":"css_px","applies_when":expectation["applies_when"]}));
        nodes.push(n);
    }
    s["nodes"] = json!(nodes);
    let e = json!({"snapshot_id":s["id"],"revision":s["revision"],"context":s["context"],"result_space":qs[0]["anchors"][0]["coordinate_space"],"transforms":[],"conditions":null});
    let mut b: Json =
        serde_json::from_str(include_str!("../../../fixtures/export/observed-brief.json")).unwrap();
    b["metadata"]["title"] = json!("CE1 synthetic geometry");
    b["views"][0]["title"] = json!("Synthetic controls");
    b["views"][0]["state"] = json!("Authored deterministic test state");
    b["views"][0]["scope"] = json!("Synthetic nodes only");
    b["views"][0]["environment"] = json!("Synthetic fixture; no runtime UI evidence");
    b["views"][0]["safe_source_reference"] = json!("CE1 authored vectors");
    b["views"][0]["source"]["snapshot"] = s;
    (b, qs, e, expectation)
}
fn package(b: Json) -> Package {
    compile(
        &serde_json::from_value::<DrawingBrief>(b).unwrap(),
        limits(),
    )
    .unwrap()
}
fn data(p: &Package, f: &str) -> Json {
    serde_json::from_slice(&p.files()[f]).unwrap()
}
fn insights(p: &Package) -> Json {
    data(p, "scene.json")["views"][0]["design_analysis"].clone()
}

fn relation(b: &mut Json, q: &[Json], e: &Json, operation: &str) {
    b["views"][0]["design_analysis"] = json!([{"kind":"relation","caption":"Selected measured relationship","operation":operation,"operands":q,"evaluation":e}]);
}
fn pattern_request(b: &mut Json, q: &[Json], e: &Json) {
    b["views"][0]["design_analysis"] = json!([{"kind":"pattern","caption":"Selected widths","basis":"Caller-selected same control state","operands":q,"evaluation":e}]);
}

#[test]
fn exact_machine_values_and_approximate_human_labels_remain_separate() {
    for (widths, operation, expected, residual, text) in [
        (vec![4.125, 4.25], "equal", 4.125, Some(0.125), "≈4"),
        (
            vec![179., 179., 8., 366.],
            "balance",
            366.,
            Some(0.),
            "179 + 179 + 8 = 366",
        ),
        (
            vec![0.125, 0.25, 0.375],
            "balance",
            0.375,
            Some(0.),
            "rounded labels are not an exact sum",
        ),
        (vec![1., 8.], "ratio", 0.125, None, "0.125 times"),
        (
            vec![1., 4096.],
            "ratio",
            0.000244140625,
            None,
            "<0.001 times",
        ),
    ] {
        let (mut b, q, e, _) = fixture(&widths);
        relation(&mut b, &q, &e, operation);
        let p = package(b);
        let i = insights(&p);
        assert_eq!(i[0]["facts"]["result"]["value"], json!(expected));
        assert_eq!(i[0]["facts"]["result"]["residual"], json!(residual));
        for (j, width) in widths.iter().enumerate() {
            assert_eq!(
                i[0]["facts"]["measurements"][j]["measurement"]["value"]["value"]["amount"],
                json!(width)
            );
        }
        for file in ["prompt.txt", "drawing-brief.md"] {
            assert!(
                String::from_utf8_lossy(&p.files()[file]).contains(text),
                "{operation}: {text}"
            );
        }
    }
}

#[test]
fn check_comparison_tolerance_source_and_exception_are_preserved() {
    for (comparison, expected, tolerance, status) in [
        ("equal", 4., 0., "rule_violation"),
        ("equal", 4., 1., "pass"),
        ("at_least", 4., 0., "pass"),
        ("at_least", 6., 0., "rule_violation"),
        ("at_most", 6., 0., "pass"),
        ("at_most", 4., 0., "rule_violation"),
        ("greater_than", 5., 0., "rule_violation"),
        ("greater_than", 4., 0., "pass"),
    ] {
        let (mut b, q, e, mut x) = fixture(&[5.]);
        x["targets"] = q[0]["targets"].clone();
        x["rule"]["anchors"] = q[0]["anchors"].clone();
        x["rule"]["comparison"] = json!(comparison);
        x["rule"]["expected"] = json!(expected);
        x["rule"]["tolerance"] = json!(tolerance);
        x["expected_from"] = json!("private-decision-reference");
        let exception =
            json!({"source":"Decision A","reason":"Caller declares this variant intentional"});
        b["views"][0]["design_analysis"] = json!([{"kind":"check","caption":"Width contract","expectation":x,"evaluation":e,"exception":exception}]);
        let p = package(b);
        let i = insights(&p);
        assert_eq!(i[0]["status"], status);
        let result = &i[0]["facts"]["result"];
        assert_eq!(result["expectation"]["rule"]["comparison"], comparison);
        assert_eq!(result["expectation"]["rule"]["expected"], expected);
        assert_eq!(result["expectation"]["rule"]["tolerance"], tolerance);
        assert!(
            result["expectation"]["expected_from"]
                .as_str()
                .unwrap()
                .starts_with("REF")
        );
        assert_eq!(result["exception"], exception);
        assert!(
            i[0]["summary"]
                .as_str()
                .unwrap()
                .contains("Check result retained")
        );
    }
}

#[test]
fn aliases_join_queries_measurements_context_and_scene_without_private_text() {
    let (mut b, mut q, e, _) = fixture(&[4., 4., 5.]);
    for (query, fraction) in q.iter_mut().zip([0., 0.25, 1.]) {
        query["anchors"][0]["fraction"] = json!(fraction);
    }
    let s = &mut b["views"][0]["source"]["snapshot"];
    for node in s["nodes"].as_array_mut().unwrap() {
        node["properties"][0]["evidence"]["method"] = json!("private-method-canary");
    }
    pattern_request(&mut b, &q, &e);
    let p = package(b);
    let scene = data(&p, "scene.json");
    let v = &scene["views"][0];
    let facts = &v["design_analysis"][0]["facts"];
    assert_eq!(facts["evaluation"]["snapshot_id"], v["snapshot_ref"]);
    assert_eq!(facts["evaluation"]["revision"], v["snapshot_revision"]);
    assert_eq!(
        facts["evaluation"]["context"]["scope_id"],
        v["coverage"]["scope_id"]
    );
    assert_eq!(
        facts["evaluation"]["context"]["surfaces"][0],
        v["components"][0]["surface"]
    );
    for j in 0..3 {
        assert_eq!(
            facts["queries"][j]["anchors"][0]["fraction"],
            q[j]["anchors"][0]["fraction"]
        );
        assert_eq!(facts["queries"][j]["operation"], "width");
        assert_eq!(
            facts["queries"][j]["anchors"][0]["element"],
            facts["queries"][j]["targets"][0]
        );
        assert_eq!(
            facts["measurements"][j]["measurement"]["space"],
            facts["evaluation"]["result_space"]
        );
        assert_eq!(
            facts["measurements"][j]["measurement"]["evidence"][0]["observation_id"],
            v["observations"][0]["id"]
        );
    }
    for bytes in p.files().values() {
        let text = String::from_utf8_lossy(bytes);
        for private in [
            "private-method-canary",
            "private-node-",
            "private-query-",
            "native-app",
            "fixture.authored_layout",
        ] {
            assert!(!text.contains(private), "leaked {private}");
        }
    }
}

#[test]
fn no_majority_uncertainty_and_state_unknown_never_become_violations() {
    for (widths, status) in [
        (vec![4.125, 4.25, 4.375], "no_pattern"),
        (vec![4., 4., 5., 5.], "no_pattern"),
        (vec![4., 4., 4.], "repeated_value"),
    ] {
        let (mut b, q, e, _) = fixture(&widths);
        pattern_request(&mut b, &q, &e);
        assert_eq!(insights(&package(b))[0]["status"], status);
    }
    let (mut b, mut q, e, _) = fixture(&[4., 4., 5.]);
    for query in &mut q {
        query["applies_when"]["platform"] = json!("selected-platform");
    }
    pattern_request(&mut b, &q, &e);
    let i = insights(&package(b));
    assert_eq!(i[0]["status"], "unknown");
    assert_eq!(i[0]["facts"]["result"]["candidate"], Json::Null);
    assert_eq!(i[0]["facts"]["measurements"].as_array().unwrap().len(), 3);
}

#[test]
fn exact_input_output_and_request_caps_refuse_one_byte_or_request_over() {
    let (mut b, q, e, _) = fixture(&[4., 4., 5.]);
    pattern_request(&mut b, &q, &e);
    let brief: DrawingBrief = serde_json::from_value(b.clone()).unwrap();
    let mut encoded = serde_json::to_vec_pretty(&brief).unwrap();
    encoded.push(b'\n');
    let mut l = limits();
    l.max_input_bytes = encoded.len();
    assert!(DrawingBrief::from_json(&encoded, l).is_ok());
    l.max_input_bytes -= 1;
    assert_eq!(
        DrawingBrief::from_json(&encoded, l).unwrap_err(),
        ExportError::InputLimit
    );
    let p = compile(&brief, limits()).unwrap();
    let bytes = p.files().values().map(Vec::len).sum::<usize>();
    l = limits();
    l.max_output_bytes = bytes;
    assert!(compile(&brief, l).is_ok());
    l.max_output_bytes -= 1;
    assert_eq!(compile(&brief, l).unwrap_err(), ExportError::OutputLimit);
    b["views"][0]["design_analysis"] = json!(vec![b["views"][0]["design_analysis"][0].clone(); 4]);
    l = limits();
    l.max_components = 3;
    assert_eq!(
        compile(&serde_json::from_value(b).unwrap(), l).unwrap_err(),
        ExportError::InputLimit
    );
}

#[test]
fn old_empty_requests_are_byte_identical_and_injected_results_reject() {
    let (mut b, q, e, _) = fixture(&[4., 4., 5.]);
    let old = package(b.clone());
    b["views"][0]["design_analysis"] = json!([]);
    assert_eq!(old.files(), package(b.clone()).files());
    assert_eq!(data(&old, "manifest.json")["package_version"], "0.1.0");
    pattern_request(&mut b, &q, &e);
    b["views"][0]["design_analysis"][0]["result"] = json!({"candidate":4});
    assert!(serde_json::from_value::<DrawingBrief>(b).is_err());
}

#[test]
fn every_public_analysis_annotation_uses_existing_privacy_gate() {
    let (mut b, q, e, _) = fixture(&[4., 4., 5.]);
    pattern_request(&mut b, &q, &e);
    for field in ["caption", "basis"] {
        for text in [
            "/private/source",
            "https://private.invalid",
            "{{hidden}}",
            "bad\u{0001}text",
        ] {
            let mut bad = b.clone();
            bad["views"][0]["design_analysis"][0][field] = json!(text);
            assert_eq!(
                compile(&serde_json::from_value(bad).unwrap(), limits()).unwrap_err(),
                ExportError::PrivateContent
            );
        }
    }
}

#[test]
fn duplicate_width_does_not_ship_a_false_recommendation() {
    let (mut b, mut q, e, _) = fixture(&[4., 5.]);
    let mut duplicate = q[0].clone();
    duplicate["id"] = json!("same-width-another-name");
    duplicate["anchors"][0]["fraction"] = json!(0.);
    q.insert(1, duplicate);
    pattern_request(&mut b, &q, &e);
    let result = compile(&serde_json::from_value(b).unwrap(), limits());
    assert!(
        result.is_err(),
        "two distinct widths must reject; actual summary {:?}",
        result.map(|p| insights(&p)[0]["summary"].clone())
    );
}

#[test]
fn signed_zero_ratio_is_printed_as_zero() {
    let (mut b, q, e, _) = fixture(&[10., 10., 10.]);
    for (node, x) in b["views"][0]["source"]["snapshot"]["nodes"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .zip([0., 10., 16.])
    {
        node["properties"][0]["state"]["value"]["value"]["shape"]["value"]["x"] = json!(x);
    }
    let gaps = q
        .windows(2)
        .map(|pair| {
            let mut g = pair[0].clone();
            g["operation"] = json!("gap");
            g["targets"] = json!([pair[0]["targets"][0], pair[1]["targets"][0]]);
            g["anchors"] = json!([pair[0]["anchors"][0], pair[1]["anchors"][0]]);
            g["anchors"][0]["fraction"] = json!(1.);
            g["anchors"][1]["fraction"] = json!(0.);
            g
        })
        .collect::<Vec<_>>();
    relation(&mut b, &gaps, &e, "ratio");
    let p = package(b);
    let i = insights(&p);
    assert_eq!(
        i[0]["facts"]["result"]["value"].as_f64().unwrap().to_bits(),
        (-0.0_f64).to_bits()
    );
    for file in ["prompt.txt", "drawing-brief.md"] {
        let text = String::from_utf8_lossy(&p.files()[file]);
        assert!(text.contains(": 0 times as large."));
        assert!(!text.contains("-0 times"));
    }
    assert!(
        !i[0]["summary"].as_str().unwrap().contains("-0 times"),
        "display signed zero must be 0; actual {}",
        i[0]["summary"]
    );
}

#[test]
fn view_attribution_and_unknown_source_results_stay_separate() {
    let (mut b, q, e, _) = fixture(&[4., 4., 5.]);
    pattern_request(&mut b, &q, &e);
    let mut second = b["views"][0].clone();
    second["id"] = json!("other-view");
    second["source"]["snapshot"]["nodes"][0]["properties"][0]["state"] =
        json!({"availability":"unknown","reason":"private-unknown-reason"});
    b["views"].as_array_mut().unwrap().push(second);
    let p = package(b);
    let scene = data(&p, "scene.json");
    assert_eq!(
        scene["views"][0]["design_analysis"][0]["status"],
        "possible_inconsistency"
    );
    assert_eq!(scene["views"][1]["design_analysis"][0]["status"], "unknown");
    assert_eq!(
        scene["views"][1]["design_analysis"][0]["facts"]["measurements"][0]["reason"],
        "unknown_property"
    );
    for bytes in p.files().values() {
        assert!(!String::from_utf8_lossy(bytes).contains("private-unknown-reason"));
    }
}

#[test]
fn unreviewed_visible_text_cannot_leak_through_operand_labels() {
    let (mut b, q, e, _) = fixture(&[4., 4., 5.]);
    let s = &mut b["views"][0]["source"]["snapshot"];
    for fields in ["context", "coverage"] {
        s[fields]["fields"]
            .as_array_mut()
            .unwrap()
            .push(json!("visible_text"));
    }
    s["observations"][0]["coverage"]["fields"]
        .as_array_mut()
        .unwrap()
        .push(json!("visible_text"));
    for n in s["nodes"].as_array_mut().unwrap() {
        let source = n["properties"][0]["evidence"].clone();
        n["properties"].as_array_mut().unwrap().push(json!({"selection":"requested","field":"visible_text","sensitivity":"public","evidence":source,"state":{"availability":"known","value":{"type":"text","value":"UNREVIEWED-CANARY"}}}));
    }
    let mut evaluation = e;
    evaluation["context"] = s["context"].clone();
    b["views"][0]["source"]["public_text_fields"] = json!([]);
    pattern_request(&mut b, &q, &evaluation);
    let p = package(b);
    for bytes in p.files().values() {
        assert!(!String::from_utf8_lossy(bytes).contains("UNREVIEWED-CANARY"));
    }
}
