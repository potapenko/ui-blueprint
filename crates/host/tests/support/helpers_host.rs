//! Actual direct children stand in for native helper resources; no SDK/capture UI.
use super::*;
use uiblueprint_host::{helpers::HelperKind, process_api::Transfer};
fn peer() -> SpawnSpec {
    let path = Path::new(env!("CARGO_BIN_EXE_session-worker"))
        .parent()
        .unwrap()
        .join("examples/process_peer");
    assert!(
        path.is_file(),
        "build existing process_peer before helper tests"
    );
    SpawnSpec::new(&path).unwrap()
}
fn attached<'a>(
    host: &mut RuntimeHost<'a, DarwinPlatform>,
) -> uiblueprint_host::domain::SessionHandle<'a> {
    let mut input = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    input.bytes_mut().copy_from_slice(DESCRIPTOR);
    let s = host.attach(input, deadline()).unwrap();
    assert!(matches!(next(host), HostEvent::Attached { .. }));
    s
}
fn shutdown(host: &mut RuntimeHost<'_, DarwinPlatform>) {
    let stop = deadline();
    loop {
        assert!(Instant::now() < stop);
        if matches!(host.shutdown().unwrap(), HostEvent::ShutdownComplete) {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
}
#[test]
fn capture_claim_and_ingress_survive_until_real_helper_reap_without_blocking_ax() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let domain = HostDomain::new::<DarwinPlatform>(limits()).unwrap();
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
        DarwinPlatform,
    )
    .unwrap();
    let a = attached(&mut host);
    let b = attached(&mut host);
    let owned = domain.usage().parent_owned_bytes;
    let cap = host
        .spawn_helper(a, HelperKind::Capture, peer(), deadline())
        .unwrap();
    let ax = host
        .spawn_helper(b, HelperKind::ExternalSemantics, peer(), deadline())
        .unwrap();
    assert!(matches!(
        host.spawn_helper(b, HelperKind::Capture, peer(), deadline()),
        Err(HostError::Busy)
    ));
    let mut size = 0;
    let stop = deadline();
    while size < 64 {
        assert!(Instant::now() < stop);
        match host.read_helper(cap).unwrap() {
            Transfer::Bytes(n) => size += n,
            Transfer::WouldBlock => thread::sleep(Duration::from_millis(1)),
            Transfer::Closed => panic!("complete helper info"),
        }
    }
    let raw = host.take_helper_bytes(cap).unwrap();
    assert_eq!(raw.bytes().len(), 64);
    assert_eq!(raw.bytes()[0], 0xa1);
    assert!(
        matches!(
            host.spawn_helper(b, HelperKind::Capture, peer(), deadline()),
            Err(HostError::Busy)
        ),
        "not released by terminate request alone"
    );
    loop {
        assert!(Instant::now() < stop);
        match host.next_event().unwrap() {
            HostEvent::HelperClosed { helper } => {
                assert_eq!(helper, cap);
                break;
            }
            HostEvent::Pending => thread::sleep(Duration::from_millis(1)),
            _ => panic!("helper cleanup"),
        }
    }
    let cap_b = host
        .spawn_helper(b, HelperKind::Capture, peer(), deadline())
        .unwrap();
    assert!(matches!(
        host.spawn_helper(b, HelperKind::ExternalSemantics, peer(), deadline()),
        Err(HostError::ResourceLimit)
    ));
    assert_eq!(
        domain.usage().parent_owned_bytes,
        owned,
        "all helper control/backing precharged"
    );
    assert_eq!(
        raw.bytes()[0],
        0xa1,
        "caller bytes outlive helper and allow independent slot"
    );
    host.close_helper(ax).unwrap();
    host.close_helper(cap_b).unwrap();
    shutdown(&mut host);
    assert_eq!(raw.bytes()[0], 0xa1);
    drop(raw);
    assert_eq!(domain.usage().reserved_sessions, 0);
}
#[test]
fn helper_ingress_cap_refuses_before_growth_and_wrong_session_write_is_rejected() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let mut config = limits();
    config.ingress_bytes = 8;
    let domain = HostDomain::new::<DarwinPlatform>(config).unwrap();
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
        DarwinPlatform,
    )
    .unwrap();
    let a = attached(&mut host);
    let b = attached(&mut host);
    let helper = host
        .spawn_helper(a, HelperKind::Capture, peer(), deadline())
        .unwrap();
    let mut input = host.reserve_input(b, 1).unwrap();
    input.bytes_mut()[0] = b'I';
    assert_eq!(
        host.write_helper(helper, &input, 0),
        Err(HostError::StaleOperation)
    );
    drop(input);
    let stop = deadline();
    loop {
        assert!(Instant::now() < stop);
        match host.read_helper(helper) {
            Err(HostError::ResourceLimit) => break,
            Ok(Transfer::Bytes(_) | Transfer::WouldBlock) => {
                thread::sleep(Duration::from_millis(1))
            }
            _ => panic!("bounded ingress"),
        }
    }
    assert!(
        host.take_helper_bytes(helper).is_err(),
        "truncated ingress is not completed output"
    );
    shutdown(&mut host);
    assert_eq!(domain.usage().reserved_sessions, 0);
}

mod delayed {
    use super::*;
    use std::{
        os::fd::BorrowedFd,
        sync::atomic::{AtomicBool, Ordering},
    };
    use uiblueprint_host::{
        process::DarwinChild,
        process_api::{OwnedProcess, PollInterest, ProcessPlatform, ProcessState},
    };
    static WORKER_REAPED: AtomicBool = AtomicBool::new(false);
    struct Platform {
        count: usize,
    }
    struct Child {
        real: DarwinChild,
        helper: bool,
        terminated: Option<Instant>,
    }
    impl ProcessPlatform for Platform {
        type Child = Child;
        fn validate_parent_reaping() -> Result<(), HostError> {
            DarwinPlatform::validate_parent_reaping()
        }
        fn spawn(&mut self, spec: &SpawnSpec) -> Result<Child, HostError> {
            let real = DarwinPlatform.spawn(spec)?;
            let helper = self.count > 0;
            self.count += 1;
            Ok(Child {
                real,
                helper,
                terminated: None,
            })
        }
        fn poll(&mut self, i: &mut [PollInterest<'_>], ms: u32) -> Result<(), HostError> {
            DarwinPlatform.poll(i, ms)
        }
    }
    impl OwnedProcess for Child {
        fn write_input(&mut self, b: &[u8]) -> Result<Transfer, HostError> {
            self.real.write_input(b)
        }
        fn read_output(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
            self.real.read_output(b)
        }
        fn read_fatal(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
            self.real.read_fatal(b)
        }
        fn close_input(&mut self) {
            self.real.close_input();
        }
        fn terminate(&mut self) -> Result<(), HostError> {
            self.terminated.get_or_insert(Instant::now());
            self.real.terminate()
        }
        fn try_reap(&mut self) -> Result<ProcessState, HostError> {
            if self.helper
                && self
                    .terminated
                    .is_some_and(|t| t.elapsed() < Duration::from_millis(100))
            {
                return Ok(ProcessState::Running);
            }
            let state = self.real.try_reap()?;
            if !self.helper && state != ProcessState::Running {
                WORKER_REAPED.store(true, Ordering::SeqCst);
            }
            Ok(state)
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
    #[test]
    fn root_grant_survives_worker_reap_until_registered_helper_reap_is_confirmed() {
        let _serial = RUNTIME_TEST.lock().unwrap();
        WORKER_REAPED.store(false, Ordering::SeqCst);
        let mut config = limits();
        config.cleanup_ms = 1;
        let domain = HostDomain::new::<Platform>(config).unwrap();
        let mut host = RuntimeHost::new(
            &domain,
            SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
            Platform { count: 0 },
        )
        .unwrap();
        let mut input = host
            .reserve_attach_input(target(), DESCRIPTOR.len())
            .unwrap();
        input.bytes_mut().copy_from_slice(DESCRIPTOR);
        let session = host.attach(input, deadline()).unwrap();
        assert!(matches!(next(&mut host), HostEvent::Attached { .. }));
        let helper = host
            .spawn_helper(session, HelperKind::Capture, peer(), deadline())
            .unwrap();
        let reserved = domain.usage().retained_reserved_bytes;
        host.detach(session).unwrap();
        let stop = deadline();
        let mut helper_pending = false;
        while !WORKER_REAPED.load(Ordering::SeqCst) {
            assert!(Instant::now() < stop);
            match host.next_event().unwrap() {
                HostEvent::HelperCleanupPending { helper: h } => {
                    assert_eq!(h, helper);
                    helper_pending = true;
                }
                HostEvent::CleanupPending { .. } | HostEvent::Pending => {
                    thread::sleep(Duration::from_millis(1))
                }
                _ => panic!("worker must not close session before helper confirmation"),
            }
        }
        assert_eq!(domain.usage().retained_reserved_bytes, reserved);
        assert_eq!(domain.usage().reserved_sessions, 1);
        let mut helper_closed = false;
        loop {
            assert!(Instant::now() < stop);
            match host.next_event().unwrap() {
                HostEvent::HelperClosed { helper: h } => {
                    assert_eq!(h, helper);
                    helper_closed = true;
                }
                HostEvent::Closed { session: s } => {
                    assert_eq!(s, session);
                    assert!(helper_closed);
                    break;
                }
                HostEvent::HelperCleanupPending { .. } => helper_pending = true,
                HostEvent::CleanupPending { .. } | HostEvent::Pending => {
                    thread::sleep(Duration::from_millis(1))
                }
                _ => panic!("bounded helper cleanup"),
            }
        }
        assert!(helper_pending);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert_eq!(
            domain.usage().retained_reserved_bytes,
            uiblueprint_engine::cache::QuotaLedger::backing_bytes()
        );
        assert!(matches!(
            host.shutdown().unwrap(),
            HostEvent::ShutdownComplete
        ));
    }
}

#[test]
fn helper_deadline_cleans_only_its_owned_child_and_worker_remains_usable() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let domain = HostDomain::new::<DarwinPlatform>(limits()).unwrap();
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
        DarwinPlatform,
    )
    .unwrap();
    let session = attached(&mut host);
    let helper = host
        .spawn_helper(
            session,
            HelperKind::Capture,
            peer(),
            Instant::now() + Duration::from_millis(10),
        )
        .unwrap();
    assert!(matches!(next(&mut host),HostEvent::HelperClosed{helper:h} if h==helper));
    assert_eq!(domain.usage().reserved_sessions, 1);
    assert_eq!(host.read_helper(helper), Err(HostError::StaleOperation));
    let mut input = host.reserve_input(session, QUERY.len()).unwrap();
    input.bytes_mut().copy_from_slice(QUERY);
    host.submit(
        session,
        OperationClass::Validate,
        input,
        OutputRequest {
            channels: 1,
            frame_bytes: 65536,
            total_bytes: 65536,
            input_format: 1,
            retained_partition: 0,
        },
        deadline(),
    )
    .unwrap();
    let result = complete(&mut host);
    assert_eq!(result.terminal, Terminal::Completed);
    assert_eq!(result.bytes(0), Some(QUERY));
    drop(result);
    shutdown(&mut host);
}
