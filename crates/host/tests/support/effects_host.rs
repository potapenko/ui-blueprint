//! These tests dispatch only to the explicitly selected finite fake endpoint.
//! Counter delivery/receipt proves nonce ownership, never real application input.
use super::*;
use uiblueprint_host::{
    domain::SessionHandle,
    host_types::{EffectReceipt, OperationHandle},
};
const ACTION: &[u8] = include_bytes!("../../../../fixtures/golden/ENV-ACTION-VALID.json");
fn fake_spec() -> SpawnSpec {
    let peer = Path::new(env!("CARGO_BIN_EXE_session-worker"))
        .parent()
        .unwrap()
        .join("examples/effect_peer");
    assert!(
        peer.is_file(),
        "build the effect_peer example before runtime tests"
    );
    SpawnSpec::new(&peer).unwrap()
}
fn attach<'a, P: uiblueprint_host::process_api::ProcessPlatform + 'static>(
    host: &mut RuntimeHost<'a, P>,
    mutation: bool,
) -> SessionHandle<'a> {
    let Artifact::Session(s) = Document::from_json(DESCRIPTOR, DESCRIPTOR.len())
        .unwrap()
        .artifact
    else {
        panic!("session")
    };
    let target = TargetLease::authorized(&s.target, mutation).unwrap();
    let mut lease = host.reserve_attach_input(target, DESCRIPTOR.len()).unwrap();
    lease.bytes_mut().copy_from_slice(DESCRIPTOR);
    let session = host.attach(lease, deadline()).unwrap();
    assert!(matches!(next(host),HostEvent::Attached{session:s,..} if s==session));
    session
}
fn submit<'a, P: uiblueprint_host::process_api::ProcessPlatform + 'static>(
    host: &mut RuntimeHost<'a, P>,
    session: SessionHandle<'a>,
    mode: u8,
    until: Instant,
) -> Result<OperationHandle<'a>, HostError> {
    let mut input = host.reserve_input(session, ACTION.len() + 1)?;
    input.bytes_mut()[0] = mode;
    input.bytes_mut()[1..].copy_from_slice(ACTION);
    host.submit(
        session,
        OperationClass::Mutation,
        input,
        OutputRequest {
            channels: 1,
            frame_bytes: 65536,
            total_bytes: 65536,
            input_format: 0,
            retained_partition: 0,
        },
        until,
    )
}
fn finish<P: uiblueprint_host::process_api::ProcessPlatform + 'static>(
    host: &mut RuntimeHost<'_, P>,
) {
    let stop = deadline();
    loop {
        assert!(Instant::now() < stop);
        if matches!(host.shutdown().unwrap(), HostEvent::ShutdownComplete) {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
}

mod permit_fault {
    use super::*;
    use std::os::fd::BorrowedFd;
    use uiblueprint_host::{process::DarwinChild, process_api::*};
    pub(super) struct Platform(pub u8);
    pub struct Child {
        real: DarwinChild,
        fault: u8,
        altered: Option<[u8; CONTROL_BYTES]>,
        offset: usize,
        ack_release: Option<Instant>,
    }
    impl ProcessPlatform for Platform {
        type Child = Child;
        fn validate_parent_reaping() -> Result<(), HostError> {
            DarwinPlatform::validate_parent_reaping()
        }
        fn spawn(&mut self, spec: &SpawnSpec) -> Result<Child, HostError> {
            DarwinPlatform.spawn(spec).map(|real| Child {
                real,
                fault: self.0,
                altered: None,
                offset: 0,
                ack_release: None,
            })
        }
        fn poll(&mut self, interests: &mut [PollInterest<'_>], ms: u32) -> Result<(), HostError> {
            DarwinPlatform.poll(interests, ms)
        }
    }
    impl OwnedProcess for Child {
        fn write_input(&mut self, bytes: &[u8]) -> Result<Transfer, HostError> {
            if self.fault == 4
                && bytes.len() == CONTROL_BYTES
                && Control::decode(bytes.try_into().unwrap())
                    .is_ok_and(|c| c.kind == ControlKind::Ack)
            {
                let release = *self
                    .ack_release
                    .get_or_insert_with(|| Instant::now() + Duration::from_millis(300));
                if Instant::now() < release {
                    return Ok(Transfer::WouldBlock);
                }
            }
            if (1..=3).contains(&self.fault)
                && self.altered.is_none()
                && bytes.len() == CONTROL_BYTES
                && let Ok(mut c) = Control::decode(bytes.try_into().unwrap())
                && c.kind == ControlKind::EffectPermit
            {
                match self.fault {
                    1 => c.correlation.operation += 1,
                    2 => c.value = 0,
                    _ => c.value += 1,
                }
                self.altered = Some(c.encode());
                self.offset = 0;
            }
            if let Some(altered) = &self.altered {
                assert_eq!(
                    bytes.len(),
                    CONTROL_BYTES - self.offset,
                    "same fixed parent write cursor"
                );
                let result = self.real.write_input(&altered[self.offset..])?;
                if let Transfer::Bytes(n) = result {
                    self.offset += n;
                    if self.offset == CONTROL_BYTES {
                        self.altered = None;
                    }
                }
                Ok(result)
            } else {
                self.real.write_input(bytes)
            }
        }
        fn read_output(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
            self.real.read_output(b)
        }
        fn read_fatal(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
            self.real.read_fatal(b)
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
}

#[test]
fn typed_action_admission_clamps_parent_from_start_and_refuses_late_ack_terminal_or_effect() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    for (mode, outer, ack_fault) in [
        (b'1', 2000, 0),
        (b'2', 2000, 0),
        (b'2', 2000, 4),
        (b'3', 2000, 0),
        (b'4', 200, 0),
        (b'5', 2000, 0),
        (b'6', 2000, 0),
        (b'7', 2000, 0),
        (b'8', 2000, 0),
        (b'0', 2000, 0),
        (b'9', 3000, 0),
    ] {
        let domain = HostDomain::new::<permit_fault::Platform>(limits()).unwrap();
        let mut host =
            RuntimeHost::new(&domain, fake_spec(), permit_fault::Platform(ack_fault)).unwrap();
        let session = attach(&mut host, true);
        let mut input = host.reserve_input(session, ACTION.len() + 1).unwrap();
        input.bytes_mut()[0] = mode;
        input.bytes_mut()[1..].copy_from_slice(ACTION);
        let request = OutputRequest {
            channels: 1,
            frame_bytes: 65536,
            total_bytes: 65536,
            input_format: 1,
            retained_partition: 0,
        };
        let class = if mode == b'3' {
            OperationClass::Mutation
        } else {
            OperationClass::Prepare
        };
        host.submit(
            session,
            class,
            input,
            request,
            Instant::now() + Duration::from_millis(outer),
        )
        .unwrap();
        let c = complete(&mut host);
        if mode == b'9' {
            assert_eq!(c.terminal, Terminal::Completed);
            assert_eq!(c.committed(), 1);
        } else if matches!(mode, b'6' | b'7' | b'8' | b'0') {
            assert_eq!(c.terminal, Terminal::Failed(HostError::InvalidControl));
            assert_eq!(c.committed(), 0);
        } else {
            assert_eq!(c.terminal, Terminal::TimedOut);
        }
        if mode == b'3' {
            assert!(c.effect_unknown());
        } else {
            assert_eq!(c.effect, EffectReceipt::NotDispatched);
        }
        if mode == b'2' && ack_fault == 0 {
            assert_eq!(
                c.bytes(0),
                Some(ACTION),
                "ACKed body survives rejected late terminal"
            );
        }
        if ack_fault == 4 {
            assert_eq!(
                c.committed(),
                0,
                "late ACK never commits full received payload"
            );
        }
        drop(c);
        finish(&mut host);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert_eq!(domain.usage().completion_groups, 0);
        assert!(!domain.usage().abandoned);
    }
}
#[test]
fn actual_worker_effect_bridge_refuses_corrupt_permit_and_parent_rejects_wrong_nonce() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    for fault in 1..=3 {
        let domain = HostDomain::new::<permit_fault::Platform>(limits()).unwrap();
        let mut host =
            RuntimeHost::new(&domain, fake_spec(), permit_fault::Platform(fault)).unwrap();
        let session = attach(&mut host, true);
        submit(&mut host, session, b'C', deadline()).unwrap();
        let c = complete(&mut host);
        assert!(matches!(
            c.terminal,
            Terminal::Failed(HostError::WorkerFailed | HostError::InvalidControl)
        ));
        assert!(c.effect_unknown());
        if fault <= 2 {
            assert_eq!(
                c.committed(),
                0,
                "invalid correlation/zero nonce cannot obtain a delivery token"
            );
        } else {
            assert_eq!(
                c.bytes(0),
                Some(ACTION),
                "complete prior ACK survives nonce terminal mismatch"
            );
        }
        drop(c);
        finish(&mut host);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert!(!domain.usage().abandoned);
    }
}
#[test]
fn fake_delivery_confirms_one_nonce_per_operation_and_reuses_worker() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let domain = HostDomain::new::<DarwinPlatform>(limits()).unwrap();
    let mut host = RuntimeHost::new(&domain, fake_spec(), DarwinPlatform).unwrap();
    let session = attach(&mut host, true);
    let mut previous = 0;
    for _ in 0..2 {
        submit(&mut host, session, b'C', deadline()).unwrap();
        let c = complete(&mut host);
        assert_eq!(c.terminal, Terminal::Completed);
        assert_eq!(c.bytes(0), Some(ACTION));
        let EffectReceipt::Confirmed { nonce } = c.effect else {
            panic!("fake delivery receipt")
        };
        assert!(nonce > previous);
        previous = nonce;
    }
    assert_eq!(domain.usage().reserved_sessions, 1);
    finish(&mut host);
}

#[test]
fn marked_pre_dispatch_refusal_cannot_gain_effect_authority_or_false_success() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    for mode in [b'R', b'S', b'N', b'G', b'M', b'U', b'O'] {
        let domain = HostDomain::new::<DarwinPlatform>(limits()).unwrap();
        let mut host = RuntimeHost::new(&domain, fake_spec(), DarwinPlatform).unwrap();
        let session = attach(&mut host, true);
        let owned = domain.usage().parent_owned_bytes;
        submit(&mut host, session, mode, deadline()).unwrap();
        let c = complete(&mut host);
        assert!(
            matches!(c.terminal, Terminal::Failed(_)),
            "refusal never completes mutation"
        );
        if mode == b'O' {
            assert!(c.effect_unknown());
        } else {
            assert_eq!(c.effect, EffectReceipt::NotDispatched);
        }
        if matches!(mode, b'R' | b'S' | b'N' | b'G') {
            assert_eq!(c.committed(), 1);
            assert_eq!(
                c.bytes(0),
                Some(include_bytes!("../../../../fixtures/golden/G01-READONLY.json").as_slice())
            );
        } else {
            assert_eq!(c.committed(), 0);
        }
        assert_eq!(domain.usage().parent_owned_bytes, owned);
        if mode == b'R' {
            let original = c.bytes(0).unwrap().to_vec();
            submit(&mut host, session, b'C', deadline()).unwrap();
            let success = complete(&mut host);
            assert_eq!(success.terminal, Terminal::Completed);
            assert!(matches!(success.effect, EffectReceipt::Confirmed { .. }));
            assert_eq!(
                c.bytes(0),
                Some(original.as_slice()),
                "refusal latch belongs only to its operation"
            );
            drop(success);
        }
        drop(c);
        finish(&mut host);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert_eq!(domain.usage().completion_groups, 0);
        assert!(!domain.usage().abandoned);
    }
}
#[test]
fn fake_loss_or_duplicate_permit_request_is_unknown_and_never_retried() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    for mode in [b'L', b'D'] {
        let domain = HostDomain::new::<DarwinPlatform>(limits()).unwrap();
        let mut host = RuntimeHost::new(&domain, fake_spec(), DarwinPlatform).unwrap();
        let session = attach(&mut host, true);
        submit(&mut host, session, mode, deadline()).unwrap();
        let c = complete(&mut host);
        assert!(matches!(
            c.terminal,
            Terminal::Failed(HostError::WorkerFailed | HostError::InvalidControl)
        ));
        assert!(c.effect_unknown());
        assert_eq!(c.committed(), 0);
        assert_eq!(c.missing(), 1);
        assert_eq!(domain.usage().reserved_sessions, 1);
        assert!(matches!(
            host.reserve_input(session, 1),
            Err(HostError::Busy)
        ));
        assert!(matches!(next(&mut host), HostEvent::Closed { .. }));
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert!(matches!(host.next_event().unwrap(), HostEvent::Pending));
        assert!(matches!(
            host.reserve_input(session, 1),
            Err(HostError::StaleOperation)
        ));
        drop(c);
        finish(&mut host);
    }
}
#[test]
fn readonly_cancel_and_deadline_refuse_before_fake_dispatch() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let domain = HostDomain::new::<DarwinPlatform>(limits()).unwrap();
    let mut host = RuntimeHost::new(&domain, fake_spec(), DarwinPlatform).unwrap();
    let read = attach(&mut host, false);
    assert!(matches!(
        submit(&mut host, read, b'C', deadline()),
        Err(HostError::PermissionDenied)
    ));
    host.detach(read).unwrap();
    assert!(matches!(next(&mut host), HostEvent::Closed { .. }));
    let session = attach(&mut host, true);
    let op = submit(&mut host, session, b'C', deadline()).unwrap();
    let c = host.cancel(op).unwrap();
    assert_eq!(c.effect, EffectReceipt::NotDispatched);
    assert_eq!(c.terminal, Terminal::Cancelled);
    assert!(matches!(next(&mut host), HostEvent::Closed { .. }));
    drop(c);
    let session = attach(&mut host, true);
    submit(
        &mut host,
        session,
        b'C',
        Instant::now() + Duration::from_millis(2),
    )
    .unwrap();
    thread::sleep(Duration::from_millis(3));
    let c = complete(&mut host);
    assert_eq!(c.effect, EffectReceipt::NotDispatched);
    assert_eq!(c.terminal, Terminal::TimedOut);
    assert!(matches!(next(&mut host), HostEvent::Closed { .. }));
    drop(c);
    finish(&mut host);
}
#[test]
fn same_target_mutation_claim_refuses_parallel_dispatch() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let domain = HostDomain::new::<DarwinPlatform>(limits()).unwrap();
    let mut host = RuntimeHost::new(&domain, fake_spec(), DarwinPlatform).unwrap();
    let a = attach(&mut host, true);
    let b = attach(&mut host, true);
    let op = submit(&mut host, a, b'C', deadline()).unwrap();
    assert!(matches!(
        submit(&mut host, b, b'C', deadline()),
        Err(HostError::Busy)
    ));
    drop(host.cancel(op).unwrap());
    assert!(matches!(next(&mut host), HostEvent::Closed { .. }));
    submit(&mut host, b, b'C', deadline()).unwrap();
    let c = complete(&mut host);
    assert_eq!(c.terminal, Terminal::Completed);
    drop(c);
    finish(&mut host);
}

#[test]
fn physical_lane_is_shared_across_targets_and_cancel_after_permit_is_unknown() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    let domain = HostDomain::new::<DarwinPlatform>(limits()).unwrap();
    let mut host = RuntimeHost::new(&domain, fake_spec(), DarwinPlatform).unwrap();
    let a = attach(&mut host, true);
    // Known bounded fixtures only: update every target occurrence consistently.
    let b_descriptor = std::str::from_utf8(DESCRIPTOR)
        .unwrap()
        .replace("native-app", "second-app");
    let b_action = std::str::from_utf8(ACTION)
        .unwrap()
        .replace("native-app", "second-app");
    let Artifact::Session(s) = Document::from_json(b_descriptor.as_bytes(), 65536)
        .unwrap()
        .artifact
    else {
        panic!("session")
    };
    let mut input = host
        .reserve_attach_input(
            TargetLease::authorized(&s.target, true).unwrap(),
            b_descriptor.len(),
        )
        .unwrap();
    input.bytes_mut().copy_from_slice(b_descriptor.as_bytes());
    let b = host.attach(input, deadline()).unwrap();
    assert!(matches!(next(&mut host), HostEvent::Attached { .. }));
    let a_op = submit(&mut host, a, b'P', deadline()).unwrap();
    let mut input = host.reserve_input(b, b_action.len() + 1).unwrap();
    input.bytes_mut()[0] = b'P';
    input.bytes_mut()[1..].copy_from_slice(b_action.as_bytes());
    let b_op = host
        .submit(
            b,
            OperationClass::Mutation,
            input,
            OutputRequest {
                channels: 1,
                frame_bytes: 65536,
                total_bytes: 65536,
                input_format: 0,
                retained_partition: 0,
            },
            deadline(),
        )
        .unwrap();
    let refused = complete(&mut host);
    assert_eq!(refused.terminal, Terminal::Failed(HostError::Busy));
    assert_eq!(refused.effect, EffectReceipt::NotDispatched);
    let permitted = if refused.operation == a_op {
        b_op
    } else {
        assert_eq!(refused.operation, b_op);
        a_op
    };
    let cancelled = host.cancel(permitted).unwrap();
    assert_eq!(cancelled.terminal, Terminal::Cancelled);
    assert!(cancelled.effect_unknown());
    assert_eq!(cancelled.committed(), 0);
    drop(refused);
    drop(cancelled);
    finish(&mut host);
    assert_eq!(domain.usage().reserved_sessions, 0);
}

#[test]
fn action_metadata_requires_typed_admission_effect_state_and_exact_commit_and_survives_loss() {
    use uiblueprint_host::publication::ActionPublicationStatus as Status;
    let _serial = RUNTIME_TEST.lock().unwrap();
    for (mode, class, format, expected) in [
        (b'a', OperationClass::Prepare, 1, Some(Status::Prepared)),
        (
            b'b',
            OperationClass::Mutation,
            1,
            Some(Status::VerifiedSuccess),
        ),
        (
            b'c',
            OperationClass::Mutation,
            1,
            Some(Status::VerifiedMismatch),
        ),
        (b'd', OperationClass::Mutation, 1, Some(Status::Uncertain)),
        (b'e', OperationClass::Prepare, 1, None),
        (b'f', OperationClass::Mutation, 1, None),
        (b'g', OperationClass::Mutation, 1, None),
        (b'h', OperationClass::Mutation, 1, None),
        (b'i', OperationClass::Prepare, 1, None),
        (b'j', OperationClass::Mutation, 0, None),
        (
            b'k',
            OperationClass::Mutation,
            1,
            Some(Status::VerifiedSuccess),
        ),
        (
            b'l',
            OperationClass::Mutation,
            1,
            Some(Status::VerifiedSuccess),
        ),
    ] {
        let domain = HostDomain::new::<DarwinPlatform>(limits()).unwrap();
        let mut host = RuntimeHost::new(&domain, fake_spec(), DarwinPlatform).unwrap();
        let session = attach(&mut host, true);
        let mut input = host.reserve_input(session, ACTION.len() + 1).unwrap();
        input.bytes_mut()[0] = mode;
        input.bytes_mut()[1..].copy_from_slice(ACTION);
        host.submit(
            session,
            class,
            input,
            OutputRequest {
                channels: 1,
                frame_bytes: 65536,
                total_bytes: 65536,
                input_format: format,
                retained_partition: 0,
            },
            deadline(),
        )
        .unwrap();
        let c = complete(&mut host);
        assert_eq!(c.action_status(), expected, "mode {}", mode as char);
        if mode <= b'd' {
            assert_eq!(c.terminal, Terminal::Completed);
        } else {
            assert!(matches!(c.terminal, Terminal::Failed(_)));
        }
        if expected.is_some() {
            assert_eq!(c.bytes(0), Some(ACTION));
        } else {
            assert_eq!(c.committed(), 0);
        }
        if matches!(mode, b'k' | b'l') {
            assert!(
                c.effect_unknown(),
                "tag cannot confirm missing/wrong nonce terminal"
            );
        }
        drop(c);
        finish(&mut host);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert_eq!(domain.usage().completion_groups, 0);
        assert!(!domain.usage().abandoned);
    }
}

#[test]
fn action_refused_terminal_requires_typed_admitted_ack_and_never_follows_possible() {
    use uiblueprint_host::publication::ActionPublicationStatus as Status;
    let _serial = RUNTIME_TEST.lock().unwrap();
    for (mode, class, format, status, accepted) in [
        (
            b'm',
            OperationClass::Mutation,
            1,
            Some(Status::Refused),
            true,
        ),
        (b'n', OperationClass::Prepare, 1, None, true),
        (b'o', OperationClass::Mutation, 1, None, false),
        (b'p', OperationClass::Prepare, 0, None, false),
        (
            b'q',
            OperationClass::Mutation,
            1,
            Some(Status::Uncertain),
            false,
        ),
        (
            b'r',
            OperationClass::Prepare,
            1,
            Some(Status::Prepared),
            false,
        ),
        (
            b's',
            OperationClass::Mutation,
            1,
            Some(Status::Refused),
            false,
        ),
        (b'u', OperationClass::Validate, 0, None, false),
        (b'v', OperationClass::Prepare, 1, None, false),
        (
            b'w',
            OperationClass::Mutation,
            1,
            Some(Status::Refused),
            false,
        ),
    ] {
        let domain = HostDomain::new::<DarwinPlatform>(limits()).unwrap();
        let mut host = RuntimeHost::new(&domain, fake_spec(), DarwinPlatform).unwrap();
        let session = attach(&mut host, true);
        let mut input = host.reserve_input(session, ACTION.len() + 1).unwrap();
        input.bytes_mut()[0] = mode;
        input.bytes_mut()[1..].copy_from_slice(ACTION);
        host.submit(
            session,
            class,
            input,
            OutputRequest {
                channels: 1,
                frame_bytes: 65536,
                total_bytes: 65536,
                input_format: format,
                retained_partition: 0,
            },
            if matches!(mode, b'v' | b'w') {
                Instant::now() + Duration::from_millis(200)
            } else {
                deadline()
            },
        )
        .unwrap();
        let c = complete(&mut host);
        assert_eq!(
            c.terminal,
            if matches!(mode, b'v' | b'w') {
                Terminal::TimedOut
            } else {
                Terminal::Failed(if accepted {
                    HostError::ActionRefused
                } else {
                    HostError::InvalidControl
                })
            },
            "mode {}",
            mode as char
        );
        assert_eq!(c.action_status(), status);
        if accepted {
            assert_eq!(c.bytes(0), Some(ACTION));
            assert_eq!(c.effect, EffectReceipt::NotDispatched);
        }
        if mode == b'q' {
            assert!(c.effect_unknown());
            assert_eq!(c.bytes(0), Some(ACTION));
        }
        if mode == b'o' {
            assert_eq!(c.committed(), 0);
        }
        drop(c);
        finish(&mut host);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert_eq!(domain.usage().completion_groups, 0);
        assert!(!domain.usage().abandoned);
    }
}
