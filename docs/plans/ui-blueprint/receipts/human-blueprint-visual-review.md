# HBP-V — independent visual review

Initial review, 2026-10-10. Reviewer: continuing HBP-V critic. Status: baseline
rejected for the new human-blueprint goal; no final candidate accepted.
Consumer: root's initial criteria and HBP-I's first real ImageGen iteration.
Only this receipt is writable; product/spec/source and every image are read-only.

## Basis and independence

Authority: [human-blueprints plan](../human-blueprints.md), explicit user Evolve
request recorded there, finite HBP-V packet. The user requests blue/white flat
outlines, useful dimensions/details, readable human labels and actual generated
images. Leonardo means care and craft, not a historical drawing style.

Traversal before inspecting evidence: `docs/specs/README.md` registry 35 →
`product/README.md` → EXPORT@1 → DRAWING-PACKAGE/STYLE/GEOMETRY/PROMPT-A/PROMPT-B/
REVIEW@1 and reference/DRAWING-EXAMPLE@1 → explicit MODEL/GEOMETRY/PROJECTIONS/
IDENTITY/BOUNDARIES/PRIVACY@1 closure; all selected CONTENT clauses read completely.
Global implementation, product-truth core/routing/evidence/delivery/coordination
and QA contracts read. Scope is visual review, not implementation authority.
CLI, analysis arithmetic, Rust, collectors, Native/Web acquisition and unrelated
product siblings are excluded; no implementation source or builder narrative read.
Selected evidence is four PNGs, then bounded raw-node and machine-package facts.
The new user authority supersedes old visible ID/decimal/inventory requirements;
HBP-I must register the corresponding presentation delta before implementation.
No conflict authorizes changing raw values, source identity, privacy or scope.

## Direct image observations

Inputs, all retained unchanged:

- `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-e04-real-tbaWNv/director-reference.png` — 412 × 404.
- `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-e04-real-tbaWNv/settings-reference-complete.png` — 844 × 1236.
- `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-e05-1jq7290v/director-generated.png` — 1672 × 941.
- `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-e05-1jq7290v/settings-generated.png` — 1672 × 941.

Viewed through `view_image(detail=original)` before narratives. Dimensions verified
read-only with `sips`. This establishes original-resolution/tool-inline inspection;
no controlled 1280-pixel delivery viewport or physical print legibility is claimed.
`after-title-spacing.png` was never opened. No images/crops were created or deleted.

**Director:** the source has a label/info control, closed-value trigger, open popup,
one “Directors” heading, one `spiel` input, and three suggestions with counts.
The generated image adds a second “Directors” row inside a control-shaped outline
between the true heading and input. This is a source-fidelity error, not taste.
All three suggestion labels/counts remain recognizable, which should be preserved.
The popup should remain visibly wider than its trigger; the generated near-equal
widths weaken that relationship. Schematic status permits approximate raster scale,
not invented components. The bottom width line does reach the popup edges: no
unsupported anchor-defect claim is made for it.

Director also uses shaded blue panels and a pale filled active row, giving a
recoloured UI impression. Its dense grid, ID leaders, two large inventory tables
and elaborate footer compete with the actual form. Seven/eight-digit decimal labels
recur around controls and in tables. Tiny footer provenance cannot serve ordinary
client reading even though large control labels are legible. No enlarged detail
answers a question that is hard to see on the overview.

**Settings:** the seven preference rows, content section, two off-position toggles,
helper text and Close control are recognizable and ordered like the source.
However, the central illustration is an unmistakable dark screenshot-like UI panel,
with grey control fills, shading and coloured background variation. This directly
fails the user's drawing grammar. Two outer dimensions are large; useful internal
relationships are absent. Large side columns repeat scope, notes, legend and raw
observation metadata while the long helper text is squeezed inside a small form.
Empty lower-left space does not compensate for the crowded right metadata column.
The result documents a screenshot surrounded by paperwork, rather than explaining
its construction. Both generated images need a new composition, not more callouts.

Raw-node cross-check: Director has one heading node and one INPUT with value `spiel`;
its trigger width is 179.703125 CSS px, popup width 194.2578125. Settings has seven
SELECT nodes, two BUTTON nodes with checked=false, and Close. The outer form is
421.640625 × 617.7421875 CSS px. PNG source pixels are not CSS dimensions.

## Proposed composition, within the user envelope

Make the complete outline overview the dominant region, with a modest title and
one compact provenance/units/status footer. Place the few outer dimensions outside
its perimeter. Reserve a clearly separated adjacent area for useful enlarged
regions; link each by a named locator on the overview. Keep the blue ground
continuous through controls and detail views. Distinguish active/off states through
white outlines, a source-faithful knob position and a concise label where needed.

Director needs its intact overview and one result-row detail: row bounds, text/count
alignment and a measured gap or inset only when anchors are supplied. Do not turn
text bounds into visible input borders. Settings needs the intact seven-row form,
content section and footer, with a typical select-row detail and a content-toggle
detail if required for helper-text legibility. Repetition can share dimension notes;
it cannot remove controls. Detail captions should say what the reader learns, e.g.
“Preference row” or “Content toggle”, rather than demand a machine-ID lookup.
These are editorial recommendations, not new unmeasured geometry or product intent.

## Finite acceptance criteria for actual candidates

| ID | Required observation / failure condition |
| --- | --- |
| V1 Drawing grammar | Solid blue/cyan ground throughout; white flat contours, text and dimension graphics. No screenshot-like/dark/grey/purple panels, gradient fills, shadows, glass, perspective, decorative grid or invented ornament. Antialiasing is not a third design colour. |
| V2 Source fidelity | Director contains exactly one popup heading, one `spiel` input and the three correct suggestions/counts; Settings retains all seven preference rows/current labels, content section with both off toggles, helper text and Close. Preserve order, nesting, obvious relative geometry and state; do not add outlined controls for text nodes. |
| V3 Hierarchy | At whole-sheet view the reader finds title → complete overview → selected details → compact notes. Inventory tables, machine IDs, timestamps and unknown-property lists are absent. Overview and details dominate; metadata never competes with them. |
| V4 Readability | At the candidate's declared delivery size, read every meaningful control label, displayed value, dimension and required note without zoom. Verify the actual raster, not a higher-resolution promise. If helper text/details fail, enlarge/recompose or use another planned sheet; do not silently drop them. No text/line collisions or clipped labels. |
| V5 Useful geometry | Overview communicates outer extent and major relationships; at least a useful repeated-control detail makes internal structure clearer. Every printed measurement has two unambiguous, correct endpoints; leaders name the intended region, do not cross labels and do not resemble action flow. No detached dimension or unrelated detail crop. |
| V6 Honest annotation | Human-readable rounded labels and concise CSS-px/rounding note; no long decimal walls, fabricated radius/padding/tolerance or 1:1 raster promise. Source/validation/approval and schematic status remain truthful in compact prose. Every visible number is checked against the supplied source/approved presentation mapping. |

All six are acceptance conditions, not a weighted beauty score. Native resolution
and declared client viewing size are separate checks. No arbitrary font-size,
exact two-detail quota, CAD-scale accuracy or ISO compliance is imposed. Once a
criterion passes, revisit it only if a later candidate changes relevant content.

## Primary-source research and concrete use

- [NN/G, Visual Hierarchy in UX](https://www.nngroup.com/articles/visual-hierarchy-ux-definition/), Kelley Gordon, 2021-01-17, read 2026-10-10: hierarchy uses scale, contrast and grouping; space separates groups and headings stay close to their content. Application here: make the overview dominant, use a small type hierarchy and whitespace to separate details, remove competitive metadata enclosures. This is visual communication guidance, not a mandatory UI pixel scale.
- [Onshape, Detail View](https://cad.onshape.com/help/Content/Drawing/detail_view.htm), official help, read 2026-10-10: a detail enlarges a selected region of an existing view, with a source label/profile or connection. Application here: a detail must answer a small-geometry/legibility question and identify its exact parent region; a decorative duplicate is insufficient.
- [Onshape, Drawing Dimensions](https://cad.onshape.com/help/Content/Drawing/drawing_dimensions.htm), official help, read 2026-10-10: dimensions associate with selected geometric points/entities, text can be placed outside extension lines, and display precision is explicit. Application here: judge endpoint association separately from attractive text placement; round display labels without altering underlying facts. Onshape's manufacturing units/tolerances do not become UI requirements.

Research informs these recommendations; the user's colour/style and source-truth
constraints provide authority. No closed ISO text was read or compliance claimed.

## Initial disposition

Both old images fail V1/V3/V5/V6 under the new request; Director also fails V2.
Settings' recognizable source composition is a useful starting point, not approval.
V4 requires an explicit final viewing target and fresh actual-image review. No new
candidate, compiler/privacy compatibility result or final acceptance is claimed.
Documentation-only verification: local plan link resolves and `git diff --check`
passes for this receipt. HBP-V remains available for subsequent candidates in this
same critic role; no extra agents, chats, goals or visible apps were used.
