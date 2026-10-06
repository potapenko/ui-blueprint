# S01 analysis0.2 schema implementation

Status: first coherent canonical types/input-API slice ready for root handoff and
checkpoint. Full result validation/dispatcher/accounting/schema/fixture proof is
still in progress; raw serde decode is not claimed complete semantic validation.

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

First coherent checkpoint awaits root Git grant; return exact SHA/push/source
identity and limits. This receipt will be updated through the same finite packet;
it does not claim full schema capability, independent acceptance or live gates.
