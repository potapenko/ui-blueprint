# P1 saved-candidate independent review

- Reviewer: fresh collaboration `/root/p1_candidate_review`; no author context,
  no nested agents, no file changes, builds/tests/runtime or external communication.
- Base `61022a919b1a064e901f5021fdcabf6e28b41eef`; artifact
  `80697107c5d246a61e93fe06a23275554653b45f`; [packet](../packets/P1-review.md).
- Classification: verification. Scoped verdict: reject pending one new P2;
  no whole-P1/product acceptance. Later/uncommitted D05 work excluded.
- Initial source/contract observations preceded author evidence. Second stage
  reconciled pinned S01/G01/L01 and both D02 author receipts and retained evidence.

## New finding P1-R1

`crates/plugin-api/src/lib.rs:405–407`, `depth_within`, at reviewed revision.
The graph validator permits acyclic shared children. Depth traversal expands every
path without memoization: 64 nodes in32 two-node layers with shared successors fit
finite size/depth bounds but produce more than2^32 visits at max_depth32.
The receive clock check occurs before that synchronous traversal, delaying timeout
and cancellation despite admitted request bounds. Use a memoized longest-path
calculation or an equivalent polynomial algorithm; plain visited alone loses the
maximum-depth invariant when a node has several paths from roots.
Applicable authority: D05@1, lines17–22 in artifact8069710 (bounded requests/work).
Existing tests do not cover that DAG. This is a static finding, not a reproduced
hang or a measured performance result. Repair owner: Integration/plugin API.
Focused regression must cover shared-child longest-path semantics and bounded work.
Do not hide the defect by changing fixture/request limits to an easier case.

## Reconciliation and limits

No other new high-confidence finding. Saved D02 material is consistent with the
narrow claim that real Web/Native responses crossed the common boundary; live
observations and injected controls stay separate. Reviewer checked source hashes
against the candidate and saved Native responses. No runtime independently rerun.
Code review cannot promote these scoped examples to complete platform support,
production privacy/isolation, D05 memory policy or P1 compatibility freeze.

Four earlier validator P2 were present at this review's base and not relisted as
new. Explicit JSON measurement/result-space/applicability limits remain incomplete
capabilities, not new findings manufactured from later goal scope.

The user previously restricted review repairs to an explicit subsequent instruction.
Root requested authorization for all five findings; until received, P1-R1 and the
four earlier findings remain awaiting_authority. No repair was made by reviewer/root.
Independent authorized work may continue; this candidate is not accepted.
