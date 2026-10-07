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
modality Accessibility—not SetChecked fallback. It requires a separately selected
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
