//! Finite fake delivery endpoint. No platform input, SDK, side effects or retries.
//! All successful publication is an unchanged canonical Action fixture; the fake
//! delivery is a local counter, never a claim that an application's state changed.
use std::{
    fs::File,
    io::{Read, Write},
    os::fd::FromRawFd,
};
use uiblueprint_host::{
    process::DarwinPlatform,
    process_api::{CHILD_INPUT_FD, CHILD_OUTPUT_FD, WorkerPlatform},
    worker_config::{MAX_CONFIG_BYTES, WorkerConfig},
    *,
};
use uiblueprint_schema::model::{Artifact, Document};
struct Io {
    input: File,
    output: File,
}
impl Io {
    fn control(&mut self) -> Control {
        let mut b = [0; CONTROL_BYTES];
        self.input.read_exact(&mut b).unwrap();
        Control::decode(&b).unwrap()
    }
    fn send(&mut self, c: Control) {
        self.output.write_all(&c.encode()).unwrap();
    }
    fn body<'a>(&mut self, c: Control, b: &'a mut [u8]) -> &'a [u8] {
        let n = usize::try_from(c.length).unwrap();
        assert!(n <= b.len());
        self.input.read_exact(&mut b[..n]).unwrap();
        &b[..n]
    }
}
fn main() {
    DarwinPlatform::setup_main(8 * 1_048_576).unwrap();
    // SAFETY: Native's exact-path test spawn supplies these two live descriptors.
    // This entrypoint adopts them once; no other owners or threads access them.
    let mut io = unsafe {
        Io {
            input: File::from_raw_fd(CHILD_INPUT_FD),
            output: File::from_raw_fd(CHILD_OUTPUT_FD),
        }
    };
    let config = io.control();
    assert_eq!(config.kind, ControlKind::Configure);
    let mut cb = [0; MAX_CONFIG_BYTES];
    let config = WorkerConfig::decode(io.body(config, &mut cb)).unwrap();
    let attach = io.control();
    let mut buffer = [0; 65536];
    let Artifact::Session(session) = Document::from_json(io.body(attach, &mut buffer), 65536)
        .unwrap()
        .artifact
    else {
        panic!("session")
    };
    assert!(config.target.matches(&session.target));
    let request =
        DarwinPlatform::watchdog_stack_request(config.limits.watchdog_stack_bytes).unwrap();
    let stack = std::thread::Builder::new()
        .stack_size(request)
        .spawn(|| DarwinPlatform::current_stack_bytes().unwrap())
        .unwrap()
        .join()
        .unwrap();
    // This short-lived probe verifies handshake metadata only. This fake peer is
    // not the production worker/watchdog or an allocation enforcement proof.
    io.send(Control {
        kind: ControlKind::Ready,
        length: 0,
        value: 0,
        auxiliary: stack as u64,
        ..attach
    });
    let mut sequence = 0;
    loop {
        let operation = io.control();
        assert_eq!(operation.kind, ControlKind::Submit);
        assert_eq!(operation.class, OperationClass::Mutation);
        assert!(operation.correlation.operation > sequence);
        sequence = operation.correlation.operation;
        let input = io.body(operation, &mut buffer);
        assert!(input.len() > 1);
        let mode = input[0];
        let canonical = &input[1..];
        let Artifact::Action(action) = Document::from_json(canonical, 65536).unwrap().artifact
        else {
            panic!("action")
        };
        assert!(config.target.matches(&action.snapshot.context.target));
        let ready = Control {
            kind: ControlKind::EffectReady,
            class: operation.class,
            slot: 0,
            flags: u8::from(mode == b'P'),
            correlation: operation.correlation,
            length: 0,
            value: 0,
            auxiliary: 0,
        };
        io.send(ready);
        let permit = io.control();
        assert_eq!(permit.kind, ControlKind::EffectPermit);
        assert_eq!(permit.correlation, ready.correlation);
        assert_ne!(permit.value, 0);
        let mut deliveries = 0; // Deliberately finite fake endpoint: exactly one consume.
        deliveries += 1;
        assert_eq!(deliveries, 1);
        match mode {
            b'L' => return, // Lost acknowledgement after possible fake delivery.
            b'D' => {
                io.send(ready);
                let _ = io.control();
                panic!("duplicate permit accepted");
            }
            b'P' => {
                // Keep the fake physical delivery unresolved until parent cancel
                // closes input. The outer test/RuntimeHost owns the finite deadline
                // and kill/reap bound; no sleep is used as readiness evidence.
                let mut unexpected = [0];
                assert_eq!(io.input.read(&mut unexpected).unwrap(), 0);
                return;
            }
            b'C' => (),
            _ => panic!("finite test mode"),
        }
        let frame = Control {
            kind: ControlKind::Frame,
            class: operation.class,
            slot: 0,
            flags: 0,
            correlation: operation.correlation,
            length: canonical.len() as u64,
            value: 0,
            auxiliary: 0,
        };
        io.send(frame);
        io.output.write_all(canonical).unwrap();
        io.send(Control {
            kind: ControlKind::Commit,
            ..frame
        });
        let ack = io.control();
        assert_eq!(ack.kind, ControlKind::Ack);
        assert!(ack.matches(frame));
        io.send(Control {
            kind: ControlKind::Terminal,
            length: 0,
            value: 0,
            auxiliary: permit.value,
            ..frame
        });
    }
}
