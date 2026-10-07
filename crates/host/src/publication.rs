//! Parent-side byte publication; no graph/JSON decoding, allocation or DTO clone.
use crate::{Control, ControlKind, Correlation, HostError, OperationClass, buffers::OutputGroup};

pub struct Publication<'a> {
    group: OutputGroup<'a>,
    correlation: Correlation,
    class: OperationClass,
    requested: u8,
    committed: u8,
    incomplete: u8,
    total: usize,
    limit: usize,
    active: Option<Control>,
    received: usize,
    pending_ack: bool,
    terminal: bool,
}
pub struct CommittedFrames<'a> {
    group: OutputGroup<'a>,
    pub committed: u8,
    pub missing: u8,
    pub incomplete: u8,
}
impl CommittedFrames<'_> {
    pub fn frame(&self, slot: usize) -> Option<&[u8]> {
        self.group.frame(slot)
    }
}
impl<'a> Publication<'a> {
    pub fn new(
        group: OutputGroup<'a>,
        correlation: Correlation,
        class: OperationClass,
        requested: u8,
        limit: usize,
    ) -> Result<Self, HostError> {
        if requested == 0 || requested & !7 != 0 || limit == 0 {
            return Err(HostError::InvalidInput);
        }
        for slot in 0..3 {
            if (requested & (1 << slot) != 0) != group.frames[slot].is_some() {
                return Err(HostError::InvalidInput);
            }
        }
        Ok(Self {
            group,
            correlation,
            class,
            requested,
            committed: 0,
            incomplete: 0,
            total: 0,
            limit,
            active: None,
            received: 0,
            pending_ack: false,
            terminal: false,
        })
    }
    fn envelope(&self, control: Control) -> Result<(), HostError> {
        if self.terminal {
            return Err(HostError::StaleOperation);
        }
        if control.correlation != self.correlation || control.class != self.class {
            return Err(HostError::StaleOperation);
        }
        if control.flags > 1
            || (control.flags != 0
                && !matches!(
                    self.class,
                    OperationClass::Observe | OperationClass::Mutation
                ))
            || control.value != 0
            || control.auxiliary != 0
            || control.slot >= 3
        {
            return Err(HostError::InvalidControl);
        }
        Ok(())
    }
    pub fn begin(&mut self, control: Control) -> Result<(), HostError> {
        self.envelope(control)?;
        if control.kind != ControlKind::Frame
            || self.active.is_some()
            || self.pending_ack
            || self.requested & (1 << control.slot) == 0
            || self.committed & (1 << control.slot) != 0
        {
            return Err(HostError::InvalidControl);
        }
        let length = usize::try_from(control.length).map_err(|_| HostError::ResourceLimit)?;
        if length == 0
            || self
                .total
                .checked_add(length)
                .is_none_or(|n| n > self.limit)
        {
            return Err(HostError::ResourceLimit);
        }
        let frame = self.group.frames[control.slot as usize]
            .as_mut()
            .ok_or(HostError::InvalidState)?;
        frame.set_len(length)?;
        self.active = Some(control);
        self.received = 0;
        Ok(())
    }
    /// The process driver copies directly into this already reserved target slice.
    pub fn remaining_mut(&mut self) -> Result<&mut [u8], HostError> {
        if self.terminal || self.pending_ack {
            return Err(HostError::InvalidState);
        }
        let header = self.active.ok_or(HostError::InvalidState)?;
        let frame = self.group.frames[header.slot as usize]
            .as_mut()
            .ok_or(HostError::InvalidState)?;
        Ok(&mut frame.as_mut_slice()[self.received..])
    }
    pub fn advance(&mut self, bytes: usize) -> Result<(), HostError> {
        let remaining = self.remaining_mut()?.len();
        if bytes > remaining {
            return Err(HostError::InvalidControl);
        }
        self.received += bytes;
        Ok(())
    }
    pub fn commit(&mut self, control: Control) -> Result<Control, HostError> {
        self.envelope(control)?;
        let frame = self.active.ok_or(HostError::InvalidState)?;
        if control.kind != ControlKind::Commit
            || !control.matches(frame)
            || control.flags != frame.flags
            || self.pending_ack
            || self.received as u64 != frame.length
        {
            return Err(HostError::InvalidControl);
        }
        self.pending_ack = true;
        Ok(Control {
            kind: ControlKind::Ack,
            ..control
        })
    }
    /// Only call after the entire matching ACK was sent before terminalization.
    pub fn ack_sent(&mut self, ack: Control) -> Result<(), HostError> {
        self.envelope(ack)?;
        let frame = self.active.ok_or(HostError::InvalidState)?;
        if !self.pending_ack
            || ack.kind != ControlKind::Ack
            || !ack.matches(frame)
            || ack.flags != frame.flags
        {
            return Err(HostError::InvalidControl);
        }
        self.committed |= 1 << frame.slot;
        if frame.flags == 1 && self.class == OperationClass::Observe {
            self.incomplete |= 1 << frame.slot;
        }
        self.total += self.received;
        self.active = None;
        self.pending_ack = false;
        Ok(())
    }
    pub fn terminalize(&mut self) {
        self.terminal = true;
        self.active = None;
        self.pending_ack = false;
        for (slot, frame) in self.group.frames.iter_mut().enumerate() {
            if self.committed & (1 << slot) == 0 {
                *frame = None;
            }
        }
    }
    pub fn finish(mut self) -> CommittedFrames<'a> {
        self.terminalize();
        CommittedFrames {
            group: self.group,
            committed: self.committed,
            missing: self.requested & !self.committed,
            incomplete: self.incomplete,
        }
    }
    pub(crate) fn idle(&self) -> bool {
        !self.terminal && self.active.is_none() && !self.pending_ack
    }
    pub(crate) fn remaining_bytes(&self) -> usize {
        self.limit - self.total
    }
    pub fn committed(&self) -> u8 {
        self.committed
    }
}
