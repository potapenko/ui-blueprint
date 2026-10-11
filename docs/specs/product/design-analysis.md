# Explainable geometry and consistency recommendations

- Node type: leaf; domain: `uib.design-analysis`; contract: `UIB.DESIGN-ANALYSIS@1`.
- Authority: Active / Stability: Evolving; accepted/released baseline: none.
- Authority source: user 2026-10-11 approved recommendations and explicitly ordered
  execution: «План есть, создавай цель и делай»; [approved plan](../../plans/ui-blueprint/design-relations.md).
- Read when: composing measurements or exporting design-analysis requests.
- Requires: [ANALYSIS@2](analysis.md), [EXPORT@2](export.md), [CLI-EXPORT@2](cli-export.md).
- Do not read when: existing inputs omit design_analysis.

## UIB.DESIGN-ANALYSIS.INPUT

Each DrawingBrief ViewInput optionally contains design_analysis, default empty.
Direct observed metadata accepts the same optional list; --brief and existing
--snapshot/--metadata flags remain the entrypoints, with no model/network call.
Every request is explicitly caller-selected and contains a reviewed public caption.
relation: operation equal/ratio/balance, operands (canonical GeometryQuery list),
evaluation (canonical bound EvaluationInput). Equal compares at least two values;
ratio divides two; balance compares the sum of all but the last to the last.
pattern: basis (reviewed caller explanation of comparable family/state), operands,
evaluation. At least three distinct Width/Height/Gap queries of the same operation,
axis, frame kind and quantity kind. The caller-selected family is attributed, not
claimed as automatically inferred semantic membership.
check: expectation, evaluation, optional exception {source,reason}; exception is an
explicit caller-declared decision reference, never independently verified intent.
It annotates the computed check and cannot change pass/fail/unknown or tolerance.
No embedded computed result is accepted; engine recomputes all requests from the
unchanged view Snapshot. Proposed views with analysis reject before publication.

## UIB.DESIGN-ANALYSIS.TRUTH

Reuse bound canonical evaluation, source resolution, availability and evidence.
Composition uses one result Space and compatible quantities/frame kinds. Preserve
operand queries, full exact results/evidence and unknowns; no display arithmetic.
Ratio is a new composition over two quantities, not a redefinition of core Ratio.
Zero denominator is unknown. Nonfinite arithmetic is a typed failure. Equal and
balance report exact residual, not pass/fail or an invented comparison tolerance.
Pattern candidates use exact original values, not rounded labels. A unique strict
majority supported by at least two of at least three distinct measurements supplies
a candidate; ties/no majority produce no recommendation. This is a disclosed
selection heuristic, not statistical confidence, a token, requirement or defect.
Pattern recommendations require measurements from the same Observation, compatible
frame/axis/kind and no reported nonzero uncertainty; otherwise show unavailable
pattern analysis. Unknown measurements never silently disappear from the sample.
Exceptions retain their operand indices and signed deviations from the candidate.
Known checks retain comparison, expected_from, expected and explicit tolerance;
inequalities are not converted to equality suggestions. Unknown is not a failure.

## UIB.DESIGN-ANALYSIS.OUTPUT

Observed values stay unchanged. A package with selected analysis uses version0.3.0
(including CLI receipt), adds design_analysis to the corresponding scene view,
and includes human explanations in drawing-brief.md and prompt.txt. Without
requests all old package fields/versions/behavior remain unchanged.
Machine results retain safe aliases of queries, bindings, expectation/exception
references and contributing evidence; no raw process IDs/private method strings.
Source text never becomes executable instructions. Captions/basis/exception text
follow the existing explicitly reviewed public annotation policy.
Print a separate concise relationships/recommendations block. It is an explicit
exception to the old prompt's no-other-notes rule, not permission to alter UI.
Preserve exact-vs-approximate display distinctions. ImageGen only draws prepared
statements; no hidden grid inference, cause invention or automatic repairs.

## UIB.DESIGN-ANALYSIS.LIMITS

Requests and operands are individually capped by existing max_components and
serialized inputs/whole outputs by existing byte limits. Refuse invalid binding,
incompatible source, unsafe annotations or overflow before writing the package.
No new persistent directory policy, collector, action, dependency or core/analysis
wire change. No imported results, token ingestion or global constraint solver.

## Acceptance

Focused independent literals: equal/ratio/balance, remainder width, zero denominator,
unknown, incompatible units/frame, full binding, majority/tie/rounding/uncertainty,
check pass/fail/unknown/inequality and declared exception without suppression.
Actual CLI package proof includes old no-request compatibility and new0.3 output,
alias/privacy/refusal checks. Saved real-form operands ground examples; fresh image
QA checks the added text and unchanged source geometry. Registration is not QA.
