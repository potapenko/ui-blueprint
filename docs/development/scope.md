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
Full projections, geometry-based neighbors and live W01/M01/P01
qualification remain separate work. The public caller is described below. Canonical wire, host, cache and arithmetic
are unchanged.

## Public saved-data neighbor caller

[CLI-NEIGHBORS@1](../specs/product/cli-neighbors.md) exposes the accepted borrowed
API directly, with an explicit relation-entry cap and no spatial inference:

```sh
uiblueprint neighbors --snapshot observation.json \
  --ref '{"namespace":"web.dom","key":"7"}' --max-relations 1 \
  --max-input-bytes 200000 --max-output-bytes 200000 --json
```

Use an actual SourceKey from the selected source; the example key belongs to the
historical F01 D05 response used for this caller proof. FILE accepts a canonical
Snapshot Document or one observed ChannelResponse, using the existing CLI loader.
No live collection, source-ID discovery, action ref or separate graph is created.

Compact output is default. JSON is a CLI-owned envelope output_version1.0.0,
kind=relation_neighbors, source=saved, live_revalidated=false, selector,
snapshot (full unchanged source), selection {max_relations, returned_relations,
omitted_relations, truncated}, neighbors [{direction, relation, counterpart}].
Direction is outgoing/incoming/self_loop; Relation and counterpart Node are the
unchanged canonical objects, including Evidence and unavailable properties.
A cap of0 is valid and returns no selected edges but counts every omitted incident
edge. Repeated links/counterparts are retained; absence of an explicit edge does
not disprove a relationship in the live interface.

Source coverage remains independent of selection. For the saved F01 sample, key
web.dom:7 with cap1 returns one outgoing corresponds_to edge to web.ax:7 and reports
one omitted edge; the full32-node/17-relation Snapshot still says partial with
unknown_count=null. It does not become a complete UI or an action-ready mapping.

max-input-bytes bounds source plus selector UTF-8 bytes. max-output-bytes bounds
the entire response including newline; the full source is included even when
selection is capped, so byte overflow rejects2 rather than silently dropping
source facts. Cap limits output selection, not acquisition/CPU/RSS. Exit0 means
selection produced (including partial source/capped output),4 missing exact seed,
2 invalid/limit,1 IO/allocation failure. No partial stdout before validation/encode
failure; constant sanitized diagnostics on stderr. No output files are created.
Existing inspect's JSON envelope/compact behavior and all other commands remain.
