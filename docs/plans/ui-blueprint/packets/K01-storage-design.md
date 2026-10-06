# K01 storage ownership handoff

- Class: coordination; finite Core packet, model/reasoning inherit.
- Immediate consumer: Integration D05 retention/admission decision, then actual
  K01 storage implementation. No new diagnostic framework or cache simulator.
- Authority: user-approved PLAN.UIB@1 at `358c757`, explicit parallel worker
  request and coordination-only root; current `master`, no worktree/branch.
- Ready: pure replay `e4ecee7`, D05 working allocation evidence `2a5dfef`.
- Mode: Restore existing CACHE/LIFECYCLE requirements; design only in this packet.
  A proposed Rust ownership choice is not yet a new product contract.

## Spec Basis and traversal receipt

Root read AGENTS → specs/README (`UIB.ROUTING@1`, registry4) → product branch →
CACHE, EXCHANGE, PROJECTIONS, LIFECYCLE → MODEL, IDENTITY, PRIVACY, BOUNDARIES;
decision branch → D03/D04/D05 and acceptance branch → GOLDEN/PERFORMANCE →
GEOMETRY, FORMS, ACTIONS, NATIVE, ROADMAP and C01-EVIDENCE. All these leaf CONTENT
clauses are @1, fully read; no semantic revision drift. Rust route is RUST.md,
DEV.RUST@2 → D01@1/D07@2 → RUST-BOUNDARIES/REUSE@1. Workers reuse their already
read current closure and read missing/changed content before source inspection.
Runbook, registry and original approved plan apply. Excluded: live adapter/input,
export internals, mobile, projection heuristics, optional future infrastructure.

Required: separate snapshot/relations/check caches; full context keys; bounded
memory/revisions/lifetime; atomic replay; evicted base means resync, never deletion;
detach releases owned state; TTL is not freshness; privacy before cache/storage.
Observed evidence: replay candidate exists; D05 measured canonical owned capacity
and parser/framing/session allocations. Total process cap is not established.
Integration proposal: canonical snapshot ownership, borrowed reads and explicit
caller-supplied limits. Evaluate against actual consumers; do not cite this proposal
as authority or manufacture a compatible implementation by weakening CACHE.

## Finite deliverable and boundaries

Inspect only existing canonical model/replay/accounting owners and relevant R03
source ledger. Produce a concrete implementation handoff for the next K01 packet:

1. Exact Rust owner/API and full cache-key contents; ownership of input Snapshot,
   retained revisions, derived results, indexes, session and aggregate accounting.
2. Enumeration of every retained allocation/capacity, including index and key
   storage; operations whose temporary clones/work need a separate reservation.
   Explain how reads, replacement, eviction, detach and replay release ownership.
3. Required limit fields and admission/eviction/error semantics. Leave numerical
   production choices to Integration; give sizing formulas or exact missing
   evidence, not invented defaults or claims that layout bytes equal RSS.
4. Minimal code write set and focused tests: quota boundaries/overflow, context
   separation, unknown history, TTL/invalidation, atomic replay failure, lost base,
   cross-session isolation, detach/release and redacted-versus-empty persistence.
5. Identify an implementable first K01 storage slice preserving the full K01 goal;
   expose exact shared API dependencies, not a parallel schema or hidden fallback.

Writes only `docs/development/cache.md` (clearly labelled implementation proposal)
and `docs/plans/ui-blueprint/receipts/K01-storage-design.md`. No source edits,
runtime, raw evidence copies, benchmarks, dependencies, shared manifests or spec
policy edits. Five protected review findings remain awaiting user repair authority;
do not repair or design a second validator to route around them.

Root mediates the D05 handoff; no other chats, agents or external communications.
Check changed local links and scoped diff whitespace only. Return concrete proposal
and exact dependency, obtain root Git lease, commit+push only these two paths,
return SHA/push/receipt and release. Do not begin implementation until the next
packet pins the D05 decision. Record waiting_resource with owner/recheck if needed.
