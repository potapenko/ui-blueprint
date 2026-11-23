# Source-selected SwiftUI parent and popover candidate

- Node type: leaf; domain: `uib.swiftui-debug-capture.pair`.
- Contract: `UIB.SWIFTUI-CAPTURE-PAIR@1`; Authority: Active; Stability: Evolving.
- Authority: direct approval of the C04 plan in chat
  `01a20a04-5ef7-7350-9876-bcd66392126f`, 2026-11-23: implement and check a
  candidate, leave SwiftUI e19b20c47 unchanged, perform no new capture.
- Requires: [debug capture](swiftui-debug-capture.md),
  [cached transform](swiftui-cache-transform.md), [GEOMETRY@2](geometry.md),
  [IDENTITY@1](identity.md), [PRIVACY@1](privacy.md).
- Read when: source-specialized part selection or paired native windows.
- Accepted/released baseline: none. Offline tests are not runtime qualification.

## PAIR.SELECTION

Optional `scope.owner_argument_type` narrows the existing exact nominal owner
to the nominal type of its single generic type parameter. The initial decoder
admits only a flat generic struct with one ordinary key type parameter and the
qualified 64-bit metadata layout. No model values, generic witness payloads,
text, coordinates, ordinal selection or caller-provided metadata addresses.
Unknown metadata/layout refuses; two matching body occurrences remain ambiguous.
The selector may additionally name `scope.part` with exact nominal `type` and
proper-descendant `containing_type`. Select exactly one such part within the
already unique owner forest. This selects a runtime structural part, not the
computed Swift source property's name or an accessibility element.
These additions apply to direct structural selection only; old native selectors
and old exact nominal selection retain their meanings and limits.

## PAIR.WINDOWS

Optional request `paired_surface` has exactly `window_id` and `scope` and names
one different window in the same admitted process. Both scopes are explicit.
The initial pair accepts direct structural scopes only, not native-anchor modes
or leaf cuts whose discovery accounting has not been qualified across graphs.
Revalidate each window/root/host. No title, position, first-window fallback or
reading content of unselected windows. Each selected window must still have one
supported outer hosting root; unsupported host types remain an explicit refusal.
Use one stopped-process interval and the existing read/call/deadline budgets.
Raw discovery/traversal work, emitted nodes, native view walk and output bytes
are bounded across the pair, not separately doubled. Preserve distinct surfaces,
root spaces, observations and original numeric layout values.

## PAIR.COORDINATES

Reuse the qualified cached translation and native host-to-window basis.
When both native mappings are known, derive popup-window to parent-window
top-left pt translation from same-interval NSWindow frames. This destination
is the first selected window, not AX screen space. No display height, PNG,
screen scale, parent-origin summation or fitted offsets are inferred.
Unknown per-node transform remains unknown; window-frame availability cannot
repair a dirty SwiftUI transform. An unsupported native mapping keeps independent
local spaces. Preserve provenance for the native coordinate facts.

## PAIR.ATTACHMENT

The source declaration `.popover` on disclosure proves source intent, not a
runtime association between two chosen windows. Parent-window membership and
coincident rectangles also do not prove the trigger-to-content relationship.
This candidate must report `attachment.status=unknown` until an independently
qualified presentation/anchor reader binds both selected occurrences. It must
not manufacture `anchored_to`, `corresponds_to` or component membership from the
request, matching type names or the source declaration. No caller-supplied proof
flag may turn this into known. No such reader is qualified by this contract.

## PAIR.PUBLICATION / COMPATIBILITY

Produce one canonical core0.1 Snapshot with separately attributed nodes/surfaces
from this one acquisition, plus a terminal pair coverage receipt. Do not merge
previously saved observations. Preserve exact old single-window output behavior.
Failed selection, stale window, quota or cleanup refusal publishes no pair.
Partial layout and unknown attachment never mean C04 accepted. Product admission,
backend pin and owner recovery remain closed/unchanged; request fields cannot
bypass admission. Installation and release are outside this candidate handoff.

## PAIR.ACCEPT

Offline tests: generic metadata shape/identity, duplicate owner/part, missing
part, stale/foreign window, aggregate limits, dirty/unmapped geometry, native
coordinate conversion, canonical Rust validation/measurement and private-data
exclusion. Preserve existing capture/cache/transform/backend regressions.
Pending separately: source-shaped live fixture, independent privacy/lifecycle
review, actual host/part shape on unchanged e19b20c47, presentation association
decoder qualification, and authorized C04 runtime proof for hello + Comedy.
No new capture, app launch or debugger attach is authorized by this candidate.

## Change record

C04-PAIR-001, Evolve under the explicit plan approval above: add opt-in bounded
source selection and paired-surface candidate without donor changes or new core
wire vocabulary. Protect AX/SDK/GPUI attribution, limits, recovery and product
admission. The missing runtime attachment witness remains a disclosed capability
gap, not merely an unchecked positive test.
