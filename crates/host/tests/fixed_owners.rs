use uiblueprint_host::*;
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
#[test]
fn fixed_owner_inventory_and_lower_caps_are_checked_before_publication() {
    let limits = limits();
    let expected = ParentBuffers::root_bytes() + 256 + 2 * 4096 + 4 * 1024 + 6 * 2048;
    let buffers = ParentBuffers::new(limits, 256).expect("bounded setup");
    assert_eq!(buffers.usage().owned_bytes, expected);
    let mut exact = limits;
    exact.parent_bytes = expected;
    assert!(ParentBuffers::new(exact, 256).is_ok());
    exact.parent_bytes -= 1;
    assert!(matches!(
        ParentBuffers::new(exact, 256),
        Err(HostError::ResourceLimit)
    ));
    let mut bad = limits;
    bad.workers = 5;
    assert_eq!(bad.validate(), Err(HostError::InvalidLimits));
    bad = limits;
    bad.worker_bytes = 64 * MIB;
    bad.publication_reserve = 1;
    assert_eq!(bad.validate(), Err(HostError::InvalidLimits));
    bad = limits;
    bad.retained_per_worker = 64 * MIB;
    assert_eq!(bad.validate(), Err(HostError::InvalidLimits));
}
#[test]
fn returned_frame_lease_does_not_borrow_or_block_other_worker_slots() {
    let buffers = ParentBuffers::new(limits(), 256).expect("pool");
    let before = buffers.usage();
    let mut first = buffers
        .reserve(
            BufferClass::Output {
                group: 0,
                channel: 0,
            },
            4,
        )
        .expect("result");
    first.as_mut_slice().copy_from_slice(b"data");
    assert!(matches!(
        buffers.reserve(
            BufferClass::Output {
                group: 0,
                channel: 0
            },
            4
        ),
        Err(HostError::Busy)
    ));
    let mut independent = buffers
        .reserve(BufferClass::Input { worker: 1 }, 3)
        .expect("B proceeds");
    independent.as_mut_slice().copy_from_slice(b"new");
    assert_eq!(first.as_slice(), b"data");
    assert_eq!(buffers.usage().leased_slots, 2);
    assert_eq!(buffers.usage().owned_bytes, before.owned_bytes);
    drop(first);
    let second = buffers
        .reserve(
            BufferClass::Output {
                group: 0,
                channel: 0,
            },
            4,
        )
        .expect("release");
    assert_eq!(second.as_slice(), &[0; 4]);
    drop(second);
    drop(independent);
    assert_eq!(buffers.usage(), before);
}
#[test]
fn size_and_slot_refusals_do_not_allocate_or_erase_live_bytes() {
    let buffers = ParentBuffers::new(limits(), 0).expect("pool");
    let mut input = buffers
        .reserve(BufferClass::Input { worker: 0 }, 4)
        .expect("input");
    input.as_mut_slice().copy_from_slice(b"keep");
    assert_eq!(input.set_len(4097), Err(HostError::ResourceLimit));
    assert_eq!(input.as_slice(), b"keep");
    assert!(matches!(
        buffers.reserve(BufferClass::Input { worker: 2 }, 0),
        Err(HostError::InvalidInput)
    ));
    assert!(matches!(
        buffers.reserve(
            BufferClass::Output {
                group: 1,
                channel: 1
            },
            2049
        ),
        Err(HostError::ResourceLimit)
    ));
    assert_eq!(buffers.usage().leased_slots, 1);
}
#[test]
fn private_control_encoding_is_fixed_and_does_not_inspect_payload_text() {
    let header = Control {
        kind: ControlKind::Frame,
        class: OperationClass::Measure,
        slot: 2,
        flags: 0,
        correlation: Correlation {
            session_epoch: 7,
            operation: 9,
        },
        length: 512,
        value: 0,
        auxiliary: 0,
    };
    let encoded = header.encode();
    assert_eq!(encoded.len(), 64);
    assert_eq!(Control::decode(&encoded), Ok(header));
    assert!(header.matches(Control {
        kind: ControlKind::Commit,
        ..header
    }));
    assert!(!header.matches(Control {
        correlation: Correlation {
            session_epoch: 8,
            operation: 9
        },
        ..header
    }));
    for position in [0, 12, 56] {
        let mut bad = encoded;
        bad[position] = 255;
        assert_eq!(Control::decode(&bad), Err(HostError::InvalidControl));
    }
    let mut bad = encoded;
    bad[10] = 3;
    assert_eq!(Control::decode(&bad), Err(HostError::InvalidControl));
    let mut bad = encoded;
    bad[8] = 255;
    assert_eq!(Control::decode(&bad), Err(HostError::InvalidControl));
}
