# G02 actual saved Web measurement — qualified result

Authority: [finite packet](../packets/G02-live-data-measure.md), approved PLAN.UIB@1.
Only this receipt is written durably. Basis: registry11 → product → ANALYSIS@1
SCOPE/VERSION/CLI, TYPES/VALIDATION@1 and CLI/EXCHANGE/GEOMETRY/MODEL/PRIVACY/
IDENTITY/BOUNDARIES closure, D03@2, RUST.md/DEV.RUST@2. Restore, no contract delta.
Historical closed-session frames are not fresh observations or reusable live refs.

## Exact inputs and independent expectation

Original ACKed files under application state:
`/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/P2/W01-guarded-live/e5b9cc4a-9262-4ac8-a508-0636dd750521/`.

| Original | Bytes | SHA256 |
| --- | ---: | --- |
| sized-before.json |3084|aa19c4702e428b781b99fb10f4ec4602050aa5a5cbed06b3524a5959f1a57e7a|
| sized-after.json |3072|19aee76822668c6b993a4a11a4fb206beef43c48824a3f36cb39bf0151c555fc|

Hashes matched before and after use; original bytes were untouched. F01 authored
oracle [expected.json](../../../../fixtures/web/expected.json) B04 text_before/
text_after independently specifies32×16→48×24 css_px (literalε0.01).
Oracle SHA256 `501664fc3a3c0319b3a766d3d9b97c65ecdb905c636c27ae44c841704a0cb321`.

Snapshot extraction wrapped the complete ChannelResponse.result.data in an
existing core0.1 Snapshot Document in task-temp; parsed Snapshot equality was
asserted. No property/context/timestamp/freshness/consistency/projection changed.
Source IDs/revisions: web-snapshot:1:2/2 and web-snapshot:1:3/3; scope f01-sized,
node web.dom:33, session live-session-1. Exact target/Space ID
4819F611B754ED4F855AB26602122367; target generation
a9cae75a-0546-47fd-b32f-bcb11cc139ab; Surface generation
9A01B15CA81D69C01AC699E11B56311A. Space is viewport/css_px/top_left, local_only.
Environment revisions remain f01-800x600-dpr1-sized-before and -sized-after.

## Existing command and actual outcome before correction

Built only the existing target using
`cargo build --locked --offline -p uiblueprint-cli --bin uiblueprint` (exit0,2.67s).
Rust1.96.0/aarch64-apple-darwin; source tree was clean for these build inputs.
Saved source revision0590fa6e1871742a6933eb869b5db999742f9e45 matched consumed
Rust/manifests and export template.43 Rust/manifests were hashed before/after build;
the embedded export template was also checked against that saved revision.
Binary SHA256 `b487111b2cf84cdaca40dde43e6dd2a95611fce4a8a431c21cfdf2da07aa0bf5`.
Copied this verified binary to task-temp before invoking it.

For each PHASE=before/after and OP=width/height, exact command shape:

```text
TEMP/uiblueprint measure --snapshot TEMP/snapshot-PHASE.json --query TEMP/query-PHASE-OP.json --space 4819F611B754ED4F855AB26602122367 --max-input-bytes 65536 --max-output-bytes 65536 --json
```

Each factual analysis0.2 query used its exact scope, key and full Space; one
layout_bounds anchor, fraction0, x for width/y for height, length/css_px and
null applicability conditions. No Expectation, evaluation override or transform.
Every command had an enforced10s timeout; aggregate inputs3350–3364 bytes and
outputs4169–4184 bytes stayed below65536. Observed calls took0.015–0.025s.

| Case | Original known property / independent oracle | CLI engine result | Exit |
| --- | --- | --- | ---: |
| before width |32 css_px /32|unknown: unstable_state|4|
| before height |16 css_px /16|unknown: unstable_state|4|
| after width |48 css_px /48|unknown: unstable_state|4|
| after height |24 css_px /24|unknown: unstable_state|4|

These are four raw observed property values matching the oracle, NOT four known
engine measurements. Every emitted MeasurementCase retained the complete original
Snapshot and exact EvaluationInput binding/Space (equality checked). Unknown output
has no fabricated numeric result. Its contributing Evidence is web.dom:1:2 or
web.dom:1:3, namespace web.dom, reported, method cssom-getBoundingClientRect,
uncertainty null. Both original observations have consistency=unknown with reason
sequential-reads-not-atomic, source coverage partial and unchanged worker clock.

## Initial residuals before correction

Actual refusal owner: engine resolve::record_evidence returns UnstableState whenever
consistency != Stable. [resolve.rs](../../../../crates/engine/src/resolve.rs) SHA256
94e8fceb07fae9c12af551fd791d21d9f9405ef94e5204f4adedca4fadf2c812.
No source restamping, collector change or weaker engine rule was authorized here.
Known measurement demonstration therefore remains open despite matching raw bounds.

Imported-result recomputation was NOT executed. Public engine::verify_analysis_result
exists, and guarded host OperationClass::Verify calls it, but the existing CLI has
only measure/check argument routes (plus separate export); its JSON round-trip/
schema validation is not this verification. Existing verification tests consume
fixed fixtures, not these temporary result files. No existing standalone caller
for these files was found in the relevant CLI/engine/host routes. A narrowly
authorized invocation of that existing Rust API/host path is the remaining caller
dependency; this packet explicitly excludes adding code/wrappers. No schema-only
success, repeated CLI call or raw-value comparison is substituted for recomputation.

No browser, SDK, action, broad suite or shipping source mutation occurred. Only
owned temporary extraction/query/result/build-identity files and copied executable
were created outside Git/config; Core removes that task directory after accepted
handoff. Original retained Web evidence remains root-owned and untouched.

## Consistency guard investigation after the saved unknown result

Root authorized the direct engine condition correction if supported by current
norms, while excluding schema writes. MODEL.CONTENT separates availability from
stable/unstable/unknown consistency; EXCHANGE.CONTENT does not promise global
atomicity; ANALYSIS-VALIDATION.EVIDENCE states unstable source -> unstable_state.
No selected normative clause requires unknown consistency to become unstable.
Nearest existing analysis condition test explicitly uses Consistency::Unstable.

Candidate engine change is only resolve::record_evidence `!= Stable` ->
`== Unstable`, with a focused test in existing engine/tests/analysis.rs for known
width/height under stable/unknown, genuinely unstable refusal, source immutability,
unavailable properties and incomplete scope. The first focused run FAILED at
engine recomputation: InvalidContract(UnknownMeasurement). Exact second owner is
schema/src/analysis/results.rs::validate_sources, which independently requires
`!known || observation.consistency == Consistency::Stable` at line83. It prevents
the new known result from passing the existing canonical result validator.

This is a concrete shared-source dependency, not successful verification. No
schema write, weakened test, restamped source or new caller was made. Candidate
engine/test changes are not ready to checkpoint until that separate owner is
reconciled; four affected CLI calls have not been repeated against this candidate.

## Reconciled direct correction and four known measurements

Root's amended packet grants the exact schema owner and nearest test in addition
to engine/test/receipt. Read protected G01 packet/receipt, current analysis clauses
and nearest tests. No normative requirement makes Unknown consistency Unstable:
MODEL.CONTENT makes availability and consistency independent; EXCHANGE.CONTENT
preserves timing/uncertainty without promising global atomicity; GEOMETRY.CONTENT
refuses actual transient state, and ANALYSIS-VALIDATION.EVIDENCE says unstable
source -> unstable_state. Accepted nearest tests exercise Unstable, missing data,
binding, shapes and scope; they do not establish a contrary Unknown prohibition.
The two source expressions conflated these states: an implementation defect.

Final production change is only engine `!= Stable` -> `== Unstable` and schema
`!known || == Stable` -> `!known || != Unstable`. It retains uncertainty, not an
assertion of stable or coherent global UI. No flags, parameters, versions, APIs,
wrappers, capture modifications or new arithmetic. Source properties/Evidence,
Context, unavailable/redacted values, incomplete scope and binding guards remain.
Exact5 changed files: engine/src/resolve.rs, engine/tests/analysis.rs,
schema/src/analysis/results.rs, schema/tests/analysis_contract.rs (all under crates),
and this receipt. The older four-unknown evidence above remains historical fact.

Focused checks passed, all with cargo --locked --offline and exact test names:

- engine/test analysis: unknown_consistency_preserves_known_dimensions_and_unstable_refusal.
  Width/height stable and unknown retain authored30/10; Unstable refuses; source
  and Evidence are unchanged; unavailable properties and incomplete scope refuse.
  Existing public verify_analysis_result recomputes these regression declarations.
- schema/test analysis_contract:
  known_dimensions_keep_unknown_consistency_but_reject_unstable_or_unavailable_sources.
- Existing engine test conditions_are_observed_bound_and_never_inferred.
- Existing schema tests known_declarations_cannot_claim_unsupported_source_shapes_or_baseline_x
  and unbound_evaluation_does_not_pretend_to_resolve_its_source.

The new negative fixtures initially omitted mandatory consistency_reason when
assigning Unknown; fixed those fixture declarations, with no validator relaxation.
All five named cases then passed. No full suite or new recomputation caller.

Rebuilt the same CLI command (exit0,2.92s) with source hashes checked before/after.
Binary SHA256 `556b03864aa306f0233938ebdd1e67ac2160e74d7f37d3d02dd199e5fcf42903`.
Source44 digest `969911d28d1427e2ed34c41a8390b967de6e7e96d0d72460a0a7aa5ebad02645`
is SHA256 of compact sorted JSON path→SHA256 for cli/engine/schema/export src Rust,
their Cargo manifests, root Cargo/lock/toolchain and export prompt-template.txt.
Only the two named production source hashes differ from the original build.
Four unchanged queries/inputs/budgets were run against the verified copied binary
TEMP/uiblueprint-corrected; each call retained its enforced10s timeout.

| Case | Known engine quantity | Exit | Result SHA256 |
| --- | --- | ---: | --- |
| before width |32 css_px|0|570086f9f4ee3ca994191d28bc63cb9c86694225ed242b70776a3468c9cd0658|
| before height |16 css_px|0|c143237f541e72771e78a35e2263ae2aae1bf933e3a28b3c33187f68b1846a55|
| after width |48 css_px|0|1ef0a42940774d4716b7817f233e26be4f6fe6d7b5c6cbe72fbc94dc73b64dd7|
| after height |24 css_px|0|247bd57a1c29334fbaa2088b5f54ce3aee7aa948ccc2701b2830a4920b1103b8|

All amounts match the independent F01 oracle; QuantityKind is length, full Space
is the original viewport/css_px/top_left, details scalar, contributing Evidence
unchanged (web.dom:1:2 before, web.dom:1:3 after). Complete embedded Snapshot and
EvaluationInput id/revision/Context/Space equality were asserted, including retained
consistency=unknown. Original frame hashes still match. These are factual dimensions
of recorded rects, not a claim of globally atomic UI or fresh live refs.

The first corrected CLI call already succeeded; the temporary report reader used
the wrong nesting for canonical Value::Quantity and stopped afterward. That reader
was corrected; the saved successful result was consumed without rerunning its CLI.
The other three calls then ran once. No product or expected value was changed.

Imported actual-result verification via a separate caller remains unexecuted and
is no longer a requirement for this finite demonstration per root's amendment.
The CLI itself computed these four facts using the existing bound Rust engine and
validated round-trip output; no schema-validity-as-recomputation claim is made.
Future import still requires the existing verification API. Source/test SHA256:
resolve.rs e7a9d51163f2ebf96c9d94646f0fe2b7ccb3a88afeaf1f73148caebb32f8d7c1;
results.rs d04acb59338052449c85760b4ab7d6240467c533d7b3ebf57f0daaf97a403920;
engine analysis test f5540e79d094dcdfbc03621c4fcaccb45298f17c04ee2d60f7eded8438131eee;
schema analysis_contract test0c30edf5c0465838f77be8fede4e838990ac000d31a15ff61555057dcc551117.
