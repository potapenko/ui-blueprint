use serde_json::json;
use std::{fs, path::PathBuf};
use uiblueprint_schema::{
    SchemaVersion,
    analysis::*,
    model::{Artifact, Document, Id},
    validation::ValidationError,
};

fn source() -> (
    uiblueprint_schema::model::Snapshot,
    uiblueprint_schema::model::Expectation,
) {
    let document = Document::from_json(
        include_bytes!("../../../fixtures/golden/GEO-GAP.json"),
        1_048_576,
    )
    .expect("independent authored geometry fixture");
    let Artifact::Finding(case) = document.artifact else {
        panic!("finding fixture")
    };
    (case.snapshot, case.expectation)
}

#[test]
fn canonical_query_extraction_has_no_normative_fields() {
    let (snapshot, expectation) = source();
    let query = GeometryQuery::from_expectation(&expectation).expect("geometric query");
    validate_query(&query).expect("valid factual query");
    assert_eq!(query.id, expectation.id);
    assert_eq!(query.scope_id, snapshot.context.scope_id);
    let value = serde_json::to_value(&query).expect("query JSON");
    for excluded in ["expected", "comparison", "tolerance", "expected_from"] {
        assert!(value.get(excluded).is_none());
    }
    let mut invalid = query.clone();
    invalid.anchors.pop();
    assert_eq!(
        validate_query(&invalid),
        Err(ValidationError::InvalidGeometry)
    );
    invalid = query;
    invalid.anchors[1].axis = Id("y".into());
    assert_eq!(
        validate_query(&invalid),
        Err(ValidationError::InvalidGeometry)
    );
}

#[test]
fn exact_analysis_version_and_strict_records_do_not_broaden_core() {
    let (_, expectation) = source();
    let document = AnalysisDocument {
        schema_version: AnalysisVersion::CURRENT,
        artifact: AnalysisArtifact::GeometryQuery(Box::new(
            GeometryQuery::from_expectation(&expectation).unwrap(),
        )),
    };
    let bytes = serde_json::to_vec(&document).unwrap();
    assert_eq!(
        serde_json::from_slice::<AnalysisDocument>(&bytes).unwrap(),
        document
    );
    assert!(Document::from_json(&bytes, bytes.len()).is_err());
    assert_eq!(SchemaVersion::CURRENT.as_str(), "0.1.0");
    let mut value = serde_json::to_value(&document).unwrap();
    value["schema_version"] = json!("0.1.0");
    assert!(serde_json::from_value::<AnalysisDocument>(value).is_err());
    assert!(serde_json::from_value::<AnalysisVersion>(json!({"0.2.0":null})).is_err());
    assert!(
        serde_json::from_value::<MeasurementDetails>(json!({"kind":"scalar","value":3})).is_err()
    );
    let value = json!({"status":"unknown","reason":"unknown_property","evidence":[],"value":0});
    assert!(serde_json::from_value::<MeasurementResult>(value).is_err());
    let value = json!(["query", "scope", [], "gap", [], "length", "px", {}]);
    assert!(serde_json::from_value::<GeometryQuery>(value).is_err());
}

#[test]
fn unbound_evaluation_does_not_pretend_to_resolve_its_source() {
    let (snapshot, expectation) = source();
    let query = GeometryQuery::from_expectation(&expectation).unwrap();
    let mut evaluation = EvaluationInput {
        snapshot_id: snapshot.id.clone(),
        revision: snapshot.revision,
        context: snapshot.context.clone(),
        result_space: query.anchors[0].coordinate_space.clone(),
        transforms: vec![],
        conditions: None,
    };
    validate_bound_evaluation(&snapshot, &evaluation).expect("same source");
    evaluation.snapshot_id = Id("different-snapshot".into());
    validate_evaluation_input(&evaluation).expect("unbound declaration only");
    assert_eq!(
        validate_bound_evaluation(&snapshot, &evaluation),
        Err(ValidationError::IncompatibleContext)
    );
    evaluation.snapshot_id = snapshot.id.clone();
    evaluation.revision += 1;
    assert_eq!(
        validate_bound_evaluation(&snapshot, &evaluation),
        Err(ValidationError::IncompatibleContext)
    );
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn analysis_fixture(name: &str) -> Vec<u8> {
    fs::read(
        root()
            .join("fixtures/analysis")
            .join(format!("{name}.json")),
    )
    .expect("authored analysis vector")
}

#[test]
fn authored_vectors_match_contract_and_structural_expectations() {
    let manifest: Vec<serde_json::Value> =
        serde_json::from_slice(&fs::read(root().join("fixtures/analysis/manifest.json")).unwrap())
            .unwrap();
    let schema = json_schema().unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    for row in manifest {
        let name = row["path"].as_str().unwrap();
        let bytes = fs::read(root().join("fixtures/analysis").join(name)).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(
            validator.is_valid(&value),
            row["structural_valid"].as_bool().unwrap(),
            "structural {name}"
        );
        let actual = AnalysisDocument::from_json(&bytes, bytes.len());
        assert_eq!(
            actual.is_ok(),
            row["contract_valid"].as_bool().unwrap(),
            "semantic {name}: {:?}",
            actual.as_ref().err()
        );
        if let Ok(doc) = actual {
            let encoded = serde_json::to_vec(&doc).unwrap();
            assert_eq!(
                AnalysisDocument::from_json(&encoded, encoded.len()).unwrap(),
                doc,
                "roundtrip {name}"
            );
        }
    }
}

#[test]
fn generated_analysis_schema_matches_and_dispatch_stays_version_exact() {
    let saved: serde_json::Value = serde_json::from_slice(
        &fs::read(root().join("schemas/uiblueprint-analysis-0.2.0.schema.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(saved, json_schema().unwrap());
    let bytes = analysis_fixture("measurement-gap");
    validate_input_document(&bytes, bytes.len()).unwrap();
    assert_eq!(
        validate_input_document(&bytes, bytes.len() - 1),
        Err(ValidationError::ResourceLimit)
    );
    let core = include_bytes!("../../../fixtures/golden/ENV-SNAPSHOT-VALID.json");
    validate_input_document(core, core.len()).unwrap();
    assert!(AnalysisDocument::from_json(core, core.len()).is_err());
    for bytes in [
        br#"{"schema_version":"0.2.0","schema_version":"0.1.0","artifact":{}}"#.as_slice(),
        br#"{"artifact":{},"schema_version":"0.3.0"}"#.as_slice(),
        br#"["0.2.0",{}]"#.as_slice(),
        br#"{"schema_version":"0.2.0","artifact":{"kind":"arbitrary","data":{}}}"#.as_slice(),
    ] {
        assert!(validate_input_document(bytes, bytes.len()).is_err());
    }
}

#[test]
fn declaration_validation_never_claims_arithmetic_recomputation() {
    for name in [
        "tampered-quantity-contract-only",
        "tampered-check-contract-only",
        "tampered-unknown-contract-only",
    ] {
        let bytes = analysis_fixture(name);
        AnalysisDocument::from_json(&bytes, bytes.len())
            .expect("contract declarations only; engine MUST reject recomputation mismatch");
    }
}

#[test]
fn known_declarations_cannot_claim_unsupported_source_shapes_or_baseline_x() {
    let mut doc =
        AnalysisDocument::from_json(&analysis_fixture("measurement-gap"), 1_048_576).unwrap();
    let AnalysisArtifact::Measurement(case) = &mut doc.artifact else {
        panic!("measurement")
    };
    let uiblueprint_schema::model::Property::Requested {
        state:
            uiblueprint_schema::model::Availability::Known {
                value: uiblueprint_schema::model::Value::Geometry(g),
            },
        ..
    } = &mut case.snapshot.nodes[0].properties[0]
    else {
        panic!("geometry")
    };
    g.shape = uiblueprint_schema::model::Shape::Polygon(vec![
        uiblueprint_schema::model::Point { x: 10.0, y: 20.0 },
        uiblueprint_schema::model::Point { x: 40.0, y: 20.0 },
        uiblueprint_schema::model::Point { x: 40.0, y: 30.0 },
    ]);
    assert!(doc.validate().is_err());
    let mut doc =
        AnalysisDocument::from_json(&analysis_fixture("baseline-origin-conversion"), 1_048_576)
            .unwrap();
    let AnalysisArtifact::Measurement(case) = &mut doc.artifact else {
        panic!("measurement")
    };
    case.evaluation.transforms[0].affine[1] = 1.0;
    assert!(
        validate_measurement_case(case).is_err(),
        "baseline has no measured x"
    );
    let MeasurementResult::Known { measurement } = &case.result else {
        panic!("known")
    };
    case.result = MeasurementResult::Unknown {
        reason: MeasurementUnknownReason::UnsupportedShape,
        evidence: measurement.evidence.clone(),
    };
    doc.validate()
        .expect("explicit unavailable baseline keeps source evidence");
}

#[test]
fn typed_nonfinite_metadata_and_extra_capacity_cannot_hide() {
    let bytes = analysis_fixture("observed-conditions");
    let mut doc = AnalysisDocument::from_json(&bytes, bytes.len()).unwrap();
    let AnalysisArtifact::Measurement(case) = &mut doc.artifact else {
        panic!("measurement")
    };
    case.evaluation
        .conditions
        .as_mut()
        .unwrap()
        .values
        .text_scale = Some(f64::NAN);
    assert!(
        doc.validate().is_err(),
        "typed NaN cannot disappear into optional null"
    );
    let bytes = analysis_fixture("measurement-gap");
    let mut doc = AnalysisDocument::from_json(&bytes, bytes.len()).unwrap();
    let wire = serde_json::to_vec(&doc).unwrap();
    let before = uiblueprint_schema::owned_size::owned(&doc).unwrap();
    let AnalysisArtifact::Measurement(case) = &mut doc.artifact else {
        panic!("measurement")
    };
    let MeasurementResult::Known { measurement } = &mut case.result else {
        panic!("known")
    };
    let old = measurement.evidence.capacity();
    measurement.evidence.reserve_exact(10);
    let added = (measurement.evidence.capacity() - old)
        * std::mem::size_of::<uiblueprint_schema::model::Evidence>();
    let after = uiblueprint_schema::owned_size::owned(&doc).unwrap();
    assert_eq!(after.1.bytes - before.1.bytes, added);
    assert_eq!(serde_json::to_vec(&doc).unwrap(), wire);
}
