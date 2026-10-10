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

## R1 independent review — 2026-10-10

Candidate pin: `43d62d46602929b485b3e136c19e69f5cb8a9af6`.
Re-established route: registry 36 → EXPORT@2 and DRAWING-PACKAGE/STYLE/GEOMETRY/
PROMPT-A/PROMPT-B/REVIEW@2 → their new explicit DRAWING@1.2 HBP-HUMAN-001 section;
unchanged @1 source/model/privacy dependencies reused from the complete initial read.
The updated presentation rule has explicit precedence over historical template text.

Actual images: `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-hbp-ebwz2bfy/director-r1.png`
and sibling `settings-r1.png`; both 1536 × 1024, declared delivery review at 100%.
Inspected using original-resolution image view. Initial observations were sent to
root before reading either literal `*-r1-package/prompt.txt`; no implementation
receipt or author verdict read. E04 source images/raw nodes remain the comparator.

**Verdict: R1 is not accepted.** The new composition fixes the large prior failures:
no dark screenshot panels, no inventory/ID tables, no long decimals, no duplicated
Director heading-as-control. Main forms and enlarged details are clearly separated;
control/value/help labels are readable at this native size. Remaining defects are
specific source geometry/anchor failures, not a request for more visual decoration.

| Criterion | Director R1 | Settings R1 |
| --- | --- | --- |
| V1 | Pass for blue/white flat drawing grammar; no intentional gradient/panel treatment. | Near-pass, but filled white switch knobs violate the explicitly outlined-knob rule; use blue-interior white-outline knobs, retaining the left/off position. |
| V2 | Fail: input and result rows no longer share left/right edges or equal width; popup and trigger no longer share their source left edge. Counts, text, one heading/input and three results are preserved. | Pass for visible composition/content/state: seven correct preference rows, content section, both off toggles, all helper text and Close remain. |
| V3 | Pass: overview/details/notes hierarchy, ample space and no machine-data competition. | Pass: same; no new layout overhaul required. |
| V4 | Pass at declared native 1536 × 1024: labels, dimensions and footer are readable; main problem is their meaning, not type size. | Pass at declared native size, including small helper text/footer. This is not a claim for reduced-size embeds/printing. |
| V5 | Fail: several dimensions float without their named edge pair; Detail A spends space repeating already legible trigger geometry while carrying detached gap marks. | Fail: overall height ends at the wrong boundary; top inset and detail spacing marks are not associated with their named edges. |
| V6 | Prepared rounded numbers/units/status are legible and no unsupported new values were found; cannot fully pass while displayed anchor meaning is wrong. | Same: correct numerical tokens do not make the depicted measurements correct. |

Concrete blocking corrections:

1. **Settings overall height:** the line labelled `≈618 (form height)` ends at
   the top of the Close/footer region. It must span the complete form from top
   boundary to bottom boundary; the existing ≈57 bottom-region dimension is a
   subordinate span, not an extra height outside ≈618. Source FORM height is
   617.7421875 CSS px and includes the footer. Keep the number; fix its endpoints.
2. **Settings ≈43 and ≈8:** ≈43 means FORM top → content DIV top, but its short
   line sits beside the header/first row without those endpoints. Detail A's
   ≈8 above/below the select has no identifiable row bounds; the lower mark is
   a one-ended arrow. Either explicitly identify the actual two source boundaries
   or omit that weaker candidate and show the supplied ≈16 clear gap between two
   adjacent select controls. Preserve the useful ≈198 × ≈33 control-size detail.
3. **Director ≈14/≈4/≈5/≈8:** the ≈14 decoration above the title is not the
   supplied wrapper-top → trigger-top inset. The ≈4 mark floats above the info
   icon without extending to label-right and icon-left. The ≈5 mark does not
   trace info-bottom → trigger-top. Detail B's left ≈8 is a detached arrow; its
   right ≈8 has no depicted popup edge. Prefer fewer useful anchored dimensions:
   trigger-to-popup ≈6, input-to-list ≈6, row gap ≈3, plus popup-to-input ≈8
   where the popup boundary is actually visible. Omit unnecessary wrapper measures.
4. **Director alignment:** source trigger/popup share x=24.59375; input and all
   three result rows share x=32.6171875 and width=178.2109375. Re-establish those
   alignments in overview and enlargement. This is a categorical relationship,
   not a demand for CAD-accurate raster scale. The current enlargement repeats
   the incorrect wider/left-shifted result row, so both views must be corrected.
5. **Settings knobs:** replace the solid white circles with outlined circles;
   do not change their position or add a checked state.

Do not repeat the overall composition experiment: retain the accepted hierarchy,
legibility, source labels and compact footer. Complete the useful geometry with a
small selected set of correct dimensions; multiplying labels is not the remedy.
Captions that uniquely name the actual repeated source control are sufficient;
this review does not newly require decorative leader connections for every detail.

Focused research revisit: [Onshape Drawing Dimensions](https://cad.onshape.com/help/Content/Drawing/drawing_dimensions.htm)
explicitly describes association with geometric entities and identifies a detached
association as a dangling dimension. That directly supports treating R1's floating
arrows as communication failures. Our remedy applies this association principle;
no Onshape colour convention, manufacturing unit or ISO requirement is imported.

A read-only PNG sample check found only small ground-colour variation (a few RGB
levels) in clear regions. This is not escalated into a separate gradient blocker:
there is no intentional tonal UI treatment in R1. The outline-knob failure is
plainly visible. Sampling used Python standard-library PNG decoding after the
optional Pillow import proved unavailable; no image was transformed or created.
Documentation verification: new prose/links reviewed and `git diff --check` passed.
No code, source data, prompt, frozen PNG or author receipt was modified.

## R2 independent review — 2026-10-10

Pin `c274c92321771ed0732b955eeefa8cf504491523`; same retained E04 sources.
Images: `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-hbp-ebwz2bfy/director-r2.png`
and sibling `settings-r2.png`, both 1536 × 1024, reviewed at declared native size.
Read the changed registry/export provenance corrections and DRAWING@1.2 R2 anchor
clarification; remaining R1 closure/criteria are unchanged. Actual images preceded
literal `director-r2-final-package/prompt.txt` and `settings-r2-final-package/prompt.txt`.
Initial observations sent before author reconciliation; implementation receipt not
needed/read. This is the same independent HBP-V role, not a new reviewer.

**Verdict: R2 not accepted.** Both pass V1/V3/V4: flat blue/white drawing, outlined
off knobs, clear overview/detail hierarchy and readable native-size labels. Settings
passes visible-content V2, with all controls/help text retained. Director fails V2:
the source's entered `spiel` became `Director name` in both overview and enlargement.
The first result is emphasized, but the mandated “Keyboard-active; not applied”
note is absent. “Any director” remains unchanged, which should be preserved.

Director's overview now restores trigger/popup left alignment and input/result
alignment; the meaningful gap/inset detail is substantially clearer. HBP-V does
not introduce a new pixel-perfect ratio gate for the small residual differences
in schematic enlargement. Director V5's earlier detached wrapper/icon measures
are removed and the main measurement relationships are now understandable.
Settings V5/V6 still fail on dimension association, despite correct numeric tokens.
Both retain truthful draft/unverified/schematic footer language and rounded labels;
Settings should retain the supplied ≈ prefixes consistently in its details.

Shortest corrective set for R3:

1. **Director:** show `spiel` as the actual input value in both views, distinguishing
   it from the accessible name `Director name`. Restore the explicit keyboard-active/
   not-applied note on the first result. This is existing state fidelity, not new
   input-value collection/export authority. Literal prompt currently lists the
   accessible name as generic input text while its state annotation supplies `spiel`;
   remove that ambiguity in the bounded presentation instructions.
2. **Settings preference detail:** connect ≈16 from first select BOTTOM to second
   select TOP; the current arrow starts at the row divider below the first select.
   Put the named label-layout RIGHT boundary at the start of ≈12; the current named
   dotted line runs through the label while ≈12 starts from a different unlabelled
   line. Connect each ≈22 outer inset to its actual form-side boundary and named
   label-left/select-right edge, not an arbitrary detail-frame segment.
3. **Settings Close:** remove the overview's 15-px arrow ABOVE Close. The required
   ≈15 values mean Close-right → footer-right and Close-bottom → footer-bottom;
   they do not mean footer-top → Close-top (source ≈11). In the enlargement, bring
   the right-inset extension to the actual Close right edge and identify the footer
   right/bottom boundaries. A dashed detail enclosure alone is not an identified
   source footer edge. Keep the now-correct whole-form ≈618 span including footer.

Keep the accepted composition, readable helper text, outlined switches and useful
content-switch dimensions. No extra decoration or mandatory new detail is requested.
The existing official Onshape association evidence already resolves these same
anchor errors; no new research question or speculative standard was introduced.
Independent content-critic findings were not substituted for this visual assessment.
`git diff --check` passed; only this receipt changed, all frozen inputs remain intact.
