use serde_json::json;
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
