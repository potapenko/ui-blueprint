# Specification registry
- Node type: root; contract: `UIB.ROUTING@1`; registry revision: 32.
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
`D05-RET-002` originally selected [retained memory policy](development/decisions/d05-memory.md)
under D05@2/ROADMAP before K01 storage; that decision left D02/wire unchanged.
Working-memory implementation and live acceptance remain open.
`L01-ANALYSIS-001` registers [D03@2](development/decisions/d03-data.md) and
[ANALYSIS@1](product/analysis.md) under delegated ROADMAP representation authority,
using accepted handoff8fdf608. Local analysis0.2 reuses protected core0.1 data;
registration precedes implementation and does not accept runtime or new arithmetic.
`ANALYSIS-FLOAT-001` records [D07@3](development/decisions/d07-reuse.md)'s verified
need for the same pinned serde_json runtime float_roundtrip feature, preserving
source-number fidelity without new dependencies, tolerance or core wire changes.
`W01-TRANSPORT-001` records [D07@4](development/decisions/d07-reuse.md)'s narrow
numeric-loopback ws codec/log-boundary adoption before Web transport source work.
All source-audit guards apply; runtime float_roundtrip and open D05 gates remain.
`D02-WORKER-001` / `D05-WORK-001` / `D05-PARTITION-001` register reviewed designdb629fc:
[D02@2](development/decisions/d02-boundaries.md), [D05@3](development/decisions/d05-limits.md),
[D05-MEMORY@2](development/decisions/d05-memory.md), [D05-WORK@1](development/decisions/d05-working-memory.md).
Delegated technical registration is not runtime acceptance; D07@4, wire meanings
and all implementation/platform/positive D06 gates remain protected.
`H01-PROCESS-001` registers [D07@5](development/decisions/d07-reuse.md)'s exact libc
OS-binding use at the host boundary from Core's root-accepted source/license request.
Existing locked version/policies remain; registration does not accept unsafe/runtime proof.
`D05-NATIVE-ACQUISITION-001` registers [D05@4](development/decisions/d05-limits.md) and [Native acquisition@1](development/decisions/d05-native-acquisition.md) under [root's ROADMAP/D05 selection](../plans/ui-blueprint/packets/M01-acquisition-registration.md) of45c2667.
Native-only metrics/ceilings/admissions add no pure Rust Swift dependency; common quotas/wire/permissions/F02/D06 and implementation/live gates remain unchanged.
`D05-NATIVE-IMAGE-002`: [Native acquisition@2](development/decisions/d05-native-acquisition.md) reconciles explicit user system-temp/no-image-deletion authority via [M03 packet](../plans/ui-blueprint/packets/M03-popup-capture.md); staging/partial/final images remain, descriptors/helpers retire, incomplete/stale payloads stay unpublished. Non-image cleanup, quotas/privacy/wire/positive gates unchanged.
`G03-NATIVE-AX-001`: [NATIVE@2](product/native.md)/[CLI@7](product/cli.md) adds explicit read-only ordinary-process AXFocusedWindow geometry; observation-scoped identity, no fixture files/CG mapping/capture/action authority; canonical formats and old providers unchanged.
`G09-EXPORT-INPUT-001`: [CLI@8](product/cli.md)/[CLI-EXPORT@1](product/cli-export.md) adds explicit Snapshot or observed ChannelResponse plus caller metadata to document export under delegated P6; existing --brief, compiler versions, numerical/privacy/status contracts and other CLI commands preserved.
`G10-NEIGHBORS-001`: [CLI@9](product/cli.md)/[CLI-NEIGHBORS@1](product/cli-neighbors.md) exposes accepted G02 relation selection with explicit cap/source attribution under P3/P6; no engine/wire or existing-command change.
`A04-CLI-001`: [CLI@10](product/cli.md)/[CLI-ACTIONS@2](product/cli-actions.md) connects Web Focus/Type to explicit canonical Expectation input under P5/P6; legacy SetChecked, geometry, authority/effect/ACK and canonical versions unchanged.
`A05-CLI-001`: [CLI@11](product/cli.md)/[CLI-ACTIONS@3](product/cli-actions.md) adds the compiling Web native-button Activate port with explicit independently held public result identity under P5/P6; existing transport/permissions/canonical versions and other commands preserved.
`G11-COMPONENT-001`: [CLI@12](product/cli.md) extends compact design inspect with reported component members/properties and attributed context under [G11 packet](../plans/ui-blueprint/packets/G02-scope.md#g11-component-parts-through-existing-design-inspect--2026-10-08). Existing JSON1.0.0/graph/identity/neighbor meaning unchanged; registration precedes source acceptance.
`G12-GEOMETRY-DIFF-001`: [CLI@13](product/cli.md)/[CLI-GEOMETRY-DIFF@1](product/cli-geometry-diff.md) adds explicit selected-space rect comparison under [G12](../plans/ui-blueprint/packets/G02-recorded-diff.md#g12-geometry-comparison-in-a-selected-space--2026-10-08), preserving raw JSON1.0.0/core/analysis/arithmetic/cache.
`M05-COMPOSITION-001`: [EXCHANGE@2](product/exchange.md#uibexchangecomposition--explicit-axprobe-design-response)/[D03@3](development/decisions/d03-data.md) registers probe-wrapper/design/exact AX+probe observations and per-nested-channel request/session Supported-or-Partial authorization under [S01-native-proof M05](../plans/ui-blueprint/packets/S01-native-proof.md); homogeneous replies/126goldens/wire preserved.
`G13-GRAPH-DIFF-001`: [CLI@14](product/cli.md)/[CLI-GRAPH-DIFF@1](product/cli-graph-diff.md) registers additive saved structure/relations/components/metadata/focus comparison under [G13](../plans/ui-blueprint/packets/autonomous-tasks-2026-10-08.md#g13--сравнение-структуры-связей-и-фокуса), preserving raw JSON1.0.0, G12, canonical sources and cache.
`E03-EXPORT-COMPARE-001`: [CLI@15](product/cli.md)/[CLI-EXPORT@2](product/cli-export.md) connects saved pairs to engine-attributed six-file compare export under [E03](../plans/ui-blueprint/packets/E03-observed-compare.md); comparison packages0.2, other exports/core/raw diff unchanged.
`M02-N-SESSION-001`: [NATIVE-SESSION@1](product/native-session.md)/[CLI@16](product/cli.md) registers bounded attached Native form refs/private exchange/CLI under the assigned [M02-N packet](../plans/ui-blueprint/packets/M02-native-workflow.md). Existing wire, Web and ordinary AX behavior remain protected; runtime/review acceptance separate.
`V02-PROTECTED-001`: [NATIVE-SESSION@2](product/native-session.md)/[D03@4](development/decisions/d03-data.md) reuses existing FillSecret through a private one-use Native delivery source under [V02](../plans/ui-blueprint/packets/V02-protected-input.md). No core/analysis/wire/legacy change; actual capability and privacy acceptance separate.
`N03-POPUP-001`: [NATIVE-SESSION@3](product/native-session.md)/[NATIVE-POPUP@1](product/native-session-popup.md) registers explicit held popup/parent composition under [N03](../plans/ui-blueprint/packets/N03-native-popup-e2e.md), before source edits. Ordinary forms, PROTECTED same-Surface policy, canonical wire, nonce and consumers stay unchanged; independent acceptance remains separate.
## Select a route
`W06-FIDELITY-001`: [WEB-DOCUMENTS@1](product/web-documents.md) registers raw AX focusability and explicit bounded whole-document selection under [W06](../plans/ui-blueprint/packets/W06-web-fidelity.md), preserving core0.1/analysis0.2/ordinary Web scopes/D06; registration is not runtime acceptance.
`L01-INSPECT-001` / `L01-OBSERVE-001` / `L01-DIFF-001/002`: [CLI@6](product/cli.md) preserves inspect/observe and reconciles [recorded diff@2](product/cli-diff.md) under [selected L01 packet](../plans/ui-blueprint/packets/L01-recorded-diff.md); distinct environments stay attributed, CACHE/Delta/core0.1/analysis0.2 and live gates unchanged.
`L01-ACTIONS-001`: [CLI-ACTIONS@1](product/cli-actions.md)/CLI@6 registers first single-step Prepare/Execute syntax, exact trusted target authority, canonical compact/JSON outcome and truthful exits under [selected packet](../plans/ui-blueprint/packets/L01-actions-contract.md). Registration precedes implementation; core0.1/analysis0.2/connection1.0.0 and existing commands unchanged, private producer metadata and CLI runtime acceptance pending.
`L01-GEOMETRY-INPUT-001`: [ANALYSIS@2](product/analysis.md) adds direct observed ChannelResponse input to local measure/check under root's selected read-only geometry goal. Original Snapshot/evidence, TYPES/VALIDATION@1, core0.1/analysis0.2, output/arithmetic and inspect/diff/transport unchanged; source/runtime acceptance separate.
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
