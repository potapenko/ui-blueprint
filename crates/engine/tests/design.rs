use uiblueprint_engine::{design::*, *};
use uiblueprint_schema::{analysis::*, model::*};

fn inputs(widths: &[f64]) -> (Snapshot, Vec<GeometryQuery>, EvaluationInput) {
    let doc: Document = serde_json::from_str(include_str!(
        "../../../fixtures/golden/GEO-SIZE-RATIO__width.json"
    ))
    .unwrap();
    let Artifact::Finding(case) = doc.artifact else {
        panic!()
    };
    let mut snapshot = case.snapshot;
    let base = snapshot.nodes[0].clone();
    let query = GeometryQuery::from_expectation(&case.expectation).unwrap();
    let mut queries = vec![];
    snapshot.nodes.clear();
    for (i, width) in widths.iter().enumerate() {
        let mut node = base.clone();
        node.key.key = Id(format!("n{i}"));
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut node.properties[0]
        else {
            panic!()
        };
        let Shape::Rect(r) = &mut g.shape else {
            panic!()
        };
        r.width = *width;
        let mut q = query.clone();
        q.id = Id(format!("q{i}"));
        q.targets = vec![node.key.clone()];
        q.anchors[0].element = node.key.clone();
        queries.push(q);
        snapshot.nodes.push(node);
    }
    let evaluation = EvaluationInput {
        snapshot_id: snapshot.id.clone(),
        revision: snapshot.revision,
        context: snapshot.context.clone(),
        result_space: query.anchors[0].coordinate_space.clone(),
        transforms: vec![],
        conditions: None,
    };
    (snapshot, queries, evaluation)
}

#[test]
fn exact_balance_and_ratio_do_not_require_grid_aligned_widths() {
    let (s, q, e) = inputs(&[179., 179., 8., 366.]);
    let r = compose(&s, &q, &e, Composition::Balance).unwrap();
    assert_eq!(r.value, Some(366.));
    assert_eq!(r.residual, Some(0.));
    let (s, q, e) = inputs(&[12., 8.]);
    assert_eq!(
        compose(&s, &q, &e, Composition::Ratio).unwrap().value,
        Some(1.5)
    );
}
#[test]
fn equality_is_not_based_on_display_rounding() {
    let (s, q, e) = inputs(&[4.1, 4.2]);
    let r = compose(&s, &q, &e, Composition::Equal).unwrap();
    assert!(r.residual.unwrap() > 0.09);
    let (s, q, e) = inputs(&[4.1, 4.2, 4.3]);
    assert_eq!(pattern(&s, &q, &e).unwrap().candidate, None);
}
#[test]
fn candidate_is_a_strict_majority_with_indexed_exceptions() {
    let (s, q, e) = inputs(&[4., 4., 4., 5.]);
    let r = pattern(&s, &q, &e).unwrap();
    assert_eq!(r.candidate, Some(4.));
    assert_eq!(r.support, 3);
    assert_eq!(r.exceptions, vec![(3, 1.)]);
    let (s, q, e) = inputs(&[4., 4., 5., 5.]);
    assert_eq!(pattern(&s, &q, &e).unwrap().candidate, None);
    let (s, q, e) = inputs(&[4., 4., 4.]);
    assert!(pattern(&s, &q, &e).unwrap().exceptions.is_empty());
}
#[test]
fn duplicated_measurements_cannot_manufacture_support() {
    let (s, mut q, e) = inputs(&[4., 4., 5.]);
    q[1] = q[0].clone();
    assert!(pattern(&s, &q, &e).is_err());
}

#[test]
fn clear_gaps_identify_the_five_pixel_exception() {
    let (mut s, q, e) = inputs(&[10.; 5]);
    for (node, x) in s.nodes.iter_mut().zip([0., 14., 28., 42., 57.]) {
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut node.properties[0]
        else {
            panic!()
        };
        let Shape::Rect(r) = &mut g.shape else {
            panic!()
        };
        r.x = x;
    }
    let gaps = q
        .windows(2)
        .map(|pair| {
            let mut query = pair[0].clone();
            query.operation = GeometryRelation::Gap;
            query.targets.push(pair[1].targets[0].clone());
            query.anchors[0].fraction = 1.;
            query.anchors.push(pair[1].anchors[0].clone());
            query.anchors[1].fraction = 0.;
            query
        })
        .collect::<Vec<_>>();
    let result = pattern(&s, &gaps, &e).unwrap();
    assert_eq!(result.candidate, Some(4.));
    assert_eq!(result.exceptions, vec![(3, 1.)]);
}

#[test]
fn mixed_observations_unknown_spaces_and_overflow_do_not_create_advice() {
    let (mut s, q, mut e) = inputs(&[4., 4., 5.]);
    let mut observation = s.observations[0].clone();
    observation.id = Id("other-observation".into());
    s.observations.push(observation);
    let Property::Requested { evidence, .. } = &mut s.nodes[2].properties[0] else {
        panic!()
    };
    evidence.observation_id = Id("other-observation".into());
    assert_eq!(pattern(&s, &q, &e).unwrap().candidate, None);
    e.result_space.id = Id("unrelated-same-unit-space".into());
    assert_eq!(compose(&s, &q, &e, Composition::Equal).unwrap().value, None);
    let (s, q, e) = inputs(&[f64::MAX, f64::MAX, 1.]);
    assert_eq!(
        compose(&s, &q, &e, Composition::Balance).unwrap_err(),
        GeometryError::NonFiniteCalculation
    );
}
#[test]
fn zero_unknown_and_uncertainty_have_no_invented_result() {
    let (s, q, e) = inputs(&[4., 0.]);
    assert_eq!(
        compose(&s, &q, &e, Composition::Ratio).unwrap().unavailable,
        Some("undefined_ratio")
    );
    let (mut s, q, e) = inputs(&[4., 4., 5.]);
    let Property::Requested { evidence, .. } = &mut s.nodes[0].properties[0] else {
        panic!()
    };
    evidence.uncertainty = Some(Uncertainty {
        absolute: 1.,
        units: Some(Unit::CssPx),
        method: Id("test".into()),
    });
    assert_eq!(pattern(&s, &q, &e).unwrap().candidate, None);
    let Property::Requested { state, .. } = &mut s.nodes[0].properties[0] else {
        panic!()
    };
    *state = Availability::Unknown {
        reason: Id("not_measured".into()),
    };
    let r = pattern(&s, &q, &e).unwrap();
    assert_eq!(r.unavailable, Some("unknown_operand"));
    assert_eq!(r.measurements.len(), 3);
}
#[test]
fn incompatible_binding_units_and_frames_reject() {
    let (s, mut q, mut e) = inputs(&[4., 4., 5.]);
    e.revision += 1;
    assert!(pattern(&s, &q, &e).is_err());
    e.revision -= 1;
    q[1].anchors[0].frame_kind = FrameKind::VisibleRegion;
    assert!(pattern(&s, &q, &e).is_err());
    q[1].anchors[0].frame_kind = FrameKind::LayoutBounds;
    q[1].units = Unit::Pt;
    assert!(pattern(&s, &q, &e).is_err());
}
