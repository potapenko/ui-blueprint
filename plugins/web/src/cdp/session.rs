use super::{
    Binding, ErrorKind, EventLoss, Failure, IDS, Limits, Reply, ReplyKind, Ticket, Usage,
    WireEvent, codec, copy_text, next_epoch, next_id, quota, transport_failure,
};
use crate::transport::{
    self, Cancellation, Event as TransportEvent, OperationLimits, SendProgress, SendStatus,
    Transport,
};
use serde::Serialize;
use std::{fmt, sync::Arc};

// A narrow private IO seam permits deterministic pending-write/failure proofs.
// Production always uses the one original, exclusively borrowed transport Operation.
trait ExchangeIo {
    fn send_text(&mut self, text: &str) -> Result<SendStatus, transport::Failure>;
    fn resume_send(&mut self) -> Result<SendStatus, transport::Failure>;
    fn receive(&mut self) -> Result<TransportEvent, transport::Failure>;
}
impl ExchangeIo for transport::Operation<'_> {
    fn send_text(&mut self, text: &str) -> Result<SendStatus, transport::Failure> {
        transport::Operation::send_text(self, text)
    }
    fn resume_send(&mut self) -> Result<SendStatus, transport::Failure> {
        transport::Operation::resume_send(self)
    }
    fn receive(&mut self) -> Result<TransportEvent, transport::Failure> {
        transport::Operation::receive(self)
    }
}
struct State {
    binding: Arc<Binding>,
    epoch: u64,
    active: Option<u32>,
    events: Vec<Option<WireEvent>>,
    head: usize,
    queued: usize,
    loss_generation: u64,
    unreported_loss: usize,
    event_pool: Arc<quota::Pool>,
}
impl State {
    fn lose_event(&mut self) -> Result<(), Failure> {
        self.loss_generation = self
            .loss_generation
            .checked_add(1)
            .ok_or(Failure::plain(ErrorKind::CounterExhausted))?;
        self.unreported_loss = self
            .unreported_loss
            .checked_add(1)
            .ok_or(Failure::plain(ErrorKind::CounterExhausted))?;
        Ok(())
    }
    fn event_permit(
        &mut self,
        wire_capacity: usize,
        metadata: usize,
    ) -> Result<Option<quota::Permit>, Failure> {
        let charge = wire_capacity
            .checked_add(metadata)
            .ok_or(Failure::plain(ErrorKind::Budget))?;
        match self.event_pool.reserve(charge) {
            Ok(permit) if self.queued < self.events.len() => Ok(Some(permit)),
            _ => {
                self.lose_event()?;
                Ok(None)
            }
        }
    }
    fn push_event(&mut self, wire: String, method: String, permit: quota::Permit) {
        // Admission reserved a slot, and this exclusive owner cannot interleave insertion.
        let index = (self.head + self.queued) % self.events.len();
        self.events[index] = Some(WireEvent {
            wire,
            method,
            binding: self.binding.clone(),
            _permit: permit,
        });
        self.queued += 1;
    }
}
/// One IO owner and one pending request; different Clients share no dispatch lock.
pub struct Client {
    transport: Option<Transport>,
    state: State,
    limits: Limits,
    result_pool: Arc<quota::Pool>,
}
impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Client {{ epoch: {}, active: {}, queued_events: {} }}",
            self.state.epoch,
            self.state.active.is_some(),
            self.state.queued
        )
    }
}
impl Client {
    /// Takes ownership of an authorized, connected transport. Invalid input drops it.
    pub fn new(transport: Transport, binding: Binding, limits: Limits) -> Result<Self, Failure> {
        limits.validate()?;
        let ids = [
            Some(&binding.target.id),
            Some(&binding.target.generation),
            binding.cdp_session_id.as_ref(),
        ];
        if ids.into_iter().flatten().any(|id| {
            id.0.is_empty()
                || id.0.chars().count() > 256
                || id.0.capacity() > limits.max_metadata_bytes
        }) {
            return Err(Failure::plain(ErrorKind::InvalidBinding));
        }
        let mut events = Vec::new();
        events
            .try_reserve_exact(limits.max_events)
            .map_err(|_| Failure::plain(ErrorKind::Budget))?;
        if events.capacity() > limits.max_events {
            return Err(Failure::plain(ErrorKind::Budget));
        }
        events.resize_with(limits.max_events, || None);
        let binding = Arc::new(binding);
        let epoch = next_epoch()?;
        Ok(Self {
            transport: Some(transport),
            state: State {
                binding,
                epoch,
                active: None,
                events,
                head: 0,
                queued: 0,
                loss_generation: 0,
                unreported_loss: 0,
                event_pool: quota::Pool::new(limits.max_events, limits.event_bytes),
            },
            limits,
            result_pool: quota::Pool::new(limits.max_results, limits.result_bytes),
        })
    }
    /// Actual codec configuration while an owner remains and is not cancelled.
    /// No default is returned for a detached or cancelled connection.
    pub fn transport_limits(&self) -> Option<transport::Limits> {
        self.transport
            .as_ref()
            .filter(|transport| !transport.cancellation().is_cancelled())
            .map(Transport::limits)
    }
    pub fn cancellation(&self) -> Option<Cancellation> {
        self.transport.as_ref().map(Transport::cancellation)
    }
    /// Checks origin only, even after detach; does not assert liveness or authority.
    pub fn owns_ticket(&self, ticket: &Ticket) -> bool {
        self.state.epoch == ticket.epoch && Arc::ptr_eq(&self.state.binding, &ticket.binding)
    }
    pub fn result_usage(&self) -> Usage {
        self.result_pool.usage()
    }
    pub fn event_usage(&self) -> Usage {
        self.state.event_pool.usage()
    }
    pub fn take_event_loss(&mut self) -> Option<EventLoss> {
        if self.state.unreported_loss == 0 {
            return None;
        }
        let dropped = std::mem::take(&mut self.state.unreported_loss);
        Some(EventLoss {
            generation: self.state.loss_generation,
            dropped,
        })
    }
    pub fn event_loss_generation(&self) -> u64 {
        self.state.loss_generation
    }
    pub fn pop_event(&mut self) -> Option<WireEvent> {
        if self.state.queued == 0 {
            return None;
        }
        let event = self.state.events[self.state.head].take();
        self.state.head = (self.state.head + 1) % self.state.events.len();
        self.state.queued -= 1;
        event
    }
    /// Stops dispatch and closes only this socket. Completed data remains usable.
    pub fn detach(&mut self) {
        self.state.active = None;
        self.transport.take();
        for slot in &mut self.state.events {
            slot.take();
        }
        self.state.head = 0;
        self.state.queued = 0;
    }
    /// Reserve a result slot and register a non-reusable ticket before dispatch.
    /// One outstanding request only; Rust borrowing prevents concurrent prepares.
    pub fn prepare<P: Serialize>(
        &mut self,
        method: &str,
        params: &P,
        budget: OperationLimits,
    ) -> Result<Pending<'_>, Failure> {
        if self.state.active.is_some() {
            return Err(Failure::plain(ErrorKind::Busy));
        }
        if method.is_empty() || method.len() > self.limits.max_metadata_bytes {
            return Err(Failure::plain(ErrorKind::Encoding));
        }
        drop(
            self.transport
                .as_mut()
                .ok_or(Failure::plain(ErrorKind::Detached))?
                .operation(budget)
                .map_err(transport_failure)?,
        );
        let permit = self.result_pool.reserve(self.limits.max_message_bytes)?;
        let id = next_id(&IDS)?;
        let encoded = codec::encode(
            id,
            method,
            params,
            self.state
                .binding
                .cdp_session_id
                .as_ref()
                .map(|id| id.0.as_str()),
            self.limits.max_request_bytes,
            self.limits.max_metadata_bytes,
        )?;
        let ticket = Ticket {
            epoch: self.state.epoch,
            id,
            binding: self.state.binding.clone(),
        };
        self.state.active = Some(id);
        Ok(Pending {
            client: self,
            ticket,
            encoded: Some(encoded),
            permit: Some(permit),
            budget,
            finished: false,
        })
    }
}
impl Drop for Client {
    fn drop(&mut self) {
        self.detach();
    }
}
/// Dropping/cancelling an uncompleted ticket invalidates the owned connection.
pub struct Pending<'a> {
    client: &'a mut Client,
    ticket: Ticket,
    encoded: Option<String>,
    permit: Option<quota::Permit>,
    budget: OperationLimits,
    finished: bool,
}
impl Pending<'_> {
    pub fn ticket(&self) -> &Ticket {
        &self.ticket
    }
    pub fn cancel(mut self) -> Failure {
        self.client.detach();
        self.finished = true;
        Failure {
            kind: ErrorKind::Cancelled,
            wire_id: Some(self.ticket.id),
            send_progress: SendProgress::NotQueued,
        }
    }
    /// Finite send/flush/read of this one command, no reconnect or application retry.
    pub fn run(mut self) -> Result<Reply, Failure> {
        let encoded = self
            .encoded
            .take()
            .ok_or(Failure::plain(ErrorKind::Detached))?;
        let outcome = (|| {
            let transport = self
                .client
                .transport
                .as_mut()
                .ok_or(Failure::plain(ErrorKind::Detached))?;
            let cancellation = transport.cancellation();
            let mut operation = transport
                .operation(self.budget)
                .map_err(transport_failure)?;
            let result = exchange(
                &mut operation,
                &mut self.client.state,
                self.client.limits,
                &self.ticket,
                &encoded,
            );
            // Parsing is within the original deadline too; cancellation cannot revive a ticket.
            if result.is_ok() && cancellation.is_cancelled() {
                Err(Failure {
                    kind: ErrorKind::Cancelled,
                    wire_id: Some(self.ticket.id),
                    send_progress: SendProgress::Flushed,
                })
            } else if result.is_ok() && std::time::Instant::now() >= self.budget.deadline {
                Err(Failure {
                    kind: ErrorKind::Transport(transport::ErrorKind::Timeout),
                    wire_id: Some(self.ticket.id),
                    send_progress: SendProgress::Flushed,
                })
            } else {
                result
            }
        })();
        drop(encoded);
        self.client.state.active = None;
        self.finished = true;
        match outcome {
            Ok((wire, kind)) => {
                let permit = self
                    .permit
                    .take()
                    .ok_or(Failure::plain(ErrorKind::Budget))?;
                Ok(Reply {
                    wire,
                    ticket: self.ticket.clone(),
                    kind,
                    loss_generation: self.client.state.loss_generation,
                    _permit: permit,
                })
            }
            Err(mut e) => {
                e.wire_id = Some(self.ticket.id);
                self.client.detach();
                Err(e)
            }
        }
    }
}
impl Drop for Pending<'_> {
    fn drop(&mut self) {
        if !self.finished {
            self.client.detach();
        }
    }
}
fn exchange(
    operation: &mut impl ExchangeIo,
    state: &mut State,
    limits: Limits,
    ticket: &Ticket,
    encoded: &str,
) -> Result<(String, ReplyKind), Failure> {
    let mut progress = operation.send_text(encoded).map_err(transport_failure)?;
    while progress == SendStatus::Pending {
        progress = operation.resume_send().map_err(transport_failure)?;
    }
    loop {
        let event = operation.receive().map_err(transport_failure)?;
        let wire = match event {
            TransportEvent::Text(wire) => wire,
            TransportEvent::Ping | TransportEvent::Pong | TransportEvent::WouldBlock => continue,
            TransportEvent::PeerClose => {
                return Err(Failure {
                    kind: ErrorKind::Detached,
                    wire_id: Some(ticket.id),
                    send_progress: SendProgress::Flushed,
                });
            }
        };
        let decoded = handle_wire(state, limits, ticket, wire);
        match decoded {
            Ok(Some(reply)) => return Ok(reply),
            Ok(None) => (),
            Err(mut e) => {
                e.send_progress = SendProgress::Flushed;
                return Err(e);
            }
        }
    }
}
fn handle_wire(
    state: &mut State,
    limits: Limits,
    ticket: &Ticket,
    wire: String,
) -> Result<Option<(String, ReplyKind)>, Failure> {
    if wire.capacity() > limits.max_message_bytes {
        return Err(Failure::plain(ErrorKind::Budget));
    }
    let envelope = codec::inspect(&wire, limits.max_metadata_bytes)?;
    if envelope.session.as_deref() != state.binding.cdp_session_id.as_ref().map(|s| s.0.as_str()) {
        return Err(Failure::plain(ErrorKind::WrongSession));
    }
    match envelope.kind {
        codec::Kind::Reply { id, outcome } => {
            if state.active != Some(id) || id != ticket.id || state.epoch != ticket.epoch {
                return Err(Failure::plain(ErrorKind::UnexpectedReply));
            }
            Ok(Some((wire, outcome)))
        }
        codec::Kind::UncorrelatedError(code) => {
            Err(Failure::plain(ErrorKind::UncorrelatedError(code)))
        }
        codec::Kind::Event { method } => {
            let Some(permit) = state.event_permit(wire.capacity(), limits.max_metadata_bytes)?
            else {
                return Ok(None);
            };
            let method = match method {
                std::borrow::Cow::Owned(text) => text,
                std::borrow::Cow::Borrowed(text) => copy_text(text, limits.max_metadata_bytes)?,
            };
            state.push_event(wire, method, permit);
            Ok(None)
        }
    }
}

#[cfg(test)]
#[path = "session_tests.rs"]
mod tests;
