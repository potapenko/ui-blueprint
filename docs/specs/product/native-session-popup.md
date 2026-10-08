# Native popup action composition
- Node type: leaf; domain: `uib.native.session.popup`; contract: `UIB.NATIVE-POPUP@1`.
- Authority: Active / Evolving; no independent acceptance or release claim.
- Authority source: original COMPLETION Native E2E and approved PLAN.UIB@1, delegated ROADMAP technical representation in [N03](../../plans/ui-blueprint/packets/N03-native-popup-e2e.md).
- Requires: [NATIVE-SESSION@3](native-session.md), [NATIVE@2](native.md), [ACTIONS@1](actions.md), [IDENTITY@1](identity.md), [PROJECTIONS@1](projections.md).
- Read when: explicit own-fixture popup form with a separately held parent result.

## UIB.NATIVE-POPUP.BINDING — N03-POPUP-001
Restore the original popup E2E, preserving canonical0.1/analysis0.2 and ordinary
single-Surface sessions. Existing `collection:"form"` may explicitly provide the
existing popup `binding`/`identity_path`, `parent_binding`/`parent_identity_path`,
and new `parent_form_identifiers`. This private option is selected before code.
Parent/popup must be distinct windows of the same exact process incarnation,
with fixture identifiers a/popup-a or b/popup-b and separate live generations.
Both Surfaces, popup first, must already be in caller session/request authority.
No implicit addition, title/rect matching, whole-app search or reparenting.

`form_identifiers` selects popup controls; `parent_form_identifiers` selects parent
controls and must include the existing f02.popup anchor. Each list is nonempty,
unique and bounded to256 UTF-8 bytes/identifier; combined count at most8. Existing
session/config/acquisition limits remain. No protected_input binding is admitted
for this optional popup form: PROTECTED's same-Surface restriction is unchanged.

Reuse exact parent AXIdentifier and sourced AXPopover ancestry with the fixture's
owner marker; the own content attachment supplies physical popup binding. Retain
original parent/popover/control CF objects for the session. Revalidation proves
current unique connectivity and CF equality, never replaces a retired handle.
Reported popup owner/initiated_by/anchor retain fixture binding provenance, separate
from AX property evidence. Only explicit selected controls are returned.

## UIB.NATIVE-POPUP.ACTION
Reuse canonical Activate(Semantic), explicit caller Expectation and parent one-use
permit. Before resolve and delivery, revalidate both Surface generations, every
selected handle, exact process and focused parent AX window/input owner. A public popup
Activate may name a distinct, independently held public parent result. No secret
cross-Surface exception, physical pointer claim, implicit Enter or retry.

For that action's phase3 only, read the original held parent result freshly after
revalidating parent continuity. Actor self-closure is allowed: do not read/reacquire
it, report it current, or substitute another node. Preserve original authorized
Context but include only the observed parent result/Surface record; partial coverage
is explicit. Missing/remounted result remains uncertainty, never success. All later
Observe/Prepare/Act require the original popup live again and refuse its retirement;
a reopened generation needs a new explicitly bound session and new refs.

Fixture Confirm sets parent Result to popup-a/popup-b and closes popup in the same
SwiftUI action. Therefore measure/capture and explicit resize/local comparison occur
while open, then Confirm/result/closure, then old-ref refusal and fresh reopen.
Opening and resize use existing product Activate; explicit fixture Snapshot publishes
bindings only and is setup, not proof of action delivery. Independent UI/fixture
observation proves opened state, parent result and no effect in the other same-title
window. Preserve raw source/evidence through existing measure/diff/compare consumers.

## UIB.NATIVE-POPUP.PROOF
Required: real whole-chain positive, before/after/ref records; two-window isolation;
closed/recreated stale refusal; parent result remount/ambiguity/owner refusal;
protected-input exclusion; bounded cancel/deadline/post-Possible unknown/no retry;
old form/secure/one-shot compatibility and unchanged canonical validation. Existing
kernel and host owners remain authoritative. Author checks are not Q01 independent
acceptance; M05/M06/D06/P7 gates are not closed by this registration.
