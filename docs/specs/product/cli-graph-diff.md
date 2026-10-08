# Recorded source graph comparison

- Node type: leaf; domain: `uib.cli.graph-diff`; contract: `UIB.CLI-GRAPH-DIFF@1`.
- Authority: Active / Stability: Evolving; accepted/released baseline: none.
- Authority source: approved PLAN.UIB@1 G02/P6 and delegated representation in [G13](../../plans/ui-blueprint/packets/autonomous-tasks-2026-10-08.md#g13--сравнение-структуры-связей-и-фокуса).
- Read when: implementing or using explicit saved graph comparison.
- Do not read when: existing raw property or selected-space geometry diff suffices.
- Requires: [CLI](cli.md), [CLI-DIFF@2](cli-diff.md), [PROJECTIONS@1](projections.md), [FORMS@1](forms.md); their explicit closure applies.

## UIB.CLI-GRAPH-DIFF.INPUT

```text
diff --graph --before FILE --after FILE --max-input-bytes N --max-output-bytes N --max-entries N [--json]
```

Files, aggregate positive input/output byte bounds, canonical validation, compatible
source Context (allowing distinct environments), no embedded reference IO and
nonnegative entry cap follow CLI-DIFF@2. --graph is mutually exclusive with
--geometry and every geometry-only flag; duplicate/unknown flags reject2.
Core0.1/analysis0.2 and default raw JSON1.0.0 remain unchanged.

## UIB.CLI-GRAPH-DIFF.COMPARE

Compare node presence/properties using existing raw semantics, then matched nodes'
ordered children and metadata (surface, native_role, extensions, source_declarations),
relations, component mappings and five focus axes: keyboard, accessibility,
active_descendant, text_selection, composition_state. Original records stay borrowed.
Exact SourceKey matches nodes; no label/geometry matching or action authority.
Relations match (kind, from, to) plus zero-based occurrence within that tuple, since
canonical records allow duplicates. A changed endpoint/kind gives two missing-side
records, not guessed continuity. Components match exact logical_component_key.
Members, children and metadata arrays compare literally in reported order; no
sorting, inferred parent, inferred component membership or reconstructed tree.
Node/property entries retain raw order; then matched-node children/metadata in before
order, before relations then after-only relations, before components then after-only
components, then the five focus axes in the order above. Absent nodes produce one
presence entry, without fabricating their missing properties/metadata as known empty.

Content and evidence flags are independent. Properties/extensions/composition use
raw content/Evidence rules. Relation content is its tuple; evidence includes its
Evidence and referenced Observation. Component members are content, declaration_source
and provenance are evidence metadata. Node surface/native_role are content;
extension values and declaration namespace/name/state/sensitivity are content,
extension Evidence/Observation and declaration source are evidence. Metadata array
order is literal content. Geometry-bearing declaration state uses raw transform
content/evidence separation. Children have no standalone Evidence. Focus refs compare
status/target/reason as content and Evidence/Observation separately; known none,
unknown and not_requested stay distinct. Text selection compares anchor/focus/units
as content, Evidence/Observation separately; absent optional selection is not empty.
Missing-side entries have content_changed=true; evidence_changed reflects differing
available evidence (node presence retains raw true/false flags).

Missing means absent in that saved record, never deleted/created, even under complete
coverage. No source freshness/chronology/atomicity/cause claim, Delta or cache mutation.
Snapshot/Observation metadata, surface_records, captures and source_state are retained
but not standalone compared domains; no complete Snapshot equality claim. Observations
referenced by compared Evidence participate in evidence_changed.

## UIB.CLI-GRAPH-DIFF.DATA

JSON has exactly output_version="1.0.0", kind="graph_difference", source="saved",
live_revalidated=false, comparison_scope="nodes_properties_children_metadata_relations_components_focus",
before/after (full unchanged canonical Snapshots), entries and omitted_entries.
Each entry has kind, before_index, after_index, field, before_present, after_present,
content_changed, evidence_changed. kind is node_presence/property/children/node_metadata/
relation/component/focus_keyboard/focus_accessibility/focus_active_descendant/
focus_text_selection/focus_composition. Indices address each side's nodes/relations/
components array; focus indices are null. field is canonical Field only for property,
otherwise null. Property indices address nodes even when that field is absent;
presence flags disambiguate. Focus indices are always null.
Entries reference originals rather than copying a second graph. Output is one complete
bounded object plus newline. Compact gives escaped original attribution, coverage,
omission count, flags and changed before/after facts; no terminal controls from input.

## UIB.CLI-GRAPH-DIFF.FAILURE

Complete report0, entry-truncated report4 with exact omitted count independent of
source coverage; incompatible sources4/context_mismatch with empty stdout.
Invalid input/flags/bounds2; IO/allocation1. Prepublication failure leaves stdout empty;
physical write failure may be partial. Diagnostics contain fixed codes, no raw paths/UI.
Acceptance: independent literals for every kind, evidence-only, all focus states,
partial/missing, namespaces/generations, duplicate relations, source immutability,
redacted/unknown/empty, exact entry/byte bounds and old raw/G12 regressions.

`G13-GRAPH-DIFF-001`: Restore G02 scope with delegated additive CLI representation,
CLI@13→@14 / registry26→27 before code. Existing raw/G12 contracts, schema, cache,
collectors and live qualifications protected. Registration is not independent acceptance.
