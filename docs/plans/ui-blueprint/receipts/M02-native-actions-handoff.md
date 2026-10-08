# M02 native SetChecked — bounded source handoff

Classification diagnostic; Discover, finite packet M02-native-actions-handoff.
Approved PLAN.UIB@1 P5/M02; source inspection/handoff only, no implementation/runtime.
Read saved8507aa46d36a3fdbd20195fdc1e1c89beb366911 (Native owners unchanged since23f22fb),
not concurrent Core/Web WIP. Single chat/master/inherit; no delegation or input lane.
Only this new receipt in the existing receipts directory is writable.

Traversal: AGENTS/product-truth → registry17 → ACTIONS/FORMS/NATIVE CONTENT@1 and
MODEL/EXCHANGE/IDENTITY/GEOMETRY/PROJECTIONS/LIFECYCLE/CACHE/PRIVACY/BOUNDARIES closure;
D02@2/D04@1/D05@4/acquisition@2/MEMORY@2/WORK@1 plus established D01/D03/D06/D07/
EVIDENCE, Native/PILOTS/GOLDEN, RUST/DEV.RUST. Reused applicable full current content.
New route read completely: reference branch → EXECUTOR-SOURCES@1/native-catalog
NATIVE-SOURCES@1 → previously read REUSE@1. Actual filenames are executor-catalog.md
and native-catalog.md. Existing R02 AXorcist value/action ledger suffices for mechanism
provenance; no upstream transfer/new dependency. Excluded Web implementation/CLI changes,
real apps, source edits/tests/UI/SDK mutation/capture/general audit. No contract delta.

## Control and capability evidence

[Fixture.swift](../../../../fixtures/native/Fixture.swift) PilotView exposes existing
SwiftUI Toggle("Enabled", isOn:$checked), AXIdentifier f02.enabled, checked Bool with
onChange changed(). Own Snapshot records state.checked; it is an oracle, not external
setter evidence. Actual prior CUA states showed checkbox Description Enabled/Value0,
without the '(settable)' marker that tool displayed for text fields/scroll values.
This proves an exposed checkbox state, not an actual AXIsAttributeSettable result.
Known source expectation: two-state SwiftUI Bool; raw live CF type/value encoding and
accepted setter value remain unmeasured. Do not infer numeric1 vs CFBoolean from CUA0.

Local primary Apple SDK evidence read: ApplicationServices/HIServices Headers
AXUIElement.h lines187–223 specifies AXUIElementIsAttributeSettable separately from
AXUIElementSetAttributeValue; latter may reject an unrecognized value with IllegalArgument.
AXAttributeConstants.h lines314–342 states checkbox AXValue is not settable in the
normal checkbox model and changes in response to AXPress. AXRoleConstants.h merely
names AXCheckBox; it does not supply a universal numeric/Boolean value enum.
SDK root: /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/.
These are static public API evidence, not a measurement of this SwiftUI instance.
No fresh settable read or mutation was authorized/performed by this packet.

Conclusion: no supported native SetChecked setter is established. Expected ordinary
AXCheckBox path is unsupported for setter; runtime read must distinguish known false
from unavailable/error unknown. Proposed value admissibility is separately unknown.
Never advertise writable/value_allowed true merely because role/AXPress exists.
No toggle/AXPress/coordinate fallback can silently satisfy SetChecked(bool)/Setter.

## Existing owners and exact missing boundary

[WindowAX.swift](../../../../tests/bridges/native/WindowAX.swift): NativeAXAccess has
attribute/batch/actions/count/page/isElement/prepare reads, no settable/setter port.
nativeAXScalar preserves CFBoolean flag versus finite NSNumber number; collectWindowAX
maps AXCheckBox role but its selected fields exclude Checked. Its source key is
observationID-handle-index, not a reusable native object identity. No BackendRef or
session AX-handle registry is emitted. Existing nativeMayReadValue admits classified
nonsecure public checkbox value reads; secure exclusion remains protected.

[Collector.swift](../../../../tests/bridges/native/Collector.swift) resolveWindow/
resolveElement provide bounded exact identifier search rooted in authorized window;
NativeCurrentIdentity plus public process incarnation/CG owner establishes current
fixture surface. Fresh locator uniqueness does not prove continuity of an old element:
remounted f02.enabled must not silently repair an old ref. A future native provider
needs a held opaque AXUIElement identity across resolve/permit/deliver/verify, with
process/surface generation checks at setter point; same identifier alone insufficient.

[HostProtocol.swift](../../../../plugins/macos/HostProtocol.swift) receive currently
accepts Observe only; existing Configure/Submit/Ticket and one-channel reply contain
no action-phase/parent-permit-to-helper delivery mechanism. [HostHelper.swift](../../../../plugins/macos/HostHelper.swift)
is one-shot collection, returns then exits; it cannot presently retain an AX element
across independent public Observe→Prepare→Execute invocations. Mutation cannot be
introduced by placing an Act payload on the observation helper request.

[actions.rs](../../../../crates/plugin-api/src/actions.rs) already supplies unchanged
SetCheckedProvider::resolve_exact(ActionCase,ClockReading,remaining_ms)->ActionCase,
deliver(Action,DeliveryPermit,remaining_ms)->DeliveryStatus, observe_after(...)->Snapshot.
Kernel dispatch freshly resolves same requested key/target/surface before EffectGate,
requests parent permit once, preserves post-Possible uncertainty/no retry. verify needs
Confirmed delivery plus fresh same-key Checked flag/evidence, not saved current label.
[validation/outcomes.rs](../../../../crates/schema/src/validation/outcomes.rs) separately
requires enabled/current/unique and reported writable/value_allowed true plus set_checked
intent; numeric AXValue cannot simply be published as Checked without supported mapping.

Saved host worker_action::ActionOperation and worker_effect::WorkerEffectGate already
own kernel/parent EffectReady→EffectPermit and ACKed typed outcomes. Preserve these;
no new gate/nonce/schema/graph needed. worker_main Prepare/Mutation routing and
worker_ops action_input/prepare_input are presently web-feature-specific; worker_native
exchange/native_broker only provide observation helper calls. Native provider/routing,
held native identity lifetime, bounded private action exchange/permit delivery and
Checked/read/settable/value mapping are missing implementation dependencies. No wire
encoding or new protocol/signature is selected by this handoff; Core owns that boundary.

Minimum conditional split: first a separately authorized bounded read-only capability
check on exact own f02.enabled (role/raw AXValue type, Enabled, exact current binding,
AXUIElementIsAttributeSettable result, available native actions). If false, stop setter
provider before permit. If unexpectedly true, establish accepted Bool value encoding
independently before any setter; retain exact identity through existing parent permit
then one AXUIElementSetAttributeValue call, modality Setter. API success is delivery
acceptance only until the provider establishes the required delivery confirmation and
fresh Checked state; failed/unknown post-Possible outcome must never retry.
Native narrow owners would be existing AX/property/Collector and helper protocol/lifetime;
Core narrow owners worker_native/native_broker/worker_main/worker_ops composition.
Shared kernel/schema/quotas/privacy/effect authority remain protected. Exact new private
signatures need owner selection after capability evidence; no speculative framework.

## Valid alternative and acceptance boundary

If checkbox setter is unsupported, smallest source-grounded alternate native action
candidate is explicit Activate on an existing control with actually reported AXPress,
canonical modality Semantic—not SetChecked fallback. It requires a separately selected
activation kernel/provider packet; current kernel handles SetChecked only. Existing
f02.sample.a button increments Count and offers a concrete independent outcome oracle;
its current reported action/capability must still be read before delivery. No new control
or functionality is proposed, and that alternate is not implemented/authorized here.

Required future setter evidence: exact capability/value mapping, no prepare effects,
fresh same-handle resolution, stale/remount/same-title refusal before permit, exactly
one authorized setter, both bool targets including already-desired state, independent
Checked observation plus fixture applied state, post-Possible loss/cancel/timeout unknown
without retry, parent nonce/ACK/owned-helper reap and zero cross-window effects.
Unsupported/unknown capability is a truthful refusal, never a positive M02 gate.
This source-only receipt runs document link/route/whitespace checks, no build/tests or
runtime. Checkpoint/push requires short exact1 Git lease; no resources/images created.


## Capability observation activation — current binding setup dependency

Root appended Next bounded capability observation: exact own F02 f02.enabled and
f02.sample.a, read-only role/raw value CF type/Enabled/settable(Value)/actions/current
identity, AX1s/overall120s/owned cleanup. No setter/AXPress/UI input/focus/permission
or fixture/source changes. Immediate consumer: supported Native delivery selection.
Reused current full M02 closure and read the complete appended packet before action.

Smallest saved Native23f22fb setup owners checked: Fixture.swift::publish is the sole
writer of a.json and OPEN a-identity.json. publish is reached by existing explicit
Snapshot button (snapshotRequest++/publish in off mode); no startup publisher. The
actual just-launched no-Snapshot run in saved16df191 also had an empty run directory.
Current identity storage is not supplied by app launch or passive AX observation.
Collector::resolveWindow("a") locates a unique AX window; windowOwnedBy independently
checks a supplied physical CGWindowID/PID. Neither establishes a new trusted fixture
surface generation/current identity_path without Snapshot. Public AX identifier,
PID/title/order/rect alone must not replace that required binding.

Thus the activated fresh read-only diagnostic lacks its named current surface binding
under the simultaneous no-UI-input/no-AXPress/no-fixture-change boundary. No app/build/
SDK/UI/test was launched, no capability result or false/unknown value was fabricated.
Do not reuse a stale removed run's files/generation or silently derive physical identity.
Exact minimum missing setup: root-selected permission for the existing own-fixture
Snapshot semantic setup action only (then SDK capability reads remain read-only), or
an exact already-current owned fixture binding furnished within a live authorized
lifecycle. No new source/fixture mechanism, setter or control needed for that setup.
If root intends an AX-only diagnostic without physical/current surface evidence,
that reduced diagnostic scope must be explicit and cannot claim fully attributed
capability/provider acceptance. Return this dependency before runtime, not a generic
missing-tool claim or setter-support assumption. No independent task expanded.

Only this receipt appended, document whitespace/links checked; short exact1 save lease
required. No runtime resources/input lane/images/temp artifacts created. Capability
CF value/settable/action outcomes and Native implementation choice remain unresolved.


## Clarified Snapshot setup — actual read-only capability adapter failed

Root explicitly clarified existing semantic f02.snapshot action is permitted solely
to publish current binding; target checkbox/button remain input/setter/AXPress-free.
Updated packet read before setup. Previous binding dependency resolved by this exact
setup exception, not new source. Canonical modality name is Semantic; the earlier
platform label Accessibility was never a proposed wire enum (wording corrected).

Frozen Native23f22fb fixture/Collector/WindowAX/acquisition/artifact/protocol inputs
archived to system task-temp; no Core/Web WIP. Tiny task-temp Capability.swift reuses
Collector.resolveWindow/resolveElement, NativeAXAccess/NativeAcquisition/NativeJSONFrame.
Bounded two-control query only,160/depth9/512KiB,profile unchanged, AX1s watchdog/local
deadline; no secret/other-window content/setter/PerformAction/permission API.
Fixture and adapter compiled cleanly before live, no test suite/framework/product edit.
Adapter SHA83405d48854adac577e2d39904940d8b6ca89049ab3f99d7dc705b1f9e35dfe9;
binary60a4bae7bf9fba5a04830c48a2f4b33f533a9543d4a59bde7ba9aef83b74879e;
fixture8c82ed594c44495967aa674e69d002810494e85605c4e042ef31e1c26d2ed3df.

Own PID44138 launched via exact bundle/executable/run argument. Fresh CUA full Window A
state immediately before semantic Snapshot index4; only Snapshot clicked, no popup or
control input. Snapshot1 current identity verified before diagnostic dispatch: OPEN,
bundle local.uiblueprint.f02.off,launch1791417771.736886,target_generation44138:1791417771.736886,
window11030/identifiera/surface_generation281A6E4C-1927-41F0-9EDC-A8BEDE1AB977.
Exact current file independently read pre/post diagnostic, equal; live full CUA AX
state also exactly equal, focus Open B/Count0/checkboxValue0/Resultnone preserved.

Actual diagnostic process exited -5 (SIGTRAP), stdout empty. Sanitized exact stderr:
Swift/ErrorType.swift:254: Fatal error: Error raised at top level: capability.NativeAcquisitionError.invalidValue
This is a failed diagnostic adapter operation, not unsupported permission/tool or
native setter result. No per-phase error receipt exists, so the failing validation/
lookup/read/publication phase is unestablished. Do not infer target control capability,
CF type/accepted value or runtime settable=false from SDK/source/CUA appearance.
No second SDK query or expanded limits/read scope; no setter/AXPress effects possible
in this read-only adapter. Current binding/live state facts do not manufacture the
missing control capability output or full M02 acceptance.

Launcher start273877.6683115/deadline273997.6683115, finally before timing assertion:
elapsed31.385507083s,cleanup0.124926042s,timeout=false,fixture absent; diagnostic child
reaped by subprocess, no owned process/input lane remains. Images0; old images untouched.
Consumed non-image run outputs/identity/bundle/build/source/cache/adapter removed,
directory absence verified. Immediate next dependency: narrowly make diagnostic
error boundary report actual phase/status rather than top-level throw, then root
selects whether to repeat the same bounded capability read; no product-source repair
or further SDK attempt is silently authorized by this failure. Receipt exact1 save
awaits Git lease. Native provider/capability selection remains open, static checkbox
non-settable guidance retained solely as static evidence.


## Phase-aware adapter — actual exact control capability established

Root authorized direct transient error-boundary repair within same diagnostic packet,
then one bounded same2control read. Product source/fixture/limits unchanged; saved
Native23f22fb inputs only. Temporary adapter added named phases and last AX call/status,
handled top-level errors without SIGTRAP and per-control resolution unavailable without
fabricating value or terminating the other read. Existing owner invocation inspected:
nativeAXAttribute returns nil on unavailable, nativeAXElements/resolveElement enforce
complete bounded unique mapping and throw invalidValue on unresolved/read failure.
Earlier failure phase was not recorded and remains unknown; this successful fresh
run does not retroactively establish its cause. No framework/general logging audit.

Fresh fixture/adapter compiled before timer; adapter source
ddcf8a0e3a4fbb0edc44583ded85fa73c001babe7e5025b536ba1d5638e5af57,
binaryb7f40bf822b9a8a5ba576cf8534e5cb5ae2d351cfa70f5cfecb8a7d601f1d399,
fixture8c82ed594c44495967aa674e69d002810494e85605c4e042ef31e1c26d2ed3df.
No production helper/protocol/schema/Rust owner change or test suite. CUA fresh exact
Window A state→existing Snapshot index4 only→Snapshot1/current own binding. No popup,
field/control input, setter, target AXPress, focus/permission/display manipulation.

Own PID45693/bundlelocal.uiblueprint.f02.off/launch1791417966.013506,
target_generation45693:1791417966.013506/window11055/identifiera/
surface_generationC9654CB2-DE78-4409-8C1E-A8F30B6977E1. Current identity independently
read OPEN before/after SDK diagnostic and equal; diagnostic also checked current
file/public process launch/CG owner before/after and each exact control. AXWindows
identifiera and unique bounded exact control resolution rooted in that window;
each AXUIElementGetPid returned success/PID45693. No other window content/source ref
continuity or product BackendRef lifetime claimed by this capability diagnostic.

Actual SDK result exit0/no stderr, public fields only:
- f02.enabled: AXRole success/CFString AXCheckBox; AXValue success/CFNumber type22,
  value0; AXEnabled success/CFBoolean type21,true. AXUIElementIsAttributeSettable(Value)
  success/valuefalse. Reported actions success/[AXPress]. No setter attempted; proposed
  new value admissibility remains untested because writable=false. This actual instance
  cannot support the selected SetChecked setter path, not merely a static prediction.
- f02.sample.a: AXRole success/CFString AXButton; AXEnabled success/CFBoolean,true.
  AXValue status-25212/no returned value preserved as unavailable (not false/empty).
  Settable(Value) success/valuefalse; reported actions success/[AXPress]. This establishes
  current Semantic Activate capability candidate only; no AXPress delivery or Count
  outcome/provider/parent-permit proof. Does not substitute toggle for SetChecked.

Native read interval274089.71137879166→274089.85765200004 in diagnostic process
monotonic, within unchanged1s/depth9/cap160/512KiB/profile. Exact full live CUA AX
before/after SDK query equal: checkboxValue0/Count0/Resultnone/focus Open B/forms/scroll
state unchanged. No saved Snapshot equality masquerading as live verification, no
pixels/hit proof. Diagnostic records mutation_calls0, no mutation entrypoint exists.

Launcher start274071.955392166/deadline274191.955392166. Finally before timing assertion:
elapsed27.800881042s/cleanup0.118551500s/timeoutfalse/exact own fixture absence confirmed;
SDK child reaped, no own process/input lane remains. Images0, old images untouched.
Transient source/binaries/cache/run values retained through diagnosis/fix/actual result,
then consumed non-image files removed and directory absence verified. No next runtime
or implementation automatically started. Existing receipt only updated; exact1 short
checkpoint/push lease required. Consumer: select existing-control Semantic Activate
provider/kernel boundary; SetChecked setter must refuse Unsupported before permit.
M02 actual delivery/forms/focus/type and full P5/P7 remain unaccepted.


## A02 Native/Core held identity boundary — engineering proposal

Read saved e3a6880 (current Core/Web WIP excluded): native_broker::NativeBroker,
helper_runtime::{spawn_registered,poll_helpers}, helpers::Helper, supervisor helper/
effect cleanup; worker_native::NativeExchange; Swift HostHelper/HostProtocol and
WindowAX::key. No capfacts re-read, runtime/source edits/tests or new framework.
A02 early API handed off by root: one ActionExecution::prepare_action(case,Expectation,...),
provider resolve_exact/observe_after receive &Expectation; SetChecked compatibility.
This API is integration input, not permission to change private protocol here.

Observed lifetime: broker Admit spawns a registered one-shot helper under active
Observe deadline; reads one NDJSON line, moves charged HelperBytes, sends opaque
body to worker. Helper::take_line starts cleanup; deadline expiry also stops helper.
Swift main receives Configure+Submit/Observe, uses local watchdog, replies and returns.
Parent poll_helpers confirms reap before slot reuse; lost/reap failure quarantines.
Current macos.ax key is observationID-handle-index, no reusable CF object handle.
Existing saved Observe refs consequently cannot establish held native continuity
for later public Prepare/Execute; resolveElement(identifier) is not an old-ref repair.

Minimum proposed ownership: one parent-registered non-capture Native action helper
holds only exact target and Expectation result AXUIElement objects in a fixed bounded
registry; Rust guarded provider owns canonical refs/Expectation/kernel, parent owns
process/byte leases, correlations and EffectReady/Possible/one-use permit. Swift owns
CF retains/releases only; no second graph/kernel. No unregistered child/daemon or
capture lease for action. Registry tokens bind helper serial/session epoch, surface
generation and exact held object; reported canonical SourceKey remains stable within
that lifetime, not regenerated from traversal position on each re-read. Reap invalidates
all tokens. Legacy one-shot observation remains supported but refs are non-actionable.

Action-capable observation must originate in that held helper; retain exact objects
when producing its Snapshot, not infer them later from arbitrary saved identifiers.
Prepare re-reads held target and separately held result node, public process/window/
current surface and unique/current/enabled/capability; return fresh canonical evidence
for the same keys only. Deleted/remounted/disconnected object refuses, never binds a
replacement by label/identifier/coordinates. Expectation result continuity checked
independently; Activation Count outcome must not be inferred from source declaration.

Minimum private exchange phases: bound Observe/Prepare reads; Deliver on the same
held token only after actual parent effect permit, carrying its one-use nonce/exact
operation correlation/remaining deadline; PostRead(Expectation result token); Close.
No delivery on read request or UI-supplied nonce. Worker invokes existing A02 provider
ports and canonical validation; parent transports bounded bytes/fixed metadata without
JSON parse. Swift rechecks held object/capability/current identity at dispatch, invokes
one reported AXPress for explicit Semantic Activate and returns honest delivery state;
fresh result read remains separate. Lost reply/timeout after Possible stays unknown,
never re-dispatch. Exact private tags/encoding are a Core-selected protocol delta,
not specified or invented by this source handoff.

Root selected lifecycle requirement: refs survive sequential Observe/Prepare/Act
within ONE attached RuntimeHost session; after helper reap old refs invalidate/resync.
No daemon or hidden helper retention across terminated CLI processes. Single transaction
with refs dying between those session operations does not satisfy this requirement.
Existing helpers have fixed per-operation deadline and cleanup on taking reply: need
Core-selected explicit bounded session residency, separate request deadlines and
nonterminal reply reuse. No numeric cap/default/private encoding selected here.
Dependency remains until shared owner grant; do not silently extend deadlines or claim
current one-shot refs work. Public invocation lifetime must remain truthfully scoped.

Core write split for the selected attached-session workflow: existing native_broker/worker_native
phase exchange; helpers/helper_runtime bounded reply reuse/lifetime; supervisor reuse
only registered matching helper and stop/reap/quarantine on cancel/detach/session EOF/
deadline; worker_main/worker_ops Native routing to A02 ActionExecution and unchanged
worker_effect gate/publication ACK owner. native_binding only if selected trusted
lifetime/token metadata cannot fit existing opaque configuration. No pool/cap growth.
Swift split: HostProtocol validates selected private phases/token/permit correlations;
HostHelper owns bounded held object registry/request loop/EOF termination/watchdog;
existing Collector/WindowAX re-read exact target/result properties and canonical keys.
Fixture/schema/common kernel untouched by Native; Core owns A02 extension independently.

Reuse current cleanup1s and confirmed child reap; no slot/token reuse on CleanupPending.
Parent closes/kills only exact registered helper, Swift drops CF refs on clean Close;
process death clears remaining SDK handles. No callback after cancellation permits
new input. Required proof: old one-shot ref refusal, same-ID remount, target/result
identity mismatch, forged/replayed/wrong-session permit rejection before delivery,
exactly one authorized AXPress, fresh Count outcome, post-Possible loss unknown/no retry,
request deadline/cancel/detach/EOF/failed reap and bounded registry/resource admission.
Proposal only, no authority to implement private encoding/lifetime/limits yet.


## Task-wide Native Activate source — bounded held target/result owner

Root relayed user's autonomous implementation/test/end-to-end direction, later corrected
review policy to one focused review of coherent risky integration, not per-mechanical
change rounds. Task-wide owner Native plugins/macos + tests/bridges/native/docs/receipt;
fixture/schema/Core Rust protected. A02 kernel accepted; no repeated kernel review.
Current target outcome remains actual Semantic Activate f02.sample.a + fresh Count result
under real parent permit, not a source-only completion claim.

Exact4 current writes: new plugins/macos/NativeHeldAction.swift, new
acquisition/HeldActionChecks.swift, native-helper.md, this receipt. No transport tags,
nonce authority, private residency numbers or delivery implemented before Core interface.
NativeHeldAction retains exactly held target/result plus scoped window/application CF
objects, worker-owned canonical macos.ax keys and trusted current binding callback.
Initial bind uses bounded exact window/element owners, explicit node/depth budgets;
validate fresh-resolves unique connectivity then requires CFEqual with the stored
window/target/result, never replaces old objects by identifier. read surrounds actual
collectWindowAX node/property read with current/identity validation and preserves keys.
Different target/result object required; invalidate blocks all further reads, owner drop
releases refs. Target and Expectation result identity are checked independently. No graph,
framework/SDK changes or new runtime flag; unknown properties remain existing availability.

Source compiled with existing HOST_HELPER/CAPTURE_LIBRARY collector/acquisition/protocol
closure. Focused9 synthetic checks passed: target/result stable keys and value-property
reads; same-identifier target remount refuses before property batch; result/window remount,
current-generation loss and invalidation refuse. Mutation calls0/live SDKfalse.
An initial compile was invalidated because the test input changed during build; rebuilt
final unchanged inputs, clean compile/pass. No product defect/expectation/cap adjustment.
This proves owner machinery only, not actual native delivery/current SDK ref lifetime.

Integration handoff: construct NativeHeldAction on originating action-capable Observe;
hold through one attached RuntimeHost session, call validate before A02 resolve and
read(.result) for explicit Expectation; canonical keys supplied by guarded worker/source
identity owner. trusted current callback must check live process/CG window/current surface,
not cached Snapshot flags. On cancel/detach/deadline/EOF/reap invalidate/drop owner and
refuse old refs. Existing one-shot helper results remain non-actionable; no cross-CLI
retention. Native provider cannot invent permit or dispatch from Read/Prepare.

One concrete remaining shared dependency returned to root/Core: selected attached-session
private exchange + bounded residency + actual EffectPermit→held-object delivery path.
Swift owner/reads are ready; Core owns shared protocol/nonce/lifetime, Native will attach
reported AXPress delivery only to explicit Semantic Activate when that interface exists.
No checkbox AXPress fallback: actual false setter remains Unsupported. Ordinary provider
integration tests and one actual own-fixture Activate/Count follow in same task-wide
outcome once real parent permit ready; no extra approval on mechanical owner fixes.
Current implementation is a coherent preparatory slice, not completion of that outcome.

Non-image compile/module-cache products consumed/removed and absence verified; no live
fixture/input/SDK operation or image created, no lane held. Source pins follow below.
Changed docs links/whitespace checked; exact4 short Git checkpoint lease requested,
no unrelated WIP/index staged. Source review, if root retains it, groups completed
integration risk rather than this mechanical owner step.

plugins/macos/NativeHeldAction.swift: c4b600a48e96ebd86ca845e4396e34515ebe715862e9863e405a26437befeebe.
tests/bridges/native/acquisition/HeldActionChecks.swift: 0e9530c85bd16ffcc8581db90b7285324cbccd562300975de83b944db7bfba7a.


## User priority correction — coherent held-owner WIP checkpoint

Root relayed direct user product clarification: primary value is real UI geometry/
structure for AI-assisted Web/Mac interface development. Stop further P5 action/helper
transport work now; preserve current code as WIP, no discard/refactor/infrastructure
expansion. Source9 focused tests already passed; no started check remains. This save
contains only the exact4 held-owner source/test/docs/receipt paths above; no delivery,
protocol or actual Activate implemented. Action scope remains deferred, not removed.
Read-only geometry does not require action-ready refs/parent nonce/held-action helper.
Next consumer: existing AX bounds plus measured opt-in probe internal parts, truthful
units/spaces/unknown and concise agent output. Earlier AX/probe/guarded Measure facts
remain their evidence; selecting a fresh geometry scenario follows root's product
context, no invented Mac product goal or new action prerequisite.
