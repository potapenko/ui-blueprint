//! Active begin→helper→guarded receive→ACK composition with a finite non-UI producer.
use super::*;
use std::{
    os::fd::BorrowedFd,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
use uiblueprint_host::{
    native_binding::NativeHelperBinding,
    process::DarwinChild,
    process_api::{OwnedProcess, PollInterest, ProcessPlatform, ProcessState, Transfer},
};
use uiblueprint_schema::{
    SchemaVersion,
    model::{Channel, ChannelResult, Id, Operation},
};
static EXPECT_ACK: AtomicBool = AtomicBool::new(true);
static AX_ACK: AtomicBool = AtomicBool::new(false);
static HELPERS: AtomicUsize = AtomicUsize::new(0);
struct Platform;
struct Child(DarwinChild);
impl ProcessPlatform for Platform {
    type Child = Child;
    fn validate_parent_reaping() -> Result<(), HostError> {
        DarwinPlatform::validate_parent_reaping()
    }
    fn spawn(&mut self, spec: &SpawnSpec) -> Result<Child, HostError> {
        if spec.executable().to_bytes().ends_with(b"/native_peer") {
            let prior = HELPERS.fetch_add(1, Ordering::SeqCst);
            if prior > 0 && EXPECT_ACK.load(Ordering::SeqCst) {
                assert!(
                    AX_ACK.load(Ordering::SeqCst),
                    "capture cannot start before AX ACK"
                );
            }
        }
        DarwinPlatform.spawn(spec).map(Child)
    }
    fn poll(&mut self, i: &mut [PollInterest<'_>], ms: u32) -> Result<(), HostError> {
        DarwinPlatform.poll(i, ms)
    }
}
impl OwnedProcess for Child {
    fn write_input(&mut self, b: &[u8]) -> Result<Transfer, HostError> {
        let r = self.0.write_input(b)?;
        if b.len() == CONTROL_BYTES
            && matches!(r, Transfer::Bytes(CONTROL_BYTES))
            && Control::decode(b.try_into().unwrap()).is_ok_and(|c| {
                c.kind == ControlKind::Ack && c.class == OperationClass::Observe && c.slot == 0
            })
        {
            AX_ACK.store(true, Ordering::SeqCst);
        }
        Ok(r)
    }
    fn read_output(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
        self.0.read_output(b)
    }
    fn read_fatal(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
        self.0.read_fatal(b)
    }
    fn close_input(&mut self) {
        self.0.close_input();
    }
    fn terminate(&mut self) -> Result<(), HostError> {
        self.0.terminate()
    }
    fn try_reap(&mut self) -> Result<ProcessState, HostError> {
        self.0.try_reap()
    }
    fn input_fd(&self) -> Option<BorrowedFd<'_>> {
        self.0.input_fd()
    }
    fn output_fd(&self) -> BorrowedFd<'_> {
        self.0.output_fd()
    }
    fn fatal_fd(&self) -> BorrowedFd<'_> {
        self.0.fatal_fd()
    }
}
fn request(clock: &str) -> Vec<u8> {
    let mut d = Document::from_json(
        include_bytes!("../../../../fixtures/golden/ENV-REQUEST-VALID.json"),
        65536,
    )
    .unwrap();
    let Artifact::Request(r) = &mut d.artifact else {
        panic!("request")
    };
    r.clock_domain = Id(clock.into());
    r.limits.deadline_ms = 1000;
    r.limits.max_output_bytes = 131072;
    r.operation = Operation::Observe {
        channels: vec![Channel::ExternalSemantics, Channel::RenderedCapture],
    };
    serde_json::to_vec(&d).unwrap()
}
fn result<'a>(host: &mut RuntimeHost<'a, Platform>) -> HostCompletion<'a> {
    let end = deadline();
    loop {
        assert!(Instant::now() < end);
        match host.next_event().unwrap() {
            HostEvent::Complete(c) => return c,
            HostEvent::Pending | HostEvent::HelperClosed { .. } => {
                thread::sleep(Duration::from_millis(1))
            }
            _ => panic!("observation completion"),
        }
    }
}
fn output() -> OutputRequest {
    OutputRequest {
        channels: 1,
        frame_bytes: 65536,
        total_bytes: 131072,
        input_format: 1,
        retained_partition: 0,
    }
}
#[test]
fn native_requests_are_collected_after_real_begin_and_ax_ack_precedes_capture() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    for mode in [b'F', b'X', b'W', b'B'] {
        AX_ACK.store(false, Ordering::SeqCst);
        HELPERS.store(0, Ordering::SeqCst);
        EXPECT_ACK.store(mode != b'B', Ordering::SeqCst);
        let domain = HostDomain::new::<Platform>(limits()).unwrap();
        let mut host = RuntimeHost::new(
            &domain,
            SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
            Platform,
        )
        .unwrap();
        let mut input = host
            .reserve_attach_input(target(), DESCRIPTOR.len())
            .unwrap();
        input.bytes_mut().copy_from_slice(DESCRIPTOR);
        let session = host.attach(input, deadline()).unwrap();
        let HostEvent::Attached { clock, .. } = next(&mut host) else {
            panic!("attach")
        };
        let mut input = host.reserve_input(session, QUERY.len()).unwrap();
        input.bytes_mut().copy_from_slice(QUERY);
        host.submit(
            session,
            OperationClass::Validate,
            input,
            output(),
            deadline(),
        )
        .unwrap();
        assert_eq!(result(&mut host).terminal, Terminal::Completed);
        let peer = Path::new(env!("CARGO_BIN_EXE_session-worker"))
            .parent()
            .unwrap()
            .join("examples/native_peer");
        assert!(peer.is_file());
        host.configure_native_helpers(
            session,
            NativeHelperBinding::authorized(SpawnSpec::new(&peer).unwrap(), 3, &[mode]).unwrap(),
        )
        .unwrap();
        let body = request(clock.as_str());
        let mut input = host.reserve_input(session, body.len()).unwrap();
        input.bytes_mut().copy_from_slice(&body);
        let mut out = output();
        out.channels = 3;
        if mode == b'B' {
            out.frame_bytes = 128;
        }
        let op = host
            .submit_native_observe(session, input, out, deadline())
            .unwrap();
        assert_eq!(
            op.sequence, 2,
            "helper must receive Ticket1, not operation2"
        );
        let completed = result(&mut host);
        match mode {
            b'F' => {
                assert_eq!(completed.terminal, Terminal::Completed);
                assert_eq!(completed.committed(), 3);
                assert_eq!(completed.missing(), 0);
                let doc = Document::from_json(completed.bytes(1).unwrap(), 65536).unwrap();
                assert_eq!(doc.schema_version, SchemaVersion::CURRENT);
                let Artifact::ChannelResponse(c) = doc.artifact else {
                    panic!("channel")
                };
                assert_eq!(c.dispatch_sequence, 1);
                assert!(matches!(c.result, ChannelResult::Failed(_)));
                assert!(AX_ACK.load(Ordering::SeqCst));
                assert_eq!(HELPERS.load(Ordering::SeqCst), 2);
            }
            b'X' => {
                assert_eq!(
                    completed.terminal,
                    Terminal::Failed(HostError::WorkerFailed)
                );
                assert_eq!(completed.committed(), 1);
                assert_eq!(completed.missing(), 2);
                assert!(completed.bytes(0).is_some());
            }
            b'W' => {
                assert_eq!(completed.committed(), 0);
                assert_eq!(HELPERS.load(Ordering::SeqCst), 1);
                assert!(matches!(completed.terminal, Terminal::Failed(_)));
            }
            b'B' => {
                assert_eq!(completed.committed(), 0);
                assert!(matches!(
                    completed.terminal,
                    Terminal::Failed(HostError::ResourceLimit)
                ));
            }
            _ => unreachable!(),
        }
        drop(completed);
        let end = deadline();
        loop {
            assert!(Instant::now() < end);
            if matches!(host.shutdown().unwrap(), HostEvent::ShutdownComplete) {
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert!(!domain.usage().abandoned);
    }
}
