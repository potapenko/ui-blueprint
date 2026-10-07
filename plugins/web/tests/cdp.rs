use serde_json::{Value, json};
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
use tungstenite::{Message, WebSocket, accept};
use uiblueprint_schema::model::{Id, Identity};
use uiblueprint_web::{
    cdp::*,
    transport::{self, OperationLimits, SendProgress},
};

const CANARY: &str = "PRIVATE_CDP_CANARY";
struct Capture(AtomicBool);
static LOGS: Capture = Capture(AtomicBool::new(false));
static INIT: Once = Once::new();
impl log::Log for Capture {
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
        transport::install_log_boundary(&LOGS).expect("filtered logger");
        log::set_max_level(log::LevelFilter::Trace);
    });
}
fn limits() -> Limits {
    Limits {
        max_request_bytes: 1024,
        max_message_bytes: 2048,
        max_metadata_bytes: 128,
        max_results: 2,
        result_bytes: 4096,
        max_events: 2,
        event_bytes: 4352,
    }
}
fn op(ms: u64) -> OperationLimits {
    OperationLimits {
        deadline: Instant::now() + Duration::from_millis(ms),
        max_read_bytes: 16384,
        max_write_bytes: 16384,
        max_work: 1024,
    }
}
fn binding(session: Option<&str>, generation: &str) -> Binding {
    Binding {
        target: Identity {
            id: Id("owned-target".into()),
            generation: Id(generation.into()),
        },
        cdp_session_id: session.map(|s| Id(s.into())),
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
fn connect(peer: &Peer) -> transport::Transport {
    logging();
    transport::Transport::connect(
        &peer.url,
        transport::Limits {
            endpoint_bytes: 1024,
            handshake_bytes: 2048,
            read_buffer_bytes: 64,
            write_buffer_bytes: 64,
            write_buffer_max: 4096,
            frame_bytes: 2048,
            message_bytes: 2048,
            outbound_bytes: 1024,
        },
        op(700),
    )
    .expect("owned connect")
}
fn client(peer: &Peer, session: Option<&str>, generation: &str, caps: Limits) -> Client {
    Client::new(connect(peer), binding(session, generation), caps).expect("bound client")
}
fn command(ws: &mut WebSocket<TcpStream>) -> Value {
    serde_json::from_str(&ws.read().expect("one command").into_text().expect("text"))
        .expect("command JSON")
}
fn send(ws: &mut WebSocket<TcpStream>, v: Value) {
    ws.send(Message::Text(v.to_string().into()))
        .expect("peer send");
}
fn answer(ws: &mut WebSocket<TcpStream>, c: &Value) {
    let mut r = json!({"id":c["id"],"result":{"payload":CANARY}});
    if let Some(s) = c.get("sessionId") {
        r["sessionId"] = s.clone();
    }
    send(ws, r);
}
fn request(c: &mut Client) -> Reply {
    c.prepare("Runtime.fixture", &json!({}), op(700))
        .expect("registered")
        .run()
        .expect("correlated reply")
}
fn refused(c: &mut Client) -> ErrorKind {
    c.prepare("Runtime.fixture", &json!({}), op(700))
        .err()
        .expect("dispatch refused")
        .kind
}

#[test]
fn reply_interleaved_event_exact_wire_and_private_diagnostics() {
    let peer = Peer::new(|s| {
        let mut ws = accept(s).expect("handshake");
        let c = command(&mut ws);
        assert_eq!(c["sessionId"], CANARY);
        assert_eq!(c["method"], "Runtime.fixture");
        send(
            &mut ws,
            json!({"method":CANARY,"params":{"secret":CANARY},"sessionId":CANARY}),
        );
        let raw = format!(
            "{{ \"sessionId\":\"{CANARY}\", \"result\":{{\"v\":\"{CANARY}\"}}, \"id\":{} }}",
            c["id"]
        );
        ws.send(Message::Text(raw.into()))
            .expect("exact raw result");
    });
    let mut c = client(&peer, Some(CANARY), "generation-a", limits());
    let pending = c
        .prepare("Runtime.fixture", &json!({}), op(700))
        .expect("prepare");
    let ticket = pending.ticket().clone();
    let reply = pending.run().expect("reply");
    assert_eq!(reply.kind(), ReplyKind::Result);
    assert!(c.owns_ticket(&ticket));
    assert_eq!(reply.ticket().wire_id(), ticket.wire_id());
    assert!(reply.wire().starts_with("{ \"sessionId\""));
    let event = c.pop_event().expect("event separate");
    assert_eq!(event.method(), CANARY);
    assert!(event.wire().contains(CANARY));
    assert_eq!(event.binding().target.generation.0, "generation-a");
    for d in [
        format!("{reply:?}"),
        format!("{event:?}"),
        format!("{ticket:?}"),
        format!("{:?}", ticket.binding()),
        format!("{c:?}"),
    ] {
        assert!(!d.contains(CANARY));
    }
    assert!(!LOGS.0.load(Ordering::Relaxed));
    c.detach();
    assert!(reply.wire().contains(CANARY));
    assert_eq!(c.result_usage().slots, 1);
    drop(reply);
    assert_eq!(c.result_usage().slots, 0);
    peer.finish();
}

#[test]
fn caller_held_results_reserve_before_dispatch_and_release_on_drop() {
    let peer = Peer::new(|s| {
        let mut ws = accept(s).expect("handshake");
        for _ in 0..2 {
            let c = command(&mut ws);
            answer(&mut ws, &c);
        }
    });
    let mut caps = limits();
    caps.max_results = 1;
    caps.result_bytes = caps.max_message_bytes;
    let mut c = client(&peer, None, "g", caps);
    let first = request(&mut c);
    assert_eq!(
        c.result_usage(),
        Usage {
            slots: 1,
            reserved_bytes: caps.max_message_bytes
        }
    );
    assert_eq!(refused(&mut c), ErrorKind::Budget);
    drop(first);
    let second = request(&mut c);
    assert_eq!(second.kind(), ReplyKind::Result);
    drop(c);
    assert!(second.wire().contains(CANARY));
    peer.finish();
}

#[test]
fn caller_held_events_count_and_loss_requires_explicit_consumer_resync() {
    let peer = Peer::new(|s| {
        let mut ws = accept(s).expect("handshake");
        for n in 0..3 {
            let c = command(&mut ws);
            send(
                &mut ws,
                json!({"method":"Runtime.changed","params":{"n":n}}),
            );
            answer(&mut ws, &c);
        }
    });
    let mut caps = limits();
    caps.max_events = 1;
    let mut c = client(&peer, None, "g", caps);
    drop(request(&mut c));
    let event = c.pop_event().expect("first event");
    assert_eq!(c.event_usage().slots, 1);
    let r = request(&mut c);
    assert_eq!(r.event_loss_generation(), 1);
    drop(r);
    assert!(c.pop_event().is_none());
    assert_eq!(
        c.take_event_loss(),
        Some(EventLoss {
            generation: 1,
            dropped: 1
        })
    );
    assert_eq!(c.take_event_loss(), None);
    drop(event);
    assert_eq!(c.event_usage().slots, 0);
    drop(request(&mut c));
    assert_eq!(c.event_loss_generation(), 1);
    assert!(c.pop_event().is_some());
    peer.finish();
}

#[test]
fn protocol_error_is_data_but_uncorrelated_error_cannot_complete() {
    let peer = Peer::new(|s| {
        let mut ws = accept(s).expect("handshake");
        let c = command(&mut ws);
        send(
            &mut ws,
            json!({"id":c["id"],"error":{"code":-32000,"message":CANARY,"data":CANARY}}),
        );
        let _ = command(&mut ws);
        send(&mut ws, json!({"error":{"code":-32700,"message":CANARY}}));
    });
    let mut c = client(&peer, None, "g", limits());
    let first = request(&mut c);
    assert_eq!(first.kind(), ReplyKind::Error { code: -32000 });
    assert!(!format!("{first:?}").contains(CANARY));
    let error = c
        .prepare("Runtime.fixture", &json!({}), op(700))
        .expect("prepare")
        .run()
        .expect_err("uncorrelated refusal");
    assert_eq!(error.kind, ErrorKind::UncorrelatedError(-32700));
    assert_eq!(error.send_progress, SendProgress::Flushed);
    assert!(!format!("{error:?} {error}").contains(CANARY));
    assert!(first.wire().contains(CANARY));
    assert_eq!(refused(&mut c), ErrorKind::Detached);
    peer.finish();
}

#[test]
fn wrong_missing_sessions_and_unknown_ids_fail_closed() {
    for mode in 0..4 {
        let peer = Peer::new(move |s| {
            let mut ws = accept(s).expect("handshake");
            let c = command(&mut ws);
            let r = match mode {
                0 => json!({"id":c["id"],"result":{},"sessionId":"other"}),
                1 => json!({"id":c["id"],"result":{}}),
                2 => {
                    json!({"id":c["id"].as_u64().expect("ID")+1,"result":{},"sessionId":"session"})
                }
                _ => json!({"result":{},"sessionId":"session"}),
            };
            send(&mut ws, r);
        });
        let mut c = client(&peer, Some("session"), "g", limits());
        let error = c
            .prepare("Runtime.fixture", &json!({}), op(700))
            .expect("prepare")
            .run()
            .expect_err("bad routing");
        assert_eq!(
            error.kind,
            match mode {
                0 | 1 => ErrorKind::WrongSession,
                2 => ErrorKind::UnexpectedReply,
                _ => ErrorKind::Envelope,
            }
        );
        assert_eq!(refused(&mut c), ErrorKind::Detached);
        peer.finish();
    }
}

#[test]
fn duplicate_reply_cannot_satisfy_next_ticket_or_damage_prior_result() {
    let peer = Peer::new(|s| {
        let mut ws = accept(s).expect("handshake");
        let old = command(&mut ws);
        answer(&mut ws, &old);
        let new = command(&mut ws);
        assert_ne!(old["id"], new["id"]);
        answer(&mut ws, &old);
    });
    let mut c = client(&peer, None, "g", limits());
    let first = request(&mut c);
    let error = c
        .prepare("Runtime.fixture", &json!({}), op(700))
        .expect("next ticket")
        .run()
        .expect_err("late duplicate");
    assert_eq!(error.kind, ErrorKind::UnexpectedReply);
    assert_eq!(refused(&mut c), ErrorKind::Detached);
    assert!(first.wire().contains(CANARY));
    assert_eq!(c.result_usage().slots, 1);
    peer.finish();
}

#[test]
fn cancel_and_drop_before_dispatch_emit_no_command() {
    for explicit in [false, true] {
        let peer = Peer::new(|s| {
            let mut ws = accept(s).expect("handshake");
            assert!(ws.read().is_err(), "no command after cancellation");
        });
        let mut c = client(&peer, None, "g", limits());
        let p = c
            .prepare("Runtime.fixture", &json!({}), op(700))
            .expect("prepare");
        if explicit {
            let e = p.cancel();
            assert_eq!(e.kind, ErrorKind::Cancelled);
            assert_eq!(e.send_progress, SendProgress::NotQueued);
        } else {
            drop(p);
        }
        assert_eq!(c.result_usage().slots, 0);
        assert_eq!(refused(&mut c), ErrorKind::Detached);
        peer.finish();
    }
}

#[test]
fn cancelled_wait_preserves_identity_and_independent_connection_progress() {
    let (sent, received) = mpsc::channel();
    let peer = Peer::new(move |s| {
        let mut ws = accept(s).expect("handshake");
        let _ = command(&mut ws);
        sent.send(()).expect("receipt");
        assert!(
            ws.read().is_err(),
            "cancel must close socket without another command"
        );
    });
    let mut c = client(&peer, None, "old-generation", limits());
    let cancel = c.cancellation().expect("handle");
    let pending = c
        .prepare("Runtime.fixture", &json!({}), op(1200))
        .expect("prepare");
    let old_ticket = pending.ticket().clone();
    thread::scope(|scope| {
        let waiter = scope.spawn(move || pending.run());
        received
            .recv_timeout(Duration::from_secs(1))
            .expect("sent before cancel");
        let other = Peer::new(|s| {
            let mut ws = accept(s).expect("handshake");
            let c = command(&mut ws);
            answer(&mut ws, &c);
        });
        let mut fresh = client(&other, None, "new-generation", limits());
        let reply = request(&mut fresh);
        assert!(!fresh.owns_ticket(&old_ticket));
        assert_ne!(reply.ticket().wire_id(), old_ticket.wire_id());
        assert_eq!(
            reply.ticket().binding().target.generation.0,
            "new-generation"
        );
        other.finish();
        cancel.cancel();
        let error = waiter
            .join()
            .expect("bounded waiter")
            .expect_err("cancelled");
        assert_eq!(error.kind, ErrorKind::Cancelled);
        assert_eq!(error.send_progress, SendProgress::Flushed);
    });
    assert_eq!(refused(&mut c), ErrorKind::Detached);
    peer.finish();
}

#[test]
fn original_deadline_covers_delayed_dispatch_and_wait() {
    let peer = Peer::new(|s| {
        let mut ws = accept(s).expect("handshake");
        assert!(ws.read().is_err(), "expired prepared request not sent");
    });
    let mut c = client(&peer, None, "g", limits());
    let p = c
        .prepare("Runtime.fixture", &json!({}), op(10))
        .expect("prepare");
    thread::sleep(Duration::from_millis(20));
    let e = p.run().expect_err("original deadline");
    assert_eq!(e.kind, ErrorKind::Transport(transport::ErrorKind::Timeout));
    assert_eq!(e.send_progress, SendProgress::NotQueued);
    assert_eq!(refused(&mut c), ErrorKind::Detached);
    peer.finish();
    let peer = Peer::new(|s| {
        let mut ws = accept(s).expect("handshake");
        let _ = command(&mut ws);
        assert!(ws.read().is_err(), "timeout closes, no retry");
    });
    let mut c = client(&peer, None, "g", limits());
    let e = c
        .prepare("Runtime.fixture", &json!({}), op(50))
        .expect("prepare")
        .run()
        .expect_err("bounded wait");
    assert_eq!(e.kind, ErrorKind::Transport(transport::ErrorKind::Timeout));
    assert_eq!(e.send_progress, SendProgress::Flushed);
    assert_eq!(refused(&mut c), ErrorKind::Detached);
    peer.finish();
}

#[test]
fn malformed_response_and_peer_close_invalidate_pending() {
    for malformed in [true, false] {
        let peer = Peer::new(move |s| {
            let mut ws = accept(s).expect("handshake");
            let c = command(&mut ws);
            if malformed {
                send(&mut ws, json!({"id":c["id"],"result":null}));
            } else {
                ws.close(None).expect("close");
            }
        });
        let mut c = client(&peer, None, "g", limits());
        let e = c
            .prepare("Runtime.fixture", &json!({}), op(700))
            .expect("prepare")
            .run()
            .expect_err("invalid completion");
        assert_eq!(
            e.kind,
            if malformed {
                ErrorKind::Envelope
            } else {
                ErrorKind::Detached
            }
        );
        assert_eq!(c.result_usage().slots, 0);
        peer.finish();
    }
}

#[test]
fn invalid_encoding_does_not_poison_connection_or_send_null_params() {
    let peer = Peer::new(|s| {
        let mut ws = accept(s).expect("handshake");
        let c = command(&mut ws);
        assert_eq!(c["params"], json!({}));
        answer(&mut ws, &c);
    });
    let mut c = client(&peer, None, "g", limits());
    assert_eq!(
        c.prepare("Runtime.fixture", &(), op(700))
            .err()
            .expect("null rejected")
            .kind,
        ErrorKind::Envelope
    );
    assert_eq!(
        c.prepare(
            "Runtime.fixture",
            &json!({"long":"x".repeat(1200)}),
            op(700)
        )
        .err()
        .expect("bounded writer")
        .kind,
        ErrorKind::Encoding
    );
    assert_eq!(c.result_usage().slots, 0);
    drop(request(&mut c));
    peer.finish();
}

#[test]
fn invalid_binding_and_limits_close_taken_socket_before_dispatch() {
    for bad_identity in [true, false] {
        let peer = Peer::new(|s| {
            let mut ws = accept(s).expect("handshake");
            assert!(
                ws.read().is_err(),
                "invalid client must close without sending"
            );
        });
        let mut caps = limits();
        if !bad_identity {
            caps.max_events = 0;
        }
        let result = Client::new(
            connect(&peer),
            binding(None, if bad_identity { "" } else { "g" }),
            caps,
        );
        assert_eq!(
            result.expect_err("invalid client").kind,
            if bad_identity {
                ErrorKind::InvalidBinding
            } else {
                ErrorKind::InvalidLimits
            }
        );
        peer.finish();
    }
}
