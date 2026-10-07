//! Directly registered child ownership and opaque bounded ingress. This layer
//! neither grants platform permission nor parses a helper's untrusted payload.
use crate::{
    domain::{DomainInner, SessionHandle},
    process_api::{OwnedProcess, ProcessPlatform, ProcessState, SpawnSpec, Transfer},
    *,
};
use std::time::{Duration, Instant};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelperKind {
    ExternalSemantics,
    Capture,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelperHandle<'a> {
    pub(crate) session: SessionHandle<'a>,
    pub(crate) slot: usize,
    pub(crate) serial: u64,
}
pub struct HelperBytes<'a> {
    pub helper: HelperHandle<'a>,
    bytes: ByteLease<'a>,
}
impl HelperBytes<'_> {
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}
struct CaptureLease<'a> {
    domain: &'a DomainInner,
    serial: u64,
}
impl Drop for CaptureLease<'_> {
    fn drop(&mut self) {
        if self.domain.capture.get() == Some(self.serial) {
            self.domain.capture.set(None);
        }
    }
}
pub(crate) struct Helper<'a, C: OwnedProcess> {
    pub handle: HelperHandle<'a>,
    child: C,
    deadline: Instant,
    pub cleanup: Option<Instant>,
    pub lost: bool,
    pub reported: bool,
    ingress: Option<ByteLease<'a>>,
    used: usize,
    capture: Option<CaptureLease<'a>>,
}
impl<'a, C: OwnedProcess> Helper<'a, C> {
    pub(crate) fn spawn<P: ProcessPlatform<Child = C>>(
        domain: &'a DomainInner,
        session: SessionHandle<'a>,
        slot: usize,
        kind: HelperKind,
        spec: &SpawnSpec,
        deadline: Instant,
        platform: &mut P,
    ) -> Result<Self, HostError> {
        domain.check_reaping()?;
        if Instant::now() >= deadline {
            return Err(HostError::DeadlineExpired);
        }
        let serial = domain
            .helper_serial
            .get()
            .checked_add(1)
            .ok_or(HostError::Overflow)?;
        let capture = if kind == HelperKind::Capture {
            if domain.capture.get().is_some() {
                return Err(HostError::Busy);
            }
            domain.capture.set(Some(serial));
            Some(CaptureLease { domain, serial })
        } else {
            None
        };
        let ingress = domain.buffers.reserve(
            BufferClass::Ingress {
                worker: session.slot,
                helper: slot,
            },
            domain.limits.ingress_bytes,
        )?;
        let mut child = platform.spawn(spec)?;
        // Lost must be retained as an owner, never returned as ordinary spawn error.
        let lost = match child.try_reap() {
            Ok(ProcessState::Running) => false,
            Ok(ProcessState::Exited { .. } | ProcessState::Signaled { .. }) => {
                return Err(HostError::WorkerFailed);
            }
            Err(_) => {
                child.close_input();
                true
            }
        };
        domain.helper_serial.set(serial);
        Ok(Self {
            handle: HelperHandle {
                session,
                slot,
                serial,
            },
            child,
            deadline,
            cleanup: None,
            lost,
            reported: false,
            ingress: Some(ingress),
            used: 0,
            capture,
        })
    }
    fn io_ready(&mut self) -> Result<(), HostError> {
        if self.lost {
            return Err(HostError::CleanupPending);
        }
        if self.cleanup.is_some() {
            return Err(HostError::StaleOperation);
        }
        if Instant::now() >= self.deadline {
            self.stop(Instant::now());
            return Err(HostError::DeadlineExpired);
        }
        Ok(())
    }
    pub(crate) fn write(
        &mut self,
        input: &crate::host_types::InputLease<'a>,
        offset: usize,
    ) -> Result<Transfer, HostError> {
        self.io_ready()?;
        if input.session != self.handle.session {
            return Err(HostError::StaleOperation);
        }
        let bytes = input.bytes().get(offset..).ok_or(HostError::InvalidInput)?;
        if bytes.is_empty() {
            return Ok(Transfer::Bytes(0));
        }
        match self.child.write_input(bytes)? {
            Transfer::Bytes(0) | Transfer::Closed => {
                self.stop(Instant::now());
                Err(HostError::WorkerFailed)
            }
            value => Ok(value),
        }
    }
    pub(crate) fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, HostError> {
        self.io_ready()?;
        if bytes.is_empty() {
            return Ok(0);
        }
        match self.child.write_input(bytes)? {
            Transfer::Bytes(0) | Transfer::Closed => Err(HostError::WorkerFailed),
            Transfer::Bytes(n) => Ok(n),
            Transfer::WouldBlock => Ok(0),
        }
    }
    pub(crate) fn read_line(&mut self, cap: usize) -> Result<Option<usize>, HostError> {
        let result = self.read_limit(cap)?;
        let bytes = self
            .ingress
            .as_ref()
            .ok_or(HostError::InvalidState)?
            .as_slice();
        if let Some(end) = bytes[..self.used].iter().position(|&b| b == b'\n') {
            if end == 0 || end + 1 != self.used || self.used > cap {
                return Err(HostError::InvalidControl);
            }
            return Ok(Some(end));
        }
        if self.used >= cap {
            return Err(HostError::ResourceLimit);
        }
        if result == Transfer::Closed {
            return Err(HostError::WorkerFailed);
        }
        Ok(None)
    }
    pub(crate) fn take_line(&mut self, length: usize) -> Result<HelperBytes<'a>, HostError> {
        if length.checked_add(1) != Some(self.used) {
            return Err(HostError::InvalidControl);
        }
        let mut frame = self.take()?;
        frame.bytes.set_len(length)?;
        Ok(frame)
    }
    pub(crate) fn read(&mut self) -> Result<Transfer, HostError> {
        let cap = self.ingress.as_ref().ok_or(HostError::InvalidState)?.len();
        self.read_limit(cap)
    }
    fn read_limit(&mut self, cap: usize) -> Result<Transfer, HostError> {
        self.io_ready()?;
        let ingress = self.ingress.as_mut().ok_or(HostError::InvalidState)?;
        let cap = cap.min(ingress.len());
        if self.used >= cap {
            self.stop(Instant::now());
            return Err(HostError::ResourceLimit);
        }
        let result = self
            .child
            .read_output(&mut ingress.as_mut_slice()[self.used..cap])?;
        if let Transfer::Bytes(n) = result {
            self.used = self.used.checked_add(n).ok_or(HostError::Overflow)?;
        }
        Ok(result)
    }
    pub(crate) fn take(&mut self) -> Result<HelperBytes<'a>, HostError> {
        self.io_ready()?;
        let mut bytes = self.ingress.take().ok_or(HostError::InvalidState)?;
        bytes.set_len(self.used)?;
        self.stop(Instant::now());
        Ok(HelperBytes {
            helper: self.handle,
            bytes,
        })
    }
    pub(crate) fn stop(&mut self, now: Instant) {
        self.child.close_input();
        if self.cleanup.is_none() {
            if !self.lost && self.child.terminate() == Err(HostError::CleanupPending) {
                self.lost = true;
            }
            self.cleanup = now.checked_add(Duration::from_millis(
                self.handle.session.domain.limits.cleanup_ms,
            ));
        }
    }
    pub(crate) fn policy_lost(&mut self) {
        self.child.close_input();
        self.lost = true;
    }
    pub(crate) fn poll_cleanup(&mut self, now: Instant) -> Result<bool, HostError> {
        if self.lost {
            return Err(HostError::CleanupPending);
        }
        if self.cleanup.is_none() && now >= self.deadline {
            self.stop(now);
        }
        if self.cleanup.is_none() {
            return Ok(false);
        }
        match self.child.try_reap() {
            Ok(ProcessState::Exited { .. } | ProcessState::Signaled { .. }) => {
                self.capture = None;
                Ok(true)
            }
            Ok(ProcessState::Running) => {
                if self.cleanup.is_some_and(|until| now >= until) {
                    Err(HostError::CleanupPending)
                } else {
                    Ok(false)
                }
            }
            Err(_) => {
                self.lost = true;
                Err(HostError::CleanupPending)
            }
        }
    }
}
