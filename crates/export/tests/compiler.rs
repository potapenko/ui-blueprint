use uiblueprint_export::*;
use uiblueprint_schema::model::*;
fn set_test_role(node: &mut Node, native: &str) {
    node.native_role = Availability::Known {
        value: Value::Text(native.into()),
    };
    let role = match native {
        "FORM" => Role::Form,
        "LABEL" => Role::Text,
        "SELECT" => Role::Combobox,
        "INPUT" => Role::Textbox,
        "DIV" => Role::Group,
        "BUTTON" => Role::Button,
        _ => panic!("explicit test role"),
    };
    let Property::Requested {
        field: Field::Role,
        state,
        ..
    } = &mut node.properties[0]
    else {
        panic!()
    };
    *state = Availability::Known {
        value: Value::Role(role),
    };
}
fn limits() -> ExportLimits {
    ExportLimits {
        max_input_bytes: 2_000_000,
        max_output_bytes: 4_000_000,
        max_components: 256,
        max_views: 8,
        components_per_detail: 12,
    }
}

#[test]
fn dense_prompt_defines_objects_once_and_resolves_detail_references() {
    let mut b = fixture("proposed");
    b.views[0].state = "first suggestion keyboard-active; none applied".into();
    let layout = proposed(&mut b);
    let control = layout.components[9].clone();
    for i in 10..160 {
        let mut c = control.clone();
        c.id = format!("dense-{i}");
        c.label = format!("Public control {i}");
        layout.components.push(c);
    }
    let original = serde_json::to_vec(&b).unwrap();
    let p = compile(&b, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    for i in 10..160 {
        assert!(prompt.contains(&format!("Public control {i}")));
    }
    let definitions = prompt
        .lines()
        .filter(|l| l.starts_with("Object ") && l.contains(" [#"))
        .collect::<Vec<_>>();
    assert_eq!(definitions.len(), 160);
    for line in prompt.lines().filter_map(|l| l.strip_prefix("Reuse ")) {
        for reference in line.trim_end_matches('.').split(", ") {
            assert_eq!(
                definitions
                    .iter()
                    .filter(|l| l.contains(&format!("[{reference}]")))
                    .count(),
                1
            );
        }
    }
    assert_eq!(prompt.matches(&b.views[0].state).count(), 1);
    assert_eq!(
        data(&p, "scene.json")["views"][0]["components"]
            .as_array()
            .unwrap()
            .len(),
        160
    );
    assert_eq!(serde_json::to_vec(&b).unwrap(), original);
}

#[test]
fn accessible_help_text_survives_without_becoming_an_input_value() {
    let mut b = fixture("observed");
    let SourceInput::Observed {
        snapshot,
        public_text_fields,
    } = &mut b.views[0].source
    else {
        panic!()
    };
    public_text_fields.push(Field::AccessibilityName);
    let node = &mut snapshot.nodes[0];
    set_test_role(node, "LABEL");
    let Property::Requested { state, .. } = &mut node.properties[1] else {
        panic!()
    };
    *state = Availability::Known {
        value: Value::Text("Public help kept verbatim".into()),
    };
    let p = compile(&b, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(
        prompt
            .lines()
            .any(|l| l.starts_with("Object label layout \"Public help kept verbatim\""))
    );
    assert!(prompt.contains("name is NOT field content"));
    assert!(prompt.contains("Missing value/state is unknown"));
}
fn fixture(name: &str) -> DrawingBrief {
    let path = format!(
        "{}/../../fixtures/export/{name}-brief.json",
        env!("CARGO_MANIFEST_DIR")
    );
    DrawingBrief::from_json(&std::fs::read(path).expect("fixture"), limits()).expect("valid brief")
}
fn data(p: &Package, file: &str) -> serde_json::Value {
    serde_json::from_slice(&p.files()[file]).expect("JSON")
}
fn proposed(b: &mut DrawingBrief) -> &mut ProposedLayout {
    match &mut b.views[0].source {
        SourceInput::Proposed { layout } => layout,
        _ => panic!("proposal"),
    }
}
#[test]
fn exact_example_full_template_and_independent_statuses() {
    let b = fixture("proposed");
    let p = compile(&b, limits()).expect("compile");
    assert_eq!(p.files().len(), 9);
    assert_eq!(data(&p, "manifest.json")["package_version"], "0.4.0");
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    for section in [
        "ЗАДАЧА",
        "ОСНОВАНИЕ",
        "ПРОЕКЦИЯ И ЛИСТ",
        "ГРАФИЧЕСКИЙ ЯЗЫК",
        "ОБЪЕКТЫ И ИЕРАРХИЯ",
        "ГЕОМЕТРИЯ",
        "НАНЕСЕНИЕ РАЗМЕРОВ",
        "СОСТОЯНИЯ И ДЕЙСТВИЯ",
        "ОСНОВНАЯ НАДПИСЬ И ВЕДОМОСТЬ",
        "РЕЗУЛЬТАТ",
    ] {
        assert!(prompt.contains(section));
    }
    assert!(!prompt.contains("{{"));
    assert!(prompt.contains("Размеры по подписям; не измерять по изображению"));
    let manifest = data(&p, "manifest.json");
    assert_eq!(manifest["validation_status"], "unverified");
    assert_eq!(manifest["approval_status"], "draft");
    assert_eq!(manifest["local_numeric_validation"], "checked");
    let scene = data(&p, "scene.json");
    assert_eq!(
        scene["views"][0]["components"].as_array().unwrap().len(),
        10
    );
    assert_eq!(
        scene["views"][0]["components"][9]["geometry"]["shape"]["value"]["x"],
        1008.0
    );
    assert_eq!(data(&p, "sheets.json").as_array().unwrap().len(), 3);
}

#[test]
fn separate_prompts_cover_source_and_own_every_selected_dimension_once() {
    let b = fixture("proposed");
    let p = compile(&b, limits()).unwrap();
    let m = data(&p, "manifest.json");
    let sheets = m["sheet_prompts"].as_array().unwrap();
    assert_eq!(sheets.len(), 3);
    assert_eq!(sheets[0]["sheet_id"], "G01");
    let overview = sheets[0]["component_ids"].as_array().unwrap();
    for i in 0..10 {
        assert!(overview.contains(&serde_json::json!(format!("N{i:02}"))));
    }
    let mut owned = Vec::new();
    for sheet in sheets {
        let text = std::str::from_utf8(&p.files()[sheet["file"].as_str().unwrap()]).unwrap();
        assert!(text.contains(&format!(
            "Generate ONLY sheet {}",
            sheet["sheet_id"].as_str().unwrap()
        )));
        assert!(!text.contains("{{"));
        owned.extend(sheet["dimension_ids"].as_array().unwrap().iter().cloned());
    }
    let unique = owned
        .iter()
        .map(|x| x.as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(unique.len(), owned.len());
    assert_eq!(
        unique,
        m["required_dimension_ids"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap())
            .collect()
    );
    for id in [
        "N00-width",
        "N00-height",
        "N06-width",
        "N06-height",
        "N09-width",
        "N09-height",
        "field-gap",
    ] {
        assert!(unique.contains(id), "{id}");
    }
    let d1 = std::str::from_utf8(&p.files()["prompt-D01.txt"]).unwrap();
    assert!(!d1.contains("VIEW PLAN: complete overview"));
    assert!(d1.contains("VIEW PLAN: ONLY this detail sheet's assigned measurement blocks"));
    assert!(d1.contains("MEASUREMENT ONLY"));
    assert!(!d1.contains("Use the original reference for artwork appearance"));
    assert!(!d1.contains("; text ") && !d1.contains("box[x"));
    assert!(d1.contains("excludes its reported parent"));
    assert!(d1.contains("WIDTH 400 CSS px") && d1.contains("HEIGHT 48 CSS px"));
    let d2 = std::str::from_utf8(&p.files()["prompt-D02.txt"]).unwrap();
    assert!(d2.contains("WIDTH 120 CSS px") && d2.contains("HEIGHT 48 CSS px"));
    let bytes = p.files().values().map(Vec::len).sum();
    let mut exact = limits();
    exact.max_output_bytes = bytes;
    assert!(compile(&b, exact).is_ok());
    exact.max_output_bytes -= 1;
    assert_eq!(compile(&b, exact).unwrap_err(), ExportError::OutputLimit);
    let mut unsplit = b.clone();
    unsplit.details.clear();
    let old = compile(&unsplit, limits()).unwrap();
    assert_eq!(old.version(), "0.1.0");
    assert_eq!(old.files().len(), 6);
    assert_eq!(old.files()["scene.json"], p.files()["scene.json"]);
    assert_eq!(old.files()["dimensions.json"], p.files()["dimensions.json"]);
}
#[test]
fn observed_full_scope_exact_fractional_geometry_and_unknowns() {
    let b = fixture("observed");
    let p = compile(&b, limits()).unwrap();
    let scene = data(&p, "scene.json");
    let v = &scene["views"][0];
    assert_eq!(v["components"].as_array().unwrap().len(), 32);
    assert_eq!(v["relations"].as_array().unwrap().len(), 17);
    assert_eq!(v["coverage"]["status"], "partial");
    assert!(v["coverage"]["unknown_count"].is_null());
    assert_eq!(
        v["components"][0]["properties"][2]["state"]["value"]["value"]["shape"]["value"]["width"],
        97.296875
    );
    assert_eq!(
        v["components"][0]["properties"][3]["state"]["availability"],
        "unknown"
    );
    assert!(data(&p, "sheets.json").as_array().unwrap().len() > 1);
    let all = String::from_utf8(p.files().values().flatten().copied().collect()).unwrap();
    assert!(!all.contains("fixture-ref-"));
    assert!(!all.contains("payload_ref"));
    // Unknown consistency remains attributed; it is not an affirmative unstable state.
    assert_eq!(v["observations"][0]["consistency"], "unknown");
}
#[test]
fn reject_wrong_anchor_unit_value_and_chain() {
    for variant in 0..5 {
        let mut b = fixture("proposed");
        let p = proposed(&mut b);
        match variant {
            0 => p.dimensions[0].anchors[0].component = "missing".into(),
            1 => p.dimensions[0].units = Unit::Pt,
            2 => p.dimensions[0].value = Some(1199.0),
            3 => p.chains[0].terms.pop().map(|_| ()).unwrap(),
            _ => p.dimensions[0].source_kind = SourceKind::Observed,
        }
        assert!(compile(&b, limits()).is_err(), "variant {variant}");
    }
}
#[test]
fn reject_cycles_nonfinite_and_duplicate_components() {
    let mut b = fixture("proposed");
    proposed(&mut b).components[0].parent = Some("N09".into());
    assert!(compile(&b, limits()).is_err());
    let mut b = fixture("proposed");
    let p = proposed(&mut b);
    p.components.push(p.components[0].clone());
    assert!(compile(&b, limits()).is_err());
    let mut b = fixture("proposed");
    proposed(&mut b).dimensions[0].value = Some(f64::NAN);
    assert!(compile(&b, limits()).is_err());
}
#[test]
fn bounds_and_private_diagnostics() {
    let b = fixture("proposed");
    let mut l = limits();
    l.max_output_bytes = 100;
    assert_eq!(compile(&b, l).unwrap_err(), ExportError::OutputLimit);
    l = limits();
    l.max_components = 9;
    assert_eq!(compile(&b, l).unwrap_err(), ExportError::InputLimit);
    l = limits();
    l.max_input_bytes = 10;
    assert!(DrawingBrief::from_json(b"secret-canary-invalid", l).is_err());
    let mut b = b;
    b.metadata.title = "/Users/private/secret-canary".into();
    let e = compile(&b, limits()).unwrap_err();
    assert_eq!(e, ExportError::PrivateContent);
    assert!(!format!("{e:?}").contains("canary"));
}
#[test]
fn explicit_text_policy_sensitive_properties_and_source_ids() {
    let mut b = fixture("observed");
    let SourceInput::Observed {
        snapshot,
        public_text_fields,
    } = &mut b.views[0].source
    else {
        panic!()
    };
    public_text_fields.clear();
    let p = &mut snapshot.nodes[0].properties[1];
    if let Property::Requested { state, .. } = p {
        *state = Availability::Known {
            value: Value::Text("CANARY_PUBLIC_NOT_APPROVED".into()),
        };
    }
    let p = compile(&b, limits()).unwrap();
    let all = String::from_utf8(p.files().values().flatten().copied().collect()).unwrap();
    assert!(!all.contains("CANARY_PUBLIC_NOT_APPROVED"));
    assert!(all.contains("redacted"));
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    if let Property::Requested { sensitivity, .. } = &mut snapshot.nodes[0].properties[1] {
        *sensitivity = Sensitivity::Sensitive;
    }
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::InvalidSource
    );
}
#[test]
fn document_default_and_explain_alias_do_not_accept_proposal() {
    let mut value = serde_json::to_value(fixture("observed")).unwrap();
    value.as_object_mut().unwrap().remove("purpose");
    let bytes = serde_json::to_vec(&value).unwrap();
    assert_eq!(
        DrawingBrief::from_json(&bytes, limits()).unwrap().purpose,
        Purpose::Document
    );
    value["purpose"] = "explain".into();
    assert_eq!(
        DrawingBrief::from_json(&serde_json::to_vec(&value).unwrap(), limits())
            .unwrap()
            .purpose,
        Purpose::Document
    );
    let mut b = fixture("proposed");
    b.purpose = Purpose::Document;
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::InvalidSource
    );
}
#[test]
fn named_approval_is_separate_from_ungenerated_image() {
    let mut b = fixture("proposed");
    b.metadata.approval.status = ApprovalStatus::Accepted;
    assert!(compile(&b, limits()).is_err());
    b.metadata.approval.named_record = Some("user-approved-intent-revision-1".into());
    let p = compile(&b, limits()).unwrap();
    assert_eq!(data(&p, "manifest.json")["validation_status"], "unverified");
}
#[test]
fn detail_and_compare_require_explicit_valid_references_and_bases() {
    let mut b = fixture("proposed");
    b.purpose = Purpose::Detail;
    assert!(compile(&b, limits()).is_ok());
    b.details[0].components.push("missing".into());
    assert!(compile(&b, limits()).is_err());
    let mut b = fixture("proposed");
    b.purpose = Purpose::Compare;
    b.views.push(fixture("observed").views.remove(0));
    b.comparisons.push(ComparisonRequest {
        geometry_space: None,
        before: "proposal".into(),
        after: "observed".into(),
        different_basis: None,
    });
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::IncompatibleViews
    );
    b.comparisons[0].different_basis = Some(
        "Different synthetic requirement and historical F01 runtime; no identity equivalence"
            .into(),
    );
    assert!(compile(&b, limits()).is_ok());
}
#[test]
fn assumed_flow_is_not_confirmed_and_requires_separate_sheet() {
    let mut b = fixture("observed");
    b.purpose = Purpose::Flow;
    b.transitions.push(FlowInput {
        before: "observed".into(),
        after: None,
        description: "Available action; destination unknown".into(),
        transition: None,
        actions: vec![],
    });
    let p = compile(&b, limits()).unwrap();
    assert_eq!(
        data(&p, "scene.json")["flow"][0]["status"],
        "unverified_or_unknown_destination"
    );
    assert!(
        data(&p, "sheets.json")
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s["id"] == "F01")
    );
}
#[test]
fn output_refuses_existing_destination() {
    let dir = std::env::temp_dir().join(format!("uib-e01-existing-{}", std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(dir.join("baseline"), "keep").unwrap();
    let p = compile(&fixture("proposed"), limits()).unwrap();
    assert_eq!(
        p.write_new(&dir).unwrap_err(),
        ExportError::DestinationExists
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("baseline")).unwrap(),
        "keep"
    );
    std::fs::remove_file(dir.join("baseline")).unwrap();
    std::fs::remove_dir(dir).unwrap();
}

#[test]
fn confirmed_flow_retains_modality_and_requires_action_and_after_evidence() {
    let doc = Document::from_json(
        include_bytes!("../../../fixtures/golden/GOLDEN01.json"),
        1_000_000,
    )
    .unwrap();
    let Artifact::GoldenChain(chain) = doc.artifact else {
        panic!("golden chain")
    };
    let mut b = fixture("observed");
    b.purpose = Purpose::Flow;
    b.views[0].id = "before".into();
    b.views[0].safe_source_reference = "GOLDEN01 synthetic evidence; not runtime proof".into();
    b.views[0].environment = "Synthetic canonical test data".into();
    b.views[0].source = SourceInput::Observed {
        snapshot: Box::new(chain.before.clone()),
        public_text_fields: vec![Field::Name],
    };
    let mut after = b.views[0].clone();
    after.id = "after".into();
    after.state = "checked".into();
    after.source = SourceInput::Observed {
        snapshot: Box::new(chain.after.clone()),
        public_text_fields: vec![Field::Name],
    };
    b.views.push(after);
    b.transitions.push(FlowInput {
        before: "before".into(),
        after: Some("after".into()),
        description: "Synthetic GOLDEN01 set_checked scenario, not live runtime proof".into(),
        transition: Some(chain.verification.clone()),
        actions: vec![chain.action.clone()],
    });
    let p = compile(&b, limits()).unwrap();
    let flow = data(&p, "scene.json");
    assert_eq!(flow["flow"][0]["status"], "confirmed_transition");
    assert_eq!(flow["flow"][0]["steps"][0]["modality"], "setter");
    assert_eq!(flow["flow"][0]["steps"][0]["intent"], "set_checked(true)");
    b.transitions[0].actions.clear();
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::InvalidReference
    );
    b.transitions[0].actions.push(chain.action.clone());
    b.transitions[0].after = None;
    assert!(compile(&b, limits()).is_err());
}

// Literal widths/heights read from the retained F01 source rectangles, not compiler
// output. Partial coverage and unknown consistency do not erase these known anchors
// (accepted engine changes 256f2a2 and 2491dec). Historical package files stay intact.
fn current_observed_dimensions() -> (String, String) {
    let historical = std::fs::read_to_string(format!(
        "{}/../../fixtures/export/observed-package/dimensions.json",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();
    let expected = [
        ("N000", 97.296875, 32.0),
        ("N002", 200.0, 60.0),
        ("N004", 98.78125, 32.0),
        ("N006", 120.0, 40.0),
        ("N008", 120.0, 40.0),
        ("N010", 100.0, 40.0),
        ("N012", 60.0, 30.0),
        ("N014", 360.0, 123.0),
        ("N016", 188.0, 21.0),
        ("N018", 360.0, 22.0),
        ("N020", 360.0, 22.0),
        ("N022", 87.625, 32.0),
        ("N024", 0.0, 18.0),
        ("N026", 146.9375, 32.0),
        ("N028", 100.0, 30.0),
        ("N030", 32.0, 16.0),
    ];
    let parsed: serde_json::Value = serde_json::from_str(&historical).unwrap();
    assert_eq!(parsed[0]["dimensions"].as_array().unwrap().len(), 32);
    assert_eq!(historical.matches("\"value\": null").count(), 32);
    assert_eq!(
        historical
            .matches("\"unknown_reason\": \"unstable_state\"")
            .count(),
        32
    );
    let mut current = historical.clone();
    for (index, (component, width, height)) in expected.into_iter().enumerate() {
        for (axis, value) in [width, height].into_iter().enumerate() {
            let dimension = &parsed[0]["dimensions"][index * 2 + axis];
            assert_eq!(dimension["id"], format!("M{:04}", index * 2 + axis + 1));
            for anchor in dimension["anchors"].as_array().unwrap() {
                assert_eq!(anchor["component"], component);
            }
            current = current.replacen("\"value\": null", &format!("\"value\": {value:?}"), 1);
        }
    }
    current = current.replace(
        "\"unknown_reason\": \"unstable_state\"",
        "\"unknown_reason\": null",
    );
    (historical, current)
}
#[test]
fn human_captions_preserve_unicode_and_distinguish_source_groups() {
    for name in [
        "⏱️ Speed",
        "Русский 世界 e\u{301}",
        "Quoted \"name\"",
        "First line\nSecond\tline",
    ] {
        let mut b = fixture("observed");
        let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
            panic!()
        };
        let Property::Requested { state, .. } = &mut snapshot.nodes[0].properties[1] else {
            panic!()
        };
        *state = Availability::Known {
            value: Value::Text(name.into()),
        };
        let original = serde_json::to_vec(&b).unwrap();
        let p = compile(&b, limits()).unwrap();
        assert_eq!(serde_json::to_vec(&b).unwrap(), original);
        assert_eq!(
            data(&p, "scene.json")["views"][0]["components"][0]["properties"][1]["state"]["value"]
                ["value"],
            name
        );
        let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
        assert!(!prompt.contains("\\u{"));
        let object = prompt
            .lines()
            .find(|line| line.starts_with("Object ") && line.contains(" [#1]"))
            .unwrap();
        assert!(object.contains(&serde_json::to_string(name).unwrap()));
        assert!(!prompt.lines().any(|line| line.starts_with("Second")));
        let mut exact = limits();
        exact.max_output_bytes = p.files().values().map(Vec::len).sum();
        assert!(compile(&b, exact).is_ok());
        exact.max_output_bytes -= 1;
        assert_eq!(compile(&b, exact).unwrap_err(), ExportError::OutputLimit);
    }
    for name in [
        "forbidden\u{1}caption",
        "forbidden\rline",
        "forbidden\u{7f}caption",
    ] {
        let mut b = fixture("observed");
        let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
            panic!()
        };
        let Property::Requested { state, .. } = &mut snapshot.nodes[0].properties[1] else {
            panic!()
        };
        *state = Availability::Known {
            value: Value::Text(name.into()),
        };
        let p = compile(&b, limits()).unwrap();
        assert_eq!(
            data(&p, "scene.json")["views"][0]["components"][0]["properties"][1]["state"]["availability"],
            "redacted"
        );
        assert!(
            !std::str::from_utf8(&p.files()["prompt.txt"])
                .unwrap()
                .contains("forbidden")
        );
    }
    let mut b = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    for node in &mut snapshot.nodes {
        set_test_role(node, "DIV");
        node.children.clear();
    }
    snapshot.relations.clear();
    for i in [2, 6] {
        set_test_role(&mut snapshot.nodes[i], "BUTTON");
    }
    snapshot.nodes[0].children = vec![snapshot.nodes[2].key.clone()];
    snapshot.nodes[4].children = vec![snapshot.nodes[6].key.clone()];
    for (i, width, height) in [(2, 16., 16.), (6, 228., 80.)] {
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[i].properties[2]
        else {
            panic!()
        };
        g.shape = Shape::Rect(Rect {
            x: 0.,
            y: 0.,
            width,
            height,
        });
    }
    let p = compile(&b, limits()).unwrap();
    let text = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(text.contains("DIMENSIONED RECTANGLE button, item 1 in group 1: WIDTH 16 CSS px ABOVE; HEIGHT 16 CSS px LEFT"));
    assert!(text.contains("DIMENSIONED RECTANGLE button, item 1 in group 2: WIDTH 228 CSS px ABOVE; HEIGHT 80 CSS px LEFT"));
}

#[test]
fn human_presentation_preserves_exact_historical_machine_fields() {
    for name in ["observed", "proposed"] {
        let brief = fixture(name);
        let original = serde_json::to_vec(&brief).unwrap();
        let package = compile(&brief, limits()).unwrap();
        assert_eq!(serde_json::to_vec(&brief).unwrap(), original);
        let historical = |file: &str| {
            std::fs::read_to_string(format!(
                "{}/../../fixtures/export/{name}-package/{file}",
                env!("CARGO_MANIFEST_DIR")
            ))
            .unwrap()
        };
        let old_scene: serde_json::Value = serde_json::from_str(&historical("scene.json")).unwrap();
        let scene = data(&package, "scene.json");
        assert_eq!(scene["guide"], "UIB.DRAWING@1.2");
        assert_eq!(scene["views"].to_string(), old_scene["views"].to_string());
        assert_eq!(
            data(&package, "sheets.json").to_string(),
            serde_json::from_str::<serde_json::Value>(&historical("sheets.json"))
                .unwrap()
                .to_string()
        );
        let expected: serde_json::Value = serde_json::from_str(&if name == "observed" {
            current_observed_dimensions().1
        } else {
            historical("dimensions.json")
        })
        .unwrap();
        let actual = data(&package, "dimensions.json");
        let old = expected[0]["dimensions"].as_array().unwrap();
        let new = actual[0]["dimensions"].as_array().unwrap();
        assert_eq!(
            serde_json::to_string(&new[..old.len()]).unwrap(),
            serde_json::to_string(old).unwrap()
        );
        assert_eq!(actual[0]["chains"], expected[0]["chains"]);
        let human = std::str::from_utf8(&package.files()["drawing-brief.md"]).unwrap();
        assert!(!human.contains("```json"));
        for text in [
            &brief.metadata.document_id,
            &brief.metadata.revision,
            &brief.metadata.audience,
            &brief.metadata.owner,
            &brief.metadata.retention,
            &brief.views[0].scope,
            &brief.views[0].state,
        ] {
            assert!(human.contains(text), "missing brief meaning: {text}");
        }
        for file in [
            "scene.json",
            "dimensions.json",
            "sheets.json",
            "manifest.json",
        ] {
            assert!(human.contains(&format!("]({file})")));
        }
        assert!(human.contains("Image validation: unverified"));
        assert!(human.contains("Reference images: none included or verified"));
        if name == "observed" {
            assert!(human.contains("| Original UI image |"));
        } else {
            assert!(!human.contains("| Original UI image |"));
            assert!(human.contains("| Optional for proposed appearance |"));
            assert!(human.contains("Declared chain:"));
        }
        let prompt = std::str::from_utf8(&package.files()["prompt.txt"]).unwrap();
        assert!(!prompt.contains("COMMON "));
        assert!(!prompt.contains("observation_id"));
        assert!(!prompt.contains("clock_domain"));
        assert!(prompt.contains("rounded labels"));
        assert!(prompt.contains("image unverified"));
    }
}

#[test]
fn explicitly_unstable_observation_keeps_dimensions_unknown_with_evidence() {
    let mut brief = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!("observed fixture")
    };
    for observation in &mut snapshot.observations {
        observation.consistency = Consistency::Unstable;
        observation.consistency_reason = Some(Id("synthetic_unstable_source".into()));
    }
    let original = serde_json::to_vec(&brief).unwrap();
    let package = compile(&brief, limits()).unwrap();
    assert_eq!(serde_json::to_vec(&brief).unwrap(), original);
    let dimensions = data(&package, "dimensions.json");
    let dimensions = dimensions[0]["dimensions"].as_array().unwrap();
    assert!(dimensions.len() > 32);
    for dimension in dimensions {
        assert!(dimension["value"].is_null());
        assert_eq!(dimension["unknown_reason"], "unstable_state");
        assert!(!dimension["evidence"].as_array().unwrap().is_empty());
        assert!(dimension["requirement_ref"].is_null());
        assert!(dimension["check_tolerance"].is_null());
    }
    assert_eq!(
        data(&package, "scene.json")["views"][0]["coverage"]["status"],
        "partial"
    );
    assert_eq!(
        data(&package, "manifest.json")["validation_status"],
        "unverified"
    );
}

#[test]
fn factual_query_known_extents_keep_source_evidence_without_normative_fields() {
    let doc = Document::from_json(
        include_bytes!("../../../fixtures/golden/GEO-SIZE-RATIO__width.json"),
        100_000,
    )
    .unwrap();
    let Artifact::Finding(case) = doc.artifact else {
        panic!("canonical authored fixture")
    };
    let mut b = fixture("observed");
    b.views[0].source = SourceInput::Observed {
        snapshot: Box::new(case.snapshot),
        public_text_fields: vec![],
    };
    b.views[0].safe_source_reference =
        "GEO-SIZE-RATIO authored fixture; not runtime evidence".into();
    b.views[0].environment = "Synthetic canonical test data".into();
    let before = serde_json::to_vec(&b).unwrap();
    let package = compile(&b, limits()).unwrap();
    assert_eq!(serde_json::to_vec(&b).unwrap(), before);
    let dimensions = data(&package, "dimensions.json");
    let first = &dimensions[0]["dimensions"][0];
    assert_eq!(first["value"], 30.0);
    assert_eq!(dimensions[0]["dimensions"][1]["value"], 10.0);
    assert_eq!(first["source_kind"], "observed");
    assert_eq!(first["units"], "css_px");
    assert!(first["requirement_ref"].is_null());
    assert!(first["check_tolerance"].is_null());
    assert!(first["unknown_reason"].is_null());
    let scene = data(&package, "scene.json");
    assert_eq!(
        first["evidence"],
        serde_json::json!([scene["views"][0]["components"][0]["properties"][0]["evidence"]])
    );
    assert_eq!(
        data(&package, "manifest.json")["validation_status"],
        "unverified"
    );
}

// Independent reader for the human table notation. Reconstruct nested arrays/objects
// from the documented paths, rather than using the exporter's field factoring code.
#[test]
fn prompt_details_follow_only_the_actual_sheet_plan() {
    let mut b = fixture("proposed");
    b.details.clear();
    let p = compile(&b, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("general:"));
    assert!(!prompt.contains("detail:"));
    assert_eq!(data(&p, "sheets.json").as_array().unwrap().len(), 1);
    let mut l = limits();
    l.components_per_detail = 6;
    let p = compile(&b, l).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("detail:"));
    assert_eq!(prompt.matches("detail:").count(), 2);
    let sheets = data(&p, "sheets.json");
    assert_eq!(sheets[1]["parent_view"], "G01");
    assert_eq!(
        sheets[2]["components"],
        serde_json::json!(["N06", "N07", "N08", "N09"])
    );
    assert!(prompt.contains("Object"));
}

#[test]
fn display_rounding_keeps_machine_property_order_and_signed_zero() {
    let mut b = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    let mut expected: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../../../fixtures/export/observed-package/scene.json"
    ))
    .unwrap();
    // These are source changes, not expectations copied from the candidate output.
    for index in [0, 1] {
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[index * 2].properties[2]
        else {
            panic!()
        };
        let Shape::Rect(rect) = &mut g.shape else {
            panic!()
        };
        rect.x = if index == 0 { -0.0 } else { 0.0 };
        expected["views"][0]["components"][index * 2]["properties"][2]["state"]["value"]["value"]
            ["shape"]["value"]["x"] = serde_json::json!(rect.x);
    }
    snapshot.nodes[2].properties.swap(0, 2);
    expected["views"][0]["components"][2]["properties"]
        .as_array_mut()
        .unwrap()
        .swap(0, 2);
    let p = compile(&b, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert_eq!(
        data(&p, "scene.json")["views"][0]["components"].to_string(),
        expected["views"][0]["components"].to_string()
    );
    assert!(!prompt.contains("x -0"));
    assert!(prompt.contains("x 0"));
}

#[test]
fn derived_insets_use_source_anchors_and_engine_evidence() {
    let mut brief = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!()
    };
    for (index, x, width) in [(0, 10.0, 100.0), (2, 35.25, 25.0)] {
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[index].properties[2]
        else {
            panic!()
        };
        let Shape::Rect(r) = &mut g.shape else {
            panic!()
        };
        r.x = x;
        r.width = width;
    }
    let original = serde_json::to_vec(&brief).unwrap();
    let package = compile(&brief, limits()).unwrap();
    assert_eq!(serde_json::to_vec(&brief).unwrap(), original);
    let dims = data(&package, "dimensions.json");
    let inset = dims[0]["dimensions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| {
            d["label"] == "Root left edge inset"
                && d["anchors"][0]["component"] == "N000"
                && d["anchors"][1]["component"] == "N002"
        })
        .unwrap();
    assert_eq!(inset["value"], 25.25);
    assert_eq!(inset["anchors"][0]["edge"], "left");
    assert_eq!(inset["anchors"][1]["edge"], "left");
    assert!(!inset["evidence"].as_array().unwrap().is_empty());
    assert!(inset["check_tolerance"].is_null());
}

#[test]
fn prompt_requires_anchored_control_rhythm_and_retains_coordinate_context() {
    let mut brief = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!()
    };
    for (i, role, x, y, width, height) in [
        (0, "FORM", 10.0, 20.0, 400.0, 500.0),
        (2, "LABEL", 30.0, 60.0, 160.0, 20.0),
        (4, "SELECT", 202.0, 54.0, 198.25, 33.125),
        (6, "SELECT", 202.0, 102.671875, 198.25, 33.125),
    ] {
        set_test_role(&mut snapshot.nodes[i], role);
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[i].properties[2]
        else {
            panic!()
        };
        let Shape::Rect(r) = &mut g.shape else {
            panic!()
        };
        *r = Rect {
            x,
            y,
            width,
            height,
        };
    }
    snapshot.nodes[0].children = [2, 4, 6].map(|i| snapshot.nodes[i].key.clone()).to_vec();
    let package = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&package.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("≈16 CSS px"));
    assert!(prompt.contains("12 CSS px"));
    assert!(
        prompt
            .lines()
            .any(|line| line.starts_with("EDGE FRAGMENT (VERTICAL")
                && line.contains("Bottom boundary")
                && line.contains("Top boundary"))
    );
    let fragments = prompt
        .lines()
        .filter(|line| line.starts_with("EDGE FRAGMENT ("))
        .count();
    assert!(fragments > 0);
    assert_eq!(prompt.matches("print START ").count(), fragments);
    assert_eq!(prompt.matches("print END ").count(), fragments);
    assert!(prompt.contains("Placement context 1: LayoutBounds;"));
    assert!(prompt.contains("Viewport, CSS px, TopLeft; transform local only"));
    assert!(prompt.contains("WIDTH ≈198 CSS px") && prompt.contains("HEIGHT ≈33 CSS px"));
    assert!(prompt.contains("↕ ≈16 CSS px"));
    assert!(prompt.contains("│←─ 12 CSS px ─→│"));
    assert!(prompt.contains("HEIGHT ≈33 CSS px LEFT"));
    assert!(!prompt.contains("[\"css_px\"]"));
}

#[test]
fn native_select_rows_get_anchored_spacing_without_crossing_owners() {
    let mut b = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    for node in &mut snapshot.nodes {
        set_test_role(node, "DIV");
        node.children.clear();
    }
    for (i, x, y, width, height) in [
        (0, 0., 0., 900., 500.),
        (2, 430., 180., 400., 50.),
        (4, 630.828125, 189.390625, 197.921875, 33.125),
        (6, 430., 230., 400., 50.),
        (8, 630.828125, 238.546875, 197.921875, 33.125),
    ] {
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[i].properties[2]
        else {
            panic!()
        };
        g.shape = Shape::Rect(Rect {
            x,
            y,
            width,
            height,
        });
    }
    for i in [4, 8] {
        snapshot.nodes[i].native_role = Availability::Known {
            value: Value::Text("select".into()),
        };
        let Property::Requested { state, .. } = &mut snapshot.nodes[i].properties[0] else {
            panic!()
        };
        *state = Availability::Unknown {
            reason: Id("not-collected".into()),
        };
    }
    snapshot.nodes[0].children = [2, 6].map(|i| snapshot.nodes[i].key.clone()).to_vec();
    snapshot.nodes[2].children = vec![snapshot.nodes[4].key.clone()];
    snapshot.nodes[6].children = vec![snapshot.nodes[8].key.clone()];
    let original = serde_json::to_vec(&b).unwrap();
    let mut l = limits();
    l.components_per_detail = 256;
    let p = compile(&b, l).unwrap();
    assert_eq!(serde_json::to_vec(&b).unwrap(), original);
    let dims = data(&p, "dimensions.json");
    let gaps = dims[0]["dimensions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["label"] == "Repeated control clear gap")
        .collect::<Vec<_>>();
    assert_eq!(gaps.len(), 1);
    assert_eq!(gaps[0]["value"], 16.03125);
    assert_eq!(gaps[0]["anchors"][0]["component"], "N004");
    assert_eq!(gaps[0]["anchors"][0]["edge"], "bottom");
    assert_eq!(gaps[0]["anchors"][1]["component"], "N008");
    assert_eq!(gaps[0]["anchors"][1]["edge"], "top");
    assert!(!gaps[0]["evidence"].as_array().unwrap().is_empty());
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert_eq!(prompt.matches("↕ ≈16 CSS px").count(), 1);
    assert!(prompt.contains("WIDTH ≈198 CSS px") && prompt.contains("HEIGHT ≈33 CSS px"));
    let mut exact = l;
    exact.max_output_bytes = p.files().values().map(Vec::len).sum();
    assert!(compile(&b, exact).is_ok());
    exact.max_output_bytes -= 1;
    assert_eq!(compile(&b, exact).unwrap_err(), ExportError::OutputLimit);
    for variant in [
        "unknown",
        "space",
        "button",
        "different-owner",
        "intervening-row",
    ] {
        let mut b = b.clone();
        let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
            panic!()
        };
        match variant {
            "unknown" | "space" => {
                let Property::Requested { state, .. } = &mut snapshot.nodes[8].properties[2] else {
                    panic!()
                };
                if variant == "unknown" {
                    *state = Availability::Unknown {
                        reason: Id("not-collected".into()),
                    };
                } else {
                    let Availability::Known {
                        value: Value::Geometry(g),
                    } = state
                    else {
                        panic!()
                    };
                    g.coordinate_space.id = Id("unrelated".into());
                }
            }
            "button" => set_test_role(&mut snapshot.nodes[8], "BUTTON"),
            "different-owner" => snapshot.nodes[0].children.pop().map(|_| ()).unwrap(),
            "intervening-row" => {
                let key = snapshot.nodes[10].key.clone();
                snapshot.nodes[0].children.insert(1, key);
            }
            _ => unreachable!(),
        }
        let p = compile(&b, l).unwrap();
        assert!(
            !data(&p, "dimensions.json")[0]["dimensions"]
                .as_array()
                .unwrap()
                .iter()
                .any(|d| d["label"] == "Repeated control clear gap"),
            "{variant}"
        );
        assert!(
            !std::str::from_utf8(&p.files()["prompt.txt"])
                .unwrap()
                .contains("↕ ≈16 CSS px"),
            "{variant}"
        );
    }
}

#[test]
fn visible_text_accessible_name_and_reviewed_draft_never_publish_raw_values() {
    let mut brief = fixture("observed");
    brief.views[0].state =
        "Input open; public draft orchard; first suggestion keyboard-active; none applied".into();
    let SourceInput::Observed {
        snapshot,
        public_text_fields,
    } = &mut brief.views[0].source
    else {
        panic!()
    };
    *public_text_fields = vec![Field::VisibleText, Field::AccessibilityName];
    set_test_role(&mut snapshot.nodes[0], "INPUT");
    let Property::Requested { state, .. } = &mut snapshot.nodes[0].properties[1] else {
        panic!()
    };
    *state = Availability::Known {
        value: Value::Text("Accessible search name".into()),
    };
    for (field, text) in [
        (Field::VisibleText, ""),
        (Field::Value, "UNAPPROVED_RAW_VALUE_CANARY"),
    ] {
        let mut property = snapshot.nodes[0].properties[1].clone();
        let Property::Requested {
            field: f, state, ..
        } = &mut property
        else {
            panic!()
        };
        *f = field;
        *state = Availability::Known {
            value: Value::Text(text.into()),
        };
        snapshot.nodes[0].properties.push(property);
        for node in snapshot.nodes.iter_mut().skip(1) {
            let mut unavailable = node.properties[1].clone();
            let Property::Requested {
                field: f, state, ..
            } = &mut unavailable
            else {
                panic!()
            };
            *f = field;
            *state = Availability::Unknown {
                reason: Id("test-not-collected".into()),
            };
            node.properties.push(unavailable);
        }
        snapshot.context.fields.push(field);
        snapshot.coverage.fields.push(field);
        for observation in &mut snapshot.observations {
            observation.coverage.fields.push(field);
        }
    }
    let package = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&package.files()["prompt.txt"]).unwrap();
    let input = prompt
        .lines()
        .find(|line| line.starts_with("Object input \"Accessible search name\": Textbox"))
        .unwrap();
    assert!(!input.contains("; text "));
    assert_eq!(input.matches("Accessible search name").count(), 1);
    assert!(prompt.contains("public draft orchard"));
    assert!(prompt.contains("name is NOT field content"));
    assert!(!input.contains("visible text \"Accessible search name\""));
    let all = String::from_utf8(package.files().values().flatten().copied().collect()).unwrap();
    assert!(!all.contains("UNAPPROVED_RAW_VALUE_CANARY"));
    let mut clipped = brief.clone();
    let SourceInput::Observed { snapshot, .. } = &mut clipped.views[0].source else {
        panic!()
    };
    for property in &mut snapshot.nodes[0].properties {
        if let Property::Requested {
            field: Field::VisibleText,
            state,
            ..
        } = property
        {
            *state = Availability::Known {
                value: Value::Text("Three partial words".into()),
            };
        }
    }
    let mut visible = snapshot.nodes[0].properties[2].clone();
    let Property::Requested {
        field,
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut visible
    else {
        panic!()
    };
    *field = Field::VisibleRegion;
    g.frame_kind = FrameKind::VisibleRegion;
    let Shape::Rect(rect) = &mut g.shape else {
        panic!()
    };
    rect.height = 4.0;
    snapshot.nodes[0].properties[4] = visible;
    let original = serde_json::to_vec(&clipped).unwrap();
    let p = compile(&clipped, limits()).unwrap();
    let text = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    let lines = text.lines().collect::<Vec<_>>();
    let clip = lines
        .iter()
        .position(|l| l.starts_with("COMPONENT CLIP #1"))
        .unwrap();
    assert!(lines[clip + 1].starts_with("Object input"));
    assert!(
        lines[clip + 1]
            .contains("text (draw only its proved clipped fragment) \"Three partial words\"")
    );
    assert_eq!(serde_json::to_vec(&clipped).unwrap(), original);
    let SourceInput::Observed {
        public_text_fields, ..
    } = &mut brief.views[0].source
    else {
        panic!()
    };
    public_text_fields.clear();
    let redacted = compile(&brief, limits()).unwrap();
    let all = String::from_utf8(redacted.files().values().flatten().copied().collect()).unwrap();
    assert!(!all.contains("Accessible search name"));
    assert!(!all.contains("UNAPPROVED_RAW_VALUE_CANARY"));
}

#[test]
fn popup_prompt_preserves_distinct_input_and_result_width_owners() {
    let mut brief = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!()
    };
    for (i, role, x, y, width, height) in [
        (0, "DIV", 10.0, 20.0, 238.25, 180.0),
        (2, "INPUT", 30.0, 54.0, 198.25, 25.0),
        (4, "BUTTON", 30.0, 85.0, 198.25, 24.0),
        (6, "BUTTON", 30.0, 112.0, 198.25, 24.0),
    ] {
        set_test_role(&mut snapshot.nodes[i], role);
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[i].properties[2]
        else {
            panic!()
        };
        let Shape::Rect(r) = &mut g.shape else {
            panic!()
        };
        *r = Rect {
            x,
            y,
            width,
            height,
        };
    }
    snapshot.nodes[0].children = [2, 4, 6].map(|i| snapshot.nodes[i].key.clone()).to_vec();
    let package = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&package.files()["prompt.txt"]).unwrap();
    assert_eq!(prompt.matches("WIDTH ≈198 CSS px").count(), 2);
    assert_eq!(prompt.matches("LOCAL INSET DETAIL").count(), 1);
    let inset = prompt.split("LOCAL INSET DETAIL").nth(1).unwrap();
    assert!(inset.contains("print END \"input") && inset.contains("20 CSS px"));
    assert!(!inset.contains("Right"));
    let overview = prompt.split("LOCAL DETAIL").next().unwrap();
    assert!(overview.contains("WIDTH ≈238 CSS px"));
    let dimensions = data(&package, "dimensions.json");
    let all = dimensions[0]["dimensions"].as_array().unwrap();
    assert!(all.iter().any(|d| d["anchors"][0]["component"] == "N004"
        && d["label"] == "Width LayoutBounds"
        && d["value"] == 198.25));
    assert!(all.iter().any(|d| d["anchors"][0]["component"] == "N002"
        && d["anchors"][1]["component"] == "N000"
        && d["label"] == "Measured right inset"
        && d["value"] == 20.0));
}

#[test]
fn single_noninteractive_scope_gets_its_known_frame_without_role_inference() {
    let mut b = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    snapshot.nodes.truncate(1);
    snapshot.nodes[0].children.clear();
    snapshot.relations.clear();
    snapshot.components.clear();
    snapshot.nodes[0].native_role = Availability::Known {
        value: Value::Text("SPAN".into()),
    };
    let Property::Requested { state, .. } = &mut snapshot.nodes[0].properties[0] else {
        panic!()
    };
    *state = Availability::Unknown {
        reason: Id("not-collected".into()),
    };
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut snapshot.nodes[0].properties[2]
    else {
        panic!()
    };
    g.shape = Shape::Rect(Rect {
        x: 25.09375,
        y: 343.59375,
        width: 174.671875,
        height: 42.3125,
    });
    let original = serde_json::to_vec(&b).unwrap();
    let p = compile(&b, limits()).unwrap();
    assert_eq!(serde_json::to_vec(&b).unwrap(), original);
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("Object span: SPAN [#1]"));
    assert!(
        prompt.contains(
            "DIMENSIONED RECTANGLE span: WIDTH ≈175 CSS px ABOVE; HEIGHT ≈42 CSS px LEFT"
        )
    );
    let dims = data(&p, "dimensions.json");
    for (i, value, start, end) in [
        (0, 174.671875, "left", "right"),
        (1, 42.3125, "top", "bottom"),
    ] {
        assert_eq!(dims[0]["dimensions"][i]["value"], value);
        for j in [0, 1] {
            assert_eq!(dims[0]["dimensions"][i]["anchors"][j]["component"], "N000");
        }
        assert_eq!(dims[0]["dimensions"][i]["anchors"][0]["edge"], start);
        assert_eq!(dims[0]["dimensions"][i]["anchors"][1]["edge"], end);
        assert_eq!(
            dims[0]["dimensions"][i]["anchors"][0]["frame_kind"],
            "layout_bounds"
        );
    }
    let mut exact = limits();
    exact.max_output_bytes = p.files().values().map(Vec::len).sum();
    assert!(compile(&b, exact).is_ok());
    exact.max_output_bytes -= 1;
    assert_eq!(compile(&b, exact).unwrap_err(), ExportError::OutputLimit);
    // A small text-only scope is principal even below the repeated-control size threshold.
    let mut text = b.clone();
    let SourceInput::Observed { snapshot, .. } = &mut text.views[0].source else {
        panic!()
    };
    set_test_role(&mut snapshot.nodes[0], "LABEL");
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut snapshot.nodes[0].properties[2]
    else {
        panic!()
    };
    g.shape = Shape::Rect(Rect {
        x: 0.,
        y: 0.,
        width: 9.,
        height: 6.,
    });
    let p = compile(&text, limits()).unwrap();
    assert!(
        std::str::from_utf8(&p.files()["prompt.txt"])
            .unwrap()
            .contains(
                "DIMENSIONED RECTANGLE label layout: WIDTH 9 CSS px ABOVE; HEIGHT 6 CSS px LEFT"
            )
    );
    for variant in ["unknown", "hit-only", "visible-only", "two-leaves", "ax"] {
        let mut v = b.clone();
        let SourceInput::Observed { snapshot, .. } = &mut v.views[0].source else {
            panic!()
        };
        if variant == "two-leaves" {
            let mut other = snapshot.nodes[0].clone();
            other.key.key = Id("second-leaf".into());
            snapshot.nodes.push(other);
        } else {
            let mut geometry = snapshot.nodes[0].properties[2].clone();
            let Property::Requested { state, .. } = &mut snapshot.nodes[0].properties[2] else {
                panic!()
            };
            *state = Availability::Unknown {
                reason: Id("not-collected".into()),
            };
            if variant != "unknown" {
                let Property::Requested {
                    field,
                    state:
                        Availability::Known {
                            value: Value::Geometry(g),
                        },
                    ..
                } = &mut geometry
                else {
                    panic!()
                };
                let (target, f, kind) = match variant {
                    "hit-only" => (3, Field::HitRegion, FrameKind::HitRegion),
                    "visible-only" => (4, Field::VisibleRegion, FrameKind::VisibleRegion),
                    "ax" => (
                        2,
                        Field::AccessibilityBounds,
                        FrameKind::AccessibilityBounds,
                    ),
                    _ => unreachable!(),
                };
                *field = f;
                g.frame_kind = kind;
                snapshot.nodes[0].properties[target] = geometry;
                if variant == "ax" {
                    for fields in std::iter::once(&mut snapshot.context.fields)
                        .chain(std::iter::once(&mut snapshot.coverage.fields))
                        .chain(
                            snapshot
                                .observations
                                .iter_mut()
                                .map(|o| &mut o.coverage.fields),
                        )
                    {
                        for field in fields {
                            if *field == Field::LayoutBounds {
                                *field = Field::AccessibilityBounds;
                            }
                        }
                    }
                }
            }
        }
        let p = compile(&v, limits()).unwrap_or_else(|e| panic!("{variant}: {e:?}"));
        let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
        if variant == "ax" {
            assert!(prompt.contains("AX FRAME MEASUREMENT (not painted UI), EXACT OUTSIDE CAPTION \"span\": WIDTH ≈175 CSS px ABOVE; HEIGHT ≈42 CSS px LEFT"));
            assert!(prompt.contains("EMPTY WHITE dashed rectangle"));
        } else {
            assert!(!prompt.contains("DIMENSIONED RECTANGLE span:"), "{variant}");
            assert!(
                !prompt.contains("AX FRAME MEASUREMENT (not painted UI), EXACT"),
                "{variant}"
            );
        }
    }
}

#[test]
fn no_input_dialog_selects_one_existing_direct_child_inset() {
    let mut b = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    for node in &mut snapshot.nodes {
        set_test_role(node, "DIV");
        node.children.clear();
    }
    snapshot.nodes[0].native_role = Availability::Known {
        value: Value::Text("dialog".into()),
    };
    let Property::Requested { state, .. } = &mut snapshot.nodes[0].properties[0] else {
        panic!()
    };
    *state = Availability::Unknown {
        reason: Id("native-role-only".into()),
    };
    for i in [6, 8] {
        set_test_role(&mut snapshot.nodes[i], "BUTTON");
    }
    for (i, x, y, w, h) in [
        (0, 0., 0., 200., 120.),
        (2, 8., 8., 184., 16.),
        (4, 8., 30., 184., 80.),
        (6, 8., 30., 90., 30.),
        (8, 102., 30., 90., 30.),
    ] {
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[i].properties[2]
        else {
            panic!()
        };
        g.shape = Shape::Rect(Rect {
            x,
            y,
            width: w,
            height: h,
        });
    }
    snapshot.nodes[0].children = [2, 4].map(|i| snapshot.nodes[i].key.clone()).to_vec();
    snapshot.nodes[4].children = [6, 8].map(|i| snapshot.nodes[i].key.clone()).to_vec();
    let original = serde_json::to_vec(&b).unwrap();
    let mut l = limits();
    l.components_per_detail = 256;
    let p = compile(&b, l).unwrap();
    let text = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert_eq!(text.matches("LOCAL INSET DETAIL").count(), 1);
    let tail = text.split("LOCAL INSET DETAIL").nth(1).unwrap();
    assert!(
        tail.contains("dialog Left boundary")
            && tail.contains("container Left boundary")
            && tail.contains("8 CSS px")
    );
    let dims = data(&p, "dimensions.json");
    assert!(
        dims[0]["dimensions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["label"] == "Measured left inset"
                && d["value"] == 8.
                && d["anchors"][0]["component"] == "N000"
                && d["anchors"][1]["component"] == "N002")
    );
    assert_eq!(serde_json::to_vec(&b).unwrap(), original);
    let mut exact = l;
    exact.max_output_bytes = p.files().values().map(Vec::len).sum();
    assert!(compile(&b, exact).is_ok());
    exact.max_output_bytes -= 1;
    assert_eq!(compile(&b, exact).unwrap_err(), ExportError::OutputLimit);
    for variant in ["unknown", "space", "not-dialog", "not-children"] {
        let mut b = b.clone();
        let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
            panic!()
        };
        match variant {
            "unknown" | "space" => {
                for i in [2, 4] {
                    let Property::Requested { state, .. } = &mut snapshot.nodes[i].properties[2]
                    else {
                        panic!()
                    };
                    if variant == "unknown" {
                        *state = Availability::Unknown {
                            reason: Id("not-collected".into()),
                        };
                    } else {
                        let Availability::Known {
                            value: Value::Geometry(g),
                        } = state
                        else {
                            panic!()
                        };
                        g.coordinate_space.id = Id("unrelated".into());
                    }
                }
            }
            "not-dialog" => {
                set_test_role(&mut snapshot.nodes[0], "DIV");
                let Property::Requested { state, .. } = &mut snapshot.nodes[0].properties[1] else {
                    panic!()
                };
                *state = Availability::Known {
                    value: Value::Text("Popup capture region".into()),
                };
            }
            "not-children" => snapshot.nodes[0].children.clear(),
            _ => unreachable!(),
        }
        let p = compile(&b, l).unwrap();
        assert!(
            !std::str::from_utf8(&p.files()["prompt.txt"])
                .unwrap()
                .contains("LOCAL INSET DETAIL"),
            "{variant}"
        );
    }
}

#[test]
fn editable_combo_options_and_independent_same_size_controls_get_useful_geometry() {
    let mut b = fixture("observed");
    let SourceInput::Observed {
        snapshot,
        public_text_fields,
    } = &mut b.views[0].source
    else {
        panic!()
    };
    public_text_fields.push(Field::InputKind);
    for node in &mut snapshot.nodes {
        set_test_role(node, "DIV");
        node.children.clear();
    }
    for (i, native) in [
        (4, "dialog"),
        (8, "combobox"),
        (10, "listbox"),
        (12, "option"),
        (14, "option"),
    ] {
        snapshot.nodes[i].native_role = Availability::Known {
            value: Value::Text(native.into()),
        };
        let Property::Requested { state, .. } = &mut snapshot.nodes[i].properties[0] else {
            panic!()
        };
        *state = Availability::Unknown {
            reason: Id("native-role-only".into()),
        };
    }
    set_test_role(&mut snapshot.nodes[2], "BUTTON");
    set_test_role(&mut snapshot.nodes[18], "BUTTON");
    for (i, x, y, w, h) in [
        (0, 0., 0., 100., 20.),
        (2, 0., 0., 100., 20.),
        (4, 0., 26., 140., 120.),
        (6, 8., 46., 124., 80.),
        (8, 8., 46., 124., 24.),
        (10, 8., 76., 124., 44.),
        (12, 8., 76., 124., 20.),
        (14, 8., 100., 124., 20.),
        (16, 200., 0., 124., 20.),
        (18, 200., 0., 124., 20.),
    ] {
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[i].properties[2]
        else {
            panic!()
        };
        g.shape = Shape::Rect(Rect {
            x,
            y,
            width: w,
            height: h,
        });
    }
    for (parent, children) in [
        (0, vec![2, 4]),
        (4, vec![6]),
        (6, vec![8, 10]),
        (10, vec![12, 14]),
        (16, vec![18]),
    ] {
        snapshot.nodes[parent].children = children
            .iter()
            .map(|&i| snapshot.nodes[i].key.clone())
            .collect();
    }
    for (i, node) in snapshot.nodes.iter_mut().enumerate() {
        let mut p = node.properties[0].clone();
        let Property::Requested { field, state, .. } = &mut p else {
            panic!()
        };
        *field = Field::InputKind;
        *state = if i == 8 {
            Availability::Known {
                value: Value::Text("text".into()),
            }
        } else {
            Availability::Unknown {
                reason: Id("not-collected".into()),
            }
        };
        node.properties.push(p);
    }
    snapshot.context.fields.push(Field::InputKind);
    snapshot.coverage.fields.push(Field::InputKind);
    for o in &mut snapshot.observations {
        o.coverage.fields.push(Field::InputKind);
    }
    let mut l = limits();
    l.components_per_detail = 256;
    let source = serde_json::to_vec(&b).unwrap();
    let p = compile(&b, l).unwrap();
    let dims = data(&p, "dimensions.json");
    let all = dims[0]["dimensions"].as_array().unwrap();
    for (a, b, label, value) in [
        ("N004", "N008", "Measured left inset", 8.),
        ("N008", "N012", "Input to first control gap", 6.),
    ] {
        assert!(all.iter().any(|d| d["anchors"][0]["component"] == a
            && d["anchors"][1]["component"] == b
            && d["label"] == label
            && d["value"] == value
            && !d["evidence"].as_array().unwrap().is_empty()));
    }
    let text = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert_eq!(text.matches("LOCAL INSET DETAIL").count(), 1);
    assert!(
        text.split("LOCAL INSET DETAIL")
            .nth(1)
            .unwrap()
            .contains("select, item 1 in group 3 Left boundary")
    );
    assert!(text.contains("WIDTH 140 CSS px BELOW this measured source frame; HEIGHT 120 CSS px"));
    assert!(text.contains("WIDTH 124 CSS px ABOVE; HEIGHT 20 CSS px"));
    assert!(
        text.contains("Input to first") || text.contains("option, item 1 in group 4 Top boundary")
    );
    assert_eq!(serde_json::to_vec(&b).unwrap(), source);
    let mut exact = l;
    exact.max_output_bytes = p.files().values().map(Vec::len).sum();
    assert!(compile(&b, exact).is_ok());
    exact.max_output_bytes -= 1;
    assert_eq!(compile(&b, exact).unwrap_err(), ExportError::OutputLimit);
    let mut withheld = b.clone();
    let SourceInput::Observed {
        public_text_fields, ..
    } = &mut withheld.views[0].source
    else {
        panic!()
    };
    public_text_fields.retain(|f| *f != Field::InputKind);
    let withheld = compile(&withheld, l).unwrap();
    assert!(
        !data(&withheld, "dimensions.json")[0]["dimensions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["label"] == "Input to first control gap")
    );
    // Independent same-size buttons in separate parents need their own pair.
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    for i in [12, 14] {
        set_test_role(&mut snapshot.nodes[i], "BUTTON");
    }
    let p = compile(&b, l).unwrap();
    let text = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert_eq!(
        text.matches("WIDTH 124 CSS px ABOVE; HEIGHT 20 CSS px")
            .count(),
        2
    );
    // Incompatible and unavailable input geometry creates no derived popup facts.
    for unknown in [false, true] {
        let mut invalid = b.clone();
        let SourceInput::Observed { snapshot, .. } = &mut invalid.views[0].source else {
            panic!()
        };
        for i in [12, 14] {
            snapshot.nodes[i].native_role = Availability::Known {
                value: Value::Text("option".into()),
            };
            let Property::Requested { state, .. } = &mut snapshot.nodes[i].properties[0] else {
                panic!()
            };
            *state = Availability::Unknown {
                reason: Id("native-role-only".into()),
            };
        }
        let Property::Requested { state, .. } = &mut snapshot.nodes[8].properties[2] else {
            panic!()
        };
        if unknown {
            *state = Availability::Unknown {
                reason: Id("missing".into()),
            };
        } else {
            let Availability::Known {
                value: Value::Geometry(g),
            } = state
            else {
                panic!()
            };
            g.coordinate_space.id = Id("different".into());
        }
        let p = compile(&invalid, l).unwrap();
        assert!(
            !data(&p, "dimensions.json")[0]["dimensions"]
                .as_array()
                .unwrap()
                .iter()
                .any(|d| d["label"] == "Input to first control gap")
        );
    }
}

#[test]
fn ax_only_known_frames_and_canonical_button_keep_true_attribution() {
    let mut brief = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!()
    };
    snapshot.nodes[0].native_role = Availability::Known {
        value: Value::Text("AXButton".into()),
    };
    for fields in std::iter::once(&mut snapshot.context.fields)
        .chain(std::iter::once(&mut snapshot.coverage.fields))
        .chain(
            snapshot
                .observations
                .iter_mut()
                .map(|o| &mut o.coverage.fields),
        )
    {
        for field in fields {
            if *field == Field::LayoutBounds {
                *field = Field::AccessibilityBounds;
            }
        }
    }
    for node in &mut snapshot.nodes {
        for p in &mut node.properties {
            if let Property::Requested { field, state, .. } = p
                && *field == Field::LayoutBounds
            {
                *field = Field::AccessibilityBounds;
                if let Availability::Known {
                    value: Value::Geometry(g),
                } = state
                {
                    g.frame_kind = FrameKind::AccessibilityBounds;
                }
            }
        }
    }
    let original = serde_json::to_vec(&brief).unwrap();
    let package = compile(&brief, limits()).unwrap();
    assert_eq!(serde_json::to_vec(&brief).unwrap(), original);
    let prompt = std::str::from_utf8(&package.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("Object button: Button"));
    assert!(prompt.contains("window-sized semantic Play/Search region"));
    assert!(prompt.contains("Measurement context 1: AccessibilityBounds;"));
    assert!(
        prompt.contains("AX FRAME MEASUREMENT (not painted UI), EXACT OUTSIDE CAPTION \"button\"")
    );
    assert_eq!(
        prompt
            .matches("AX coordinates are measurement-only")
            .count(),
        1
    );
    let records = prompt
        .lines()
        .filter(|line| line.starts_with("Object ") && line.contains("box[x"))
        .collect::<Vec<_>>();
    assert!(!records.is_empty());
    assert!(
        records
            .iter()
            .all(|line| line.contains("; M") && !line.contains("; P"))
    );
    assert!(!prompt.contains("AX measurement only, not artwork placement; measurement"));
    assert!(prompt.contains("NO text,\nicon, button, word, glyph or UI reconstruction inside it"));
    assert!(!prompt.contains("BELOW this measured source frame"));
    assert!(!prompt.contains("enclosing outline BELOW"));
    assert!(!prompt.contains("LayoutBounds"));
    let dims = data(&package, "dimensions.json");
    assert_eq!(dims[0]["dimensions"][0]["value"], 97.296875);
    assert_eq!(
        dims[0]["dimensions"][0]["anchors"][0]["frame_kind"],
        "accessibility_bounds"
    );
    for label in ["x", "?", "ID", "a \"quoted\" label"] {
        let mut named = brief.clone();
        let SourceInput::Observed { snapshot, .. } = &mut named.views[0].source else {
            panic!()
        };
        let Property::Requested { state, .. } = &mut snapshot.nodes[0].properties[1] else {
            panic!()
        };
        *state = Availability::Known {
            value: Value::Text(label.into()),
        };
        let p = compile(&named, limits()).unwrap();
        let text = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
        let caption = serde_json::to_string(&format!("button {label:?}")).unwrap();
        assert!(text.contains(&format!("EXACT OUTSIDE CAPTION {caption}: WIDTH")));
        for line in text.lines().filter(|line| {
            line.starts_with("AX FRAME MEASUREMENT (") || line.starts_with("AX FRAME EXTENT (")
        }) {
            assert!(line.contains("EXACT OUTSIDE CAPTION \""));
        }
        let mut exact = limits();
        exact.max_output_bytes = p.files().values().map(Vec::len).sum();
        assert!(compile(&named, exact).is_ok());
        exact.max_output_bytes -= 1;
        assert_eq!(
            compile(&named, exact).unwrap_err(),
            ExportError::OutputLimit
        );
    }
    let mut private = brief.clone();
    let SourceInput::Observed {
        snapshot,
        public_text_fields,
    } = &mut private.views[0].source
    else {
        panic!()
    };
    public_text_fields.clear();
    let Property::Requested { state, .. } = &mut snapshot.nodes[0].properties[1] else {
        panic!()
    };
    *state = Availability::Known {
        value: Value::Text("CAPTION_PRIVATE_CANARY".into()),
    };
    let p = compile(&private, limits()).unwrap();
    assert!(
        p.files()
            .values()
            .all(|bytes| !String::from_utf8_lossy(bytes).contains("CAPTION_PRIVATE_CANARY"))
    );
    assert!(
        std::str::from_utf8(&p.files()["prompt.txt"])
            .unwrap()
            .contains("EXACT OUTSIDE CAPTION \"button\": WIDTH")
    );
}

#[test]
fn mixed_frames_choose_reported_layout_without_aliasing_other_known_frames() {
    let mut brief = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!()
    };
    snapshot.nodes[0].native_role = Availability::Known {
        value: Value::Text("DIV".into()),
    };
    for node in &mut snapshot.nodes {
        let mut ax = node.properties[2].clone();
        let Property::Requested { field, state, .. } = &mut ax else {
            panic!()
        };
        *field = Field::AccessibilityBounds;
        if let Availability::Known {
            value: Value::Geometry(g),
        } = state
        {
            g.frame_kind = FrameKind::AccessibilityBounds;
            let Shape::Rect(r) = &mut g.shape else {
                panic!()
            };
            r.width = 201.5;
        }
        node.properties.push(ax);
    }
    snapshot.context.fields.push(Field::AccessibilityBounds);
    snapshot.coverage.fields.push(Field::AccessibilityBounds);
    for o in &mut snapshot.observations {
        o.coverage.fields.push(Field::AccessibilityBounds);
    }
    let package = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&package.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("Object button: Button"));
    assert!(prompt.contains("Placement context 1: LayoutBounds;"));
    assert!(prompt.contains("DIMENSIONED RECTANGLE button"));
    assert!(!prompt.contains("REQUIRED Width AccessibilityBounds"));
    let dims = data(&package, "dimensions.json");
    assert!(
        dims[0]["dimensions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["label"] == "Width AccessibilityBounds" && d["value"] == 201.5)
    );
    // A missing canonical role may use the existing reported HTML-native fallback.
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!()
    };
    let Property::Requested { state, .. } = &mut snapshot.nodes[0].properties[0] else {
        panic!()
    };
    *state = Availability::Unknown {
        reason: Id("test-role-not-exposed".into()),
    };
    snapshot.nodes[0].native_role = Availability::Known {
        value: Value::Text("BUTTON".into()),
    };
    let package = compile(&brief, limits()).unwrap();
    assert!(
        std::str::from_utf8(&package.files()["prompt.txt"])
            .unwrap()
            .contains("Object button: BUTTON [#")
    );
}

#[test]
fn non_input_source_never_gets_search_or_inset_recipes() {
    let mut brief = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!()
    };
    for node in &mut snapshot.nodes {
        set_test_role(node, "BUTTON");
    }
    let package = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&package.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("VIEW PLAN"));
    assert!(!prompt.contains("Input and adjacent controls — partial spacing"));
    assert!(!prompt.contains("INSET DETAIL"));
    assert!(!prompt.contains("Search and result spacing"));
    assert!(!prompt.contains("If a blueprint reference"));
    assert!(!prompt.contains("REPLACE the conflicting"));
}

#[test]
fn public_placeholder_and_clipped_region_are_distinct_from_full_layout_and_value() {
    let mut brief = fixture("observed");
    brief.views[0].state = "Input empty; no value entered".into();
    let SourceInput::Observed {
        snapshot,
        public_text_fields,
    } = &mut brief.views[0].source
    else {
        panic!()
    };
    set_test_role(&mut snapshot.nodes[0], "INPUT");
    public_text_fields.push(Field::Placeholder);
    for node in &mut snapshot.nodes {
        let mut p = node.properties[1].clone();
        let Property::Requested { field, state, .. } = &mut p else {
            panic!()
        };
        *field = Field::Placeholder;
        *state = Availability::Known {
            value: Value::Text("Type a query".into()),
        };
        node.properties.push(p);
    }
    snapshot.context.fields.push(Field::Placeholder);
    snapshot.coverage.fields.push(Field::Placeholder);
    for o in &mut snapshot.observations {
        o.coverage.fields.push(Field::Placeholder);
    }
    let mut visible = snapshot.nodes[0].properties[2].clone();
    let Property::Requested {
        field,
        state,
        evidence,
        ..
    } = &mut visible
    else {
        panic!()
    };
    *field = Field::VisibleRegion;
    evidence.provenance = Provenance::Derived;
    let Availability::Known {
        value: Value::Geometry(g),
    } = state
    else {
        panic!()
    };
    g.frame_kind = FrameKind::VisibleRegion;
    let Shape::Rect(r) = &mut g.shape else {
        panic!()
    };
    r.height = 11.0;
    snapshot.nodes[0].properties[4] = visible;
    let package = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&package.files()["prompt.txt"]).unwrap();
    let input = prompt
        .lines()
        .find(|line| line.starts_with("Object input: Textbox"))
        .unwrap();
    assert!(input.contains("Placeholder=\"Type a query\""));
    assert!(
        input.contains("CLIPPED LAYOUT: use COMPONENT CLIP") && input.contains("COMPONENT CLIP #1")
    );
    assert!(input.contains("h 32"));
    assert!(prompt.contains("visible rect[") && prompt.contains("h 11] CSS px"));
    assert!(
        !prompt
            .lines()
            .filter(|s| s.starts_with("DIMENSIONED RECTANGLE"))
            .any(|s| s.contains("input:") && s.contains("HEIGHT 32 CSS px"))
    );
    let scene = data(&package, "scene.json");
    assert_eq!(
        scene["views"][0]["components"][0]["properties"][2]["state"]["value"]["value"]["shape"]["value"]
            ["height"],
        32.0
    );
    assert_eq!(
        scene["views"][0]["components"][0]["properties"][4]["state"]["value"]["value"]["shape"]["value"]
            ["height"],
        11.0
    );
}

#[test]
fn control_detail_does_not_merge_equal_sizes_from_unrelated_spaces() {
    let mut brief = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!()
    };
    set_test_role(&mut snapshot.nodes[0], "DIV");
    for i in [2, 4] {
        set_test_role(&mut snapshot.nodes[i], "SELECT");
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[i].properties[2]
        else {
            panic!()
        };
        g.coordinate_space.id = Id(format!("independent-space-{i}"));
        let Shape::Rect(r) = &mut g.shape else {
            panic!()
        };
        *r = Rect {
            x: 10.0,
            y: 10.0,
            width: 80.0,
            height: 30.0,
        };
    }
    snapshot.nodes[0].children = [2, 4].map(|i| snapshot.nodes[i].key.clone()).to_vec();
    let package = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&package.files()["prompt.txt"]).unwrap();
    assert!(
        !prompt
            .lines()
            .filter(|s| s.starts_with("DETAIL"))
            .any(|s| s.contains("controls [3, 5]"))
    );
    assert_eq!(
        prompt
            .lines()
            .filter(|s| s.starts_with("DIMENSIONED RECTANGLE select")
                && s.contains("WIDTH 80 CSS px")
                && s.contains("HEIGHT 30 CSS px"))
            .count(),
        2
    );
}

#[test]
fn clipping_requires_proved_layout_subset_and_preserves_other_frame_sizes() {
    for (field, kind, variant, clipped) in [
        (Field::LayoutBounds, FrameKind::LayoutBounds, "subset", true),
        (Field::LayoutBounds, FrameKind::LayoutBounds, "equal", false),
        (
            Field::LayoutBounds,
            FrameKind::LayoutBounds,
            "overlap",
            false,
        ),
        (Field::LayoutBounds, FrameKind::LayoutBounds, "space", false),
        (
            Field::LayoutBounds,
            FrameKind::LayoutBounds,
            "unknown",
            false,
        ),
        (
            Field::AccessibilityBounds,
            FrameKind::AccessibilityBounds,
            "subset",
            false,
        ),
        (Field::HitRegion, FrameKind::HitRegion, "subset", false),
        (Field::PaintBounds, FrameKind::PaintBounds, "subset", false),
    ] {
        let mut brief = fixture("observed");
        let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
            panic!()
        };
        for fields in std::iter::once(&mut snapshot.context.fields)
            .chain(std::iter::once(&mut snapshot.coverage.fields))
            .chain(
                snapshot
                    .observations
                    .iter_mut()
                    .map(|o| &mut o.coverage.fields),
            )
        {
            for f in fields {
                if *f == Field::LayoutBounds {
                    *f = field;
                } else if *f == field {
                    *f = Field::PaintBounds;
                }
            }
        }
        for node in &mut snapshot.nodes {
            if field == Field::HitRegion {
                let Property::Requested { field: f, .. } = &mut node.properties[3] else {
                    panic!()
                };
                *f = Field::PaintBounds;
            }
            for p in &mut node.properties {
                if let Property::Requested {
                    field: f, state, ..
                } = p
                    && *f == Field::LayoutBounds
                {
                    *f = field;
                    if let Availability::Known {
                        value: Value::Geometry(g),
                    } = state
                    {
                        g.frame_kind = kind;
                    }
                }
            }
        }
        let mut visible = snapshot.nodes[0].properties[2].clone();
        let Property::Requested {
            field: f, state, ..
        } = &mut visible
        else {
            panic!()
        };
        *f = Field::VisibleRegion;
        let Availability::Known {
            value: Value::Geometry(g),
        } = state
        else {
            panic!()
        };
        g.frame_kind = FrameKind::VisibleRegion;
        let Shape::Rect(r) = &mut g.shape else {
            panic!()
        };
        if variant != "equal" {
            r.height = if kind == FrameKind::PaintBounds {
                20.0
            } else {
                11.0
            };
        }
        if variant == "overlap" {
            r.x -= 10.0;
        }
        if variant == "space" {
            g.coordinate_space.id = Id("separate-visible-space".into());
        }
        if variant == "unknown" {
            *state = Availability::Unknown {
                reason: Id("not-collected".into()),
            };
        }
        snapshot.nodes[0].properties[4] = visible;
        let original = serde_json::to_vec(&brief).unwrap();
        let p = compile(&brief, limits()).unwrap_or_else(|e| panic!("{kind:?}/{variant}: {e:?}"));
        assert_eq!(serde_json::to_vec(&brief).unwrap(), original);
        let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
        let object = prompt
            .lines()
            .find(|s| s.starts_with("Object button:"))
            .unwrap();
        assert_eq!(
            object.contains("CLIPPED LAYOUT"),
            clipped,
            "{kind:?}/{variant}"
        );
        assert!(!prompt.contains("in context 0"));
        if !clipped {
            assert!(
                prompt.lines().any(
                    |s| s.starts_with(if kind == FrameKind::AccessibilityBounds {
                        "AX FRAME MEASUREMENT (not painted UI), EXACT OUTSIDE CAPTION \"button\":"
                    } else {
                        "DIMENSIONED RECTANGLE button:"
                    }) && s.contains(if kind == FrameKind::PaintBounds {
                        "HEIGHT 20 CSS px"
                    } else {
                        "HEIGHT 32 CSS px"
                    })
                ),
                "{kind:?}/{variant}"
            );
        }
        if !clipped && kind != FrameKind::PaintBounds && !matches!(variant, "equal" | "unknown") {
            assert!(object.contains("DISTINCT FRAME (clipping not established)"));
        }
    }
}

#[test]
fn clipping_is_component_scoped_directional_and_sheet_scoped() {
    let mut b = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    for (index, top) in [(0, true), (2, false)] {
        let mut visible = snapshot.nodes[index].properties[2].clone();
        let Property::Requested {
            field,
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut visible
        else {
            panic!()
        };
        *field = Field::VisibleRegion;
        g.frame_kind = FrameKind::VisibleRegion;
        let Shape::Rect(r) = &mut g.shape else {
            panic!()
        };
        if top {
            r.y += 4.0;
            r.height -= 4.0;
        } else {
            r.width -= 4.0;
        }
        snapshot.nodes[index].properties[4] = visible;
    }
    let source = serde_json::to_vec(&b).unwrap();
    let p = compile(&b, limits()).unwrap();
    for file in ["prompt.txt", "prompt-G01.txt"] {
        let text = std::str::from_utf8(&p.files()[file]).unwrap();
        let clips = text
            .lines()
            .filter(|l| l.starts_with("COMPONENT CLIP"))
            .collect::<Vec<_>>();
        assert_eq!(clips.len(), 2, "{file}");
        assert!(clips[0].starts_with("COMPONENT CLIP #1"));
        assert!(clips[0].contains("known clipped edges [Top]"));
        assert!(clips[1].starts_with("COMPONENT CLIP #3"));
        assert!(clips[1].contains("known clipped edges [Right]"));
        assert!(clips.iter().all(|l| !l.contains("crop of the artwork")));
    }
    for file in ["prompt-D01.txt", "prompt-D02.txt", "prompt-D03.txt"] {
        assert!(
            !std::str::from_utf8(&p.files()[file])
                .unwrap()
                .lines()
                .any(|line| line.starts_with("COMPONENT CLIP #"))
        );
    }
    assert_eq!(serde_json::to_vec(&b).unwrap(), source);
}

#[test]
fn clipped_empty_dimension_details_request_source_artwork_consistently() {
    let mut b = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    for index in [0, 2] {
        let mut visible = snapshot.nodes[index].properties[2].clone();
        let Property::Requested {
            field,
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut visible
        else {
            panic!()
        };
        *field = Field::VisibleRegion;
        g.frame_kind = FrameKind::VisibleRegion;
        let Shape::Rect(r) = &mut g.shape else {
            panic!()
        };
        r.y += 4.0;
        r.height -= 4.0;
        snapshot.nodes[index].properties[4] = visible;
    }
    b.details = [0, 2]
        .into_iter()
        .map(|i| DetailRequest {
            view: b.views[0].id.clone(),
            components: vec![format!("N{i:03}")],
            title: format!("Clipped source {i}"),
        })
        .collect();
    let mut l = limits();
    l.components_per_detail = 256;
    let p = compile(&b, l).unwrap();
    let m = data(&p, "manifest.json");
    for sheet in m["sheet_prompts"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|s| s["sheet_id"] != "G01")
    {
        assert!(sheet["dimension_ids"].as_array().unwrap().is_empty());
        let prompt = std::str::from_utf8(&p.files()[sheet["file"].as_str().unwrap()]).unwrap();
        assert!(prompt.contains("Selected source detail only; no prepared measurements"));
        assert!(prompt.contains("VIEW PLAN: selected source-detail artwork only"));
        assert!(prompt.contains("SOURCE FRAGMENT ONLY"));
        assert!(prompt.contains("INCLUSION BOUNDARY"));
        assert!(!prompt.contains("MEASUREMENT ONLY"));
        assert!(!prompt.contains("ONLY this detail sheet's assigned measurement blocks"));
        assert!(prompt.contains("known clipped edges [Top]"));
    }
}

#[test]
fn ax_source_fragment_uses_tree_membership_without_construction_coordinates() {
    let mut b = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    for node in &mut snapshot.nodes {
        set_test_role(node, "DIV");
        node.children.clear();
        for property in &mut node.properties {
            if let Property::Requested { field, state, .. } = property
                && *field == Field::LayoutBounds
            {
                *field = Field::AccessibilityBounds;
                if let Availability::Known {
                    value: Value::Geometry(g),
                } = state
                {
                    g.frame_kind = FrameKind::AccessibilityBounds;
                }
            }
        }
    }
    for fields in std::iter::once(&mut snapshot.context.fields)
        .chain(std::iter::once(&mut snapshot.coverage.fields))
        .chain(
            snapshot
                .observations
                .iter_mut()
                .map(|o| &mut o.coverage.fields),
        )
    {
        for field in fields.iter_mut() {
            if *field == Field::LayoutBounds {
                *field = Field::AccessibilityBounds;
            }
        }
        fields.push(Field::Enabled);
    }
    for (i, name) in [
        (0, "Fragment parent"),
        (2, "Included label"),
        (6, "Excluded parent"),
    ] {
        let Property::Requested { state, .. } = &mut snapshot.nodes[i].properties[1] else {
            panic!()
        };
        *state = Availability::Known {
            value: Value::Text(name.into()),
        };
    }
    set_test_role(&mut snapshot.nodes[4], "LABEL");
    set_test_role(&mut snapshot.nodes[2], "LABEL");
    let children = vec![snapshot.nodes[2].key.clone(), snapshot.nodes[4].key.clone()];
    snapshot.nodes[0].children = children;
    for (i, node) in snapshot.nodes.iter_mut().enumerate() {
        let mut flag = node.properties[0].clone();
        let Property::Requested { field, state, .. } = &mut flag else {
            panic!()
        };
        *field = Field::Enabled;
        *state = if i == 2 {
            Availability::Known {
                value: Value::Flag(false),
            }
        } else {
            Availability::Unknown {
                reason: Id("not-collected".into()),
            }
        };
        node.properties.push(flag);
    }
    b.details = vec![DetailRequest {
        view: b.views[0].id.clone(),
        components: vec!["N002".into()],
        title: "Source fragment".into(),
    }];
    let original = serde_json::to_vec(&b).unwrap();
    let mut l = limits();
    l.components_per_detail = 256;
    let p = compile(&b, l).unwrap();
    let text = std::str::from_utf8(&p.files()["prompt-D01.txt"]).unwrap();
    assert!(text.contains("SOURCE FRAGMENT MEMBERS"));
    assert!(text.contains("- Source member label layout \"Included label\""));
    assert!(text.contains("container \"Fragment parent\""));
    assert!(text.contains("source child 1 of 2"));
    assert!(text.contains("Enabled=false"));
    for excluded in ["box[x", "[#", "Measurement context", "Excluded parent"] {
        assert!(!text.contains(excluded), "{excluded}");
    }
    assert_eq!(
        data(&p, "manifest.json")["sheet_prompts"][1]["component_ids"],
        serde_json::json!(["N002"])
    );
    assert!(
        data(&p, "manifest.json")["sheet_prompts"][1]["dimension_ids"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(serde_json::to_vec(&b).unwrap(), original);
    assert!(
        data(&p, "scene.json")["views"][0]["components"][2]["properties"]
            .to_string()
            .contains("accessibility_bounds")
    );
    assert!(text.contains("structural orientation only, not proved visual correspondence"));
    assert!(!text.contains("identifies the item in the reference"));
    // Repeated ancestor labels are not identities. Exercise descent, siblings,
    // ascent and a distinct root with the same name, retaining every source member.
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    for node in &mut snapshot.nodes {
        node.children.clear();
        set_test_role(node, "DIV");
    }
    for (i, name) in [
        (0, "Root"),
        (16, "Root"),
        (2, "Same parent"),
        (4, "Same parent"),
    ] {
        let Property::Requested { state, .. } = &mut snapshot.nodes[i].properties[1] else {
            panic!()
        };
        *state = Availability::Known {
            value: Value::Text(name.into()),
        };
    }
    for (parent, children) in [
        (0, vec![2, 4, 12]),
        (2, vec![6]),
        (4, vec![8]),
        (6, vec![10]),
        (16, vec![14]),
    ] {
        snapshot.nodes[parent].children = children
            .iter()
            .map(|&i| snapshot.nodes[i].key.clone())
            .collect();
    }
    b.details[0].components = [2, 4, 6, 8, 10, 12, 14]
        .into_iter()
        .map(|i| format!("N{i:03}"))
        .collect();
    let p = compile(&b, l).unwrap();
    let text = std::str::from_utf8(&p.files()["prompt-D01.txt"]).unwrap();
    assert_eq!(
        text.lines()
            .filter(|l| l.starts_with("- Source member "))
            .count(),
        7
    );
    assert_eq!(
        text.matches("Source hierarchy (orientation only): container \"Root\".")
            .count(),
        2
    );
    assert!(text.contains("within previous context → container \"Same parent\" (child 1 of 3)"));
    assert!(text.contains(
        "keep first 1 level of previous context → container \"Same parent\" (child 2 of 3)"
    ));
    assert!(text.contains("return to first 1 level of previous context."));
    // Source-only AX arrangement uses exact same-space bounds, without requiring
    // or claiming an AX-to-image transform. Non-AX artwork remains unchanged.
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    for node in &mut snapshot.nodes {
        node.children.clear();
    }
    for (i, x, y) in [(2, 100.0, 10.0), (4, 0.0, 30.0), (6, 100.0, 30.0)] {
        set_test_role(&mut snapshot.nodes[i], "LABEL");
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[i].properties[2]
        else {
            panic!()
        };
        g.transform = TransformState::Unknown {
            reason: Id("no-image-mapping".into()),
        };
        let Shape::Rect(rect) = &mut g.shape else {
            panic!()
        };
        rect.x = x;
        rect.y = y;
        rect.width = 20.0;
        rect.height = 10.0;
    }
    b.details[0].components = [2, 4, 6].into_iter().map(|i| format!("N{i:03}")).collect();
    let original = serde_json::to_vec(&b).unwrap();
    let p = compile(&b, l).unwrap();
    let text = std::str::from_utf8(&p.files()["prompt-D01.txt"]).unwrap();
    assert!(text.contains("preceding source member: below; left"));
    assert!(text.contains("preceding source member: vertical spans overlap; right"));
    assert!(text.contains("transform unknown; same Surface and Observation"));
    assert!(!text.contains("WIDTH") && !text.contains("box[x"));
    assert_eq!(serde_json::to_vec(&b).unwrap(), original);
    let mut exact = l;
    exact.max_output_bytes = p.files().values().map(Vec::len).sum();
    assert!(compile(&b, exact).is_ok());
    exact.max_output_bytes -= 1;
    assert_eq!(compile(&b, exact).unwrap_err(), ExportError::OutputLimit);
    for mismatch in ["space", "units", "transform", "unknown"] {
        let mut variant = b.clone();
        let SourceInput::Observed { snapshot, .. } = &mut variant.views[0].source else {
            panic!()
        };
        let Property::Requested { state, .. } = &mut snapshot.nodes[4].properties[2] else {
            panic!()
        };
        if mismatch == "unknown" {
            *state = Availability::Unknown {
                reason: Id("not-collected".into()),
            };
        } else {
            let Availability::Known {
                value: Value::Geometry(g),
            } = state
            else {
                panic!()
            };
            match mismatch {
                "space" => g.coordinate_space.id = Id("unrelated-space".into()),
                "units" => g.coordinate_space.units = Unit::Pt,
                "transform" => g.transform = TransformState::LocalOnly {},
                _ => unreachable!(),
            }
        }
        let p = compile(&variant, l).unwrap();
        let text = std::str::from_utf8(&p.files()["prompt-D01.txt"]).unwrap();
        assert!(!text.contains("Unprinted AX-frame order"), "{mismatch}");
    }
}

#[test]
fn fragment_draft_rules_follow_input_membership_without_parsing_caller_state() {
    let mut b = fixture("observed");
    b.views[0].state = "Public draft orchard for the observed input; no action performed".into();
    let original_state = b.views[0].state.clone();
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    for n in &mut snapshot.nodes {
        set_test_role(n, "DIV");
        n.children.clear();
    }
    set_test_role(&mut snapshot.nodes[0], "INPUT");
    set_test_role(&mut snapshot.nodes[2], "LABEL");
    for index in [0, 2] {
        let mut visible = snapshot.nodes[index].properties[2].clone();
        let Property::Requested {
            field,
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut visible
        else {
            panic!()
        };
        *field = Field::VisibleRegion;
        g.frame_kind = FrameKind::VisibleRegion;
        let Shape::Rect(r) = &mut g.shape else {
            panic!()
        };
        r.y += 4.;
        r.height -= 4.;
        snapshot.nodes[index].properties[4] = visible;
    }
    b.details = [0, 2]
        .into_iter()
        .map(|i| DetailRequest {
            view: b.views[0].id.clone(),
            components: vec![format!("N{i:03}")],
            title: format!("Fragment {i}"),
        })
        .collect();
    let before = serde_json::to_vec(&b).unwrap();
    let mut l = limits();
    l.components_per_detail = 256;
    let p = compile(&b, l).unwrap();
    let included = std::str::from_utf8(&p.files()["prompt-D01.txt"]).unwrap();
    let excluded = std::str::from_utf8(&p.files()["prompt-D02.txt"]).unwrap();
    assert!(included.contains("Source member input"));
    assert!(included.contains("COMPONENT CLIP #1") && included.contains("clipping reference [#1]"));
    assert!(excluded.contains("COMPONENT CLIP #3") && excluded.contains("clipping reference [#3]"));
    assert!(included.contains("caller draft only when explicitly reported"));
    assert!(!included.contains("NON-RENDERING WHOLE-VIEW PROVENANCE"));
    assert!(!excluded.contains("Source member input"));
    assert!(!excluded.contains("caller draft only when explicitly reported"));
    assert!(excluded.contains("NON-RENDERING WHOLE-VIEW PROVENANCE"));
    assert!(excluded.contains("Caller-draft rendering does not apply"));
    assert!(included.contains("orchard") && excluded.contains("orchard"));
    assert_eq!(data(&p, "scene.json")["views"][0]["state"], original_state);
    assert_eq!(
        data(&p, "manifest.json")["sheet_prompts"][1]["component_ids"],
        serde_json::json!(["N000"])
    );
    assert_eq!(
        data(&p, "manifest.json")["sheet_prompts"][2]["component_ids"],
        serde_json::json!(["N002"])
    );
    assert_eq!(serde_json::to_vec(&b).unwrap(), before);
    // A second potential input outside the fragment makes a prose-only draft
    // binding ambiguous. Preserve it as provenance, never guess a field owner.
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    set_test_role(&mut snapshot.nodes[3], "INPUT");
    let p = compile(&b, l).unwrap();
    assert!(
        std::str::from_utf8(&p.files()["prompt-D01.txt"])
            .unwrap()
            .contains("NON-RENDERING WHOLE-VIEW PROVENANCE")
    );
}

#[test]
fn clipped_document_layout_pair_owns_a_separate_empty_sketch() {
    let mut b = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    for n in &mut snapshot.nodes {
        n.children.clear();
    }
    set_test_role(&mut snapshot.nodes[0], "DIV");
    set_test_role(&mut snapshot.nodes[1], "LABEL");
    snapshot.nodes[0].children = vec![snapshot.nodes[1].key.clone()];
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut snapshot.nodes[0].properties[2]
    else {
        panic!()
    };
    g.shape = Shape::Rect(Rect {
        x: 0.,
        y: 0.,
        width: 800.,
        height: 2307.0703125,
    });
    let mut visible = snapshot.nodes[0].properties[2].clone();
    let Property::Requested {
        field,
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        evidence,
        ..
    } = &mut visible
    else {
        panic!()
    };
    *field = Field::VisibleRegion;
    g.frame_kind = FrameKind::VisibleRegion;
    evidence.provenance = Provenance::Derived;
    let Shape::Rect(r) = &mut g.shape else {
        panic!()
    };
    r.height = 666.5625;
    snapshot.nodes[0].properties[4] = visible;
    let source = serde_json::to_vec(&b).unwrap();
    let p = compile(&b, limits()).unwrap();
    let text = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(
        text.contains("SEPARATE FULL-LAYOUT SKETCH — outer source container (LayoutBounds): EMPTY")
    );
    assert!(text.contains("WIDTH 800 CSS px ABOVE; HEIGHT ≈2307 CSS px LEFT"));
    assert!(text.contains("different frame: Derived VisibleRegion"));
    assert!(text.contains("h ≈667] CSS px"));
    assert_eq!(text.matches("h ≈667] CSS px").count(), 1);
    assert!(text.contains("CLIPPED LAYOUT: use COMPONENT CLIP #1"));
    assert!(!text.contains("666.5625") && !text.contains("2307.0703125"));
    assert!(!text.contains("\"shape\":\"rect\""));
    let mut exact = limits();
    exact.max_output_bytes = p.files().values().map(Vec::len).sum();
    assert!(compile(&b, exact).is_ok());
    exact.max_output_bytes -= 1;
    assert_eq!(compile(&b, exact).unwrap_err(), ExportError::OutputLimit);
    assert!(!text.contains("DIMENSIONED RECTANGLE outer source container:"));
    assert!(!text.contains("outer source container — width 800"));
    let dims = data(&p, "dimensions.json");
    let root = dims[0]["dimensions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["anchors"][0]["component"] == "N000")
        .collect::<Vec<_>>();
    for (kind, value) in [
        ("layout_bounds", 2307.0703125),
        ("visible_region", 666.5625),
    ] {
        assert!(
            root.iter()
                .any(|d| d["anchors"][0]["frame_kind"] == kind && d["value"] == value)
        );
        assert!(
            root.iter()
                .any(|d| d["anchors"][0]["frame_kind"] == kind && d["value"] == 800.)
        );
    }
    assert_eq!(serde_json::to_vec(&b).unwrap(), source);
    for (x, clipped) in [(-0.0_f64, true), (-0.125, false)] {
        let mut fractional = b.clone();
        let SourceInput::Observed { snapshot, .. } = &mut fractional.views[0].source else {
            panic!()
        };
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[0].properties[4]
        else {
            panic!()
        };
        g.shape = Shape::Rect(Rect {
            x,
            y: 0.125,
            width: 0.25,
            height: 0.375,
        });
        let original = serde_json::to_vec(&fractional).unwrap();
        let p = compile(&fractional, limits()).unwrap();
        let text = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
        assert!(!text.contains("-0") && !text.contains("0.125") && !text.contains("0.375"));
        if clipped {
            assert!(text.contains("visible rect[x 0, y <0.5, w <0.5, h <0.5] CSS px"));
            assert!(text.contains("CLIPPED LAYOUT: use COMPONENT CLIP #1"));
        } else {
            assert!(text.contains("DISTINCT FRAME (clipping not established): reported visible-region rectangle[x −<0.5, y <0.5, w <0.5, h <0.5]"));
        }
        let scene = data(&p, "scene.json");
        let raw = &scene["views"][0]["components"][0]["properties"][4]["state"]["value"]["value"]["shape"]
            ["value"];
        assert_eq!(raw["x"].as_f64().unwrap().to_bits(), x.to_bits());
        assert_eq!(raw["height"], 0.375);
        assert_eq!(serde_json::to_vec(&fractional).unwrap(), original);
    }
    // Equal full/visible rectangles do not get the clipped-frame assignment.
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut snapshot.nodes[0].properties[4]
    else {
        panic!()
    };
    let Shape::Rect(r) = &mut g.shape else {
        panic!()
    };
    r.height = 2307.0703125;
    let p = compile(&b, limits()).unwrap();
    let text = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(!text.contains("SEPARATE FULL-LAYOUT SKETCH"));
    assert!(
        text.contains("WIDTH 800 CSS px BELOW this measured source frame; HEIGHT ≈2307 CSS px")
    );
}

#[test]
fn fragment_container_does_not_authorize_unselected_descendants() {
    let mut b = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    set_test_role(&mut snapshot.nodes[0], "DIV");
    snapshot.nodes[0].children = vec![snapshot.nodes[1].key.clone()];
    b.details = vec![DetailRequest {
        view: b.views[0].id.clone(),
        components: vec!["N000".into()],
        title: "Selected boundary".into(),
    }];
    let mut l = limits();
    l.components_per_detail = 256;
    let p = compile(&b, l).unwrap();
    let text = std::str::from_utf8(&p.files()["prompt-D01.txt"]).unwrap();
    assert!(text.contains("SOURCE FRAGMENT ONLY"));
    assert!(text.contains("BOUNDARY CONTEXT: "));
    assert!(!text.contains("Object outer source container"));
    assert_eq!(
        data(&p, "manifest.json")["sheet_prompts"][1]["component_ids"],
        serde_json::json!(["N000"])
    );
}

#[test]
fn object_states_and_context_links_survive_compact_presentation() {
    let mut brief = fixture("observed");
    brief.views[0].state =
        "Page controls disabled; enablement of other controls is not declared".into();
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!()
    };
    for (i, name) in [(0, "First control"), (2, "Second control")] {
        set_test_role(&mut snapshot.nodes[i], "BUTTON");
        let Property::Requested { state, .. } = &mut snapshot.nodes[i].properties[1] else {
            panic!()
        };
        *state = Availability::Known {
            value: Value::Text(name.into()),
        };
    }
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut snapshot.nodes[2].properties[2]
    else {
        panic!()
    };
    g.coordinate_space.id = Id("separate-control-space".into());
    for (i, node) in snapshot.nodes.iter_mut().enumerate() {
        let mut property = node.properties[0].clone();
        let Property::Requested { field, state, .. } = &mut property else {
            panic!()
        };
        *field = Field::Enabled;
        *state = match i {
            0 | 2 => Availability::Known {
                value: Value::Flag(i == 2),
            },
            _ => Availability::Unknown {
                reason: Id("not-collected".into()),
            },
        };
        let mut focus = property.clone();
        let Property::Requested { field, state, .. } = &mut focus else {
            panic!()
        };
        *field = Field::Focused;
        *state = if i <= 2 {
            Availability::Known {
                value: Value::Flag(i == 0),
            }
        } else {
            Availability::Unknown {
                reason: Id("not-collected".into()),
            }
        };
        node.properties.push(property);
        node.properties.push(focus);
    }
    snapshot
        .context
        .fields
        .extend([Field::Enabled, Field::Focused]);
    snapshot
        .coverage
        .fields
        .extend([Field::Enabled, Field::Focused]);
    for o in &mut snapshot.observations {
        o.coverage.fields.extend([Field::Enabled, Field::Focused]);
    }
    let original = serde_json::to_vec(&brief).unwrap();
    let p = compile(&brief, limits()).unwrap();
    assert_eq!(serde_json::to_vec(&brief).unwrap(), original);
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    for (name, context) in [("First control", 1), ("Second control", 3)] {
        let object = prompt
            .lines()
            .find(|s| s.starts_with("Object") && s.contains(name))
            .unwrap();
        assert!(object.contains(if name == "First control" {
            "Enabled=false"
        } else {
            "Enabled=true"
        }));
        assert!(object.contains(&format!("P{context}")));
    }
    assert!(prompt.contains("Placement context 3: LayoutBounds; REF"));
    assert_eq!(
        prompt
            .matches("outside \"Focused\" tag in every view of this object")
            .count(),
        1
    );
    assert!(prompt.lines().any(|l| l.starts_with("Object")
        && l.contains("First control")
        && l.contains("Focused=true; outside")));
    for object in prompt.lines().filter(|s| {
        s.starts_with("Object") && !s.contains("First control") && !s.contains("Second control")
    }) {
        assert!(!object.contains("Enabled=true") && !object.contains("Enabled=false"));
    }
    for bytes in p
        .files()
        .iter()
        .filter(|(n, _)| n.ends_with(".txt"))
        .map(|(_, b)| b)
    {
        let prompt = std::str::from_utf8(bytes).unwrap();
        for target in prompt
            .lines()
            .filter(|s| s.starts_with("Recorded Enabled="))
        {
            assert_eq!(
                target.contains("Enabled=false"),
                target.contains("First control")
            );
            assert_eq!(
                target.contains("Enabled=true"),
                target.contains("Second control")
            );
        }
    }

    // False/unknown/redacted source focus does not acquire a per-target positive
    // instruction, even with explicit caller prose. That prose remains annotation.
    for focus in [
        Availability::Known {
            value: Value::Flag(false),
        },
        Availability::Unknown {
            reason: Id("not-collected".into()),
        },
        Availability::Redacted {},
    ] {
        let mut variant = brief.clone();
        variant.views[0].state =
            "First control is focused, caller-reported; no action performed".into();
        let SourceInput::Observed { snapshot, .. } = &mut variant.views[0].source else {
            panic!()
        };
        let property = snapshot.nodes[0]
            .properties
            .iter_mut()
            .find(|p| p.field() == Field::Focused)
            .unwrap();
        let Property::Requested {
            state, sensitivity, ..
        } = property
        else {
            panic!()
        };
        *state = focus.clone();
        if matches!(focus, Availability::Redacted {}) {
            *sensitivity = Sensitivity::Sensitive;
        }
        let original = serde_json::to_vec(&variant).unwrap();
        let p = compile(&variant, limits()).unwrap();
        let text = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
        assert_eq!(text.matches(&variant.views[0].state).count(), 1);
        let object = text
            .lines()
            .find(|l| l.starts_with("Object") && l.contains("First control"))
            .unwrap();
        assert!(!object.contains("outside \"Focused\" tag"));
        assert_eq!(
            object.contains("Focused=false"),
            matches!(
                focus,
                Availability::Known {
                    value: Value::Flag(false)
                }
            )
        );
        let scene = data(&p, "scene.json");
        let field = scene["views"][0]["components"][0]["properties"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["field"] == "focused")
            .unwrap();
        let expected = serde_json::to_value(&focus).unwrap();
        assert_eq!(field["state"]["availability"], expected["availability"]);
        if matches!(focus, Availability::Known { .. }) {
            assert_eq!(field["state"]["value"], expected["value"]);
        }
        if matches!(focus, Availability::Unknown { .. }) {
            assert!(
                field["state"]["reason"]
                    .as_str()
                    .is_some_and(|r| !r.is_empty())
            );
        }
        assert_eq!(serde_json::to_vec(&variant).unwrap(), original);
        let mut exact = limits();
        exact.max_output_bytes = p.files().values().map(Vec::len).sum();
        assert!(compile(&variant, exact).is_ok());
        exact.max_output_bytes -= 1;
        assert_eq!(
            compile(&variant, exact).unwrap_err(),
            ExportError::OutputLimit
        );
    }
    let mut brief = fixture("proposed");
    proposed(&mut brief).components[0].state_and_actions =
        "Disabled until record is ready; CDP unknown; Save not executed".into();
    let p = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("reported state/actions (not executed): \"Disabled until record is ready; CDP unknown; Save not executed\""));
}

#[test]
fn scope_and_exclusions_guide_selection_without_becoming_printable_ui() {
    let mut brief = fixture("proposed");
    brief.views[0].scope = "SCOPE_PROCEDURE_CANARY".into();
    brief.views[0].safe_source_reference = "SOURCE_PROCEDURE_CANARY".into();
    brief.views[0]
        .not_depicted
        .push("EXCLUSION_PROCEDURE_CANARY".into());
    proposed(&mut brief).components[0].label = "record CDP unknown".into();
    let p = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("; text \"record CDP unknown\""));
    assert!(prompt.contains("PRINTABLE SHEET TEXT"));
    for canary in ["SCOPE_PROCEDURE_CANARY", "EXCLUSION_PROCEDURE_CANARY"] {
        assert!(prompt.split("ГЕОМЕТРИЯ").next().unwrap().contains(canary));
        assert!(
            !prompt
                .split("PRINTABLE SHEET TEXT")
                .nth(1)
                .unwrap()
                .contains(canary)
        );
        assert!(
            std::str::from_utf8(&p.files()["drawing-brief.md"])
                .unwrap()
                .contains(canary)
        );
    }
    assert!(!prompt.contains("SOURCE_PROCEDURE_CANARY"));
    assert!(prompt.find(&brief.views[0].state).unwrap() < prompt.find("ГЕОМЕТРИЯ").unwrap());
    assert!(!prompt.contains("record 1"));
}

#[test]
fn unnamed_source_text_is_kept_without_fabricated_content_or_painted_bounds() {
    let mut brief = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!()
    };
    set_test_role(&mut snapshot.nodes[0], "LABEL");
    let p = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    let text = prompt
        .lines()
        .find(|l| l.starts_with("Object label layout: Text [#1]"))
        .unwrap();
    assert!(!text.contains("; text "));
    assert!(text.contains("box[x"));
    assert!(prompt.contains("An unnamed Text record still has a place"));
    assert!(prompt.contains("CLIPPING TAKES PRIORITY"));
    assert!(prompt.contains("icon-font\nglyph"));
    assert!(prompt.contains("Keep inline clickable\nwords inline and unboxed"));
}

#[test]
fn unused_measurement_recipes_are_not_instructions_to_invent_details() {
    let mut brief = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!()
    };
    for node in &mut snapshot.nodes {
        node.children.clear();
    }
    let Property::Requested { state, .. } = &mut snapshot.nodes[0].properties[1] else {
        panic!()
    };
    *state = Availability::Known {
        value: Value::Text("EDGE FRAGMENT ( NAMED-ANCHOR RELATION:".into()),
    };
    let p = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(!prompt.contains("AX FRAME MEASUREMENT or AX FRAME EXTENT"));
    assert!(!prompt.contains("Print “AX frame”"));
    assert!(!prompt.contains("LOCAL EDGE DETAILS"));
    assert!(!prompt.contains("EDGE FRAGMENT blocks are independent"));
    assert!(prompt.contains("DIMENSIONED RECTANGLE"));
    let mut proposed_brief = fixture("proposed");
    proposed(&mut proposed_brief).components[0].label = "AX FRAME MEASUREMENT".into();
    let p = compile(&proposed_brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(!prompt.contains("AX FRAME MEASUREMENT or AX FRAME EXTENT"));
}

#[test]
fn caller_context_preserves_declared_meaning_without_selected_inference() {
    let mut brief = fixture("observed");
    brief.views[0].state =
        "Menu open; first suggestion keyboard-active; none applied; no input operation performed"
            .into();
    let p = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert_eq!(prompt.matches(&brief.views[0].state).count(), 1);
    assert!(!prompt.contains("PRINT THIS VIEW'S STATE CAPTION"));
    assert!(prompt.contains("source state retained"));
    brief.views[0].state = "Menu open".into();
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!()
    };
    for node in &mut snapshot.nodes {
        let mut property = node.properties[0].clone();
        let Property::Requested { field, state, .. } = &mut property else {
            panic!()
        };
        *field = Field::Selected;
        *state = Availability::Known {
            value: Value::Flag(true),
        };
        node.properties.push(property);
    }
    snapshot.context.fields.push(Field::Selected);
    snapshot.coverage.fields.push(Field::Selected);
    for observation in &mut snapshot.observations {
        observation.coverage.fields.push(Field::Selected);
    }
    let p = compile(&brief, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("Selected=true"));
    assert!(!prompt.contains("CAPTION exactly:"));
}

#[test]
fn whole_view_caller_context_is_not_duplicated_into_measurement_blocks() {
    let mut brief = fixture("observed");
    brief.views[0].state = "Page controls disabled; first suggestion keyboard-active; none applied. Other actions blocked. Public input draft orchard.".into();
    let original = serde_json::to_vec(&brief).unwrap();
    let before = compile(&brief, limits()).unwrap();
    assert_eq!(serde_json::to_vec(&brief).unwrap(), original);
    for (name, bytes) in before.files().iter().filter(|(n, _)| n.ends_with(".txt")) {
        let prompt = std::str::from_utf8(bytes).unwrap();
        assert_eq!(prompt.matches(&brief.views[0].state).count(), 1, "{name}");
        let geometry = prompt.split("ГЕОМЕТРИЯ").nth(1).unwrap();
        for clause in [
            "Page controls disabled",
            "first suggestion keyboard-active",
            "Other actions blocked",
            "Public input draft orchard",
        ] {
            assert!(!geometry.contains(clause), "{name}: {clause}");
        }
        assert!(prompt.contains("annotation, not observed per-control properties"));
        assert!(!prompt.contains("Enabled=false") && !prompt.contains("Enabled=true"));
    }
    brief.views[0].state = "Settings visible; off state declared; enablement unknown".into();
    let after = compile(&brief, limits()).unwrap();
    let a = data(&before, "scene.json");
    let b = data(&after, "scene.json");
    assert_eq!(a["views"][0]["components"], b["views"][0]["components"]);
    for file in ["dimensions.json", "manifest.json"] {
        assert_eq!(before.files()[file], after.files()[file], "{file}");
    }
}

#[test]
fn nonpositive_gaps_do_not_receive_positive_symbolic_separation() {
    for next_y in [40.0, 30.0] {
        let mut brief = fixture("observed");
        let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
            panic!()
        };
        for (i, role, y) in [(0, "DIV", 0.0), (2, "SELECT", 10.0), (4, "SELECT", next_y)] {
            set_test_role(&mut snapshot.nodes[i], role);
            let Property::Requested {
                state:
                    Availability::Known {
                        value: Value::Geometry(g),
                    },
                ..
            } = &mut snapshot.nodes[i].properties[2]
            else {
                panic!()
            };
            let Shape::Rect(r) = &mut g.shape else {
                panic!()
            };
            *r = Rect {
                x: 10.0,
                y,
                width: 80.0,
                height: 30.0,
            };
        }
        snapshot.nodes[0].children = [2, 4].map(|i| snapshot.nodes[i].key.clone()).to_vec();
        let p = compile(&brief, limits()).unwrap();
        let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
        assert!(!prompt.contains("↕ 0 CSS px") && !prompt.contains("↕ −10 CSS px"));
        let d = data(&p, "dimensions.json");
        assert!(
            d[0]["dimensions"]
                .as_array()
                .unwrap()
                .iter()
                .any(|d| d["label"] == "Vertical edge gap" && d["value"] == next_y - 40.0)
        );
    }
}

#[test]
fn center_anchors_keep_axis_without_border_corner_sketches() {
    for (start, end, value, direction) in [
        (Edge::CenterX, Edge::Right, 600.0, "HORIZONTAL"),
        (Edge::CenterY, Edge::Bottom, 400.0, "VERTICAL"),
        (Edge::Right, Edge::CenterX, 600.0, "HORIZONTAL"),
        (Edge::CenterX, Edge::CenterX, 0.0, "HORIZONTAL"),
    ] {
        let mut b = fixture("proposed");
        let layout = proposed(&mut b);
        let mut d = layout.dimensions[0].clone();
        d.anchors[0].edge = start;
        d.anchors[1].edge = end;
        d.value = Some(value);
        layout.dimensions = vec![d];
        layout.chains.clear();
        let original = serde_json::to_vec(&b).unwrap();
        let p =
            compile(&b, limits()).unwrap_or_else(|e| panic!("{start:?}/{end:?}/{value}: {e:?}"));
        assert_eq!(serde_json::to_vec(&b).unwrap(), original);
        let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
        assert!(prompt.contains(&format!("NAMED-ANCHOR RELATION: {direction};")));
        assert!(prompt.contains(&format!("{start:?} anchor → END")));
        assert!(!prompt.contains("Symbolic layout: START border first"));
        assert_eq!(
            data(&p, "dimensions.json")[0]["dimensions"][0]["value"],
            value
        );
    }
}

#[test]
fn container_frame_width_has_below_slot_without_claiming_enclosure() {
    let b = fixture("proposed");
    let p = compile(&b, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(
        prompt
            .contains("WIDTH 1200 CSS px BELOW this measured source frame; HEIGHT 800 CSS px LEFT")
    );
    let owner = prompt.find("FRAME OWNER (unprinted): #1.").unwrap();
    let measure = prompt[owner..].find("DIMENSIONED RECTANGLE").unwrap();
    assert!(!prompt[owner..owner + measure].contains("FRAME OWNER (unprinted): #2"));
    assert!(prompt.contains("PRINT BELOW-FRAME WIDTH LABEL exactly:"));
    assert!(prompt.contains("width 1200 CSS px"));
    assert!(prompt.contains("label-layout boundary belongs to the named text"));
    for (x, y) in [(1300.0, 900.0), (40.0, 40.0)] {
        let mut witness = b.clone();
        let layout = proposed(&mut witness);
        let mut child = layout.components[9].clone();
        child.id = "extra-descendant".into();
        child.parent = Some(layout.components[0].id.clone());
        child.role = Role::Text;
        child.label = "Unmeasured descendant".into();
        child.geometry.shape = Shape::Rect(Rect {
            x,
            y,
            width: 200.,
            height: 100.,
        });
        layout.components.push(child);
        let original = serde_json::to_vec(&witness).unwrap();
        let candidate = compile(&witness, limits()).unwrap();
        let text = std::str::from_utf8(&candidate.files()["prompt.txt"]).unwrap();
        assert!(
            text.contains("WIDTH 1200 CSS px BELOW this measured source frame; HEIGHT 800 CSS px")
        );
        assert!(text.contains("A container frame need not enclose its descendants;"));
        assert!(!text.contains("enclosing shape") && !text.contains("enclosing outline"));
        assert_eq!(
            data(&candidate, "dimensions.json"),
            data(&p, "dimensions.json")
        );
        assert_eq!(serde_json::to_vec(&witness).unwrap(), original);
        let mut exact = limits();
        exact.max_output_bytes = candidate.files().values().map(Vec::len).sum();
        assert!(compile(&witness, exact).is_ok());
        exact.max_output_bytes -= 1;
        assert_eq!(
            compile(&witness, exact).unwrap_err(),
            ExportError::OutputLimit
        );
    }
}

#[test]
fn multiline_ui_text_is_retained_but_not_promoted_to_anchor_caption() {
    let mut b = fixture("proposed");
    proposed(&mut b).components[9].label = "Open menu\nexpand_more".into();
    let p = compile(&b, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("; text \"Open menu\\nexpand_more\""));
    assert!(
        prompt
            .lines()
            .filter(|s| s.starts_with("DIMENSIONED RECTANGLE")
                || s.starts_with("EDGE FRAGMENT")
                || s.starts_with("NAMED-ANCHOR RELATION"))
            .all(|s| !s.contains("expand_more"))
    );
    assert!(
        std::str::from_utf8(&p.files()["scene.json"])
            .unwrap()
            .contains("Open menu\\nexpand_more")
    );
    proposed(&mut b).components[9].label = "expand_more".into();
    let p = compile(&b, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("DIMENSIONED RECTANGLE button \"expand_more\""));
}

#[test]
fn singleton_input_gets_size_view_and_anchor_fragments_have_no_whole_objects() {
    let mut b = fixture("proposed");
    proposed(&mut b).components[9].role = Role::Textbox;
    proposed(&mut b).components[9].label = "Standalone input".into();
    let p = compile(&b, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    let sizes = prompt.split("LOCAL DETAIL").skip(1).collect::<Vec<_>>();
    assert!(sizes.iter().any(|s| s.contains("Reuse #10.")
        && s.contains("DIMENSIONED RECTANGLE input \"Standalone input\"")));
    let fragments = prompt
        .split("LOCAL EDGE DETAILS")
        .nth(1)
        .unwrap()
        .split("НАНЕСЕНИЕ РАЗМЕРОВ")
        .next()
        .unwrap();
    assert!(!fragments.contains("Object ") && !fragments.contains("box[x"));
    assert!(!fragments.contains("DIMENSIONED RECTANGLE"));
}

#[test]
fn footer_anchor_does_not_become_whole_parent_in_control_size_view() {
    let mut b = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    set_test_role(&mut snapshot.nodes[0], "DIV");
    snapshot.nodes[0].native_role = Availability::Known {
        value: Value::Text("FOOTER".into()),
    };
    set_test_role(&mut snapshot.nodes[2], "BUTTON");
    let Property::Requested { state, .. } = &mut snapshot.nodes[2].properties[1] else {
        panic!()
    };
    *state = Availability::Known {
        value: Value::Text("Close".into()),
    };
    for (i, r) in [
        (
            0,
            Rect {
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 100.0,
            },
        ),
        (
            2,
            Rect {
                x: 140.0,
                y: 50.0,
                width: 40.0,
                height: 30.0,
            },
        ),
    ] {
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[i].properties[2]
        else {
            panic!()
        };
        g.shape = Shape::Rect(r);
    }
    snapshot.nodes[0].children = vec![snapshot.nodes[2].key.clone()];
    let p = compile(&b, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    let detail = prompt
        .split("LOCAL DETAIL \"Close — dimensions\"")
        .nth(1)
        .unwrap()
        .split("LOCAL DETAIL")
        .next()
        .unwrap()
        .split("LOCAL EDGE DETAILS")
        .next()
        .unwrap();
    assert!(detail.contains("WIDTH 40 CSS px") && detail.contains("HEIGHT 30 CSS px"));
    assert!(!detail.contains("Object container") && !detail.contains("native \"FOOTER\""));
    let fragments = prompt
        .split("LOCAL EDGE DETAILS")
        .nth(1)
        .unwrap()
        .split("НАНЕСЕНИЕ РАЗМЕРОВ")
        .next()
        .unwrap();
    assert!(fragments.contains("20 CSS px"));
    assert!(!fragments.contains("box[x") && !fragments.contains("Object "));
}
