# Controlled-data atomic replay

K01-replay provides `uiblueprint_engine::replay::replay(base, delta, snapshot_id)`.
It implements the existing [CACHE contract](../specs/product/cache.md) on canonical
immutable `Snapshot`/`Delta` values. It performs no IO, clock read, collection,
subscription or retention. It does not select memory limits or implement a cache.

## API and publication

The caller supplies the new Snapshot ID because canonical Delta carries revisions
but no result ID. The ID is distinct from the base, nonempty and within the
canonical 256-character limit. Revision, source_state, context and coverage come
from the Delta; no source_state or freshness proof is invented.

1. The existing shared `validate_delta` checks the base, full compatibility,
   revision step, complete selected-field upserts, justified removals and graph.
2. Replay builds a separate candidate. A matching source key replaces the entire
   Node, including its properties/children/extensions/declarations. Existing node
   order is preserved, explicit new nodes append in Delta order, and only named
   validated removals delete nodes. Unknown/unsupported/redacted/false/empty are
   copied exactly, never merged with old values.
3. Delta relations and focus replace their complete prior records together with
   node/children changes. Partial scope, missing upserts, projection filtering or
   an empty change list never implies deletion. Scope/fields/projection changes
   require a compatible full snapshot rather than a merge across contexts.
4. Shared `validate_snapshot` checks the complete candidate before it is returned.
   Failure publishes nothing; both borrowed inputs remain unchanged.

`ReplayError::ResyncRequired` distinguishes context mismatch, revision mismatch,
unresolved references and Observation ID conflict. Other canonical input/result
validation failures preserve their typed `ValidationError`; malformed caller IDs
return `InvalidSnapshotId`. Errors contain no private payload or original input.
There is no second wire schema or independent replacement validator.

## Evidence and metadata

Observation identity is immutable for replay. Identical records with the same ID
are reused; a changed record reusing an old ID is rejected instead of restamping
old properties. Delta Observations keep their input order. Referenced older
Observations follow in base order; old records no longer referenced are omitted.
This ordering is deterministic, not a comparison of cross-process clocks.

Reference collection covers node/extension properties, geometry transforms,
source-declaration geometry, relations, focus/selection/composition, surface
records and captures. Evidence values and original intervals/source/freshness
are preserved. Semantic updates do not refresh retained pixel measurements.
Unrequested history remains in the original base; no historical value is injected
into a fresh requested property. The supplied Delta remains the operation record
for removal evidence; the output Snapshot represents materialized state.

Delta has no component-mapping, surface-record or capture replacement fields.
Replay retains those base records unchanged with their evidence. If a removal
leaves a retained component member or surface anchor dangling, replay returns
resync. The caller must supply a full Snapshot with the intended metadata; replay
neither invents a cleanup nor silently removes historical captures.

## Evidence and boundaries

The accepted [R03 source ledger](../research/R03-core.md) supplies AccessKit's
full-node replacement/coherent state comparison mechanism and its limitations.
This is original Rust code against UI Blueprint contracts, with no upstream copy,
implicit reachability deletion, panic-based validation or foreign dependency.
R03 U1/U2 and the unchanged [independent oracle corpus](../../fixtures/golden-oracles/README.md)
supply replacement, reparenting, context, removal and full-state expectations.

Tests compare the independently authored full source snapshot with replay at one
recorded source checkpoint; they do not compare two sequential live captures.
Additional focused tests cover false/empty/unavailable replacements, all currently
representable context dimensions, revision overflow, atomic references/focus,
unchanged base after rejection, explicit empty scope and old capture provenance.
Unsupported schema versions are rejected by the existing canonical parser; the
current typed enum has only the one candidate version.

The five protected schema/plugin review findings remain outside this owner and
are not repaired or accepted by these tests. Production retention, memory/queue
limits, accounting, eviction and cache lifecycle require D05 and a later bounded
handoff. This finite pure replay candidate does not complete K01/S01 or runtime
acceptance. Commands/checkpoint status are in the [receipt](../plans/ui-blueprint/receipts/K01-replay.md).
