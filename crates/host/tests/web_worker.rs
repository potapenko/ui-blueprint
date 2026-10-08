#![cfg(all(target_os = "macos", feature = "web"))]
//! Synthetic CDP peer + the real guarded worker/parent. No browser or fake ACK.
#[path = "support/web_worker_data.rs"]
mod data;
#[path = "support/web_worker_peer.rs"]
mod peer;
#[path = "support/web_worker_process.rs"]
mod process;
use std::{
    path::Path,
    sync::{Arc, Mutex, atomic::Ordering},
    thread,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    OperationClass,
    authority::TargetLease,
    domain::{HostDomain, SessionHandle},
    host_types::{HostCompletion, HostEvent, OutputRequest, Terminal},
    process_api::SpawnSpec,
    supervisor::RuntimeHost,
    web_config::{WebRef, WebSelection},
    worker_tape,
};
use uiblueprint_schema::model::*;
static SERIAL: Mutex<()> = Mutex::new(());
const QUERY: &[u8] = include_bytes!("../../../fixtures/analysis/query-gap.json");
fn deadline() -> Instant {
    Instant::now() + Duration::from_secs(5)
}
fn next<'a>(host: &mut RuntimeHost<'a, process::Platform>) -> HostEvent<'a> {
    let end = deadline();
    loop {
        assert!(Instant::now() < end, "bounded host event");
        match host.next_event().expect("actual parent progress") {
            HostEvent::Pending => thread::sleep(Duration::from_millis(1)),
            event => return event,
        }
    }
}
fn complete<'a>(host: &mut RuntimeHost<'a, process::Platform>) -> HostCompletion<'a> {
    match next(host) {
        HostEvent::Complete(value) => value,
        _ => panic!("expected terminal completion"),
    }
}
fn stop(host: &mut RuntimeHost<'_, process::Platform>) {
    let end = deadline();
    loop {
        assert!(Instant::now() < end, "owned cleanup deadline");
        if matches!(
            host.shutdown().expect("shutdown"),
            HostEvent::ShutdownComplete
        ) {
            return;
        }
        thread::sleep(Duration::from_millis(1));
    }
}
fn tape_length(parts: &[&[u8]]) -> usize {
    8 + worker_tape::SEGMENTS * 8 + parts.iter().map(|p| p.len()).sum::<usize>()
}
fn attach<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    peer: &peer::Peer,
) -> (SessionHandle<'a>, String) {
    attach_authorized(host, peer, false)
}
fn attach_authorized<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    peer: &peer::Peer,
    mutation: bool,
) -> (SessionHandle<'a>, String) {
    let descriptor = serde_json::to_vec(&data::descriptor()).expect("fixture descriptor");
    let setup = serde_json::to_vec(&data::setup(peer.url.clone())).expect("trusted fixture config");
    let parts = [descriptor.as_slice(), setup.as_slice()];
    let length = tape_length(&parts);
    let mut input = host
        .reserve_attach_input(
            TargetLease::authorized(&data::target(), mutation).expect("fixture authority"),
            length,
        )
        .expect("real attach lease");
    assert_eq!(
        worker_tape::encode(&parts, input.bytes_mut()).expect("existing private tape"),
        length
    );
    let session = host.attach_web(input, deadline()).expect("real Web attach");
    match next(host) {
        HostEvent::Attached {
            session: actual,
            clock,
        } => {
            assert_eq!(session, actual);
            (session, clock.as_str().into())
        }
        HostEvent::Complete(c) => panic!("Web attach failed: {:?}", c.terminal),
        _ => panic!("attached event"),
    }
}
fn output(channels: u8) -> OutputRequest {
    OutputRequest {
        channels,
        frame_bytes: 65536,
        total_bytes: 65536,
        input_format: 0,
        retained_partition: 0,
    }
}

fn action_pair<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    session: SessionHandle<'a>,
    first: &Document,
    request: &Document,
    class: OperationClass,
) -> uiblueprint_host::host_types::OperationHandle<'a> {
    let first = serde_json::to_vec(first).unwrap();
    let second = serde_json::to_vec(request).unwrap();
    let parts = [first.as_slice(), second.as_slice()];
    let length = tape_length(&parts);
    let mut input = host.reserve_input(session, length).unwrap();
    worker_tape::encode(&parts, input.bytes_mut()).unwrap();
    let mut outputs = output(1);
    outputs.input_format = 1;
    host.submit(session, class, input, outputs, deadline())
        .unwrap()
}
fn action_seed(snapshot: &Snapshot, clock: &str) -> Document {
    let node = &snapshot.nodes[0];
    let observation = &snapshot.observations[0];
    let evidence = Evidence {
        observation_id: observation.id.clone(),
        source_namespace: node.key.namespace.clone(),
        provenance: Provenance::Reported,
        method: Id("unresolved-preparation".into()),
        uncertainty: None,
    };
    let action = Action {
        id: Id("set-checkbox".into()),
        context: snapshot.context.clone(),
        backend_ref: BackendRef {
            session_id: snapshot.context.session_id.clone(),
            key: node.key.clone(),
            snapshot_id: snapshot.id.clone(),
            observation_id: observation.id.clone(),
            target: snapshot.context.target.clone(),
            surface: node.surface.clone(),
        },
        intent: Intent::SetChecked { value: true },
        modality: InputModality::Setter,
        input_space: None,
        required_enabled: true,
        authorized_scope: snapshot.context.scope_id.clone(),
        unique_match: false,
        resolution: Resolution {
            evidence,
            writable: Availability::Unknown {
                reason: Id("not_observed".into()),
            },
            value_allowed: Availability::Unknown {
                reason: Id("not_observed".into()),
            },
            available_intents: vec![],
        },
    };
    Document {
        schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
        artifact: Artifact::Request(Box::new(Request {
            clock_domain: Id(clock.into()),
            request_id: Id("prepare-checkbox".into()),
            context: snapshot.context.clone(),
            limits: Limits {
                max_elements: 32,
                max_depth: 8,
                max_output_bytes: 65536,
                deadline_ms: 2000,
            },
            freshness_policy: FreshnessPolicy::CurrentRequired,
            operation: Operation::Prepare { action },
        })),
    }
}

#[test]
fn guarded_prepare_then_act_uses_actual_kernel_bridge_and_preserves_non_success_outcomes() {
    let _serial = SERIAL.lock().unwrap();
    for (mode, expected) in [
        (0, Outcome::Succeeded),
        (1, Outcome::Succeeded),
        (2, Outcome::Failed),
        (3, Outcome::Failed),
        (4, Outcome::ActionOutcomeUnknown),
    ] {
        let peer = peer::Peer::new();
        peer.state.checkbox_native.store(true, Ordering::Release);
        peer.state.checkbox_enabled.store(true, Ordering::Release);
        peer.state.checkbox_writable.store(true, Ordering::Release);
        peer.state
            .checkbox_checked
            .store(mode == 1, Ordering::Release);
        let trace = Arc::new(process::Trace::default());
        let domain = HostDomain::new::<process::Platform>(data::limits()).unwrap();
        let mut host = RuntimeHost::new(
            &domain,
            SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
            process::Platform(trace.clone()),
        )
        .unwrap();
        let (session, clock) = attach_authorized(&mut host, &peer, true);
        let mut observed = data::request(&clock, vec![Channel::ExternalSemantics]);
        let Artifact::Request(request) = &mut observed.artifact else {
            panic!("request")
        };
        request.context.fields = vec![Field::Enabled, Field::Checked];
        submit(
            &mut host,
            session,
            &observed,
            &data::selection(false),
            1,
            deadline(),
        );
        let first = complete(&mut host);
        let first_bytes = first.bytes(0).unwrap().to_vec();
        let doc = Document::from_json(&first_bytes, 65536).unwrap();
        let Artifact::ChannelResponse(response) = doc.artifact else {
            panic!("channel")
        };
        let ChannelResult::Observed(snapshot) = response.result else {
            panic!("snapshot")
        };
        let source = if mode % 2 == 0 {
            Document::from_json(&first_bytes, 65536).unwrap()
        } else {
            Document {
                schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
                artifact: Artifact::Snapshot(snapshot.clone()),
            }
        };
        let seed = action_seed(&snapshot, &clock);
        assert!(Document::from_json(&serde_json::to_vec(&seed).unwrap(), 65536).is_ok());
        action_pair(&mut host, session, &source, &seed, OperationClass::Prepare);
        let prepared = complete(&mut host);
        assert_eq!(prepared.terminal, Terminal::Completed);
        assert_eq!(
            prepared.action_status(),
            Some(uiblueprint_host::publication::ActionPublicationStatus::Prepared)
        );
        assert_eq!(
            prepared.effect,
            uiblueprint_host::host_types::EffectReceipt::NotDispatched
        );
        assert_eq!(trace.effect_permits.load(Ordering::Acquire), 0);
        assert_eq!(peer.state.setter_calls.load(Ordering::Acquire), 0);
        assert_eq!(trace.prepare_acks.load(Ordering::Acquire), 1);
        let prepared_doc = Document::from_json(prepared.bytes(0).unwrap(), 65536).unwrap();
        let Artifact::Action(case) = &prepared_doc.artifact else {
            panic!("prepared case")
        };
        assert!(matches!(
            case.action.resolution.writable,
            Availability::Known {
                value: Value::Flag(true)
            }
        ));
        assert!(case.action.unique_match);
        let mut act = seed.clone();
        let Artifact::Request(request) = &mut act.artifact else {
            panic!("request")
        };
        request.context = case.snapshot.context.clone();
        request.operation = Operation::Act {
            action: case.action.clone(),
        };
        drop(prepared); // Keep old Observe lease; free the second group for Act.
        if mode == 2 {
            peer.state.checkbox_enabled.store(false, Ordering::Release);
        }
        if mode == 3 {
            peer.state.wrong_post_checked.store(true, Ordering::Release);
        }
        if mode == 4 {
            peer.state.lost_post_binding.store(true, Ordering::Release);
        }
        action_pair(
            &mut host,
            session,
            &prepared_doc,
            &act,
            OperationClass::Mutation,
        );
        let result = complete(&mut host);
        assert_eq!(result.committed(), 1);
        let report = Document::from_json(result.bytes(0).unwrap(), 65536).unwrap();
        let Artifact::TransitionContext(report) = report.artifact else {
            panic!("transition")
        };
        assert_eq!(report.transition.steps[0].outcome, expected);
        use uiblueprint_host::publication::ActionPublicationStatus as Status;
        assert_eq!(
            result.action_status(),
            Some(match mode {
                0 | 1 => Status::VerifiedSuccess,
                2 => Status::Refused,
                3 => Status::VerifiedMismatch,
                4 => Status::Uncertain,
                _ => unreachable!(),
            })
        );
        if mode == 2 {
            assert!(matches!(result.terminal, Terminal::Failed(_)));
            assert_eq!(
                result.effect,
                uiblueprint_host::host_types::EffectReceipt::NotDispatched
            );
            assert_eq!(trace.effect_permits.load(Ordering::Acquire), 0);
            assert_eq!(peer.state.setter_calls.load(Ordering::Acquire), 0);
            assert_eq!(
                report.transition.steps[0].delivery,
                DeliveryStatus::NotDispatched
            );
        } else {
            assert_eq!(result.terminal, Terminal::Completed);
            assert!(matches!(
                result.effect,
                uiblueprint_host::host_types::EffectReceipt::Confirmed { .. }
            ));
            assert_eq!(trace.effect_permits.load(Ordering::Acquire), 1);
            assert_eq!(peer.state.setter_calls.load(Ordering::Acquire), 1);
            assert_eq!(
                report.transition.steps[0].delivery,
                DeliveryStatus::Confirmed
            );
        }
        assert_eq!(trace.mutation_acks.load(Ordering::Acquire), 1);
        assert_eq!(
            first.bytes(0).unwrap(),
            first_bytes.as_slice(),
            "old ACK survives new action"
        );
        drop(result);
        drop(first);
        stop(&mut host);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert_eq!(domain.usage().completion_groups, 0);
        assert!(!domain.usage().abandoned);
    }
}

#[test]
fn prepare_rejects_payload_ref_clock_and_small_budget_without_effect_authority() {
    let _serial = SERIAL.lock().unwrap();
    for mode in 0..7 {
        let peer = peer::Peer::new();
        peer.state.checkbox_native.store(true, Ordering::Release);
        peer.state.checkbox_enabled.store(true, Ordering::Release);
        peer.state.checkbox_writable.store(true, Ordering::Release);
        let trace = Arc::new(process::Trace::default());
        let domain = HostDomain::new::<process::Platform>(data::limits()).unwrap();
        let mut host = RuntimeHost::new(
            &domain,
            SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
            process::Platform(trace.clone()),
        )
        .unwrap();
        let (session, clock) = attach(&mut host, &peer); // actual read-only TargetLease
        let mut observed = data::request(&clock, vec![Channel::ExternalSemantics]);
        let Artifact::Request(request) = &mut observed.artifact else {
            panic!("request")
        };
        request.context.fields = vec![Field::Enabled, Field::Checked];
        submit(
            &mut host,
            session,
            &observed,
            &data::selection(false),
            1,
            deadline(),
        );
        let first = complete(&mut host);
        let doc = Document::from_json(first.bytes(0).unwrap(), 65536).unwrap();
        let Artifact::ChannelResponse(response) = doc.artifact else {
            panic!("channel")
        };
        let ChannelResult::Observed(snapshot) = response.result else {
            panic!("snapshot")
        };
        let mut source = Document {
            schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
            artifact: Artifact::Snapshot(snapshot.clone()),
        };
        let mut seed = action_seed(&snapshot, &clock);
        if mode == 6 {
            source = Document::from_json(first.bytes(0).unwrap(), 65536).unwrap();
            let Artifact::ChannelResponse(response) = &mut source.artifact else {
                panic!("channel")
            };
            response.result = ChannelResult::Failed(Issue {
                code: ErrorCode::Unsupported,
                scope_id: snapshot.context.scope_id.clone(),
                failed_step: None,
                recovery_class: Id("reobserve_source".into()),
            });
            source.validate().unwrap();
        }
        let Artifact::Request(request) = &mut seed.artifact else {
            panic!("seed")
        };
        match mode {
            0 => {
                let Operation::Prepare { action } = &request.operation else {
                    panic!("prepare")
                };
                request.operation = Operation::Act {
                    action: action.clone(),
                };
            }
            1 => request.clock_domain = Id("wrong-clock".into()),
            2 => {
                let Operation::Prepare { action } = &mut request.operation else {
                    panic!("prepare")
                };
                action.backend_ref.snapshot_id = Id("wrong-snapshot".into());
            }
            4 => request.limits.max_output_bytes = 64,
            _ => (),
        }
        let calls = peer.state.calls.load(Ordering::Acquire);
        let operation = if mode == 3 {
            action_pair(&mut host, session, &seed, &source, OperationClass::Prepare)
        } else {
            action_pair(&mut host, session, &source, &seed, OperationClass::Prepare)
        };
        let completed = if mode == 5 {
            host.cancel(operation).unwrap()
        } else {
            complete(&mut host)
        };
        assert!(matches!(
            completed.terminal,
            Terminal::Failed(_) | Terminal::Cancelled
        ));
        assert_eq!(
            completed.effect,
            uiblueprint_host::host_types::EffectReceipt::NotDispatched
        );
        assert_eq!(trace.effect_permits.load(Ordering::Acquire), 0);
        assert_eq!(peer.state.setter_calls.load(Ordering::Acquire), 0);
        if mode != 4 {
            assert_eq!(
                peer.state.calls.load(Ordering::Acquire),
                calls,
                "invalid preparation refused before fresh SDK read"
            );
        }
        if mode == 4 || mode == 5 {
            assert_eq!(completed.committed(), 0);
            assert!(completed.bytes(0).is_none());
        } else {
            assert_eq!(completed.committed(), 1);
            Document::from_json(completed.bytes(0).unwrap(), 65536).unwrap();
        }
        drop(completed);
        drop(first);
        stop(&mut host);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert_eq!(domain.usage().completion_groups, 0);
        assert!(!domain.usage().abandoned);
    }
}

#[test]
fn actual_action_cancel_before_ready_and_after_possible_never_dispatches_setter() {
    let _serial = SERIAL.lock().unwrap();
    for before_ready in [true, false] {
        let peer = peer::Peer::new();
        peer.state.checkbox_native.store(true, Ordering::Release);
        peer.state.checkbox_enabled.store(true, Ordering::Release);
        peer.state.checkbox_writable.store(true, Ordering::Release);
        let trace = Arc::new(process::Trace::default());
        let domain = HostDomain::new::<process::Platform>(data::limits()).unwrap();
        let mut host = RuntimeHost::new(
            &domain,
            SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
            process::Platform(trace.clone()),
        )
        .unwrap();
        let (session, clock) = attach_authorized(&mut host, &peer, true);
        let mut observed = data::request(&clock, vec![Channel::ExternalSemantics]);
        let Artifact::Request(request) = &mut observed.artifact else {
            panic!("request")
        };
        request.context.fields = vec![Field::Enabled, Field::Checked];
        submit(
            &mut host,
            session,
            &observed,
            &data::selection(false),
            1,
            deadline(),
        );
        let result = complete(&mut host);
        let doc = Document::from_json(result.bytes(0).unwrap(), 65536).unwrap();
        drop(result);
        let Artifact::ChannelResponse(response) = doc.artifact else {
            panic!("channel")
        };
        let ChannelResult::Observed(snapshot) = response.result else {
            panic!("snapshot")
        };
        let source = Document {
            schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
            artifact: Artifact::Snapshot(snapshot.clone()),
        };
        let mut request = action_seed(&snapshot, &clock);
        action_pair(
            &mut host,
            session,
            &source,
            &request,
            OperationClass::Prepare,
        );
        let ready = complete(&mut host);
        let source = Document::from_json(ready.bytes(0).unwrap(), 65536).unwrap();
        drop(ready);
        let Artifact::Action(case) = &source.artifact else {
            panic!("prepared")
        };
        let Artifact::Request(r) = &mut request.artifact else {
            panic!("request")
        };
        r.context = case.snapshot.context.clone();
        r.operation = Operation::Act {
            action: case.action.clone(),
        };
        trace
            .hold_effect_permit
            .store(!before_ready, Ordering::Release);
        let operation = action_pair(
            &mut host,
            session,
            &source,
            &request,
            OperationClass::Mutation,
        );
        if !before_ready {
            let end = deadline();
            while !trace.effect_waiting.load(Ordering::Acquire) {
                assert!(Instant::now() < end);
                assert!(matches!(host.next_event().unwrap(), HostEvent::Pending));
                thread::sleep(Duration::from_millis(1));
            }
        }
        let cancelled = host.cancel(operation).unwrap();
        assert_eq!(cancelled.terminal, Terminal::Cancelled);
        assert_eq!(cancelled.committed(), 0);
        assert_eq!(cancelled.effect_unknown(), !before_ready);
        assert_eq!(peer.state.setter_calls.load(Ordering::Acquire), 0);
        drop(cancelled);
        stop(&mut host);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert!(!domain.usage().abandoned);
    }
}
fn submit<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    session: SessionHandle<'a>,
    request: &Document,
    selection: &WebSelection,
    channels: u8,
    end: Instant,
) -> uiblueprint_host::host_types::OperationHandle<'a> {
    let request = serde_json::to_vec(request).expect("canonical request");
    let selection = serde_json::to_vec(selection).expect("selection config");
    let parts = [request.as_slice(), selection.as_slice()];
    let length = tape_length(&parts);
    let mut input = host
        .reserve_input(session, length)
        .expect("real input lease");
    assert_eq!(
        worker_tape::encode(&parts, input.bytes_mut()).expect("tape"),
        length
    );
    host.submit_web_observe(session, input, output(channels), end)
        .expect("actual Web submit")
}
fn response(bytes: &[u8]) -> Box<ChannelResponse> {
    let doc = Document::from_json(bytes, 65536)
        .expect("bounded fixture client validation, outside production parent");
    let Artifact::ChannelResponse(value) = doc.artifact else {
        panic!("canonical channel")
    };
    value
}
fn snapshot(response: &ChannelResponse) -> &Snapshot {
    let ChannelResult::Observed(value) = &response.result else {
        panic!("observed channel")
    };
    value
}

#[test]
fn real_begin_permit_ack_and_reusable_first_and_reference_requests() {
    let _serial = SERIAL.lock().expect("test ownership");
    let peer = peer::Peer::new();
    let trace = Arc::new(process::Trace::default());
    let domain = HostDomain::new::<process::Platform>(data::limits()).expect("real guarded domain");
    let spec =
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).expect("actual worker");
    let mut host = RuntimeHost::new(&domain, spec, process::Platform(trace.clone())).expect("host");
    let (session, clock) = attach(&mut host, &peer);
    // Different host operation and canonical observation sequences must remain distinct.
    let mut input = host
        .reserve_input(session, QUERY.len())
        .expect("validation input");
    input.bytes_mut().copy_from_slice(QUERY);
    let mut request = output(1);
    request.input_format = 1;
    host.submit(
        session,
        OperationClass::Validate,
        input,
        request,
        deadline(),
    )
    .expect("ordinary prior operation");
    let validation = complete(&mut host);
    assert_eq!(validation.terminal, Terminal::Completed);
    assert_eq!(validation.bytes(0), Some(QUERY));
    drop(validation);
    let before = peer.state.calls.load(Ordering::Acquire);
    trace.hold_permit.store(true, Ordering::Release);
    let operation = submit(
        &mut host,
        session,
        &data::request(&clock, vec![Channel::ExternalSemantics]),
        &data::selection(false),
        1,
        deadline(),
    );
    let end = deadline();
    while !trace.permit_waiting.load(Ordering::Acquire) {
        assert!(Instant::now() < end, "real ObserveReady reaches parent");
        assert!(matches!(
            host.next_event().expect("permit progress"),
            HostEvent::Pending
        ));
        thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(
        peer.state.calls.load(Ordering::Acquire),
        before,
        "no source work while REAL permit is withheld"
    );
    assert_eq!(trace.permits.load(Ordering::Acquire), 0);
    trace.hold_permit.store(false, Ordering::Release);
    let first = complete(&mut host);
    assert_eq!(first.terminal, Terminal::Completed);
    assert_eq!(first.committed(), 1);
    assert_eq!(trace.permits.load(Ordering::Acquire), 1);
    assert_eq!(trace.observe_acks.load(Ordering::Acquire), 1);
    let observed = response(first.bytes(0).expect("actual committed bytes"));
    assert_eq!(
        observed.dispatch_sequence,
        trace.ticket.load(Ordering::Acquire)
    );
    assert_ne!(
        observed.dispatch_sequence, operation.sequence,
        "not a fabricated host-sequence Ticket"
    );
    let source = snapshot(&observed);
    assert_eq!(source.context.target, data::target());
    assert!(
        source
            .observations
            .iter()
            .all(|o| o.clock_domain.0 == clock)
    );
    assert_eq!(source.nodes.len(), 2);
    let dom = source
        .nodes
        .iter()
        .find(|node| node.key.namespace.0 == "web.dom")
        .expect("source node");
    let dom_observation = source
        .observations
        .iter()
        .find(|o| o.source_namespace.0 == "web.dom")
        .expect("real observation");
    let selection = WebSelection::References {
        nodes: vec![WebRef {
            reference: BackendRef {
                session_id: source.context.session_id.clone(),
                target: source.context.target.clone(),
                surface: dom.surface.clone(),
                key: dom.key.clone(),
                snapshot_id: source.id.clone(),
                observation_id: dom_observation.id.clone(),
            },
            sensitivity: Sensitivity::Public,
        }],
    };
    let old = first.bytes(0).expect("retained first").to_vec();
    let lookups = peer.state.selections.load(Ordering::Acquire);
    submit(
        &mut host,
        session,
        &data::request(&clock, vec![Channel::ExternalSemantics]),
        &selection,
        1,
        deadline(),
    );
    let second = complete(&mut host);
    assert_eq!(second.terminal, Terminal::Completed);
    assert_eq!(
        peer.state.selections.load(Ordering::Acquire),
        lookups,
        "existing ref does not search"
    );
    assert_eq!(first.bytes(0), Some(old.as_slice()));
    assert_eq!(domain.usage().completion_groups, 2);
    assert!(peer.state.releases.load(Ordering::Acquire) >= 2);
    stop(&mut host);
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert_eq!(first.bytes(0), Some(old.as_slice()));
    drop(second);
    drop(first);
    assert_eq!(domain.usage().completion_groups, 0);
    assert!(!domain.usage().abandoned);
}

#[test]
fn real_parent_cancel_during_second_ack_preserves_only_first_committed_channel() {
    let _serial = SERIAL.lock().expect("test ownership");
    let peer = peer::Peer::new();
    let trace = Arc::new(process::Trace::default());
    trace.hold_second_ack.store(true, Ordering::Release);
    let domain = HostDomain::new::<process::Platform>(data::limits()).expect("domain");
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).expect("worker"),
        process::Platform(trace.clone()),
    )
    .expect("host");
    let (session, clock) = attach(&mut host, &peer);
    let operation = submit(
        &mut host,
        session,
        &data::request(
            &clock,
            vec![Channel::ExternalSemantics, Channel::RenderedCapture],
        ),
        &data::selection(false),
        3,
        deadline(),
    );
    let end = deadline();
    while !trace.second_ack_waiting.load(Ordering::Acquire) {
        assert!(Instant::now() < end, "second actual ACK reached");
        assert!(matches!(
            host.next_event().expect("progress"),
            HostEvent::Pending
        ));
        thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(trace.observe_acks.load(Ordering::Acquire), 1);
    let result = host.cancel(operation).expect("real parent cancel");
    assert_eq!(result.terminal, Terminal::Cancelled);
    assert_eq!(result.committed(), 1);
    assert_eq!(result.missing(), 2);
    let bytes = result.bytes(0).expect("previous committed data").to_vec();
    assert!(result.bytes(1).is_none());
    assert_eq!(snapshot(&response(&bytes)).nodes.len(), 2);
    stop(&mut host);
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert_eq!(result.bytes(0), Some(bytes.as_slice()));
    drop(result);
    assert_eq!(domain.usage().completion_groups, 0);
}

#[test]
fn guarded_source_privacy_and_context_refusal_use_real_observation_admission() {
    let _serial = SERIAL.lock().expect("test ownership");
    let peer = peer::Peer::new();
    peer.state.secret.store(true, Ordering::Release);
    let trace = Arc::new(process::Trace::default());
    let domain = HostDomain::new::<process::Platform>(data::limits()).expect("domain");
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).expect("worker"),
        process::Platform(trace.clone()),
    )
    .expect("host");
    let (session, clock) = attach(&mut host, &peer);
    let before = peer.state.calls.load(Ordering::Acquire);
    let mut wrong = data::request(&clock, vec![Channel::ExternalSemantics]);
    let Artifact::Request(request) = &mut wrong.artifact else {
        panic!("request")
    };
    request.context.target.generation = data::id("wrong");
    submit(
        &mut host,
        session,
        &wrong,
        &data::selection(false),
        1,
        deadline(),
    );
    let denied = complete(&mut host);
    assert_eq!(
        denied.terminal,
        Terminal::Failed(uiblueprint_host::HostError::PermissionDenied)
    );
    assert_eq!(denied.committed(), 0);
    assert_eq!(trace.permits.load(Ordering::Acquire), 0);
    assert_eq!(peer.state.calls.load(Ordering::Acquire), before);
    drop(denied);
    submit(
        &mut host,
        session,
        &data::request(&clock, vec![Channel::ExternalSemantics]),
        &data::selection(true),
        1,
        deadline(),
    );
    let private = complete(&mut host);
    assert_eq!(private.terminal, Terminal::Completed);
    let bytes = private.bytes(0).expect("redacted result");
    assert!(
        !std::str::from_utf8(bytes)
            .expect("JSON")
            .contains(peer::CANARY)
    );
    let observed = response(bytes);
    assert!(matches!(
        snapshot(&observed).nodes[0]
            .properties
            .iter()
            .find(|p| p.field() == Field::Value),
        Some(Property::Requested {
            state: Availability::Redacted {},
            ..
        })
    ));
    stop(&mut host);
    assert_eq!(domain.usage().reserved_sessions, 0);
}

#[test]
fn parent_deadline_stops_real_guarded_cdp_wait_and_reaps_only_owned_worker() {
    let _serial = SERIAL.lock().expect("test ownership");
    let peer = peer::Peer::new();
    let trace = Arc::new(process::Trace::default());
    let domain = HostDomain::new::<process::Platform>(data::limits()).expect("domain");
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).expect("worker"),
        process::Platform(trace),
    )
    .expect("host");
    let (session, clock) = attach(&mut host, &peer);
    peer.state.stall.store(true, Ordering::Release);
    submit(
        &mut host,
        session,
        &data::request(&clock, vec![Channel::ExternalSemantics]),
        &data::selection(false),
        1,
        Instant::now() + Duration::from_millis(120),
    );
    let result = complete(&mut host);
    assert_eq!(result.terminal, Terminal::TimedOut);
    assert_eq!(result.committed(), 0);
    assert!(peer.state.stalled.load(Ordering::Acquire));
    stop(&mut host);
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert!(!domain.usage().abandoned);
}

#[test]
fn real_reference_and_document_refusals_preserve_a_previous_committed_snapshot() {
    let _serial = SERIAL.lock().expect("test ownership");
    let peer = peer::Peer::new();
    let trace = Arc::new(process::Trace::default());
    let domain = HostDomain::new::<process::Platform>(data::limits()).expect("domain");
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).expect("worker"),
        process::Platform(trace),
    )
    .expect("host");
    let (session, clock) = attach(&mut host, &peer);
    submit(
        &mut host,
        session,
        &data::request(&clock, vec![Channel::ExternalSemantics]),
        &data::selection(false),
        1,
        deadline(),
    );
    let first = complete(&mut host);
    assert_eq!(first.terminal, Terminal::Completed);
    let saved = first.bytes(0).expect("first source bytes").to_vec();
    let document = response(&saved);
    let s = snapshot(&document);
    let dom = &s.nodes[0];
    let mut reference = BackendRef {
        session_id: s.context.session_id.clone(),
        target: s.context.target.clone(),
        surface: dom.surface.clone(),
        key: dom.key.clone(),
        snapshot_id: s.id.clone(),
        observation_id: s.observations[0].id.clone(),
    };
    reference.surface.generation = data::id("wrong-ref-generation");
    let wrong = WebSelection::References {
        nodes: vec![WebRef {
            reference,
            sensitivity: Sensitivity::Public,
        }],
    };
    let before = peer.state.calls.load(Ordering::Acquire);
    submit(
        &mut host,
        session,
        &data::request(&clock, vec![Channel::ExternalSemantics]),
        &wrong,
        1,
        deadline(),
    );
    let refused = complete(&mut host);
    assert_eq!(
        refused.terminal,
        Terminal::Failed(uiblueprint_host::HostError::ResyncRequired)
    );
    assert_eq!(refused.committed(), 0);
    assert_eq!(
        peer.state.calls.load(Ordering::Acquire),
        before,
        "ref refusal before another source command"
    );
    drop(refused);
    peer.state.drift.store(true, Ordering::Release);
    submit(
        &mut host,
        session,
        &data::request(&clock, vec![Channel::ExternalSemantics]),
        &data::selection(false),
        1,
        deadline(),
    );
    let changed = complete(&mut host);
    assert_eq!(
        changed.terminal,
        Terminal::Failed(uiblueprint_host::HostError::ResyncRequired)
    );
    assert_eq!(changed.committed(), 0);
    assert_eq!(first.bytes(0), Some(saved.as_slice()));
    stop(&mut host);
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert_eq!(first.bytes(0), Some(saved.as_slice()));
}

#[test]
fn selection_configuration_cannot_introduce_endpoint_or_unbounded_bytes() {
    use uiblueprint_host::{HostError, web_config::WebSelection};
    let forbidden=br#"{"selection":"initial","ids":[],"max_visited_nodes":1,"endpoint":"ws://127.0.0.1:1/override"}"#;
    assert!(matches!(
        WebSelection::decode(forbidden, forbidden.len()),
        Err(HostError::InvalidInput)
    ));
    assert!(matches!(
        WebSelection::decode(b"{}", 1),
        Err(HostError::ResourceLimit)
    ));
    assert!(matches!(
        WebSelection::decode(b"[]", 2),
        Err(HostError::InvalidInput)
    ));
}

#[test]
fn retained_history_survives_received_event_or_loss_and_other_session_progresses() {
    let _serial = SERIAL.lock().expect("test ownership");
    for lost in [false, true] {
        let a_peer = peer::Peer::new();
        let b_peer = peer::Peer::new();
        let domain = HostDomain::new::<process::Platform>(data::limits()).unwrap();
        let spec = SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap();
        let mut host = RuntimeHost::new(
            &domain,
            spec,
            process::Platform(Arc::new(process::Trace::default())),
        )
        .unwrap();
        let (a, a_clock) = attach(&mut host, &a_peer);
        // A distinct canonical session uses the same existing trusted attach path.
        let mut b_descriptor = data::descriptor();
        let Artifact::Session(descriptor) = &mut b_descriptor.artifact else {
            unreachable!()
        };
        descriptor.session_id = Id("web-session-b".into());
        let descriptor = serde_json::to_vec(&b_descriptor).unwrap();
        let setup = serde_json::to_vec(&data::setup(b_peer.url.clone())).unwrap();
        let parts = [descriptor.as_slice(), setup.as_slice()];
        let length = tape_length(&parts);
        let mut lease = host
            .reserve_attach_input(
                TargetLease::authorized(&data::target(), false).unwrap(),
                length,
            )
            .unwrap();
        worker_tape::encode(&parts, lease.bytes_mut()).unwrap();
        let b = host.attach_web(lease, deadline()).unwrap();
        let b_clock = match next(&mut host) {
            HostEvent::Attached { session, clock } => {
                assert_eq!(session, b);
                clock.as_str().to_owned()
            }
            _ => panic!("second attached"),
        };
        let a_request = data::request(&a_clock, vec![Channel::ExternalSemantics]);
        let mut b_request = data::request(&b_clock, vec![Channel::ExternalSemantics]);
        let Artifact::Request(request) = &mut b_request.artifact else {
            unreachable!()
        };
        request.context.session_id = Id("web-session-b".into());
        submit(
            &mut host,
            a,
            &a_request,
            &data::selection(false),
            1,
            deadline(),
        );
        let first = complete(&mut host);
        assert_eq!(first.terminal, Terminal::Completed);
        let original = first.bytes(0).unwrap().to_vec();
        let a_response = response(&original);
        let a_snapshot = snapshot(&a_response).clone();
        submit(
            &mut host,
            b,
            &b_request,
            &data::selection(false),
            1,
            deadline(),
        );
        let second = complete(&mut host);
        assert_eq!(second.terminal, Terminal::Completed);
        let b_response = response(second.bytes(0).unwrap());
        let b_snapshot = snapshot(&b_response).clone();
        drop(second);
        let records = [(a, a_snapshot), (b, b_snapshot)];
        let mut historical = Vec::new();
        for (session, snapshot) in &records {
            let bytes = serde_json::to_vec(&Document {
                schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
                artifact: Artifact::Snapshot(Box::new(snapshot.clone())),
            })
            .unwrap();
            let mut input = host.reserve_input(*session, bytes.len()).unwrap();
            input.bytes_mut().copy_from_slice(&bytes);
            let mut out = output(1);
            out.retained_partition = 1;
            host.submit(*session, OperationClass::Retain, input, out, deadline())
                .unwrap();
            let retained = complete(&mut host);
            assert_eq!(retained.terminal, Terminal::Completed);
            assert_eq!(retained.bytes(0), Some(bytes.as_slice()));
            historical.push(bytes);
            drop(retained);
        }
        let b_calls = b_peer.state.calls.load(Ordering::Acquire);
        a_peer
            .state
            .events_once
            .store(if lost { 5 } else { 1 }, Ordering::Release);
        submit(
            &mut host,
            a,
            &a_request,
            &data::selection(false),
            1,
            deadline(),
        );
        let changed = complete(&mut host);
        if lost {
            assert_eq!(
                changed.terminal,
                Terminal::Failed(uiblueprint_host::HostError::ResyncRequired)
            );
            assert_eq!(changed.committed(), 0);
        } else {
            assert_eq!(changed.terminal, Terminal::Completed);
            assert_eq!(changed.committed(), 1);
        }
        assert_eq!(a_peer.state.events_once.load(Ordering::Acquire), 0);
        assert_eq!(b_peer.state.calls.load(Ordering::Acquire), b_calls);
        drop(changed);
        // Explicit duplicate Retain reads original recorded data. It cannot reveal
        // invalidated/current-required state; direct Core cache tests own that proof.
        let calls = [
            a_peer.state.calls.load(Ordering::Acquire),
            b_peer.state.calls.load(Ordering::Acquire),
        ];
        for ((session, _), bytes) in records.iter().zip(&historical) {
            let mut input = host.reserve_input(*session, bytes.len()).unwrap();
            input.bytes_mut().copy_from_slice(bytes);
            let mut out = output(1);
            out.retained_partition = 1;
            host.submit(*session, OperationClass::Retain, input, out, deadline())
                .unwrap();
            let retained = complete(&mut host);
            assert_eq!(retained.terminal, Terminal::Completed);
            assert_eq!(retained.bytes(0), Some(bytes.as_slice()));
            drop(retained);
        }
        assert_eq!(
            [
                a_peer.state.calls.load(Ordering::Acquire),
                b_peer.state.calls.load(Ordering::Acquire)
            ],
            calls,
            "Retain/invalidation does not collect UI"
        );
        submit(
            &mut host,
            b,
            &b_request,
            &data::selection(false),
            1,
            deadline(),
        );
        let independent = complete(&mut host);
        assert_eq!(independent.terminal, Terminal::Completed);
        drop(independent);
        if lost {
            let stopped_calls = a_peer.state.calls.load(Ordering::Acquire);
            submit(
                &mut host,
                a,
                &a_request,
                &data::selection(false),
                1,
                deadline(),
            );
            let refused = complete(&mut host);
            assert_eq!(
                refused.terminal,
                Terminal::Failed(uiblueprint_host::HostError::ResyncRequired)
            );
            assert_eq!(refused.committed(), 0);
            assert_eq!(
                a_peer.state.calls.load(Ordering::Acquire),
                stopped_calls,
                "loss never silently rebinds or recollects"
            );
            drop(refused);
            assert!(host.detach(a).unwrap().is_none());
            assert!(matches!(next(&mut host), HostEvent::Closed { session } if session == a));
            // Synthetic event overflow and an explicitly new transport/worker.
            // This is separate from the actual Chromium TCP-loss scenario.
            let recovery_peer = peer::Peer::new();
            let (recovered, recovered_clock) = attach(&mut host, &recovery_peer);
            let request = data::request(&recovered_clock, vec![Channel::ExternalSemantics]);
            submit(
                &mut host,
                recovered,
                &request,
                &data::selection(false),
                1,
                deadline(),
            );
            let fresh = complete(&mut host);
            assert_eq!(fresh.terminal, Terminal::Completed);
            assert_eq!(fresh.committed(), 1);
            assert_eq!(recovery_peer.state.selections.load(Ordering::Acquire), 1);
            drop(fresh);
            assert!(host.detach(recovered).unwrap().is_none());
            assert!(
                matches!(next(&mut host), HostEvent::Closed { session } if session == recovered)
            );
        }
        assert_eq!(first.bytes(0), Some(original.as_slice()));
        stop(&mut host);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert_eq!(first.bytes(0), Some(original.as_slice()));
        drop(first);
        assert_eq!(domain.usage().completion_groups, 0);
        assert!(!domain.usage().abandoned);
    }
}

fn forms_expected_pair<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    session: SessionHandle<'a>,
    source: &Document,
    request: &Document,
    expected: &Document,
    class: OperationClass,
) {
    let encoded = [
        serde_json::to_vec(source).unwrap(),
        serde_json::to_vec(request).unwrap(),
        serde_json::to_vec(expected).unwrap(),
    ];
    let parts: Vec<&[u8]> = encoded.iter().map(Vec::as_slice).collect();
    let mut input = host.reserve_input(session, tape_length(&parts)).unwrap();
    worker_tape::encode(&parts, input.bytes_mut()).unwrap();
    let mut requested = output(1);
    requested.input_format = 1;
    host.submit(session, class, input, requested, deadline())
        .unwrap();
}
fn forms_seed(
    snapshot: &Snapshot,
    clock: &str,
    intent: Intent,
    field: Field,
    wanted: Value,
) -> (Document, Document) {
    let mut request = action_seed(snapshot, clock);
    let Artifact::Request(r) = &mut request.artifact else {
        panic!("request")
    };
    let Operation::Prepare { action } = &mut r.operation else {
        panic!("prepare")
    };
    action.modality = if matches!(intent, Intent::Type { .. }) {
        InputModality::Keyboard
    } else {
        InputModality::Semantic
    };
    action.intent = intent;
    let expected = Document {
        schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
        artifact: Artifact::Expectation(Box::new(Expectation {
            id: Id("caller-explicit-state".into()),
            scope_id: snapshot.context.scope_id.clone(),
            targets: vec![action.backend_ref.key.clone()],
            rule: Rule::PropertyEquals {
                field,
                expected: wanted,
            },
            applies_when: ContextConditions {
                platform: None,
                input_mode: None,
                text_scale: None,
            },
            expected_from: Id("caller_scenario".into()),
        })),
    };
    request.validate().unwrap();
    expected.validate().unwrap();
    (request, expected)
}
fn forms_act(prepared: &Document, seed: &Document) -> Document {
    let Artifact::Action(case) = &prepared.artifact else {
        panic!("prepared")
    };
    let mut act = seed.clone();
    let Artifact::Request(request) = &mut act.artifact else {
        panic!("request")
    };
    request.context = case.snapshot.context.clone();
    request.operation = Operation::Act {
        action: case.action.clone(),
    };
    act
}
#[test]
fn guarded_focus_then_type_uses_explicit_expected_value_and_truthful_ack_outcomes() {
    use uiblueprint_host::{
        host_types::EffectReceipt, publication::ActionPublicationStatus as Status,
    };
    let _serial = SERIAL.lock().unwrap();
    for (mode, wanted_status, wanted_outcome) in [
        (0, Status::VerifiedSuccess, Outcome::Succeeded),
        (1, Status::VerifiedMismatch, Outcome::Failed),
        (2, Status::Uncertain, Outcome::ActionOutcomeUnknown),
    ] {
        let peer = peer::Peer::new();
        peer.state.text_control.store(true, Ordering::Release);
        *peer.state.text.lock().unwrap() = "prefix".into();
        let trace = Arc::new(process::Trace::default());
        let domain = HostDomain::new::<process::Platform>(data::limits()).unwrap();
        let mut host = RuntimeHost::new(
            &domain,
            SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
            process::Platform(trace.clone()),
        )
        .unwrap();
        let (session, clock) = attach_authorized(&mut host, &peer, true);
        let mut observe = data::request(&clock, vec![Channel::ExternalSemantics]);
        let Artifact::Request(r) = &mut observe.artifact else {
            panic!("request")
        };
        r.context.fields = vec![
            Field::Enabled,
            Field::Focused,
            Field::Value,
            Field::InputKind,
            Field::Readonly,
        ];
        submit(
            &mut host,
            session,
            &observe,
            &data::selection(false),
            1,
            deadline(),
        );
        let observed = complete(&mut host);
        let original = observed.bytes(0).unwrap().to_vec();
        let source = Document::from_json(&original, 65536).unwrap();
        let snapshot = snapshot(&response(&original)).clone();
        let (seed, expected) = forms_seed(
            &snapshot,
            &clock,
            Intent::Focus {},
            Field::Focused,
            Value::Flag(true),
        );
        forms_expected_pair(
            &mut host,
            session,
            &source,
            &seed,
            &expected,
            OperationClass::Prepare,
        );
        let prepared = complete(&mut host);
        assert_eq!(prepared.terminal, Terminal::Completed);
        assert_eq!(prepared.action_status(), Some(Status::Prepared));
        assert_eq!(trace.effect_permits.load(Ordering::Acquire), 0);
        let prepared_doc = Document::from_json(prepared.bytes(0).unwrap(), 65536).unwrap();
        drop(prepared);
        forms_expected_pair(
            &mut host,
            session,
            &prepared_doc,
            &forms_act(&prepared_doc, &seed),
            &expected,
            OperationClass::Mutation,
        );
        let focus = complete(&mut host);
        assert_eq!(focus.action_status(), Some(Status::VerifiedSuccess));
        assert_eq!(focus.terminal, Terminal::Completed);
        assert_eq!(peer.state.focus_calls.load(Ordering::Acquire), 1);
        let Artifact::TransitionContext(focus_report) =
            Document::from_json(focus.bytes(0).unwrap(), 65536)
                .unwrap()
                .artifact
        else {
            panic!("transition")
        };
        let after = focus_report.after.unwrap();
        drop(focus);
        let source = Document {
            schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
            artifact: Artifact::Snapshot(Box::new(after.clone())),
        };
        let (seed, expected) = forms_seed(
            &after,
            &clock,
            Intent::Type {
                text: "suffix".into(),
            },
            Field::Value,
            Value::Text("prefixsuffix".into()),
        );
        forms_expected_pair(
            &mut host,
            session,
            &source,
            &seed,
            &expected,
            OperationClass::Prepare,
        );
        let prepared = complete(&mut host);
        assert_eq!(prepared.action_status(), Some(Status::Prepared));
        let prepared_doc = Document::from_json(prepared.bytes(0).unwrap(), 65536).unwrap();
        drop(prepared);
        peer.state.ignore_type.store(mode == 1, Ordering::Release);
        peer.state
            .lose_focus_after_type
            .store(mode == 2, Ordering::Release);
        forms_expected_pair(
            &mut host,
            session,
            &prepared_doc,
            &forms_act(&prepared_doc, &seed),
            &expected,
            OperationClass::Mutation,
        );
        let typed = complete(&mut host);
        assert_eq!(typed.action_status(), Some(wanted_status));
        assert_eq!(typed.terminal, Terminal::Completed);
        assert!(matches!(typed.effect, EffectReceipt::Confirmed { .. }));
        let Artifact::TransitionContext(report) =
            Document::from_json(typed.bytes(0).unwrap(), 65536)
                .unwrap()
                .artifact
        else {
            panic!("transition")
        };
        assert_eq!(report.transition.steps[0].outcome, wanted_outcome);
        assert_eq!(peer.state.type_calls.load(Ordering::Acquire), 1);
        assert_eq!(
            peer.state.focus_calls.load(Ordering::Acquire),
            1,
            "Type cannot repair focus"
        );
        assert_eq!(trace.effect_permits.load(Ordering::Acquire), 2);
        assert_eq!(trace.mutation_acks.load(Ordering::Acquire), 2);
        assert_eq!(observed.bytes(0), Some(original.as_slice()));
        drop(typed);
        drop(observed);
        stop(&mut host);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert_eq!(domain.usage().completion_groups, 0);
        assert!(!domain.usage().abandoned);
    }
}

#[test]
fn forms_expected_record_binding_and_live_privacy_or_staleness_refuse_before_effect() {
    use uiblueprint_host::host_types::EffectReceipt;
    let _serial = SERIAL.lock().unwrap();
    for mode in 0..9 {
        let peer = peer::Peer::new();
        peer.state.text_control.store(true, Ordering::Release);
        *peer.state.text.lock().unwrap() = "prefix".into();
        let trace = Arc::new(process::Trace::default());
        let domain = HostDomain::new::<process::Platform>(data::limits()).unwrap();
        let mut host = RuntimeHost::new(
            &domain,
            SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
            process::Platform(trace.clone()),
        )
        .unwrap();
        let (session, clock) = attach_authorized(&mut host, &peer, true);
        let mut observe = data::request(&clock, vec![Channel::ExternalSemantics]);
        let Artifact::Request(r) = &mut observe.artifact else {
            panic!("request")
        };
        r.context.fields = vec![
            Field::Enabled,
            Field::Focused,
            Field::Value,
            Field::InputKind,
            Field::Readonly,
        ];
        submit(
            &mut host,
            session,
            &observe,
            &data::selection(false),
            1,
            deadline(),
        );
        let first = complete(&mut host);
        let original = first.bytes(0).unwrap().to_vec();
        let source = Document::from_json(&original, 65536).unwrap();
        let snapshot = snapshot(&response(&original)).clone();
        let (mut seed, mut expected) = forms_seed(
            &snapshot,
            &clock,
            Intent::Focus {},
            Field::Focused,
            Value::Flag(true),
        );
        if mode == 6 {
            let Artifact::Expectation(e) = &mut expected.artifact else {
                panic!("expectation")
            };
            e.rule = Rule::PropertyEquals {
                field: Field::Focused,
                expected: Value::Flag(false),
            };
        }
        if mode == 2 {
            let Artifact::Expectation(e) = &mut expected.artifact else {
                panic!("expectation")
            };
            e.scope_id = Id("outside-scope".into());
        }
        if mode == 3 {
            let Artifact::Request(r) = &mut seed.artifact else {
                panic!("request")
            };
            let Operation::Prepare { action } = &mut r.operation else {
                panic!("prepare")
            };
            action.backend_ref.snapshot_id = Id("stale-snapshot".into());
        }
        let before = peer.state.calls.load(Ordering::Acquire);
        if mode == 7 {
            peer.state.secret.store(true, Ordering::Release);
        }
        if mode == 8 {
            peer.state.stale_node.store(true, Ordering::Release);
        }
        if mode == 0 {
            action_pair(&mut host, session, &source, &seed, OperationClass::Prepare);
        } else {
            forms_expected_pair(
                &mut host,
                session,
                &source,
                &seed,
                if mode == 1 { &source } else { &expected },
                OperationClass::Prepare,
            );
        }
        let prepared = complete(&mut host);
        if mode < 4 || mode == 6 || mode >= 7 {
            assert!(matches!(prepared.terminal, Terminal::Failed(_)));
            assert_eq!(prepared.effect, EffectReceipt::NotDispatched);
            if mode < 4 || mode == 6 {
                assert_eq!(
                    peer.state.calls.load(Ordering::Acquire),
                    before,
                    "invalid third record/ref refused before SDK"
                );
            }
            if mode >= 7 {
                assert_eq!(
                    prepared.terminal,
                    Terminal::Failed(uiblueprint_host::HostError::ActionRefused)
                );
                assert!(
                    !String::from_utf8_lossy(prepared.bytes(0).unwrap()).contains(peer::CANARY)
                );
            }
            drop(prepared);
        } else {
            assert_eq!(prepared.terminal, Terminal::Completed);
            let prepared_doc = Document::from_json(prepared.bytes(0).unwrap(), 65536).unwrap();
            drop(prepared);
            peer.state.secret.store(mode == 4, Ordering::Release);
            peer.state.stale_node.store(mode == 5, Ordering::Release);
            forms_expected_pair(
                &mut host,
                session,
                &prepared_doc,
                &forms_act(&prepared_doc, &seed),
                &expected,
                OperationClass::Mutation,
            );
            let refused = complete(&mut host);
            assert_eq!(refused.effect, EffectReceipt::NotDispatched);
            assert_eq!(
                refused.terminal,
                Terminal::Failed(uiblueprint_host::HostError::ActionRefused)
            );
            assert!(!String::from_utf8_lossy(refused.bytes(0).unwrap()).contains(peer::CANARY));
            drop(refused);
        }
        assert_eq!(trace.effect_permits.load(Ordering::Acquire), 0);
        assert_eq!(peer.state.focus_calls.load(Ordering::Acquire), 0);
        assert_eq!(peer.state.type_calls.load(Ordering::Acquire), 0);
        assert_eq!(first.bytes(0), Some(original.as_slice()));
        drop(first);
        stop(&mut host);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert_eq!(domain.usage().completion_groups, 0);
        assert!(!domain.usage().abandoned);
    }
}
