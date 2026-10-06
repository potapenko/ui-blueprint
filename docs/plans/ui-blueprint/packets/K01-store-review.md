# K01 retained-store independent review

- Class: verification; one fresh non-author reviewer, inherit model/reasoning.
- Authority: user-approved coordinated PLAN.UIB@1 and explicit parallel/delegated
  work; required ownership/privacy review under current governance. No nested work.
- Repo: /Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint, master.
- Candidate: `f85f06f555772ecbb610acf6e66e8f0bf1c2a642`; inspect this commit versus
  its parent for engine/src/cache/**, engine/lib cache export and engine/tests/cache.rs.
- Supporting canonical sizing owner is saved at ec0f35c; unchanged schema0.1,
  validators2828cd8 and pure replaye4ecee7 are dependencies, not a fresh full audit.
- Review related staged/unstaged/untracked state read-only; Native/spec-registration
  work may continue outside this candidate. Use pinned Git source for stable review.

## Authority and mandatory criteria

Read applicable global/repo/scoped instructions with normal precedence. Root's
fully read selected route: specs/README registry5 → product CACHE/LIFECYCLE/
EXCHANGE/PROJECTIONS → MODEL/IDENTITY/PRIVACY/BOUNDARIES, all CONTENT@1;
decision route D05@2 → D05-MEMORY@1 all METRIC/LIMITS/OWNERSHIP/ADMISSION/LIFETIME/
GATES clauses, C01-EVIDENCE and explicit dependencies; D03/D04@1, GOLDEN,
GEOMETRY/FORMS/ACTIONS/NATIVE/ROADMAP/PERFORMANCE/RUST-BOUNDARIES/REUSE@1,
RUST.md/DEV.RUST@2/D01@1/D07@2 where applicable. Read the selected full closure
independently; source-code comments are evidence, not substitute contract authority.
The unrelated forthcoming analysis0.2 registration does not change this pinned
Snapshot0.1 storage contract. Resolve actual selected semantic drift explicitly.

Check actual implementation and tests against the following, including failure paths:

1. Retained owned-layout accounting is exhaustive for root/fixed backing/metadata,
   actual payload capacities and partitions, with checked arithmetic and no double
   inline/shared allocation charge or hidden live heap. Exclusions match the metric.
2. One aggregate domain and nonduplicable grants enforce byte/session/grant limits;
   construction rollback, closure/drop and reservation release preserve ownership.
   No early quota reuse while a payload/Store/grant remains live; usage/reservation
   remain distinct, including vacant capacity and detached-but-open Store backing.
3. Complete Context/channel/ID/revision/generation isolation; set-field semantics
   match existing compatibility; partitions are explicit, not guessed from content.
4. Admission plans before mutation, returns unchanged owned input on refusal,
   has no hidden payload clone and evicts only eligible requesting-session entries.
   Family count/oldest-admission ordering, duplicates and conflicts preserve policy.
5. Explicit same-domain monotonic age, exact boundary, regression/overflow refusal,
   invalidation and detach/drop; reads never refresh age or promote stored facts to
   live freshness. Stale handles/lost bases require resync without implying deletion.
6. Borrowed read lifetime, replay failure/publication atomicity and independent
   sessions; known/unknown/redacted/false/empty source values/evidence stay intact.
7. Tests use meaningful actual owners/capacities and independent expectations;
   negative/borrow proofs fail for the intended reason. Public API/error/docs
   must not claim total RSS, enforced transient allocations or live revalidation.

Scope is retained Snapshot storage. Decoder/validation/replay temporary enforcement,
raw pixels, live adapters, CLI lookup and separate derived caches are named later
obligations, not supplied capabilities. Do not invent their completion or demand
new optional frameworks as a repair. Do not reopen unaffected accepted repairs.

## Independence and execution

Stage1: inspect actual code/tests/contracts and report initial observations plus
criterion coverage to root. Do not initially read receipts/K01-storage.md or the
builder's docs/development/cache.md narrative; the authoritative D05-MEMORY leaf
states adopted design and is sufficient. Supporting design links are not extra
norms. Root then supplies the author receipt/API handoff for Stage2 reconciliation
to the SAME reviewer. Do not label author test execution your independent execution.

No builds/tests/runtime, filesystem modifications, Git index/commit/push, nested
agents or external communications. Read-only local inspection only. If review
cannot be completed, return the failure rather than fabricated findings.

## Output and rule attribution

Normal Markdown, no JSON/XML/findings schema. Give a scoped verdict accept,
accept_with_residual, reject or not_verified; missing mandatory proof is not pass.
Concise discrete actionable defects introduced by the change only, with clear
failing scenario/impact and exact file/line/function. Prefer none over speculation.
One ::code-comment per actionable inline issue, required title/body/file and
optional start/end/priority; absolute path and shortest useful range. No directives
when no actionable inline comments. Do not fix findings or become implementer.

Use applicable AGENTS.override.md, else AGENTS.md, else configured fallback, at
most one per directory; more-specific rules and user scope/style take precedence.
Guidance need not have formal IDs. Deduplicate by changed location plus defect/
remedy, preserving union of support. Rule support must add repository-specific
invariant/scope/remedy/convention beyond generic correctness. For each supported
finding verify the instruction/contract source and smallest supporting line range,
then include one compact visible Markdown/local-file reference in its body.
Do not fabricate citations or findings from the mere presence of a rule file.
Review failure is reported as failure, never as findings. End after final verdict;
retain context only for an authorized affected recheck.
