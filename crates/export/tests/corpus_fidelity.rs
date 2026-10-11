//! Independent literals for CP2's two source-to-export fidelity regressions.
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
fn brief() -> Json {
    let mut b: Json =
        serde_json::from_str(include_str!("../../../fixtures/export/observed-brief.json")).unwrap();
    b["views"][0]["source"]["public_text_fields"] = json!(["visible_text"]);
    let s = &mut b["views"][0]["source"]["snapshot"];
    for owner in ["context", "coverage"] {
        for field in s[owner]["fields"].as_array_mut().unwrap() {
            if field == "accessibility_name" {
                *field = json!("visible_text");
            }
        }
    }
    for o in s["observations"].as_array_mut().unwrap() {
        for field in o["coverage"]["fields"].as_array_mut().unwrap() {
            if field == "accessibility_name" {
                *field = json!("visible_text");
            }
        }
    }
    for n in s["nodes"].as_array_mut().unwrap() {
        n["properties"][1]["field"] = json!("visible_text");
    }
    b
}
fn set_text(b: &mut Json, text: &str) {
    b["views"][0]["source"]["snapshot"]["nodes"][0]["properties"][1]["state"] =
        json!({"availability":"known", "value":{"type":"text","value":text}});
}
fn package(b: &Json) -> Package {
    compile(
        &serde_json::from_value::<DrawingBrief>(b.clone()).unwrap(),
        limits(),
    )
    .unwrap()
}
fn all(p: &Package) -> String {
    p.files()
        .values()
        .map(|v| std::str::from_utf8(v).unwrap())
        .collect()
}

#[test]
fn reviewed_public_formulas_retain_exact_text_and_original_source() {
    for text in [
        "V/Vs/Ves",
        "Ved/V2",
        "Do/Does + subject + V",
        "Do/Does + not + V",
        "N/Ns",
        "Be/Been",
        "X1/X2",
    ] {
        let mut b = brief();
        set_text(&mut b, text);
        let original = b.clone();
        let p = package(&b);
        for f in ["scene.json", "drawing-brief.md", "prompt.txt"] {
            assert!(
                std::str::from_utf8(&p.files()[f]).unwrap().contains(text),
                "{f}: {text}"
            );
        }
        assert_eq!(b, original);
    }
}

#[test]
fn formula_exception_does_not_open_paths_urls_templates_or_unreviewed_channels() {
    for text in [
        "/private/canary",
        "../private/canary",
        "./local/file",
        "~/Documents/file",
        "private/canary",
        "Folder/File",
        "report/file.txt",
        "C:/Private/Token",
        "https://example.invalid/secret",
        "example.invalid/secret",
        "V/Vs//Ves",
        "V/Vs/",
        "V\\Vs",
        "{{V/Vs}}",
        "V/Vs\u{0001}",
    ] {
        let mut b = brief();
        set_text(&mut b, text);
        let p = package(&b);
        let scene: Json = serde_json::from_slice(&p.files()["scene.json"]).unwrap();
        assert_eq!(
            scene["views"][0]["components"][0]["properties"][1]["state"]["availability"],
            "redacted",
            "{text}"
        );
        assert!(!all(&p).contains(text), "leaked {text}");
    }
    for channel in [
        "unreviewed",
        "sensitive",
        "value",
        "accessible_name",
        "metadata",
    ] {
        let mut b = brief();
        set_text(&mut b, "V/Vs/Ves");
        match channel {
            "unreviewed" => b["views"][0]["source"]["public_text_fields"] = json!([]),
            "sensitive" => {
                b["views"][0]["source"]["snapshot"]["nodes"][0]["properties"][1]["sensitivity"] =
                    json!("sensitive");
                assert_eq!(
                    compile(&serde_json::from_value(b).unwrap(), limits()).unwrap_err(),
                    ExportError::InvalidSource
                );
                continue;
            }
            "metadata" => {
                b["metadata"]["title"] = json!("V/Vs/Ves");
                assert_eq!(
                    compile(&serde_json::from_value(b).unwrap(), limits()).unwrap_err(),
                    ExportError::PrivateContent
                );
                continue;
            }
            _ => {
                let field = if channel == "value" {
                    "value"
                } else {
                    "accessibility_name"
                };
                let s = &mut b["views"][0]["source"]["snapshot"];
                for owner in ["context", "coverage"] {
                    for f in s[owner]["fields"].as_array_mut().unwrap() {
                        if f == "visible_text" {
                            *f = json!(field);
                        }
                    }
                }
                for o in s["observations"].as_array_mut().unwrap() {
                    for f in o["coverage"]["fields"].as_array_mut().unwrap() {
                        if f == "visible_text" {
                            *f = json!(field);
                        }
                    }
                }
                for n in s["nodes"].as_array_mut().unwrap() {
                    n["properties"][1]["field"] = json!(field);
                }
                b["views"][0]["source"]["public_text_fields"] = if channel == "value" {
                    json!([])
                } else {
                    json!([field])
                };
            }
        }
        assert!(!all(&package(&b)).contains("V/Vs/Ves"), "{channel}");
    }
}

#[test]
fn detail_caption_uses_own_label_owner_or_relation_never_an_unrelated_neighbor() {
    let mut b = brief();
    let s = &mut b["views"][0]["source"]["snapshot"];
    for n in s["nodes"].as_array_mut().unwrap() {
        n["children"] = json!([]);
    }
    for (i, native, role, label) in [
        (0, "LABEL", "text", "Offensive :"),
        (1, "SELECT", "combobox", ""),
        (2, "LABEL", "text", "Only slang"),
        (3, "DIV", "group", ""),
    ] {
        s["nodes"][i]["native_role"] =
            json!({"availability":"known","value":{"type":"text","value":native}});
        s["nodes"][i]["properties"][0]["state"] =
            json!({"availability":"known","value":{"type":"role","value":role}});
        s["nodes"][i]["properties"][1]["state"] =
            json!({"availability":"known","value":{"type":"text","value":label}});
    }
    s["nodes"][0]["children"] = json!([s["nodes"][1]["key"]]);
    s["nodes"][3]["children"] = json!([s["nodes"][2]["key"], s["nodes"][0]["key"]]);
    let shape =
        json!({"shape":"rect","value":{"x":100.0,"y":100.0,"width":124.234375,"height":13.5}});
    s["nodes"][1]["properties"][2]["state"] = s["nodes"][0]["properties"][2]["state"].clone();
    s["nodes"][1]["properties"][2]["state"]["value"]["value"]["shape"] = shape;
    let prompt = |b: &Json| String::from_utf8(package(b).files()["prompt.txt"].clone()).unwrap();
    let p = prompt(&b);
    assert!(p.contains("LOCAL DETAIL \"Offensive : — dimensions\""));
    assert!(!p.contains("LOCAL DETAIL \"Only slang — dimensions\""));
    b["views"][0]["source"]["snapshot"]["nodes"][0]["native_role"]["value"]["value"] = json!("DIV");
    let p = prompt(&b);
    assert!(!p.contains("LOCAL DETAIL \"Offensive : — dimensions\""));
    assert!(p.contains("LOCAL DETAIL \"Control dimensions and spacing\""));
    let s = &mut b["views"][0]["source"]["snapshot"];
    let relation = json!({"kind":"labelled_by", "from":s["nodes"][1]["key"], "to":s["nodes"][0]["key"], "evidence":s["nodes"][0]["properties"][0]["evidence"]});
    s["relations"].as_array_mut().unwrap().push(relation);
    assert!(prompt(&b).contains("LOCAL DETAIL \"Offensive : — dimensions\""));
    let s = &mut b["views"][0]["source"]["snapshot"];
    s["nodes"][1]["properties"][1]["state"]["value"]["value"] = json!("Own sourced name");
    for owner in ["context", "coverage"] {
        for f in s[owner]["fields"].as_array_mut().unwrap() {
            if f == "visible_text" {
                *f = json!("accessibility_name");
            }
        }
    }
    for o in s["observations"].as_array_mut().unwrap() {
        for f in o["coverage"]["fields"].as_array_mut().unwrap() {
            if f == "visible_text" {
                *f = json!("accessibility_name");
            }
        }
    }
    for n in s["nodes"].as_array_mut().unwrap() {
        n["properties"][1]["field"] = json!("accessibility_name");
    }
    b["views"][0]["source"]["public_text_fields"] = json!(["accessibility_name"]);
    assert!(prompt(&b).contains("LOCAL DETAIL \"Own sourced name — dimensions\""));
}

#[test]
fn visible_control_payload_is_not_promoted_to_a_semantic_icon_caption() {
    for text in ["grade", "Continue"] {
        let mut b = brief();
        set_text(&mut b, text);
        let p = package(&b);
        let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
        assert!(prompt.contains(&format!("visible-text payload {text:?}")));
        assert!(!prompt.contains(&format!("LOCAL DETAIL \"{text} — dimensions\"")));
        assert!(!prompt.contains(&format!("DIMENSIONED RECTANGLE button {text:?}")));
        let scene: Json = serde_json::from_slice(&p.files()["scene.json"]).unwrap();
        assert_eq!(
            scene["views"][0]["components"][0]["properties"][1]["state"]["value"]["value"],
            text
        );
    }
}

#[test]
fn select_captions_bind_wrapping_labels_and_reviewed_display_text() {
    let mut b = brief();
    let s = &mut b["views"][0]["source"]["snapshot"];
    for n in s["nodes"].as_array_mut().unwrap() {
        n["children"] = json!([]);
    }
    for (i, native, role, label) in [
        (0, "SELECT", "combobox", "Overall usefulness"),
        (1, "LABEL", "text", "Sorting :"),
        (2, "SELECT", "combobox", "A1"),
        (3, "LABEL", "text", "Level :"),
        (4, "SELECT", "combobox", "1-20"),
        (5, "DIV", "group", ""),
    ] {
        s["nodes"][i]["native_role"] =
            json!({"availability":"known","value":{"type":"text","value":native}});
        s["nodes"][i]["properties"][0]["state"] =
            json!({"availability":"known","value":{"type":"role","value":role}});
        s["nodes"][i]["properties"][1]["state"] =
            json!({"availability":"known","value":{"type":"text","value":label}});
    }
    for (parent, child) in [(1, 0), (3, 2), (5, 4)] {
        s["nodes"][parent]["children"] = json!([s["nodes"][child]["key"]]);
    }
    let g = s["nodes"][0]["properties"][2]["state"].clone();
    for (i, width) in [(0, 50.), (2, 60.), (4, 70.)] {
        s["nodes"][i]["properties"][2]["state"] = g.clone();
        s["nodes"][i]["properties"][2]["state"]["value"]["value"]["shape"]["value"]["width"] =
            json!(width);
    }
    let p = package(&b);
    let text = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    for name in [
        "select under \"Sorting :\", displaying \"Overall usefulness\"",
        "select under \"Level :\", displaying \"A1\"",
        "select displaying \"1-20\"",
    ] {
        assert!(
            text.contains(&format!("DIMENSIONED RECTANGLE {name}:")),
            "{name}"
        );
    }
    assert!(!text.contains("pagination"));
}
