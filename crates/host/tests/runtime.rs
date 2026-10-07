#![cfg(target_os = "macos")]
use std::{
    path::Path,
    thread,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    authority::TargetLease,
    domain::HostDomain,
    host_types::{HostCompletion, HostEvent, OutputRequest, Terminal},
    process::DarwinPlatform,
    process_api::SpawnSpec,
    supervisor::RuntimeHost,
    *,
};
use uiblueprint_schema::model::{Artifact, Document};
static RUNTIME_TEST: std::sync::Mutex<()> = std::sync::Mutex::new(());
const MIB: usize = 1_048_576;
const DESCRIPTOR: &[u8] = include_bytes!("../../../fixtures/golden/ENV-CAPABILITY-VALID.json");
const QUERY: &[u8] = include_bytes!("../../../fixtures/analysis/query-gap.json");
fn limits() -> HostLimits {
    HostLimits {
        workers: 2,
        worker_bytes: 64 * MIB,
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
fn target() -> TargetLease {
    let Artifact::Session(s) = Document::from_json(DESCRIPTOR, DESCRIPTOR.len())
        .unwrap()
        .artifact
    else {
        panic!("session")
    };
    TargetLease::authorized(&s.target, false).unwrap()
}
fn deadline() -> Instant {
    Instant::now() + Duration::from_secs(5)
}
fn next<'a, P: uiblueprint_host::process_api::ProcessPlatform + 'static>(
    host: &mut RuntimeHost<'a, P>,
) -> HostEvent<'a> {
    let stop = deadline();
    loop {
        assert!(Instant::now() < stop, "bounded host event");
        match host.next_event().expect("host event") {
            HostEvent::Pending => thread::sleep(Duration::from_millis(1)),
            event => return event,
        }
    }
}
fn complete<'a, P: uiblueprint_host::process_api::ProcessPlatform + 'static>(
    host: &mut RuntimeHost<'a, P>,
) -> HostCompletion<'a> {
    match next(host) {
        HostEvent::Complete(c) => c,
        _ => panic!("completion"),
    }
}
#[test]
fn real_worker_reuses_session_and_keeps_caller_completion_during_other_session() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let domain = HostDomain::new::<DarwinPlatform>(limits()).expect("domain");
    assert!(matches!(
        HostDomain::new::<DarwinPlatform>(limits()),
        Err(HostError::Busy)
    ));
    let spec = SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap();
    let mut host = RuntimeHost::new(&domain, spec, DarwinPlatform).unwrap();
    let baseline = domain.usage();
    let mut input = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    input.bytes_mut().copy_from_slice(DESCRIPTOR);
    let first = host.attach(input, deadline()).unwrap();
    match next(&mut host) {
        HostEvent::Attached { session, .. } => assert_eq!(session, first),
        HostEvent::Complete(c) => panic!("attach failed: {:?}", c.terminal),
        _ => panic!("attached"),
    };
    let mut input = host.reserve_input(first, QUERY.len()).unwrap();
    input.bytes_mut().copy_from_slice(QUERY);
    let request = OutputRequest {
        channels: 1,
        frame_bytes: 65536,
        total_bytes: 65536,
        input_format: 1,
        retained_partition: 0,
    };
    host.submit(first, OperationClass::Validate, input, request, deadline())
        .unwrap();
    let held = complete(&mut host);
    assert_eq!(held.terminal, Terminal::Completed);
    assert_eq!(held.bytes(0), Some(QUERY));
    assert_eq!(domain.usage().completion_groups, 1);
    let mut input = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    input.bytes_mut().copy_from_slice(DESCRIPTOR);
    let second = host.attach(input, deadline()).unwrap();
    assert!(matches!(next(&mut host), HostEvent::Attached { .. }));
    let mut input = host.reserve_input(second, QUERY.len()).unwrap();
    input.bytes_mut().copy_from_slice(QUERY);
    host.submit(second, OperationClass::Validate, input, request, deadline())
        .unwrap();
    let second_result = complete(&mut host);
    assert_eq!(second_result.terminal, Terminal::Completed);
    assert_eq!(held.bytes(0), Some(QUERY));
    let full_input = host.reserve_input(first, 1).unwrap();
    assert!(matches!(
        host.submit(
            first,
            OperationClass::Validate,
            full_input,
            request,
            deadline()
        ),
        Err(HostError::ResourceLimit)
    ));
    assert_eq!(held.bytes(0), Some(QUERY));
    assert_eq!(second_result.bytes(0), Some(QUERY));
    drop(second_result);
    let mut input = host.reserve_input(first, 1).unwrap();
    input.bytes_mut()[0] = b'!';
    host.submit(first, OperationClass::Validate, input, request, deadline())
        .unwrap();
    assert_eq!(
        complete(&mut host).terminal,
        Terminal::Failed(HostError::InvalidInput)
    );
    assert_eq!(domain.usage().reserved_sessions, 2);
    drop(held);
    let stop = deadline();
    loop {
        assert!(Instant::now() < stop);
        if matches!(host.shutdown().unwrap(), HostEvent::ShutdownComplete) {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert_eq!(
        domain.usage().retained_reserved_bytes,
        baseline.retained_reserved_bytes
    );
    assert_eq!(domain.usage().completion_groups, 0);
}

#[path = "support/reaping_host.rs"]
mod reaping;

#[test]
fn real_quota_fatal_during_fixed_buffer_setup_keeps_parent_alive_and_reaps() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let mut config = limits();
    config.worker_bytes = 2 * MIB;
    // The configured 2MiB input allocation cannot fit the 1MiB ordinary cap.
    // This is a fixed setup allocation, not hostile JSON or an OOM stress probe.
    let domain = HostDomain::new::<DarwinPlatform>(config).unwrap();
    let spec = SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap();
    let mut host = RuntimeHost::new(&domain, spec, DarwinPlatform).unwrap();
    let mut input = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    input.bytes_mut().copy_from_slice(DESCRIPTOR);
    host.attach(input, deadline()).unwrap();
    let result = complete(&mut host);
    assert_eq!(result.terminal, Terminal::Failed(HostError::ResourceLimit));
    assert_eq!(result.committed(), 0);
    assert_eq!(domain.usage().reserved_sessions, 1);
    assert!(matches!(next(&mut host), HostEvent::Closed { .. }));
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert_eq!(
        domain.usage().retained_reserved_bytes,
        uiblueprint_engine::cache::QuotaLedger::backing_bytes()
    );
    assert!(!domain.usage().abandoned);
}

#[test]
fn shutdown_rejects_a_previously_reserved_attach_without_spawning() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let domain = HostDomain::new::<DarwinPlatform>(limits()).unwrap();
    let spec = SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap();
    let mut host = RuntimeHost::new(&domain, spec, DarwinPlatform).unwrap();
    let mut input = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    input.bytes_mut().copy_from_slice(DESCRIPTOR);
    assert!(matches!(
        host.shutdown().unwrap(),
        HostEvent::ShutdownComplete
    ));
    assert!(matches!(
        host.attach(input, deadline()),
        Err(HostError::InvalidState)
    ));
    assert_eq!(domain.usage().reserved_sessions, 0);
}

#[path = "support/lifecycle_host.rs"]
mod lifecycle;
