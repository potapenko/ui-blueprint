# R03: typed updates, geometry and projections

Diagnostic / Discover; bounded execution of [R03](../plans/ui-blueprint/packets/R03.md)
and [research-common](../plans/ui-blueprint/packets/research-common.md).
Immediate consumers: C01 D03/D07, S01, G01 and K01. No shipping capability delivered.

## Authority and traversal receipt

Actual user launch and fixed `PLAN.UIB@1` revision `358c757e7eab84a3989d150dbad57924d866601a`
are recorded in the [registry](../plans/ui-blueprint/task-registry.md).
C00 candidate `92d2bf89ea9acab091fd166eef4432408e353744`, independent acceptance
`9bedeccbf4c5a92206d24d0794b299e54a2b22e1`. Research began on master at
`26d62fd47c95259effee3560c56a52c2c9bfe418`; coordination base was `7c48392`.

Read order: AGENTS → [registry](../specs/README.md) (`UIB.ROUTING@1`) →
[R03 route](../specs/reference/research-routes.md#r03) (`UIB.RESEARCH-ROUTES@1`,
`UIB.RESEARCH-ROUTES.ROUTE`) → the following full leaf closure, in this order:

1. [BOUNDARIES](../specs/product/boundaries.md), [REUSE](../specs/reference/reuse.md),
   [CORE-SOURCES](../specs/reference/core-catalog.md), [PROBE-SOURCES](../specs/reference/probe-catalog.md).
2. [MODEL](../specs/product/model.md), [IDENTITY](../specs/product/identity.md),
   [EXCHANGE](../specs/product/exchange.md), [PROJECTIONS](../specs/product/projections.md).
3. [PRIVACY](../specs/product/privacy.md), [LIFECYCLE](../specs/product/lifecycle.md),
   [CACHE](../specs/product/cache.md), [GEOMETRY](../specs/product/geometry.md).
4. [ROADMAP](../specs/product/roadmap.md), [PILOTS](../specs/acceptance/pilots.md).

Every listed leaf is `UIB.<ID>@1`, clause `UIB.<ID>.CONTENT`. Fourteen leaves
resolve the explicit cross-domain dependencies; no further product closure needed.
Supporting routes read: global implementation, product-truth core/routing/worker
coordination, QA and external timeout governance; local [RUST](../../RUST.md),
[DEV.RUST@1](../specs/development/rust.md), finite packet and receipt format.
Excluded: live adapters, product-specific N/W profiles, CLI/export, DrawingBrief,
ML/mobile, F1–F4 and GOLDEN01 implementation. No discovered contract conflict or
semantic revision drift. Catalog assertions are historical; observations below
come from fresh reads of pinned upstream code and adjacent tests on 2026-10-06.

Specified: status/selection/provenance remain distinct, removals require evidence,
delta context must match, geometry needs spaces/units/transforms, projection does
not destroy graph data. Observed: the upstream implementations below have narrower
assumptions. Proposed: the concrete representation/validation strategies below.
No contract delta; D03/D07 remain C01 decisions, shared types remain S01's owner.

## Source ledger

Read the named symbols/mechanisms and cited adjacent test cases; this is not an
audit of every line of each repository. All source links are immutable revisions.
Upstream tests were read, not executed. No upstream code, assets or tests copied;
the small Rust fixture is independently written against our contracts.

### A — AccessKit

Revision [466e24e252103f4d82bd6fbab98141b551e46e0c](https://github.com/AccessKit/accesskit/tree/466e24e252103f4d82bd6fbab98141b551e46e0c).
Entry [accesskit/src/lib.rs](https://github.com/AccessKit/accesskit/blob/466e24e252103f4d82bd6fbab98141b551e46e0c/accesskit/src/lib.rs):
NodeId, Node/Properties equality, TreeUpdate, ActionRequest and adjacent property
setter/clearer examples. NodeId is local to TreeId. TreeUpdate includes full
replacement nodes, optional tree metadata and required current focus. Removing
a child from its parent's list removes its unreachable subtree under this
producer-owned tree contract. There is no UI Blueprint base-revision or fields
envelope in that update.

Consumer [accesskit_consumer/src/tree.rs](https://github.com/AccessKit/accesskit/blob/466e24e252103f4d82bd6fbab98141b551e46e0c/accesskit_consumer/src/tree.rs):
TreeState::update, pending children/nodes/grafts, unreachable traversal,
compute_effective_focus, Tree::update_and_process_changes and process_changes.
The algorithm resolves order-independent nodes/parents, tracks reachability,
updates next_state, compares old/new focus, emits change callbacks and synchronizes
state. It assumes structurally valid producer data; violations panic. Its
next_state is mutated before final validation, so it is not our recoverable
validate-then-publish transaction for untrusted deltas.

Adjacent tests read in that same file: `remove_child_from_root_node`,
`move_focus_between_siblings`, `reparent_subtree_by_removing_old_graft`.
They exercise parent update/removal, focus-only update and graft ownership move.
Transfer: typed identity, full replacement and coherent old/new graph comparison.
Do not transfer implicit reachability deletion to partial external observations,
panic recovery, one-tree semantics, camelCase wire types, or action authority.
Decision: independently implement the selected contract; do not add AccessKit as
an engine dependency merely because it is Rust. Fixtures U1/U2 below; S01/K01.

Selected conditions: lib.rs has AccessKit MIT-or-Apache headers **and Chromium
derivation notice**; tree.rs has MIT-or-Apache header. Inspected root
[LICENSE-MIT](https://github.com/AccessKit/accesskit/blob/466e24e252103f4d82bd6fbab98141b551e46e0c/LICENSE-MIT),
[LICENSE-APACHE](https://github.com/AccessKit/accesskit/blob/466e24e252103f4d82bd6fbab98141b551e46e0c/LICENSE-APACHE)
redistribution terms and [LICENSE.chromium](https://github.com/AccessKit/accesskit/blob/466e24e252103f4d82bd6fbab98141b551e46e0c/LICENSE.chromium).
A future copy must retain applicable notices, including Chromium source/binary
conditions and non-endorsement; MIT OR Apache alone is not a complete file audit.
Root/schema/consumer Cargo manifests read: schema uses uuid and optional
serde/schemars/pyo3/enumn; consumer uses accesskit/hashbrown. None added here.
No NOTICE-named file found in the complete pinned Git tree.

### G — Galen relations and range arithmetic

Revision [6c7dc1f11d097e6aa49c45d6a77ee688741657a4](https://github.com/galenframework/galen/tree/6c7dc1f11d097e6aa49c45d6a77ee688741657a4).
Under `galen-core/src/main/java/com/galenframework/`, read
[SpecValidationInside](https://github.com/galenframework/galen/blob/6c7dc1f11d097e6aa49c45d6a77ee688741657a4/galen-core/src/main/java/com/galenframework/validation/specs/SpecValidationInside.java),
`validation/specs/SpecValidationNear.java`, `validation/specs/SpecValidationComplex.java`,
`validation/MetaBasedValidation.java`, `validation/SpecValidation.java`,
`specs/Range.java`, `specs/RangeValue.java`, `speclang2/specs/SpecAlignedProcessor.java`.
Signed edge subtraction plus inversion selects side distances; the Range holds
check handles open/inclusive bounds. Alignment parser validates physical sides
and accepts explicit error rate. Complex validation resolves both objects first.

Important differences: inside permits up to 2 px corner escape unless `partly`;
RangeValue truncates comparison values to declared decimal precision, so an
integer exact 10 can accept 10.99999. SpecValidation may throw availability errors
under global visibility checking; it does not model our independent unknown axis.
Read [RangeTest](https://github.com/galenframework/galen/blob/6c7dc1f11d097e6aa49c45d6a77ee688741657a4/galen-core/src/test/java/com/galenframework/tests/specs/RangeTest.java)
including `holdsData`, and `tests/validation/InsideValidationTest.java` good
samples and initial bad samples: full/partly inside, percent ranges and explicit
2 px overlap case. These source tests confirm those semantics, not our defaults.
Decision: reimplement signed relations and explicit ranges against our contract;
reject hidden 2 px allowance, precision truncation as tolerance, and Java/Selenium
runtime. Fixtures G1/G2; G01. Bounding boxes alone establish no paint occlusion.

Selected Java files carry Ivan Shubin Apache-2.0 headers; inspected
[LICENSE-2.0.txt](https://github.com/galenframework/galen/blob/6c7dc1f11d097e6aa49c45d6a77ee688741657a4/LICENSE-2.0.txt)
and core POM dependency/license declarations (commons-lang3 and Selenium among
others). No NOTICE-named file in pinned tree. Copying would require license,
attribution and modification notices; no code copied or dependencies adopted.

### E — Galen Extras group rules

Revision [600fb9abc6bea91e6228a93929602f5433908448](https://github.com/galenframework/galen-extras/tree/600fb9abc6bea91e6228a93929602f5433908448).
Read [galen-extras-rules.js](https://github.com/galenframework/galen-extras/blob/600fb9abc6bea91e6228a93929602f5433908448/galen-extras/galen-extras-rules.js),
`galen-extras/galen-extras-rules.gspec` alignment/equal-distance rules,
`examples.gspec`, `rules-tests/alignment.test.js`, `rules-tests/conditional-rules.test.js`.
Table rules expand adjacent and row-offset pairs. Equal-spacing rules derive the
first gap with abs/parseInt then compare subsequent gaps within ±1 px; alignment
defaults to 1 px. Conditional helpers emit no body for empty matches, and examples
switch desktop/tablet/mobile layouts. Tests inspect expanded spec strings.
Decision: adopt explicit ordered-pair expansion as a concept; reimplement under
known coverage and applies_when. Do not infer row order from arbitrary map
iteration, abs away a negative gap, inherit ±1, or turn empty matches into a
successful measurement. Fixture E1; G01. No need for Galen JS runner or eval.

Selected JS/gspec headers and root
[LICENSE-2.0.txt](https://github.com/galenframework/galen-extras/blob/600fb9abc6bea91e6228a93929602f5433908448/LICENSE-2.0.txt)
are Apache-2.0; selected test scripts have no separate header. No NOTICE-named
file in tree. Galen-provided rule/findAll runtime is an upstream dependency,
not a Rust-engine dependency. No copying; future derivative needs Apache notices.

### B — Explicit Compose anchors

Revision [09287ace7ea4a79cb5a1a08521cd69a365f7bdac](https://github.com/popovanton0/Blueprint/tree/09287ace7ea4a79cb5a1a08521cd69a365f7bdac).
Read [BlueprintId.kt](https://github.com/popovanton0/Blueprint/blob/09287ace7ea4a79cb5a1a08521cd69a365f7bdac/blueprint/src/commonMain/kotlin/com/popovanton0/blueprint/BlueprintId.kt),
DSL `Anchor.kt`, `Dimension.kt`, `GroupScope.kt`, `MeasureUnit.kt`; in the same
package `Blueprint.kt`: composable setup, drawBlueprint, getTarget, extendingLine,
calculateStartOffset, dimensionLabel and pxToDimensionLabel. A marker maps a
caller ID to attached LayoutCoordinates after positioning; detached targets are
removed. Anchor fraction 0..1 selects a position along a root-relative box.
Dimension keeps two anchors and a unit, with dp default; Sp is experimental and
documented as not working correctly. Drawing applies line-width corrections.

Read `blueprint/src/androidUnitTest/kotlin/com/popovanton0/blueprint/Dsl.kt`
and screenshot-test functions for fractional dimensions, padding disabled,
global disabled and removed composition children. Tests encode the DSL and
selected rendering expectations; no upstream tests or screenshots executed here.
Decision: adapt the explicit marker/anchor concept, independently implement
normalized measurements. Reject density rounding, default applyPadding=true,
line-stroke adjustment and formatted labels as measurement truth. Blueprint
changes LocalDensity and can add padding: it cannot prove an inert native probe.
Fixtures B1/P1; G01 and R02/P01 handoff only; no runtime lane used.

Selected Kotlin has no extra per-file license header. Root
[LICENSE](https://github.com/popovanton0/Blueprint/blob/09287ace7ea4a79cb5a1a08521cd69a365f7bdac/LICENSE)
is Apache-2.0 and [NOTICE](https://github.com/popovanton0/Blueprint/blob/09287ace7ea4a79cb5a1a08521cd69a365f7bdac/NOTICE)
attributes Anton Popov (2023). Preserve both on a future derivative. Inspected
module Gradle dependencies: Compose runtime/foundation and androidx annotation;
test dependencies include JUnit and Compose test tooling. None adopted.

### P — Compose Preview projections and survivor cache

Revision [95398528a6d5b1f4fa4ce92e1a73f830608eb3fb](https://github.com/GusWard/Blueprint-Compose-Preview/tree/95398528a6d5b1f4fa4ce92e1a73f830608eb3fb).
Read [BlueprintPreview.kt](https://github.com/GusWard/Blueprint-Compose-Preview/blob/95398528a6d5b1f4fa4ce92e1a73f830608eb3fb/blueprint-compose-preview/src/main/java/uk/co/gusward/blueprint/compose/preview/preview/BlueprintPreview.kt):
staticBlueprintCache and initialization/layout/pre-draw callbacks,
extractBlueprintItemsFromSemantics and traverseSemanticsNode. Read sibling
`items/BlueprintItemData.kt`, `grid/logic/BlueprintLineCalculator.kt` and
`src/test/java/uk/co/gusward/blueprint/compose/preview/grid/logic/BlueprintLineCalculatorUnitTest.kt`.
Those tests cover vertical/horizontal connections, parent connections and
blocking line intersections, not many-to-many source identity or cache freshness.

Extraction attempts reflected unmerged semantics, falls back to merged nodes,
chooses inner/outer coordinates based on role, suppresses labeled children and
filters contained items. Bounds are rounded to 0.1 and small items excluded.
Line calculation uses pair deduplication and visual line-of-sight heuristics.
The bounded 50-entry survivor cache only adopts nonempty new maps, so an empty
new composition can retain old contents. Its cache key is a Compose composite
hash, not our target/surface/schema/scope/fields context.
Decision: use as counterexample and visualization reference, not a collector or
cache dependency. Preserve original source nodes; represent suppression as view
membership; distinguish unavailable extraction from confirmed empty scope.
Fixtures P1/P2; S01/K01. Reflection, role-based bounds and rectangle containment
do not prove paint contour, component identity, actionability or full coverage.

Selected Kotlin files have no separate license header. Root
[LICENSE](https://github.com/GusWard/Blueprint-Compose-Preview/blob/95398528a6d5b1f4fa4ce92e1a73f830608eb3fb/LICENSE)
is Apache-2.0 with Gus Ward attribution. No NOTICE-named file in pinned tree.
Inspected module Gradle: Android/Compose/Material/navigation, JUnit/MockK for tests.
No code copied, no reflection/runtime dependency added. Future copying must carry
license, applicable attribution and modification notices.

## Recommendations and own falsifiable fixtures

These are D03/D07 proposals where they choose a representation; normative
behavior comes from the pinned contracts above. Only U1 was executed in R03.
Other rows specify exact original fixtures for the named owners, not passed tests.

| ID / consumer | Independent input and expected result | Proposed implementation boundary |
| --- | --- | --- |
| U1 / S01,K01 | [Executable and oracle](../../experiments/core/README.md): true/old → false/empty; then unknown/redacted/unsupported; wrong base, omitted requested field and changed selection reject without mutation; narrow full leaves old label historical | Availability enum with value only for known; selection separate; replace complete source-node representation within compatible fields/projection; validate before publish |
| U2 / K01 | rev1 nodes root→[a,b], a→[c], focus=c. rev2 root→[b], b→[c], remove a, focus=c. Expected c survives with new parent, a gone, no dangling edge. Negative: same removal with unchanged root/focus pointing to a rejects whole update. Partial response omitting a alone does not delete a | Children/relations/focus publish in one revision; explicit confirmed removals; graph validator distinct from text diff |
| G1 / G01 | css_px in one local space: icon right=20; label left=29; expected gap 8±1 → measured 9 pass. left=29.01 → fail. left=18 → signed gap −2 fail. Unknown label bound → unknown, no value. 10.99999 against exact 10±0 → fail | Signed arithmetic without display rounding; finite values and nonnegative explicit tolerance, expected_from=fixture G1 |
| G2 / G01 | Container (0,0,100,100); child (0,5,102,95). Full inside tolerance 0 → fail (2 escape); explicit tolerance 2 → pass. Same numbers pt versus css_px without transform → unknown/missing_transform. Only tested left/top margins cannot prove full containment | Keep frame kind, space ID, units, origin and transform evidence; no inherited Galen 2 px default; transform all corners for non-axis-aligned cases |
| E1 / G01 | Explicit desktop ordered boxes x=[0,18,37,60], width=10, expected adjacent gaps 8±1 → [8,9,13], last fails. Partial/truncated membership → global equal-spacing unknown. Mobile applies_when selects a separate vertical rule, not desktop ordering. Empty/unavailable list cannot pass measurement | Closed relation operators and applies_when; aggregate only with required coverage; no JS execution or first-gap-derived product expectation |
| B1 / G01 | icon local x=2,w=10; label x=20; parent-to-surface translation 100 css_px; then surface-to-image scale 2 px/css_px. Surface gap=8 css_px, image gap=16 px. Translate surface by 50: local gap remains 8. Missing either required transform → unknown | Anchor=(source key, frame kind, physical edge/fraction); explicit directional transform with from/to spaces and observation. No global scale or drawing-stroke corrections |
| P1 / S01,G02 | AX nodes a1,a2; probe nodes p-container,p-icon,p-label. Explicit reported mapping a1→all three and a2→p-label. Design shows probe parts; interaction shows a1,a2; source graph retains all five. Same bounds without mapping does not create logical_component_key; p-icon receives no actionable ref | Relation records between namespaced IDs, evidence and mapping method; separate view membership. Backend action refs never generated by heuristic match |
| P2 / K01 | rev1 known scope={x}; rev2 complete confirmed empty same scope → x removed by explicit removal. If collection fails instead: coverage unknown, old x only historical/stale. If new projection suppresses x: no graph removal | Separate empty/unknown/partial; no survivor cache promotion or nonempty-only publication |

Minimal communication examples, **not** S01 public type definitions:
`Requested<T> = Known(T) | Unknown(reason) | Unsupported(reason) | Redacted`;
`selection = requested | not_requested`; relation endpoints are source keys,
not merged IDs. Evidence must carry Observation/source/time/method; production
errors and serialization remain S01 choices. Availability, freshness, consistency,
coverage and live/cache source are independent; uncertainty does not disappear
when projection or response formatting changes.

String diff is insufficient: identical label strings can hide different source
identity, parent/focus, generation, provenance or redaction status. Conversely,
text reformatting need not indicate a graph change. Survivor cache is insufficient:
nonempty old data is not evidence of fresh data, and complete empty is meaningful.
Unknown-as-pass is insufficient: no transform/bounds/member coverage means no
measurement proving the expectation, even if upstream quietly omits a rule.

## Result, checks and remaining work

U1 executed with macOS 27.0.1 arm64 and rustc 1.96.0 (ac68faa20 2026-05-25),
edition 2021, no third-party crates. rustfmt check and rustc `-D warnings` passed;
the executable returned 0 and the exact independently specified stdout.
The naive merge counterexample diverged as expected. This accepts only the
bounded experiment. No Cargo/root toolchain/API edits and no upstream runtimes run.

Immediate handoff: C01 reconciles D03 representation and D07 no-copy decision;
S01 owns serializable types/invalid envelopes and GOLDEN01; G01 implements and
checks G1/G2/E1/B1 after accepted S01; K01 implements U2/P2 and full/delta oracle
on one controlled source state. G02 later verifies P1 against real source mapping.
R02/P01 must prove probe off/on inertness separately. No latency gates proposed
or performance results claimed; no Mac/Web runtime acceptance follows from mocks.
Checkpoint and final documentation checks are tracked in the
[terminal receipt](../plans/ui-blueprint/receipts/R03.md).
