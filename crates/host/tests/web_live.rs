#![cfg(all(target_os = "macos", feature = "web"))]
//! Explicit opt-in finite live fixture consumer. No compile/default-test side effects.
#[path = "support/web_worker_data.rs"]
mod baseline;
#[path = "support/web_worker_process.rs"]
mod process;
#[path = "support/web_resync.rs"]
mod resync;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    io::{self, BufRead, BufReader, Read, Write},
    path::Path,
    sync::{Arc, atomic::Ordering},
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
    web_config::{WebId, WebRef, WebRootSeed, WebSelection},
    worker_tape,
};
use uiblueprint_schema::model::*;
const MAX_LINE: u64 = 262144;
struct Fixture {
    input: BufReader<io::Stdin>,
    output: io::Stdout,
    sequence: u64,
}
impl Fixture {
    fn new() -> Self {
        Self {
            input: BufReader::new(io::stdin()),
            output: io::stdout(),
            sequence: 0,
        }
    }
    fn call(&mut self, command: &str, payload: Value) -> Value {
        self.sequence += 1;
        let message = json!({"sequence":self.sequence,"command":command,"payload":payload});
        let text = serde_json::to_string(&message).expect("fixture command encoding");
        assert!(text.len() as u64 <= MAX_LINE);
        writeln!(self.output, "@UIB_LIVE {text}").expect("fixture channel");
        self.output.flush().expect("fixture flush");
        let mut line = String::new();
        self.input
            .by_ref()
            .take(MAX_LINE + 1)
            .read_line(&mut line)
            .expect("fixture reply");
        assert!(
            line.ends_with('\n') && line.len() as u64 <= MAX_LINE,
            "bounded fixture reply"
        );
        let mut reply: Value = serde_json::from_str(&line).expect("fixture reply JSON");
        assert_eq!(reply["sequence"].as_u64(), Some(self.sequence));
        assert_eq!(
            reply["ok"], true,
            "fixture/oracle step failed (raw data suppressed)"
        );
        reply["result"].take()
    }
    fn before(&mut self, page: &str, case: &str) {
        self.call("before", json!({"page":page,"case":case}));
    }
    fn check(&mut self, page: &str, case: &str, kind: &str, document: Option<&Document>) {
        self.call(
            "check",
            json!({"page":page,"case":case,"kind":kind,"document":document}),
        );
    }
    fn check_frame(
        &mut self,
        page: &str,
        case: &str,
        kind: &str,
        document: &Document,
        bytes: &[u8],
    ) {
        assert!(bytes.len() <= 65536, "bounded canonical evidence");
        let canonical = std::str::from_utf8(bytes).expect("unchanged canonical UTF-8");
        self.call(
            "check",
            json!({"page":page,"case":case,"kind":kind,"document":document,"canonical":canonical}),
        );
    }
    fn outcome(&mut self, stage: &str, completion: &HostCompletion<'_>) {
        let diagnostic=completion.diagnostic().map(|d|json!({"stage":d.stage as u8,"cause":d.cause as u8,"remote_cleanup":d.remote_cleanup,"send_progress":d.send_progress,"code":d.code,"count":d.count}));
        let effect = match completion.effect {
            uiblueprint_host::host_types::EffectReceipt::NotDispatched => "not_dispatched",
            uiblueprint_host::host_types::EffectReceipt::Possible { .. } => "possible",
            uiblueprint_host::host_types::EffectReceipt::Confirmed { .. } => "confirmed",
        };
        self.call("outcome",json!({"page":"a","stage":stage,"terminal":terminal_code(completion.terminal),"committed":completion.committed(),"missing":completion.missing(),"operation":completion.operation.sequence,"diagnostic":diagnostic,"effect":effect}));
    }
    fn action_check(
        &mut self,
        page: &str,
        case: &str,
        kind: &str,
        completion: Option<&HostCompletion<'_>>,
    ) {
        let document = completion
            .and_then(|c| c.bytes(0))
            .map(|b| Document::from_json(b, 65536).expect("canonical action frame"));
        let canonical = completion
            .and_then(|c| c.bytes(0))
            .map(|b| std::str::from_utf8(b).expect("canonical UTF8"));
        self.call(
            "action_check",
            json!({"page":page,"case":case,"kind":kind,"document":document,"canonical":canonical}),
        );
    }
    fn stimulus(&mut self, action: &str) {
        self.call("stimulus", json!({"page":"a","action":action}));
    }
    fn binding(&mut self, page: &str) -> Binding {
        serde_json::from_value(self.call("binding", json!({"page":page})))
            .expect("owned actual CDP binding")
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Binding {
    endpoint: String,
    target: Identity,
    surface: Identity,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RootDiscovery {
    binding: Binding,
    document_backend_id: u32,
    backend_node_id: u32,
}
fn rooted_selection(attached: &Attached<'_>, discovery: &RootDiscovery) -> WebSelection {
    assert_eq!(attached.binding.target, discovery.binding.target);
    assert_eq!(attached.binding.surface, discovery.binding.surface);
    WebSelection::Rooted {
        root: WebRootSeed {
            session_id: attached.session_id.clone(),
            target: discovery.binding.target.clone(),
            surface: discovery.binding.surface.clone(),
            document_backend_id: discovery.document_backend_id,
            backend_node_id: discovery.backend_node_id,
            sensitivity: Sensitivity::Public,
        },
        max_visited_nodes: 256,
    }
}
struct Attached<'a> {
    handle: SessionHandle<'a>,
    binding: Binding,
    clock: String,
    session_id: Id,
}
fn deadline() -> Instant {
    Instant::now() + Duration::from_secs(5)
}
fn next<'a>(host: &mut RuntimeHost<'a, process::Platform>) -> HostEvent<'a> {
    let end = deadline();
    loop {
        assert!(Instant::now() < end, "bounded real host progress");
        match host.next_event().expect("host event") {
            HostEvent::Pending => thread::sleep(Duration::from_millis(1)),
            event => return event,
        }
    }
}
fn complete<'a>(host: &mut RuntimeHost<'a, process::Platform>) -> HostCompletion<'a> {
    match next(host) {
        HostEvent::Complete(c) => c,
        _ => panic!("expected canonical completion"),
    }
}
fn tape_length(parts: &[&[u8]]) -> usize {
    8 + worker_tape::SEGMENTS * 8 + parts.iter().map(|p| p.len()).sum::<usize>()
}
fn attach<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    binding: Binding,
    number: u32,
) -> Attached<'a> {
    attach_authorized(host, binding, number, false)
}
fn attach_authorized<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    binding: Binding,
    number: u32,
    mutation: bool,
) -> Attached<'a> {
    let session_id = Id(format!("live-session-{number}"));
    let mut descriptor = baseline::descriptor();
    let Artifact::Session(session) = &mut descriptor.artifact else {
        panic!("descriptor")
    };
    session.session_id = session_id.clone();
    session.target = binding.target.clone();
    session.surfaces = vec![binding.surface.clone()];
    session.allowed_scopes = [
        "left",
        "sized",
        "draft",
        "popup",
        "rooted",
        "mutation-child",
        "action-target",
    ]
    .map(|s| Id(format!("f01-{s}")))
    .into();
    session.capabilities[0].reason = Some(Id("bounded-live-source-under-verification".into()));
    let mut setup = baseline::setup(binding.endpoint.clone());
    setup.surface = binding.surface.clone();
    let descriptor = serde_json::to_vec(&descriptor).expect("descriptor bytes");
    let setup = serde_json::to_vec(&setup).expect("setup bytes");
    let parts = [descriptor.as_slice(), setup.as_slice()];
    let length = tape_length(&parts);
    let mut lease = host
        .reserve_attach_input(
            TargetLease::authorized(&binding.target, mutation).expect("owned fixture authority"),
            length,
        )
        .expect("attach lease");
    assert_eq!(
        worker_tape::encode(&parts, lease.bytes_mut()).expect("existing Tape"),
        length
    );
    let handle = host
        .attach_web(lease, deadline())
        .expect("actual guarded attach");
    match next(host) {
        HostEvent::Attached { session, clock } => {
            assert_eq!(session, handle);
            Attached {
                handle,
                binding,
                clock: clock.as_str().into(),
                session_id,
            }
        }
        HostEvent::Complete(c) => panic!("guarded attach failed: {:?}", c.terminal),
        _ => panic!("attach event"),
    }
}
fn initial(id: &str) -> WebSelection {
    let mut selection = baseline::selection(false);
    let WebSelection::Initial {
        ids,
        max_visited_nodes,
    } = &mut selection
    else {
        panic!("initial")
    };
    *ids = vec![WebId {
        id: Id(id.into()),
        sensitivity: Sensitivity::Public,
    }];
    *max_visited_nodes = 256;
    selection
}
fn reference(value: &BackendRef) -> WebSelection {
    WebSelection::References {
        nodes: vec![WebRef {
            reference: value.clone(),
            sensitivity: Sensitivity::Public,
        }],
    }
}
fn observe<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    attached: &Attached<'a>,
    id: &str,
    fields: Vec<Field>,
    selection: WebSelection,
    case: &str,
) -> HostCompletion<'a> {
    let mut doc = baseline::request(&attached.clock, vec![Channel::ExternalSemantics]);
    let Artifact::Request(r) = &mut doc.artifact else {
        panic!("request")
    };
    r.request_id = Id(case.into());
    r.context.session_id = attached.session_id.clone();
    r.context.target = attached.binding.target.clone();
    r.context.surfaces = vec![attached.binding.surface.clone()];
    r.context.scope_id = Id(format!("f01-{id}"));
    r.context.fields = fields;
    r.context.environment_revision = Id(format!("f01-800x600-dpr1-{case}"));
    r.limits.deadline_ms = 250;
    let request = serde_json::to_vec(&doc).expect("request");
    let selection = serde_json::to_vec(&selection).expect("selection");
    let parts = [request.as_slice(), selection.as_slice()];
    let length = tape_length(&parts);
    let mut input = host
        .reserve_input(attached.handle, length)
        .expect("real input lease");
    assert_eq!(
        worker_tape::encode(&parts, input.bytes_mut()).expect("Tape"),
        length
    );
    host.submit_web_observe(
        attached.handle,
        input,
        OutputRequest {
            channels: 1,
            frame_bytes: 65536,
            total_bytes: 65536,
            input_format: 0,
            retained_partition: 0,
        },
        Instant::now() + Duration::from_millis(250),
    )
    .expect("explicit current observe");
    complete(host)
}
fn retain_snapshot<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    attached: &Attached<'a>,
    document: &Document,
) -> HostCompletion<'a> {
    let Artifact::ChannelResponse(response) = &document.artifact else {
        panic!("observed channel")
    };
    let ChannelResult::Observed(snapshot) = &response.result else {
        panic!("observed Snapshot")
    };
    let bytes = serde_json::to_vec(&Document {
        schema_version: document.schema_version,
        artifact: Artifact::Snapshot(snapshot.clone()),
    })
    .expect("original Snapshot bytes");
    let mut input = host
        .reserve_input(attached.handle, bytes.len())
        .expect("retained input");
    input.bytes_mut().copy_from_slice(&bytes);
    host.submit(
        attached.handle,
        OperationClass::Retain,
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
    .expect("explicit Retain");
    let completion = complete(host);
    assert_eq!(completion.terminal, Terminal::Completed);
    assert_eq!(
        completion.bytes(0),
        Some(bytes.as_slice()),
        "original Snapshot/context/time remain byte-equal"
    );
    completion
}
fn prepare_action<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    attached: &Attached<'a>,
    source: &Document,
    value: bool,
) -> HostCompletion<'a> {
    let (snapshot, reference) = match &source.artifact {
        Artifact::ChannelResponse(response) => {
            let ChannelResult::Observed(snapshot) = &response.result else {
                panic!("observed")
            };
            (snapshot.as_ref().clone(), dom_ref(source))
        }
        Artifact::Action(case) => (case.snapshot.clone(), case.action.backend_ref.clone()),
        _ => panic!("actual source Snapshot"),
    };
    let node = snapshot
        .nodes
        .iter()
        .find(|n| n.key == reference.key)
        .unwrap();
    let Property::Requested { evidence, .. } = node
        .properties
        .iter()
        .find(|p| p.field() == Field::Enabled)
        .unwrap()
    else {
        unreachable!()
    };
    let unknown = Availability::Unknown {
        reason: Id("not-prepared".into()),
    };
    let action = Action {
        id: Id(format!("live-checked-{value}")),
        context: snapshot.context.clone(),
        backend_ref: reference,
        intent: Intent::SetChecked { value },
        modality: InputModality::Setter,
        input_space: None,
        required_enabled: true,
        authorized_scope: snapshot.context.scope_id.clone(),
        unique_match: false,
        resolution: Resolution {
            evidence: evidence.clone(),
            writable: unknown.clone(),
            value_allowed: unknown,
            available_intents: vec![],
        },
    };
    let request = Request {
        request_id: Id("prepare-checkbox".into()),
        clock_domain: Id(attached.clock.clone()),
        context: snapshot.context.clone(),
        limits: Limits {
            max_elements: 32,
            max_depth: 8,
            max_output_bytes: 65536,
            deadline_ms: 250,
        },
        freshness_policy: FreshnessPolicy::CurrentRequired,
        operation: Operation::Prepare { action },
    };
    let source = Document {
        schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(snapshot)),
    };
    action_submit(host, attached, OperationClass::Prepare, &source, request)
        .expect("read-only Prepare submit");
    complete(host)
}
fn action_submit<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    attached: &Attached<'a>,
    class: OperationClass,
    source: &Document,
    request: Request,
) -> Result<uiblueprint_host::host_types::OperationHandle<'a>, uiblueprint_host::HostError> {
    let source = serde_json::to_vec(source).unwrap();
    let request = serde_json::to_vec(&Document {
        schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
        artifact: Artifact::Request(Box::new(request)),
    })
    .unwrap();
    let parts = [source.as_slice(), request.as_slice()];
    let length = tape_length(&parts);
    let mut input = host.reserve_input(attached.handle, length)?;
    worker_tape::encode(&parts, input.bytes_mut())?;
    host.submit(
        attached.handle,
        class,
        input,
        OutputRequest {
            channels: 1,
            frame_bytes: 65536,
            total_bytes: 65536,
            input_format: 1,
            retained_partition: 0,
        },
        Instant::now() + Duration::from_millis(250),
    )
}
fn submit_act<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    attached: &Attached<'a>,
    prepared: &Document,
) -> Result<uiblueprint_host::host_types::OperationHandle<'a>, uiblueprint_host::HostError> {
    let Artifact::Action(case) = &prepared.artifact else {
        panic!("real prepared ActionCase")
    };
    let request = Request {
        request_id: Id("act-checkbox".into()),
        clock_domain: Id(attached.clock.clone()),
        context: case.snapshot.context.clone(),
        limits: Limits {
            max_elements: 32,
            max_depth: 8,
            max_output_bytes: 65536,
            deadline_ms: 250,
        },
        freshness_policy: FreshnessPolicy::CurrentRequired,
        operation: Operation::Act {
            action: case.action.clone(),
        },
    };
    action_submit(host, attached, OperationClass::Mutation, prepared, request)
}
fn action_fields() -> Vec<Field> {
    vec![Field::Enabled, Field::Checked, Field::InputKind]
}
fn checkbox_actions(
    host: &mut RuntimeHost<'_, process::Platform>,
    fixture: &mut Fixture,
    trace: &process::Trace,
) {
    use uiblueprint_host::host_types::EffectReceipt;
    let a = attach_authorized(host, fixture.binding("a"), 1, true);
    fixture.before("a", "action-observe");
    let observed = observe(
        host,
        &a,
        "action-target",
        action_fields(),
        initial("action-target"),
        "action-observe",
    );
    fixture.outcome("action-observe", &observed);
    fixture.action_check("a", "action-observe", "observe", Some(&observed));
    let source = decoded(&observed);
    drop(observed);
    fixture.before("a", "action-prepare");
    let prepared = prepare_action(host, &a, &source, true);
    fixture.outcome("action-prepare", &prepared);
    assert_eq!(prepared.effect, EffectReceipt::NotDispatched);
    fixture.action_check("a", "action-prepare", "prepare", Some(&prepared));
    let case = decoded(&prepared);
    drop(prepared);
    fixture.before("a", "action-success");
    submit_act(host, &a, &case).unwrap();
    let acted = complete(host);
    fixture.outcome("action-success", &acted);
    assert_eq!(acted.terminal, Terminal::Completed);
    assert!(matches!(acted.effect, EffectReceipt::Confirmed { .. }));
    fixture.action_check("a", "action-success", "success", Some(&acted));
    drop(acted);
    assert_eq!(trace.effect_permits.load(Ordering::Acquire), 1);
    // Prepare the old exact node, then replace it in separate fixture setup.
    fixture.before("a", "action-prepare-stale");
    let stale = prepare_action(host, &a, &case, false);
    fixture.outcome("action-prepare-stale", &stale);
    fixture.action_check("a", "action-prepare-stale", "prepare", Some(&stale));
    let stale_case = decoded(&stale);
    drop(stale);
    fixture.stimulus("action-remount");
    fixture.before("a", "action-stale");
    submit_act(host, &a, &stale_case).unwrap();
    let refused = complete(host);
    fixture.outcome("action-stale", &refused);
    assert_eq!(refused.effect, EffectReceipt::NotDispatched);
    assert_ne!(refused.terminal, Terminal::Completed);
    fixture.action_check("a", "action-stale", "refused", Some(&refused));
    drop(refused);
    detach(host, &a);
    let b = attach(host, fixture.binding("b"), 2);
    fixture.before("b", "action-readonly-observe");
    let observed = observe(
        host,
        &b,
        "action-target",
        action_fields(),
        initial("action-target"),
        "action-readonly-observe",
    );
    fixture.outcome("action-readonly-observe", &observed);
    fixture.action_check("b", "action-readonly-observe", "observe", Some(&observed));
    let source = decoded(&observed);
    drop(observed);
    fixture.before("b", "action-readonly-prepare");
    let prepared = prepare_action(host, &b, &source, true);
    fixture.outcome("action-readonly-prepare", &prepared);
    fixture.action_check("b", "action-readonly-prepare", "prepare", Some(&prepared));
    let case = decoded(&prepared);
    drop(prepared);
    fixture.before("b", "action-readonly-refused");
    assert_eq!(
        submit_act(host, &b, &case).unwrap_err(),
        uiblueprint_host::HostError::PermissionDenied
    );
    fixture.action_check("b", "action-readonly-refused", "readonly_refused", None);
    detach(host, &b);
    let c = attach_authorized(host, fixture.binding("b"), 3, true);
    fixture.before("b", "action-unknown-observe");
    let observed = observe(
        host,
        &c,
        "action-target",
        action_fields(),
        initial("action-target"),
        "action-unknown-observe",
    );
    fixture.outcome("action-unknown-observe", &observed);
    fixture.action_check("b", "action-unknown-observe", "observe", Some(&observed));
    let source = decoded(&observed);
    drop(observed);
    fixture.before("b", "action-unknown-prepare");
    let prepared = prepare_action(host, &c, &source, true);
    fixture.outcome("action-unknown-prepare", &prepared);
    fixture.action_check("b", "action-unknown-prepare", "prepare", Some(&prepared));
    let case = decoded(&prepared);
    drop(prepared);
    let permits = trace.effect_permits.load(Ordering::Acquire);
    assert_eq!(
        permits, 1,
        "readonly/stale cases never grant another permit"
    );
    trace.hold_effect_permit.store(true, Ordering::Release);
    fixture.before("b", "action-unknown");
    let operation = submit_act(host, &c, &case).unwrap();
    let end = Instant::now() + Duration::from_millis(250);
    while !trace.effect_waiting.load(Ordering::Acquire) {
        assert!(Instant::now() < end, "actual permit barrier");
        match host.next_event().unwrap() {
            HostEvent::Pending => thread::yield_now(),
            HostEvent::Complete(completion) => {
                fixture.outcome("action-unknown", &completion);
                panic!("action ended before the genuine effect permit barrier");
            }
            _ => panic!("unexpected event before effect permit"),
        }
    }
    let completion = host.cancel(operation).unwrap();
    fixture.outcome("action-unknown", &completion);
    assert!(matches!(completion.effect, EffectReceipt::Possible { .. }));
    assert!(completion.effect_unknown());
    assert_eq!(completion.committed(), 0);
    assert_eq!(
        trace.effect_permits.load(Ordering::Acquire),
        permits,
        "permit never forwarded; no fabricated ACK or retry"
    );
    fixture.action_check("b", "action-unknown", "possible_before_delivery", None);
    drop(completion);
    trace.hold_effect_permit.store(false, Ordering::Release);
}
fn terminal_code(terminal: Terminal) -> &'static str {
    use uiblueprint_host::HostError as E;
    match terminal {
        Terminal::Completed => "completed",
        Terminal::Cancelled => "cancelled",
        Terminal::TimedOut => "timed_out",
        Terminal::Failed(error) => match error {
            E::InvalidLimits => "invalid_limits",
            E::ResourceLimit => "resource_limit",
            E::AllocationFailure => "allocation_failure",
            E::Overflow => "overflow",
            E::Busy => "busy",
            E::InvalidInput => "invalid_input",
            E::InvalidState => "invalid_state",
            E::InvalidControl => "invalid_control",
            E::StaleOperation => "stale_operation",
            E::DeadlineExpired => "deadline_expired",
            E::PermissionDenied => "permission_denied",
            E::Io => "io",
            E::WorkerFailed => "worker_failed",
            E::SystemAllocationFailure => "system_allocation_failure",
            E::CleanupPending => "cleanup_pending",
            E::ResyncRequired => "resync_required",
            E::ActionRefused => "action_refused",
        },
    }
}
fn decoded(c: &HostCompletion<'_>) -> Document {
    assert_eq!(
        c.terminal,
        Terminal::Completed,
        "real observed completion required"
    );
    assert_eq!(c.committed(), 1);
    Document::from_json(c.bytes(0).expect("ACKed bytes"), 65536)
        .expect("bounded fixture-client decode")
}
fn dom_ref(document: &Document) -> BackendRef {
    let Artifact::ChannelResponse(response) = &document.artifact else {
        panic!("channel")
    };
    let ChannelResult::Observed(snapshot) = &response.result else {
        panic!("observed")
    };
    let node = snapshot
        .nodes
        .iter()
        .find(|n| n.key.namespace.0 == "web.dom")
        .expect("DOM source");
    let observation = snapshot
        .observations
        .iter()
        .find(|o| o.source_namespace.0 == "web.dom")
        .expect("source observation");
    BackendRef {
        session_id: snapshot.context.session_id.clone(),
        key: node.key.clone(),
        snapshot_id: snapshot.id.clone(),
        observation_id: observation.id.clone(),
        target: snapshot.context.target.clone(),
        surface: node.surface.clone(),
    }
}
fn refused(c: &HostCompletion<'_>) {
    assert_ne!(
        c.terminal,
        Terminal::Completed,
        "stale/wrong identity cannot complete"
    );
    assert_eq!(c.committed(), 0);
    assert!(c.bytes(0).is_none());
}
fn detach<'a>(host: &mut RuntimeHost<'a, process::Platform>, attached: &Attached<'a>) {
    assert!(host.detach(attached.handle).expect("detach").is_none());
    loop {
        if let HostEvent::Closed { session } = next(host) {
            assert_eq!(session, attached.handle);
            break;
        }
    }
}
fn shutdown(host: &mut RuntimeHost<'_, process::Platform>) {
    let end = deadline();
    loop {
        assert!(Instant::now() < end, "owned worker cleanup");
        if matches!(
            host.shutdown().expect("shutdown"),
            HostEvent::ShutdownComplete
        ) {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
}
fn left_fields() -> Vec<Field> {
    vec![Field::Role, Field::AccessibilityName, Field::LayoutBounds]
}

#[test]
#[ignore = "requires explicit root W01-live runtime activation and owned launcher"]
fn guarded_live_f01() {
    assert_eq!(
        std::env::var("UIB_WEB_LIVE_ALLOW").as_deref(),
        Ok("1"),
        "runtime not activated"
    );
    assert_eq!(
        std::fs::canonicalize(std::env::var("UIB_WEB_LIVE_WORKER").expect("pinned worker path"))
            .expect("worker exists"),
        std::fs::canonicalize(env!("CARGO_BIN_EXE_session-worker")).expect("compiled worker"),
        "test must use the pinned worker artifact"
    );
    let mut fixture = Fixture::new();
    let trace = Arc::new(process::Trace::default());
    let domain =
        HostDomain::new::<process::Platform>(baseline::limits()).expect("real guard domain");
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).expect("built worker"),
        process::Platform(trace.clone()),
    )
    .expect("real RuntimeHost");
    let diagnostic =
        std::env::var("UIB_WEB_LIVE_CASE").is_ok_and(|v| v == "first_observe_diagnostic");
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if std::env::var("UIB_WEB_LIVE_CASE").as_deref() == Ok("form_reads") {
            let a = attach(&mut host, fixture.binding("a"), 1);
            for case in [
                "form-forward",
                "form-backward",
                "form-collapsed",
                "form-private",
            ] {
                // Explicit fixture setup is outside the product observation and
                // does not qualify Focus/Type delivery or application commit.
                fixture.stimulus(case);
                let mut selection = initial("draft");
                let WebSelection::Initial { ids, .. } = &mut selection else {
                    unreachable!("initial selection")
                };
                ids.push(WebId {
                    id: Id("applied".into()),
                    sensitivity: Sensitivity::Public,
                });
                fixture.before("a", case);
                let response = observe(
                    &mut host,
                    &a,
                    "draft",
                    vec![Field::Focused, Field::Value],
                    selection,
                    case,
                );
                fixture.outcome(case, &response);
                let document = decoded(&response);
                document.validate().expect("canonical form observation");
                assert_eq!(response.missing(), 0);
                assert!(matches!(
                    response.effect,
                    uiblueprint_host::host_types::EffectReceipt::NotDispatched
                ));
                if case == "form-private" {
                    fixture.check("a", case, "form_reads", Some(&document));
                } else {
                    fixture.check_frame(
                        "a",
                        case,
                        "form_reads",
                        &document,
                        response.bytes(0).expect("ACKed form observation"),
                    );
                }
                drop(response);
            }
            return;
        }
        if std::env::var("UIB_WEB_LIVE_CASE").as_deref() == Ok("actions") {
            checkbox_actions(&mut host, &mut fixture, &trace);
            return;
        }
        if std::env::var("UIB_WEB_LIVE_CASE").as_deref() == Ok("resync") {
            resync::run(&mut host, &mut fixture);
            return;
        }
        if std::env::var("UIB_WEB_LIVE_CASE").as_deref() == Ok("b05") {
            let a = attach(&mut host, fixture.binding("a"), 1);
            fixture.before("a", "b05-initial");
            let first = observe(
                &mut host,
                &a,
                "mutation-child",
                vec![Field::LayoutBounds],
                initial("mutation-child"),
                "b05-initial",
            );
            fixture.outcome("b05-initial", &first);
            let original = decoded(&first);
            let original_ref = dom_ref(&original);
            fixture.check_frame(
                "a",
                "b05-initial",
                "b05_initial",
                &original,
                first.bytes(0).expect("ACKed original"),
            );
            let first_bytes = first.bytes(0).unwrap().to_vec();
            fixture.before("a", "b05-retain-initial");
            let retained = retain_snapshot(&mut host, &a, &original);
            fixture.outcome("b05-retain-initial", &retained);
            fixture.check("a", "b05-retain-initial", "recorded_history", None);
            drop(retained);
            for (action, case, kind, history) in [
                (
                    "parentWide",
                    "b05-parent",
                    "b05_parent",
                    "b05-history-parent",
                ),
                ("fontLarge", "b05-font", "b05_font", "b05-history-font"),
            ] {
                fixture.stimulus(action);
                fixture.before("a", case);
                let current = observe(
                    &mut host,
                    &a,
                    "mutation-child",
                    vec![Field::LayoutBounds],
                    reference(&original_ref),
                    case,
                );
                fixture.outcome(case, &current);
                let document = decoded(&current);
                fixture.check_frame(
                    "a",
                    case,
                    kind,
                    &document,
                    current.bytes(0).expect("ACKed current"),
                );
                drop(current);
                fixture.before("a", history);
                let retained = retain_snapshot(&mut host, &a, &original);
                fixture.outcome(history, &retained);
                fixture.check("a", history, "recorded_history", None);
                drop(retained);
                assert_eq!(first.bytes(0).unwrap(), first_bytes);
            }
            drop(first);
            return;
        }
        if std::env::var("UIB_WEB_LIVE_CASE").as_deref() == Ok("rooted") {
            fixture.stimulus("popup");
            let discovery: RootDiscovery =
                serde_json::from_value(fixture.call("root", json!({"page":"a"})))
                    .expect("actual fixture root");
            let a = attach(&mut host, fixture.binding("a"), 1);
            fixture.before("a", "rooted-current");
            let first = observe(
                &mut host,
                &a,
                "rooted",
                vec![
                    Field::Role,
                    Field::AccessibilityName,
                    Field::LayoutBounds,
                    Field::Focused,
                    Field::Expanded,
                ],
                rooted_selection(&a, &discovery),
                "rooted-current",
            );
            fixture.outcome("rooted-current", &first);
            let document = decoded(&first);
            let issued = dom_ref(&document);
            assert_eq!(issued.key.key.0, discovery.backend_node_id.to_string());
            assert_eq!(issued.session_id, a.session_id);
            assert_eq!(issued.surface, discovery.binding.surface);
            fixture.check_frame(
                "a",
                "rooted-current",
                "rooted",
                &document,
                first.bytes(0).expect("ACKed rooted frame"),
            );
            let preserved = first.bytes(0).unwrap().to_vec();
            for case in ["rooted-wrong-binding", "rooted-wrong-document"] {
                let mut selected = rooted_selection(&a, &discovery);
                let WebSelection::Rooted { root, .. } = &mut selected else {
                    unreachable!()
                };
                if case == "rooted-wrong-binding" {
                    root.surface.generation = Id("wrong-fixture-generation".into());
                } else {
                    root.document_backend_id = if root.document_backend_id == 1 { 2 } else { 1 };
                }
                fixture.before("a", case);
                let response = observe(&mut host, &a, "rooted", left_fields(), selected, case);
                fixture.outcome(case, &response);
                assert_eq!(terminal_code(response.terminal), "resync_required");
                assert_eq!(response.committed(), 0);
                fixture.check("a", case, "refused", None);
                drop(response);
            }
            fixture.stimulus("root-remount");
            fixture.before("a", "rooted-stale");
            let response = observe(
                &mut host,
                &a,
                "rooted",
                left_fields(),
                rooted_selection(&a, &discovery),
                "rooted-stale",
            );
            fixture.outcome("rooted-stale", &response);
            assert_eq!(terminal_code(response.terminal), "resync_required");
            assert_eq!(response.committed(), 0);
            fixture.check("a", "rooted-stale", "refused", None);
            drop(response);
            assert_eq!(first.bytes(0).unwrap(), preserved);
            drop(first);
            return;
        }
        if std::env::var("UIB_WEB_LIVE_CASE").as_deref() == Ok("popup_relations") {
            fixture.stimulus("popup");
            let a = attach(&mut host, fixture.binding("a"), 1);
            let selection = WebSelection::Initial {
                ids: [
                    "open-popup",
                    "portal",
                    "close-popup",
                    "draft",
                    "suggestions",
                ]
                .into_iter()
                .map(|id| WebId {
                    id: Id(id.into()),
                    sensitivity: Sensitivity::Public,
                })
                .collect(),
                max_visited_nodes: 256,
            };
            fixture.before("a", "popup-context");
            let response = observe(
                &mut host,
                &a,
                "popup",
                vec![
                    Field::Role,
                    Field::AccessibilityName,
                    Field::LayoutBounds,
                    Field::Focused,
                    Field::Expanded,
                    Field::Value,
                    Field::InputKind,
                    Field::HitRegion,
                ],
                selection,
                "popup-context",
            );
            fixture.outcome("popup-context", &response);
            let document = decoded(&response);
            fixture.check_frame(
                "a",
                "popup-context",
                "popup",
                &document,
                response.bytes(0).expect("ACKed popup context"),
            );
            drop(response);
            return;
        }
        let a = attach(&mut host, fixture.binding("a"), 1);
        fixture.before("a", "left-initial");
        let first = observe(
            &mut host,
            &a,
            "left",
            left_fields(),
            initial("left"),
            "left-initial",
        );
        fixture.outcome("left-initial", &first);
        let first_doc = decoded(&first);
        let left = dom_ref(&first_doc);
        fixture.check_frame(
            "a",
            "left-initial",
            "left",
            &first_doc,
            first.bytes(0).expect("ACKed first bytes"),
        );
        let first_bytes = first.bytes(0).expect("held first").to_vec();
        if diagnostic {
            drop(first);
            return;
        }
        fixture.before("a", "sized-before");
        let sized = observe(
            &mut host,
            &a,
            "sized",
            vec![Field::LayoutBounds],
            initial("sized"),
            "sized-before",
        );
        fixture.outcome("sized-before", &sized);
        let sized_doc = decoded(&sized);
        let sized_ref = dom_ref(&sized_doc);
        fixture.check_frame(
            "a",
            "sized-before",
            "sized_before",
            &sized_doc,
            sized.bytes(0).expect("ACKed sized bytes"),
        );
        drop(sized);
        fixture.stimulus("textLarge");
        fixture.before("a", "sized-after");
        let sized = observe(
            &mut host,
            &a,
            "sized",
            vec![Field::LayoutBounds],
            reference(&sized_ref),
            "sized-after",
        );
        fixture.outcome("sized-after", &sized);
        let doc = decoded(&sized);
        fixture.check_frame(
            "a",
            "sized-after",
            "sized_after",
            &doc,
            sized.bytes(0).expect("ACKed changed bytes"),
        );
        assert_ne!(dom_ref(&doc).snapshot_id, sized_ref.snapshot_id);
        drop(sized);
        fixture.stimulus("private");
        fixture.before("a", "private");
        let private = observe(
            &mut host,
            &a,
            "draft",
            vec![Field::Value, Field::AccessibilityName],
            initial("draft"),
            "private",
        );
        fixture.outcome("private", &private);
        let doc = decoded(&private);
        assert!(
            !std::str::from_utf8(private.bytes(0).expect("bytes"))
                .expect("JSON")
                .contains("W01_LIVE_PRIVATE_CANARY"),
            "privacy canary escaped"
        );
        fixture.check("a", "private", "private", Some(&doc));
        drop(private);
        let b = attach(&mut host, fixture.binding("b"), 2);
        assert_ne!(a.binding.target.id, b.binding.target.id);
        fixture.before("b", "cross-target");
        let wrong = observe(
            &mut host,
            &b,
            "left",
            left_fields(),
            reference(&left),
            "cross-target",
        );
        fixture.outcome("cross-target", &wrong);
        refused(&wrong);
        fixture.check("b", "cross-target", "refused", None);
        drop(wrong);
        detach(&mut host, &b);
        fixture.call(
            "browser_alive",
            json!({"page":"b","case":"other-tab-survives-worker-detach"}),
        );
        fixture.stimulus("remount");
        fixture.before("a", "old-remount-ref");
        let stale = observe(
            &mut host,
            &a,
            "left",
            left_fields(),
            reference(&left),
            "old-remount-ref",
        );
        fixture.outcome("old-remount-ref", &stale);
        refused(&stale);
        fixture.check("a", "old-remount-ref", "refused", None);
        drop(stale);
        assert_eq!(first.bytes(0), Some(first_bytes.as_slice()));
        detach(&mut host, &a);
        let a = attach(&mut host, fixture.binding("a"), 3);
        fixture.before("a", "new-remount-binding");
        let fresh = observe(
            &mut host,
            &a,
            "left",
            left_fields(),
            initial("left"),
            "new-remount-binding",
        );
        fixture.outcome("new-remount-binding", &fresh);
        let fresh_doc = decoded(&fresh);
        let fresh_ref = dom_ref(&fresh_doc);
        assert_ne!(
            fresh_ref.key, left.key,
            "actual remount must change backend identity"
        );
        fixture.check("a", "new-remount-binding", "left", Some(&fresh_doc));
        drop(fresh);
        fixture.stimulus("navigate");
        fixture.before("a", "old-navigation-ref");
        let stale = observe(
            &mut host,
            &a,
            "left",
            left_fields(),
            reference(&fresh_ref),
            "old-navigation-ref",
        );
        fixture.outcome("old-navigation-ref", &stale);
        refused(&stale);
        fixture.check("a", "old-navigation-ref", "refused", None);
        drop(stale);
        detach(&mut host, &a);
        let binding = fixture.binding("a");
        assert_ne!(binding.surface.generation, fresh_ref.surface.generation);
        let a = attach(&mut host, binding, 4);
        fixture.before("a", "new-document-binding");
        let fresh = observe(
            &mut host,
            &a,
            "left",
            left_fields(),
            initial("left"),
            "new-document-binding",
        );
        fixture.outcome("new-document-binding", &fresh);
        let doc = decoded(&fresh);
        fixture.check("a", "new-document-binding", "left", Some(&doc));
        drop(fresh);
        shutdown(&mut host);
        assert_eq!(domain.usage().reserved_sessions, 0);
        assert!(!domain.usage().abandoned);
        assert_eq!(first.bytes(0), Some(first_bytes.as_slice()));
        drop(first);
        assert_eq!(domain.usage().completion_groups, 0);
    }));
    // Keep the actual host alive outside the case's unwind, so even a failed
    // assertion explicitly drives shutdown/reap and reports real domain state.
    let cleanup = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| shutdown(&mut host)));
    let usage = domain.usage();
    let confirmed = cleanup.is_ok() && usage.reserved_sessions == 0 && !usage.abandoned;
    fixture.call("worker_cleanup",json!({"page":"a","confirmed":confirmed,"reserved_sessions":usage.reserved_sessions,"completion_groups":usage.completion_groups,"abandoned":usage.abandoned}));
    if confirmed {
        fixture.call(
            "browser_alive",
            json!({"page":"a","case":"fixture-survives-all-worker-reaps"}),
        );
    }
    assert!(confirmed, "actual worker cleanup unconfirmed");
    if let Err(panic) = result {
        std::panic::resume_unwind(panic);
    }
}
