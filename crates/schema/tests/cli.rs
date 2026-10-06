use serde_json::Value;
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_uiblueprint-validate"))
}

#[test]
fn cli_matches_independent_expected_exit_for_every_wire_fixture() {
    let manifest: Vec<Value> = serde_json::from_slice(
        &fs::read(root().join("fixtures/golden/manifest.json")).expect("manifest"),
    )
    .expect("JSON");
    for entry in manifest {
        let path = root()
            .join("fixtures/golden")
            .join(entry["path"].as_str().expect("fixture"));
        let output = command()
            .args(["--max-bytes", "1048576"])
            .arg(path)
            .output()
            .expect("validator process");
        let expected = entry["validator_exit"].as_i64().expect("expected exit") as i32;
        assert_eq!(
            output.status.code(),
            Some(expected),
            "case {}",
            entry["case_id"]
        );
        let answer: Value =
            serde_json::from_slice(&output.stdout).expect("one machine JSON response");
        assert_eq!(answer["valid"], expected == 0);
        assert!(
            output.stderr.is_empty(),
            "validator must not print source data"
        );
    }
}

#[test]
fn cli_io_limits_stdin_and_private_diagnostics_have_distinct_results() {
    let missing = root().join("fixtures/golden/intentionally-absent-s01-input.json");
    assert!(!missing.exists());
    let output = command()
        .args(["--max-bytes", "1024"])
        .arg(missing)
        .output()
        .expect("IO negative");
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        serde_json::from_slice::<Value>(&output.stdout).expect("JSON")["code"],
        "io_error"
    );
    let output = command()
        .args(["--max-bytes", "1"])
        .arg(root().join("fixtures/golden/ENV-REQUEST-VALID.json"))
        .output()
        .expect("bounded read");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        serde_json::from_slice::<Value>(&output.stdout).expect("JSON")["code"],
        "resource_limit"
    );

    let mut child = command()
        .args(["--max-bytes", "1024", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("stdin validator");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(br#"{"schema_version":"0.1.0","private":"S01_CLI_SECRET"}"#)
        .expect("write bounded invalid input");
    let output = child.wait_with_output().expect("terminal response");
    assert_eq!(output.status.code(), Some(2));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("S01_CLI_SECRET"));
    assert!(!String::from_utf8_lossy(&output.stderr).contains("S01_CLI_SECRET"));
    assert!(output.stderr.is_empty());
}
