use serde_json::{Value, json};
use uiblueprint_export::*;

fn limits() -> ExportLimits {
    ExportLimits {
        max_input_bytes: 2_000_000,
        max_output_bytes: 4_000_000,
        max_components: 128,
        max_views: 2,
        components_per_detail: 100,
    }
}
fn example() -> Value {
    let mut b: Value =
        serde_json::from_str(include_str!("../../../fixtures/export/observed-brief.json")).unwrap();
    b["views"][0]["source"]["public_text_fields"] = json!(["accessibility_name"]);
    let s = &mut b["views"][0]["source"]["snapshot"];
    s["relations"] = json!([]);
    s["nodes"].as_array_mut().unwrap().truncate(4);
    let keys: Vec<_> = s["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n["key"].clone())
        .collect();
    let template = s["nodes"][0]["properties"].clone();
    for (i, n) in s["nodes"].as_array_mut().unwrap().iter_mut().enumerate() {
        n["children"] = if i == 0 { json!(keys[1..]) } else { json!([]) };
        n["properties"] = template.clone();
        n["properties"].as_array_mut().unwrap().truncate(3);
        n["extensions"] = json!([]);
        n["native_role"]["value"]["value"] = json!(if i == 0 { "AXGroup" } else { "AXButton" });
        n["properties"][0]["state"] = json!({"availability":"known","value":{"type":"role","value":if i==0 {"group"}else{"button"}}});
        n["properties"][1]["state"] = json!({"availability":"known","value":{"type":"text","value":(["Panel","Orion","Lyra","Vega"][i])}});
        n["properties"][2]["field"] = json!("accessibility_bounds");
        let g = &mut n["properties"][2]["state"]["value"]["value"];
        g["frame_kind"] = json!("accessibility_bounds");
        g["shape"]["value"] = if i == 0 {
            json!({"x":0,"y":0,"width":140,"height":40})
        } else {
            json!({"x":10+(i-1)*45,"y":10,"width":30,"height":20})
        };
    }
    let fields = json!(["role", "accessibility_name", "accessibility_bounds"]);
    s["context"]["fields"] = fields.clone();
    s["coverage"]["fields"] = fields.clone();
    for o in s["observations"].as_array_mut().unwrap() {
        o["coverage"]["fields"] = fields.clone();
    }
    b
}
fn package(b: &Value) -> Package {
    compile(
        &DrawingBrief::from_json(&serde_json::to_vec(b).unwrap(), limits()).unwrap(),
        limits(),
    )
    .unwrap()
}
fn prompt(p: &Package) -> &str {
    std::str::from_utf8(&p.files()["prompt.txt"]).unwrap()
}
fn dimensions(p: &Package) -> Vec<Value> {
    serde_json::from_slice::<Value>(&p.files()["dimensions.json"]).unwrap()[0]["dimensions"]
        .as_array()
        .unwrap()
        .clone()
}
#[test]
fn source_names_sizes_rhythm_and_alignment_generate_without_a_reference() {
    let b = example();
    let original = b.clone();
    let p = package(&b);
    let text = prompt(&p);
    assert!(text.starts_with("ONE ENGINEERING DRAWING"));
    for name in ["Orion", "Lyra", "Vega"] {
        assert!(text.contains(name));
    }
    assert!(text.contains("size 30 × 20 CSS px"));
    assert!(text.contains("REPETITION: 2 gaps equal exactly in source, display 15 CSS px"));
    assert!(text.contains("draw a shared center axis"));
    assert!(!text.contains("Reference UI"));
    assert!(!text.contains("PlayPhrase.me A04"));
    assert!(
        dimensions(&p)
            .iter()
            .any(|d| d["label"] == "Measured center alignment" && d["value"].as_f64() == Some(0.0))
    );
    assert_eq!(b, original);
    let scene: Value = serde_json::from_slice(&p.files()["scene.json"]).unwrap();
    assert_eq!(scene["views"][0]["components"].as_array().unwrap().len(), 4);
}
#[test]
fn changing_source_changes_prompt_and_does_not_keep_a_false_equal_gap() {
    let mut b = example();
    b["views"][0]["source"]["snapshot"]["nodes"][1]["properties"][2]["state"]["value"]["value"]["shape"]
        ["value"]["width"] = json!(32);
    b["views"][0]["source"]["snapshot"]["nodes"][1]["properties"][1]["state"]["value"]["value"] =
        json!("Renamed control");
    let p = package(&b);
    let text = prompt(&p);
    assert!(text.contains("Renamed control"));
    assert!(!text.contains("Orion"));
    assert!(text.contains("size 32 × 20 CSS px"));
    assert!(text.contains("13 CSS px; dimension these boundaries"));
    assert!(!text.contains("REPETITION: 2 gaps equal exactly in source, display 15"));
}
#[test]
fn unrelated_spaces_do_not_gain_spacing_and_output_budget_still_refuses() {
    let mut b = example();
    b["views"][0]["source"]["snapshot"]["nodes"][3]["properties"][2]["state"]["value"]["value"]["coordinate_space"]
        ["id"] = json!("separate-space");
    let p = package(&b);
    assert!(!dimensions(&p).iter().any(|d| {
        d["label"] == "Measured edge relation"
            && d["anchors"]
                .as_array()
                .unwrap()
                .iter()
                .any(|a| a["component"] == "N003")
    }));
    let brief = DrawingBrief::from_json(&serde_json::to_vec(&b).unwrap(), limits()).unwrap();
    let mut cap = limits();
    cap.max_output_bytes = p.files().values().map(Vec::len).sum();
    assert!(compile(&brief, cap).is_ok());
    cap.max_output_bytes -= 1;
    assert_eq!(compile(&brief, cap).unwrap_err(), ExportError::OutputLimit);
}

#[test]
fn public_text_policy_and_unstable_measurements_survive_geometry_composition() {
    let mut b = example();
    b["views"][0]["source"]["public_text_fields"] = json!([]);
    let p = package(&b);
    for private in ["Orion", "Lyra", "Vega"] {
        assert!(
            p.files()
                .values()
                .all(|bytes| !String::from_utf8_lossy(bytes).contains(private))
        );
    }
    let mut unstable = example();
    for observation in unstable["views"][0]["source"]["snapshot"]["observations"]
        .as_array_mut()
        .unwrap()
    {
        observation["consistency"] = json!("unstable");
        observation["consistency_reason"] = json!("changed_during_observation");
    }
    let p = package(&unstable);
    assert!(!prompt(&p).contains("REPEATED SIZE:"));
    assert!(!prompt(&p).contains("REPETITION:"));
    assert!(prompt(&p).contains("size unknown × unknown"));
}
