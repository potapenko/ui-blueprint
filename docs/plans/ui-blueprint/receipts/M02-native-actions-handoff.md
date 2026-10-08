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
