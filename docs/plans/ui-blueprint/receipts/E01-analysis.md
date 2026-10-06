# E01 factual query caller adaptation

## Authority and basis

Finite [E01-analysis-adaptation packet](../packets/E01-analysis-adaptation.md)
at `8b100c2`, under approved PLAN.UIB@1 and registered
[ANALYSIS.MIGRATION](../../../specs/product/analysis.md#uibanalysismigration).
Mode Restore; current master; no nested work, worktree, runtime/model or source
application operation. This is a caller migration, not new product arithmetic.

Traversal: current spec registry revision6 → product branch → ANALYSIS@1,
ANALYSIS-TYPES@1 and ANALYSIS-VALIDATION@1 completely read, plus D03@2. Reused the
full current required CLI/EXCHANGE/GEOMETRY/MODEL/PRIVACY/IDENTITY/BOUNDARIES and
EXPORT/DRAWING closure, RUST/DEV.RUST@2 and applicable decision dependencies.
Existing instructions/execution route remain unchanged from the established basis.
Registered0.2 analysis representation does not change core0.1 source data,
DrawingBrief/package versions, existing output meaning or accepted E02 fixes.

Writes only observed.rs, focused compiler.rs tests, export development API handoff
and this receipt. No schema/engine/CLI/Cargo/fixture/spec/proposal arithmetic edits.

## Concrete provider and adaptation

Schema API saved at `67d7e49`. Core's concrete compiling working API was inspected
before changes/checks; it is not described as a saved engine provider:

```rust
measure_query(&Snapshot, &GeometryQuery, &EvaluationContext)
    -> Result<MeasurementResult, GeometryError>
MeasurementResult::Known { measurement }
MeasurementResult::Unknown { reason, evidence }
```

Engine re-exports schema-owned GeometryQuery/Measurement/MeasurementResult,
MeasurementDetails as Details and MeasurementUnknownReason as UnknownReason.
The canonical query fields are id, scope_id, targets, operation, anchors,
quantity_kind, units and applies_when. No speculative signature/DTO was added.

Observed width/height extraction now constructs GeometryQuery directly and calls
measure_query. The prior expected0, equal comparison, tolerance0 and artificial
expected_from were removed. Context still selects the same full source Space,
no extra transforms and no supplied conditions. Result matching uses canonical
Known{measurement}; unknown reasons and all reached evidence are unchanged.
No change to alias assignment, dimension fields/anchors, source kinds, unit values,
coverage, sheet plans, unknowns, statuses, package content or source Snapshot.

## Preliminary scoped evidence and exact provider inputs

Inspection/check HEAD `00b19d3f2bd6a76c98b9b2f87f28f5b71b6a9ed9` with active Core /
Integration migration changes. The42-file provider map was unchanged before and
after scoped checks: root Cargo.toml/Cargo.lock/rust-toolchain.toml plus every
schema/engine `.rs` and Cargo.toml, including new working files. SHA256 of sorted
compact JSON path→file-SHA256 mapping:
`6ff0f12325e5fbfa25f83480c119711e12f03f43482a6dba857851fa382894de`.
Concrete engine src/lib.rs SHA256
`7bbe95ff293a0d0bc7f4bbe0c83100d42cfe19f0fab6c48dd6f89eb3dd6d42c5`;
schema analysis/types.rs SHA256
`af914af4deb276619797037d384f93d96af811af8a602ac4ddfc6411dcf981bd`.

Passed:

- `cargo check --locked -p uiblueprint-export --all-targets`
- `cargo fmt -p uiblueprint-export -- --check`
- `cargo clippy --locked -p uiblueprint-export --all-targets -- -D warnings`
- `cargo test --locked -p uiblueprint-export`:20 tests passed, including all7
  accepted proposal-arithmetic regressions and the new full-package equality case.
- After adding the final known-result case,
  `cargo test --locked -p uiblueprint-export --test compiler factual_query_`:
  both focused migration tests passed. Total21 distinct export tests verified;
  unchanged suites were not repeated.

Both saved observed/proposed packages match structurally for every JSON file and
byte-for-byte for prompt/brief text. The known-result canonical authored fixture
independently asserts width30/height10 css_px, unchanged input, exact contributing
evidence, no requirement_ref/check_tolerance and unverified image status. These
synthetic/retained fixtures are not a new runtime observation. Existing observed
partial coverage and unknown dimensions remain exactly as saved.

Own source/test set (observed.rs and compiler.rs) SHA256 by the same map recipe:
`f86332d8555b9d7507972596dcba30f8329753b8f00b3fc48e57b72bae335006`.
Source-ready/frozen. Final proof still requires saved stable schema/engine/CLI
integration; a shared migration barrier may supply matching evidence once. These
preliminary results do not accept an unsaved provider or a broader schema migration.
Root granted the four listed paths for scoped commit+push after reading this
receipt and accepting the checks only as preliminary author evidence. Master,
empty index and unchanged own source/test hash were verified before staging.
Exact checkpoint SHA/push and Git release return in the terminal receipt. Source
stays frozen until the shared migration barrier. Same E02 repairs remain closed;
no broad E02 review or unrelated suite is requested here.
