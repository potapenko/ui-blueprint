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
    // D05 observations are consistency=unknown: G01 is allowed to retain unknown dimensions.
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
