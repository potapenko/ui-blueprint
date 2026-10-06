# D05 finite admission and storage policy decision

- Class: coordination; Integration owner, inherit model/reasoning, no nested work.
- Consumer: K01 bounded storage implementation and first W01/M01 shipping adapters.
- Authority: ROADMAP D05 explicitly delegates engineering limits to the agent under
  user-approved PLAN.UIB@1 (`358c757`); direct parallel-work authorization retained
  in registry. Current `master` only. No new product scope or review-fix authority.
- Ready evidence: sizing `c6a5df6`, Web `fcf48be`, Native `51c7809`, working allocation
  diagnostic `2a5dfef`; E01 integration checkpoint is completed first.
- Economy: choose the next implementable policy from existing evidence; no more
  open-ended profiling or broad synthetic diagnostics without an exact missing
  bound that prevents the decision.

## Spec Basis and decision boundary

Use the fully read current closure and root traversal receipt in
[K01 storage design](K01-storage-design.md#spec-basis-and-traversal-receipt).
D05@1 is the current normative policy; changes to chosen semantics require a
recorded Contract Delta and revision under ROADMAP's delegated D05 authority.
Do not alter other CONTENT@1 norms or relabel partial evidence complete.

Requirement: finite request, queue, revision and session/process memory limits
before first live adapter; explicit deadlines, overflow/oversize/eviction and
bounded resync. Observed: wire size is not retained or transient storage, member
order matters, actual platform samples do not cover configured maxima. Existing
proposals (caller limits, borrowed reads, canonical accounting owner) are design
inputs only, not already accepted requirements. K01's exact representation is an
external dependency to be supplied by Core; work on independent parser/host policy
while waiting, return the exact ownership question rather than polling indefinitely.

## Finite outcome

Prepare the minimal concrete D05 decision enabling implementation:

1. Specify the memory metric and owner for retained data, transient decoder/replay
   work, framing, per-session and aggregate process quotas. State explicit coverage
   and exclusions; do not call layout accounting or encoded-byte limits an RSS cap.
2. Choose implementable admission/enforcement and numeric policy based on existing
   examples and allowed input bounds. If a universal decoder bound is unavailable,
   identify a concrete bounded enforcement mechanism and its exact implementation
   owner/checks. Do not invent a worst-case multiplier from measured samples.
3. Reconcile Core's storage ownership handoff when root provides it; publish the
   retention/admission policy before K01 source implementation. Explain what K01
   can implement now and what still prevents live W01/M01 acceptance.
4. Return one bounded implementation write set and acceptance mapping for remaining
   enforcement. No request for more measurements without an exact decision it settles.

Writes: `docs/specs/development/decisions/d05-limits.md`, a new linked
`docs/specs/development/decisions/d05-memory.md` if needed to keep nodes <=100 lines,
decision README/root registry revision metadata only if a contract revision requires
it, and `docs/plans/ui-blueprint/receipts/D05-policy.md`. Do not touch product source,
Cargo, fixture outputs, other policy or owner receipts during this packet.
Unresolved choices remain visibly proposals until reconciled; no self-authorizing
change to the requirements. No validator/schema cutover or five protected fixes.

Documentation verification: route/links, clause consistency, scoped diff whitespace.
Root receives the concise decision plus scope/evidence exclusions; no build or
runtime claim. Checkpoint+push after root Git lease, exact paths only; return
SHA, push result, revision/consumer handoff and resource release. Stop after the
finite decision; enforcement source work gets a separate bounded packet.
