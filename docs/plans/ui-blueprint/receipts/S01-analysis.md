# S01 analysis0.2 schema implementation

Status: first API slice67d7e49 saved/pushed; remaining schema source is coherent and
checked. Authorized ANALYSIS-FLOAT-001 closes the measured production fidelity
dependency; integrated consumer checks/independent acceptance remain separate.

## Authority and basis

[S01-analysis-schema packet](../packets/S01-analysis-schema.md)06d339b under
PLAN.UIB@1, D03 delegation, accepted decision8fdf608 and registered7a61a65.
Current master; no nested work, runtime, new dependency or foreign source edits.
Reused complete current AGENTS -> registry6 -> ANALYSIS/TYPES/VALIDATION@1 and
CLI/EXCHANGE/GEOMETRY/MODEL/PRIVACY/IDENTITY/BOUNDARIES closure; D03@2,
D05@2/D05-MEMORY@1, RUST.md/DEV.RUST@2/D01@1/D07@2 and explicit dependencies.
Mode Restore the registered record/version/binding contract, not new intent.
Five shared repairs and E02 repairs remain accepted and unchanged.

## Concrete first slice

schema::analysis exports exact canonical records/version/artifact enums, factual
query extraction, result/reason helpers and real query/evaluation validators.
MeasurementResult::Known uses `{ measurement }`; Unknown uses `{ reason,evidence }`.
Details uses Scalar {}, Insets { left,top,right,bottom }, Intersection { rect },
Gaps { values }. Rust MissingTarget serializes as target_unresolved. All scalar
unavailable strings and helper semantics preserve the old engine's vocabulary.
Core can use/re-export those types without adding its own serialization model.
Full signatures and pending function names are in the
[API handoff](../../../development/schema.md#s01-analysis-api-slice--concrete-consumer-handoff).

Record macro reuse is crate-local; core model records/Artifact/SchemaVersion are
unchanged. Existing require/evidence/validate_anchor helpers only gain crate-local
visibility. Their bodies/meaning are unchanged. Source binding validates Snapshot
ID/revision/full Context and real Observation references. Standalone evaluation
validates declarations only; no absent source is fabricated or declared verified.

First write set: src/analysis/mod.rs/types.rs/validation.rs; schema lib export;
model.rs record-macro visibility; core validation.rs helper visibility; new
tests/analysis_contract.rs; schema.md implementation/status; this receipt.
No engine/CLI/export/cache/bridge/Cargo/old-fixture edits. New result validators,
dispatcher, owned_size and generator/files are not faked by placeholder functions.

## Preliminary proof

- cargo +1.96.0 check --locked --offline -p uiblueprint-schema --lib: passed.
- New analysis_contract tests:3 passed (factual extraction/arity/axes, exact version
  and strict shapes with core rejection, unbound versus bound identity/revision).
- Existing published_schema_matches_the_canonical_rust_records: passed; core
  generated schema remains unchanged.
- Scoped schema lib/new-test Clippy -D warnings passed.
- Schema package fmt check, owned-document links and scoped whitespace passed.
  The only format repair wrapped a newly crate-visible helper signature.
  Seven own Rust paths, sorted compact JSON path->SHA256 map digest:
  `87e5ee7b9dce448d0dfcb48638241a0ffd0aa52d3c082f7fc2451c2efd7aadcc`.
  Core model differs only by the record-macro re-export; core schema/fixtures,
  accepted repair tests and Cargo remain unchanged.

These tests use the existing development feature graph. They do NOT establish
production float fidelity. The remaining packet must test the actual production
decoder feature graph because dev jsonschema enables serde_json float_roundtrip.
No Cargo/D07 change is made preemptively; a concrete failure goes through root.

## Remaining authorized work

Implement the real result/declaration validators and exact-version bounded parser/
dispatcher; exhaustive accounting; generated0.2 schema and parity; independent
valid/invalid analysis cases and validator executable checks; production-feature
round-trip edges. Preserve old schema/126 fixtures and accepted regression behavior.
Arithmetic verification remains the engine's job. Root can dispatch Core/Export
against this compiled API while Integration finishes the disjoint schema files.
Shared consumer suites wait for the later coordinated stable-input barrier.

First coherent checkpoint67d7e49a0072eead590c65e379742554f5591fb8 was saved/pushed
under the nine-path grant; source hash matched, index empty and Git lease released.
Root dispatched Core consumers from that saved API. Further work stays in this
finite packet; it does not claim independent acceptance or live gates.

## Remaining implementation is concrete

AnalysisDocument::from_json/validate, validate_measurement_case,
validate_geometry_check_case, json_schema and validate_input_document are now real.
Two narrow modules separate codec/version dispatch from result consistency; input
declarations remain in validation.rs. There is no second graph/calculation engine.
Contract checks bind immutable source identity, eligibility/full evidence, declared
space connectivity, known property/condition/coverage restrictions, result/details
shape and Finding declarations. Coordinates/quantity/details are not recomputed;
the three deliberately tampered contract-valid fixtures require engine rejection.
Baseline x-dependent transforms/unsupported source shapes cannot claim known output.

The validator binary retains existing flags/output/exits and strict0.1 core parser,
adding exact0.2 dispatch. Its version probe consumes the complete object, skips
untyped values and rejects duplicate versions; strict target decoder handles the
whole shape. No raw serde/key/value/path errors are retained or reported.
New analysis owned-layout accounting is exhaustive; core Snapshot walk unchanged.
New generator/schema and48 authored fixtures preserve independent GEO literals,
explicit synthetic transforms/conditions and separate contract/arithmetic outcomes.
No candidate engine generated expected numbers; no old fixtures/schema changed.

Scoped schema check/fmt/Clippy and31 tests passed before the final neighboring-case
addition; the added unsupported-shape/baseline-x regression then passed separately
and Clippy passed again. Total32 distinct schema tests checked. The48-case analysis
manifest passes structural/semantic parity and actual validator-binary exit checks;
the old126-case manifest and core schema equality pass unchanged. No wider consumer
suite repeated while Core/Export migration is active.

## Production fidelity dependency — original evidence

The actual production schema rlib was built with cargo1.96 --locked --offline
--lib --message-format=json. Compiler-artifact metadata confirmed serde_json1.0.151
features alloc/default/std, schema features empty: no dev jsonschema/float_roundtrip.
A temporary Rust caller linked those exact rlibs and exercised public decoders;
all caller files/binaries were task-owned OS-temp and removed. No raw logs persisted.
An initial bounded2054-value diagnostic found204 changed bit patterns. The permanent
minimal regression driver is fixtures/analysis/check_production_roundtrip.py:
22 fixed cases across core0.1 Geometry.x and analysis0.2 Anchor.fraction/condition
metadata, including subnormal/minimum, zero sign and maximum finite values.
It reproduces6 failures, the same three values in each decoder:

| JSON literal | Expected f64 bits | Production decoded bits |
| --- | --- | --- |
|0.9394596570041933 |3fee100db2d7d206 |3fee100db2d7d205 |
|0.9958796677339681 |3fefde3f09756650 |3fefde3f0975664f |
|0.9216624818153901 |3fed7e42512b1d0b |3fed7e42512b1d0a |

The pinned crate's Cargo.toml declares float_roundtrip=[] (no new dependency).
Its src/de.rs selects lexical::parse_concise_float for that feature; the current
fallback instead converts significand to f64 and scales by powers of ten. This
explains the distinct decoder paths; the actual bit failures are the proof.
Development test success has feature unification and cannot accept production.

Exact required dependency: root assigns a bounded D07 revision plus root workspace
serde_json runtime float_roundtrip feature for the SAME pinned version, then rerun
the22-case production driver and affected checks. No cargo update/new dependency,
wire promotion, expected-value adjustment or tolerance is needed/proposed.
That proposal was not self-authorizing; the actual amendment and resolution follow.

Remaining source/schema/fixture input-set SHA256:
`11f6d3a5fbda477a4dbd5cb7e473dc9f759a2af8b96f7a07a52938ffc2ed4c66`.
Sorted compact JSON path->file-SHA256 map covers62 files: all analysis module and
fixture files except fixture README; validator binary, owned_size, new generator,
two analysis test files and generated0.2 schema. Documentation is excluded from
the code identity.48 cases:33 contract-valid (28 expected engine matches,3 expected
engine mismatches,2 standalone inputs),15 contract-invalid. This identity preceded
the explicitly authorized Cargo/D07 delta below; core wire/fixtures stayed unchanged.

## ANALYSIS-FLOAT-001 — authorized and verified resolution

Root amended this packet at00b19d3, explicitly permitting the same-version feature
experiment, then D07@3/route metadata BEFORE root manifest mutation if successful.
The task-temp production Cargo profile reused the pinned lock and the same schema
path plus serde_json=1.0.151/float_roundtrip. It reran the EXACT initial corpus:
six edge values plus2048 deterministic LCG-derived finite fractions, seed
0x9e3779b97f4a7c15, multiplier6364136223846793005, increment1442695040888963407,
mantissa mask0x000fffffffffffff with exponent0x3fe0000000000000.
Result:0 bit mismatches/2054 values, versus204 before. All17 registry packages and
versions matched the prior production build; only float_roundtrip joined alloc/
default/std. No dev jsonschema was present. Temporary project/caller/target were
run-owned outside the repository and removed after the bounded proof succeeded.

Pinned serde_json lexical module license header was inspected: MIT OR Apache-2.0,
copyright Alexander Huszagh. No source copied; I01 retains existing upstream notices.
D07@3 and registry7 were then recorded, followed by the single workspace dependency
feature edit. Cargo.lock remains byte-identical; no update/new package/default change.
Configured production driver then passed22/22 core0.1/analysis0.2 fixed cases with
features alloc/default/float_roundtrip/std and empty schema feature list.
Root/Core were notified that manifest input is stable:
Cargo.toml SHA2561df267dd297e776644e380192198ab919b8fff1f6ae8383841e589ef77cc821b.
Web's independent decode-bound audit must qualify this actual feature path.

After the feature/neighbor-case changes, schema all-target check/fmt/Clippy and
the full32-test schema suite passed once. Includes48 new vector/parity/validator
cases,126 unchanged legacy cases, canonical sizing and accepted seven regression
tests. No consumer workspace suite repeated during Core/Export migration.
The measured fidelity dependency is closed; this is not a promise of arbitrary
decimal accuracy beyond f64, a changed geometric tolerance, D05 peak proof or live QA.

Final schema input-set SHA256:
`72b5cd20b11630f8ac2202fe8ab131f6f37f1d4f59d0bc6a0f37cc8fc74e56f6` (214 files).
Recipe: sorted compact JSON path->SHA256 map of root Cargo.toml/Cargo.lock/
rust-toolchain.toml and all non-Markdown files under crates/schema, fixtures/analysis,
fixtures/golden, fixtures/golden-oracles and schemas; then SHA256 of that JSON.
Own docs/links/<=100-line spec nodes/whitespace pass. API signatures/types are stable
for Core; source is ready for the next checkpoint/final consumer barrier. Shared
Core/Export source is not included in this package-specific proof or staging.

## Exact remaining checkpoint paths

Checkpoint-ready:67 paths below, relative to the repository root. Core/CLI/Export
changes are excluded. Checks and final schema input hash above are unchanged;
this path inventory is documentation only. No index reservation until root grant.

```text
Cargo.toml
crates/schema/examples/generate_analysis_schema.rs
crates/schema/src/analysis/codec.rs
crates/schema/src/analysis/mod.rs
crates/schema/src/analysis/results.rs
crates/schema/src/bin/uiblueprint-validate.rs
crates/schema/src/owned_size.rs
crates/schema/tests/analysis_cli.rs
crates/schema/tests/analysis_contract.rs
docs/development/schema.md
docs/plans/ui-blueprint/receipts/S01-analysis.md
docs/specs/README.md
docs/specs/development/decisions/README.md
docs/specs/development/decisions/d07-reuse.md
fixtures/analysis/README.md
fixtures/analysis/baseline-origin-conversion.json
fixtures/analysis/build.py
fixtures/analysis/check-converted.json
fixtures/analysis/check-pass.json
fixtures/analysis/check_production_roundtrip.py
fixtures/analysis/distinct-observations.json
fixtures/analysis/empty-intersection.json
fixtures/analysis/evaluation-local.json
fixtures/analysis/height.json
fixtures/analysis/insets.json
fixtures/analysis/intersection.json
fixtures/analysis/invalid-check-outcome.json
fixtures/analysis/invalid-condition-reference.json
fixtures/analysis/invalid-details.json
fixtures/analysis/invalid-empty-evidence.json
fixtures/analysis/invalid-evidence-source.json
fixtures/analysis/invalid-kind.json
fixtures/analysis/invalid-known-missing-target.json
fixtures/analysis/invalid-query-normative-field.json
fixtures/analysis/invalid-result-space.json
fixtures/analysis/invalid-revision.json
fixtures/analysis/invalid-scalar-payload.json
fixtures/analysis/invalid-singular-transform.json
fixtures/analysis/invalid-transform-generation.json
fixtures/analysis/invalid-units.json
fixtures/analysis/invalid-unknown-value.json
fixtures/analysis/known-zero.json
fixtures/analysis/manifest.json
fixtures/analysis/measurement-gap.json
fixtures/analysis/missing-conditions.json
fixtures/analysis/missing-target.json
fixtures/analysis/missing-transform.json
fixtures/analysis/multi-hop-conversion.json
fixtures/analysis/not-applicable.json
fixtures/analysis/not-requested.json
fixtures/analysis/observed-conditions.json
fixtures/analysis/ordered-gaps.json
fixtures/analysis/overflow.json
fixtures/analysis/query-gap.json
fixtures/analysis/ratio.json
fixtures/analysis/redacted.json
fixtures/analysis/signed-gap.json
fixtures/analysis/tampered-check-contract-only.json
fixtures/analysis/tampered-quantity-contract-only.json
fixtures/analysis/tampered-unknown-contract-only.json
fixtures/analysis/unknown-property.json
fixtures/analysis/unstable-conditions.json
fixtures/analysis/unstable-source.json
fixtures/analysis/unsupported-property.json
fixtures/analysis/unsupported-shape.json
fixtures/analysis/width.json
schemas/uiblueprint-analysis-0.2.0.schema.json
```
