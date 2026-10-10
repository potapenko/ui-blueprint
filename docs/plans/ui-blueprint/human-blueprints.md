# Human-readable blueprints — artefact acceptance and fresh-prompt goal

## Authority and outcome

The previous P0–P7 goal is complete at 5ff73b1. This is a new explicitly requested
Evolve goal in the same project: improve the actual ImageGen prompt and generated
blueprints for people, with one implementation owner and exactly two continuing
critics. User requested: remove noisy text/long decimals; explain useful element
sizes and details; decide what belongs on the drawing; only blue and white; never
paste a screenshot-looking UI inside the blueprint; iterate images, use internet
research, and work toward outstanding client-facing clarity. The Leonardo reference
means care/detail/craft, not Renaissance styling, sepia, handwriting or decoration.
User directly said to start the goal and agents and proceed; no reconfirmation of
this supplied brief is needed. Root coordinates and does no product code or image QA.

Final capability: a real CLI-generated, self-contained drawing prompt produces a
clear faithful blueprint of Director and Settings from the existing real data.
Useful overview and selected enlarged details, important dimensions and relationships
are legible; no machine-data dump or screenshot panel. Raw numerical/source truth
and privacy stay intact. Both critics must independently approve the final actual
images in their scopes, not merely the prompt or compiler tests. Report actual limits
honestly; perfection is an aspiration, not a promise of metrically exact raster.

## Spec Basis and accepted change envelope

Root recovered active AGENTS, implementation/root orchestration and the existing
product-truth/QA routes. Repository HEAD 5ff73b1, registry 35; prior product source
and installed baseline a026584, E05 source/privacy review a307bff and I02 b06f3b2.
Read route: docs/specs/README → product/README → EXPORT@1, DRAWING-PACKAGE/STYLE/
GEOMETRY/PROMPT-A/PROMPT-B/REVIEW@1, reference/DRAWING-EXAMPLE@1; CLI@16 and
CLI-EXPORT@2 INPUT/BOUNDS; ANALYSIS@2/TYPES/VALIDATION@1 and explicit dependencies
EXCHANGE@2, GEOMETRY/PROJECTIONS/MODEL/IDENTITY/BOUNDARIES/PRIVACY@1. Original
UIB.DRAWING@1.1 in docs/engineering-blueprint-guide.md is the current visual authority.
Full applicable closure was read by root in the preceding E04/E05 work and remains
unchanged on this same saved HEAD; global implementation/root contracts reread now.
RUST/DEV.RUST@2 and D01/D07 closure apply to implementation/build selection.

New user authority supersedes old visual instructions requiring every raw decimal
and inventory item on the drawing. It does NOT authorize rounding machine data,
pruning observed controls from the source, changing units/measurements/identity,
weakening privacy or relabelling unknown data. Register the precise presentation
Contract Delta/revisions BEFORE implementation; update original guide and selected
routed leaves/root references consistently. No unrelated Native/Web/host/schema/
Cargo behavior, design of the site, dependencies, renderer or new feature platform.

Provisional editorial policy, to refine with initial critic evidence before the
first candidate: human labels use sensible rounding (e.g. 421.640625 CSS px becomes
approximately 422 px), with a clear concise units/rounding note; exact raw values
remain in machine artifacts. Avoid negative-zero/misleading zero, false precision
or invented tolerances. Each visible measurement must have a reason and a clear
pair of anchors. Explain overall extent, important controls, insets/gaps/alignment
and a useful repeated-control detail. Do not demand a dimension for every DOM text
box. Keep the full meaningful form visible; group repetition editorially, never
merge source identities or infer unmeasured CSS padding/radius/hit geometry.

Public visual language: flat solid blue/cyan ground, white outlines/text/dimensions,
with line hierarchy and ample breathing room. No screenshot texture/native UI
fills, black/grey/purple cards, gradients, glass, perspective or invented controls.
No source IDs, M/N inventory walls, raw JSON, timestamps, internal status grids or
large unknown-property lists on the picture. Keep necessary provenance/scale status
compact and truthful; declared draft/checked/approval statuses remain separate.
Human-readable component names and a few detail references are preferable to a
legend requiring the client to decode dozens of IDs. Precise drawing criteria must
be tied to user needs, not critic taste alone.

## Baseline materials and research

Saved real inputs/reference PNGs:
/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-e04-real-tbaWNv
- director-reference.png, settings-reference-complete.png, original lossless PNGs;
- director/settings-raw.json, -snapshot.json, -metadata.json, recorded command arrays.

Prior generated images and literal packages:
/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-e05-1jq7290v
- director-generated.png, settings-generated.png, corresponding *-package/prompt.txt.
Use actual images and raw facts before prior author/reviewer narratives. The source
is explicitly imported real browser data, not new live acquisition. Exact output
is schematic, not CAD. No live website/Native input is needed for this new goal.
Never inspect unrelated repository after-title-spacing.png.

User requested ongoing internet research. Each critic researches its distinct
question from primary expert/official sources initially, and revisits concrete
uncertainties exposed by later images. Do not browse randomly or impose formal
engineering standards not actually read. Keep short source-grounded rules with URLs.
Initial leads (not product authority): NN/G visual hierarchy/progressive disclosure;
Onshape official dimension precision/placement documentation. Native CSS measurements
remain interface units, not manufacturing tolerances. Research informs presentation,
not the underlying product facts or user-defined blue/white style.

## Roles, ownership and cycle

Exactly these three subagents, fork_turns none, inherited model/effort; no nesting.
This direct user request authorizes internal critics for this goal, unlike the old
P0–P7 ordinary visible-chat workflow. Reuse the SAME two critics for every candidate.
Do not spawn replacements/extra reviewers to obtain a preferred verdict.

- HBP-C: content/usefulness critic. Read-only product; own receipt
  receipts/human-blueprint-content-review.md. Establish what the client needs to
  learn, meaningful dimension/detail priorities, what to remove, what is missing.
  Validate rounding, labels/anchors/units/state/source fidelity and final focused
  implementation/privacy compatibility evidence. Internet research on information
  hierarchy and useful dimensioning. No generation or product mutation.
- HBP-V: visual critic. Read-only product; own receipt
  receipts/human-blueprint-visual-review.md. Independently inspect actual images at
  native and intended viewing size: strict blue/white drawing grammar, no screenshot
  panels, hierarchy, typography/spacing/legibility, leaders, detail composition and
  unnecessary decoration. Research primary drawing/visual-communication examples.
  No generation, source code/spec edits or extra agents.
- HBP-I: implementation/generation owner. Own crates/export source/template/tests,
  directly affected export/CLI tests, original engineering-blueprint-guide and
  corresponding drawing/export leaves plus spec-root routing, docs/development/export.md,
  and receipts/human-blueprint-implementation.md. Declare exact write set before edits.
  Preserve canonical schema/engine/data/collector and protected P0–P7 domains.
  No new parameter/config/framework when editing an existing expression suffices.
  Read source/plan independently while critics inspect the baseline; wait for root's
  unified initial criteria before registering presentation choices and implementing.

Root writes this coordination plan, task-registry and execution runbook only.
Critics initially return their own artifact observations BEFORE seeing the builder's
solution narrative. Root consolidates nonduplicated requirements; builder executes
one complete iteration: spec delta → code/prompt → focused checks → actual ImageGen
on both cases → checkpoint/push → neutral candidate handoff (source/data/image/prompt
paths, revision, viewing size). Critics inspect that same candidate independently.
Only after initial observations may root supply builder explanations for reconciliation.
Return concrete blockers/remedies and acceptance by criterion, not vague ratings.

Builder consumes both critiques and repairs the next actual candidate; keep prior
images unchanged. No new research/QA round for unchanged accepted aspects. No fixed
iteration quota and no endless optional perfection work: continue until user criteria
and both critic scopes pass on actual outputs, or identify a precise genuine external
blocker. Do not lower the target to an attractive but unreadable or incomplete image.
At final checkpoint, reproduce through the public CLI and smallest affected installed
path; preserve exact machine data, caller limits/refusals and public text safeguards.
Keep imagegen-prompt model-free. Report source→prompt→image→critique coverage honestly.

## Assets, operations and saving

System imagegen skill, built-in image_gen default; no API key/CLI fallback without
explicit new authority. Inspect local references before generation. An input source
screenshot can guide geometry but must never appear as a raster UI panel in output.
Use only current tool schema; no promised resolution that the returned image lacks.
All images/variants and containing directories are retained in system temp forever
from the agent's perspective (OS/user owns lifecycle); no deletion at any stage.
Copy tool-managed originals to temp without moving/deleting originals; originals
also remain. No Python/SVG replacement for the requested ImageGen output or image edit.
Do not create persistent directories, run apps, modify the site or control Codex UI.

Current master only. Shared fcntl.flock /tmp/ui-blueprint-master-git.lock for empty
index check → exact own paths stage → commit → push established canonical SSH remote
→ release. No git add ., reset/stash/clean/force, new keys, branches/worktrees.
Own transient nonimages may be cleaned after acceptance; shared review inputs and
all images are protected. Each completed coherent step and final receipt is committed
and pushed. Source/visual judgments stay with workers; root receives bounded results.

## Completion

Both real cases have directly generated usable prompts and actual images approved
by BOTH critics against explicit user-derived criteria; meaningful dimensions are
readable and rounded for people, blue/white outline-only design holds, essential
controls/data are correct, irrelevant noise is gone, details answer real questions.
Affected code/compatibility/privacy/install checks pass; exact machine facts are
preserved; source/rule changes and final receipts are saved/pushed. Root shows the
best final images inline with concise outcome/limits and closes the goal only then.
No claim of mathematical perfection, human approval or external publication.

## Initial criteria selected by root — 2026-10-10

Independent initial critiques are saved at content3c4730d and visualc55acfc;
root read both full receipts before this decision. Adopt C1–C7 from
[HBP-C](receipts/human-blueprint-content-review.md) and V1–V6 from
[HBP-V](receipts/human-blueprint-visual-review.md) as the concrete user-derived
acceptance map. They govern actual images as well as the source/prompt boundary.
Visual critic corrected its tentative lower-anchor claim; do not treat that
withdrawn observation as an established defect. The duplicate Director row and
Settings screenshot panel are confirmed independent findings.

First candidate must explain Director trigger/popup/search/result dimensions,
attachment/list/row gaps and measured insets; Settings panel, select size and
rhythm, measured column/edge insets, switch and Close size/position. Use a complete
overview and purposeful enlarged detail(s), not every raw box on one crowded sheet.
Exact reference values and anchors are in HBP-C's table; they are test expectations,
not constants to hardcode into the generic compiler. Do not confuse the Director
wrapper's42.90625 height with the whole open popup's extent. Both Settings switches
stay off; Director keyboard-active suggestion is not an applied filter. Preserve
public source labels; spiel remains explicitly reviewed caller metadata, never a
reason to export arbitrary protected values.

The prompt itself must be a concise human drawing assignment. Keeping the entire
old metadata/inventory dump and merely naming it NONPRINTING is insufficient.
Machine artifacts retain full exact source data; the prompt selects the information
needed to draw the complete meaningful form and its approved dimensions/details.
Any supplemental exact facts retained for placement must have a concrete consumer;
no raw clock/evidence/status/ID walls. Output is blue ground/white drawing throughout,
not screenshot recolouring. One compact factual footer replaces big provenance tables.

HBP-I may extend its exact source set to observed.rs for required gap/inset values
computed through the EXISTING engine::measure_query on explicit known anchors.
This adds accurate derived dimensions without changing schema/engine arithmetic or
altering existing raw dimensions. Additive derived output and editorial selection
must be included in the pre-code Contract Delta. Do not promise byte-identical
human artifacts when their authorized content changes; verify preservation of the
underlying exact fields and existing compatibility/safety behavior instead.

Builder now owns the full first iteration: refine exact plan, register visual rules,
implement minimal generic formatting/selection/measurements, focused tests, literal
public CLI prompts, actual ImageGen output and neutral candidate handoff. No further
root grant needed for its internal steps. Actual raster size/viewing context must
be stated and judged; do not claim unsupported4K or impose an arbitrary font quota.
Both critics continue independently on the same revision/images after handoff.


## R1 disposition and R2 — 2026-10-10

R1 source43d62d4 / installed proof8181a98 is saved, not accepted. Content1a9200e
and visual39c43f6 independently reject incorrect/missing dimension anchors,
Director alignment/state and Settings filled knobs. Hierarchy and native-size
readability pass. One consolidated repair goes to the same HBP-I; the same critics
wait for actual R2 images. Their receipts hold the exact source anchors/remedies.
R2 makes a small required drawing brief per view, preserves useful context around
arrows, corrects the above failures, restores author-reported Space/transform
attribution, and repairs historical authority wording. No new feature, fixture
constants, whole-layout redesign or additional agent is authorized. The source
of each requirement remains the direct user brief and registered HBP-HUMAN-001;
critic findings are observed evidence, and mandatory per-view selection is the
chosen implementation remedy, not external research creating product authority.
Root writes only this plan and the registry for this checkpoint. Final acceptance
still requires both actual-image verdicts and final-source compatibility evidence.


## Final acceptance — 2026-10-10

Definition of done satisfied by Director R8 + retained Settings R4, both1536×1024
at native100%. HBP-C250efbd accepts C1–C7; HBP-V4bf5759 accepts V1–V6. Source
b008a44 and implementation receiptfcc550d close the final frame/role restoration,
focused23compiler/20CLI checks, Clippy/formatting and installed core reproduction.
The installed final source reproduces all six files of both R8 packages exactly.
Settings R4 remains attributed to cd099a1/R4 prompt; its R8 package reconciles final
source compatibility, not a newly generated image. No remaining mandatory gap.

The useful final composition is full overview → partial repeated-control spacing
view → isolated real-border inset detail → compact factual note. This resolves the
ambiguous duplicated size chains without removing meaningful controls or machine
facts. Intermediate failures and their actual corrections remain in the two critic
receipts; no criterion was weakened or replaced by a favourable new reviewer.

All generated images/tool originals are retained. The task registry holds exact
paths and immutable revisions. Scope is the named schematic imported-reference
artifacts and bounded source compatibility, not CAD scale or universal ImageGen
reproducibility. Root saves the final checkpoint, delivers images inline and closes
the verified goal; there is no further implementation or review packet to dispatch.


## New active goal FRESH-01 — 2026-10-10

Actual user authority after prior goal completion: «создавай себе новую цель» to
improve the script's generated prompt from the observed R1–R8 problems, so ImageGen
can produce the desired blueprints. Root explicitly explained that previous final
images depended on editing prior generated images and did not establish first-pass
reliability. The user then directed another full pass. This continues the same
explicitly requested one-builder/two-continuing-critic scope; no new agent fan-out.
Prior artefacts remain accepted as images; they are not fresh-generation evidence.

Outcome: improve the existing generic CLI prompt/compiler and demonstrate its
bounded fresh-generation quality on several real forms. An original UI screenshot
may be a reference; a previous generated blueprint, hand-added prompt wrapper,
manual image fix or cherry-picked reroll cannot qualify. Every attempted image is
retained and its actual result reported. Perfect arbitrary future stochastic output
is not promised; the final report must distinguish verified matrix from unknowns.

Root recovered current global AGENTS/implementation, local AGENTS and
spec registry36 → product branch → EXPORT2 and its previously fully read drawing/
geometry/projections/model/identity/boundaries/privacy/example closure. Original
DRAWING1.2 HBP-HUMAN-001 rules remain authoritative. CLI16/CLI-EXPORT2 and selected
analysis/types/validation/exchange dependencies continue for source verification.
RUST/DEV.RUST and prior installed workflow apply only to actually changed owners.
Baseline product b008a44, final earlier coordination1599ce4. No authority/epoch drift.
New requirement is fresh-prompt validation; source/schema/collector/privacy/core
contracts remain protected. Plan is an execution record, not new product authority.

First finite wave: HBP-I diagnoses current prompt and produces one baseline fresh
Director and one baseline fresh Settings using exact current CLI output and only
original E04 references. No source change in this first packet. HBP-C and HBP-V
independently translate their existing observed defects into concise generic
requirements and a fair frozen evaluation, not more optional image-style rules.
Root requested one additional existing safe real form from authorized web advisor
Research website UI blueprint (01a1102f-e21d-7251-9597-c29a1c66d088); no website code
change, new project work, publication or synthetic substitute. Exact baseline paths
and new-form availability are pending, not invented.

Proposed evaluation, to pin after those bounded receipts: three real forms
(Director, Settings, new form) with two independent fresh images each from a frozen
source/prompt revision. Report all outcomes; do not edit failures into passes or
pretend six samples establish universal reliability. Keep C1–C7/V1–V6 substantive
facts/clarity/privacy while avoiding CAD precision/taste-only gates. First qualify
input evidence/coverage/privacy; no unapproved private values. If source changes,
recheck affected cases and explicitly distinguish earlier versions/results.

Implementation then owns one complete generic repair and validation packet:
register necessary contract clarification before code, edit existing template/
selection only as supported by observed evidence, focused tests, literal CLI
packages, fresh generation matrix, neutral evidence to same two critics, saved
source install reproduction. No new flags/framework/renderer/dependencies or
fixture-specific product constants. Review examines generator and matrix, not
endless retouching of a selected image. Exact final threshold/claim must be pinned
before candidate generation; cannot be weakened after viewing results.

Root write set: this existing plan, task-registry, execution. HBP-I retains exact
exporter/template/affected-test/spec ownership and own implementation receipt;
critics write only their respective existing receipts. No nested delegation.
All original system-temp/image retention, current master/shared Git lock/exact
staging/commit+push, unrelated after-title-spacing.png protection, no new persistent
directories/worktrees/branches, no Codex Computer Use constraints continue. Read
only applicable missing instructions, don't repeat unchanged broad test/audit waves.


### FRESH-01 protocol pinned before candidate images

Root adopts the matched HBP-C9aaba61 / HBP-V97e0371 proposals. Final cohort is
Director, Settings and mobile Clip Search Filters visible viewport (390×844,
verified responsive UI) × two separate fresh ImageGen calls each. Third case was
selected by the authorized web advisor before any candidate images; live reference
capture and exact paths remain pending. Baseline Director/Settings fresh pair is
separate diagnostic evidence, never a substitute for favourable candidate cells.

All six outputs must pass unchanged substantive C1–C6/V1–V6, with bounded C7 on
frozen source/packages, to support the final bounded cohort claim. No statistical
independence/rate estimate, universal perfection, CAD or future-rerun guarantee.
Every failure stays in its cohort; a materially changed prompt revision gets a new
complete declared cohort, never replacement of only failed cells or image retouch.
Source/question priorities and exact input paths are pinned before those calls.
A runtime input gap is waiting_evidence, not permission to fabricate a holdout.

This acceptance protocol is root's verification choice under the user's new prompt
quality goal; it adds no product feature, schema or arbitrary raster precision gate.
Existing user-approved image quality criteria remain. Prior image acceptance stays
historical and is not silently relabelled as pipeline reliability.


### FRESH-01 baseline reconciliation and third input

Both critics independently rejected current b008a44 fresh Director/Settings pair.
They confirmed an unconditional search/results/inset detail recipe and old edit
preamble in the actual Settings CLI prompt; invented search/results in its image
are thus supported template contamination, not speculation about ImageGen. Director
also has redundant inset/state-detail failures. Baseline image directory is recorded
in the registry. These observations authorize correction of the existing generic
prompt within the approved scope, not fixture-specific exceptions.

Third input was obtained by the authorized web advisor and independently read by
both critics before candidate outputs. Source handoff in system-temp
uib-mobile-filters-OBW93z documents non-atomic but bracket-stable viewport390×844,
53visible/partialDOM/26addressedAX, zero saved raw input values, restoration and
partial lower-content coverage. Full captured scope includes Back/Filters,Source
modes,Years,14genres,Director and truncatedCast. Actual fullCast44height differs
from visible27; do not confuse clipping with control height. There are no result
rows/popup/Apply/Close/footer in this scope. Expected layout questions come from
these measured facts, not generic recipes copied from the first two cases.


### FRESH-01-I accepted implementation packet

Root accepted the builder's finite plan after both independent baseline findings.
Authority is the user's explicit new-goal execution request plus existing source-
faithful/self-contained EXPORT2/DRAWING1.2 obligations (Restore), not an invention
from an attractive image. The unconditional Director recipe/edit preamble is the
confirmed implementation defect. Generic per-view selection is the chosen remedy.

Exact product writes: crates/export/src/compile.rs, prompt-template.txt in that
directory, crates/export/tests/compiler.rs, docs/development/export.md, existing
implementation receipt. Reuse current roles/topology/anchors to emit one coherent
view plan; supported control-size/repeated-adjacency/inset details only when source
supports them. No new framework/model/schema/config/flag/dependency or screen-name
constants. Preserve machine files/frame/Space/roles/privacy/refusals and distinguish
placeholders/public draft, pressed/selected/checked and clipping. Temp third-case
import must retain actual provenance and explicit limitations, not pretend to be
canonical live acquisition. A genuinely missing contract is returned before coding.

Focused positive/negative generic selection and protected-data tests; no unchanged
full-platform checks. Freeze committed source, literal packages and exact original
refs before first candidate call. Return short neutral manifest+adapter limitations,
then execute entire predefined6 without microstep approval if no authority/data gap.
No prototype/reroll images between baseline and cohort. All outputs go to same two
critics image-first before author narratives. Verify installed final source and
literal packages; checkpoint/push under established lock. Later changes get a new
cohort, preserving every previous failure. Root does not perform source/image QA.


### Frozen cohort FRESH01-C1

Source76f14ac5a2ce6cb45a54e9c16b64342492d361fb saved/pushed before first candidate
call. Manifest `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-fresh01-baseline-hhcyuw74/frozen-cohort-manifest.json`
pins literal prompts, exact commands, snapshot/metadata/ref hashes and all6 planned
output paths. Director8790 / Settings14496 / Mobile17431 prompt characters; all
five non-prompt files match prior baseline or qualified mobile carrier exactly.
Author compiler26/CLI20/Clippy/formatting passed; this is not image acceptance.
Matrix uses original UI references only, no history/edits/wrappers/rerolls. Both
continuing critics inspect all actual images before author outcome narratives.


### FRESH01-C1 disposition before further implementation

All6 literal fresh requests returned images; none passed every mandatory criterion
in either critic's scope. V00c7a06 records the six individual failures; content
reconciled all required omissions/anchors against actual plans before declaring0/6.
Do not reinterpret the frozen threshold after seeing these results. Settings no
longer invents search/results, so applicability repair is useful but insufficient.
Images fail mostly to carry explicit source-edge associations or full dimension
bundles; required numbers were present. Compact local geometry/measurement bundles
are an evidence-informed next hypothesis, not proof that prompt length alone caused
failures. Any next candidate must preserve complete meaningful structure/state and
same useful-size/anchor criteria, with no image retouch or cherry-picked reroll.

Content additionally verified new clipped(c) inference conflates different frame
meanings: any unequal VisibleRegion can mark AX/Hit/Paint fallback as CLIPPED and
suppress dimensions. Next bounded Restore must require proved comparable layout/
visible subset or report separate visibility without that claim. Mobile actual
27visible/44layout remains valid, machine facts stay unchanged. Final C7 not accepted
until this exact gap is repaired/verified. Saved-source installedbf70583 evidence
is retained as successful execution, not a substitute for acceptance.

Root requested a finite source/prompt proposal from same builder, while source and
C1 artefacts remain frozen. A further implementation packet/new6-image cohort is
not yet dispatched. Preserve separate immutable cohort manifests and all failed
outputs; do not overwrite C1's frozen inputs/results. No new reviewer or product
criterion is introduced to obtain a favourable verdict.


### FRESH01-C2-I accepted finite repair

Root accepted the builder's compact per-view plan under existing user authority,
C1observations and C7helperfinding. Same five product write paths as FRESH-01-I.
Use render-needed controls/text/state and indispensable anchor context per actual
view; preserve full meaningful overview and all original machine/source records.
One complete W/H bundle belongs to its shape; each scalar has one view owner.
Gap/inset fragment contains two actual connected source-border pieces and one
arrow, not detached badges/legend or an extra invented container. Keep state and
layout/visible facts beside their affected object. Remove redundant construction
IDs/coordinate dumps, not mandatory measurements or meaningful source content.

Restore clipped() only for proved comparable known layout/visible proper subset;
reuse existing engine Inside when narrow. Different/unknown/nonrect/overlapping
or AX/Hit/Paint frames retain attributed visibility without CLIPPED/size suppression.
Do not create a new conversion/visibility framework merely for this helper; safe
conservative distinct-frame reporting is preferable to unsupported inference.
Preserve valid source mobile27/44. Focused frame/containment negatives and positive,
compact-plan/W-H ownership/content regressions and exact machine comparisons.

After tests, freeze new source/three literal prompts/refs into a NEW immutable C2
manifest and distinct system-temp outputs. Never overwrite C1. Same three inputs,
same criteria, full new six fresh requests; no prototypes/edits/wrappers/historyrefs
or cherry-picked rerolls. Freeze notification precedes calls; no microstep approval
needed unless a real authority/data gap appears. Neutral all6 to same critics before
author outcome narrative; saved-source installed reproduction. No broader features,
new agents, branches/worktrees, website/collector edits or Codex Computer Use.


### FRESH01-C2 frozen before images

Sourcec32712a8c0ffd08e043cb86b48d2b1fb028b64ea, new independent system-temp root
uib-fresh01-c2-2652yyb8, frozen-cohort-manifest.json pins the same qualified source
and original-reference hashes, three new literal prompts and all six output paths.
Prompt lengths7633/9993/12035, without claiming length alone proves improvement.
Manifest links retained C1 rather than replacing it. Same fixed criteria and original
UI-only generation policy; all six requests run without edits/rerolls/wrappers.
Author27focused compiler checks pass, five nonprompt files of each case unchanged.
Visual acceptance and final independent source/installed reconciliation pending.


### C2 disposition and accepted FRESH01-C3-I

All six C2 outputs retained. HBP-Cee7a45b accepts mobile2's C1–C6 only (1/6),
HBP-V9fca9be records0/6 complete passes: technical source-ID/metadata blocks violate
original V3 even where mobile geometry/state are correct. V1 passes all six; mobile
V2/V4/V5/V6 pass. Other actual defects include missing Director not-applied state,
misspelled Settings text and detached/wrong-axis scalar dimensions. No new gate or
retrospective threshold change. Saved-source installerd22d7b7 reproduces all three
C2 packages exactly, independently from acceptance.

Clipping repair passes C7's reviewed scope. Three concrete compact-object losses
remain: Enabled, state_and_actions and a per-object Space/context reference. They
were present before compaction; global coordinate lists without object linkage do
not preserve multi-Space attribution. This is bounded changed-owner repair, not a
new audit programme.

C3 same builder/same five write paths now owns one complete finite correction:
resolve compiler-generated record references into source-derived human labels;
explicit printable labels/captions and one compact truthful footer; leave CDP/AX
procedure/unknown-property dumps in machine brief/manifest. Suppress by source
channel/ownership, never regex-delete those words from genuine UI text. Restore
safe meaningful states and compact object/context linkage without a metadata dump;
state/actions remain reported or proposed, not claims of executed interaction.
Keep true scope/partial/non-atomic/unit/status distinctions. Strengthen local
source-border scalar orientation/complete geometry while preserving every required
size/state and unambiguous anchors—no badge-only substitute or relaxed C3.

Focused state/context/alias/privacy/output regressions, unchanged clipping evidence
reused. New committed source/three literal prompts/original refs get a NEW immutable
C3 manifest and six fresh outputs, no prototype/retouch/reroll/wrapper/history images.
Preserve all C1/C2 failures. Same critics inspect all6 image-first, then bounded source
and installed evidence. No extra planning approval needed or broader features,
frameworks/schema/flags/agents/directories/branches/worktrees/collector/site changes.


### FRESH01-C3 frozen before images

Sourcee92dd7dfa0ac18b68bee9e9150bf1e420ff437ff saved/pushed. New system-temp root
uib-fresh01-c3-6cv0u75c and frozen-cohort-manifest.json (hash9c247e9647a62e2cecb30a578e9edaaf2ffeb39d9867a4e6c8fd5cab98a537c9)
retain exact original inputs/refs, pin three literal packages and six new paths.
Author29focused tests/Clippy/formatting passed; five machine files of each case equal
C2. This does not assert visual acceptance or final C7. All6freshcalls follow same
unchanged protocol; C1/C2 outputs and their failed verdicts remain immutable.


### C3 disposition and accepted FRESH01-C4-I — 2026-10-11

C68342bb/V76b4735 independently accept mobile2 only (1/6); no full cohort pass.
ID/procedure/unknown-property pollution is gone. Remaining actual failures are
missing required state badges, wrong source-border association and omitted Close
height. The Director semantic badge exists in required printable instructions;
its absence is not labelled missing source data. C7's bounded Enabled/state/actions/
Space restoration passes; installedc312dbe reproduces all three frozen packages.

Same builder now implements one bounded next hypothesis: symbolic ASCII/box-drawing
mini-layouts inside literal prompt for scalar pairs. Exactly two actual source-edge
fragments, prepared number and human endpoint names; vertical upper-start/arrow/
lower-end, horizontal two verticals/arrow between. No third baseline/extra whole
box/legend. Critics confirmed this preserves C3/V5; corners were a technique, not
an acceptance gate. Redraw in blueprint linework; do not print construction tokens.
Signed/zero/reversed/unknown cases must not imply invented positive separation;
conservative existing attribution is preferable where a sketch cannot be truthful.
No new geometry engine or metric-CAD promise.

Use explicit per-view printable caption strings for required semantic states and
retain state across repetitions. Keep source-derived meaning (selected is not
silently applied). Put W/H height left when a right inset exists, so a height guide
cannot stand in for the measured object edge. Both actual edge dimensions remain
required; no badge-only fallback or removal of difficult scalar facts. Current
noise filtering, safe states/context, clipping and all exact machine facts stay.
Same five writable paths; focused changed-format/state tests, no broader audit.
Freeze NEW source/literal packages/original refs in separate C4 system-temp manifest,
then all6 fresh calls under unchanged protocol, no prototype/retouch/history/wrapper/
reroll selection. Same two critics image-first and bounded installed reconciliation.


### FRESH01-C4 freeze before images

Source8db76ba7cf4d3298f5a5cd89388a97967595659d saved/pushed; separate system-temp
uib-fresh01-c4-d3yemcfo/frozen-cohort-manifest.json hash79eb884fffe90a19481ae1daae98d4a076bd9ade599abf5edbcd6e4d69e290a9
pins same original source/ref hashes, new literal packages and all six output paths.
Author31focused compiler tests/Clippy/formatting pass; five nonprompt files per case
equal C3. Known positive common-TopLeft scalar pairs get symbolic layouts; other
cases retain conservative attribution. Both Director views carry exact declared
state caption; mobile repeated-state clauses do not infer Applied from Selected.
Six fresh requests execute under unchanged protocol; image/source acceptance pending.


### C4 disposition and accepted FRESH01-C5-I — 2026-10-11

Both critics accept D1/S1/S2/M1, 4/6. M1's initial content doubt was reconciled using
its long exterior fragment, actual Source-label edge and overall viewport context;
requiring an extra literal name would be a new gate. M2 lacks that endpoint context
and remains ambiguous. D2 omits required popup width194. No output was replaced and
no criterion weakened. V5e790f2 records its full scoped evidence. Installedb57d9e4
reproduces all three exact8db76ba packages, separate from image acceptance.

One new symbolic-helper compatibility gap: valid CenterX/CenterY anchors can be
misclassified as vertical/bottom borders. This is within the assigned orientation
scope. C5 restricts corner sketches to actual L/R/T/B border pairs and keeps centers,
mixed/unsupported/nonpositive cases as correctly oriented named-anchor fallback.
No new engine or meaning change; ordinary cohort inputs did not exercise this gap.

Same builder/same five paths owns a small complete next correction: an explicit
below-shape slot and exact label for the already selected outer/enclosing overview
width, separate from contained/trigger widths; exact printable human captions and
context for both source edges of every scalar fragment. Unpainted label-layout or
scope boundaries must not be mistaken for another control's border. No added
measurements, fixture constants, badge-only substitutes or restyling. Preserve the
four accepted examples' semantics and existing source/state/privacy/context logic.
Tiny focused center-axis/eligibility/slot regressions, exact machine comparisons,
then NEW frozen C5 six fresh outputs from the same original refs and unchanged
criteria. All C1–C4 failures stay; no prototypes/retouches/reroll selection. Same
critics image-first, bounded installed reconciliation, no expanded audit/features.


### FRESH01-C5 freeze before images

Source405882351f90bb5e468c20989d7d4c3f0baed115 saved/pushed; separate system-temp
uib-fresh01-c5-5gew_bxr/frozen-cohort-manifest.json hash12c83375746d5e2a41483032815b44b87b4b39d1aac2a2a1affefc6bc1e18037
pins unchanged qualified inputs/original refs, new literal packages and six paths.
Author33focused tests/Clippy/formatting pass; five machine files×3 equal C4.
Existing Proposed distances are absolute, so positive values alone do not prove
source-edge ordering; those retain named-anchor fallback rather than a new corner
recipe. Same full six fresh first calls/criteria, no previous image references,
retouches/wrappers/rerolls; C1–C4 remain independent retained cohorts.
