use uiblueprint_export::*;
use uiblueprint_schema::model::*;
fn limits() -> ExportLimits {
    ExportLimits {
        max_input_bytes: 2_000_000,
        max_output_bytes: 4_000_000,
        max_components: 256,
        max_views: 8,
        components_per_detail: 12,
    }
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
    assert_eq!(p.files().len(), 6);
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
    assert!(prompt.contains("G01 general"));
    assert!(!prompt.contains("D01 detail"));
    assert_eq!(data(&p, "sheets.json").as_array().unwrap().len(), 1);
    let mut l = limits();
    l.components_per_detail = 6;
    let p = compile(&b, l).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("D01 detail"));
    assert!(prompt.contains("D02 detail"));
    let sheets = data(&p, "sheets.json");
    assert_eq!(sheets[1]["parent_view"], "G01");
    assert_eq!(
        sheets[2]["components"],
        serde_json::json!(["N06", "N07", "N08", "N09"])
    );
    assert!(prompt.contains("10:"));
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
        snapshot.nodes[i].native_role = Availability::Known {
            value: Value::Text(role.into()),
        };
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
    assert!(prompt.contains("label ≈16 CSS px"));
    assert!(prompt.contains("label 12 CSS px"));
    assert!(prompt.lines().any(|line| line.starts_with("REQUIRED")
        && line.contains("start record 5")
        && line.contains("Bottom edge; end record 7")
        && line.contains("Top edge; label ≈16 CSS px")));
    assert!(prompt.contains("end record 7"));
    assert!(prompt.contains("Geometry context 1: LayoutBounds; Space"));
    assert!(prompt.contains("Viewport, CSS px, TopLeft; transform local only"));
    assert!(!prompt.contains("REQUIRED Measured top inset"));
    assert!(prompt.contains("Records [5, 7]: keep exactly shared left AND right edges"));
    assert!(!prompt.contains("[\"css_px\"]"));
}
