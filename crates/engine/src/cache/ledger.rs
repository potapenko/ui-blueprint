use super::{CacheError, MAX_SESSIONS, MIB, add};
use std::{cell::Cell, mem::size_of};

/// Explicit aggregate ceilings. No constructor supplies defaults.
#[derive(Clone, Copy, Debug)]
pub struct LedgerLimits {
    pub retained_bytes: usize,
    pub session_slots: usize,
    pub grants: usize,
}
/// A reserved retained allowance, independent of currently used bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Allowance {
    pub bytes: usize,
    pub session_slots: usize,
}
#[derive(Clone, Copy)]
struct Record {
    serial: u64,
    allowance: Allowance,
    used: usize,
}

/// The one host-owned domain. Fixed inline grant slots are charged even if vacant.
/// It is intentionally not Sync: updates are synchronous, with no callbacks.
/// All Stores borrow this owner, so it cannot disappear before their grants.
pub struct QuotaLedger {
    limits: LedgerLimits,
    records: [Cell<Option<Record>>; MAX_SESSIONS],
    serial: Cell<u64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LedgerUsage {
    pub backing_bytes: usize,
    /// Ledger backing plus outstanding allowances, not payload usage.
    pub reserved_bytes: usize,
    /// Ledger backing plus published Store owned-layout charges, counted once.
    pub used_bytes: usize,
    pub session_slots: usize,
    pub grants: usize,
}
impl QuotaLedger {
    pub const fn backing_bytes() -> usize {
        size_of::<Self>()
    }
    /// # Errors
    /// Rejects zero/above-profile limits and a domain smaller than its own root.
    pub fn new(limits: LedgerLimits) -> Result<Self, CacheError> {
        if limits.retained_bytes < size_of::<Self>()
            || limits.retained_bytes > 64 * MIB
            || !(1..=MAX_SESSIONS).contains(&limits.session_slots)
            || !(1..=MAX_SESSIONS).contains(&limits.grants)
        {
            return Err(CacheError::InvalidLimits);
        }
        Ok(Self {
            limits,
            records: std::array::from_fn(|_| Cell::new(None)),
            serial: Cell::new(0),
        })
    }
    pub fn usage(&self) -> LedgerUsage {
        let mut usage = LedgerUsage {
            backing_bytes: size_of::<Self>(),
            reserved_bytes: size_of::<Self>(),
            used_bytes: size_of::<Self>(),
            session_slots: 0,
            grants: 0,
        };
        for record in self.records.iter().filter_map(Cell::get) {
            // At most four admitted allowances; reserve checked their domain sum.
            usage.reserved_bytes += record.allowance.bytes;
            usage.used_bytes += record.used;
            usage.session_slots += record.allowance.session_slots;
            usage.grants += 1;
        }
        usage
    }
    /// Reserve before Store construction. Grant is move-only and returns its
    /// allowance on Drop; forgetting ownership cannot make capacity reusable.
    /// # Errors
    /// Refuses exhausted byte/session/grant caps or a sequence overflow.
    pub fn reserve(&self, allowance: Allowance) -> Result<Grant<'_>, CacheError> {
        if allowance.bytes == 0 || allowance.session_slots == 0 {
            return Err(CacheError::InvalidLimits);
        }
        let usage = self.usage();
        if add(usage.reserved_bytes, allowance.bytes)? > self.limits.retained_bytes
            || add(usage.session_slots, allowance.session_slots)? > self.limits.session_slots
            || usage.grants >= self.limits.grants
        {
            return Err(CacheError::ResourceLimit);
        }
        let serial = self
            .serial
            .get()
            .checked_add(1)
            .ok_or(CacheError::CounterOverflow)?;
        let slot = self
            .records
            .iter()
            .position(|slot| slot.get().is_none())
            .ok_or(CacheError::ResourceLimit)?;
        self.records[slot].set(Some(Record {
            serial,
            allowance,
            used: 0,
        }));
        self.serial.set(serial);
        Ok(Grant {
            ledger: self,
            slot,
            serial,
            allowance,
        })
    }
}

/// Move-only lease, with no owned payload or hidden allocation. Do not implement
/// Clone/Copy: allowance reuse is only possible after the real owner drops it.
pub struct Grant<'a> {
    pub(super) ledger: &'a QuotaLedger,
    slot: usize,
    pub(super) serial: u64,
    allowance: Allowance,
}
impl Grant<'_> {
    pub fn allowance(&self) -> Allowance {
        self.allowance
    }
    pub(super) fn publish_usage(&self, used: usize) {
        // Private callers preflight used<=allowance; this grant is the unique
        // writer of its occupied ledger slot and has no callback/reentrancy.
        debug_assert!(used <= self.allowance.bytes);
        self.ledger.records[self.slot].set(Some(Record {
            serial: self.serial,
            allowance: self.allowance,
            used,
        }));
    }
}
impl Drop for Grant<'_> {
    fn drop(&mut self) {
        if self.ledger.records[self.slot]
            .get()
            .is_some_and(|r| r.serial == self.serial)
        {
            self.ledger.records[self.slot].set(None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grant_sequence_does_not_wrap_or_publish() {
        let ledger = QuotaLedger::new(LedgerLimits {
            retained_bytes: 1024,
            session_slots: 1,
            grants: 1,
        })
        .expect("ledger");
        ledger.serial.set(u64::MAX);
        let before = ledger.usage();
        assert!(matches!(
            ledger.reserve(Allowance {
                bytes: 1,
                session_slots: 1
            }),
            Err(CacheError::CounterOverflow)
        ));
        assert_eq!(ledger.usage(), before);
    }
}
