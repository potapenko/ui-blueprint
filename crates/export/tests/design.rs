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

#[test]
fn relation_and_pattern_are_delivered_with_exact_safe_evidence() {
    let (mut b, q, e, _) = fixture(&[4., 4., 4., 5.]);
    b["views"][0]["design_analysis"] = json!([
        {"kind":"pattern","caption":"Selected control widths","basis":"Same caller-selected component variant and state","operands":q,"evaluation":e},
        {"kind":"relation","caption":"First two widths","operation":"equal","operands":[q[0],q[1]],"evaluation":e}
    ]);
    let p = package(b);
    assert_eq!(data(&p, "manifest.json")["package_version"], "0.3.0");
    let i = insights(&p);
    assert_eq!(i[0]["status"], "possible_inconsistency");
    assert_eq!(i[0]["facts"]["result"]["candidate"], 4.);
    assert_eq!(i[0]["facts"]["result"]["exceptions"], json!([[3, 1.]]));
    assert_eq!(i[1]["facts"]["result"]["residual"], 0.);
    for file in ["prompt.txt", "drawing-brief.md"] {
        let text = String::from_utf8_lossy(&p.files()[file]);
        assert!(text.contains("Check exception"));
        assert!(text.contains("No automatic correction"));
    }
    for bytes in p.files().values() {
        let text = String::from_utf8_lossy(bytes);
        assert!(!text.contains("private-node-"));
        assert!(!text.contains("private-query-"));
        assert!(!text.contains("fixture_literal"));
    }
}

#[test]
fn sourced_checks_preserve_inequality_tolerance_and_declared_exception() {
    let (mut b, q, e, mut expectation) = fixture(&[5.]);
    expectation["targets"] = q[0]["targets"].clone();
    expectation["rule"]["anchors"] = q[0]["anchors"].clone();
    expectation["rule"]["expected"] = json!(4);
    expectation["rule"]["tolerance"] = json!(0);
    let req = json!({"kind":"check","caption":"Control width rule","expectation":expectation,"evaluation":e,"exception":{"source":"Decision A","reason":"This named variant is intentionally wider"}});
    b["views"][0]["design_analysis"] = json!([req]);
    let p = package(b.clone());
    assert_eq!(insights(&p)[0]["status"], "rule_violation");
    assert!(
        insights(&p)[0]["summary"]
            .as_str()
            .unwrap()
            .contains("Check result retained")
    );
    b["views"][0]["design_analysis"][0]["expectation"]["rule"]["comparison"] = json!("at_least");
    let p = package(b.clone());
    assert_eq!(insights(&p)[0]["status"], "pass");
    assert!(
        insights(&p)[0]["summary"]
            .as_str()
            .unwrap()
            .contains("at least 4")
    );
    b["views"][0]["design_analysis"][0]["expectation"]["rule"]["comparison"] = json!("equal");
    b["views"][0]["design_analysis"][0]["expectation"]["rule"]["tolerance"] = json!(1);
    assert_eq!(insights(&package(b.clone()))[0]["status"], "pass");
    b["views"][0]["source"]["snapshot"]["nodes"][0]["properties"][0]["state"] =
        json!({"availability":"unknown","reason":"missing"});
    assert_eq!(insights(&package(b))[0]["status"], "unknown");
}

#[test]
fn old_packages_and_exact_source_values_stay_unchanged() {
    let (b, _, _, _) = fixture(&[4.125]);
    let p = package(b.clone());
    assert_eq!(data(&p, "manifest.json")["package_version"], "0.1.0");
    assert!(insights(&p).is_null());
    assert!(
        !String::from_utf8_lossy(&p.files()["prompt.txt"])
            .contains("RELATIONSHIPS AND RECOMMENDATIONS")
    );
    let mut explicit = b;
    explicit["views"][0]["design_analysis"] = json!([]);
    let same = package(explicit);
    assert_eq!(p.files(), same.files());
}

#[test]
fn unsafe_annotation_bad_binding_and_operand_limit_refuse() {
    let (mut b, q, e, _) = fixture(&[4., 4., 4.]);
    b["views"][0]["design_analysis"] = json!([{"kind":"pattern","caption":"Controls","basis":"Known family","operands":q,"evaluation":e}]);
    let mut bad = b.clone();
    bad["views"][0]["design_analysis"][0]["basis"] = json!("/private/source");
    assert_eq!(
        compile(
            &serde_json::from_value::<DrawingBrief>(bad).unwrap(),
            limits()
        )
        .unwrap_err(),
        ExportError::PrivateContent
    );
    let mut bad = b.clone();
    bad["views"][0]["design_analysis"][0]["evaluation"]["revision"] = json!(999);
    assert_eq!(
        compile(
            &serde_json::from_value::<DrawingBrief>(bad).unwrap(),
            limits()
        )
        .unwrap_err(),
        ExportError::InvalidGeometry
    );
    let mut small = limits();
    small.max_components = 2;
    assert_eq!(
        compile(&serde_json::from_value::<DrawingBrief>(b).unwrap(), small).unwrap_err(),
        ExportError::InputLimit
    );
}
