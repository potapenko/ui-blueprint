//! Finite byte families enter only the real quota-installed session-worker.
//! The parent builds/copies bounded bytes, never parses hostile JSON/DTO/Content.
#![cfg(target_os = "macos")]
#![forbid(unsafe_code)]
use std::{
    cell::Cell,
    os::fd::BorrowedFd,
    path::Path,
    rc::Rc,
    sync::{Mutex, OnceLock},
    thread,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    authority::TargetLease,
    domain::{HostDomain, SessionHandle},
    host_types::{HostCompletion, HostEvent, OutputRequest, Terminal},
    process::{DarwinChild, DarwinPlatform},
    process_api::{OwnedProcess, PollInterest, ProcessPlatform, ProcessState, SpawnSpec, Transfer},
    supervisor::RuntimeHost,
    *,
};
use uiblueprint_schema::{SchemaVersion, model::*};

const MIB: usize = 1_048_576;
const DESCRIPTOR: &[u8] = include_bytes!("../../../fixtures/golden/ENV-CAPABILITY-VALID.json");
const SNAPSHOT: &[u8] = include_bytes!("../../../fixtures/golden/ENV-SNAPSHOT-VALID.json");
const REQUEST: &[u8] = include_bytes!("../../../fixtures/golden/ENV-REQUEST-VALID.json");
const QUERY: &[u8] = include_bytes!("../../../fixtures/analysis/query-gap.json");
const DELTA: &[u8] = include_bytes!("../../../fixtures/golden/ENV-DELTA-VALID.json");
const GAP_CASE: &[u8] = include_bytes!("../../../fixtures/analysis/measurement-gap.json");
static SERIAL: Mutex<()> = Mutex::new(());
static GUARDED: OnceLock<()> = OnceLock::new();

fn limits(worker_bytes: usize) -> HostLimits {
    HostLimits {
        workers: 2,
        worker_bytes,
        publication_reserve: MIB,
        bootstrap_bytes: MIB,
        parent_bytes: 32 * MIB,
        input_bytes: 2 * MIB,
        ingress_bytes: 512 * 1024,
        output_bytes: 512 * 1024,
        request_output_bytes: 2 * MIB,
        completion_groups: 2,
        control_bytes: 4096,
        cleanup_ms: 1000,
        retained_domain_bytes: 64 * MIB,
        retained_per_worker: 15 * MIB,
        main_stack_bytes: 8 * MIB,
        watchdog_stack_bytes: MIB,
    }
}
fn deadline() -> Instant {
    Instant::now() + Duration::from_secs(5)
}
fn target() -> TargetLease {
    let Artifact::Session(s) = Document::from_json(DESCRIPTOR, DESCRIPTOR.len())
        .unwrap()
        .artifact
    else {
        panic!("normal descriptor")
    };
    TargetLease::authorized(&s.target, false).unwrap()
}
fn spec() -> SpawnSpec {
    SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap()
}
fn next<'a, P: ProcessPlatform + 'static>(host: &mut RuntimeHost<'a, P>) -> HostEvent<'a> {
    let stop = deadline();
    loop {
        assert!(Instant::now() < stop, "bounded actual host event");
        match host.next_event().expect("host event") {
            HostEvent::Pending => thread::sleep(Duration::from_millis(1)),
            event => return event,
        }
    }
}
fn complete<'a, P: ProcessPlatform + 'static>(host: &mut RuntimeHost<'a, P>) -> HostCompletion<'a> {
    match next(host) {
        HostEvent::Complete(c) => c,
        _ => panic!("expected terminal completion"),
    }
}
fn attach<'a, P: ProcessPlatform + 'static>(
    host: &mut RuntimeHost<'a, P>,
) -> (SessionHandle<'a>, String) {
    let mut input = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    input.bytes_mut().copy_from_slice(DESCRIPTOR);
    let session = host.attach(input, deadline()).unwrap();
    match next(host) {
        HostEvent::Attached {
            session: actual,
            clock,
        } => {
            assert_eq!(actual, session);
            (session, clock.as_str().into())
        }
        _ => panic!("normal fixture must attach before hostile work"),
    }
}
fn output(format: u8, channels: u8) -> OutputRequest {
    OutputRequest {
        channels,
        frame_bytes: 512 * 1024,
        total_bytes: 2 * MIB,
        input_format: format,
        retained_partition: 0,
    }
}
fn submit<'a, P: ProcessPlatform + 'static>(
    host: &mut RuntimeHost<'a, P>,
    session: SessionHandle<'a>,
    bytes: &[u8],
    class: OperationClass,
    request: OutputRequest,
) {
    assert!(bytes.len() <= 2 * MIB);
    let mut input = host.reserve_input(session, bytes.len()).unwrap();
    input.bytes_mut().copy_from_slice(bytes);
    host.submit(session, class, input, request, deadline())
        .unwrap();
}
fn shutdown<P: ProcessPlatform + 'static>(host: &mut RuntimeHost<'_, P>) {
    let stop = deadline();
    while Instant::now() < stop {
        if matches!(host.shutdown().unwrap(), HostEvent::ShutdownComplete) {
            return;
        }
        thread::sleep(Duration::from_millis(1));
    }
    panic!("owned host cleanup must finish")
}

// Observe bytes already returned to the real supervisor. No second FD read,
// altered result, delayed IO/reap, JSON parse or production API is introduced.
struct FatalAudit {
    record: Rc<Cell<Option<Control>>>,
}
struct FatalChild {
    real: DarwinChild,
    record: Rc<Cell<Option<Control>>>,
    bytes: [u8; CONTROL_BYTES],
    used: usize,
}
impl ProcessPlatform for FatalAudit {
    type Child = FatalChild;
    fn validate_parent_reaping() -> Result<(), HostError> {
        DarwinPlatform::validate_parent_reaping()
    }
    fn spawn(&mut self, spec: &SpawnSpec) -> Result<Self::Child, HostError> {
        Ok(FatalChild {
            real: DarwinPlatform.spawn(spec)?,
            record: self.record.clone(),
            bytes: [0; CONTROL_BYTES],
            used: 0,
        })
    }
    fn poll(&mut self, interests: &mut [PollInterest<'_>], wait_ms: u32) -> Result<(), HostError> {
        DarwinPlatform.poll(interests, wait_ms)
    }
}
impl OwnedProcess for FatalChild {
    fn write_input(&mut self, b: &[u8]) -> Result<Transfer, HostError> {
        self.real.write_input(b)
    }
    fn read_output(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
        self.real.read_output(b)
    }
    fn read_fatal(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
        let result = self.real.read_fatal(b)?;
        if let Transfer::Bytes(n) = result {
            assert!(
                n <= b.len() && n <= CONTROL_BYTES - self.used,
                "one fixed actual fatal record"
            );
            self.bytes[self.used..self.used + n].copy_from_slice(&b[..n]);
            self.used += n;
            if self.used == CONTROL_BYTES {
                self.record.set(Some(
                    Control::decode(&self.bytes).expect("valid private control"),
                ));
            }
        }
        Ok(result)
    }
    fn close_input(&mut self) {
        self.real.close_input()
    }
    fn terminate(&mut self) -> Result<(), HostError> {
        self.real.terminate()
    }
    fn try_reap(&mut self) -> Result<ProcessState, HostError> {
        self.real.try_reap()
    }
    fn input_fd(&self) -> Option<BorrowedFd<'_>> {
        self.real.input_fd()
    }
    fn output_fd(&self) -> BorrowedFd<'_> {
        self.real.output_fd()
    }
    fn fatal_fd(&self) -> BorrowedFd<'_> {
        self.real.fatal_fd()
    }
}
fn prove_guard_before_hostile() {
    GUARDED.get_or_init(|| {
        let domain = HostDomain::new::<DarwinPlatform>(limits(2 * MIB)).unwrap();
        let mut host = RuntimeHost::new(&domain, spec(), DarwinPlatform).unwrap();
        let mut input = host
            .reserve_attach_input(target(), DESCRIPTOR.len())
            .unwrap();
        input.bytes_mut().copy_from_slice(DESCRIPTOR);
        host.attach(input, deadline()).unwrap();
        let completion = complete(&mut host);
        let terminal = completion.terminal;
        let committed = completion.committed();
        drop(completion);
        let closed = matches!(next(&mut host), HostEvent::Closed { .. });
        shutdown(&mut host);
        assert_eq!(terminal, Terminal::Failed(HostError::ResourceLimit));
        assert_eq!(committed, 0);
        assert!(closed);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert_eq!(
            domain.usage().retained_reserved_bytes,
            uiblueprint_engine::cache::QuotaLedger::backing_bytes()
        );
        assert!(!domain.usage().abandoned);
    });
}

fn array_family(version: &str, kind: &str, bound: usize, kind_first: bool) -> Vec<u8> {
    let depth = 120;
    let prefix = if kind_first {
        format!("{{\"schema_version\":\"{version}\",\"artifact\":{{\"kind\":\"{kind}\",\"data\":[")
    } else {
        format!("{{\"schema_version\":\"{version}\",\"artifact\":{{\"data\":[")
    };
    let suffix = if kind_first {
        "]}}".to_owned()
    } else {
        format!("],\"kind\":\"{kind}\"}}}}")
    };
    let branch = 2 * depth + 2;
    let count = (bound - prefix.len() - suffix.len() + 1) / (branch + 1);
    let mut bytes = Vec::with_capacity(bound);
    bytes.extend_from_slice(prefix.as_bytes());
    for index in 0..count {
        if index > 0 {
            bytes.push(b',');
        }
        bytes.extend(std::iter::repeat_n(b'[', depth));
        bytes.extend_from_slice(b"\"\"");
        bytes.extend(std::iter::repeat_n(b']', depth));
    }
    bytes.extend_from_slice(suffix.as_bytes());
    assert!(bytes.len() <= bound);
    bytes
}

#[test]
fn actual_guard_precedes_hostile_core_and_analysis_tag_order_families() {
    let _serial = SERIAL.lock().unwrap();
    prove_guard_before_hostile();
    for (version, kind, format) in [("0.1.0", "request", 0), ("0.2.0", "geometry_query", 1)] {
        for kind_first in [true, false] {
            let domain = HostDomain::new::<DarwinPlatform>(limits(64 * MIB)).unwrap();
            let mut host = RuntimeHost::new(&domain, spec(), DarwinPlatform).unwrap();
            let (session, _) = attach(&mut host);
            let bytes = array_family(version, kind, 2 * MIB, kind_first);
            submit(
                &mut host,
                session,
                &bytes,
                OperationClass::Validate,
                output(format, 1),
            );
            let completion = complete(&mut host);
            let terminal = completion.terminal;
            let committed = completion.committed();
            drop(completion);
            if !kind_first {
                assert!(matches!(next(&mut host), HostEvent::Closed { .. }));
            }
            shutdown(&mut host);
            assert_eq!(
                terminal,
                Terminal::Failed(if kind_first {
                    HostError::InvalidInput
                } else {
                    HostError::ResourceLimit
                })
            );
            assert_eq!(committed, 0);
            assert_eq!(domain.usage().reserved_sessions, 0);
            assert_eq!(
                domain.usage().retained_reserved_bytes,
                uiblueprint_engine::cache::QuotaLedger::backing_bytes()
            );
            assert!(!domain.usage().abandoned);
        }
    }
}

#[test]
fn bounded_malformed_string_numeric_and_map_inputs_never_decode_in_parent() {
    let _serial = SERIAL.lock().unwrap();
    prove_guard_before_hostile();
    let domain = HostDomain::new::<DarwinPlatform>(limits(64 * MIB)).unwrap();
    let mut host = RuntimeHost::new(&domain, spec(), DarwinPlatform).unwrap();
    let (session, _) = attach(&mut host);
    let escaped = format!(
        "{{\"schema_version\":\"0.2.0\",\"artifact\":{{\"data\":{{\"id\":\"{}\"}},\"kind\":\"geometry_query\"}}}}",
        "\\u0061".repeat(10_000)
    );
    let numeric = format!(
        "{{\"schema_version\":\"0.2.0\",\"artifact\":{{\"data\":{{\"id\":0.{}1}},\"kind\":\"geometry_query\"}}}}",
        "0".repeat(60_000)
    );
    let maps = format!(
        "{{\"schema_version\":\"0.1.0\",\"artifact\":{{\"data\":{{{}\"final\":[]}},\"kind\":\"request\"}}}}",
        "\"PRIVATE-CANARY\":[],".repeat(3_000)
    );
    for (bytes, format) in [
        (escaped.as_bytes(), 1),
        (numeric.as_bytes(), 1),
        (maps.as_bytes(), 0),
    ] {
        assert!(bytes.len() < 65_536);
        submit(
            &mut host,
            session,
            bytes,
            OperationClass::Validate,
            output(format, 1),
        );
        let result = complete(&mut host);
        assert_eq!(result.terminal, Terminal::Failed(HostError::InvalidInput));
        assert_eq!(result.committed(), 0);
    }
    submit(
        &mut host,
        session,
        QUERY,
        OperationClass::Validate,
        output(1, 1),
    );
    let healthy = complete(&mut host);
    assert_eq!(healthy.terminal, Terminal::Completed);
    assert_eq!(healthy.bytes(0), Some(QUERY));
    drop(healthy);
    shutdown(&mut host);
    assert_eq!(domain.usage().reserved_sessions, 0);
}

#[test]
fn acked_canonical_semantics_survives_later_channel_quota_failure() {
    let _serial = SERIAL.lock().unwrap();
    prove_guard_before_hostile();
    let domain = HostDomain::new::<DarwinPlatform>(limits(16 * MIB)).unwrap();
    let mut host = RuntimeHost::new(&domain, spec(), DarwinPlatform).unwrap();
    let (session, clock) = attach(&mut host);
    // Only small authored normal records are decoded in this parent.
    let Artifact::Request(mut request) = Document::from_json(REQUEST, REQUEST.len())
        .unwrap()
        .artifact
    else {
        panic!("request")
    };
    request.clock_domain = Id(clock);
    request.operation = Operation::Observe {
        channels: vec![Channel::ExternalSemantics, Channel::RenderedCapture],
    };
    request.limits.deadline_ms = 3000;
    request.limits.max_output_bytes = (2 * MIB) as u64;
    let Artifact::Snapshot(snapshot) = Document::from_json(SNAPSHOT, SNAPSHOT.len())
        .unwrap()
        .artifact
    else {
        panic!("snapshot")
    };
    let response = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::ChannelResponse(Box::new(ChannelResponse {
            request_id: request.request_id.clone(),
            session_id: snapshot.context.session_id.clone(),
            dispatch_sequence: 1,
            target: snapshot.context.target.clone(),
            channel: Channel::ExternalSemantics,
            result: ChannelResult::Observed(snapshot),
        })),
    };
    let first = serde_json::to_vec(&response).unwrap();
    let request = serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Request(request),
    })
    .unwrap();
    let hostile = array_family("0.1.0", "channel_response", 512 * 1024, false);
    let size = 40 + request.len() + first.len() + hostile.len();
    let mut input = host.reserve_input(session, size).unwrap();
    assert_eq!(
        worker_tape::encode(&[&request, &first, &hostile], input.bytes_mut()).unwrap(),
        size
    );
    host.submit(
        session,
        OperationClass::Observe,
        input,
        output(0, 3),
        deadline(),
    )
    .unwrap();
    let result = complete(&mut host);
    assert_eq!(result.terminal, Terminal::Failed(HostError::ResourceLimit));
    assert_eq!(result.committed(), 1);
    assert_eq!(result.missing(), 2);
    assert_eq!(result.bytes(0), Some(first.as_slice()));
    assert!(result.bytes(1).is_none());
    assert!(!result.effect_unknown());
    assert!(matches!(next(&mut host), HostEvent::Closed { .. }));
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert_eq!(
        result.bytes(0),
        Some(first.as_slice()),
        "lease survives actual worker reap"
    );
    drop(result);
    shutdown(&mut host);
    assert_eq!(domain.usage().completion_groups, 0);
}

#[test]
fn canonical_retain_replay_and_encoding_refusal_use_actual_worker_paths() {
    let _serial = SERIAL.lock().unwrap();
    prove_guard_before_hostile();
    let domain = HostDomain::new::<DarwinPlatform>(limits(64 * MIB)).unwrap();
    let mut host = RuntimeHost::new(&domain, spec(), DarwinPlatform).unwrap();
    let (session, _) = attach(&mut host);
    // These are small authored normal records, not hostile input decoding.
    let delta_doc = Document::from_json(DELTA, DELTA.len()).unwrap();
    let Artifact::Delta(case) = &delta_doc.artifact else {
        panic!("delta fixture")
    };
    let base = serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(case.base.clone())),
    })
    .unwrap();
    let mut request = output(0, 1);
    request.retained_partition = 1;
    submit(&mut host, session, &base, OperationClass::Retain, request);
    let retained = complete(&mut host);
    assert_eq!(retained.terminal, Terminal::Completed);
    assert_eq!(retained.bytes(0), Some(base.as_slice()));
    let id = b"\"allocator-proof-replay\"";
    let size = 40 + DELTA.len() + id.len();
    let mut input = host.reserve_input(session, size).unwrap();
    worker_tape::encode(&[DELTA, id], input.bytes_mut()).unwrap();
    host.submit(session, OperationClass::Replay, input, request, deadline())
        .unwrap();
    let replayed = complete(&mut host);
    assert_eq!(replayed.terminal, Terminal::Completed);
    let mut expected = case
        .source_snapshot
        .clone()
        .expect("independent full source");
    expected.id = Id("allocator-proof-replay".into());
    let expected = serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(expected)),
    })
    .unwrap();
    assert_eq!(replayed.bytes(0), Some(expected.as_slice()));
    drop(replayed);
    let gap = uiblueprint_schema::analysis::AnalysisDocument::from_json(GAP_CASE, GAP_CASE.len())
        .unwrap();
    let uiblueprint_schema::analysis::AnalysisArtifact::Measurement(gap) = gap.artifact else {
        panic!("authored measurement")
    };
    let source = serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(gap.snapshot)),
    })
    .unwrap();
    let query = serde_json::to_vec(&uiblueprint_schema::analysis::AnalysisDocument {
        schema_version: uiblueprint_schema::analysis::AnalysisVersion::CURRENT,
        artifact: uiblueprint_schema::analysis::AnalysisArtifact::GeometryQuery(Box::new(
            gap.query,
        )),
    })
    .unwrap();
    let evaluation = serde_json::to_vec(&uiblueprint_schema::analysis::AnalysisDocument {
        schema_version: uiblueprint_schema::analysis::AnalysisVersion::CURRENT,
        artifact: uiblueprint_schema::analysis::AnalysisArtifact::EvaluationInput(Box::new(
            gap.evaluation,
        )),
    })
    .unwrap();
    let size = 40 + source.len() + query.len() + evaluation.len();
    let mut input = host.reserve_input(session, size).unwrap();
    worker_tape::encode(&[&source, &query, &evaluation], input.bytes_mut()).unwrap();
    let mut tiny = output(1, 1);
    tiny.frame_bytes = 16;
    tiny.total_bytes = 16;
    host.submit(session, OperationClass::Measure, input, tiny, deadline())
        .unwrap();
    let refused = complete(&mut host);
    assert_eq!(refused.terminal, Terminal::Failed(HostError::ResourceLimit));
    assert_eq!(refused.committed(), 0);
    assert_eq!(
        retained.bytes(0),
        Some(base.as_slice()),
        "earlier ACKed lease survives encoding refusal"
    );
    drop(refused);
    drop(retained);
    shutdown(&mut host);
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert_eq!(domain.usage().completion_groups, 0);
}

#[test]
fn phase_fixed_output_writer_refuses_valid_padded_query_without_partial_frame() {
    let _serial = SERIAL.lock().unwrap();
    let config = limits(64 * MIB);
    let domain = HostDomain::new::<DarwinPlatform>(config).unwrap();
    let mut host = RuntimeHost::new(&domain, spec(), DarwinPlatform).unwrap();
    let (session, _) = attach(&mut host);
    submit(
        &mut host,
        session,
        QUERY,
        OperationClass::Validate,
        output(1, 1),
    );
    let held = complete(&mut host);
    assert_eq!(held.terminal, Terminal::Completed);
    let reserved = domain.usage().retained_reserved_bytes;
    // Predeclared exact boundary: output slice512KiB, input512KiB+1. Whitespace
    // does not alter the valid canonical query. No hostile DTO is parsed here.
    let mut padded = QUERY.to_vec();
    padded.resize(config.output_bytes + 1, b' ');
    assert!(padded.len() <= config.input_bytes);
    submit(
        &mut host,
        session,
        &padded,
        OperationClass::Validate,
        output(1, 1),
    );
    let refused = complete(&mut host);
    assert_eq!(refused.terminal, Terminal::Failed(HostError::ResourceLimit));
    assert_eq!(refused.committed(), 0);
    assert!(refused.bytes(0).is_none());
    assert_eq!(held.bytes(0), Some(QUERY));
    assert_eq!(domain.usage().retained_reserved_bytes, reserved);
    assert_eq!(domain.usage().reserved_sessions, 1);
    drop(refused);
    submit(
        &mut host,
        session,
        QUERY,
        OperationClass::Validate,
        output(1, 1),
    );
    let healthy = complete(&mut host);
    assert_eq!(healthy.terminal, Terminal::Completed);
    assert_eq!(healthy.bytes(0), Some(QUERY));
    drop(healthy);
    drop(held);
    shutdown(&mut host);
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert_eq!(domain.usage().completion_groups, 0);
}

#[test]
fn phase_retained_quota_refusal_keeps_base_replayable_and_caller_lease_intact() {
    let _serial = SERIAL.lock().unwrap();
    // Fixed before execution:32KiB child retained domain; ordinary working heap
    // remains63MiB. A64KiB property cannot fit even after evicting every old entry.
    let mut config = limits(64 * MIB);
    config.retained_per_worker = 32 * 1024;
    let domain = HostDomain::new::<DarwinPlatform>(config).unwrap();
    let mut host = RuntimeHost::new(&domain, spec(), DarwinPlatform).unwrap();
    let (session, _) = attach(&mut host);
    let delta = Document::from_json(DELTA, DELTA.len()).unwrap();
    let Artifact::Delta(case) = delta.artifact else {
        panic!("authored delta")
    };
    let base = serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(case.base.clone())),
    })
    .unwrap();
    let mut request = output(0, 1);
    request.retained_partition = 1;
    submit(&mut host, session, &base, OperationClass::Retain, request);
    let held = complete(&mut host);
    assert_eq!(held.terminal, Terminal::Completed);
    assert_eq!(held.bytes(0), Some(base.as_slice()));
    let reserved = domain.usage().retained_reserved_bytes;
    let mut large = case.base.clone();
    large.id = Id("authored-retained-over-quota".into());
    large.revision += 1;
    large.source_state = Some(Id("authored-retained-large-state".into()));
    let Property::Requested { state, .. } = large.nodes[0]
        .properties
        .iter_mut()
        .find(|p| p.field() == Field::Name)
        .unwrap()
    else {
        panic!("authored requested name")
    };
    *state = Availability::Known {
        value: Value::Text("x".repeat(64 * 1024)),
    };
    assert!(uiblueprint_schema::owned_size::owned(&large).unwrap().2 > config.retained_per_worker);
    let large = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(large)),
    };
    large
        .validate()
        .expect("valid authored larger candidate; not a schema refusal");
    let large = serde_json::to_vec(&large).unwrap();
    assert!(large.len() < request.frame_bytes);
    submit(&mut host, session, &large, OperationClass::Retain, request);
    let refused = complete(&mut host);
    assert_eq!(refused.terminal, Terminal::Failed(HostError::ResourceLimit));
    assert_eq!(refused.committed(), 0);
    assert_eq!(held.bytes(0), Some(base.as_slice()));
    assert_eq!(domain.usage().retained_reserved_bytes, reserved);
    assert_eq!(domain.usage().reserved_sessions, 1);
    drop(refused);
    // Replay is the actual existing lookup consumer: success proves the old base
    // was not evicted by a candidate that could never fit, not just held wire bytes.
    let id = b"\"retained-refusal-replay\"";
    let mut input = host
        .reserve_input(session, 40 + DELTA.len() + id.len())
        .unwrap();
    worker_tape::encode(&[DELTA, id], input.bytes_mut()).unwrap();
    host.submit(session, OperationClass::Replay, input, request, deadline())
        .unwrap();
    let replayed = complete(&mut host);
    assert_eq!(replayed.terminal, Terminal::Completed);
    let mut expected = case.source_snapshot.expect("independent full source");
    expected.id = Id("retained-refusal-replay".into());
    let expected = serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(expected)),
    })
    .unwrap();
    assert_eq!(replayed.bytes(0), Some(expected.as_slice()));
    assert_eq!(held.bytes(0), Some(base.as_slice()));
    drop(replayed);
    drop(held);
    shutdown(&mut host);
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert_eq!(domain.usage().completion_groups, 0);
}

#[test]
fn replay_clone_quota_has_private_phase_and_keeps_acked_bytes() {
    let _serial = SERIAL.lock().unwrap();
    const TEXT: usize = 480 * 1024;
    const ORDINARY: usize = 4 * MIB - 128 * 1024;
    let config = limits(ORDINARY + MIB);
    assert_eq!(config.ordinary_bytes().unwrap(), ORDINARY);
    // Fixed buffers + retained text + decoded base text + cloned result text
    // exceed ordinary quota before counting any other live metadata. This is
    // one predeclared case, not a cap search; actual fatal phase decides proof.
    assert!(config.input_bytes + config.output_bytes + 3 * TEXT > ORDINARY);
    let record = Rc::new(Cell::new(None));
    let domain = HostDomain::new::<FatalAudit>(config).unwrap();
    let mut host = RuntimeHost::new(
        &domain,
        spec(),
        FatalAudit {
            record: record.clone(),
        },
    )
    .unwrap();
    let (session, _) = attach(&mut host);
    let Artifact::Snapshot(mut base) = Document::from_json(SNAPSHOT, SNAPSHOT.len())
        .unwrap()
        .artifact
    else {
        panic!("normal source")
    };
    base.id = Id("replay-pressure-base".into());
    base.source_state = Some(Id("authored-replay-pressure-base".into()));
    let Property::Requested { state, .. } = base.nodes[0]
        .properties
        .iter_mut()
        .find(|p| p.field() == Field::Name)
        .unwrap()
    else {
        panic!("name")
    };
    *state = Availability::Known {
        value: Value::Text("x".repeat(TEXT)),
    };
    let base_doc = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(base.clone()),
    };
    base_doc
        .validate()
        .expect("valid authored unescaped text source");
    let bytes = serde_json::to_vec(&base_doc).unwrap();
    assert!(
        bytes.len() < config.output_bytes,
        "normal Retain must fit publication cap"
    );
    let mut request = output(0, 1);
    request.retained_partition = 1;
    submit(&mut host, session, &bytes, OperationClass::Retain, request);
    let held = complete(&mut host);
    if held.terminal != Terminal::Completed {
        let terminal = held.terminal;
        drop(held);
        shutdown(&mut host);
        panic!(
            "predeclared Retain prerequisite failed: {terminal:?}, fatal={:?}; do not tune cap",
            record.get()
        );
    }
    assert_eq!(held.bytes(0), Some(bytes.as_slice()));
    assert!(record.get().is_none());
    let delta = Delta {
        base_revision: base.revision,
        revision: base.revision + 1,
        source_state: Some(Id("authored-replay-pressure-next".into())),
        context: base.context.clone(),
        observations: vec![],
        upsert: vec![],
        removed: vec![],
        relations: base.relations.clone(),
        focus: base.focus.clone(),
        coverage: base.coverage.clone(),
    };
    let delta_doc = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Delta(Box::new(DeltaCase {
            base: *base,
            update: delta,
            source_snapshot: None,
        })),
    };
    delta_doc
        .validate()
        .expect("empty compatible delta, not malformed data");
    let delta = serde_json::to_vec(&delta_doc).unwrap();
    let id = b"\"replay-pressure-result\"";
    let mut input = host
        .reserve_input(session, 40 + delta.len() + id.len())
        .unwrap();
    worker_tape::encode(&[&delta, id], input.bytes_mut()).unwrap();
    let operation = host
        .submit(session, OperationClass::Replay, input, request, deadline())
        .unwrap();
    let result = complete(&mut host);
    let terminal = result.terminal;
    let committed = result.committed();
    drop(result);
    shutdown(&mut host); // Actual provider reap before any final assertion/receipt.
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert_eq!(
        domain.usage().retained_reserved_bytes,
        uiblueprint_engine::cache::QuotaLedger::backing_bytes()
    );
    assert_eq!(
        held.bytes(0),
        Some(bytes.as_slice()),
        "ACKed bytes survive quota death/reap"
    );
    let observed = record.get();
    drop(held);
    assert_eq!(domain.usage().completion_groups, 0);
    assert_eq!(
        terminal,
        Terminal::Failed(HostError::ResourceLimit),
        "fatal={observed:?}"
    );
    assert_eq!(committed, 0);
    let fatal = observed.expect("actual private fatal bytes, not inferred phase");
    assert_eq!(fatal.kind, ControlKind::Fatal);
    assert_eq!(fatal.class, OperationClass::Replay);
    assert_eq!(fatal.correlation.operation, operation.sequence);
    assert_eq!(fatal.value, 1);
    assert_eq!(
        fatal.flags, 3,
        "Replay expected; earlier Decode is an attribution gap, not a pass"
    );
}
