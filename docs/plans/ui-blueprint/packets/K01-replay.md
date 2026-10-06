# K01 controlled-data atomic replay

- Class: shipping_product; finite Core owner; model/reasoning inherit.
- Consumer: actual K01 cache/replay and later L01 changes/diff integration.
- Authority: approved PLAN.UIB@1 explicitly permits starting cache on GOLDEN01
  before live acceptance, plus direct user parallel-work request.
- Ready basis: schema9d2df15, saved engineb1475c8/membership84a87a6, independent
  oracle corpusd539a0e, R0326a19fa. Current master; no new branch/worktree.
- Economy: implement one needed pure replay operation now; retention/memory policy
  stays dependent on D05 rather than building a throwaway cache or waiting idle.

## Spec Basis and scope

AGENTS → spec registry → CACHE@1, EXCHANGE@1, PROJECTIONS@1, LIFECYCLE@1 and their
MODEL/IDENTITY/BOUNDARIES/PRIVACY dependencies; D03/D04/D05@1, GOLDEN@1 and full
explicit dependencies. Reuse current complete G01/L01 closure, RUST.md/DEV.RUST@2,
D01@1/D07@2 and relevant R03 update/replay source ledger and independent vectors.
Read missing/changed nodes only. Exclude runtime adapters/input, export, projection
heuristics and D05 production-policy selection. No semantic spec change.

Finite Stage A result: pure canonical Base Snapshot + Delta → new Snapshot or
typed resync/error, with original base unchanged on failure. Use existing shared
types/validation; no second wire schema, foreign engine or graph field vocabulary.
Match complete context/base revision, replace whole selected-field node records,
apply only explicitly justified removals, and publish nodes/children/relations/
focus together. Retain required evidence/observations and historical origin;
unknown/redacted/unsupported updates cannot inherit old known values as current.
Scope/projection/coverage limitations must not become invented deletions.
Reject incompatible/unresolved references without partially mutating the base.

This is actual replay logic, not a cache simulator. No eviction implementation,
TTL freshness claim, memory defaults, subscription, IO or platform work in this
slice. D05 must define accounting/budgets before the bounded cache phase. Return
any concrete storage/policy dependency instead of inventing it in replay.

Five existing review defects remain awaiting explicit user repair authority.
Do not edit schema/plugin validators or their tests to repair those findings,
or label them fixed by the new engine. Implement normal replay requirements and
compare independent full-state oracles in engine tests; do not build a replacement
validator to work around the protected owner. A shared API gap goes through root.

## Exact ownership and verification

Writes: crates/engine/src/replay.rs or a replay/** module directory;
crates/engine/src/lib.rs only to export this owner; crates/engine/tests/replay.rs;
docs/development/replay.md; docs/plans/ui-blueprint/receipts/K01-replay.md.
No changes to arithmetic/resolve/G01 tests, CLI, schema/plugin API, root Cargo/lock,
fixtures/oracles or platform code. Existing engine membership/dependencies suffice;
report any real dependency rather than updating the shared manifest yourself.

Independent tests: valid full/delta equivalence for the same recorded source state,
context/base mismatch, full-node replacement including known→unknown, justified
removal vs scope/virtualization, atomic children/relations/focus and unchanged base
on rejection. Reuse R03/U1/U2 and relevant golden-oracles as evidence, not expected
values calculated by the implementation. No live capture equivalence claim.
Scoped Rust1.96.0 --locked check/fmt/Clippy/tests, no unrelated suite. Real functions
and tests, no empty placeholder modules. Keep public APIs/errors documented.

Git lease from root only when checkpoint-ready; commit and push only own paths,
return exact SHA/push/checks/limits and release. No nested agents/chats/goals,
other projects, runtime lane or unrequested next packet. This finite replay slice
does not complete K01/S01 or accept the protected review gaps.
