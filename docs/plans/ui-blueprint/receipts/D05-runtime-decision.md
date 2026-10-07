# D05 runtime decision handoff

Status: one implementable mechanism selected as a proposed technical delta;
documentation checkpoint-ready. No runtime/source/spec/Cargo change or advance
acceptance claim. Root must register the deltas and assign finite source owners.

## Authority and basis

[Finite packet](../packets/D05-runtime-decision.md)c826b86 under approved PLAN.UIB@1
and delegated ROADMAP D02/D05 choices. Current master only, no nested agents/chats,
other projects, profiler, benchmark or new source survey. Later root clarification
kept this task at exactly two docs and transferred the not-started schema R1/R2
repair to Core. Web's transport/D07/Cargo lease is disjoint and unchanged here.

Reused complete current governance and spec registry7 -> D02@1/D05@2/
D05-MEMORY@1/D07@3 with their CACHE/LIFECYCLE/EXCHANGE/MODEL/PRIVACY/BOUNDARIES/
IDENTITY/GEOMETRY/PROJECTIONS/NATIVE/PERFORMANCE/ROADMAP closure, D03@2 and the
ANALYSIS/TYPES/VALIDATION@1 boundary where0.2 uses the same worker decoder.
RUST.md/DEV.RUST@2 and their selected dependencies remain applicable. Read the
current D02/D05 leaves and actual cache documentation completely for this decision.

Evidence: [audit07b9715](../../../research/D05-decode-bound.md), working evidence
2a5dfef, historical proposal8103675, D07@3 float qualification, current
[CacheStore](../../../development/cache.md). Narrow source read inspected actual
ledger.rs and Store field/drop/construction ownership; no general source reopening.
Native capture lifecycle documentation was read only to preserve its capture-only
lease, completed AX and owned-helper reap constraints, not to repeat platform QA.

## Requirement, observation, decision

Required: bounded work/memory/lifetime, preserved completed channels, independent
Target progress, real ownership/cleanup, no stale ref revival or uncertain action
retry, model-free on-demand sessions and no mandatory Rust process per measurement.
Observed: no complete prospective reserve is certified for typed prefixes,
validation scratch or float_roundtrip numeric work; data-before-kind can buffer
large invalid input. Infallible std/serde allocation is not a recoverable Result.
Actual Grant borrows a local ledger, is move-only and returns allowance on Drop;
it cannot cross a process boundary unchanged. Store fixed slots/ownership already
provide retained enforcement but not decode/replay/encoding working enforcement.

Selected [working-memory decision](../../../development/working-memory.md): one
reusable Rust session worker with pre-allocation charging and fatal quota exit,
outside a fixed-pool supervisor. Canonical parser/validator/engine/K01 stay in the
worker; no second parser/graph/analytics framework. The supervisor never reparses
graph payloads, formats raw errors or performs uncharged input-proportional clones.
Its trusted control/framing allocation inventory is small and explicitly subject
to implementation proof; another syntax census/general profiling wave is not selected.

## Concrete ownership and failure boundary

Proposed profile:4 workers;64MiB worker Rust requested/reserved layouts each,
ordinary cap63MiB plus1MiB publication allowance;32MiB parent buffers/controls.
Total managed Rust reservation ceiling288MiB, with K01 retained quotas nested,
not added again. Input2MiB;512KiB canonical/native frames; one request/session;
3 channels,2MiB total response;8 completion groups;2 registered helpers/session.
Explicit local/parent deadlines and1s cleanup precede quarantine. Stack/OS/SDK/
pixel costs remain separate named categories, not a false RSS cap.

Allocator forwards valid unchanged layouts/pointers, charges before alloc/zeroed,
reserves full new realloc layout alongside old, uses checked atomic counters and
cannot unwind/allocate/log from callbacks. A fixed nonblocking private fatal status
precedes immediate exit; absent status means generic failure, not guessed quota.
Only worker executable installs it; existing library unsafe policies remain intact.

Parent reserves publication slots before work. Worker validates/redacts and emits
one complete canonical frame plus fixed commit metadata. Parent commits/ACKs only
the full epoch/operation/channel-matched frame before deadline. Worker waits for
ACK before the next risky channel/work. Earlier ACKed bytes survive later worker/
capture failure; partial frames never masquerade as completed output. Returned
leases stay charged until actual release and do not prevent another session's progress.

Root retains the REAL QuotaLedger Grant in WorkerLease until all owned processes
are reaped. A subordinate child ledger partitions that grant and charges its own
root inside it; no child receives another global64MiB. Formula and actual-usage/
reservation distinction are explicit in the decision. Existing CacheStore methods,
limits, keys, transfer/eviction/TTL and Drop order are reused, not replaced.
No early release on terminal message/PID disappearance; cleanup failure quarantines
resources and fails closed. Lost worker epoch means lost bases/resync, never UI
deletion or fresh action authority for preserved historical bytes.

Future input permits require a parent-held redacted effect-possible receipt before
one-use dispatch, correct Target/global physical lanes, and no permit after cancel.
Worker loss after possible dispatch gives action_outcome_unknown with no retry.
Initial proof uses a fake endpoint; real action implementation remains its own packet.
Helpers are registered owned children; user applications are never cleanup targets.

## Exact delta and next executable consumer

Root registers D02-WORKER-001/D02@2, D05-WORK-001/D05@3 plus working leaf, and
D05-PARTITION-001/D05-MEMORY@2 before code. Core0.1/analysis0.2 schemas and existing
quality/deadline/privacy meanings stay unchanged. No new public completion JSON or
core ErrorCode is invented; the first boundary is typed host API/private IPC.

One new real crates/host owner implements safe host API, fixed pools/protocol,
supervisor/worker/process/allocator modules and focused tests; root serializes
membership/lock and any justified OS-binding dependency decision. Exact candidate
paths and sequence are in the decision. Core exposes existing K01/analysis calls;
platform owners integrate bounded transport/helper lifetimes after host proof.
No source owner is assigned implicitly by this documentation checkpoint.

Required proof is finite: allocator cap/overflow/realloc/fatal behavior; malformed
audit family inside the guarded child only; failures in each phase; exact preserved
channels; slots/lease saturation; cross-session progress; no grant reuse before
reap; cancel/late frames/lost bases; fake post-dispatch unknown outcome; watchdog and
cleanup quarantine. Parent allocation inventory, platform acquisition/SDK/pixels,
capture permissions/lifetime and unchanged D06 positive gates still require actual
implementation evidence. These are named acceptance duties, not proofs fabricated
by the decision. No principle-level product fork remains for another user choice.

## Verification and save boundary

Only docs/development/working-memory.md and this receipt are written. Links, route/
requirement-versus-proposal consistency and scoped whitespace checked; no builds,
runtime, profiling, source/spec/Cargo mutation or schema repair. Git grant pending;
scoped commit/push SHA and release return in chat. Stop after the finite decision.
