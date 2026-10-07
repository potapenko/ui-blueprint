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

## Existing command and actual outcome

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

## Concrete residuals, not successful recomputation

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
