//! A real owned worker is held before configuration; no fake canonical success.
//! The wrapper withholds input and reap confirmation to exercise supervisor
//! independence/quarantine deterministically without changing signal policy.
use super::*;
use std::os::fd::BorrowedFd;
use uiblueprint_host::{
    host_types::OperationHandle,
    process::DarwinChild,
    process_api::{OwnedProcess, PollInterest, ProcessPlatform, ProcessState, Transfer},
};
struct Platform {
    first: bool,
}
struct Child {
    real: DarwinChild,
    stalled: bool,
    terminated: Option<Instant>,
}
impl ProcessPlatform for Platform {
    type Child = Child;
    fn validate_parent_reaping() -> Result<(), HostError> {
        DarwinPlatform::validate_parent_reaping()
    }
    fn spawn(&mut self, spec: &SpawnSpec) -> Result<Child, HostError> {
        let real = DarwinPlatform.spawn(spec)?;
        let stalled = self.first;
        self.first = false;
        Ok(Child {
            real,
            stalled,
            terminated: None,
        })
    }
    fn poll(&mut self, i: &mut [PollInterest<'_>], ms: u32) -> Result<(), HostError> {
        DarwinPlatform.poll(i, ms)
    }
}
impl OwnedProcess for Child {
    fn write_input(&mut self, b: &[u8]) -> Result<Transfer, HostError> {
        if self.stalled {
            Ok(Transfer::WouldBlock)
        } else {
            self.real.write_input(b)
        }
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
        if self.stalled
            && self
                .terminated
                .is_none_or(|t| t.elapsed() < Duration::from_millis(100))
        {
            Ok(ProcessState::Running)
        } else {
            self.real.try_reap()
        }
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
fn stalled_attach_does_not_block_b_and_cancel_quarantines_until_confirmed_reap() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let mut config = limits();
    config.cleanup_ms = 1;
    let domain = HostDomain::new::<Platform>(config).unwrap();
    let spec = SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap();
    let mut host = RuntimeHost::new(&domain, spec, Platform { first: true }).unwrap();
    let mut a = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    a.bytes_mut().copy_from_slice(DESCRIPTOR);
    let a = host.attach(a, deadline()).unwrap();
    let mut b = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    b.bytes_mut().copy_from_slice(DESCRIPTOR);
    let b = host.attach(b, deadline()).unwrap();
    assert!(matches!(next(&mut host),HostEvent::Attached{session,..} if session==b));
    let reserved = domain.usage().retained_reserved_bytes;
    let cancelled = host
        .cancel(OperationHandle {
            session: a,
            sequence: 0,
        })
        .unwrap();
    assert_eq!(cancelled.terminal, Terminal::Cancelled);
    assert_eq!(cancelled.committed(), 0);
    assert!(matches!(next(&mut host),HostEvent::CleanupPending{session} if session==a));
    assert_eq!(domain.usage().retained_reserved_bytes, reserved);
    assert!(matches!(
        host.reserve_attach_input(target(), 1),
        Err(HostError::ResourceLimit)
    ));
    let mut input = host.reserve_input(b, QUERY.len()).unwrap();
    input.bytes_mut().copy_from_slice(QUERY);
    host.submit(
        b,
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
    let stop = deadline();
    let mut closed = false;
    let mut completed = false;
    while !(closed && completed) {
        assert!(Instant::now() < stop);
        match host.next_event().unwrap() {
            HostEvent::Closed { session } => {
                assert_eq!(session, a);
                closed = true;
            }
            HostEvent::Complete(c) => {
                assert_eq!(c.operation.session, b);
                assert_eq!(c.terminal, Terminal::Completed);
                assert_eq!(c.bytes(0), Some(QUERY));
                completed = true;
            }
            HostEvent::Pending => thread::sleep(Duration::from_millis(1)),
            _ => panic!("bounded cleanup/progress event"),
        }
    }
    assert_eq!(domain.usage().reserved_sessions, 1);
    assert!(!domain.usage().abandoned);
    assert!(matches!(
        host.reserve_input(a, 1),
        Err(HostError::StaleOperation)
    ));
    // A's root grant is now reusable only because actual Darwin reap was forwarded.
    let lease = host.reserve_attach_input(target(), 1).unwrap();
    drop(lease);
    loop {
        assert!(Instant::now() < stop);
        if matches!(host.shutdown().unwrap(), HostEvent::ShutdownComplete) {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(domain.usage().reserved_sessions, 0);
}
#[test]
fn parent_deadline_expires_stalled_attach_and_suppresses_future_dispatch() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let domain = HostDomain::new::<Platform>(limits()).unwrap();
    let spec = SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap();
    let mut host = RuntimeHost::new(&domain, spec, Platform { first: true }).unwrap();
    let mut input = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    input.bytes_mut().copy_from_slice(DESCRIPTOR);
    let session = host
        .attach(input, Instant::now() + Duration::from_millis(10))
        .unwrap();
    let result = complete(&mut host);
    assert_eq!(result.terminal, Terminal::TimedOut);
    assert_eq!(
        result.effect,
        uiblueprint_host::host_types::EffectReceipt::NotDispatched
    );
    assert!(matches!(
        host.reserve_input(session, 1),
        Err(HostError::Busy)
    ));
    assert!(matches!(next(&mut host), HostEvent::Closed { .. }));
    assert!(matches!(
        host.reserve_input(session, 1),
        Err(HostError::StaleOperation)
    ));
}
