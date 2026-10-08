//! Replay error classification through the real guarded worker; no live adapter.
#![cfg(target_os = "macos")]
#![forbid(unsafe_code)]
use std::{
    path::Path,
    thread,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    HostError, HostLimits, OperationClass,
    authority::TargetLease,
    domain::{HostDomain, SessionHandle},
    host_types::{HostCompletion, HostEvent, OutputRequest, Terminal},
    process::DarwinPlatform,
    process_api::SpawnSpec,
    supervisor::RuntimeHost,
    worker_tape,
};
use uiblueprint_schema::{SchemaVersion, model::*};
const DESCRIPTOR: &[u8] = include_bytes!("../../../fixtures/golden/ENV-CAPABILITY-VALID.json");
const DELTA: &[u8] = include_bytes!("../../../fixtures/golden/ENV-DELTA-VALID.json");
const WRONG_ARTIFACT: &[u8] = include_bytes!("../../../fixtures/golden/ENV-SESSION-INVALID.json");
fn deadline() -> Instant {
    Instant::now() + Duration::from_secs(5)
}
fn next<'a>(host: &mut RuntimeHost<'a, DarwinPlatform>) -> HostEvent<'a> {
    let end = deadline();
    loop {
        assert!(Instant::now() < end, "bounded host progress");
        match host.next_event().unwrap() {
            HostEvent::Pending => thread::sleep(Duration::from_millis(1)),
            event => return event,
        }
    }
}
fn complete<'a>(host: &mut RuntimeHost<'a, DarwinPlatform>) -> HostCompletion<'a> {
    match next(host) {
        HostEvent::Complete(c) => c,
        _ => panic!("completion"),
    }
}
fn submit<'a>(
    host: &mut RuntimeHost<'a, DarwinPlatform>,
    session: SessionHandle<'a>,
    bytes: &[u8],
    replay: bool,
) -> HostCompletion<'a> {
    let id = b"\"S11\"";
    let parts = [bytes, id.as_slice()];
    let length = if replay {
        8 + worker_tape::SEGMENTS * 8 + bytes.len() + id.len()
    } else {
        bytes.len()
    };
    let mut input = host.reserve_input(session, length).unwrap();
    if replay {
        worker_tape::encode(&parts, input.bytes_mut()).unwrap();
    } else {
        input.bytes_mut().copy_from_slice(bytes);
    }
    host.submit(
        session,
        if replay {
            OperationClass::Replay
        } else {
            OperationClass::Retain
        },
        input,
        OutputRequest {
            channels: 1,
            frame_bytes: 65536,
            total_bytes: 65536,
            input_format: 0,
            retained_partition: 1,
        },
        deadline(),
    )
    .unwrap();
    complete(host)
}
fn refused(c: HostCompletion<'_>, expected: HostError, case: &str) {
    assert_eq!(c.terminal, Terminal::Failed(expected), "{case}");
    assert_eq!(c.committed(), 0, "{case}");
    assert!(c.bytes(0).is_none(), "{case}");
}
#[test]
fn replay_recovery_is_reserved_for_delta_compatibility() {
    const MIB: usize = 1_048_576;
    let limits = HostLimits {
        workers: 2,
        worker_bytes: 64 * MIB,
        publication_reserve: MIB,
        bootstrap_bytes: MIB,
        parent_bytes: 32 * MIB,
        input_bytes: 2 * MIB,
        ingress_bytes: 512 * 1024,
        output_bytes: 512 * 1024,
        request_output_bytes: 2 * MIB,
        completion_groups: 2,
        control_bytes: 4096,
        cleanup_ms: 1000,
        retained_domain_bytes: 64 * MIB,
        retained_per_worker: 15 * MIB,
        main_stack_bytes: 8 * MIB,
        watchdog_stack_bytes: MIB,
    };
    let domain = HostDomain::new::<DarwinPlatform>(limits).unwrap();
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
        DarwinPlatform,
    )
    .unwrap();
    let Artifact::Session(descriptor) = Document::from_json(DESCRIPTOR, DESCRIPTOR.len())
        .unwrap()
        .artifact
    else {
        panic!("descriptor")
    };
    let mut input = host
        .reserve_attach_input(
            TargetLease::authorized(&descriptor.target, false).unwrap(),
            DESCRIPTOR.len(),
        )
        .unwrap();
    input.bytes_mut().copy_from_slice(DESCRIPTOR);
    let session = host.attach(input, deadline()).unwrap();
    assert!(matches!(next(&mut host),HostEvent::Attached{session:actual,..} if actual==session));

    // This canonical non-Delta fails semantic validation with IncompatibleContext.
    assert_eq!(
        Document::from_json(WRONG_ARTIFACT, WRONG_ARTIFACT.len()).unwrap_err(),
        uiblueprint_schema::validation::ValidationError::IncompatibleContext
    );
    refused(
        submit(&mut host, session, WRONG_ARTIFACT, true),
        HostError::InvalidInput,
        "invalid session_context is not delta recovery",
    );
    refused(
        submit(&mut host, session, DESCRIPTOR, true),
        HostError::InvalidInput,
        "valid non-Delta",
    );
    refused(
        submit(&mut host, session, b"{", true),
        HostError::InvalidInput,
        "malformed JSON",
    );
    refused(
        submit(&mut host, session, DELTA, true),
        HostError::ResyncRequired,
        "lost base",
    );

    let original = Document::from_json(DELTA, DELTA.len()).unwrap();
    let Artifact::Delta(case) = &original.artifact else {
        panic!("delta")
    };
    let base = serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(case.base.clone())),
    })
    .unwrap();
    let retained = submit(&mut host, session, &base, false);
    assert_eq!(retained.terminal, Terminal::Completed);
    assert_eq!(retained.bytes(0), Some(base.as_slice()));
    for variant in ["scope", "revision", "missing-property", "private-value"] {
        let mut candidate = original.clone();
        let Artifact::Delta(delta) = &mut candidate.artifact else {
            unreachable!()
        };
        let expected = match variant {
            "scope" => {
                delta.update.context.scope_id = Id("other-scope".into());
                HostError::ResyncRequired
            }
            "revision" => {
                delta.update.base_revision -= 1;
                HostError::ResyncRequired
            }
            "missing-property" => {
                delta.update.upsert[0].properties.clear();
                HostError::InvalidInput
            }
            "private-value" => {
                let Property::Requested { sensitivity, .. } =
                    &mut delta.update.upsert[0].properties[0]
                else {
                    panic!("requested")
                };
                *sensitivity = Sensitivity::Sensitive;
                HostError::InvalidInput
            }
            _ => unreachable!(),
        };
        refused(
            submit(
                &mut host,
                session,
                &serde_json::to_vec(&candidate).unwrap(),
                true,
            ),
            expected,
            variant,
        );
        assert_eq!(
            retained.bytes(0),
            Some(base.as_slice()),
            "earlier ACK survives refusal"
        );
    }
    let output = submit(&mut host, session, DELTA, true);
    assert_eq!(output.terminal, Terminal::Completed);
    let expected = serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(case.source_snapshot.clone().unwrap())),
    })
    .unwrap();
    assert_eq!(output.bytes(0), Some(expected.as_slice()));
    drop(output);
    let history = submit(&mut host, session, &base, false);
    assert_eq!(history.terminal, Terminal::Completed);
    assert_eq!(history.bytes(0), Some(base.as_slice()));
    drop(history);
    drop(retained);
    let end = deadline();
    loop {
        assert!(Instant::now() < end, "owned worker cleanup");
        if matches!(host.shutdown().unwrap(), HostEvent::ShutdownComplete) {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert_eq!(domain.usage().completion_groups, 0);
    assert!(!domain.usage().abandoned);
}
