# Compile and review a blueprint

Use the bundle selected in [the skill](../SKILL.md). Prompt compilation is local
and model-free. Generating an image is a separate user-authorized tool operation.

## Choose the prompt author

For a human-facing blueprint, prefer the agent-composed workflow below: the library
supplies exact facts and you turn them into a selective drawing assignment. The
literal CLI export remains available when requested. Both paths require explicit
user authorization to generate an image. Keep these provenance claims separate.

## Prepare document annotations

Use the original core 0.1.0 Snapshot or one observed ChannelResponse as `SNAPSHOT`.
Do not reconstruct measured coordinates, change source IDs or merge channel records.
Write a separate `METADATA` JSON file in system temp. This is its complete basic shape:

```json
{
  "metadata": {
    "document_id": "UI-DOC", "revision": "1", "title": "Selected interface",
    "audience": "Developer", "language": "en", "date": "2026-10-13",
    "owner": "Document author", "retention": "Transient package for this review",
    "specification_refs": ["UIB.DRAWING@1.2"],
    "approval": {"status": "draft", "named_record": null},
    "page_format": "A3 proportions", "output_size": "3840 x 2160 output pixels"
  },
  "state": "unknown", "scope": "Selected source scope", "environment": "unknown",
  "safe_source_reference": "Observation selected for this document",
  "not_depicted": ["Uncollected or hidden UI is not claimed"],
  "public_text_fields": []
}
```

Replace the example document ID, revision, title, audience, language, date and owner
with actual document annotations; choose retention/output size for the requested
deliverable. Keep unavailable state/environment explicit as unknown. These annotations
do not create new runtime facts. Approval stays draft unless an actual named approval
exists. `public_text_fields: []` is conservative; explicitly allow reviewed safe text
only when necessary. Raw value fields and pixels are not automatically exported.

### Metadata format failures

`specification_refs` must be nonempty; use the applicable guide reference from
this skill or the bundle. The current CLI's public-metadata syntax guard also
rejects `/`, backslashes, URLs and template delimiters, even in benign prose.
Thus `export_private_content` can indicate metadata syntax, not a detected secret.
Identify the actual field before proceeding. In a separate metadata copy, an
explicitly reviewed public annotation such as “layout / accessibility” may be
rephrased as “layout and accessibility”; preserve its meaning and original file.
Never mass-strip unknown strings, erase privacy markers, change the Snapshot or
reinterpret a refused private payload as public. If the cause remains unclear,
report the gap before sending data to an image model. Raw source privacy and
caller-annotation syntax are separate checks.

## Compile from the original observation

Create a unique temporary parent; the package child must not already exist.
The block's exit is the actual compiler result. Limits below are example choices:

```sh
(
  umask 077
  UIB_EXPORT=$(mktemp -d "${TMPDIR:-/tmp}/uib-export.XXXXXX") || exit 1
  printf 'Export directory: %s\n' "$UIB_EXPORT" >&2
  "$UIB_BUNDLE/uiblueprint" imagegen-prompt \
    --snapshot "$SNAPSHOT" --metadata "$METADATA" \
    --out "$UIB_EXPORT/document" --purpose document \
    --max-input-bytes 2000000 --max-output-bytes 4000000 \
    --max-components 256 --max-views 8 --components-per-detail 12 --json
)
```

Both input files and the assembled brief must fit the input bound. All package
files plus stdout share the output bound. Existing destinations and invalid/private
inputs refuse; do not delete an existing export to force success. Exit 0 means
package written, 1 IO/internal, 2 invalid/limit/destination, 5 unsupported profile.

Read the manifest and receipt. Six base files are `drawing-brief.md`, `scene.json`,
`dimensions.json`, `sheets.json`, `manifest.json` and `prompt.txt`. Split single-view
packages may also contain `prompt-G01.txt`, `prompt-D01.txt`, etc., enumerated in
`sheet_prompts` under export 0.4.0. Do not assume a fixed six-file inventory or confuse
package version with CLI release/core 0.1.0/analysis 0.2.0. Source, local numeric
validation, image verification and human approval are separate statuses.

For a proposal, use a complete explicitly proposed DrawingBrief with `--brief FILE
--purpose propose` and the same output/limit flags, replacing snapshot/metadata.
The bundle's `example-proposed-brief.json` demonstrates its shape; never present its
synthetic geometry as an observation or the user's design requirements. Detail/flow
also use a complete brief with the required source/state/transition evidence.

## Literal CLI prompt — only when selected

When the user asks for literal CLI output, use the exported prompt literally. For separately planned sheets, use the complete
per-sheet prompt enumerated by the manifest; do not concatenate sheets or replace
their assignments with a hand-written summary. Include only privacy-reviewed source
references belonging to the selected target/state. A generated drawing is not a
source screenshot or a fresh measurement. Do not attach raw JSON/evidence dumps to
the model merely because they are present in the package.

The current Blue Engineering presentation uses a uniform blue background and white
outlines/text/dimensions, including inside controls. Preserve complete selected
scope, useful detail views, anchored dimensions, prepared rounded labels and a short
status footer. No invented dimensions, printed machine IDs, decorative grid, filled
dark panels or screenshot panels. Original machine numbers remain exact; rounded
display labels do not change units or create exact dimension sums.

## Review and deliver

Check the actual returned image at delivery size against the package: component
inventory and state, safe text, every required dimension/units/anchors, relative
layout, scope/coverage, readability and privacy. Internal IDs support correspondence;
they need not be printed. Check all required sheets, not just the overview. Distinguish
keyboard-active from applied selection and accessibility names from field contents.
Keep an unreviewed or failed image unverified/draft; compilation is not image approval.
Even a reviewed raster is schematic, not a verified CAD scale.

Save generated images and any references/crops in system temp; retain them without
agent cleanup and show them inline using absolute paths. Copy tool-managed outputs
to temp without modifying originals and disclose that the originals remain. Remove
only run-owned transient non-image files after use; preserve directories containing
images. A text-only prompt request is complete with the package and relevant status;
do not call ImageGen simply to demonstrate the export path.

## Agent-composed blueprint: teach the geometry

Use this path when the user wants a useful human-readable engineering drawing,
including when the literal CLI prompt is too noisy. The library owns facts and
calculations; you own editorial selection and the drawing instructions. Say which
path produced the prompt. Never present your composed text as literal CLI output.
The CLI export remains a supported alternative; do not rewrite it when literal
output was explicitly requested.

The question the sheet must answer is: **what could a developer reconstruct or
check from this drawing that is not already obvious in a screenshot?** Prioritize
sizes, clear gaps, insets to named boundaries, common edges/centers, repeated
control sizes and measured proportions. A white tracing of a screenshot is not
an engineering explanation. A video/photo/map is a labelled empty region, never
a drawing of its pixels. The blueprint must work without a screenshot reference.

### Turn source data into a drawing brief

1. Read the selected observation through inspect or the saved structured data.
   Identify the outer frame, main regions, reported groups and meaningful controls.
   Separate source labels from input values. Use only public allowed fields.
   Accessibility frames describe measurement envelopes, not painted borders.
   Do not infer missing functions, clipping, text metrics or a layout tree.
2. Choose the few relationships that explain the composition. Examples: common
   left edge of fields; equal gaps along a tab row; a central action between equal
   buttons; a field and trailing actions inside a footer; an overlay's attachment
   to a named edge. These are selection examples, not a mandatory screen structure.
   Reported hierarchy is useful; geometric proximity alone is not semantic ownership.
3. Use `references/analysis.md` to run the Rust `measure` command for selected
   relationships. Use source dimensions already computed by the export or measure
   them too. Never calculate drawing quantities by eyeballing a screenshot or ask
   ImageGen to calculate them. Query actual source keys and a common Surface,
   frame kind and Space. A declared value, an observation and a suggestion differ.
4. Keep a compact fact list beside your prompt: each visible dimension/relationship
   has source objects, edges, units and a known engine result. Exact values stay
   there; round display labels to whole UI units with ≈ for nonintegers. Preserve
   nonzero values below half a unit as <0.5 rather than zero. An actual zero means
   a shared edge/axis, not a tiny dimension arrow. Rounded equality is not a pattern.
5. Select useful groups for enlarged details. A small overview locates those groups;
   the details carry the informative dimensions. Usually a few details fit on one
   sheet. Do not spend most of the image on empty context or draw every text box.
   When more sheets are needed, explain their scope; one user-requested result
   remains one generation. Unshown details stay in the data, not silently “absent”.
6. Describe the relationships as plainly as their evidence allows: “these controls
   have equal measured width”, “the two edge gaps match”, “their centers align”.
   Tie each statement to named elements. Do not infer a global spacing scale,
   developer intention or a recommended target from one observed coincidence.
   Give a recommendation only against an explicit requirement or a clearly labelled
   proposal; a measured4-versus5 difference alone does not prove a bug.

### Prompt scaffold — fill with this interface's measured facts

Write the skill/instructions in English. Compose user-facing labels and your
response in the user's conversation language unless they requested another;
preserve actual UI labels as source text. The model instruction itself may be
English. Never hard-code a particular product, source IDs, dimensions or the
composition of a previous example into the reusable skill.

Use the following outline, adapting the number and arrangement of details. This
is a scaffold to fill, not text to submit with unresolved placeholders:

```text
Create one readable engineering drawing of [selected interface/scope].
Goal: explain [the most useful measured spatial relationships].
Label language: [user's language]; preserve quoted UI labels.

Style: flat blue background, white lines/text only, same blue inside shapes.
Draw measurement envelopes, thin dimension/extension lines and shared axes.
No screenshot tracing, film/photo contents, decorative icons, gradients or grid.
Source basis: [layout OR Accessibility frames], [units], [partial/known coverage].
These are [observed/proposed] facts. Schematic drawing; do not measure the raster.

Small overview: [outer dimensions and region placement from data].
Media: plain region named [source-supported name], contents omitted.
Mark the regions linked to details. Do not invent dimensions for unknown regions.

Detail [name tied to overview]:
- Objects: [brief human captions, dimensions, relative arrangement].
- Dimension chain: [named edge → named edge = prepared display value + units].
- Alignment: [named coincident edges/centers]; draw one shared guide, no zero arrow.
- Repetition: [specific objects/gaps known to be equal]; show a common dimension
  or bracket and a short explanation, not a dump of identical labels.
- State/limits: [only relevant source or explicitly attributed caller facts].
[Repeat for the other useful details, not every source node.]

Numbers and anchors above are authoritative. Copy their units and approximation
marks; do not add dimensions or calculate sums from rounded labels. Captions
identify the measured frame. An inset is not proof of CSS/SwiftUI padding.
Use readable names, not AXUnknown, node IDs, occurrence counters or debug enums.
Use neutral names when source names are unavailable; never guess a control's job.
Do not print input coordinates, paths, generation instructions or metadata tables.
Footer: [short source/coverage qualification] · [units] · ≈ rounded · schematic.
```

Keep the final prompt self-contained and selective. No raw JSON, evidence dump,
long source inventory or giant safety section goes to ImageGen. Do not strip the
very dimensions and relationships that justify the drawing. Position information
may be unprinted drawing input; it must not turn into coordinate clutter.

### One attempt, observable feedback

Save the composed prompt before generation, then pass that exact text to the
available image-generation tool. A reference image is not required for geometry;
when independently authorized, it may clarify identity/appearance but never supplies
measurements or permission to trace media. Use the requested number of attempts;
for a one-result request, generate once and do not silently reroll or retouch.

Inspect the actual output against the fact list: dimension values, approximation
marks, units, arrow endpoints, axes, equality claims and scope. Also ask whether
its useful facts can be read at delivery size. A useful template does not make a
raster accurate: state concrete remaining errors without claiming CAD fidelity.
Show the image inline and identify the prompt as agent-composed from library facts.
Retain the exact prompt and selected package for the user's review in system temp;
keep all images and tool originals. Do not turn auxiliary logs into repository files.

For a cold-start skill test, give a fresh chat only this installed skill, a compatible
bundle, raw source inputs and an ordinary task. Do not provide a successful prompt,
prior image, preferred measurements, this conversation or the expected answer.
Evaluate the returned artifact and name the specific missing instruction before
changing the skill. One weak image is not permission for an unbounded generation loop.
