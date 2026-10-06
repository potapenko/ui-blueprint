# E02 compiler and CLI candidate review

- Reviewer: fresh non-author `/root/e02_candidate_review`, fork_turns none,
  one child, no nested delegation. [Packet](../packets/E02-candidate-review.md)
  at `4e04161`; artifact `fe0653752b85443e52db690400de6d161fdd9142`.
- Verdict: **reject** for the scoped candidate. Two actionable P2 defects.
- Method: read-only code/contracts/static artifact inspection, no builds, tests,
  runtime, file changes or external-service communication. Relevant dirty files
  checked; unrelated Core design docs excluded. Final scoped sources unchanged.
- Two-stage independence: initial observations and criteria coverage received
  before author receipts were provided; same reviewer then reconciled E01 and
  E01-cli receipts, shared-input hashes and retained observed source.

## Findings and authority

E02-R1 — `crates/export/src/proposal.rs:136–137`: vertical edges ignore the
declared bottom_left origin. With A y=0,height=10 and B y=30,height=20, the correct
A.top→B.bottom distance is20; current assignments calculate50. Consequently the
validator rejects the correct dimension and accepts the wrong one as numerically
checked. Remedy: resolve vertical edges from the declared origin. Rule basis:
[GEOMETRY@1](../../../specs/product/geometry.md#uib-geometry-content), line27 at the
artifact, routed by [AGENTS.md](../../../../AGENTS.md), line20. Owner Export.

E02-R2 — `crates/export/src/proposal.rs:83`: exact f64 equality rejects a valid
rectangle x=0.2,width=0.1 with authored left→right dimension0.1 because computed
`(x + width) - x` is approximately0.10000000000000003. Remedy: account for arithmetic
roundoff without inventing a UI measurement tolerance. Generic numerical correctness
finding; no fabricated repository-specific attribution. Owner Export.

Recheck both through focused proposal-validator cases and actual CLI outcome,
with separate handling of genuine mismatches and incompatible coordinate origins.
Original review instruction forbids fixes without user request. Both findings are
`awaiting_authority`, not implementation permission. Alongside four S01 findings
and P1-R1, seven findings now await repair approval; new async question asks about
all seven or only these two. No answer/authorization is inferred from goal continuation.

## Reconciliation and coverage

Reviewer verified receipt copies against their pinned revisions, the current
CLI shared-input hash against the author claim, and original retained Web source
SHA256 against E01 provenance. Its32 nodes/17relations/160properties/numeric facts
match the deliberate safe fixture with consistent identifier substitutions.
Proposed example rectangles and both arithmetic chains match the contract.

Remaining reviewed criteria have static support: full package/template, mode
gates, independent statuses, bounded IO, destination preservation, privacy
handling and unchanged check/measure paths. Existing author tests do not cover
the two defects. They were not executed independently by this review.

Live adapters, G02 attribution, generated-image review and whole E02/P6/P7
acceptance remain outside this scoped verdict. Code findings prevent acceptance
of the proposal validator; other acceptance gaps are not silently converted to pass.
Retain reviewer context for any authorized affected recheck; no replacement review.
