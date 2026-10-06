# Specification registry

- Node type: root; contract: `UIB.ROUTING@1`; registry revision: 7.
- Authority: Active; stability: Evolving; accepted/released implementation: none.
- Read when: selecting product/development contracts or recovering uncertain routes.
- Do not read when: a fully read, current selected closure already governs the task.
- Requires: select only the route below; branch summaries do not replace leaves.

## Authority and precedence

`UIB.ROUTING.AUTHORITY`: user confirmation on 2026-10-06 made
[UIB.TZ@1.4](../ui-blueprint-spec.md) and
[UIB.DRAWING@1.1](../engineering-blueprint-guide.md) Active / Evolving.
General specification governs product boundaries; drawing guide governs export
within them. Neither overrides global safety. Active does not mean implemented,
accepted or released. D01–D07, preliminary names and future phases retain their scope.

`UIB.ROUTING.FIDELITY`: C00 leaves are faithful restatements of those sources,
with stable clause IDs and semantic revision 1, not new product decisions.
If a difference is found, the original Active norm prevails; repair transcription
without inventing a product choice. [Inverse source map](reference/source-map.md)
connects every source section to its routed owner. Source links are provenance,
not a requirement to reread both full originals for each task. Read selected
leaves completely and follow explicit `Requires`; follow additional task routes
only when their trigger applies. Catalog/reference links do not authorize work
in another project or import all upstream contracts.

`UIB.ROUTING.SCOPE`: [PLAN.UIB@1](../plans/ui-blueprint-development.md) was
approved and launched; exact approval and original commit are preserved in the
[task registry](../plans/ui-blueprint/task-registry.md). Root is coordination-only;
finite workers implement/review within packets. Approval covers P0–P7, not F1–F4,
mobile implementations or unrelated products. Plan status is not runtime evidence.

Historical authority delta `UIB-AUTH-001`: Draft 1.3 / Draft 1.0 → Active 1.4 / 1.1
on user confirmation; no wire schema or released behavior changed. Field-level
`draft` and historical statements remain meaningful. C00 adds routing only.

C01 engineering choices under ROADMAP D01–D07 are registered in
[UIB.DECISIONS@1](development/decisions/README.md). `C01-DEC-001` selects development
policy (`DEV.RUST@2`) and initial candidate interfaces/gates; it does not accept
product runtime or alter existing CONTENT@1 requirements. The linked direct user
clarification preserves explicit request-driven collection, without periodic polling.

`D05-RET-002` selects [D05@2 retained memory policy](development/decisions/d05-memory.md)
under ROADMAP's delegated engineering authority before K01 storage implementation.
Working-memory enforcement and live acceptance remain open; D02/wire contracts unchanged.

`L01-ANALYSIS-001` registers [D03@2](development/decisions/d03-data.md) and
[ANALYSIS@1](product/analysis.md) under delegated ROADMAP representation authority,
using accepted handoff8fdf608. Local analysis0.2 reuses protected core0.1 data;
registration precedes implementation and does not accept runtime or new arithmetic.

`ANALYSIS-FLOAT-001` records [D07@3](development/decisions/d07-reuse.md)'s verified
need for the same pinned serde_json runtime float_roundtrip feature, preserving
source-number fidelity without new dependencies, tolerance or core wire changes.

## Select a route

| Task | Entry | Authority / selection |
| --- | --- | --- |
| Product behavior/schema/engine/plugin/CLI/export | [Product tree](product/README.md) | Current norms; select the smallest applicable leaf closure |
| R01 browser / R02 native / R03 core research | [Exact research routes](reference/research-routes.md) | Pinned source-reading scopes; no design decision D01–D07 by C00 |
| Fixtures, GOLDEN01, pilots, integration or performance | [Acceptance tree](acceptance/README.md) | Positive/negative evidence requirements, not claimed results |
| Source borrowing or provenance | [Reference tree](reference/README.md) | Select mechanism; historical catalog is not a fresh code/license audit |
| Rust source | [RUST.md](../../RUST.md) plus selected product leaf | Local engineering rules, no product authority by themselves |
| Toolchain, Cargo, dependencies, targets/features/checks | [DEV.RUST@2](development/rust.md) | Active / Evolving; C01 setup choices, product builds still unaccepted |
| D01–D07 decisions, T01/S01 implementation handoff | [Decision route](development/decisions/README.md) | Active / Evolving; delegated ROADMAP choices and explicit unresolved proof obligations |
| New product contract | [Feature template](templates/feature-spec.md) | Register authority/revision/dependencies before implementation; template grants none |

## Routing invariants

`UIB.ROUTING.NODES`: every new node is at most 100 physical lines. Stable clause
IDs identify meaning; source line ranges aid fidelity checks and do not replace
IDs. Original imported documents remain unchanged. Leaf `CONTENT` clauses retain
normative distinctions and source-local examples; example numbers are not defaults.
Explicit `Requires` links name semantic dependencies; navigation/provenance links
are not automatic preload. All current routed nodes have no accepted/released
baseline; future/reference evidence remains future/reference even inside Active sources.

`UIB.ROUTING.PROVENANCE`: repository instruction/development separation is adapted
from ai-friendly-search-engine; routing/revision and authority-vs-release structure
also draws on swiftui-semantic-audit. These references import no product dependencies.
[Repository instructions](../../AGENTS.md) retain local routing only.
