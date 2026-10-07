#![cfg(target_os = "macos")]
//! Bounded real publication-reserve probe; no substitute serializer/ACK/guard.
use std::{
    path::Path,
    thread,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    process::{DarwinChild, DarwinPlatform},
    process_api::{OwnedProcess, ProcessPlatform, ProcessState, SpawnSpec, Transfer},
    publication::Publication,
    *,
};
const MIB: usize = 1_048_576;
const BUFFER: usize = 512 * 1024;
fn limits() -> HostLimits {
    HostLimits {
        workers: 1,
        worker_bytes: 4 * MIB,
        publication_reserve: MIB,
        bootstrap_bytes: MIB,
        parent_bytes: 2 * MIB,
        input_bytes: 4096,
        ingress_bytes: 1024,
        output_bytes: BUFFER,
        request_output_bytes: 2 * MIB,
        completion_groups: 1,
        control_bytes: 4096,
        cleanup_ms: 1000,
        retained_domain_bytes: 4 * MIB,
        retained_per_worker: MIB,
        main_stack_bytes: 8 * MIB,
        watchdog_stack_bytes: MIB,
    }
}
fn deadline() -> Instant {
    Instant::now() + Duration::from_secs(5)
}
struct Probe {
    child: DarwinChild,
    reaped: bool,
}
impl Drop for Probe {
    fn drop(&mut self) {
        if !self.reaped {
            let _ = self.child.terminate();
            let end = deadline();
            while Instant::now() < end {
                if matches!(
                    self.child.try_reap(),
                    Ok(ProcessState::Exited { .. } | ProcessState::Signaled { .. })
                ) {
                    self.reaped = true;
                    break;
                }
                thread::yield_now();
            }
            assert!(self.reaped, "owned probe cleanup");
        }
    }
}
fn send(child: &mut DarwinChild, bytes: &[u8]) {
    let mut n = 0;
    let end = deadline();
    while n < bytes.len() {
        assert!(Instant::now() < end);
        match child.write_input(&bytes[n..]).unwrap() {
            Transfer::Bytes(0) | Transfer::Closed => panic!("input closed"),
            Transfer::Bytes(k) => n += k,
            Transfer::WouldBlock => thread::yield_now(),
        }
    }
}
fn control(child: &mut DarwinChild, fatal: bool) -> Control {
    let mut bytes = [0; CONTROL_BYTES];
    let mut n = 0;
    let end = deadline();
    while n < bytes.len() {
        assert!(Instant::now() < end);
        let result = if fatal {
            child.read_fatal(&mut bytes[n..])
        } else {
            child.read_output(&mut bytes[n..])
        };
        match result.unwrap() {
            Transfer::Bytes(k) => n += k,
            Transfer::WouldBlock => thread::yield_now(),
            Transfer::Closed => panic!("partial control"),
        }
    }
    Control::decode(&bytes).unwrap()
}
#[test]
fn canonical_publication_scratch_fits_reserved_allowance_and_is_released() {
    let path = Path::new(env!("CARGO_BIN_EXE_session-worker"))
        .parent()
        .unwrap()
        .join("examples/publication_probe");
    assert!(path.is_file(), "build actual publication_probe first");
    let spec = SpawnSpec::new(&path).unwrap();
    for mode in 0..=6 {
        let buffers = ParentBuffers::new(limits(), 0).unwrap();
        let operation = Control {
            kind: ControlKind::Submit,
            class: OperationClass::Verify,
            slot: 0,
            flags: 1,
            correlation: Correlation {
                session_epoch: 77,
                operation: 41,
            },
            length: 0,
            value: mode,
            auxiliary: BUFFER as u64 | ((2 * MIB as u64) << 32),
        };
        let mut publication = Publication::new(
            buffers.reserve_group(1).unwrap(),
            operation.correlation,
            operation.class,
            1,
            2 * MIB,
        )
        .unwrap();
        let mut probe = Probe {
            child: DarwinPlatform.spawn(&spec).unwrap(),
            reaped: false,
        };
        send(&mut probe.child, &operation.encode());
        let first = control(&mut probe.child, false);
        let metric = if first.kind == ControlKind::Frame {
            assert_ne!(mode, 5);
            publication.begin(first).unwrap();
            let end = deadline();
            while !publication.remaining_mut().unwrap().is_empty() {
                assert!(Instant::now() < end);
                match probe
                    .child
                    .read_output(publication.remaining_mut().unwrap())
                    .unwrap()
                {
                    Transfer::Bytes(n) => publication.advance(n).unwrap(),
                    Transfer::WouldBlock => thread::yield_now(),
                    Transfer::Closed => panic!("partial canonical frame"),
                }
            }
            let commit = control(&mut probe.child, false);
            let ack = publication.commit(commit).unwrap();
            if mode == 6 {
                probe.child.close_input();
            } else {
                send(&mut probe.child, &ack.encode());
                publication.ack_sent(ack).unwrap();
            }
            control(&mut probe.child, false)
        } else {
            assert_eq!(mode, 5);
            first
        };
        assert_eq!(metric.kind, ControlKind::Terminal);
        assert_eq!(metric.correlation, operation.correlation);
        assert_eq!(
            metric.flags,
            if mode == 5 {
                1
            } else if mode == 6 {
                2
            } else {
                0
            }
        );
        if mode == 5 {
            assert_eq!(metric.value, 1, "one serde IO error box");
            assert_eq!(metric.auxiliary, 40, "pinned aarch64 ErrorImpl layout");
        } else {
            assert_eq!(
                metric.value, 0,
                "canonical success and fixed IO need no scratch heap"
            );
            assert_eq!(metric.auxiliary, 0);
        }
        assert!(metric.auxiliary <= MIB as u64);
        let done = publication.finish();
        assert_eq!(done.committed, if mode >= 5 { 0 } else { 1 });
        assert_eq!(done.missing, if mode >= 5 { 1 } else { 0 });
        if mode < 5 {
            assert!(!done.frame(0).unwrap().is_empty());
            assert_eq!(done.frame(0).unwrap().len() as u64, metric.length);
        }
        let report = control(&mut probe.child, true);
        assert_eq!(report.kind, ControlKind::Fatal);
        assert_eq!(
            report.value, 5,
            "explicit counter-report sentinel, not real parent death"
        );
        assert_eq!(report.length, mode);
        assert_eq!(
            report.auxiliary, 0,
            "output backing, ballast and scratch actually released"
        );
        let end = deadline();
        loop {
            assert!(Instant::now() < end);
            match probe.child.try_reap().unwrap() {
                ProcessState::Running => thread::yield_now(),
                ProcessState::Exited { code } => {
                    assert_eq!(code, 105);
                    probe.reaped = true;
                    break;
                }
                other => panic!("unexpected exit {other:?}"),
            }
        }
        eprintln!(
            "publication mode={mode} scratch_calls={} peak={} encoded={}",
            metric.value, metric.auxiliary, metric.length
        );
    }
}
