use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    sync::{
        Arc, Mutex, Once,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use tungstenite::{
    Message, accept, accept_hdr,
    protocol::frame::{
        Frame,
        coding::{Data, OpCode},
    },
};
use uiblueprint_web::transport::*;

const CANARY: &str = "PRIVATE_TRANSPORT_CANARY";
struct Capture(Mutex<Vec<String>>);
static LOGS: Capture = Capture(Mutex::new(Vec::new()));
static INIT: Once = Once::new();
impl log::Log for Capture {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }
    fn log(&self, r: &log::Record<'_>) {
        self.0
            .lock()
            .expect("logs")
            .push(format!("{} {}", r.target(), r.args()));
    }
    fn flush(&self) {}
}
fn logging() {
    INIT.call_once(|| {
        install_log_boundary(&LOGS).expect("install filtered host logger");
        log::set_max_level(log::LevelFilter::Trace);
    });
}
fn limits() -> Limits {
    Limits {
        endpoint_bytes: 1024,
        handshake_bytes: 2048,
        read_buffer_bytes: 64,
        write_buffer_bytes: 64,
        write_buffer_max: 2048,
        frame_bytes: 512,
        message_bytes: 768,
        outbound_bytes: 512,
    }
}
fn op(ms: u64) -> OperationLimits {
    OperationLimits {
        deadline: Instant::now() + Duration::from_millis(ms),
        max_read_bytes: 8192,
        max_write_bytes: 8192,
        max_work: 512,
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
fn connect(p: &Peer) -> Transport {
    logging();
    Transport::connect(&p.url, limits(), op(700)).expect("owned transport connect")
}

#[test]
fn text_control_and_sanitized_logging() {
    let peer = Peer::new(|stream| {
        let mut ws = accept(stream).expect("handshake");
        assert_eq!(
            ws.read().expect("command").into_text().expect("text"),
            "one command"
        );
        ws.send(Message::Ping(CANARY.as_bytes().to_vec().into()))
            .expect("ping");
        assert!(ws.read().expect("pong").is_pong());
        ws.send(Message::text(CANARY)).expect("text");
        ws.close(Some(tungstenite::protocol::CloseFrame {
            code: tungstenite::protocol::frame::coding::CloseCode::Normal,
            reason: CANARY.into(),
        }))
        .expect("close");
        let _ = ws.read();
    });
    let mut transport = connect(&peer);
    let mut action = transport.operation(op(500)).expect("operation");
    assert_eq!(
        action.send_text("one command").expect("send"),
        SendStatus::Flushed
    );
    assert!(matches!(action.receive().expect("ping"), Event::Ping));
    let event = action.receive().expect("text");
    assert!(!format!("{event:?}").contains(CANARY));
    assert!(matches!(event,Event::Text(s) if s==CANARY));
    assert!(matches!(action.receive().expect("close"), Event::PeerClose));
    drop(action);
    drop(transport);
    peer.finish();
    log::info!(target:"host", "sanitized-host-marker");
    let logs = LOGS.0.lock().expect("logs");
    assert!(logs.iter().any(|s| s.contains("sanitized-host-marker")));
    assert!(
        !logs
            .iter()
            .any(|s| s.contains(CANARY) || s.starts_with("tungstenite"))
    );
}
#[test]
fn endpoint_and_invalid_limits_fail_before_connect() {
    logging();
    let mut invalid = limits();
    invalid.write_buffer_max = invalid.write_buffer_bytes;
    assert_eq!(
        Transport::connect("private", invalid, op(10))
            .unwrap_err()
            .kind,
        ErrorKind::InvalidLimits
    );
    for endpoint in [
        "wss://127.0.0.1:123/",
        "ws://localhost:123/",
        "ws://192.0.2.1:123/",
        "ws://user:secret@127.0.0.1:123/",
        "ws://127.0.0.1:0/",
        "ws://127.0.0.1:123/#fragment",
    ] {
        let e = Transport::connect(endpoint, limits(), op(10)).unwrap_err();
        assert_eq!(e.kind, ErrorKind::EndpointRejected);
        assert!(!format!("{e:?}").contains("secret"));
    }
    let mut budget = op(10);
    budget.max_work = 0;
    assert_eq!(
        Transport::connect("private", limits(), budget)
            .unwrap_err()
            .kind,
        ErrorKind::InvalidLimits
    );
    let mut overflow = limits();
    overflow.outbound_bytes = usize::MAX;
    assert_eq!(
        Transport::connect("private", overflow, op(10))
            .unwrap_err()
            .kind,
        ErrorKind::InvalidLimits
    );
}
// Upstream Callback fixes this ErrorResponse type; boxing it would change that API.
#[allow(clippy::result_large_err)]
fn extension_response(
    _: &tungstenite::handshake::server::Request,
    mut response: tungstenite::handshake::server::Response,
) -> Result<tungstenite::handshake::server::Response, tungstenite::handshake::server::ErrorResponse>
{
    response
        .headers_mut()
        .insert("sec-websocket-extensions", CANARY.parse().expect("header"));
    Ok(response)
}

#[test]
fn unsolicited_extension_and_bad_handshake_are_sanitized() {
    logging();
    let peer = Peer::new(|s| {
        let _ = accept_hdr(s, extension_response);
    });
    let e = Transport::connect(&peer.url, limits(), op(500)).unwrap_err();
    assert_eq!(e.kind, ErrorKind::Extensions);
    assert!(!format!("{e}").contains(CANARY));
    peer.finish();
    let peer = Peer::new(|mut s| {
        let mut request = [0; 2048];
        let _ = s.read(&mut request);
        s.write_all(format!("HTTP/1.1 400 {CANARY}\r\nX-Private: {CANARY}\r\n\r\n").as_bytes())
            .expect("response");
    });
    let e = Transport::connect(&peer.url, limits(), op(500)).unwrap_err();
    assert_eq!(e.kind, ErrorKind::Handshake);
    assert!(!format!("{e:?} {e}").contains(CANARY));
    peer.finish();
}
#[test]
fn slow_drip_uses_absolute_handshake_deadline() {
    logging();
    let peer = Peer::new(|mut s| {
        let mut request = [0; 2048];
        let _ = s.read(&mut request);
        for b in b"HTTP/1.1 101 Switching Protocols\r\n" {
            if s.write_all(&[*b]).is_err() {
                break;
            }
            thread::sleep(Duration::from_millis(8));
        }
    });
    let start = Instant::now();
    let e = Transport::connect(&peer.url, limits(), op(70)).unwrap_err();
    assert_eq!(e.kind, ErrorKind::Timeout);
    assert!(start.elapsed() < Duration::from_millis(400));
    peer.finish();
}
#[test]
fn frame_fragment_binary_and_protocol_rejections() {
    for variant in 0..4 {
        let peer = Peer::new(move |s| {
            let mut ws = accept(s).expect("handshake");
            match variant {
                0 => {
                    let _ = ws.send(Message::text("x".repeat(600)));
                }
                1 => {
                    for final_frame in [false, true] {
                        let f = Frame::message(
                            vec![b'x'; 400],
                            OpCode::Data(if final_frame {
                                Data::Continue
                            } else {
                                Data::Text
                            }),
                            final_frame,
                        );
                        if ws.send(Message::Frame(f)).is_err() {
                            break;
                        }
                    }
                }
                2 => {
                    let _ = ws.send(Message::Binary(vec![1, 2].into()));
                }
                _ => {
                    let _ = ws.get_mut().write_all(&[0x83, 0]);
                }
            }
        });
        let mut t = connect(&peer);
        let e = t
            .operation(op(500))
            .expect("operation")
            .receive()
            .expect_err("rejected");
        assert_eq!(
            e.kind,
            match variant {
                0 | 1 => ErrorKind::Capacity,
                2 => ErrorKind::Binary,
                _ => ErrorKind::Protocol,
            }
        );
        drop(t);
        peer.finish();
    }
}
#[test]
fn outbound_overflow_and_no_phantom_flush() {
    let peer = Peer::new(|s| {
        let mut ws = accept(s).expect("handshake");
        assert_eq!(
            ws.read()
                .expect("only permitted command")
                .into_text()
                .expect("text"),
            "ok"
        );
    });
    let mut t = connect(&peer);
    let mut action = t.operation(op(500)).expect("operation");
    assert_eq!(
        action.resume_send().unwrap_err().kind,
        ErrorKind::NoPendingWrite
    );
    let e = action.send_text(&"x".repeat(513)).unwrap_err();
    assert_eq!(e.kind, ErrorKind::Capacity);
    assert_eq!(e.send_progress, SendProgress::NotQueued);
    assert_eq!(action.send_text("ok").expect("send"), SendStatus::Flushed);
    drop(action);
    drop(t);
    peer.finish();
}
#[test]
fn cancellation_wakes_blocked_read_and_other_target_progresses() {
    let blocked = Peer::new(|s| {
        let mut ws = accept(s).expect("handshake");
        let _ = ws.read();
    });
    let mut t = connect(&blocked);
    let cancel = t.cancellation();
    let (tx, rx) = mpsc::channel();
    let worker = thread::spawn(move || {
        let e = t
            .operation(op(1500))
            .expect("operation")
            .receive()
            .expect_err("cancelled");
        tx.send(e).expect("result");
    });
    let independent = Peer::new(|s| {
        let mut ws = accept(s).expect("handshake");
        ws.send(Message::text("independent")).expect("response");
    });
    let mut second = connect(&independent);
    assert!(
        matches!(second.operation(op(200)).expect("operation").receive().expect("independent"),Event::Text(s)if s=="independent")
    );
    drop(second);
    independent.finish();
    let start = Instant::now();
    cancel.cancel();
    let e = rx
        .recv_timeout(Duration::from_millis(500))
        .expect("read woke");
    assert_eq!(e.kind, ErrorKind::Cancelled);
    assert!(start.elapsed() < Duration::from_millis(500));
    worker.join().expect("worker");
    blocked.finish();
    assert!(cancel.is_cancelled());
}
#[test]
fn close_without_server_eof_is_bounded() {
    let peer = Peer::new(|s| {
        let mut ws = accept(s).expect("handshake");
        assert!(ws.read().expect("close request").is_close());
        let _ = ws.flush();
        thread::sleep(Duration::from_millis(250));
    });
    let mut t = connect(&peer);
    let start = Instant::now();
    let e = t
        .operation(op(60))
        .expect("operation")
        .close()
        .expect_err("server holds EOF");
    assert_eq!(e.kind, ErrorKind::Timeout);
    assert!(start.elapsed() < Duration::from_millis(400));
    drop(t);
    peer.finish();
}
#[test]
fn operation_byte_and_work_caps() {
    for work_limit in [false, true] {
        let peer = Peer::new(|s| {
            let mut ws = accept(s).expect("handshake");
            let _ = ws.send(Message::text("x".repeat(300)));
        });
        let mut t = connect(&peer);
        let mut budget = op(500);
        if work_limit {
            budget.max_work = 1;
        } else {
            budget.max_read_bytes = 20;
        }
        let mut action = t.operation(budget).expect("operation");
        let first = action.receive();
        let e = match first {
            Err(e) => e,
            Ok(_) if work_limit => action.receive().expect_err("next work step refused"),
            Ok(_) => panic!("byte limit bypassed"),
        };
        assert_eq!(
            e.kind,
            if work_limit {
                ErrorKind::WorkLimit
            } else {
                ErrorKind::ByteLimit
            }
        );
        drop(action);
        drop(t);
        peer.finish();
    }
}

#[test]
fn cancel_before_and_during_handshake() {
    logging();
    let attempt = Connecting::new(limits()).expect("attempt");
    let cancel = attempt.cancellation();
    cancel.cancel();
    let listener = TcpListener::bind("127.0.0.1:0").expect("owned cancellation endpoint");
    let cancelled_url = format!("ws://{}/not-used", listener.local_addr().expect("addr"));
    assert_eq!(
        attempt.connect(&cancelled_url, op(100)).unwrap_err().kind,
        ErrorKind::Cancelled
    );
    let (ready_tx, ready_rx) = mpsc::channel();
    let peer = Peer::new(move |mut stream| {
        let mut bytes = [0; 2048];
        assert!(stream.read(&mut bytes).expect("handshake received") > 0);
        ready_tx.send(()).expect("ready");
        let _ = stream.read(&mut bytes);
    });
    let attempt = Connecting::new(limits()).expect("attempt");
    let cancel = attempt.cancellation();
    let url = peer.url.clone();
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        tx.send(attempt.connect(&url, op(1500)).unwrap_err())
            .expect("send result");
    });
    ready_rx
        .recv_timeout(Duration::from_millis(500))
        .expect("peer accepted handshake");
    cancel.cancel();
    assert_eq!(
        rx.recv_timeout(Duration::from_millis(500))
            .expect("handshake cancellation wakes")
            .kind,
        ErrorKind::Cancelled
    );
    handle.join().expect("worker");
    peer.finish();
}

#[test]
fn foreign_logger_is_refused_without_replacement() {
    const CHILD: &str = "UIB_TRANSPORT_LOG_TEST_CHILD";
    if std::env::var_os(CHILD).is_some() {
        log::set_logger(&LOGS).expect("foreign logger");
        log::set_max_level(log::LevelFilter::Trace);
        assert_eq!(
            install_log_boundary(&LOGS).unwrap_err().kind,
            ErrorKind::LoggingBoundary
        );
        // Fails before connection attempt construction, no network endpoint required.
        assert!(matches!(
            Connecting::new(limits()),
            Err(Failure {
                kind: ErrorKind::LoggingBoundary,
                ..
            })
        ));
        log::info!(target:"foreign-host", "logger-still-ours");
        assert!(
            LOGS.0
                .lock()
                .expect("logs")
                .iter()
                .any(|s| s.contains("logger-still-ours"))
        );
        return;
    }
    let mut child = std::process::Command::new(std::env::current_exe().expect("test executable"))
        .args(["--exact", "foreign_logger_is_refused_without_replacement"])
        .env(CHILD, "1")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("owned child");
    let end = Instant::now() + Duration::from_secs(3);
    loop {
        if let Some(status) = child.try_wait().expect("wait") {
            assert!(status.success());
            break;
        }
        if Instant::now() >= end {
            child.kill().expect("kill owned child");
            child.wait().expect("reap");
            panic!("logging isolation test deadline");
        }
        thread::sleep(Duration::from_millis(1));
    }
}
