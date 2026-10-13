# USE-MAC-COMPARE1 — external recorded comparison plan

2026-10-13. Finite Core investigation and implementation proposal, not delivered
comparison functionality. Only this receipt is writable in this packet.

## Authority and Spec Basis

The [current ownership record](../task-registry.md) assigns Core this source-backed
plan. [USE-MAC1](corpus-native-advice.md#use-mac1--actual-consumer-comparison-feedback-2026-10-13)
records consumer turn `01a134b3-eca4-7ba2-b982-2e72d8e5c61e` in Mac chat
`01a1102f-791c-7e91-bec3-1877ea004d51`. Root subsequently relayed the user's direct
instruction: “пусть сделает нормальное сравнение двух внешних снимков и понятную
диагностику отказа. Когда исправит, повтори этот же кейс у себя, без изменений
приложения”. The user delegates the technical solution to UI Blueprint's owner.
This packet still stops at the concrete plan; root will issue implementation.
No changes to PlayPhrase.me, its bootstrap, runtime, profile or source project.

Traversal: repository/global AGENTS → implementation/product-truth/QA governance →
[registry41](../../../specs/README.md) → [product routes](../../../specs/product/README.md)
→ CLI18 → CLI-DIFF2, CLI-GRAPH-DIFF1, CLI-GEOMETRY-DIFF1; complete dependency
closure CACHE1/IDENTITY1/MODEL1/EXCHANGE2/PRIVACY1/GEOMETRY1/BOUNDARIES1/
PROJECTIONS1/FORMS1/LIFECYCLE1 and ANALYSIS2/TYPES1/VALIDATION1. NATIVE5
AX-READ governs ordinary observation-scoped bindings. Existing fixture-session
assessment is retained: NATIVE-SESSION3 is a native_fixture workflow, not a
generic external AX session or an app-integration prerequisite.

Compare-export added the explicit CLI-EXPORT3.COMPARE → EXPORT2/drawing closure:
PACKAGE3, STYLE2, GEOMETRY2, PROMPT-A2/B2, REVIEW2, EXAMPLE1 and the linked
DRAWING1.2 human-presentation amendment. Selected routed files total 1,913 physical
lines, plus that amendment and governance; no full original specification preload.
Some dependency/navigation labels still name older additive CLI/EXCHANGE/export
revisions; current leaf headers govern. Compare-export meaning is unchanged by
CLI-EXPORT3's single-view detail additions. No revision conflict affects this plan.
Excluded: live actions, popup/visible/static-text collection changes, design-analysis
requests, new export/detail presentation, Web first-use work and future platforms.

Contract requirement: existing diff uses exact keys and compatible session/Surface
bindings; failed compatibility gives constant stderr and empty stdout. IDENTITY1
permits explicitly marked diff candidates, without action authority. Observed
implementation matches those constraints. Disposition: missing supported external
comparison capability, not a proved defect in strict diff. The new representation
below is an agent proposal for root's Evolve selection under the direct user request;
it does not derive authority from this receipt or change an Active contract now.

## Exact evidence and cause

Original inputs, untouched:

```text
/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CN-B19-bway_om4/B19-01-minimal/observed.json
/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CN-B19-bway_om4/B19-02-minimal/observed.json
```

| Fact | Before | After |
| --- | --- | --- |
| Bytes | 477244 | 476851 |
| SHA256 | `8de14b5f3956b39654810a83b9873ac9d1dc0c51ad12e84f02a4d58f93ad7074` | `3e57cfb3973b36d52af5bf102fb1b33a6821356c0f1335f9e5379d8e217c827e` |
| Observation prefix | `16ddacd65f5641afb255f9a7a8de85cc` | `8c88b84352074b20bc73fe1c17d6195e` |
| Nodes / relations / components | 160 / 0 / 0 | 160 / 0 / 0 |
| Coverage / consistency | partial / unknown | partial / unknown |

Both are core0.1 observed ChannelResponses, revision1. Matching Context axes:
schema0.1, plugin macos0.1, target `macos-pid-90325` generation
`90325:1791839565.812851`, scope `CN-B19-focused-ax`, design projection and fields
role/accessibility_name/accessibility_bounds. Different axes: environment_revision,
session_id and surfaces. Environment changes are already permitted by recorded diff;
session and Surface changes cause the refusal. SourceKey intersection is **zero**.
Observation clocks differ too; neither capture chronology nor clock continuity follows.

`WindowAX.swift::collectWindowAX` generates `macos.ax` keys from Observation ID
plus traversal handle index. Each saved Surface is `ax-focused-REQUEST_ID` with
generation REQUEST_ID. Held AXFocusedWindow identity was checked within each Observe,
not across requests. Identical PID/incarnation and frame do not establish the same
window across files. Removing the Context guard would merely produce absent-side
nodes; rewriting keys or matching equal ordinal indices would invent continuity.

Public nonempty AXIdentifier counts: before111 nodes/90 distinct/88 unique; after96/
75/73. There are75 shared values,73 unique on both sides and2 duplicate values.
These are candidate evidence only: reuse/remount remains possible. One unique-ID
candidate has before node40 / after node36, both AXButton, with reported rects
`(764,552.5,392,36)` → `(764,480.5,392,36)` in each side's ax-screen/pt/top_left.
Each side's node0 AXWindow rect is `(0,30,1920,1050)`. No identifier-based mapping
has been promoted to source identity or approved app semantics.

Source owners inspected: engine `diff.rs::{compare_recorded,sources_compatible,
compare_graph,compare_geometry,resolve_geometry,same_content,same_evidence}`;
CLI `main.rs::{execute_diff,diff_failure}`, input/arguments/output; Native
`tests/bridges/native/WindowAX.swift::collectWindowAX`; export
`compare.rs::compile`, `validate.rs` and CLI `export.rs::load_brief`. Existing
engine diff, CLI graph/geometry/analysis and export observed-compare tests establish
the relevant regression owners; no suites were run for this documentation packet.

## Existing paths and bounded repros

The consumer's built/verified Native bundle3e995ec, CLI SHA
`eaa10d2d7bd53c8c64a80b075f5ca7acb5f8887479649997c38268919e4ba883`, returned
raw diff exit4 / empty stdout / `context_mismatch`. That bundle was already removed.
Additional local repros used the existing `target/debug/uiblueprint`, SHA
`a4e9af497f060094cb3eca31a6afc362d1c7258e79d3837f7551aaaf024a55ed`;
its source/build provenance is unestablished, so these are supplemental diagnostics,
not release acceptance. Raw, graph and geometry each returned4, zero stdout bytes,
`context_mismatch\n`, with input/output caps2097152 and raw/graph entry cap30.
Geometry selected the original before node0 key, accessibility_bounds, ax-screen.
Both original hashes remained unchanged.

No current command closes the requested paired workflow:

- `diff`, `diff --graph`, `diff --geometry`: same compatibility gate and exact keys.
- `imagegen-prompt --before/--after`: uses engine compare_graph. A supplied
  `different_basis` allows a side-by-side package marked `incompatible_context`,
  empty entries and `not_compared`; it creates neither mapping nor useful diff.
  This conclusion is source/contract-backed; no package or image was generated.
- `inspect`: exposes each side using its own key; no cross-observation matching.
- `measure`: works independently with each side's original keys and bound evaluation.
  Two actual local calls for window-top → candidate-button-top gap returned known
  522.5pt and450.5pt, exit0, with their separate original Evidence. This supplies
  useful operands but does not provide the utility's requested correspondence/report.
- `native-session`: fixture-bound refs do not address these ordinary AX files.

Supported per-side measurement syntax is documented in ANALYSIS2:
`measure --snapshot S --query Q --space ax-screen --max-input-bytes 2097152
--max-output-bytes 2097152 --json`. Each Q is analysis0.2 GeometryQuery: operation
gap, length/pt, axis y, both fractions0, accessibility_bounds, targets node0 and
node40 before/node36 after, actual scope, null applicability conditions. Query files
were generated in a unique system-temp directory, then individually removed;
empty directory removal and absence were verified. No images or application state
were created, no original observation was edited, and no build was started.

## Recommended minimum implementation

1. **Register the additive contract first.** Introduce a routed comparison leaf
   and update registry/CLI routing before code. Preserve CLI-DIFF2, GRAPH1 and
   GEOMETRY-DIFF1 verbatim. New command keeps old JSON/stderr behavior unchanged:

   ```text
   compare --before B --after A --correspondence C --max-input-bytes N --max-output-bytes N --max-entries N [--json]
   compare --before B --after A --diagnose --max-input-bytes N --max-output-bytes N [--json]
   ```

2. **Accept explicit correspondence, never restamp.** C is a CLI-local strict
   map-only version1 input, rejecting duplicate/unknown fields and array records.
   Fields: correspondence_version, before_binding, after_binding, surface_pairs,
   node_pairs, measurement_pairs. Each binding contains the existing canonical
   Snapshot id/revision/full Context and must equal its own source exactly.
   Surface pairs contain original before/after Identity and caller declaration;
   node pairs contain bounded pair ID, original before/after SourceKey and caller
   declaration. Require one-to-one, unique, existing endpoints, matching namespaces,
   and nodes belonging to their explicitly paired Surfaces. Initial scope retains
   equal schema/plugin/Target incarnation/scope/projection/field sets and explicitly
   pairs the full Surface sets. Only declared session/Surface differences are
   admitted. Caller declaration is `caller_declared_unverified`, not Evidence,
   stable identity, confidence, an automatic AXIdentifier match or an action ref.

3. **Compare selected facts in the engine.** Add `diff::compare_corresponded`
   with borrowed originals and fallible bounded entries, reusing same_content /
   same_evidence. Compare mapped nodes' literal properties/availability, preserving
   separate original keys, Surface, Evidence/Observation and source coverage.
   Return properties with independent content/evidence flags and original per-side
   indices; map order then source field order is deterministic. Retain unselected
   node counts as unexamined, never created/deleted or unchanged. Full graph/focus/
   components/relations, automatic candidate generation and canonical Delta are
   outside this first implementation.

4. **Make layout QA useful without guessed coordinate continuity.** Each optional
   measurement pair contains ID and two existing GeometryQuery/EvaluationInput
   records, strictly bound independently to each Snapshot. Initially support scalar
   width/height/gap; require corresponding mapped target/anchor order, same operation,
   frame kind, fractions/axis/applicability and quantity units. Reuse
   `measure_query_bound`; both calculations retain their own full Spaces and Evidence.
   Compare two known compatible scalar lengths with finite after-minus-before;
   preserve either unknown reason and emit null change otherwise. A window-to-control
   edge gap is calculated inside each source, so comparing those lengths needs no
   inferred common screen origin. Report this as a difference between declared
   corresponding measurements, not verified object motion or global layout cause.
   Literal frame properties remain attributed on both sides. Do not compute cross-
   observation dx/dy, infer window-local transforms or promote equal ax-screen IDs
   into a verified common coordinate system. Existing selected-space geometry diff
   retains its stronger requirements. The exact candidate above supplies a useful
   522.5→450.5pt gap, change−72pt, conditional on the caller's explicit correspondence.

5. **Ship bounded diagnostics and the usable command together.** `compare --diagnose`
   validates both inputs and emits a separately versioned `comparison_diagnostic`
   report: fixed differing-axis enums, node counts, exact-key intersection count and
   fixed supported-next-step codes (`explicit_correspondence_required`,
   `per_side_measure_available`, or incompatible protected basis). No raw paths,
   UI labels, AXIdentifier strings or decoder text in diagnostics. For this pair:
   environment/session/surfaces differ; only session/surfaces block old diff; exact
   matches0. Explain caller-declared matching and partial scope in compact output.
   Successful comparison uses a new `corresponded_difference`1.0 envelope retaining
   original before/after Snapshots and correspondence, selected entries/measurements,
   unexamined/omitted counts, source=saved, live_revalidated=false and limitations.
   No changes to old report schemas or failure text. Aggregate input budget covers
   B+A+C; complete output including newline is bounded before publication. Report0
   means complete factual comparison, not normative QA pass; unknown/truncation or
   diagnostic incompatibility4, malformed/binding/limits2, IO/internal1. Errors remain
   constant sanitized stderr with no prepublication partial stdout.

## Owners, verification and handoff

Future write set: engine `src/diff.rs` and focused `tests/diff.rs`; CLI
`src/{arguments,input,main,output}.rs` and a focused correspondence binary test;
new routed comparison contract, specification registry/CLI route;
`docs/development/diff.md` and the implementation receipt. No schema/wire/Cargo/
dependencies/Native collector/fixture/app/cache/action changes are needed. Root
must serialize shared CLI/spec paths against current owners. README/packaging edits
belong to the separate first-use owner; link the comparison instructions through
that handoff, not an overlapping edit here.

Acceptance for the future selected implementation:

- Exact original pair still fails old raw/graph/geometry with4/empty stdout and
  unchanged constant stderr; original hashes and embedded canonical source equality.
- New diagnostic names actual axes and supported next step, without private/UI text.
- Explicit node0→node0 and node40→node36 declarations produce selected property
  changes and independently recomputed gap522.5→450.5pt/change−72pt with both Evidence;
  caller-declared/unverified correspondence and partial coverage remain visible.
- Missing mapping, reused binding, duplicate/ambiguous/dangling endpoints, wrong
  namespace/Surface/Target incarnation or protected Context mismatch refuse safely.
- Candidate IDs/names/boxes and same ordinal never authorize automatic matching;
  duplicate AXIdentifier values remain unresolved without an explicit declaration.
- Known→unknown/redacted/unsupported/not_requested/empty remain distinct, with null
  numeric change when measurement is unavailable; missing transform and evidence-only
  changes preserve their reasons. No common-clock, created/deleted or action claims.
- Entry cap0/small, aggregate input boundary, full output boundary, duplicate/unknown
  argument/input members, nonfinite calculation and escaped untrusted compact text.
- Focused new engine/binary tests plus existing affected raw/graph/geometry/analysis
  regressions; no full live suite or ImageGen. Build a pinned consumer bundle via the
  existing distribution path, then the Mac consumer retries the same untouched pair
  and documented new command without changing or launching the app for this check.

No additional user decision is missing: the user delegated the technical comparison
solution. Root selection and an implementation packet are the next authority step.
This receipt is the plan deliverable only; no new feature, consumer retry, independent
acceptance or released comparison capability is claimed. Documentation verification:
local links/route references and diff whitespace check; checkpoint contains only
this receipt. Unrelated working-tree changes and after-title-spacing.png stay untouched.
