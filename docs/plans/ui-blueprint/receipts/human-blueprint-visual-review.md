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

## R3 independent review — 2026-10-10

Source `bdd73d40d90d26086f2aa6b279ac40b52413b1c9`; images in the same retained
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-hbp-ebwz2bfy/` root:
`director-r3.png`, `settings-r3.png`, both 1536 × 1024 at declared 100% native size.
Read the sole applicable contract delta, DRAWING@1.2 HBP-HUMAN-001 R3 text-channel/
anchor clarification, before images. Reused unchanged selected closure and V1–V6.
Images preceded bounded literal `director-r3-package/prompt.txt` and
`settings-r3-package/prompt.txt` inspection; author narrative was not needed/read.
Initial observations were sent independently before root allowed reconciliation.

**Verdict: R3 not yet accepted; retain the successful areas.** Director now correctly
shows `spiel` in both views and explicitly identifies “Keyboard-active; not applied”.
Settings now correctly connects ≈16 between adjacent selects, ≈12 from the named
label-layout right edge, and both ≈15 insets to named footer right/bottom edges.
The whole-form height includes Close/footer. These R2 defects are resolved.

| Criterion | Director R3 | Settings R3 |
| --- | --- | --- |
| V1 | Pass: same blue/white outline grammar. | Pass, including blue-interior off knobs. |
| V2 | Source text/state/counts pass; enlarged input/result alignment remains visibly inconsistent. | Pass: meaningful source controls, values and help text remain complete. |
| V3 | Main hierarchy still passes, but “record 4” and “record 5” construction IDs have leaked into visible detail captions. | Pass. |
| V4 | Pass at native delivery size. | Pass at native delivery size. |
| V5 | Fail on enlarged width/inset anchor associations, detailed below. | Fail only on missing required Close height ≈30; repaired gaps/footer associations are useful and clear. |
| V6 | Rounded labels/status remain honest, but wrong edge associations prevent full pass. | Visible labels/status pass; required-dimension omission remains V5. |

Finite repair list:

1. Remove `(record 4)` / `(record 5)` from Director's visible captions. These are
   construction identifiers, expressly marked non-printing in the literal prompt.
   Keep readable human captions; no new legend, heading system or detail is needed.
2. Fix Director's enlarged popup dimensions against its actual drawn boundaries.
   Its ≈178 width starts inside the search control rather than at its left edge;
   ≈194 similarly uses an inset span inside the popup. Both ≈8 marks span short
   free-standing guides instead of popup border → input edge. Place those extension
   lines on the real popup/input boundaries and align input/results on common
   left/right edges. The input is visibly shifted right relative to the result
   column, so this is the existing relationship requirement, not a new raster
   tolerance. Preserve the repaired text/state and the already readable ≈6/≈3 gaps.
3. Restore the mandatory `≈30 px` Close height in the Settings footer detail,
   connected to Close top/bottom. Keep ≈52 width and the now-correct named ≈15
   right/bottom insets. The literal prompt explicitly requires height; it is absent
   from both overview and detail, so the gap is not a new reviewer preference.

No new palette, composition, typography, ornament, scale or research requirement.
Existing primary-source dimension-association guidance covers the remaining defects.
Do not redo accepted content, settings gap semantics or footer geometry to obtain
these local repairs. `git diff --check` passed; only this receipt changed.

## R4 independent review — 2026-10-10

Pin `cd099a19b73760f3a2144439b10e3a04b9daf79c`; no selected contract changes from R3.
Images in retained `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-hbp-ebwz2bfy/`:
`director-r4.png` and `settings-r4.png`, 1536 × 1024, 100% native inspection.
Actual images preceded bounded pending-dimension lines from each
`*-r4-final-package/prompt.txt`; no author narrative read or needed. Same E04 source
and previously established source measurements; no new acceptance criteria.

**Settings R4: HBP-V accepts V1–V6 for this actual image at native delivery size.**
The missing Close height ≈30 is now present with top/bottom association. Accepted
complete content, outlined off switches, adjacent-select gap, named label edge,
named footer right/bottom insets, whole-form height, readable labels and honest
footer remain intact. This is visual/source-presentation acceptance, not compiler,
privacy, whole-product, reduced-size/print or human approval. Preserve this image;
no further Settings aesthetic iteration is requested.

**Director R4: not accepted; V1/V3/V4 pass, V2/V5 remain partial/fail, V6 cannot
fully pass the incorrect measurement meaning.** Record 4/5 text is removed and
correct `spiel`/keyboard-active state is preserved. The single remaining repair
is the enlarged popup geometry already identified in R3. The ≈194 span still
starts/ends inside the popup rather than at its border; ≈178 starts inside the
input; ≈8 spans short isolated guides rather than border-to-input edges. Input
and result rows still use different left/right alignments in the enlargement.
The literal R4 prompt continues to require the correct source boundaries and
shared alignment, so this is an unchanged output defect, not a contract ambiguity.

Repair only that detail: establish common input/result left/right edges, extend
≈194 to both popup borders, extend ≈178 to the input sides, and terminate each
≈8 between the corresponding popup and input side. Keep the correct overview,
text/state, ≈6/≈3 vertical gaps, source labels and accepted composition. No new
pixel tolerance or change to machine numbers is requested. R4 has closed two of
R3's three corrections; it has not yet implemented the third.

Only the receipt changed; frozen PNGs/source/packages untouched. Documentation
`git diff --check` passed. Existing primary association guidance remains sufficient;
no fresh research or design expansion was needed for the same unresolved defect.

Root reconciliation: subsequent work may freeze Settings R4 and repair Director
only. The duplicate ≈194 in the enlarged detail may instead be removed, because
the complete overview already supplies that outer dimension. Retain the useful
≈178 input width and both ≈8 insets with their correct boundary pairs; removing
all of those relationships would not resolve the pending usefulness/anchor defect.

## R5 Director-only review — 2026-10-10

Image `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-hbp-ebwz2bfy/director-r5.png`,
1536 × 1024, inspected at native size before narrative. It edits Director R4;
source `cd099a19b73760f3a2144439b10e3a04b9daf79c` and literal R4 final prompt remain
unchanged. No new contract/data/code checks are needed. Settings R4 stays frozen
and accepted by HBP-V; it was not reopened.

**R5 rejected.** V1/V3/V4 retain the established style/hierarchy/readability pass;
V2/V5 fail and V6 remains incomplete because the displayed measurement meaning is
wrong. The field still has different left/right edges from the result rows. The
≈8 guides remain detached from the actual popup border, and ≈194 still spans an
interior line rather than both outer borders. This is the same pending R4 defect.

The edit additionally changed the previously accepted overview: it inserted the
same faulty width/inset chains and made the input visibly narrower than its result
rows there too. The source heading “Directors” also disappeared from the enlarged
popup, leaving only the drawing annotation “Popup container”. Correct `spiel`,
counts, three results and “Keyboard-active; not applied” remain; do not disturb them.

Required repair remains local and finite: restore the earlier overview, retain
“Directors” in the enlarged source view, align input/results and correctly attach
≈178 and both ≈8 to the actual input/popup boundaries. A redundant enlarged ≈194
may be removed while its correct overview counterpart remains. Merely moving
unattached short guides closer together does not establish endpoint association.
No new visual standard, numeric tolerance or optional design request is introduced.
Only this receipt changes; image/source/package remain untouched. `git diff --check`
passed before saving the checkpoint.

## R6 Director-only review — 2026-10-10

Source `5139f8eb0ce4e72ecc1c0e62f0f3e074490d1960`; no selected contract delta.
Image `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-hbp-ebwz2bfy/director-r6.png`,
1536 × 1024, inspected at native size before author narrative. Then read affected
presentation/required-dimension lines in `director-r6-package/prompt.txt`. Settings
R4 remains accepted at its own `cd099a1`/R4 package, not attributed to R6 generation.

**R6 rejected on two bounded defects.** V2/V3/V4 pass: source labels, `spiel`, three
counts/results, “Directors”, explicit keyboard-active/not-applied state, hierarchy
and native-size legibility are retained. The overview is again useful; ≈178 width
and common control-column relationships are substantially clearer. No CAD precision
claim or new ratio threshold is required. Remaining V1/V5/V6 issues:

- V1 regression: both keyboard-active rows have a visibly lighter blue fill. Restore
  continuous ground with only an extra white outline and the existing state note.
- V5/V6: the named “popup left edge” is a separate line outside the actual popup;
  the named “input left edge” is another line to the right of the true input-left.
  Naming these lines does not make them source boundaries. The 8-px graphic still
  does not measure popup-border → input-border. The literal prompt requests one
  direct left inset at input mid-height and no detached guides; only this useful
  inset must be fixed, not both former mirrored inset marks. Use prepared ≈8 label.

Root's bounded remedy proposal is suitable: an isolated enlarged fragment titled
“Popup-to-search inset” can show the two actual nested left borders, each with a
short connected top segment to identify the nesting, and one ≈8 double arrow
between them. It remains a same-source detail, not new geometry. Remove the false
named guide lines from the larger view, preserve the complete overview and useful
≈178/≈26/≈24/≈6/≈3 dimensions, and avoid an extra enclosing detail frame that could
be mistaken for a source boundary. This resolves the existing association problem;
it does not establish another style criterion or require another research wave.

No unrelated check or Settings rerun. Only this receipt changed; frozen assets,
source and literal packages remain untouched. `git diff --check` passed.

## R7 Director-only review — 2026-10-10

Source `9f5b9a867bcc80c9d019b14166e24b9f07fe95a8`; selected contracts unchanged.
Viewed `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-hbp-ebwz2bfy/director-r7.png`
at 1536 × 1024 native, then checked affected literal `director-r7-package/prompt.txt`
lines. Initial observations preceded reconciliation; no author narrative used.
Settings R4 remains accepted at its own R4 source/package and was not inspected again.

**R7 not yet accepted.** The isolated “Popup-to-search inset” finally connects its
8-px dimension to two real nested corner borders; active rows now use white double
outlines with blue interiors. These R6 fixes pass and should be preserved. V3/V4
hierarchy/readability and meaningful source text/state/counts remain adequate.

The larger popup detail nevertheless repeats an unwanted 8-px graphic with false
guides and a stray dark marker. Its ≈178 line still starts inside the input instead
of at its left side, and the input remains narrower/right-shifted relative to the
result column. Thus V2 alignment and V5/V6 measurement association remain unaccepted;
V1's row-fill issue is resolved, while removing the redundant graphic also removes
its dark artifact. The literal prompt expressly prohibits that duplicate inset.

Root's proposed simplification is within the existing scope: make the large detail
a clearly named partial “Search and result spacing” view containing the input and
first two result rows on one shared left/right column, with ≈178/≈26/≈24/≈6/≈3.
Omit the enclosing popup border/heading and all inset guides from that partial view.
The complete overview still preserves every source control/heading/result, including
the third result; the accepted separate corner detail alone owns ≈8. Keep `spiel`,
the first two result names/counts and the keyboard-active/not-applied annotation.
Attach ≈178 to the actual common control-left/control-right boundaries. This removes
competing reference frames; it neither relaxes source fidelity nor adds a criterion.

Only these existing errors remain; no new style, research, print-size or CAD gate.
Frozen images, source and packages remain untouched; `git diff --check` passed.

## R8 final Director visual acceptance — 2026-10-10

Source `b008a4408d3e5e5b73e9cd36da0932c217411d25`; selected contracts unchanged.
Actual image `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-hbp-ebwz2bfy/director-r8.png`,
1536 × 1024, inspected at declared native size before author narrative. Then checked
all required dimension lines, state and rounding footer in the literal
`director-r8-final-package/prompt.txt`; no author verdict used as visual evidence.

**HBP-V accepts Director R8 against V1–V6 at this delivery size.**

| Criterion | Final direct evidence |
| --- | --- |
| V1 | Blue ground continues through controls; white contours/type/dimensions; active row uses a double white outline, without the R6 lighter fill. No screenshot panel, grid, record-ID artefact or decorative dark marker. |
| V2 | Overview preserves Director/info, Any director, one Directors heading, `spiel`, all three correct result names/counts and order. Partial detail preserves the first two results and explicit keyboard-active/not-applied state. Main input/results share the intended column; no CAD-precision claim. |
| V3 | Complete overview, clearly named partial spacing detail, separate purposeful inset fragment and compact provenance footer. No construction-ID/inventory noise or competing popup frame in the spacing view. |
| V4 | All source labels, counts, dimensions, state annotation and footer are readable at native 1536 × 1024. No material collision/clipping in the final corrected areas. |
| V5 | ≈178 now spans actual input left/right edges; ≈26/≈24 heights and 6/3 gaps relate to the depicted controls. The sole inset joins the two actual nested corner borders. Overview retains trigger/popup sizes and attachment gap; obsolete duplicate inset guides are gone. |
| V6 | Visible 180/29/194/143/178/26/24/6/8/3 values match the prepared rounded source-derived labels. CSS px and the all-label rounding/schematic note remain clear, with no tolerance or CAD-scale claim. Draft, partial coverage and image-unverified generation status are retained, not silently promoted to human approval. |

The integer gap/inset labels rely on the explicit whole-sheet rounded-label note;
this does not assert exact values or alter their source numbers. The accepted image
is the actual R8 edit produced with its literal final CLI package, not evidence that
an arbitrary rerun will reproduce the same raster. No new generation is requested.

**Accepted visual pair:** Director R8 at `b008a44`/R8-final package plus unchanged
Settings R4 at `cd099a1`/R4-final package. Settings is not attributed to the newer
Director prompt revision. Both visual/source-presentation scopes pass HBP-V; this
receipt does not substitute for HBP-C, installed/compiler/privacy checks, human
approval, live collection or reduced-size/print validation. Existing E04 source
coverage remains partial/imported. No residual visual blocker remains in this scope.

Only this receipt changed; frozen images/packages/source remain intact. Documentation
`git diff --check` passed. No unrelated check, extra agent or further image was used.

## FRESH-01 — generic prompt and bounded fresh-generation assessment

New explicit user objective received 2026-10-10: the real CLI should prepare useful
blueprints from scratch, rather than perfect selected images through editing. Same
continuing HBP-V, V1–V6, receipt-only ownership and registry36/EXPORT@2/DRAWING@1.2
closure. R1–R8 are prior observed evidence, not fresh-generation acceptance. No
unchanged code re-audit, new agent, new standard or speculative research is needed.

Generic recommendations from the repeated observed failures:

- Resolve the displayed text channel explicitly: visible label, accessibility name
  and permitted caller draft have different meanings. The finished drawing must
  receive one unambiguous field-content instruction. A parent aggregate must not
  become a duplicate control/header; editing instructions must not enter the normal
  fresh-document prompt.
- Use a complete overview for source coverage and named partial details for specific
  relationships. A spacing detail needs only input/result rectangles in one common
  column. Omit competing container frames when they do not provide a measured edge.
  A small same-source corner fragment can own an inset without duplicating it in
  the full detail. This is a general composition rule, not a hardcoded Director skin.
- Supply a small deterministic set of useful measurements with explicit semantic
  endpoint pairs and display labels. Separate drawing annotations from source text.
  Put a dimension only in the view containing both its actual boundaries; naming
  a disconnected guide does not establish association. Do not ask for optional
  candidate tables, duplicate measurements or dimensions on every text node.
- Preserve categorical geometry (nesting, order, common edges, equal-width control
  columns) while identifying raster scale as schematic. Do not promise CAD accuracy.
- State the graphic rule once clearly: continuous solid blue ground, white outlines/
  type/dimensions, no source screenshot fills. Known keyboard-active state uses an
  extra outline plus its explicit non-applied meaning; known off knobs stay outlined.
  Keep all safe meaningful text, units/rounding and compact source/status notes.

Proposed fixed assessment, for root to pin before candidate generation:

1. Select Director, Settings and one additional authorized real form with original
   UI image and measured source facts. The third form should introduce a different
   useful relationship rather than being chosen after seeing successful output.
2. Freeze one CLI revision and each literal prompt/source package. Run exactly two
   independent fresh calls per form, original UI references only. Record every
   resulting image's actual native size. No previous blueprint, image edit, manual
   prompt wrapper, best-of selection or replacement of an unattractive result.
3. Review all six actual images independently before author explanation. Check
   V1–V6 against their own source/required labels at the declared delivery size.
   Record each cell's pass/fail and exact blocking region; report source/prompt
   omissions separately from generation deviations. The two current baseline
   images are a separate baseline pair, not substitutes for favourable cohort cells.
4. Report the full matrix and full-pass count out of six. Six successes support
   only this bounded cohort, not universal reliability or statistical assurance.
   Any failing cell remains visible and prevents an all-cohort success claim.
   A later generic compiler revision may receive a newly declared fresh cohort;
   it does not replace failed cells or trigger individual-image polishing.

Fairness boundary: missing/changed source labels or state, invented controls,
wrong or ambiguous endpoint association, missing required useful dimensions,
screenshot-like fills and unreadable mandatory text are material failures.
Schematic proportionality, small raster alignment variation, antialiasing and taste
are not new gates. An obviously interior start of a width dimension is an anchor
failure, not a minor proportionality issue. A valid partial detail does not omit
scope when the full overview preserves it. Repeated known-source sizes may share
one clear dimension rather than creating a new exhaustive-label requirement.

This proposal evaluates the generic pipeline through a fixed collection of honest
results. It does not pre-approve a future prompt or image. Await neutral baseline
paths, then the root-pinned cohort; no fresh image has been reviewed for FRESH-01 yet.
Only this receipt changed; `git diff --check` passed before its checkpoint.

## FRESH-01 baseline pair and third-source scope — 2026-10-10

Separate baseline, not the final 3 × 2 cohort pinned by root at `1bf96d7`.
Source `b008a44`; actual fresh images in
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-fresh01-baseline-hhcyuw74/`:
`director-baseline.png`, `settings-baseline.png`, each 1536 × 1024, reviewed native.
Per neutral packet: exactly one fresh call each, original E04 UI references only,
no blueprint edit/history/wrapper/reroll. Images preceded literal baseline
`director-package/prompt.txt` and `settings-package/prompt.txt`; no author plan used.

**Baseline: 0/2 full visual passes.** Both remain blue/white, scannable and readable;
these are material content/association failures, not a rejection of their style.

| Case | Material observation | Prompt reconciliation |
| --- | --- | --- |
| Director | Source text/counts/complete overview are recognizable, but the first result loses its keyboard-active outline in the repeated partial view while retaining that state in the overview. Overview input and results also use different left edges. V2 therefore fails; principal size/gap graphics remain substantially useful. | The literal prompt explicitly requires the keyboard-active second outline and common source-equal edges. These are generation deviations. Duplicate overview inset and its awkward vertical text are secondary instruction deviations, not an added aesthetic gate. |
| Settings | Invented search field, “Result 1”/“Result 2” and “Popup-to-search inset” have no source counterpart. The 16 gap measures row-container frames instead of adjacent select edges; Close insets have no actual footer boundaries and an incorrect top-side 15 appears. Required switch ≈39 × ≈24 is missing. V2/V5/V6 fail. | The generic literal prompt unconditionally requests a search/results partial view and popup corner fragment even for Settings. This is a demonstrated template mismatch. The generated 422-wide search and 49-high results misuse source numbers on invented objects. Required switch dimensions exist in the prompt but the image omits them: an additional generation deviation. |

Both literal prompts also contain the old “If a blueprint reference is supplied,
edit it” preamble. No blueprint was supplied here, so the output is still a fresh
baseline, but that preamble does not belong in the generic fresh-document request.
The Settings mismatch cannot be repaired by merely strengthening “do not invent”
while simultaneously requiring nonexistent source regions.

Bounded generic remedy: compile only detail relationships supported by this input's
actual controls/anchors; omit recipes whose source roles/relations are absent. Map
each chosen detail's source elements, state and dimensions together, rather than
combining every historical repair recipe. Repeated source state must carry into
each displayed instance. Preserve the useful simple partial/isolated-corner patterns
where applicable, and remove image-repair language from ordinary document prompts.
No baseline-image retouch or success reroll is requested.

Third-source visual scope was pinned BEFORE its candidate generation from
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-mobile-filters-OBW93z/handoff.md`,
original `mobile-filters.jpg`, capture metadata and bounded DOM/identity/AX facts.
This is a responsive browser reference, not native iOS/mobile acceptance; 390 × 844
CSS px, DPR 1, one viewport, non-atomic capture with reported stable frame/rect checks.
Visible composition: Back/Filters header; Source with All active, Movies, Episodes;
Years From/To empty; 14 genre buttons in two columns/seven rows; Director placeholder
“Type a director” and “Type at least 2 characters”; Cast heading and clipped input.

Cast INPUT bounds are y=817..861, but the recorded visible intersection ends at844.
Preserve that partial coverage/clip boundary rather than showing a fully observed
input or inventing lower Actor Voice, Apply/Clear, results or footer controls.
Placeholder text is not an entered value. Full input bounds and visible intersection
are different facts. Existing DOM grouping/dividers must not become invented buttons.
Years and genre pairs provide legitimate measured size/gap relationships (179-wide
controls, 44-high year fields, 40-high genre buttons, 8-pixel column gaps); these are
examples for source-driven selection, not newly mandated extra views or constants.
Lower content is omitted, not absent; no whole-app/source-bundle or canonical import
claim follows from this reference. Conversion limitations remain the builder's duty.

All baseline results remain reported, separate from future cohort cells. Only this
receipt changed; all images/data remain retained. `git diff --check` passed.

## FRESH01-C1 — all six frozen fresh outputs, 2026-10-10

Source `76f14ac5a2ce6cb45a54e9c16b64342492d361fb`; selected contract closure unchanged.
Cohort root:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-fresh01-baseline-hhcyuw74/`.
Read ALL six actual images before any literal prompt or author context, then read
`cohort-results.json`, `frozen-cohort-manifest.json` and affected per-view/state/
required-dimension text in the three `*-frozen-package/prompt.txt` files. All three
literal prompt SHA-256 values match the frozen manifest. The neutral record reports
six fresh calls, six returned images, zero edits/rerolls/wrappers/refusals; the two
baseline outputs remain separate. No prior accepted edited image was substituted.

**Final HBP-V cohort result: 0/6 complete passes.** Every cell is retained and reported.
No new font threshold, CAD ratio, pixel tolerance or aesthetic preference was used.
All six have readable type and a scannable overview/detail hierarchy (V3/V4 pass).
The former Settings search/results hallucination is absent: the source-driven view
plan fixed that baseline problem. The remaining findings concern actual edge meaning,
required dimensions and the existing outline-only rule.

| Cell / actual native review size | Verdict and precise blocking region |
| --- | --- |
| `director-cohort-1.png`, 1536 × 1024 | Fail V5/V6. Overview trigger-to-popup 6 and detail 6/3 gap arrows terminate on small detached ticks inside the gaps, not the named control borders. In the lower-right inset, the 8 dimension uses ticks above/right of the two actual corner borders. V1/source text/state remain sound; rejection is wrong association, not the length of the drawn gaps. |
| `director-cohort-2.png`, 1536 × 1024 | Fail V5; inset association is not unambiguous enough for V6. Main overview/spacing detail are usable and source text/state preserved. In the right inset, a middle short corner/guide is labelled “Director filter options (parent)” while a separate lower corner is labelled input; the 8 arrow ends at the upper guide rather than a continuous identified input border. This contradicts the intended two-fragment parent/input explanation. Not a CAD-proportion failure; the parent/input attribution itself is ambiguous. |
| `settings-cohort-1.png`, 1536 × 1024 | Fail V1/V5/V6. Both overview off-knobs are solid white despite blue-interior outline rule. Required Close height 30 is absent. Overview 22 arrows measure space outside the form rather than form-to-label/select inset. The Close right-15 starts on a detached guide to the right of the actual button. Content, left/off positions and readable composition are preserved. |
| `settings-cohort-2.png`, 1223 × 1286 | Fail V1/V5/V6. Overview off-knobs are again filled white and Close height 30 is absent. In the upper-right select detail, width 198 starts inside the select instead of at its left border; the 12 label-gap guide is not identified as the source label-layout edge. Footer right-15 uses a detached right-of-button guide. These are not harmless differences in raster proportions. |
| `mobile-cohort-1.png`, 1024 × 1536 | Fail V5/V6. In Genres detail the required vertical 8 (Action bottom → Animation top) appears below the second row as an unrelated mark. Bottom-left root-to-Source inset 12 ends at an unconnected short tick, not the separately drawn Source edge. All 14 genres, states and safe visible labels remain recognizable. Cast clipping is less explicit than the source/second sample, but that ambiguity is not needed to establish this failure. |
| `mobile-cohort-2.png`, 1103 × 1426 | Fail V5 on omission: required Director input width366/height44 and root-to-Source left inset12 are absent from the sheet. These were frozen requirements, not additions after viewing it. Its 27-visible-pixel Cast annotation/broken lower edge, actual controls and useful Source/Years/Genres details are preserved. Visible numerical values and status wording show no separate V6 fabrication. |

Director 2 was re-viewed narrowly before final judgment because it is the closest
case. The issue is not that its 8 span looks too wide/narrow: its right endpoint and
parent/input labels are distributed over an extra middle fragment, so the reader
cannot trace one clear pair of named source borders. No stronger defect is inferred.
A plain complete source overview is not enough to validate an ambiguous added detail.

Literal reconciliation / attribution:

- Both Settings outputs received explicit width52 AND height30 for Close, actual
  footer right/bottom boundaries, and blue-interior off-knobs. These failures are
  output deviations; the frozen planner did not omit those requirements.
- Mobile's overview explicitly requires Director366 ×44 and root-to-Source12;
  its Genres detail explicitly requires Action-bottom → Animation-top8. Missing/
  misattached graphics are output deviations. Showing Director measurements in
  another useful detail, as sample1 did, is not itself grounds for rejection.
- Director explicitly requires gap arrows on actual borders and exactly two inset
  fragments with one directly associated arrow. The failed corner/gap diagrams
  contradict that wording. State repeats now remain correct, unlike the baseline.
- Fresh-only and supported-view instructions are present; obsolete edit wording
  and unconditional search/results views have been removed. There is no evidence
  here of the old generic-template omission. More precise failure causes inside
  the image model cannot be established from these six outputs alone.

Smallest shared generic recommendation, not six manual image fixes: make each view
an atomic drawing instruction that carries its primitives, state rendering and full
size/gap tuple together (e.g. both width and height of the same control). Reduce
competing dimension-guide/leader primitives; an inset may use a clearly named cropped
strip of the two actual source borders, with its arrow directly between them, rather
than extra projected corner fragments and separate annotation branches. Preserve the
same measured facts and endpoint meanings; do not lower the acceptance threshold or
silently discard useful required dimensions. This is a candidate prompt simplification,
not a proven fix or authorization to retouch/reroll these six images.

The cohort demonstrates improved source-specific composition but does not establish
reliable fresh acceptance. Preserve all cells and distinguish this result from the
older accepted Director R8 + Settings R4 edited pair. No universal generation claim,
new statistical guarantee or hidden best-of selection is justified. Only this receipt
changed; images/packages/source remain untouched. `git diff --check` passed.

## FRESH01-C2 — all six frozen fresh outputs, 2026-10-10

Source `c32712a8c0ffd08e043cb86b48d2b1fb028b64ea`; selected contracts unchanged.
Root `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-fresh01-c2-2652yyb8/`.
All six images were viewed first, before literal instructions or author narrative.
Then read the frozen manifest/results and affected per-view, source-text, dimension
and state instructions. All three prompt SHA-256 values match the frozen manifest.
Ledger: six requests/returns, zero refusals/edits/rerolls/manual wrappers. C1 and the
baseline pair remain separate; none of their failures was replaced by this cohort.

**Result: 0/6 full V1–V6 passes, with material progress in accepted subscopes.**
V1 passes all six: off-knob and active-row fills are fixed. Native-size meaningful
text remains readable (V4); no typography-size, raster ratio or style-taste threshold
was introduced. Mobile source coverage/dimensions are substantially improved.

| Actual output / native size | Final bounded verdict and observed evidence |
| --- | --- |
| `director-cohort-1.png`, 1223 × 1286 | Fail V3: numerous printed `(record 4/5/7/9/10)` construction IDs in captions and endpoint prose. V2 state explanation is also incomplete: double-outline active rows are visible, but the required keyboard-active/not-applied distinction is absent from the sheet. Meaningful source labels, `spiel`, three results/counts and outline-only rendering are preserved. No extra failure is imposed for its choice of panel layout. |
| `director-cohort-2.png`, 1225 × 1284 | Fail V5/V6: overview 6-px trigger-to-popup arrow floats above the popup-width dimension; its lower end reaches neither popup top nor an extension from that edge. This is endpoint association, not a demand for a proportional 6-pixel raster gap. Content/state, readable source overview, useful spacing detail and two-corner inset otherwise improve; the bottom state panel correctly explains keyboard-active/not-applied for all instances. V1/V2/V3/V4 pass. |
| `settings-cohort-1.png`, 1036 × 1518 | Fail V5/V6: the 12-px label-right → select-left gap is drawn as a VERTICAL arrow near the select's upper-left corner, changing the measured relation. Fail V3: a large source-information block prints CDP identity, carrier-ID and unknown-property machinery rather than a compact client scope note. Close52 ×30 and hollow off-knobs are now present. Source form coverage and the main overview remain useful. |
| `settings-cohort-2.png`, 1036 × 1518 | Fail V5/V6: footer right-15 begins at a detached guide to the right of Close rather than its actual right edge; bottom-15 is correctly associated. Fail V3: the Notes block exposes source-node/CDP identity and excluded-pixel implementation text, violating the existing no-construction-metadata/client-note boundary. Close width/height and off-state outline corrections are retained. |
| `mobile-cohort-1.png`, 1024 × 1536 | Fail V3: the root-to-Source detail prints `(record 1)` and `(record 12)`, and the large Notes panel prints raw `filters_present` interpretation plus AX/CDP identity/collection machinery. V1/V2/V4/V5/V6 pass in the visual/source-presentation scope: all source groups/14 genres and active/empty states remain; Director366 ×44, root12 and visible Cast27 versus full44 are present with intelligible associations. |
| `mobile-cohort-2.png`, 1024 × 1536 | Fail V3: the nine-item Notes wall includes raw `filters_present`, caller annotation/Snapshot provenance, AX/CDP identity and pixel-export implementation details. This is the pre-existing metadata-noise criterion, not a preference for fewer panels. V1/V2/V4/V5/V6 pass: safe visible content, Source All state, placeholders, required sizes/gaps/root inset and explicit partial Cast boundary are materially preserved. |

The mobile successes are bounded visual subscopes, not a full-cell pass hidden behind
an average score. Additional small rounding/raster deviations were not promoted into
failures. Extra source-derived Cast size evidence is truthful; the failure is printing
internal procedural machinery, not explaining the useful clipping boundary. Likewise,
Settings' restored Close30 is accepted rather than repeatedly reopened.

Literal reconciliation:

- C2 explicitly says to print human captions, never object keys or construction
  metadata, and to use one small footer with no metadata inventory. Therefore the
  recorded ID/technical-note leaks are output deviations against an existing rule.
- The same prompts nevertheless contain substantial source `Scope / Not depicted`
  prose about CDP/AX identity, carrier records, unknown fields and raw flags. Outputs
  reproduce that supplied material. This identifies content exposure, not proof
  that total prompt length caused the leak or any other error.
- Director explicitly requests keyboard-active/not-applied text; its sample1 omits
  that meaning while sample2 preserves it. Settings12 is explicitly right→left,
  not top→bottom; Director6 and Close15 also have the correct named pairs in the
  frozen recipes. These are not planner omissions or new after-view requirements.
- Width/height bundles and clipping instructions coincide with observed improvement,
  but two outputs per case do not establish causality or universal reliability.

Smallest generic next hypothesis: separate facts necessary to draw/understand the
client artifact from internal source mechanics BEFORE forming the generation text.
Keep complete machine evidence in its existing package, while the public drawing
brief retains accurate units, state, partial scope and useful clipping limitations.
Use human endpoint names in view recipes rather than printable-looking `record N`
prose; do not rely solely on another “do not print metadata” sentence. Preserve every
required source fact/measurement and the successful atomic size/state bundles.
Remaining relation-direction/attachment failures still require the existing endpoint
contract; deleting dimensions or relaxing it is not a remedy. This is a prompt/design
hypothesis for a future declared cohort, not authorization to repair these six PNGs.

All six outcomes are retained. No unchanged product-source audit, extra agent, image
editing or generation was performed. Only this receipt changed; `git diff --check`
passed before the checkpoint.

## FRESH01-C3 — all six frozen fresh outputs, 2026-10-10

Source `e92dd7dfa0ac18b68bee9e9150bf1e420ff437ff`; selected contract closure unchanged.
Root `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-fresh01-c3-6cv0u75c/`.
All six actual PNGs preceded literal prompt/manifest/result inspection and any author
narrative. Three frozen prompt hashes match. Ledger records six requests/returns,
zero edits/rerolls/manual wrappers/refusals. Native returned sizes are listed below;
C1/C2 failures and the earlier edited pair remain separate evidence.

**HBP-V result: 1/6 complete visual passes — mobile sample2.** C3 removes the printed
construction-ID/technical-note problem. V1/V3 pass across the cohort; meaningful
source text is generally readable. Failures below retain the existing state/anchor/
required-dimension criteria, not a new layout taste or CAD-precision requirement.

| Cell / native size | Final scoped verdict |
| --- | --- |
| `director-cohort-1.png`, 1536 × 1024 | Fail V2: no keyboard-active/not-applied badge or equivalent explanation anywhere on the sheet. Fail V5: separate lower-right vertical-gap fragment places both named source-border corners on its upper level, then measures to a third, unnamed lower line. Its 6 value therefore does not have the specified named start/end pair. Main overview, source text/counts, input width and normal spacing detail remain useful. |
| `director-cohort-2.png`, 1536 × 1024 | Fail V2: keyboard-active/not-applied explanation is absent; the added third result in the partial spacing view also receives a double-outline treatment despite its inactive source state. Correct `spiel`, counts, full overview and most useful dimensions survive. No new failure is based on the schematic width ratios or fragment corner styling. |
| `settings-cohort-1.png`, 1036 × 1518 | Fail V5: required Close height≈30 is absent. The rotated main-height annotation is malformed/not clearly readable as the supplied≈618, so that label does not receive V4/V6 acceptance; no precise alternate value is asserted. The form content, off-state contours, switch dimensions and compact footer remain substantially sound. |
| `settings-cohort-2.png`, 1024 × 1536 | Fail V5: footer right-inset15 begins at the adjacent height-dimension guide rather than being unambiguously attached to the actual Close right border. Bottom15 and Close52 ×30 are present. Other source content/state, outline style, readability and compact presentation pass. This is the existing endpoint-association issue, not a rule about where details should be laid out. |
| `mobile-cohort-1.png`, 1024 × 1536 | Fail V2: All is active in the overview, but the Source enlargement renders All like the inactive Movies/Episodes controls, losing the repeated active state. The Cast27 cut line is also drawn separately above the viewport's lower cut, making the source clipping relationship unclear. Required Director dimensions, root inset, all14 genres and safe placeholders remain. |
| `mobile-cohort-2.png`, 1203 × 1307 | **Pass V1–V6 at native size.** Complete visible groups/text/14 genres and Source All state survive in overview/detail; empty-input placeholders stay placeholders. Cast terminates at the visibly cut viewport boundary, with no invented lower controls. Required390/844, Director366 ×44, root12, Source≈119 ×42/gap2, Years179 ×44/gap8 and Genres179 ×40/horizontal+vertical8 are present and relate unambiguously to their named neighboring objects. No construction IDs, metadata wall, dark/filled panels or false status claim. |

The accepted mobile sample2 does not promise pixel-exact dimensions or touch targets.
Small raster standoffs in otherwise unambiguous neighboring-control dimensions are
not promoted into a new CAD gate; C2 already accepted that bounded geometric scope.
The moved title and portrait composition are not failures. A literal27 annotation
is not newly required: the source's partial Cast boundary is preserved visibly and
the sheet declares partial coverage. The full44 layout is not falsely shown as visible.

Literal reconciliation:

- Both Director prompts explicitly require a short keyboard-active/not-applied badge
  for each repeated active control. Omitting it is an output deviation, not removal
  from the printable-text channel; that channel explicitly includes state badges.
- Settings explicitly requires paired Close52 ×30 and form422 ×618, with right15
  starting at Close-right and ending at footer-right. These failures are not planner
  omissions. Numeric truth remains in the frozen package.
- Mobile's state facts explicitly say All active and apply to every repeated view;
  its Cast object carries the reported27-high visible region ending at viewport844.
  The accepted sample satisfies those facts without inventing below-viewport content.
- Compact printable captions/footer now exclude previous internal source mechanics.
  This observed improvement does not prove that filtering alone caused all changes.

Root's next-remedy hypothesis is compatible with unchanged V5: emit a compact
symbolic illustration of exactly TWO actual named source-border fragments and their
prepared scalar relation. For vertical relations: named upper START bottom border,
arrow, named lower END top border. For horizontal relations: two named vertical
fragments and the arrow between them. Keep labels outside, no third baseline or
competing whole box/legend. Corners were an implementation choice, not an acceptance
gate. This may address the observed Director1 third-line failure; effectiveness is
unproven until a declared fresh cohort. It does not repair state-badge omissions or
missing dimension labels by itself, and does not authorize editing these frozen PNGs.

All six are reported without outcome weighting or best-of substitution. The accepted
mobile cell is one bounded success, not whole-cohort or universal fresh reliability.
No extra generation, source modification or agents. Only this receipt changed;
`git diff --check` passed before the checkpoint.
