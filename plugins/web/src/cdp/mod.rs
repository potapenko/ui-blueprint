//! Sequential, bounded CDP correlation. Raw wire is data, never diagnostic text.
mod codec;
mod quota;
mod session;
use crate::transport::{self, SendProgress};
pub use quota::Usage;
pub use session::{Client, Pending};
use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicU32, AtomicU64, Ordering},
    },
};
use uiblueprint_schema::model::{Id, Identity};

pub const MAX_WIRE_ID: u32 = i32::MAX as u32;
// Process-wide scalars prevent ID reuse across independent connections; no IO or locks.
static IDS: AtomicU32 = AtomicU32::new(0);
static EPOCHS: AtomicU64 = AtomicU64::new(0);
fn next_id(counter: &AtomicU32) -> Result<u32, Failure> {
    counter
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
            (n < MAX_WIRE_ID).then(|| n + 1)
        })
        .map(|n| n + 1)
        .map_err(|_| Failure::plain(ErrorKind::CounterExhausted))
}
fn next_epoch() -> Result<u64, Failure> {
    increment_epoch(&EPOCHS)
}
fn increment_epoch(counter: &AtomicU64) -> Result<u64, Failure> {
    counter
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
        .map(|n| n + 1)
        .map_err(|_| Failure::plain(ErrorKind::CounterExhausted))
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidLimits,
    InvalidBinding,
    Encoding,
    Envelope,
    Budget,
    Detached,
    Busy,
    CounterExhausted,
    WrongSession,
    UnexpectedReply,
    UncorrelatedError(i32),
    Cancelled,
    Transport(transport::ErrorKind),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Failure {
    pub kind: ErrorKind,
    pub wire_id: Option<u32>,
    pub send_progress: SendProgress,
}
impl Failure {
    fn plain(kind: ErrorKind) -> Self {
        Self {
            kind,
            wire_id: None,
            send_progress: SendProgress::NotQueued,
        }
    }
}
impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} id={:?} progress={:?}",
            self.kind, self.wire_id, self.send_progress
        )
    }
}
impl std::error::Error for Failure {}
fn transport_failure(f: transport::Failure) -> Failure {
    Failure {
        kind: if f.kind == transport::ErrorKind::Cancelled {
            ErrorKind::Cancelled
        } else {
            ErrorKind::Transport(f.kind)
        },
        wire_id: None,
        send_progress: f.send_progress,
    }
}

/// Caller must establish target/connection identity and authority before construction.
/// Rebinding requires a new Client/owned connection; replies cannot update generations.
pub struct Binding {
    pub target: Identity,
    pub cdp_session_id: Option<Id>,
}
impl fmt::Debug for Binding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Binding {{ target: redacted, session: {} }}",
            self.cdp_session_id.is_some()
        )
    }
}
#[derive(Clone)]
/// Correlation provenance only. Holding or cloning this is never dispatch authority.
pub struct Ticket {
    epoch: u64,
    id: u32,
    binding: Arc<Binding>,
}
impl Ticket {
    pub fn wire_id(&self) -> u32 {
        self.id
    }
    pub fn binding(&self) -> &Binding {
        &self.binding
    }
}
impl fmt::Debug for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ticket {{ epoch: {}, id: {} }}", self.epoch, self.id)
    }
}
#[derive(Clone, Copy, Debug)]
/// Finite retained ownership caps; parser/transport scratch and total RSS are separate.
pub struct Limits {
    pub max_request_bytes: usize,
    pub max_message_bytes: usize,
    pub max_metadata_bytes: usize,
    pub max_results: usize,
    pub result_bytes: usize,
    pub max_events: usize,
    pub event_bytes: usize,
}
impl Limits {
    fn validate(self) -> Result<(), Failure> {
        let all = [
            self.max_request_bytes,
            self.max_message_bytes,
            self.max_metadata_bytes,
            self.max_results,
            self.result_bytes,
            self.max_events,
            self.event_bytes,
        ];
        if all.iter().any(|&n| n == 0 || n > isize::MAX as usize)
            || self.result_bytes < self.max_message_bytes
            || self.max_metadata_bytes > self.max_message_bytes
        {
            return Err(Failure::plain(ErrorKind::InvalidLimits));
        }
        self.max_events
            .checked_mul(std::mem::size_of::<Option<WireEvent>>())
            .filter(|&n| n <= isize::MAX as usize)
            .ok_or(Failure::plain(ErrorKind::InvalidLimits))?;
        self.max_message_bytes
            .checked_add(self.max_metadata_bytes)
            .ok_or(Failure::plain(ErrorKind::InvalidLimits))?;
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplyKind {
    Result,
    Error { code: i32 },
}
/// The original complete wire frame is moved, never converted into a payload graph.
/// Borrow only; caller-created copies require their own allocation/ownership budget.
pub struct Reply {
    wire: String,
    ticket: Ticket,
    kind: ReplyKind,
    loss_generation: u64,
    _permit: quota::Permit,
}
impl Reply {
    pub fn wire(&self) -> &str {
        &self.wire
    }
    pub fn ticket(&self) -> &Ticket {
        &self.ticket
    }
    pub fn kind(&self) -> ReplyKind {
        self.kind
    }
    pub fn event_loss_generation(&self) -> u64 {
        self.loss_generation
    }
}
impl fmt::Debug for Reply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Reply {{ ticket: {:?}, kind: {:?}, bytes: {} }}",
            self.ticket,
            self.kind,
            self.wire.len()
        )
    }
}
/// Raw event data. Its reservation includes caller-held lifetime after pop_event.
pub struct WireEvent {
    wire: String,
    method: String,
    binding: Arc<Binding>,
    _permit: quota::Permit,
}
impl WireEvent {
    pub fn wire(&self) -> &str {
        &self.wire
    }
    pub fn method(&self) -> &str {
        &self.method
    }
    pub fn binding(&self) -> &Binding {
        &self.binding
    }
}
impl fmt::Debug for WireEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WireEvent {{ bytes: {}, method_bytes: {} }}",
            self.wire.len(),
            self.method.len()
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventLoss {
    pub generation: u64,
    pub dropped: usize,
}
fn copy_text(text: &str, limit: usize) -> Result<String, Failure> {
    if text.len() > limit {
        return Err(Failure::plain(ErrorKind::Budget));
    }
    let mut result = String::new();
    result
        .try_reserve_exact(text.len())
        .map_err(|_| Failure::plain(ErrorKind::Budget))?;
    if result.capacity() > limit {
        return Err(Failure::plain(ErrorKind::Budget));
    }
    result.push_str(text);
    Ok(result)
}

#[cfg(test)]
mod tests;
