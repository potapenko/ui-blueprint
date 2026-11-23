# Native-backed SwiftUI part selection

- Node type: leaf; domain: `uib.swiftui-debug-capture.native-parts`.
- Contract: `UIB.SWIFTUI-NATIVE-PARTS@1`; Authority: Active; Stability: Evolving.
- Authority: direct S03 repair plan approval in chat
  `01a20923-023c-7cd0-8421-a2e20177df71`, 2026-11-23, and its shared-reader
  handoff from S13 `01a20922-f6ad-7651-997b-cef1b3f09b12`.
- Requires: [debug capture](swiftui-debug-capture.md),
  [native owner](swiftui-native-owner.md), [PRIVACY@1](privacy.md),
  [GEOMETRY@2](geometry.md), [IDENTITY@1](identity.md).
- Read when: collecting repeated native-backed parts inside one SwiftUI owner.
- Accepted/released baseline: none. Consumer admission/review remain separate.

## PARTS.SCOPE

Explicit `scope.native_match="overlays_in_owner"` requires `type`,
`containing_type`, `native_view_type`, and `selection="geometry_records"`.
No `leaf_types`, model predicates, label values, coordinates, index or first-match
selection. The source owner declares the native representable's role and enclosing
container. Enumerate all matching native views in the selected hosting root under
the existing128-view bound. Verify each view's exact selected NSWindow membership,
provider and current weak layout reference; incomplete enumeration refuses.

Find exactly one graph-owned layoutComputer witness for every candidate. Follow
each witness's validated parent/subgraph path to the exact graph root. The nearest
recorded `SwiftUI._OverlayModifier` is the candidate part boundary; it must have
the requested proper container ancestor and one source body occurrence of `type`.
Skip only complete paths proving nonmembership. Missing, foreign, stale, cyclic,
ambiguous or incomplete paths refuse. All selected parts must belong to the same
body occurrence. Equal native classes or source type names alone are insufficient.

Return every qualified part subtree. Do not pick one row by its ordinal or
rectangle. Distinct roots must not overlap; each native witness must occur exactly
once in the selected walk. This is partial mounted-part coverage, not an entire
owner, list, visible region or application. Virtualized/unmounted rows remain absent
from coverage. Native cursor frames are not SwiftUI layout bounds.

## PARTS.BUDGET

Within one stopped-process observation, cache at most1024 validated32-byte
TreeElement structural headers. Resolve candidate witnesses once per subgraph;
cache type/owner facts per record rather than rereading them for every candidate.
Index, parent paths, selected traversal and transform context share a union of at
most1024 unique raw records. Depth, byte/call/output/node limits remain unchanged.
Reuse a header only after graph/page validation and protected-range checks. Recheck
protected ranges on every cache hit. No payload/model cache, live subscription,
cross-capture reuse, persisted address or widened memory-read authority is added.

Immutable per-class instance extents and runtime ivar offsets may be cached only
inside the same observation. Validate descriptor/window/provider/weak generation
for every represented view. Do not cache a view's weak handle or geometry across
captures. Clear caches with the reader; an instance cache is not a global registry.

## PARTS.OUTPUT / FAILURE

Use existing canonical core0.1 geometry, original local spaces and qualified
transforms. Dirty transforms remain unknown; never evaluate graph callbacks or
copy an ancestor's mapping over a dirty child. Preserve source timestamps and
partial coverage. No actor ID or semantic row identity is inferred from this path.

The selection provenance source is `native_overlay_parts`, with one branch per
selected native-backed overlay, opaque observation-scoped branch/occurrence IDs
and exact source type. It explicitly differs from `body_owner` forest and
`native_owner_ancestor`. Array position confers no semantic or action authority.
Ambiguity, missing witness, incomplete scan, stale binding and quota exhaustion
publish no successful partial document. Native mapping and backend recovery use
their existing governing contracts without new attach or termination permission.

## PARTS.ACCEPT

Required: same-class rows plus an out-of-container toggle; duplicate/stale/foreign
bindings; two source owners; incomplete ancestry; sibling/parent cycles; depth and
record limits; protected cache hits; no-oracle source-shaped fixture; source row
and label geometry across ordinary focus/scroll/resize; canonical/Rust consumption.
Product qualification needs the unchanged Actors donor and owner-run evidence.
Review of new selection/privacy/shared accounting precedes product admission.

`S03-NATIVE-PARTS-001` is an additive Evolve inside the approved repair outcome:
replace failing whole-owner traversal with explicit native-backed part scope.
Old scope meanings, SDK/AX paths, wire versions, privacy and release boundaries
remain protected. Active contract registration is not runtime acceptance.
