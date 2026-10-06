# Pure geometry candidate

G01 introduces `uiblueprint-engine` for L01/E01/G02. It reads canonical
`Snapshot`, `Expectation`, `Space`, `Transform`, `Evidence` and condition types
from `uiblueprint-schema`. It performs no IO, collection, action, clock read or
cache mutation. This is a controlled-data candidate, not P1/runtime acceptance.

## Entry points and explicit context

`measure(snapshot, expectation, &EvaluationContext)` returns a known quantity
with geometric details and contributing evidence, or a typed unknown reason.
`check` additionally applies the expectation's comparison and explicit tolerance,
returning a canonical `Finding`, the measurement and the tolerance used. Finding
IDs initially equal expectation IDs; a storage consumer can assign its own ID.
Invalid canonical inputs, relation arity/quantity kinds and nonfinite arithmetic
return `GeometryError`. Unknown inputs never pass, regardless of tolerance.

`EvaluationContext` borrows the explicitly requested result space, an optional
slice of canonical directional transforms, and optional observed canonical
`ContextConditions` with `Evidence`. It is a calculation input, not another wire
schema. Snapshot does not represent platform/input-mode/text-scale facts: absent
required applicability evidence yields `applicability_unknown`; a proved mismatch
returns `not_applicable`/unknown. Neither case is a passed check.

Anchor spaces identify source geometry; their fraction selects a coordinate in
[0,1]. Physical x/y are coordinate axes, not inferred leading/trailing directions.
Distance on xy uses both fractions as a point inside each rectangle; fraction
0.5 gives centers. Target order is explicit and is never spatially sorted.

## Quantities

| Relation | Derived scalar / prerequisites |
| --- | --- |
| width / height / ratio | One rect's width, height, width/height; zero denominator unknown |
| gap | Second anchor coordinate minus first; signed x or y |
| distance | Absolute axial distance or xy Euclidean distance |
| aligned | Maximum minus minimum anchor coordinate; at least two anchors |
| baseline | Maximum minus minimum reported baseline y; no inference from layout |
| inside | Minimum of all four signed child-to-container insets |
| overflow | Maximum escape across four sides, clamped to zero |
| intersects | Positive rectangle intersection area; edge contact has area zero |
| equal_spacing | Spread of signed adjacent edge gaps in explicit box order; at least three |

Alignment and equal-spacing require complete scope coverage with no declared
omitted/unknown members. A missing target/property, unrequested/unsupported/
redacted property, unstable/unknown observation consistency, missing mapping or
unsupported shape remains unknown. Each anchor requires its specified frame kind to match its property; explicitly
different frame kinds remain distinct and AX never replaces requested layout. Insets are
not declared padding; intersection is not visual occlusion. Area/ratio use
canonical `QuantityKind`, preserving `source_units` without new length units.

Comparisons follow canonical schema semantics: equal uses absolute error;
at-least and at-most extend their threshold by the explicit tolerance;
greater-than requires exceeding expected+tolerance. No decimal truncation,
implicit 1/2-pixel allowance or precision-derived tolerance is applied.

## Spaces, transforms and provenance

Conversion requires an explicit directional path; no implicit inverse, global
scale or origin flip. Each consumed transform must bind the snapshot's target,
node surface and environment revision, with an Observation/source/method.
The affine convention is x'=a*x+c*y+e, y'=b*x+d*y+f. All four corners are mapped.
Axis-preserving scales/translations/flips/quarter turns remain exact rectangles.
A rotated/sheared nonrectangular result, quad/polygon/fragments input remains
`unsupported_shape`; an enclosing box never proves containment/intersection.
Baseline conversion requires a y mapping independent of unavailable x.

Known measurements preserve source and transform evidence, including distinct
Observation IDs. The wire Finding can reference only one Observation, so it uses
the first contributing one; consumers needing full provenance use Measurement.
Unavailable results retain any evidence reached before the failure, a bounded
reason and no fabricated measurement. Their Finding references the first such
Observation when present.

## Source and verification boundary

Implementation is original Rust arithmetic. The accepted [R03 ledger](../research/R03-core.md)
provides Galen signed relations, Extras explicit ordered pairs and Compose
anchors, with exact upstream revisions/license dispositions. No upstream source
or runner dependency was copied. R03 G1/G2/E1/B1 and the unchanged independent
[GEO corpus](../../fixtures/golden-oracles/README.md) supply expected numbers.

Tests exercise canonical GEO fixtures plus separately authored arithmetic,
negative availability, transforms and applicability. Test-only numeric assertion
allowances for IEEE-754 representation are not product rule tolerances.
Scoped check/fmt/Clippy/test results and residuals live in the [G01 receipt](../plans/ui-blueprint/receipts/G01.md).
Shared schema/validator, oracle files and workspace/lock remain other owners.
