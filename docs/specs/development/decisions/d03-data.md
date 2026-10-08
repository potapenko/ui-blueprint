# D03 — candidate data and rules ownership

- Domain: `uib.development.d03`; accepted/released baseline: none.
- Authority source: [C01-DEC-001](README.md#meaning-and-precedence), [L01-ANALYSIS-001](../../product/analysis.md#change-record).
- Node type: leaf; contract: `UIB.D03@3`; clause: `UIB.D03.CONTENT`; supersedes @2.
- Authority: Active / Stability: Evolving; candidate schema, no released format.
- Read when: S01 envelopes/validator, G01 rules or K01 graph delta.
- Do not read when: no shared data/compatibility choice is affected.
- Requires: [MODEL@1](../../product/model.md), [EXCHANGE@2](../../product/exchange.md),
  [IDENTITY@1](../../product/identity.md), [GEOMETRY@1](../../product/geometry.md),   [PROJECTIONS@1](../../product/projections.md), [FORMS@1](../../product/forms.md),
  [CACHE@1](../../product/cache.md), [ACTIONS@1](../../product/actions.md),   [GOLDEN@1](../../acceptance/golden.md), [evidence](evidence.md);
  [ANALYSIS@2](../../product/analysis.md) and its closure for local analysis serialization.
- Owner/deadline: S01 definitions and valid/invalid examples before P1 tests;
  G01 measurements, K01 atomic replay; final compatibility after both pilots/P3.

## Requirement, observed evidence and chosen representation

R03 shows why complete selected-field replacement differs from merging old values; R01/R02 show distinct DOM/AX/probe identities and incomplete design information.
Choose Rust structs/newtypes and closed enums with serde JSON at the boundary. Schema owns wire types/validation; engine owns materialized graph/analysis, plugin
API owns lifecycle/capability methods. No second schema in adapters or experiments.

Initial core transport wire version is `0.1.0`; advertised accepted range initially exactly `0.1.0`. It is not the illustrative `0.3-draft` from CACHE. A compatible
range can expand only with tests. Core fields are snake_case, unknown core fields rejected; bounded namespaced extensions preserve native identifiers explicitly.
Breaking pre-release wire changes advance minor version and fixtures together;
editorial-only changes do not. Never deserialize incompatible data optimistically.

| Responsibility | Required expression / validation |
| --- | --- |
| Identity | Typed session/target/surface/generation/source-element keys and Observation/Snapshot/revision IDs; serialized opaque strings, never coordinates/title as identity; native_role retained when available |
| Property | Known carries typed value (false/empty valid); unknown/unsupported/redacted carry reason/evidence and no value. not_requested is selection, not availability. No nullable property in initial candidate unless separately defined and exemplified |
| Provenance | reported/derived/estimated plus source, Observation interval, method/time and meaningful precision. Declarations and normative expectations remain separate |
| Observation | start/end and clock_domain; channel observations may differ. live/cache source, current/stale/unverified freshness, consistency and scope/fields coverage are independent |
| Geometry | Frame kind distinguishes layout/AX/hit/visible/paint; coordinate-space ID, units enum px/css_px/pt/dp, origin and sourced transform. Validate finite numbers, valid shapes and compatible transforms; unavailable transform → missing_transform |
| Forms | visible_text and accessibility_name separate; keyboard/accessibility focus, active descendant, selection units and composition state separate; draft/applied/business outcome require explicit evidence |
| Graph/views | Many-to-many represents/corresponds_to with evidence, explicit component mapping only; view suppression does not remove source nodes or create action refs |
| Delta | Full upsert in selected fields/projection, explicit justified removal, whole-context/base compatibility; children/relations/focus published together or resync_required |
| Source oracle | source_state only where reproducible; never invent global OS revision. Full/delta compares the same recorded checkpoint and coverage |
| Errors/actions | Existing ACTIONS error vocabulary; delivery and verification separated, partial completed_steps preserved; timeout after potential effect → action_outcome_unknown, not automatic retry |

Only `Known` contains `value`; omission of a requested property is invalid, not
implicitly unknown. Unrequested history can remain explicitly historical under
its old Observation; it is not included as a current value. A complete empty
scope differs from failed extraction or a narrower projection. Raw handles and
secret material cannot enter serialized graph/errors; redaction precedes storage.

## Rules choice and acceptance

Choose a declarative JSON relation record, not executable JS or a textual DSL
parser in P1: id, scope, targets, relation, parameters, units, tolerance,
applies_when and expected_from. Relation is a closed enum for the operations in
GEOMETRY; applies_when is typed context predicates, not arbitrary code. S01 defines
records; G01 owns arithmetic. No default Galen 2 px/Extras ±1 px tolerance or
precision truncation. Unknown required measurement cannot yield pass. Output
includes pass/fail/unknown, value when measured, units/tolerance/Observation/reason.
Human shorthand may be added later without becoming a second execution engine.

S01 publishes machine-readable JSON Schema plus semantic validator; serialization
alone cannot validate graph identities/units/delta compatibility. GOLDEN01 includes
valid/invalid examples for request, capability/session, property, observation,
snapshot, delta, action/transition, expectation/finding and error. Test false,
empty, redacted, missing required property, unsupported/unknown, wrong generation,
invalid unit/transform/version, lost base, cancellation after delivery and two
same labels. Rust golden equality is structural, not formatted text equality.
Validator exit codes: 0 valid, 2 invalid contract/input, 1 IO/internal failure;
machine result only on stdout, sanitized diagnostics on stderr. These are validator
codes; public CLI action exit mapping remains L01/A01 work before publication.

RC01–RC05 context exercises generic nested surfaces/anchors, separate labels,
resize/reflow and source mappings; no Settings/Genre/Director or mobile-specific
business meaning in schema/engine. Mobile shapes in types do not claim adapters.
Schema candidate stays Evolving until Mac/Web P2/P5 and P3 acceptance, even if
S01 parser/tests pass. No exported DrawingBrief implementation is decided here.

## Local analysis boundary — L01-ANALYSIS-001

ROADMAP delegates this technical choice under PLAN.UIB@1; root accepted the exact
handoff8fdf608 before [registration](../../../plans/ui-blueprint/packets/L01-analysis-registration.md).
D03@2 adds schema-owned analysis0.2 query/evaluation/measurement/check artifacts
through ANALYSIS/TYPES/VALIDATION@1. Reuse unchanged core0.1 Snapshot/Context and
strict transport/parser/negotiation/cache keys; old generated schema/126 fixtures
do not migrate. No crate/dependency bump is implied by this local wire version.
Measure JSON gains factual0.2 output; full check JSON explicitly selects0.2 while
legacy check remains0.1. Schema contract validation is not arithmetic verification;
the sole engine recomputes imported results. No fabricated facts or source mutation.
Compatibility: unreleased new analysis format with explicit version rejection;
old product meaning, accepted shared/E02 fixes and remaining live gates protected.
Source packets and affected consumer proof follow this registration, not vice versa.


## M05-COMPOSITION-001

Under the selected [M05 Integration packet](../../../plans/ui-blueprint/packets/S01-native-proof.md#m05-design-composition-admission--2026-10-08),
D03@3 adopts EXCHANGE@2.COMPOSITION: homogeneous ChannelResponse plus only a
probe-wrapper/design/exact AX+probe Snapshot. Request-aware session admission must
check every nested channel's requested membership and Supported/Partial observe
capability before publication; wrapper/slot/evidence/limits remain distinct.
No wire field/version/schema/126-golden change; the prior homogeneous-only semantic
restriction is explicitly relaxed only for this pair. Old ordinary single-channel,
failed-channel, cache, analysis and source/action-identity contracts stay protected.
This registration is not implementation or Native runtime acceptance.
