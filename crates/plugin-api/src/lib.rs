//! Pure observation-session lifecycle. No SDKs, handles, clocks or background work.
//! Callers own transport, unique attachment IDs and actual collection deadlines.
#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use uiblueprint_schema::{SchemaVersion, model::*, validation::contexts_compatible};

/// Explicit caller limits, not frozen production memory defaults (D05-RES).
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_frame_bytes: usize,
    pub max_in_flight: usize,
    pub max_pending_encoded_bytes: usize,
}

/// A reading of the parent session's monotonic clock, in milliseconds.
#[derive(Clone, Debug)]
pub struct ClockReading {
    pub domain: Id,
    pub milliseconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ticket {
    pub request_id: Id,
    pub session_id: Id,
    pub sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidFrame,
    UnsupportedOperation,
    IncompatibleVersion,
    OutsideSession,
    Detached,
    Busy,
    ResourceLimit,
    StaleTicket,
    DeadlineExpired,
    InvalidClock,
    UnexpectedChannel,
    DuplicateChannel,
    InvalidChannel,
    Incomplete,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Terminal {
    Completed,
    Cancelled,
    TimedOut,
    Detached,
}

/// Completed means all requested channels replied, not that every channel succeeded.
pub struct Completion {
    pub ticket: Ticket,
    pub terminal: Terminal,
    pub channels: BTreeMap<Channel, ChannelResult>,
    pub missing_channels: Vec<Channel>,
}

struct Pending {
    ticket: Ticket,
    context: Context,
    requested: Vec<Channel>,
    max_elements: usize,
    max_depth: u32,
    max_output_bytes: usize,
    request_encoded_bytes: usize,
    freshness_policy: FreshnessPolicy,
    deadline: u64,
    encoded_bytes: usize,
    channels: BTreeMap<Channel, ChannelResult>,
}

/// One attachment; give every new attachment a new session_id. Reusing IDs across
/// attachments is a caller contract violation. detach is terminal for this object.
pub struct ObservationSession {
    descriptor: SessionDescriptor,
    clock_domain: Id,
    last_clock: u64,
    limits: Limits,
    detached: bool,
    sequence: u64,
    pending_encoded_bytes: usize,
    pending: BTreeMap<u64, Pending>,
}

impl ObservationSession {
    pub fn attach(
        descriptor: SessionDescriptor,
        clock_domain: Id,
        limits: Limits,
    ) -> Result<Self, Error> {
        if !descriptor
            .supported_versions
            .contains(&SchemaVersion::CURRENT)
        {
            return Err(Error::IncompatibleVersion);
        }
        if limits.max_frame_bytes == 0
            || limits.max_in_flight == 0
            || limits.max_pending_encoded_bytes < limits.max_frame_bytes
        {
            return Err(Error::ResourceLimit);
        }
        Document {
            schema_version: SchemaVersion::CURRENT,
            artifact: Artifact::Session(Box::new(descriptor.clone())),
        }
        .validate()
        .map_err(|_| Error::InvalidFrame)?;
        if clock_domain.0.is_empty() {
            return Err(Error::InvalidClock);
        }
        Ok(Self {
            descriptor,
            clock_domain,
            last_clock: 0,
            limits,
            detached: false,
            sequence: 0,
            pending_encoded_bytes: 0,
            pending: BTreeMap::new(),
        })
    }

    fn clock(&mut self, now: &ClockReading) -> Result<u64, Error> {
        if now.domain != self.clock_domain || now.milliseconds < self.last_clock {
            return Err(Error::InvalidClock);
        }
        self.last_clock = now.milliseconds;
        Ok(now.milliseconds)
    }

    /// Admission does not execute a collector. Only Observe belongs here; Rust
    /// analysis and the future action executor do not acquire an implicit backend.
    pub fn begin(&mut self, frame: &[u8], now: &ClockReading) -> Result<Ticket, Error> {
        if self.detached {
            return Err(Error::Detached);
        }
        let now = self.clock(now)?;
        if self.pending.len() >= self.limits.max_in_flight {
            return Err(Error::Busy);
        }
        if frame.len() > self.limits.max_frame_bytes {
            return Err(Error::ResourceLimit);
        }
        let doc = Document::from_json(frame, self.limits.max_frame_bytes)
            .map_err(|_| Error::InvalidFrame)?;
        let Artifact::Request(request) = doc.artifact else {
            return Err(Error::InvalidFrame);
        };
        self.check_request(&request)?;
        let Operation::Observe { channels } = request.operation else {
            return Err(Error::UnsupportedOperation);
        };
        let deadline = now
            .checked_add(request.limits.deadline_ms)
            .ok_or(Error::ResourceLimit)?;
        let bytes = self
            .pending_encoded_bytes
            .checked_add(frame.len())
            .ok_or(Error::ResourceLimit)?;
        if bytes > self.limits.max_pending_encoded_bytes {
            return Err(Error::ResourceLimit);
        }
        let sequence = self.sequence.checked_add(1).ok_or(Error::ResourceLimit)?;
        let ticket = Ticket {
            request_id: request.request_id,
            session_id: self.descriptor.session_id.clone(),
            sequence,
        };
        self.pending.insert(
            sequence,
            Pending {
                ticket: ticket.clone(),
                context: request.context,
                requested: channels,
                max_elements: request.limits.max_elements as usize,
                max_depth: request.limits.max_depth,
                max_output_bytes: usize::try_from(request.limits.max_output_bytes)
                    .map_err(|_| Error::ResourceLimit)?,
                request_encoded_bytes: frame.len(),
                freshness_policy: request.freshness_policy,
                deadline,
                encoded_bytes: frame.len(),
                channels: BTreeMap::new(),
            },
        );
        self.sequence = sequence;
        self.pending_encoded_bytes = bytes;
        Ok(ticket)
    }

    fn check_request(&self, r: &Request) -> Result<(), Error> {
        if r.clock_domain != self.clock_domain {
            return Err(Error::InvalidClock);
        }
        if r.context.session_id != self.descriptor.session_id
            || r.context.target != self.descriptor.target
            || r.context.plugin != self.descriptor.plugin
            || !self.descriptor.allowed_scopes.contains(&r.context.scope_id)
            || !r
                .context
                .surfaces
                .iter()
                .all(|s| self.descriptor.surfaces.contains(s))
        {
            return Err(Error::OutsideSession);
        }
        Ok(())
    }

    /// A completed channel is retained even if another channel later fails or
    /// times out. Frames must already be redacted at the adapter boundary.
    pub fn receive(
        &mut self,
        ticket: &Ticket,
        frame: &[u8],
        now: &ClockReading,
    ) -> Result<(), Error> {
        if self.detached {
            return Err(Error::Detached);
        }
        let now = self.clock(now)?;
        let pending = self
            .pending
            .get(&ticket.sequence)
            .filter(|p| &p.ticket == ticket)
            .ok_or(Error::StaleTicket)?;
        if now >= pending.deadline {
            return Err(Error::DeadlineExpired);
        }
        if frame.len() > self.limits.max_frame_bytes {
            return Err(Error::ResourceLimit);
        }
        let doc = Document::from_json(frame, self.limits.max_frame_bytes)
            .map_err(|_| Error::InvalidFrame)?;
        let Artifact::ChannelResponse(response) = doc.artifact else {
            return Err(Error::InvalidFrame);
        };
        self.check_response(pending, &response)?;
        let global = self
            .pending_encoded_bytes
            .checked_add(frame.len())
            .ok_or(Error::ResourceLimit)?;
        let local = pending
            .encoded_bytes
            .checked_add(frame.len())
            .ok_or(Error::ResourceLimit)?;
        if global > self.limits.max_pending_encoded_bytes
            || local - pending.request_encoded_bytes > pending.max_output_bytes
        {
            return Err(Error::ResourceLimit);
        }
        let pending = self
            .pending
            .get_mut(&ticket.sequence)
            .ok_or(Error::StaleTicket)?;
        pending.channels.insert(response.channel, response.result);
        pending.encoded_bytes = local;
        self.pending_encoded_bytes = global;
        Ok(())
    }

    fn check_response(&self, p: &Pending, r: &ChannelResponse) -> Result<(), Error> {
        if r.request_id != p.ticket.request_id
            || r.session_id != p.ticket.session_id
            || r.dispatch_sequence != p.ticket.sequence
            || r.target != p.context.target
        {
            return Err(Error::StaleTicket);
        }
        if !p.requested.contains(&r.channel) {
            return Err(Error::UnexpectedChannel);
        }
        if p.channels.contains_key(&r.channel) {
            return Err(Error::DuplicateChannel);
        }
        match &r.result {
            ChannelResult::Observed(s) => {
                if !contexts_compatible(&p.context, &s.context) || s.nodes.len() > p.max_elements {
                    return Err(Error::InvalidChannel);
                }
                if !depth_within(s, p.max_depth)
                    || (p.freshness_policy == FreshnessPolicy::CurrentRequired
                        && s.observations
                            .iter()
                            .any(|o| o.freshness != Freshness::Current))
                {
                    return Err(Error::InvalidChannel);
                }
                if !self.descriptor.capabilities.iter().any(|c| {
                    c.channel == r.channel
                        && c.operation.0 == "observe"
                        && matches!(
                            c.status,
                            CapabilityStatus::Supported | CapabilityStatus::Partial
                        )
                }) {
                    return Err(Error::InvalidChannel);
                }
            }
            ChannelResult::Failed(issue) => {
                if issue.scope_id != p.context.scope_id {
                    return Err(Error::OutsideSession);
                }
            }
        }
        Ok(())
    }

    pub fn complete(&mut self, ticket: &Ticket, now: &ClockReading) -> Result<Completion, Error> {
        let now = self.clock(now)?;
        let p = self
            .pending
            .get(&ticket.sequence)
            .filter(|p| &p.ticket == ticket)
            .ok_or(Error::StaleTicket)?;
        if now >= p.deadline {
            return Err(Error::DeadlineExpired);
        }
        if p.channels.len() != p.requested.len() {
            return Err(Error::Incomplete);
        }
        self.take(ticket, Terminal::Completed)
    }
    pub fn cancel(&mut self, ticket: &Ticket) -> Result<Completion, Error> {
        self.take(ticket, Terminal::Cancelled)
    }
    pub fn expire(&mut self, ticket: &Ticket, now: &ClockReading) -> Result<Completion, Error> {
        let now = self.clock(now)?;
        let p = self
            .pending
            .get(&ticket.sequence)
            .filter(|p| &p.ticket == ticket)
            .ok_or(Error::StaleTicket)?;
        if now < p.deadline {
            return Err(Error::Incomplete);
        }
        self.take(ticket, Terminal::TimedOut)
    }
    pub fn detach(&mut self) -> Vec<Completion> {
        self.detached = true;
        let pending = std::mem::take(&mut self.pending);
        self.pending_encoded_bytes = 0;
        pending
            .into_values()
            .map(|p| completion(p, Terminal::Detached))
            .collect()
    }
    fn take(&mut self, ticket: &Ticket, terminal: Terminal) -> Result<Completion, Error> {
        if !self
            .pending
            .get(&ticket.sequence)
            .is_some_and(|p| &p.ticket == ticket)
        {
            return Err(Error::StaleTicket);
        }
        let pending = self
            .pending
            .remove(&ticket.sequence)
            .ok_or(Error::StaleTicket)?;
        self.pending_encoded_bytes -= pending.encoded_bytes;
        Ok(completion(pending, terminal))
    }
    pub fn pending_encoded_bytes(&self) -> usize {
        self.pending_encoded_bytes
    }
}

fn completion(p: Pending, terminal: Terminal) -> Completion {
    let missing_channels = p
        .requested
        .into_iter()
        .filter(|c| !p.channels.contains_key(c))
        .collect();
    Completion {
        ticket: p.ticket,
        terminal,
        channels: p.channels,
        missing_channels,
    }
}

fn depth_within(snapshot: &Snapshot, limit: u32) -> bool {
    let nodes: BTreeMap<_, _> = snapshot.nodes.iter().map(|n| (&n.key, n)).collect();
    // Semantic validation has already rejected cycles/dangling children. Stop
    // at the request limit instead of calculating an unbounded graph traversal.
    for node in &snapshot.nodes {
        let mut pending = vec![(node, 1)];
        while let Some((node, depth)) = pending.pop() {
            if depth > limit {
                return false;
            }
            for child in &node.children {
                if let Some(child) = nodes.get(child) {
                    pending.push((child, depth + 1));
                }
            }
        }
    }
    true
}
