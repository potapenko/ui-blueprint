# Local CLI and canonical analysis

Local analysis commands use the existing engine and registered [ANALYSIS@2](../specs/product/analysis.md), reusing core0.1 source records. They collect no live data, invoke no model and load no embedded references. Explicit Observe is separate below; export remains unchanged.

## Guarded live observation

```text
cargo build --locked -p uiblueprint-cli --features macos
cargo build --locked -p uiblueprint-host --bin session-worker
uiblueprint observe --connection CONNECTION.json --request REQUEST.json --worker /absolute/path/session-worker --max-input-bytes 65536 --max-output-bytes 65536
```

For Web select CLI `--features web` and host `--features web`; combined callers
use CLI `--features macos,web`. Default CLI features stay empty and saved-data
commands never launch workers. Live host is macOS-only; Web does not invoke Swift.
Use the matching freshly built private host/worker pair, not an arbitrary older
worker; installable placement/packaging remains I01. There is no automatic install.

[CLI@5 OBSERVE](../specs/product/cli.md) defines connection_version1.0.0 and exits.
Connection is an explicit trusted operator JSON file containing target Identity,
canonical session SessionDescriptor, positive attach_deadline_ms, all16 explicit
host_limits fields and provider. It is never loaded from UI or observation output.
Provider `{backend:"native_fixture",helper_executable:"/absolute/path/helper",
configuration:"opaque UTF-8 configuration",channels:MASK}` reuses current4032-byte
trusted helper configuration and exact own-fixture identity; arbitrary application
PID/title discovery is not supported. Provider `{backend:"web",setup:WebSetup,
selection:WebSelection}` reuses the existing strict host config types for an already
established numeric-loopback CDP target/document. No browser/app bootstrap occurs.
Field shapes/caps are [host Web config](../../crates/host/src/web_config.rs) and
[host limits](../../crates/host/src/limits.rs); each limit is supplied explicitly.
The connection parser rejects unknown/duplicate fields and invalid versions;
selector/config records are not another canonical graph or private memory layout.

REQUEST.json is an existing core0.1 Request Document with operation Observe.
Target/session/plugin/surfaces/scope and channel authority must match connection.
Only clock_domain is replaced in the owned request after actual Attached; the
file, identities, fields, limits and freshness policy stay unchanged. Aggregate
input cap covers connection plus Request bytes. Worker/helper executable paths
are explicit and validated; no configuration secrets enter argv or diagnostics.

Output is committed canonical ChannelResponse NDJSON, channel0/1/2 order, with one
newline per unchanged frame. No combined Snapshot or response graph parse occurs
in parent. Reserve newline budget before dispatch; reject an oversized line before
any part of that line. Earlier ACKed channels survive later worker/probe/capture
failure. Complete canonical Failed and declared partial/unknown source coverage
are carried by fixed ACKed metadata, not inferred from delivery alone.
Leases survive bounded shutdown and publication. Cleanup is attempted on all
paths and must confirm worker/helper reap; user target app/browser is never closed.

Exit0 means all requested channels delivered with no incomplete metadata;4 means
unavailable/partial/cancel/timeout. Invalid/limit2; IO/worker/unconfirmed cleanup1;
unsupported backend/platform/feature5. Fixed error codes use stderr, never UI/config
payload. A partial NDJSON result can accompany a nonzero exit; consumers must use
both status and canonical records. Cleanup or physical write failure returns1.
Actual arbitrary-app support, target discovery, persistent session CLI, distribution
and positive platform qualification remain separate; no live input/action is added.

## Saved-data syntax and inputs

```text
uiblueprint measure --snapshot S --query Q --space ID --max-input-bytes N --max-output-bytes N [--evaluation E] [--json]
uiblueprint measure --snapshot S --expectation X --space ID --max-input-bytes N --max-output-bytes N [--evaluation E] [--json]
uiblueprint check --snapshot S --expectation X --space ID --max-input-bytes N --max-output-bytes N [--evaluation E] [--json --result-version 0.2.0]
uiblueprint --help
```

S accepts strict core0.1 Snapshot OR observed ChannelResponse containing it; X is core0.1 Expectation, Q/E are analysis0.2 GeometryQuery/EvaluationInput. A single Observe line can go directly to measure/check as to inspect/diff; preserve the embedded Snapshot/evidence. Failed/no-Snapshot responses reject2. Measure accepts exactly one query/expectation; check requires expectation and rejects query. Duplicate/unknown/incompatible flags reject. Compatibility measure extracts factual fields from the actual expectation, never a normative placeholder or pass/fail.
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

## Inspection of saved observations

```text
uiblueprint inspect --snapshot FILE --ref '{"namespace":"web.dom","key":"33"}' --view interaction|design --max-input-bytes N --max-output-bytes N [--json]
```

The selector is the existing strict canonical SourceKey JSON object, not a
namespace:key grammar or BackendRef. Its UTF-8 bytes and the named input file
share one aggregate input budget. View and positive byte limits are required.
Input accepts a core0.1 Snapshot Document or an observed ChannelResponse containing
its unchanged Snapshot; failed channels and other artifacts reject as invalid input.
Selection uses full namespace/key equality, never labels, geometry or a live lookup.

Compact output explicitly says saved observation/no live revalidation. It retains
source Context, original projection/coverage/freshness/consistency/Observation,
native role, selected properties with availability and Evidence, and explicit
incident relations. Missing/unrequested fields stay not_requested; unknown,
unsupported, redacted, false and empty remain distinct. Each geometry keeps its
frame kind, Space and source. Snapshot focus is separate from node Focused.
Interaction keeps seed semantic facts first; design orders geometry first and
prints full recorded properties/bounds of exact members from reported component
mappings containing the seed. Multiple groups remain separate, shared parts appear
once. Compact component_selection counts/exposure stay separate from source coverage;
missing declared parts is not_exposed, never inferred from names/boxes or guessed
combine/ignore causes. Attributed group relations remain context, not new membership.
Existing JSON1.0.0 is unchanged; byte overflow still produces no partial result. Neither
view creates inner parts, changes the graph, promotes AX bounds to layout bounds,
or creates action refs. Strings are escaped as data. No external references load.

Found node returns0 even with partial/unknown fields; missing exact SourceKey gives
target_unresolved/4. Invalid/limit2, IO1 and unsupported mode5 remain distinct.
The full result is bounded before stdout, including newline: overflow emits no
partial result. `--json` emits the [CLI@5 INSPECT](../specs/product/cli.md) envelope:
`output_version="1.0.0"`, `kind="inspection"`, `source="saved"`,
`live_revalidated=false`, canonical `selector` and `requested_view`, and the full
unchanged canonical `snapshot`. One JSON object plus newline; borrowed canonical
fields serialize directly through the bounded writer. No second graph or inferred
action ref. Version1.0.0 belongs to this CLI envelope, not core0.1 or analysis0.2;
the product does not import it. Existing measure/check/export paths are unchanged.

## Recorded node/property differences

```text
uiblueprint diff --before BEFORE.json --after AFTER.json --max-input-bytes 65536 --max-output-bytes 65536 --max-entries 100 [--json]
```

[CLI-DIFF@2](../specs/product/cli-diff.md) uses the saved engine recorded comparison.
Each file is a canonical Snapshot or observed ChannelResponse; one aggregate input
budget covers both. Exact SourceKey/Field and matching session/schema/plugin/Target/
Surface generations, scope/projection/fields are required. Different environment
revisions remain as two original records/Spaces, without conversion or common
transform. No source ID is rewritten to force a match. Invalid records refuse.

Compact identifies saved/not revalidated source and node_presence_and_properties
scope, original coverage/Observations and before/after property availability. JSON
envelope1.0.0 retains both complete unchanged canonical Snapshots, with entries that
reference exact keys/fields and presence/content_changed/evidence_changed flags.
Missing-in-after is record absence, never deleted or generated Removal/Delta.
Source-only changes remain distinct from content/value changes; original Evidence
is accessible in the embedded records. Relations/focus/other graph metadata are
outside this comparison and are not claimed unchanged by an empty report.

Explicit max-entries0 returns no entries plus actual omitted count. Complete report
returns0 with or without differences, not a UI requirement pass/fail. Truncation4
retains source coverage independently; incompatible valid contexts give
context_mismatch/4 and empty stdout. Invalid/input/output limit2, IO/allocation1.
Output is fully bounded including final newline before stdout; no partial result
on overflow. Existing inspect/observe/measure/check/export behavior remains.
Real Web sized-before/after records retain different environment revisions and
report layout32×16→48×24 css_px with original Evidence, without restamping or an
inferred arithmetic displacement. Live changes/full graph comparison remain separate.

## Exits and scope

| Exit | Meaning |
| --- | --- |
| 0 | Check pass / measure known / help |
| 1 | IO/internal, including serialization fidelity or stdout failure |
| 2 | Invalid argument/document/binding/geometry/space or byte limit |
| 3 | Measured check fail |
| 4 | Check/measure unknown, including missing/mismatched applicability |
| 5 | Unsupported command/rule/version or unrepresentable legacy result |

Bounded diagnostics: invalid_arguments, invalid_input, invalid_input_file, input_limit, output_limit, io_error, invalid_geometry, invalid_analysis, unknown_space, ambiguous_space, unsupported_command, unsupported_rule, unsupported_result_version, analysis_roundtrip_mismatch. No raw path/argument/serde/private payload error is echoed. Local analysis does not itself observe/act; schema-validator exits remain0/2/1.
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
views. Alternatively, [CLI-EXPORT@1](../specs/product/cli-export.md) connects an
unchanged canonical observation without re-entering any measured geometry:

```text
uiblueprint imagegen-prompt --snapshot FILE --metadata FILE --out NEW_DIRECTORY --max-input-bytes N --max-output-bytes N --max-components N --max-views N --components-per-detail N [--purpose document|explain] [--profile blue-engineering] [--json]
```

The direct form loads one core0.1 Snapshot Document or observed ChannelResponse
through the existing strict snapshot loader. A failed response, wrong artifact
or multiple JSON records rejects `invalid_input` (2). Select one observed channel
line explicitly; no automatic merge or silent channel selection. `--brief` cannot
be combined with --snapshot/--metadata. Missing metadata returns
`export_metadata_required` (2); direct-input non-document purposes reject2.
The default direct mode is document. Source fact extraction is automatic; document
annotations remain caller-authored. Metadata's exact object/example is documented
in [export](export.md#observed-file-to-package). No implicit stored-ID resolver.

The optional purpose overrides the input brief's purpose before compiler
validation. Without it, the brief's explicit purpose applies; a missing JSON
purpose defaults to document. Thus propose must be explicit either in the brief
or the flag. `explain` aliases document. Detail, flow and compare retain exactly
the compiler's data/evidence gates and current G02 attribution limitation.
The only implemented profile is blue-engineering; another profile returns
`unsupported_profile` (5). Unknown/duplicate flags and invalid positive limits
return `invalid_arguments` (2). All numeric limits are required, with no hidden
production defaults.

Inputs are explicitly named local regular files, read through the existing
bounded reader: one brief, or Snapshot/ChannelResponse plus metadata sharing the
same aggregate input-byte limit. No embedded reference, path, URL, source ID or payload_ref is
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


## Saved relation neighbors

`uiblueprint neighbors --snapshot FILE --ref SOURCE_KEY_JSON --max-relations N
--max-input-bytes N --max-output-bytes N [--json]` selects one step of existing
incident relations through the accepted engine API. FILE is a canonical Snapshot
or observed ChannelResponse; --ref is the same strict SourceKey as inspect.
All flags are required once; max-relations accepts0, byte budgets remain positive.

See [CLI-NEIGHBORS@1](../specs/product/cli-neighbors.md) for exact JSON shape,
exits and bounds, and [scope example](scope.md#public-saved-data-neighbor-caller)
for a usable command. Selection cap/omitted/truncated are separate from unchanged
source coverage. Direction, counterpart Node and Evidence are preserved. Output
is saved context only; no live revalidation, inferred link or stable action ref.
Inspect remains unchanged; it still shows all incident relation keys in compact
and the original Snapshot in its version1.0.0 inspection envelope. Neighbors adds
explicit capped selection and counterpart data, rather than another graph owner.


## A04 single-step Web Focus and Type

Current caller status: **saved and Web-consumer-ready**. CLI source c15ffb3 plus
Core8493c14 passed the Web-feature build, four action binary preflight tests,
semantic-status/ACK/cleanup test and affected Clippy. Worker rule/binding/privacy
cases are separately attributed to Core's saved proof. The Web-owned actual public
Focus→Type run and grouped acceptance remain pending; CLI preflight is not delivery
proof. Matching CLI/worker products are retained for that immediate consumer.

[CLI-ACTIONS@2](../specs/product/cli-actions.md) adds explicit caller Expectation
transport to the existing action path. Build with the existing `web` feature.
One invocation still handles one step; no batch runner or implicit Focus is added.

```sh
uiblueprint action prepare --connection connection.json --snapshot observed.json \
  --request focus-prepare.json --expectation focus-expected.json \
  --worker /absolute/path/session-worker --max-input-bytes 131072 \
  --max-output-bytes 65536 --json
uiblueprint action execute --connection connection.json --plan focus-plan.json \
  --request focus-act.json --expectation focus-expected.json \
  --worker /absolute/path/session-worker --max-input-bytes 131072 \
  --max-output-bytes 65536 --json
```

Save the complete successful Prepare ActionCase as focus-plan.json; Act Request
must contain that exact Action, rather than the unresolved preparation seed.
The Expectation is a normal core0.1 Document with artifact.kind="expectation".
Focus uses Intent::Focus/InputModality::Semantic and a caller PropertyEquals rule
with field="focused", expected={type:"flag",value:true}, the exact action SourceKey
as its single target, matching authorized scope and explicit expected_from.
After verified Focus, a separate fresh observed source/Prepare/Execute Type step
uses Intent::Type/InputModality::Keyboard. Its Expectation is field="value" with
expected={type:"text",value:"the explicitly expected whole draft value"}. The
value is supplied by the caller; inserting "tail" does not imply a final value
of "tail", an applied setting, submitted form or business success. No secret
input/expected values belong in diagnostic documents or command-line arguments.

Both commands require --expectation FILE for Focus/Type; absent returns
expectation_required/2 before attach. Native action/Activate/other modalities stay
unsupported5. SetChecked/Setter without the flag keeps the original two-document
worker transport and kernel-owned Checked predicate. With the flag, all four
explicit input files share the existing aggregate byte budget. The canonical
source and Expectation bytes are passed unchanged in Tape order
[source, request, expectation]; only Request.clock_domain is rebound after Attach.
Parent never parses source/expectation graph bodies or grants rights from them.
The guarded worker validates kind/order/scope/target/field/privacy/current binding;
caller expected state is not invented to turn a result into success.

The actual target permit, shared Focus/Keyboard lane, fixed Frame/Commit/ACK
semantic metadata, post-Possible uncertainty, no-retry and owned cleanup are the
existing host/kernel owners. JSON/compact output and exits retain CLI-ACTIONS
semantics:0 verified source-state success,3 known mismatch,4 refusal/uncertainty,
1 IO/unconfirmed cleanup,2 pre-Possible invalid/limit,5 unsupported before dispatch.
Missing/malformed/mismatched expectation must never establish success; worker
refusal may retain a matching canonical Error document. UI/body text never enters
stderr. No changes to observe, geometry, diff, export, neighbors or SetChecked.

This is the public caller handoff for the combined A03/A04 integration. Core owns
saved/check-ready worker composition; Web owns the one actual isolated public
Focus→Type chain and its runtime proof. CLI preflight tests are not delivery proof,
and a source-state result does not close full B02/P5/P7 or Native capability.


## A05 narrow native-button activation

[CLI-ACTIONS@3](../specs/product/cli-actions.md) extends the same explicit
single-step command with `Intent::Activate {}` and `InputModality::Semantic`.
It requires --expectation FILE on Prepare and Execute, exactly like Focus/Type;
source, Request, Expectation order and all budgets/output/exits stay unchanged.
The parent transports that record without inspecting or inventing its result.

```sh
uiblueprint action prepare --connection connection.json --snapshot observed.json \
  --request activate-prepare.json --expectation result-expected.json \
  --worker /absolute/path/session-worker --max-input-bytes 131072 \
  --max-output-bytes 65536 --json
uiblueprint action execute --connection connection.json --plan activate-plan.json \
  --request activate-act.json --expectation result-expected.json \
  --worker /absolute/path/session-worker --max-input-bytes 131072 \
  --max-output-bytes 65536 --json
```

First supported port: actor is an observed native HTMLButtonElement; Expectation
is PropertyEquals(Value,Text) on one distinct independently bound public web.dom
result, in the same authorized Surface/scope. Result must be INPUT of type
text/search/url/tel or plain OUTPUT. Readonly/disabled results are readable;
required requested fields are enabled,value,input_kind. This is not textarea,
select, same-node result, generic arbitrary DOM or Native activation. Source/private/
stale/missing-result validation remains in the existing guarded worker/provider.

In our F01 fixture, selecting the observed option expects Value(Text("London"))
on the distinct draft node; a separate fresh Commit step expects it on the
applied output node. Selection is not Apply. Neither button label, click return
nor prior draft determines the expected outcome. An option may remove itself;
verification reads the separately held result without reacquiring either node by
label or coordinates. Missing/private/unavailable after-state cannot verify success.

Delivery is the provider's fixed isolated standard HTMLElement.click with
userGesture=false and untrusted script Semantic attribution. It does not prove
pointer/hardware input and does not invoke onclick directly or use a value setter,
Enter, synthetic dispatchEvent or a fallback backend. Existing real parent permit,
ACK/effect metadata, deadline/cleanup and post-Possible no-retry rules apply.
The explicit caller must freshly observe unexpected transition/disabled Commit
and stop with zero dependent Execute calls; no new scenario/batch runner exists.

A05 caller source a9ad665 and provider/host5605206 are saved. Their combined
baseline0661359 passed Web-feature build, four affected action binary tests,
status/ACK/cleanup test and affected Clippy; matching CLI/worker products are
retained for Web's one actual public selection→apply/unexpected-stop sequence.
That runtime consumer and grouped acceptance are pending here. Prior accepted
A03/A04 and geometry/export/neighbors remain protected; full B02/P5/P7 is not
inferred from this port, preflight checks or registration.
