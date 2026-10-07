//! Explicit-operation WebSocket transport. No CDP parser, browser launch or background reads.
mod io;
mod logging;
mod operation;
pub use io::Cancellation;
use io::Guarded;
pub use logging::install_log_boundary;
pub use operation::Operation;
use std::{
    fmt,
    net::{SocketAddr, TcpStream},
    time::Instant,
};
use tungstenite::{
    WebSocket, client::IntoClientRequest, handshake::HandshakeError, protocol::WebSocketConfig,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidLimits,
    EndpointRejected,
    LoggingBoundary,
    Connect,
    Handshake,
    Extensions,
    Timeout,
    Cancelled,
    ByteLimit,
    WorkLimit,
    Io,
    Protocol,
    Capacity,
    Binary,
    Closed,
    PendingWrite,
    NoPendingWrite,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendProgress {
    NotQueued,
    Queued,
    PossiblyWritten,
    Flushed,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Failure {
    pub kind: ErrorKind,
    pub send_progress: SendProgress,
}
impl Failure {
    fn plain(kind: ErrorKind) -> Self {
        Self {
            kind,
            send_progress: SendProgress::NotQueued,
        }
    }
}
impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({:?})", self.kind, self.send_progress)
    }
}
impl std::error::Error for Failure {}

/// Explicit caller caps, not allocator/RSS limits or production defaults.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub endpoint_bytes: usize,
    pub handshake_bytes: usize,
    pub read_buffer_bytes: usize,
    pub write_buffer_bytes: usize,
    pub write_buffer_max: usize,
    pub frame_bytes: usize,
    pub message_bytes: usize,
    pub outbound_bytes: usize,
}
impl Limits {
    fn validate(self) -> Result<(), Failure> {
        let positive = [
            self.endpoint_bytes,
            self.handshake_bytes,
            self.read_buffer_bytes,
            self.write_buffer_max,
            self.frame_bytes,
            self.message_bytes,
            self.outbound_bytes,
        ];
        if positive.iter().any(|&n| n == 0 || n > isize::MAX as usize)
            || self.write_buffer_bytes > isize::MAX as usize
        {
            return Err(Failure::plain(ErrorKind::InvalidLimits));
        }
        let required = self
            .outbound_bytes
            .checked_add(14)
            .and_then(|n| n.checked_add(139));
        if self.write_buffer_max <= self.write_buffer_bytes
            || required.is_none_or(|n| n > self.write_buffer_max)
        {
            return Err(Failure::plain(ErrorKind::InvalidLimits));
        }
        positive
            .iter()
            .try_fold(0usize, |total, n| total.checked_add(*n))
            .ok_or(Failure::plain(ErrorKind::InvalidLimits))?;
        Ok(())
    }
    fn config(self) -> WebSocketConfig {
        WebSocketConfig::default()
            .read_buffer_size(self.read_buffer_bytes)
            .write_buffer_size(self.write_buffer_bytes)
            .max_write_buffer_size(self.write_buffer_max)
            .max_frame_size(Some(self.frame_bytes))
            .max_message_size(Some(self.message_bytes))
            .accept_unmasked_frames(false)
    }
}
/// One unchanged absolute deadline/budget across sends, pending flushes and control frames.
#[derive(Clone, Copy, Debug)]
pub struct OperationLimits {
    pub deadline: Instant,
    pub max_read_bytes: usize,
    pub max_write_bytes: usize,
    pub max_work: usize,
}
impl OperationLimits {
    fn validate(self) -> Result<(), Failure> {
        if [self.max_read_bytes, self.max_write_bytes, self.max_work]
            .iter()
            .any(|&n| n == 0 || n > isize::MAX as usize)
            || self
                .max_read_bytes
                .checked_add(self.max_write_bytes)
                .is_none()
        {
            return Err(Failure::plain(ErrorKind::InvalidLimits));
        }
        if Instant::now() >= self.deadline {
            return Err(Failure::plain(ErrorKind::Timeout));
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendStatus {
    Flushed,
    Pending,
}
pub enum Event {
    Text(String),
    Ping,
    Pong,
    PeerClose,
    WouldBlock,
}
impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => write!(f, "Text {{ bytes: {} }}", text.len()),
            Self::Ping => f.write_str("Ping"),
            Self::Pong => f.write_str("Pong"),
            Self::PeerClose => f.write_str("PeerClose"),
            Self::WouldBlock => f.write_str("WouldBlock"),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloseStatus {
    HandshakeComplete,
    ForcedShutdown,
}

/// A single-use connection attempt. Obtain cancellation before the handshake.
/// TCP connect itself is bounded by connect_timeout; cancellation is observed on
/// return because std does not expose its in-progress descriptor.
pub struct Connecting {
    limits: Limits,
    cancel: Cancellation,
    transferred: bool,
}
impl Connecting {
    pub fn new(limits: Limits) -> Result<Self, Failure> {
        limits.validate()?;
        if !logging::installed() {
            return Err(Failure::plain(ErrorKind::LoggingBoundary));
        }
        Ok(Self {
            limits,
            cancel: Cancellation::new(None),
            transferred: false,
        })
    }
    pub fn cancellation(&self) -> Cancellation {
        self.cancel.clone()
    }
    pub fn connect(
        mut self,
        endpoint: &str,
        operation: OperationLimits,
    ) -> Result<Transport, Failure> {
        let result =
            Transport::connect_owned(endpoint, self.limits, operation, self.cancel.clone());
        self.transferred = result.is_ok();
        result
    }
}
impl Drop for Connecting {
    fn drop(&mut self) {
        if !self.transferred {
            self.cancel.cancel();
        }
    }
}

pub struct Transport {
    ws: Option<WebSocket<Guarded>>,
    limits: Limits,
    cancel: Cancellation,
}
impl fmt::Debug for Transport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Transport { endpoint: redacted }")
    }
}
impl Transport {
    /// Caller must authorize the exact endpoint. This validates literal loopback ws,
    /// not application permissions. No DNS, redirects, TLS or custom headers.
    pub fn connect(
        endpoint: &str,
        limits: Limits,
        operation: OperationLimits,
    ) -> Result<Self, Failure> {
        limits.validate()?;
        operation.validate()?;
        Connecting::new(limits)?.connect(endpoint, operation)
    }
    fn connect_owned(
        endpoint: &str,
        limits: Limits,
        mut operation: OperationLimits,
        cancel: Cancellation,
    ) -> Result<Self, Failure> {
        operation.validate()?;
        if cancel.is_cancelled() {
            return Err(Failure::plain(ErrorKind::Cancelled));
        }
        let address = endpoint_address(endpoint, limits.endpoint_bytes)?;
        let request = endpoint
            .into_client_request()
            .map_err(|_| Failure::plain(ErrorKind::EndpointRejected))?;
        let mut request_bytes = request
            .uri()
            .path_and_query()
            .map_or(1, |p| p.as_str().len())
            + "GET  HTTP/1.1\r\n\r\n".len();
        for (name, value) in request.headers() {
            request_bytes = request_bytes
                .checked_add(name.as_str().len())
                .and_then(|n| n.checked_add(value.len()))
                .and_then(|n| n.checked_add(4))
                .ok_or(Failure::plain(ErrorKind::InvalidLimits))?;
        }
        if request_bytes > limits.handshake_bytes || request_bytes > operation.max_write_bytes {
            return Err(Failure::plain(ErrorKind::Capacity));
        }
        let remaining = operation.deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(Failure::plain(ErrorKind::Timeout));
        }
        let socket = TcpStream::connect_timeout(&address, remaining).map_err(|_| {
            Failure::plain(if cancel.is_cancelled() {
                ErrorKind::Cancelled
            } else if Instant::now() >= operation.deadline {
                ErrorKind::Timeout
            } else {
                ErrorKind::Connect
            })
        })?;
        let duplicate = socket
            .try_clone()
            .map_err(|_| Failure::plain(ErrorKind::Io))?;
        cancel.attach(duplicate).map_err(Failure::plain)?;
        operation.max_read_bytes = operation.max_read_bytes.min(limits.handshake_bytes);
        operation.max_write_bytes = operation.max_write_bytes.min(limits.handshake_bytes);
        let guarded = Guarded::new(Box::new(socket), cancel.clone(), operation);
        match tungstenite::client::client_with_config(request, guarded, Some(limits.config())) {
            Ok((mut ws, response)) => {
                if response.headers().contains_key("sec-websocket-extensions") {
                    ws.get_ref().shutdown();
                    return Err(Failure::plain(ErrorKind::Extensions));
                }
                ws.get_mut().check().map_err(Failure::plain)?;
                Ok(Self {
                    ws: Some(ws),
                    limits,
                    cancel,
                })
            }
            Err(HandshakeError::Interrupted(mid)) => {
                let kind = mid.get_ref().get_ref().failure.unwrap_or(
                    if Instant::now() >= operation.deadline {
                        ErrorKind::Timeout
                    } else {
                        ErrorKind::Handshake
                    },
                );
                drop(mid);
                Err(Failure::plain(kind))
            }
            Err(HandshakeError::Failure(error)) => Err(Failure::plain(match error {
                tungstenite::Error::Io(e) => io::failure_kind(&e).unwrap_or(ErrorKind::Handshake),
                _ => ErrorKind::Handshake,
            })),
        }
    }
    /// The immutable configuration used to construct this transport's codec.
    /// This copy does not establish connection liveness or change any limit.
    pub fn limits(&self) -> Limits {
        self.limits
    }
    pub fn cancellation(&self) -> Cancellation {
        self.cancel.clone()
    }
    pub fn operation(&mut self, limits: OperationLimits) -> Result<Operation<'_>, Failure> {
        limits.validate()?;
        if !logging::installed() {
            return Err(Failure::plain(ErrorKind::LoggingBoundary));
        }
        let ws = self.ws.as_mut().ok_or(Failure::plain(ErrorKind::Closed))?;
        ws.get_mut().reset(limits);
        if let Err(kind) = ws.get_mut().check() {
            self.abort();
            return Err(Failure::plain(kind));
        }
        Ok(Operation::new(self))
    }
    fn abort(&mut self) {
        self.cancel.cancel();
        self.ws.take();
    }
}
impl Drop for Transport {
    fn drop(&mut self) {
        self.abort();
    }
}
fn endpoint_address(endpoint: &str, cap: usize) -> Result<SocketAddr, Failure> {
    let refused = || Failure::plain(ErrorKind::EndpointRejected);
    if endpoint.len() > cap || endpoint.contains('#') || endpoint.chars().any(char::is_whitespace) {
        return Err(refused());
    }
    let rest = endpoint.strip_prefix("ws://").ok_or_else(refused)?;
    let (authority, _) = rest.split_once('/').ok_or_else(refused)?;
    let address: SocketAddr = authority.parse().map_err(|_| refused())?;
    if !address.ip().is_loopback() || address.port() == 0 {
        return Err(refused());
    }
    Ok(address)
}

#[cfg(test)]
mod tests;
