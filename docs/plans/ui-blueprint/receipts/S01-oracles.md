# S01-oracles receipt

- Status: `review` — corpus authored and checked; root granted the exclusive
  commit-only lease. Saved by this path-limited checkpoint; not product acceptance.
- Consumer: S01 Integration maps facts into its sole wire schema; G01/K01 later
  consume geometry/replay expectations. This packet adds no production capability.
- Branch: current `master`; initial observed HEAD `7765e7a`; no branch/worktree.
- Authority: [finite packet](../packets/S01-oracles.md), approved `PLAN.UIB@1` and
  actual launch/parallelization recorded by root in the [registry](../task-registry.md).
- Basis: C01 `42d2e6b`, T01 `28a08d3`, candidate schema `0.1.0`.

## Deliverable and exact checkpoint write set

- [README](../../../../fixtures/golden-oracles/README.md)
- [inputs.json](../../../../fixtures/golden-oracles/inputs.json)
- [expected.json](../../../../fixtures/golden-oracles/expected.json)
- [sources.json](../../../../fixtures/golden-oracles/sources.json)
- This file: `docs/plans/ui-blueprint/receipts/S01-oracles.md`.

97 paired case IDs; 12 separately covered valid/invalid envelope families;
GOLDEN01 complete synthetic path and every named negative; D03/D04 availability,
identity, context, coverage, clock, replacement/history and form distinctions;
20 geometry cases with explicit inputs, tolerances and expected values/unknowns.
Every case records clauses, input facts, operation, answer, evidence requirements,
protected distinction and invalid reason where applicable. Inputs and answers are
separate; oracle notation is explicitly not wire schema. 16 CONTENT clauses map
back to source nodes and case IDs. All data is synthetic.

## Traversal and revision handling

AGENTS → specs/README → decisions/README → handoff and D03/D04/D05;
product/README and acceptance/README → GOLDEN → explicit dependency closure.
32 spec nodes were read completely; revisions, paths and resolved size are in
sources.json. Supporting reads include the approved plan, execution route, finite
packet/registry and RUST required through handoff dependencies. No Rust work ran.
D07 concurrently advanced to @2 for S01 schema-tooling dependencies; both changed
D07/decision-route files were reread. D03/D04/D05/GOLDEN and semantic CONTENT
remain unchanged from C01. No oracle semantic answer depends on the D07 delta.
Excluded: crates/schema and plugin-api implementation/tests, actual wire goldens,
real-case answer keys, runtime/browser/native work, full DrawingBrief and mobile.
Requirement source is the contracts; only literal synthetic scenario parameters
and the local oracle notation are author choices. No product delta was made.

## Validation and remaining boundary

In-memory Python checked strict JSON/duplicate keys, unique paired IDs, all family
pairs, required negative coverage, clause/path/revision mappings and arithmetic
for 11 numeric groups, plus the literal selected-field replacement facts. These
checks passed; all 11 local Markdown links and node lengths passed. Task-owned
whitespace validation passed. No temporary raw evidence was persisted. No Rust suite,
product validator, geometry engine, cache replay or real delivery was tested.
This author's arithmetic/consistency checks are not independent product acceptance.

No product ambiguity requires a user decision. Ratio/area retain their mathematical
meaning and source-length units; Integration owns wire encoding, without adding
unapproved geometry-length enum variants. Runtime/provenance truth checks require
stated contextual evidence; structural schema validation alone cannot prove them.
Commit-only scope: master verified, index initially empty, exactly the five paths
above staged. Lease releases with terminal SHA receipt; root passes the corpus to
Integration. Semantic answers unchanged. Later wire mapping, engine checks,
D02/D05 proofs and live acceptance remain their assigned packets.
