# D02 — packaging and request-owned sessions

- Domain: `uib.development.d02`; accepted/released implementation: none.
- Node type: leaf; contract: `UIB.D02@2`; clauses: `UIB.D02.CONTENT`, `UIB.D02.WORKER`, `UIB.D02.PUBLICATION`, `UIB.D02.LIFECYCLE`; supersedes @1.
- Authority: Active / Evolving; C01-DEC-001 and D02-WORKER-001 below.
- Read when: plugin/host boundary, adapter or lifecycle integration.
- Do not read when: only unchanged pure geometry is affected.
- Requires: [BOUNDARIES@1](../../product/boundaries.md), [EXCHANGE@1](../../product/exchange.md), [LIFECYCLE@1](../../product/lifecycle.md), [NATIVE@1](../../product/native.md), [PRIVACY@1](../../product/privacy.md), [D01](d01-support.md), [D03](d03-data.md), [D05](d05-limits.md), [evidence](evidence.md).
- Owner/deadline: assigned host owner before bounded live use; W01/M01 own adapters.

## UIB.D02.CONTENT — preserved boundaries

Rust owns analytics/state; collectors remain narrow and bounded. Statically link
schema/engine/plugin API and selected adapters; no dynamic ABI, network daemon,
service framework, mandatory model or Apple SDK in core. Default CLI can own a
short session; reusable APIs support multiple explicit requests and no idle UI
collection. Optional events invalidate only; no polling to manufacture freshness.
Web uses owned CDP plus scoped DOM/CSSOM and separately addressed AX. No whole
DOMSnapshot then filtering; unsupported boundaries remain partial. D07 governs
transport adoption. Do not copy another accessibility engine or full CLI chain.
Native uses owned thin public-API Swift helpers and bounded UTF-8 NDJSON; escaped
newlines are data. Framing precedes parse; diagnostics never share machine output.
Own handles/process lifetime by session. Capture admission/serialization is limited
to capture resource/deadline, never unrelated AX/Rust work. No global native-call
queue or lock across AX/capture waits. Async cancellation does not prove OS cleanup.
Attach negotiates plugin/version/schema, exact target/generations and per-channel
capabilities; observe carries scope/fields/limits/freshness. Detach invalidates
handles/subscriptions and cleans owned helpers, never closes user applications.

## UIB.D02.WORKER — adopted reusable failure boundary

Use one reusable allocation-guarded Rust worker per attached session, supervised
outside its failure boundary. The real host owner is crates/host, with statically
linked canonical libraries; no second graph/parser/engine. Attach may launch the
worker; measurements on that session reuse it. Guard/profile: [D05-WORK@1](d05-working-memory.md).
HostDomain owns root retained ledger and parent buffers; RuntimeHost borrows it.
Caller authority is a trusted TargetLease, not UI data/discovery. Fixed slot/epoch/
sequence handles bind attach, reserve_input, submit, next_event, cancel/detach/shutdown.
OperationClass must agree with canonical payload; read-only denies mutation intents.
Only worker executes typed decode/validation, ObservationSession, K01 and engine.
HostCompletion owns terminal metadata, committed canonical byte leases, missing
channels and effect receipt. It is not a new graph DTO. No new public completion
JSON/ErrorCode/CLI syntax is implied; publication requires its consumer contract.
Legacy in-process APIs/file CLI are not silently relabelled memory-enforced.
Bounded operations, including analysis0.2 decode, use the worker path. The new
local failure boundary does not claim a same-user native-code security sandbox.

## UIB.D02.PUBLICATION — completion outside the worker

Reserve parent channel slots and worker publication buffer before work; refuse
capacity exhaustion before collection, preserving earlier leases. Validate
correlation/context and adapter redaction, then stream unchanged canonical bytes
without cloning retained snapshots or building unbounded Value. Publication
allowance covers wrappers/errors/control; code must prove this bounded path.
Private fixed headers carry protocol version/session epoch/operation/channel/length,
then payload and commit record. Parent copies directly into reserved slot; only
complete matching payload+commit before terminalization is committed and ACKed.
Worker waits for ACK before next risky channel/work. Do not advertise completion
before that boundary. Oversize/failed channels remain incomplete, never empty success.
Earlier ACKed channels survive later worker/capture failure, timeout and cancellation.
Parent never reparses bodies; payload UI text cannot forge length-framed controls.
Typed reuse occurs in an admitted worker. Caller-held completion stays charged
until real release, independently of Store teardown or pending_encoded_bytes=0;
its lease must not block another host slot. Logs contain bounded codes/counts only.

## UIB.D02.LIFECYCLE — deadline and truthful effects

Parent monotonic deadline is authoritative; pass remaining duration, with local
worker/helper clock domains/watchdogs rather than serialized shared Instant claims.
Worker owns the adapter ObservationSession clock; attach returns that domain for
canonical requests. Reject every late commit/ACK after parent terminalization.
Cancel/expiry stops new dispatch and kills only registered owned workers/helpers;
no unregistered grandchildren. Capture lease remains held until helper exit/reap;
AX stays independent. Liveness/cleanup watchdogs never collect UI.
Before future external mutation, parent holds a minimal redacted effect_possible
receipt outside worker, then issues a one-use nonce under Target mutation ownership
and global physical-input lane where needed. No nonce after cancel/expiry.
Before any permit report not_dispatched; after possible delivery, loss/timeout gives
action_outcome_unknown, stops dependent steps and never retries automatically.
At cleanup deadline without confirmed reap return CleanupPending; quarantine slot,
capture lease and root grant. Never replace/reuse early or infer release from ID
loss. Keep bounded cleanup ownership; failed destructor cleanup fails closed,
retaining reservation. HostDomain outlives all leases; shutdown is incomplete while
owned children remain. OS inability to terminate is explicit failure, not success.

## Proof and change record

D02-PROOF still requires real F01/current F02 common-wire evidence with identity,
coverage, versions/oversize/malformed frames, AX completion plus capture failure,
cancel/detach/late replies. Mocks alone do not freeze P1. Historical R01/R02/F02
and later fault work do not prove this new host. Platform owner/lane gates remain;
W01/M01 prove collection/input, K02/V01 lifecycle/privacy/isolation. New allocator/
publication/quarantine/action-nonce proof and unchanged D06 positives stay open.
D02-WORKER-001 adopts reviewed designdb629fc under ROADMAP/PLAN.UIB@1 through
[registration packet](../../../plans/ui-blueprint/packets/D05-runtime-registration.md).
Core0.1/analysis0.2 shapes, product authority/privacy/deadlines remain unchanged.
