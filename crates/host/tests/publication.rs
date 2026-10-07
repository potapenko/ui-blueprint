use uiblueprint_host::{publication::Publication, *};
const MIB: usize = 1_048_576;
fn limits() -> HostLimits {
    HostLimits {
        workers: 2,
        worker_bytes: 4 * MIB,
        publication_reserve: MIB,
        bootstrap_bytes: MIB,
        parent_bytes: MIB,
        input_bytes: 4096,
        ingress_bytes: 1024,
        output_bytes: 2048,
        request_output_bytes: 6144,
        completion_groups: 2,
        control_bytes: 4096,
        cleanup_ms: 1000,
        retained_domain_bytes: 4 * MIB,
        retained_per_worker: MIB,
        main_stack_bytes: 8 * MIB,
        watchdog_stack_bytes: MIB,
    }
}
fn header(slot: u8, length: u64) -> Control {
    Control {
        kind: ControlKind::Frame,
        class: OperationClass::Observe,
        slot,
        flags: 0,
        correlation: Correlation {
            session_epoch: 1,
            operation: 2,
        },
        length,
        value: 0,
        auxiliary: 0,
    }
}
#[test]
fn bytes_commit_ack_and_terminal_are_distinct_and_prior_results_survive() {
    let pool = ParentBuffers::new(limits(), 0).expect("pool");
    let group = pool.reserve_group(3).expect("reserve all outputs first");
    let mut stream = Publication::new(
        group,
        header(0, 0).correlation,
        OperationClass::Observe,
        3,
        4096,
    )
    .expect("stream");
    let body = b"{\"text\":\"UIBHST01 is payload, not control\"}";
    let first = header(0, body.len() as u64);
    stream.begin(first).expect("frame");
    stream.remaining_mut().expect("body").copy_from_slice(body);
    stream.advance(body.len()).expect("copy complete");
    assert_eq!(stream.committed(), 0);
    let ack = stream
        .commit(Control {
            kind: ControlKind::Commit,
            ..first
        })
        .expect("commit");
    assert_eq!(stream.committed(), 0);
    assert!(stream.begin(header(1, 5)).is_err());
    stream.ack_sent(ack).expect("complete ACK sent");
    assert_eq!(stream.committed(), 1);
    stream.begin(header(1, 5)).expect("later channel");
    stream.remaining_mut().expect("partial")[..2].copy_from_slice(b"xx");
    stream.advance(2).expect("advance");
    stream.terminalize();
    assert!(
        stream
            .commit(Control {
                kind: ControlKind::Commit,
                ..header(1, 5)
            })
            .is_err()
    );
    let completed = stream.finish();
    assert_eq!(completed.committed, 1);
    assert_eq!(completed.missing, 2);
    assert_eq!(completed.frame(0), Some(body.as_slice()));
    assert!(completed.frame(1).is_none());
    let other = pool
        .reserve_group(1)
        .expect("another operation while caller holds completion");
    assert_eq!(pool.usage().leased_groups, 2);
    assert!(matches!(
        pool.reserve_group(1),
        Err(HostError::ResourceLimit)
    ));
    drop(other);
    drop(completed);
    assert_eq!(pool.usage().leased_groups, 0);
}
#[test]
fn wrong_duplicate_partial_late_and_oversized_controls_never_publish() {
    let pool = ParentBuffers::new(limits(), 0).expect("pool");
    let mut stream = Publication::new(
        pool.reserve_group(1).expect("group"),
        header(0, 0).correlation,
        OperationClass::Observe,
        1,
        4,
    )
    .expect("stream");
    assert!(
        stream
            .begin(Control {
                correlation: Correlation {
                    session_epoch: 9,
                    operation: 2
                },
                ..header(0, 3)
            })
            .is_err()
    );
    assert!(stream.begin(header(0, 5)).is_err());
    stream.begin(header(0, 3)).expect("bounded");
    stream.remaining_mut().expect("bytes")[..2].copy_from_slice(b"ok");
    stream.advance(2).expect("partial");
    assert!(
        stream
            .commit(Control {
                kind: ControlKind::Commit,
                ..header(0, 3)
            })
            .is_err()
    );
    assert_eq!(stream.committed(), 0);
    assert!(stream.advance(2).is_err());
    stream.remaining_mut().expect("last")[0] = b'!';
    stream.advance(1).expect("last byte");
    let ack = stream
        .commit(Control {
            kind: ControlKind::Commit,
            ..header(0, 3)
        })
        .expect("commit");
    assert!(
        stream
            .commit(Control {
                kind: ControlKind::Commit,
                ..header(0, 3)
            })
            .is_err()
    );
    stream.terminalize();
    assert!(stream.ack_sent(ack).is_err());
    let ended = stream.finish();
    assert_eq!(ended.committed, 0);
    assert!(ended.frame(0).is_none());
}
