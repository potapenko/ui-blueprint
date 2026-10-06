# Local CLI and canonical analysis

Local commands use the existing engine and registered [ANALYSIS@1](../specs/product/analysis.md), reusing core0.1 source records. No model, live collection or reference IO. Export remains the separate unchanged command below.

## Syntax and inputs

```text
uiblueprint measure --snapshot S --query Q --space ID --max-input-bytes N --max-output-bytes N [--evaluation E] [--json]
uiblueprint measure --snapshot S --expectation X --space ID --max-input-bytes N --max-output-bytes N [--evaluation E] [--json]
uiblueprint check --snapshot S --expectation X --space ID --max-input-bytes N --max-output-bytes N [--evaluation E] [--json --result-version 0.2.0]
uiblueprint --help
```

S/X are strict core0.1 Snapshot/Expectation Documents; Q/E are strict analysis0.2 GeometryQuery/EvaluationInput Documents. Measure accepts exactly one query/expectation; check requires expectation and rejects query. Duplicate/unknown/incompatible flags reject. Compatibility measure extracts factual fields from the actual expectation, never a normative placeholder or pass/fail.
All named regular files share ONE aggregate input byte budget before parsing. No budget/tolerance/space defaults. Only those files are read: no stdin, URLs or embedded references. Source IDs, generations, coverage, timestamps and freshness stay unchanged; saved analysis is not a fresh UI read.
SPACE_ID resolves unambiguously among query anchors, existing geometry/baselines/capture transforms and supplied transform endpoints. Same ID with different Space definitions rejects. E must select that complete Space and bind Snapshot ID/revision/full Context and existing evidence. Without E: source binding, selected Space, no extra transforms/conditions. Missing facts stay unknown, not guessed.

## Output and versions

Compact output reports saved-data attribution, snapshot/target/generation/scope, coverage/fields, byte budgets, original Observations, factual query/anchors, selected Space/conditions, quantity/details/evidence or unavailable reason. Actual Expectation/source/tolerance appear only when supplied. Query mode invents no normative fields. Strings are escaped.
Measure JSON emits canonical analysis0.2 MeasurementCase. Check JSON defaults to core0.1 FindingCase; explicit result-version0.2.0 emits GeometryCheckCase with full evaluation, measurement and expectation. Explicit0.1.0 keeps legacy check. Result-version requires JSON; measure accepts only0.2.0. Unknown has no fake value; known zero/empty intersection stays known.
Legacy0.1 refuses extra transforms, any condition record or converted selected Space: unsupported_result_version/5, never silent downgrade. Empty E restating binding/same Space remains legal. Compact/0.2 support full registered inputs; no old core validator is weakened.
Typed validation and bounded encoding precede stdout; output budget includes newline. No partial stdout on validation/encoding/oversize failure. Analysis output passes the actual canonical decoder and exact round-trip equality; fidelity failure is analysis_roundtrip_mismatch/1, never an epsilon adjustment. OS write failure can leave a partial write and returns1. Diagnostics never enter machine stdout.
Schema valid/0 means declaration-valid, not recomputed or observed truth. CLI always computes through engine. Imported analysis MUST pass engine::verify_analysis_result before reuse as a computed result; all semantic fields are recomputed, except an assigned Finding.id may differ.

## Concrete synthetic example

Prepare the canonical source from the supplied fixture in a unique temporary directory; do not reuse its declared result as the calculation:

```sh
python3 - <<'PY'
import json, pathlib, tempfile
case = json.load(open('fixtures/analysis/measurement-gap.json'))['artifact']['data']
folder = pathlib.Path(tempfile.mkdtemp(prefix='uib-analysis-example-'))
source = {'schema_version':'0.1.0','artifact':{'kind':'snapshot','data':case['snapshot']}}
path = folder / 'snapshot.json'
path.write_text(json.dumps(source))
print(path)
PY
cargo run --locked -p uiblueprint-cli -- measure --snapshot PRINTED_PATH --query fixtures/analysis/query-gap.json --evaluation fixtures/analysis/evaluation-local.json --space local-form --max-input-bytes 131072 --max-output-bytes 131072 --json
```

Replace PRINTED_PATH with the printed path. Expected fact: gap8 css_px in local-form, analysis0.2/source Context0.1, no normative pass/fail. Remove only that temporary directory after use. Example byte bounds are not production defaults or process-memory promises.

## Exits and scope

| Exit | Meaning |
| --- | --- |
| 0 | Check pass / measure known / help |
| 1 | IO/internal, including serialization fidelity or stdout failure |
| 2 | Invalid argument/document/binding/geometry/space or byte limit |
| 3 | Measured check fail |
| 4 | Check/measure unknown, including missing/mismatched applicability |
| 5 | Unsupported command/rule/version or unrepresentable legacy result |

Bounded diagnostics: invalid_arguments, invalid_input, invalid_input_file, input_limit, output_limit, io_error, invalid_geometry, invalid_analysis, unknown_space, ambiguous_space, unsupported_command, unsupported_rule, unsupported_result_version, analysis_roundtrip_mismatch. No raw path/argument/serde/private payload error is echoed. Observe/action/plugin commands are absent; schema-validator exits remain0/2/1.
The sole engine uses schema-owned results. Cache/replay, bridges/transport, core schema/fixtures and export package format stay protected. Live condition acquisition, transform discovery, generic diff, cache ID resolution and D05 peak enforcement remain separate. [Analysis receipt](../plans/ui-blueprint/receipts/L01-analysis-engine-cli.md) records current proof; [initial L01 receipt](../plans/ui-blueprint/receipts/L01.md) retains the old boundary.

## E01 public imagegen-prompt command

Finite choices below are recorded before E01-cli source edits under the approved
P6 packet. This command uses the existing export compiler; it adds no graph,
measurement, schema validator or package writer.

```text
uiblueprint imagegen-prompt --brief FILE --out NEW_DIRECTORY --max-input-bytes N --max-output-bytes N --max-components N --max-views N --components-per-detail N [--purpose document|explain|propose|detail|flow|compare] [--profile blue-engineering] [--json]
```

`--brief` is the export-owned DrawingBrief JSON, including canonical Snapshot
records inside observed views or an explicit ProposedLayout inside proposed
views. A bare Snapshot is invalid brief input (2). `--snapshot FILE` without
DrawingBrief metadata returns `export_metadata_required` (2): document identity, safe source labels, state,
environment, retention, approval, requirements and scope cannot be guessed.
Stored Snapshot-ID resolution awaits K01 and is not emulated by a path lookup.

The optional purpose overrides the input brief's purpose before compiler
validation. Without it, the brief's explicit purpose applies; a missing JSON
purpose defaults to document. Thus propose must be explicit either in the brief
or the flag. `explain` aliases document. Detail, flow and compare retain exactly
the compiler's data/evidence gates and current G02 attribution limitation.
The only implemented profile is blue-engineering; another profile returns
`unsupported_profile` (5). Unknown/duplicate flags and invalid positive limits
return `invalid_arguments` (2). All numeric limits are required, with no hidden
production defaults.

Input is one explicitly named local regular file, read through the existing
bounded reader. No embedded reference, path, URL, source ID or payload_ref is
opened. Input bytes and the compiler's serialized in-memory input validation
must both fit max-input-bytes. The compiler's own node/view/density limits apply.
`max-output-bytes` bounds the aggregate six package files **plus stdout including
its final newline**. The whole package and success response are prepared and
checked before creating the destination. Oversize input/output produces no
partial stdout and no destination. These are caller bounds, not a process-memory
cap or D05 calibration.

The existing package writer creates only a new directory and refuses existing
exports, symlinks or baselines. Its six files retain the full A+B prompt, scene,
dimensions and sheets. No images/references are added automatically. Compact
stdout reports mode, view/component counts, coverage per view, independent
source/validation/approval statuses, package bytes and unresolved comparison
attribution. It contains no input/output path or raw collector identifier.

`--json` emits one export-owned versioned result object, followed by a newline:
`result_version="0.1.0"`, `command="imagegen-prompt"`,
`status="package_written"`, `purpose`, `views` (safe view ID, source_kind,
coverage status or proposed, omitted_count, unknown_count, component count),
`package_bytes`, six fixed `files`, `local_numeric_validation="checked"`,
`validation_status="unverified"`, `approval_status`, `generated_image=false`,
`references_count=0`, and `comparison_attribution` (not_requested or
unresolved_g02). This is a package receipt, not a replacement normalized schema.
Unknown quantities stay unknown in the package; successful compilation does not
make measurements pass, generate/verify an image, or approve the source.

Export exits:0 means all package files were written and the success result was
written to stdout;1 means IO/internal failure;2 means invalid/missing/oversize/
sensitive input or existing destination;5 means unsupported profile. Other CLI
commands keep their existing exits and behavior. Compiler failures map to bounded
constant codes: `export_invalid_input`, `export_invalid_source`,
`export_invalid_reference`, `export_invalid_geometry`, `export_invalid_chain`,
`export_incompatible_views`, `export_private_content`,
`export_approval_record_required`, `export_destination_exists`, `input_limit`,
`output_limit` or `io_error`. Parse/missing top-level metadata uses
`export_metadata_required`; malformed JSON uses `export_invalid_input`.
No raw serde, filesystem, payload or argument errors are printed. A write failure
is never reported as success. If stdout itself fails after files were written,
exit1 reports IO failure and the completed package remains at the explicit
location; retrying cannot overwrite it. No automatic retry is performed.
