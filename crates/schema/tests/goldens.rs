use serde_json::Value;
use std::{collections::BTreeSet, fs, path::PathBuf};
use uiblueprint_schema::{model::Document, validation::json_schema};

fn structurally_valid(value: &Value) -> Result<bool, &'static str> {
    static VALIDATOR: std::sync::OnceLock<jsonschema::Validator> = std::sync::OnceLock::new();
    let validator = VALIDATOR.get_or_init(|| {
        jsonschema::validator_for(&json_schema().expect("generated schema"))
            .expect("schema compiles")
    });
    Ok(validator.is_valid(value))
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn published_schema_matches_the_canonical_rust_records() {
    let saved: Value = serde_json::from_slice(
        &fs::read(root().join("schemas/uiblueprint-0.1.0.schema.json")).expect("published schema"),
    )
    .expect("schema JSON");
    assert_eq!(saved, json_schema().expect("generate schema"));
}

#[test]
fn nested_array_records_and_extra_tag_only_members_are_rejected() {
    let mut snapshot: Value = serde_json::from_slice(
        &fs::read(root().join("fixtures/golden/ENV-SNAPSHOT-VALID.json")).expect("snapshot"),
    )
    .expect("JSON");
    snapshot["artifact"]["data"]["context"]["target"] = serde_json::json!(["native-app", "g1"]);
    let bytes = serde_json::to_vec(&snapshot).expect("encoded negative");
    assert!(Document::from_json(&bytes, bytes.len()).is_err());
    assert!(!structurally_valid(&snapshot).expect("schema"));
    let mut geometry: Value = serde_json::from_slice(
        &fs::read(root().join("fixtures/golden/GEO-INVALID-SHAPE.json")).expect("geometry"),
    )
    .expect("JSON");
    geometry["artifact"]["data"]["geometry"]["shape"]["value"]["width"] = 30.into();
    geometry["artifact"]["data"]["geometry"]["transform"]["extra"] = true.into();
    let bytes = serde_json::to_vec(&geometry).expect("encoded negative");
    assert!(Document::from_json(&bytes, bytes.len()).is_err());
    assert!(!structurally_valid(&geometry).expect("schema"));
}

#[test]
fn independent_cases_and_every_expanded_variant_match_the_wire_validator() {
    let root = root();
    let manifest: Vec<Value> = serde_json::from_slice(
        &fs::read(root.join("fixtures/golden/manifest.json")).expect("manifest"),
    )
    .expect("manifest JSON");
    let oracle: Value = serde_json::from_slice(
        &fs::read(root.join("fixtures/golden-oracles/expected.json")).expect("independent oracle"),
    )
    .expect("oracle JSON");
    let expected: Vec<_> = oracle["cases"].as_array().expect("cases").iter().collect();
    let mut seen = BTreeSet::new();
    let mut failures = Vec::new();
    for entry in manifest {
        let id = entry["case_id"].as_str().expect("case id");
        seen.insert(id.to_string());
        let expected_exit = expected
            .iter()
            .find(|e| e["case_id"] == id)
            .expect("independent expected case")["validator_exit"]
            .as_u64()
            .expect("exit");
        assert_eq!(entry["validator_exit"], expected_exit);
        let file = entry["path"].as_str().expect("fixture path");
        let bytes = fs::read(root.join("fixtures/golden").join(file)).expect("wire fixture");
        let actual = Document::from_json(&bytes, 1_048_576);
        if actual.is_ok() != (expected_exit == 0) {
            failures.push(format!("{file}: {:?}", actual.as_ref().err()));
        }
        let structural = serde_json::from_slice::<Value>(&bytes)
            .ok()
            .is_some_and(|v| structurally_valid(&v).expect("schema compiles"));
        if structural
            != entry["structural_valid"]
                .as_bool()
                .expect("structural expectation")
        {
            failures.push(format!("{file}: structural parity"));
        }
        if let Ok(doc) = actual {
            let encoded = serde_json::to_vec(&doc).expect("encode valid wire record");
            assert_eq!(
                Document::from_json(&encoded, encoded.len()).expect("round-trip semantic validity"),
                doc
            );
        }
    }
    assert_eq!(
        seen,
        expected
            .iter()
            .map(|e| e["case_id"].as_str().expect("id").to_string())
            .collect()
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn nonfinite_typed_geometry_and_sensitive_known_values_are_rejected_without_echo() {
    let path = root().join("fixtures/golden/GEO-TRANSFORM.json");
    let mut doc =
        Document::from_json(&fs::read(path).expect("fixture"), 1_048_576).expect("valid transform");
    if let uiblueprint_schema::model::Artifact::Geometry(case) = &mut doc.artifact
        && let uiblueprint_schema::model::Shape::Rect(rect) = &mut case.geometry.shape
    {
        rect.x = f64::INFINITY;
    }
    assert!(doc.validate().is_err());
    let mut raw: Value = serde_json::from_slice(
        &fs::read(root().join("fixtures/golden/PROP-KNOWN_EMPTY.json")).expect("property fixture"),
    )
    .expect("json");
    raw["artifact"]["data"]["property"]["sensitivity"] = Value::String("sensitive".into());
    raw["artifact"]["data"]["property"]["state"]["value"]["value"] =
        Value::String("S01_TEST_SECRET".into());
    let bytes = serde_json::to_vec(&raw).expect("encoded adversarial record");
    let error =
        Document::from_json(&bytes, bytes.len()).expect_err("sensitive value cannot be stored");
    assert!(!error.to_string().contains("S01_TEST_SECRET"));
    assert!(!format!("{error:?}").contains("S01_TEST_SECRET"));
}
