# Model-free engineering export — E01 candidate

`uiblueprint-export` compiles a complete DrawingBrief into six files:
`manifest.json`, `drawing-brief.md`, `scene.json`, `dimensions.json`, `sheets.json`
and `prompt.txt`. It does not collect runtime data, open source references,
generate an image, execute actions, or require a model, network or API key.
This is an Evolving candidate under [EXPORT@1](../specs/product/export.md) and
its full [DrawingBrief closure](../specs/product/drawing-package.md).

## Entry points and integration

`DrawingBrief::from_json(bytes, ExportLimits)` bounds input before parsing and
validates it. `compile(&brief, limits)` also validates programmatically assembled
inputs, then returns immutable `Package::files()`. `Package::write_new(path)`
creates only the explicitly named new directory. Existing files, directories and
symlinks are refused; accepted revisions cannot be overwritten. IO errors and
validation errors retain no input strings or paths. Failed writes clean up only
files created by that invocation; successful packages belong to the named owner
and retention condition in the brief.

`ExportLimits` has explicit input bytes, total output bytes, component/view counts
and components-per-detail. No numbers are product defaults or calibrated D05 caps.
Encoding is bounded; total package bytes include all six files. Rust values
already exist before a programmatic call: this API is not a process memory cap.

The public `uiblueprint imagegen-prompt` command now accepts an explicit local
DrawingBrief with embedded canonical Snapshot or explicit ProposedLayout, reviewed
metadata and caller limits. Its [finite CLI contract](cli.md#e01-public-imagegen-prompt-command)
records syntax, aggregate output bounds, versioned receipt and exit behavior.
No engine/schema wire change is requested. Direct saved observe output → document
package is now connected through --snapshot plus explicit --metadata, using the
same canonical loader as analysis. Live acquisition is separate; no hidden session
or stored-ID lookup, recollection or permission expansion occurs during export.
The engine's factual `measure_query(&Snapshot, &GeometryQuery,
&EvaluationContext) -> Result<MeasurementResult, GeometryError>` API computes
observed rect width/height. Export constructs a canonical GeometryQuery with
scope, targets, operation, anchors, quantity kind, units and empty applicability
conditions; it creates no placeholder Expectation, expected value, comparison,
tolerance or normative source. The engine re-exports schema::analysis result types;
Known carries `measurement`, Unknown retains reason and reached evidence. Export
still adds only artifact-specific labels/anchors and safe source aliases.
The ANALYSIS@1 migration leaves DrawingBrief/package version, complete package
contents, source facts and independent statuses unchanged. The registered
analysis0.2 format does not relabel embedded source0.1 data. Final migration
acceptance depends on the saved shared schema/engine/CLI integration barrier.

Run the public command on the saved examples:

```sh
cargo run --locked -p uiblueprint-cli --bin uiblueprint -- imagegen-prompt \
  --brief fixtures/export/proposed-brief.json --purpose propose \
  --out /tmp/my-new-proposal-package --max-input-bytes 2000000 \
  --max-output-bytes 4000000 --max-components 256 --max-views 8 \
  --components-per-detail 12 --json
cargo run --locked -p uiblueprint-cli --bin uiblueprint -- imagegen-prompt \
  --brief fixtures/export/observed-brief.json --purpose document \
  --out /tmp/my-new-observed-package --max-input-bytes 2000000 \
  --max-output-bytes 4000000 --max-components 256 --max-views 8 \
  --components-per-detail 12
```

The numbers above are finite example parameters. The destination must not exist.
CLI max-output-bytes includes both all package files and the response on stdout.
The earlier `uiblueprint-export --example compile` remains a library demonstration;
the commands above exercise the actual public binary.

## Observed file to package

Use a saved canonical Snapshot Document or one observed ChannelResponse Document
as `observation.json`. The source can come directly from one selected Observe
response; no manual geometry reconstruction is required. For multi-channel NDJSON,
select one response explicitly. Failed responses or concatenated records reject.
The command below is local only and does not create a fresh observation:

```sh
uiblueprint imagegen-prompt --snapshot observation.json --metadata metadata.json \
  --out "$TMPDIR/my-new-observed-package" --max-input-bytes 2000000 \
  --max-output-bytes 4000000 --max-components 256 --max-views 8 \
  --components-per-detail 12 --json
```

Example `metadata.json` (caller-authored document annotations, no measured data):

```json
{
  "metadata": {
    "document_id": "UI-DOC", "revision": "1", "title": "Selected interface",
    "audience": "Developer", "language": "en", "date": "2026-10-08",
    "owner": "Document author", "retention": "Until this review is complete",
    "specification_refs": ["UIB.DRAWING@1.1"],
    "approval": {"status": "draft", "named_record": null},
    "page_format": "A3 proportions", "output_size": "3840 x 2160 output pixels"
  },
  "state": "unknown", "scope": "Selected source scope", "environment": "unknown",
  "safe_source_reference": "Observation selected by the document author",
  "not_depicted": ["Uncollected or hidden UI is not claimed"],
  "public_text_fields": []
}
```

All fields are required; use explicit unknown where runtime context is unavailable.
The metadata uses the existing Metadata type and named-approval validation.
Direct input forms one view `observed`, named by document title, with an explicit
note distinguishing caller annotations from source facts. It uses all returned
source nodes, canonical coverage, units, evidence and unknowns unchanged. No title,
date, environment, full coverage, current freshness or acceptance is inferred from
an ID. Public text requires the existing allowlist; [] keeps unreviewed text out.
The source and metadata files share the input cap, and the assembled brief must
also fit the compiler's existing serialization cap. The full six-file package,
versioned receipt, new-directory writer and output cap remain unchanged. No images
are created or copied. Direct saved pairs also support compare as documented below; detail and flow use --brief.

## Source and mode contract

`SourceInput::Observed` borrows the canonical schema's Snapshot representation;
the existing schema validator is reused. The resulting scene is a visualization
projection, not a second normalized graph or an action target. Every selected
source node is retained in input order (`N000`, `N001`, …), including unavailable
properties. Separate source identities, geometry kinds, shapes, transforms,
origins, units, evidence provenance/uncertainty, observation intervals and
coverage are preserved. Snapshot revision is explicit. Unknown source methods
and reasons can remain opaque safe references; no guessed explanation is added.

`SourceInput::Proposed` contains an export-owned ProposedLayout with explicit
requirements, components, local rectangular target geometry, dimensions and
arithmetic chains. It is never converted to a runtime Snapshot. The validator
rejects dangling/cyclic parents, duplicate IDs, inconsistent anchors, cross-space
or cross-unit dimensions, incorrect edge distances and inconsistent sums.
Arithmetic tolerance is explicit and distinct from measurement uncertainty.
Target quantities have requirement references and no fabricated runtime evidence.

Modes:

- `document` is the default; `explain` aliases it. It requires observations.
- `propose` requires explicit proposed layouts and their requirements.
- `detail` requires named component selections linked to the full general view.
- `compare` requires explicit before/after views. Incompatible or mixed bases
  require an explicit difference statement. Output keeps complete source views
  side by side; observed pairs use the existing Rust graph comparator described below.
  No heuristic identity matching or second analytical engine is introduced.
- `flow` requires explicit links. A link without Transition evidence stays
  unverified, including an unknown destination. Confirmed links require canonical
  Transition and Action records, matching before/after evidence, actual modality,
  confirmed delivery and succeeded verification. Action values are omitted from
  client output. Canonical synthetic evidence remains synthetic in its source
  label; tests do not establish live action delivery.

Each view has one named state, source, environment and scope. General sheets retain
all objects; explicit detail requests and density-driven groups add detail sheets.
Flow gets a separate F01 sheet. Responsive views must be supplied independently
with actual/proposed dimensions and environment annotations. No interpolation,
screenshot stretching, inferred breakpoint or composed Observation is performed.
Sheet density is a caller readability policy, not proof of generated legibility.

## Prompt, statuses and privacy

The full sequential A+B template is embedded in the crate. Every placeholder is
replaced with facts or explicit unknowns. The prompt includes inventories, source
bases, scene relationships, dimensions, sheets, state/action information, units,
style, forbidden changes, title block and review rules. Blue Engineering is a
presentation preset. It cannot alter measurements. Output is always schematic:
«Размеры по подписям; не измерять по изображению».

`source_kind`, `validation_status` and `approval_status` are separate. Compilation
sets `local_numeric_validation=checked`; image `validation_status` remains
`unverified`. Named externally supplied approval records are required for accepted
or superseded intent. The compiler neither creates that approval nor claims image
verification. E02/image QA, independent acceptance and optional generation remain
separate work.

Metadata, proposal labels, requirements, flow descriptions and public text field
selection are explicit caller-reviewed public annotations. Arbitrary UI values
are never approval or instructions. Snapshot text is excluded unless its field is
explicitly selected and public; value fields stay redacted. Source declarations
and extension text are excluded. Sensitive known properties are rejected by the
canonical validator. Private source IDs and clock domains become local aliases;
paths/URLs and unresolved template tokens are rejected in public annotations and
redacted from permitted source text. This defense is not universal secret
recognition: callers must classify sensitive material before constructing input.

No pixel bytes/references are accepted or auto-copied in this candidate. Captures
and `payload_ref` are excluded, and the package reference list is explicitly empty.
Permitted clean-image attachment remains a separate caller action; absence of
pixel-redaction proof cannot cause automatic inclusion. No embedded file/URL is
opened. The raw retained observations are not copied into Git.

## Examples and checks

[Proposed brief](../../fixtures/export/proposed-brief.json) is the exact
[DRAWING-EXAMPLE@1](../specs/reference/drawing-example.md) target layout, not a
PlayPhrase.me design. The 1200×800 surface, ten rectangles, field gap 8, column gap
24 and both authored chains are retained; radius remains unknown.

[Observed brief](../../fixtures/export/observed-brief.json) is a deliberately
pseudonymized canonical derivative of the saved F01 D05 `overlay-on` channel.
Source checkpoint `fcf48be57c31d3e3cfa48f38fa1b16d1d6af9357`; original channel SHA256
`11e2a77cf6460636050e90ed0f6f63bcf3412c062bb3eb917d677be035858180`.
All 32 source nodes, 17 relations, 160 requested properties and numeric facts are
retained. Opaque identifiers are consistently replaced; no geometry is inferred
from an image. Coverage remains partial, consistency unknown and unknown_count
unknown. It is historical observation of our controlled F01 runtime, not a new
collection, full application view or real-site acceptance. Method/namespace
strings in this safe input are known fixture provenance, not arbitrary paths.

The committed `observed-package` and `proposed-package` directories are immutable
historical E01 compiler outputs. They are not refreshed in place. In particular,
the historical observed package marks 32 extents unknown with unstable_state.
Current compilation of the same unchanged observed brief produces the 32 known
widths/heights of its 16 explicitly reported DOM rectangles, including
97.296875×32 css_px and the known zero width of N024. This follows accepted
known-anchor semantics (`256f2a2`) and the distinction between unknown consistency
and affirmative instability (`2491dec`); partial scope does not establish missing
geometry for a named known anchor. Source coverage stays partial and consistency
stays unknown. No new observation, complete scope or image validation is claimed.

The current package regression retains byte comparison of the five unchanged files
for both examples. Its independent literal extent inventory updates only expected
numeric values and their unknown_reason fields in memory, including the exact
embedded block in drawing-brief. The prompt tables are decoded independently and
compared with the historical component inventories and those same literal dimensions.
Evidence, anchors, privacy, source facts and statuses remain preserved. A separate synthetic unstable
variant verifies all 32 dimensions remain unknown with reached evidence. Historical
files are never overwritten, and expected values are not captured from candidate
compiler output. Run the existing commands above into a new destination for a
current example package; no second persistent example tree is needed.

Scoped verification: `cargo check --locked -p uiblueprint-export --all-targets`,
`cargo fmt -p uiblueprint-export -- --check`,
`cargo clippy --locked -p uiblueprint-export --all-targets -- -D warnings`, and
`cargo test --locked -p uiblueprint-export`. Tests cover template completeness,
example arithmetic, full observed scope, unknowns, statuses, privacy, mode gates,
flow attribution, references, limits and refusal to overwrite a destination.
Shared-owner review defects remain protected; green export tests do not accept
them or establish E02/P6/P7 product acceptance.

## Two saved observations to compare

[CLI-EXPORT@2](../specs/product/cli-export.md) connects two Snapshot Documents or
explicitly selected observed ChannelResponse Documents:

```sh
uiblueprint imagegen-prompt --before before.json --after after.json \
  --metadata comparison-metadata.json --purpose compare --out "$TMPDIR/new-comparison" \
  --max-input-bytes 2000000 --max-output-bytes 4000000 --max-components 256 \
  --max-views 8 --components-per-detail 12 --json
```

Pair metadata is `{metadata, before, after, different_basis, geometry_space}`.
`metadata` is the full document Metadata above. Each side requires title, state,
scope, environment, safe_source_reference, not_depicted and public_text_fields.
These are reviewed caller annotations, never replacement source facts.
`different_basis` and `geometry_space` are optional nullable fields. Space selects
an exact unambiguous source ID; it cannot supply geometry, transforms or Evidence.
Both sides must contain the same full selected Space definition. Original inputs
remain unchanged. Missing mappings produce typed unknown, never guessed values.

Existing `--brief` observed comparisons receive the same engine result; each
ComparisonRequest can also specify geometry_space. The package keeps all source
views and supplements scene/prompt/brief with `comparison_results`: engine scope,
status, aliased source contexts, entries with source-array indices and public
before/after facts, independent content_changed/evidence_changed, geometry and
limitations. The selected Space ID is aliased in client output. Changes to private
or unreviewed content retain flags with redacted facts; no raw values are added.
Children, node metadata, relations, mappings and five focus axes participate.
Surface records, captures and Snapshot envelope are retained by the ordinary
projection where permitted, but are not standalone compared domains. Empty entries
mean only no recorded differences in the named scope, never complete UI equality.
Absence is not deletion. Source coverage remains separate from result completeness.

An incompatible pair without different_basis refuses export_incompatible_views/2.
With that note it retains both views and status=incompatible_context without
matches. Proposal/mixed pairs retain their earlier side-by-side mode and explicit
status=different_source_bases. Receipts report engine_recorded_graph only if all
pairs were compared; otherwise partially_compared or not_compared. No unresolved_g02.
Comparison packages/receipts use0.2.0; no-comparison exports retain0.1.0 and their
previous contents. All six files plus stdout share the output cap; incomplete
comparison output is refused before directory creation. No model or image is run.

Runnable synthetic example from repository root (`UIBLUEPRINT_BIN` may select the
built binary). It uses independent GOLDEN01 expected false→true, not live evidence:

```sh
python3 - <<'PYEXAMPLE'
import json, os, pathlib, subprocess, tempfile
root = pathlib.Path('.')
chain = json.loads((root / 'fixtures/golden/GOLDEN01.json').read_text())['artifact']['data']
metadata = json.loads((root / 'fixtures/export/observed-brief.json').read_text())['metadata']
metadata['title'] = 'Synthetic checkbox comparison'
side = dict(title='Synthetic checkbox', state='Recorded fixture state', scope='Fixture form',
            environment='Synthetic records; no live collection',
            safe_source_reference='GOLDEN01 synthetic pair',
            not_depicted=['Geometry and pixels not collected'], public_text_fields=[])
with tempfile.TemporaryDirectory(prefix='uib-compare-example-') as directory:
    work = pathlib.Path(directory)
    for name in ('before', 'after'):
        (work / (name + '.json')).write_text(json.dumps(dict(schema_version='0.1.0',
            artifact=dict(kind='snapshot', data=chain[name]))))
    (work / 'metadata.json').write_text(json.dumps(dict(metadata=metadata, before=side,
        after=side, different_basis=None, geometry_space=None)))
    run = subprocess.run([os.environ.get('UIBLUEPRINT_BIN', 'uiblueprint'), 'imagegen-prompt',
        '--before', str(work / 'before.json'), '--after', str(work / 'after.json'),
        '--metadata', str(work / 'metadata.json'), '--out', str(work / 'package'),
        '--max-input-bytes', '2000000', '--max-output-bytes', '4000000',
        '--max-components', '256', '--max-views', '8', '--components-per-detail', '12',
        '--json'], check=True, capture_output=True, text=True)
    scene = json.loads((work / 'package/scene.json').read_text())
    changes = [e for e in scene['comparison_results'][0]['entries'] if e['content_changed']]
    assert len(changes) == 1 and changes[0]['field'] == 'checked'
    assert changes[0]['before']['state']['value']['value'] is False
    assert changes[0]['after']['state']['value']['value'] is True
    assert len(list((work / 'package').iterdir())) == 6
    print('Six-file synthetic compare passed: checked false -> true')
assert not pathlib.Path(directory).exists()
PYEXAMPLE
```

This local example removes only its generated non-image input/package files.
Independent privacy/input acceptance and any generated-image validation remain
separate. E03 author proof is recorded in the [receipt](../plans/ui-blueprint/receipts/E03-observed-compare.md).

## Real form → ImageGen verification (E04)

The [E04 receipt](../plans/ui-blueprint/receipts/E04-real-form-imagegen.md)
records Director and Settings from the real local PlayPhrase.me website. Both
six-file packages passed local compilation and preserved the imported DOM sizes.
**Direct delivery of either unmodified prompt to built-in ImageGen failed:** the
service accepted at most32,000 characters; these prompts contained123,391 and
529,860 characters. `package_written` does not establish model compatibility.
No product code or schema was changed by this experiment.

Reproduce the bounded workflow:

1. Open a task-owned local browser tab. Director: Clip Search → Director → draft
   `spiel`, without Enter or selecting a suggestion. Settings: Search → Settings,
   without changing a select or switch. Restore draft/close only owned resources.
2. Collect the selected real DOM/CSSOM structure, public text, state and rectangles
   with source intervals/document identity. Prefer canonical Observe. The E04 IAB
   surface lacked `Target.getTargetInfo` and an exposed page WebSocket endpoint,
   so E04 used an explicitly attributed one-off browser-reference Snapshot carrier.
   This is an imported-reference export experiment, not live collector acceptance.
3. Capture an independent lossless PNG with `Page.captureScreenshot(format=png,
   fromSurface=true)`; retain its original. Check actual browser DPR, CSS viewport,
   CDP physical viewport and PNG IHDR dimensions. Crop without resizing from that
   original and inspect the form before attaching it. CUA/JPEG observations do not
   substitute for this reference. All images stay in system temp without deletion.
4. Supply reviewed metadata and public text allowlist, then run the actual CLI:

   ```sh
   target/debug/uiblueprint imagegen-prompt \
     --snapshot "$E04_CASE/snapshot.json" --metadata "$E04_CASE/metadata.json" \
     --out "$E04_CASE/new-package" --purpose document \
     --max-input-bytes 2000000 --max-output-bytes 4000000 \
     --max-components 256 --max-views 8 --components-per-detail 64 --json
   ```

   `E04_CASE` denotes an explicitly selected system-temp case directory; the output
   must not exist. Bounds are experiment parameters, not product defaults. E04's
   exact filenames/arguments are retained in each `*-command.json` in the receipt.
   Keep all six files, compare every emitted extent with the source, and inspect
   privacy, coverage, unknowns and independent statuses before model submission.
5. First submit `prompt.txt` verbatim with the clean source reference. Record an
   input refusal as a failed direct export path. Do not silently shorten it or
   remove components. E04's separately saved second-attempt transport variants
   deduplicated repeated data; Settings used explicit compact tables. Those are
   caller workarounds, not outputs of an improved product compiler.
6. Compare the generated image with both reference and machine package: complete
   controls/text/state, exact dimensions/units/anchors/IDs, coverage, title block,
   unknowns, readability and actual output size. Preserve failures and exact prompt
   deltas. A visually attractive image with missing controls stays unverified/draft.

The package's document-level `max-output-bytes` is not a model prompt-character
budget. The E04-era `compile.rs::prompt` repeated full component JSON in inventory and
state sections and embedded verbose dimensions; it also passed the general sheet
through `detail_views`. E05 repairs these findings as recorded below.
Do not replace real evidence with synthetic geometry or relabel a transport
workaround as a successful unchanged CLI→ImageGen integration.


## Literal real-form prompts after E05

The [E05 receipt](../plans/ui-blueprint/receipts/E05-usable-imagegen-prompts.md)
records the same unchanged Director and Settings inputs on an attributed current
build. Prompt formatting now emits one component/state inventory and one dimension
inventory. Each is a text table with explicit nested field paths, common values
that apply to every row, and ordered row values. All facts remain in the prompt;
`absent` is distinct from JSON null, false, empty text and empty arrays. Common
values are compared by exact JSON spelling, preserving signed zero. This is local
presentation of already sanitized export records, not a new machine format, model
API, source projection, size flag or implicit data-pruning policy.

Other prompt JSON is compact. The source context includes the existing safe view,
snapshot/revision and Surface records. State instructions refer back to the complete
component inventory. Detail instructions list only actual detail sheet IDs; an empty
list means no details, and the full sheet plan remains embedded in the prompt.
The five other package files and all existing modes/versions remain unchanged.

Director shrank from123,391 to18,653 characters; Settings from529,860 to29,793.
Both literal CLI prompts were submitted unchanged, without a wrapper or manual
summary, to built-in ImageGen with the same inspected lossless reference PNGs.
Both generated images. All58 components and116 dimensions reconstruct exactly;
all116 extents also match independent raw CSSOM numbers bit-for-bit.

Model acceptance is distinct from image acceptance. Director contains an extra
header row and misbound component leaders; Settings omits the component/dimension
inventories. Both outputs are1672×941 despite requested3840×2160 and introduce dark
fills/gradients. Both stay unverified/draft. The saved images and detailed comparison
are in the receipt; no CAD accuracy, live acquisition, human approval or marketing
acceptance follows. These two successful submissions do not establish a universal
model input limit or guarantee that every larger scope fits; the existing explicit
package byte bound still refuses overflow without silently deleting facts.
