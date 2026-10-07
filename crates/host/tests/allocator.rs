#![cfg(target_os = "macos")]
#![forbid(unsafe_code)]
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    Control, ControlKind, OperationClass,
    process::{DarwinChild, DarwinPlatform},
    process_api::{OwnedProcess, ProcessPlatform, ProcessState, SpawnSpec, Transfer},
};

struct OwnedProbe {
    child: DarwinChild,
    reaped: bool,
}
impl Drop for OwnedProbe {
    fn drop(&mut self) {
        if !self.reaped {
            let _ = self.child.terminate();
            let end = Instant::now() + Duration::from_secs(1);
            while Instant::now() < end {
                if self
                    .child
                    .try_reap()
                    .is_ok_and(|s| s != ProcessState::Running)
                {
                    self.reaped = true;
                    break;
                }
                thread::sleep(Duration::from_millis(1));
            }
        }
    }
}
fn executable() -> PathBuf {
    std::env::var_os("H01_ALLOCATOR_PROBE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::current_exe()
                .expect("test executable")
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("examples/allocator_probe")
        })
}
fn run(mode: u8) -> (ProcessState, Option<Control>, usize) {
    let path = executable();
    assert!(
        path.is_file(),
        "build the existing-source allocator_probe example first"
    );
    let mut probe = OwnedProbe {
        child: DarwinPlatform
            .spawn(&SpawnSpec::new(&path).unwrap())
            .expect("owned disposable probe"),
        reaped: false,
    };
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        assert!(Instant::now() < deadline, "bounded probe command");
        match probe.child.write_input(&[mode]).expect("owned input") {
            Transfer::Bytes(1) => break,
            Transfer::WouldBlock => thread::sleep(Duration::from_millis(1)),
            _ => panic!("probe input closed"),
        }
    }
    let mut bytes = [0_u8; 64];
    let mut used = 0;
    let mut state = ProcessState::Running;
    let mut closed = false;
    let mut callbacks = [0_u8; 4];
    let mut calls = 0;
    let mut output_closed = false;
    while Instant::now() < deadline {
        if !output_closed {
            assert!(
                calls < callbacks.len(),
                "bounded one-call forwarding marker"
            );
            match probe
                .child
                .read_output(&mut callbacks[calls..])
                .expect("probe marker lane")
            {
                Transfer::Bytes(n) => calls += n,
                Transfer::WouldBlock => (),
                Transfer::Closed => output_closed = true,
            }
        }
        if used < bytes.len() && !closed {
            match probe
                .child
                .read_fatal(&mut bytes[used..])
                .expect("private fatal lane")
            {
                Transfer::Bytes(n) => used += n,
                Transfer::WouldBlock => (),
                Transfer::Closed => closed = true,
            }
        }
        if state == ProcessState::Running {
            state = probe.child.try_reap().expect("exclusive owned reap");
            probe.reaped = state != ProcessState::Running;
        }
        if probe.reaped && (closed || used == bytes.len()) && output_closed {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert!(
        probe.reaped,
        "owned probe must exit and be reaped within bound"
    );
    assert!(
        used == 0 || used == bytes.len(),
        "partial status is not attributable"
    );
    let record = if used == bytes.len() {
        Some(Control::decode(&bytes).expect("actual fixed fatal encoding"))
    } else {
        None
    };
    if let Some(record) = record {
        assert_eq!(record.kind, ControlKind::Fatal);
        assert_eq!(record.class, OperationClass::Validate);
        assert_eq!(record.correlation.session_epoch, 77);
        assert_eq!(record.correlation.operation, 41);
    }
    assert!(output_closed);
    assert!(callbacks[..calls].iter().all(|b| *b == 0xa1));
    (state, record, calls)
}
fn fatal(mode: u8, reason: u64, requested: u64, live: u64, phase: u8) {
    let (exit, record, calls) = run(mode);
    assert_eq!(calls, 0);
    assert_eq!(
        exit,
        ProcessState::Exited {
            code: 100 + reason as i32
        }
    );
    let record = record.expect("attributed status required");
    assert_eq!(
        (record.value, record.length, record.auxiliary, record.flags),
        (reason, requested, live, phase)
    );
}

#[test]
fn actual_system_backed_allocations_zeroing_and_realloc_balance() {
    // ParentGone is the probe's explicit reporting sentinel, not simulated EOF.
    for mode in [b'0', b'A', b'Q'] {
        fatal(mode, 5, mode as u64, 0, 1);
    }
}
#[test]
fn real_guard_refuses_ordinary_and_full_new_plus_old_realloc_charge() {
    fatal(b'O', 1, 4097, 0, 1);
    fatal(b'R', 1, 2048, 3072, 1);
    fatal(b'0', 5, b'0' as u64, 0, 1); // The parent survived both exits and owns/reaps another child.
}
#[test]
fn publication_allowance_is_thread_owned_and_cannot_escape() {
    fatal(b'P', 5, b'P' as u64, 0, 4);
    fatal(b'I', 3, 0, 4097, 6);
    fatal(b'N', 3, 0, 0, 6);
    fatal(b'T', 1, 4097, 0, 6);
}
#[test]
fn fatal_encoding_and_absence_have_distinct_attribution() {
    fatal(b'S', 2, 64, 0, 1); // Direct fatal reporting, NOT a System-null-path test.
    fatal(b'C', 7, 0, 0, 1);
    let (state, status, calls) = run(b'M');
    assert_eq!(calls, 0);
    assert_eq!(state, ProcessState::Exited { code: 101 });
    assert!(
        status.is_none(),
        "exit code alone must not invent a fatal record"
    );
}

#[test]
fn injected_small_null_releases_its_precharge_without_erasing_live_ownership() {
    for mode in [b'F', b'Z', b'G'] {
        let (state, record, calls) = run(mode);
        assert_eq!(state, ProcessState::Exited { code: 102 });
        let record = record.expect("actual shared System-null fatal path");
        assert_eq!(
            (record.value, record.length, record.auxiliary, record.flags),
            (2, 64, 32, 1)
        );
        assert_eq!(calls, 1, "one forwarding call after successful precharge");
    }
    fatal(b'B', 1, 4097, 0, 1); // Failed precharge never invokes the callback.
    fatal(b'H', 1, 2048, 3072, 1); // Realloc reserves old+FULL new before callback.
}

#[test]
fn direct_semantic_validator_allocation_is_refused_by_full_guard() {
    let (state, record, ready) = run(b'V');
    assert_eq!(
        ready, 1,
        "positive validator and real ballast control completed"
    );
    assert_eq!(state, ProcessState::Exited { code: 105 });
    let record = record.expect("positive control's explicit reporting sentinel");
    assert_eq!(
        (record.value, record.length, record.auxiliary, record.flags),
        (5, u64::from(b'V'), 0, 2)
    );

    let (state, record, ready) = run(b'W');
    assert_eq!(ready, 1, "same positive prerequisite before interception");
    assert_eq!(state, ProcessState::Exited { code: 101 });
    let record = record.expect("actual allocator refusal during direct validation");
    assert_eq!((record.value, record.auxiliary, record.flags), (1, 4096, 2));
    assert!(
        (1..=4096).contains(&record.length),
        "predeclared small validator allocation, no fitted layout/cap"
    );
    // run proves actual owned-child reap. Probe-selected phase2 identifies this
    // direct call only; no integrated worker phase or ACK-preservation claim.
}
