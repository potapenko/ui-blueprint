# A02-F — analysis expectations reconciled

2026-10-08. Supporting fixture correction delivered for Q01 affected recheck;
no new product capability or full P7 acceptance claimed.

## Authority and selected basis

The [A02-F packet](../packets/A02-analysis-expectations.md) and this chat's
dispatch authorize the full cycle, scoped checkpoint and push on current master,
under approved PLAN.UIB@1. Single chat; no agents, other chats or messages.
Mode: Restore stale verification expectations to the accepted distinction in
2491dec/256f2a2. No product contract delta, new arithmetic or user decision.
The [Q01 failure](Q01-integrated-acceptance.md) is independent prior evidence;
the execution below is this task's own author verification, not independent acceptance.

Traversal: global/local AGENTS → implementation/product-truth core/routing/change/
evidence/delivery/coordination and QA → specs/README (registry30) → product/README
→ ANALYSIS@2 (SCOPE/VERSION/CLI), ANALYSIS-TYPES@1 (INPUT/RESULT/OWNERS),
ANALYSIS-VALIDATION@1 (BINDING/EVIDENCE/CONTRACT/VERIFY/ACCEPTANCE), GEOMETRY@1;
full explicit closure MODEL/IDENTITY/PRIVACY/BOUNDARIES@1, EXCHANGE@2, CLI@16.
RUST/DEV.RUST@2 → D01@1/D07@5 → ROADMAP/RUST-BOUNDARIES/REUSE@1 and
C01-EVIDENCE/DECISIONS@1. Older dependency labels resolve additive current leaves;
no drift affecting this analysis meaning. These selected nodes were read fully.
Supporting context: packet, approval record in task registry, Q01 receipt, exact
fixture/test/generator/source owners and the accepted source diffs. E03c6b2357
was inspected only for historical-preservation precedent, not numeric authority.
Excluded sibling domains: live Native/Web/UI/actions, export changes, Q02/Q03,
real PlayPhrase.me operations, mobile/F1–F4. No runtime collection or model calls.

## Reconciliation and exact writes

Contract requirement: schema-valid declarations are not recomputed proof;
engine must recompute immutable source/evaluation and reject inconsistent imports.
Observed source: record_evidence refuses explicit Consistency::Unstable only.
Historical unstable-source.json has OG1=unknown; unstable-conditions.json has
OC1=unknown (OG1=stable). Their reason label authored_unstable is not an enum
override. Known rectangles independently imply 48 − (10 + 30) = 8 css_px;
the supplied macos condition matches the query where required. Both stored
results instead claim unknown/unstable_state. Classification: stale evidence,
not a demonstrated product defect or authority to restore Unknown→Unstable.

Minimal implementation choice, declared before edits: change only the two
engine_verification labels from match to mismatch in the existing manifest and
generator, explain schema-vs-engine meaning in the fixture README, add this receipt.
Exact write set: fixtures/analysis/manifest.json, fixtures/analysis/build.py,
fixtures/analysis/README.md and this receipt. Historical JSON/names, production
engine/schema/CLI/host/platform source, Cargo, public formats and spec norms stay.
The unchanged test executes every manifest row, checks exact ResultMismatch,
rejects unclassified result cases and retains independent positive coverage.
No criteria removed, skips introduced or tests mirrored. Existing known-gap8,
Unknown-known dimensions, explicit Unstable geometry/conditions and partial
named-anchor cases suffice; crates/engine/tests/analysis.rs is byte-unchanged.

## Own execution on saved source

Source/build: system-temp git archive of
94724dfd412f966d3d7a90db29aec8be7e35d650, overlaid only with this task's fixture
metadata/generator/README. No V02 WIP consumed; image assets excluded from extraction.

- Before: `cargo test --locked --offline -p uiblueprint-engine --test analysis every_manifest_engine_expectation_is_verified_at_its_contract_layer`
  fails exit101 at analysis.rs:792: exactly unstable-source.json and
  unstable-conditions.json declare match but return Err(ResultMismatch).
  Counts before:28 match/3 mismatch/2 input-only/26 contract-invalid.
- After: `cargo test --locked --offline -p uiblueprint-engine --test analysis`
  passes15/15, zero ignored or filtered. All59 manifest entries are consumed:
  26 match/5 mismatch/2 input-only/26 contract-invalid.
- `cargo test --locked --offline -p uiblueprint-schema --test analysis_contract authored_vectors_match_contract_and_structural_expectations`
  passes1/1; all59 structural/semantic expectations and valid round trips checked.
  Eleven unrelated tests filtered by this targeted command, not skipped failures.
- Generator run in the temp copy reproduces the corrected manifest exactly and
  preserves all59 analysis documents plus127 golden JSON files, including its
  manifest. Canonical byte comparison against94724df also preserves both schemas:
  188 JSON/schema files total, excluding the intentionally changed analysis manifest.
- Changed local links, generator syntax and `git diff --check` pass.
  Broader untouched suites/runtime intentionally not run; no full-suite claim.

Preserved SHA-256:

| Historical file | SHA-256 |
| --- | --- |
| unstable-source.json | 1a9148915b6b5fbe62f334ec25fc105c677008f100c64cace5a03c9a908f95cd |
| unstable-conditions.json | f1aa51c85017f50d269cedc69aff1a59229971b3aed3db61ff41630ecd62f929 |

Corrected manifest SHA-256:
0c091267f40e9b07086cdecf63cfcc50a790c5f008a55a88e10d82def65c4489.

## Ownership, cleanup and follow-up

Only own non-image temporary source/build files were created; they are removed
and absence verified before checkpoint. No images created, copied or deleted;
foreign after-title-spacing.png and all historical assets remain untouched.
No desktop, process or Q03 handoff ownership taken. Temporary Git lock is shared,
not removed. Exact-path checkpoint and canonical master push use
fcntl.flock on /tmp/ui-blueprint-master-git.lock; SHA/push result in final chat.
Q01's affected independent recheck consumes this correction. V02 and remaining
Native/privacy/D06/Q03/P7 gates are separate dependencies of overall acceptance,
not prerequisites to this fixture correction and not closed by these tests.
