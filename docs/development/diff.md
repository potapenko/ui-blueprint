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

## External observations with declared correspondence

[CLI-COMPARE@2](../specs/product/cli-compare.md) adds a separate saved-data command:

```text
uiblueprint compare --before B --after A --diagnose --max-input-bytes 2097152 --max-output-bytes 2097152 [--json]
uiblueprint compare --before B --after A --correspondence C --max-entries 100 --max-input-bytes 2097152 --max-output-bytes 2097152 [--json]
```

Diagnose returns actual differing Context axes, which axes block the old diff, exact
key-match count and supported next steps. Its report contains no UI text or paths.
Exit4 means strict incompatibility, with a useful report. `compare` uses explicitly
selected caller pairs, not identifier/name/box/ordinal matching. Cross-observation
identity remains unverified; the engine never rewrites either source. Canonical
Snapshot or observed ChannelResponse files work directly. No application changes,
fixture session, live capture, permissions or model are needed for these saved files.

C is local correspondence1.0 JSON with exact fields below. Existing canonical types
retain their strict shape; all declarations and IDs are nonempty/max256 characters.
Unknown/duplicate members, nested array records, duplicate endpoints/pair IDs,
dangling refs, wrong namespace/Surface and mismatched original bindings reject.

```text
{
  correspondence_version: "1.0.0",
  before_binding: {snapshot_id, revision, context},
  after_binding: {snapshot_id, revision, context},
  surface_pairs: [{before: Identity, after: Identity, declaration: Id}],
  node_pairs: [{id: Id, before: SourceKey, after: SourceKey, declaration: Id}],
  measurement_pairs: [{id: Id, before_query: GeometryQuery, after_query: GeometryQuery,
                      before_evaluation: EvaluationInput, after_evaluation: EvaluationInput}]
}
```

This is a field/type guide; supply actual JSON values. Each binding must copy that
source's original ID/revision/full Context exactly. Surface pairs cover both full
Surface sets. Equal schema/plugin/Target incarnation/scope/projection/field sets
remain required. Declaration records your QA correspondence assumption; it does not
become reported Evidence. An empty measurement_pairs array is valid. No missing node
or field acquires an old value. Complete0 is a factual report, not normative QA pass.
Unknown measurements or entry truncation give4; invalid/limits2, IO/internal1.

Property entries separate content/Evidence changes and reference original node
indices and fields in both complete unchanged sources. Every selected pair also
exposes reported child counts, unmapped counts, mapped order and each mapped child's
zero-based position in the original parent lists. A mapped subsequence can remain
unchanged while an element moves from recorded position2 to0. Null position means
absent from that recorded parent list. Null content_changed means the full reported
lists were not compared because some children are unmapped. Neither means deletion.
Even all_reported_children_mapped does not prove full UI coverage or visibility.
Native bounds remain accessibility bounds; source partial coverage remains partial.

Compact groups fields under each declared pair, with original keys printed once.
Before/after facts keep exact numbers, availability/reasons, content/Evidence flags
and per-side source references. Repeated Evidence/Space/transform metadata uses local
E/S/T display labels defined once; these labels prove neither identity nor a common
coordinate system. Source Context/coverage/Observations remain attributed on each side.
Reported child lists use indices in that side's unchanged original Snapshot; unresolved
refs retain their original keys. Equal indices across sides are not matching evidence.
Counts, unmapped children, original positions, unknown measurements and omitted/
unexamined coverage remain explicit. Full `--json` output remains byte-identical.

Optional width/height/gap queries and evaluations use unchanged analysis0.2 types
inside C (bare canonical records, not AnalysisDocument wrappers). They must bind to
their own Snapshot and describe the same declared targets/anchors/operation and
units. The engine recomputes both independently and subtracts known scalar lengths.
It does not infer common screen origin, object motion, window-local transforms or
chronology. Missing/redacted/unsupported data retains typed unknown and null change.

### Actual Actor consumer example

The consumer explicitly selected Close7→7, query9→9, viewport12→12, content27→27
and Tom Hardy40→36 from the original CN-B19 pair. The index agreement for some pairs
is not the matching basis. Close/query and Tom Hardy have caller-checked identifiers;
viewport/content are explicitly declared semantic counterparts. The example's
surface declaration also remains unverified. Review those choices for your own QA.
A ready correspondence is handed off with the pinned build; no Window node pair is
needed for this scenario. To prepare the same example from the untouched files:

```python
import json, tempfile
from pathlib import Path
before_file = Path("/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CN-B19-bway_om4/B19-01-minimal/observed.json")
after_file = before_file.parent.parent / "B19-02-minimal/observed.json"
a, b = [json.loads(p.read_bytes())["artifact"]["data"]["result"]["data"] for p in (before_file, after_file)]
pairs = [("close",7,7), ("query",9,9), ("viewport",12,12), ("content",27,27), ("tom_hardy",40,36)]
bind = lambda s: dict(snapshot_id=s["id"], revision=s["revision"], context=s["context"])
assert len(a["context"]["surfaces"]) == len(b["context"]["surfaces"]) == 1
c = dict(correspondence_version="1.0.0", before_binding=bind(a), after_binding=bind(b),
         surface_pairs=[dict(before=a["context"]["surfaces"][0], after=b["context"]["surfaces"][0],
                             declaration="Caller pairs Actor QA surfaces; window identity remains unverified")],
         node_pairs=[dict(id=name, before=a["nodes"][i]["key"], after=b["nodes"][j]["key"],
                          declaration="Explicit Actor consumer selection; not observed identity") for name,i,j in pairs],
         measurement_pairs=[])
for name,i,j in pairs:
    if name not in ("viewport", "content"): continue
    for operation in ("width", "height"):
        queries, evaluations = [], []
        for side,index in ((a,i),(b,j)):
            geometry = next(p["state"]["value"]["value"] for p in side["nodes"][index]["properties"] if p["field"] == "accessibility_bounds")
            space, key = geometry["coordinate_space"], side["nodes"][index]["key"]
            queries.append(dict(id=f"{name}-{operation}", scope_id=side["context"]["scope_id"], targets=[key],
                operation=operation, anchors=[dict(element=key, frame_kind="accessibility_bounds", coordinate_space=space,
                fraction=0, axis="x" if operation=="width" else "y")], quantity_kind="length", units=space["units"],
                applies_when=dict(platform=None,input_mode=None,text_scale=None)))
            evaluations.append(dict(**bind(side),result_space=space,transforms=[],conditions=None))
        c["measurement_pairs"].append(dict(id=f"{name}-{operation}",before_query=queries[0],after_query=queries[1],
            before_evaluation=evaluations[0],after_evaluation=evaluations[1]))
with tempfile.NamedTemporaryFile(mode="w",prefix="uib-actor-correspondence-",suffix=".json",delete=False) as file:
    json.dump(c,file,indent=2); print(file.name)
# This named consumer input is retained for the retry; its consumer owns removal afterwards.
```

Use the printed file as C. Expected saved facts: unchanged Close/query/viewport
bounds, viewport392×360pt, content392×576→392×36pt, content reported children16→1,
Tom Hardy's recorded child position2→0. Scrollarea reported children2→1 includes an
unmapped scrollbar on the before side; after node28 is a different node and is not
paired. The report does not claim15 actors deleted,16 rows visible, painted popup
bounds, checkbox/focus/draft state, or fresh live UI. Both original source hashes
remain unchanged. Strict old `diff` still refuses this pair with context_mismatch.
