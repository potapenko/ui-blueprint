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
fn run_stdin(bytes: &[u8], limit: usize) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_uiblueprint-validate"))
        .args(["--max-bytes", &limit.to_string(), "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn validator_binary_matches_authored_analysis_contract_results() {
    let manifest: Vec<Value> =
        serde_json::from_slice(&fs::read(root().join("fixtures/analysis/manifest.json")).unwrap())
            .unwrap();
    for row in manifest {
        let path = root()
            .join("fixtures/analysis")
            .join(row["path"].as_str().unwrap());
        let bytes = fs::read(&path).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_uiblueprint-validate"))
            .args(["--max-bytes", &bytes.len().to_string()])
            .arg(&path)
            .output()
            .unwrap();
        let valid = row["contract_valid"].as_bool().unwrap();
        assert_eq!(
            output.status.code(),
            Some(if valid { 0 } else { 2 }),
            "{}",
            path.display()
        );
        let result: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(result["valid"], valid);
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn two_version_dispatch_is_bounded_strict_and_payload_free() {
    let bytes = fs::read(root().join("fixtures/analysis/query-gap.json")).unwrap();
    assert_eq!(run_stdin(&bytes, bytes.len()).status.code(), Some(0));
    let output = run_stdin(&bytes, bytes.len() - 1);
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        serde_json::from_slice::<Value>(&output.stdout).unwrap()["code"],
        "resource_limit"
    );
    let core = include_bytes!("../../../fixtures/golden/ENV-SNAPSHOT-VALID.json");
    assert_eq!(run_stdin(core, core.len()).status.code(), Some(0));
    for bytes in [
        br#"{"schema_version":"PRIVATE-VERSION-CANARY","artifact":{}}"#.as_slice(),
        br#"{"schema_version":"0.2.0","artifact":{"kind":"PRIVATE-TAG-CANARY","data":{}}}"#
            .as_slice(),
        br#"{"schema_version":"0.2.0","schema_version":"PRIVATE-DUPLICATE-CANARY","artifact":{}}"#
            .as_slice(),
    ] {
        let output = run_stdin(bytes, bytes.len());
        assert_eq!(output.status.code(), Some(2));
        assert!(!String::from_utf8_lossy(&output.stdout).contains("CANARY"));
        assert!(!String::from_utf8_lossy(&output.stderr).contains("CANARY"));
        assert!(output.stderr.is_empty());
    }
}
