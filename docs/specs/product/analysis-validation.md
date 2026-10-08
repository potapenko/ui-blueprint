# Local analysis validation and recomputation

- Node type: leaf; domain: `uib.analysis.validation`; contract: `UIB.ANALYSIS-VALIDATION@1`.
- Authority: Active / Stability: Evolving; accepted/released implementation: none.
- Authority source: `L01-ANALYSIS-001`, accepted8fdf608 under ROADMAP D03/PLAN.UIB@1.
- Read when: validating/computing/importing analysis artifacts or designing their tests.
- Do not read when: an unaffected existing core validator path is sufficient.
- Requires: [TYPES@1](analysis-types.md), [GEOMETRY@1](geometry.md), [MODEL@1](model.md), [PRIVACY@1](privacy.md), [IDENTITY@1](identity.md).
- Compatibility/CLI and change record: [ANALYSIS@2](analysis.md).

## UIB.ANALYSIS-VALIDATION.BINDING

Use unchanged canonical Snapshot validation. Evaluation must match Snapshot ID,
revision and compatible full Context, including schema/plugin versions, target/
Surface generations, scope/fields/projection and environment revision. Never mutate
Snapshot/Observations to make the input pass. Query scope matches the Snapshot;
targets are nonempty/unique and contain every anchor target. A missing actual node
remains target_unresolved/unknown rather than an invented or silently removed target.
Reuse existing Anchor validity and G01 shape rules in one schema-owned vocabulary:

| Operation | Arity / quantity |
| --- | --- |
| width, height, ratio |1; length except ratio |
| aligned, baseline |at least2; length |
| equal_spacing |at least3; length |
| gap, distance, inside, overflow, intersects |2; length except intersects area |

Gap/aligned/equal_spacing require a shared x/y axis; distance requires a common
valid axis. Other Anchor rules retain existing behavior. Shape validation moves,
not arithmetic. Known units/kind/details/Space must agree with query/evaluation.
Unit/result-space mismatch can remain missing_transform/unknown as existing G01
does; it never permits a known quantity in an unrelated coordinate space.

## UIB.ANALYSIS-VALIDATION.EVIDENCE

Transform inputs are ordered directional canonical records with finite nonsingular
affines, complete from/to Spaces and target/environment/declared Surface binding.
A consumed transform must bind the anchor node Surface. Preserve G01 path order,
all-corner geometry, supported-shape and baseline-y restrictions. No implicit
inverse, origin/scale conversion or transform discovery. An arbitrary attached
transform or matching units is not evidence that arithmetic was performed.
All transform/condition Evidence resolves to existing Snapshot Observations with
matching namespace and valid method/uncertainty. There is no supplemental Observation
registry in this contract. New independent facts require a normal new canonical
Snapshot carrying their actual observations, not fabricated records in evaluation.
Non-null condition fields share one Evidence. Never merge different-observation
facts under it; per-field multi-observation input remains a separate finite follow-up.
Conditions are supplied observed facts, not copies of applies_when or guesses from
OS/backend/name. Missing required facts -> applicability_unknown; proved mismatch
-> not_applicable/unknown; unstable source -> unstable_state. Empty applies_when
requires no conditions. Preserve existing stored freshness/availability semantics.

## UIB.ANALYSIS-VALIDATION.CONTRACT

Schema checks strict shape, binding, query arity/dimensions, finite result/details,
eligible referenced evidence, unknown-without-value and declared Finding consistency.
Known results need evidence; unknown may have none before any property was reached.
Arbitrary result evidence absent from eligible property/transform/condition inputs
rejects. Do not require every unused input source as contributing evidence.
Finding expectation_id/snapshot_id match the bundle; known measured equals its
Measurement value and status agrees with the explicit comparison/tolerance; unknown
has no measured value and the matching reason. Primary observation is the first
measurement Evidence's Observation, or none. Full secondary provenance remains.
Check predicates keep existing finite comparison arithmetic and no implicit epsilon.
Old0.1 FindingCase/common-source-space validation remains unchanged. Do not apply
that limited gate to0.2 instead of its full selected-space/evaluation/measurement
contract. Reuse nongeometric identity/outcome helpers only without weakening legacy.
Contract-valid declarations do not prove observed reality or recomputed arithmetic.
A tampered amount with internally consistent status can be contract-valid; that
does not authorize its reuse as a computed result. Errors retain no private payload.

## UIB.ANALYSIS-VALIDATION.VERIFY

Engine alone implements measure_query and faithfully factors existing measure's
Expectation input through the factual query; check retains the actual expectation.
Engine verify_analysis_result validates the contract then recomputes the exact
query/check from immutable Snapshot and EvaluationInput. Compare quantity, full
Space, details, all evidence, unknown reason and Finding semantic fields. An assigned
Finding.id may differ; identity links/status/measured/observation/reason must agree.
No schema->engine dependency or second arithmetic implementation. CLI outputs come
from actual engine evaluation; imported/cached external analysis MUST be recomputed
before reuse as computed results. Schema validator valid/0 alone is insufficient.
Use the same deterministic f64/serialization contract, not a new tolerance. Exact
round-trip edge vectors must pass; a fidelity failure is a serialization dependency,
not permission for decimal truncation, epsilon or a relaxed UI measurement rule.

## UIB.ANALYSIS-VALIDATION.ACCEPTANCE

Independent literals/negative mutations cover factual/no-Expectation known0/signed
gap/area/ratio/empty intersection/gaps; incompatible same-unit spaces; sourced
multi-hop/unit/origin/baseline transforms; missing/redacted/unsupported inputs;
distinct/unused evidence; missing/mismatched/unstable conditions; binding mismatch;
tampered value/Space/details/evidence/outcome rejected by engine recomputation;
strict0.1/0.2 cross-decoder rejection; unchanged core schema/126 fixtures; exact
aggregate input/output bounds, round-trip and sanitized errors. Candidate output
is not its own independent expected fixture. Preserve accepted shared/E02 regressions.
