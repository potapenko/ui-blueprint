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
    let descriptor = serde_json::to_vec(&data::descriptor()).expect("fixture descriptor");
    let setup = serde_json::to_vec(&data::setup(peer.url.clone())).expect("trusted fixture config");
    let parts = [descriptor.as_slice(), setup.as_slice()];
    let length = tape_length(&parts);
    let mut input = host
        .reserve_attach_input(
            TargetLease::authorized(&data::target(), false).expect("fixture authority"),
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
