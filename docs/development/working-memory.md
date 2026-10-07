# D05 runtime enforcement decision

**Selected implementation direction: one reusable, allocation-guarded Rust worker
per attached session, with a bounded supervisor outside its failure boundary.**
This is a concrete proposal for root's D02/D05 registration, not current policy,
implemented enforcement or live acceptance. It closes the mechanism choice; no
further general profiling or universal-multiplier search is proposed.

Authority: approved PLAN.UIB@1 and ROADMAP's delegated D02/D05 choices through the
[finite packet](../plans/ui-blueprint/packets/D05-runtime-decision.md). Preserve
[D02@1](../specs/development/decisions/d02-boundaries.md),
[D05@2](../specs/development/decisions/d05-limits.md),
[retained policy@1](../specs/development/decisions/d05-memory.md) and D07@3 until
the deltas below are reviewed/registered. Existing core0.1 and analysis0.2 remain.

## Why this mechanism

The [source audit07b9715](../research/D05-decode-bound.md) leaves typed-prefix
capacity/lifetimes, validation scratch and float_roundtrip numeric work unproved.
The no-number malformed family can allocate over33MB before rejection under a
512KiB frame cap. That is a lower bound, not a reserve multiplier. A syntax census
could reject that family, but does not certify the complete admitted grammar.
Choose actual charging before allocation for the current canonical decoder,
validator, replay and encoder. Do not replace them with a new parser/framework.

Ordinary std/serde allocations are not uniformly fallible Results. A quota callback
cannot safely unwind; a refused infallible allocation can terminate its process.
The worker boundary is therefore required by this chosen enforcement technique
and completed-result/isolation requirements, not merely by preference for processes.
The supervisor does no untrusted graph/JSON decode. This is local failure isolation,
not a security sandbox against malicious same-user native code.

## Concrete owners and API

Introduce one real `crates/host` owner: bounded supervision, fixed framing/pools,
worker lifecycle and quota enforcement. Link the existing schema/engine/plugin API
statically into its session-worker executable. No daemon, network server, async
framework, second analytics implementation or mandatory model dependency.
Attach starts a worker; many explicit operations reuse it. Idle workers do not
collect UI. One-shot CLI invocations may own a short host lifetime; a measurement
on an existing session never requires launching another Rust worker.

The initial safe API is deliberately encoded at the supervisor boundary:

```text
HostDomain::new(HostLimits) -> owned root QuotaLedger + ParentBuffers
RuntimeHost::new(&HostDomain, trusted SpawnSpec)
attach(authorized TargetLease, InputLease<core0.1 SessionDescriptor>) -> SessionHandle
reserve_input(session, declared_bytes) -> InputLease
submit(session, OperationClass, InputLease, explicit parent deadline) -> OperationHandle
next_event / cancel / detach / shutdown -> HostEvent or CleanupPending
HostCompletion { terminal, committed channel leases, missing channels, effect receipt }
```

Handles contain fixed slot/generation/sequence values. `OperationClass` separates
attach/observe/local analysis and future authorized mutation; canonical payload
operation must agree. Only the worker runs `ObservationSession`, typed validation,
K01, engine or analysis verification. User authority comes from the caller's actual
authorized TargetLease, not UI data or a discovered element. Original core types
are reused in the worker; HostCompletion is ownership/terminal metadata, not another
normalized graph. Its bytes are complete canonical documents, not a new graph DTO.

Existing local library calls/legacy file CLI are not automatically claimed to have
this host's memory enforcement. Bounded host operations, including local0.2 decode,
must use the worker path. Existing CLI syntax/wire output remains unchanged until
its owner wires that path explicitly. No new public completion JSON is invented
here: first ship/prove the Rust host API and private IPC; publication of observation
CLI terminal metadata belongs to its explicit consumer contract, not a silent0.1
ErrorCode addition. Host quota refusal is a typed HostError::ResourceLimit.

## Metrics, initial ceilings and phase overlap

All limits are explicit constructor/profile inputs, validated before attachment;
these are proposed maximums for the first implementation, not hidden defaults or
claims that every allowed byte/node combination completes successfully.

| Owner / resource | Predeclared ceiling and enforcement |
| --- | --- |
| Active workers |4, matching the aggregate session-slot ceiling; reserve before spawn |
| Worker Rust allocation requests |64MiB per process; all globally allocated Rust layouts plus pending realloc reservation, from startup through shutdown |
| Ordinary/publication split |Ordinary admission stops at63MiB; the final1MiB is reserved for publishing an already validated channel and bounded terminal handling |
| Bootstrap |1MiB before the trusted fixed configuration is accepted; raising to the admitted worker limit requires the pre-reserved host lease |
| Supervisor Rust-owned buffers/control |32MiB total owned-layout pool, initialized fallibly before any child; no input-proportional allocation after reservation |
| Inbound operation |One per session, at most2MiB; acquire/fill a fixed InputLease, no hidden external Vec copy |
| Native ingress |At most two registered helper streams/session, each512KiB; separate slots prevent a partial capture frame monopolizing AX framing |
| Published canonical frame |512KiB/channel; at most3 channels and2MiB cumulative request output, also respecting smaller caller limits |
| Completion retention |8 fixed groups total, at most one active operation/session; completed leases count until actually returned/dropped |
| Cleanup |Parent terminalizes at the request deadline/cancel; owned kill/reap allowance1s, then quarantine rather than false release |
| Threads/helpers |One worker operation thread plus one allocation-free watchdog; at most2 owned native helpers/session; no unregistered grandchildren |

MiB=1,048,576 bytes. Supervisor payload slots need at most8MiB inbound +4MiB
native ingress +12MiB completion data. Fixed headers/identities/child records/spawn
workspace and actual backing must fit the remaining8MiB; reject setup otherwise.
Use bounded fixed control records (maximum4KiB), checked lengths/counters and no
data-dependent JSON/serde/error formatting there. A single preallocated byte owner
can supply disjoint fixed buffer leases; no general arena allocator is required.
Returning a completion lease cannot block progress of other host slots by borrowing
the entire mutable host. Lease/pool lifetimes and disjoint buffers enforce ownership.

The reservation ceiling is256MiB worker requested layouts +32MiB supervisor owned
layouts =288MiB for this managed Rust domain. Retained K01 memory is NESTED inside
worker quotas and is not added again. Reservation is not RSS or actual usage.
Input, old/new DTOs, parser rejection/error strings, numeric Bigints, validation
clones/maps, replay candidate/base, serialization and surviving worker Completions
all share the actual worker cap; their unknown overlap cannot exceed its guard.
Committed encoded copies in the supervisor are real separate memory, charged there.

Allocator headers/rounding/internal realloc bookkeeping, code pages, OS buffers,
stacks, Swift/SDK/pixel allocations and another application's memory are excluded
from that metric. Worker thread count is fixed; request explicit stack ceilings
(initial target8MiB main/1MiB watchdog) through the supported platform owner and
prove their setup separately. Do not label288MiB an address-space/RSS limit or
silently count opaque helper/pixel cost as zero.

## Enforcement before allocation

Only the session-worker executable installs the quota GlobalAlloc. Existing
schema/engine/plugin/CLI `forbid(unsafe_code)` policies stay intact. A small private
allocator module forwards unchanged valid pointers/layouts to System; safe libraries
do not install an allocator. Atomics and fixed records perform checked charging.
Reserve layout.size before alloc/alloc_zeroed; on realloc reserve the FULL new
layout while the old remains charged, then release old charge on success or the
new reservation on failure. This conservatively covers moving growth without
claiming System-internal precision. Dealloc releases after forwarding its layout.
Counter overflow/underflow is a fatal invariant, never wrapping or permitting growth.

Quota exhaustion uses an allocation-free best-effort fixed status write to a
preopened nonblocking private descriptor, then immediate process termination.
No panic/unwind, formatting, logging, mutex, allocator recursion or recovery through
infallible Vec is allowed. Distinguish quota exhaustion, System allocation failure
and generic worker failure; missing status must not be guessed as quota refusal.
Private exit codes/status records are not public CLI exit promises. Disable owned
worker/helper core dumps before untrusted input; raw stderr/panic payloads are not
forwarded, persisted or interpreted as control messages.

The supervisor uses fallible fixed-pool creation and allocation-free bounded
steady-state framing/control. Process setup uses bounded SpawnSpec storage and a
narrow owned-process platform wrapper, not an unbounded environment/argv collector.
Every Rust-owned control allocation must be inside the pool or precharged fixed
root inventory. OS spawning internals remain the named opaque platform category.
No arbitrary graph deserialization, `Value` buffering, large DTO clone or uncharged
result formatting is permitted outside the worker. The source/check packet must
prove this small supervisor path; a merely measured control allowance is insufficient.

## Completion publication survives the worker

Before collection/decode, reserve all requested result slots in the supervisor and
the worker's reusable512KiB publication buffer. Capacity failure refuses before
starting work and preserves earlier requests' leases. No unbounded waiting queue.
For each completed channel the worker validates correlation/context and adapter
redaction, then streams the unchanged canonical document into its bounded buffer.
Use borrowing/streaming serialization rather than cloning a retained Snapshot or
building an unbounded Value. The1MiB publication allowance covers small wrapper/
error/control work; source and focused allocation checks must prove that path.

Private IPC uses fixed headers containing protocol version, host session epoch,
operation sequence, channel and byte length, followed by payload and a fixed commit
record. Parent copies directly into that operation's reserved channel slot. Only a
complete matching payload+commit received before terminalization becomes a
committed result. Parent acknowledges; worker waits for ACK before beginning the
next channel or further allocation-heavy work. Completion is not advertised before
this publication boundary. An oversized/failed channel remains incomplete, never
successful empty data. Earlier ACKed channels survive subsequent allocation failure,
capture timeout, cancellation or worker death.

The parent NEVER reparses those bodies. UI text cannot create control frames inside
the length-framed payload. Any typed reuse occurs in an admitted worker and counts
again; stdout streaming requires no graph decode. A caller-owned completion remains
charged independently of worker/Store teardown and zero pending_encoded_bytes.
Debug/terminal diagnostics print bounded codes/counts, never the retained bytes.

## Reusing the actual K01 allowance model

The current `Grant<'ledger>` borrows a process-local QuotaLedger and returns its
reservation on Drop. It cannot be serialized, duplicated or reconstructed as an
equivalent grant in another process. Keep the real root grant in a supervisor
WorkerLease until actual process cleanup is proved.

Proposed D05-MEMORY delta: the root ledger is the single aggregate authority and
may partition a grant into a supervised child domain. With global64MiB, choose
`A=floor((64MiB-QuotaLedger::backing_bytes())/4)` bytes and one session slot per
worker. Root reserves A; child creates its LOCAL ledger with at most A total bytes
and reserves its actual CacheStore allowance after charging local ledger backing.
Thus both ledger roots and Store storage count once under the aggregate; the child
does not obtain another64MiB. Root grants remain move-only/reserved until reap.
Keep K01's existing slot/family/8MiB-entry/16MiB-session/300,000ms limits, ownership
transfer, eviction/invalidation and Drop order. No second cache/index or source key.

Root `QuotaLedger::usage().used_bytes` alone is NOT cross-process actual usage.
Report authoritative outstanding reservations separately from last reported child
usage (with epoch/time or unknown after failure). Usage telemetry cannot release a
grant. Detach may lower child used bytes but does not return the root allowance;
only confirmed teardown does. A lost worker invalidates its session epoch and all
cache handles; later bases require resync, not UI deletion or ref revival. Preserved
old canonical data may be analyzed as recorded data, never restamped as a fresh
attachment or silently imported into a different live session identity.

## Deadline, cancellation, action and cleanup state

Parent monotonic deadline is authoritative. Pass remaining duration; worker/helper
use their own named local domains and a watchdog, never compare serialized Instant
values as a common clock. The worker is the adapter's local ObservationSession
clock owner; attach returns that domain for its canonical requests. Core transport
shape does not change. Parent rejects every late commit/ACK after terminalization.
Cancel/deadline stop new dispatch, close admission and terminate only registered
owned workers/helpers. All helpers are directly supervised or transferred with a
proved parent ownership handle; worker-spawned unregistered descendants are forbidden.
Capture keeps its separate lease until its helper exits/reaps; AX does not wait on
that global capture lease. Parent/worker liveness watchdogs do not collect UI.

Before any future external mutation, parent must hold outside the worker a minimal
redacted `effect_possible` receipt and issue a one-use dispatch nonce under the
authorized Target mutation lane (plus global physical-input lane when applicable).
No grant after cancel/expiry. Once issued, a timeout/worker loss may mean delivery:
return action_outcome_unknown and stop dependent steps; no automatic retry. Before
any permit, report not_dispatched. Read-only paths reject mutation dispatch intents.
The first host slice implements/proves this state machine with a fake delivery
endpoint; actual A01/W02/M02 input remains a separately authorized consumer.

At1s cleanup expiry without confirmed reap, return CleanupPending and quarantine
the slot, capture lease and root grant. Never replace/reuse them early or claim
memory freed because an ID disappeared. Other admitted sessions continue. Keep a
bounded reaper/liveness owner until terminal; failed destructor cleanup must fail
closed (retain/leak the reservation rather than dropping a still-live Grant).
The caller's HostDomain outlives every worker and result lease; its lifetime is not
the worker's lifetime. Host shutdown is not complete while owned children remain. An OS-level inability
to terminate is an explicit cleanup failure, not a larger hidden timeout or success.

## Proposed deltas and finite implementation order

1. **D02-WORKER-001 / D02@2:** explicit reusable Rust session-worker boundary,
   encoded completion/ACK ownership, private fixed control protocol, local clock
   domains, parent deadline/dispatch authority and registered-child cleanup.
2. **D05-WORK-001 / D05@3 plus working-memory leaf:** the metrics/profile/actual
   allocator enforcement, publication reserve, parent pool, exhaustion and gates.
3. **D05-PARTITION-001 / D05-MEMORY@2:** supervised subordinate retained ledgers,
   root grant held until reap, telemetry vs authority; all existing Store semantics
   and numeric ceilings otherwise unchanged. No core0.1/analysis0.2 schema delta.
4. Root assigns one `crates/host/**` owner and serialized root Cargo membership/
   lock changes. Concrete files: lib.rs, limits.rs, buffers.rs, protocol.rs,
   supervisor.rs, process.rs, worker.rs, quota_allocator.rs and the session-worker
   binary, with focused tests. Unsafe is confined to allocator/fatal-signal and
   owned-process platform modules with documented/tested invariants. Use std and
   existing pinned dependencies; any actual OS-binding dependency requires its own
   narrow D07 inspection before adoption. No new runtime/parser/framework is selected.
5. Core provides calls into existing K01/analysis APIs; no current store rewrite.
   Platform owners integrate their bounded adapters/helper handles only after the
   host failure protocol passes. CLI routes bounded session operations explicitly;
   legacy in-process calls are not relabelled enforced by installing an unreviewed
   allocator into their library. Shared source/check ownership is pinned before work.

## Falsifiable proof before live acceptance

- Guard tests: precharge before System, exact cap/overflow, zeroed allocations,
  concurrent atomic accounting, realloc old+new reservation, real allocation failure
  and allocation-free fatal status; no callback unwind. Observe actual allocations
  so optimizer-elided work is not mistaken for a quota test.
- Run the audit's hostile tag-order/deep-array family ONLY inside the proven guarded
  child, including a2MiB instance whose lower bound exceeds its quota. Test both
  core0.1 and analysis0.2 malformed/tag-buffered/long-string/numeric inputs and the
  current float_roundtrip feature. Outcomes may be bounded invalid_input or quota
  refusal as appropriate; no measured multiplier or changed accepted member order.
- Force exhaustion in decode/rejection, validation, replay candidate, encoding and
  retained admission. Parent stays alive; all ACKed channels match submitted
  canonical bytes, partial/uncommitted frames never become results, and source
  truth/privacy/unknown distinctions remain. Reuse one worker across normal requests.
- Exhaust each input/completion/control slot before dispatch; parent performs no
  proportional parse/clone/grow and holds actual leases through caller return.
  Verify no cross-worker allowance duplication, no early grant/capture release,
  exact duplicate/cancel/late-frame rejection, lost-base resync and real cleanup.
- Stall/fail Target A while B completes AX/local analysis. Force post-permit worker
  loss with fake input: one possible delivery, unknown outcome, zero duplicate retry.
  Exercise parent death/EOF/watchdog and cleanup quarantine without touching user apps.
- Existing F01/F02 positives must fit the predeclared profile and retain their
  original fields/coverage/deadlines. Warm/cold IPC/startup costs remain inside D06
  gates; this decision does not assert those timings passed or relax them.

Platform gates remain explicit: Web must bound acquisition and transport before
buffering; Native must bound its own retained/copied strings/pixel buffers and
helper/thread/handle lifetime, preserve capture lease/permission semantics and prove
required positive capture. Opaque SDK/OS cost is not a Rust quota or RSS claim.
Unqualified platform behavior cannot be labelled accepted through successful Rust
quota tests. Parent closed-control allocation inventory, worker guard/publication,
clock/cleanup and both affected platform integrations must be implemented and checked
before claiming the full live D05 gate; no additional broad profiling is prerequisite.
