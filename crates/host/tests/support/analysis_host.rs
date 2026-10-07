//! Actual guarded analysis dispatch, not a repeated engine arithmetic suite.
use super::*;
use uiblueprint_host::worker_tape;
use uiblueprint_schema::{
    SchemaVersion,
    analysis::{AnalysisArtifact, AnalysisDocument, AnalysisVersion, MeasurementResult},
    model::{CheckStatus, Value},
};
const MEASUREMENT: &[u8] = include_bytes!("../../../../fixtures/analysis/measurement-gap.json");
const CHECK: &[u8] = include_bytes!("../../../../fixtures/analysis/check-pass.json");
const TAMPERED: &[u8] =
    include_bytes!("../../../../fixtures/analysis/tampered-check-contract-only.json");
fn encode_core(artifact: Artifact) -> Vec<u8> {
    serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact,
    })
    .unwrap()
}
fn encode_analysis(artifact: AnalysisArtifact) -> Vec<u8> {
    serde_json::to_vec(&AnalysisDocument {
        schema_version: AnalysisVersion::CURRENT,
        artifact,
    })
    .unwrap()
}
#[test]
fn guarded_measure_check_and_recomputation_preserve_canonical_binding() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let expected = AnalysisDocument::from_json(MEASUREMENT, 65536).unwrap();
    let AnalysisArtifact::Measurement(case) = &expected.artifact else {
        panic!("measurement")
    };
    let snapshot = encode_core(Artifact::Snapshot(Box::new(case.snapshot.clone())));
    let query = encode_analysis(AnalysisArtifact::GeometryQuery(Box::new(
        case.query.clone(),
    )));
    let evaluation = encode_analysis(AnalysisArtifact::EvaluationInput(Box::new(
        case.evaluation.clone(),
    )));
    let checked = AnalysisDocument::from_json(CHECK, 65536).unwrap();
    let AnalysisArtifact::GeometryCheck(check) = &checked.artifact else {
        panic!("check")
    };
    let expectation = encode_core(Artifact::Expectation(Box::new(check.expectation.clone())));
    // This fixture is deliberately contract-valid but fails actual recomputation.
    AnalysisDocument::from_json(TAMPERED, 65536).unwrap();
    let domain = HostDomain::new::<DarwinPlatform>(limits()).unwrap();
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
        DarwinPlatform,
    )
    .unwrap();
    let mut input = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    input.bytes_mut().copy_from_slice(DESCRIPTOR);
    let session = host.attach(input, deadline()).unwrap();
    assert!(matches!(next(&mut host), HostEvent::Attached { .. }));
    let output = OutputRequest {
        channels: 1,
        frame_bytes: 65536,
        total_bytes: 65536,
        input_format: 1,
        retained_partition: 0,
    };
    let parts = [snapshot.as_slice(), query.as_slice(), evaluation.as_slice()];
    let length = 40 + parts.iter().map(|p| p.len()).sum::<usize>();
    let mut input = host.reserve_input(session, length).unwrap();
    worker_tape::encode(&parts, input.bytes_mut()).unwrap();
    host.submit(session, OperationClass::Measure, input, output, deadline())
        .unwrap();
    let measurement = complete(&mut host);
    assert_eq!(measurement.terminal, Terminal::Completed);
    let actual = AnalysisDocument::from_json(measurement.bytes(0).unwrap(), 65536).unwrap();
    assert_eq!(actual, expected);
    let AnalysisArtifact::Measurement(m) = &actual.artifact else {
        panic!("result")
    };
    let MeasurementResult::Known { measurement: m } = &m.result else {
        panic!("known")
    };
    let Value::Quantity { amount, .. } = &m.value else {
        panic!("quantity")
    };
    assert_eq!(*amount, 8.0);
    assert_eq!(m.space, case.evaluation.result_space);
    let mut input = host
        .reserve_input(session, measurement.bytes(0).unwrap().len())
        .unwrap();
    input
        .bytes_mut()
        .copy_from_slice(measurement.bytes(0).unwrap());
    host.submit(session, OperationClass::Verify, input, output, deadline())
        .unwrap();
    let verified = complete(&mut host);
    assert_eq!(verified.terminal, Terminal::Completed);
    assert_eq!(verified.bytes(0), measurement.bytes(0));
    drop(verified);
    drop(measurement);
    let parts = [
        snapshot.as_slice(),
        expectation.as_slice(),
        evaluation.as_slice(),
    ];
    let length = 40 + parts.iter().map(|p| p.len()).sum::<usize>();
    let mut input = host.reserve_input(session, length).unwrap();
    worker_tape::encode(&parts, input.bytes_mut()).unwrap();
    host.submit(session, OperationClass::Check, input, output, deadline())
        .unwrap();
    let result = complete(&mut host);
    assert_eq!(result.terminal, Terminal::Completed);
    let got = AnalysisDocument::from_json(result.bytes(0).unwrap(), 65536).unwrap();
    let AnalysisArtifact::GeometryCheck(got) = got.artifact else {
        panic!("check output")
    };
    assert_eq!(got.finding.status, CheckStatus::Pass);
    assert_eq!(got.measurement, check.measurement);
    assert_eq!(got.snapshot, check.snapshot);
    assert_eq!(got.expectation, check.expectation);
    assert_eq!(got.evaluation, check.evaluation);
    drop(result);
    let mut input = host.reserve_input(session, TAMPERED.len()).unwrap();
    input.bytes_mut().copy_from_slice(TAMPERED);
    host.submit(session, OperationClass::Verify, input, output, deadline())
        .unwrap();
    let refused = complete(&mut host);
    assert_eq!(refused.terminal, Terminal::Failed(HostError::InvalidInput));
    assert_eq!(refused.committed(), 0);
    assert_eq!(refused.missing(), 1);
    drop(refused);
    let end = deadline();
    loop {
        assert!(Instant::now() < end);
        if matches!(host.shutdown().unwrap(), HostEvent::ShutdownComplete) {
            break;
        }
        thread::yield_now();
    }
    assert_eq!(domain.usage().reserved_sessions, 0);
}
