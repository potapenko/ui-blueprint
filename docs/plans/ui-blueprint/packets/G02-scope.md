# G02 explicit relation neighbors

Class shipping_product; retained Core owner01a111a7-9887-7983-9aa0-c08dfa2d46bc,
inherit, no nested delegation, master only. User-authorized PLAN.UIB@1 parallel
execution; [runbook](../execution.md) and scope checkpoint/push apply.

## Basis and ready outcome

Restore PROJECTIONS.CONTENT: scope uses explicit relationships first; source
identity, many-to-many links and original graph remain intact. Root read current
AGENTS → registry11 → product branch → PROJECTIONS/GEOMETRY/MODEL/IDENTITY/
BOUNDARIES@1 and full NATIVE→EXCHANGE/LIFECYCLE/PRIVACY closure, D03@2 and its
core-data dependencies, RUST.md/DEV.RUST@2. Reuse current full closure from
[source handoff](G02-source-handoff.md), not its summary as substitute authority.
No semantic specification delta or new wire. Exclude analysis serialization,
platform collection/actions, cache/CLI/export, geometry-based grouping and mobile.

Core's finite source handoff found geometry/check, bound analysis, replay/cache
already present, but no neighbors/view API. Existing Snapshot.relations,
RelationKind, SourceKey, Node and Evidence suffice; no schema change needed.
This implementation decision uses that observed source plus PROJECTIONS.CONTENT;
the proposed borrowed API is an engineering realization, not a new product rule.

Implement one-step neighbors over existing incident relations, retaining the
counterpart nodes, relation direction/order/Evidence and all source namespaces.
Use the canonical validation::validate_snapshot and existing types. Borrow the
data; do not clone/build a second graph, rewrite Snapshot/context/projection,
merge identities, infer edges from names/rectangles or create action refs.
Use explicit output limits and distinguish selection truncation from original
coverage. Missing seed or invalid input refuses explicitly. No implicit product
defaults, invented freshness or missing-data substitution. Return source context
and existing availability honestly. Choose the smallest clear API consistent
with the handoff; no new framework or generic query language.

Immediate consumer: G02 scope selection on stored canonical graphs, later
interaction/design views. Live source qualification still belongs to W01/M01/P01;
this pure API does not claim their completeness. Economy: three engine files and
focused deterministic tests; expand only for an actual missing contract/type.

## Ownership and acceptance

Write only crates/engine/src/scope.rs (new), crates/engine/src/lib.rs (export only),
crates/engine/tests/scope.rs (new), docs/development/scope.md (new) and
docs/plans/ui-blueprint/receipts/G02-scope.md (new). Declare actual subset first.
No host/Web/Native/schema/plugin-api/Cargo/lock/spec/registry changes. Return a
concrete dependency before crossing these boundaries.

Focused independent literal expectations: LabelledBy/ErrorFor in both directions;
many-to-many Represents/CorrespondsTo across namespaces without merging; equal
names/boxes alone yield no edge; missing seed refusal; bounded selection reports
incompleteness; original partial/unknown/redacted/provenance/graph remains intact.
Cover concrete edge cases from the implementation without expanding to a new
test framework. Run scoped format, engine check/Clippy and new relevant tests;
unchanged full workspace/live suites are unnecessary. Existing validation
allocations retain their established ownership; no allocation-free/RSS claim.

Web/Native compile/runtime consume engine inputs: coordinate a short input hold
only for ready builds/runs, never hold the whole lane during implementation.
Use separate task-temp target output. Each coherent step saves exact paths and
pushes after root Git grant. Return API, actual checks, saved input identity,
scope/self-review and residuals; final G02 integration/adapter acceptance remains
open. No new independent reviewer wave for this bounded deterministic supporting
slice unless an actual protected-domain change appears.
