# C00 review — faithful contract routing

- Class: verification; mode: Reconcile verification, no semantic delta allowed.
- Consumer: permission to dispatch R01/R02/R03 against normalized contracts.
- Ready only after C00 checkpoint; candidate commit is pinned in dispatch.
- Authority: PLAN.UIB@1 at `358c757e7eab84a3989d150dbad57924d866601a`,
  user launch and coordination-only correction in [registry](../task-registry.md).
- Work in current `master`, no branches/worktrees, nested delegation or goal.
- Fresh reviewer context; model/reasoning inherit.
- Economy basis: one fidelity review of shared contracts prevents divergent
  interpretation across later workers; no runtime/build review in this packet.

## Neutral basis

Read applicable instructions, [spec registry](../../../specs/README.md),
the immutable source [ТЗ](../../../ui-blueprint-spec.md) `UIB.TZ@1.4`,
[drawing guide](../../../engineering-blueprint-guide.md) `UIB.DRAWING@1.1`,
and all new selected normalized contract nodes and source mapping.
The source documents govern meaning; the normalized tree must restate them
without imposing new product choices. The task is document fidelity, not
implementation, legal advice, upstream verification, runtime or product redesign.
Read the C00 objective/write boundary in [packet](C00.md); do not read author
receipt or conclusions before recording your own initial observations.

## Mandatory criteria

1. Original ТЗ/guide content is unchanged from approved source revision.
2. Normative requirements, conditional exceptions, unknown/partial distinctions,
   Mac/Web parity, D01–D07 deadlines, GOLDEN01, safety and complete export contract
   remain represented; no new choice is silently elevated from proposal.
3. Stable IDs/revisions, ≤100-line new nodes, accurate source mapping and complete
   dependency closure make R01/R02/R03 and later domains selectable.
4. Provenance links do not require every worker to reload both giant originals;
   genuine semantic dependencies cannot be dropped to reduce context.
5. Future mobile/ML/F1–F4 scope remains excluded from P0–P7 and preserved as future.
6. Registry/AGENTS authority matches actual user launch and root coordination-only.
7. Changed local links/anchors and the applicable docs-only checks pass.
8. Changes respect C00 allowed paths, no modifications to other projects or code.

## Two-stage output and write boundary

Stage 1: inspect candidate and record criterion coverage/initial observations.
Return those in a final message and stop; do not issue final acceptance yet.
Root will send the author receipt and any exact prior findings in a follow-up.
Stage 2: reconcile them with your recorded observations, then return verdict:
accept | accept_with_residual | reject | not_verified. Include candidate revision,
criteria results, evidence gaps, blocking findings, repair owner and exact recheck.
Missing mandatory proof cannot be an accepted residual.

Read-only except `docs/plans/ui-blueprint/receipts/C00-review.md` for your short
observations and final receipt. Do not fix defects yourself; return them to root.
No raw logs or generated command captures in the repository. Do not mutate product
docs, coordination registry, plans or author receipt. Root reads your final output;
no cross-chat send required. Git lease for this receipt must be explicitly granted
at dispatch/follow-up before staging/commit; never include others' changes.

Waiting: record exact missing candidate/evidence/authority; do not reinterpret
product intent or launch implementation. Completion: final criterion-backed verdict,
receipt and path-limited checkpoint after Git lease; next action belongs to root.
