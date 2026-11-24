# Word/Next source-limit diagnosis

Owner: [source-limit defect](codex://threads/01a20f35-c7e4-7490-9d62-2e06b6357999).
Consumer: [Random](codex://threads/01a20844-7030-7701-a26a-88793ef9752e).
Status: diagnostic change released in0.10.1; consumer confirmed the source-node cap.
General runtime support for this oversized Word/Next state remains unsupported.
Word/Next product geometry separately passed via the official marker SDK.

## Authority and Spec Basis

The defect assignment asks to reproduce/fix Word/Next under existing limits and
preserve uniqueness, completeness and privacy. The first concrete plan was
explicitly approved by coordinator01a1e565 on2026-11-24 with immediate execution:
if a real hard limit is reached, finish available diagnosis and name the contract
decision before changing it. SDK owner01a20a46 offers finite review; owner01a20ed3
handed off Collector/Session/tests/runtime contract after discovery5a270081.
Current branch master; base5e18738257e9de1dee4dc333a2df40169358e52b.
Packaging, versioning and publication remain with01a20929. App operation and
marker-based or runtime acceptance remain exclusively with Random.

Traversal: specs/README registry93 → SWIFTUI-RUNTIME@6 → BOUNDARIES@1, MODEL@1,
IDENTITY@1, GEOMETRY@2, PRIVACY@1, EXCHANGE@2, LIFECYCLE@1, CACHE@1,
PROJECTIONS@1, CLI@21. Implementation/QA, product-truth core/routing/change/evidence/
delivery/coordination and operational logging clauses apply. Registry94 and
Runtime@7 RUNTIME.LIMIT-DIAGNOSTICS were registered before implementation.
Mode: additive diagnostic Evolve within the approved defect plan; no cap change.
Contract requires complete bounded source traversal before unique selection.
Observed0.9 source collapses all guards into sourceLimit. Proposal adds fixed
numeric evidence of the actual guard; it does not reinterpret caps or scope.

Write scope: RuntimeCollector/Session, Python caller, separate SourceLimitTests,
focused Python checks and source-limit harness reusing the existing owned subtree
fixture, runtime spec/registry/README/installed skill reference and this receipt. Protected: prior successful
geometry, default/scoped/discovery behavior, field allowlist, all caps, protocol,
core schemas, cancellation, request binding, app state and consumer-owned evidence.
No raw captures, command logs or private descriptors belong in Git.

## Evidence

Consumer0.9.0 a4d1c290 has three successful states with11033/10406/13163 source
nodes; Word/Next initially reported sourceLimit without guard detail. That older
response alone did not establish the cause. Saved outcome paths are under
`/Volumes/Develop-2TB/Develop/tmp/playphraseme-s13-uib090-jckkjby7`.

Owned SwiftUIRuntimeSubtreeFixture with1100 rows produces source_nodes,
limit16384, observed16385 in both scoped capture and discovery. No Snapshot or
inventory is published, stdout is empty, exit remains4. This count is a known
lower bound including pending children, not a full rejected-tree count. Actual0.9
caller against the candidate SDK still returns exactly failed/sourceLimit.
At exactly16384 admitted nodes, collection of actual SwiftUI getter data succeeds;
one additional node refuses16385. Type metadata length65536 passes,65537 refuses;
control presence refuses without exporting its value. Initial live unit invocation
omitted SWIFTUI_VIEW_DEBUG and exposed no tree; explicit profile29 corrects the
fixture setup. The live test now skips unless explicitly enabled; no production
policy or expectation was relaxed.

Python13/13 passed, including strict detail shape/kind/count admission, malformed
and canary-bearing detail refusal, old endpoint absence and owned-response cleanup.
Swift13/13 passed with profile29 from the internally copied XCTest bundle.
The existing live subtree suite passed:357 retained nodes from7595, Rust widths
120→180→120, absent/duplicate selectors, stale binding, cancellation, canary
exclusion and four distinct concurrent captures. Actual0.9 caller still passes
411-node whole and357-node scoped captures. The discovery suite passed with
7595 nodes/94 types/473732bytes, fresh removal/duplicate cases, correct source
refusal, cancellation and matching411-node small whole/inventory counts.
Independent compatibility/privacy review by SDK owner01a20a46 accepted4e88c57f
without blockers. Reviewer inspected exactdiff, ran Python13/13/diff-check, and
checked saved capture/discovery diagnostics, exact legacy refusal and live-suite
summaries. Swift/source boundary/live captures remain author-run, not reviewer-run.
Reviewed SHA256 prefixes: Collector240ae427807204b6, Session08396800f321402a,
caller4d7c5f6ef94d8aa1. Reviewer requested the same diagnostic explanation in the
installed runtime skill reference; that documentation-only follow-up adds no
behavior or new approval cycle.

Owned external evidence:
`/Volumes/Develop-2TB/Develop/tmp/uib-source-limit-01a20f35-2rd876rk/evidence`.
Former internal fixture/runtime: `/tmp/uib-source-limit-01a20f35-gzzo71sz`.
The owned process stopped; temporary builds/cache/app/private runtime were removed
after review, recovering212565269bytes. The small ownership note remains. Retain
external fixture evidence for the source-scope contract handoff; no screenshots.

## Separate consumer product acceptance

Random reported fresh official SwiftUI marker geometry and Rust0.10.0 agreement:
card576×151pt, Replay/Next261×39pt, gap7pt, Next Enter34×15pt. Its evidence is
`/Volumes/Develop-2TB/Develop/tmp/playphraseme-s13-word-final-obnv6_kc`.
This closes the consumer's product measurements, not general runtime support.
In the same window with SDK/caller5e187382, runtime capture and discovery both
returned sourceLimit/4 without geometry. The0.10.1 repeat below establishes the
guard. No read of the consumer's private process or descriptor was made here.

## Consumer repeat and decision boundary

Release owner delivered [v0.10.1](https://github.com/potapenko/ui-blueprint/releases/tag/v0.10.1),
commit9ce1e01428f768f62a7615f3607f5324b2b43772. Random then reported one repeat
in the original full Word/Next state with matching SDK/caller0.10.1: failed,
sourceLimit, kind=source_nodes, limit=16384, observed=16385. This is consumer-run
app evidence, separate from the owned fixture. The diagnostic is accepted in the
real failure path. It proves at least16385 encountered source nodes; it does not
establish the final full-tree count or a failure of depth/type/output limits.
The actual repeat used:

```sh
python3 -B "$CALLER" capture --session "$SESSION" \
  --window "$WINDOW" --generation "$GENERATION" \
  --subtree-type PlayphrasemeApp.RandomCompletionDialog --out "$NEW_SNAPSHOT"
```

No further unchanged repeat is needed. Runtime@7 RUNTIME.SUBTREE requires a
complete window-source read under RUNTIME.DATA limits before selecting the unique
type. The confirmed source-node overflow therefore prevents this path from proving
uniqueness or publishing geometry; dropping unvisited branches would violate it.

To support this case while keeping the current caps, a separate approved evolution
must define an explicitly addressable smaller acquisition scope BEFORE whole-window
collection, its fresh identity, completeness and uniqueness domain, and inherited
geometry context. That changes RUNTIME.REQUEST/SUBTREE/DATA; a feasible source-backed
mechanism must be established before implementation. This is a required design
decision, not an already implemented or qualified alternative. Raising the node cap
is a different resource-contract change and remains outside this assignment.
The present task changes neither contract nor app to bypass that boundary. General
runtime Word/Next geometry remains unsupported; the independent official marker
geometry acceptance stands.
