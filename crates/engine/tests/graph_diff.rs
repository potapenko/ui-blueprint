use uiblueprint_engine::diff::{
    DiffError, DiffLimits, GraphDiff, GraphEntry, GraphKind, compare_graph,
};
use uiblueprint_schema::{model::*, validation};

fn source() -> Snapshot {
    let doc = Document::from_json(
        include_bytes!("../../../fixtures/golden/GEO-SIZE-RATIO__width.json"),
        65536,
    )
    .unwrap();
    let Artifact::Finding(case) = doc.artifact else {
        panic!("fixture")
    };
    let mut s = case.snapshot;
    let mut b = s.nodes[0].clone();
    b.key.key = Id("B".into());
    let mut c = b.clone();
    c.key.key = Id("C".into());
    s.nodes.extend([b, c]);
    s.nodes[0].children = vec![s.nodes[1].key.clone(), s.nodes[2].key.clone()];
    s.relations = vec![Relation {
        kind: RelationKind::LabelledBy,
        from: s.nodes[0].key.clone(),
        to: s.nodes[1].key.clone(),
        evidence: evidence(&s),
    }];
    s.components = vec![ComponentMapping {
        logical_component_key: Id("component".into()),
        members: vec![s.nodes[0].key.clone(), s.nodes[1].key.clone()],
        declaration_source: Id("reported-map".into()),
        provenance: Provenance::Reported,
    }];
    s.nodes[0].extensions = vec![ExtensionProperty {
        namespace: Id("fixture.extra".into()),
        name: Id("label".into()),
        property: text(&s, ""),
    }];
    s.nodes[0].source_declarations = vec![SourceDeclaration {
        namespace: Id("fixture.declaration".into()),
        name: Id("type".into()),
        state: Availability::Known {
            value: Value::Text("Row".into()),
        },
        sensitivity: Sensitivity::Public,
        source: Id("app".into()),
    }];
    s.focus.keyboard = FocusRef::Known {
        target: s.nodes[0].key.clone(),
        evidence: evidence(&s),
    };
    s.focus.accessibility = FocusRef::None {
        evidence: evidence(&s),
    };
    s.focus.active_descendant = FocusRef::Unknown {
        reason: Id("not_observed".into()),
    };
    s.focus.text_selection = Some(TextSelection {
        anchor: 2,
        focus: 4,
        units: Id("utf16".into()),
        evidence: evidence(&s),
    });
    s.focus.composition_state = text(&s, "idle");
    validation::validate_snapshot(&s).unwrap();
    s
}
fn evidence(s: &Snapshot) -> Evidence {
    let Property::Requested { evidence, .. } = &s.nodes[0].properties[0] else {
        panic!("evidence")
    };
    evidence.clone()
}
fn text(s: &Snapshot, value: &str) -> Property {
    Property::Requested {
        field: Field::Name,
        sensitivity: Sensitivity::Public,
        evidence: evidence(s),
        state: Availability::Known {
            value: Value::Text(value.into()),
        },
    }
}
fn diff<'a>(a: &'a Snapshot, b: &'a Snapshot, cap: usize) -> GraphDiff<'a> {
    compare_graph(a, b, DiffLimits { max_entries: cap }).unwrap()
}
fn one(a: &Snapshot, b: &Snapshot, kind: GraphKind, content: bool, evidence: bool) -> GraphEntry {
    let d = diff(a, b, 100);
    assert_eq!(d.entries.len(), 1, "{:#?}", d.entries);
    let e = d.entries[0];
    assert_eq!(
        (e.kind, e.content_changed, e.evidence_changed),
        (kind, content, evidence)
    );
    e
}
#[test]
fn children_order_and_reparenting_are_literal_borrowed_records() {
    let a = source();
    let mut b = a.clone();
    b.nodes[0].children.reverse();
    let e = one(&a, &b, GraphKind::Children, true, false);
    assert_eq!((e.before_index, e.after_index), (Some(0), Some(0)));
    b.nodes[0].children = vec![b.nodes[1].key.clone()];
    b.nodes[1].children = vec![b.nodes[2].key.clone()];
    let originals = (a.clone(), b.clone());
    let d = diff(&a, &b, 100);
    assert_eq!(
        d.entries
            .iter()
            .map(|e| (e.kind, e.before_index))
            .collect::<Vec<_>>(),
        vec![
            (GraphKind::Children, Some(0)),
            (GraphKind::Children, Some(1))
        ]
    );
    assert!(std::ptr::eq(d.before, &a));
    assert!(std::ptr::eq(d.after, &b));
    assert_eq!((a.clone(), b.clone()), originals);
}
#[test]
fn relations_match_tuple_and_duplicate_occurrence_without_guessing_continuity() {
    let mut a = source();
    a.relations.push(a.relations[0].clone());
    let mut b = a.clone();
    b.relations[1].evidence.method = Id("renewed".into());
    let e = one(&a, &b, GraphKind::Relation, false, true);
    assert_eq!((e.before_index, e.after_index), (Some(1), Some(1)));
    b.relations.pop();
    let e = one(&a, &b, GraphKind::Relation, true, true);
    assert_eq!((e.before_index, e.after_index), (Some(1), None));
    assert!(!e.after_present);
    a.relations.pop();
    b = a.clone();
    b.relations[0].to = b.nodes[2].key.clone();
    let d = diff(&a, &b, 10);
    assert_eq!(
        d.entries
            .iter()
            .map(|e| (e.kind, e.before_present, e.after_present))
            .collect::<Vec<_>>(),
        vec![
            (GraphKind::Relation, true, false),
            (GraphKind::Relation, false, true)
        ]
    );
    b = a.clone();
    b.relations[0].kind = RelationKind::Controls;
    assert_eq!(diff(&a, &b, 10).entries.len(), 2);
}
#[test]
fn component_members_and_declaration_sources_have_separate_flags() {
    let a = source();
    let mut b = a.clone();
    b.components[0].members.push(b.nodes[2].key.clone());
    one(&a, &b, GraphKind::Component, true, false);
    b = a.clone();
    b.components[0].declaration_source = Id("other-source".into());
    one(&a, &b, GraphKind::Component, false, true);
    b.components[0].logical_component_key = Id("distinct-component".into());
    let d = diff(&a, &b, 10);
    assert_eq!(d.entries.len(), 2);
    assert_eq!(
        (d.entries[0].before_present, d.entries[0].after_present),
        (true, false)
    );
    assert_eq!(
        (d.entries[1].before_present, d.entries[1].after_present),
        (false, true)
    );
}
#[test]
fn metadata_surface_native_role_extensions_and_declarations_are_compared() {
    let mut a = source();
    let mut second = a.context.surfaces[0].clone();
    second.id = Id("second".into());
    a.context.surfaces.push(second.clone());
    let mut b = a.clone();
    b.nodes[0].surface = second;
    one(&a, &b, GraphKind::NodeMetadata, true, false);
    b = a.clone();
    b.nodes[0].native_role = Availability::Known {
        value: Value::Text("AXGroup".into()),
    };
    one(&a, &b, GraphKind::NodeMetadata, true, false);
    b = a.clone();
    b.nodes[0].extensions[0].property = text(&b, "new-label");
    one(&a, &b, GraphKind::NodeMetadata, true, false);
    b = a.clone();
    let Property::Requested { evidence, .. } = &mut b.nodes[0].extensions[0].property else {
        panic!()
    };
    evidence.method = Id("new-method".into());
    one(&a, &b, GraphKind::NodeMetadata, false, true);
    b = a.clone();
    b.nodes[0].source_declarations[0].state = Availability::Unknown {
        reason: Id("not_exposed".into()),
    };
    one(&a, &b, GraphKind::NodeMetadata, true, false);
    b = a.clone();
    b.nodes[0].source_declarations[0].source = Id("other-declaration-source".into());
    one(&a, &b, GraphKind::NodeMetadata, false, true);
}
#[test]
fn each_focus_axis_retains_known_none_unknown_not_requested_and_selection_units() {
    let a = source();
    let mut b = a.clone();
    b.focus.keyboard = FocusRef::None {
        evidence: evidence(&b),
    };
    one(&a, &b, GraphKind::FocusKeyboard, true, false);
    b = a.clone();
    b.focus.accessibility = FocusRef::Known {
        target: b.nodes[1].key.clone(),
        evidence: evidence(&b),
    };
    one(&a, &b, GraphKind::FocusAccessibility, true, false);
    b = a.clone();
    b.focus.active_descendant = FocusRef::NotRequested {};
    one(&a, &b, GraphKind::FocusActiveDescendant, true, false);
    b = a.clone();
    b.focus.text_selection.as_mut().unwrap().focus = 7;
    one(&a, &b, GraphKind::FocusTextSelection, true, false);
    b = a.clone();
    b.focus.text_selection.as_mut().unwrap().units = Id("unicode_scalars".into());
    one(&a, &b, GraphKind::FocusTextSelection, true, false);
    b = a.clone();
    b.focus.text_selection = None;
    let e = one(&a, &b, GraphKind::FocusTextSelection, true, true);
    assert!(!e.after_present);
    b = a.clone();
    b.focus.composition_state = text(&b, "composing");
    one(&a, &b, GraphKind::FocusComposition, true, false);
    b = a.clone();
    let FocusRef::Known { evidence, .. } = &mut b.focus.keyboard else {
        panic!()
    };
    evidence.method = Id("renewed".into());
    one(&a, &b, GraphKind::FocusKeyboard, false, true);
    b = a.clone();
    b.focus.text_selection.as_mut().unwrap().evidence.method = Id("renewed".into());
    one(&a, &b, GraphKind::FocusTextSelection, false, true);
}
#[test]
fn referenced_observation_changes_are_evidence_only_and_envelope_is_not_graph_content() {
    let a = source();
    let mut b = a.clone();
    b.observations[0].end += 1.0;
    let d = diff(&a, &b, 100);
    assert_eq!(
        d.entries.iter().map(|e| e.kind).collect::<Vec<_>>(),
        vec![
            GraphKind::Property,
            GraphKind::Property,
            GraphKind::Property,
            GraphKind::NodeMetadata,
            GraphKind::Relation,
            GraphKind::FocusKeyboard,
            GraphKind::FocusAccessibility,
            GraphKind::FocusTextSelection,
            GraphKind::FocusComposition
        ]
    );
    assert!(
        d.entries
            .iter()
            .all(|e| !e.content_changed && e.evidence_changed)
    );
    b = a.clone();
    b.revision += 1;
    b.context.environment_revision = Id("environment-two".into());
    b.coverage.status = CoverageStatus::Partial;
    b.coverage.omitted_count = None;
    assert!(diff(&a, &b, 100).entries.is_empty());
}
#[test]
fn partial_absence_namespaces_caps_and_context_generations_are_explicit() {
    let a = source();
    let mut b = a.clone();
    b.nodes.pop();
    b.nodes[0].children.pop();
    b.coverage.status = CoverageStatus::Partial;
    b.coverage.omitted_count = None;
    let d = diff(&a, &b, 100);
    assert_eq!(d.entries.len(), 2);
    assert_eq!(
        (
            d.entries[0].kind,
            d.entries[0].before_present,
            d.entries[0].after_present
        ),
        (GraphKind::NodePresence, true, false)
    );
    assert_eq!(d.entries[1].kind, GraphKind::Children);
    for cap in 0..=2 {
        let d = diff(&a, &b, cap);
        assert_eq!(d.entries.len(), cap);
        assert_eq!(d.omitted_entries, 2 - cap);
    }
    b = a.clone();
    b.nodes[2].key.namespace = Id("different.namespace".into());
    b.nodes[0].children[1] = b.nodes[2].key.clone();
    let d = diff(&a, &b, 100);
    assert_eq!(
        d.entries.iter().map(|e| e.kind).collect::<Vec<_>>(),
        vec![
            GraphKind::NodePresence,
            GraphKind::NodePresence,
            GraphKind::Children
        ]
    );
    b = a.clone();
    b.context.target.generation = Id("g2".into());
    assert_eq!(
        compare_graph(&a, &b, DiffLimits { max_entries: 10 }).unwrap_err(),
        DiffError::IncompatibleContext
    );
    b = a.clone();
    b.context.session_id = Id("other-session".into());
    assert_eq!(
        compare_graph(&a, &b, DiffLimits { max_entries: 10 }).unwrap_err(),
        DiffError::IncompatibleContext
    );
    b = a.clone();
    b.nodes[1].key = b.nodes[0].key.clone();
    assert!(matches!(
        compare_graph(&a, &b, DiffLimits { max_entries: 10 }),
        Err(DiffError::InvalidSnapshot(_))
    ));
}
#[test]
fn redacted_unknown_empty_and_not_requested_never_share_content() {
    let a = source();
    for state in [
        Availability::Redacted {},
        Availability::Unknown {
            reason: Id("unobserved".into()),
        },
        Availability::Unsupported {
            reason: Id("unsupported".into()),
        },
    ] {
        let mut b = a.clone();
        let Property::Requested { state: s, .. } = &mut b.nodes[0].extensions[0].property else {
            panic!()
        };
        *s = state;
        one(&a, &b, GraphKind::NodeMetadata, true, false);
    }
    let mut b = a.clone();
    b.nodes[0].extensions[0].property = Property::NotRequested { field: Field::Name };
    one(&a, &b, GraphKind::NodeMetadata, true, true);
    b = a.clone();
    b.nodes[0].extensions.clear();
    one(&a, &b, GraphKind::NodeMetadata, true, true);
}

#[test]
fn focus_composition_field_and_surface_generation_remain_semantic() {
    let a = source();
    let mut b = a.clone();
    let Property::Requested { field, .. } = &mut b.focus.composition_state else {
        panic!()
    };
    *field = Field::Description;
    one(&a, &b, GraphKind::FocusComposition, true, false);
    b = a.clone();
    b.context.surfaces[0].generation = Id("next-generation".into());
    for n in &mut b.nodes {
        n.surface = b.context.surfaces[0].clone();
    }
    assert_eq!(
        compare_graph(&a, &b, DiffLimits { max_entries: 10 }).unwrap_err(),
        DiffError::IncompatibleContext
    );
}
