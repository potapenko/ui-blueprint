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
// Prompt embeds the single view object; drawing-brief embeds the full array.
fn single_dimension_view(array: &str) -> String {
    array
        .strip_prefix("[\n")
        .unwrap()
        .strip_suffix("\n]")
        .unwrap()
        .lines()
        .map(|line| line.strip_prefix("  ").unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}
#[test]
fn current_packages_preserve_history_except_accepted_known_anchor_results() {
    let (historical_dimensions, current_dimensions) = current_observed_dimensions();
    for name in ["observed", "proposed"] {
        let brief = fixture(name);
        let original = serde_json::to_vec(&brief).unwrap();
        let package = compile(&brief, limits()).expect("existing package");
        assert_eq!(serde_json::to_vec(&brief).unwrap(), original);
        assert_eq!(package.files().len(), 6);
        for (file, actual) in package.files() {
            let path = format!(
                "{}/../../fixtures/export/{name}-package/{file}",
                env!("CARGO_MANIFEST_DIR")
            );
            let mut expected =
                std::fs::read_to_string(path).expect("immutable historical artifact");
            if name == "observed" {
                let replacement = match file.as_str() {
                    "dimensions.json" | "drawing-brief.md" => {
                        Some((historical_dimensions.clone(), current_dimensions.clone()))
                    }
                    "prompt.txt" => Some((
                        single_dimension_view(&historical_dimensions),
                        single_dimension_view(&current_dimensions),
                    )),
                    _ => None,
                };
                if let Some((old, new)) = replacement {
                    assert_eq!(
                        expected.matches(&old).count(),
                        1,
                        "{file}: exact historical dimension block"
                    );
                    expected = expected.replacen(&old, &new, 1);
                }
            }
            // All six files, including every unrelated byte, remain checked.
            // No normalization of source facts, privacy, evidence, anchors or statuses.
            assert!(actual == expected.as_bytes(), "{name}/{file}");
        }
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
    assert_eq!(dimensions.len(), 32);
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
