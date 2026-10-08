use serde_json::{Value as Json, json};
use std::{
    net::{Shutdown, TcpListener, TcpStream},
    sync::{
        Arc, Mutex, Once,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use tungstenite::{Message, accept};
use uiblueprint_schema::{SchemaVersion, model::*, validation};
use uiblueprint_web::{
    cdp,
    collector::{self, Clock, Collector, ErrorKind, NodeRef, Publication, Scope},
    transport,
};

const CANARY: &str = "PRIVATE_COLLECTOR_CANARY";
struct Logger(AtomicBool);
static LOGGER: Logger = Logger(AtomicBool::new(false));
static INIT: Once = Once::new();
impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }
    fn log(&self, r: &log::Record<'_>) {
        if format!("{} {}", r.target(), r.args()).contains(CANARY) {
            self.0.store(true, Ordering::Relaxed);
        }
    }
    fn flush(&self) {}
}
fn logging() {
    INIT.call_once(|| {
        transport::install_log_boundary(&LOGGER).expect("filtered logging");
        log::set_max_level(log::LevelFilter::Trace);
    });
}
fn id(s: &str) -> Id {
    Id(s.into())
}
fn target() -> Identity {
    Identity {
        id: id("target"),
        generation: id("target-generation"),
    }
}
fn surface() -> Identity {
    Identity {
        id: id("frame"),
        generation: id("loader"),
    }
}
fn plugin() -> PluginIdentity {
    PluginIdentity {
        id: id("web"),
        version: id("0.1.0"),
    }
}
fn limits() -> collector::Limits {
    collector::Limits {
        max_nodes: 16,
        max_methods: 100,
        max_reply_bytes: 8192,
        max_total_reply_bytes: 65536,
        max_text_bytes: 600,
        max_handle_bytes: 256,
        max_ax_properties: 32,
        io_read_bytes: 16384,
        io_write_bytes: 16384,
        io_work: 2048,
    }
}
fn op() -> transport::OperationLimits {
    transport::OperationLimits {
        deadline: Instant::now() + Duration::from_secs(2),
        max_read_bytes: 16384,
        max_write_bytes: 16384,
        max_work: 2048,
    }
}
fn request() -> Request {
    Request {
        clock_domain: id("worker-clock"),
        request_id: id("request-1"),
        context: Context {
            schema_version: SchemaVersion::CURRENT,
            session_id: id("session"),
            target: target(),
            surfaces: vec![surface()],
            scope_id: id("scope"),
            projection: Projection::Interaction,
            fields: vec![
                Field::Role,
                Field::Name,
                Field::AccessibilityName,
                Field::LayoutBounds,
                Field::Enabled,
                Field::Checked,
                Field::Value,
            ],
            plugin: plugin(),
            environment_revision: id("environment"),
        },
        limits: Limits {
            max_elements: 32,
            max_depth: 8,
            max_output_bytes: 65536,
            deadline_ms: 1500,
        },
        freshness_policy: FreshnessPolicy::CurrentRequired,
        operation: Operation::Observe {
            channels: vec![Channel::ExternalSemantics],
        },
    }
}
fn scope(ids: &[u32]) -> Scope {
    Scope {
        scope_id: id("scope"),
        nodes: ids
            .iter()
            .map(|n| NodeRef {
                reference: BackendRef {
                    session_id: id("session"),
                    target: target(),
                    surface: surface(),
                    key: SourceKey {
                        namespace: id("web.dom"),
                        key: Id(n.to_string()),
                    },
                    snapshot_id: id("previous-observation-snapshot"),
                    observation_id: id("previous-source-observation"),
                },
                sensitivity: Sensitivity::Public,
            })
            .collect(),
    }
}
struct Peer {
    url: String,
    done: mpsc::Receiver<()>,
    join: Option<JoinHandle<()>>,
    stop: Arc<AtomicBool>,
    socket: Arc<Mutex<Option<TcpStream>>>,
}
impl Peer {
    fn new(f: impl FnOnce(TcpStream) + Send + 'static) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("owned listener");
        let addr = listener.local_addr().expect("addr");
        listener.set_nonblocking(true).expect("nonblocking accept");
        let (tx, done) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let cancelled = stop.clone();
        let socket = Arc::new(Mutex::new(None));
        let slot = socket.clone();
        let join = thread::spawn(move || {
            let end = Instant::now() + Duration::from_secs(2);
            loop {
                if cancelled.load(Ordering::Acquire) || Instant::now() >= end {
                    break;
                }
                match listener.accept() {
                    Ok((stream, _)) => {
                        stream
                            .set_nonblocking(false)
                            .expect("blocking accepted socket");
                        stream
                            .set_read_timeout(Some(Duration::from_millis(500)))
                            .expect("timeout");
                        stream
                            .set_write_timeout(Some(Duration::from_millis(500)))
                            .expect("timeout");
                        *slot.lock().expect("slot") =
                            Some(stream.try_clone().expect("peer shutdown handle"));
                        f(stream);
                        break;
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(1))
                    }
                    Err(e) => panic!("accept: {e:?}"),
                }
            }
            let _ = tx.send(());
        });
        Self {
            url: format!("ws://{addr}/fixture"),
            done,
            join: Some(join),
            stop,
            socket,
        }
    }
    fn finish(mut self) {
        self.done
            .recv_timeout(Duration::from_secs(3))
            .expect("bounded peer completion");
        self.join_owned().expect("peer assertions");
    }
    fn join_owned(&mut self) -> Result<(), &'static str> {
        if let Some(join) = self.join.take() {
            let end = Instant::now() + Duration::from_secs(3);
            while !join.is_finished() && Instant::now() < end {
                thread::sleep(Duration::from_millis(1));
            }
            if !join.is_finished() {
                return Err("owned peer failed to stop");
            }
            join.join().map_err(|_| "peer assertion failed")?;
        }
        Ok(())
    }
}
impl Drop for Peer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(s) = self.socket.lock().expect("slot").take() {
            let _ = s.shutdown(Shutdown::Both);
        }
        let result = self.join_owned();
        if !thread::panicking() {
            result.expect("peer cleanup");
        }
    }
}
struct Fixture {
    peer: Peer,
    calls: Arc<Mutex<Vec<String>>>,
}
impl Fixture {
    fn new(change: impl FnMut(&str, &Json, usize) -> Option<Json> + Send + 'static) -> Self {
        Self::with_verification(
            change,
            |_| json!({"result":{"type":"object","value":{"current":true}}}),
        )
    }
    fn with_verification(
        mut change: impl FnMut(&str, &Json, usize) -> Option<Json> + Send + 'static,
        mut verify: impl FnMut(&Json) -> Json + Send + 'static,
    ) -> Self {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let seen = calls.clone();
        let peer = Peer::new(move |s| {
            let mut ws = accept(s).expect("handshake");
            let mut reads = 0;
            let mut initial_ids = Vec::<String>::new();
            let mut rooted = false;
            while let Ok(message) = ws.read() {
                let Message::Text(text) = message else { break };
                let command: Json = serde_json::from_str(&text).expect("command");
                let method = command["method"].as_str().expect("method");
                seen.lock().expect("calls").push(method.into());
                reads += 1;
                let is_verification = method == "Runtime.callFunctionOn"
                    && command["params"]["functionDeclaration"]
                        .as_str()
                        .is_some_and(|s| {
                            s.starts_with("function verifyNodes(")
                                || s.starts_with("function verifyRooted(")
                        });
                let is_selection = method == "Runtime.callFunctionOn"
                    && command["params"]["functionDeclaration"]
                        .as_str()
                        .is_some_and(|s| s.starts_with("function selectIds("));
                let default = match method {
                    "Target.getTargetInfo" => json!({"targetInfo":{"targetId":"target"}}),
                    "Page.getFrameTree" => {
                        json!({"frameTree":{"frame":{"id":"frame","loaderId":"loader"}}})
                    }
                    "DOM.getDocument" => {
                        assert_eq!(command["params"]["depth"], 0);
                        assert_eq!(command["params"]["pierce"], false);
                        json!({"root":{"backendNodeId":1,"nodeType":9}})
                    }
                    "Page.createIsolatedWorld" => {
                        assert_eq!(command["params"]["frameId"], "frame");
                        assert_eq!(command["params"]["grantUniveralAccess"], false);
                        json!({"executionContextId":3})
                    }
                    "Accessibility.enable"
                    | "Runtime.releaseObjectGroup"
                    | "DOM.focus"
                    | "Input.insertText" => json!({}),
                    "DOM.resolveNode" => {
                        assert_eq!(command["params"]["executionContextId"], 3);
                        json!({"object":{"type":"object","subtype":"node","objectId":format!("node-{}",command["params"]["backendNodeId"])}})
                    }
                    "Runtime.callFunctionOn" => {
                        assert_eq!(command["params"]["returnByValue"], !is_selection);
                        assert_eq!(command["params"]["userGesture"], false);
                        assert_eq!(
                            command["params"]["throwOnSideEffect"],
                            is_selection
                                || (is_verification
                                    && command["params"]["functionDeclaration"]
                                        .as_str()
                                        .unwrap_or("")
                                        .starts_with("function verifyNodes(")),
                            "only fixed native read/rooted verification use ordinary evaluation"
                        );
                        if is_selection {
                            rooted = command["params"]["arguments"][0]["value"]["rooted"] == true;
                            if rooted {
                                assert_eq!(command["params"]["objectId"], "node-11");
                            }
                            initial_ids = serde_json::from_value(
                                command["params"]["arguments"][0]["value"]["ids"].clone(),
                            )
                            .expect("ID arguments are data");
                            if rooted {
                                initial_ids = vec!["left".into(), "right".into()];
                            }
                            json!({"result":{"type":"object","objectId":"selection-container"}})
                        } else if is_verification {
                            verify(&command)
                        } else {
                            serde_json::from_str(include_str!("fixtures/collector/dom.json"))
                                .expect("DOM literal")
                        }
                    }
                    "Runtime.getProperties" => {
                        assert_eq!(command["params"]["objectId"], "selection-container");
                        assert_eq!(command["params"]["ownProperties"], true);
                        assert_eq!(command["params"]["generatePreview"], false);
                        let missing = initial_ids
                            .iter()
                            .any(|id| !matches!(id.as_str(), "left" | "right"));
                        let mut properties = vec![
                            json!({"name":"status","value":{"type":"string","value":if missing{"missing"}else{"selected"}}}),
                            json!({"name":"visited","value":{"type":"number","value":20}}),
                        ];
                        if !missing {
                            for (i, id) in initial_ids.iter().enumerate() {
                                let backend = if id == "left" { 11 } else { 12 };
                                properties.push(json!({"name":format!("node_{i}"),"value":{"type":"object","subtype":"node","objectId":format!("node-{backend}")}}));
                            }
                        }
                        if rooted {
                            properties.push(json!({"name":"parent","value":{"type":"object","subtype":"node","objectId":"node-1"}}));
                        }
                        json!({"result":properties})
                    }
                    "DOM.describeNode" => {
                        assert_eq!(command["params"]["depth"], 0);
                        assert_eq!(command["params"]["pierce"], false);
                        let backend = command["params"]["objectId"]
                            .as_str()
                            .expect("object")
                            .strip_prefix("node-")
                            .expect("selected original handle")
                            .parse::<u32>()
                            .expect("backend");
                        json!({"node":{"backendNodeId":backend,"nodeType":1}})
                    }
                    "Accessibility.getPartialAXTree" => {
                        assert_eq!(command["params"]["fetchRelatives"], false);
                        let mut ax: Json =
                            serde_json::from_str(include_str!("fixtures/collector/ax.json"))
                                .expect("AX literal");
                        let backend = command["params"]["backendNodeId"]
                            .as_u64()
                            .expect("backend");
                        ax["nodes"][0]["backendDOMNodeId"] = json!(backend);
                        ax["nodes"][0]["nodeId"] = json!(format!("ax-{backend}"));
                        ax
                    }
                    _ => panic!("unexpected method"),
                };
                let mut replacement = if is_verification
                    && !command["params"]["functionDeclaration"]
                        .as_str()
                        .unwrap_or("")
                        .starts_with("function verifyRooted(")
                {
                    None
                } else {
                    change(method, &command, reads)
                };
                // Test-only control: emit real CDP events before the ordinary method reply.
                if let Some(events) = replacement
                    .as_ref()
                    .and_then(|v| v.get("fixtureEvents"))
                    .and_then(Json::as_array)
                {
                    for event in events {
                        if ws.send(Message::Text(event.to_string().into())).is_err() {
                            return;
                        }
                    }
                    replacement = None;
                }
                let envelope = if let Some(mut custom) = replacement {
                    if custom.get("error").is_some() {
                        custom["id"] = command["id"].clone();
                        custom
                    } else {
                        json!({"id":command["id"],"result":custom})
                    }
                } else {
                    json!({"id":command["id"],"result":default})
                };
                if ws.send(Message::Text(envelope.to_string().into())).is_err() {
                    break;
                }
            }
        });
        Self { peer, calls }
    }
    fn attach(&self, caps: collector::Limits) -> Collector {
        self.try_attach(caps)
            .expect("verified synthetic attachment")
    }
    fn try_attach(&self, caps: collector::Limits) -> Result<Collector, collector::Failure> {
        self.try_attach_limits(caps, 8192, 8192)
    }
    fn client(&self, frame_bytes: usize, message_bytes: usize) -> cdp::Client {
        logging();
        let transport = transport::Transport::connect(
            &self.peer.url,
            transport::Limits {
                endpoint_bytes: 1024,
                handshake_bytes: 2048,
                read_buffer_bytes: 64,
                write_buffer_bytes: 64,
                write_buffer_max: 20000,
                frame_bytes,
                message_bytes,
                outbound_bytes: 16000,
            },
            op(),
        )
        .expect("connect");
        cdp::Client::new(
            transport,
            cdp::Binding {
                target: target(),
                cdp_session_id: None,
            },
            cdp::Limits {
                max_request_bytes: 16000,
                max_message_bytes: 8192,
                max_metadata_bytes: 256,
                max_results: 1,
                result_bytes: 8192,
                max_events: 4,
                event_bytes: 34000,
            },
        )
        .expect("CDP")
    }
    fn try_attach_limits(
        &self,
        caps: collector::Limits,
        frame_bytes: usize,
        message_bytes: usize,
    ) -> Result<Collector, collector::Failure> {
        Collector::attach(
            self.client(frame_bytes, message_bytes),
            collector::Binding {
                session_id: id("session"),
                target: target(),
                surface: surface(),
                cdp_session_id: None,
                clock: Clock {
                    domain: id("worker-clock"),
                    origin: Instant::now() - Duration::from_secs(1),
                },
                allowed_scopes: vec![id("scope")],
                plugin: plugin(),
            },
            caps,
            Instant::now() + Duration::from_secs(2),
        )
    }
    fn methods(&self) -> Vec<String> {
        self.calls.lock().expect("calls").clone()
    }
    fn finish(self) {
        self.peer.finish();
    }
}
fn collect(c: &mut Collector, r: &Request, s: &Scope) -> (collector::Report, Vec<Document>) {
    let mut output = Vec::new();
    let report = c
        .observe(r, s, 41, Instant::now() + Duration::from_secs(2), |d| {
            d.validate()
                .expect("canonical Document roundtrip and semantics");
            output.push(d);
            Publication::Acknowledged
        })
        .expect("observed");
    (report, output)
}
fn snapshot(doc: &Document) -> &Snapshot {
    let Artifact::ChannelResponse(r) = &doc.artifact else {
        panic!("channel envelope")
    };
    assert_eq!(r.dispatch_sequence, 41);
    let ChannelResult::Observed(s) = &r.result else {
        panic!("observed")
    };
    s
}
fn known(node: &Node, field: Field) -> &Value {
    node.properties
        .iter()
        .find(|p| p.field() == field)
        .and_then(Property::known)
        .expect("known property")
}

#[test]
fn canonical_dom_ax_literals_preserve_sources_geometry_false_empty_and_same_labels() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let (_, docs) = collect(&mut c, &request(), &scope(&[11, 12]));
    let s = snapshot(&docs[0]);
    validation::validate_snapshot(s).expect("canonical semantics");
    let expected: Json = serde_json::from_str(include_str!("fixtures/collector/expected.json"))
        .expect("independent expectation");
    assert_eq!((s.nodes.len(), s.relations.len()), (4, 2));
    assert_eq!(s.coverage.status, CoverageStatus::Partial);
    assert!(s.source_state.is_none());
    assert!(
        s.observations.iter().all(|o| o.start >= 1000.0),
        "must use provided worker clock origin"
    );
    let dom = &s.nodes[0];
    let ax = &s.nodes[2];
    assert_eq!(known(ax, Field::Name), known(ax, Field::AccessibilityName));
    assert_ne!(dom.key, ax.key);
    assert_eq!(dom.key.namespace.0, "web.dom");
    assert_eq!(ax.key.namespace.0, "web.ax");
    assert_eq!(known(ax, Field::Role), &Value::Role(Role::Checkbox));
    assert_eq!(
        known(ax, Field::AccessibilityName),
        &Value::Text(
            expected["accessibility_name"]
                .as_str()
                .expect("name")
                .into()
        )
    );
    assert_eq!(known(dom, Field::Checked), &Value::Flag(false));
    assert_eq!(known(dom, Field::Value), &Value::Text(String::new()));
    assert_eq!(known(ax, Field::Checked), &Value::Flag(false));
    assert_eq!(known(ax, Field::Enabled), &Value::Flag(true));
    let Value::Geometry(g) = known(dom, Field::LayoutBounds) else {
        panic!("geometry")
    };
    assert_eq!(g.frame_kind, FrameKind::LayoutBounds);
    assert_eq!(g.coordinate_space.units, Unit::CssPx);
    assert_eq!(g.coordinate_space.origin, Origin::TopLeft);
    assert!(matches!(g.transform, TransformState::LocalOnly {}));
    let Shape::Rect(actual) = &g.shape else {
        panic!("rect shape")
    };
    let expected_rect: Rect =
        serde_json::from_value(expected["rect"].clone()).expect("independent rectangle");
    assert_eq!(actual, &expected_rect);
    assert!(
        ax.properties
            .iter()
            .find(|p| p.field() == Field::LayoutBounds)
            .expect("requested unknown")
            .known()
            .is_none()
    );
    assert!(
        dom.properties
            .iter()
            .find(|p| p.field() == Field::AccessibilityName)
            .expect("requested unknown")
            .known()
            .is_none()
    );
    assert_eq!(
        known(&s.nodes[3], Field::AccessibilityName),
        known(ax, Field::AccessibilityName)
    );
    assert!(
        s.observations
            .iter()
            .all(|o| o.clock_domain.0 == "worker-clock"
                && o.end >= o.start
                && o.consistency == Consistency::Unknown)
    );
    assert!(
        fixture
            .methods()
            .contains(&"Runtime.releaseObjectGroup".into())
    );
    drop(c);
    fixture.finish();
}

fn viewport_peer(mode: u8) -> Fixture {
    let mut reads = 0;
    Fixture::new(move |method, command, _| {
        if method != "Runtime.callFunctionOn"
            || !command["params"]["functionDeclaration"]
                .as_str()
                .is_some_and(|f| f.starts_with("function readNode("))
        {
            return None;
        }
        reads += 1;
        let mut read: Json =
            serde_json::from_str(include_str!("fixtures/collector/dom.json")).unwrap();
        let mut facts = json!({"scrollX":-12.25,"scrollY":100.5,"width":1000,"height":600,"dpr":2,"scale":1,"offsetLeft":0,"offsetTop":0,"pageLeft":-12.25,"pageTop":100.5,"visualWidth":985,"visualHeight":600});
        if mode == 1 {
            facts["scale"] = json!(2);
        }
        if mode == 4 && reads == 2 || mode == 5 && reads == 3 {
            facts["scrollY"] = json!(101.5);
            facts["pageTop"] = json!(101.5);
        }
        if mode == 6 {
            facts["width"] = json!(-1);
        }
        if mode == 7 {
            facts["scrollX"] = Json::Null;
        }
        if mode == 8 {
            facts["width"] = json!(0);
        }
        if mode == 9 {
            facts["offsetLeft"] = json!(1);
            facts["pageLeft"] = json!(-11.25);
        }
        if mode == 10 {
            facts["pageTop"] = json!(99);
        }
        let mut after = facts.clone();
        if mode == 3 {
            after["scrollY"] = json!(101.5);
            after["pageTop"] = json!(101.5);
        }
        read["result"]["value"]["rect"]["viewport"] = if mode == 2 {
            json!({"before":null,"after":null})
        } else {
            json!({"before":facts,"after":after})
        };
        if mode == 11 {
            assert_eq!(
                command["params"]["arguments"][0]["value"]["sensitive"],
                true
            );
            read["result"]["value"]["value"] = json!(CANARY);
        }
        Some(read)
    })
}

#[test]
fn viewport_mapping_preserves_original_rect_and_binds_sourced_document_translation() {
    let fixture = viewport_peer(0);
    let mut c = fixture.attach(limits());
    let mut r = request();
    r.context.fields = vec![Field::LayoutBounds];
    let (report, docs) = collect(&mut c, &r, &scope(&[11, 12]));
    let s = snapshot(&docs[0]);
    assert_eq!(report.dom_nodes, 2);
    for node in &s.nodes {
        let Value::Geometry(g) = known(node, Field::LayoutBounds) else {
            panic!("geometry")
        };
        assert_eq!(
            g.shape,
            Shape::Rect(Rect {
                x: 40.0,
                y: 60.0,
                width: 120.0,
                height: 40.0
            })
        );
        assert_eq!(g.frame_kind, FrameKind::LayoutBounds);
        let TransformState::Known { transform } = &g.transform else {
            panic!("sourced mapping")
        };
        assert_eq!(transform.from, g.coordinate_space);
        assert_eq!(transform.to.kind, SpaceKind::Document);
        assert_eq!(transform.to.units, Unit::CssPx);
        assert_eq!(transform.to.origin, Origin::TopLeft);
        assert_eq!(transform.affine, [1.0, 0.0, 0.0, 1.0, -12.25, 100.5]);
        assert_eq!(transform.target, r.context.target);
        assert_eq!(transform.surface, r.context.surfaces[0]);
        assert_eq!(
            transform.environment_revision,
            r.context.environment_revision
        );
        assert_eq!(transform.evidence.provenance, Provenance::Derived);
        assert_eq!(
            transform.evidence.method,
            id("cssom-scroll-viewport-to-document")
        );
        assert_eq!(transform.evidence.observation_id, s.observations[0].id);
        assert_eq!(g.coordinate_space.kind, SpaceKind::Viewport);
    }
    assert_eq!(s.coverage.status, CoverageStatus::Partial);
    drop(c);
    fixture.finish();
}

#[test]
fn viewport_mapping_keeps_unconfirmed_scale_offsets_and_missing_facts_unknown() {
    for mode in [1, 2, 8, 9, 10] {
        let fixture = viewport_peer(mode);
        let mut c = fixture.attach(limits());
        let mut r = request();
        r.context.fields = vec![Field::LayoutBounds];
        let (_, docs) = collect(&mut c, &r, &scope(&[11]));
        let Value::Geometry(g) = known(&snapshot(&docs[0]).nodes[0], Field::LayoutBounds) else {
            panic!("geometry")
        };
        assert!(matches!(g.transform, TransformState::Unknown { .. }));
        assert_eq!(
            g.shape,
            Shape::Rect(Rect {
                x: 40.0,
                y: 60.0,
                width: 120.0,
                height: 40.0
            })
        );
        drop(c);
        fixture.finish();
    }
}

#[test]
fn viewport_mapping_refuses_context_changes_or_invalid_facts_without_publication() {
    for mode in [3, 4, 5, 6, 7] {
        let fixture = viewport_peer(mode);
        let mut c = fixture.attach(limits());
        let mut r = request();
        r.context.fields = vec![Field::LayoutBounds];
        let mut published = false;
        let error = c
            .observe(&r, &scope(&[11, 12]), 41, op().deadline, |_| {
                published = true;
                Publication::Acknowledged
            })
            .unwrap_err();
        assert_eq!(
            error.kind,
            if mode >= 6 {
                ErrorKind::Malformed
            } else {
                ErrorKind::ResyncRequired
            },
            "mode {mode}"
        );
        assert!(!published);
        drop(c);
        fixture.finish();
    }
}

#[test]
fn viewport_final_read_preserves_caller_sensitivity() {
    let fixture = viewport_peer(11);
    let mut c = fixture.attach(limits());
    let mut r = request();
    r.context.fields = vec![Field::LayoutBounds];
    let mut selected = scope(&[11]);
    selected.nodes[0].sensitivity = Sensitivity::Sensitive;
    let (_, docs) = collect(&mut c, &r, &selected);
    assert!(!serde_json::to_string(&docs[0]).unwrap().contains(CANARY));
    let Value::Geometry(g) = known(&snapshot(&docs[0]).nodes[0], Field::LayoutBounds) else {
        panic!("geometry")
    };
    assert!(matches!(g.transform, TransformState::Known { .. }));
    drop(c);
    fixture.finish();
}

#[test]
fn failed_ax_keeps_dom_and_publishes_canonical_partial_once() {
    let fixture = Fixture::new(|m, _, _| {
        (m == "Accessibility.getPartialAXTree")
            .then(|| json!({"error":{"code":-32601,"message":CANARY}}))
    });
    let mut c = fixture.attach(limits());
    let (report, docs) = collect(&mut c, &request(), &scope(&[11, 12]));
    assert_eq!(
        report.ax,
        collector::SourceStatus::Failed(ErrorKind::Protocol(-32601))
    );
    let s = snapshot(&docs[0]);
    assert_eq!(s.nodes.len(), 2);
    assert!(s.nodes.iter().all(|n| n.key.namespace.0 == "web.dom"));
    assert_eq!(
        fixture
            .methods()
            .iter()
            .filter(|m| *m == "Accessibility.getPartialAXTree")
            .count(),
        1
    );
    assert!(
        !serde_json::to_string(&docs)
            .expect("safe canonical")
            .contains(CANARY)
    );
    drop(c);
    fixture.finish();
}

#[test]
fn sensitive_source_is_redacted_before_retention_and_ax_is_not_read() {
    let fixture = Fixture::new(|m, _, _| {
        if m == "Runtime.callFunctionOn" {
            Some(
                json!({"result":{"type":"object","value":{"connected":true,"sameDocument":true,"tag":CANARY,"sensitive":true,"value":CANARY}}}),
            )
        } else {
            None
        }
    });
    let mut c = fixture.attach(limits());
    let (_, docs) = collect(&mut c, &request(), &scope(&[11]));
    let s = snapshot(&docs[0]);
    assert_eq!(s.nodes.len(), 1);
    assert!(matches!(s.nodes[0].native_role, Availability::Redacted {}));
    assert!(matches!(
        s.nodes[0]
            .properties
            .iter()
            .find(|p| p.field() == Field::Value)
            .expect("value"),
        Property::Requested {
            sensitivity: Sensitivity::Sensitive,
            state: Availability::Redacted {},
            ..
        }
    ));
    assert!(
        !serde_json::to_string(&docs)
            .expect("canonical")
            .contains(CANARY)
    );
    assert!(
        !fixture
            .methods()
            .iter()
            .any(|m| m == "Accessibility.getPartialAXTree")
    );
    assert!(!LOGGER.0.load(Ordering::Relaxed));
    assert!(!format!("{c:?}").contains(CANARY));
    drop(c);
    fixture.finish();
}

#[test]
fn canonical_ref_context_mismatch_refuses_before_any_method() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let before = fixture.methods().len();
    for which in 0..5 {
        let mut r = request();
        let mut s = scope(&[11]);
        match which {
            0 => r.context.target.generation = id("other"),
            1 => s.nodes[0].reference.session_id = id("other"),
            2 => s.nodes[0].reference.surface.generation = id("other"),
            3 => s.nodes[0].reference.target.id = id("other"),
            _ => s.nodes[0].reference.key.namespace = id("web.ax"),
        };
        let e = c
            .observe(&r, &s, 41, op().deadline, |_| panic!("no publication"))
            .expect_err("wrong bound ref");
        assert_eq!(e.kind, ErrorKind::StaleTarget);
        assert_eq!(fixture.methods().len(), before);
    }
    drop(c);
    fixture.finish();
}
#[test]
fn wrong_actual_target_stops_before_dependent_frame_read() {
    let fixture = Fixture::new(|m, _, _| {
        (m == "Target.getTargetInfo").then(|| json!({"targetInfo":{"targetId":"other-tab"}}))
    });
    assert_eq!(
        fixture.try_attach(limits()).expect_err("wrong target").kind,
        ErrorKind::StaleTarget
    );
    assert_eq!(fixture.methods(), ["Target.getTargetInfo"]);
    fixture.finish();
}
#[test]
fn changed_loader_or_root_document_invalidates_before_node_reads() {
    for root_change in [false, true] {
        let mut seen = 0;
        let fixture = Fixture::new(move |m, _, _| {
            if m == if root_change {
                "DOM.getDocument"
            } else {
                "Page.getFrameTree"
            } {
                seen += 1;
                if seen == 3 {
                    return Some(if root_change {
                        json!({"root":{"backendNodeId":999,"nodeType":9}})
                    } else {
                        json!({"frameTree":{"frame":{"id":"frame","loaderId":"new-loader"}}})
                    });
                }
            }
            None
        });
        let mut c = fixture.attach(limits());
        let e = c
            .observe(&request(), &scope(&[11]), 41, op().deadline, |_| {
                panic!("stale cannot publish")
            })
            .expect_err("document replaced");
        assert_eq!(e.kind, ErrorKind::StaleTarget);
        assert!(!fixture.methods().iter().any(|m| m == "DOM.resolveNode"));
        let before = fixture.methods().len();
        assert!(
            c.observe(&request(), &scope(&[11]), 41, op().deadline, |_| panic!(
                "stale"
            ))
            .is_err()
        );
        assert_eq!(fixture.methods().len(), before);
        drop(c);
        fixture.finish();
    }
}
#[test]
fn detached_or_foreign_document_node_refuses_and_releases_group() {
    for connected in [false, true] {
        let fixture = Fixture::new(move |m, _, _| {
            (m=="Runtime.callFunctionOn").then(||json!({"result":{"type":"object","value":{"connected":connected,"sameDocument":false,"tag":"INPUT","sensitive":false}}}))
        });
        let mut c = fixture.attach(limits());
        let e = c
            .observe(&request(), &scope(&[11]), 41, op().deadline, |_| {
                panic!("no remount fallback")
            })
            .expect_err("stale source");
        assert_eq!(e.kind, ErrorKind::StaleTarget);
        let calls = fixture.methods();
        assert!(calls.contains(&"Runtime.releaseObjectGroup".into()));
        assert!(!calls.contains(&"Accessibility.getPartialAXTree".into()));
        drop(c);
        fixture.finish();
    }
}
#[test]
fn navigation_after_collection_does_not_publish_fresh_old_data() {
    let mut frames = 0;
    let fixture = Fixture::new(move |m, _, _| {
        if m == "Page.getFrameTree" {
            frames += 1;
            if frames == 4 {
                return Some(json!({"frameTree":{"frame":{"id":"frame","loaderId":"navigated"}}}));
            }
        }
        None
    });
    let mut c = fixture.attach(limits());
    let failure = c
        .observe(&request(), &scope(&[11]), 41, op().deadline, |_| {
            panic!("old doc")
        })
        .expect_err("late navigation");
    assert_eq!(failure.kind, ErrorKind::StaleTarget);
    assert_eq!(
        failure.remote_cleanup,
        collector::RemoteCleanup::Unconfirmed
    );
    assert!(
        !fixture
            .methods()
            .contains(&"Runtime.releaseObjectGroup".into()),
        "no further RPC after invalidated binding"
    );
    drop(c);
    fixture.finish();
}
#[test]
fn missing_ax_and_wrong_backend_are_not_invented_from_dom() {
    for wrong in [false, true] {
        let fixture = Fixture::new(move |m, _, _| {
            if m == "Accessibility.getPartialAXTree" {
                Some(if wrong {
                    json!({"nodes":[{"nodeId":"foreign","ignored":false,"backendDOMNodeId":999,"frameId":"frame"}]})
                } else {
                    json!({"nodes":[]})
                })
            } else {
                None
            }
        });
        let mut c = fixture.attach(limits());
        if wrong {
            assert_eq!(
                c.observe(&request(), &scope(&[11]), 41, op().deadline, |_| panic!(
                    "wrong mapping"
                ))
                .expect_err("wrong backend")
                .kind,
                ErrorKind::StaleTarget
            );
        } else {
            let (r, d) = collect(&mut c, &request(), &scope(&[11]));
            assert_eq!(r.ax, collector::SourceStatus::Partial);
            assert_eq!(snapshot(&d[0]).nodes.len(), 1);
            assert!(snapshot(&d[0]).relations.is_empty());
        }
        drop(c);
        fixture.finish();
    }
}
#[test]
fn explicit_empty_scope_is_complete_and_does_not_resolve_any_node() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let (r, d) = collect(&mut c, &request(), &scope(&[]));
    assert_eq!(r.dom, collector::SourceStatus::Complete);
    let s = snapshot(&d[0]);
    assert_eq!(s.coverage.status, CoverageStatus::Complete);
    assert!(s.nodes.is_empty());
    assert_eq!(s.coverage.unknown_count, Some(0));
    assert!(!fixture.methods().contains(&"DOM.resolveNode".into()));
    drop(c);
    fixture.finish();
}
#[test]
fn node_method_and_request_byte_limits_refuse_acquisition() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let before = fixture.methods().len();
    let mut r = request();
    r.limits.max_elements = 1;
    assert_eq!(
        c.observe(&r, &scope(&[11]), 41, op().deadline, |_| panic!("limit"))
            .expect_err("source node admission")
            .kind,
        ErrorKind::Limit
    );
    assert_eq!(fixture.methods().len(), before);
    let mut r = request();
    r.limits.max_output_bytes = 16;
    assert!(
        c.observe(&r, &scope(&[11]), 41, op().deadline, |_| panic!(
            "wire limit"
        ))
        .is_err()
    );
    assert!(!fixture.methods().contains(&"DOM.resolveNode".into()));
    drop(c);
    fixture.finish();
    let fixture = Fixture::new(|_, _, _| None);
    let mut caps = limits();
    caps.max_methods = 9;
    let mut c = fixture.attach(caps);
    let before = fixture.methods().len();
    assert_eq!(
        c.observe(&request(), &scope(&[11]), 41, op().deadline, |_| panic!(
            "method admission"
        ))
        .expect_err("method reserve includes cleanup")
        .kind,
        ErrorKind::Limit
    );
    assert_eq!(fixture.methods().len(), before);
    drop(c);
    fixture.finish();
}
#[test]
fn fields_gate_ax_and_preserve_unrequested_values() {
    let fixture = Fixture::new(|m, c, _| {
        if m == "Runtime.callFunctionOn" {
            assert_eq!(
                c["params"]["arguments"][0]["value"]["fields"],
                json!(["layout_bounds"])
            );
        }
        None
    });
    let mut c = fixture.attach(limits());
    let mut r = request();
    r.context.fields = vec![Field::LayoutBounds];
    r.limits.max_elements = 1;
    let (report, docs) = collect(&mut c, &r, &scope(&[11]));
    assert_eq!(report.ax, collector::SourceStatus::NotRequested);
    assert_eq!(snapshot(&docs[0]).nodes[0].properties.len(), 1);
    assert!(
        !fixture
            .methods()
            .contains(&"Accessibility.getPartialAXTree".into())
    );
    drop(c);
    fixture.finish();
}
#[test]
fn callback_owns_first_completed_channel_when_cancel_stops_the_next() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let cancellation = c.cancellation().expect("owned cancel");
    let mut r = request();
    r.operation = Operation::Observe {
        channels: vec![Channel::ExternalSemantics, Channel::RenderedCapture],
    };
    let mut owned = Vec::new();
    let error = c
        .observe(&r, &scope(&[11]), 41, op().deadline, |doc| {
            owned.push(doc);
            cancellation.cancel();
            Publication::Acknowledged
        })
        .expect_err("stop after first publication");
    assert!(matches!(
        error.kind,
        ErrorKind::Cdp(_) | ErrorKind::StaleTarget
    ));
    assert_eq!(owned.len(), 1);
    assert_eq!(snapshot(&owned[0]).nodes.len(), 2);
    drop(c);
    validation::validate_snapshot(snapshot(&owned[0])).expect("owned completion survives teardown");
    fixture.finish();
}
#[test]
fn publication_stop_and_expired_request_do_not_dispatch_more() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let before = fixture.methods().len();
    assert_eq!(
        c.observe(&request(), &scope(&[11]), 41, Instant::now(), |_| panic!(
            "expired"
        ))
        .expect_err("absolute deadline")
        .kind,
        ErrorKind::Timeout
    );
    assert_eq!(fixture.methods().len(), before);
    drop(c);
    fixture.finish();
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let e = c
        .observe(&request(), &scope(&[11]), 41, op().deadline, |_| {
            Publication::Stop
        })
        .expect_err("publisher refusal");
    assert_eq!(e.kind, ErrorKind::PublicationStopped);
    let before = fixture.methods().len();
    assert!(
        c.observe(&request(), &scope(&[11]), 41, op().deadline, |_| panic!(
            "detached"
        ))
        .is_err()
    );
    assert_eq!(fixture.methods().len(), before);
    drop(c);
    fixture.finish();
}
#[test]
fn pixels_and_probe_are_explicit_failed_channels_not_claimed_dom_capabilities() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let mut r = request();
    r.operation = Operation::Observe {
        channels: vec![Channel::RenderedCapture, Channel::OptInLayoutProbe],
    };
    let before = fixture.methods().len();
    let (_, docs) = collect(&mut c, &r, &scope(&[]));
    assert_eq!(docs.len(), 2);
    assert_eq!(fixture.methods().len(), before);
    for doc in docs {
        let Artifact::ChannelResponse(response) = doc.artifact else {
            panic!("channel")
        };
        assert!(matches!(
            response.result,
            ChannelResult::Failed(Issue {
                code: ErrorCode::Unsupported,
                ..
            })
        ));
    }
    drop(c);
    fixture.finish();
}

#[test]
fn lost_events_or_document_event_stop_current_refs_without_recollection() {
    for overflow in [false, true] {
        let fixture = Fixture::new(move |m, _, n| {
            if m == "Target.getTargetInfo" && n > 8 {
                Some(
                    json!({"fixtureEvents":if overflow{vec![json!({"method":"Runtime.consoleAPICalled","params":{}});5]}else{vec![json!({"method":"DOM.documentUpdated","params":{}})]}}),
                )
            } else {
                None
            }
        });
        let mut c = fixture.attach(limits());
        let before = fixture.methods().len();
        let e = c
            .observe(&request(), &scope(&[11]), 41, op().deadline, |_| {
                panic!("invalidated")
            })
            .expect_err("lost continuity");
        assert_eq!(
            e.kind,
            if overflow {
                ErrorKind::ResyncRequired
            } else {
                ErrorKind::StaleTarget
            }
        );
        assert_eq!(fixture.methods().len(), before + 1);
        assert!(c.pending_invalidation());
        assert!(
            c.pending_invalidation(),
            "reading never consumes the signal"
        );
        c.acknowledge_invalidation();
        assert!(!c.pending_invalidation());
        assert_eq!(
            fixture.methods().len(),
            before + 1,
            "signal processing never dispatches"
        );
        drop(c);
        fixture.finish();
    }
}
#[test]
fn cancel_during_ax_stops_dispatch_and_reports_unconfirmed_remote_cleanup() {
    let handle = Arc::new(Mutex::new(None::<transport::Cancellation>));
    let cancel = handle.clone();
    let fixture = Fixture::new(move |m, _, _| {
        if m == "Accessibility.getPartialAXTree" {
            cancel
                .lock()
                .expect("handle")
                .as_ref()
                .expect("set after attach")
                .cancel();
        }
        None
    });
    let mut c = fixture.attach(limits());
    *handle.lock().expect("handle") = c.cancellation();
    let e = c
        .observe(&request(), &scope(&[11]), 41, op().deadline, |_| {
            panic!("cancelled source not published")
        })
        .expect_err("cancelled read");
    assert!(matches!(e.kind, ErrorKind::Cdp(cdp::ErrorKind::Cancelled)));
    assert_eq!(e.remote_cleanup, collector::RemoteCleanup::Unconfirmed);
    assert!(
        c.pending_invalidation(),
        "exchange failure must survive cleanup"
    );
    let calls = fixture.methods();
    assert_eq!(
        calls.last().map(String::as_str),
        Some("Accessibility.getPartialAXTree")
    );
    drop(c);
    fixture.finish();
}
#[test]
fn cleanup_failure_detaches_without_claiming_release() {
    let fixture = Fixture::new(|m, _, _| {
        (m == "Runtime.releaseObjectGroup")
            .then(|| json!({"error":{"code":-32000,"message":CANARY}}))
    });
    let mut c = fixture.attach(limits());
    let e = c
        .observe(&request(), &scope(&[11]), 41, op().deadline, |_| {
            panic!("unreleased group")
        })
        .expect_err("cleanup refusal");
    assert_eq!(e.kind, ErrorKind::CleanupUnconfirmed);
    assert_eq!(e.remote_cleanup, collector::RemoteCleanup::Unconfirmed);
    assert!(!format!("{e:?} {e}").contains(CANARY));
    let before = fixture.methods().len();
    assert!(
        c.observe(&request(), &scope(&[11]), 41, op().deadline, |_| panic!(
            "detached"
        ))
        .is_err()
    );
    assert_eq!(fixture.methods().len(), before);
    drop(c);
    fixture.finish();
}
#[test]
fn malformed_method_objects_and_wrong_ax_frame_do_not_publish() {
    for mode in 0..3 {
        let fixture = Fixture::new(move |m, _, _| match (m, mode) {
            ("Runtime.callFunctionOn", 0) => Some(
                json!({"result":{"type":"object","value":{"connected":null,"sameDocument":true,"tag":"INPUT","sensitive":false}}}),
            ),
            ("Runtime.callFunctionOn", 1) => {
                Some(json!({"result":{"type":"object","value":[true,true,"INPUT",false]}}))
            }
            ("Accessibility.getPartialAXTree", 2) => Some(
                json!({"nodes":[{"nodeId":"ax","ignored":false,"backendDOMNodeId":11,"frameId":"wrong"}]}),
            ),
            _ => None,
        });
        let mut c = fixture.attach(limits());
        let e = c
            .observe(&request(), &scope(&[11]), 41, op().deadline, |_| {
                panic!("malformed")
            })
            .expect_err("method refusal");
        assert_eq!(
            e.kind,
            if mode == 2 {
                ErrorKind::StaleTarget
            } else {
                ErrorKind::Malformed
            }
        );
        assert_eq!(e.remote_cleanup, collector::RemoteCleanup::Released);
        drop(c);
        fixture.finish();
    }
}
#[test]
fn output_cap_and_late_ack_are_errors_but_acknowledged_data_stays_owned() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture
        .try_attach_limits(limits(), 1024, 1024)
        .expect("codec fits smaller request allowance");
    let mut r = request();
    r.limits.max_output_bytes = 3000;
    let e = c
        .observe(&r, &scope(&[11]), 41, op().deadline, |_| {
            panic!("canonical output cap")
        })
        .expect_err("canonical cap");
    assert_eq!(e.kind, ErrorKind::Limit);
    assert!(
        fixture
            .methods()
            .contains(&"Runtime.releaseObjectGroup".into())
    );
    drop(c);
    fixture.finish();
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let mut owned = Vec::new();
    let absolute = Instant::now() + Duration::from_millis(100);
    let e = c
        .observe(&request(), &scope(&[11]), 41, absolute, |doc| {
            owned.push(doc);
            thread::sleep(Duration::from_millis(110));
            Publication::Acknowledged
        })
        .expect_err("original deadline remains authoritative");
    assert_eq!(e.kind, ErrorKind::Timeout);
    assert_eq!(owned.len(), 1);
    assert_eq!(snapshot(&owned[0]).nodes.len(), 2);
    drop(c);
    fixture.finish();
}
#[test]
fn caller_sensitivity_cannot_be_downgraded_by_method_payload() {
    let fixture = Fixture::new(|m, _, _| {
        (m=="Runtime.callFunctionOn").then(||json!({"result":{"type":"object","value":{"connected":true,"sameDocument":true,"tag":CANARY,"sensitive":false,"value":CANARY}}}))
    });
    let mut c = fixture.attach(limits());
    let mut s = scope(&[11]);
    s.nodes[0].sensitivity = Sensitivity::Sensitive;
    let (_, docs) = collect(&mut c, &request(), &s);
    assert!(
        !serde_json::to_string(&docs)
            .expect("canonical")
            .contains(CANARY)
    );
    assert!(
        !fixture
            .methods()
            .contains(&"Accessibility.getPartialAXTree".into())
    );
    drop(c);
    fixture.finish();
}

#[test]
fn snapshot_and_observation_ids_do_not_alias_across_collectors() {
    let mut saved = Vec::new();
    for _ in 0..2 {
        let fixture = Fixture::new(|_, _, _| None);
        let mut c = fixture.attach(limits());
        let (report, docs) = collect(&mut c, &request(), &scope(&[11]));
        assert_eq!((report.visited_dom, report.queried_ax), (1, 1));
        assert_eq!(report.omitted_nodes, None);
        saved.extend(docs);
        drop(c);
        fixture.finish();
    }
    assert_ne!(snapshot(&saved[0]).id, snapshot(&saved[1]).id);
    assert_ne!(
        snapshot(&saved[0]).observations[0].id,
        snapshot(&saved[1]).observations[0].id
    );
}
#[test]
fn ax_numeric_bool_values_and_mixed_checked_are_not_coerced_to_false() {
    for value in [
        json!({"type":"number","value":0.25}),
        json!({"type":"integer","value":10000000000000000u64}),
        json!({"type":"boolean","value":false}),
    ] {
        let expected = value.clone();
        let fixture = Fixture::new(move |m, _, _| {
            if m == "Accessibility.getPartialAXTree" {
                Some(
                    json!({"nodes":[{"nodeId":"ax","ignored":false,"backendDOMNodeId":11,"value":value,"properties":[{"name":"checked","value":{"type":"tristate","value":"mixed"}}]}]}),
                )
            } else {
                None
            }
        });
        let mut c = fixture.attach(limits());
        let (_, docs) = collect(&mut c, &request(), &scope(&[11]));
        let node = &snapshot(&docs[0]).nodes[1];
        match expected["type"].as_str().expect("type") {
            "boolean" => assert_eq!(known(node, Field::Value), &Value::Flag(false)),
            _ => assert_eq!(
                known(node, Field::Value),
                &Value::Number(expected["value"].as_f64().expect("number"))
            ),
        }
        assert!(
            node.properties
                .iter()
                .find(|p| p.field() == Field::Checked)
                .expect("checked")
                .known()
                .is_none()
        );
        drop(c);
        fixture.finish();
    }
}

#[test]
fn request_output_budget_is_cumulative_across_canonical_channels() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let mut r = request();
    r.operation = Operation::Observe {
        channels: vec![Channel::RenderedCapture, Channel::OptInLayoutProbe],
    };
    r.limits.max_output_bytes = 550;
    let mut docs = Vec::new();
    let e = c
        .observe(&r, &scope(&[]), 41, op().deadline, |d| {
            docs.push(d);
            Publication::Acknowledged
        })
        .expect_err("second channel exceeds remaining budget");
    assert_eq!(e.kind, ErrorKind::Limit);
    assert_eq!(docs.len(), 1);
    assert!(serde_json::to_vec(&docs[0]).expect("bytes").len() <= 550);
    drop(c);
    fixture.finish();
}

#[test]
fn removed_earlier_node_without_events_cannot_publish_current() {
    let removed = Arc::new(AtomicBool::new(false));
    let removal = removed.clone();
    let seen = Arc::new(Mutex::new(Vec::<String>::new()));
    let reads = seen.clone();
    let retained = seen.clone();
    let fixture = Fixture::with_verification(
        move |method, command, _| {
            if method == "Runtime.callFunctionOn" {
                let object = command["params"]["objectId"]
                    .as_str()
                    .expect("read handle")
                    .to_owned();
                reads.lock().expect("read handles").push(object.clone());
                if object == "node-12" {
                    removal.store(true, Ordering::Release);
                }
            }
            None
        },
        move |command| {
            assert!(removed.load(Ordering::Acquire));
            assert_eq!(
                *retained.lock().expect("original reads"),
                ["node-11", "node-12"]
            );
            assert_eq!(
                command["params"]["arguments"],
                json!([{"objectId":"node-1"},{"objectId":"node-11"},{"objectId":"node-12"}])
            );
            json!({"result":{"type":"object","value":{"current":false}}})
        },
    );
    let mut c = fixture.attach(limits());
    let error = c
        .observe(&request(), &scope(&[11, 12]), 41, op().deadline, |_| {
            panic!("removed node cannot publish Current")
        })
        .expect_err("late removal");
    assert_eq!(error.kind, ErrorKind::StaleTarget);
    assert_eq!(error.remote_cleanup, collector::RemoteCleanup::Released);
    assert_eq!(*seen.lock().expect("reads"), ["node-11", "node-12"]);
    assert_eq!(
        fixture
            .methods()
            .iter()
            .filter(|m| m.as_str() == "DOM.resolveNode")
            .count(),
        3,
        "no locator/re-resolve fallback"
    );
    assert_eq!(
        fixture.methods().last().map(String::as_str),
        Some("Runtime.releaseObjectGroup")
    );
    drop(c);
    fixture.finish();
}
#[test]
fn compatible_codec_caps_and_original_handle_recheck_allow_publication() {
    let checked = Arc::new(AtomicBool::new(false));
    let observed = checked.clone();
    let fixture = Fixture::with_verification(
        |_, _, _| None,
        move |command| {
            assert_eq!(command["params"]["objectId"], "node-1");
            assert_eq!(
                command["params"]["arguments"],
                json!([{"objectId":"node-1"},{"objectId":"node-11"}])
            );
            observed.store(true, Ordering::Release);
            json!({"result":{"type":"object","value":{"current":true}}})
        },
    );
    let mut caps = limits();
    caps.max_reply_bytes = 1024;
    let mut c = fixture
        .try_attach_limits(caps, 1024, 1024)
        .expect("compatible payload caps with larger wire budget and CDP cap");
    let (_, docs) = collect(&mut c, &request(), &scope(&[11]));
    assert!(checked.load(Ordering::Acquire));
    assert_eq!(snapshot(&docs[0]).nodes.len(), 2);
    drop(c);
    fixture.finish();
}
#[test]
fn incompatible_frame_or_message_cap_refuses_before_first_cdp_command() {
    for (frame, message) in [(8192, 8192), (8192, 1024), (1024, 8192)] {
        let fixture =
            Fixture::new(|_, _, _| panic!("no command permitted under incompatible codec caps"));
        let mut caps = limits();
        caps.max_reply_bytes = 1024;
        assert_eq!(
            fixture
                .try_attach_limits(caps, frame, message)
                .expect_err("incompatible acquisition cap")
                .kind,
            ErrorKind::InvalidInput
        );
        assert!(fixture.methods().is_empty());
        fixture.finish();
    }
}
#[test]
fn actual_transport_caps_are_read_only_and_absent_after_detach_or_cancel() {
    for detached in [false, true] {
        let fixture = Fixture::new(|_, _, _| panic!("getter has no IO"));
        let mut client = fixture.client(1024, 2048);
        let mut copy = client.transport_limits().expect("actual config");
        assert_eq!((copy.frame_bytes, copy.message_bytes), (1024, 2048));
        copy.frame_bytes = 1;
        assert_eq!(copy.frame_bytes, 1);
        assert_eq!(
            client
                .transport_limits()
                .expect("immutable original")
                .frame_bytes,
            1024
        );
        if detached {
            client.detach();
        } else {
            client.cancellation().expect("handle").cancel();
        }
        assert!(client.transport_limits().is_none());
        drop(client);
        assert!(fixture.methods().is_empty());
        fixture.finish();
    }
}

fn initial(ids: &[&str]) -> collector::InitialScope {
    collector::InitialScope {
        scope_id: id("scope"),
        ids: ids
            .iter()
            .map(|value| collector::DomId {
                id: id(value),
                sensitivity: Sensitivity::Public,
            })
            .collect(),
        max_visited_nodes: 64,
    }
}
#[test]
fn first_request_issues_refs_only_for_actual_acknowledged_snapshot() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let mut docs = Vec::new();
    let result = c
        .observe_initial(
            &request(),
            &initial(&["left", "right"]),
            41,
            op().deadline,
            |doc| {
                doc.validate().expect("canonical first observation");
                docs.push(doc);
                Publication::Acknowledged
            },
        )
        .expect("first request without prior refs");
    let s = snapshot(&docs[0]);
    assert_eq!(result.references.len(), 2);
    assert_eq!(
        result.selection,
        collector::SelectionReport {
            visited_nodes: 20,
            selected_nodes: 2
        }
    );
    assert_eq!(s.nodes.len(), 4);
    for (r, backend) in result.references.iter().zip([11, 12]) {
        assert_eq!(r.key.key.0, backend.to_string());
        assert_eq!(r.key.namespace.0, "web.dom");
        assert_eq!(r.snapshot_id, s.id);
        assert_eq!(r.observation_id, s.observations[0].id);
        assert_eq!(r.target, s.context.target);
        assert_eq!(r.surface, s.context.surfaces[0]);
        assert_eq!(r.session_id, s.context.session_id);
    }
    assert_eq!(
        fixture
            .methods()
            .iter()
            .filter(|m| m.as_str() == "DOM.resolveNode")
            .count(),
        1,
        "bootstrap uses original objects, not invented refs"
    );
    let before = fixture
        .methods()
        .iter()
        .filter(|m| m.as_str() == "Runtime.getProperties")
        .count();
    let refs = Scope {
        scope_id: id("scope"),
        nodes: result
            .references
            .into_iter()
            .map(|reference| NodeRef {
                reference,
                sensitivity: Sensitivity::Public,
            })
            .collect(),
    };
    let (_, later) = collect(&mut c, &request(), &refs);
    assert_ne!(snapshot(&later[0]).id, s.id);
    assert_eq!(
        fixture
            .methods()
            .iter()
            .filter(|m| m.as_str() == "Runtime.getProperties")
            .count(),
        before,
        "existing-ref path never falls back to search"
    );
    drop(c);
    fixture.finish();
}
#[test]
fn initial_missing_ambiguous_incomplete_and_boundary_never_choose_a_node() {
    for (status, kind) in [
        ("missing", collector::SelectionStatus::Missing),
        ("ambiguous", collector::SelectionStatus::Ambiguous),
        ("incomplete", collector::SelectionStatus::Incomplete),
        ("unsupported", collector::SelectionStatus::Unsupported),
        ("timeout", collector::SelectionStatus::TimedOut),
    ] {
        let fixture = Fixture::new(move |m, _, _| {
            (m=="Runtime.getProperties").then(||json!({"result":[{"name":"status","value":{"type":"string","value":status}},{"name":"visited","value":{"type":"number","value":5}}]}))
        });
        let mut c = fixture.attach(limits());
        let e = c
            .observe_initial(&request(), &initial(&["left"]), 41, op().deadline, |_| {
                panic!("no fabricated observation")
            })
            .expect_err("selection refusal");
        assert_eq!(
            e.kind,
            ErrorKind::Selection {
                status: kind,
                visited_nodes: 5
            }
        );
        assert_eq!(e.remote_cleanup, collector::RemoteCleanup::Released);
        assert!(!fixture.methods().contains(&"DOM.describeNode".into()));
        drop(c);
        fixture.finish();
    }
}
#[test]
fn initial_ref_input_and_method_allowance_are_checked_before_selection() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut caps = limits();
    caps.max_methods = 12;
    let mut c = fixture.attach(caps);
    let before = fixture.methods().len();
    let e = c
        .observe_initial(&request(), &initial(&["left"]), 41, op().deadline, |_| {
            panic!("not admitted")
        })
        .expect_err("bootstrap method reserve");
    assert_eq!(e.kind, ErrorKind::Limit);
    assert_eq!(fixture.methods().len(), before);
    let mut duplicate = initial(&["left", "left"]);
    duplicate.max_visited_nodes = 0;
    assert_eq!(
        c.observe_initial(&request(), &duplicate, 41, op().deadline, |_| panic!(
            "invalid"
        ))
        .expect_err("zero traversal allowance")
        .kind,
        ErrorKind::InvalidInput
    );
    drop(c);
    fixture.finish();
}
#[test]
fn initial_original_object_remount_and_document_drift_refuse_publication() {
    for drift in [false, true] {
        let mut frames = 0;
        let fixture = Fixture::new(move |method, command, _| {
            if method == "Page.getFrameTree" {
                frames += 1;
                if drift && frames == 4 {
                    return Some(
                        json!({"frameTree":{"frame":{"id":"frame","loaderId":"new-document"}}}),
                    );
                }
            }
            if !drift
                && method == "Runtime.callFunctionOn"
                && command["params"]["functionDeclaration"]
                    .as_str()
                    .is_some_and(|s| s.starts_with("function readNode("))
            {
                return Some(
                    json!({"result":{"type":"object","value":{"connected":false,"sameDocument":true,"sensitive":false}}}),
                );
            }
            None
        });
        let mut c = fixture.attach(limits());
        assert_eq!(
            c.observe_initial(
                &request(),
                &initial(&["left"]),
                41,
                op().deadline,
                |_| panic!("no current data on drift/remount")
            )
            .expect_err("original identity lost")
            .kind,
            ErrorKind::StaleTarget
        );
        assert_eq!(
            fixture
                .methods()
                .iter()
                .filter(|m| m.as_str() == "Runtime.getProperties")
                .count(),
            1,
            "no automatic selector retry"
        );
        drop(c);
        fixture.finish();
    }
}

#[test]
fn initial_publication_refusal_cannot_return_refs() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let e = c
        .observe_initial(&request(), &initial(&["left"]), 41, op().deadline, |_| {
            Publication::Stop
        })
        .expect_err("no confirmed publication");
    assert_eq!(e.kind, ErrorKind::PublicationStopped);
    assert!(
        fixture
            .methods()
            .contains(&"Runtime.releaseObjectGroup".into())
    );
    drop(c);
    fixture.finish();
}

#[test]
fn initial_selection_keeps_redaction_and_never_retains_description_attributes() {
    let fixture = Fixture::new(|method, command, _| match method {
        "DOM.describeNode" => {
            Some(json!({"node":{"backendNodeId":11,"nodeType":1,"attributes":["value",CANARY]}}))
        }
        "Runtime.callFunctionOn"
            if command["params"]["functionDeclaration"]
                .as_str()
                .is_some_and(|s| s.starts_with("function readNode(")) =>
        {
            Some(
                json!({"result":{"type":"object","value":{"connected":true,"sameDocument":true,"sensitive":false,"tag":CANARY,"value":CANARY}}}),
            )
        }
        _ => None,
    });
    let mut c = fixture.attach(limits());
    let mut scope = initial(&["left"]);
    scope.ids[0].sensitivity = Sensitivity::Sensitive;
    let mut docs = Vec::new();
    let report = c
        .observe_initial(&request(), &scope, 41, op().deadline, |d| {
            docs.push(d);
            Publication::Acknowledged
        })
        .expect("classified first result");
    assert_eq!(report.references.len(), 1);
    assert!(
        !serde_json::to_string(&docs)
            .expect("safe data")
            .contains(CANARY)
    );
    assert!(!format!("{report:?}").contains(CANARY));
    assert!(
        !fixture
            .methods()
            .contains(&"Accessibility.getPartialAXTree".into())
    );
    drop(c);
    fixture.finish();
}
#[test]
fn initial_lookup_and_later_reads_share_reply_budget_without_reset() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture
        .try_attach_limits(limits(), 1024, 1024)
        .expect("small compatible codec");
    let mut r = request();
    r.limits.max_output_bytes = 1400;
    let e = c
        .observe_initial(&r, &initial(&["left"]), 41, op().deadline, |_| {
            panic!("no reset budget")
        })
        .expect_err("cumulative admission fails between selection and later reads");
    assert_eq!(e.kind, ErrorKind::Limit);
    assert!(
        !fixture
            .methods()
            .contains(&"Accessibility.getPartialAXTree".into())
    );
    drop(c);
    fixture.finish();
}
#[test]
fn initial_final_original_handle_check_still_catches_unannounced_removal() {
    let fixture = Fixture::with_verification(
        |_, _, _| None,
        |command| {
            assert_eq!(
                command["params"]["arguments"],
                json!([{"objectId":"node-1"},{"objectId":"node-11"},{"objectId":"node-12"}])
            );
            json!({"result":{"type":"object","value":{"current":false}}})
        },
    );
    let mut c = fixture.attach(limits());
    let e = c
        .observe_initial(
            &request(),
            &initial(&["left", "right"]),
            41,
            op().deadline,
            |_| panic!("no removed refs"),
        )
        .expect_err("R1 applies to initial path");
    assert_eq!(e.kind, ErrorKind::StaleTarget);
    assert_eq!(e.remote_cleanup, collector::RemoteCleanup::Released);
    drop(c);
    fixture.finish();
}
#[test]
fn initial_malformed_container_or_overshoot_cannot_create_refs() {
    for mode in 0..3 {
        let fixture = Fixture::new(move |method, _, _| {
            if method == "Runtime.getProperties" {
                Some(
                    json!({"result":[{"name":"status","value":{"type":"string","value":"selected"}},{"name":"visited","value":{"type":"number","value":if mode==0{65}else{20}}},{"name":"node_0","value":{"type":"object","subtype":if mode==1{"array"}else{"node"},"objectId":"node-11"},"get":if mode==2{json!({"type":"function","objectId":"untrusted-getter"})}else{Json::Null}}]}),
                )
            } else {
                None
            }
        });
        let mut c = fixture.attach(limits());
        let e = c
            .observe_initial(&request(), &initial(&["left"]), 41, op().deadline, |_| {
                panic!("malformed")
            })
            .expect_err("strict result owner");
        assert_eq!(e.kind, ErrorKind::Malformed);
        assert_eq!(e.remote_cleanup, collector::RemoteCleanup::Released);
        assert!(!fixture.methods().contains(&"DOM.describeNode".into()));
        drop(c);
        fixture.finish();
    }
}

#[test]
fn malformed_acquisition_sites_preserve_refusal_and_cleanup() {
    use collector::MalformedSite as Site;
    for (mode, site) in [
        (0, Site::SelectionReply),
        (1, Site::SelectionException),
        (2, Site::PropertiesShape),
        (3, Site::ReadException),
    ] {
        let fixture = Fixture::new(move |method, command, _| {
            let function = command["params"]["functionDeclaration"]
                .as_str()
                .unwrap_or("");
            let selected = function.starts_with("function selectIds(");
            let reading = function.starts_with("function readNode(");
            if method == "Runtime.callFunctionOn" && selected && mode == 0 {
                Some(json!({"result":[]}))
            } else if method == "Runtime.callFunctionOn"
                && ((selected && mode == 1) || (reading && mode == 3))
            {
                Some(json!({"result":{"type":"object","subtype":"error"},
                    "exceptionDetails":{"text":CANARY}}))
            } else if method == "Runtime.getProperties" && mode == 2 {
                Some(json!({"result":[]}))
            } else {
                None
            }
        });
        let mut c = fixture.attach(limits());
        let error = c
            .observe_initial(&request(), &initial(&["left"]), 41, op().deadline, |_| {
                panic!("malformed response cannot publish")
            })
            .expect_err("malformed acquisition");
        assert_eq!(error.kind, ErrorKind::Malformed);
        assert_eq!(error.malformed_site, Some(site));
        assert_eq!(error.remote_cleanup, collector::RemoteCleanup::Released);
        assert!(!format!("{error:?}").contains(CANARY));
        assert_eq!(
            fixture.methods().last().map(String::as_str),
            Some("Runtime.releaseObjectGroup")
        );
        drop(c);
        fixture.finish();
    }
}

#[test]
fn selected_dom_relations_preserve_binding_direction_and_privacy() {
    for sensitive in [false, true] {
        let fixture = Fixture::new(move |method, command, _| {
            if method != "Runtime.callFunctionOn"
                || !command["params"]["functionDeclaration"]
                    .as_str()
                    .unwrap_or("")
                    .starts_with("function readNode(")
            {
                return None;
            }
            assert_eq!(command["params"]["arguments"][2]["objectId"], "node-11");
            assert_eq!(command["params"]["arguments"][3]["objectId"], "node-12");
            let first = command["params"]["objectId"] == "node-11";
            let mut reply: Json =
                serde_json::from_str(include_str!("fixtures/collector/dom.json")).unwrap();
            let value = &mut reply["result"]["value"];
            value["focused"] = json!(first);
            if first {
                value["controls"] = json!([1]);
                value["activeDescendant"] = json!(1);
            } else {
                value["declaredAnchor"] = json!(0);
            }
            Some(reply)
        });
        let mut c = fixture.attach(limits());
        let mut selected = initial(&["left", "right"]);
        if sensitive {
            selected.ids[1].sensitivity = Sensitivity::Sensitive;
        }
        let mut r = request();
        r.context.fields.push(Field::Focused);
        let mut docs = Vec::new();
        c.observe_initial(&r, &selected, 41, op().deadline, |d| {
            d.validate().unwrap();
            docs.push(d);
            Publication::Acknowledged
        })
        .unwrap();
        let s = snapshot(&docs[0]);
        let relations: Vec<_> = s
            .relations
            .iter()
            .filter(|r| r.kind != RelationKind::CorrespondsTo)
            .collect();
        if sensitive {
            assert!(relations.is_empty());
            assert!(matches!(
                s.focus.active_descendant,
                FocusRef::Unknown { .. }
            ));
        } else {
            assert_eq!(relations.len(), 2);
            assert_eq!(
                (
                    relations[0].kind,
                    relations[0].from.key.0.as_str(),
                    relations[0].to.key.0.as_str()
                ),
                (RelationKind::Controls, "11", "12")
            );
            assert_eq!(
                (
                    relations[1].kind,
                    relations[1].from.key.0.as_str(),
                    relations[1].to.key.0.as_str()
                ),
                (RelationKind::AnchoredTo, "12", "11")
            );
            assert_eq!(relations[0].evidence.method.0, "dom-aria-controls");
            assert_eq!(
                relations[1].evidence.method.0,
                "fixture-data-anchor-attribute"
            );
            assert!(relations.iter().all(|r| r.from.namespace.0 == "web.dom"
                && r.to.namespace.0 == "web.dom"
                && r.evidence.observation_id == s.observations[0].id));
            assert!(
                matches!(&s.focus.active_descendant, FocusRef::Known { target, .. } if target.key.0 == "12" && target.namespace.0 == "web.dom")
            );
        }
        assert!(matches!(s.focus.keyboard, FocusRef::Unknown { .. }));
        drop(c);
        fixture.finish();
    }
}
#[test]
fn invalid_relation_index_or_duplicate_refuses_before_publication() {
    for indexes in [json!([2]), json!([1, 1])] {
        let fixture = Fixture::new(move |method, command, _| {
            if method != "Runtime.callFunctionOn"
                || !command["params"]["functionDeclaration"]
                    .as_str()
                    .unwrap_or("")
                    .starts_with("function readNode(")
            {
                return None;
            }
            let mut reply: Json =
                serde_json::from_str(include_str!("fixtures/collector/dom.json")).unwrap();
            reply["result"]["value"]["controls"] = indexes.clone();
            Some(reply)
        });
        let mut c = fixture.attach(limits());
        let error = c
            .observe_initial(
                &request(),
                &initial(&["left", "right"]),
                41,
                op().deadline,
                |_| panic!("invalid endpoint cannot publish"),
            )
            .unwrap_err();
        assert_eq!(error.kind, ErrorKind::Malformed);
        assert_eq!(error.remote_cleanup, collector::RemoteCleanup::Released);
        drop(c);
        fixture.finish();
    }
}

// Synthetic transport evidence exercises the shipping decoder/normalizer. Native
// getter behavior is separately checked by the existing offline script harness.
fn form_read_peer(value: Json) -> Fixture {
    Fixture::new(move |method, command, _| {
        (method == "Runtime.callFunctionOn"
            && command["params"]["functionDeclaration"]
                .as_str()
                .is_some_and(|s| s.starts_with("function readNode(")))
        .then(|| json!({"result":{"type":"object","value":value}}))
    })
}
fn form_read_value() -> Json {
    json!({"connected":true,"sameDocument":true,"tag":"INPUT","sensitive":false,
        "value":"A💡B","focused":true,"documentFocused":true,"invalid":false,
        "selection":{"start":1,"end":3,"direction":"forward","documentFocused":true}})
}
#[test]
fn form_read_selection_maps_utf16_orientation_and_collapsed_offsets() {
    for (tag, direction, start, end, expected) in [
        ("INPUT", "forward", 1, 3, (1, 3)),
        ("INPUT", "backward", 1, 3, (3, 1)),
        ("TEXTAREA", "none", 4, 4, (4, 4)),
        ("INPUT", "none", 0, 0, (0, 0)),
    ] {
        let mut value = form_read_value();
        value["tag"] = json!(tag);
        value["selection"]["direction"] = json!(direction);
        value["selection"]["start"] = json!(start);
        value["selection"]["end"] = json!(end);
        let fixture = form_read_peer(value);
        let mut c = fixture.attach(limits());
        let mut r = request();
        r.context.fields = vec![Field::Focused, Field::Value, Field::Invalid];
        let (_, docs) = collect(&mut c, &r, &scope(&[11]));
        let s = snapshot(&docs[0]);
        let selection = s.focus.text_selection.as_ref().expect("actual selection");
        assert_eq!((selection.anchor, selection.focus), expected);
        assert_eq!(selection.units.0, "utf16_code_units");
        assert_eq!(selection.evidence.observation_id, s.observations[0].id);
        assert_eq!(selection.evidence.source_namespace.0, "web.dom");
        assert!(
            matches!(&s.focus.keyboard, FocusRef::Known { target, evidence }
            if target.key.0 == "11" && target.namespace.0 == "web.dom"
                && evidence.observation_id == selection.evidence.observation_id)
        );
        assert_eq!(
            known(&s.nodes[0], Field::Value),
            &Value::Text("A💡B".into())
        );
        assert_eq!(known(&s.nodes[0], Field::Invalid), &Value::Flag(false));
        assert!(matches!(
            s.focus.composition_state,
            Property::NotRequested { .. }
        ));
        assert!(matches!(
            s.focus.active_descendant,
            FocusRef::Unknown { .. }
        ));
        drop(c);
        fixture.finish();
    }
}
#[test]
fn form_read_selection_withholds_unestablished_private_or_unrequested_facts() {
    for mode in 0..13 {
        let mut value = form_read_value();
        match mode {
            0 => value["selection"] = Json::Null,
            1 => value["selection"]["direction"] = json!("none"),
            2 => value["selection"]["direction"] = json!("unknown"),
            3 => value["selection"]["end"] = json!(5),
            4 => value["selection"]["start"] = json!(4),
            5 => value["selection"]["documentFocused"] = json!(false),
            6 => value["focused"] = json!(false),
            7 => value["sensitive"] = json!(true),
            8 => value["value"] = Json::Null,
            9 => value["value"] = json!("x".repeat(601)),
            _ => {}
        }
        let fixture = form_read_peer(value);
        let mut c = fixture.attach(limits());
        let mut r = request();
        r.context.fields = if mode == 10 {
            vec![Field::Focused]
        } else if mode == 11 {
            vec![Field::Value]
        } else {
            vec![Field::Focused, Field::Value]
        };
        let selected = scope(if mode == 12 { &[11, 12] } else { &[11] });
        let (_, docs) = collect(&mut c, &r, &selected);
        let s = snapshot(&docs[0]);
        assert!(s.focus.text_selection.is_none(), "mode {mode}");
        assert_eq!(
            matches!(s.focus.keyboard, FocusRef::Known { .. }),
            !matches!(mode, 6 | 7 | 11 | 12),
            "focus is independent of selection/value availability: mode {mode}"
        );
        if mode == 7 {
            assert!(matches!(
                s.nodes[0].properties[1],
                Property::Requested {
                    state: Availability::Redacted { .. },
                    ..
                }
            ));
        }
        drop(c);
        fixture.finish();
    }
}
#[test]
fn form_read_caller_privacy_clears_selection_and_value() {
    let fixture = form_read_peer(form_read_value());
    let mut c = fixture.attach(limits());
    let mut r = request();
    r.context.fields = vec![Field::Focused, Field::Value];
    let mut selected = scope(&[11]);
    selected.nodes[0].sensitivity = Sensitivity::Sensitive;
    let (_, docs) = collect(&mut c, &r, &selected);
    assert!(snapshot(&docs[0]).focus.text_selection.is_none());
    let encoded = serde_json::to_string(&docs[0]).unwrap();
    assert!(!encoded.contains("A💡B"));
    assert!(!encoded.contains("utf16_code_units"));
    drop(c);
    fixture.finish();
}
#[test]
fn form_read_malformed_selection_refuses_without_publication() {
    for selection in [
        json!([]),
        json!({"start":-1,"end":3,"direction":"forward","documentFocused":true}),
        json!({"start":0.5,"end":3,"direction":"forward","documentFocused":true}),
    ] {
        let mut value = form_read_value();
        value["selection"] = selection;
        let fixture = form_read_peer(value);
        let mut c = fixture.attach(limits());
        let error = c
            .observe(&request(), &scope(&[11]), 41, op().deadline, |_| {
                panic!("malformed selection cannot publish")
            })
            .unwrap_err();
        assert_eq!(error.kind, ErrorKind::Malformed);
        assert_eq!(error.remote_cleanup, collector::RemoteCleanup::Released);
        drop(c);
        fixture.finish();
    }
}
#[test]
fn form_read_output_preserves_empty_value_and_bounded_unknown() {
    for value in [json!(""), json!("London"), Json::Null] {
        let mut read = form_read_value();
        read["tag"] = json!("OUTPUT");
        read["value"] = value.clone();
        read["selection"] = Json::Null;
        let fixture = form_read_peer(read);
        let mut c = fixture.attach(limits());
        let mut r = request();
        r.context.fields = vec![Field::Value, Field::Invalid];
        let (_, docs) = collect(&mut c, &r, &scope(&[11]));
        let s = snapshot(&docs[0]);
        if let Some(value) = value.as_str() {
            assert_eq!(known(&s.nodes[0], Field::Value), &Value::Text(value.into()));
        } else {
            assert!(matches!(&s.nodes[0].properties[0], Property::Requested {
                state: Availability::Unknown { reason }, ..
            } if reason.0 == "output-value-not-qualified-within-read-bound"));
        }
        assert_eq!(known(&s.nodes[0], Field::Invalid), &Value::Flag(false));
        assert!(s.focus.text_selection.is_none());
        drop(c);
        fixture.finish();
    }
}

fn rooted() -> collector::RootedScope {
    collector::RootedScope {
        scope_id: id("scope"),
        max_visited_nodes: 256,
        root: collector::RootSeed {
            session_id: id("session"),
            target: target(),
            surface: surface(),
            document_backend_id: 1,
            backend_node_id: 11,
            sensitivity: Sensitivity::Public,
        },
    }
}
#[test]
fn rooted_seed_publishes_actual_refs_without_initial_ids_or_fake_provenance() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let mut docs = Vec::new();
    let result = c
        .observe_rooted(&request(), &rooted(), 41, op().deadline, |d| {
            d.validate().unwrap();
            docs.push(d);
            Publication::Acknowledged
        })
        .unwrap();
    assert_eq!(result.references.len(), 2);
    assert_eq!(result.selection.selected_nodes, 2);
    for reference in result.references {
        assert_eq!(reference.snapshot_id, snapshot(&docs[0]).id);
        assert_eq!(
            reference.observation_id,
            snapshot(&docs[0]).observations[0].id
        );
    }
    assert_eq!(snapshot(&docs[0]).coverage.status, CoverageStatus::Partial);
    drop(c);
    fixture.finish();
}
#[test]
fn rooted_seed_wrong_binding_or_document_refuses_before_any_collection() {
    for mode in 0..6 {
        let fixture = Fixture::new(|_, _, _| None);
        let mut c = fixture.attach(limits());
        let before = fixture.methods().len();
        let mut scope = rooted();
        match mode {
            0 => scope.root.session_id = id("wrong"),
            1 => scope.root.target.generation = id("wrong"),
            2 => scope.root.surface.generation = id("wrong"),
            3 => scope.root.document_backend_id = 2,
            4 => scope.root.backend_node_id = 0,
            _ => scope.max_visited_nodes = 0,
        }
        assert!(
            c.observe_rooted(&request(), &scope, 41, op().deadline, |_| panic!(
                "invalid root"
            ))
            .is_err()
        );
        assert_eq!(fixture.methods().len(), before);
        drop(c);
        fixture.finish();
    }
}
#[test]
fn rooted_removal_or_reparenting_and_limited_search_never_publish() {
    for mode in 0..4 {
        let mut verified = 0;
        let fixture = Fixture::new(move |method, command, _| {
            if mode == 0 && method == "Runtime.getProperties" {
                return Some(
                    json!({"result":[{"name":"status","value":{"type":"string","value":"incomplete"}},{"name":"visited","value":{"type":"number","value":256}}]}),
                );
            }
            if method == "Runtime.callFunctionOn"
                && command["params"]["functionDeclaration"]
                    .as_str()
                    .unwrap_or("")
                    .starts_with("function verifyRooted(")
            {
                verified += 1;
                if (mode == 1 && verified == 1) || (mode == 2 && verified == 3) {
                    return Some(json!({"result":{"type":"object","value":{"current":false}}}));
                }
            }
            None
        });
        let mut c = fixture.attach(limits());
        let publication = if mode == 3 {
            Publication::Stop
        } else {
            Publication::Acknowledged
        };
        let error = c
            .observe_rooted(&request(), &rooted(), 41, op().deadline, |_| {
                assert_eq!(mode, 3);
                publication
            })
            .unwrap_err();
        assert_eq!(
            error.kind,
            match mode {
                0 => ErrorKind::Selection {
                    status: collector::SelectionStatus::Incomplete,
                    visited_nodes: 256
                },
                3 => ErrorKind::PublicationStopped,
                _ => ErrorKind::StaleTarget,
            }
        );
        drop(c);
        fixture.finish();
    }
}
#[test]
fn rooted_sensitive_seed_redacts_all_descendants() {
    let fixture = Fixture::new(|_, _, _| None);
    let mut c = fixture.attach(limits());
    let mut scope = rooted();
    scope.root.sensitivity = Sensitivity::Sensitive;
    c.observe_rooted(&request(), &scope, 41, op().deadline, |d| {
        let s = snapshot(&d);
        assert_eq!(s.nodes.len(), 2);
        assert!(s.relations.is_empty());
        for n in &s.nodes {
            assert!(n.properties.iter().any(|p| matches!(
                p,
                Property::Requested {
                    field: Field::Value,
                    state: Availability::Redacted { .. },
                    ..
                }
            )));
        }
        Publication::Acknowledged
    })
    .unwrap();
    assert!(
        !fixture
            .methods()
            .contains(&"Accessibility.getPartialAXTree".into())
    );
    drop(c);
    fixture.finish();
}

#[test]
fn received_content_events_coalesce_until_cache_owner_acknowledges() {
    for failure in [false, true] {
        let mut emitted = false;
        let fixture = Fixture::new(move |method, _, n| {
            if method == "Target.getTargetInfo" && n > 8 && !emitted {
                emitted = true;
                return Some(json!({"fixtureEvents":[
                    {"method":"Accessibility.nodesUpdated","params":{"private":CANARY}},
                    {"method":"DOM.attributeModified","params":{}},
                    {"method":"CSS.styleSheetChanged","params":{}}
                ]}));
            }
            if failure && method == "Runtime.callFunctionOn" {
                return Some(json!({"error":{"code":-32000,"message":CANARY}}));
            }
            None
        });
        let mut c = fixture.attach(limits());
        assert!(!c.pending_invalidation());
        let result = c.observe(&request(), &scope(&[11]), 41, op().deadline, |_| {
            Publication::Acknowledged
        });
        assert_eq!(result.is_err(), failure);
        assert!(c.pending_invalidation());
        let count = fixture.methods().len();
        assert!(c.pending_invalidation());
        assert_eq!(fixture.methods().len(), count);
        c.acknowledge_invalidation();
        assert!(!c.pending_invalidation());
        assert_eq!(fixture.methods().len(), count);
        drop(c);
        fixture.finish();
    }
}
#[test]
fn changed_document_and_cancelled_connection_leave_pending_invalidation() {
    for failure in [false, true] {
        let fixture = Fixture::new(move |method, _, n| {
            if n > 8 && method == "Page.getFrameTree" {
                return Some(json!({"frameTree":{"frame":{"id":"frame","loaderId":"different"}}}));
            }
            None
        });
        let mut c = fixture.attach(limits());
        if failure {
            c.cancellation().unwrap().cancel();
        }
        assert!(
            c.observe(&request(), &scope(&[11]), 41, op().deadline, |_| panic!(
                "no publication"
            ))
            .is_err()
        );
        assert!(c.pending_invalidation());
        let count = fixture.methods().len();
        c.acknowledge_invalidation();
        assert!(!c.pending_invalidation());
        assert_eq!(fixture.methods().len(), count);
        drop(c);
        fixture.finish();
    }
}

// Synthetic action ports/peer facts only: no browser input or real parent nonce.
fn checkbox_case(c: &mut Collector, wanted: bool) -> ActionCase {
    let mut docs = Vec::new();
    c.observe(&request(), &scope(&[11]), 41, op().deadline, |d| {
        docs.push(d);
        Publication::Acknowledged
    })
    .unwrap();
    let snapshot = snapshot(&docs[0]).clone();
    let source = snapshot
        .observations
        .iter()
        .find(|o| o.source_namespace.0 == "web.dom")
        .unwrap();
    let action = Action {
        id: id("set-checkbox"),
        context: snapshot.context.clone(),
        backend_ref: BackendRef {
            session_id: snapshot.context.session_id.clone(),
            key: SourceKey {
                namespace: id("web.dom"),
                key: id("11"),
            },
            snapshot_id: snapshot.id.clone(),
            observation_id: source.id.clone(),
            target: target(),
            surface: surface(),
        },
        intent: Intent::SetChecked { value: wanted },
        modality: InputModality::Setter,
        input_space: None,
        required_enabled: true,
        authorized_scope: id("scope"),
        unique_match: true,
        resolution: Resolution {
            evidence: Evidence {
                observation_id: source.id.clone(),
                source_namespace: id("web.dom"),
                provenance: Provenance::Reported,
                method: id("synthetic-initial-capability"),
                uncertainty: None,
            },
            writable: Availability::Known {
                value: Value::Flag(true),
            },
            value_allowed: Availability::Known {
                value: Value::Flag(true),
            },
            available_intents: vec![id("set_checked")],
        },
    };
    validation::validate_action(&snapshot, &action).unwrap();
    ActionCase { snapshot, action }
}
struct ActionClock {
    tick: u64,
    cancelled: bool,
}

fn text_case(c: &mut Collector, focus: bool, text: &str) -> (ActionCase, Expectation) {
    let mut r = request();
    r.context.fields = vec![
        Field::Enabled,
        Field::Focused,
        Field::Value,
        Field::InputKind,
        Field::Readonly,
    ];
    let (_, docs) = collect(c, &r, &scope(&[11]));
    let snapshot = snapshot(&docs[0]).clone();
    let node = snapshot
        .nodes
        .iter()
        .find(|n| n.key.namespace.0 == "web.dom")
        .unwrap();
    let Property::Requested { evidence, .. } = &node.properties[0] else {
        panic!("source evidence")
    };
    let action = Action {
        id: id("form-action"),
        context: snapshot.context.clone(),
        backend_ref: BackendRef {
            session_id: snapshot.context.session_id.clone(),
            target: target(),
            surface: surface(),
            key: node.key.clone(),
            snapshot_id: snapshot.id.clone(),
            observation_id: evidence.observation_id.clone(),
        },
        intent: if focus {
            Intent::Focus {}
        } else {
            Intent::Type { text: text.into() }
        },
        modality: if focus {
            InputModality::Semantic
        } else {
            InputModality::Keyboard
        },
        input_space: None,
        required_enabled: true,
        authorized_scope: id("scope"),
        unique_match: true,
        resolution: Resolution {
            evidence: evidence.clone(),
            writable: Availability::Known {
                value: Value::Flag(true),
            },
            value_allowed: Availability::Known {
                value: Value::Flag(true),
            },
            available_intents: vec![id(if focus { "focus" } else { "type" })],
        },
    };
    let expected = Expectation {
        id: id("explicit-form-result"),
        scope_id: id("scope"),
        targets: vec![node.key.clone()],
        rule: Rule::PropertyEquals {
            field: if focus { Field::Focused } else { Field::Value },
            expected: if focus {
                Value::Flag(true)
            } else {
                Value::Text(format!("prefix{text}"))
            },
        },
        applies_when: ContextConditions {
            platform: None,
            input_mode: None,
            text_scale: None,
        },
        expected_from: id("explicit-test-scenario"),
    };
    validation::validate_action(&snapshot, &action).unwrap();
    (ActionCase { snapshot, action }, expected)
}
fn text_peer(mode: u8, focus: bool) -> (Fixture, Arc<std::sync::atomic::AtomicUsize>) {
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let count = calls.clone();
    let mut reads = 0;
    let mut focused = !focus;
    let mut value = String::from("prefix");
    let fixture = Fixture::new(move |method, command, _| {
        let delivered = count.load(Ordering::Acquire) > 0;
        if mode == 9 && delivered && method == "Page.getFrameTree" {
            return Some(json!({"frameTree":{"frame":{"id":"frame","loaderId":"different"}}}));
        }
        if method == "DOM.focus" || method == "Input.insertText" {
            count.fetch_add(1, Ordering::AcqRel);
            if focus {
                assert_eq!(method, "DOM.focus");
                assert_eq!(command["params"]["objectId"], "node-11");
                if mode != 14 {
                    focused = true;
                }
            } else {
                assert_eq!(method, "Input.insertText");
                assert_eq!(command["params"]["text"], "Lon💡");
                if mode != 7 {
                    value.push_str("Lon💡");
                }
            }
            return Some(if mode == 10 {
                json!({"error":{"code":-32000,"message":CANARY}})
            } else {
                json!({})
            });
        }
        if method == "Runtime.callFunctionOn"
            && command["params"]["functionDeclaration"]
                .as_str()
                .is_some_and(|s| s.starts_with("function readNode("))
        {
            reads += 1;
            let fresh = reads > 1;
            if delivered && mode == 11 {
                return Some(
                    json!({"result":{"type":"undefined"},"exceptionDetails":{"text":CANARY}}),
                );
            }
            let actual_focus = focused && !(fresh && mode == 4) && !(delivered && mode == 8);
            let n = value.encode_utf16().count();
            return Some(json!({"result":{"type":"object","value":{
                "connected":!(fresh&&mode==6),"sameDocument":true,"tag":"INPUT","inputKind":"text",
                "sensitive":fresh&&mode==3,"enabled":!(fresh&&mode==1),"readonly":fresh&&mode==2,
                "focused":actual_focus,"documentFocused":!(fresh&&mode==5),
                "value":if delivered&&mode==13 {Json::Null}else{json!(value)},
                "selection":{"start":n,"end":n,"direction":"none","documentFocused":!(fresh&&mode==5)}
            }}}));
        }
        None
    });
    (fixture, calls)
}
#[test]
fn form_provider_native_focus_and_type_verify_explicit_full_value_once() {
    use uiblueprint_plugin_api::actions::*;
    for focus in [true, false] {
        let (fixture, calls) = text_peer(0, focus);
        let mut c = fixture.attach(limits());
        let (case, expected) = text_case(&mut c, focus, "Lon💡");
        let mut control = ActionClock {
            tick: 0,
            cancelled: false,
        };
        let start = control.now();
        let mut gate = ActionGate::default();
        let mut execution = ActionExecution::prepare_action(
            case,
            expected,
            id("transition"),
            id("step"),
            start,
            1500,
        )
        .unwrap();
        {
            let mut provider = collector::WebActionProvider::new(&mut c, request().limits);
            assert_eq!(
                execution
                    .dispatch(&mut provider, &mut gate, &mut control)
                    .unwrap(),
                DeliveryStatus::Confirmed
            );
            assert_eq!(
                execution.verify(&mut provider, &mut control).unwrap(),
                CheckStatus::Pass
            );
            assert!(
                execution
                    .dispatch(&mut provider, &mut gate, &mut control)
                    .is_err()
            );
        }
        let result = execution.finish().unwrap();
        assert_eq!(result.transition.steps[0].outcome, Outcome::Succeeded);
        assert!(matches!(
            result.after.as_ref().unwrap().focus.keyboard,
            FocusRef::Known { .. }
        ));
        if !focus {
            assert_eq!(
                known(&result.after.as_ref().unwrap().nodes[0], Field::Value),
                &Value::Text("prefixLon💡".into())
            );
        }
        assert_eq!(calls.load(Ordering::Acquire), 1);
        assert_eq!(gate.calls, 1);
        assert!(c.pending_invalidation());
        assert_eq!(
            fixture.methods().last().unwrap(),
            "Runtime.releaseObjectGroup"
        );
        drop(c);
        fixture.finish();
    }
}
#[test]
fn form_provider_refuses_fresh_ineligible_target_and_large_text_before_permit() {
    use uiblueprint_plugin_api::actions::*;
    for mode in [1, 2, 3, 4, 5, 6, 12] {
        let (fixture, calls) = text_peer(mode, false);
        let mut c = fixture.attach(limits());
        let text = if mode == 12 {
            "x".repeat(601)
        } else {
            "Lon💡".into()
        };
        let (case, expected) = text_case(&mut c, false, &text);
        let mut control = ActionClock {
            tick: 0,
            cancelled: false,
        };
        let start = control.now();
        let mut gate = ActionGate::default();
        let mut execution = ActionExecution::prepare_action(
            case,
            expected,
            id("transition"),
            id("step"),
            start,
            1500,
        )
        .unwrap();
        {
            let mut provider = collector::WebActionProvider::new(&mut c, request().limits);
            assert_eq!(
                execution
                    .dispatch(&mut provider, &mut gate, &mut control)
                    .unwrap(),
                DeliveryStatus::NotDispatched
            );
        }
        assert_eq!(calls.load(Ordering::Acquire), 0);
        assert_eq!(gate.calls, 0);
        drop(c);
        fixture.finish();
    }
}
#[test]
fn form_provider_post_state_mismatch_loss_or_uncertain_delivery_never_passes() {
    use uiblueprint_plugin_api::actions::*;
    for mode in [7, 8, 9, 10, 11, 13, 14] {
        let focus = mode == 14;
        let (fixture, calls) = text_peer(mode, focus);
        let mut c = fixture.attach(limits());
        let (case, expected) = text_case(&mut c, focus, "Lon💡");
        let mut control = ActionClock {
            tick: 0,
            cancelled: false,
        };
        let start = control.now();
        let mut gate = ActionGate::default();
        let mut execution = ActionExecution::prepare_action(
            case,
            expected,
            id("transition"),
            id("step"),
            start,
            1500,
        )
        .unwrap();
        {
            let mut provider = collector::WebActionProvider::new(&mut c, request().limits);
            let delivery = execution
                .dispatch(&mut provider, &mut gate, &mut control)
                .unwrap();
            if mode == 10 {
                assert_eq!(delivery, DeliveryStatus::Unknown);
                assert!(execution.verify(&mut provider, &mut control).is_err());
            } else {
                assert_eq!(delivery, DeliveryStatus::Confirmed);
                assert_eq!(
                    execution.verify(&mut provider, &mut control).unwrap(),
                    if mode == 7 {
                        CheckStatus::Fail
                    } else {
                        CheckStatus::Unknown
                    }
                );
            }
        }
        assert_eq!(calls.load(Ordering::Acquire), 1);
        let result = execution.finish().unwrap();
        assert_eq!(
            result.transition.steps[0].outcome,
            if mode == 7 {
                Outcome::Failed
            } else {
                Outcome::ActionOutcomeUnknown
            }
        );
        assert!(!serde_json::to_string(&result).unwrap().contains(CANARY));
        drop(c);
        fixture.finish();
    }
}
#[test]
fn form_provider_preparation_is_read_only_and_replaces_unknown_capabilities() {
    use uiblueprint_plugin_api::ClockReading;
    for focus in [true, false] {
        let (fixture, calls) = text_peer(0, focus);
        let mut c = fixture.attach(limits());
        let (mut case, _) = text_case(&mut c, focus, "Lon💡");
        case.action.unique_match = false;
        case.action.resolution.writable = Availability::Unknown {
            reason: id("not-prepared"),
        };
        case.action.resolution.value_allowed = Availability::Unknown {
            reason: id("not-prepared"),
        };
        case.action.resolution.available_intents.clear();
        let mut requested = request();
        requested.context = case.snapshot.context.clone();
        requested.clock_domain = id("worker-clock");
        requested.operation = Operation::Prepare {
            action: case.action,
        };
        let prepared = collector::WebActionProvider::new(&mut c, requested.limits.clone())
            .prepare_exact(
                &case.snapshot,
                &requested,
                None,
                &ClockReading {
                    domain: id("worker-clock"),
                    milliseconds: 1,
                },
                1500,
            )
            .unwrap();
        validation::validate_action(&prepared.snapshot, &prepared.action).unwrap();
        assert!(prepared.action.unique_match);
        assert_eq!(
            prepared.action.resolution.available_intents,
            vec![id(if focus { "focus" } else { "type" })]
        );
        assert_eq!(calls.load(Ordering::Acquire), 0);
        assert!(!c.pending_invalidation());
        assert_eq!(
            fixture.methods().last().unwrap(),
            "Runtime.releaseObjectGroup"
        );
        drop(c);
        fixture.finish();
    }
}
#[test]
fn form_provider_cancel_before_or_after_delivery_never_retries() {
    use uiblueprint_plugin_api::actions::*;
    for after in [false, true] {
        let (fixture, calls) = text_peer(0, false);
        let mut c = fixture.attach(limits());
        let (case, expected) = text_case(&mut c, false, "Lon💡");
        let mut control = ActionClock {
            tick: 0,
            cancelled: !after,
        };
        let start = control.now();
        let mut gate = ActionGate::default();
        let mut execution = ActionExecution::prepare_action(
            case,
            expected,
            id("transition"),
            id("step"),
            start,
            1500,
        )
        .unwrap();
        {
            let mut provider = collector::WebActionProvider::new(&mut c, request().limits);
            if after {
                assert_eq!(
                    execution
                        .dispatch(&mut provider, &mut gate, &mut control)
                        .unwrap(),
                    DeliveryStatus::Confirmed
                );
                control.cancelled = true;
                assert!(execution.verify(&mut provider, &mut control).is_err());
            } else {
                assert!(
                    execution
                        .dispatch(&mut provider, &mut gate, &mut control)
                        .is_err()
                );
            }
            assert!(
                execution
                    .dispatch(&mut provider, &mut gate, &mut control)
                    .is_err()
            );
        }
        assert_eq!(calls.load(Ordering::Acquire), usize::from(after));
        assert_eq!(gate.calls, usize::from(after));
        if after {
            assert_eq!(
                execution.finish().unwrap().transition.steps[0].outcome,
                Outcome::ActionOutcomeUnknown
            );
        }
        drop(c);
        fixture.finish();
    }
}
impl uiblueprint_plugin_api::actions::ActionControl for ActionClock {
    fn now(&mut self) -> uiblueprint_plugin_api::ClockReading {
        self.tick += 1;
        uiblueprint_plugin_api::ClockReading {
            domain: id("worker-clock"),
            milliseconds: self.tick,
        }
    }
    fn cancelled(&self) -> bool {
        self.cancelled
    }
}
#[derive(Default)]
struct ActionGate {
    calls: usize,
}
impl uiblueprint_plugin_api::actions::EffectGate for ActionGate {
    fn authorize(
        &mut self,
        _: &Action,
        _: &uiblueprint_plugin_api::ClockReading,
    ) -> Result<
        uiblueprint_plugin_api::actions::DeliveryPermit,
        uiblueprint_plugin_api::actions::GateFailure,
    > {
        self.calls += 1;
        Ok(uiblueprint_plugin_api::actions::DeliveryPermit::from_parent_nonce(7).unwrap())
    }
}
fn checkbox_peer(mode: u8) -> (Fixture, Arc<std::sync::atomic::AtomicUsize>) {
    let writes = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let count = writes.clone();
    let mut checked = false;
    let mut probed = false;
    let fixture = Fixture::new(move |method, command, _| {
        if mode == 12 && method == "Runtime.releaseObjectGroup" && probed {
            return Some(json!({"error":{"code":-32000,"message":CANARY}}));
        }
        if mode == 11 && method == "Page.getFrameTree" && count.load(Ordering::Acquire) > 0 {
            return Some(
                json!({"frameTree":{"frame":{"id":"frame","loaderId":"changed-after-delivery"}}}),
            );
        }
        if method != "Runtime.callFunctionOn" {
            return None;
        }
        let function = command["params"]["functionDeclaration"]
            .as_str()
            .unwrap_or("");
        if function.starts_with("function checkboxState(") {
            probed = true;
            assert_eq!(command["params"]["objectId"], "node-11");
            return Some(json!({"result":{"type":"object","value":{
                "connected":mode!=5,"sameDocument":true,"nativeCheckbox":mode!=2,"sensitive":mode==10,
                "writable":mode!=4,"enabled":mode!=1,"checked":if mode==9&&count.load(Ordering::Acquire)>0{Json::Null}else{json!(checked)},"indeterminate":mode==3
            }}}));
        }
        if function.starts_with("function setChecked(") {
            assert_eq!(command["params"]["objectId"], "node-11");
            assert_eq!(command["params"]["arguments"][0]["objectId"], "node-1");
            count.fetch_add(1, Ordering::AcqRel);
            if mode == 7 {
                return Some(json!({"error":{"code":-32000,"message":CANARY}}));
            }
            if mode != 6 {
                checked = command["params"]["arguments"][1]["value"]
                    .as_bool()
                    .unwrap();
            }
            return Some(json!({"result":{"type":"object","value":{"status":"applied"}}}));
        }
        None
    });
    (fixture, writes)
}
#[test]
fn checkbox_provider_sets_boolean_once_and_verifies_actual_state() {
    use uiblueprint_plugin_api::actions::*;
    for wanted in [true, false] {
        let (fixture, writes) = checkbox_peer(0);
        let mut c = fixture.attach(limits());
        let case = checkbox_case(&mut c, wanted);
        let old_id = case.snapshot.id.clone();
        let mut control = ActionClock {
            tick: 0,
            cancelled: false,
        };
        let start = ActionControl::now(&mut control);
        let mut execution =
            SetCheckedExecution::prepare(case, id("transition"), id("step"), start, 1500).unwrap();
        let mut gate = ActionGate::default();
        {
            let mut provider = collector::CheckboxProvider::new(&mut c, request().limits);
            assert_eq!(
                execution
                    .dispatch(&mut provider, &mut gate, &mut control)
                    .unwrap(),
                DeliveryStatus::Confirmed
            );
            assert_eq!(
                execution.verify(&mut provider, &mut control).unwrap(),
                CheckStatus::Pass
            );
            assert!(
                execution
                    .dispatch(&mut provider, &mut gate, &mut control)
                    .is_err()
            );
        }
        let result = execution.finish().unwrap();
        assert_ne!(result.before.id, old_id);
        assert_eq!(result.transition.steps[0].outcome, Outcome::Succeeded);
        assert_eq!(
            writes.load(Ordering::Acquire),
            1,
            "already-equal still sets, never toggles/retries"
        );
        assert_eq!(gate.calls, 1);
        assert!(c.pending_invalidation());
        assert_eq!(
            fixture.methods().last().map(String::as_str),
            Some("Runtime.releaseObjectGroup")
        );
        drop(c);
        fixture.finish();
    }
}
#[test]
fn checkbox_provider_rejects_ineligible_current_source_before_gate() {
    use uiblueprint_plugin_api::actions::*;
    for mode in [1, 2, 3, 4, 5, 10] {
        let (fixture, writes) = checkbox_peer(mode);
        let mut c = fixture.attach(limits());
        let case = checkbox_case(&mut c, true);
        let mut control = ActionClock {
            tick: 0,
            cancelled: false,
        };
        let start = ActionControl::now(&mut control);
        let mut execution =
            SetCheckedExecution::prepare(case, id("transition"), id("step"), start, 1500).unwrap();
        let mut gate = ActionGate::default();
        {
            let mut provider = collector::CheckboxProvider::new(&mut c, request().limits);
            assert_eq!(
                execution
                    .dispatch(&mut provider, &mut gate, &mut control)
                    .unwrap(),
                DeliveryStatus::NotDispatched
            );
        }
        assert_eq!(gate.calls, 0);
        assert_eq!(writes.load(Ordering::Acquire), 0);
        assert_eq!(
            execution.finish().unwrap().transition.steps[0].outcome,
            Outcome::Failed
        );
        drop(c);
        fixture.finish();
    }
}
#[test]
fn checkbox_provider_does_not_turn_accepted_or_uncertain_delivery_into_success() {
    use uiblueprint_plugin_api::actions::*;
    for mode in [6, 7, 9, 11] {
        let (fixture, writes) = checkbox_peer(mode);
        let mut c = fixture.attach(limits());
        let case = checkbox_case(&mut c, true);
        let mut control = ActionClock {
            tick: 0,
            cancelled: false,
        };
        let start = ActionControl::now(&mut control);
        let mut gate = ActionGate::default();
        let mut execution =
            SetCheckedExecution::prepare(case, id("transition"), id("step"), start, 1500).unwrap();
        {
            let mut provider = collector::CheckboxProvider::new(&mut c, request().limits);
            let delivered = execution
                .dispatch(&mut provider, &mut gate, &mut control)
                .unwrap();
            if mode == 7 {
                assert_eq!(delivered, DeliveryStatus::Unknown);
                assert!(execution.verify(&mut provider, &mut control).is_err());
            } else {
                assert_eq!(delivered, DeliveryStatus::Confirmed);
                assert_eq!(
                    execution.verify(&mut provider, &mut control).unwrap(),
                    if mode == 6 {
                        CheckStatus::Fail
                    } else {
                        CheckStatus::Unknown
                    }
                );
            }
        }
        assert_eq!(writes.load(Ordering::Acquire), 1);
        let result = execution.finish().unwrap();
        assert_eq!(
            result.transition.steps[0].outcome,
            if mode == 6 {
                Outcome::Failed
            } else {
                Outcome::ActionOutcomeUnknown
            }
        );
        drop(c);
        fixture.finish();
    }
}
#[test]
fn checkbox_provider_cancel_before_dispatch_and_expired_budget_do_not_write() {
    use uiblueprint_plugin_api::actions::*;
    let (fixture, writes) = checkbox_peer(0);
    let mut c = fixture.attach(limits());
    let case = checkbox_case(&mut c, true);
    let before = fixture.methods().len();
    let mut control = ActionClock {
        tick: 0,
        cancelled: true,
    };
    let start = ActionControl::now(&mut control);
    let mut gate = ActionGate::default();
    let mut execution =
        SetCheckedExecution::prepare(case.clone(), id("transition"), id("step"), start, 1500)
            .unwrap();
    {
        let mut provider = collector::CheckboxProvider::new(&mut c, request().limits);
        assert!(
            execution
                .dispatch(&mut provider, &mut gate, &mut control)
                .is_err()
        );
    }
    assert_eq!(fixture.methods().len(), before);
    assert_eq!(gate.calls, 0);
    assert_eq!(writes.load(Ordering::Acquire), 0);
    {
        let mut provider = collector::CheckboxProvider::new(&mut c, request().limits);
        assert!(
            provider
                .resolve_exact(
                    &case,
                    &Expectation {
                        id: id("expected-checked"),
                        scope_id: case.action.authorized_scope.clone(),
                        targets: vec![case.action.backend_ref.key.clone()],
                        rule: Rule::PropertyEquals {
                            field: Field::Checked,
                            expected: Value::Flag(true)
                        },
                        applies_when: ContextConditions {
                            platform: None,
                            input_mode: None,
                            text_scale: None
                        },
                        expected_from: id("test_scenario"),
                    },
                    &uiblueprint_plugin_api::ClockReading {
                        domain: id("worker-clock"),
                        milliseconds: 1
                    },
                    0
                )
                .is_err()
        );
    }
    assert_eq!(writes.load(Ordering::Acquire), 0);
    drop(c);
    fixture.finish();
}

#[test]
fn checkbox_provider_preserves_resource_refusal_before_gate() {
    use uiblueprint_plugin_api::actions::*;
    let (fixture, writes) = checkbox_peer(0);
    let mut c = fixture.attach(limits());
    let case = checkbox_case(&mut c, true);
    let before = fixture.methods().len();
    let mut control = ActionClock {
        tick: 0,
        cancelled: false,
    };
    let start = ActionControl::now(&mut control);
    let mut gate = ActionGate::default();
    let mut execution =
        SetCheckedExecution::prepare(case, id("transition"), id("step"), start, 1500).unwrap();
    let mut bounded = request().limits;
    bounded.max_output_bytes = 32;
    {
        let mut provider = collector::CheckboxProvider::new(&mut c, bounded);
        assert_eq!(
            execution
                .dispatch(&mut provider, &mut gate, &mut control)
                .unwrap(),
            DeliveryStatus::NotDispatched
        );
    }
    assert_eq!(execution.issue().unwrap().code, ErrorCode::IncompleteScope);
    assert_eq!(gate.calls, 0);
    assert_eq!(writes.load(Ordering::Acquire), 0);
    assert_eq!(fixture.methods().len(), before);
    drop(c);
    fixture.finish();
}

fn checkbox_prepare_request(c: &mut Collector) -> (Snapshot, Request) {
    let mut docs = Vec::new();
    let observed_request = request();
    c.observe(&observed_request, &scope(&[11]), 41, op().deadline, |d| {
        docs.push(d);
        Publication::Acknowledged
    })
    .unwrap();
    let snapshot = snapshot(&docs[0]).clone();
    let node = snapshot
        .nodes
        .iter()
        .find(|n| n.key.namespace.0 == "web.dom")
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
        reason: id("not_prepared"),
    };
    let action = Action {
        id: id("prepare-checkbox"),
        context: snapshot.context.clone(),
        backend_ref: BackendRef {
            session_id: snapshot.context.session_id.clone(),
            key: node.key.clone(),
            snapshot_id: snapshot.id.clone(),
            observation_id: evidence.observation_id.clone(),
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
            evidence: evidence.clone(),
            writable: unknown.clone(),
            value_allowed: unknown,
            available_intents: vec![],
        },
    };
    let requested = Request {
        request_id: id("prepare-request"),
        clock_domain: id("worker-clock"),
        context: snapshot.context.clone(),
        limits: observed_request.limits,
        freshness_policy: FreshnessPolicy::CurrentRequired,
        operation: Operation::Prepare { action },
    };
    validation::validate_request(&requested).unwrap();
    let Operation::Prepare { action } = &requested.operation else {
        unreachable!()
    };
    assert!(
        validation::validate_action(&snapshot, action).is_err(),
        "input deliberately contains no capability proof"
    );
    (snapshot, requested)
}
#[test]
fn checkbox_preparation_produces_real_resolution_without_permit_or_setter_then_resolves_again() {
    use uiblueprint_plugin_api::actions::*;
    let (fixture, writes) = checkbox_peer(0);
    let mut c = fixture.attach(limits());
    let (snapshot, request) = checkbox_prepare_request(&mut c);
    let before = snapshot.clone();
    let now = uiblueprint_plugin_api::ClockReading {
        domain: id("worker-clock"),
        milliseconds: 1,
    };
    let case = collector::CheckboxProvider::new(&mut c, request.limits.clone())
        .prepare_exact(&snapshot, &request, None, &now, 1500)
        .unwrap();
    assert_eq!(snapshot, before);
    assert_eq!(writes.load(Ordering::Acquire), 0);
    assert!(!c.pending_invalidation());
    assert_ne!(case.snapshot.id, snapshot.id);
    validation::validate_action(&case.snapshot, &case.action).unwrap();
    assert!(case.action.unique_match);
    assert_eq!(
        case.action.resolution.evidence.method.0,
        "native-checkbox-setter-capability"
    );
    assert_eq!(
        case.action.resolution.evidence.observation_id,
        case.action.backend_ref.observation_id
    );
    assert_eq!(
        fixture.methods().last().map(String::as_str),
        Some("Runtime.releaseObjectGroup")
    );
    let prepared_id = case.snapshot.id.clone();
    let mut control = ActionClock {
        tick: 1,
        cancelled: false,
    };
    let start = ActionControl::now(&mut control);
    let mut gate = ActionGate::default();
    let mut execution =
        SetCheckedExecution::prepare(case, id("transition"), id("step"), start, 1400).unwrap();
    {
        let mut provider = collector::CheckboxProvider::new(&mut c, request.limits);
        assert_eq!(
            execution
                .dispatch(&mut provider, &mut gate, &mut control)
                .unwrap(),
            DeliveryStatus::Confirmed
        );
        assert_eq!(
            execution.verify(&mut provider, &mut control).unwrap(),
            CheckStatus::Pass
        );
    }
    let transition = execution.finish().unwrap();
    assert_ne!(
        transition.before.id, prepared_id,
        "dispatch repeats native resolution"
    );
    assert_eq!(gate.calls, 1);
    assert_eq!(writes.load(Ordering::Acquire), 1);
    drop(c);
    fixture.finish();
}
#[test]
fn checkbox_preparation_refuses_current_capability_or_cleanup_failure_without_effect() {
    for mode in [1, 2, 3, 4, 5, 10, 12] {
        let (fixture, writes) = checkbox_peer(mode);
        let mut c = fixture.attach(limits());
        let (snapshot, request) = checkbox_prepare_request(&mut c);
        let result = collector::CheckboxProvider::new(&mut c, request.limits.clone())
            .prepare_exact(
                &snapshot,
                &request,
                None,
                &uiblueprint_plugin_api::ClockReading {
                    domain: id("worker-clock"),
                    milliseconds: 1,
                },
                1500,
            );
        assert!(result.is_err());
        assert_eq!(writes.load(Ordering::Acquire), 0);
        drop(c);
        fixture.finish();
    }
}
#[test]
fn checkbox_preparation_rejects_missing_stale_private_or_changed_request_before_source() {
    for mode in 0..6 {
        let (fixture, writes) = checkbox_peer(0);
        let mut c = fixture.attach(limits());
        let (mut snapshot, mut request) = checkbox_prepare_request(&mut c);
        let Operation::Prepare { action } = &mut request.operation else {
            unreachable!()
        };
        match mode {
            0 => action.backend_ref.key.key = id("missing"),
            1 => action.backend_ref.snapshot_id = id("stale"),
            2 => action.backend_ref.surface.generation = id("stale"),
            3 => action.modality = InputModality::Pointer,
            4 => request.context.scope_id = id("wrong"),
            _ => {
                let node = snapshot
                    .nodes
                    .iter_mut()
                    .find(|n| n.key.namespace.0 == "web.dom")
                    .unwrap();
                let property = node
                    .properties
                    .iter_mut()
                    .find(|p| p.field() == Field::Value)
                    .unwrap();
                let Property::Requested {
                    sensitivity, state, ..
                } = property
                else {
                    unreachable!()
                };
                *sensitivity = Sensitivity::Sensitive;
                *state = Availability::Redacted {};
            }
        }
        let calls = fixture.methods().len();
        assert!(
            collector::CheckboxProvider::new(&mut c, request.limits.clone())
                .prepare_exact(
                    &snapshot,
                    &request,
                    None,
                    &uiblueprint_plugin_api::ClockReading {
                        domain: id("worker-clock"),
                        milliseconds: 1
                    },
                    1500
                )
                .is_err()
        );
        assert_eq!(fixture.methods().len(), calls);
        assert_eq!(writes.load(Ordering::Acquire), 0);
        drop(c);
        fixture.finish();
    }
}

fn activation_peer(mode: u8, input_result: bool) -> (Fixture, Arc<std::sync::atomic::AtomicUsize>) {
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let clicked = calls.clone();
    let verify_clicked = calls.clone();
    let mut actor_reads = 0;
    let mut result_reads = 0;
    let mut result_resolves = 0;
    let fixture = Fixture::with_verification(
        move |method, command, _| {
            let delivered = clicked.load(Ordering::Acquire) > 0;
            if method == "DOM.resolveNode" && command["params"]["backendNodeId"] == 12 {
                result_resolves += 1;
                assert!(!delivered, "never reacquire the result after activation");
                if mode == 9 && result_resolves > 1 {
                    return Some(json!({"error":{"code":-32000,"message":CANARY}}));
                }
            }
            if method != "Runtime.callFunctionOn" {
                return None;
            }
            let function = command["params"]["functionDeclaration"].as_str().unwrap();
            if function.starts_with("function activateButton(") {
                assert_eq!(command["params"]["objectId"], "node-11");
                assert_eq!(
                    command["params"]["arguments"],
                    json!([{"objectId":"node-1"},{"objectId":"node-12"}])
                );
                assert_eq!(command["params"]["userGesture"], false);
                assert!(function.contains("HTMLElement.prototype.click.call(this)"));
                assert!(!function.contains("dispatchEvent"));
                clicked.fetch_add(1, Ordering::AcqRel);
                return Some(if mode == 15 {
                    json!({"error":{"code":-32000,"message":CANARY}})
                } else {
                    json!({"result":{"type":"object","value":{"invoked":true}}})
                });
            }
            if !function.starts_with("function readNode(") {
                return None;
            }
            let actor = command["params"]["objectId"] == "node-11";
            if actor {
                actor_reads += 1;
                assert!(
                    !delivered,
                    "removed actor must not be reread/published after delivery"
                );
            } else {
                assert_eq!(command["params"]["objectId"], "node-12");
                result_reads += 1;
            }
            let fresh = if actor {
                actor_reads > 1
            } else {
                result_reads > 1
            };
            let sensitive = (fresh && ((actor && mode == 2) || (!actor && mode == 3)))
                || (delivered && mode == 12);
            let connected = !(fresh && ((actor && mode == 4) || (!actor && mode == 5)))
                && !(delivered && mode == 10);
            let same = !(fresh && !actor && mode == 6 || delivered && mode == 11);
            let tag = if actor {
                if fresh && mode == 8 { "DIV" } else { "BUTTON" }
            } else if fresh && mode == 7 {
                "SPAN"
            } else if input_result {
                "INPUT"
            } else {
                "OUTPUT"
            };
            Some(json!({"result":{"type":"object","value":{
                "connected":connected,"sameDocument":same,"tag":tag,"sensitive":sensitive,
                "enabled":if actor {json!(!(fresh && mode == 1))}else if input_result{json!(false)}else{Json::Null},"inputKind":if !actor && input_result {json!("text")}else{Json::Null},
                "readonly":if !actor && input_result{json!(true)}else{Json::Null},"value":if actor || (delivered && mode == 13) {Json::Null}
                    else if sensitive {json!(CANARY)}else if delivered {json!(if mode==14{"different"}else{"London"})}else{json!("")}
            }}}))
        },
        move |command| {
            if verify_clicked.load(Ordering::Acquire) > 0 {
                assert_eq!(
                    command["params"]["arguments"],
                    json!([{"objectId":"node-1"},{"objectId":"node-12"}]),
                    "only the held result survives"
                );
            }
            json!({"result":{"type":"object","value":{"current":true}}})
        },
    );
    (fixture, calls)
}

fn activation_case(c: &mut Collector) -> (ActionCase, Expectation) {
    let mut r = request();
    r.context.fields = vec![
        Field::Enabled,
        Field::Value,
        Field::InputKind,
        Field::Readonly,
    ];
    let (_, docs) = collect(c, &r, &scope(&[11, 12]));
    let snapshot = snapshot(&docs[0]).clone();
    let evidence = snapshot
        .observations
        .iter()
        .find(|o| o.source_namespace.0 == "web.dom")
        .unwrap();
    let action = Action {
        id: id("activate-button"),
        context: snapshot.context.clone(),
        backend_ref: BackendRef {
            session_id: id("session"),
            key: SourceKey {
                namespace: id("web.dom"),
                key: id("11"),
            },
            snapshot_id: snapshot.id.clone(),
            observation_id: evidence.id.clone(),
            target: target(),
            surface: surface(),
        },
        intent: Intent::Activate {},
        modality: InputModality::Semantic,
        input_space: None,
        required_enabled: true,
        authorized_scope: id("scope"),
        unique_match: true,
        resolution: Resolution {
            evidence: Evidence {
                observation_id: evidence.id.clone(),
                source_namespace: id("web.dom"),
                provenance: Provenance::Reported,
                method: id("synthetic-button-capability"),
                uncertainty: None,
            },
            writable: Availability::Known {
                value: Value::Flag(true),
            },
            value_allowed: Availability::Known {
                value: Value::Flag(true),
            },
            available_intents: vec![id("activate")],
        },
    };
    let expected = Expectation {
        id: id("explicit-applied-value"),
        scope_id: id("scope"),
        targets: vec![SourceKey {
            namespace: id("web.dom"),
            key: id("12"),
        }],
        rule: Rule::PropertyEquals {
            field: Field::Value,
            expected: Value::Text("London".into()),
        },
        applies_when: ContextConditions {
            platform: None,
            input_mode: None,
            text_scale: None,
        },
        expected_from: id("explicit-fixture-application"),
    };
    (ActionCase { snapshot, action }, expected)
}

#[test]
fn activation_preparation_holds_readonly_input_or_output_result_without_delivery() {
    for input in [false, true] {
        let (fixture, calls) = activation_peer(0, input);
        let mut c = fixture.attach(limits());
        let (case, expected) = activation_case(&mut c);
        let source = case.snapshot.clone();
        let mut request = request();
        request.context = case.snapshot.context.clone();
        request.operation = Operation::Prepare {
            action: case.action,
        };
        let now = uiblueprint_plugin_api::ClockReading {
            domain: id("worker-clock"),
            milliseconds: 1,
        };
        let prepared = collector::WebActionProvider::new(&mut c, request.limits.clone())
            .prepare_exact(&source, &request, Some(&expected), &now, 1500)
            .unwrap();
        assert_eq!(prepared.snapshot.nodes.len(), 2);
        assert_eq!(source, case.snapshot);
        assert_eq!(
            known(&prepared.snapshot.nodes[1], Field::Value),
            &Value::Text("".into())
        );
        assert_eq!(
            prepared.action.resolution.available_intents,
            vec![id("activate")]
        );
        assert_eq!(
            prepared.action.resolution.evidence.method,
            id("dom.HTMLElement.click-native-button-untrusted")
        );
        assert_eq!(calls.load(Ordering::Acquire), 0);
        assert_eq!(
            fixture.methods().last().unwrap(),
            "Runtime.releaseObjectGroup"
        );
        drop(c);
        fixture.finish();
    }
}

#[test]
fn activation_verifies_held_result_after_actor_removal_without_reacquisition() {
    use uiblueprint_plugin_api::actions::*;
    for input in [false, true] {
        let (fixture, calls) = activation_peer(0, input);
        let mut c = fixture.attach(limits());
        let (case, expected) = activation_case(&mut c);
        let mut clock = ActionClock {
            tick: 0,
            cancelled: false,
        };
        let start = clock.now();
        let mut gate = ActionGate::default();
        let mut execution = ActionExecution::prepare_action(
            case,
            expected,
            id("transition"),
            id("step"),
            start,
            1500,
        )
        .unwrap();
        {
            let mut provider = collector::WebActionProvider::new(&mut c, request().limits);
            assert_eq!(
                execution
                    .dispatch(&mut provider, &mut gate, &mut clock)
                    .unwrap(),
                DeliveryStatus::Confirmed
            );
            assert_eq!(
                execution.verify(&mut provider, &mut clock).unwrap(),
                CheckStatus::Pass
            );
            assert!(
                execution
                    .dispatch(&mut provider, &mut gate, &mut clock)
                    .is_err()
            );
        }
        let result = execution.finish().unwrap();
        let after = result.after.unwrap();
        assert_eq!(after.nodes.len(), 1);
        assert_eq!(after.nodes[0].key.key, id("12"));
        assert_eq!(
            known(&after.nodes[0], Field::Value),
            &Value::Text("London".into())
        );
        assert_eq!(result.transition.steps[0].outcome, Outcome::Succeeded);
        assert_eq!(calls.load(Ordering::Acquire), 1);
        assert_eq!(gate.calls, 1);
        drop(c);
        fixture.finish();
    }
}

#[test]
fn activation_refuses_fresh_disabled_private_stale_missing_or_wrong_control_before_permit() {
    use uiblueprint_plugin_api::actions::*;
    for mode in 1..=9 {
        let (fixture, calls) = activation_peer(mode, false);
        let mut c = fixture.attach(limits());
        let (case, expected) = activation_case(&mut c);
        let mut clock = ActionClock {
            tick: 0,
            cancelled: false,
        };
        let start = clock.now();
        let mut gate = ActionGate::default();
        let mut execution = ActionExecution::prepare_action(
            case,
            expected,
            id("transition"),
            id("step"),
            start,
            1500,
        )
        .unwrap();
        {
            let mut provider = collector::WebActionProvider::new(&mut c, request().limits);
            assert_eq!(
                execution
                    .dispatch(&mut provider, &mut gate, &mut clock)
                    .unwrap(),
                DeliveryStatus::NotDispatched,
                "mode {mode}"
            );
        }
        assert_eq!(
            execution.finish().unwrap().transition.steps[0].outcome,
            Outcome::Failed
        );
        assert_eq!(gate.calls, 0);
        assert_eq!(calls.load(Ordering::Acquire), 0);
        drop(c);
        fixture.finish();
    }
}

#[test]
fn activation_result_loss_privacy_unknown_mismatch_or_delivery_loss_never_retries() {
    use uiblueprint_plugin_api::actions::*;
    for mode in 10..=15 {
        let (fixture, calls) = activation_peer(mode, false);
        let mut c = fixture.attach(limits());
        let (case, expected) = activation_case(&mut c);
        let mut clock = ActionClock {
            tick: 0,
            cancelled: false,
        };
        let start = clock.now();
        let mut gate = ActionGate::default();
        let mut execution = ActionExecution::prepare_action(
            case,
            expected,
            id("transition"),
            id("step"),
            start,
            1500,
        )
        .unwrap();
        {
            let mut provider = collector::WebActionProvider::new(&mut c, request().limits);
            let delivery = execution
                .dispatch(&mut provider, &mut gate, &mut clock)
                .unwrap();
            assert_eq!(
                delivery,
                if mode == 15 {
                    DeliveryStatus::Unknown
                } else {
                    DeliveryStatus::Confirmed
                }
            );
            if mode == 15 {
                assert!(
                    execution.verify(&mut provider, &mut clock).is_err(),
                    "uncertain delivery already terminalized"
                );
            } else {
                assert_eq!(
                    execution.verify(&mut provider, &mut clock).unwrap(),
                    if mode == 14 {
                        CheckStatus::Fail
                    } else {
                        CheckStatus::Unknown
                    }
                );
            }
            assert!(
                execution
                    .dispatch(&mut provider, &mut gate, &mut clock)
                    .is_err()
            );
        }
        let result = execution.finish().unwrap();
        assert_ne!(result.transition.steps[0].outcome, Outcome::Succeeded);
        assert!(!serde_json::to_string(&result).unwrap().contains(CANARY));
        assert_eq!(calls.load(Ordering::Acquire), 1);
        assert_eq!(gate.calls, 1);
        drop(c);
        fixture.finish();
    }
}

#[test]
fn activation_prepare_requires_explicit_distinct_present_public_result() {
    for mode in 0..8 {
        let (fixture, calls) = activation_peer(0, false);
        let mut c = fixture.attach(limits());
        let (mut case, mut expected) = activation_case(&mut c);
        if mode == 1 {
            expected.targets[0].key = id("999");
        }
        if mode == 2 {
            expected.targets[0] = case.action.backend_ref.key.clone();
        }
        if mode == 3 {
            expected.rule = Rule::PropertyEquals {
                field: Field::Value,
                expected: Value::Text("x".repeat(601)),
            };
        }
        if matches!(mode, 4 | 6 | 7) {
            let node = case
                .snapshot
                .nodes
                .iter_mut()
                .find(|n| n.key == expected.targets[0])
                .unwrap();
            let Property::Requested {
                sensitivity, state, ..
            } = node
                .properties
                .iter_mut()
                .find(|p| p.field() == Field::Value)
                .unwrap()
            else {
                unreachable!()
            };
            if mode == 4 {
                *sensitivity = Sensitivity::Sensitive;
            }
            *state = if mode == 7 {
                Availability::Unsupported {
                    reason: id("unexposed-result"),
                }
            } else {
                Availability::Redacted {}
            };
        }
        let mut request = request();
        if mode == 5 {
            request.limits.max_elements = 1;
        }
        request.context = case.snapshot.context.clone();
        request.operation = Operation::Prepare {
            action: case.action,
        };
        let count = fixture.methods().len();
        let now = uiblueprint_plugin_api::ClockReading {
            domain: id("worker-clock"),
            milliseconds: 1,
        };
        assert!(
            collector::WebActionProvider::new(&mut c, request.limits.clone())
                .prepare_exact(
                    &case.snapshot,
                    &request,
                    if mode == 0 { None } else { Some(&expected) },
                    &now,
                    1500
                )
                .is_err()
        );
        assert_eq!(fixture.methods().len(), count);
        assert_eq!(calls.load(Ordering::Acquire), 0);
        drop(c);
        fixture.finish();
    }
}

fn popup_geometry_peer(mode: u8) -> Fixture {
    Fixture::new(move |method, command, _| {
        if method != "Runtime.callFunctionOn"
            || !command["params"]["functionDeclaration"]
                .as_str()
                .is_some_and(|s| s.starts_with("function readNode("))
        {
            return None;
        }
        let mut read: Json =
            serde_json::from_str(include_str!("fixtures/collector/dom.json")).unwrap();
        read["result"]["value"]["hit"] = json!({"x":100,"y":80,"matches":mode != 1});
        let bounds = json!({"x":40,"y":60,"width":120,"height":40});
        read["result"]["value"]["clip"] = json!({"rect":{"x":40,"y":60,"width":60,"height":40},
            "bounds":bounds,"current":bounds,"intersects":true,"ratio":0.5});
        match mode {
            2 => read["result"]["value"]["clip"]["ratio"] = json!(2),
            3 => read["result"]["value"]["clip"]["current"]["x"] = json!(41),
            4 => read["result"]["value"]["clip"]["rect"]["width"] = json!(-1),
            5 => read["result"]["value"]["hit"]["x"] = json!("private"),
            6 => read["result"]["value"]["clip"]["bounds"]["width"] = json!(121),
            _ => (),
        }
        Some(read)
    })
}
#[test]
fn popup_geometry_keeps_clipping_and_single_hit_sample_separate_from_visibility() {
    for mode in [0, 1, 7] {
        let fixture = popup_geometry_peer(mode);
        let mut c = fixture.attach(limits());
        let mut r = request();
        r.context.fields = if mode == 7 {
            vec![Field::LayoutBounds]
        } else {
            vec![
                Field::LayoutBounds,
                Field::HitRegion,
                Field::VisibleRegion,
                Field::PaintBounds,
            ]
        };
        let (_, docs) = collect(&mut c, &r, &scope(&[11]));
        let s = snapshot(&docs[0]);
        validation::validate_snapshot(s).unwrap();
        let n = &s.nodes[0];
        assert_eq!(s.coverage.status, CoverageStatus::Partial);
        if mode == 7 {
            assert!(n.extensions.is_empty());
        } else {
            assert_eq!(n.extensions.len(), 5);
            assert!(n.extensions.iter().all(|e| matches!(&e.property, Property::Requested { evidence, .. }
                if evidence.provenance==Provenance::Reported && evidence.source_namespace==id("web.dom"))));
            for field in [Field::VisibleRegion, Field::PaintBounds] {
                assert!(matches!(
                    n.properties.iter().find(|p| p.field() == field).unwrap(),
                    Property::Requested {
                        state: Availability::Unknown { .. },
                        ..
                    }
                ));
            }
            let hit = n
                .properties
                .iter()
                .find(|p| p.field() == Field::HitRegion)
                .unwrap();
            if mode == 0 {
                let Some(Value::Geometry(g)) = hit.known() else {
                    panic!("known sample")
                };
                assert_eq!(
                    g.shape,
                    Shape::Rect(Rect {
                        x: 100.0,
                        y: 80.0,
                        width: 0.0,
                        height: 0.0
                    })
                );
            } else {
                assert!(hit.known().is_none());
            }
            let clipped = n
                .extensions
                .iter()
                .find(|e| e.name == id("intersection_rect_not_occlusion"))
                .unwrap();
            let Some(Value::Geometry(g)) = clipped.property.known() else {
                panic!("clip")
            };
            assert_eq!(
                g.shape,
                Shape::Rect(Rect {
                    x: 40.0,
                    y: 60.0,
                    width: 60.0,
                    height: 40.0
                })
            );
        }
        c.detach();
        drop(c);
        fixture.finish();
    }
}
#[test]
fn popup_geometry_malformed_or_changed_source_cannot_publish() {
    for mode in 2..=6 {
        let fixture = popup_geometry_peer(mode);
        let mut c = fixture.attach(limits());
        let mut r = request();
        r.context.fields = vec![Field::LayoutBounds, Field::HitRegion, Field::VisibleRegion];
        let error = c
            .observe(
                &r,
                &scope(&[11]),
                1,
                Instant::now() + Duration::from_secs(2),
                |_| panic!("no publication"),
            )
            .expect_err("malformed or unstable source");
        assert_eq!(
            error.kind,
            if mode == 3 || mode == 6 {
                ErrorKind::ResyncRequired
            } else {
                ErrorKind::Malformed
            }
        );
        c.detach();
        drop(c);
        fixture.finish();
    }
}
