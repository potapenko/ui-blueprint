# Declared correspondence between saved observations
- Node type: leaf; domain: `uib.cli.corresponded-comparison`.
- Contract: `UIB.CLI-COMPARE@1`; clauses: `.INPUT`, `.FACTS`, `.STRUCTURE`, `.MEASURE`, `.DIAGNOSE`, `.OUTPUT`, `.FAILURE`.
- Authority: Active / Stability: Evolving; accepted/released baseline: none.
- Authority source: user's direct external-comparison request in [USE-MAC1](../../plans/ui-blueprint/receipts/corpus-native-advice.md#use-mac1--actual-consumer-comparison-feedback-2026-10-13), root's USE-MAC-COMPARE2 implementation selection of [2dd3701 plan](../../plans/ui-blueprint/receipts/consumer-native-comparison1.md), including selected reported-child structure.
- Read when: compare/diagnose saved observations with caller-declared correspondence.
- Do not read when: unchanged exact-key diff suffices.
- Requires: [CLI](cli.md), [CLI-DIFF2](cli-diff.md), [CLI-GRAPH-DIFF1](cli-graph-diff.md), [ANALYSIS2](analysis.md), [IDENTITY1](identity.md), [NATIVE5](native.md); their explicit closures apply.
- Protected: raw/graph/geometry diff, cache/Delta/action/collector, core0.1/analysis0.2, old report versions and constant stderr.

## UIB.CLI-COMPARE.INPUT
```text
compare --before B --after A --correspondence C --max-input-bytes N --max-output-bytes N --max-entries N [--json]
compare --before B --after A --diagnose --max-input-bytes N --max-output-bytes N [--json]
```
B/A accept validated Snapshot or observed ChannelResponse. All explicit files share
one positive input-byte bound. Positive output-byte bound includes newline; entry
cap is nonnegative. Diagnose excludes correspondence/entry cap; duplicate/unknown
flags reject. No reference IO or live collection. C is a strict map-only local
record, with duplicate/unknown members and nested array records rejected:
correspondence_version="1.0.0", before_binding, after_binding, surface_pairs,
node_pairs, measurement_pairs. Binding={snapshot_id,revision,context}; full canonical
Context and ID/revision equal that side's original exactly, including environment.
SurfacePair={before:Identity,after:Identity,declaration:Id}; NodePair={id:Id,
before:SourceKey,after:SourceKey,declaration:Id}. All are required, declaration is
a bounded caller explanation, not canonical Evidence. Pair IDs/endpoints are unique,
one-to-one and existing, namespaces match, nodes belong to paired Surfaces. Surface
pairs cover both full Context Surface sets; node_pairs is nonempty. Equal schema,
plugin, Target incarnation, scope/projection/field sets remain required. Only declared
session/Surface differences relax old read-only compatibility, never stored bindings.

## UIB.CLI-COMPARE.FACTS
Correspondence status is always caller_declared_unverified. No automatic identifier,
name/box/ordinal matching, stable identity, remount continuity or action ref follows.
Engine compares literal properties using raw content/Evidence rules. Both original
keys, records, availability, clocks, freshness/coverage/consistency stay unchanged.
Map order then before-field order/after-only fields determines property entries.
Unselected nodes and metadata/relations/components/focus remain explicitly unexamined;
missing records/fields are never created/deleted or filled from an old known value.

## UIB.CLI-COMPARE.STRUCTURE
Each selected node pair also returns one reported-children entry, including unchanged
lists. Preserve the original ordered SourceKeys on each side through the referenced
nodes. Compare their subsequences of explicitly mapped children by pair ID/order;
mapped_order_changed is a factual comparison conditional on caller correspondence.
Report per-side unmapped child counts. content_changed is the mapped-order result
only when every reported child is mapped; otherwise null with status=reported_subset.
All-mapped status=all_reported_children_mapped still means reported lists only,
never complete UI structure or deletion evidence. Source partial coverage stays separate.
No inferred parent, sorted order, unmapped-child identity or full-graph equality.

## UIB.CLI-COMPARE.MEASURE
MeasurementPair={id:Id,before_query:GeometryQuery,after_query:GeometryQuery,
before_evaluation:EvaluationInput,after_evaluation:EvaluationInput}. These existing
canonical records are independently bound and computed by engine measure_query_bound.
Initial operations width/height/gap; require corresponding mapped target/anchor order,
equal operation/frame kind/fraction/axis/applicability/quantity units. Result Spaces
must have equal kind/origin/units; their IDs and origins are not a common world binding.
Two known scalar lengths yield finite after-minus-before; either unknown preserves
its exact reason/Evidence and null change. Original full Spaces/queries/evaluations
and results stay attributed. This compares independently measured lengths, not dx/dy,
verified object motion, common monotonic time, inferred window transforms or UI cause.

## UIB.CLI-COMPARE.DIAGNOSE
Separate comparison_diagnostic1.0 report contains fixed differing_axes and
strict_diff_blocking_axes, protected_basis_compatible, strict_diff_compatible,
before_nodes/after_nodes/exact_key_matches and fixed next_steps. No source/paths/UI
labels/config/AXIdentifier strings. Next steps name supported exact-key diff or
explicit correspondence/per-side measure, or protected_basis_incompatible. Compact
explains the original binding and partial-scope requirements, not just a refusal code.

## UIB.CLI-COMPARE.OUTPUT
JSON corresponded_difference1.0: output_version,kind,source="saved",
live_revalidated=false,correspondence_status,comparison_scope,limitations,
before/after (full unchanged Snapshots),correspondence (original C),entries,
measurements,unexamined_before_nodes,unexamined_after_nodes,omitted_entries.
Entries use kind=property|children,pair_index,before_index,after_index. Property
adds field,before_present/after_present,content_changed/evidence_changed. Children
adds mapped_order_changed,unmapped_before_children/unmapped_after_children,status,
content_changed. Measurements use pair_index,before/after results,change (number/null).
Entry cap applies to combined entries and measurements in that order; exact omission
count is separate from source coverage. No imported-report parser or second graph.
Compact escapes untrusted text and shows actual properties/child sequences/measurements
and unexamined counts. Entire bounded encoding precedes stdout publication.

## UIB.CLI-COMPARE.FAILURE
Complete report0 is not normative QA pass; unknown measurement or truncation4.
Diagnose returns0 only for strict-compatible sources, otherwise4 with its report.
Malformed/binding/pair/protected-basis/limit failures2; IO/allocation/nonfinite internal
calculation1. Constant sanitized stderr, no prepublication partial stdout. Physical
write failure may be partial/IO1. Accepted plan's raw-diff mismatch remains4/empty.
Acceptance: original Actor pair; mapped/unmapped/reordered children; independent
scalar math; binding/duplicate/dangling/namespace/Surface/Target guards; unknowns,
redaction/empty/Evidence, finite arithmetic, byte/entry limits and old-mode regressions.
