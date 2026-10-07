# Explicit relation neighbors

`uiblueprint_engine::scope::relation_neighbors(snapshot, seed, NeighborLimits)`
selects one step of existing incident relations from a stored canonical Snapshot.
This realizes the explicit-links-first part of [PROJECTIONS@1](../specs/product/projections.md),
preserving [MODEL@1](../specs/product/model.md) and [IDENTITY@1](../specs/product/identity.md).
It does not collect UI or construct a new interaction/design projection.

`NeighborLimits { max_relations }` is an explicit output limit, with no default.
Zero returns an empty selection with the number of omitted incident relations.
`NeighborView` borrows the source Snapshot and exact seed Node, and owns only a
Vec of borrowed `RelationNeighbor { relation, counterpart, direction }` entries.
Direction is Outgoing, Incoming or SelfLoop. One entry represents one source
Relation in source order; duplicate links and repeated counterparts remain distinct.
Self-loops appear once per recorded Relation. Full namespace/key equality selects
the seed. No name, geometry, child-list or component-membership inference is used.

`omitted_relations` counts only incident relations excluded by this selection's
output cap. It never replaces source Coverage or claims complete live knowledge.
Snapshot coverage, observation intervals/freshness, projection, source declarations,
redacted/unknown properties and original relation Evidence remain unchanged and
available through the borrowed source. A complete local selection can have an
unknown or partial source. A borrowed counterpart is not a new actionable ref.

The existing canonical validator runs first. Invalid Snapshot, missing seed and
failed output reservation return typed `ScopeError`, without partial output or
input mutation. The function scans stored relations and resolves selected endpoints
in existing nodes; it creates no cloned graph/index or long-lived cache. Output
reservation is fallible and bounded by min(incident count, max_relations).
Validation retains its existing allocations; this output limit is not an acquisition,
execution-time, allocator or RSS bound. Production admission remains the caller's
existing responsibility.

Implementation: [scope.rs](../../crates/engine/src/scope.rs).
Deterministic cases: [scope tests](../../crates/engine/tests/scope.rs).
Check status: [G02 receipt](../plans/ui-blueprint/receipts/G02-scope.md).
Full projections, geometry-based neighbors, CLI integration and live W01/M01/P01
qualification remain separate work. Canonical wire, host, cache and arithmetic
are unchanged.
