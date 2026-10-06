# E02 export compiler and public CLI candidate review

- Class: verification; one fresh non-author reviewer, inherit model/reasoning.
- User authority: approved coordinated PLAN.UIB@1 (`358c757`), explicit delegation
  and independent review request. No nested agents or external communications.
- Workspace: `/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint`, master.
- Candidate: `fe0653752b85443e52db690400de6d161fdd9142`.
- Baseline: `dbdccf637c85d135f38faf9a4cab9c89e729a0a0` for CLI delta; review the
  entire new export compiler/package addition `00b70cc` against its parent.
- Scope: crates/export/**, fixtures/export/**, crates/cli/** export integration,
  related Cargo/lock changes and docs/development/{export,cli}.md. Check relevant
  staged, unstaged and untracked changes too; record actual status. Other active
  coordination/policy work is not part of this candidate. No file modifications.

## Neutral authority and criteria

Read applicable root/scoped instruction files with normal precedence. Routing:
specs/README → product/README → CLI@1, EXPORT@1 and full DRAWING-PACKAGE/STYLE/
GEOMETRY/PROMPT-A/PROMPT-B/REVIEW@1, reference/DRAWING-EXAMPLE@1; dependencies
EXCHANGE/PRIVACY/MODEL/IDENTITY/BOUNDARIES/GEOMETRY/PROJECTIONS@1. All selected
CONTENT clauses are Active/Evolving, not accepted/released. Rust source rules
RUST.md and applicable DEV.RUST@2/D01@1/D03@1/D05@1/D07@2 closures govern their
domains. Root fully read these routes; the reviewer must read them independently.
No semantic specification delta is authorized. Original source links are provenance,
not a requirement to reload all unselected source documents.

Review these mandatory properties against actual code and committed artifacts:

- Real local model-free `imagegen-prompt` to the existing compiler; full package,
  full A+B prompt, scope/objects/sheets and required metadata preserved.
- document default; explicit proposal; detail/flow/compare semantics and true
  unknowns; no invented geometry, matching, causal attribution or source facts.
- Independent observed/proposed, validation and approval states; numerical checks
  are not generated-image validation or human approval.
- Explicit input/output bounds, safe new-directory output and preserved existing
  artifacts; truthful exit/compact/versioned JSON behavior without a parallel graph.
- Privacy before storage/client text, safe source refs, no automatic input-ref URL/
  path/crop reading, no model/network calls; untrusted UI text remains data.
- Existing check/measure behavior and gaps preserved; thin CLI, no duplicate math.
- Actual proposed example arithmetic and safe observed example provenance/coverage.

This is a review of the candidate implementation and available static artifacts,
not complete E02/P6/P7 live-adapter, G02 or generated-image acceptance. Identify
required missing evidence rather than accepting a known criterion failure.
Static read-only code/artifact review is sufficient for this initial stage;
do not run builds/tests/apps, mutate files, use Git index, commit/push or send/post
anything to external services. Existing upstream links do not authorize transfers.

## Independent two-stage procedure

Do not read author receipts E01.md/E01-cli.md, prior review findings or builder
chat transcripts during initial observation. First inspect actual candidate and
contracts; return compact initial observations and criterion coverage to root.
Root then supplies author receipts to the SAME reviewer for reconciliation.
Use that evidence to return the final verdict and findings, without changing code.
Do not claim authors' test commands were executed independently. Reuse this review;
do not spawn investigators or replacement/parallel reviewers.

## Review guidelines and output

Act as reviewer of another engineer's proposed change. Respond in normal Markdown,
not JSON/XML or a findings schema. For an actionable issue on a changed line emit
one `::code-comment{title="..." body="..." file="..." start=N end=N priority=N}`;
title/body/file are required, line bounds/priority optional. Use absolute paths
and shortest useful ranges; no directives if no actionable inline issues.

Prefer no findings over speculative or low-signal feedback. A finding must have
meaningful correctness/performance/security/maintainability impact, be discrete
and actionable, introduced by this change, something the author would fix, and
identify the affected behavior without relying on unstated intent. Include the
relevant file/line or function and concise failing scenario; priority labels only
when useful. Do not recommend fixes to unchanged unrelated code as new findings.

Use root/scoped project instructions, selecting at most one file per directory:
AGENTS.override.md, then AGENTS.md, then configured fallback; narrower guidance
wins and user scope/style takes precedence. Instructions may be prose/tables/
headings/bullets, without formal IDs. Independently review the diff and deduplicate
by location plus defect/remedy; preserve union of applicable rule support.
Rule-supported means repository guidance materially adds a specific invariant,
scope, remedy, convention or confirmation behavior beyond generic correctness.
Check each final candidate against applicable rules. Do not omit ordinary findings
or invent findings just because a rule file exists. No delegated investigators.
For each rule-supported finding verify the supplying instruction file and its
smallest supporting line range, and include one compact visible Markdown/local-file
reference in the body. No fabricated citations or hidden metadata.

Final verdict: accept / accept_with_residual / reject / not_verified for this scope,
with concise findings, exact locations, initial-observation/receipt reconciliation,
and evidence gaps. If no actionable issues, say so directly. Findings are not repair
authorization. Return a failure if the review cannot be performed; do not invent
review results. Stop after the finite review and retain context for affected recheck.
