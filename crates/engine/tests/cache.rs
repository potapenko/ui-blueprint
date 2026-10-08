use std::mem::size_of;
use uiblueprint_engine::cache::*;
use uiblueprint_schema::{
    model::*,
    owned_size::{HeapSize, owned},
};
const MIB: usize = 1_048_576;
fn snapshot(revision: u64) -> Snapshot {
    let path = format!(
        "{}/../../fixtures/golden/ENV-SNAPSHOT-VALID.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let doc = Document::from_json(&std::fs::read(path).expect("fixture"), 100_000)
        .expect("canonical snapshot");
    let Artifact::Snapshot(mut s) = doc.artifact else {
        panic!("snapshot")
    };
    s.id = Id(format!("S{revision}"));
    s.revision = revision;
    s.source_state = Some(Id(format!("checkpoint-{revision}")));
    *s
}
fn delta_case() -> DeltaCase {
    let path = format!(
        "{}/../../fixtures/golden/ENV-DELTA-VALID.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let doc = Document::from_json(&std::fs::read(path).expect("fixture"), 100_000).expect("delta");
    let Artifact::Delta(case) = doc.artifact else {
        panic!("delta")
    };
    *case
}
fn limits() -> StoreLimits {
    StoreLimits {
        session_slots: 2,
        snapshot_slots_per_session: 4,
        revisions_per_family: 2,
        entry_bytes: MIB,
        session_bytes: 2 * MIB,
        retention_ms: 1000,
    }
}
fn ledger() -> QuotaLedger {
    QuotaLedger::new(LedgerLimits {
        retained_bytes: 64 * MIB,
        session_slots: 4,
        grants: 4,
    })
    .expect("ledger")
}
fn header(s: &Snapshot) -> SessionIdentity {
    SessionIdentity {
        session_id: s.context.session_id.clone(),
        target: s.context.target.clone(),
        plugin: s.context.plugin.clone(),
    }
}
fn clock(id: &Id, t: u64) -> Clock<'_> {
    Clock {
        domain: id,
        milliseconds: t,
    }
}
fn incoming(s: Snapshot) -> Incoming {
    Incoming {
        snapshot: s,
        partition: vec![Channel::ExternalSemantics],
    }
}
fn key<'a>(s: &'a Snapshot, p: &'a [Channel]) -> SnapshotKey<'a> {
    SnapshotKey {
        context: &s.context,
        partition: p,
        snapshot_id: &s.id,
        revision: s.revision,
    }
}
fn store(ledger: &QuotaLedger) -> CacheStore<'_> {
    CacheStore::new(
        ledger
            .reserve(Allowance {
                bytes: 8 * MIB,
                session_slots: 2,
            })
            .expect("grant"),
        limits(),
        Id("clock".into()),
        0,
    )
    .expect("store")
}
fn header_heap(h: &SessionIdentity) -> usize {
    h.session_id.0.capacity()
        + h.target.id.0.capacity()
        + h.target.generation.0.capacity()
        + h.plugin.id.0.capacity()
        + h.plugin.version.0.capacity()
}

#[test]
fn aggregate_grants_count_backing_reservations_and_actual_usage_separately() {
    let ledger = ledger();
    let initial = ledger.usage();
    assert_eq!(initial.used_bytes, QuotaLedger::backing_bytes());
    let first = store(&ledger);
    let second = store(&ledger);
    let usage = ledger.usage();
    assert_eq!(
        usage.reserved_bytes,
        QuotaLedger::backing_bytes() + 16 * MIB
    );
    assert_eq!(
        usage.used_bytes,
        QuotaLedger::backing_bytes() + first.usage().owned_bytes + second.usage().owned_bytes
    );
    assert_eq!(usage.session_slots, 4);
    assert!(matches!(
        ledger.reserve(Allowance {
            bytes: 1,
            session_slots: 1
        }),
        Err(CacheError::ResourceLimit)
    ));
    drop(first);
    assert_eq!(ledger.usage().session_slots, 2);
    assert_eq!(
        ledger.usage().reserved_bytes,
        QuotaLedger::backing_bytes() + 8 * MIB
    );
    drop(second);
    assert_eq!(ledger.usage(), initial);
    let exact = QuotaLedger::new(LedgerLimits {
        retained_bytes: QuotaLedger::backing_bytes() + 10,
        session_slots: 1,
        grants: 1,
    })
    .expect("exact ledger");
    assert!(matches!(
        exact.reserve(Allowance {
            bytes: 11,
            session_slots: 1
        }),
        Err(CacheError::ResourceLimit)
    ));
    let grant = exact
        .reserve(Allowance {
            bytes: 10,
            session_slots: 1,
        })
        .expect("exact grant");
    assert_eq!(
        exact.usage().reserved_bytes,
        QuotaLedger::backing_bytes() + 10
    );
    drop(grant);
}
#[test]
fn construction_failure_returns_live_grant_and_fixed_capacity_is_charged() {
    let ledger = ledger();
    let grant = ledger
        .reserve(Allowance {
            bytes: 1,
            session_slots: 4,
        })
        .expect("reserved");
    let refusal = CacheStore::new(grant, limits(), Id("clock".into()), 0)
        .err()
        .expect("backing cannot fit");
    assert_eq!(refusal.error, CacheError::ResourceLimit);
    assert_eq!(ledger.usage().session_slots, 4);
    assert_eq!(ledger.usage().used_bytes, QuotaLedger::backing_bytes());
    assert!(matches!(
        ledger.reserve(Allowance {
            bytes: 1,
            session_slots: 1
        }),
        Err(CacheError::ResourceLimit)
    ));
    drop(refusal);
    assert_eq!(ledger.usage().session_slots, 0);
    let domain = Id("clock".into());
    let layout = CacheStore::layout();
    let expected = layout.store + domain.0.capacity() + 2 * layout.session_slot;
    let mut cache = CacheStore::new(
        ledger
            .reserve(Allowance {
                bytes: 8 * MIB,
                session_slots: 2,
            })
            .expect("grant"),
        limits(),
        domain,
        0,
    )
    .expect("store");
    assert_eq!(cache.usage().owned_bytes, expected);
    assert_eq!(cache.usage().session_capacity, 2);
    let h = header(&snapshot(10));
    let session_heap = header_heap(&h) + 4 * layout.revision_slot;
    let handle = cache.open_session(h).expect("session");
    assert_eq!(
        cache.session_usage(handle).expect("usage").owned_bytes,
        layout.session + session_heap
    );
    assert_eq!(cache.usage().owned_bytes, expected + session_heap);
    cache.detach(handle).expect("detach");
    assert_eq!(cache.usage().owned_bytes, expected);
    assert_eq!(
        ledger.usage().reserved_bytes,
        QuotaLedger::backing_bytes() + 8 * MIB
    );
}
#[test]
fn profile_ceilings_and_inconsistent_constructors_refuse() {
    for bad in [
        LedgerLimits {
            retained_bytes: 64 * MIB + 1,
            session_slots: 4,
            grants: 4,
        },
        LedgerLimits {
            retained_bytes: 64 * MIB,
            session_slots: 5,
            grants: 4,
        },
        LedgerLimits {
            retained_bytes: 64 * MIB,
            session_slots: 4,
            grants: 5,
        },
        LedgerLimits {
            retained_bytes: 0,
            session_slots: 1,
            grants: 1,
        },
    ] {
        assert!(matches!(
            QuotaLedger::new(bad),
            Err(CacheError::InvalidLimits)
        ));
    }
    let ledger = ledger();
    for which in 0..7 {
        let mut l = limits();
        match which {
            0 => l.session_slots = 0,
            1 => l.snapshot_slots_per_session = 17,
            2 => l.revisions_per_family = 3,
            3 => l.entry_bytes = 8 * MIB + 1,
            4 => l.session_bytes = 16 * MIB + 1,
            5 => l.retention_ms = 300_001,
            _ => {
                l.entry_bytes = size_of::<Snapshot>();
                l.session_bytes = size_of::<Snapshot>();
            }
        }
        let grant = ledger
            .reserve(Allowance {
                bytes: 8 * MIB,
                session_slots: 2,
            })
            .expect("grant");
        let rejected = CacheStore::new(grant, l, Id("clock".into()), 0)
            .err()
            .expect("invalid limits");
        assert_eq!(rejected.error, CacheError::InvalidLimits);
        drop(rejected);
    }
}
#[test]
fn exact_entry_session_and_store_byte_boundaries_use_actual_capacities() {
    for below in [false, true] {
        let ledger = ledger();
        let mut input = incoming(snapshot(10));
        input.snapshot.nodes.reserve_exact(8);
        let entry = owned(&input.snapshot).expect("canonical size").2
            + input.partition.capacity() * size_of::<Channel>();
        let mut l = limits();
        l.entry_bytes = entry - usize::from(below);
        let mut cache = CacheStore::new(
            ledger
                .reserve(Allowance {
                    bytes: 8 * MIB,
                    session_slots: 2,
                })
                .expect("grant"),
            l,
            Id("clock".into()),
            0,
        )
        .expect("store");
        let h = cache
            .open_session(header(&input.snapshot))
            .expect("session");
        let prior = cache.usage();
        let cap = input.snapshot.nodes.capacity();
        let id = Id("clock".into());
        let result = cache.admit_full(h, input, clock(&id, 0));
        if below {
            let rejected = result.expect_err("over entry bound");
            assert_eq!(rejected.error, CacheError::ResourceLimit);
            assert_eq!(rejected.input.snapshot.nodes.capacity(), cap);
            assert_eq!(cache.usage(), prior);
        } else {
            let admitted = result.expect("exact bound");
            assert_eq!(
                cache.usage().owned_bytes - prior.owned_bytes,
                entry - size_of::<Snapshot>()
            );
            assert_eq!(
                cache
                    .read(admitted.handle, ReadPolicy::Recorded, clock(&id, 0))
                    .expect("stored")
                    .snapshot
                    .nodes
                    .capacity(),
                cap
            );
        }
    }
    for boundary in 0..3 {
        let ledger = ledger();
        let input = incoming(snapshot(10));
        let h = header(&input.snapshot);
        let layout = CacheStore::layout();
        let payload = input.snapshot.heap().expect("heap").bytes
            + input.partition.heap().expect("partition").bytes;
        let session_heap = header_heap(&h) + 4 * layout.revision_slot;
        let domain = Id("clock".into());
        let fixed = layout.store + domain.0.capacity() + 2 * layout.session_slot;
        let mut l = limits();
        l.entry_bytes = owned(&input.snapshot).expect("owned").2
            + input.partition.heap().expect("partition").bytes;
        l.session_bytes = layout.session + session_heap + payload - usize::from(boundary == 1);
        let allowance = fixed + session_heap + payload - usize::from(boundary == 2);
        let mut cache = CacheStore::new(
            ledger
                .reserve(Allowance {
                    bytes: allowance,
                    session_slots: 2,
                })
                .expect("grant"),
            l,
            domain,
            0,
        )
        .expect("store");
        let handle = cache.open_session(h).expect("fixed session fits");
        let before = cache.usage();
        let id = Id("clock".into());
        let result = cache.admit_full(handle, input, clock(&id, 0));
        if boundary == 0 {
            assert!(result.is_ok());
            assert_eq!(cache.usage().owned_bytes, allowance);
        } else {
            assert_eq!(
                result.expect_err("one byte over").error,
                CacheError::ResourceLimit
            );
            assert_eq!(cache.usage(), before);
        }
    }
}
#[test]
fn family_fifo_duplicate_age_and_conflicting_identity_are_explicit() {
    let ledger = ledger();
    let mut cache = store(&ledger);
    let id = Id("clock".into());
    let s = snapshot(10);
    let handle = cache.open_session(header(&s)).expect("session");
    let first = cache
        .admit_full(handle, incoming(s.clone()), clock(&id, 0))
        .expect("first");
    let second = cache
        .admit_full(handle, incoming(snapshot(11)), clock(&id, 1))
        .expect("second");
    let before = cache.usage();
    let duplicate = cache
        .admit_full(handle, incoming(s.clone()), clock(&id, 500))
        .expect("duplicate");
    assert!(duplicate.reused);
    assert_eq!(duplicate.handle, first.handle);
    assert_eq!(cache.usage(), before);
    assert_eq!(
        cache
            .read(first.handle, ReadPolicy::Recorded, clock(&id, 500))
            .expect("read")
            .expires_at_ms,
        1000
    );
    let mut conflict = s.clone();
    conflict.source_state = Some(Id("conflict".into()));
    assert_eq!(
        cache
            .admit_full(handle, incoming(conflict), clock(&id, 500))
            .expect_err("immutable identity")
            .error,
        CacheError::ConflictingIdentity
    );
    assert_eq!(cache.usage(), before);
    let third = cache
        .admit_full(handle, incoming(snapshot(12)), clock(&id, 500))
        .expect("third");
    assert_eq!(third.evicted_entries, 1);
    assert!(matches!(
        cache.read(first.handle, ReadPolicy::Recorded, clock(&id, 500)),
        Err(CacheError::ResyncRequired)
    ));
    assert!(
        cache
            .read(second.handle, ReadPolicy::Recorded, clock(&id, 500))
            .is_ok()
    );
}
#[test]
fn full_context_partition_and_owner_generations_prevent_aliases() {
    let ledger = ledger();
    let mut cache = store(&ledger);
    let mut other = store(&ledger);
    let id = Id("clock".into());
    let s = snapshot(10);
    let session = cache.open_session(header(&s)).expect("session");
    let other_session = other.open_session(header(&s)).expect("other store");
    let first = cache
        .admit_full(session, incoming(s.clone()), clock(&id, 0))
        .expect("first");
    let second = cache
        .admit_full(
            session,
            Incoming {
                snapshot: s.clone(),
                partition: vec![Channel::ExternalSemantics, Channel::RenderedCapture],
            },
            clock(&id, 0),
        )
        .expect("declared combined partition");
    assert_ne!(first.handle, second.handle);
    let elsewhere = other
        .admit_full(other_session, incoming(s.clone()), clock(&id, 0))
        .expect("same IDs isolated");
    assert!(matches!(
        cache.read(elsewhere.handle, ReadPolicy::Recorded, clock(&id, 0)),
        Err(CacheError::ResyncRequired)
    ));
    for dimension in 0..8 {
        let mut different = s.clone();
        match dimension {
            0 => different.context.session_id = Id("other".into()),
            1 => different.context.target.generation = Id("g2".into()),
            2 => different.context.surfaces[0].generation = Id("w2".into()),
            3 => different.context.scope_id = Id("other".into()),
            4 => different.context.projection = Projection::Design,
            5 => different.context.fields = vec![Field::Role],
            6 => different.context.plugin.version = Id("other".into()),
            _ => different.context.environment_revision = Id("other".into()),
        }
        assert!(matches!(
            cache.lookup(
                session,
                key(&different, &[Channel::ExternalSemantics]),
                ReadPolicy::Recorded,
                clock(&id, 0)
            ),
            Err(CacheError::ResyncRequired)
        ));
    }
    let mut reordered = s.clone();
    reordered.context.fields.reverse();
    assert!(
        cache
            .lookup(
                session,
                key(&reordered, &[Channel::ExternalSemantics]),
                ReadPolicy::Recorded,
                clock(&id, 0)
            )
            .is_ok()
    );
    cache.detach(session).expect("detach");
    let reopened = cache.open_session(header(&s)).expect("reopen slot");
    assert_ne!(session, reopened);
    cache
        .admit_full(reopened, incoming(s), clock(&id, 0))
        .expect("new attachment storage");
    assert!(matches!(
        cache.read(first.handle, ReadPolicy::Recorded, clock(&id, 0)),
        Err(CacheError::ResyncRequired)
    ));
}
#[test]
fn session_invalidation_marks_all_contexts_without_touching_history_or_other_sessions() {
    let ledger = ledger();
    let mut cache = store(&ledger);
    let id = Id("clock".into());
    let first = snapshot(10);
    let mut second = snapshot(11);
    second.context.scope_id = Id("another-scope".into());
    second.coverage.scope_id = second.context.scope_id.clone();
    for observation in &mut second.observations {
        observation.coverage.scope_id = second.context.scope_id.clone();
    }
    let mut outside = snapshot(12);
    outside.context.session_id = Id("other-session".into());
    let session = cache.open_session(header(&first)).unwrap();
    let other = cache.open_session(header(&outside)).unwrap();
    let a = cache
        .admit_full(session, incoming(first.clone()), clock(&id, 10))
        .unwrap();
    let b = cache
        .admit_full(session, incoming(second.clone()), clock(&id, 20))
        .unwrap();
    let c = cache
        .admit_full(other, incoming(outside.clone()), clock(&id, 30))
        .unwrap();
    let usage = cache.usage();
    let reservations = ledger.usage();
    let bytes = [&first, &second, &outside].map(|s| serde_json::to_vec(s).unwrap());
    // Context-only invalidation remains selective and compatible.
    assert_eq!(cache.invalidate(session, &first.context), Ok(1));
    assert!(
        cache
            .read(a.handle, ReadPolicy::Recorded, clock(&id, 40))
            .unwrap()
            .invalidated
    );
    assert!(
        !cache
            .read(b.handle, ReadPolicy::Recorded, clock(&id, 40))
            .unwrap()
            .invalidated
    );
    assert_eq!(cache.invalidate_session(session), Ok(2));
    assert_eq!(
        cache.invalidate_session(session),
        Ok(2),
        "idempotent flags, matching-entry count"
    );
    for (index, handle, source, invalidated, expires) in [
        (0, a.handle, &first, true, 1010),
        (1, b.handle, &second, true, 1020),
        (2, c.handle, &outside, false, 1030),
    ] {
        let recorded = cache
            .read(handle, ReadPolicy::Recorded, clock(&id, 40))
            .unwrap();
        assert_eq!(recorded.invalidated, invalidated);
        assert_eq!(recorded.snapshot, source);
        assert_eq!(serde_json::to_vec(recorded.snapshot).unwrap(), bytes[index]);
        assert_eq!(recorded.expires_at_ms, expires);
        assert!(matches!(
            cache.read(handle, ReadPolicy::CurrentRequired, clock(&id, 40)),
            Err(CacheError::RevalidationRequired)
        ));
    }
    assert_eq!(cache.usage(), usage, "no heap release/growth or eviction");
    assert_eq!(ledger.usage(), reservations, "grant and charge unchanged");
    let mut foreign = store(&ledger);
    let foreign_session = foreign.open_session(header(&first)).unwrap();
    assert!(matches!(
        cache.invalidate_session(foreign_session),
        Err(CacheError::InvalidHandle)
    ));
    assert!(
        !cache
            .read(c.handle, ReadPolicy::Recorded, clock(&id, 40))
            .unwrap()
            .invalidated
    );
    cache.detach(session).unwrap();
    assert!(matches!(
        cache.invalidate_session(session),
        Err(CacheError::InvalidHandle)
    ));
    assert_eq!(cache.invalidate_session(other), Ok(1));
    let empty = cache.open_session(header(&first)).unwrap();
    assert_eq!(cache.invalidate_session(empty), Ok(0));
}

#[test]
fn clock_expiry_and_invalidation_do_not_relabel_source_freshness() {
    let ledger = ledger();
    let mut cache = store(&ledger);
    let id = Id("clock".into());
    let s = snapshot(10);
    let source = s.clone();
    let session = cache.open_session(header(&s)).expect("session");
    let empty = cache.usage();
    let admitted = cache
        .admit_full(session, incoming(s), clock(&id, 10))
        .expect("entry");
    assert!(matches!(
        cache.read(admitted.handle, ReadPolicy::CurrentRequired, clock(&id, 10)),
        Err(CacheError::RevalidationRequired)
    ));
    assert_eq!(
        cache
            .invalidate(session, &source.context)
            .expect("invalidate"),
        1
    );
    let read = cache
        .read(admitted.handle, ReadPolicy::Recorded, clock(&id, 1009))
        .expect("recorded");
    assert!(read.invalidated);
    assert_eq!(read.snapshot, &source);
    assert!(matches!(
        cache.read(admitted.handle, ReadPolicy::Recorded, clock(&id, 1010)),
        Err(CacheError::ResyncRequired)
    ));
    assert!(matches!(
        cache.read(admitted.handle, ReadPolicy::Recorded, clock(&id, 1009)),
        Err(CacheError::InvalidClock)
    ));
    let wrong = Id("other-clock".into());
    assert!(matches!(
        cache.expire(session, clock(&wrong, 1010)),
        Err(CacheError::InvalidClock)
    ));
    let released = cache
        .expire(session, clock(&id, 1010))
        .expect("expiry cleanup");
    assert_eq!(released.snapshots, 1);
    assert_eq!(cache.usage(), empty);
    assert_eq!(
        ledger.usage().reserved_bytes,
        QuotaLedger::backing_bytes() + 8 * MIB
    );
    let before = cache.usage();
    assert_eq!(
        cache
            .admit_full(session, incoming(snapshot(11)), clock(&id, u64::MAX))
            .expect_err("expiry overflow")
            .error,
        CacheError::InvalidClock
    );
    assert_eq!(cache.usage(), before);
    assert!(
        cache
            .admit_full(session, incoming(snapshot(11)), clock(&id, 1010))
            .is_ok()
    );
}

#[test]
fn planned_eviction_rolls_back_and_never_steals_another_sessions_bytes() {
    let ledger = ledger();
    let a = snapshot(10);
    let mut b = snapshot(10);
    b.context.session_id = Id("a2".into());
    let expected_a = a.clone();
    let expected_b = b.clone();
    let a_header = header(&a);
    let b_header = header(&b);
    let layout = CacheStore::layout();
    let domain = Id("clock".into());
    let fixed = layout.store + domain.0.capacity() + 2 * layout.session_slot;
    let payload = |s: &Snapshot| s.heap().expect("heap").bytes + size_of::<Channel>();
    let allowance = fixed
        + header_heap(&a_header)
        + header_heap(&b_header)
        + 2 * layout.revision_slot
        + payload(&a)
        + payload(&b);
    let mut l = limits();
    l.snapshot_slots_per_session = 1;
    l.revisions_per_family = 1;
    let mut cache = CacheStore::new(
        ledger
            .reserve(Allowance {
                bytes: allowance,
                session_slots: 2,
            })
            .expect("grant"),
        l,
        domain,
        0,
    )
    .expect("store");
    let ah = cache.open_session(a_header).expect("a");
    let bh = cache.open_session(b_header).expect("b");
    let id = Id("clock".into());
    let ar = cache
        .admit_full(ah, incoming(a), clock(&id, 0))
        .expect("a entry");
    let br = cache
        .admit_full(bh, incoming(b), clock(&id, 0))
        .expect("b entry");
    assert_eq!(cache.usage().owned_bytes, allowance);
    let before = cache.usage();
    let mut too_large = snapshot(11);
    too_large.nodes.reserve_exact(64);
    let capacity = too_large.nodes.capacity();
    let rejection = cache
        .admit_full(ah, incoming(too_large), clock(&id, 1))
        .expect_err("own victims cannot make room");
    assert_eq!(rejection.error, CacheError::ResourceLimit);
    assert_eq!(rejection.input.snapshot.nodes.capacity(), capacity);
    assert_eq!(cache.usage(), before);
    assert_eq!(
        cache
            .read(ar.handle, ReadPolicy::Recorded, clock(&id, 1))
            .expect("a intact")
            .snapshot,
        &expected_a
    );
    assert_eq!(
        cache
            .read(br.handle, ReadPolicy::Recorded, clock(&id, 1))
            .expect("b intact")
            .snapshot,
        &expected_b
    );
    cache.detach(ah).expect("detach only a");
    assert_eq!(
        cache
            .read(br.handle, ReadPolicy::Recorded, clock(&id, 1))
            .expect("b survives")
            .snapshot,
        &expected_b
    );
}
#[test]
fn family_limit_is_satisfied_before_general_oldest_eviction() {
    let ledger = ledger();
    let mut l = limits();
    l.snapshot_slots_per_session = 3;
    let mut cache = CacheStore::new(
        ledger
            .reserve(Allowance {
                bytes: 8 * MIB,
                session_slots: 2,
            })
            .expect("grant"),
        l,
        Id("clock".into()),
        0,
    )
    .expect("store");
    let id = Id("clock".into());
    let session = cache.open_session(header(&snapshot(10))).expect("session");
    let oldest_other = cache
        .admit_full(
            session,
            Incoming {
                snapshot: snapshot(10),
                partition: vec![Channel::ExternalSemantics, Channel::RenderedCapture],
            },
            clock(&id, 0),
        )
        .expect("other family");
    let a1 = cache
        .admit_full(session, incoming(snapshot(10)), clock(&id, 1))
        .expect("a1");
    let a2 = cache
        .admit_full(session, incoming(snapshot(11)), clock(&id, 2))
        .expect("a2");
    let a3 = cache
        .admit_full(session, incoming(snapshot(12)), clock(&id, 3))
        .expect("a3");
    assert_eq!(a3.evicted_entries, 1);
    assert!(
        cache
            .read(oldest_other.handle, ReadPolicy::Recorded, clock(&id, 3))
            .is_ok()
    );
    assert!(
        cache
            .read(a2.handle, ReadPolicy::Recorded, clock(&id, 3))
            .is_ok()
    );
    assert!(matches!(
        cache.read(a1.handle, ReadPolicy::Recorded, clock(&id, 3)),
        Err(CacheError::ResyncRequired)
    ));
}
#[test]
fn replay_admission_matches_independent_full_state_and_refusals_preserve_base() {
    let case = delta_case();
    let ledger = ledger();
    let mut cache = store(&ledger);
    let id = Id("clock".into());
    let session = cache.open_session(header(&case.base)).expect("session");
    let base = cache
        .admit_full(session, incoming(case.base.clone()), clock(&id, 0))
        .expect("base");
    let before = cache.usage();
    let mut bad = case.update.clone();
    bad.base_revision -= 1;
    assert!(matches!(
        cache.apply_delta(
            base.handle,
            vec![Channel::ExternalSemantics],
            &bad,
            Id("S11".into()),
            clock(&id, 1)
        ),
        Err(CacheError::Replay(_))
    ));
    assert_eq!(cache.usage(), before);
    assert_eq!(
        cache
            .read(base.handle, ReadPolicy::Recorded, clock(&id, 1))
            .expect("base intact")
            .snapshot,
        &case.base
    );
    let expected = case.source_snapshot.expect("independent full oracle");
    let after = cache
        .apply_delta(
            base.handle,
            vec![Channel::ExternalSemantics],
            &case.update,
            expected.id.clone(),
            clock(&id, 2),
        )
        .expect("replay");
    assert_eq!(
        cache
            .read(after.handle, ReadPolicy::Recorded, clock(&id, 2))
            .expect("new state")
            .snapshot,
        &expected
    );
    cache
        .admit_full(session, incoming(snapshot(12)), clock(&id, 3))
        .expect("evict oldest");
    assert_eq!(
        cache
            .apply_delta(
                base.handle,
                vec![Channel::ExternalSemantics],
                &case.update,
                Id("new".into()),
                clock(&id, 4)
            )
            .expect_err("lost base"),
        CacheError::ResyncRequired
    );
}
#[test]
fn unavailable_false_empty_and_redacted_survive_and_sensitive_input_never_persists() {
    let ledger = ledger();
    let mut cache = store(&ledger);
    let id = Id("clock".into());
    let session = cache.open_session(header(&snapshot(10))).expect("session");
    let states = [
        Availability::Known {
            value: Value::Flag(false),
        },
        Availability::Unknown {
            reason: Id("read_failed".into()),
        },
        Availability::Unsupported {
            reason: Id("not_exposed".into()),
        },
        Availability::Redacted {},
    ];
    for (index, state) in states.into_iter().enumerate() {
        let mut s = snapshot(index as u64 + 10);
        for property in &mut s.nodes[0].properties {
            if let Property::Requested {
                field,
                state: current,
                ..
            } = property
            {
                if *field == Field::Checked {
                    *current = state.clone();
                }
                if *field == Field::Name {
                    *current = Availability::Known {
                        value: Value::Text(String::new()),
                    };
                }
            }
        }
        let expected = s.clone();
        let admitted = cache
            .admit_full(session, incoming(s), clock(&id, index as u64))
            .expect("availability preserved");
        assert_eq!(
            cache
                .read(
                    admitted.handle,
                    ReadPolicy::Recorded,
                    clock(&id, index as u64)
                )
                .expect("read")
                .snapshot,
            &expected
        );
    }
    let before = cache.usage();
    let mut secret = snapshot(20);
    secret.nodes[0].source_declarations.push(SourceDeclaration {
        namespace: Id("fixture.private".into()),
        name: Id("secret".into()),
        state: Availability::Known {
            value: Value::Text("CACHE_PRIVATE_CANARY".into()),
        },
        sensitivity: Sensitivity::Sensitive,
        source: Id("fixture".into()),
    });
    let rejected = cache
        .admit_full(session, incoming(secret), clock(&id, 4))
        .expect_err("redaction before cache");
    assert!(matches!(rejected.error, CacheError::InvalidSnapshot(_)));
    assert!(!format!("{rejected:?}").contains("CACHE_PRIVATE_CANARY"));
    assert_eq!(cache.usage(), before);
}
#[test]
fn partition_rejection_returns_unchanged_owned_capacity_and_store() {
    let ledger = ledger();
    let mut cache = store(&ledger);
    let id = Id("clock".into());
    let session = cache.open_session(header(&snapshot(10))).expect("session");
    for values in [
        vec![],
        vec![Channel::ExternalSemantics, Channel::ExternalSemantics],
        vec![Channel::RenderedCapture, Channel::ExternalSemantics],
    ] {
        let mut partition = Vec::with_capacity(32);
        partition.extend(values.clone());
        let before = cache.usage();
        let rejected = cache
            .admit_full(
                session,
                Incoming {
                    snapshot: snapshot(10),
                    partition,
                },
                clock(&id, 0),
            )
            .expect_err("invalid set");
        assert_eq!(rejected.error, CacheError::InvalidPartition);
        assert_eq!(rejected.input.partition, values);
        assert_eq!(rejected.input.partition.capacity(), 32);
        assert_eq!(cache.usage(), before);
    }
}

#[test]
fn independent_ceilings_do_not_promise_simultaneous_saturation() {
    let ledger = ledger();
    let mut l = limits();
    l.session_slots = 1;
    l.snapshot_slots_per_session = 1;
    l.revisions_per_family = 2;
    l.entry_bytes = 8 * MIB;
    l.session_bytes = 2 * MIB;
    let mut cache = CacheStore::new(
        ledger
            .reserve(Allowance {
                bytes: 4 * MIB,
                session_slots: 1,
            })
            .expect("grant"),
        l,
        Id("clock".into()),
        0,
    )
    .expect("independent caps");
    let id = Id("clock".into());
    let session = cache.open_session(header(&snapshot(10))).expect("session");
    let first = cache
        .admit_full(session, incoming(snapshot(10)), clock(&id, 0))
        .expect("small entry fits");
    let second = cache
        .admit_full(session, incoming(snapshot(11)), clock(&id, 1))
        .expect("physical slot cap dominates");
    assert_eq!(second.evicted_entries, 1);
    assert!(matches!(
        cache.read(first.handle, ReadPolicy::Recorded, clock(&id, 1)),
        Err(CacheError::ResyncRequired)
    ));
}

#[test]
fn partial_replay_keeps_unobserved_history_and_scope_mismatch_is_atomic() {
    let mut case = delta_case();
    // An omitted source node is still historical, even under a newer partial
    // revision. This authored oracle does not claim a fresh read of that node.
    case.update.upsert.clear();
    case.update.removed.clear();
    case.update.coverage.status = CoverageStatus::Partial;
    case.update.coverage.omitted_count = Some(1);
    let ledger = ledger();
    let mut cache = store(&ledger);
    let id = Id("clock".into());
    let session = cache.open_session(header(&case.base)).unwrap();
    let base = cache
        .admit_full(session, incoming(case.base.clone()), clock(&id, 0))
        .unwrap();
    cache.invalidate_session(session).unwrap();
    let before = cache.usage();
    let mut wrong = case.update.clone();
    wrong.context.scope_id = Id("another-scope".into());
    assert_eq!(
        cache
            .apply_delta(
                base.handle,
                vec![Channel::ExternalSemantics],
                &wrong,
                Id("S11".into()),
                clock(&id, 1)
            )
            .unwrap_err(),
        CacheError::Replay(uiblueprint_engine::replay::ReplayError::ResyncRequired(
            uiblueprint_engine::replay::ResyncReason::ContextMismatch
        ))
    );
    assert_eq!(cache.usage(), before);
    let after = cache
        .apply_delta(
            base.handle,
            vec![Channel::ExternalSemantics],
            &case.update,
            Id("S11".into()),
            clock(&id, 2),
        )
        .unwrap();
    let recorded = cache
        .read(after.handle, ReadPolicy::Recorded, clock(&id, 2))
        .unwrap();
    assert_eq!(recorded.snapshot.nodes, case.base.nodes);
    assert!(
        recorded
            .snapshot
            .observations
            .contains(&case.base.observations[0])
    );
    assert_eq!(recorded.snapshot.coverage.status, CoverageStatus::Partial);
    assert_eq!(recorded.snapshot.coverage.omitted_count, Some(1));
    assert!(matches!(
        cache.read(after.handle, ReadPolicy::CurrentRequired, clock(&id, 2)),
        Err(CacheError::RevalidationRequired)
    ));
    let original = cache
        .read(base.handle, ReadPolicy::Recorded, clock(&id, 2))
        .unwrap();
    assert_eq!(original.snapshot, &case.base);
    assert!(original.invalidated);
}

#[test]
fn controlled_full_delta_oracle_preserves_unavailable_and_empty_values() {
    for state in [
        Availability::Unknown {
            reason: Id("controlled-read-unavailable".into()),
        },
        Availability::Unsupported {
            reason: Id("controlled-not-exposed".into()),
        },
        Availability::Redacted {},
        Availability::Known {
            value: Value::Flag(false),
        },
    ] {
        let mut case = delta_case();
        let mut expected = case.source_snapshot.take().unwrap();
        // Two independently supplied checkpoint representations receive the same
        // authored facts. The expected full state is never produced by replay.
        for node in [&mut expected.nodes[0], &mut case.update.upsert[0]] {
            for property in &mut node.properties {
                if let Property::Requested {
                    field,
                    state: value,
                    ..
                } = property
                {
                    match field {
                        Field::Checked => *value = state.clone(),
                        Field::Name => {
                            *value = Availability::Known {
                                value: Value::Text(String::new()),
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        case.source_snapshot = Some(expected.clone());
        Document {
            schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
            artifact: Artifact::Delta(Box::new(case.clone())),
        }
        .validate()
        .unwrap();
        let ledger = ledger();
        let mut cache = store(&ledger);
        let id = Id("clock".into());
        let session = cache.open_session(header(&case.base)).unwrap();
        let base = cache
            .admit_full(session, incoming(case.base.clone()), clock(&id, 0))
            .unwrap();
        let after = cache
            .apply_delta(
                base.handle,
                vec![Channel::ExternalSemantics],
                &case.update,
                expected.id.clone(),
                clock(&id, 1),
            )
            .unwrap();
        let stored = cache
            .read(after.handle, ReadPolicy::Recorded, clock(&id, 1))
            .unwrap();
        assert_eq!(stored.snapshot, &expected);
        assert_eq!(
            serde_json::to_vec(stored.snapshot).unwrap(),
            serde_json::to_vec(&expected).unwrap()
        );
        assert_eq!(
            cache
                .read(base.handle, ReadPolicy::Recorded, clock(&id, 1))
                .unwrap()
                .snapshot,
            &case.base
        );
    }
}
