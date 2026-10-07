//! Actual parent root/retained ownership. No canonical payload deserialization.
use crate::{authority::TargetLease, buffers::OutputGroup, limits::MAX_WORKERS, *};
use std::{
    any::TypeId,
    cell::Cell,
    mem::{forget, size_of},
    ptr,
};
use uiblueprint_engine::cache::{Allowance, Grant, LedgerLimits, QuotaLedger};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum SlotPhase {
    Free,
    Reserved,
    Booting,
    Attached,
    Running,
    Quarantined,
}
#[derive(Clone, Copy)]
pub(crate) struct Slot {
    pub epoch: u64,
    pub phase: SlotPhase,
    pub sequence: u64,
    pub target: Option<TargetLease>,
}
pub(crate) struct DomainInner {
    pub limits: HostLimits,
    pub platform: TypeId,
    pub ledger: QuotaLedger,
    pub buffers: ParentBuffers,
    pub slots: [Cell<Slot>; MAX_WORKERS],
    pub epoch: Cell<u64>,
    pub runtime_bytes: Cell<usize>,
    pub runtime_active: Cell<bool>,
    pub abandoned: Cell<bool>,
    pub reaping: crate::reaping::ParentReapingLease,
}
/// Heap location is stable for process/result leases. On failed destructor cleanup
/// the inner owner is intentionally retained, never freed behind leaked grants.
pub struct HostDomain {
    inner: Vec<DomainInner>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DomainUsage {
    pub parent_owned_bytes: usize,
    pub retained_reserved_bytes: usize,
    pub reserved_sessions: usize,
    pub completion_groups: usize,
    pub abandoned: bool,
    pub reaping_poisoned: bool,
}
#[derive(Clone, Copy)]
pub struct SessionHandle<'a> {
    pub(crate) domain: &'a DomainInner,
    pub(crate) slot: usize,
    pub(crate) epoch: u64,
}
impl std::fmt::Debug for SessionHandle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionHandle")
            .field("slot", &self.slot)
            .field("epoch", &self.epoch)
            .finish()
    }
}
impl PartialEq for SessionHandle<'_> {
    fn eq(&self, b: &Self) -> bool {
        ptr::eq(self.domain, b.domain) && self.slot == b.slot && self.epoch == b.epoch
    }
}
impl Eq for SessionHandle<'_> {}
impl HostDomain {
    /// Acquire the actual supported parent policy before allocating/spawning.
    /// The caller must keep SIGCHLD disposition and managed wait ownership stable
    /// until this domain is dropped after confirmed cleanup of all children.
    pub fn new<P: crate::process_api::ProcessPlatform + 'static>(
        limits: HostLimits,
    ) -> Result<Self, HostError> {
        let lease = crate::reaping::ParentReapingLease::acquire(P::validate_parent_reaping)?;
        Self::with_reaping(limits, lease, TypeId::of::<P>())
    }
    pub(crate) fn with_reaping(
        limits: HostLimits,
        reaping: crate::reaping::ParentReapingLease,
        platform: TypeId,
    ) -> Result<Self, HostError> {
        limits.validate()?;
        let mut inner = Vec::new();
        inner
            .try_reserve_exact(1)
            .map_err(|_| HostError::AllocationFailure)?;
        let root = add(
            size_of::<Self>(),
            mul(inner.capacity(), size_of::<DomainInner>())?,
        )?;
        let extra = root
            .checked_sub(size_of::<ParentBuffers>())
            .ok_or(HostError::Overflow)?;
        let ledger = QuotaLedger::new(LedgerLimits {
            retained_bytes: limits.retained_domain_bytes,
            session_slots: limits.workers,
            grants: limits.workers,
        })
        .map_err(|_| HostError::InvalidLimits)?;
        let buffers = ParentBuffers::new(limits, extra)?;
        inner.push(DomainInner {
            limits,
            platform,
            ledger,
            buffers,
            slots: std::array::from_fn(|_| {
                Cell::new(Slot {
                    epoch: 0,
                    phase: SlotPhase::Free,
                    sequence: 0,
                    target: None,
                })
            }),
            epoch: Cell::new(0),
            runtime_bytes: Cell::new(0),
            runtime_active: Cell::new(false),
            abandoned: Cell::new(false),
            reaping,
        });
        Ok(Self { inner })
    }
    pub(crate) fn inner(&self) -> &DomainInner {
        &self.inner[0]
    }
    pub fn usage(&self) -> DomainUsage {
        let d = self.inner();
        DomainUsage {
            parent_owned_bytes: d.buffers.usage().owned_bytes + d.runtime_bytes.get(),
            retained_reserved_bytes: d.ledger.usage().reserved_bytes,
            reserved_sessions: d.slots[..d.limits.workers]
                .iter()
                .filter(|s| s.get().phase != SlotPhase::Free)
                .count(),
            completion_groups: d.buffers.usage().leased_groups,
            abandoned: d.abandoned.get(),
            reaping_poisoned: d.reaping.poisoned(),
        }
    }
}
impl Drop for HostDomain {
    fn drop(&mut self) {
        if self.inner().abandoned.get() {
            forget(std::mem::take(&mut self.inner));
        }
    }
}

pub(crate) struct RuntimeRoot<'a> {
    domain: &'a DomainInner,
}
impl DomainInner {
    pub(crate) fn check_reaping(&self) -> Result<(), HostError> {
        if self.reaping.check().is_ok() {
            return Ok(());
        }
        let mut live = false;
        for cell in &self.slots[..self.limits.workers] {
            let mut slot = cell.get();
            if matches!(
                slot.phase,
                SlotPhase::Booting
                    | SlotPhase::Attached
                    | SlotPhase::Running
                    | SlotPhase::Quarantined
            ) {
                slot.phase = SlotPhase::Quarantined;
                cell.set(slot);
                live = true;
            }
        }
        // Preserve both the concrete root ledger and the process-wide lease if
        // a destructor is reached without confirmed cleanup of uncertain PIDs.
        if live {
            self.abandoned.set(true);
        }
        Err(HostError::CleanupPending)
    }
    pub(crate) fn reserve_runtime(&self, bytes: usize) -> Result<RuntimeRoot<'_>, HostError> {
        self.check_reaping()?;
        if self.abandoned.get() || self.runtime_active.get() {
            return Err(HostError::Busy);
        }
        if add(self.buffers.usage().owned_bytes, bytes)? > self.limits.parent_bytes {
            return Err(HostError::ResourceLimit);
        }
        self.runtime_active.set(true);
        self.runtime_bytes.set(bytes);
        Ok(RuntimeRoot { domain: self })
    }
    pub(crate) fn reserve_session(
        &self,
        target: TargetLease,
    ) -> Result<SessionReservation<'_>, HostError> {
        self.check_reaping()?;
        if self.abandoned.get() {
            return Err(HostError::CleanupPending);
        }
        let slot = self.slots[..self.limits.workers]
            .iter()
            .position(|s| s.get().phase == SlotPhase::Free)
            .ok_or(HostError::ResourceLimit)?;
        let epoch = self.epoch.get().checked_add(1).ok_or(HostError::Overflow)?;
        let grant = self
            .ledger
            .reserve(Allowance {
                bytes: self.limits.retained_per_worker,
                session_slots: 1,
            })
            .map_err(|_| HostError::ResourceLimit)?;
        self.slots[slot].set(Slot {
            epoch,
            phase: SlotPhase::Reserved,
            sequence: 0,
            target: Some(target),
        });
        self.epoch.set(epoch);
        Ok(SessionReservation {
            domain: self,
            slot,
            epoch,
            grant: Some(grant),
            reaped: true,
        })
    }
    pub(crate) fn check(&self, handle: SessionHandle<'_>) -> Result<Slot, HostError> {
        if !ptr::eq(self, handle.domain) || handle.slot >= self.limits.workers {
            return Err(HostError::StaleOperation);
        }
        let slot = self.slots[handle.slot].get();
        if slot.epoch != handle.epoch || slot.phase == SlotPhase::Free {
            return Err(HostError::StaleOperation);
        }
        Ok(slot)
    }
    pub(crate) fn group(&self, mask: u8) -> Result<OutputGroup<'_>, HostError> {
        self.buffers.reserve_group(mask)
    }
}
impl Drop for RuntimeRoot<'_> {
    fn drop(&mut self) {
        if !self.domain.abandoned.get() {
            self.domain.runtime_active.set(false);
            self.domain.runtime_bytes.set(0);
        }
    }
}

pub(crate) struct SessionReservation<'a> {
    pub domain: &'a DomainInner,
    pub slot: usize,
    pub epoch: u64,
    grant: Option<Grant<'a>>,
    reaped: bool,
}
impl<'a> SessionReservation<'a> {
    pub fn handle(&self) -> SessionHandle<'a> {
        SessionHandle {
            domain: self.domain,
            slot: self.slot,
            epoch: self.epoch,
        }
    }
    pub fn child_started(&mut self) {
        self.reaped = false;
        let mut state = self.domain.slots[self.slot].get();
        state.phase = SlotPhase::Booting;
        self.domain.slots[self.slot].set(state);
    }
    pub fn child_reaped(&mut self) {
        self.reaped = true;
    }
    pub fn allowance(&self) -> usize {
        self.domain.limits.retained_per_worker
    }
    pub fn quarantine(&self) {
        let mut state = self.domain.slots[self.slot].get();
        state.phase = SlotPhase::Quarantined;
        self.domain.slots[self.slot].set(state);
    }
}
impl Drop for SessionReservation<'_> {
    fn drop(&mut self) {
        if self.reaped {
            self.domain.slots[self.slot].set(Slot {
                epoch: self.epoch,
                phase: SlotPhase::Free,
                sequence: 0,
                target: None,
            });
        } else {
            self.quarantine();
            self.domain.abandoned.set(true);
            if let Some(grant) = self.grant.take() {
                forget(grant);
            }
        }
    }
}
