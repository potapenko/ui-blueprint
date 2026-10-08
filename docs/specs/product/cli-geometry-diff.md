# Selected-space recorded geometry comparison

- Node type: leaf; domain: `uib.cli.geometry-diff`; contract: `UIB.CLI-GEOMETRY-DIFF@1`.
- Authority: Active / Stability: Evolving; accepted/released baseline: none.
- Authority source: approved PLAN.UIB@1 P3/P6 and [root-selected G12](../../plans/ui-blueprint/packets/G02-recorded-diff.md#g12-geometry-comparison-in-a-selected-space--2026-10-08).
- Read when: opt-in diff --geometry on saved records.
- Do not read when: unchanged raw diff is sufficient.
- Requires: [CLI@13](cli.md), [CLI-DIFF@2](cli-diff.md), [ANALYSIS@2](analysis.md), [GEOMETRY@1](geometry.md); explicit closure applies.

## UIB.CLI-GEOMETRY-DIFF.INPUT

```text
diff --geometry --before FILE --after FILE --ref SOURCE_KEY_JSON --frame-kind KIND --space ID --max-input-bytes N --max-output-bytes N [--before-evaluation E] [--after-evaluation E] [--json]
```

KIND is layout_bounds/accessibility_bounds/hit_region/visible_region/paint_bounds.
Exact canonical SourceKey selects one recorded node; no name/box/Surface matching.
Both files accept core0.1 Snapshot or observed ChannelResponse. All named regular
files plus selector UTF-8 bytes share one positive aggregate input budget. Output
cap includes newline and bounds full encoding before stdout. No embedded ref IO.
Raw mode still requires max-entries; --geometry excludes max-entries. Geometry
flags/evaluations are invalid without --geometry; duplicates/unknown flags reject2.

Reuse ANALYSIS.CLI Space discovery/default EvaluationInput on each side. ID selects
an unambiguous complete existing Space; optional E is canonical analysis0.2
EvaluationInput strictly bound to that side and agreeing with ID/full definition.
No extra transforms/conditions if omitted. Source records are never restamped.
Both selected result Spaces must match fully, not by ID alone. Raw source-compatible
session/plugin/Target/Surface generations/scope/projection/fields hold; environment
differences may exist between records but each evaluation/mapping binds its own.

## UIB.CLI-GEOMETRY-DIFF.RESULT

Resolve each frame through existing directional/all-corner evidenced paths. No
guessed inverse/scale/origin/source identity, tolerance or polygon envelope claim.
Per-side known Rect retains contributing Evidence; unknown retains its exact reason
and reached Evidence. Missing node/property/mapping remains unknown, never zero.
Only two known resolved Rects yield finite signed after-minus-before dx/dy/dwidth/
dheight in selected Space units. No norm/pass-fail, causality/global UI/layout
claim follows. Choosing screen measures screen motion; local/document Space can
separate sourced window/scroll movement from rect changes in that chosen Space.

JSON is a CLI-owned envelope with exactly output_version="1.0.0",
kind="geometry_difference", source="saved", live_revalidated=false,
selector, frame_kind, before/after (full original canonical Snapshots),
before_evaluation/after_evaluation (unchanged canonical EvaluationInputs),
result_space (full Space), before_geometry/after_geometry, displacement.
Geometry is {status:"known",rect:Rect,evidence:[Evidence]} or
{status:"unknown",reason:existing MeasurementUnknownReason,evidence:[Evidence]}.
displacement is {dx,dy,dwidth,dheight} only for both known, otherwise null.
This versions only the local selection report, not core0.1/analysis0.2. No second
graph/parser/imported-result framework. Serialize borrowed originals boundedly.
Compact presents original attribution/coverage, each resolved state/Space/Evidence
and displacement or unavailable; untrusted strings escaped. No source upgrade.

## UIB.CLI-GEOMETRY-DIFF.FAILURE

Known0 means finite factual pair, including zero; unknown4 emits the report with
no displacement. Incompatible sources/full result Spaces4/context_mismatch emits
no output; wrong binding/malformed/unknown or ambiguous Space/input/output bounds2.
IO/allocation/nonfinite internal calculation1; unsupported finite mode5. Physical
stdout write failure may be partial/IO1; all prepublication failures stdout-empty.
Existing raw diff JSON1.0.0, arithmetic/schema/cache/Delta/other CLI modes unchanged.

Acceptance: sourced screen→local translation, viewport→document scroll, resize,
missing transforms/unknown frames, strict per-side binding and same-ID/full-Space
mismatch, original source/Evidence/units/partial/redacted preservation, aggregate
bounds and raw diff/measure compatibility. Synthetic literals are not live proof.
`G12-GEOMETRY-DIFF-001`: additive explicit-space consumer selected before code;
CLI@12→@13/registry24→25; no canonical shape/wire/collector or raw-mode change.
