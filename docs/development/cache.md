# Canonical retained Snapshot store

Implemented candidate: `uiblueprint_engine::cache`, under [D05@2 / D05-MEMORY@1](../specs/development/decisions/d05-memory.md). This replaces the earlier design proposal with the actual retained-owner API. It is not full K01, live freshness, decoder/replay/encoding peak enforcement or an RSS cap. [Receipt](../plans/ui-blueprint/receipts/K01-storage.md) records checks and checkpoint identities.

## Owners and explicit limits

A host constructs one `QuotaLedger` for its retained domain. `LedgerLimits` explicitly sets retained bytes, session slots and grants, at/below 64MiB /4 /4. The fixed inline grant table is charged even when vacant. `reserve(Allowance { bytes, session_slots })` returns a move-only `Grant`; a Store never implicitly receives the whole domain. Grant bytes plus ledger backing must fit before reservation succeeds.
`CacheStore::new(grant, limits, clock_domain, initial_ms)` consumes that grant. `StoreLimits` requires explicit session/entry slot counts, per-family revisions, entry/session byte caps and retention age, within D05's 4 sessions,16 slots/session,2 revisions/family,8MiB entry,16MiB session and300,000ms ceilings. Maxima are independent ceilings, not simultaneous-saturation promises. Actual fixed backing and checked arithmetic must fit; construction refusal returns the still-reserved grant after partial allocations/domain drop.
A Store owns fixed session slots; `open_session(SessionIdentity)` allocates that session's fixed revision slots. The header owns canonical session ID, target ID/generation and plugin ID/version. No `Arc`, persistent map/index, cloned Context key table, payload sharing, IO, worker process or timer exists here. Slot capacities never grow during admission. Scalar generation handles also bind the original ledger/Store; they are not serialized backend/action references.
Store payload/backing/domain fields drop before its final Grant field returns the allowance. `detach` frees one session's heaps but keeps vacant Store backing and the entire grant reservation. Dropping the Store releases all backing/payloads before its allowance becomes reusable. Borrowed reads prevent mutation/drop of their owner; the compile-fail test checks this Rust boundary.

## Actual API

| Operation | Result / ownership |
| --- | --- |
| `open_session(identity)` | SessionHandle; rejected header returns unchanged in `Rejected<SessionIdentity>` |
| `admit_full(session, Incoming { snapshot, partition }, now)` | Move admission with RevisionHandle, reused flag and retention-eviction count; rejection returns unchanged Snapshot/partition, including spare capacity |
| `lookup(session, SnapshotKey, policy, now)` | Borrowed StoredRead by full key; no owned copy |
| `read(revision_handle, policy, now)` | Borrow original Snapshot/partition plus separate invalidated flag and expiration; generations bind its original full key |
| `apply_delta(base_handle, partition, delta, result_id, now)` | Existing pure replay then staged admission; candidate/partition drop on failure, old base remains until successful commit |
| `invalidate(session, context)` | Mark matching canonical contexts, without restamping values/Observations or collecting |
| `invalidate_session(session)` | Mark every retained context of that exact session; other sessions and original records stay intact |
| `expire(session, now)` / `detach(session)` | Real ownership release counts; no UI deleted event |
| `usage()` / `session_usage(handle)` / `QuotaLedger::usage()` | Distinct actual owned-layout charges, capacities/counts and outstanding reservations |

`Incoming` remains caller/Completion-charged while staged. Successful new admission moves the original owned buffers; a successful exact duplicate drops its redundant input and reuses the existing handle without age refresh. Failed `admit_full` returns ownership, not an uncharged clone. The caller must reconcile its external charge at the return boundary; this API does not inspect or zero a transport's counters. `Rejected<T>` Debug prints only the error, never its payload.

## Key and admission contract

Family key: complete canonical Context (schema version, session ID, target and Surface IDs/generations, scope, projection, fields, plugin ID/version, environment revision) plus explicit channel partition. Lookup adds Snapshot ID/revision and binds the scalar handle generation. Canonical set fields use existing compatibility semantics; IDs/titles/coordinates or a hash alone cannot substitute for the full key.
Partition is a caller-owned nonempty unique **sorted** Vec of canonical Channels. Its backing capacity is counted, including slack; malformed order/duplicates return unchanged rather than being normalized through a hidden copy. ChannelResponse owners supply their explicit channel; combined/imported records require a declared partition. Missing/failed/history observations never infer requested channels.
Size checks, existing canonical validation, expiry/counter arithmetic and a fixed-size eviction plan precede any published mutation. Same-family ID or revision reused for a different record refuses; exact record reuse keeps original age and invalidation state. All admitted values/evidence/coverage remain canonical and unchanged.
Eviction first satisfies the family revision ceiling, then general slot/byte pressure, choosing oldest successful admission within the requesting session. Reads never update LRU. A failed plan leaves all entries/retained charges/admission generations intact; another session is never evicted to make room. A missing/expired/evicted/stale-generation base returns `ResyncRequired`, not invented UI removal.

## Capacity accounting

Canonical `schema::owned_size::{Heap, Overflow, HeapSize, owned}` is the single exhaustive safe field inventory, extracted from the original diagnostic. Store records add only their own header/slot/container metadata. All sums/products are checked; overflowing handles/admission counters refuse without wrapping. Public `CacheStore::layout()` reports the actual inline layouts for accounting verification.

| Allocation | Charged storage |
| --- | --- |
| Ledger | `size_of(QuotaLedger)` once, including fixed grant cells; no hidden heap |
| Store | `size_of(CacheStore)` + clock-domain String capacity + actual session-slot Vec capacity×slot layout |
| Session | Session root inline once for its own ceiling; header Id/Identity/Plugin string heaps and actual revision-slot capacity×slot layout |
| Occupied revision | Snapshot root is already inline in a revision slot. Add exhaustive Snapshot heap and partition backing only; entry ceiling separately includes Snapshot inline+heap+partition backing |
| Canonical descendants | Context/IDs/source_state, properties/values/children/extensions/declarations, Observations/evidence/coverage, relations, mappings, surface records, focus, captures/transforms and all nested String/Vec/Box capacities |
| Planning | Fixed16-element victim array and bounded scans; no retained index allocation. Canonical validators and pure replay still allocate separate working memory |

Store total is its root/backing plus each occupied session's heap charge; Session inline is already part of its Store slot and is not counted twice. Eviction/expiry release entry heaps, leaving reserved slot backing charged. Detach releases header/revision backing/payload heaps but leaves vacant Store slots charged. Ledger used bytes are ledger backing plus actual Store charges; reserved bytes are ledger backing plus grants, **not** used+grants.
The metric excludes allocator overhead/fragmentation/RSS, external thread/runtime/SDK/helper state, and buffers/files behind payload_ref. Captures remain metadata with their original Observation; the Store does not dereference or keep external pixel content alive. Returned/caller-cloned Snapshots and surviving Completions require their own owner charge; zero session encoded bytes is not proof of release.

## Time, invalidation and working-memory boundary

Every timed operation receives an explicit same-domain monotonic `Clock`; domain mismatch, regression and expiry arithmetic overflow reject. Successful admission fixes expiration once; age-boundary reads refuse. Host-owned expiry/detach scheduling releases expired storage without any UI polling. Valid clock samples are remembered even when a later read/quota decision refuses, preventing time rewind after an expired read; this guard does not alter entry age, admission counters or retained charge.
`ReadPolicy::Recorded` returns the immutable stored facts with cache disposition. `CurrentRequired` returns `RevalidationRequired`: this finite owner has no live revalidation API and does not treat original current flags or an unexpired TTL as current runtime evidence. Invalidated records remain available only as recorded data and cannot satisfy current-required reads. Full K01/live freshness remains a separate integration gate.
`invalidate_session` scans only the existing bounded revision slots of a validated session handle. Its count includes already-marked entries, matching context-only `invalidate`; flags are idempotent. It does not change expiration, timestamps, canonical bytes, charges, eviction order or grant ownership, and rejects foreign/retired handles before mutation. `CanonicalSession::invalidate_retained_session` exposes that same operation to the guarded Web worker after ObservationRun's borrow ends. The adapter retains/acknowledges its pending event signal only after this hook succeeds. This provider does not subscribe, collect or infer removal; conservative session-wide invalidation is used when an already-received event lacks a narrower trusted scope.
`apply_delta` is useful controlled-data replay integration, not peak-bounded replay. Base+Delta, candidate clones, observations and canonical validation temporaries can coexist before admission. Incoming parsing, validation/replay, output encoding, returned Completions and other host buffers are outside retained enforcement. No fake opaque permit, post-parse counter or multiplier is claimed to enforce those allocations; no subprocess policy is selected.

## Verification and remaining scope

Focused proof covers exact entry/session/Store/ledger byte boundaries, actual reserved capacity and vacant backing, grants across Stores, constructor rollback, count/sequence overflow, full-key/channel/generation isolation, unchanged rejected input and atomic failed eviction, scoped oldest/family eviction, duplicate age/conflict, clock boundary/regression/domain/overflow, real detach/drop lifecycle, unavailable/false/empty/redacted persistence and sensitive canary rejection.
Stored replay matches the independently authored full Snapshot at the same recorded checkpoint; lost bases resync. Geometry/replay tests remain passing. These author checks are not independent acceptance or live timing/memory evidence.
Separate derived relation/check caches, disk/raw-wire/pixel history, CLI ID resolution, host allocation enforcement and live adapters are not implemented by this finite slice. They remain original goal obligations. No schema models/validators, pure replay/geometry, CLI/export, shared Cargo or fixture/oracle data changed here.
