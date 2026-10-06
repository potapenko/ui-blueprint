# C00-review — initial observations, Stage 1

- Reviewer: fresh non-author chat `01a110a5-8af7-7ef1-8fb6-fd989e78e666`, 2026-10-06.
- Candidate: `92d2bf89ea9acab091fd166eef4432408e353744`, current branch `master`.
- Immutable source: `358c757e7eab84a3989d150dbad57924d866601a`;
  `UIB.TZ@1.4` and `UIB.DRAWING@1.1`.
- Mode: Reconcile verification; no semantic delta or product repair authorized.
- Authority: [neutral packet](../packets/C00-review.md), dispatch granting this
  receipt's exclusive Git lease, actual P0–P7 launch/ownership in the registry.
- Only write: this receipt. Root's disjoint registry modification stays untouched.
- Stage 1 recorded before reading the author receipt or author verdict/narrative.
  No final acceptance verdict is issued here.

## Traversal and basis

Global AGENTS → implementation/product-truth/QA routes → product-truth core,
routing, evidence, delivery, coordination → repository AGENTS →
[spec root](../../../specs/README.md) → Product/Acceptance/Reference branches →
[source map](../../../specs/reference/source-map.md) and research routes → all
43 mapped leaves, including their explicit Requires closure.
Both originals were read; their already-read content was reused for leaf bodies
after mechanical comparison of every mapped nonblank line. Leaf metadata and
additional routing text were read separately. Only prompt A/B adds code fences.
Supporting process: C00 packet, approved plan and current execution/packets/
acceptance documents; registry used for authority, ownership and link targets.

Pinned closure: `UIB.ROUTING@1`; PRODUCT-ROUTES, ACCEPTANCE-ROUTES,
REFERENCE-ROUTES, SOURCE-MAP, RESEARCH-ROUTES at `@1` (`ROUTE` clauses);
all following `UIB.<ID>@1`, clause `UIB.<ID>.CONTENT`:
BOUNDARIES, MODEL, EXCHANGE, NATIVE, GEOMETRY, PROJECTIONS, FORMS, IDENTITY,
ACTIONS, LIFECYCLE, CACHE, PRIVACY, CLI, ROADMAP, RUST-BOUNDARIES, EXPORT,
DRAWING-STYLE, DRAWING-GEOMETRY, DRAWING-PACKAGE, DRAWING-PROMPT-A,
DRAWING-PROMPT-B, DRAWING-REVIEW, PILOTS, NATIVE-PILOTS, WEB-PILOTS, COMPLETION,
PERFORMANCE, GOLDEN, REUSE, WEB-SOURCES, CORE-SOURCES, PROBE-SOURCES,
NATIVE-SOURCES, EXECUTOR-SOURCES, EXTENSIONS, OTHER-PLATFORMS,
PROFILE-SCENARIOS, PROVENANCE, START-NAME, FUTURE-HOST, FUTURE-DASHBOARD,
FUTURE-ATLAS, DRAWING-EXAMPLE.

Resolved product material: originals 1,395 lines; candidate root and 48 new
nodes 2,293 lines / 319,051 UTF-8 bytes. Cross-domain dependencies are the
declared Requires edges; all targets exist. Excluded: upstream code/web research,
other project contents, Rust implementation/toolchain work, runtime/visual QA,
and author receipt until Stage 2. Future leaves are reviewed for fidelity only.
No product revision drift: candidate AGENTS/spec tree equals the inspected
working tree. Coordination state is separately based on `2115e99` plus the
root-owned registry edit, not attributed to the candidate.

## Criterion coverage and initial observations

| Criterion | Independently observed evidence | Stage 1 coverage |
| --- | --- | --- |
| 1. Originals unchanged | Git blob comparison against the immutable source: both originals identical. | Covered; no discrepancy. |
| 2. Normative fidelity | All 43 mapped bodies match source nonblank lines; only prompt A/B fence boundaries added. TZ line 611 is the omitted parent heading; all its content is mapped. Unknown/partial, Mac/Web positives, M05, D01–D07 deadlines, GOLDEN01, safety and complete DrawingBrief retained. | Covered; no semantic change found. |
| 3. IDs, size, mapping, closure | 48 new nodes, maximum 83 physical lines; distinct contract IDs, metadata and mapped CONTENT anchors. R01 16/16, R02 18/18, R03 14/14 listed/transitive nodes; no missing Requires target. | Covered; no missing edge in the declared research closures. |
| 4. Provenance versus dependencies | Registry and leaves distinguish provenance from Requires; research closures do not require full originals. EXPORT reaches all six drawing leaves and the example through a 14-node closure. | Covered; selected routes preserve substantive dependencies. |
| 5. Future scope | Future host/dashboard/atlas, extension/mobile and other-platform content preserved verbatim with future-only routing; P0–P7 boundaries retained. | Covered; no future implementation authorized. |
| 6. Authority and coordination | AGENTS/registry root agree with recorded user launch and coordination-only correction; current plan/execution preserve source revision. C00 lease text allows only disjoint root metadata while worker owns index. | Covered; no conflicting current instruction found. |
| 7. Documentation checks | 448 repository-local links/anchors across candidate docs and applicable coordination docs, plus 4 registry link targets: no errors. Candidate AGENTS/spec diff and working-tree `git diff --check`: clean. | Covered for inspected docs; author receipt content/links deliberately deferred to Stage 2. |
| 8. Write boundary | Candidate diff-tree: 51 paths, all within C00's allowed set; no code or coordination-owner paths included. Reviewer writes only this file. | Covered for repository candidate; no external-project inspection performed. |

Checks used Git blob/diff/path inspection and bounded Python checks for mapped
line fidelity, node metadata/size, graph closure, local files and Markdown anchors.
26 external-project provenance targets were not opened. No Rust/runtime checks
were selected: the packet is documentation fidelity and changes no code.

## Handoff

Initial blocking findings: none in the inspected candidate and coordination docs.
This is supporting verification, not a delivered runtime capability or acceptance.
Remaining evidence: author receipt and any exact prior findings from root;
reconcile them in this same context, inspect deferred receipt links/check claims,
then issue the criterion-backed Stage 2 verdict. Missing proof is not a residual.
No repair is proposed at Stage 1. Stop after the path-limited checkpoint;
do not start R01/R02/R03 or read the author receipt before Stage 2 dispatch.
