# H01 bounded host — implementation in progress

Norms: [D02@2](../specs/development/decisions/d02-boundaries.md), [D05@3](../specs/development/decisions/d05-limits.md), [D05-MEMORY@2](../specs/development/decisions/d05-memory.md), [D05-WORK@1](../specs/development/decisions/d05-working-memory.md). Source handoff: [reviewed working-memory design](working-memory.md). The connected runtime and guarded worker now execute canonical requests. H01 failure coverage and live acceptance remain incomplete.

## Working foundation

`uiblueprint-host` statically depends on the existing schema/engine/plugin API and approved serde/json. No dependency versions/features changed. The narrow approved libc0.2.190 binding is available on Unix; no framework or new registry version was introduced.
`HostLimits` requires explicit values within D05 ceilings, including ordinary/publication split, root retained partition, fixed parent/input/ingress/result/control budgets and stack/cleanup bounds. No Default or measured amplification multiplier. Worker ordinary admission cannot exceed63MiB even when a caller selects a smaller publication reserve.
`ParentBuffers::new(limits, other_fixed_roots)` uses fallible fixed setup, counts actual root/backing capacities and drops partial construction on failure. Inputs, two separate ingress slots per worker and three output slots per completion group are disjoint fixed owners. ByteLease exposes only bounded slices/length and cannot grow a Vec. Exclusive borrows cover individual slots, not the entire host: a held output lease leaves another worker's input available. Logical bytes are cleared on release/reuse; this is not a forensic zeroization/RSS claim.
`other_fixed_roots` is an explicit inventory charge, NOT evidence that unimplemented supervisor/process owners are already closed. RuntimeHost must supply its actual complete control/spawn/lifecycle inventory before children. RuntimeHost now charges its fixed Vec root, SpawnSpec, worker/config/control/input/publication handles and actual platform owner inline layout before spawning; helper ownership is still pending.
`Control` is a fixed64-byte private record: magic/version, closed kind/class, result slot, flags, epoch/operation and three numeric values. Encoding/decoding allocates nothing; invalid magic/reserved bytes/tags/slot reject. Body bytes are opaque. Message-specific state/flags, partial frame/commit/ACK and terminal/late checks belong to the supervisor; `matches` alone is not publication proof.

## Compiling platform API handoff

`src/process_api.rs` is the bounded substitution boundary, with no dummy success or placeholder implementation. Core owns it; a root-assigned platform worker can implement `src/process.rs` and its own tests/peer without touching Core's limits/pools/protocol/supervisor/worker/allocator files.

- `SpawnSpec::new(&Path) -> Result<SpawnSpec, HostError>` copies an explicit absolute executable path into a fixed4096-byte root, rejects interior NUL/overflow, and exposes a valid CStr. No shell, inherited environment or arbitrary argv collection.
- `ProcessPlatform::spawn(&mut self, &SpawnSpec) -> Result<Self::Child, HostError>`, associated Child: OwnedProcess. Use exact-path posix_spawn; only owned returned PID/FDs are cleanup targets.
- `ProcessPlatform::poll(&mut self, &mut [PollInterest], wait_ms:u32) -> Result<(),HostError>` handles at most36 interests (4×(1 worker+2 helpers)×3 lanes) with fixed backing. Relative bounded wait, no hidden EINTR retry/deadline extension.
- OwnedProcess: `write_input`, `read_output`, `read_fatal` → Transfer::{Bytes,WouldBlock,Closed}; `close_input`; `terminate` only the owned unreaped child; `try_reap` → Running/Exited{code}/Signaled{signal}; borrowed input/output/fatal descriptors. No PID supplied from UI/canonical input.
- Child protocol descriptors:3 input,4 output,5 fatal;0/1/2 go to /dev/null. Parent descriptors nonblocking/CLOEXEC; do not inherit other sessions' FDs. File actions must avoid source/target descriptor collision. Raw stderr/panic bytes are neither persisted nor interpreted.
- WorkerPlatform: `setup_main(main_stack_bytes) -> Result<actual_stack_bytes,HostError>` must set/verify core-dump suppression and main stack before untrusted input; `current_stack_bytes()` runs inside the actual watchdog to verify its explicit stack; `fatal_exit(fd, &[u8;64], code) -> !` performs one best-effort nonblocking fixed write then _exit, without Rust allocation/format/unwind. fd<0 skips unavailable bootstrap status; parent must not guess quota cause.

Native implementation errors are bounded HostError categories, never payload/path dumps. No successful cleanup claim before confirmed wait/reap; uncertain/lost ownership must fail closed rather than signal a reused/user PID. Core owns root Grant, quarantine/deadline/effect/publication state, and holds it until cleanup. The OS layer returns facts; it cannot release a grant based on exit telemetry.
Initial target is pinned aarch64-apple-darwin. Spawn/file-action C internals and OS buffers remain the explicit opaque platform category, not Rust parent-pool accounting or a security sandbox. Actual FD/stack/core-dump/reap/allocation proofs remain required.

## Dependency evidence

D07@5/H01-PROCESS-001 checkpoint `7b1489f5a23955978fdcba78c3d5131417768805` precedes adoption. libc EXACT0.2.190, default-features=false, existing lock checksum `ce5d3ddc6d3fa000eb1536d85e147bfe31aacaba692ed6a876f95cb7c855be78`. Selected Cargo/license/unix/bsd/apple installed files matched the crate archive. Full MIT read (Rust Project Developers); MIT OR Apache-2.0; no NOTICE; MSRV1.65. No copied binding declarations/source.
Read only posix_spawn/file-actions/flags, socketpair/fcntl/poll/read/write/close, kill/waitpid, getrlimit/setrlimit, _exit and relevant pthread stack queries. The selected dependency enables those real consumers; this audit is not successful runtime proof.

## Next implemented stages and proof boundary

Core: actual HostDomain/RuntimeHost and input/completion group lifecycle; trusted target/operation binding; parent fixed-frame publication/ACK; reusable worker invoking canonical APIs/K01; executable-only quota GlobalAlloc; deadlines, nonce, quarantine and failure handling. Platform process slice is disjoint only after root assigns its exact files.
First prove guard/fatal/owned cleanup on small deterministic processes. Only then run hostile2MiB/tag-order/array/numeric source-audit families inside that guard. No unguarded adversarial parsing, echo-only canonical-processing claim, caller-lease early release, SDK/pixel or full D06 assertion. Current checks/checkpoints live in [H01 receipt](../plans/ui-blueprint/receipts/H01-host.md).

## Connected publication/counter stage

ParentBuffers now reserves whole completion groups before dispatch. Group capacity
remains occupied while caller-held committed frames exist, even after worker teardown.
Publication copies directly into reserved slices, distinguishes body/commit/ACK,
rejects wrong/duplicate/partial/late/oversized controls, and retains only fully ACKed
channels on terminalization. Other worker/group slots remain usable. This is the
actual safe byte state machine, not yet a complete supervisor/canonical worker.
QuotaCounter provides checked atomic precharge/release/peak accounting, including
full-new-layout realloc reservations while old bytes remain live and no wraparound.
It does not install GlobalAlloc or prove real System/fatal enforcement by itself.

Native's actual Darwin process module and process_peer target are now linked.
The approved WorkerPlatform::watchdog_stack_request extension computes a checked
creation request only. Native measured main8,372,224 and watchdog1,044,480 bytes
below unchanged8MiB/1MiB ceilings in its peer; real Core watchdog must independently
query its created thread before untrusted work. See [process handoff](host-process.md).
The next connected stage below supersedes the unconnected-WIP status. The stage-B
checks above still establish only their original safe publication/counter scope.

## Connected runtime and parent lifetime integration

HostDomain acquires one process-wide, non-cloneable ParentReapingLease through the
actual ProcessPlatform::validate_parent_reaping predicate. RuntimeHost uses the same
platform type. No signal handler is installed or changed. Detected policy drift
permanently closes admission, quarantines live slots, transfers already ACKed bytes
to callers and retains the real parent grants. Failed destruction intentionally
retains the stable domain/runtime backing and the global claim. It never declares
uncertain resources released. Normal shutdown returns completions and reaps before
reusing slots; callers should drain it explicitly. Dropping the host discards any
uncollected completions as part of owner destruction, while caller-held completions
remain valid and charged. Retained reservation includes the root ledger's192 bytes.

Embedding callers must continuously preserve default SIGCHLD without auto-reap or
custom handlers and exclusive managed-child reaping until complete cleanup. Local
checks cannot atomically defend against arbitrary racing same-process native code.
Native's own boundary checks remain necessary. No same-user security sandbox claim.

The macOS session-worker executable alone installs GuardedAllocator. It precharges
requested layouts, keeps old+full-new realloc layouts charged across System, and
uses fixed fatal status/_exit. The actual watchdog checks its measured stack extent
before canonical input and joins before inherited FD owners drop on normal/error
unwind. Watchdog EOF/deadline work never collects UI. Main and watchdog ceilings
remain8MiB/1MiB; initial bootstrap guard1MiB, then trusted configured limits.

Worker code calls canonical core0.1/analysis0.2 decoders, engine computation/result
verification, CacheStore/replay and ObservationSession. The private Tape codec only
segments existing canonical documents into bounded input bytes. Parent code never
parses graph/JSON. Actual adapter collection, helper registration/capture ownership
and fake action delivery/nonce remain incomplete; Mutation currently refuses, and
no external input is delivered. Connected Validate and malformed-input paths have
runtime proof; other canonical paths are compiled but await operation-specific proof.

Scoped tests prove two reusable sessions, a held completion during independent
progress, invalid-input refusal, real small setup quota fatal and confirmed reap.
An isolated signal-policy peer forwards to real Darwin process/predicate code,
pauses output reads after ACK solely for deterministic drift injection, and verifies
quarantine, retained grants, preserved bytes, stopped admission and no further
kill/wait. It restores only its own policy and independently reaps its sole worker
after EOF. The operator/test-runner signal policy is untouched. Independent R1
consumer review remains pending; Native-only acceptance is not full H01 acceptance.

Hostile parser families, full realloc/System/invariant/fault-phase proofs, deadline/
late-frame/cancel matrix, helpers, nonce and complete D06/live gates remain open.
No unchanged Native tests were rerun; see the current [receipt](../plans/ui-blueprint/receipts/H01-host.md).

## Lifecycle follow-up

A previously reserved AttachInput is now refused after shutdown before spawn; its
unused reservation drops normally. Focused real-worker tests also saturate both
completion groups and preserve both caller-held results on admission refusal.
A narrow process wrapper withholds configuration writes for A and later withholds
reap confirmation for100ms, while still using real owned Darwin children. B attaches
and validates canonical bytes independently; cancelling A reports CleanupPending at
the explicit1ms test cleanup allowance and retains its root grant until actual reap
is forwarded. A separate10ms parent-deadline case refuses later dispatch and stale
handles after cleanup. These are deterministic injected supervision conditions,
not claims that Darwin termination itself stalled or production deadlines changed.

## Returned ownership loss before configuration

Spawn success is not sufficient for admission: Native can return an owned wrapper
already latched Lost after a detected post-spawn policy violation. RuntimeHost now
queries that returned owner with try_reap before any configuration/input. Running
continues; confirmed exit releases normally; an uncertain/lost result closes input
and retains the actual child, reservation and roots in a permanently quarantined
slot. Current policy recovery does not restore that owner's authority. The host
performs no later I/O, termination or reaping on that latched-lost owner. New session
admission closes; existing independent slots remain separately owned. A disposable
regression uses the real Native lost-state transition while restoring current policy,
proves zero config/readiness I/O and reservation retention, then independently reaps
its sole EOF-exiting child for test teardown. Independent R1 recheck remains required.

## Parent effect permit and finite fake endpoint

The parent now reserves a fixed per-Target mutation claim before dispatch. A second
mutation on the same trusted Target/generation refuses; reads and distinct Targets
remain independent. The private EffectReady message may request the single physical
lane. Parent validates active correlation/class/deadline, holds that lane if needed,
records Possible outside the worker and only then sends a fresh one-use nonce.
Repeated permit requests fail. The claim/lane remains held through uncertain worker
cleanup, including quarantine, and releases after confirmed terminal delivery or reap.
No permit is sent after parent cancel/expiry. Control reading waits until pending
configuration/input/ACK/permit writes finish, with a fresh deadline check before
control dispatch. Output remains opaque and uses the same commit/ACK boundary.

A successful fake endpoint echoes its issued nonce in terminal metadata; Confirmed
means delivery acknowledgement, not verification that an application outcome
succeeded. Loss, cancel or failure after issuance retains Possible/unknown and never
retries automatically. Production session-worker still refuses Mutation: real
resolve/precondition checks, platform input/SDK integration and application outcome
verification are not enabled by this host state machine.

Only the explicitly selected test example effect_peer performs fake delivery, a
local counter consuming one permit. It publishes an unchanged existing canonical
Action fixture, requests no platform permission, touches no UI and never performs
real input. Five proof scenarios cover reuse/distinct nonces, loss/duplicate request,
read-only/pre-permit cancel/deadline, same-Target exclusion, distinct-Target physical
lane contention and post-permit cancellation with unknown effect. The unresolved
fake physical call waits for parent cancellation/EOF; host/test deadlines own its
bounded cleanup. No stability sleep or real input claim is used.

## Registered helper owner and Native consumer handoff

Current connected ownership API in src/helper_runtime.rs/host RuntimeHost:

- spawn_helper(session, HelperKind::{ExternalSemantics,Capture}, trusted SpawnSpec,
  parent Instant deadline) reserves one of two fixed ingress/child slots before
  direct owned spawn. Parent owns executable authority; raw helper/UI data cannot
  select a program/PID. Capture acquisition is exclusive across the domain; AX
  admission is independent. This grants no platform permission or SDK capability.
- write_helper(handle, &InputLease, offset) borrows already charged input from that
  exact session and returns a bounded nonblocking Transfer. read_helper(handle)
  reads only into that helper's fixed ingress up to the caller profile cap512KiB.
  Stale session/helper generations and expired deadlines refuse further I/O.
- take_helper_bytes moves the raw ingress lease to the caller and starts helper
  termination. Bytes remain charged, may outlive helper/session cleanup and do not
  borrow the whole RuntimeHost. They are unvalidated raw bytes, not a committed
  canonical channel. close_helper starts owned cleanup without producing data.
- next_event/shutdown drive HelperClosed/HelperCleanupPending and worker cleanup.
  Capture lease survives terminate requests until actual helper reap. Session root
  Grant survives Rust worker reap while any helper remains; final Closed/reuse waits
  for all children. Lost owners quarantine, retain backing and stop admission.

Domain holds the helper serial/capture claim; RuntimeState's precharged fixed worker
inventory contains both helper owners. No parent growable request queue, JSON/graph
parser or SDK allocation was added. Native ingress uses the already allocated
separate slots; copied SDK strings, pixels and helper stacks remain an explicit
external owner category for Native's later proof, not counted as free Rust memory.
The existing process_peer used in tests is a non-UI resource stand-in; its use does
not establish real Native SDK/capture behavior or helper RSS bounds.

Current composition limit is exact: reserve_input requires Attached, and
worker_main::run/CanonicalSession::observe consume a preassembled Tape of existing
canonical documents. take_helper_bytes closes a helper, so it cannot yet transfer
the first AX frame while the same AX+capture process remains active. No active-worker
helper request/reply path exists, and this API is not ready-made live M01 composition.
TargetLease binds trusted Target/generation; the actual guarded session validates
SessionDescriptor scope/capabilities and owns the local clock returned by Attached.
Parent does not infer that scope/clock from raw helper bytes. The future bounded
broker must carry that validated worker context and original deadline, and return
raw bounded frames to guarded parsing/normalization before canonical publication.
Earlier AX must reach full parent commit/ACK before capture's next risky step.
Native's source-grounded handoff supplies existing framing/binding/redaction shape;
no replacement generic graph protocol or new public completion JSON is selected.

Core owns helpers.rs/helper_runtime.rs, domain/host_types/supervisor wiring, private
worker composition and its tests. Native process.rs/process/**, process tests/peer
remain protected; Integration owns allocator/hostile proof files. Current allocator
pin is2f1bf278…41e50e; other three worker pins remain unchanged pending the exact
broker handoff. No UI/SDK/input or live acceptance is claimed by this stage.

## Compiling worker producer boundary for disjoint Web ownership

The optional host feature `web` selects only the existing uiblueprint-web path and
workspace log; defaults remain empty. Cargo.lock adds those two host edges only.
Default and web worker configurations compile; provider logging/connection remains
an explicit future WebSession attach step, not an implicit library startup effect.

Core's actual binary-private boundary is now:

- CanonicalSession::begin_observation(input, now, requested_mask) returns the decoded
  existing Request and ObservationRun with the real ObservationSession Ticket. It
  validates Observe/mask and performs actual begin before acquisition.
- ObservationRun::receive_channel(bytes, expected_channel) validates the canonical
  envelope/channel and invokes real receive with the worker clock. finish performs
  complete; Drop cancels unfinished work. Failed canonical ChannelResponse records
  remain records, not invented empty success.
- worker_main::admit_observation(io, operation, ticket_sequence, request_deadline_ms,
  channels) sends fixed ObserveReady and validates ObservePermit, tightening the
  actual worker watchdog and returning a worker-local Instant deadline.
- worker_main::clock_origin supplies the real origin. FixedOutput writes only into
  the preallocated publication slice. publish(io, operation, slot, bytes) performs
  the existing Frame/Commit/matching ACK exchange; it grants no fake acknowledgement.

NativeExchange uses those primitives and the existing publication buffer for a
single HelperReply, enforcing correlation/Ticket/serial/channel/length and an ACK
before another native collection. Its dispatcher is compiled in the worker, but
parent ObserveReady/HelperRequest service and public live submission wiring are
still incomplete. New private Control kinds13..16 are not a public graph/CLI format.
This is a compiling internal handoff, not an already connected Native/Web producer.

Proposed finite Web owner write transfer (only after root assigns it):
src/worker_web.rs, src/web_config.rs, tests/web_worker.rs and its own
 tests/support/web_worker_* under crates/host. Core keeps worker_main/ops/
worker_observation, main/module hooks, Cargo, protocol, parent broker/lifecycle.
The current web_config provides explicit setup caps plus Initial/References selection;
these are private configuration, reusing canonical Id/Identity/BackendRef types.

Concrete module entrypoints for that consumer:

```text
WebSession::attach(raw_descriptor, setup, TargetLease, clock_id, origin,
                   HostLimits, deadline) -> Result<WebSession, HostError>
WebSession::observe(&mut self, &mut CanonicalSession, &mut WorkerIo,
                    &mut publication_buffer, Control, input, now)
                    -> Result<(), HostError>
```

Attach must use the existing mandatory logging filter before Transport/Client/
Collector construction, authorized endpoint/binding and actual worker clock. Observe
uses Tape only to segment Request plus explicit selection CONFIGURATION, never
pre-collected responses: real begin/admission precedes Collector::observe_initial
or observe, with actual Ticket.sequence. Callback moves the real Document, guarded
encoding and receive precede publish, and Acknowledged follows only full parent ACK.
No fabricated refs, parent graph decode, browser launch or live acceptance. Return any
missing bounded representation/retained-ref consumer rather than inventing a new
public graph or uncharged cache. Core wires the finished module after its concrete
source handoff; it does not add an absent module or dummy implementation.

## Connected Native parent exchange (finite-producer proof)

The earlier missing-parent-service note is now superseded for the Native path.
configure_native_helpers(session, NativeHelperBinding::authorized(executable,
channels, opaque_trusted_configuration)) installs a parent-owned executable/binding
while Attached; configuration fits control_bytes minus the64-byte header. Core does
not parse that Native configuration. Native's entrypoint owns actual target/process
incarnation/Surface/scope/redaction/artifact validation, using its grounded source.
submit_native_observe takes ONLY a canonical Request in InputLease, OutputRequest
and original parent deadline. No completed responses are supplied in advance.

Worker validates Request and calls real begin before ObserveReady. Parent binds its
Ticket, tightens the original deadline by the validated declared request budget
measured from parent submit, then sends ObservePermit with remaining duration.
The worker tightens its own watchdog/local deadline before acquisition. Parent
retains the original charged input slot through the operation and reuses it for
helper request writes; there is no second request Vec or Running reserve_input.
Parent helpers are selected only from its registered binding, never a worker path.
AX is received/validated and reaches real parent ACK before starting a capture
helper. A genuine AX transport refusal may leave AX missing while independent
capture proceeds. Failed canonical replies retain their Issue as committed responses;
overall Completed then means responses completed, not successful pixel capture.

Private worker/parent controls (all64-byte existing Control layout):

| Kind | slot/flags | value / auxiliary | Body |
| --- | --- | --- | --- |
| ObserveReady13 |0 / requested channel mask |actual Ticket.sequence / request deadline ms |none |
| ObservePermit14 |same |same Ticket / parent remaining ms |none |
| HelperRequest15 |channel0 AX or1 capture /0 |Ticket /0 |none |
| HelperReply16 |channel / status |helper serial (or0 if never spawned) / Ticket |one canonical JSON frame when status0 |

All retain session epoch/operation correlation and Observe class. Reply status0
is data;1 resource,2 deadline,3 generic failure,4 denied,5 cleanup pending. Failures
carry no body; they never become successful empty data. The worker may preserve
other completed channels. Whole-operation parent expiry stops dispatch/publication
and starts owned cleanup; previous ACKed frames stay outside that failure boundary.

Concrete Native entrypoint contract, for its separately assigned source owner:
parent directly spawns the trusted executable with empty env/no extra argv; FD3
input, FD4 output, FD5 private status;0/1/2 go to /dev/null. It sends Configure(kind1,
class Observe, selected channel, correlation, length=config bytes, value=Ticket,
auxiliary=reply cap INCLUDING LF), configuration bytes, then Submit(kind3, same
class/channel/correlation, length=original Request bytes, value=Ticket,
auxiliary=current remaining ms), then that unchanged Request. Input is length-framed,
not line-scanned; JSON whitespace is valid. Reply is exactly one UTF-8 NDJSON canonical
ChannelResponse plus LF. Parent admits LF too and strips only that delimiter before
guarded parsing. No shell/env fault modes, copied helper implementation or graph DTO.

The selected same-source Native adaptation must perform only the selected channel
and return one response; today's combined legacy helper is not compatible by mere
wrapping. Native retains its actual helper clock/provenance, real Target/Surface/
scope/permission/redaction checks and output-artifact owner. Core only validates
framing, correlation and budgets, then guarded schema/ObservationSession receive.
Taking this channel starts helper cleanup; capture/session reservations remain held
until actual reap. Source-level/current fixture proof is not SDK/live qualification.

Actual runtime proof uses native_peer, a finite Rust non-UI producer following this
contract. After an ordinary Validate operation, host operation2 produces real
Observation Ticket1; the helper asserts that exact Ticket. A forwarding process
wrapper observes AX ACK before spawning capture in valid-AX cases. Canonical capture
permission failure is retained, capture EOF preserves AX, wrong Ticket refuses before
publication, and a deliberately oversized NDJSON reply is bounded before parse/copy.
No capture, UI, browser or SDK call was performed. Native entrypoint adaptation,
Web module connection, remaining fault/phase proof and live/D06 gates remain open.

## Web shared caller hooks

With feature web, RuntimeHost::attach_web(AttachInput, deadline) consumes
Tape(canonical SessionDescriptor, trusted WebSetup) and waits for the actual guarded
WebSession::attach before Attached. Endpoint/configuration is trusted attachment
authority, not observation/UI text. The existing logging boundary precedes transport.
RuntimeHost::submit_web_observe(session, InputLease, OutputRequest, deadline) consumes
Tape(canonical Request, WebSelection). It routes the real guarded WebSession observe,
with actual begin/Ticket and the same parent ObserveReady/Permit deadline handshake;
no Native binding or pre-collected replies are required. Caller sets Request clock
from Attached. Defaults do not enable Web, and no source/provider code is duplicated.
These shared hooks compile; Web's owned peer/runtime proof and later browser/live
qualification remain separate. The module/config/tests belong to the Web owner.

## Parent inventory map and finite allocation observation

D05's managed parent inventory is the explicit owner graph below. It is not a
measurement-derived multiplier or a cap on the containing application's RSS.

| Owner | Accounted storage / lifetime |
| --- | --- |
| HostDomain | Inline Vec handle once + actual Vec<DomainInner> backing capacity; retained on uncertain teardown |
| DomainInner | Inline limits/platform identity, real fixed QuotaLedger, ParentBuffers headers, four slot/target records, epochs/effect lanes/helper serial/capture claim/reaping state |
| ParentBuffers | Fixed36 Vec headers and8 group cells inside DomainInner; actual configured input/ingress/output Vec capacities added once, including vacant backing |
| RuntimeHost | Inline domain reference/Vec handle once + actual Vec<RuntimeState> backing capacity |
| RuntimeState | Stateless Darwin provider, fixed SpawnSpec, four Worker records, shutdown state, four fixed native bindings and RuntimeRoot charge |
| Worker and Helper records | All fixed config/control/fatal headers, offsets, deadlines, correlated handles, direct Darwin child/FD owners, two helper owners and bounded NativeBroker state inline in RuntimeState |
| Native bindings | Fixed executable/configuration arrays inline, even while vacant; no argv/env/JSON owner grows during dispatch |
| Input/ingress/result leases | Borrow/move existing pool backing. Helper raw bytes and published canonical bytes occupy distinct already charged slots; returned leases keep their owners live |

The ledger's192-byte root appears in both parent layout accounting and the distinct
retained-reservation metric; those metrics must not be summed as independent memory.
Static process ownership flags have fixed binary storage with no data-dependent
backing. The allocation observation does not measure binary/static image, transient
stack beyond reported inline roots, System-internal rounding/bookkeeping, C/OS spawn
or socket storage, helper/SDK/pixel memory, or another caller's allocations.

A separate test executable installs a transparent System-forwarding allocation
OBSERVER, never the production quota guard. Fixed atomics count only the selected
parent test thread; worker processes retain their real independent GlobalAlloc.
An observable64-byte positive control confirms the observer detects real storage.
On the declared two-worker/two-completion-group profile, setup reports9,497,752 bytes:
9,437,184 payload backing +60,512 heap control backing +56 inline root bytes. The
observer independently saw14 requests totalling9,497,696 bytes, exactly matching
reported inventory after adding those inline roots.

With setup complete, attach/clock framing, a normal request/ACK/release, actual
Native private exchange, a held completion while B advances, saturation refusal
and confirmed shutdown each produced zero additional Rust allocation requests on
that parent thread. Source ownership establishes the bound; this finite runtime
observation tests those named paths only. It does not prove every future path or
close platform/live/SDK/D06 gates. The independent producer review419b75c accepted
its scoped source connection and RuntimeState/native-binding precharge participation;
it was not a standalone complete inventory/steady-state/runtime H01 verdict.

## Actual parent-death watchdog repair

A disposable supervisor now exercises the real guarded Web worker against the
existing owned CDP peer. The peer withholds Runtime.callFunctionOn after real
admission, leaving the operation thread in its network wait rather than reading
parent input. The outer test SIGKILLs only its owned supervisor Child; RuntimeHost
Drop cannot run. Read-only pgrep scoped to that live parent establishes its sole
worker, and exact ps PID/state queries observe worker exit. No numeric worker PID
is signalled. Zombie is distinguished from running state; orphan reap is the OS's
responsibility, not a claimed waitpid by this observer. Query subprocesses and peer
teardown have explicit bounds and owned cleanup.

The original test failed: worker remained live in state S until the CDP peer was
closed during test teardown. A separate owned socketpair reproduced Darwin behavior:
poll(events=0) after peer closure returned0/revents0; poll(POLLIN) returned readable
and HUP (17). This observation established the empty-interest-mask defect in the
watchdog's existing parent_alive path. The minimal repair requests POLLIN while
retaining the same bounded poll and HUP/ERR checks. It never reads or consumes
protocol bytes, introduces no new syscall/API/dependency, and leaves deadlines,
limits and FD/protocol ownership unchanged.

The repaired actual-death case passes while the CDP peer remains open. Its polling
loop has a1s deadline, but a bounded process query may finish after that deadline;
the explicit elapsed assertion is<2s from observing the stalled peer, before the
peer3s/request10s timeouts. No precise measured exit latency is claimed. Existing reusable-worker,
Native begin/helper/ACK/cancel/deadline and real guarded Web initial/reference
flows pass the affected checks. This is narrow actual supervisor-death/liveness
proof, not SDK/browser/live or whole-process RSS qualification.

## Pinned publication allowance closure

The current publication closure is finite: core0.1 Document/borrowed Snapshot,
analysis0.2 Measurement/GeometryCheck, canonical Web ChannelResponse, and validated
Native/legacy bytes. It is not an arbitrary user-defined Serialize surface.

| Source owner | Obligation and bounded storage | Evidence / boundary |
| --- | --- | --- |
| worker_main input/publication setup |512KiB reusable output backing is allocated under ordinary quota before decode | Existing installed-guard proof; terminal publication probe also charges this backing explicitly, separately from reserve |
| model/analysis Serialize derives + serde_core | Records, struct tags, adjacent variants, strings, Vec/arrays/Option/Box traverse borrowed data; no flatten/custom Serialize/map-key error or Content-buffering variant | Pinned type/derive/primitive source closure; proc-macro construction Vecs are compiler allocations, not runtime scratch |
| serde_json compact Serializer | Stack Compound state; string slices and at most six-byte escape fragments; itoa40/zmij24-byte stack buffers | Actual std/alloc/float_roundtrip feature graph, no arbitrary_precision/raw_value; success modes observe zero scratch heap |
| FixedOutput error → serde_json::Error::io | Slice overflow returns a simple ErrorKind; serializer owns one boxed ErrorImpl40 bytes on pinned arm64; propagation does not format UI text or make one box per ancestor | Source trace + exact predeclared IO-refusal measurement: one40-byte peak, then live scratch0 before reserve exit |
| guarded_encode / Web callback | PublicationGuard encloses encoding/error lifetime; Web drops original Document before ordinary receive, then separately guards publication | No decoder/normalizer scratch is silently charged to publication; those remain ordinary working allocations |
| Native/legacy copy + publish / WorkerIo | Existing bounded bytes; stack64-byte controls, concrete File IO and matching ACK, no graph/Vec clone | Source trace and real publish/Publication ACK; healthy and ACK-EOF probe paths observe zero scratch |
| guard/fatal/terminal | Real precharge and publication ownership remain enforced; failure uses fixed status/control, not formatted payload | Accepted guard source/null/fatal evidence reused; probe verifies full release, no reserve escape and actual owned-child reap |

With ordinary already fully occupied, the terminal probe charges524288 output bytes
plus4096 ballast, then permits the unchanged final1048576 reserve. Canonical success
(including60KB raw mixed escapes), analysis result variants and Failed ChannelResponse
use zero additional heap; writer IO error peaks at40 bytes; ACK EOF uses zero and
commits nothing. Success encoding sizes were2762/142750/6076/9502/367 bytes. Error
prefix16 bytes was never published. All scratch frees before PublicationGuard exits;
backing/ballast then free and actual guard reports live0. Seven modes were declared
before execution, with no fitted limits. This terminal window deliberately excludes
pre-existing System-owned DTOs: it closes publication scratch, not whole-worker peak.

For these pinned types/features/writer/IO owners, successful scratch is input-size
independent and the reachable serializer IO allocation is fixed40 bytes. Thus the
initial1MiB publication allowance covers the named path even when ordinary capacity
is full; existing earlier-ACK/failure/parent ownership proof supplies the integrated
lifetime boundary. A smaller explicit reserve can still truthfully refuse; no new
minimum/default is implied. New custom serializers, buffering features, arbitrary
writers, dependency revisions or output paths require affected requalification.
This is author source/runtime closure submitted for scoped acceptance, not a waiver
of ordinary parser/SDK memory or live/D06 requirements.
