# Word/Next source-limit diagnosis

Owner: [source-limit defect](codex://threads/01a20f35-c7e4-7490-9d62-2e06b6357999).
Consumer: [Random](codex://threads/01a20844-7030-7701-a26a-88793ef9752e).
Status: diagnostic candidate; consumer cause and successful Word/Next geometry pending.

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
fixture, runtime spec/registry/README and this receipt. Protected: prior successful
geometry, default/scoped/discovery behavior, field allowlist, all caps, protocol,
core schemas, cancellation, request binding, app state and consumer-owned evidence.
No raw captures, command logs or private descriptors belong in Git.

## Evidence

Consumer0.9.0 a4d1c290 has three successful states with11033/10406/13163 source
nodes; Word/Next reports sourceLimit without guard detail. Its cause remains
unknown. Saved outcome paths are under
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
Independent review is pending; no consumer acceptance is inferred.

Owned external evidence:
`/Volumes/Develop-2TB/Develop/tmp/uib-source-limit-01a20f35-2rd876rk/evidence`.
Internal fixture/runtime:
`/tmp/uib-source-limit-01a20f35-gzzo71sz`.
Retention: evidence through review and consumer handoff; temporary builds/app only
until reviewer releases their use and the owned process exits. No screenshots.

## Consumer repeat and decision boundary

After the release owner's matching SDK/caller delivery and consumer-owned rebuild:

```sh
python3 -B "$CALLER" capture --session "$SESSION" \
  --window "$WINDOW" --generation "$GENERATION" \
  --subtree-type PlayphrasemeApp.RandomCompletionDialog --out "$NEW_SNAPSHOT"
```

Keep the actual Word/Next state. Return only status/reason/source_limit on failure;
never share the private session descriptor. If it succeeds, validate and measure
the new Snapshot in the normal owner workflow. A source_nodes/depth overflow
cannot be repaired by skipping unvisited branches and still prove window-wide
uniqueness. Supporting that case needs an explicitly approved source-scope or
resource-contract change with its own completeness/identity proof. Raising caps,
changing the state, introducing app markers or claiming geometry are outside this
diagnostic candidate. No Word/Next pass is claimed.
