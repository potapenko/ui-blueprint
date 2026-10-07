# Recorded membership and property comparison

`uiblueprint_engine::diff::compare_recorded(before, after, DiffLimits { max_entries })`
validates both canonical Snapshots and requires matching session/target/surface
generations, schema/plugin and scope/projection/field sets. Distinct environment
revisions remain attributed to their original records, as selected by
[CLI-DIFF@2](../specs/product/cli-diff.md). CACHE/Delta contexts_compatible retains
strict environment equality. It compares exact SourceKey then Field, without inference
from labels, rectangles or source order. [CACHE@1](../specs/product/cache.md)
governs the distinction between absence and justified removal.

The borrowed RecordedDiff retains both whole original Snapshots and a bounded Vec
of Difference entries. NodePresence is BeforeOnly or AfterOnly, with the original
Node on the present side. It is record membership, never application creation or
deletion, even for complete source coverage. A node absent on one side contributes
one entry rather than expanding each property into an inferred change.

For matched nodes, Property entries retain both Node references, Field, explicit
Both/BeforeOnly/AfterOnly presence, and optional original Property references.
content_changed covers selection, sensitivity and availability/value/reason changes.
evidence_changed separately covers Evidence and its referenced Observation, including
Evidence inside known geometry transforms. Transform geometry/Space/affine/binding
changes are content; a transform Evidence-only change is not geometry change.
Both flags can be true. Known→Unknown never supplies an old known value as new data.
Unchanged properties are omitted; renewed source evidence is not silently dropped.

Order is deterministic: before nodes and their fields in source order, then
after-only fields for each matched node, then after-only nodes in after order.
No derived index or cloned graph is retained. Two scans count exact differences
and reserve only min(actual count, max_entries) entries fallibly. Zero is allowed;
omitted_entries reports differences excluded by this cap, independent of both
source coverages. The cap is not an acquisition/work/RSS guarantee; canonical
validation retains its existing allocation behavior.

InvalidSnapshot, IncompatibleContext and Capacity errors return no partial result.
Source identity, timestamps, consistency, coverage, reasons and values remain
unchanged. Revision order does not establish chronology or stable cross-snapshot
identity. This comparison neither verifies atomic acquisition nor attributes causes.
Different geometry units/kinds/Spaces stay separate recorded values; no conversion,
common transform or normalized displacement is computed from an environment change.

Scope is node presence and properties only. Relations, children, native_role,
node surface placement, extensions, source declarations, component mappings, focus
and other whole-graph metadata are retained through the original Snapshot but are
not compared by this slice. Empty entries therefore do not prove complete graph
equality. No Delta/Removal, cache/replay mutation, CLI schema, live event listener
or action ref is produced. Future removal generation needs explicit removal evidence.
[Tests](../../crates/engine/tests/diff.rs) and [receipt](../plans/ui-blueprint/receipts/G02-recorded-diff.md)
record current author proof; full diff/changes and live acceptance remain separate.
