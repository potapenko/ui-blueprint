# Pure geometry and canonical analysis

`uiblueprint-engine` computes deterministic geometry on immutable canonical data without IO, collection, actions, clocks, cache mutation or a model. [ANALYSIS@1](../specs/product/analysis.md) owns factual queries/versioned results; [GEOMETRY@1](../specs/product/geometry.md) still governs arithmetic.

## Entry points and ownership

| API | Meaning |
| --- | --- |
| `measure_query(snapshot, query, &EvaluationContext)` | Factual GeometryQuery; no expected value/tolerance/normative source |
| `measure(snapshot, expectation, &EvaluationContext)` | Compatibility wrapper validating the actual Expectation and extracting only query fields |
| `check(snapshot, expectation, &EvaluationContext)` | Sourced requirement, explicit comparison/tolerance, Finding plus full measurement |
| `measure_query_bound(snapshot, query, &EvaluationInput)` | Full binding validation before the same calculation |
| `check_bound(snapshot, expectation, &EvaluationInput)` | Full bound check without the narrower legacy0.1 FindingCase gate |
| `verify_analysis_result(&AnalysisDocument)` | Contract validation then exact engine recomputation of imported results |

GeometryQuery and Measurement are schema-owned. Engine re-exports `MeasurementResult`, `MeasurementDetails as Details`, `MeasurementUnknownReason as UnknownReason`; no second result DTO or arity/dimension/axis validator. Variants: `Known { measurement }`, `Unknown { reason, evidence }`, `Scalar {}`, `Insets { ... }`, `Intersection { rect }`, `Gaps { values }`.
EvaluationContext remains the borrowed local facade. It carries no Snapshot identity: imported EvaluationInput must use bound APIs; private adaptation occurs after validation. Bad IDs/revisions/full Context/generations/evidence reject without changing Snapshot. New facts need existing canonical Observations, not a supplemental registry or guessed metadata.
GeometryError retains typed input/rule/transform/nonfinite failures. Unavailable inputs are unknown, never pass. Finding.id initially equals expectation.id; callers may assign another storage ID. Query mode invents no Expectation/expected_from.

## Quantities

| Relation | Derived scalar / prerequisite |
| --- | --- |
| width / height / ratio | Rect width, height, width/height; zero denominator unknown |
| gap | Second anchor minus first; signed x/y |
| distance | Absolute axial or xy Euclidean distance; fraction0.5 selects centers |
| aligned / baseline | Coordinate spread / reported baseline-y spread; at least2 |
| inside / overflow | Minimum four signed insets / maximum escape clamped to zero |
| intersects | Rect intersection area; edge contact is known area0 with rect=None |
| equal_spacing | Spread of signed adjacent edge gaps in explicit order; at least3 |

Canonical query validation owns shape vocabulary; arithmetic remains G01 Rust. Physical axes are not inferred leading/trailing, target order is not spatial sorting, and each anchor requires its own frame kind. AX does not replace layout, measured insets are not padding, intersection is not visual occlusion.
Alignment/equal-spacing require complete scope with no omitted/unknown members. Missing target/property, unrequested/unsupported/redacted data, unstable/unknown consistency, missing mapping and unsupported shape stay unknown. Sensitive-redacted properties retain attempted source Evidence without exposing a quantity. Missing applicability facts→applicability_unknown, mismatch→not_applicable, unstable source→unstable_state. Empty applies_when consumes no unnecessary conditions.
Area/ratio retain QuantityKind and selected source_units; area squares coordinate units, ratio is dimensionless. Equal uses absolute error; at-least/at-most extend threshold by explicit tolerance; greater-than requires expected+tolerance. No hidden epsilon, precision allowance or truncation.

## Spaces and evidence

Output uses the complete selected Space. Sourced directional paths preserve units/origin: no implicit inverse, global scale or discovery. Affine convention: x'=a*x+c*y+e, y'=b*x+d*y+f. Consumed transforms bind target, anchor-node Surface and environment. Canonical bound input also validates all declared transforms/conditions against existing Observation references.
All four corners are mapped. Axis-preserving translations/scales/flips/quarter turns remain exact rects; rotated/sheared nonrectangular results and quad/polygon/fragments remain unsupported_shape. An enclosing box cannot prove containment/intersection. Baseline y cannot depend on an unavailable x.
Evidence deduplicates full equality in first-use order, not Observation ID alone. Distinct methods/uncertainties/observations remain; unused inputs are not invented as contributors. Unknown retains reached sources and no numeric placeholder; its evaluation still carries result Space. Finding references first contributing Observation; analysis0.2 retains secondary provenance and evaluation.

## Verification and compatibility

Schema checks declarations/binding/consistency, not geometry truth. Engine verification recomputes quantity, Space, details, ordered evidence, unknown reason and Finding semantics. Only caller-assigned Finding.id may differ. Contract-valid tampered amount/details/eligible-unused evidence/unknown reason/false pass fails recomputation.
AnalysisDocument is strict0.2; embedded Snapshot/Context and transport remain core0.1. Cache/replay, old fixtures and generated core schema do not migrate. Legacy check JSON defaults to0.1; full results explicitly select0.2 ([CLI](cli.md)). Export uses factual queries; its package format stays unchanged.
Exact finite-f64 decoding uses [D07@3 ANALYSIS-FLOAT-001](../specs/development/decisions/d07-reuse.md#analysis-float-001--exact-source-number-preservation) on the same serde_json version. No comparison is relaxed for serialization. Actual binary/round-trip checks supplement in-process tests.
[R03](../research/R03-core.md) and unchanged [independent GEO oracles](../../fixtures/golden-oracles/README.md) remain the numeric basis; no upstream source/runner copied. New checks cover factual/unknown values, bound conversion/baselines, conditions, evidence and tampering. [Analysis receipt](../plans/ui-blueprint/receipts/L01-analysis-engine-cli.md) records current proof; [G01](../plans/ui-blueprint/receipts/G01.md) preserves earlier history. Live condition acquisition/transform discovery and D05 working-memory enforcement remain separate.
