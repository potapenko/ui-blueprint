use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
use uiblueprint_schema::{SchemaVersion, model::*};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Case {
    directory: PathBuf,
    snapshot: Snapshot,
    expectation: Expectation,
}
impl Case {
    fn new(name: &str) -> Self {
        let path = format!(
            "{}/../../fixtures/golden/{name}.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let document: Document =
            serde_json::from_slice(&fs::read(path).expect("fixture")).expect("canonical fixture");
        let Artifact::Finding(case) = document.artifact else {
            panic!("finding fixture")
        };
        let directory = std::env::temp_dir().join(format!(
            "uiblueprint-l01-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&directory).expect("unique task temporary directory");
        let result = Self {
            directory,
            snapshot: case.snapshot,
            expectation: case.expectation,
        };
        result.save();
        result
    }
    fn save(&self) {
        let document = |artifact| Document {
            schema_version: SchemaVersion::CURRENT,
            artifact,
        };
        fs::write(
            self.directory.join("snapshot.json"),
            serde_json::to_vec(&document(Artifact::Snapshot(Box::new(
                self.snapshot.clone(),
            ))))
            .expect("serialize snapshot"),
        )
        .expect("save snapshot");
        fs::write(
            self.directory.join("expectation.json"),
            serde_json::to_vec(&document(Artifact::Expectation(Box::new(
                self.expectation.clone(),
            ))))
            .expect("serialize expectation"),
        )
        .expect("save expectation");
    }
    fn run(&self, command: &str, json: bool, input: usize, output: usize) -> Output {
        let Rule::Geometry { anchors, .. } = &self.expectation.rule else {
            panic!("geometry fixture")
        };
        self.run_at(
            command,
            json,
            input,
            output,
            &anchors[0].coordinate_space.id.0,
        )
    }
    fn run_at(
        &self,
        command: &str,
        json: bool,
        input: usize,
        output: usize,
        space: &str,
    ) -> Output {
        let mut child = Command::new(env!("CARGO_BIN_EXE_uiblueprint"));
        child
            .arg(command)
            .arg("--snapshot")
            .arg(self.directory.join("snapshot.json"))
            .arg("--expectation")
            .arg(self.directory.join("expectation.json"))
            .args([
                "--space",
                space,
                "--max-input-bytes",
                &input.to_string(),
                "--max-output-bytes",
                &output.to_string(),
            ]);
        if json {
            child.arg("--json");
        }
        child.output().expect("invoke CLI")
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.directory).expect("remove only this case's temporary directory");
    }
}
fn assert_error(output: Output, exit: i32, code: &str) {
    assert_eq!(output.status.code(), Some(exit));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, format!("{code}\n").as_bytes());
}

#[test]
fn independent_geo_checks_emit_canonical_full_evidence_and_distinct_exits() {
    for (name, exit, status, amount) in [
        ("GEO-GAP", 0, CheckStatus::Pass, Some(8.0)),
        ("GEO-GAP-FAIL", 3, CheckStatus::Fail, Some(8.0)),
        ("GEO-BASELINE-UNKNOWN", 4, CheckStatus::Unknown, None),
        ("GEO-FRAME-KIND", 4, CheckStatus::Unknown, None),
    ] {
        let case = Case::new(name);
        let output = case.run("check", true, 100_000, 100_000);
        assert_eq!(
            output.status.code(),
            Some(exit),
            "{name}: {:?}",
            output.stderr
        );
        assert!(output.stderr.is_empty());
        let document = Document::from_json(&output.stdout, 100_000).expect("canonical output");
        let Artifact::Finding(result) = document.artifact else {
            panic!("finding output")
        };
        assert_eq!(result.snapshot, case.snapshot);
        assert_eq!(result.expectation, case.expectation);
        assert_eq!(result.finding.status, status);
        let measured = result.finding.measured.as_ref().map(|v| match v {
            Value::Quantity { amount, .. } => *amount,
            _ => panic!("quantity"),
        });
        assert_eq!(measured, amount);
    }
}
#[test]
fn compact_is_default_saved_analysis_with_source_and_limits() {
    let case = Case::new("GEO-GAP");
    let output = case.run("check", false, 100_000, 100_000);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let text = String::from_utf8(output.stdout).expect("utf8");
    for expected in [
        "analysis=saved_snapshot",
        "check=Pass",
        "snapshot=Id(\"S11\")",
        "coverage=",
        "selected_fields=",
        "input_bytes=100000",
        "observation=\"OG1\"",
        "freshness=Current",
        "expected_from=\"fixture_literal\"",
        "amount: 8.0",
        "evidence=",
    ] {
        assert!(text.contains(expected), "missing {expected}");
    }
}
#[test]
fn measure_returns_fact_without_inventing_success_for_failed_expectation() {
    let case = Case::new("GEO-GAP-FAIL");
    let output = case.run("measure", false, 100_000, 100_000);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let text = String::from_utf8(output.stdout).expect("utf8");
    assert!(text.contains("measurement=known"));
    assert!(text.contains("amount: 8.0"));
    assert!(!text.contains("check="));
    let unknown = Case::new("GEO-BASELINE-UNKNOWN").run("measure", false, 100_000, 100_000);
    assert_eq!(unknown.status.code(), Some(4));
    assert!(
        String::from_utf8(unknown.stdout)
            .expect("utf8")
            .contains("measurement=unknown")
    );
}
#[test]
fn missing_shared_measurement_json_contract_is_explicit() {
    assert_error(
        Case::new("GEO-GAP").run("measure", true, 100_000, 100_000),
        5,
        "consumer_contract_gap",
    );
}
#[test]
fn input_limit_is_aggregate_and_output_limit_has_no_partial_json() {
    let case = Case::new("GEO-GAP");
    let total = (fs::metadata(case.directory.join("snapshot.json"))
        .expect("size")
        .len()
        + fs::metadata(case.directory.join("expectation.json"))
            .expect("size")
            .len()) as usize;
    assert_error(
        case.run("check", true, total - 1, 100_000),
        2,
        "input_limit",
    );
    let exact = case.run("check", true, total, 100_000);
    assert_eq!(exact.status.code(), Some(0));
    let size = exact.stdout.len();
    assert_error(case.run("check", true, total, size - 1), 2, "output_limit");
    assert_eq!(case.run("check", true, total, size).stdout, exact.stdout);
    assert_error(case.run("check", false, total, 5), 2, "output_limit");
}
#[test]
fn invalid_documents_and_io_errors_never_echo_private_payloads_or_paths() {
    let case = Case::new("GEO-GAP");
    for bytes in [
        b"PRIVATE_CANARY_L01_not_json".as_slice(),
        b"{\"schema_version\":\"9.0.0\",\"private\":\"PRIVATE_CANARY_L01\"}".as_slice(),
    ] {
        fs::write(case.directory.join("snapshot.json"), bytes).expect("test input");
        assert_error(
            case.run("check", true, 100_000, 100_000),
            2,
            "invalid_input",
        );
    }
    fs::remove_file(case.directory.join("snapshot.json")).expect("remove own file");
    assert_error(case.run("check", true, 100_000, 100_000), 1, "io_error");
}
#[test]
fn compact_escapes_untrusted_control_text() {
    let mut case = Case::new("GEO-GAP");
    case.expectation.expected_from = Id("fixture\n\u{1b}[31mPRIVATE_CANARY_L01".into());
    case.save();
    let output = case.run("check", false, 100_000, 100_000);
    assert_eq!(output.status.code(), Some(0));
    assert!(!output.stdout.contains(&0x1b));
    let text = String::from_utf8(output.stdout).expect("utf8");
    assert!(text.contains("fixture\\n\\u{1b}"));
}
#[test]
fn applicability_is_unknown_without_new_observation_or_inference() {
    let mut case = Case::new("GEO-GAP");
    case.expectation.applies_when.platform = Some(Id("web".into()));
    case.save();
    let output = case.run("check", true, 100_000, 100_000);
    assert_eq!(output.status.code(), Some(4));
    let document = Document::from_json(&output.stdout, 100_000).expect("unknown canonical result");
    let Artifact::Finding(result) = document.artifact else {
        panic!("finding")
    };
    assert_eq!(
        result.finding.reason,
        Some(Id("applicability_unknown".into()))
    );
}
#[test]
fn finite_command_and_argument_contract() {
    for (args, exit, code) in [
        (
            vec!["observe", "PRIVATE_CANARY_L01"],
            5,
            "unsupported_command",
        ),
        (vec!["check"], 2, "invalid_arguments"),
        (
            vec!["check", "--max-input-bytes", "0"],
            2,
            "invalid_arguments",
        ),
        (vec!["check", "--json", "--json"], 2, "invalid_arguments"),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
            .args(args)
            .output()
            .expect("invoke");
        assert_error(output, exit, code);
    }
    let output = Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
        .arg("--help")
        .output()
        .expect("help");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    assert!(output.stdout.starts_with(b"UI Blueprint"));
}

#[test]
fn converted_json_does_not_drop_selected_result_space() {
    let mut case = Case::new("GEO-SIZE-RATIO__width");
    let Property::Requested {
        evidence,
        state: Availability::Known {
            value: Value::Geometry(geometry),
        },
        ..
    } = &mut case.snapshot.nodes[0].properties[0]
    else {
        panic!("geometry")
    };
    let mut to = geometry.coordinate_space.clone();
    to.id = Id("converted".into());
    geometry.transform = TransformState::Known {
        transform: Box::new(Transform {
            from: geometry.coordinate_space.clone(),
            to,
            affine: [2.0, 0.0, 0.0, 2.0, 0.0, 0.0],
            target: case.snapshot.context.target.clone(),
            surface: case.snapshot.context.surfaces[0].clone(),
            environment_revision: case.snapshot.context.environment_revision.clone(),
            evidence: evidence.clone(),
        }),
    };
    case.save();
    assert_error(
        case.run_at("check", true, 100_000, 100_000, "converted"),
        5,
        "consumer_contract_gap",
    );
    let compact = case.run_at("measure", false, 100_000, 100_000, "converted");
    assert_eq!(compact.status.code(), Some(0));
    let text = String::from_utf8(compact.stdout).expect("utf8");
    assert!(text.contains("amount: 60.0"));
    assert!(text.contains("converted"));
}

#[test]
fn unknown_space_and_sensitive_canonical_input_fail_without_payload_output() {
    let mut case = Case::new("GEO-GAP");
    assert_error(
        case.run_at("check", true, 100_000, 100_000, "missing-space"),
        2,
        "unknown_space",
    );
    case.snapshot.nodes[0]
        .source_declarations
        .push(SourceDeclaration {
            namespace: Id("fixture.private".into()),
            name: Id("secret".into()),
            state: Availability::Known {
                value: Value::Text("PRIVATE_CANARY_L01_SENSITIVE".into()),
            },
            sensitivity: Sensitivity::Sensitive,
            source: Id("fixture".into()),
        });
    case.save();
    assert_error(
        case.run("check", true, 100_000, 100_000),
        2,
        "invalid_input",
    );
}
