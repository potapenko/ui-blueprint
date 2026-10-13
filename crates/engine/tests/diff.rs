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
    for mismatch in 0..7 {
        let mut after = before.clone();
        match mismatch {
            0 => after.context.session_id = Id("other-session".into()),
            1 => after.context.target.generation = Id("new".into()),
            2 => after.context.projection = Projection::Design,
            3 => after.context.plugin.version = Id("other-version".into()),
            4 => {
                after.context.scope_id = Id("other-scope".into());
                after.coverage.scope_id = after.context.scope_id.clone();
                for observation in &mut after.observations {
                    observation.coverage.scope_id = after.context.scope_id.clone();
                }
            }
            5 => {
                after.context.surfaces[0].generation = Id("other-surface-generation".into());
                for node in &mut after.nodes {
                    node.surface = after.context.surfaces[0].clone();
                }
            }
            _ => {
                after.context.fields.push(Field::Value);
                after.coverage.fields.push(Field::Value);
                for observation in &mut after.observations {
                    observation.coverage.fields.push(Field::Value);
                }
                for node in &mut after.nodes {
                    let Property::Requested { evidence, .. } = &node.properties[0] else {
                        panic!("evidence")
                    };
                    node.properties.push(Property::Requested {
                        field: Field::Value,
                        sensitivity: Sensitivity::Public,
                        evidence: evidence.clone(),
                        state: Availability::Unknown {
                            reason: Id("not_available".into()),
                        },
                    });
                }
            }
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

#[test]
fn environment_changes_are_attributed_records_while_delta_compatibility_stays_strict() {
    let mut before = source();
    width(&mut before, 32.0);
    let mut after = before.clone();
    after.context.environment_revision = Id("font-size-after".into());
    width(&mut after, 48.0);
    let originals = (
        serde_json::to_vec(&before).unwrap(),
        serde_json::to_vec(&after).unwrap(),
    );
    assert!(
        !validation::contexts_compatible(&before.context, &after.context),
        "Delta/cache guard stays strict"
    );
    let result = diff(&before, &after, 10);
    assert_eq!(result.entries.len(), 1);
    assert_eq!(
        result.before.context.environment_revision,
        before.context.environment_revision
    );
    assert_eq!(
        result.after.context.environment_revision,
        after.context.environment_revision
    );
    assert!(matches!(
        result.entries[0],
        Difference::Property {
            content_changed: true,
            ..
        }
    ));
    assert_eq!(
        (
            serde_json::to_vec(&before).unwrap(),
            serde_json::to_vec(&after).unwrap()
        ),
        originals
    );
    geometry(&mut after).coordinate_space.units = Unit::Pt;
    geometry(&mut after).coordinate_space.id = Id("other-recorded-space".into());
    let result = diff(&before, &after, 10);
    let Difference::Property {
        before: Some(old),
        after: Some(new),
        ..
    } = result.entries[0]
    else {
        panic!("property")
    };
    let Some(Value::Geometry(a)) = old.known() else {
        panic!("before")
    };
    let Some(Value::Geometry(b)) = new.known() else {
        panic!("after")
    };
    assert_eq!(a.coordinate_space.units, Unit::CssPx);
    assert_eq!(b.coordinate_space.units, Unit::Pt);
    // Only original records are returned; no converted value/displacement exists.
    assert_eq!(b.coordinate_space.id.0, "other-recorded-space");
}

fn evaluation(s: &Snapshot, space: Space) -> uiblueprint_schema::analysis::EvaluationInput {
    uiblueprint_schema::analysis::EvaluationInput {
        snapshot_id: s.id.clone(),
        revision: s.revision,
        context: s.context.clone(),
        result_space: space,
        transforms: vec![],
        conditions: None,
    }
}
fn translated(s: &mut Snapshot, from: &Space, to: &Space, x: f64, y: f64, tx: f64, ty: f64) {
    let Property::Requested { evidence, .. } = &s.nodes[0].properties[0] else {
        panic!("source")
    };
    let transform = Transform {
        from: from.clone(),
        to: to.clone(),
        affine: [1.0, 0.0, 0.0, 1.0, tx, ty],
        target: s.context.target.clone(),
        surface: s.nodes[0].surface.clone(),
        environment_revision: s.context.environment_revision.clone(),
        evidence: evidence.clone(),
    };
    let g = geometry(s);
    g.coordinate_space = from.clone();
    g.transform = TransformState::Known {
        transform: Box::new(transform),
    };
    let Shape::Rect(r) = &mut g.shape else {
        panic!("rect")
    };
    r.x = x;
    r.y = y;
}
#[test]
fn geometry_comparison_separates_sourced_window_and_scroll_motion_from_rect_changes() {
    use uiblueprint_engine::diff::{ResolvedRect, compare_geometry};
    for scroll in [false, true] {
        let mut a = source();
        let mut b = a.clone();
        b.context.environment_revision = Id("after-environment".into());
        let mut original = geometry(&mut a).coordinate_space.clone();
        original.id = Id(if scroll { "viewport" } else { "screen" }.into());
        original.kind = if scroll {
            SpaceKind::Viewport
        } else {
            SpaceKind::Screen
        };
        let mut local = original.clone();
        local.id = Id(if scroll { "document" } else { "window-local" }.into());
        local.kind = if scroll {
            SpaceKind::Document
        } else {
            SpaceKind::Local
        };
        if scroll {
            translated(&mut a, &original, &local, 10.0, 100.0, 0.0, 0.0);
            translated(&mut b, &original, &local, 10.0, 50.0, 0.0, 50.0);
        } else {
            translated(&mut a, &original, &local, 110.0, 20.0, -100.0, 0.0);
            translated(&mut b, &original, &local, 210.0, 20.0, -200.0, 0.0);
        }
        let originals = (a.clone(), b.clone());
        let key = a.nodes[0].key.clone();
        let old = evaluation(&a, original.clone());
        let new = evaluation(&b, original);
        let screen = compare_geometry(&a, &b, &key, FrameKind::LayoutBounds, &old, &new).unwrap();
        assert_eq!(
            if scroll {
                screen.displacement.unwrap().dy
            } else {
                screen.displacement.unwrap().dx
            },
            if scroll { -50.0 } else { 100.0 }
        );
        let old = evaluation(&a, local.clone());
        let new = evaluation(&b, local.clone());
        let resolved = compare_geometry(&a, &b, &key, FrameKind::LayoutBounds, &old, &new).unwrap();
        let delta = resolved.displacement.unwrap();
        assert_eq!(
            (delta.dx, delta.dy, delta.dwidth, delta.dheight),
            (0.0, 0.0, 0.0, 0.0)
        );
        assert!(
            matches!(&resolved.before_geometry,ResolvedRect::Known{evidence,..} if !evidence.is_empty())
        );
        assert_eq!((a.clone(), b.clone()), originals);
        width(&mut b, 48.0);
        let new = evaluation(&b, local);
        assert_eq!(
            compare_geometry(&a, &b, &key, FrameKind::LayoutBounds, &old, &new)
                .unwrap()
                .displacement
                .unwrap()
                .dwidth,
            18.0
        );
    }
}
#[test]
fn geometric_unknown_mapping_never_becomes_zero_and_wrong_bindings_refuse() {
    use uiblueprint_engine::{
        GeometryError, UnknownReason,
        diff::{ResolvedRect, compare_geometry},
    };
    let mut a = source();
    let b = a.clone();
    let key = a.nodes[0].key.clone();
    let own = geometry(&mut a).coordinate_space.clone();
    let mut local = own.clone();
    local.id = Id("missing-local-path".into());
    let old = evaluation(&a, local.clone());
    let new = evaluation(&b, local);
    let result = compare_geometry(&a, &b, &key, FrameKind::LayoutBounds, &old, &new).unwrap();
    assert!(result.displacement.is_none());
    assert!(matches!(
        result.before_geometry,
        ResolvedRect::Unknown {
            reason: UnknownReason::MissingTransform,
            ..
        }
    ));
    let mut bad = new.clone();
    bad.snapshot_id = Id("wrong".into());
    assert!(compare_geometry(&a, &b, &key, FrameKind::LayoutBounds, &old, &bad).is_err());
    bad = new.clone();
    bad.result_space.units = Unit::Pt;
    assert!(matches!(
        compare_geometry(&a, &b, &key, FrameKind::LayoutBounds, &old, &bad),
        Err(GeometryError::InvalidInput(
            validation::ValidationError::IncompatibleContext
        ))
    ));
    let mut other = b.clone();
    other.context.surfaces[0].generation = Id("other".into());
    other.nodes[0].surface = other.context.surfaces[0].clone();
    other.nodes[1].surface = other.context.surfaces[0].clone();
    let input = evaluation(&other, own);
    assert!(compare_geometry(&a, &other, &key, FrameKind::LayoutBounds, &old, &input).is_err());
}

mod declared_correspondence {
    use super::*;
    use uiblueprint_engine::{MeasurementResult, diff::*};
    use uiblueprint_schema::analysis::*;

    fn binding(s: &Snapshot) -> SnapshotBinding {
        SnapshotBinding {
            snapshot_id: s.id.clone(),
            revision: s.revision,
            context: s.context.clone(),
        }
    }
    fn pair() -> (Snapshot, Snapshot, Correspondence) {
        let doc = Document::from_json(
            include_bytes!("../../../fixtures/golden/GEO-GAP.json"),
            65536,
        )
        .unwrap();
        let Artifact::Finding(case) = doc.artifact else {
            panic!("fixture")
        };
        let mut a = case.snapshot;
        a.nodes.truncate(2);
        for node in &mut a.nodes {
            node.children.clear();
        }
        a.relations.clear();
        a.components.clear();
        let mut b = a.clone();
        b.id = Id("after".into());
        b.context.session_id = Id("other-session".into());
        b.context.surfaces[0] = Identity {
            id: Id("other-window".into()),
            generation: Id("other-generation".into()),
        };
        for n in &mut b.nodes {
            n.key.key.0.insert_str(0, "after-");
            n.surface = b.context.surfaces[0].clone();
        }
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut b.nodes[1].properties[0]
        else {
            panic!("geometry")
        };
        let Shape::Rect(rect) = &mut g.shape else {
            panic!("rect")
        };
        rect.x = 60.0;
        let node_pairs = a
            .nodes
            .iter()
            .zip(&b.nodes)
            .enumerate()
            .map(|(i, (x, y))| NodePair {
                id: Id(format!("pair-{i}")),
                before: x.key.clone(),
                after: y.key.clone(),
                declaration: Id("authored QA correspondence, not identity".into()),
            })
            .collect();
        let old = GeometryQuery::from_expectation(&case.expectation).unwrap();
        let mut new = old.clone();
        for (k, n) in new.targets.iter_mut().zip(&b.nodes) {
            *k = n.key.clone();
        }
        for (anchor, n) in new.anchors.iter_mut().zip(&b.nodes) {
            anchor.element = n.key.clone();
        }
        let evaluation = |s: &Snapshot| EvaluationInput {
            snapshot_id: s.id.clone(),
            revision: s.revision,
            context: s.context.clone(),
            result_space: old.anchors[0].coordinate_space.clone(),
            transforms: vec![],
            conditions: None,
        };
        let c = Correspondence {
            before_binding: binding(&a),
            after_binding: binding(&b),
            surface_pairs: vec![SurfacePair {
                before: a.context.surfaces[0].clone(),
                after: b.context.surfaces[0].clone(),
                declaration: Id("authored windows".into()),
            }],
            node_pairs,
            measurement_pairs: vec![MeasurementPair {
                id: Id("gap".into()),
                before_evaluation: evaluation(&a),
                after_evaluation: evaluation(&b),
                before_query: old,
                after_query: new,
            }],
        };
        (a, b, c)
    }
    fn amount(v: &MeasurementResult) -> f64 {
        let MeasurementResult::Known { measurement } = v else {
            panic!("known")
        };
        let Value::Quantity { amount, .. } = measurement.value else {
            panic!("quantity")
        };
        amount
    }
    #[test]
    fn independent_lengths_and_original_sources_with_strict_legacy_refusal() {
        let (a, b, c) = pair();
        let before = a.clone();
        let after = b.clone();
        let r = compare_corresponded(&a, &b, &c, 100).unwrap();
        assert_eq!(
            (
                amount(&r.measurements[0].before),
                amount(&r.measurements[0].after),
                r.measurements[0].change
            ),
            (8.0, 20.0, Some(12.0))
        );
        assert!(r.entries.iter().any(|v| matches!(
            v,
            CorrespondedEntry::Property {
                content_changed: true,
                field: Field::LayoutBounds,
                ..
            }
        )));
        assert_eq!(r.omitted_entries, 0);
        assert_eq!(a, before);
        assert_eq!(b, after);
        assert!(matches!(
            compare_recorded(&a, &b, DiffLimits { max_entries: 100 }),
            Err(DiffError::IncompatibleContext)
        ));
        let d = diagnose_comparison(&a, &b).unwrap();
        assert_eq!(d.strict_diff_blocking_axes, vec!["session_id", "surfaces"]);
        assert_eq!(d.exact_key_matches, 0);
        assert!(d.protected_basis_compatible);
        assert!(!d.strict_diff_compatible);
    }
    #[test]
    fn selected_children_order_preserves_unmapped_scope() {
        let (mut a, mut b, mut c) = pair();
        c.measurement_pairs.clear();
        for s in [&mut a, &mut b] {
            let mut n = s.nodes[1].clone();
            n.key.key.0.push_str("-unmapped");
            s.nodes.push(n);
            s.nodes[0].children = vec![s.nodes[1].key.clone(), s.nodes[2].key.clone()];
        }
        b.nodes[0].children.reverse();
        let r = compare_corresponded(&a, &b, &c, 100).unwrap();
        assert_eq!(r.unexamined_before_nodes, 1);
        assert_eq!(
            mapped_child_positions(&r, 0, 0).unwrap(),
            vec![MappedChildPosition {
                child_pair_index: 1,
                before_position: Some(0),
                after_position: Some(1)
            }]
        );
        assert!(r.entries.iter().any(|v| matches!(
            v,
            CorrespondedEntry::Children {
                pair_index: 0,
                mapped_order_changed: false,
                unmapped_before_children: 1,
                unmapped_after_children: 1,
                ..
            }
        )));
        c.node_pairs.push(NodePair {
            id: Id("third".into()),
            before: a.nodes[2].key.clone(),
            after: b.nodes[2].key.clone(),
            declaration: Id("explicit third".into()),
        });
        b.nodes[0].children = vec![b.nodes[2].key.clone(), b.nodes[1].key.clone()];
        let r = compare_corresponded(&a, &b, &c, 100).unwrap();
        assert!(r.entries.iter().any(|v| matches!(
            v,
            CorrespondedEntry::Children {
                pair_index: 0,
                mapped_order_changed: true,
                unmapped_before_children: 0,
                unmapped_after_children: 0,
                ..
            }
        )));
    }
    #[test]
    fn binding_one_to_one_dangling_surface_and_query_guards() {
        let (a, b, c) = pair();
        for variant in 0..8 {
            let mut bad = c.clone();
            let expected = match variant {
                0 => {
                    bad.before_binding.revision += 1;
                    CorrespondenceError::BindingMismatch
                }
                1 => {
                    bad.node_pairs[1].after = bad.node_pairs[0].after.clone();
                    CorrespondenceError::InvalidPair
                }
                2 => {
                    bad.node_pairs[0].after.key = Id("missing".into());
                    CorrespondenceError::InvalidPair
                }
                3 => {
                    bad.surface_pairs[0].after.generation = Id("missing".into());
                    CorrespondenceError::InvalidPair
                }
                4 => {
                    bad.node_pairs[0].id = bad.node_pairs[1].id.clone();
                    CorrespondenceError::InvalidPair
                }
                5 => {
                    bad.measurement_pairs[0]
                        .after_evaluation
                        .context
                        .environment_revision = Id("fabricated".into());
                    CorrespondenceError::InvalidMeasurement
                }
                6 => {
                    bad.measurement_pairs[0].after_query.anchors.reverse();
                    CorrespondenceError::InvalidMeasurement
                }
                _ => {
                    bad.measurement_pairs[0].after_query.units = Unit::Pt;
                    CorrespondenceError::InvalidMeasurement
                }
            };
            assert!(
                matches!(compare_corresponded(&a,&b,&bad,100),Err(e) if e==expected),
                "variant {variant}"
            );
        }
        let mut other = b.clone();
        other.context.target.generation = Id("restarted".into());
        let mut bad = c.clone();
        bad.after_binding = binding(&other);
        assert!(matches!(
            compare_corresponded(&a, &other, &bad, 100),
            Err(CorrespondenceError::ProtectedBasis)
        ));
    }
    #[test]
    fn unknown_redacted_and_zero_cap_are_not_known_zero_or_old_values() {
        let (a, mut b, c) = pair();
        for state in [
            Availability::Unknown {
                reason: Id("not_observed".into()),
            },
            Availability::Unsupported {
                reason: Id("not_supported".into()),
            },
            Availability::Redacted {},
        ] {
            let Property::Requested { state: actual, .. } = &mut b.nodes[1].properties[0] else {
                panic!("property")
            };
            *actual = state;
            let r = compare_corresponded(&a, &b, &c, 100).unwrap();
            assert_eq!(r.measurements[0].change, None);
            assert_eq!(r.unknown_measurements, 1);
            assert!(matches!(
                r.measurements[0].after,
                MeasurementResult::Unknown { .. }
            ));
            let zero = compare_corresponded(&a, &b, &c, 0).unwrap();
            assert!(zero.entries.is_empty() && zero.measurements.is_empty());
            assert_eq!(zero.omitted_entries, r.entries.len() + r.measurements.len());
        }
    }
    #[test]
    fn finite_input_with_overflowing_gap_refuses_internal_arithmetic() {
        let (a, mut b, c) = pair();
        for (node, x) in b.nodes.iter_mut().zip([-1.5e308, 1.5e308]) {
            let Property::Requested {
                state:
                    Availability::Known {
                        value: Value::Geometry(g),
                    },
                ..
            } = &mut node.properties[0]
            else {
                panic!("geometry")
            };
            let Shape::Rect(rect) = &mut g.shape else {
                panic!("rect")
            };
            rect.x = x;
        }
        assert!(matches!(
            compare_corresponded(&a, &b, &c, 100),
            Err(CorrespondenceError::NonFinite)
        ));
    }
    #[test]
    fn namespace_separation_and_missing_transform_remain_explicit() {
        let (a, b, mut c) = pair();
        let mut other = b.clone();
        for node in &mut other.nodes {
            node.key.namespace = Id("fixture.other".into());
            for property in &mut node.properties {
                if let Property::Requested { evidence, .. } = property {
                    evidence.source_namespace = Id("fixture.other".into());
                }
            }
        }
        for observation in &mut other.observations {
            observation.source_namespace = Id("fixture.other".into());
        }
        let mut bad = c.clone();
        bad.measurement_pairs.clear();
        for (p, n) in bad.node_pairs.iter_mut().zip(&other.nodes) {
            p.after = n.key.clone();
        }
        assert!(matches!(
            compare_corresponded(&a, &other, &bad, 100),
            Err(CorrespondenceError::InvalidPair)
        ));
        c.measurement_pairs[0].after_evaluation.result_space.id = Id("other-local".into());
        let r = compare_corresponded(&a, &b, &c, 100).unwrap();
        assert_eq!(r.measurements[0].change, None);
        assert!(matches!(
            r.measurements[0].after,
            MeasurementResult::Unknown {
                reason: uiblueprint_engine::UnknownReason::MissingTransform,
                ..
            }
        ));
    }
}
