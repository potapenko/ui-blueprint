# Compile and review a blueprint

Use the bundle selected in [the skill](../SKILL.md). Prompt compilation is local
and model-free. Generating an image is a separate user-authorized tool operation.

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

## Generate only when requested

Use the exported prompt literally. For separately planned sheets, use the complete
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
