# Canonical local analysis types

- Node type: leaf; domain: `uib.analysis.types`; contract: `UIB.ANALYSIS-TYPES@1`.
- Authority: Active / Stability: Evolving; accepted/released implementation: none.
- Authority source: `L01-ANALYSIS-001`, accepted handoff8fdf608 under ROADMAP D03/PLAN.UIB@1.
- Read when: defining/serializing analysis0.2 inputs or results.
- Do not read when: an unchanged core0.1 artifact is the only consumer.
- Requires: [MODEL@1](model.md), [EXCHANGE@1](exchange.md), [GEOMETRY@1](geometry.md).
- Route/compatibility: [ANALYSIS@1](analysis.md); binding/verification: [VALIDATION@1](analysis-validation.md).

## UIB.ANALYSIS-TYPES.ENVELOPE

AnalysisDocument is `{schema_version: AnalysisVersion, artifact: AnalysisArtifact}`.
AnalysisVersion serializes exactly as `"0.2.0"`. Artifact uses strict `{kind,data}`:

| kind | data type |
| --- | --- |
| geometry_query | GeometryQuery |
| evaluation_input | EvaluationInput |
| measurement | MeasurementCase |
| geometry_check | GeometryCheckCase |

All records are map-only/snake_case. Reject duplicate/unknown members, array-shaped
records and payload on tag-only variants. Reuse existing strict record machinery,
ID bounds, canonical geometry/evidence types and serde parser. Optional metadata
uses explicit JSON null, not property unknown. No second graph/JSON parser exists.

## UIB.ANALYSIS-TYPES.INPUT

Exact fields (pseudocode types reuse the existing canonical owners):

```rust
GeometryQuery {
    id: Id, scope_id: Id, targets: Vec<SourceKey>,
    operation: GeometryRelation, anchors: Vec<Anchor>,
    quantity_kind: QuantityKind, units: Unit,
    applies_when: ContextConditions,
}
ObservedConditions { values: ContextConditions, evidence: Evidence }
EvaluationInput {
    snapshot_id: Id, revision: u64, context: Context,
    result_space: Space, transforms: Vec<Transform>,
    conditions: Option<ObservedConditions>,
}
```

Query contains no expected/comparison/tolerance/expected_from. Applies_when is an
explicit applicability condition, not executable code or an inferred requirement.
The compatibility extraction from an actual Expectation copies only the listed
query fields, preserving their order/meaning and never synthesizing normative values.
Evaluation stores selected full Space even when the result is unknown. Transform
order is retained. Conditions group only facts sharing their single Evidence.

## UIB.ANALYSIS-TYPES.RESULT

```rust
Measurement { value: Value, space: Space, details: MeasurementDetails, evidence: Vec<Evidence> }
MeasurementCase { snapshot: Snapshot, query: GeometryQuery, evaluation: EvaluationInput, result: MeasurementResult }
GeometryCheckCase { snapshot: Snapshot, expectation: Expectation, evaluation: EvaluationInput, measurement: MeasurementResult, finding: Finding }
```

MeasurementResult is internally tagged by status. Known is exactly
`{status:"known",measurement:Measurement}`; unknown is exactly
`{status:"unknown",reason:MeasurementUnknownReason,evidence:[Evidence]}`.
Known value is existing Value::Quantity, including zero. Unknown contains no
numeric value/details/placeholder. Finding remains the existing canonical leaf;
the bundle retains the actual Expectation, full measurement and evaluation.
MeasurementDetails is internally tagged by kind: scalar has no payload; insets
has finite left/top/right/bottom; intersection has rect:Option<Rect>; gaps has
values:Vec<f64>. Scalar is a strict empty-struct variant. Null intersection is
known empty intersection/area0, not unavailable. Preserve negative insets/gaps.
Inside/overflow use insets; intersects uses intersection; equal_spacing uses gaps
of length anchors.len()-1; other supported operations use scalar. No padding or
visual occlusion claim follows. Full Space equals the selected result Space.
QuantityKind preserves length/area/ratio. Quantity.source_units denotes selected
coordinate units: area uses their square, ratio is dimensionless; no new length units.

MeasurementUnknownReason is closed to the current engine's wire strings:
not_requested, unknown_property, unsupported_property, redacted_property,
target_unresolved, missing_transform, frame_kind_mismatch, unsupported_shape,
incomplete_scope, unstable_state, applicability_unknown, not_applicable,
undefined_ratio. Preserve those distinctions and no hidden fallback measurement.
Known evidence includes every contributing property/condition/transform source;
unknown includes reached/attempted sources. Deduplicate full Evidence equality in
first-use order, not by Observation ID alone. Keep distinct contributing observations;
unused input evidence need not be listed as contributing. Invent no aggregate source.

## UIB.ANALYSIS-TYPES.OWNERS

Canonical result/details/reason types live in schema::analysis; engine re-exports
its public result names and retains its borrowed EvaluationContext facade. Update
local constructors/patterns as needed; no second result model or calculator.
Add exhaustive owned_size accounting for new types. Existing Snapshot layout,
core version/artifact variants and canonical storage accounting remain unchanged.
Source snapshots are retained intact, including redacted/false/empty distinctions,
coverage/freshness/consistency and source clock domains. No payload reference IO.
