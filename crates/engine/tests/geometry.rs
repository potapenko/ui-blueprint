use uiblueprint_engine::*;
use uiblueprint_schema::{model::*, validation};

fn fixture(name: &str) -> FindingCase {
    let path = format!(
        "{}/../../fixtures/golden/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path).expect("canonical fixture")).expect("JSON");
    serde_json::from_value(json["artifact"]["data"].clone()).expect("canonical finding case")
}
fn space(case: &FindingCase) -> Space {
    let Rule::Geometry { anchors, .. } = &case.expectation.rule else {
        panic!("geometry rule")
    };
    anchors[0].coordinate_space.clone()
}
fn run(case: &FindingCase) -> Check {
    check(
        &case.snapshot,
        &case.expectation,
        &EvaluationContext {
            space: &space(case),
            transforms: &[],
            conditions: None,
        },
    )
    .expect("valid calculation")
}
fn amount(result: &Check) -> f64 {
    let Some(Value::Quantity { amount, .. }) = result.finding.measured else {
        panic!("measured quantity: {result:?}")
    };
    amount
}
fn rect(case: &mut FindingCase, index: usize) -> &mut Rect {
    let property = case.snapshot.nodes[index]
        .properties
        .iter_mut()
        .find(|p| p.field() == Field::LayoutBounds)
        .expect("layout");
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = property
    else {
        panic!("geometry")
    };
    let Shape::Rect(r) = &mut g.shape else {
        panic!("rect")
    };
    r
}
fn geometry(case: &mut FindingCase, index: usize) -> &mut Geometry {
    let property = case.snapshot.nodes[index]
        .properties
        .iter_mut()
        .find(|p| p.field() == Field::LayoutBounds)
        .expect("layout");
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = property
    else {
        panic!("geometry")
    };
    g
}
fn expectation(case: &mut FindingCase, expected_value: f64, tolerance_value: f64) {
    let Rule::Geometry {
        expected,
        tolerance,
        ..
    } = &mut case.expectation.rule
    else {
        panic!("rule")
    };
    *expected = expected_value;
    *tolerance = tolerance_value;
}
fn source(case: &FindingCase) -> Evidence {
    let Property::Requested { evidence, .. } = &case.snapshot.nodes[0].properties[0] else {
        panic!("source")
    };
    evidence.clone()
}
fn mapping(case: &FindingCase, from: Space, to: Space, affine: [f64; 6]) -> Transform {
    Transform {
        from,
        to,
        affine,
        target: case.snapshot.context.target.clone(),
        surface: case.snapshot.nodes[0].surface.clone(),
        environment_revision: case.snapshot.context.environment_revision.clone(),
        evidence: source(case),
    }
}

#[test]
fn independent_geo_literal_vectors() {
    // Numbers authored before the engine in the independent GEO oracle corpus.
    for (name, expected, status) in [
        ("GEO-GAP", 8.0, CheckStatus::Pass),
        ("GEO-GAP-FAIL", 8.0, CheckStatus::Fail),
        ("GEO-CENTERS", 33.0, CheckStatus::Pass),
        ("GEO-SIZE-RATIO__width", 30.0, CheckStatus::Pass),
        ("GEO-SIZE-RATIO__height", 10.0, CheckStatus::Pass),
        ("GEO-SIZE-RATIO__ratio", 3.0, CheckStatus::Pass),
        ("GEO-ALIGNED", 0.0, CheckStatus::Pass),
        ("GEO-INSIDE", 10.0, CheckStatus::Pass),
        ("GEO-INTERSECTS", 25.0, CheckStatus::Pass),
        ("GEO-OVERFLOW", 10.0, CheckStatus::Fail),
        ("GEO-EQUAL-SPACING", 0.0, CheckStatus::Pass),
    ] {
        let case = fixture(name);
        let result = run(&case);
        assert_eq!(amount(&result), expected, "{name}");
        assert_eq!(result.finding.status, status, "{name}");
        validation::validate_finding(&case.snapshot, &case.expectation, &result.finding)
            .expect("canonical output compatibility");
        assert_eq!(run(&case), result, "deterministic {name}");
    }
}
#[test]
fn geometric_details_do_not_claim_padding_or_occlusion() {
    let inside = run(&fixture("GEO-INSIDE"));
    let MeasurementResult::Known { measurement: m } = inside.measurement else {
        panic!("known")
    };
    assert_eq!(
        m.details,
        Details::Insets {
            left: 10.0,
            top: 20.0,
            right: 80.0,
            bottom: 30.0
        }
    );
    let intersects = run(&fixture("GEO-INTERSECTS"));
    let MeasurementResult::Known { measurement: m } = intersects.measurement else {
        panic!("known")
    };
    assert_eq!(
        m.details,
        Details::Intersection {
            rect: Some(Rect {
                x: 35.0,
                y: 25.0,
                width: 5.0,
                height: 5.0
            })
        }
    );
}
#[test]
fn r03_g1_signed_tolerance_and_no_decimal_truncation() {
    let mut case = fixture("GEO-GAP");
    rect(&mut case, 0).x = 0.0;
    rect(&mut case, 0).width = 20.0;
    expectation(&mut case, 8.0, 1.0);
    for (left, expected, status) in [
        (29.0, 9.0, CheckStatus::Pass),
        (29.01, 9.01, CheckStatus::Fail),
        (18.0, -2.0, CheckStatus::Fail),
    ] {
        rect(&mut case, 1).x = left;
        let result = run(&case);
        assert!((amount(&result) - expected).abs() < 1e-12);
        assert_eq!(result.finding.status, status);
    }
    rect(&mut case, 1).x = 30.99999;
    expectation(&mut case, 10.0, 0.0);
    assert_eq!(run(&case).finding.status, CheckStatus::Fail);
}
#[test]
fn r03_g2_full_containment_has_no_hidden_two_pixel_allowance() {
    let mut case = fixture("GEO-INSIDE");
    *rect(&mut case, 0) = Rect {
        x: 0.0,
        y: 5.0,
        width: 102.0,
        height: 95.0,
    };
    *rect(&mut case, 1) = Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };
    expectation(&mut case, 0.0, 0.0);
    assert_eq!(amount(&run(&case)), -2.0);
    assert_eq!(run(&case).finding.status, CheckStatus::Fail);
    expectation(&mut case, 0.0, 2.0);
    assert_eq!(run(&case).finding.status, CheckStatus::Pass);
}
#[test]
fn r03_e1_ordered_gaps_and_partial_membership() {
    let mut case = fixture("GEO-EQUAL-SPACING");
    for (index, x) in [0.0, 18.0, 37.0].into_iter().enumerate() {
        rect(&mut case, index).x = x;
        rect(&mut case, index).width = 10.0;
    }
    let mut node = case.snapshot.nodes[2].clone();
    node.key.key = Id("D".into());
    case.snapshot.nodes.push(node);
    rect(&mut case, 3).x = 60.0;
    let key = case.snapshot.nodes[3].key.clone();
    case.expectation.targets.push(key.clone());
    let Rule::Geometry { anchors, .. } = &mut case.expectation.rule else {
        panic!("rule")
    };
    let mut anchor = anchors[2].clone();
    anchor.element = key;
    anchors.push(anchor);
    expectation(&mut case, 0.0, 1.0);
    let result = run(&case);
    assert_eq!(amount(&result), 5.0);
    assert_eq!(result.finding.status, CheckStatus::Fail);
    let MeasurementResult::Known { measurement: m } = result.measurement else {
        panic!("known")
    };
    assert_eq!(
        m.details,
        Details::Gaps {
            values: vec![8.0, 9.0, 13.0]
        }
    );
    case.snapshot.coverage.status = CoverageStatus::Partial;
    assert_eq!(
        run(&case).measurement.unknown_reason(),
        Some(UnknownReason::IncompleteScope)
    );
}
#[test]
fn unavailable_is_never_pass_even_with_large_tolerance() {
    let mut case = fixture("GEO-GAP");
    expectation(&mut case, 8.0, 999.0);
    for (state, reason) in [
        (
            Availability::Unknown {
                reason: Id("missing".into()),
            },
            UnknownReason::UnknownProperty,
        ),
        (
            Availability::Unsupported {
                reason: Id("not_exposed".into()),
            },
            UnknownReason::UnsupportedProperty,
        ),
        (Availability::Redacted {}, UnknownReason::RedactedProperty),
    ] {
        let Property::Requested { state: current, .. } = &mut case.snapshot.nodes[1].properties[0]
        else {
            panic!("property")
        };
        *current = state;
        let result = run(&case);
        assert_eq!(result.measurement.unknown_reason(), Some(reason));
        assert_eq!(result.finding.status, CheckStatus::Unknown);
        assert!(result.finding.measured.is_none());
    }
    for name in [
        "GEO-BASELINE-UNKNOWN",
        "GEO-FRAME-KIND",
        "GEO-MISSING-TRANSFORM",
    ] {
        assert_eq!(
            run(&fixture(name)).finding.status,
            CheckStatus::Unknown,
            "{name}"
        );
    }
}
#[test]
fn baseline_is_measured_not_inferred_from_layout_bottom() {
    let case = fixture("GEO-BASELINE");
    let result = run(&case);
    assert!((amount(&result) - 0.2).abs() < 1e-12);
    assert_eq!(result.finding.status, CheckStatus::Pass);
}
#[test]
fn r03_b1_sourced_transform_chain_preserves_translation_invariance() {
    let mut case = fixture("GEO-GAP");
    rect(&mut case, 0).x = 2.0;
    rect(&mut case, 0).width = 10.0;
    rect(&mut case, 1).x = 20.0;
    let local = space(&case);
    let mut surface = local.clone();
    surface.id = Id("surface".into());
    surface.kind = SpaceKind::Surface;
    let mut image = surface.clone();
    image.id = Id("image".into());
    image.units = Unit::Px;
    let mut first = mapping(
        &case,
        local.clone(),
        surface.clone(),
        [1.0, 0.0, 0.0, 1.0, 100.0, 0.0],
    );
    let second = mapping(
        &case,
        surface.clone(),
        image.clone(),
        [2.0, 0.0, 0.0, 2.0, 0.0, 0.0],
    );
    let transforms = [first.clone(), second.clone()];
    let result = check(
        &case.snapshot,
        &case.expectation,
        &EvaluationContext {
            space: &surface,
            transforms: &transforms,
            conditions: None,
        },
    )
    .expect("surface");
    assert_eq!(amount(&result), 8.0);
    let Rule::Geometry { units, .. } = &mut case.expectation.rule else {
        panic!("rule")
    };
    *units = Unit::Px;
    expectation(&mut case, 16.0, 0.0);
    for translation in [100.0, 150.0] {
        first.affine[4] = translation;
        let transforms = [first.clone(), second.clone()];
        let result = check(
            &case.snapshot,
            &case.expectation,
            &EvaluationContext {
                space: &image,
                transforms: &transforms,
                conditions: None,
            },
        )
        .expect("image");
        assert_eq!(amount(&result), 16.0);
        assert_eq!(result.finding.status, CheckStatus::Pass);
    }
    for transforms in [vec![first], vec![second], vec![]] {
        let result = check(
            &case.snapshot,
            &case.expectation,
            &EvaluationContext {
                space: &image,
                transforms: &transforms,
                conditions: None,
            },
        )
        .expect("unknown path");
        assert_eq!(
            result.measurement.unknown_reason(),
            Some(UnknownReason::MissingTransform)
        );
    }
}
#[test]
fn transform_context_units_origin_and_shape_cannot_false_pass() {
    let mut case = fixture("GEO-SIZE-RATIO__width");
    let local = space(&case);
    let mut output = local.clone();
    output.id = Id("frame".into());
    output.units = Unit::Px;
    let transform = mapping(
        &case,
        local.clone(),
        output.clone(),
        [2.0, 0.0, 0.0, 2.0, 6.0, 8.0],
    );
    let Rule::Geometry { units, .. } = &mut case.expectation.rule else {
        panic!("rule")
    };
    *units = Unit::Px;
    expectation(&mut case, 60.0, 0.0);
    let result = check(
        &case.snapshot,
        &case.expectation,
        &EvaluationContext {
            space: &output,
            transforms: std::slice::from_ref(&transform),
            conditions: None,
        },
    )
    .expect("transform");
    assert_eq!(amount(&result), 60.0);
    let mut stale = transform.clone();
    stale.environment_revision = Id("display-changed".into());
    assert_eq!(
        check(
            &case.snapshot,
            &case.expectation,
            &EvaluationContext {
                space: &output,
                transforms: &[stale],
                conditions: None,
            }
        )
        .expect("unknown")
        .finding
        .status,
        CheckStatus::Unknown
    );
    let mut wrong_target = transform.clone();
    wrong_target.target.generation = Id("other".into());
    let mut wrong_surface = transform.clone();
    wrong_surface.surface.generation = Id("other".into());
    let mut wrong_origin = transform.clone();
    wrong_origin.from.origin = Origin::BottomLeft;
    for bad in [wrong_target, wrong_surface, wrong_origin] {
        assert_eq!(
            check(
                &case.snapshot,
                &case.expectation,
                &EvaluationContext {
                    space: &output,
                    transforms: &[bad],
                    conditions: None
                }
            )
            .expect("unknown")
            .finding
            .status,
            CheckStatus::Unknown
        );
    }
    let mut shear = transform;
    shear.affine[2] = 1.0;
    assert_eq!(
        check(
            &case.snapshot,
            &case.expectation,
            &EvaluationContext {
                space: &output,
                transforms: &[shear],
                conditions: None
            }
        )
        .expect("unsupported")
        .measurement
        .unknown_reason(),
        Some(UnknownReason::UnsupportedShape)
    );
    geometry(&mut case, 0).shape = Shape::Polygon(vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 0.0, y: 1.0 },
    ]);
    assert_eq!(
        check(
            &case.snapshot,
            &case.expectation,
            &EvaluationContext {
                space: &output,
                transforms: &[],
                conditions: None
            }
        )
        .expect("unsupported")
        .measurement
        .unknown_reason(),
        Some(UnknownReason::UnsupportedShape)
    );
}
#[test]
fn applicability_requires_observed_context_and_is_never_inferred() {
    let mut case = fixture("GEO-GAP");
    case.expectation.applies_when.platform = Some(Id("fixture".into()));
    assert_eq!(
        run(&case).measurement.unknown_reason(),
        Some(UnknownReason::ApplicabilityUnknown)
    );
    let evidence = source(&case);
    let mut actual = ContextConditions {
        platform: Some(Id("other".into())),
        input_mode: None,
        text_scale: None,
    };
    let output = space(&case);
    let result = check(
        &case.snapshot,
        &case.expectation,
        &EvaluationContext {
            space: &output,
            transforms: &[],
            conditions: Some((&actual, &evidence)),
        },
    )
    .expect("context");
    assert_eq!(
        result.measurement.unknown_reason(),
        Some(UnknownReason::NotApplicable)
    );
    actual.platform = Some(Id("fixture".into()));
    assert_eq!(
        check(
            &case.snapshot,
            &case.expectation,
            &EvaluationContext {
                space: &output,
                transforms: &[],
                conditions: Some((&actual, &evidence))
            }
        )
        .expect("context")
        .finding
        .status,
        CheckStatus::Pass
    );
}
#[test]
fn invalid_and_nonfinite_inputs_return_errors_not_panics_or_passes() {
    for width in [-1.0, f64::INFINITY, f64::NAN] {
        let mut case = fixture("GEO-SIZE-RATIO__width");
        rect(&mut case, 0).width = width;
        assert!(
            check(
                &case.snapshot,
                &case.expectation,
                &EvaluationContext {
                    space: &space(&case),
                    transforms: &[],
                    conditions: None
                }
            )
            .is_err()
        );
    }
    let mut case = fixture("GEO-SIZE-RATIO__width");
    rect(&mut case, 0).x = f64::MAX;
    rect(&mut case, 0).width = f64::MAX;
    assert_eq!(
        check(
            &case.snapshot,
            &case.expectation,
            &EvaluationContext {
                space: &space(&case),
                transforms: &[],
                conditions: None
            }
        ),
        Err(GeometryError::NonFiniteCalculation)
    );
    let mut case = fixture("GEO-GAP");
    let Rule::Geometry { anchors, .. } = &mut case.expectation.rule else {
        panic!("rule")
    };
    anchors.pop();
    assert_eq!(
        check(
            &case.snapshot,
            &case.expectation,
            &EvaluationContext {
                space: &space(&case),
                transforms: &[],
                conditions: None
            }
        ),
        Err(GeometryError::InvalidRule)
    );
    let mut case = fixture("GEO-SIZE-RATIO__ratio");
    rect(&mut case, 0).height = 0.0;
    assert_eq!(
        run(&case).measurement.unknown_reason(),
        Some(UnknownReason::UndefinedRatio)
    );
}

#[test]
fn every_comparison_uses_explicit_tolerance_and_boundary() {
    let mut case = fixture("GEO-GAP");
    for (comparison, expected, tolerance, status) in [
        (Comparison::Equal, 9.0, 1.0, CheckStatus::Pass),
        (Comparison::AtLeast, 9.0, 1.0, CheckStatus::Pass),
        (Comparison::AtLeast, 9.01, 1.0, CheckStatus::Fail),
        (Comparison::AtMost, 7.0, 1.0, CheckStatus::Pass),
        (Comparison::AtMost, 6.99, 1.0, CheckStatus::Fail),
        (Comparison::GreaterThan, 7.0, 1.0, CheckStatus::Fail),
        (Comparison::GreaterThan, 6.99, 1.0, CheckStatus::Pass),
    ] {
        expectation(&mut case, expected, tolerance);
        let Rule::Geometry {
            comparison: current,
            ..
        } = &mut case.expectation.rule
        else {
            panic!("rule")
        };
        *current = comparison;
        assert_eq!(run(&case).finding.status, status);
    }
}

#[test]
fn unknown_retains_attempted_source_evidence() {
    let mut case = fixture("GEO-GAP");
    let Property::Requested { state, .. } = &mut case.snapshot.nodes[1].properties[0] else {
        panic!("property")
    };
    *state = Availability::Unknown {
        reason: Id("unavailable".into()),
    };
    let result = run(&case);
    let MeasurementResult::Unknown { reason, evidence } = &result.measurement else {
        panic!("unknown")
    };
    assert_eq!(*reason, UnknownReason::UnknownProperty);
    assert_eq!(evidence[0].observation_id, Id("OG1".into()));
    assert_eq!(result.finding.observation_id, Some(Id("OG1".into())));
    validation::validate_finding(&case.snapshot, &case.expectation, &result.finding)
        .expect("unknown output is canonical");
}

#[test]
fn all_four_corners_and_origin_flip_are_preserved() {
    let mut case = fixture("GEO-SIZE-RATIO__width");
    let input = space(&case);
    let mut output = input.clone();
    output.id = Id("rotated".into());
    output.origin = Origin::BottomLeft;
    // A=(10,20,30,10), quarter turn maps corners to (-20,10), (-20,40),
    // (-30,40), (-30,10). The exact transformed width is 10, not 30.
    let transform = mapping(
        &case,
        input,
        output.clone(),
        [0.0, 1.0, -1.0, 0.0, 0.0, 0.0],
    );
    expectation(&mut case, 10.0, 0.0);
    let result = check(
        &case.snapshot,
        &case.expectation,
        &EvaluationContext {
            space: &output,
            transforms: &[transform],
            conditions: None,
        },
    )
    .expect("quarter turn");
    assert_eq!(amount(&result), 10.0);
    assert_eq!(result.finding.status, CheckStatus::Pass);
}
