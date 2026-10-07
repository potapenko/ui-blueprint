# ANALYSIS-R1/R2 canonical decoder and oracle repair

Status: source/fixtures frozen, scoped author checks passed; checkpoint/review pending.
No self-acceptance or full P1/P6/live claim.

## Authority, route and ownership

[Updated packet](../packets/L01-analysis-repair.md) `0736ec7` explicitly retired
Integration's not-started source assignment and transferred exact decoder/fixture/
test paths to Core. Integration remained on D05 docs; Web owns Cargo/lock.
Approved PLAN.UIB@1/in-scope repair authority applies. Current master, no agents,
worktrees, new dependencies, new parser, DTO, calculator or semantic/tolerance delta.

Reused current fully read ANALYSIS/TYPES/VALIDATION@1 closure and RUST/DEV.RUST@2;
read the complete updated packet and [independent review](L01-analysis-review.md).
Requirements: TYPES.ENVELOPE map-only records, TYPES.RESULT attempted evidence,
VALIDATION.VERIFY/ACCEPTANCE independent declared vector recomputation.
D07@4 was read after Web's stabilized dependency change; existing versions and
runtime float_roundtrip remain unchanged. Core did not mutate those shared inputs.

## Repairs

R1: a private analysis-local helper applies the same `deserialize_map` →
`MapAccessDeserializer` boundary as canonical model::record, then delegates to
Serde's derived decoder. One macro declaration retains each public enum/variant/
field inventory; no serde_json::Value parser or parallel public representation.
MeasurementDetails, MeasurementResult and AnalysisArtifact now require object
input at every occurrence. Existing object serialization, duplicate/unknown-field
checks, IDs/version validation and generated schemas stay unchanged.

R2: build.py's redacted expected result copies the already authored attempted
property Evidence; it does not call engine or alter numerical expectations.
The generator writes only changed outputs. Original47 other analysis cases stay
byte-identical; only redacted result evidence and11 new R1 negative files/manifest
entries change. Numbers in the negatives are copied from authored object vectors.

New negative vectors cover all four Details variants (including scalar), both
Result variants, all four Artifact variants and result nested inside GeometryCheck.
All are structural=false/contract=false/not_checked. Direct enum parity tests
retain positive object forms including null known-empty intersection; explicit
duplicate/unknown/tag-only-extra rejection and actual validator sanitized errors
are checked. The Core manifest test was previously saved at cffd8d3 and unchanged.

## Frozen scoped checks

After root released the Cargo barrier, verified both supplied hashes before/after:
Cargo.toml `a87adec8d6b011a88b1ccbb40191eeae0dc2964b52bc50e01a1d9b2a073a36c2`;
Cargo.lock `a3f321614b4551712fa44ee402f697f159dc43d0a613fb85e1e8ecae371d6173`.
Rust1.96.0, metadata HEAD e37329dd83cf516ff27526f272fa51491420aae2, repair candidate
and stable Web-owned manifests. Focused commands passed:

```sh
cargo check --locked -p uiblueprint-schema -p uiblueprint-engine -p uiblueprint-cli --all-targets
cargo fmt -p uiblueprint-schema -p uiblueprint-engine -p uiblueprint-cli -- --check
cargo clippy --locked -p uiblueprint-schema -p uiblueprint-engine -p uiblueprint-cli --all-targets -- -D warnings
cargo test --locked -p uiblueprint-schema --test analysis_contract --test analysis_cli --test goldens
cargo test --locked -p uiblueprint-engine --test analysis
cargo test --locked -p uiblueprint-cli --test analysis_binary --test binary
```

49 tests passed: schema contract10/validator3/core goldens4; engine analysis13;
CLI analysis8/legacy11. All59 manifest rows exercised:28 match,3 true computation
mismatch,2 input-only NotAResult,26 contract-invalid rejected at their layer.
The exact prior redacted failure is now green without changing verifier/labels.
Both saved generated schemas match canonical generation; core0.1/126 fixtures
and prior oracle corpus remain unchanged. No Web transport or full-workspace suite.

Before/after278-file SHA256 identical:
`bc410094569952089186bdc5c9bf745a36bbdd2a41e5fc2df70a8b8946c8e724`.
Compact sorted JSON path→SHA256 includes root Cargo/lock/toolchain; all files in
crates/{schema,engine,cli,export}, fixtures/{analysis,golden,golden-oracles,export},
schemas and .cargo if present. Web source/tests intentionally excluded.
Protected core artifacts/engine production/CLI production show no diff vs c30aa20.
Redacted fixture now SHA256 `6569b66ef7185a2f0ffb9d2f2ec5202422e421b300d3ea46e1b8e1585bff373b`.
Manifest SHA256 `1dc197fa21c0531342558fb4b947c18d85580e2c83e35a305ae7e408478daaea`.

## Checkpoint / residual

Own changes: schema analysis/types.rs, tests/{analysis_contract,analysis_cli}.rs;
analysis fixtures README/build/manifest/redacted plus11 named invalid-*array*.json;
this receipt and [Core proof receipt](L01-analysis-repair.md). Engine manifest test
is already saved; no further engine/CLI/export/cache/schema-model/Cargo edits.
Own18 source/fixture digest `c2b734d6ee126e6954fcc3fab1787255b9eb22e84d16e483c5142a5b0fca9d7d`.
Links/whitespace passed. Root granted exactly20 repair paths including both receipts;
terminal chat returns SHA/push and Git release. Final saved-input proof awaits Web's
matching manifest checkpoint; unchanged checks need no rerun. Source stays frozen.
Same independent reviewer rechecks saved repair. No author acceptance, live/D05
working-memory claim, blanket schema audit or unrelated follow-up implementation.
