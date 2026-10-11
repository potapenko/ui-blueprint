//! CE1 synthetic adversarial vectors; these are not real UI corpus cases.
use uiblueprint_engine::{design::*, *};
use uiblueprint_schema::{analysis::*, model::*};

fn inputs(widths: &[f64]) -> (Snapshot, Vec<GeometryQuery>, EvaluationInput) {
    let doc: Document = serde_json::from_str(include_str!(
        "../../../fixtures/golden/GEO-SIZE-RATIO__width.json"
    ))
    .unwrap();
    let Artifact::Finding(case) = doc.artifact else {
        panic!("finding fixture")
    };
    let mut s = case.snapshot;
    let base = s.nodes[0].clone();
    let template = GeometryQuery::from_expectation(&case.expectation).unwrap();
    s.nodes.clear();
    let mut queries = Vec::new();
    for (i, width) in widths.iter().enumerate() {
        let mut node = base.clone();
        node.key.key = Id(format!("corpus-{i}"));
        s.nodes.push(node);
        rect(&mut s, i).x = 0.;
        rect(&mut s, i).width = *width;
        let mut q = template.clone();
        q.id = Id(format!("query-{i}"));
        q.targets = vec![s.nodes[i].key.clone()];
        q.anchors[0].element = q.targets[0].clone();
        queries.push(q);
    }
    let e = EvaluationInput {
        snapshot_id: s.id.clone(),
        revision: s.revision,
        context: s.context.clone(),
        result_space: template.anchors[0].coordinate_space.clone(),
        transforms: vec![],
        conditions: None,
    };
    (s, queries, e)
}
fn rect(s: &mut Snapshot, i: usize) -> &mut Rect {
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut s.nodes[i].properties[0]
    else {
        panic!("geometry")
    };
    let Shape::Rect(r) = &mut g.shape else {
        panic!("rect")
    };
    r
}
fn evidence(s: &mut Snapshot, i: usize) -> &mut Evidence {
    let Property::Requested { evidence, .. } = &mut s.nodes[i].properties[0] else {
        panic!("requested")
    };
    evidence
}
fn gap_inputs(xs: &[f64]) -> (Snapshot, Vec<GeometryQuery>, EvaluationInput) {
    let (mut s, widths, e) = inputs(&vec![10.; xs.len()]);
    for (i, x) in xs.iter().enumerate() {
        rect(&mut s, i).x = *x;
    }
    let gaps = widths
        .windows(2)
        .map(|pair| {
            let mut q = pair[0].clone();
            q.operation = GeometryRelation::Gap;
            q.targets.push(pair[1].targets[0].clone());
            q.anchors[0].fraction = 1.;
            q.anchors.push(pair[1].anchors[0].clone());
            q.anchors[1].fraction = 0.;
            q
        })
        .collect();
    (s, gaps, e)
}

#[test]
fn arithmetic_uses_exact_literals_including_remainders_and_signed_gaps() {
    for (widths, op, value, residual) in [
        (
            vec![179., 179., 8., 366.],
            Composition::Balance,
            366.,
            Some(0.),
        ),
        (vec![10., 20., 31.], Composition::Balance, 30., Some(-1.)),
        (
            vec![0.125, 0.25, 0.375],
            Composition::Balance,
            0.375,
            Some(0.),
        ),
        (vec![4.125, 4.25], Composition::Equal, 4.125, Some(0.125)),
        (vec![1., 8.], Composition::Ratio, 0.125, None),
        (vec![0., 8.], Composition::Ratio, 0., None),
        (vec![f64::MAX, f64::MAX], Composition::Ratio, 1., None),
    ] {
        let (s, q, e) = inputs(&widths);
        let r = compose(&s, &q, &e, op).unwrap();
        assert_eq!(
            (r.value, r.residual, r.unavailable),
            (Some(value), residual, None),
            "{widths:?}"
        );
        assert_eq!(r.measurements.len(), widths.len());
    }
    let (s, q, e) = gap_inputs(&[0., 6., 12.]);
    assert_eq!(
        compose(&s, &q, &e, Composition::Equal).unwrap().value,
        Some(-4.)
    );
    for zero in [0., -0.] {
        let (s, q, e) = inputs(&[4., zero]);
        let r = compose(&s, &q, &e, Composition::Ratio).unwrap();
        assert_eq!(
            (r.value, r.residual, r.unavailable),
            (None, None, Some("undefined_ratio"))
        );
    }
}

#[test]
fn majority_preserves_exact_values_original_indices_and_negative_deviations() {
    for (widths, candidate, support, exceptions) in [
        (vec![5., 4., 4., 4.], Some(4.), 3, vec![(0, 1.)]),
        (vec![3., 4., 4., 4.], Some(4.), 3, vec![(0, -1.)]),
        (
            vec![4., 5., 4., 6., 4.],
            Some(4.),
            3,
            vec![(1, 1.), (3, 2.)],
        ),
        (vec![4., 4., 5., 5.], None, 0, vec![]),
        (vec![4., 4., 5., 5., 6.], None, 0, vec![]),
        (vec![4.125, 4.25, 4.375], None, 0, vec![]),
        (vec![4.125, 4.125, 4.25], Some(4.125), 2, vec![(2, 0.125)]),
        (vec![0., -0., 0.], Some(0.), 3, vec![]),
    ] {
        let (s, q, e) = inputs(&widths);
        let r = pattern(&s, &q, &e).unwrap();
        assert_eq!(
            (r.candidate, r.support, r.exceptions, r.unavailable),
            (candidate, support, exceptions, None),
            "{widths:?}"
        );
    }
    for (positions, candidate, deviation) in [
        ([0., 14., 28., 42., 57.], 4., 1.),
        ([0., 6., 12., 18., 23.], -4., -1.),
    ] {
        let (s, q, e) = gap_inputs(&positions);
        let r = pattern(&s, &q, &e).unwrap();
        assert_eq!(
            (r.candidate, r.exceptions),
            (Some(candidate), vec![(3, deviation)])
        );
    }
}

#[test]
fn bound_context_mismatches_are_rejected_without_mutating_input() {
    let (s, q, e) = inputs(&[4., 4., 5.]);
    let baseline = s.clone();
    for field in 0..9 {
        let mut bad = e.clone();
        match field {
            0 => bad.snapshot_id = Id("other".into()),
            1 => bad.revision += 1,
            2 => bad.context.session_id = Id("other".into()),
            3 => bad.context.target.generation = Id("other".into()),
            4 => bad.context.surfaces[0].generation = Id("other".into()),
            5 => bad.context.scope_id = Id("other".into()),
            6 => bad.context.environment_revision = Id("other".into()),
            7 => bad.context.fields.clear(),
            _ => bad.context.plugin.id = Id("other".into()),
        }
        assert!(pattern(&s, &q, &bad).is_err(), "binding mutation {field}");
        assert!(compose(&s, &q, &bad, Composition::Equal).is_err());
    }
    assert_eq!(s, baseline);
}

#[test]
fn unavailable_operand_is_retained_in_every_position() {
    for index in 0..3 {
        for availability in [
            Availability::Unknown {
                reason: Id("missing".into()),
            },
            Availability::Unsupported {
                reason: Id("not_exposed".into()),
            },
            Availability::Redacted {},
        ] {
            let (mut s, q, e) = inputs(&[4., 4., 5.]);
            let Property::Requested { state, .. } = &mut s.nodes[index].properties[0] else {
                panic!()
            };
            *state = availability;
            let r = pattern(&s, &q, &e).unwrap();
            assert_eq!(
                (r.candidate, r.unavailable),
                (None, Some("unknown_operand"))
            );
            assert_eq!(r.measurements.len(), 3);
            assert!(matches!(
                r.measurements[index],
                MeasurementResult::Unknown { .. }
            ));
            assert_eq!(
                compose(&s, &q, &e, Composition::Balance).unwrap().value,
                None
            );
        }
    }
}

#[test]
fn observations_uncertainty_and_selected_state_gate_recommendations() {
    for absolute in [0., 0.125] {
        let (mut s, q, e) = inputs(&[4., 4., 5.]);
        evidence(&mut s, 0).uncertainty = Some(Uncertainty {
            absolute,
            units: Some(Unit::CssPx),
            method: Id("reported".into()),
        });
        assert_eq!(
            pattern(&s, &q, &e).unwrap().candidate,
            if absolute == 0. { Some(4.) } else { None }
        );
    }
    let (mut s, mut q, mut e) = inputs(&[4., 4., 5.]);
    let mut other = s.observations[0].clone();
    other.id = Id("other".into());
    s.observations.push(other);
    evidence(&mut s, 2).observation_id = Id("other".into());
    assert_eq!(
        pattern(&s, &q, &e).unwrap().unavailable,
        Some("incomparable_observations_or_uncertainty")
    );
    evidence(&mut s, 2).observation_id = s.observations[0].id.clone();
    for query in &mut q {
        query.applies_when.platform = Some(Id("selected-platform".into()));
    }
    assert_eq!(
        pattern(&s, &q, &e).unwrap().unavailable,
        Some("unknown_operand")
    );
    e.conditions = Some(ObservedConditions {
        values: q[0].applies_when.clone(),
        evidence: evidence(&mut s, 0).clone(),
    });
    e.conditions.as_mut().unwrap().values.platform = Some(Id("different-platform".into()));
    let r = pattern(&s, &q, &e).unwrap();
    assert_eq!(r.candidate, None);
    assert!(matches!(
        r.measurements[0],
        MeasurementResult::Unknown {
            reason: MeasurementUnknownReason::NotApplicable,
            ..
        }
    ));
    e.conditions.as_mut().unwrap().values.platform = Some(Id("selected-platform".into()));
    assert_eq!(pattern(&s, &q, &e).unwrap().candidate, Some(4.));
}

#[test]
fn same_units_do_not_join_unrelated_spaces_and_mixed_frames_reject() {
    let (s, mut q, mut e) = inputs(&[4., 4., 5.]);
    e.result_space.id = Id("unrelated".into());
    assert_eq!(
        pattern(&s, &q, &e).unwrap().unavailable,
        Some("unknown_operand")
    );
    e.result_space = q[0].anchors[0].coordinate_space.clone();
    q[1].anchors[0].frame_kind = FrameKind::AccessibilityBounds;
    assert!(pattern(&s, &q, &e).is_err());
    assert!(compose(&s, &q, &e, Composition::Equal).is_err());
}

#[test]
fn overflow_is_typed_and_never_a_finite_recommendation() {
    for (widths, op) in [
        (vec![f64::MAX, f64::MAX, 1.], Composition::Balance),
        (vec![f64::MAX, 0.125], Composition::Ratio),
    ] {
        let (s, q, e) = inputs(&widths);
        assert_eq!(
            compose(&s, &q, &e, op).unwrap_err(),
            GeometryError::NonFiniteCalculation
        );
    }
}

#[test]
fn duplicate_width_with_different_fraction_cannot_manufacture_majority() {
    let (s, mut q, e) = inputs(&[4., 5.]);
    let mut duplicate = q[0].clone();
    duplicate.id = Id("different-query-name".into());
    duplicate.anchors[0].fraction = 0.; // Width does not depend on this position.
    q.insert(1, duplicate);
    let result = pattern(&s, &q, &e);
    assert!(
        result.is_err(),
        "only two distinct measurements: actual candidate/support/exceptions {:?}",
        result.map(|r| (r.candidate, r.support, r.exceptions))
    );
}

#[test]
fn arity_units_axis_and_operation_are_not_silently_coerced() {
    let (s, q, e) = inputs(&[4., 4., 5.]);
    for count in [0, 1, 2] {
        assert!(pattern(&s, &q[..count], &e).is_err());
    }
    assert!(compose(&s, &q, &e, Composition::Ratio).is_err());
    for mutation in 0..3 {
        let mut bad = q.clone();
        match mutation {
            0 => bad[1].units = Unit::Pt,
            1 => bad[1].anchors[0].axis = Id("y".into()),
            _ => bad[1].operation = GeometryRelation::Height,
        }
        assert!(pattern(&s, &bad, &e).is_err(), "mutation {mutation}");
    }
    let mut duplicate = q.clone();
    duplicate[1] = q[0].clone();
    duplicate[1].id = Id("different-id-only".into());
    assert!(pattern(&s, &duplicate, &e).is_err());
}

#[test]
fn unresolved_target_is_unknown_and_height_uses_its_own_measurement() {
    let (mut s, mut q, e) = inputs(&[4., 4., 5.]);
    q[2].targets[0].key = Id("missing".into());
    q[2].anchors[0].element = q[2].targets[0].clone();
    let r = pattern(&s, &q, &e).unwrap();
    assert_eq!(r.unavailable, Some("unknown_operand"));
    assert!(matches!(
        r.measurements[2],
        MeasurementResult::Unknown {
            reason: MeasurementUnknownReason::MissingTarget,
            ..
        }
    ));
    q[2].targets[0] = s.nodes[2].key.clone();
    q[2].anchors[0].element = s.nodes[2].key.clone();
    for (i, query) in q.iter_mut().enumerate() {
        query.operation = GeometryRelation::Height;
        query.anchors[0].axis = Id("y".into());
        rect(&mut s, i).height = [8., 9., 8.][i];
    }
    let r = pattern(&s, &q, &e).unwrap();
    assert_eq!((r.candidate, r.exceptions), (Some(8.), vec![(1, 1.)]));
}

#[test]
fn duplicate_height_with_different_fraction_cannot_manufacture_majority() {
    for fraction in [0., 0.25, 1.] {
        let (mut s, mut q, e) = inputs(&[4., 5.]);
        for (i, query) in q.iter_mut().enumerate() {
            query.operation = GeometryRelation::Height;
            query.anchors[0].axis = Id("y".into());
            rect(&mut s, i).height = [8., 9.][i];
        }
        let mut duplicate = q[0].clone();
        duplicate.id = Id("another-height-query".into());
        duplicate.anchors[0].fraction = fraction;
        q.insert(1, duplicate);
        assert_eq!(pattern(&s, &q, &e).unwrap_err(), GeometryError::InvalidRule);
    }
}

#[test]
fn distinct_sizes_preserve_original_anchors_and_measurement_evidence() {
    for operation in [GeometryRelation::Width, GeometryRelation::Height] {
        let (mut s, mut q, e) = inputs(&[4., 4., 5.]);
        for (i, query) in q.iter_mut().enumerate() {
            query.operation = operation;
            query.anchors[0].fraction = [0., 0.25, 1.][i];
            if operation == GeometryRelation::Height {
                query.anchors[0].axis = Id("y".into());
                rect(&mut s, i).height = [4., 4., 5.][i];
            }
        }
        let original = (s.clone(), q.clone(), e.clone());
        let result = pattern(&s, &q, &e).unwrap();
        assert_eq!(
            (result.candidate, result.support, result.exceptions),
            (Some(4.), 2, vec![(2, 1.)])
        );
        for (i, measurement) in result.measurements.iter().enumerate() {
            let MeasurementResult::Known { measurement } = measurement else {
                panic!("known size")
            };
            assert_eq!(
                measurement.value,
                Value::Quantity {
                    amount: [4., 4., 5.][i],
                    kind: QuantityKind::Length,
                    source_units: Unit::CssPx
                }
            );
            assert_eq!(measurement.evidence, vec![evidence(&mut s, i).clone()]);
            assert_eq!(measurement.space, e.result_space);
        }
        assert_eq!((s, q, e), original);
    }
}

#[test]
fn gap_fractions_remain_distinct_measurements_on_the_same_elements() {
    let (s, gaps, e) = gap_inputs(&[0., 14.]);
    let queries = [(0., 0.), (0.5, 0.5), (1., 0.)]
        .into_iter()
        .enumerate()
        .map(|(i, (from, to))| {
            let mut q = gaps[0].clone();
            q.id = Id(format!("gap-{i}"));
            q.anchors[0].fraction = from;
            q.anchors[1].fraction = to;
            q
        })
        .collect::<Vec<_>>();
    let result = pattern(&s, &queries, &e).unwrap();
    assert_eq!(
        (result.candidate, result.support, result.exceptions),
        (Some(14.), 2, vec![(2, -10.)])
    );
    let mut duplicate = queries.clone();
    duplicate[1] = queries[0].clone();
    assert_eq!(
        pattern(&s, &duplicate, &e).unwrap_err(),
        GeometryError::InvalidRule
    );
}
