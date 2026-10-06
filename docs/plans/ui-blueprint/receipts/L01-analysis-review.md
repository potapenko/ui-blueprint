# Canonical analysis review

Reviewer: fresh `/root/analysis_integration_review`; artifact
`c30aa20091f7fc166b6ec0af63a39f74ceeecff7`, base `8bb5c02`.
Verdict: **reject**. Stage1 inspected pinned source/contracts/fixtures before
author receipts; Stage2 reconciled S01-analysis, L01-analysis-engine-cli and
E01-analysis receipts. No tests/builds/runtime, mutations or external communications.

## Findings

1. **ANALYSIS-R1, P2 — object-only decoding.**
   `crates/schema/src/analysis/types.rs:76–78` and the new MeasurementResult /
   AnalysisArtifact derived decoders accept positional arrays. Pinned serde
   visitors admit sequences such as details `["scalar"]` or artifact
   `["geometry_query", {...}]`; semantic validation loses the original shape.
   This violates [ANALYSIS-TYPES.ENVELOPE](../../../specs/product/analysis-types.md#uibanalysis-typesenvelope)
   lines23–24 and generated object schema. Integration owns map-only decoding
   for all three enums plus nested negative/schema-parity coverage.
2. **ANALYSIS-R2, P2 — redacted oracle and missing verification coverage.**
   `fixtures/analysis/build.py:98–100` emits empty result evidence while declaring
   engine_verification=match. Current resolve::property records attempted property
   evidence before RedactedProperty, so engine verification returns ResultMismatch.
   Existing tests never consume that manifest field. Integration repairs the fixture;
   Core exercises every declared match/mismatch with the canonical engine verifier.
   Basis: [ANALYSIS-TYPES.RESULT](../../../specs/product/analysis-types.md#uibanalysis-typesresult)
   lines83–85 and [VALIDATION.ACCEPTANCE](../../../specs/product/analysis-validation.md#uibanalysis-validationacceptance)
   lines89–96. Do not remove evidence or weaken verification to fit the old fixture.

Criteria2–6 had no additional actionable defects. Unchanged core schema/legacy
fixtures/cache/replay/plugin paths were checked; production float_roundtrip uses
the pinned dependency path without a version change. Reviewer reproduced the
281-file hash `6fe25292b369125b0c988de632aee77b69fa2de83d75a805234e682b4947f96e`.
The150 tests/doctest/production fidelity checks remain author execution evidence;
they do not cover the two findings. No full P1/P6/live acceptance follows.

Same reviewer retained for affected recheck after saved repairs; no new reviewer.
Repair authority is the approved PLAN.UIB@1 and direct user clarification recorded
in [execution](../execution.md), not the read-only review itself.
