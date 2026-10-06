//! D05-MEMORY retained owned-layout quotas and immutable Snapshot storage.
//!
//! Limits are explicit; this does not enforce decoder/replay/encoding peaks, RSS,
//! pixel leases, runtime freshness or external caller/Completion allocations.
mod admission;
mod ledger;
mod store;
mod types;

pub use ledger::{Allowance, Grant, LedgerLimits, LedgerUsage, QuotaLedger};
pub use store::{CacheStore, StoreLayout};
pub use types::*;

use uiblueprint_schema::owned_size::HeapSize;

const MIB: usize = 1_048_576;
const MAX_SESSIONS: usize = 4;
const MAX_SLOTS: usize = 16;
fn add(a: usize, b: usize) -> Result<usize, CacheError> {
    a.checked_add(b).ok_or(CacheError::Overflow)
}
fn heap(value: &impl HeapSize) -> Result<usize, CacheError> {
    value
        .heap()
        .map(|size| size.bytes)
        .map_err(|_| CacheError::Overflow)
}
fn backing<T>(capacity: usize) -> Result<usize, CacheError> {
    capacity
        .checked_mul(std::mem::size_of::<T>())
        .ok_or(CacheError::Overflow)
}
fn slots<T: Default>(count: usize) -> Result<Vec<T>, CacheError> {
    backing::<T>(count)?;
    let mut result = Vec::new();
    result
        .try_reserve_exact(count)
        .map_err(|_| CacheError::AllocationFailure)?;
    result.resize_with(count, T::default);
    Ok(result)
}
