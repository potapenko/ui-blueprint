use uiblueprint_engine::diff::{DiffError, DiffLimits, Difference, Presence, compare_recorded};
use uiblueprint_schema::{model::*, validation};

fn source() -> Snapshot {
    let document = Document::from_json(
        include_bytes!("../../../fixtures/golden/GEO-SIZE-RATIO__width.json"),
        65536,
    )
    .unwrap();
    let Artifact::Finding(case) = document.artifact else {
        panic!("fixture")
    };
    let mut snapshot = case.snapshot;
    assert_eq!(snapshot.nodes.len(), 1, "authored single-width input");
    let mut second = snapshot.nodes[0].clone();
    second.key.key = Id("recorded-B".into());
    snapshot.nodes.push(second);
    for node in &mut snapshot.nodes {
        node.children.clear();
    }
    snapshot.relations.clear();
    validation::validate_snapshot(&snapshot).unwrap();
    snapshot
}
fn geometry(snapshot: &mut Snapshot) -> &mut Geometry {
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut snapshot.nodes[0].properties[0]
    else {
        panic!("geometry")
    };
    g
}
fn width(snapshot: &mut Snapshot, value: f64) {
    let Shape::Rect(rect) = &mut geometry(snapshot).shape else {
        panic!("rect")
    };
    rect.width = value;
}
fn diff<'a>(
    a: &'a Snapshot,
    b: &'a Snapshot,
    cap: usize,
) -> uiblueprint_engine::diff::RecordedDiff<'a> {
    compare_recorded(a, b, DiffLimits { max_entries: cap }).unwrap()
}
#[test]
fn literal_width_change_and_partial_absence_are_borrowed_not_deletions() {
    let mut before = source();
    width(&mut before, 32.0);
    let mut after = before.clone();
    width(&mut after, 48.0);
    after.nodes.pop();
    after.coverage.status = CoverageStatus::Partial;
    after.coverage.omitted_count = None;
    let originals = (before.clone(), after.clone());
    let result = diff(&before, &after, 10);
    assert_eq!(result.entries.len(), 2);
    assert_eq!(result.omitted_entries, 0);
    let Difference::Property {
        field,
        presence,
        before: Some(old),
        after: Some(new),
        content_changed,
        evidence_changed,
        ..
    } = result.entries[0]
    else {
        panic!("property")
    };
    assert_eq!(field, Field::LayoutBounds);
    assert_eq!(presence, Presence::Both);
    assert!(content_changed);
    assert!(!evidence_changed);
    for (property, expected) in [(old, 32.0), (new, 48.0)] {
        let Some(Value::Geometry(g)) = property.known() else {
            panic!("known geometry")
        };
        let Shape::Rect(rect) = &g.shape else {
            panic!("rect")
        };
        assert_eq!(rect.width, expected);
    }
    let Difference::NodePresence {
        presence: Presence::BeforeOnly,
        before: Some(absent),
        after: None,
    } = result.entries[1]
    else {
        panic!("record absence")
    };
    assert!(std::ptr::eq(absent, &before.nodes[1]));
    assert!(std::ptr::eq(result.before, &before));
    assert!(std::ptr::eq(result.after, &after));
    assert!(std::ptr::eq(old, &before.nodes[0].properties[0]));
    assert_eq!((before.clone(), after.clone()), originals);
    assert_eq!(result.after.coverage.status, CoverageStatus::Partial);
}
#[test]
fn unavailable_after_never_inherits_known_value_and_missing_field_is_explicit() {
    let before = source();
    let mut after = before.clone();
    let Property::Requested { state, .. } = &mut after.nodes[0].properties[0] else {
        panic!("property")
    };
    *state = Availability::Unknown {
        reason: Id("not_observed".into()),
    };
    after.nodes[0].properties.push(Property::NotRequested {
        field: Field::Placeholder,
    });
    let result = diff(&before, &after, 10);
    let Difference::Property {
        after: Some(new),
        content_changed: true,
        evidence_changed: false,
        ..
    } = result.entries[0]
    else {
        panic!("availability")
    };
    assert!(new.known().is_none());
    assert!(matches!(
        result.entries[1],
        Difference::Property {
            presence: Presence::AfterOnly,
            field: Field::Placeholder,
            before: None,
            after: Some(Property::NotRequested { .. }),
            ..
        }
    ));
}
#[test]
fn evidence_and_observation_only_updates_are_not_value_changes() {
    let before = source();
    for change_observation in [false, true] {
        let mut after = before.clone();
        if change_observation {
            after.observations[0].end += 1.0;
        } else {
            let Property::Requested { evidence, .. } = &mut after.nodes[0].properties[0] else {
                panic!("evidence")
            };
            evidence.method = Id("new_read_method".into());
        }
        let result = diff(&before, &after, 10);
        assert!(!result.entries.is_empty());
        assert!(result.entries.iter().all(|entry| matches!(
            entry,
            Difference::Property {
                content_changed: false,
                evidence_changed: true,
                ..
            }
        )));
    }
    let mut after = before.clone();
    width(&mut after, 48.0);
    after.observations[0].end += 1.0;
    assert!(matches!(
        diff(&before, &after, 10).entries[0],
        Difference::Property {
            content_changed: true,
            evidence_changed: true,
            ..
        }
    ));
}
#[test]
fn transform_evidence_is_compared_separately_from_geometry_content() {
    let mut before = source();
    let Property::Requested { evidence, .. } = &before.nodes[0].properties[0] else {
        panic!("evidence")
    };
    let mut evidence = evidence.clone();
    evidence.method = Id("mapping".into());
    let from = geometry(&mut before).coordinate_space.clone();
    let mut to = from.clone();
    to.id = Id("mapped-space".into());
    let transform = Transform {
        from,
        to,
        affine: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        target: before.context.target.clone(),
        surface: before.nodes[0].surface.clone(),
        environment_revision: before.context.environment_revision.clone(),
        evidence,
    };
    geometry(&mut before).transform = TransformState::Known {
        transform: Box::new(transform),
    };
    let mut after = before.clone();
    let TransformState::Known { transform } = &mut geometry(&mut after).transform else {
        panic!("transform")
    };
    transform.evidence.method = Id("new_mapping_evidence".into());
    let result = diff(&before, &after, 10);
    assert_eq!(result.entries.len(), 1);
    assert!(matches!(
        result.entries[0],
        Difference::Property {
            content_changed: false,
            evidence_changed: true,
            ..
        }
    ));
}
#[test]
fn exact_namespaces_context_and_cap_are_preserved() {
    let before = source();
    let mut after = before.clone();
    after.nodes[0].key.namespace = Id("other-source".into());
    let result = diff(&before, &after, 10);
    assert_eq!(result.entries.len(), 2);
    assert!(matches!(
        result.entries[0],
        Difference::NodePresence {
            presence: Presence::BeforeOnly,
            ..
        }
    ));
    assert!(matches!(
        result.entries[1],
        Difference::NodePresence {
            presence: Presence::AfterOnly,
            ..
        }
    ));
    for cap in [0, 1, 2, usize::MAX] {
        let result = diff(&before, &after, cap);
        assert_eq!(result.entries.len(), cap.min(2));
        assert_eq!(result.omitted_entries, 2 - cap.min(2));
    }
    assert!(diff(&before, &before, 0).entries.is_empty());
    for mismatch in 0..3 {
        let mut after = before.clone();
        match mismatch {
            0 => after.context.environment_revision = Id("changed".into()),
            1 => after.context.target.generation = Id("new".into()),
            _ => after.context.projection = Projection::Design,
        }
        assert!(matches!(
            compare_recorded(&before, &after, DiffLimits { max_entries: 10 }),
            Err(DiffError::IncompatibleContext)
        ));
    }
    let mut invalid = before.clone();
    invalid.nodes.push(invalid.nodes[0].clone());
    assert!(matches!(
        compare_recorded(&before, &invalid, DiffLimits { max_entries: 0 }),
        Err(DiffError::InvalidSnapshot(_))
    ));
}
