//! Fixed parent-owned dispatch claims. A claim lives through uncertain cleanup;
//! a permit is single-use within its session/operation correlation.
use crate::{HostError, OperationClass, domain::DomainInner, limits::MAX_WORKERS};
use std::cell::Cell;
#[derive(Clone, Copy)]
struct Claim {
    epoch: u64,
    nonce: Option<u64>,
    physical: bool,
}
pub(crate) struct EffectLanes {
    claims: [Cell<Option<Claim>>; MAX_WORKERS],
    serial: Cell<u64>,
}
impl EffectLanes {
    pub(crate) fn new() -> Self {
        Self {
            claims: std::array::from_fn(|_| Cell::new(None)),
            serial: Cell::new(0),
        }
    }
}
pub(crate) struct MutationLease<'a> {
    domain: &'a DomainInner,
    slot: usize,
    epoch: u64,
}
impl<'a> MutationLease<'a> {
    pub(crate) fn acquire(domain: &'a DomainInner, slot: usize) -> Result<Self, HostError> {
        let state = domain.slots[slot].get();
        let target = state.target.ok_or(HostError::StaleOperation)?;
        if !target.permits(OperationClass::Mutation) {
            return Err(HostError::PermissionDenied);
        }
        for (index, claim) in domain.effects.claims.iter().enumerate() {
            if claim.get().is_some()
                && domain.slots[index]
                    .get()
                    .target
                    .is_some_and(|t| t.id() == target.id() && t.generation() == target.generation())
            {
                return Err(HostError::Busy);
            }
        }
        domain.effects.claims[slot].set(Some(Claim {
            epoch: state.epoch,
            nonce: None,
            physical: false,
        }));
        Ok(Self {
            domain,
            slot,
            epoch: state.epoch,
        })
    }
    pub(crate) fn permit(&self, physical: bool) -> Result<u64, HostError> {
        self.domain.check_reaping()?;
        let mut claim = self.domain.effects.claims[self.slot]
            .get()
            .ok_or(HostError::StaleOperation)?;
        if claim.epoch != self.epoch || claim.nonce.is_some() {
            return Err(HostError::InvalidControl);
        }
        if physical
            && self
                .domain
                .effects
                .claims
                .iter()
                .any(|c| c.get().is_some_and(|c| c.physical))
        {
            return Err(HostError::Busy);
        }
        let nonce = self
            .domain
            .effects
            .serial
            .get()
            .checked_add(1)
            .ok_or(HostError::Overflow)?;
        self.domain.effects.serial.set(nonce);
        claim.nonce = Some(nonce);
        claim.physical = physical;
        self.domain.effects.claims[self.slot].set(Some(claim));
        Ok(nonce)
    }
}
impl Drop for MutationLease<'_> {
    fn drop(&mut self) {
        // Runtime leaks unreaped owners intentionally; this release is reached
        // only after confirmed terminal delivery/non-dispatch or actual reap.
        if self.domain.effects.claims[self.slot]
            .get()
            .is_some_and(|c| c.epoch == self.epoch)
        {
            self.domain.effects.claims[self.slot].set(None);
        }
    }
}
