# CE1 — adversarial design analysis, wave 1

2026-10-11. Finite verification packet under [CORPUS-1000](../corpus-testing.md).
Execution authorization: originating CE1 dispatch explicitly requests autonomous
planning, tests, triage and current-master checkpoint/push. No nested delegation.
Outcome: **two confirmed unresolved defects**, with three executable failing repros.
Product implementation remains unchanged. These synthetic test vectors contribute
**zero** real UI captures, generated images or accepted blueprint corpus cases.
Consumer: root's shipping repair packets before wider real-image batches.

## Scope and Spec Basis

Write set: [engine tests](../../../../crates/engine/tests/design_corpus.rs),
[export tests](../../../../crates/export/tests/design_corpus.rs), this receipt only.
Current branch at start: master; HEAD `9734e6147bc6635b8a2b81fe5ae2b1d88c4bcecd`.
Assigned product baseline `c63b600`; engine/export source diff from that baseline
was empty during verification. Untracked `after-title-spacing.png` was not opened
or changed. No product/source/specification change, new branch or worktree.

Traversal: global AGENTS → implementation/product-truth/QA governance → repository
AGENTS → [registry37](../../../specs/README.md) → product/README →
[DESIGN-ANALYSIS@1](../../../specs/product/design-analysis.md).
Selected product closure: ANALYSIS@2, ANALYSIS-TYPES@1, ANALYSIS-VALIDATION@1,
GEOMETRY@1, MODEL@1, EXCHANGE@2, IDENTITY@1, PRIVACY@1, PROJECTIONS@1,
BOUNDARIES@1; EXPORT@2, CLI-EXPORT@2, CLI@16; DRAWING-PACKAGE/STYLE/GEOMETRY/
PROMPT-A/PROMPT-B/REVIEW@2, DRAWING-EXAMPLE@1, and DRAWING@1.2 HBP-HUMAN-001
including R2/R3. These are 21 selected product/reference leaves plus the drawing
amendment. Older dependency-link labels resolve to the registry's current leaves.
Engineering: RUST.md → DEV.RUST@2 → D01@1/D07@5 and explicit development closure
(BOUNDARIES, ROADMAP, RUST-BOUNDARIES, REUSE, decision authority/evidence).
Global product-truth core/routing/evidence/delivery/coordination were applied.
Excluded: collection, actions, Native runtime, comparison-specific CLI clauses,
new product choices, real source discovery and visual acceptance.

Required behavior comes from DESIGN-ANALYSIS.INPUT/TRUTH/OUTPUT/LIMITS and the
selected shared clauses, not from current tests. Test implementation reuses the
existing canonical width golden and observed-brief envelope through public engine
and export APIs; expected arithmetic/status literals are authored independently.
Observed implementation evidence: engine design.rs, arithmetic.rs and analysis.rs;
export design.rs, observed.rs, validate.rs, compile.rs, package.rs and types.rs;
existing design tests. No external code or dependency adopted.
Plan executed: small parameterized engine vectors, export boundary/attribution
vectors, focused target runs, defect triage, this receipt and exact-path checkpoint.
No contract delta or user decision is needed to repair the two mismatches below.

## Results and coverage

21 test functions: engine 10 (9 passing, 1 explicit unresolved repro); export 11
(9 passing, 2 explicit unresolved repros). Parameterized rows exercise:

- Exact equal/ratio/balance, fractional remainder, signed gaps, zero denominator
  including negative zero, extreme finite operands and typed overflow.
- Majority/tie/no majority, exact fractions that round alike, original exception
  indices and negative deviations, genuine 4/4/4/5 and negative-gap families.
- Nine independently mutated binding fields, source snapshot immutability,
  unknown/unsupported/redacted in all three operand positions, missing targets,
  height vs width, units/frames/axis/arity, unrelated same-unit result spaces.
- Same/different observations, zero/nonzero reported uncertainty, missing,
  mismatched and satisfied selected-platform conditions, literal duplicates and
  semantic duplicates disguised by an irrelevant anchor fraction.
- Eight sourced check comparisons/tolerance cases with declared exceptions;
  comparison/expected/tolerance/aliased expected_from survive and fail is retained.
- Exact machine values vs approximate human text, safe alias equality across
  queries/evidence/evaluation/scene, two-view isolation, private method/reason/ID
  canaries, withheld unreviewed visible text, eight unsafe annotation mutations.
- Exact input and aggregate six-file output bounds, request count cap, old empty
  request byte compatibility/version, and rejection of injected computed results.

These are deterministic API checks, not CLI binary execution, collector accuracy,
real screenshots, generated-image QA, whole-workspace acceptance or release proof.
Normal CI does not assert defective behavior as correct: known failing tests use
explicit `#[ignore = "CE1-... unresolved ..."]`, retain the intended assertions,
and are run separately below. Repair owners must remove the ignores after repair.

## CE1-001 — duplicate width fabricates majority (high priority)

Contract: DESIGN-ANALYSIS.INPUT/TRUTH requires at least three **distinct**
measurements. Two controls have widths 4 and 5. Query the first control twice with
identical frame/space/axis, changing only query ID and anchor fraction from 0.5 to
0.0; query the second once. Width arithmetic ignores fraction.
Expected: reject fewer than three distinct measurements; no recommendation.
Actual engine: candidate 4, support 2, exception `(2, +1)`.
Actual export: `possible_inconsistency`, “2 of 3 selected measurements are 4 CSS px”
and a recommendation about making the group consistent. It reaches human output.
Owner: `crates/engine/src/design.rs::pattern`, whose duplicate comparison checks
whole anchors; `crates/engine/src/arithmetic.rs::calculate_rects` confirms Width
ignores fraction. Export `design.rs::compile` exposes the engine result.
Classification: implementation defect, not new pattern policy. Repair must compare
measurement identity according to the operation while preserving valid distinct
measurements and original queries/evidence. Height shares the same ignored-fraction
arithmetic path; current minimal reproductions use Width.

Repro tests: engine `duplicate_width_with_different_fraction_cannot_manufacture_majority`;
export `duplicate_width_does_not_ship_a_false_recommendation`.

## CE1-002 — ratio displays negative zero (low priority)

Contract: DRAWING@1.2 HBP-HUMAN-001 explicitly renders signed zero as 0 while exact
machine facts remain unchanged. Three width-10 controls at x=0,10,16 give gaps
0 and -4; their ratio is IEEE -0.0.
Expected human summary: `0 times as large`; retain exact machine quantity.
Actual: `0 CSS px relative to -4 CSS px: -0 times as large.`
Owner: `crates/export/src/design.rs::ratio_label`; its separate three-decimal
formatter retains the minus sign. Ordinary dimension display already normalizes it.
Classification: presentation implementation defect. No arithmetic/tolerance change.
Repro: export `signed_zero_ratio_is_printed_as_zero`.

## Actual checks and handoff

- `cargo test --locked -p uiblueprint-engine -p uiblueprint-export --test design_corpus`:
  exit 0, 18 passed / 3 explicitly ignored; no full workspace suite.
- `cargo test --locked -p uiblueprint-engine --test design_corpus -- --ignored`:
  exit 101, exactly CE1-001 failed with candidate/support/exception above.
- `cargo test --locked -p uiblueprint-export --test design_corpus -- --ignored`:
  exit 101, exactly CE1-001 and CE1-002 failed with quoted human summaries.
- `cargo clippy --locked -p uiblueprint-engine -p uiblueprint-export --test design_corpus -- -D warnings`:
  exit 0. Targeted rustfmt check and `git diff --check`: exit 0.
- Receipt local links verified; no logs or temporary image files created.

Residual: both product defects are unresolved by this test-only packet. Next owner
needs engine duplicate identity repair and export zero display repair, then the
three repros must pass without ignoring them. CLI/live/image acceptance remains
separate. Checkpoint identity and push result are returned in the terminal receipt;
Git publication uses the assigned nonblocking fcntl lock and an initially empty index.
