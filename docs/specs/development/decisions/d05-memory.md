# D05 retained memory and admission

- Node type: leaf; domain: `uib.development.d05-memory`.
- Contract: `UIB.D05-MEMORY@1`; stable clauses use the `UIB.D05-MEMORY.` prefix below.
- Authority: Active / Stability: Evolving; accepted/released implementation: none.
- Authority source: ROADMAP.CONTENT delegated D05 decisions under approved PLAN.UIB@1; [finite packet](../../../plans/ui-blueprint/packets/D05-policy.md).
- Read when: implementing/reviewing K01 storage or retained admission/cleanup.
- Do not read when: an unaffected geometric/visual rule already has its basis.
- Requires: [CACHE@1](../../product/cache.md), [LIFECYCLE@1](../../product/lifecycle.md), [EXCHANGE@1](../../product/exchange.md), [evidence](evidence.md).
- Supporting design: [Core ownership handoff](../../../development/cache.md); only choices stated here are adopted.

## UIB.D05-MEMORY.METRIC — owned layout, not RSS
Charge each owner root inline once, actual String/Vec capacities, initialized nested
heaps, Box layouts, keys, slot backing and metadata. Checked arithmetic rejects
overflow. Vacant fixed slots remain charged; Snapshot inline inside a slot is not
added again. No hidden indexes, owned key copies or shared payloads in the first slice.
Exclude allocator headers/rounding/fragmentation, stack space beyond inline records,
OS/SDK/helper allocations and referenced pixel/file contents; these need their own
owners. This is retained owned-layout accounting, not a process RSS or peak cap.
Canonical `schema::owned_size::{Heap, Overflow, HeapSize, owned}` reuses the exhaustive
safe walker; no second DTO field inventory. `owned` reports inline/heap/total;
engine adds its own exhaustive slot/header/container charges. No wire/validator change.

## UIB.D05-MEMORY.LIMITS — explicit initial profile, no implicit defaults
| Limit | Maximum chosen before implementation |
| --- | ---: |
| Host quota domains / granted session slots | One aggregate domain /4 session slots total |
| Outstanding Store allowance grants |4 |
| Snapshot slots per session / revisions per full-key family |16 /2 |
| Occupied entry: Snapshot inline+heap+partition backing |8MiB |
| Session retained charge / host aggregate retained domain |16MiB /64MiB |
| Retention age from successful admission |300,000 monotonic milliseconds |

MiB=1,048,576 bytes. Every constructor takes explicit positive limits at or below
this profile; inconsistent fixed-backing/byte/count budgets refuse before publication.
Actual fixed capacity is verified and charged, including construction rollback.
Maxima are independent ceilings, not a promise all can be saturated together.
Evidence: [measured examples](../../../../tests/bridges/resources/working-memory.md)
include ~0.10/0.27MiB actual owned Documents and a ~4MiB synthetic dense payload.
8MiB admits those examples;16MiB permits two such synthetic revisions plus ordinary
metadata;64MiB supports multiple small independent sessions. These are engineering
ceilings with refusal, not inferred worst-case multipliers. Two revisions serve
base/current comparison;16 slots bound multiple families; five minutes bounds idle
retention between explicit work. Reads never extend age. Higher limits need revision.

## UIB.D05-MEMORY.OWNERSHIP — one aggregate grant owner
Host owns one ledger with fixed grant slots, charged ledger backing and checked
totals. Before Store creation it reserves a move-only allowance containing retained
bytes and session slots; sum of grants plus host retained-ledger charge must fit
the domain. Stores never each receive the entire domain implicitly. No process
packaging is selected. Store root/backing/session/entry charges fit its allowance.
Creation failure returns its grant only after partial allocations drop. Closing a
Store destroys all entries/backing before returning the allowance; detached session
usage falls locally but its fixed Store allowance stays reserved until Store close.
Reservation is not actual usage: report both, without counting the same payload twice.
Caller/Completion owns and charges incoming Snapshot and partition until commit;
successful admission moves ownership/charge to Store atomically, without cloning.
Rejection returns the unchanged charged input or explicitly drops it. An encoded
counter reaching zero is never permission to release a live Completion's charge.
Expose borrowed reads only; explicit caller clones require a separate caller charge.

## UIB.D05-MEMORY.ADMISSION — full context and staged commit
Family key is complete canonical Context plus explicit channel partition; lookup
adds Snapshot ID/revision and scalar handle generation. Compare canonical set fields
by existing compatibility semantics, never titles/coordinates or hash alone.
Partition is a nonempty unique sorted set of canonical Channels, owned/countable;
ChannelResponse uses its explicit channel. Combined/imported data requires a declared
partition from its owner; unavailable channels/old observations do not infer it.
Fixed slots and bounded scans replace hidden indexes. Size-check, canonical validation
and candidate/eviction planning complete before any mutation. Same full key and exact
record may reuse its handle without refreshing age; conflicting identity refuses.
Evict oldest-admitted eligible records only in the requesting session, satisfying
family count first; reads do not update LRU. Never evict another session for room.
Failure leaves entries/counters intact; lost/expired/evicted bases give resync_required,
not UI deletion. Handle/admission-counter overflow refuses without wrapping.

## UIB.D05-MEMORY.LIFETIME / UIB.D05-MEMORY.GATES

Explicit same-domain monotonic time determines expiry; overflow/regression/domain
mismatch rejects. At age boundary reads refuse; invalidation cannot satisfy current-
required reads or restamp Observation. Host schedules owned expiry/detach cleanup,
not UI polling. Drop entry heaps on expiry/eviction; drop session heaps on detach;
keep vacant Store backing charged. Borrowed reads cannot coexist with mutation.
Derived relations/checks have separate future owners within the same quotas; first
slice stores Snapshots only and does not complete K01 or invent derived-cache APIs.
Decode, canonical validation, replay, planning/encoding and caller Completions need
separate working/ownership reservations and actual enforcement. A permit alone is
not proof. Controlled-data retained storage may proceed; live acceptance and peak-
bounded replay integration wait for that host enforcement. No subprocess is selected.
Checks: exact byte/count/overflow and fixed capacity; charged transfer/rejection;
full-key/channel/session isolation; scoped eviction/resync; monotonic expiry,
invalidation and detach; known/unknown/redacted/empty persistence; real cleanup.

## Change record

`D05-RET-002`: D05@1 unresolved retained policy -> D05@2 + D05-MEMORY@1 under
ROADMAP delegation, packet56dc93b and Core design0d40af7. No wire/released change;
other CONTENT norms, D02, five review repairs and full live D05 gate stay protected.
