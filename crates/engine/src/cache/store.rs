use super::*;
use std::{cell::Cell, mem::size_of, ptr};
use uiblueprint_schema::{model::*, validation};

#[derive(Default)]
pub(super) struct SessionSlot {
    pub generation: u64,
    pub value: Option<Session>,
}
#[derive(Default)]
pub(super) struct RevisionSlot {
    pub value: Option<Entry>,
}
pub(super) struct Session {
    pub identity: SessionIdentity,
    pub slots: Vec<RevisionSlot>,
    pub heap_bytes: usize,
}
pub(super) struct Entry {
    pub snapshot: Snapshot,
    pub partition: Vec<Channel>,
    pub heap_bytes: usize,
    pub admission: u64,
    pub expires: u64,
    pub invalidated: bool,
}
/// Actual inline layouts used by the explicit capacity accounting formula.
#[derive(Clone, Copy, Debug)]
pub struct StoreLayout {
    pub store: usize,
    pub session_slot: usize,
    pub session: usize,
    pub revision_slot: usize,
}
/// Fixed storage and canonical payload owner. No shared payloads or hidden index.
/// Field order is deliberate: backing/payloads/domain drop before the Grant,
/// which is the final field and only then releases the host allowance.
///
/// A borrowed read prevents mutation of the same real owner:
/// ```compile_fail
/// use uiblueprint_engine::cache::*;
/// fn cannot_detach_while_reading(store: &mut CacheStore<'_>, session: SessionHandle<'_>,
///                              revision: RevisionHandle<'_>, now: Clock<'_>) {
///     let view = store.read(revision, ReadPolicy::Recorded, now).unwrap();
///     store.detach(session).unwrap();
///     assert!(!view.snapshot.nodes.is_empty());
/// }
/// ```
pub struct CacheStore<'ledger> {
    pub(super) sessions: Vec<SessionSlot>,
    domain: Id,
    last_clock: Cell<u64>,
    pub(super) limits: StoreLimits,
    pub(super) used: usize,
    generation: u64,
    pub(super) admission: u64,
    pub(super) grant: Grant<'ledger>,
}
impl<'ledger> CacheStore<'ledger> {
    pub const fn layout() -> StoreLayout {
        StoreLayout {
            store: size_of::<Self>(),
            session_slot: size_of::<SessionSlot>(),
            session: size_of::<Session>(),
            revision_slot: size_of::<RevisionSlot>(),
        }
    }
    /// # Errors
    /// Rejects limits outside D05, insufficient actual fixed backing or allocation
    /// failure. Partial allocations/domain drop before the move-only grant returns.
    pub fn new(
        grant: Grant<'ledger>,
        limits: StoreLimits,
        domain: Id,
        now_ms: u64,
    ) -> Result<Self, Rejected<Grant<'ledger>>> {
        let prepared = (|| {
            validate_limits(limits, grant.allowance())?;
            if !valid_id(&domain) {
                return Err(CacheError::InvalidClock);
            }
            let fixed = add(size_of::<Self>(), heap(&domain)?)?;
            if add(fixed, backing::<SessionSlot>(limits.session_slots)?)? > grant.allowance().bytes
            {
                return Err(CacheError::ResourceLimit);
            }
            let sessions = slots::<SessionSlot>(limits.session_slots)?;
            let used = add(fixed, backing::<SessionSlot>(sessions.capacity())?)?;
            if used > grant.allowance().bytes {
                return Err(CacheError::ResourceLimit);
            }
            Ok((sessions, used))
        })();
        match prepared {
            Ok((sessions, used)) => {
                grant.publish_usage(used);
                Ok(Self {
                    sessions,
                    domain,
                    last_clock: Cell::new(now_ms),
                    limits,
                    used,
                    generation: 0,
                    admission: 0,
                    grant,
                })
            }
            Err(error) => {
                drop(domain);
                Err(Rejected {
                    error,
                    input: grant,
                })
            }
        }
    }
    pub fn usage(&self) -> StoreUsage {
        StoreUsage {
            owned_bytes: self.used,
            allowance_bytes: self.grant.allowance().bytes,
            session_capacity: self.sessions.capacity(),
            sessions: self.sessions.iter().filter(|s| s.value.is_some()).count(),
            snapshots: self
                .sessions
                .iter()
                .filter_map(|s| s.value.as_ref())
                .map(|s| s.slots.iter().filter(|r| r.value.is_some()).count())
                .sum(),
        }
    }
    pub fn session_usage(&self, handle: SessionHandle<'_>) -> Result<SessionUsage, CacheError> {
        let session = self.session(handle)?;
        Ok(SessionUsage {
            owned_bytes: size_of::<Session>() + session.heap_bytes,
            snapshot_capacity: session.slots.capacity(),
            snapshots: session.slots.iter().filter(|s| s.value.is_some()).count(),
        })
    }
    /// # Errors
    /// Refuses invalid/conflicting identity, slot/byte quota or allocation failure.
    /// The original caller-owned header is returned on rejection.
    #[allow(clippy::result_large_err)] // Return ownership without allocating an error Box.
    pub fn open_session(
        &mut self,
        identity: SessionIdentity,
    ) -> Result<SessionHandle<'ledger>, Rejected<SessionIdentity>> {
        let prepared = (|| {
            if !valid_id(&identity.session_id)
                || !valid_id(&identity.target.id)
                || !valid_id(&identity.target.generation)
                || !valid_id(&identity.plugin.id)
                || !valid_id(&identity.plugin.version)
            {
                return Err(CacheError::InvalidIdentity);
            }
            if self
                .sessions
                .iter()
                .filter_map(|s| s.value.as_ref())
                .any(|s| s.identity.session_id == identity.session_id)
            {
                return Err(CacheError::ConflictingIdentity);
            }
            let slot = self
                .sessions
                .iter()
                .position(|s| s.value.is_none())
                .ok_or(CacheError::ResourceLimit)?;
            let generation = self
                .generation
                .checked_add(1)
                .ok_or(CacheError::CounterOverflow)?;
            let header = add(
                add(heap(&identity.session_id)?, heap(&identity.target)?)?,
                heap(&identity.plugin)?,
            )?;
            let minimum = add(
                header,
                backing::<RevisionSlot>(self.limits.snapshot_slots_per_session)?,
            )?;
            if add(size_of::<Session>(), minimum)? > self.limits.session_bytes
                || add(self.used, minimum)? > self.grant.allowance().bytes
            {
                return Err(CacheError::ResourceLimit);
            }
            let slots = slots::<RevisionSlot>(self.limits.snapshot_slots_per_session)?;
            let heap_bytes = add(header, backing::<RevisionSlot>(slots.capacity())?)?;
            let used = add(self.used, heap_bytes)?;
            if add(size_of::<Session>(), heap_bytes)? > self.limits.session_bytes
                || used > self.grant.allowance().bytes
            {
                return Err(CacheError::ResourceLimit);
            }
            Ok((slot, generation, slots, heap_bytes, used))
        })();
        match prepared {
            Ok((slot, generation, slots, heap_bytes, used)) => {
                self.sessions[slot] = SessionSlot {
                    generation,
                    value: Some(Session {
                        identity,
                        slots,
                        heap_bytes,
                    }),
                };
                self.generation = generation;
                self.used = used;
                self.grant.publish_usage(used);
                Ok(self.session_handle(slot))
            }
            Err(error) => Err(Rejected {
                error,
                input: identity,
            }),
        }
    }
    /// Borrow recorded data by the complete Context/partition/ID/revision key.
    /// # Errors
    /// Missing/expired entries require resync. CurrentRequired always requires an
    /// external revalidation owner, not a TTL-based promotion of saved evidence.
    pub fn lookup(
        &self,
        session: SessionHandle<'_>,
        key: SnapshotKey<'_>,
        policy: ReadPolicy,
        now: Clock<'_>,
    ) -> Result<StoredRead<'_, 'ledger>, CacheError> {
        self.clock(now)?;
        let current = self
            .session(session)
            .map_err(|_| CacheError::ResyncRequired)?;
        let slot = current
            .slots
            .iter()
            .position(|slot| {
                slot.value.as_ref().is_some_and(|e| {
                    e.snapshot.id == *key.snapshot_id
                        && e.snapshot.revision == key.revision
                        && e.partition == key.partition
                        && validation::contexts_compatible(&e.snapshot.context, key.context)
                })
            })
            .ok_or(CacheError::ResyncRequired)?;
        let entry = current.slots[slot]
            .value
            .as_ref()
            .ok_or(CacheError::ResyncRequired)?;
        self.read(
            self.revision_handle(session.slot, slot, entry.admission),
            policy,
            now,
        )
    }
    /// Handle binds the original full key and both owner/session generations.
    /// Returned borrows cannot coexist with mutation/eviction of this Store.
    /// # Errors
    /// Wrong/retired handle or expiry => resync; current UI evidence => revalidation.
    pub fn read(
        &self,
        handle: RevisionHandle<'_>,
        policy: ReadPolicy,
        now: Clock<'_>,
    ) -> Result<StoredRead<'_, 'ledger>, CacheError> {
        self.clock(now)?;
        let session = self
            .session(handle.session)
            .map_err(|_| CacheError::ResyncRequired)?;
        let entry = session
            .slots
            .get(handle.slot)
            .and_then(|s| s.value.as_ref())
            .filter(|e| e.admission == handle.generation)
            .ok_or(CacheError::ResyncRequired)?;
        if now.milliseconds >= entry.expires {
            return Err(CacheError::ResyncRequired);
        }
        if policy == ReadPolicy::CurrentRequired {
            return Err(CacheError::RevalidationRequired);
        }
        Ok(StoredRead {
            handle: self.revision_handle(handle.session.slot, handle.slot, entry.admission),
            snapshot: &entry.snapshot,
            partition: &entry.partition,
            invalidated: entry.invalidated,
            expires_at_ms: entry.expires,
        })
    }
    /// Marks matching stored contexts without altering Observations or collecting.
    pub fn invalidate(
        &mut self,
        handle: SessionHandle<'_>,
        context: &Context,
    ) -> Result<usize, CacheError> {
        self.session(handle)?;
        let mut changed = 0;
        // session() validated this occupied slot; no callbacks occur before use.
        if let Some(session) = &mut self.sessions[handle.slot].value {
            for entry in session.slots.iter_mut().filter_map(|s| s.value.as_mut()) {
                if validation::contexts_compatible(&entry.snapshot.context, context) {
                    entry.invalidated = true;
                    changed += 1;
                }
            }
        }
        Ok(changed)
    }
    /// Marks every retained context in this session without collecting, freeing,
    /// refreshing or modifying canonical records. Returns the number of matching
    /// entries, including those already marked, as context-only invalidate does.
    /// # Errors
    /// Rejects a foreign or retired session handle before changing any entry.
    pub fn invalidate_session(&mut self, handle: SessionHandle<'_>) -> Result<usize, CacheError> {
        self.session(handle)?;
        let mut matched = 0;
        if let Some(session) = &mut self.sessions[handle.slot].value {
            for entry in session
                .slots
                .iter_mut()
                .filter_map(|slot| slot.value.as_mut())
            {
                entry.invalidated = true;
                matched += 1;
            }
        }
        Ok(matched)
    }
    /// Drops expired entry heaps; vacant slot backing remains charged.
    pub fn expire(
        &mut self,
        handle: SessionHandle<'_>,
        now: Clock<'_>,
    ) -> Result<ReleasedCharge, CacheError> {
        self.clock(now)?;
        self.session(handle)?;
        let mut released = ReleasedCharge {
            owned_bytes: 0,
            snapshots: 0,
        };
        if let Some(session) = &mut self.sessions[handle.slot].value {
            for slot in &mut session.slots {
                if slot
                    .value
                    .as_ref()
                    .is_some_and(|e| now.milliseconds >= e.expires)
                    && let Some(entry) = slot.value.take()
                {
                    released.owned_bytes += entry.heap_bytes;
                    released.snapshots += 1;
                    drop(entry);
                }
            }
            session.heap_bytes -= released.owned_bytes;
        }
        self.used -= released.owned_bytes;
        self.grant.publish_usage(self.used);
        Ok(released)
    }
    /// Drops a session's real heap ownership; Store backing and grant remain.
    pub fn detach(&mut self, handle: SessionHandle<'_>) -> Result<ReleasedCharge, CacheError> {
        let session = self.session(handle)?;
        let released = ReleasedCharge {
            owned_bytes: session.heap_bytes,
            snapshots: session.slots.iter().filter(|s| s.value.is_some()).count(),
        };
        drop(self.sessions[handle.slot].value.take());
        self.used -= released.owned_bytes;
        self.grant.publish_usage(self.used);
        Ok(released)
    }
    pub(super) fn session(&self, handle: SessionHandle<'_>) -> Result<&Session, CacheError> {
        if !ptr::eq(handle.domain, self.grant.ledger) || handle.store != self.grant.serial {
            return Err(CacheError::InvalidHandle);
        }
        self.sessions
            .get(handle.slot)
            .filter(|s| s.generation == handle.generation)
            .and_then(|s| s.value.as_ref())
            .ok_or(CacheError::InvalidHandle)
    }
    pub(super) fn session_handle(&self, slot: usize) -> SessionHandle<'ledger> {
        SessionHandle {
            domain: self.grant.ledger,
            store: self.grant.serial,
            slot,
            generation: self.sessions[slot].generation,
        }
    }
    pub(super) fn revision_handle(
        &self,
        session: usize,
        slot: usize,
        generation: u64,
    ) -> RevisionHandle<'ledger> {
        RevisionHandle {
            session: self.session_handle(session),
            slot,
            generation,
        }
    }
    pub(super) fn clock(&self, now: Clock<'_>) -> Result<(), CacheError> {
        if now.domain != &self.domain || now.milliseconds < self.last_clock.get() {
            return Err(CacheError::InvalidClock);
        }
        // Clock discipline remembers every valid sample, including failed reads;
        // a caller cannot resurrect an expired entry by rewinding after refusal.
        self.last_clock.set(now.milliseconds);
        Ok(())
    }
}
fn validate_limits(l: StoreLimits, a: Allowance) -> Result<(), CacheError> {
    if !(1..=MAX_SESSIONS).contains(&l.session_slots)
        || l.session_slots > a.session_slots
        || !(1..=MAX_SLOTS).contains(&l.snapshot_slots_per_session)
        || !(1..=2).contains(&l.revisions_per_family)
        || !(1..=8 * MIB).contains(&l.entry_bytes)
        || !(1..=16 * MIB).contains(&l.session_bytes)
        || !(1..=300_000).contains(&l.retention_ms)
    {
        return Err(CacheError::InvalidLimits);
    }
    if l.entry_bytes < size_of::<Snapshot>()
        || add(
            size_of::<Session>(),
            backing::<RevisionSlot>(l.snapshot_slots_per_session)?,
        )? > l.session_bytes
    {
        return Err(CacheError::InvalidLimits);
    }
    Ok(())
}
pub(super) fn valid_id(id: &Id) -> bool {
    !id.0.is_empty() && id.0.chars().count() <= 256
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn session_and_admission_generations_refuse_overflow_without_publication() {
        let ledger = QuotaLedger::new(LedgerLimits {
            retained_bytes: 8 * MIB,
            session_slots: 1,
            grants: 1,
        })
        .expect("ledger");
        let grant = ledger
            .reserve(Allowance {
                bytes: 4 * MIB,
                session_slots: 1,
            })
            .expect("grant");
        let limits = StoreLimits {
            session_slots: 1,
            snapshot_slots_per_session: 1,
            revisions_per_family: 1,
            entry_bytes: MIB,
            session_bytes: 2 * MIB,
            retention_ms: 1,
        };
        let mut store = CacheStore::new(grant, limits, Id("clock".into()), 0).expect("store");
        let text = include_str!("../../../../fixtures/golden/ENV-SNAPSHOT-VALID.json");
        let doc = Document::from_json(text.as_bytes(), 100_000).expect("fixture");
        let Artifact::Snapshot(snapshot) = doc.artifact else {
            panic!("snapshot")
        };
        let header = SessionIdentity {
            session_id: snapshot.context.session_id.clone(),
            target: snapshot.context.target.clone(),
            plugin: snapshot.context.plugin.clone(),
        };
        store.generation = u64::MAX;
        let before = store.usage();
        let failure = store.open_session(header).expect_err("no generation wrap");
        assert_eq!(failure.error, CacheError::CounterOverflow);
        assert_eq!(store.usage(), before);
        store.generation = 0;
        let session = store.open_session(failure.input).expect("open");
        store.admission = u64::MAX;
        let before = store.usage();
        let clock_id = Id("clock".into());
        let failure = store
            .admit_full(
                session,
                Incoming {
                    snapshot: *snapshot,
                    partition: vec![Channel::ExternalSemantics],
                },
                Clock {
                    domain: &clock_id,
                    milliseconds: 0,
                },
            )
            .expect_err("no admission wrap");
        assert_eq!(failure.error, CacheError::CounterOverflow);
        assert_eq!(store.usage(), before);
    }
}
