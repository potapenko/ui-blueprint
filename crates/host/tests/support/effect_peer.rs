//! Finite fake delivery endpoint. No platform input, SDK, side effects or retries.
//! All successful publication is an unchanged canonical Action fixture; the fake
//! delivery is a local counter, never a claim that an application's state changed.
use std::time::{Duration, Instant};
#[path = "../../src/worker_effect.rs"]
mod worker_effect;
#[path = "../../src/worker_io.rs"]
#[allow(dead_code)] // This actual-I/O peer exercises effect controls, not diagnostics.
mod worker_io;
use uiblueprint_host::{
    process::DarwinPlatform,
    process_api::WorkerPlatform,
    worker_config::{MAX_CONFIG_BYTES, WorkerConfig},
    *,
};
use uiblueprint_plugin_api::{
    ClockReading,
    actions::{ActionControl, EffectGate},
};
use uiblueprint_schema::model::{Artifact, Document, Id, InputModality};
fn body<'a>(io: &mut worker_io::WorkerIo, c: Control, b: &'a mut [u8]) -> &'a [u8] {
    let n = usize::try_from(c.length).unwrap();
    assert!(n <= b.len());
    io.read(&mut b[..n]).unwrap();
    &b[..n]
}
fn main() {
    DarwinPlatform::setup_main(8 * 1_048_576).unwrap();
    let mut io = worker_io::WorkerIo::inherited().unwrap();
    let config = io.control().unwrap();
    assert_eq!(config.kind, ControlKind::Configure);
    let mut cb = [0; MAX_CONFIG_BYTES];
    let config = WorkerConfig::decode(body(&mut io, config, &mut cb)).unwrap();
    let attach = io.control().unwrap();
    let mut buffer = [0; 65536];
    let Artifact::Session(session) = Document::from_json(body(&mut io, attach, &mut buffer), 65536)
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
    io.write_control(Control {
        kind: ControlKind::Ready,
        length: 0,
        value: 0,
        auxiliary: stack as u64,
        ..attach
    })
    .unwrap();
    let mut sequence = 0;
    loop {
        let operation = io.control().unwrap();
        let origin = Instant::now();
        let mut deadline = origin
            .checked_add(Duration::from_millis(operation.value))
            .unwrap();
        assert_eq!(operation.kind, ControlKind::Submit);
        assert!(operation.correlation.operation > sequence);
        sequence = operation.correlation.operation;
        let input = body(&mut io, operation, &mut buffer);
        assert!(input.len() > 1);
        let mode = input[0];
        let metadata_mode = (b'a'..=b'w').contains(&mode);
        let budget_mode = mode.is_ascii_digit();
        assert!(
            operation.class == OperationClass::Mutation
                || (metadata_mode && operation.class == OperationClass::Validate)
                || ((budget_mode || metadata_mode) && operation.class == OperationClass::Prepare)
        );
        let canonical = &input[1..];
        let Artifact::Action(action) = Document::from_json(canonical, 65536).unwrap().artifact
        else {
            panic!("action")
        };
        assert!(config.target.matches(&action.snapshot.context.target));
        if metadata_mode {
            let ready = Control {
                kind: ControlKind::ObserveReady,
                class: operation.class,
                slot: 0,
                flags: 1,
                correlation: operation.correlation,
                length: 0,
                value: operation.correlation.operation,
                auxiliary: 2000,
            };
            if !matches!(mode, b'e' | b'j' | b'p' | b'u') {
                io.write_control(ready).unwrap();
                assert_eq!(io.control().unwrap().kind, ControlKind::ObservePermit);
            }
            if matches!(mode, b'm' | b'n' | b'o' | b'p' | b's' | b'u' | b'v' | b'w') {
                let frame = Control {
                    kind: ControlKind::Frame,
                    class: operation.class,
                    slot: 0,
                    flags: u8::from(operation.class == OperationClass::Mutation),
                    correlation: operation.correlation,
                    length: canonical.len() as u64,
                    value: 0,
                    auxiliary: 0,
                };
                if mode != b'o' {
                    io.write_control(frame).unwrap();
                    io.write(canonical).unwrap();
                    io.write_control(Control {
                        kind: ControlKind::Commit,
                        ..frame
                    })
                    .unwrap();
                    assert_eq!(io.control().unwrap().kind, ControlKind::Ack);
                }
                if matches!(mode, b'v' | b'w') {
                    std::thread::sleep(Duration::from_millis(300));
                }
                io.write_control(Control {
                    kind: ControlKind::Terminal,
                    flags: 0,
                    length: 0,
                    value: if mode == b's' { 10 } else { 9 },
                    auxiliary: 0,
                    ..frame
                })
                .unwrap();
                continue;
            }
            let nonce = if operation.class == OperationClass::Mutation && mode != b'f' {
                let clock = Id(uiblueprint_host::host_types::clock_id(
                    operation.correlation.session_epoch,
                )
                .unwrap()
                .as_str()
                .into());
                let mut timing = worker_effect::WorkerActionControl::new(clock.clone(), origin);
                let mut gate = worker_effect::WorkerEffectGate::new(
                    &mut io,
                    operation,
                    config.target,
                    clock,
                    deadline,
                )
                .unwrap();
                gate.authorize(&action.action, &timing.now())
                    .unwrap_or_else(|_| panic!("parent permit"))
                    .nonce()
            } else {
                0
            };
            let frame = Control {
                kind: ControlKind::Frame,
                class: operation.class,
                slot: 0,
                flags: match mode {
                    b'c' | b'i' => 3,
                    b'd' | b'q' => 4,
                    b'g' => 5,
                    _ => 2,
                },
                correlation: operation.correlation,
                length: canonical.len() as u64,
                value: 0,
                auxiliary: 0,
            };
            io.write_control(frame).unwrap();
            io.write(canonical).unwrap();
            io.write_control(Control {
                kind: ControlKind::Commit,
                flags: if mode == b'h' { 3 } else { frame.flags },
                ..frame
            })
            .unwrap();
            let ack = io.control().unwrap();
            assert_eq!(ack.flags, frame.flags);
            if mode == b'k' {
                return;
            }
            io.write_control(Control {
                kind: ControlKind::Terminal,
                flags: 0,
                length: 0,
                value: if matches!(mode, b'q' | b'r') { 9 } else { 0 },
                auxiliary: if matches!(mode, b'q' | b'r') {
                    0
                } else {
                    nonce + u64::from(mode == b'l')
                },
                ..frame
            })
            .unwrap();
            continue;
        }
        if budget_mode {
            if mode == b'5' {
                std::thread::sleep(Duration::from_millis(300));
            }
            if mode == b'9' {
                std::thread::sleep(Duration::from_millis(300));
            }
            let ready = Control {
                kind: ControlKind::ObserveReady,
                class: operation.class,
                slot: 0,
                flags: if mode == b'7' { 0 } else { 1 },
                correlation: operation.correlation,
                length: 0,
                value: operation.correlation.operation + u64::from(mode == b'6'),
                auxiliary: match mode {
                    b'0' => 0,
                    b'4' | b'9' => 2000,
                    _ => 200,
                },
            };
            io.write_control(ready).unwrap();
            let admitted = io.control().unwrap();
            assert_eq!(admitted.kind, ControlKind::ObservePermit);
            assert_eq!(admitted.class, operation.class);
            assert_eq!(admitted.correlation, operation.correlation);
            assert_eq!(admitted.value, operation.correlation.operation);
            assert!(admitted.auxiliary > 0);
            if mode == b'4' {
                assert!(
                    admitted.auxiliary <= 200,
                    "long canonical duration cannot extend parent"
                );
            }
            if mode == b'9' {
                assert!(
                    admitted.auxiliary <= 1700,
                    "deadline anchored at operation start, not admission now"
                );
            }
            deadline = Instant::now()
                .checked_add(Duration::from_millis(admitted.auxiliary))
                .unwrap();
            if mode == b'8' {
                io.write_control(ready).unwrap();
                let _ = io.control();
                return;
            }
            if operation.class == OperationClass::Prepare {
                if mode == b'1' || mode == b'4' {
                    std::thread::sleep(Duration::from_millis(300));
                }
                let frame = Control {
                    kind: ControlKind::Frame,
                    class: operation.class,
                    slot: 0,
                    flags: 2,
                    correlation: operation.correlation,
                    length: canonical.len() as u64,
                    value: 0,
                    auxiliary: 0,
                };
                io.write_control(frame).unwrap();
                io.write(canonical).unwrap();
                io.write_control(Control {
                    kind: ControlKind::Commit,
                    ..frame
                })
                .unwrap();
                let ack = io.control().unwrap();
                assert_eq!(ack.kind, ControlKind::Ack);
                if mode == b'2' {
                    std::thread::sleep(Duration::from_millis(300));
                }
                io.write_control(Control {
                    kind: ControlKind::Terminal,
                    flags: 0,
                    length: 0,
                    value: 0,
                    auxiliary: 0,
                    ..frame
                })
                .unwrap();
                continue;
            }
        }
        if matches!(mode, b'R' | b'S' | b'N' | b'G' | b'M' | b'U') {
            let refusal = include_bytes!("../../../../fixtures/golden/G01-READONLY.json");
            Document::from_json(refusal, 65536).unwrap();
            let frame = Control {
                kind: ControlKind::Frame,
                class: operation.class,
                slot: 0,
                flags: u8::from(mode != b'U'),
                correlation: operation.correlation,
                length: refusal.len() as u64,
                value: 0,
                auxiliary: 0,
            };
            io.write_control(frame).unwrap();
            io.write(refusal).unwrap();
            io.write_control(Control {
                kind: ControlKind::Commit,
                flags: if mode == b'M' { 0 } else { frame.flags },
                ..frame
            })
            .unwrap();
            let ack = io.control().unwrap();
            assert_eq!(ack.kind, ControlKind::Ack);
            assert_eq!(ack.flags, frame.flags);
            if mode == b'G' {
                io.write_control(Control {
                    kind: ControlKind::EffectReady,
                    length: 0,
                    flags: 0,
                    value: 0,
                    auxiliary: 0,
                    ..frame
                })
                .unwrap();
                let _ = io.control();
                return;
            }
            io.write_control(Control {
                kind: ControlKind::Terminal,
                flags: 0,
                length: 0,
                value: if mode == b'S' { 0 } else { 2 },
                auxiliary: if mode == b'N' { 7 } else { 0 },
                ..frame
            })
            .unwrap();
            continue;
        }
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
        let mut selected = action.action.clone();
        if mode == b'P' {
            selected.modality = InputModality::Pointer;
        } // existing fake physical-lane case
        let clock = Id(
            uiblueprint_host::host_types::clock_id(operation.correlation.session_epoch)
                .unwrap()
                .as_str()
                .into(),
        );
        let mut control = worker_effect::WorkerActionControl::new(clock.clone(), origin);
        let reading: ClockReading = control.now();
        let mut gate = worker_effect::WorkerEffectGate::new(
            &mut io,
            operation,
            config.target,
            clock,
            deadline,
        )
        .unwrap();
        let permit = gate
            .authorize(&selected, &reading)
            .unwrap_or_else(|_| panic!("actual parent permit required"));
        let nonce = permit.nonce();
        assert_eq!(gate.nonce(), Some(nonce));
        assert!(gate.requested());
        assert!(!control.cancelled());
        drop(gate);
        let mut deliveries = 0; // Deliberately finite fake endpoint: exactly one consume.
        deliveries += 1;
        assert_eq!(deliveries, 1);
        match mode {
            b'L' => return, // Lost acknowledgement after possible fake delivery.
            b'D' => {
                io.write_control(ready).unwrap();
                let _ = io.control();
                panic!("duplicate permit accepted");
            }
            b'P' => {
                // Keep the fake physical delivery unresolved until parent cancel
                // closes input. The outer test/RuntimeHost owns the finite deadline
                // and kill/reap bound; no sleep is used as readiness evidence.
                let mut unexpected = [0];
                assert!(io.read(&mut unexpected).is_err());
                return;
            }
            b'O' => {
                io.write_control(Control {
                    kind: ControlKind::Frame,
                    class: operation.class,
                    slot: 0,
                    flags: 1,
                    correlation: operation.correlation,
                    length: canonical.len() as u64,
                    value: 0,
                    auxiliary: 0,
                })
                .unwrap();
                let _ = io.control();
                return;
            }
            b'3' => {
                std::thread::sleep(Duration::from_millis(300));
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
        io.write_control(frame).unwrap();
        io.write(canonical).unwrap();
        io.write_control(Control {
            kind: ControlKind::Commit,
            ..frame
        })
        .unwrap();
        let ack = io.control().unwrap();
        assert_eq!(ack.kind, ControlKind::Ack);
        assert!(ack.matches(frame));
        io.write_control(Control {
            kind: ControlKind::Terminal,
            length: 0,
            value: 0,
            auxiliary: nonce,
            ..frame
        })
        .unwrap();
    }
}
