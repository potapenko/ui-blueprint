//! Bounded owned protocol peer. Synthetic method facts, never a browser.
use serde_json::{Value, json};
use std::{
    net::{Shutdown, TcpListener, TcpStream},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use tungstenite::{Message, accept};
pub const CANARY: &str = "PRIVATE_WEB_WORKER_CANARY";
#[derive(Default)]
pub struct State {
    pub calls: AtomicUsize,
    pub selections: AtomicUsize,
    pub reads: AtomicUsize,
    pub releases: AtomicUsize,
    pub drift: AtomicBool,
    pub stall: AtomicBool,
    pub stalled: AtomicBool,
    pub secret: AtomicBool,
    pub checkbox_native: AtomicBool,
    pub checkbox_enabled: AtomicBool,
    pub checkbox_writable: AtomicBool,
    pub checkbox_checked: AtomicBool,
    pub checkbox_indeterminate: AtomicBool,
    pub setter_calls: AtomicUsize,
    pub wrong_post_checked: AtomicBool,
    pub lost_post_binding: AtomicBool,
    pub events_once: AtomicUsize,
}
pub struct Peer {
    pub url: String,
    pub state: Arc<State>,
    stop: Arc<AtomicBool>,
    socket: Arc<Mutex<Option<TcpStream>>>,
    join: Option<JoinHandle<()>>,
}
impl Peer {
    pub fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("owned peer listener");
        listener.set_nonblocking(true).expect("bounded accept");
        let address = listener.local_addr().expect("address");
        let state = Arc::new(State::default());
        let shared = state.clone();
        let stop = Arc::new(AtomicBool::new(false));
        let stopping = stop.clone();
        let socket = Arc::new(Mutex::new(None));
        let slot = socket.clone();
        let join = thread::spawn(move || {
            let end = Instant::now() + Duration::from_secs(5);
            while !stopping.load(Ordering::Acquire) && Instant::now() < end {
                match listener.accept() {
                    Ok((s, _)) => {
                        s.set_nonblocking(false).expect("blocking owned stream");
                        s.set_read_timeout(Some(Duration::from_secs(3)))
                            .expect("read bound");
                        s.set_write_timeout(Some(Duration::from_secs(3)))
                            .expect("write bound");
                        *slot.lock().expect("slot") = Some(s.try_clone().expect("shutdown handle"));
                        let mut ws = accept(s).expect("synthetic handshake");
                        while let Ok(message) = ws.read() {
                            let Message::Text(text) = message else { break };
                            let command: Value = serde_json::from_str(&text).expect("CDP command");
                            shared.calls.fetch_add(1, Ordering::AcqRel);
                            let method = command["method"].as_str().expect("method");
                            let params = &command["params"];
                            for _ in 0..shared.events_once.swap(0, Ordering::AcqRel) {
                                let event =
                                    json!({"method":"Accessibility.nodesUpdated","params":{}});
                                if ws.send(Message::Text(event.to_string().into())).is_err() {
                                    return;
                                }
                            }
                            if shared.stall.load(Ordering::Acquire)
                                && method == "Runtime.callFunctionOn"
                            {
                                shared.stalled.store(true, Ordering::Release);
                                let _ = ws.read();
                                break;
                            }
                            let result = match method {
                                "Target.getTargetInfo" => {
                                    json!({"targetInfo":{"targetId":"web-target"}})
                                }
                                "Page.getFrameTree" => {
                                    json!({"frameTree":{"frame":{"id":"frame","loaderId":if shared.drift.load(Ordering::Acquire){"changed-loader"}else{"loader"}}}})
                                }
                                "DOM.getDocument" => {
                                    assert_eq!(params["depth"], 0);
                                    assert_eq!(params["pierce"], false);
                                    json!({"root":{"backendNodeId":1,"nodeType":9}})
                                }
                                "Page.createIsolatedWorld" => {
                                    assert_eq!(params["grantUniveralAccess"], false);
                                    json!({"executionContextId":3})
                                }
                                "Accessibility.enable" => json!({}),
                                "Runtime.releaseObjectGroup" => {
                                    shared.releases.fetch_add(1, Ordering::AcqRel);
                                    json!({})
                                }
                                "DOM.resolveNode" => {
                                    json!({"object":{"type":"object","subtype":"node","objectId":format!("node-{}",params["backendNodeId"])}})
                                }
                                "DOM.describeNode" => {
                                    assert_eq!(params["depth"], 0);
                                    assert_eq!(params["pierce"], false);
                                    json!({"node":{"backendNodeId":11,"nodeType":1,"attributes":["value",CANARY]}})
                                }
                                "Runtime.callFunctionOn" => {
                                    assert_eq!(params["userGesture"], false);
                                    let function = params["functionDeclaration"]
                                        .as_str()
                                        .expect("fixed function");
                                    assert_eq!(
                                        params["throwOnSideEffect"],
                                        function.starts_with("function selectIds(")
                                            || function.starts_with("function verifyNodes(")
                                    );
                                    if function.starts_with("function selectIds(") {
                                        shared.selections.fetch_add(1, Ordering::AcqRel);
                                        assert_eq!(
                                            params["arguments"][0]["value"]["ids"],
                                            json!(["left"])
                                        );
                                        json!({"result":{"type":"object","objectId":"selection-result"}})
                                    } else if function.starts_with("function verifyNodes(") {
                                        json!({"result":{"type":"object","value":{"current":true}}})
                                    } else if function.starts_with("function checkboxState(") {
                                        let checked = shared
                                            .checkbox_checked
                                            .load(Ordering::Acquire)
                                            && !(shared.wrong_post_checked.load(Ordering::Acquire)
                                                && shared.setter_calls.load(Ordering::Acquire) > 0);
                                        let connected =
                                            !(shared.lost_post_binding.load(Ordering::Acquire)
                                                && shared.setter_calls.load(Ordering::Acquire) > 0);
                                        json!({"result":{"type":"object","value":{"connected":connected,"sameDocument":true,
                                            "nativeCheckbox":shared.checkbox_native.load(Ordering::Acquire),
                                            "writable":shared.checkbox_writable.load(Ordering::Acquire),"sensitive":false,
                                            "enabled":shared.checkbox_enabled.load(Ordering::Acquire),"checked":checked,
                                            "indeterminate":shared.checkbox_indeterminate.load(Ordering::Acquire)}}})
                                    } else if function.starts_with("function setChecked(") {
                                        assert_eq!(params["objectId"], "node-11");
                                        assert!(params["arguments"][1]["value"].is_boolean());
                                        shared.setter_calls.fetch_add(1, Ordering::AcqRel);
                                        shared.checkbox_checked.store(
                                            params["arguments"][1]["value"].as_bool().unwrap(),
                                            Ordering::Release,
                                        );
                                        json!({"result":{"type":"object","value":{"status":"applied"}}})
                                    } else {
                                        shared.reads.fetch_add(1, Ordering::AcqRel);
                                        let connected = params["objectId"] == "node-11";
                                        let private = shared.secret.load(Ordering::Acquire);
                                        json!({"result":{"type":"object","value":{"connected":connected,"sameDocument":true,"tag":"INPUT","sensitive":private,"rect":{"x":40,"y":60,"width":120,"height":40},"value":if private{CANARY}else{""},"checked":shared.checkbox_checked.load(Ordering::Acquire),"enabled":true}}})
                                    }
                                }
                                "Runtime.getProperties" => {
                                    assert_eq!(params["ownProperties"], true);
                                    assert_eq!(params["generatePreview"], false);
                                    json!({"result":[{"name":"status","value":{"type":"string","value":"selected"}},{"name":"visited","value":{"type":"number","value":3}},{"name":"node_0","value":{"type":"object","subtype":"node","objectId":"node-11"}}]})
                                }
                                "Accessibility.getPartialAXTree" => {
                                    assert_eq!(params["backendNodeId"], 11);
                                    assert_eq!(params["fetchRelatives"], false);
                                    json!({"nodes":[{"nodeId":"ax-11","ignored":false,"backendDOMNodeId":11,"frameId":"frame","role":{"type":"role","value":"checkbox"},"name":{"type":"computedString","value":"Apply"},"properties":[{"name":"checked","value":{"type":"tristate","value":"false"}}]}]})
                                }
                                _ => panic!("unexpected protocol method"),
                            };
                            let mut response = json!({"id":command["id"],"result":result});
                            if let Some(session) = command.get("sessionId") {
                                response["sessionId"] = session.clone();
                            }
                            if ws.send(Message::Text(response.to_string().into())).is_err() {
                                break;
                            }
                        }
                        break;
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(1))
                    }
                    Err(e) => panic!("owned accept failed: {e:?}"),
                }
            }
        });
        Self {
            url: format!("ws://{address}/fixture"),
            state,
            stop,
            socket,
            join: Some(join),
        }
    }
}
impl Drop for Peer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(s) = self.socket.lock().expect("slot").take() {
            let _ = s.shutdown(Shutdown::Both);
        }
        if let Some(join) = self.join.take() {
            let deadline = Instant::now() + Duration::from_secs(4);
            while !join.is_finished() && Instant::now() < deadline {
                thread::sleep(Duration::from_millis(1));
            }
            if !join.is_finished() {
                if !thread::panicking() {
                    panic!("owned peer failed bounded shutdown")
                };
                return;
            }
            let done = join.join();
            if !thread::panicking() {
                done.expect("owned peer assertions");
            }
        }
    }
}
