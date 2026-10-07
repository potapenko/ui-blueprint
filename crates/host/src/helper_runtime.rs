//! Runtime's explicit helper API. Helper payloads stay opaque, precharged bytes.
use super::*;
use crate::helpers::{Helper, HelperBytes, HelperHandle, HelperKind};
impl<'a, P: ProcessPlatform + 'static> RuntimeHost<'a, P> {
    /// Caller supplies the authorized helper executable/target and deadline. This
    /// ownership API grants no platform capture permission and never opens UI.
    pub fn spawn_helper(
        &mut self,
        session: SessionHandle<'a>,
        kind: HelperKind,
        spec: SpawnSpec,
        deadline: Instant,
    ) -> Result<HelperHandle<'a>, HostError> {
        self.domain.check_reaping()?;
        let slot = self.domain.check(session)?;
        if self.state[0].shutting_down
            || !matches!(slot.phase, SlotPhase::Attached | SlotPhase::Running)
        {
            return Err(HostError::InvalidState);
        }
        let state = &mut self.state[0];
        let worker = state.workers[session.slot]
            .as_mut()
            .ok_or(HostError::StaleOperation)?;
        if worker.cleanup_deadline.is_some() || worker.ownership_lost {
            return Err(HostError::CleanupPending);
        }
        spawn_registered(worker, &mut state.platform, kind, &spec, deadline)
    }
    fn helper_mut(
        &mut self,
        handle: HelperHandle<'a>,
    ) -> Result<&mut Helper<'a, P::Child>, HostError> {
        self.domain.check_reaping()?;
        self.domain.check(handle.session)?;
        self.state[0].workers[handle.session.slot]
            .as_mut()
            .and_then(|w| w.helpers.get_mut(handle.slot))
            .and_then(Option::as_mut)
            .filter(|h| h.handle == handle)
            .ok_or(HostError::StaleOperation)
    }
    /// Borrow an already charged input lease; callers advance only by Bytes(n).
    /// No request copy, implicit retry, formatting or JSON parse occurs here.
    pub fn write_helper(
        &mut self,
        handle: HelperHandle<'a>,
        input: &InputLease<'a>,
        offset: usize,
    ) -> Result<Transfer, HostError> {
        self.helper_mut(handle)?.write(input, offset)
    }
    pub fn read_helper(&mut self, handle: HelperHandle<'a>) -> Result<Transfer, HostError> {
        self.helper_mut(handle)?.read()
    }
    /// Move raw bounded ingress to the caller, then start owned helper cleanup.
    /// These bytes are NOT a validated/committed canonical channel. Typed reuse
    /// belongs in the admitted worker; capture lease still awaits confirmed reap.
    pub fn take_helper_bytes(
        &mut self,
        handle: HelperHandle<'a>,
    ) -> Result<HelperBytes<'a>, HostError> {
        self.helper_mut(handle)?.take()
    }
    pub fn close_helper(&mut self, handle: HelperHandle<'a>) -> Result<(), HostError> {
        self.helper_mut(handle)?.stop(Instant::now());
        Ok(())
    }
}

pub(super) fn poll_helpers<'a, C: OwnedProcess>(
    worker: &mut Worker<'a, C>,
    now: Instant,
) -> Option<HostEvent<'a>> {
    for index in 0..crate::limits::HELPERS {
        let Some(helper) = worker.helpers[index].as_mut() else {
            continue;
        };
        match helper.poll_cleanup(now) {
            Ok(true) => {
                let handle = helper.handle;
                worker.helpers[index] = None;
                return Some(HostEvent::HelperClosed { helper: handle });
            }
            Ok(false) => (),
            Err(_) => {
                let lost = helper.lost;
                let event = if !helper.reported {
                    helper.reported = true;
                    Some(HostEvent::HelperCleanupPending {
                        helper: helper.handle,
                    })
                } else {
                    None
                };
                if lost {
                    worker.reservation.domain.abandoned.set(true);
                    worker.reservation.quarantine();
                    if worker.cleanup_deadline.is_none() {
                        begin_cleanup(worker, worker.reservation.domain.limits.cleanup_ms, now);
                        if worker.active.is_some() {
                            worker.pending_terminal =
                                Some(Terminal::Failed(HostError::CleanupPending));
                        }
                    }
                }
                if event.is_some() {
                    return event;
                }
            }
        }
    }
    None
}

pub(super) fn spawn_registered<'a, P: ProcessPlatform>(
    worker: &mut Worker<'a, P::Child>,
    platform: &mut P,
    kind: HelperKind,
    spec: &SpawnSpec,
    deadline: Instant,
) -> Result<HelperHandle<'a>, HostError> {
    let domain = worker.reservation.domain;
    domain.check_reaping()?;
    let session = worker.reservation.handle();
    // A reaped helper's ingress may still belong to a caller-held frame.
    // Search the two fixed slots rather than letting that lease hide the
    // other free slot. Busy admission returns no child to discard/retry.
    let mut admitted = None;
    let mut busy = false;
    for index in 0..worker.helpers.len() {
        if worker.helpers[index].is_some() {
            continue;
        }
        match Helper::spawn(domain, session, index, kind, spec, deadline, platform) {
            Ok(helper) => {
                admitted = Some((index, helper));
                break;
            }
            Err(HostError::Busy) => busy = true,
            Err(error) => return Err(error),
        }
    }
    let (index, helper) = admitted.ok_or(if busy {
        HostError::Busy
    } else {
        HostError::ResourceLimit
    })?;
    let handle = helper.handle;
    let lost = helper.lost;
    worker.helpers[index] = Some(helper);
    if lost {
        domain.abandoned.set(true);
        worker.reservation.quarantine();
        begin_cleanup(worker, domain.limits.cleanup_ms, Instant::now());
        if worker.active.is_some() {
            worker.pending_terminal = Some(Terminal::Failed(HostError::CleanupPending));
        }
        return Err(HostError::CleanupPending);
    }
    Ok(handle)
}
