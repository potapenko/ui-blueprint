use super::{
    store::{Entry, Session, valid_id},
    *,
};
use std::mem::size_of;
use uiblueprint_schema::{model::*, validation};

struct Plan {
    victims: [bool; MAX_SLOTS],
    destination: usize,
    session_heap: usize,
    store_bytes: usize,
}
impl<'ledger> CacheStore<'ledger> {
    /// Admit already owned data, moving it without cloning after all preflight.
    /// Caller retains its external charge until this returns; rejection returns
    /// the exact input. A successful duplicate drops the redundant incoming value.
    /// # Errors
    /// Invalid data, quota, identity/time/count failures leave entries and retained
    /// counters unchanged. This is not a validator/decoder peak-allocation guard.
    #[allow(clippy::result_large_err)] // Preserve rejected Snapshot ownership without allocating another Box.
    pub fn admit_full(
        &mut self,
        session: SessionHandle<'_>,
        input: Incoming,
        now: Clock<'_>,
    ) -> Result<Admission<'ledger>, Rejected<Incoming>> {
        match self.prepare_admission(session, &input, now) {
            Ok(Prepared::Existing(handle)) => Ok(Admission {
                handle,
                reused: true,
                evicted_entries: 0,
            }),
            Ok(Prepared::Insert {
                plan,
                sequence,
                expires,
                heap_bytes,
            }) => {
                let Incoming {
                    snapshot,
                    partition,
                } = input;
                let evicted_entries = plan.victims.iter().filter(|v| **v).count();
                // prepare_admission proved this occupied session/slot; no public
                // callbacks or fallible work occurs during the commit.
                if let Some(current) = &mut self.sessions[session.slot].value {
                    for (index, slot) in current.slots.iter_mut().enumerate() {
                        if plan.victims[index] {
                            slot.value = None;
                        }
                    }
                    current.slots[plan.destination].value = Some(Entry {
                        snapshot,
                        partition,
                        heap_bytes,
                        admission: sequence,
                        expires,
                        invalidated: false,
                    });
                    current.heap_bytes = plan.session_heap;
                }
                self.used = plan.store_bytes;
                self.admission = sequence;
                self.grant.publish_usage(self.used);
                Ok(Admission {
                    handle: self.revision_handle(session.slot, plan.destination, sequence),
                    reused: false,
                    evicted_entries,
                })
            }
            Err(error) => Err(Rejected { error, input }),
        }
    }
    fn prepare_admission(
        &self,
        handle: SessionHandle<'_>,
        input: &Incoming,
        now: Clock<'_>,
    ) -> Result<Prepared<'ledger>, CacheError> {
        let expires = now
            .milliseconds
            .checked_add(self.limits.retention_ms)
            .ok_or(CacheError::InvalidClock)?;
        self.clock(now)?;
        let session = self.session(handle)?;
        if input.partition.is_empty() || input.partition.windows(2).any(|p| p[0] >= p[1]) {
            return Err(CacheError::InvalidPartition);
        }
        let snapshot = &input.snapshot;
        if !valid_id(&snapshot.id) {
            return Err(CacheError::InvalidIdentity);
        }
        if snapshot.context.session_id != session.identity.session_id
            || snapshot.context.target != session.identity.target
            || snapshot.context.plugin != session.identity.plugin
        {
            return Err(CacheError::OutsideSession);
        }
        let heap_bytes = add(heap(snapshot)?, heap(&input.partition)?)?;
        if add(size_of::<Snapshot>(), heap_bytes)? > self.limits.entry_bytes {
            return Err(CacheError::ResourceLimit);
        }
        validation::validate_snapshot(snapshot).map_err(CacheError::InvalidSnapshot)?;
        for (index, entry) in session
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.value.as_ref().map(|e| (i, e)))
        {
            if same_family(entry, input)
                && (entry.snapshot.id == snapshot.id
                    || entry.snapshot.revision == snapshot.revision)
            {
                if entry.snapshot != *snapshot {
                    return Err(CacheError::ConflictingIdentity);
                }
                if now.milliseconds >= entry.expires {
                    return Err(CacheError::ResyncRequired);
                }
                return Ok(Prepared::Existing(self.revision_handle(
                    handle.slot,
                    index,
                    entry.admission,
                )));
            }
        }
        let sequence = self
            .admission
            .checked_add(1)
            .ok_or(CacheError::CounterOverflow)?;
        let plan = self.plan(session, input, heap_bytes)?;
        Ok(Prepared::Insert {
            plan,
            sequence,
            expires,
            heap_bytes,
        })
    }
    fn plan(
        &self,
        session: &Session,
        input: &Incoming,
        heap_bytes: usize,
    ) -> Result<Plan, CacheError> {
        let mut victims = [false; MAX_SLOTS];
        while session
            .slots
            .iter()
            .enumerate()
            .filter(|(i, s)| {
                !victims[*i] && s.value.as_ref().is_some_and(|e| same_family(e, input))
            })
            .count()
            >= self.limits.revisions_per_family
        {
            let oldest = oldest(session, &victims, |e| same_family(e, input))
                .ok_or(CacheError::ResourceLimit)?;
            victims[oldest] = true;
        }
        loop {
            let released: usize = session
                .slots
                .iter()
                .enumerate()
                .filter_map(|(i, s)| s.value.as_ref().filter(|_| victims[i]))
                .map(|e| e.heap_bytes)
                .sum();
            let session_heap = add(session.heap_bytes - released, heap_bytes)?;
            let store_bytes = add(self.used - released, heap_bytes)?;
            let destination = session
                .slots
                .iter()
                .enumerate()
                .position(|(i, s)| s.value.is_none() || victims[i]);
            if add(size_of::<Session>(), session_heap)? <= self.limits.session_bytes
                && store_bytes <= self.grant.allowance().bytes
                && let Some(destination) = destination
            {
                return Ok(Plan {
                    victims,
                    destination,
                    session_heap,
                    store_bytes,
                });
            }
            let victim = oldest(session, &victims, |_| true).ok_or(CacheError::ResourceLimit)?;
            victims[victim] = true;
        }
    }
    /// Controlled-data replay integration. The base remains stored until a fully
    /// validated candidate can be admitted. Candidate, maps and validation allocate
    /// working memory OUTSIDE retained quotas; no peak-bound/live claim follows.
    /// Caller supplies/owns the partition; on any refusal this method drops that
    /// partition and its candidate before returning the payload-free error.
    pub fn apply_delta(
        &mut self,
        base: RevisionHandle<'_>,
        partition: Vec<Channel>,
        delta: &Delta,
        result_id: Id,
        now: Clock<'_>,
    ) -> Result<Admission<'ledger>, CacheError> {
        let prior = self.read(base, ReadPolicy::Recorded, now)?;
        if prior.partition != partition {
            return Err(CacheError::InvalidPartition);
        }
        let snapshot =
            crate::replay::replay(prior.snapshot, delta, result_id).map_err(CacheError::Replay)?;
        self.admit_full(
            base.session,
            Incoming {
                snapshot,
                partition,
            },
            now,
        )
        .map_err(|rejected| rejected.error)
    }
}
enum Prepared<'a> {
    Existing(RevisionHandle<'a>),
    Insert {
        plan: Plan,
        sequence: u64,
        expires: u64,
        heap_bytes: usize,
    },
}
fn same_family(entry: &Entry, incoming: &Incoming) -> bool {
    entry.partition == incoming.partition
        && validation::contexts_compatible(&entry.snapshot.context, &incoming.snapshot.context)
}
fn oldest(
    session: &Session,
    victims: &[bool; MAX_SLOTS],
    eligible: impl Fn(&Entry) -> bool,
) -> Option<usize> {
    session
        .slots
        .iter()
        .enumerate()
        .filter(|(i, _)| !victims[*i])
        .filter_map(|(i, s)| {
            s.value
                .as_ref()
                .filter(|e| eligible(e))
                .map(|e| (i, e.admission))
        })
        .min_by_key(|(_, sequence)| *sequence)
        .map(|(i, _)| i)
}
