//! Finite parent Native exchange: fixed controls, one original charged request,
//! one bounded NDJSON reply per channel. No graph/JSON decode or executable from UI.
use super::*;
use crate::{
    helpers::{HelperBytes, HelperHandle, HelperKind},
    native_binding::NativeHelperBinding,
};
#[derive(Clone, Copy)]
enum Stage {
    Admit,
    Configure,
    Configuration,
    Submit,
    Request,
    Read,
    Reply,
    Body,
}
pub(super) struct NativeBroker<'a> {
    pub(super) channel: u8,
    ticket: u64,
    helper: Option<HelperHandle<'a>>,
    stage: Stage,
    offset: usize,
    header: [u8; CONTROL_BYTES],
    frame: Option<HelperBytes<'a>>,
    cap: usize,
    failed: bool,
    phase: u8,
    nonce: u64,
    resident: bool,
}
impl<'a> NativeBroker<'a> {
    pub(super) fn new(channel: u8, ticket: u64, cap: usize, phase: u8, nonce: u64) -> Self {
        Self {
            channel,
            ticket,
            helper: None,
            stage: Stage::Admit,
            offset: 0,
            header: [0; CONTROL_BYTES],
            frame: None,
            cap,
            failed: false,
            phase,
            nonce,
            resident: false,
        }
    }
    fn control<C: OwnedProcess>(
        &self,
        w: &Worker<'a, C>,
        kind: ControlKind,
        length: usize,
        value: u64,
        auxiliary: u64,
        flags: u8,
    ) -> Control {
        let active = w.active.as_ref().expect("broker belongs to active observe");
        Control {
            kind,
            class: active.class,
            slot: self.channel,
            flags,
            correlation: Correlation {
                session_epoch: active.handle.session.epoch,
                operation: active.handle.sequence,
            },
            length: length as u64,
            value,
            auxiliary,
        }
    }
    fn failure<C: OwnedProcess>(&mut self, w: &Worker<'a, C>, error: HostError) {
        let flags = match error {
            HostError::ResourceLimit => 1,
            HostError::DeadlineExpired => 2,
            HostError::PermissionDenied => 4,
            HostError::CleanupPending => 5,
            _ => 3,
        };
        self.header = self
            .control(
                w,
                ControlKind::HelperReply,
                0,
                self.helper.map_or(0, |h| h.serial),
                self.ticket,
                flags,
            )
            .encode();
        self.failed = true;
        self.stage = Stage::Reply;
        self.offset = 0;
    }
    pub(super) fn advance<P: ProcessPlatform>(
        &mut self,
        w: &mut Worker<'a, P::Child>,
        platform: &mut P,
        binding: &NativeHelperBinding,
        now: Instant,
    ) -> Result<bool, HostError> {
        let deadline = w.active.as_ref().ok_or(HostError::StaleOperation)?.deadline;
        if now >= deadline {
            return Err(HostError::DeadlineExpired);
        }
        match self.stage {
            Stage::Admit => {
                if self.channel > 2 || binding.channels & (1 << self.channel) == 0 {
                    self.failure(w, HostError::PermissionDenied);
                    return Ok(false);
                }
                let kind = if self.channel == 1 {
                    HelperKind::Capture
                } else {
                    // AX and probe use the existing non-capture resource lane.
                    // The canonical channel remains in self.channel, not a third
                    // helper entitlement: spawn_registered owns two fixed slots.
                    HelperKind::ExternalSemantics
                };
                self.resident = binding.residency.is_some();
                if self.resident && self.channel != 0 {
                    return Err(HostError::InvalidControl);
                }
                if let Some(handle) = w.resident.filter(|_| self.resident) {
                    if !w
                        .helpers
                        .iter()
                        .flatten()
                        .any(|h| h.handle == handle && h.cleanup.is_none())
                    {
                        return Err(HostError::ResyncRequired);
                    }
                    self.helper = Some(handle);
                } else {
                    if self.resident && self.phase != 0 {
                        return Err(HostError::ResyncRequired);
                    }
                    match helper_runtime::spawn_registered(
                        w,
                        platform,
                        kind,
                        &binding.executable,
                        binding.residency.unwrap_or(deadline),
                    ) {
                        Ok(handle) => {
                            self.helper = Some(handle);
                            if self.resident {
                                w.resident = Some(handle);
                            }
                        }
                        Err(HostError::Busy) => return Ok(false),
                        Err(HostError::CleanupPending) => return Err(HostError::CleanupPending),
                        Err(e) => {
                            self.failure(w, e);
                            return Ok(false);
                        }
                    }
                }
                self.header = self
                    .control(
                        w,
                        ControlKind::Configure,
                        binding.bytes().len(),
                        self.ticket,
                        self.cap as u64,
                        0,
                    )
                    .encode();
                if self.resident {
                    let mut c = Control::decode(&self.header)?;
                    c.flags = 128 | self.phase;
                    c.value = self.helper.ok_or(HostError::InvalidState)?.serial;
                    self.header = c.encode();
                }
                self.stage = Stage::Configure;
            }
            Stage::Configure | Stage::Configuration | Stage::Submit | Stage::Request => {
                let handle = self.helper.ok_or(HostError::InvalidState)?;
                let bytes = match self.stage {
                    Stage::Configure | Stage::Submit => &self.header[self.offset..],
                    Stage::Configuration => &binding.bytes()[self.offset..],
                    Stage::Request => {
                        &w.input.as_ref().ok_or(HostError::InvalidState)?.bytes()[self.offset..]
                    }
                    _ => return Err(HostError::InvalidState),
                };
                let length = bytes.len();
                let helper = w.helpers[handle.slot]
                    .as_mut()
                    .filter(|h| h.handle == handle)
                    .ok_or(HostError::StaleOperation)?;
                match helper.write_bytes(bytes) {
                    Ok(n) => self.offset += n,
                    Err(e) => {
                        helper.stop(now);
                        self.failure(w, e);
                        return Ok(false);
                    }
                }
                if length == 0
                    || self.offset
                        == match self.stage {
                            Stage::Configure | Stage::Submit => CONTROL_BYTES,
                            Stage::Configuration => binding.bytes().len(),
                            Stage::Request => w
                                .input
                                .as_ref()
                                .ok_or(HostError::InvalidState)?
                                .bytes()
                                .len(),
                            _ => 0,
                        }
                {
                    self.offset = 0;
                    self.stage = match self.stage {
                        Stage::Configure => Stage::Configuration,
                        Stage::Configuration => {
                            let size = w
                                .input
                                .as_ref()
                                .ok_or(HostError::InvalidState)?
                                .bytes()
                                .len();
                            self.header = self
                                .control(
                                    w,
                                    ControlKind::Submit,
                                    size,
                                    self.ticket,
                                    remaining_ms(deadline, Instant::now())?,
                                    0,
                                )
                                .encode();
                            if self.resident {
                                let mut c = Control::decode(&self.header)?;
                                c.flags = 128 | self.phase;
                                // Nonce is only forwarded by the validated parent phase2.
                                if self.phase == 2 {
                                    c.value = self.nonce;
                                }
                                self.header = c.encode();
                            }
                            Stage::Submit
                        }
                        Stage::Submit => Stage::Request,
                        Stage::Request => Stage::Read,
                        _ => return Err(HostError::InvalidState),
                    };
                }
            }
            Stage::Read => {
                let handle = self.helper.ok_or(HostError::InvalidState)?;
                let helper = w.helpers[handle.slot]
                    .as_mut()
                    .filter(|h| h.handle == handle)
                    .ok_or(HostError::StaleOperation)?;
                match helper.read_line(self.cap) {
                    Ok(Some(length)) => {
                        self.frame = Some(if self.resident {
                            helper.take_resident_line(length)?
                        } else {
                            helper.take_line(length)?
                        });
                        self.header = self
                            .control(
                                w,
                                ControlKind::HelperReply,
                                length,
                                handle.serial,
                                self.ticket,
                                0,
                            )
                            .encode();
                        self.offset = 0;
                        self.stage = Stage::Reply;
                    }
                    Ok(None) => (),
                    Err(e) => {
                        helper.stop(now);
                        self.failure(w, e);
                    }
                }
            }
            Stage::Reply | Stage::Body => {
                let bytes = if matches!(self.stage, Stage::Reply) {
                    &self.header[..]
                } else {
                    self.frame.as_ref().ok_or(HostError::InvalidState)?.bytes()
                };
                let length = bytes.len();
                match w.child.write_input(&bytes[self.offset..])? {
                    Transfer::Bytes(0) | Transfer::Closed => return Err(HostError::WorkerFailed),
                    Transfer::Bytes(n) => self.offset += n,
                    Transfer::WouldBlock => (),
                }
                if self.offset == length {
                    self.offset = 0;
                    if matches!(self.stage, Stage::Body) || self.failed {
                        if self.resident && !self.failed {
                            let handle = self.helper.ok_or(HostError::InvalidState)?;
                            w.helpers[handle.slot]
                                .as_mut()
                                .ok_or(HostError::StaleOperation)?
                                .restore_resident_line(
                                    self.frame.take().ok_or(HostError::InvalidState)?,
                                )?;
                        }
                        if self.failed {
                            w.active
                                .as_mut()
                                .ok_or(HostError::InvalidState)?
                                .native_failed |= 1 << self.channel;
                        }
                        return Ok(true);
                    }
                    self.stage = Stage::Body;
                }
            }
        }
        Ok(false)
    }
}
