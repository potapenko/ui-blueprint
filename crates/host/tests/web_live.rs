#![cfg(all(target_os = "macos", feature = "web"))]
//! Explicit opt-in finite live fixture consumer. No compile/default-test side effects.
#[path = "support/web_worker_data.rs"]
mod baseline;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    io::{self, BufRead, BufReader, Read, Write},
    path::Path,
    thread,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    authority::TargetLease,
    domain::{HostDomain, SessionHandle},
    host_types::{HostCompletion, HostEvent, OutputRequest, Terminal},
    process::DarwinPlatform,
    process_api::SpawnSpec,
    supervisor::RuntimeHost,
    web_config::{WebId, WebRef, WebSelection},
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
        self.call("outcome",json!({"page":"a","stage":stage,"terminal":terminal_code(completion.terminal),"committed":completion.committed(),"missing":completion.missing(),"operation":completion.operation.sequence,"diagnostic":diagnostic}));
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
struct Attached<'a> {
    handle: SessionHandle<'a>,
    binding: Binding,
    clock: String,
    session_id: Id,
}
fn deadline() -> Instant {
    Instant::now() + Duration::from_secs(5)
}
fn next<'a>(host: &mut RuntimeHost<'a, DarwinPlatform>) -> HostEvent<'a> {
    let end = deadline();
    loop {
        assert!(Instant::now() < end, "bounded real host progress");
        match host.next_event().expect("host event") {
            HostEvent::Pending => thread::sleep(Duration::from_millis(1)),
            event => return event,
        }
    }
}
fn complete<'a>(host: &mut RuntimeHost<'a, DarwinPlatform>) -> HostCompletion<'a> {
    match next(host) {
        HostEvent::Complete(c) => c,
        _ => panic!("expected canonical completion"),
    }
}
fn tape_length(parts: &[&[u8]]) -> usize {
    8 + worker_tape::SEGMENTS * 8 + parts.iter().map(|p| p.len()).sum::<usize>()
}
fn attach<'a>(
    host: &mut RuntimeHost<'a, DarwinPlatform>,
    binding: Binding,
    number: u32,
) -> Attached<'a> {
    let session_id = Id(format!("live-session-{number}"));
    let mut descriptor = baseline::descriptor();
    let Artifact::Session(session) = &mut descriptor.artifact else {
        panic!("descriptor")
    };
    session.session_id = session_id.clone();
    session.target = binding.target.clone();
    session.surfaces = vec![binding.surface.clone()];
    session.allowed_scopes = ["left", "sized", "draft"]
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
            TargetLease::authorized(&binding.target, false).expect("owned fixture authority"),
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
    host: &mut RuntimeHost<'a, DarwinPlatform>,
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
fn detach<'a>(host: &mut RuntimeHost<'a, DarwinPlatform>, attached: &Attached<'a>) {
    assert!(host.detach(attached.handle).expect("detach").is_none());
    loop {
        if let HostEvent::Closed { session } = next(host) {
            assert_eq!(session, attached.handle);
            break;
        }
    }
}
fn shutdown(host: &mut RuntimeHost<'_, DarwinPlatform>) {
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
    let domain = HostDomain::new::<DarwinPlatform>(baseline::limits()).expect("real guard domain");
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).expect("built worker"),
        DarwinPlatform,
    )
    .expect("real RuntimeHost");
    let diagnostic =
        std::env::var("UIB_WEB_LIVE_CASE").is_ok_and(|v| v == "first_observe_diagnostic");
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
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
