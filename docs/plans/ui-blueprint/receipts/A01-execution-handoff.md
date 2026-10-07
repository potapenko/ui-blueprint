# A01 executor handoff receipt

Authority: [finite packet](../packets/A01-execution-handoff.md), approved A01/P5.
Read-only source task with exactly two documentation deliverables:
docs/research/A01-executor.md and this receipt, inside existing directories.

Traversal: current instructions → specification registry/product/reference routes
→ ACTIONS/IDENTITY/FORMS/LIFECYCLE/EXCHANGE/MODEL/PRIVACY/CACHE and D02/D04/D05
explicit closure, EXECUTOR-SOURCES@1 → REUSE; RUST/QA where relevant. Existing
current full clauses reused; required executor leaf read fully before source.
Mode Discover implementation gap under unchanged contracts, not new product DSL.

Actual primary-source scope: pinned Ui.Vision command vocabulary, runner fallback/
delivery/verification branches, player states, timer and its inspector interface,
LICENSE.txt and package.json. In-memory web/local HTTP reads only, explicit10s
network timeout where used. No clone/install/execute/copy or filesystem capture.
AGPL/commercial versus ISC discrepancy preserved as behavioral-reference limit.
[Source ledger](../../../research/A01-executor.md) links each exact pinned file and
disposition. Desktop picker source excluded because no region selection is required.

Actual local scope: schema action/result/transition types and validators; plugin-api
ObservationSession; host worker mutation refusal, parent EffectReady/Permit/nonce
and claims; nearest existing fake-effect/golden cases. No broad owner audit or QA run.

Concrete next candidate: one typed SetChecked(bool) step, current resolution before
existing one-use permit, delivery distinct from checked-state verification, canonical
TransitionCase success/partial/unknown result. Core lifecycle likely belongs to a
narrow plugin-api action owner; guarded mutation integration needs its actual worker
IO consumer. Fresh-resolution/provider interface is missing and must not be replaced
by trusting saved Current/unique flags. Real UI/backend/permission gates remain open.

Single-step fits the existing one nonce/Mutation operation. Canonical scenario plan
and multi-step durable journal are absent; a fake endpoint/confirmed nonce is not
proof of user result or that completed steps survive subsequent worker loss. Root
should assign this finite core outcome before expanding actions or protocol. No
heuristic ref repair, fallback, secret handling, automatic retry or rollback claim.

Independent expected cases and platform dependencies are in the source handoff.
No source/schema/spec/fixture/Cargo mutation, browser/native input, timer/run logs,
new persistent/temporary directory or extra agent created. Documentation-only
validation: local links, route consistency and git diff --check; no builds/tests
claimed. Scoped checkpoint/push follows root's short exact2 Git lease.

## Follow-on single-step implementation

Root selected the source proposal in the appended [same packet grant](../packets/A01-execution-handoff.md#follow-on-a01-single-step-implementation-grant)
under unchanged ACTIONS/GOLDEN01/ROADMAP authority. Exact4 writes:
crates/plugin-api/src/actions.rs (existing src directory), src/lib.rs export only,
crates/plugin-api/tests/actions.rs (existing tests directory), this receipt.
No schema/wire/host/Cargo/CLI/platform/fixture or other owner's source edits.
Current registry15 and full packet-pinned closure reused; existing source ledger
c3241ed supplies reference evidence, not new product intent. No upstream copy.

Actual reusable API: SetCheckedExecution::prepare(ActionCase, transition_id,
step_id, ClockReading, remaining_ms) validates canonical declarations, then
dispatch(provider,gate,control), verify(provider,control), cancel(now), finish().
finish returns the same existing validated TransitionCase owner; no new graph/wire.
step()/issue() expose the canonical Step and optional bounded Issue. Pending
preparation/verification cannot finish as success; repeated dispatch/late callbacks
refuse using existing plugin-api Error. There is no plan/loop/retry/secret resolver.

Trusted ports, actual compiling signatures:

- SetCheckedProvider::resolve_exact(&ActionCase,&ClockReading,remaining_ms)
  ->Result<ActionCase,Issue>; returns fresh canonical facts for the SAME independently
  held Target/Surface/scope/backend key. Observation/Snapshot refs may refresh for
  that exact validated identity, never same-label or vanished-object rebinding.
- deliver(&Action,DeliveryPermit,remaining_ms)->DeliveryStatus; move-only permit
  owns an existing nonzero parent nonce. Already-equal Checked does not bypass
  this one delivery. Setter modality/held-object continuity are provider obligations;
  acceptance alone cannot justify Succeeded or a business/application effect.
- observe_after(&Action,&ClockReading,remaining_ms)->Result<Snapshot,Issue>;
  actual fresh target observation, not saved Current metadata. Kernel requires
  matching Context/ref Surface/key, current Checked Evidence/coverage and confirmed
  delivery before Succeeded. Missing/mismatch/changed binding cannot pass.
- EffectGate::authorize(&Action,&ClockReading)->Result<DeliveryPermit,GateFailure>;
  GateFailure has existing canonical Issue and effect_possible. If boundary crossed
  before I/O loss, uncertainty cannot become NotDispatched. from_parent_nonce
  validates nonzero but mints no nonce/authority; only trusted host integration calls it.
- ActionControl::now()->ClockReading and cancelled()->bool supply actual local
  monotonic control. Checks surround resolver/gate/delivery/verification; clock
  regression/domain mismatch and deadlines fail closed. Blocking OS calls still
  require the actual parent's watchdog/helper termination/confirmed reap.

Kernel uses existing ActionCase/Action/Resolution/Step/TransitionCase and validators.
It calls fresh resolution before requesting effect authority, compares exact selected
identity/intent/modality, then requests the real gate once. Provider permission facts
can change; unavailable/ambiguous/stale facts refuse before permit. Gate authority
comes from trusted caller, never UI data. Provider is independently pinned to its
allowed attachment before read. After Possible, cancelled/lost/uncertain delivery
retains ActionOutcomeUnknown; confirmed executed delivery survives in completed_steps
when verification fails or is cancelled. No later callback resumes that operation.
Known false verification produces measured Fail, without a new mismatch error code.
Accepted-but-unconfirmed delivery with matching state still cannot report success.

7 focused tests passed on Rust1.96/aarch64-apple-darwin, Cargo --locked --offline
using a unique system-temp target for this operation:

- test -p uiblueprint-plugin-api --test actions:7 passed,0 failed. Existing GOLDEN
  action/readonly/transition data, exact checked success and already-equal, initial
  and freshly changed readonly/stale/ambiguous/disabled/unwritable/value-denied/
  unsupported intent; valid same-label alternate object cannot receive input;
  accepted/missing/unknown/mismatch/wrong-binding/unverified after cannot pass;
  before/after-Possible cancellation, deadlines, gate loss, one delivery, zero nonce,
  late verification refusal and invalid clock/pending finish. Every emitted
  TransitionCase passed the existing canonical validator with exact links/outcomes.
- check -p uiblueprint-plugin-api --all-targets: passed, protects existing owners.
- clippy -p uiblueprint-plugin-api --lib --test actions -- -D warnings: passed.
- Scoped format/whitespace/local links passed. No broad suite or UI/source action.

Test-only fake ports supply typed canonical facts/static parent nonce, not actual
browser/SDK authority or dispatch; no duplicate production nonce/claims machinery.
Source dimensions and delivery/verification are distinct. Ordinary input/DTO/finish
validation overlap is caller/worker-guard-owned, no allocator-free/RSS proof claimed.
The task temp contains only non-image build files and is removed/absence-verified
after use; no image generation/output occurred. All image paths stay outside cleanup.

Exact next host dependency: Mutation execution in worker_ops/worker action consumer
must call this kernel with actual platform provider and a real WorkerIo EffectReady/
EffectPermit adapter. Adapter verifies class/epoch/operation/slot/flags/nonce and
remaining deadline, consumes nonce once, reports Possible conservatively on transport
loss and uses the existing parent TargetLease/same-target/physical claims. Parent
must retain its authoritative receipt across worker death; this pure kernel cannot
guarantee that itself or preserve typed steps after a crash without committed output.
Issue may be captured before finish for bounded diagnostics; canonical transition
publication/ACK and terminal effect semantics belong to that next actual consumer.
No live action or general A01/K02 acceptance granted by these library checks.

Root additionally granted exact Web internal dependency edge needed by its actual
trait consumer: plugins/web/Cargo.toml path ../../crates/plugin-api and corresponding
uiblueprint-web dependency entry in Cargo.lock. No package/version/feature/root
manifest change or update/network resolution. Locked/offline no-deps cargo metadata
confirmed that exact internal edge without compiling Web source. Web owns affected
consumer compilation; moving provider source was not broadly compiled by Core.
Final checkpoint6 set is the4 implementation paths above plus those2 edge files.
API/source SHA25608a271f311ce3de0abaa2d31c05f7225f6d8694ccd7d296d9287cef51c70bccc;
lib.rs294da1c5df850ac890bfb55d109e6f8e8c11b702b33edb0bb9e156615bd472ac;
action testsbb0947ec285c56b92ba19e4ae284747d0eb70ba00dd1acbb03fe6eb39f4a0d27.

## Guarded composition — coherent WIP checkpoint

Authority: [next finite host packet](../packets/A01-guarded-composition.md) plus
its selected Tape/refusal/Prepare amendments. Kernelffe1166 remains unchanged.
Actual Web preparation source is saved1a8a6023249fcbabd6921b2390efe4782aef085c,
action.rs d2b3dca7c21cccb5a6e919458384c7e089b514203e12d3a81dd7cd42b3f80089.
No live activation. Source work is a real composed path, with remaining checks
explicit below; this checkpoint is not acceptance or a replacement SDK backend.

Exact15 host checkpoint paths: src/main.rs, protocol.rs, publication.rs,
supervisor.rs, worker_main.rs, worker_ops.rs, worker_web.rs, worker_action.rs,
worker_effect.rs; tests/web_worker.rs; tests/support/{effect_peer,effects_host,
web_worker_peer,web_worker_process}.rs; this receipt. No Web collector/kernel/schema/
Cargo/Native/CLI/fixture or live harness edits. Web live files stay separately owned.

WorkerEffectGate uses real WorkerIo with existing EffectReady/EffectPermit and
trusted TargetLease/clock/deadline. It checks class/epoch/operation/slot/flags/length/
nonzero nonce, permits one request, preserves uncertainty after request transport
loss, and polls actual parent EOF/cancel before dispatch. No nonce mint/claims copy.
Caller passes the same local origin and parent-clamped watchdog deadline. Separate
worker_action owns canonical kernel/output composition; no serialized new owner.

Private OperationClass::Prepare=10 maps existing Tape(Snapshot, Prepare Request)
to actual CheckboxProvider::prepare_exact. Request already carries ref and bool in
action.backend_ref/intent; unresolved Resolution remains unknown until actual fresh
native probe creates a validated ActionCase. It uses no MutationLease/EffectReady/
physical claim/setter. Later Mutation maps Tape(ActionCase, Act Request), requires
matching existing action/request fields, actual Attached clock/session/Target and
provider scope/generation binding. Root generic submit uses input_format1/channel1;
no new public wire or CLI. Tighter Request deadline and output cap are enforced
alongside existing aggregate2MiB input/control/publication caps.

Mutation pre-Possible Frame.flags1 is only refusal marker. Parent's one inline
refusal_started bit permanently forbids EffectReady and Completed/nonce terminal
for that operation; normal pre-permit Frame remains forbidden. Matching marker
through Frame/Commit/ACK is enforced. Post-Possible refusal marker rejects. Existing
fixed RuntimeState size accounting includes the bit; pools/ceilings unchanged.
Observe incomplete flags retain their original meaning; Mutation refusal does not
add a false incomplete-channel classification. Worker publishes validated existing
Error/Transition refusal and failure terminal; parent never parses its body.

After actual permit, one SetChecked delivery and separate verification produce
validated TransitionCase through existing buffer/reserve/commit/ACK. Terminal nonce
confirms reported delivery only; canonical Failed/Unknown verification is not UI
success. Pending W03 invalidation is applied after provider borrow ends, on success
and failure. No typed-step survival claim before outside-worker commit.

Targeted checks passed with locked/offline Cargo in the operation's system-temp
target directory, Rust1.96/aarch64-apple-darwin:
- actual default effect_peer build now uses the production WorkerIo/effect gate;
- web host bin/example check compiled real saved preparation/provider;
- web_worker exact guarded_prepare_then_act_uses_actual_kernel_bridge_and_preserves_non_success_outcomes:
  4 cases passed: real guarded Observe→unknown-seed Prepare (zero effect permits/
  setters)→Act, fresh capability success/already-equal once, later disabled refusal
  before permit, and known Checked mismatch without Succeeded. Real ACK counters,
  original earlier bytes and sessions0/groups0/abandonedfalse verified.
- runtime exact effects::marked_pre_dispatch_refusal_cannot_gain_effect_authority_or_false_success:
  7 cases passed: marked canonical refusal, prohibited Completed/nonce/EffectReady,
  mismatched marker, ordinary unmarked pre-permit Frame, post-Possible marker;
  refusal latch resets only on new operation, prior bytes survive, actual reap.
- affected web lib/bin/web_worker/runtime/effect_peer Clippy -D warnings passed.
First composition test held3 completions against the existing2-group test cap;
released only prepared lease before Act, retaining old Observe lease, then passed.
No cap increase or source-budget relaxation.

Test backend replies are bounded synthetic protocol responses, not browser/SDK
dispatch. Existing effect_peer's local fake counter remains test-only; it reuses
the real fixed I/O gate rather than duplicating nonce authority. No images or new
persistent directory. Owned non-image system-temp build remains only for immediate
remaining checks in this same operation, then will be removed/absence-verified.

Remaining before author completion: further Prepare class/payload/budget/cancel
edges, mismatched permit correlation/nonce, and exact before/after-Possible failure
checks. Independent changed-boundary review and live harness activation remain
root/platform-owned. No full A01/P5 acceptance claimed by this WIP checkpoint.
web_worker_process.rs current Core hunk is actual permit/ACK counters and a held
effect-permit barrier; Web receives its ownership only after this saved handoff.

## Remaining planned edge evidence after80b7449

Production checkpoint80b7449dca32b9d9853d8aaaf353d26be69e9d4b remains unchanged.
Core then wrote only tests/web_worker.rs and support/effects_host.rs plus this
receipt. web_worker_process.rs is now Web-owned; no interleaved writes made there.
In the same locked/offline temporary target, targeted cases passed:

- prepare_rejects_payload_ref_clock_and_small_budget_without_effect_authority:
  6 cases. Read-only TargetLease, Act payload under Prepare, wrong clock/ref,
  reversed Tape,64-byte Request output cap and pre-source cancellation; no
  EffectPermit/setter, canonical refusal when admitted, empty output on budget/cancel,
  invalid seeds refused before SDK queries, sessions0/groups0/abandonedfalse.
- actual_worker_effect_bridge_refuses_corrupt_permit_and_parent_rejects_wrong_nonce:
  3 actual fixed-I/O faults. Wrong permit correlation/zero nonce cannot create
  delivery token; mismatched positive nonce cannot confirm terminal success.
  Parent preserves Possible/unknown and earlier complete ACK bytes; actual reap.
  Fault wrapper buffers only the altered64-byte header and handles partial writes;
  it is test-only and does not manufacture an accepted permit or new claims.
- actual_action_cancel_before_ready_and_after_possible_never_dispatches_setter:
  2 actual kernel/provider paths. Before Ready yields NotDispatched; after parent
  queued EffectPermit yields Possible/unknown, no setter/ACK, confirmed owned reap.
- Affected runtime/effect_peer and web_worker Clippy passed. New wrapper's nested
  if was collapsed directly; no logic/threshold change or unchanged suite repeated.

Additional required source discrepancy found: Prepare/Act currently tighten only
the child watchdog to canonical Request duration. Parent Active deadline remains
the explicit caller deadline; a shorter canonical duration has no fixed-control
parent admission yet. Late ACK/terminal suppression against that tighter duration
therefore relies on child scheduling. Before author completion, Core returned the
minimum dependency: reuse fixed admission kinds13/14 for typed Prepare/Mutation,
matching class/epoch/op/mask and clamping parent started+duration before any SDK/
effect request; no EffectPermit/authority gained, Observe ticket semantics unchanged.
This source repair awaits explicit selection; no production source changed on the
strength of the proposal. Planned edge coverage is complete, but this newly exposed
canonical-deadline guarantee and independent/live acceptance remain open.

Exact proposed repair, not yet selected: existing control kinds13/14 retain their
encoding. For Prepare/Mutation, after strict typed Tape/Request binding validation,
worker sends kind13 with matching operation class, slot0, flags=request channel1,
epoch/op correlation, length0, value=parent operation sequence (not an Observation
Ticket), auxiliary=canonical deadline_ms. Parent permits this only for current
non-live Prepare/Mutation before refusal/Possible/publication and once, clamps
active.deadline=min(caller deadline,active.started+duration), then returns kind14
with identical correlation/class/mask/value and remaining duration. Worker checks
every fixed field/nonzero remaining and tightens its existing watchdog before SDK.
Observe live path keeps its real Observation Ticket rules. This admission gives no
nonce/EffectPermit and cannot bypass mutation permission/claims/refusal latch.
Exact owners: supervisor.rs kind13/14 admission branch; worker_action.rs or
worker_main.rs narrow helper; worker_web.rs after valid request before preparation/
kernel; nearest existing held-control/late-ACK tests plus this receipt. No new
control/framing/graph/public schema/CLI/pools or generalized status framework.

## Selected parent canonical-deadline repair

After verification-only2f5c5eb, root selected the preceding fixed-control proposal
in [the same packet](../packets/A01-guarded-composition.md#required-authoritative-canonical-deadline-clamp).
Exact6 changed paths: host src/supervisor.rs, worker_main.rs, worker_web.rs;
host tests/support/effect_peer.rs, effects_host.rs; this receipt. No Web helper,
kernel/protocol enum/schema/CLI/Cargo or quota changes. Previously admitted typed
Prepare/Mutation header classes and original Observe Ticket remain distinct.

Worker_main::admit_action sends existing kind13 after valid Action/Prepare Request
binding and before any provider SDK/gate. Cookie is current operation sequence;
class/channel1/correlation/length/nonzero duration checked. Parent admits only current
typed non-live Prepare/Mutation before refusal/Possible/publication and once. It
sets min(existing deadline,active.started+duration), computes remaining at current
time and returns kind14. No root authority/nonce/helper/capture/physical permission
is conferred. EffectReady on typed Mutation before admission now refuses; legacy
non-typed nonce peer path is compatible. Child validates reply then tightens the
existing watchdog. Invalid/unparseable binding keeps outer bound and refusal path.

Source and meaningful timing evidence, all Cargo locked/offline in owned system temp:
- effects::typed_action_admission_clamps_parent_from_start_and_refuses_late_ack_terminal_or_effect:
  11 actual parent/owned-peer cases passed. Fake peer has no child watchdog, so
  shorter200ms versus outer2s conclusively exercises parent rejection of300ms-late
  payload/terminal, not child scheduling. A held ACK never commits received data
  after parent bound; prior ACK survives late terminal. Mutation after Possible
  times out unknown. Longer2s duration cannot extend outer200ms.300ms pre-admission
  delay leaves≤1700ms of2s, proving start anchor instead of now+duration. Already-
  expired/zero/wrong mask/wrong operation cookie/duplicate admission rejects with
  no effects; all cases actual reap/sessions0/groups0/abandonedfalse.
- Exact guarded_prepare_then_act_uses_actual_kernel_bridge_and_preserves_non_success_outcomes:
  passed against real production worker/kernel/saved Web preparation with admission.
- Exact real_begin_permit_ack_and_reusable_first_and_reference_requests: passed,
  preserving admitted Observe Ticket/helper publication semantics.
- Affected web host lib/bin/runtime/web_worker/effect_peer Clippy -D warnings passed;
  bin check and scoped formatting/whitespace/local links passed.

No unchanged broad suite or live action ran. Additional SDK/cleanup/pool rights are
not introduced by control13/14. Parent continues rejecting late correlation/ACK/
terminal against authoritative deadline. This closes the discovered source guarantee
gap; protected review and actual live action acceptance remain root-owned. The fake
timing/corruption wrapper is fixed64-byte test storage; new no-fault modes explicitly
leave existing EffectPermit unmodified. No operator/user/source timestamps restamped.
Owned temp target contains no images and is cleaned after final use/absence verified;
all actual images/containing directories excluded. Save/push repair separately.

Repair source SHA256 pins:
- supervisor.rs:35f961876ffb61b09a856300631ceb1654377185eac7b0c2863693942aa2343e
- worker_main.rs:6b30b8a5119d9169e4f8241f2560cc15fd63d29e7ac31bca24dcdc3ee6b1fbb5
- worker_web.rs:7bf49b5c2586506d7574338dcd5f82673ccbd3e457fdd3a97ab82415dba3382f
- effect_peer.rs:4a3509641fad60b4d2929d0214a8437e2d581960660d6a124bacb421741d99a8
- effects_host.rs:a98c0ceeef0c936d998d4f7c13d12ae07e522aef2c2bf4619c768bce73432e6c

## First action CLI registration — L01-ACTIONS-001

Selected authority: [L01-actions-contract packet](../packets/L01-actions-contract.md),
approved PLAN.UIB@1 P5/P6 under ROADMAP. Mode Evolve the unreleased CLI caller;
no schema/action/kernel meaning change. Exact5 documentation writes: new
product/cli-actions.md, product/cli.md metadata/conditional route, product/README.md,
specs/README.md registration, this existing receipt. No source/Cargo/runtime edits.

Traversal reused current instructions → registry15 → product route → CLI@5
CONTENT/OBSERVE → EXCHANGE/PRIVACY/MODEL/IDENTITY/BOUNDARIES@1; ACTIONS/FORMS/
LIFECYCLE@1 and current pinned A01 D02/D04/D05 closure; feature template read fully.
Root's selected packet supplies syntax/authority/exits, not source inference.
Registered [CLI-ACTIONS@1](../../../specs/product/cli-actions.md) SCOPE/INPUT/
AUTHORITY/OUTPUT/EXITS/ACCEPTANCE, CLI@6 and registry16. Original approval remains
358c757/task registry. Existing inspect/observe/diff/core0.1/analysis0.2/connection1.0.0
are protected; scenario/form/Native/multi-step capabilities remain future goal work.

Immediate caller uses existing Connection/Profile and attach/event/rebind/shutdown
owners in crates/cli/src/observe.rs, bounded read in input.rs, writer/publication in
output.rs, arguments.rs/main.rs dispatch. The existing private supported module
needs a narrow reuse extraction for action callers, not a copied connection parser
or second host. Prepare accepts Snapshot or observed ChannelResponse plus Prepare
Request; guarded worker validates the envelope/extracts unchanged embedded Snapshot,
and failed/no-snapshot response refuses. This direct Observe→Prepare input is root's
explicit registration-review correction. Execute uses ActionCase plus Act Request.
Rebind only Request.clock_domain after actual
Attached; preserve saved evidence and independently revalidate exact live binding.
Explicit Execute selects trusted mutation request only for the validated connection
target under policy; Prepare remains read-only. No additional force/permission flag.

Observed frozen baseline c3967ca: worker_action.rs publishes validated TransitionCase
then returns nonce for Confirmed delivery even when verification is Failed/Unknown.
HostCompletion lacks verification metadata; incomplete_channels is Observe-only.
Parent cannot distinguish those outcomes from verified success today. Worker/provider
source and Web live qualification are unchanged by this registration.

### Selected private header encoding — not implemented

Reuse existing Frame/Commit/ACK flags byte; no new kind/header size/public version.
Keep flags0 legacy opaque/ordinary and flags1 pre-Possible Mutation refusal exactly.
Root selected this concrete proposal in the appended packet review. These are exact
class-specific enum values, not combinable bit flags. For typed action routes only,
flags2 = Prepared for Prepare, VerifiedSuccess
for Mutation; flags3 = verified Checked mismatch for Mutation; flags4 = uncertain
Mutation result. Other class/flag combinations reject. Prepare Error keeps ordinary
flags0 plus failure terminal; Mutation refusal keeps flags1 and closes effect lane.
Mutation2/3/4 requires existing Possible receipt before frame; none mints/confirms
nonce or permits post-refusal dispatch. Legacy/untyped flags0 gives no verified tag.

Producer derives2 only from validated prepared ActionCase or finished Succeeded
with Confirmed delivery;3 requires actual verify() Ok(CheckStatus::Fail), confirmed
delivery and matching finished Failed case, not arbitrary Outcome::Failed;4 covers
remaining post-Possible non-success. Capture existing verify return before finish;
no kernel signature/state graph change. Parent validates identical class/correlation/
slot/length/flags on Frame/Commit/ACK, saves a fixed inline status only after ACK,
and retains it with original committed bytes. Missing/late/corrupt status never0.
Execute0 additionally needs existing nonce-confirmed terminal and real cleanup;
tag alone is insufficient. Without ACKed verified evidence, lost terminal/post-
Possible uncertainty remains4 unless IO/internal/unconfirmed cleanup1 applies;
an ACKed known mismatch remains distinct in the report and selected exit mapping.

Exact selected host owners: worker_action.rs producer/classifier; worker_main.rs
existing publish flags parameter; publication.rs strict envelope/ACK-held status;
supervisor.rs typed action/effect/refusal class guard; host_types.rs status accessor.
Changing the existing bool publish parameter to flags requires mechanical unchanged
Observe conversions at worker_native.rs:115 and worker_web.rs:334; obtain sequential
owner handoff before those callsite writes, not a new wrapper/duplicate publisher.
Account added fixed inline status in actual D05 control inventory; no pool/cap growth.
Existing controls13/14 deadline, refusal latch, Observe flags1, effect one-use and
capture/helper/physical rights remain protected. Encoding selection does not grant
source edits in this docs-only step; exact host/CLI implementation packet follows
save. Public contract requires outcome fidelity, not these private numbers.

Documentation checks only: changed local links/anchors, routing/revisions, node
line ceilings and git diff --check. CLI/source/runtime/metadata acceptance has not
run and is not claimed; required scenarios are explicit pending in the new leaf.
No image/task temp/persistent directory or resource created. Scoped checkpoint/push
follows separate exact5 Git lease; actual Web live runs independently on c3967ca.

## L01 action implementation — restart WIP, drain 2026-10-08

Authority: [current finite implementation packet](../packets/L01-actions-implementation.md),
including root's narrow worker_ops.rs::prepare_input grant. Selected CLI-ACTIONS@1 /
CLI@6 registered at c4bc255; root reported registry17's Native-only acquisition@2
delta as unrelated to these unchanged common contracts. Inherit/master/single chat;
no nested worker, new goal, branch, Cargo/dependency/schema or collector edits.
Root explicitly requested drain to a coherent saved WIP before Codex restart:
finish current bounded checks, checkpoint/push, then wait for explicit root resume.
This is not author/independent/live acceptance of the complete CLI packet.

Exact20 checkpoint paths:
- crates/cli/src/action.rs, connection.rs, arguments.rs, main.rs, observe.rs, output.rs;
- crates/cli/tests/action_binary.rs;
- crates/host/src/host_types.rs, publication.rs, supervisor.rs, worker_action.rs,
  worker_main.rs, worker_native.rs, worker_ops.rs, worker_web.rs;
- crates/host/tests/publication.rs, web_worker.rs;
- crates/host/tests/support/effect_peer.rs, web_worker_peer.rs;
- this existing receipt.
Other Native/platform/root changes remain outside index/checkpoint.

Implemented WIP: exact class-dependent flags0/1/2/3/4; fixed inline
ActionPublicationStatus retained with matching ACKed bytes, no parent graph parse.
Prepare2 is Prepared; Mutation2 VerifiedSuccess,3 VerifiedMismatch,4 Uncertain;
Mutation1 retains pre-Possible refusal/lane closure. Producer3 requires actual
verify() Fail plus Confirmed delivery and finished Failed case; arbitrary failure
cannot become a mismatch. Typed2/3/4 require prior admission/current class/state;
Mutation still requires Possible and real nonce. Legacy0 has no verified tag.
Parent's existing sizeof RuntimeState inventory includes the publication status;
no pools, quotas, effect, helper/capture/physical rights or deadline were widened.

Guarded prepare_input now accepts canonical Snapshot OR observed ChannelResponse,
moves out its unchanged embedded Snapshot after full canonical validation, and
refuses Failed/wrong input without SDK/effect. No cloning graph or restamping.
Existing publisher bool calls became exact flags0/1; Native/Web Observe semantics
preserved, no Swift or Web collector changes. Kernel/provider signatures frozen.

Public prepare/execute argument parsing/help and bounded compact/JSON callers exist.
Connection/Profile/IO helpers moved into one shared connection.rs, preserving
Observe behavior; one strict loader now serves both. Source bytes stay opaque in
CLI; only existing canonical Request clock is rebound after actual Attached.
Explicit Execute independently selects exact trusted TargetLease mutation request;
Prepare does not. JSON writes original ACKed document/newline; compact is fixed
preparation/delivery/verification/completeness/protocol/cleanup status, no UI text.
Completion success requires semantic tag plus ACK/terminal/delivery/real cleanup.

Checks actually passed, Cargo locked/offline, Rust1.96/aarch64-apple-darwin,
unique system-temp target; no live browser/app or new runtime proof:
- web host compilation and affected lib/bin/publication/runtime/web_worker/
  effect_peer Clippy -D warnings; CLI web bin/action_binary Clippy -D warnings;
- CLI default, macos and web checks; defaults remain unchanged;
- publication test4 passed, including exact class/status combinations, mismatched
  Commit/ACK, no status before ACK/after terminal, retained bytes and Observe flags;
- actual guarded_prepare_then_act... passed5 cases: two verified successes,
  fresh disabled refusal, known Checked mismatch, confirmed delivery with lost
  post-state binding unknown. Direct observed ChannelResponse and Snapshot inputs,
  typed status, one permit/setter where allowed, old bytes and owned reap verified;
- prepare_rejects_payload_ref_clock_and_small_budget... passed7 cases including
  failed ChannelResponse refusal before SDK, no effect and actual owned reap;
- typed_action_admission... passed11 cases with Prepare flags2, authoritative
  shorter/longer/start-anchored bounds, held late ACK/terminal, Possible timeout/reap;
- marked_pre_dispatch_refusal... passed7 existing real-parent peer cases;
- CLI semantic_status... passed15 focused exit cases: tag alone/missing ACK/lost
  terminal/unknown delivery/cancel/timeout/cleanup cannot make0; known mismatch3;
- action_binary2 tests passed: both command syntaxes, aggregate/output caps,
  duplicate JSON/wrong input flag, strict connection/version and missing worker;
  no SDK/worker spawned by these checks. Test-owned JSON temp removed/verified;
- existing observe_cli_uses_real_guarded_peer... passed with web,macos, existing
  host worker/native_peer: complete/partial/failed/late channel/reap paths preserved.
The first compiler loop caught two internal old bool publisher calls and mechanical
visibility edits from helper extraction; direct repairs compiled, no lint suppression.

### Exact restart remainder — no dispatch until root explicitly resumes

Known unresolved source defect for chosen exits: worker_action::issue_error and
worker_web::action_issue_error map some valid fresh Unsupported/capability refusal
to HostError::InvalidInput, just like malformed action input. Mutation uses the same
refusal flags1; Prepare ordinary Error flags0 also lacks that distinction.
Current CLI completion_exit maps that NotDispatched InvalidInput to2;
the valid fresh refusal must be4, while genuinely malformed/limit input remains2.
Do not silently relabel the canonical reason or parse parent graph bodies. Root
must select the smallest private distinction/owner change before that repair;
header2/3/4 work above does not itself solve the pre-Possible category ambiguity.
The public CLI path therefore remains WIP and unaccepted despite compiling/checks.

After explicit resume: resolve that dependency, finish parent metadata fault cases
and affected exit/compact/budget cases, then save/check source for protected review.
Actual public CLI Observe→Prepare→Execute proof follows review on exact saved
worker/caller and established real Web target, preserving old evidence and reap.
No full forms/Native delivery/multi-step/P5/P6 or release claim; host Web9d3a0a5's
accepted finite proof remains separate. No source/review/runtime continuation after
this drain checkpoint without explicit root resume.

No images created/deleted. Own non-image build temp is removed/absence-verified
after final use; all existing images/application-state evidence are untouched.
No process, physical lane or runtime handle remains held. Git/index lease is requested
only for exact20 checkpoint+bounded canonical push, then immediately released.

Restart source SHA256 pins:
- CLI action.rs:eb6e900aeff88225f26f64882d4fec1913762ef9ade330c5f386bba7fb7dd147
- CLI connection.rs:62bc0ce3593add7d0581ffbfc807646847718f995c8dd23625c5530ad45483b5
- host publication.rs:c15d18476c100d4031b8c28f6d8d2ac99a166f9becbb1ec2588f971bf6a295b6
- host supervisor.rs:1b362b181277f2a09c0f287212e0e879aac6dc1f553237900eebef5f7d260bd3
- host worker_action.rs:2d0dbdf57eb26808aa1a12ddd8dcf09814fc7c0dfbe19ed4946949f76a4a020d
- host worker_ops.rs:bce0bfbada25881480de698249c35648fc17f76246d7a096d4f0dd9314669d83

## Resumed L01 source slice — selected refusal distinction

Root relayed explicit user “Продолжай работу” on 2026-10-08; same packet resumes
from saved138d7bc. CLI-ACTIONS.EXITS requires malformed/limit2 versus valid fresh
refusal4; source conflation is an implementation defect, not a contract change.
Root selected Core's source-backed minimum: internal HostError::ActionRefused and
private Terminal.value9. Add only host/src/lib.rs enum variant and diagnostic.rs
bounded exhaustive mapping to current owned worker/supervisor/CLI/test owners.
No public canonical ErrorCode/schema/version, flags0..4, connection, limits or
Observe/legacy meaning changes. Canonical provider Issue is published unchanged.
Parent admits9 only typed Prepare/Mutation before Possible after full matching
ACKed refusal/Error, with prior typed admission;9 gives no nonce/authority/success.
Malformed/limit retains existing category2; afterPossible unresolved4, IO/cleanup1.
Independent current test subset: support/effect_peer.rs, effects_host.rs; add nearest
host tests/action_cli.rs to run actual public binaries on existing synthetic CDP
peer (no live UI), plus affected worker/CLI exit cases and this receipt. Existing
kernel/provider APIs, Web collector/Swift and other-owner changes remain protected.

### Coherent resumed source checkpoint — refusal and public caller evidence

The saved138d7bc restart refusal conflation is now repaired within the selected
internal category. Prepare publishes provider Issue unchanged rather than mapping
Unsupported to a different canonical cause; typed valid kernel refusal returns
ActionRefused after complete canonical refusal. Malformed/limit paths retain
InvalidInput/ResourceLimit and category2. Terminal9 requires typed admission, exact
class/correlation, NotDispatched, full ACKed idle publication and Mutation refusal
latch (or Prepare ordinary Error flags0). Prepared2/afterPossible/legacy/wrong class/
unknown value/late terminal cannot claim refusal. No nonce/authority or success.
DiagnosticRecord's original Observe-only code bounds/encoding remain unchanged;
new exhaustive ActionRefused mapping uses its existing coarse invalid code6, while
action semantics travel through their separate terminal9. No pool or quota change.

Exact17 writes in this resumed checkpoint:
- CLI src/action.rs, connection.rs, output.rs; tests/action_binary.rs;
- host src/lib.rs, diagnostic.rs, publication.rs, supervisor.rs, worker_action.rs,
  worker_main.rs, worker_web.rs;
- host tests/action_cli.rs, web_live.rs;
- host tests/support/effect_peer.rs, effects_host.rs, web_worker_peer.rs;
- this receipt.
Root explicitly granted web_live.rs::terminal_code's one exhaustive ActionRefused
mapping only. Release that file to Web after this save; no scenario/limit/live run
changed. Other Native/Web/root changes excluded. Web dependency now savedc63b07a
is independently source-reviewed; no collector code included in this checkpoint.

CLI also reserves the complete fixed compact-status bound before any attachment/
dispatch; insufficient compact cap returns2 safely before possible effect. Setup
completion preserves available terminal categories rather than discarding them.
An attempted stale-document-at-attach case exposed existing common attach's fatal
WorkerFailed/IO1 boundary, not a typed ResyncRequired receipt. That common boundary
is unchanged and not relabelled. Required stale-plan coverage instead establishes
node remount/currentness refusal inside the unchanged document before permit.

Actual focused evidence, Cargo locked/offline/Rust1.96/aarch64-apple-darwin, owned
system-temp target, no browser/live UI:
- effects::action_metadata_requires_typed_admission_effect_state_and_exact_commit_and_survives_loss:
  12 actual-parent/owned-peer cases. Tags2/3/4 require typed admission and Possible;
  wrong class/value/Commit, absent permit/admission and legacy forged tag rejected;
  ACKed status/bytes survive worker loss and wrong nonce, without confirming effect.
- effects::action_refused_terminal_requires_typed_admitted_ack_and_never_follows_possible:
  10 cases. Valid Prepare/Mutation9 accepted only after ACK; before ACK, legacy,
  Validate, Prepared tag, afterPossible and unknown10 rejected.300ms-late9 versus
  authoritative200ms parent deadline times out; prior ACKed bytes/status preserved.
  All22 cases actual owned reap, sessions0/groups0/abandonedfalse.
- host action_cli::public_action_cli_keeps_delivery_verification_exits_and_original_evidence_separate:
  8 cases run the actual public CLI and production worker against the existing
  synthetic CDP peer: Observe ChannelResponse→Prepare→Execute verified0, mismatch3,
  confirmed delivery/lost post binding4, fresh unsupported Execute4 and Prepare4,
  malformed plan2, compact verified status and remounted/stale node4. Fresh Issue
  code Unsupported preserved. Refusals dispatch zero setters; successful/verified
  attempts exactly one setter; original observed source file remains byte-equal.
  Each actual CLI uses its real bounded shutdown path; no actual browser proof.
- CLI semantic_status...17 focused cases and action_binary2 tests passed, adding
  fresh-refusal category4 and compact137-byte preflight (136 refuses before spawn).
- Affected host lib/bin/runtime/action_cli/web_live/effect_peer and CLI web bin/
  action_binary Clippy -D warnings passed. web_live compiled only, never ran.
  Affected macos-only CLI check passed, preserving shared exhaustive consumers.
Test peer now accepts sequential reconnects within its unchanged5s bounded window
so separate public invocations can reuse the exact configured synthetic document.
Test-only currentness flag supplies remount refusal; no production fallback added.
The terminal9 peer first sent Completed in two intended fault modes; corrected
its explicit control, then the10-case check passed. No acceptance expectation changed.

No unchanged broad suite repeated. Prior saved deadline/Observe/refusal checks remain
their earlier evidence; this slice does not self-label independent acceptance.
Protected review of saved CLI/metadata source precedes actual public CLI vertical
Web proof. Full forms/Native delivery/multi-step/P5/P6/release gates remain separate.
No remaining known refusal-exit failure in this slice; common attach fatal category
is an explicit existing limit. Root consumes saved source/test handoff for review.
Own non-image target is removed/absence-verified after final use. Test JSON temps
were removed by exact file/empty-directory cleanup; no images created or deleted.
No process, runtime handle or physical lane held. Short exact17 Git lease requested;
checkpoint+bounded exact-SHA canonical push, then release index and web_live.rs.

Resumed source SHA256 pins:
- CLI action.rs:e55d81095c61d8efe9d44171aa0b027a9a494496694d64338067439d61a0f7a4
- host lib.rs:7b23663d772a2cab34ec68bddb024ab38998268ad6fc91ce43bfc8633ad70af1
- host supervisor.rs:61ccfbc1d74e771fd79fdabfc2c209277df60af88e219135e7afb5cc64578212
- host worker_action.rs:d3df17181e8216ef56481e6e9c5f78a7093c883c75716e8ea641c6c10aca38f2
- host worker_web.rs:13c80ea449b0857901f6353cd38f0a1dd89c3f1597af3d21521f8bdd0479dc30
- action_cli test:6cccd3f6f04babff7c1221095c9fbf3ad3f56aeecc196ecbbefc44983fc4e181
