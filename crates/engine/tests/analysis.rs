use uiblueprint_engine::*;
use uiblueprint_schema::{analysis::*, model::*};

fn fixture(name: &str) -> FindingCase {
    let path = format!(
        "{}/../../fixtures/golden/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let document: Document =
        serde_json::from_slice(&std::fs::read(path).expect("fixture")).expect("canonical fixture");
    let Artifact::Finding(case) = document.artifact else {
        panic!("finding fixture")
    };
    *case
}
fn inputs(name: &str) -> (Snapshot, GeometryQuery, EvaluationInput) {
    let case = fixture(name);
    let query = GeometryQuery::from_expectation(&case.expectation).expect("query");
    let evaluation = EvaluationInput {
        snapshot_id: case.snapshot.id.clone(),
        revision: case.snapshot.revision,
        context: case.snapshot.context.clone(),
        result_space: query.anchors[0].coordinate_space.clone(),
        transforms: vec![],
        conditions: None,
    };
    (case.snapshot, query, evaluation)
}
fn measured(result: &MeasurementResult) -> &Measurement {
    let MeasurementResult::Known { measurement } = result else {
        panic!("known measurement: {result:?}")
    };
    measurement
}
fn amount(result: &MeasurementResult) -> f64 {
    let Value::Quantity { amount, .. } = measured(result).value else {
        panic!("quantity")
    };
    amount
}
fn declaration(s: Snapshot, q: GeometryQuery, e: EvaluationInput) -> AnalysisDocument {
    let result = measure_query_bound(&s, &q, &e).expect("compute");
    AnalysisDocument {
        schema_version: AnalysisVersion::CURRENT,
        artifact: AnalysisArtifact::Measurement(Box::new(MeasurementCase {
            snapshot: s,
            query: q,
            evaluation: e,
            result,
        })),
    }
}
fn property(s: &mut Snapshot, index: usize) -> &mut Property {
    &mut s.nodes[index].properties[0]
}
fn geometry(s: &mut Snapshot, index: usize) -> &mut Geometry {
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = property(s, index)
    else {
        panic!("geometry")
    };
    g
}
fn source(s: &Snapshot) -> Evidence {
    let Property::Requested { evidence, .. } = &s.nodes[0].properties[0] else {
        panic!("source")
    };
    evidence.clone()
}
fn transform(s: &Snapshot, from: Space, to: Space, affine: [f64; 6], method: &str) -> Transform {
    let mut evidence = source(s);
    evidence.method = Id(method.into());
    Transform {
        from,
        to,
        affine,
        target: s.context.target.clone(),
        surface: s.context.surfaces[0].clone(),
        environment_revision: s.context.environment_revision.clone(),
        evidence,
    }
}

#[test]
fn factual_query_has_no_normative_fields_and_preserves_independent_vectors() {
    for (name, value) in [
        ("GEO-GAP", 8.0),
        ("GEO-CENTERS", 33.0),
        ("GEO-SIZE-RATIO__width", 30.0),
        ("GEO-SIZE-RATIO__height", 10.0),
        ("GEO-SIZE-RATIO__ratio", 3.0),
        ("GEO-ALIGNED", 0.0),
        ("GEO-INSIDE", 10.0),
        ("GEO-INTERSECTS", 25.0),
        ("GEO-EQUAL-SPACING", 0.0),
    ] {
        let (s, q, e) = inputs(name);
        let json = serde_json::to_value(&q).expect("query JSON");
        for field in ["expected", "expected_from", "comparison", "tolerance"] {
            assert!(json.get(field).is_none());
        }
        let result = measure_query_bound(&s, &q, &e).expect("factual compute");
        assert_eq!(amount(&result), value, "{name}");
        let doc = declaration(s, q, e);
        verify_analysis_result(&doc).expect("recompute");
    }
    let mut case = fixture("GEO-GAP");
    let Rule::Geometry {
        expected,
        tolerance,
        ..
    } = &mut case.expectation.rule
    else {
        panic!("rule")
    };
    *expected = -9999.0;
    *tolerance = 0.0;
    let query = GeometryQuery::from_expectation(&case.expectation).expect("query");
    let context = EvaluationContext {
        space: &query.anchors[0].coordinate_space,
        transforms: &[],
        conditions: None,
    };
    assert_eq!(
        measure(&case.snapshot, &case.expectation, &context).expect("compatibility"),
        measure_query(&case.snapshot, &query, &context).expect("factual")
    );
}
#[test]
fn signed_zero_empty_intersection_and_gaps_remain_factual() {
    let (mut s, q, e) = inputs("GEO-GAP");
    let Shape::Rect(rect) = &mut geometry(&mut s, 1).shape else {
        panic!("rect")
    };
    rect.x = 38.0;
    assert_eq!(
        amount(&measure_query_bound(&s, &q, &e).expect("negative gap")),
        -2.0
    );
    let (mut s, q, e) = inputs("GEO-INTERSECTS");
    let Shape::Rect(rect) = &mut geometry(&mut s, 1).shape else {
        panic!("rect")
    };
    rect.x = 40.0;
    let result = measure_query_bound(&s, &q, &e).expect("touching edges");
    assert_eq!(amount(&result), 0.0);
    assert_eq!(
        measured(&result).details,
        Details::Intersection { rect: None }
    );
    let (s, q, e) = inputs("GEO-EQUAL-SPACING");
    assert_eq!(
        measured(&measure_query_bound(&s, &q, &e).expect("gaps")).details,
        Details::Gaps {
            values: vec![8.0, 8.0]
        }
    );
}
#[test]
fn complete_binding_is_checked_before_computation() {
    let (s, q, e) = inputs("GEO-GAP");
    for variant in 0..9 {
        let mut bad = e.clone();
        match variant {
            0 => bad.snapshot_id = Id("other".into()),
            1 => bad.revision += 1,
            2 => bad.context.session_id = Id("other".into()),
            3 => bad.context.target.generation = Id("g2".into()),
            4 => bad.context.surfaces[0].generation = Id("w2".into()),
            5 => bad.context.scope_id = Id("other".into()),
            6 => bad.context.projection = Projection::Interaction,
            7 => bad.context.fields = vec![Field::Role],
            _ => bad.context.environment_revision = Id("other".into()),
        }
        if variant == 6 {
            bad.context.projection = if s.context.projection == Projection::Interaction {
                Projection::Design
            } else {
                Projection::Interaction
            };
        }
        assert!(
            measure_query_bound(&s, &q, &bad).is_err(),
            "binding {variant}"
        );
    }
    let mut bad = q.clone();
    bad.scope_id = Id("other".into());
    assert!(measure_query_bound(&s, &bad, &e).is_err());
    let mut missing = q.clone();
    missing.targets[0].key = Id("missing".into());
    missing.anchors[0].element = missing.targets[0].clone();
    assert_eq!(
        measure_query_bound(&s, &missing, &e)
            .expect("missing actual target")
            .unknown_reason(),
        Some(UnknownReason::MissingTarget)
    );
}
#[test]
fn bound_transforms_preserve_units_origin_path_and_distinct_consumed_evidence() {
    let (s, mut q, mut e) = inputs("GEO-GAP");
    let local = e.result_space.clone();
    let mut surface = local.clone();
    surface.id = Id("surface".into());
    let mut image = surface.clone();
    image.id = Id("image".into());
    image.units = Unit::Px;
    image.origin = Origin::BottomLeft;
    let first = transform(
        &s,
        local.clone(),
        surface.clone(),
        [1.0, 0.0, 0.0, 1.0, 100.0, 0.0],
        "to_surface",
    );
    let second = transform(
        &s,
        surface,
        image.clone(),
        [2.0, 0.0, 0.0, -2.0, 0.0, 100.0],
        "to_image",
    );
    let mut unused = transform(
        &s,
        local.clone(),
        local,
        [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        "unused",
    );
    unused.from.id = Id("unreachable".into());
    e.result_space = image.clone();
    e.transforms = vec![first.clone(), second.clone(), unused];
    q.units = Unit::Px;
    let result = measure_query_bound(&s, &q, &e).expect("bound chain");
    assert_eq!(amount(&result), 16.0);
    assert_eq!(measured(&result).space, image);
    let sources = &measured(&result).evidence;
    assert_eq!(sources.len(), 3);
    assert_eq!(sources[0], source(&s));
    assert_eq!(sources[1], first.evidence);
    assert_eq!(sources[2], second.evidence);
    verify_analysis_result(&declaration(s.clone(), q.clone(), e.clone()))
        .expect("chain verification");
    e.transforms.remove(1);
    assert_eq!(
        measure_query_bound(&s, &q, &e)
            .expect("missing chain")
            .unknown_reason(),
        Some(UnknownReason::MissingTransform)
    );
    e.transforms[0].environment_revision = Id("wrong".into());
    assert!(measure_query_bound(&s, &q, &e).is_err());
}
#[test]
fn conditions_are_observed_bound_and_never_inferred() {
    let (mut s, mut q, mut e) = inputs("GEO-GAP");
    q.applies_when.platform = Some(Id("fixture".into()));
    assert_eq!(
        measure_query_bound(&s, &q, &e)
            .expect("no conditions")
            .unknown_reason(),
        Some(UnknownReason::ApplicabilityUnknown)
    );
    e.conditions = Some(ObservedConditions {
        values: ContextConditions {
            platform: Some(Id("other".into())),
            input_mode: None,
            text_scale: None,
        },
        evidence: source(&s),
    });
    assert_eq!(
        measure_query_bound(&s, &q, &e)
            .expect("mismatch")
            .unknown_reason(),
        Some(UnknownReason::NotApplicable)
    );
    e.conditions.as_mut().expect("conditions").values.platform = Some(Id("fixture".into()));
    assert_eq!(
        amount(&measure_query_bound(&s, &q, &e).expect("supplied")),
        8.0
    );
    s.observations[0].consistency = Consistency::Unstable;
    s.observations[0].consistency_reason = Some(Id("changing".into()));
    let result = measure_query_bound(&s, &q, &e).expect("unstable");
    assert_eq!(result.unknown_reason(), Some(UnknownReason::UnstableState));
    let MeasurementResult::Unknown { evidence, .. } = result else {
        panic!("unknown")
    };
    assert_eq!(evidence, vec![source(&s)]);
    e.conditions
        .as_mut()
        .expect("conditions")
        .evidence
        .observation_id = Id("fabricated".into());
    assert!(measure_query_bound(&s, &q, &e).is_err());
}
#[test]
fn imported_results_require_exact_recomputation_even_if_contract_valid() {
    let (s, q, e) = inputs("GEO-GAP");
    let good = declaration(s, q, e);
    verify_analysis_result(&good).expect("original");
    let mut tampered = good.clone();
    let AnalysisArtifact::Measurement(case) = &mut tampered.artifact else {
        panic!("case")
    };
    let MeasurementResult::Known { measurement } = &mut case.result else {
        panic!("known")
    };
    let Value::Quantity { amount, .. } = &mut measurement.value else {
        panic!("quantity")
    };
    *amount = 123.0;
    tampered
        .validate()
        .expect("declaration is not arithmetic proof");
    assert_eq!(
        verify_analysis_result(&tampered),
        Err(VerificationError::ResultMismatch)
    );
    let mut tampered = good.clone();
    let AnalysisArtifact::Measurement(case) = &mut tampered.artifact else {
        panic!("case")
    };
    case.result = MeasurementResult::Unknown {
        reason: UnknownReason::UnsupportedShape,
        evidence: vec![],
    };
    tampered.validate().expect("unknown declaration");
    assert_eq!(
        verify_analysis_result(&tampered),
        Err(VerificationError::ResultMismatch)
    );
    let (s, q, e) = inputs("GEO-INSIDE");
    let mut tampered = declaration(s, q, e);
    let AnalysisArtifact::Measurement(case) = &mut tampered.artifact else {
        panic!("case")
    };
    let MeasurementResult::Known { measurement } = &mut case.result else {
        panic!("known")
    };
    let Details::Insets { left, .. } = &mut measurement.details else {
        panic!("insets")
    };
    *left += 1.0;
    tampered.validate().expect("finite details declaration");
    assert_eq!(
        verify_analysis_result(&tampered),
        Err(VerificationError::ResultMismatch)
    );
}
#[test]
fn imported_check_only_allows_finding_id_to_differ() {
    let case = fixture("GEO-GAP");
    let (_, _, e) = inputs("GEO-GAP");
    let computed = check_bound(&case.snapshot, &case.expectation, &e).expect("check");
    let mut document = AnalysisDocument {
        schema_version: AnalysisVersion::CURRENT,
        artifact: AnalysisArtifact::GeometryCheck(Box::new(GeometryCheckCase {
            snapshot: case.snapshot,
            expectation: case.expectation,
            evaluation: e,
            measurement: computed.measurement,
            finding: computed.finding,
        })),
    };
    let AnalysisArtifact::GeometryCheck(case) = &mut document.artifact else {
        panic!("check")
    };
    case.finding.id = Id("caller_assigned".into());
    verify_analysis_result(&document).expect("caller may assign finding identity");
    let AnalysisArtifact::GeometryCheck(case) = &mut document.artifact else {
        panic!("check")
    };
    case.finding.reason = None;
    document.validate().expect("optional declared reason");
    assert_eq!(
        verify_analysis_result(&document),
        Err(VerificationError::ResultMismatch)
    );
}

#[test]
fn baseline_conversion_and_unsupported_tilt_use_actual_coordinates() {
    let (s, mut q, mut e) = inputs("GEO-BASELINE");
    let from = e.result_space.clone();
    let mut to = from.clone();
    to.id = Id("baseline-image".into());
    to.units = Unit::Px;
    e.transforms = vec![transform(
        &s,
        from,
        to.clone(),
        [2.0, 0.0, 0.0, 2.0, 6.0, 8.0],
        "baseline_scale",
    )];
    e.result_space = to;
    q.units = Unit::Px;
    let result = measure_query_bound(&s, &q, &e).expect("sourced baselines");
    assert!((amount(&result) - 0.4).abs() < 1e-12);
    verify_analysis_result(&declaration(s.clone(), q.clone(), e.clone()))
        .expect("baseline verification");
    e.transforms[0].affine[1] = 1.0;
    assert_eq!(
        measure_query_bound(&s, &q, &e)
            .expect("x unavailable for tilted baseline")
            .unknown_reason(),
        Some(UnknownReason::UnsupportedShape)
    );
}
#[test]
fn requested_unavailable_and_unrequested_data_preserve_attempted_evidence() {
    for (state, sensitivity, reason) in [
        (
            Availability::Unknown {
                reason: Id("missing".into()),
            },
            Sensitivity::Public,
            UnknownReason::UnknownProperty,
        ),
        (
            Availability::Unsupported {
                reason: Id("unsupported".into()),
            },
            Sensitivity::Public,
            UnknownReason::UnsupportedProperty,
        ),
        (
            Availability::Redacted {},
            Sensitivity::Sensitive,
            UnknownReason::RedactedProperty,
        ),
    ] {
        let (mut s, q, e) = inputs("GEO-SIZE-RATIO__width");
        let attempted = source(&s);
        let Property::Requested {
            state: actual,
            sensitivity: classification,
            ..
        } = property(&mut s, 0)
        else {
            panic!("property")
        };
        *actual = state;
        *classification = sensitivity;
        let result = measure_query_bound(&s, &q, &e).expect("unavailable");
        assert_eq!(result.unknown_reason(), Some(reason));
        let MeasurementResult::Unknown { evidence, .. } = &result else {
            panic!("unknown")
        };
        assert_eq!(evidence, &vec![attempted]);
        verify_analysis_result(&declaration(s, q, e)).expect("unavailable result verifies");
    }
    let (mut s, q, mut e) = inputs("GEO-SIZE-RATIO__width");
    let source = source(&s);
    s.context.fields = vec![Field::Role];
    s.coverage.fields = vec![Field::Role];
    for observation in &mut s.observations {
        observation.coverage.fields = vec![Field::Role];
    }
    for node in &mut s.nodes {
        node.properties = vec![
            Property::Requested {
                field: Field::Role,
                sensitivity: Sensitivity::Public,
                evidence: source.clone(),
                state: Availability::Known {
                    value: Value::Role(Role::Group),
                },
            },
            Property::NotRequested {
                field: Field::LayoutBounds,
            },
        ];
    }
    e.context = s.context.clone();
    assert_eq!(
        measure_query_bound(&s, &q, &e)
            .expect("selection is separate")
            .unknown_reason(),
        Some(UnknownReason::NotRequested)
    );
}
#[test]
fn unused_evidence_and_false_pass_declarations_do_not_survive_verification() {
    let (s, q, mut e) = inputs("GEO-GAP");
    let local = e.result_space.clone();
    e.transforms.push(transform(
        &s,
        local.clone(),
        local,
        [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        "unused_source",
    ));
    let mut doc = declaration(s, q, e);
    let AnalysisArtifact::Measurement(case) = &mut doc.artifact else {
        panic!("case")
    };
    let MeasurementResult::Known { measurement } = &mut case.result else {
        panic!("known")
    };
    assert_eq!(measurement.evidence.len(), 1);
    measurement
        .evidence
        .push(case.evaluation.transforms[0].evidence.clone());
    doc.validate()
        .expect("eligible but unused evidence can be declared");
    assert_eq!(
        verify_analysis_result(&doc),
        Err(VerificationError::ResultMismatch)
    );
    let AnalysisArtifact::Measurement(case) = &mut doc.artifact else {
        panic!("case")
    };
    let MeasurementResult::Known { measurement } = &mut case.result else {
        panic!("known")
    };
    measurement.space.origin = Origin::BottomLeft;
    assert!(matches!(
        verify_analysis_result(&doc),
        Err(VerificationError::InvalidContract(_))
    ));

    let case = fixture("GEO-GAP-FAIL");
    let (_, _, evaluation) = inputs("GEO-GAP-FAIL");
    let mut computed =
        check_bound(&case.snapshot, &case.expectation, &evaluation).expect("real fail");
    assert_eq!(computed.finding.status, CheckStatus::Fail);
    let MeasurementResult::Known { measurement } = &mut computed.measurement else {
        panic!("known")
    };
    measurement.value = Value::Quantity {
        amount: 10.0,
        kind: QuantityKind::Length,
        source_units: Unit::CssPx,
    };
    computed.finding.measured = Some(measurement.value.clone());
    computed.finding.status = CheckStatus::Pass;
    computed.finding.reason = Some(Id("expectation_satisfied".into()));
    let doc = AnalysisDocument {
        schema_version: AnalysisVersion::CURRENT,
        artifact: AnalysisArtifact::GeometryCheck(Box::new(GeometryCheckCase {
            snapshot: case.snapshot,
            expectation: case.expectation,
            evaluation,
            measurement: computed.measurement,
            finding: computed.finding,
        })),
    };
    doc.validate()
        .expect("internally consistent false pass declaration");
    assert_eq!(
        verify_analysis_result(&doc),
        Err(VerificationError::ResultMismatch)
    );
}
#[test]
fn roundtrip_recomputation_is_exact_without_new_tolerance() {
    for name in ["GEO-BASELINE", "GEO-SIZE-RATIO__ratio", "GEO-INSIDE"] {
        let (s, q, e) = inputs(name);
        let original = declaration(s, q, e);
        let bytes = serde_json::to_vec(&original).expect("encode");
        let parsed = AnalysisDocument::from_json(&bytes, bytes.len()).expect("production parser");
        assert_eq!(parsed, original);
        verify_analysis_result(&parsed).expect("exact roundtrip verification");
    }
}

#[test]
fn distinct_observations_survive_and_declared_other_surface_is_not_consumed() {
    let (mut s, q, mut e) = inputs("GEO-GAP");
    let mut second = s.observations[0].clone();
    second.id = Id("OG2".into());
    s.observations.push(second);
    let Property::Requested { evidence, .. } = property(&mut s, 1) else {
        panic!("property")
    };
    evidence.observation_id = Id("OG2".into());
    let result = measure_query_bound(&s, &q, &e).expect("distinct observations");
    assert_eq!(
        measured(&result)
            .evidence
            .iter()
            .map(|e| e.observation_id.0.as_str())
            .collect::<Vec<_>>(),
        vec!["OG1", "OG2"]
    );
    verify_analysis_result(&declaration(s.clone(), q.clone(), e.clone()))
        .expect("no aggregate observation invention");
    let other = Identity {
        id: Id("other-surface".into()),
        generation: Id("w1".into()),
    };
    s.context.surfaces.push(other.clone());
    e.context = s.context.clone();
    let mut destination = e.result_space.clone();
    destination.id = Id("same-unit-destination".into());
    let mut wrong_surface = transform(
        &s,
        e.result_space.clone(),
        destination.clone(),
        [1.0, 0.0, 0.0, 1.0, 100.0, 0.0],
        "other_surface_mapping",
    );
    wrong_surface.surface = other;
    e.result_space = destination;
    e.transforms.push(wrong_surface);
    assert_eq!(
        measure_query_bound(&s, &q, &e)
            .expect("no applicable mapping")
            .unknown_reason(),
        Some(UnknownReason::MissingTransform)
    );
}
