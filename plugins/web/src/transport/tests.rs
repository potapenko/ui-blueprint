use super::*;
use std::{
    collections::VecDeque,
    io::{self as stdio, Cursor, Read, Write},
    sync::{Arc, Mutex, Once},
    time::Duration,
};
static INIT: Once = Once::new();
struct Silent;
static SILENT: Silent = Silent;
impl log::Log for Silent {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }
    fn log(&self, _: &log::Record<'_>) {}
    fn flush(&self) {}
}
#[derive(Clone, Copy)]
enum WriteStep {
    Short(usize),
    Block,
    Zero,
    All,
}
struct State {
    steps: VecDeque<WriteStep>,
    output: Vec<u8>,
    input: Cursor<Vec<u8>>,
    shutdown: bool,
}
struct Stub(Arc<Mutex<State>>);
impl Read for Stub {
    fn read(&mut self, b: &mut [u8]) -> stdio::Result<usize> {
        let n = self.0.lock().expect("state").input.read(b)?;
        if n == 0 {
            Err(stdio::ErrorKind::WouldBlock.into())
        } else {
            Ok(n)
        }
    }
}
impl Write for Stub {
    fn write(&mut self, b: &[u8]) -> stdio::Result<usize> {
        let mut s = self.0.lock().expect("state");
        match s.steps.pop_front().unwrap_or(WriteStep::All) {
            WriteStep::Block => Err(stdio::ErrorKind::WouldBlock.into()),
            WriteStep::Zero => Ok(0),
            WriteStep::Short(n) => {
                let n = n.min(b.len());
                s.output.extend_from_slice(&b[..n]);
                Ok(n)
            }
            WriteStep::All => {
                s.output.extend_from_slice(b);
                Ok(b.len())
            }
        }
    }
    fn flush(&mut self) -> stdio::Result<()> {
        Ok(())
    }
}
impl super::io::SocketIo for Stub {
    fn timeouts(&self, _: Duration) -> stdio::Result<()> {
        Ok(())
    }
    fn shutdown(&self) {
        self.0.lock().expect("state").shutdown = true;
    }
}
fn budget() -> OperationLimits {
    OperationLimits {
        deadline: Instant::now() + Duration::from_millis(500),
        max_read_bytes: 4096,
        max_write_bytes: 4096,
        max_work: 128,
    }
}
fn stub(input: Vec<u8>, steps: Vec<WriteStep>) -> (Transport, Arc<Mutex<State>>) {
    INIT.call_once(|| install_log_boundary(&SILENT).expect("logger"));
    let limits = Limits {
        endpoint_bytes: 512,
        handshake_bytes: 1024,
        read_buffer_bytes: 64,
        write_buffer_bytes: 0,
        write_buffer_max: 1024,
        frame_bytes: 512,
        message_bytes: 512,
        outbound_bytes: 256,
    };
    let state = Arc::new(Mutex::new(State {
        steps: steps.into(),
        output: vec![],
        input: Cursor::new(input),
        shutdown: false,
    }));
    let cancel = Cancellation::new(None);
    let guard = Guarded::new(Box::new(Stub(state.clone())), cancel.clone(), budget());
    let ws = WebSocket::from_raw_socket(
        guard,
        tungstenite::protocol::Role::Client,
        Some(limits.config()),
    );
    (
        Transport {
            ws: Some(ws),
            limits,
            cancel,
        },
        state,
    )
}
#[test]
fn short_write_backpressure_flushes_original_command_once() {
    let (mut t, state) = stub(
        vec![],
        vec![WriteStep::Short(3), WriteStep::Block, WriteStep::All],
    );
    let mut op = t.operation(budget()).expect("op");
    assert_eq!(
        op.send_text("one command").expect("pending"),
        SendStatus::Pending
    );
    let refused = op.send_text("must not enqueue").unwrap_err();
    assert_eq!(refused.kind, ErrorKind::PendingWrite);
    assert_eq!(refused.send_progress, SendProgress::PossiblyWritten);
    assert_eq!(op.resume_send().expect("resume"), SendStatus::Flushed);
    drop(op);
    let bytes = state.lock().expect("state").output.clone();
    let mut peer = WebSocket::from_raw_socket(
        Cursor::new(bytes),
        tungstenite::protocol::Role::Server,
        None,
    );
    assert_eq!(
        peer.read().expect("frame").into_text().expect("text"),
        "one command"
    );
    assert!(peer.read().is_err());
    drop(t);
    assert!(state.lock().expect("state").shutdown);
}
#[test]
fn zero_write_refuses_without_codec_handshake_panic() {
    let (mut t, state) = stub(vec![], vec![WriteStep::Zero]);
    let e = t
        .operation(budget())
        .expect("op")
        .send_text("hello")
        .unwrap_err();
    assert_eq!(e.kind, ErrorKind::Io);
    assert_eq!(e.send_progress, SendProgress::Queued);
    assert!(state.lock().expect("state").output.is_empty());
    assert!(state.lock().expect("state").shutdown);
}
#[test]
fn pong_wouldblock_is_resumed_without_new_application_send() {
    let (mut t, state) = stub(
        vec![0x89, 0, 0x81, 2, b'o', b'k'],
        vec![WriteStep::Block, WriteStep::All],
    );
    let mut op = t.operation(budget()).expect("op");
    assert!(matches!(op.receive().expect("ping"), Event::Ping));
    assert_eq!(
        op.send_text("not yet").unwrap_err().kind,
        ErrorKind::PendingWrite
    );
    assert!(matches!(op.receive().expect("flush pong and read"),Event::Text(s)if s=="ok"));
    drop(op);
    let bytes = state.lock().expect("state").output.clone();
    let mut peer = WebSocket::from_raw_socket(
        Cursor::new(bytes),
        tungstenite::protocol::Role::Server,
        None,
    );
    assert!(peer.read().expect("pong").is_pong());
    assert!(peer.read().is_err());
}
#[test]
fn codec_read_ahead_does_not_bypass_next_operation_payload_limit() {
    let (mut t, _) = stub(vec![0x81, 1, b'a', 0x81, 4, b'b', b'b', b'b', b'b'], vec![]);
    assert!(
        matches!(t.operation(budget()).expect("op").receive().expect("first"),Event::Text(s)if s=="a")
    );
    let mut tiny = budget();
    tiny.max_read_bytes = 2;
    assert_eq!(
        t.operation(tiny)
            .expect("tiny op")
            .receive()
            .unwrap_err()
            .kind,
        ErrorKind::ByteLimit
    );
}
#[test]
fn pending_drop_and_deadline_do_not_send_more_bytes() {
    let (mut t, state) = stub(vec![], vec![WriteStep::Block]);
    {
        let mut op = t.operation(budget()).expect("op");
        assert_eq!(
            op.send_text("queued").expect("pending"),
            SendStatus::Pending
        );
    }
    assert!(state.lock().expect("state").output.is_empty());
    assert!(state.lock().expect("state").shutdown);
    let (mut t, state) = stub(vec![], vec![WriteStep::Block]);
    let mut short = budget();
    short.deadline = Instant::now() + Duration::from_millis(20);
    let mut op = t.operation(short).expect("short");
    assert_eq!(
        op.send_text("queued").expect("pending"),
        SendStatus::Pending
    );
    std::thread::sleep(Duration::from_millis(25));
    assert_eq!(op.resume_send().unwrap_err().kind, ErrorKind::Timeout);
    assert!(state.lock().expect("state").output.is_empty());
}

#[test]
fn zero_write_during_handshake_is_a_typed_failure() {
    let (mut transport, state) = stub(vec![], vec![WriteStep::Zero]);
    let guard = transport.ws.take().expect("test socket").into_inner();
    let error = tungstenite::client::client_with_config(
        "ws://127.0.0.1:1/stub-only",
        guard,
        Some(transport.limits.config()),
    )
    .err()
    .expect("zero write rejected");
    assert!(
        matches!(error, tungstenite::handshake::HandshakeError::Failure(tungstenite::Error::Io(e)) if e.kind() == stdio::ErrorKind::WriteZero)
    );
    assert!(state.lock().expect("state").shutdown);
}
