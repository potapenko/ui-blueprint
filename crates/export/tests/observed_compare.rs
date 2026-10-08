use serde_json::{Value as Json, json};
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
fn source() -> Snapshot {
    let d = Document::from_json(
        include_bytes!("../../../fixtures/golden/GEO-SIZE-RATIO__width.json"),
        65536,
    )
    .unwrap();
    let Artifact::Finding(case) = d.artifact else {
        panic!("fixture")
    };
    case.snapshot
}
fn evidence(s: &Snapshot) -> Evidence {
    let Property::Requested { evidence, .. } = &s.nodes[0].properties[0] else {
        panic!("evidence")
    };
    evidence.clone()
}
fn brief(a: Snapshot, z: Snapshot) -> DrawingBrief {
    let mut b: DrawingBrief = serde_json::from_slice(include_bytes!(
        "../../../fixtures/export/observed-brief.json"
    ))
    .unwrap();
    b.views.truncate(1);
    b.views[0].source = SourceInput::Observed {
        snapshot: Box::new(a),
        public_text_fields: vec![],
    };
    b.views[0].id = "before".into();
    b.views[0].safe_source_reference = "Explicit synthetic comparison fixture".into();
    let mut next = b.views[0].clone();
    next.id = "after".into();
    next.source = SourceInput::Observed {
        snapshot: Box::new(z),
        public_text_fields: vec![],
    };
    b.views.push(next);
    b.purpose = Purpose::Compare;
    b.details.clear();
    b.transitions.clear();
    b.comparisons = vec![ComparisonRequest {
        before: "before".into(),
        after: "after".into(),
        different_basis: None,
        geometry_space: None,
    }];
    b
}
fn scene(b: &DrawingBrief) -> Json {
    let p = compile(b, limits()).unwrap();
    serde_json::from_slice(&p.files()["scene.json"]).unwrap()
}
fn after(b: &mut DrawingBrief) -> &mut Snapshot {
    let SourceInput::Observed { snapshot, .. } = &mut b.views[1].source else {
        panic!("observed")
    };
    snapshot
}
#[test]
fn golden_checkbox_names_only_confirmed_property_and_preserves_independent_statuses() {
    let d = Document::from_json(
        include_bytes!("../../../fixtures/golden/GOLDEN01.json"),
        100000,
    )
    .unwrap();
    let Artifact::GoldenChain(chain) = d.artifact else {
        panic!("golden")
    };
    let b = brief(chain.before, chain.after);
    let original = serde_json::to_vec(&b).unwrap();
    let p = compile(&b, limits()).unwrap();
    let s: Json = serde_json::from_slice(&p.files()["scene.json"]).unwrap();
    let entries = s["comparison_results"][0]["entries"].as_array().unwrap();
    let content: Vec<_> = entries
        .iter()
        .filter(|e| e["content_changed"] == true)
        .collect();
    assert_eq!(content.len(), 1);
    assert_eq!(content[0]["field"], "checked");
    assert_eq!(content[0]["before"]["state"]["value"]["value"], false);
    assert_eq!(content[0]["after"]["state"]["value"]["value"], true);
    assert_eq!(p.comparison_attribution(), "engine_recorded_graph");
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("ENGINE RECORDED COMPARISON"));
    assert!(prompt.contains("\"checked\""));
    assert!(!prompt.contains("unresolved_g02"));
    assert_eq!(serde_json::to_vec(&b).unwrap(), original);
    let m: Json = serde_json::from_slice(&p.files()["manifest.json"]).unwrap();
    assert_eq!(m["package_version"], "0.2.0");
    assert_eq!(m["validation_status"], "unverified");
    assert_eq!(m["approval_status"], "draft");
    assert_eq!(m["references"], json!([]));
    assert_eq!(p.files().len(), 6);
}
#[test]
fn unchanged_unknown_redacted_partial_and_evidence_only_are_not_upgraded() {
    let a = source();
    assert_eq!(
        scene(&brief(a.clone(), a.clone()))["comparison_results"][0]["entries"],
        json!([])
    );
    for state in [
        Availability::Unknown {
            reason: Id("not_observed".into()),
        },
        Availability::Unsupported {
            reason: Id("not_exposed".into()),
        },
        Availability::Redacted {},
    ] {
        let mut z = a.clone();
        let Property::Requested { state: current, .. } = &mut z.nodes[0].properties[0] else {
            panic!()
        };
        *current = state;
        let s = scene(&brief(a.clone(), z));
        let e = &s["comparison_results"][0]["entries"][0];
        assert_eq!(e["content_changed"], true);
        assert_eq!(e["evidence_changed"], false);
        assert!(e["after"]["state"].get("value").is_none());
    }
    let mut z = a.clone();
    z.observations[0].end += 1.0;
    let s = scene(&brief(a.clone(), z));
    assert_eq!(
        s["comparison_results"][0]["entries"][0]["content_changed"],
        false
    );
    assert_eq!(
        s["comparison_results"][0]["entries"][0]["evidence_changed"],
        true
    );
    let mut z = a.clone();
    z.nodes.clear();
    z.coverage.status = CoverageStatus::Partial;
    z.coverage.omitted_count = Some(1);
    let s = scene(&brief(a, z));
    let e = &s["comparison_results"][0]["entries"][0];
    assert_eq!(e["kind"], "node_presence");
    assert_eq!(e["after_present"], false);
    assert_eq!(e["after"], Json::Null);
    assert_eq!(s["views"][1]["coverage"]["status"], "partial");
    assert!(
        s["comparison_results"][0]["limitations"]
            .as_str()
            .unwrap()
            .contains("not deleted")
    );
}
#[test]
fn every_graph_kind_has_public_before_after_facts_and_ordered_references() {
    let mut a = source();
    let mut child = a.nodes[0].clone();
    child.key.key = Id("B".into());
    a.nodes.push(child);
    a.nodes[0].children = vec![a.nodes[1].key.clone()];
    a.relations.push(Relation {
        kind: RelationKind::LabelledBy,
        from: a.nodes[0].key.clone(),
        to: a.nodes[1].key.clone(),
        evidence: evidence(&a),
    });
    a.components.push(ComponentMapping {
        logical_component_key: Id("map".into()),
        members: vec![a.nodes[0].key.clone()],
        declaration_source: Id("app".into()),
        provenance: Provenance::Reported,
    });
    let mut z = a.clone();
    z.nodes[0].children.clear();
    z.nodes[0].native_role = Availability::Known {
        value: Value::Text("public role".into()),
    };
    z.relations[0].evidence.method = Id("new_source".into());
    z.components[0].members.push(z.nodes[1].key.clone());
    z.focus.keyboard = FocusRef::Known {
        target: z.nodes[0].key.clone(),
        evidence: evidence(&z),
    };
    z.focus.accessibility = FocusRef::None {
        evidence: evidence(&z),
    };
    z.focus.active_descendant = FocusRef::Unknown {
        reason: Id("unknown".into()),
    };
    z.focus.text_selection = Some(TextSelection {
        anchor: 2,
        focus: 4,
        units: Id("utf16".into()),
        evidence: evidence(&z),
    });
    z.focus.composition_state = Property::Requested {
        field: Field::Value,
        sensitivity: Sensitivity::Public,
        evidence: evidence(&z),
        state: Availability::Known {
            value: Value::Text("PRIVATE_COMPOSITION_CANARY".into()),
        },
    };
    let s = scene(&brief(a, z));
    let e = s["comparison_results"][0]["entries"].as_array().unwrap();
    assert_eq!(
        e.iter()
            .map(|e| e["kind"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec![
            "children",
            "node_metadata",
            "relation",
            "component",
            "focus_keyboard",
            "focus_accessibility",
            "focus_active_descendant",
            "focus_text_selection",
            "focus_composition"
        ]
    );
    assert_eq!(e[0]["before"], json!(["N001"]));
    assert_eq!(e[0]["after"], json!([]));
    assert_eq!(e[2]["content_changed"], false);
    assert_eq!(e[2]["evidence_changed"], true);
    assert_eq!(e[3]["after"]["members"].as_array().unwrap().len(), 2);
    assert_eq!(e[4]["after"]["status"], "known");
    assert_eq!(e[7]["after"]["anchor"], 2);
    assert_eq!(e[8]["after"]["state"]["availability"], "redacted");
    assert!(!s.to_string().contains("PRIVATE_COMPOSITION_CANARY"));
}
#[test]
fn incompatible_sources_need_explicit_basis_and_never_gain_matches() {
    for variant in 0..4 {
        let a = source();
        let mut b = brief(a.clone(), a);
        let z = after(&mut b);
        match variant {
            0 => z.context.session_id = Id("other".into()),
            1 => z.context.target.generation = Id("other".into()),
            2 => z.context.scope_id = Id("other".into()),
            _ => z.context.environment_revision = Id("other".into()),
        }
        if variant == 2 {
            z.coverage.scope_id = z.context.scope_id.clone();
            z.observations[0].coverage.scope_id = z.context.scope_id.clone();
        }
        if variant == 3 {
            assert!(compile(&b, limits()).is_ok());
            continue;
        }
        assert_eq!(
            compile(&b, limits()).unwrap_err(),
            ExportError::IncompatibleViews
        );
        b.comparisons[0].different_basis = Some("Explicit distinct source contexts".into());
        let s = scene(&b);
        assert_eq!(s["comparison_results"][0]["status"], "incompatible_context");
        assert_eq!(s["comparison_results"][0]["entries"], json!([]));
        assert_eq!(
            compile(&b, limits()).unwrap().comparison_attribution(),
            "not_compared"
        );
    }
}
#[test]
fn selected_space_geometry_uses_engine_known_unknown_and_binding_without_source_mutation() {
    let a = source();
    let mut b = brief(a.clone(), a);
    let g = match after(&mut b).nodes[0].properties[0].known().unwrap() {
        Value::Geometry(g) => g.clone(),
        _ => panic!(),
    };
    b.comparisons[0].geometry_space = Some(g.coordinate_space.id.clone());
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut after(&mut b).nodes[0].properties[0]
    else {
        panic!()
    };
    let Shape::Rect(r) = &mut g.shape else {
        panic!()
    };
    r.width = 48.0;
    let s = scene(&b);
    let geom = &s["comparison_results"][0]["geometry"][0];
    assert_eq!(
        geom["displacement"],
        json!({"dx":0.0,"dy":0.0,"dwidth":18.0,"dheight":0.0})
    );
    assert_eq!(geom["before"]["rect"]["width"], 30.0);
    assert_eq!(geom["after"]["rect"]["width"], 48.0);
    let Property::Requested { state, .. } = &mut after(&mut b).nodes[0].properties[0] else {
        panic!()
    };
    *state = Availability::Redacted {};
    // Space still needs a sourced definition on the after side.
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::InvalidGeometry
    );
    let mut known_node = source().nodes.remove(0);
    known_node.key.key = Id("space-source".into());
    after(&mut b).nodes.push(known_node);
    let s = scene(&b);
    let geom = &s["comparison_results"][0]["geometry"][0];
    assert_eq!(geom["status"], "unknown");
    assert_eq!(geom["displacement"], Json::Null);
    assert_eq!(geom["after"]["reason"], "redacted_property");
}
#[test]
fn allowlist_and_aliases_cover_comparison_facts_in_every_package_file() {
    let mut a = source();
    a.context.fields.push(Field::Name);
    a.coverage.fields.push(Field::Name);
    a.observations[0].coverage.fields.push(Field::Name);
    let ev = evidence(&a);
    a.nodes[0].properties.push(Property::Requested {
        field: Field::Name,
        sensitivity: Sensitivity::Public,
        evidence: ev,
        state: Availability::Known {
            value: Value::Text("PRIVATE_BEFORE_CANARY".into()),
        },
    });
    let mut z = a.clone();
    let Property::Requested { state, .. } = &mut z.nodes[0].properties[1] else {
        panic!()
    };
    *state = Availability::Known {
        value: Value::Text("PRIVATE_AFTER_CANARY".into()),
    };
    let mut b = brief(a, z);
    let p = compile(&b, limits()).unwrap();
    for bytes in p.files().values() {
        let t = std::str::from_utf8(bytes).unwrap();
        assert!(!t.contains("CANARY"));
        assert!(!t.contains("fixture.authored_layout"));
    }
    for v in &mut b.views {
        let SourceInput::Observed {
            public_text_fields, ..
        } = &mut v.source
        else {
            panic!()
        };
        public_text_fields.push(Field::Name);
    }
    let p = compile(&b, limits()).unwrap();
    let t = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(t.contains("PRIVATE_BEFORE_CANARY"));
    assert!(t.contains("PRIVATE_AFTER_CANARY"));
    // Explicitly allowed field still excludes paths and template syntax.
    let Property::Requested { state, .. } = &mut after(&mut b).nodes[0].properties[1] else {
        panic!()
    };
    *state = Availability::Known {
        value: Value::Text("/private/PATH_CANARY".into()),
    };
    let p = compile(&b, limits()).unwrap();
    assert!(
        p.files()
            .values()
            .all(|x| !std::str::from_utf8(x).unwrap().contains("PATH_CANARY"))
    );
}

#[test]
fn sourced_local_transform_separates_window_motion_and_missing_mapping() {
    let mut a = source();
    let local = Space {
        id: Id("PRIVATE_SPACE_CANARY".into()),
        kind: SpaceKind::Local,
        units: Unit::CssPx,
        origin: Origin::TopLeft,
    };
    let apply = |s: &mut Snapshot, x: f64, tx: f64| {
        let Value::Geometry(g) = s.nodes[0].properties[0].known().unwrap() else {
            panic!()
        };
        let from = g.coordinate_space.clone();
        let t = Transform {
            from: from.clone(),
            to: local.clone(),
            affine: [1.0, 0.0, 0.0, 1.0, tx, 0.0],
            target: s.context.target.clone(),
            surface: s.nodes[0].surface.clone(),
            environment_revision: s.context.environment_revision.clone(),
            evidence: evidence(s),
        };
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut s.nodes[0].properties[0]
        else {
            panic!()
        };
        g.transform = TransformState::Known {
            transform: Box::new(t),
        };
        let Shape::Rect(r) = &mut g.shape else {
            panic!()
        };
        r.x = x;
    };
    apply(&mut a, 110.0, -100.0);
    let mut z = a.clone();
    z.context.environment_revision = Id("moved".into());
    apply(&mut z, 160.0, -150.0);
    let mut b = brief(a, z);
    b.comparisons[0].geometry_space = Some(local.id);
    let s = scene(&b);
    let g = &s["comparison_results"][0]["geometry"][0];
    assert_eq!(g["before"]["rect"]["x"], 10.0);
    assert_eq!(g["after"]["rect"]["x"], 10.0);
    assert_eq!(
        g["displacement"],
        json!({"dx":0.0,"dy":0.0,"dwidth":0.0,"dheight":0.0})
    );
    assert_eq!(g["after"]["evidence"].as_array().unwrap().len(), 1);
    let p = compile(&b, limits()).unwrap();
    assert!(p.files().values().all(|bytes| {
        !std::str::from_utf8(bytes)
            .unwrap()
            .contains("PRIVATE_SPACE_CANARY")
    }));
    // Keep the destination definition elsewhere, but remove the selected node's path.
    let mut other = after(&mut b).nodes[0].clone();
    other.key.key = Id("other".into());
    after(&mut b).nodes.push(other);
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut after(&mut b).nodes[0].properties[0]
    else {
        panic!()
    };
    g.transform = TransformState::LocalOnly {};
    let s = scene(&b);
    let g = &s["comparison_results"][0]["geometry"][0];
    assert_eq!(g["status"], "unknown");
    assert_eq!(g["after"]["reason"], "missing_transform");
    assert_eq!(g["displacement"], Json::Null);
}
#[test]
fn namespaces_are_membership_not_label_matching_and_space_conflicts_refuse() {
    let a = source();
    let mut z = a.clone();
    z.nodes[0].key.namespace = Id("other".into());
    let s = scene(&brief(a, z));
    let e = &s["comparison_results"][0]["entries"];
    assert_eq!(e.as_array().unwrap().len(), 2);
    assert_eq!(e[0]["kind"], "node_presence");
    assert_eq!(e[0]["after_present"], false);
    assert_eq!(e[1]["before_present"], false);
    let a = source();
    let mut b = brief(a.clone(), a);
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut after(&mut b).nodes[0].properties[0]
    else {
        panic!()
    };
    g.coordinate_space.origin = Origin::BottomLeft;
    b.comparisons[0].geometry_space = Some(g.coordinate_space.id.clone());
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::IncompatibleViews
    );
}
