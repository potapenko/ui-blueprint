# S03.A Actors picker

Status: **candidate implemented; original consumer acceptance pending**.
Updated2026-11-23. Owner `01a20923-023c-7cd0-8421-a2e20177df71`; consumer
`01a20911-330a-7fa2-aa96-980371b389a7`; shared reader owner
`01a20922-f6ad-7651-997b-cef1b3f09b12`; coordinator
`01a1e565-14f6-7be0-aa02-cd648a1b0423` handles release/install/publication.

## Authority and scope

Direct human separate-bug-chat directive01a20921-a2b9 and this chat's explicit
“Хорошо, делай по плану” authorize the repair: reproduce → select source parts →
prove coordinates → unchanged product owner-run → release handoff. Required parts:
header, Close, query, possible matches, viewport, suggestion row and label.
Donor e19b20c47 / SDK4e8a9968 / UI authority7de6680a remains immutable.
AX/PNG/Xcode geometry substitution and donor instrumentation remain forbidden.

Basis: [registry](../../../specs/README.md) →
[SWIFTUI-PROBE@3](../../../specs/product/swiftui-probe.md),
[WINDOW-PROBE@1](../../../specs/product/window-probe.md), their required closure,
[DEBUG-CAPTURE](../../../specs/product/swiftui-debug-capture.md),
[TRANSFORM](../../../specs/product/swiftui-cache-transform.md),
[NATIVE-OWNER](../../../specs/product/swiftui-native-owner.md), and
[NATIVE-PARTS@1](../../../specs/product/swiftui-native-parts.md).
The latter is the additive Evolve inside the approved fix; old modes stay unchanged.
S13 explicitly handed off the new helper/test/contract. Shared parser/reader/glue
remain S13-owned. No product admission, install or publication is claimed here.

## Current implementation

[swiftui_native_parts.py](../../../../plugins/macos/swiftui_native_parts.py) selects
all native-backed overlay parts in one declared source owner/container. It proves
window/provider/weak bindings and a complete parent path, indexes structural
layoutComputer witnesses once per subgraph, and excludes a toggle outside the list.
It does not select by index, geometry, UI text or model value. Discovery, selected
walk and transform context share1024unique raw records, with bounded cached headers
and protected-range rechecks. The per-class ivar cache revalidates each provider
and weak reference. Existing byte/call/node limits and lifecycle stay unchanged.

Public glue shape: `native_match=overlays_in_owner`, `type`, `containing_type`,
`native_view_type`, `selection=geometry_records`; no leaf_types. Helper entrypoints:
`select_parts(reader,host)`, `cached_read(reader,address,size)`,
`check_budget(reader,element)`, `record_count(reader)` and `verify_parts(reader)`.
Integrating owner must invoke budget checks on both walk and transform-context
visits, use cached_read after normal range validation, and verify every native
witness once after traversal. Provenance source is `native_overlay_parts`, not a
complete body forest. Root/branch order grants no semantic identity.

## Verified evidence

External evidence root `/Volumes/Develop-2TB/Develop/tmp/uib-actors-s03-_9n7zkwx`;
owned internal apps/runtime `/tmp/uib-actors-s03-_34wh1b_`. Ownership notes apply.

- Original open consumer manifest/13Snapshots: only toolbar actor marker;
  `actor-picker-and-authentication-overlays` unmeasured. Real donor source confirms
  rows are inline Button/Text, not separate nominal ActorRow types.
- Seven-part fixture: wide/narrow dimensions and all7root translations match public
  SwiftUI oracle. Duplicate picker refuses. Original contiguous owner candidate
  omitted a split branch; outermost same-body ancestor fixes that fixture
  (`capture-21`,165raw). S13 integrated it in8c810b8 as partial ancestor semantics.
- Source-shaped20-option fixture: whole traversal refuses1024raw. Observed
  `SwiftUI.SystemScrollView` leaf makes top-controls scope bounded. `capture-25`
  proves the same top5frames with onGeometryChange compiled out;420raw.
- `capture-37`: new helper + frozen8c810b8 with temporary fixture-only parser glue,
  real no-oracle cursor fixture. Nine native candidates → eight row branches,
  64nodes;747unique records,488selected records,2840cached header reads,214calls,
  one cached class layout. Zero recorded model-range overlaps, no graph evaluation,
  normal detach. All8row and8label frames have known native-window mappings.
  Installed0.6.1 validator passes; ordinary Rust measure returns8pt leading gap
  for every row/label pair. This is the helper's runtime result, not installed glue.
-20focused helper checks pass: scope/admission, membership, ambiguity, incomplete
  ancestry, duplicate/stale/foreign binding, cycles, depth/record budget, private
  cached reads and per-class reuse. Fixture source compiles under Swift6.
- Stock8c810b8 replay38/39/40 passes split/wide/narrow with all7exact transformed
  frames and normal detach. Wide/narrow share one process;442→302pt query width.
  Public body-owner extensions supply the correspondence. No prototype selector
  patch was used for those three captures. Earlier duplicate10 remains old evidence.

Earlier inactive label transforms were dirty; they remained unknown. After ordinary
query focus, clean transforms were observed, but capture itself reported inactive:
causality is not claimed. Capture33 blocked at task_for_pid then ended
attach_unavailable/0calls after user authorization; old target disappeared.
Fresh fixture captures36/37 succeeded. No outstanding debugger hold from those runs.

## Remaining dependency and next action

S13 integrated/registered the dedicated mode in0080570 and corrected the reported
allocator-extent finding. Source glue review confirms cached-read privacy and union
budget hooks. The helper's per-native selected-window recheck is tightened in this
handoff. Fresh008runtime awaits the shared owner's listener/recovery qualification;
do not attach the product on an unqualified peer. Distribution CACHE_FILES still
needs the new helper/transform/backend modules through its existing build owner.
Repeat exact integrated pin (including backend) before product admission. Consumer prepared
immutable Actors with SWIFTUI_VIEW_DEBUG=29 and get-task-allow; fresh identity and
state are in external consumer-ready.json and must be revalidated by that owner.
Next real case: ready-list parts, normal layout change and scroll through the same
source producer; source component attribution, coordinate mapping and recovery
must all pass. No fixture result closes that acceptance or authorizes publishing
an unverified fix. Keep running owned fixtures/evidence until integrated replay;
cleanup awaits confirmed process exit and consumers releasing these artifacts.
