# D05 retained memory and admission

- Node type: leaf; domain: `uib.development.d05-memory`; contract: `UIB.D05-MEMORY@2`; supersedes @1.
- Stable clauses: `UIB.D05-MEMORY.METRIC`, `.LIMITS`, `.OWNERSHIP`, `.ADMISSION`, `.LIFETIME`, `.GATES`.
- Authority: Active / Evolving; accepted/released full implementation: none.
- Authority source: ROADMAP/PLAN.UIB@1, D05-RET-002 and D05-PARTITION-001 below.
- Read when: retained storage/host admission/cleanup; not for an unchanged geometric rule.
- Requires: [CACHE@1](../../product/cache.md), [LIFECYCLE@1](../../product/lifecycle.md), [EXCHANGE@1](../../product/exchange.md), [evidence](evidence.md).
- Current implementation evidence: [K01 owner](../../../development/cache.md); new process partition still requires proof.

## UIB.D05-MEMORY.METRIC

Count owner root inline once; actual String/Vec capacities, initialized descendants,
Box layouts, keys/metadata and vacant fixed backing. Checked arithmetic refuses
overflow. Snapshot inline already inside its slot is not counted again. First
slice has no hidden index, owned duplicate Context key or shared payload.
Canonical schema::owned_size::{Heap,Overflow,HeapSize,owned} is one exhaustive DTO
walk; engine counts its own roots/headers/containers. Preserve wire/validator semantics.
Exclude allocator rounding/headers/fragmentation, stack beyond inline roots, OS/SDK/
helper state and external pixel/file contents; each needs its separate owner.
Owned-layout accounting is not RSS or a peak allocation cap.

## UIB.D05-MEMORY.LIMITS

| Explicit initial-profile ceiling | Maximum |
| --- | ---: |
| Aggregate retained domain / granted session slots / Store grants |64MiB /4 /4 |
| Snapshot slots per session / revisions per full-key family |16 /2 |
| Entry: Snapshot inline+heap+partition backing |8MiB |
| Session retained charge / age from successful admission |16MiB /300,000 monotonic ms |

MiB=1,048,576 bytes. No implicit Default; positive caller limits at/below profile.
Check actual fixed capacity/arithmetic before publication; rollback construction
allocations on refusal. Independent ceilings do not promise simultaneous saturation.
Measured ~0.10/0.27MiB actual Documents/~4MiB synthetic dense payload justify useful
admission examples, not a worst-case multiplier. Two revisions enable base/current;
16 slots bound families; five minutes bounds idle retention. Reads extend neither
age nor eviction order; higher limits require revision. [Evidence](../../../../tests/bridges/resources/working-memory.md).

## UIB.D05-MEMORY.OWNERSHIP

One host QuotaLedger charges its fixed backing before issuing move-only allowances
of bytes/session slots. Sum of grants+host ledger charge fits the root domain;
no Store implicitly receives the whole cap. Store roots/backing/session/payloads
fit their allowance; report used versus reserved, not their sum. Partial creation
drops before returning its grant; Store payload/backing drops before allowance reuse.
Detach frees local session heaps but retains vacant Store backing and fixed grant.
Caller/Completion charges Snapshot+partition until successful staged move; only then
transfer actual charge to Store, without cloning. Rejection returns unchanged
charged input or explicitly drops it. Zero encoded bytes cannot release live payload.
Borrowed reads only; caller copies require their own charge.

D05-PARTITION-001: the real parent Grant stays in supervisor WorkerLease; its local
ledger reference is never serialized/copied into a child. For the initial64MiB/four-
worker profile, reserve A=floor((64MiB-QuotaLedger::backing_bytes())/4) and one session
slot per worker. A supervised child creates a local ledger of at most A total bytes
and reserves its CacheStore after charging that ledger's root. Both ledger roots/
Store storage count once; no competing uncounted64MiB child domain. Smaller explicit
allowances still obey all root/Store caps. Worker working quota contains this memory.
Root grant survives worker exit reports/detach and is reusable only after confirmed
owned-process cleanup. Quarantine unreaped owners; fail closed rather than drop an
allowance that still backs live resources. Root ledger used_bytes alone is not
cross-process actual usage: show authoritative reservations separately from last
child usage with epoch/time or unknown. Telemetry never frees capacity.
Lost worker epoch invalidates all cache handles/bases: resync, not UI deletion or
revived action refs. Preserved records remain historical under their original identity.

## UIB.D05-MEMORY.ADMISSION

Family key is full canonical Context plus explicit nonempty unique sorted Channel
partition; lookup adds Snapshot ID/revision and scalar generation. Compare existing
set-field compatibility, never titles/coordinates/hash alone. ChannelResponse gives
its channel; combined/imported records need owner's declared partition, not inferred
from missing/failed/history observations. Count its Vec backing. Fixed slots/bounded
scans replace hidden indexes. Size/validation/eviction planning finish before mutation.
Exact full-key duplicate may reuse handle without age refresh; conflicting identity
refuses. Evict oldest admitted eligible entries only in requesting session, family
count first; never evict another session for room. Failure leaves entries/counters
intact. Missing/expired/evicted bases resync, not deletion. Counters never wrap.

## UIB.D05-MEMORY.LIFETIME / UIB.D05-MEMORY.GATES

Use explicit same-domain monotonic time; overflow/regression/domain mismatch rejects.
At age boundary reads refuse. Invalidation/TTL never upgrade freshness or restamp
Observation. Host schedules expiry/detach cleanup, no UI polling. Drop entry heaps
on expiry/eviction, session heaps on detach; vacant backing stays charged. Borrowed
reads prevent mutation. Separate derived relations/check caches share quotas later;
Snapshot-only slice is not full K01. Decode/replay/validation/encoding/Completion
work follows [D05-WORK@1](d05-working-memory.md); a permit/counter alone is no proof.
Tests cover exact byte/count/overflow/capacity, charged transfer/refusal, key/channel/
session isolation, eviction/resync, clock/invalidation/detach and real release,
known/unknown/redacted/empty preservation and new cross-process grant quarantine.

## Change record

D05-RET-002 selected retained policy before K01. D05-PARTITION-001 adopts reviewed
db629fc under [registration authority](../../../plans/ui-blueprint/packets/D05-runtime-registration.md), advancing @1 to @2 for supervised partition/lifecycle only.
Existing Store semantics/ceilings and all platform/live/D06 acceptance gates remain.
