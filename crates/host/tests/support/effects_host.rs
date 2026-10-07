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
fn attach<'a>(host: &mut RuntimeHost<'a, DarwinPlatform>, mutation: bool) -> SessionHandle<'a> {
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
fn submit<'a>(
    host: &mut RuntimeHost<'a, DarwinPlatform>,
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
fn finish(host: &mut RuntimeHost<'_, DarwinPlatform>) {
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
