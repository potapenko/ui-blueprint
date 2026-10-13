# Saved observations and local geometry

Use the absolute `UIB_BUNDLE` selected in [the skill](../SKILL.md). Commands work
outside the source checkout. Examples below use finite sample budgets, not universal
defaults. Input budgets cover all named files together; output includes its newline.

## Known-result smoke example

The distribution includes synthetic examples; these are not live UI observations.

```sh
"$UIB_BUNDLE/uiblueprint" --help
"$UIB_BUNDLE/uiblueprint-validate" --max-bytes 131072 "$UIB_BUNDLE/example-snapshot.json"
"$UIB_BUNDLE/uiblueprint" measure \
  --snapshot "$UIB_BUNDLE/example-snapshot.json" \
  --query "$UIB_BUNDLE/example-query.json" \
  --evaluation "$UIB_BUNDLE/example-evaluation.json" \
  --space local-form --max-input-bytes 131072 --max-output-bytes 131072 --json
"$UIB_BUNDLE/uiblueprint" check \
  --snapshot "$UIB_BUNDLE/example-snapshot.json" \
  --expectation "$UIB_BUNDLE/example-expectation.json" \
  --space local-form --max-input-bytes 131072 --max-output-bytes 131072 \
  --json --result-version 0.2.0
```

Check each command's exit separately: validator valid/0, measured gap **8 css_px**
in `local-form`/0, check pass/0. This establishes the saved-data example only, not
live permissions, platform coverage or automatic skill discovery in Codex.

## Inspect and select

`SNAPSHOT` names one core 0.1.0 Snapshot document or one observed ChannelResponse
document containing a Snapshot. `SOURCE_KEY_JSON` is a strict JSON object copied
from a reported node, for example `{"namespace":"web.dom","key":"33"}` only
if that exact key exists. Never copy illustrative IDs into a real request.

```sh
"$UIB_BUNDLE/uiblueprint" inspect --snapshot "$SNAPSHOT" \
  --ref "$SOURCE_KEY_JSON" --view design \
  --max-input-bytes 1048576 --max-output-bytes 1048576
```

Compact design inspection includes reported component members when exposed;
proximity is not component membership. `--view interaction` selects semantic
presentation. Optional `--json` returns an inspection envelope with the original
Snapshot; that envelope is not itself a Snapshot input for subsequent commands.

## Construct a factual query

Start from the bundled `example-query.json` as a shape example, writing a separate
query in system temp. Replace its scope, targets, anchors, frame kinds, complete
spaces and units with facts from the selected source. Query envelope:
`{"schema_version":"0.2.0","artifact":{"kind":"geometry_query","data":...}}`.
The `data` fields are `id`, `scope_id`, `targets`, `applies_when`, `operation`,
`anchors`, `quantity_kind`, `units`. Use the bundled analysis schema for exact types.

For a horizontal edge gap: operation `gap`, quantity kind `length`, two anchors
with axis `x`, first fraction `1` (right edge), second fraction `0` (left edge).
Each anchor contains `element` (SourceKey), `frame_kind`, `coordinate_space`
(`id`, `kind`, `units`, `origin`), `fraction` and `axis`. Both spaces/frame kinds
must come from the source. Negative gaps stay negative; do not call them padding.
Other supported relations include width, height, distance, aligned, inside,
overflow, intersects, ratio, equal_spacing and baseline, with their schema arities.

```sh
"$UIB_BUNDLE/uiblueprint" measure --snapshot "$SNAPSHOT" --query "$QUERY" \
  --space "$SPACE_ID" --max-input-bytes 1048576 --max-output-bytes 1048576 --json
```

Without `--evaluation`, the engine binds the source and selected space with no extra
transforms or observed conditions. Conditional queries may therefore remain unknown.
An explicit evaluation must match the source ID/revision/context and carry sourced
conditions/transforms; never copy the synthetic evaluation into a live-data request.

## Check a real requirement

`EXPECTATION` is a core 0.1.0 expectation document with the user's or product's actual
requirement, targets, comparison, units, tolerance, applicability and `expected_from`.
Use the bundled core schema and `example-expectation.json` for shape only. Do not
invent an expectation just to run `measure`, or copy fixture thresholds into a product.

```sh
"$UIB_BUNDLE/uiblueprint" check --snapshot "$SNAPSHOT" --expectation "$EXPECTATION" \
  --space "$SPACE_ID" --max-input-bytes 1048576 --max-output-bytes 1048576 \
  --json --result-version 0.2.0
```

Measure JSON is analysis 0.2.0; explicit check result 0.2.0 preserves the complete
evaluation. Schema validation alone is not recomputation: rerun the engine on source
inputs before trusting an externally supplied analysis result as a calculation.

## Compare compatible records

```sh
"$UIB_BUNDLE/uiblueprint" diff --before "$BEFORE" --after "$AFTER" \
  --max-input-bytes 2097152 --max-output-bytes 2097152 --max-entries 100 --json
```

This recipe compares recorded node presence and properties, not the complete graph
or a normative requirement. Preserve both environments and original records. Matching
requires compatible session, target/surface generations, scope, fields and projection;
do not restamp sources to force compatibility. Separate ordinary Mac observations or
Web helper invocations may have observation-scoped identities and fail this gate.
Report that limit rather than inventing correspondence. Absence in a partial record
does not prove deletion. Exit 0 means a complete report, even if differences exist;
exit 4 can mean truncation or incompatible contexts. Inspect coverage/omitted counts.

## Exit meanings

Analysis: 0 known measurement/check pass; 1 IO/internal; 2 invalid input or bound;
3 measured check failure; 4 unknown; 5 unsupported. Validator uses 0 valid, 2 invalid,
1 IO/internal. Export and live observation have their own success/partial meanings.
Keep diagnostic stderr separate from machine stdout; never hide a nonzero result.
