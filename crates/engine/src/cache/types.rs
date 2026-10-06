use super::QuotaLedger;
use crate::replay::ReplayError;
use std::{fmt, ptr};
use uiblueprint_schema::{model::*, validation::ValidationError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheError {
    InvalidLimits,
    ResourceLimit,
    Overflow,
    CounterOverflow,
    AllocationFailure,
    InvalidIdentity,
    InvalidPartition,
    OutsideSession,
    ConflictingIdentity,
    InvalidClock,
    InvalidHandle,
    ResyncRequired,
    RevalidationRequired,
    InvalidSnapshot(ValidationError),
    Replay(ReplayError),
}
impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for CacheError {}

/// Keeps ownership on failure. Debug deliberately omits the returned payload.
pub struct Rejected<T> {
    pub error: CacheError,
    pub input: T,
}
impl<T> fmt::Debug for Rejected<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rejected")
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}
/// Every value is mandatory and at/below the D05 profile. Store byte ceiling is
/// the explicit Grant allowance, never an implicit full host-domain allowance.
#[derive(Clone, Copy, Debug)]
pub struct StoreLimits {
    pub session_slots: usize,
    pub snapshot_slots_per_session: usize,
    pub revisions_per_family: usize,
    pub entry_bytes: usize,
    pub session_bytes: usize,
    pub retention_ms: u64,
}
/// Cache metadata only, not an alternate serialized session/graph schema.
pub struct SessionIdentity {
    pub session_id: Id,
    pub target: Identity,
    pub plugin: PluginIdentity,
}
/// Already decoded/sanitized caller-owned data. On admission error it is returned
/// unchanged. No opaque permit claims to enforce its prior decoding allocations.
pub struct Incoming {
    pub snapshot: Snapshot,
    pub partition: Vec<Channel>,
}
#[derive(Clone, Copy)]
pub struct Clock<'a> {
    pub domain: &'a Id,
    pub milliseconds: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadPolicy {
    Recorded,
    /// This owner has no live revalidation operation; current-required reads
    /// refuse rather than treating TTL or stored freshness as new evidence.
    CurrentRequired,
}
/// Full-context borrowed key. Set-valued Context members use canonical equality.
pub struct SnapshotKey<'a> {
    pub context: &'a Context,
    pub partition: &'a [Channel],
    pub snapshot_id: &'a Id,
    pub revision: u64,
}
#[derive(Clone, Copy)]
pub struct SessionHandle<'a> {
    pub(super) domain: &'a QuotaLedger,
    pub(super) store: u64,
    pub(super) slot: usize,
    pub(super) generation: u64,
}
impl PartialEq for SessionHandle<'_> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.domain, other.domain)
            && self.store == other.store
            && self.slot == other.slot
            && self.generation == other.generation
    }
}
impl Eq for SessionHandle<'_> {}
impl fmt::Debug for SessionHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionHandle")
            .field("store", &self.store)
            .field("slot", &self.slot)
            .field("generation", &self.generation)
            .finish()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RevisionHandle<'a> {
    pub(super) session: SessionHandle<'a>,
    pub(super) slot: usize,
    pub(super) generation: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Admission<'a> {
    pub handle: RevisionHandle<'a>,
    pub reused: bool,
    pub evicted_entries: usize,
}
pub struct StoredRead<'store, 'ledger> {
    pub handle: RevisionHandle<'ledger>,
    pub snapshot: &'store Snapshot,
    pub partition: &'store [Channel],
    pub invalidated: bool,
    pub expires_at_ms: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StoreUsage {
    pub owned_bytes: usize,
    pub allowance_bytes: usize,
    pub session_capacity: usize,
    pub sessions: usize,
    pub snapshots: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionUsage {
    pub owned_bytes: usize,
    pub snapshot_capacity: usize,
    pub snapshots: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReleasedCharge {
    pub owned_bytes: usize,
    pub snapshots: usize,
}
