# Shared five-fix independent recheck

Reviewer: existing non-author `/root/p1_candidate_review`, coherent shared repair
wave at `9ca645aab6816425ee64bb20eeb1201750d0d2b0`; repair code `2828cd8`.
Verdict: **accept for the five assigned fixes**. No new actionable finding.

Initial source/tests/contract observations preceded author receipt access. The
same reviewer then reconciled S01-review-repair and independently recomputed the
saved211-file hash: `db7792faeea43667960657a53560c90fd47aa4f947cf1b62daec896dc5d35afe`.
It matches Integration's check/fmt/Clippy and101-test barrier. The reviewer did
not execute builds/tests/runtime or modify files; author evidence remains attributed.

Closed counterexamples:

1. Finding anchors now intersect complete compatible Space records; equal units
   alone are insufficient and attached transform destinations require node Surface.
2. All five geometric property fields require the corresponding frame_kind.
3. DeltaCase/GoldenChain validate complete reconstructed/source state and context,
   retained metadata, coverage, focus/relations and observation provenance.
4. Resolution evidence is reported/current and backend-ref-bound; precondition
   evidence is current, source-bound and covers the required field.
5. Nonrecursive topological maximum-path computation handles shared DAG children
   in polynomial work, rejects cycles/dangling refs and preserves depth boundaries.

Seven schema regression tests and three depth tests, plus positive neighbors and
existing parity corpus, support the fixed cases. Reviewed source has no drift
from the repair commit. Unrelated Native working changes were excluded.

Scope does not accept whole P1, executor/runtime behavior, D05 working-memory
enforcement, outstanding canonical JSON result contracts or new cache code.
Export has its separate E02 verdict and is not accepted by this shared review.
Same reviewer can be reused only for a new affected regression or evidence change;
these five accepted fixes remain closed without such evidence.
