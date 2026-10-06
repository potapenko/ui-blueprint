# S01 — requested checkpoint review

- User explicitly requested one fresh read-only reviewer, including staged,
  unstaged and untracked changes; no repairs without a further user instruction.
- Reviewer: collaboration `/root/checkpoint_review`, fresh context, no delegation.
- Base `eb0edbf726029e0c0a6a3336f0aafbcbbb61a656`; saved candidate
  `1d1285982922fadfabf683d2c1f5ff576cc58734`; later six-line channel contract delta
  reviewed and saved in `61022a919b1a064e901f5021fdcabf6e28b41eef`.
- Both checkpoints and prior history pushed to origin/master; clean working tree
  verified after second push. This is saved WIP, not S01 acceptance.
- Static inspection only; counterexamples inferred from code branches, not executed.
  Reviewer reports pinned diff whitespace check passed; all 126 wire JSON examples
  covered by manifest. No files changed by reviewer.

## Findings and next owner

All P2; repair owner S01 Integration, implementation-only Restore if authorized.

1. `validation/outcomes.rs:302–305`: GEO-GAP can claim pass across distinct local
   coordinate spaces with equal units. Require a shared space or established
   transform. Contract: GEOMETRY@1, coordinate-space/transform semantics.
2. `validation.rs:341–346`: layout_bounds accepts geometry marked
   accessibility_bounds. Validate field/frame-kind agreement. Contract: D03@1,
   Geometry responsibility and GEOMETRY@1.
3. `validation.rs:146–150`: empty upsert can pass against a full source snapshot
   with changed checked value. Validate complete reconstructed/source equivalence,
   including graph/context/coverage. Contract: D03@1 Delta/Source oracle, CACHE@1.
4. `validation/outcomes.rs:56–59`: current backend_ref observation can mask stale
   resolution/precondition evidence. Require current, properly bound evidence.
   Contract: D04@1 fresh resolution and ACTIONS@1.

Reviewer verified that the later channel-contract delta does not repair these.
No runtime, schema parity, live adapter or whole-S01 acceptance follows from review.
The user-facing findings contain exact line references; line numbers above name
the reviewed revision, not a permanent code location.

## Continuation boundary

The user restricted review to findings without automatic fixes. Root asked for
explicit repair authorization; until received, these four items remain
`awaiting_authority`. Independent previously approved S01 work and F03c continue.
The implementation owner's stop receipt separately reports the redacted-with-value
golden failure, missing generated schema/plugin API/D02-D05 proof/docs/receipt.
These are unfinished original S01 work, not acceptance gaps hidden by the checkpoint.
