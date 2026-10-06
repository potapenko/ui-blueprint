use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
use uiblueprint_schema::{SchemaVersion, analysis::*, model::*};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Case {
    dir: PathBuf,
    snapshot: Snapshot,
    expectation: Expectation,
    query: GeometryQuery,
    evaluation: EvaluationInput,
}
impl Case {
    fn new() -> Self {
        let path = format!(
            "{}/../../fixtures/golden/GEO-GAP.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let doc: Document =
            serde_json::from_slice(&fs::read(path).expect("fixture")).expect("canonical fixture");
        let Artifact::Finding(case) = doc.artifact else {
            panic!("fixture")
        };
        let query = GeometryQuery::from_expectation(&case.expectation).expect("query");
        let evaluation = EvaluationInput {
            snapshot_id: case.snapshot.id.clone(),
            revision: case.snapshot.revision,
            context: case.snapshot.context.clone(),
            result_space: query.anchors[0].coordinate_space.clone(),
            transforms: vec![],
            conditions: None,
        };
        let dir = std::env::temp_dir().join(format!(
            "uib-analysis-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).expect("unique task directory");
        let result = Self {
            dir,
            snapshot: case.snapshot,
            expectation: case.expectation,
            query,
            evaluation,
        };
        result.save();
        result
    }
    fn save(&self) {
        let core = |artifact| Document {
            schema_version: SchemaVersion::CURRENT,
            artifact,
        };
        let analysis = |artifact| AnalysisDocument {
            schema_version: AnalysisVersion::CURRENT,
            artifact,
        };
        for (name, bytes) in [
            (
                "snapshot",
                serde_json::to_vec(&core(Artifact::Snapshot(Box::new(self.snapshot.clone())))),
            ),
            (
                "expectation",
                serde_json::to_vec(&core(Artifact::Expectation(Box::new(
                    self.expectation.clone(),
                )))),
            ),
            (
                "query",
                serde_json::to_vec(&analysis(AnalysisArtifact::GeometryQuery(Box::new(
                    self.query.clone(),
                )))),
            ),
            (
                "evaluation",
                serde_json::to_vec(&analysis(AnalysisArtifact::EvaluationInput(Box::new(
                    self.evaluation.clone(),
                )))),
            ),
        ] {
            fs::write(self.dir.join(name), bytes.expect("serialize fixture"))
                .expect("write own input");
        }
    }
    fn command(
        &self,
        operation: &str,
        source: &str,
        evaluation: bool,
        json: bool,
        input: usize,
        output: usize,
    ) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_uiblueprint"));
        command
            .arg(operation)
            .arg("--snapshot")
            .arg(self.dir.join("snapshot"))
            .arg(format!("--{source}"))
            .arg(self.dir.join(source))
            .args([
                "--space",
                &self.evaluation.result_space.id.0,
                "--max-input-bytes",
                &input.to_string(),
                "--max-output-bytes",
                &output.to_string(),
            ]);
        if evaluation {
            command.arg("--evaluation").arg(self.dir.join("evaluation"));
        }
        if json {
            command.arg("--json");
        }
        command
    }
    fn run(
        &self,
        operation: &str,
        source: &str,
        evaluation: bool,
        json: bool,
        version: Option<&str>,
    ) -> Output {
        let mut command = self.command(operation, source, evaluation, json, 200_000, 200_000);
        if let Some(version) = version {
            command.args(["--result-version", version]);
        }
        command.output().expect("invoke actual CLI")
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.dir).expect("remove only own temporary inputs");
    }
}
fn analysis(output: &Output, exit: i32) -> AnalysisDocument {
    assert_eq!(
        output.status.code(),
        Some(exit),
        "stderr {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    AnalysisDocument::from_json(&output.stdout, 200_000).expect("production analysis parser")
}
fn error(output: Output, exit: i32, code: &str) {
    assert_eq!(output.status.code(), Some(exit));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, format!("{code}\n").as_bytes());
}
fn evidence(case: &Case) -> Evidence {
    let Property::Requested { evidence, .. } = &case.snapshot.nodes[0].properties[0] else {
        panic!("source")
    };
    evidence.clone()
}
fn measurement(document: &AnalysisDocument) -> &Measurement {
    let AnalysisArtifact::Measurement(case) = &document.artifact else {
        panic!("measurement")
    };
    let MeasurementResult::Known { measurement } = &case.result else {
        panic!("known")
    };
    measurement
}

#[test]
fn actual_factual_measure_json_and_compact_have_no_invented_expectation() {
    let case = Case::new();
    let output = case.run("measure", "query", false, true, None);
    let doc = analysis(&output, 0);
    assert_eq!(
        measurement(&doc).value,
        Value::Quantity {
            amount: 8.0,
            kind: QuantityKind::Length,
            source_units: Unit::CssPx
        }
    );
    let AnalysisArtifact::Measurement(bundle) = &doc.artifact else {
        panic!("bundle")
    };
    assert_eq!(bundle.snapshot, case.snapshot);
    assert_eq!(bundle.query, case.query);
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("inspect keys");
    assert_eq!(json["schema_version"], "0.2.0");
    assert_eq!(
        json["artifact"]["data"]["snapshot"]["context"]["schema_version"],
        "0.1.0"
    );
    assert!(json["artifact"]["data"].get("expectation").is_none());
    assert!(json["artifact"]["data"]["query"].get("expected").is_none());
    uiblueprint_engine::verify_analysis_result(&doc).expect("independent engine recomputation");
    let compact = case.run("measure", "query", false, false, None);
    assert_eq!(compact.status.code(), Some(0));
    let text = String::from_utf8(compact.stdout).expect("utf8");
    assert!(text.contains("query="));
    assert!(!text.contains("expectation="));
    assert!(!text.contains("expected_from="));
    assert!(!text.contains("check="));
    assert!(text.contains("selected_space="));
    let compatibility = analysis(&case.run("measure", "expectation", false, true, None), 0);
    assert_eq!(measurement(&compatibility), measurement(&doc));
}
#[test]
fn legacy_check_default_and_explicit_versions_remain_distinct() {
    let case = Case::new();
    for version in [None, Some("0.1.0")] {
        let output = case.run("check", "expectation", true, true, version);
        assert_eq!(output.status.code(), Some(0));
        let core = Document::from_json(&output.stdout, 200_000).expect("legacy core result");
        assert!(matches!(core.artifact, Artifact::Finding(_)));
        assert!(AnalysisDocument::from_json(&output.stdout, 200_000).is_err());
    }
    let output = case.run("check", "expectation", true, true, Some("0.2.0"));
    let doc = analysis(&output, 0);
    assert!(Document::from_json(&output.stdout, 200_000).is_err());
    let AnalysisArtifact::GeometryCheck(bundle) = &doc.artifact else {
        panic!("check")
    };
    assert_eq!(bundle.snapshot, case.snapshot);
    assert_eq!(bundle.expectation, case.expectation);
    assert_eq!(bundle.evaluation, case.evaluation);
    uiblueprint_engine::verify_analysis_result(&doc).expect("verify");
}
#[test]
fn actual_converted_conditional_check_keeps_complete_evaluation_and_provenance() {
    let mut case = Case::new();
    let local = case.evaluation.result_space.clone();
    let mut middle = local.clone();
    middle.id = Id("middle".into());
    let mut image = middle.clone();
    image.id = Id("image".into());
    image.units = Unit::Px;
    image.origin = Origin::BottomLeft;
    let mut condition_observation = case.snapshot.observations[0].clone();
    condition_observation.id = Id("OC".into());
    condition_observation.source_namespace = Id("fixture.conditions".into());
    case.snapshot
        .observations
        .push(condition_observation.clone());
    let condition_evidence = Evidence {
        observation_id: condition_observation.id,
        source_namespace: condition_observation.source_namespace,
        provenance: Provenance::Reported,
        method: Id("observed_context".into()),
        uncertainty: None,
    };
    case.query.applies_when.platform = Some(Id("fixture".into()));
    case.expectation.applies_when = case.query.applies_when.clone();
    case.evaluation.conditions = Some(ObservedConditions {
        values: ContextConditions {
            platform: Some(Id("fixture".into())),
            input_mode: None,
            text_scale: None,
        },
        evidence: condition_evidence.clone(),
    });
    let transform = |from, to, affine, method: &str| {
        let mut source = evidence(&case);
        source.method = Id(method.into());
        Transform {
            from,
            to,
            affine,
            target: case.snapshot.context.target.clone(),
            surface: case.snapshot.context.surfaces[0].clone(),
            environment_revision: case.snapshot.context.environment_revision.clone(),
            evidence: source,
        }
    };
    case.evaluation.transforms = vec![
        transform(
            local,
            middle.clone(),
            [1.0, 0.0, 0.0, 1.0, 3.0, 4.0],
            "translate",
        ),
        transform(
            middle,
            image.clone(),
            [2.0, 0.0, 0.0, -2.0, 0.0, 100.0],
            "scale_flip",
        ),
    ];
    case.evaluation.result_space = image;
    case.query.units = Unit::Px;
    let Rule::Geometry {
        expected, units, ..
    } = &mut case.expectation.rule
    else {
        panic!("rule")
    };
    *expected = 16.0;
    *units = Unit::Px;
    case.save();
    let doc = analysis(
        &case.run("check", "expectation", true, true, Some("0.2.0")),
        0,
    );
    let AnalysisArtifact::GeometryCheck(bundle) = &doc.artifact else {
        panic!("check")
    };
    assert_eq!(bundle.snapshot, case.snapshot);
    assert_eq!(bundle.evaluation, case.evaluation);
    assert_eq!(bundle.finding.observation_id, Some(Id("OC".into())));
    let MeasurementResult::Known { measurement } = &bundle.measurement else {
        panic!("known")
    };
    assert_eq!(measurement.evidence.len(), 4);
    assert_eq!(measurement.evidence[0], condition_evidence);
    assert_eq!(
        measurement.value,
        Value::Quantity {
            amount: 16.0,
            kind: QuantityKind::Length,
            source_units: Unit::Px
        }
    );
    uiblueprint_engine::verify_analysis_result(&doc).expect("converted recomputation");
    error(
        case.run("check", "expectation", true, true, None),
        5,
        "unsupported_result_version",
    );
    assert_eq!(
        case.run("check", "expectation", true, false, None)
            .status
            .code(),
        Some(0)
    );
}
#[test]
fn missing_conditions_and_unavailable_measurements_emit_unknown_not_fake_values() {
    let mut case = Case::new();
    case.query.applies_when.platform = Some(Id("fixture".into()));
    case.save();
    let doc = analysis(&case.run("measure", "query", false, true, None), 4);
    let AnalysisArtifact::Measurement(bundle) = doc.artifact else {
        panic!("case")
    };
    assert_eq!(
        bundle.result.unknown_reason(),
        Some(MeasurementUnknownReason::ApplicabilityUnknown)
    );
    assert_eq!(bundle.evaluation.result_space, case.evaluation.result_space);
    case.query.applies_when.platform = None;
    let Property::Requested { state, .. } = &mut case.snapshot.nodes[1].properties[0] else {
        panic!("property")
    };
    *state = Availability::Redacted {};
    case.save();
    let output = case.run("measure", "query", false, true, None);
    let doc = analysis(&output, 4);
    let AnalysisArtifact::Measurement(bundle) = doc.artifact else {
        panic!("case")
    };
    assert_eq!(
        bundle.result.unknown_reason(),
        Some(MeasurementUnknownReason::RedactedProperty)
    );
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("keys");
    assert!(
        json["artifact"]["data"]["result"]
            .get("measurement")
            .is_none()
    );
}
#[test]
fn binding_versions_and_flag_combinations_refuse_without_partial_output() {
    let mut case = Case::new();
    case.evaluation.snapshot_id = Id("PRIVATE_CANARY_BAD_BINDING".into());
    case.save();
    error(
        case.run("measure", "query", true, true, None),
        2,
        "invalid_input",
    );
    case.evaluation.snapshot_id = case.snapshot.id.clone();
    case.save();
    error(
        case.run("check", "query", false, true, None),
        2,
        "invalid_arguments",
    );
    error(
        case.run("measure", "query", false, true, Some("0.1.0")),
        5,
        "unsupported_result_version",
    );
    error(
        case.run("check", "expectation", false, true, Some("9.0.0")),
        5,
        "unsupported_result_version",
    );
    error(
        case.run("check", "expectation", false, false, Some("0.2.0")),
        2,
        "invalid_arguments",
    );
    let mut both = case.command("measure", "query", false, true, 200_000, 200_000);
    both.arg("--expectation").arg(case.dir.join("expectation"));
    error(both.output().expect("both sources"), 2, "invalid_arguments");
    fs::write(
        case.dir.join("query"),
        b"{\"schema_version\":\"0.1.0\",\"private\":\"PRIVATE_CANARY\"}",
    )
    .expect("invalid input");
    error(
        case.run("measure", "query", false, true, None),
        2,
        "invalid_input",
    );
}
#[test]
fn query_evaluation_share_aggregate_input_and_output_budget() {
    let case = Case::new();
    let total = ["snapshot", "query", "evaluation"]
        .into_iter()
        .map(|p| fs::metadata(case.dir.join(p)).expect("size").len() as usize)
        .sum::<usize>();
    error(
        case.command("measure", "query", true, true, total - 1, 200_000)
            .output()
            .expect("limit"),
        2,
        "input_limit",
    );
    let good = case
        .command("measure", "query", true, true, total, 200_000)
        .output()
        .expect("exact input");
    analysis(&good, 0);
    let size = good.stdout.len();
    error(
        case.command("measure", "query", true, true, total, size - 1)
            .output()
            .expect("output limit"),
        2,
        "output_limit",
    );
    assert_eq!(
        case.command("measure", "query", true, true, total, size)
            .output()
            .expect("exact output")
            .stdout,
        good.stdout
    );
}
#[test]
fn production_roundtrip_keeps_difficult_float_bits_without_an_epsilon() {
    let mut case = Case::new();
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(geometry),
        },
        ..
    } = &mut case.snapshot.nodes[0].properties[0]
    else {
        panic!("geometry")
    };
    let Shape::Rect(rect) = &mut geometry.shape else {
        panic!("rect")
    };
    rect.x = 0.0;
    rect.width = 1.2345678901234567;
    case.query.operation = GeometryRelation::Width;
    case.query.anchors.truncate(1);
    case.query.targets.truncate(1);
    case.save();
    let doc = analysis(&case.run("measure", "query", false, true, None), 0);
    let Value::Quantity { amount, .. } = measurement(&doc).value else {
        panic!("quantity")
    };
    assert_eq!(amount.to_bits(), 1.2345678901234567_f64.to_bits());
    uiblueprint_engine::verify_analysis_result(&doc).expect("exact roundtrip recomputation");
}

#[test]
fn explicit_evaluation_space_must_resolve_unambiguously_and_match() {
    let mut case = Case::new();
    case.evaluation.result_space.units = Unit::Pt;
    case.save();
    error(
        case.run("measure", "query", true, true, None),
        2,
        "invalid_input",
    );
    case.evaluation.result_space.units = Unit::CssPx;
    let local = case.evaluation.result_space.clone();
    let mut conflicting = local.clone();
    conflicting.units = Unit::Pt;
    let source = evidence(&case);
    case.evaluation.transforms = vec![Transform {
        from: conflicting,
        to: local,
        affine: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        target: case.snapshot.context.target.clone(),
        surface: case.snapshot.context.surfaces[0].clone(),
        environment_revision: case.snapshot.context.environment_revision.clone(),
        evidence: source,
    }];
    case.save();
    error(
        case.run("measure", "query", true, true, None),
        2,
        "ambiguous_space",
    );
}
