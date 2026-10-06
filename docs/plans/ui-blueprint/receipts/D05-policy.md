# D05 retained policy decision and enforcement handoff

Status: finite retained-policy decision prepared for root checkpoint/review.
This supersedes the proposal-only receipt saved at8103675; its subprocess idea
and unadopted numeric proposals remain historical, not current policy.
D05@2/D05-MEMORY@1 select retained accounting/admission only. Implementation,
working-memory enforcement, independent acceptance and full D05-RES remain open.

## Authority, traversal and reconciliation

ROADMAP.CONTENT delegates engineering D05 choices under user-approved PLAN.UIB@1
(original358c757); [D05-policy packet](../packets/D05-policy.md)56dc93b authorizes
this finite policy decision. Root's follow-up explicitly requests retained limits,
lifetime, quota transfer/cleanup/channel partition and canonical sizing before
Core source, while leaving decode/replay enforcement as a live-gate dependency.
Root explicitly excludes selecting subprocess/D02 changes from this packet.
User clarification recorded in execution.md at36f2089 removes the mistaken demand
for renewed authority on in-plan defects. Five shared repairs are assigned to the
next S01-review-repair packet; they remain unchanged by this policy checkpoint.

Recovered AGENTS -> spec registry UIB.ROUTING@1 -> decision route D05@1 ->
GEOMETRY/PROJECTIONS/LIFECYCLE/CACHE/PERFORMANCE@1 and C01-EVIDENCE@1; full
MODEL/IDENTITY/EXCHANGE/PRIVACY/BOUNDARIES/ROADMAP dependency closure read.
Current S01/handoff D01-D06@1/D07@2, RUST.md/DEV.RUST@2 and their selected
FORMS/ACTIONS/NATIVE/GOLDEN/PILOTS/RUST-BOUNDARIES/REUSE@1 dependencies reused.
Feature template read before registering the new decision leaf. Final basis:
D05@2, D05-MEMORY@1 clauses METRIC/LIMITS/OWNERSHIP/ADMISSION/LIFETIME/GATES;
registry5 only routes that new revision. Other product CONTENT@1 stays unchanged.
Excluded: UI/runtime/input, new collection, export internals and protected repairs.

Requirements: bounded revisions/bytes/lifetime, full keys, separate cache purposes,
atomic publication, resync after eviction, privacy and independent sessions.
Observed inputs: saved owned sizing c6a5df6, Web fcf48be, Native51c7809 and working
allocation2a5dfef; Core [storage design](../../../development/cache.md)0d40af7,
its [receipt](K01-storage-design.md), existing canonical DTO/replay/accounting APIs.
No new measurements or source changes. Partial real examples remain partial;
synthetic capacity cases are not described as real or universal worst cases.

Core's fixed slots, full-key scans, exclusively owned snapshots, borrowed reads,
host allowance grants and staging-before-eviction proposals are adopted by the
new decision. Numeric/profile/lifetime choices come from ROADMAP's delegated
engineering authority, not from treating the proposal itself as a requirement.
Its proposed derived caches and peak-bounded replay integration remain later
finite work. Process packaging and quota-worker proposal are not selected.

## Contract delta D05-RET-002

Old: D05@1 required limits before live use but left retained numbers, charge metric,
quota ownership and lifecycle unresolved. New: D05@2 routes the concrete
[D05-MEMORY@1 policy](../../../specs/development/decisions/d05-memory.md).
Compatibility: initial unreleased internal ownership policy; no JSON fields,
validators, scenario limits/tolerances, public CLI, D02 packaging or release change.
This resolves K01's retained-policy dependency without declaring the live gate met.

Chosen explicit ceilings (no implicit Default):4 aggregate session slots and4
outstanding Store grants;16 Snapshot slots/session;2 revisions/full-key family;
8MiB occupied entry,16MiB/session,64MiB aggregate retained domain;300,000ms age.
All maxima are independently enforced; a request to saturate every maximum may
refuse when fixed backing and other reservations leave insufficient space.
Slot/table construction cannot silently lower requested capacity to appear valid.

The entry ceiling covers Snapshot inline+owned heap+partition backing. Store
accounting counts that inline only in its fixed slot backing, not twice. Host
ledger root/backing is charged before grants; sum of byte/session-slot allowances
must fit global bounds. Per-store usage includes all fixed root/table/session
storage and live payloads. Free slots stay charged. Derived caches eventually
share this quota while retaining separate purpose-specific owners and ceilings.

Rationale: actual supplied documents own102,043/286,841 bytes; synthetic512KiB dense
input retains4,197,487 heap bytes. The8MiB entry ceiling admits these selected
examples;16MiB provides room for two synthetic payloads and ordinary metadata.
The64MiB domain allows multiple small independent sessions without an unbounded
scope count. Two revisions enable base/current comparison;16 slots bound families.
Five minutes permits short explicit workflows while bounding idle retention;
misses recover through resync. These are chosen admission ceilings, not statistical
worst-case multipliers or guarantees of all combinations. No failed benchmark or
protected test was relaxed to choose them. Higher maxima require policy revision.

## Ownership, transfer and lifetime implementation contract

Host owns a single aggregate retained ledger. It issues move-only, non-forgeable
Store allowances with byte/session-slot counts, domain and generation identity;
Store constructors cannot fabricate their own process cap. Limits apply across
multiple Stores; fixed allowances remain reserved even when entries are evicted.
A multi-session Store is allowed. No process/socket/worker dependency is needed.
Creation failure drops partial storage before returning its allowance. Store
close/drop must destroy owned storage before its allowance can be reclaimed;
no early reuse while a live owner exists. A deliberately lost grant cannot make
capacity available again merely because a receipt is missing. Host teardown owns
reconciliation/cleanup; first K01 tests cover actual release, not just ID removal.

Incoming Snapshot/partition stays with a charged caller or Completion during
size checking, existing canonical validation and planning. Caller working/Completion
reservation is separate from the Store's already reserved retained allowance.
On success, move into the planned slot and transfer actual charge; only then release
the caller payload charge. On rejection return unchanged charged ownership or
explicitly drop it. Do not release a whole Completion while siblings remain alive.
No accounting system may equate pending_encoded_bytes=0 with payload release.
Transient overlapping reservations can exceed actual ownership without claiming
double physical usage; report committed usage and reservations separately.

A full key contains canonical Context, declared channel partition, Snapshot ID and
revision; family omits only ID/revision. Snapshot IDs/keys are borrowed from canonical
payloads; no duplicate key strings/index. ChannelResponse supplies a singleton
partition; combined/imported snapshots require the owner's explicit partition.
Normalize nonempty unique canonical Channel sets once. Unknown/failed/history
observations cannot infer requested partition. Mismatched partition is a separate
family, not permission to join channels or upgrade coverage/freshness.

Admission checks all fit/identity conditions and plans evictions before mutation.
Oldest admitted eligible records in the requesting session are removed, family
limit first; reads never refresh eviction order or TTL. Another session's data is
never evicted to admit this one. Identical full-key duplicate may reuse the original
handle/age; conflicting record refuses. Counter/clock arithmetic cannot wrap.
TTL is admission time plus explicit same-domain age, not Observation timestamp.
At the expiry boundary read refuses; host invokes timed expiry/detach cleanup
without collecting UI. Invalidation is separate from canonical freshness and never
re-stamps history. Borrowed reads prevent mutation; caller-owned clones require
caller charges. Pixel references do not keep external buffers alive or current.

## Exact next source write sets and acceptance

Integration accounting extraction, separate source packet:
- crates/schema/src/owned_size.rs and lib.rs export; preserve existing safe
  Heap/Overflow/HeapSize/owned behavior and exhaustive type/variant handling.
- crates/schema/tests/owned_memory.rs, examples/measure_owned.rs and
  crates/plugin-api/examples/resource_working.rs switch to that one implementation.
- Remove tests/bridges/resources/owned_memory.rs duplicate after imports migrate;
  update its existing README links. No Cargo/wire/validator/dependency change.
- Focused capacity-only/reserve/Box/ZST/overflow checks and affected compilation;
  confirm diagnostic sizing still consumes the canonical API. No broad runtime QA.

Core retained-store source packet after canonical sizing is available:
- crates/engine/src/cache.rs (including pure allowance ledger if needed), its lib
  export, crates/engine/tests/cache.rs, existing cache.md and a scoped receipt.
- Real fixed-slot storage, charged caller admission, full-key borrowed reads,
  scoped eviction/resync, invalidation, age and detach; no raw wire/pixel store.
- Check exact byte/count/overflow/fixed-capacity boundaries; rejection atomicity;
  full Context/channel and session isolation; generation reuse; unknown/redacted/
  false/empty persistence; scope changes do not imply deletion; no TTL freshness;
  actual payload release versus retained slot capacity; grants cannot be reused early.
- Reuse the existing canonical validator, not a parallel validator or protected fix.
  Its temporary work is explicitly outside retained metric. Controlled fixtures may
  verify storage behavior; they do not prove peak-bounded validation/replay or live use.
- Existing pure replay is unchanged. Normal atomic replay admission may use the
  same staged transfer, but any peak-bounded claim waits for actual work enforcement;
  opaque permits or post-result byte checks cannot close that missing obligation.

A single changed code path cannot certify the whole process: framing, input bytes,
parser rejection/candidate allocations, validation/planning/replay temporaries,
encoding, completed-channel owners, host controls, native helpers and pixels still
need their named reservations/enforcement. A separate finite host decision must
select a justified bound or concrete enforcement for those allocations, including
aggregate overlap and failures. No further open-ended profiling is requested.
The quota-worker direction in8103675 would require a separate D02 delta defining
session/cache ownership, external preservation of completed channels, quota-specific
failure signaling, lost-base resync, late-response rejection and owned cleanup.
It is neither adopted nor permitted for source work by the current packet.

## Verification and checkpoint boundary

Documentation only: changed links/route/revisions and whitespace, <=100-line spec
nodes, preserved shared-source/D02 boundaries. No Rust builds/tests/runtime claim.
Exact five writes: spec README registry metadata; decision README D05 row;
d05-limits.md; new d05-memory.md; this receipt. Checkpoint awaits root Git grant;
commit/push SHA and lease release return in the terminal handoff. This is a saved
policy candidate for root review, not independent acceptance or complete D05/K01.
