//! Real admitted AX publication with a withheld capture reply. The child is an
//! owned non-UI peer; this models reply delay, not an SDK latency measurement.
use super::*;
use std::{
    os::fd::BorrowedFd,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
use uiblueprint_host::{
    domain::SessionHandle,
    host_types::OperationHandle,
    native_binding::NativeHelperBinding,
    process::DarwinChild,
    process_api::{OwnedProcess, PollInterest, ProcessPlatform, ProcessState, Transfer},
};
use uiblueprint_schema::model::{Channel, Id, Operation};
static HELPERS: AtomicUsize = AtomicUsize::new(0);
static RELEASE: AtomicBool = AtomicBool::new(false);
struct Platform;
struct Child {
    real: DarwinChild,
    capture: bool,
}
impl ProcessPlatform for Platform {
    type Child = Child;
    fn validate_parent_reaping() -> Result<(), HostError> {
        DarwinPlatform::validate_parent_reaping()
    }
    fn spawn(&mut self, spec: &SpawnSpec) -> Result<Child, HostError> {
        let capture = spec.executable().to_bytes().ends_with(b"/native_peer")
            && HELPERS.fetch_add(1, Ordering::SeqCst) > 0;
        DarwinPlatform
            .spawn(spec)
            .map(|real| Child { real, capture })
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
        if self.capture && !RELEASE.load(Ordering::SeqCst) {
            Ok(Transfer::WouldBlock)
        } else {
            self.real.read_output(b)
        }
    }
    fn read_fatal(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
        self.real.read_fatal(b)
    }
    fn close_input(&mut self) {
        self.real.close_input();
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
fn begin<'a>(host: &mut RuntimeHost<'a, Platform>) -> (SessionHandle<'a>, OperationHandle<'a>) {
    let mut input = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    input.bytes_mut().copy_from_slice(DESCRIPTOR);
    let session = host.attach(input, deadline()).unwrap();
    let HostEvent::Attached { clock, .. } = next(host) else {
        panic!("attached")
    };
    let peer = Path::new(env!("CARGO_BIN_EXE_session-worker"))
        .parent()
        .unwrap()
        .join("examples/native_peer");
    host.configure_native_helpers(
        session,
        NativeHelperBinding::authorized(SpawnSpec::new(&peer).unwrap(), 3, b"F").unwrap(),
    )
    .unwrap();
    let mut doc = Document::from_json(
        include_bytes!("../../../../fixtures/golden/ENV-REQUEST-VALID.json"),
        65536,
    )
    .unwrap();
    let Artifact::Request(r) = &mut doc.artifact else {
        panic!("request")
    };
    r.clock_domain = Id(clock.as_str().into());
    r.limits.deadline_ms = 500;
    r.limits.max_output_bytes = 131072;
    r.operation = Operation::Observe {
        channels: vec![Channel::ExternalSemantics, Channel::RenderedCapture],
    };
    let bytes = serde_json::to_vec(&doc).unwrap();
    let mut input = host.reserve_input(session, bytes.len()).unwrap();
    input.bytes_mut().copy_from_slice(&bytes);
    let operation = host
        .submit_native_observe(
            session,
            input,
            OutputRequest {
                channels: 3,
                frame_bytes: 65536,
                total_bytes: 131072,
                input_format: 0,
                retained_partition: 0,
            },
            deadline(),
        )
        .unwrap();
    (session, operation)
}
#[test]
fn capture_delay_cancel_or_deadline_preserves_ax_and_discards_late_reply() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    for cancel in [true, false] {
        HELPERS.store(0, Ordering::SeqCst);
        RELEASE.store(false, Ordering::SeqCst);
        let domain = HostDomain::new::<Platform>(limits()).unwrap();
        let mut host = RuntimeHost::new(
            &domain,
            SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
            Platform,
        )
        .unwrap();
        let (session, operation) = begin(&mut host);
        let stop = deadline();
        while HELPERS.load(Ordering::SeqCst) < 2 {
            assert!(Instant::now() < stop);
            assert!(matches!(
                host.next_event().unwrap(),
                HostEvent::Pending | HostEvent::HelperClosed { .. }
            ));
            thread::sleep(Duration::from_millis(1));
        }
        let completion = if cancel {
            host.cancel(operation).unwrap()
        } else {
            loop {
                assert!(Instant::now() < stop);
                match host.next_event().unwrap() {
                    HostEvent::Complete(c) => break c,
                    HostEvent::Pending | HostEvent::HelperClosed { .. } => {
                        thread::sleep(Duration::from_millis(1))
                    }
                    _ => panic!("timed completion"),
                }
            }
        };
        assert_eq!(
            completion.terminal,
            if cancel {
                Terminal::Cancelled
            } else {
                Terminal::TimedOut
            }
        );
        assert_eq!(completion.committed(), 1);
        assert_eq!(completion.missing(), 2);
        assert!(completion.bytes(0).is_some());
        RELEASE.store(true, Ordering::SeqCst);
        loop {
            assert!(Instant::now() < stop);
            match host.next_event().unwrap() {
                HostEvent::Closed { session: s } => {
                    assert_eq!(s, session);
                    break;
                }
                HostEvent::Pending | HostEvent::HelperClosed { .. } => {
                    thread::sleep(Duration::from_millis(1))
                }
                HostEvent::Complete(_) => panic!("late success after terminal"),
                _ => panic!("cleanup"),
            }
        }
        assert_eq!(completion.committed(), 1);
        assert_eq!(domain.usage().reserved_sessions, 0);
    }
}
