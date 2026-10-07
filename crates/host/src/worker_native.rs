//! Worker-side Native exchange. Parsing/validation stays in worker_observe;
//! this owner reuses the already charged publication buffer and fixed controls.
use crate::{worker_io::WorkerIo, worker_main};
use uiblueprint_host::*;
pub struct NativeExchange<'a> {
    io: &'a mut WorkerIo,
    buffer: &'a mut [u8],
    operation: Control,
    ticket: u64,
    total: usize,
    pending: Option<u8>,
    seen: [u64; 3],
}
impl<'a> NativeExchange<'a> {
    pub fn new(io: &'a mut WorkerIo, buffer: &'a mut [u8], operation: Control) -> Self {
        Self {
            io,
            buffer,
            operation,
            ticket: 0,
            total: 0,
            pending: None,
            seen: [0; 3],
        }
    }
    pub fn begin(
        &mut self,
        ticket: u64,
        request_deadline_ms: u64,
        channels: u8,
    ) -> Result<(), HostError> {
        if self.ticket != 0
            || ticket == 0
            || request_deadline_ms == 0
            || channels != self.operation.flags & 7
        {
            return Err(HostError::InvalidState);
        }
        worker_main::admit_observation(
            self.io,
            self.operation,
            ticket,
            request_deadline_ms,
            channels,
        )?;
        self.ticket = ticket;
        Ok(())
    }
    pub fn collect(&mut self, channel: u8) -> Result<usize, HostError> {
        if self.ticket == 0
            || channel >= 3
            || self.pending.is_some()
            || self.operation.flags & (1 << channel) == 0
            || self.seen[channel as usize] != 0
        {
            return Err(HostError::InvalidState);
        }
        self.io.write_control(Control {
            kind: ControlKind::HelperRequest,
            class: OperationClass::Observe,
            slot: channel,
            flags: 0,
            correlation: self.operation.correlation,
            length: 0,
            value: self.ticket,
            auxiliary: 0,
        })?;
        let reply = self.io.control()?;
        if reply.kind != ControlKind::HelperReply
            || reply.class != OperationClass::Observe
            || reply.correlation != self.operation.correlation
            || reply.slot != channel
            || reply.auxiliary != self.ticket
        {
            return Err(HostError::InvalidControl);
        }
        if reply.flags != 0 {
            if reply.length != 0 {
                return Err(HostError::InvalidControl);
            }
            return Err(match reply.flags {
                1 => HostError::ResourceLimit,
                2 => HostError::DeadlineExpired,
                3 => HostError::WorkerFailed,
                4 => HostError::PermissionDenied,
                5 => HostError::CleanupPending,
                _ => HostError::InvalidControl,
            });
        }
        let size = usize::try_from(reply.length).map_err(|_| HostError::Overflow)?;
        if reply.value == 0
            || self.seen.contains(&reply.value)
            || size == 0
            || size > self.buffer.len()
            || size > (self.operation.auxiliary as u32) as usize
        {
            return Err(HostError::InvalidControl);
        }
        self.io.read(&mut self.buffer[..size])?;
        self.seen[channel as usize] = reply.value;
        self.pending = Some(channel);
        Ok(size)
    }
    pub fn bytes(&self, size: usize) -> &[u8] {
        &self.buffer[..size]
    }
    pub fn publish(&mut self, channel: u8, size: usize) -> Result<(), HostError> {
        if self.pending != Some(channel) {
            return Err(HostError::InvalidState);
        }
        let total = self.total.checked_add(size).ok_or(HostError::Overflow)?;
        if total > (self.operation.auxiliary >> 32) as usize {
            return Err(HostError::ResourceLimit);
        }
        worker_main::publish(self.io, self.operation, channel, &self.buffer[..size])?;
        self.total = total;
        self.pending = None;
        Ok(())
    }
}
