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
