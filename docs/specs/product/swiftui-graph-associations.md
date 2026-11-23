# Addressed SwiftUI graph associations

- Node type: leaf; domain: `uib.swiftui-debug-capture.graph-associations`.
- Contract: `UIB.SWIFTUI-GRAPH-ASSOCIATIONS@2`; Authority: Active; Stability: Evolving.
- Authority: approved S03/S13 repair plans and the explicit shared-helper handoff
  on2026-11-23 after real S13 refused `cache_association_limit` before selection.
- Requires: [debug capture](swiftui-debug-capture.md),
  [PRIVACY@1](privacy.md), [IDENTITY@1](identity.md).
- Acceptance: addressed hash lookup verified on owned fixture; dependency topology
  disproved as equivalent to recorded UI tree. Nonempty UI topology refuses.

## ASSOCIATIONS.ADDRESS

On the already admitted exact AttributeGraph image profile, read only associations
needed by the selected source path. Do not eagerly materialize the graph-wide
association map or subgraph array. A large global cardinality alone is not a
refusal; bounded actual work and allocation extents still apply.

The retained TreeData hash table is graph+0x118. Its bucket pointer/count, first
node and size are at0/8/16/24. Lookup the requested subgraph using the exact
pointer-hash arithmetic and bucket-predecessor chain verified from this profile's
find accessor. Do not substitute pointer modulo bucket count. Validate retained
table/bucket/node allocation extents before reads. Stored hash, exact key and
bucket membership must agree; cycles or incomplete chains refuse.

An entry stores owner key at16 and its link vector pointer/count/capacity at24/32/40.
Only that owner's bounded link pairs are read. Parent element ownership is checked;
no model payload or AG getter/sort/update is invoked. Allocation metadata may use
the already qualified fixed malloc_size call under the original call deadline.

## ASSOCIATIONS.TOPOLOGY

Subgraph parent links use the observed indirect-pointer-vector at+0x38: zero or
one even pointer; a tagged vector object has data/count at+0x20/+0x28, with null
data denoting its inline pointer storage. Direct children use pointer/count/capacity
at+0x40/+0x48/+0x4c and the low two tag bits are removed from child pointers.
Qualify graph ownership and invalid-state flags before inspecting referenced content.
For every selected live child, require a reciprocal parent relationship. Invalid
children may be excluded as explicitly invalid; invalid selected owners/parents refuse.

Actual SG parents/children describe dependencies, not necessarily the recorded UI
tree. Owned-fixture equality checking found a recorded UI child whose SG is its
owner's sibling; actual-parent lookup also missed4of11recorded parent relations.
Private dependency candidate helpers remain diagnostic only. Public
`parent_attachments` refuses except at the exact selected graph root;
`children` returns known empty only when the exact owner's association map has no
pair for that element, otherwise refuses. Do not grow an ancestor-family heuristic
or present a subset as complete. Any replacement requires a source-qualified
mechanism and actual equivalence before enabling nonempty topology.

## ASSOCIATIONS.BOUNDS / COMPATIBILITY

All helper state is observation-owned. At most1024 actually touched structural
records, plus the existing2MiB source-read, depth, call, output and node bounds.
Cache addressed metadata within the same stopped observation; do not persist raw
pointers or treat cached results as later live evidence. Expose touched element
offsets separately from metadata work so the collector can charge their union
with source selection/traversal, rather than resetting budgets per lookup.

This is Restore of bounded component acquisition, not permission to increase
quotas, evaluate graph callbacks, read other windows, change a donor or reinterpret
unknown geometry. Existing canonical formats, target/backend admission, recovery,
AX/SDK routes and model exclusions remain unchanged.

## ASSOCIATIONS.ACCEPT

Required checks: exact pointer-hash vectors; sparse large table lookup with no
whole-map reads; hash collisions and both bucket reductions; inline/tagged parent
vectors; malformed allocation/cardinality/cycles; foreign/invalid subgraphs;
duplicate/ambiguous links; cumulative work refusal. Compare addressed parents and
children against the prior eager reader on owned fixtures before using this
topology for product selection. Any mismatch remains unqualified and refuses.
Then repeat the original consumer case without changing its caps or source.
@2 reconciles the initial topology hypothesis with failed live equality. It keeps
the independently verified pointer hash/owner lookup and makes the unsupported
relationship explicit; no new read scope or source-call permission is created.
