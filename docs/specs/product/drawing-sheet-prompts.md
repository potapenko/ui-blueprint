# Separately executable drawing sheets

- Node type: leaf; domain: `uib.drawing.sheet-prompts`; contract: `UIB.SHEET-PROMPTS@2`; supersedes @1.
- Authority: Active / Stability: Evolving; accepted/released baseline: none.
- Authority source: root's explicit CP4 shipping_product packet, 2026-10-11, under
  user-authorized CORPUS-1000 and existing EXPORT multi-sheet requirement; [CV1](../../plans/ui-blueprint/receipts/corpus-visual-diagnostic1.md) is evidence, not authority.
- Revision2 authority: direct user approval on 2026-10-13 of [USE-AUTO-DETAIL1](../../plans/ui-blueprint/receipts/consumer-auto-detail1.md), plan checkpoint02219e9; bounded automatic-tail evolution below.
- Read when: compiling a single document/propose/detail view with planned detail sheets.
- Do not read when: single-sheet, multi-view, compare or flow output is unchanged.
- Requires: [EXPORT@2](export.md), [DRAWING-PACKAGE@3](drawing-package.md),
  [DRAWING-STYLE@2](drawing-style.md), [DRAWING-GEOMETRY@2](drawing-geometry.md),
  [PRIVACY@1](privacy.md), [CLI-EXPORT@3](cli-export.md).

## UIB.SHEET-PROMPTS.REPRESENTATION

Reuse the existing explicit detail requests and components_per_detail density
limit; no new input field or CLI switch. A single document/propose/detail view
whose existing plan has detail sheets additionally writes `prompt-G01.txt`,
`prompt-D01.txt`, etc. Each is a complete literal model instruction for one
identified sheet, with source/status/units/scope and its assigned source facts.
The six base files remain. `prompt.txt` remains the full combined assignment;
separate generation uses the enumerated sheet files, not a concatenation or an
agent-authored wrapper. Source and dimensions stay exact. Preserve the original
sheet plan except for the bounded AUTO-TAIL rule below; explicit details stay exact.

Such packages and CLI receipts use export version0.4.0. Manifest adds
sheet_prompts entries containing sheet_id, view, file, component_ids and
dimension_ids, plus required_dimension_ids for the selected human presentation.
This versions the export package only, not core0.1/analysis0.2 or input grammar.
The 0.4 packaging rule supersedes older 0.1/0.2/0.3 package/receipt selection for
this case only; existing no-split/multi-view/compare/flow version selection stays.
Existing consumers can still read six base files; consumers requiring an exact
old file set/version must recognize0.4 before using the new sheet files.

## UIB.SHEET-PROMPTS.OWNERSHIP

G01 retains the full declared source overview and overview measurements; details
distribute already-selected local measurement blocks by their source membership.
First matching detail sheet owns a block; uncovered blocks stay on the general
sheet. Other anchor/context objects may be repeated as explicitly bounded context.
Every required dimension ID has exactly one sheet owner. The overview covers all
selected source components; detail membership cannot silently remove controls.
Relationships/recommendations belong to G01 and do not reappear on every detail.
Unknowns and partial source scope are not upgraded by union coverage.

## UIB.SHEET-PROMPTS.AUTO-TAIL — USE-AUTO-DETAIL1

For one observed Document view with no explicit detail requests, omit the final
automatic chunk's separate D sheet only when it is nonempty and smaller than the
unchanged components_per_detail cap, with earlier full chunks retained, and every
member meets all conditions: known native DIV; no semantic role, label, geometry,
parent/children, declaration, extension or state/action record; exactly the three
requested public layout_bounds/hit_region/visible_region properties, all Unknown.
No member may participate in relation, mapping, focus or required dimension/anchor
context. Uncertain context conservatively retains the sheet. Do not generalize
this to zero assigned dimensions, controls, text/state, redacted/unsupported facts
or known geometry (including zero sizes). Unknown never means empty or invisible.

G01 retains every source member, caller context, exclusions, unknown caveats and
reference-visible content. Scene/dimensions remain exact; no filtering before
chunking, merging into a full detail, density change or rebalancing. Full source
union includes G01; detail-only union need not include this tail. Required dimension
IDs/owners and earlier details remain unchanged. Any explicit request preserves
the prior plan. Inputs with optional design_analysis requests retain the prior
plan, including unknown analysis results. Proposed/detail/compare/flow, multi-view
and no-split cases stay. This is the USE-AUTO-DETAIL2 protected-domain clarification
of the approved exception, not eligibility expansion or an analysis semantic change.
Sheets, manifest, brief and prompts derive consistently from the final plan;
there is no orphan prompt entry. At least one detail remains, so export0.4 stays.

## UIB.SHEET-PROMPTS.BOUNDS

All added text and manifest entries count toward the same aggregate output limit;
invalid assignments or overflow refuse before publication. No references are read,
no model is called, no new persistent directory is implied. Existing destination,
input, privacy, error/exit and independent status contracts remain unchanged.
Sheet count remains bounded by existing component/detail/input/output budgets.

## Acceptance

Actual CLI: independently usable G01 and D prompts, union source coverage, unique
dimension ownership, exact engine numbers/evidence, unchanged legacy inputs and
no-split versions, exact aggregate limits, refusal before destination creation.
AUTO-TAIL: Email CW27 loses only its automatic N024 sheet; all25 source members
remain on G01/in scene. Register D03 retains N024–026 and M0077/M0078. Cover explicit
unknown-only requests, meaningful zero-dimension fragments and protected context.
One fresh original-reference attempt per selected sheet; review actual returned
raster size, readability, every required label and complete selected scope. A plan
or several files is not evidence that separately rendered sheets are usable.
