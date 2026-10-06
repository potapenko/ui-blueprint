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
No engine/schema wire change is requested. The ordinary live observe → stored
Snapshot ID → export path still awaits the session/storage and adapter owners;
local brief export does not claim that end-to-end capability.
The existing G01 `measure` API computes observed rect width/height; export adds
only artifact-specific labels and anchors. Shared MeasurementResult/result-space
cutover must revalidate this consumer; it is not implemented here.

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
  side by side; it invents no identity matches or generic diff engine.
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

Scoped verification: `cargo check --locked -p uiblueprint-export --all-targets`,
`cargo fmt -p uiblueprint-export -- --check`,
`cargo clippy --locked -p uiblueprint-export --all-targets -- -D warnings`, and
`cargo test --locked -p uiblueprint-export`. Tests cover template completeness,
example arithmetic, full observed scope, unknowns, statuses, privacy, mode gates,
flow attribution, references, limits and refusal to overwrite a destination.
Shared-owner review defects remain protected; green export tests do not accept
them or establish E02/P6/P7 product acceptance.
