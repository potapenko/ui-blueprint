# Recorded graph, property and geometry comparison

## Opt-in structure and focus comparison

[CLI-GRAPH-DIFF@1](../specs/product/cli-graph-diff.md) adds `diff --graph` with
exactly the raw mode's before/after, byte budgets and max-entries arguments.
`uiblueprint_engine::diff::compare_graph` borrows the same validated Snapshots,
reuses raw property comparison, then compares children, node metadata, relations,
component mappings and keyboard/accessibility/active-descendant/text-selection/
composition focus. No cache or second graph is created. JSON `graph_difference`
1.0.0 references source-array indices and property fields; both full sources remain.
Missing fields/nodes/relations/components are absent records, never deletion.

Children/members/metadata arrays are literal ordered records. Relations match exact
(kind, from, to) plus occurrence for duplicates; components match their reported key.
Node metadata includes surface/native_role/extensions/declarations. Values and source
Evidence/Observation changes have separate flags, including focus and extension data.
Source coverage and entry omissions are separate; complete report0, truncated4,
incompatible context4, invalid/bounds2, IO1. Raw JSON and G12 remain unchanged.
An empty graph report does not compare standalone surface_records/captures/envelope
metadata or prove atomicity/freshness. Full selected semantics and limitations are
in the linked contract. [G13 receipt](../plans/ui-blueprint/receipts/G13-graph-diff.md)
records author verification, not independent acceptance or live collection.

Runnable synthetic example from the repository root with `uiblueprint` on PATH:

```sh
python3 - <<'PYEXAMPLE'
import copy, json, pathlib, subprocess, tempfile
source = json.loads(pathlib.Path('fixtures/golden/GEO-SIZE-RATIO__width.json').read_text())
before = source['artifact']['data']['snapshot']
evidence = before['nodes'][0]['properties'][0]['evidence']
before['focus']['keyboard'] = {'status': 'known', 'target': before['nodes'][0]['key'], 'evidence': evidence}
after = copy.deepcopy(before)
after['focus']['keyboard'] = {'status': 'none', 'evidence': evidence}
with tempfile.TemporaryDirectory(prefix='uib-graph-example-') as directory:
    paths = [pathlib.Path(directory) / name for name in ('before.json', 'after.json')]
    for path, snapshot in zip(paths, (before, after)):
        path.write_text(json.dumps({'schema_version': '0.1.0', 'artifact': {'kind': 'snapshot', 'data': snapshot}}))
    subprocess.run(['uiblueprint', 'diff', '--graph', '--before', str(paths[0]),
                    '--after', str(paths[1]), '--max-input-bytes', '65536',
                    '--max-output-bytes', '65536', '--max-entries', '10'], check=True)
assert not pathlib.Path(directory).exists()
PYEXAMPLE
```

Expected: one `focus_keyboard` content change from known target to known none,
no evidence change, zero omissions and exit0. Add `--json` for the separate envelope.
These generated records are synthetic; the example performs no UI action or capture.

## Opt-in geometry in a selected Space

[CLI-GEOMETRY-DIFF@1](../specs/product/cli-geometry-diff.md):

```text
diff --geometry --before FILE --after FILE --ref SOURCE_KEY_JSON --frame-kind layout_bounds --space ID --max-input-bytes N --max-output-bytes N [--before-evaluation E1] [--after-evaluation E2] [--json]
```

Use one exact recorded key/frame. No max-entries in this mode. Optional canonical
analysis0.2 evaluations bind independently; without them, reuse measure's existing
Space discovery/default evaluation. Original records/coverage/evidence unchanged.
Raw command/JSON below remains exact; normalization is never silently inferred.

`diff::compare_geometry(before,after,key,frame_kind,before_input,after_input)` reuses
the directional/all-corner resolver, compares full result Spaces and returns
known Rect/Evidence or precise per-side unknown. Only two knowns give finite
signed after-minus-before dx/dy/dwidth/dheight in selected units. Source screen/
viewport motion can disappear in sourced local/document Space, without a cause,
all-UI or layout-change claim. Missing paths are unknown, never zero/guessed inverse.
Source/binding incompatibility refuses; cache/Delta compatibility remains strict.
JSON1.0.0 kind geometry_difference retains originals/evaluations/result_space,
resolved states and displacement/null. Known0, unknown4/report, incompatible4/empty,
invalid/bounds2, IO/internal1. Full bounded encoding precedes stdout; no live read.

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
