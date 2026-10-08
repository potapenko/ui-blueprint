# Local measurement and check artifacts

- Node type: branch + leaf; domain: `uib.analysis`; contract: `UIB.ANALYSIS@2`; supersedes @1.
- Authority: Active / Stability: Evolving; accepted/released implementation: none.
- Authority source: ROADMAP D03 delegation under approved PLAN.UIB@1; root-accepted decision8fdf608 and [registration packet](../../plans/ui-blueprint/packets/L01-analysis-registration.md).
- Read when: local measurement/check JSON, result space, evaluation input or version integration.
- Do not read when: unchanged core transport or unrelated product behavior is sufficient.
- Requires: [types@1](analysis-types.md), [validation@1](analysis-validation.md), [CLI@6](cli.md), [EXCHANGE@1](exchange.md), [GEOMETRY@1](geometry.md), [MODEL@1](model.md), [PRIVACY@1](privacy.md).
- Implementation handoff: [exact saved decision](../../development/schema.md#l01-analysis-result-contract), checkpoint8fdf608; supporting detail, not a replacement for these clauses.

## UIB.ANALYSIS.SCOPE

Measurement is a fact with units, full selected Space, details and contributing
evidence or a typed unavailable reason. It does not invent a normative Expectation
or pass/fail. Check retains the actual sourced Expectation and bound evaluation.
Schema owns the records; the Rust engine alone computes geometry/comparisons.
Source Snapshot, times, coverage and freshness are immutable. Stored analysis is
not a fresh live observation. No model, collection, reference IO or action is added.

## UIB.ANALYSIS.VERSION

Core Document/Artifact/SchemaVersion::CURRENT, Snapshot/Context and advertised
plugin transport stay strict0.1.0. A separate schema-owned AnalysisDocument accepts
exactly0.2.0 and reuses those same canonical types, not a duplicate normalized graph.
Outer0.2 is the analysis format; embedded Context.schema_version remains0.1 and
describes the unchanged source. No coercion or relabelling. Crate package versions
are independent; this decision requires no dependency/version update by itself.
Old Document::from_json stays strict0.1; AnalysisDocument::from_json stays strict0.2.
The existing validator binary may dispatch exact versions through a bounded
schema-owned file validator. Its complete version probe skips untyped body values,
rejects duplicate version fields, then uses the chosen strict full decoder.
It never broadens plugin negotiation or resolves document paths/URLs/IDs.
Keep the core generated schema byte-identical and all126 legacy fixtures/oracle
expectations unchanged. Generate a separate analysis0.2 schema from canonical types.
Unsupported versions/artifacts/members and malformed records reject explicitly.

## UIB.ANALYSIS.CLI

After implementation, the bounded local paths are:

```text
uiblueprint measure --snapshot S --query Q --space ID --max-input-bytes N --max-output-bytes N [--evaluation E] [--json]
uiblueprint measure --snapshot S --expectation X --space ID --max-input-bytes N --max-output-bytes N [--evaluation E] [--json]
uiblueprint check --snapshot S --expectation X --space ID --max-input-bytes N --max-output-bytes N [--evaluation E] [--json --result-version 0.2.0]
```

S is core0.1 Snapshot or observed ChannelResponse containing its unchanged Snapshot;
X is core0.1 Expectation; Q/E are analysis0.2 query/evaluation documents. Failed/no-
Snapshot response or wrong artifact refuses2; full canonical validation and the same
aggregate byte bound apply. No extraction/restamping or extra files required of caller.
Measure requires exactly one of query/expectation; check requires
expectation and rejects query. The compatibility measure form extracts a factual
query from the actual expectation, without fabricating normative fields.
Without E, use snapshot binding, the explicitly selected Space, no extra transforms
and no observed conditions. ID must identify an unambiguous complete Space among
existing spaces and supplied transform endpoints, and agree with E.result_space.
All explicitly named regular input files share one aggregate byte budget. The
output budget includes newline. No embedded reference loading, file creation or
partial stdout on bounded validation/encoding failure; IO write failure remains IO.
Compact output preserves details/provenance/unknown; query mode invents no expectation.
Measure JSON emits analysis0.2 MeasurementCase. Default check JSON remains core0.1
FindingCase; explicit result-version0.2.0 emits full GeometryCheckCase. Explicit0.1.0
keeps legacy check. Result-version requires JSON; measure supports only0.2.0.
Legacy0.1 refuses nonempty extra transforms/conditions or a converted result Space;
an empty E restating source binding and the same Space remains representable.
Unsupported result versions/representations give bounded unsupported_result_version/5,
not silent downgrade or evidence loss. Compact/0.2 resolve the supported JSON gap.
Exits remain0 known/pass,1 IO/internal,2 invalid/limit,3 measured fail,4 unknown,
5 unsupported finite mode. Measurement never gains a normative pass/fail result.
The schema validator keeps flags/output/exits0/2/1; valid/0 means contract-valid
declarations, never independently recomputed arithmetic or real-world verification.

## UIB.ANALYSIS.MIGRATION

Schema implements types/strict validation/generator/accounting first; engine then
factors factual queries and re-exports canonical result types; CLI wires bound
input/output modes. Export replaces its measure-only placeholder Expectation with
GeometryQuery, preserving dimensions/evidence/statuses and DrawingBrief/package version.
K01 Snapshot keys/grants/storage and bridges do not migrate to analysis0.2. New
analysis results cannot be silently stored as Snapshots. No cache/replay algorithm,
E02 arithmetic or existing core validator behavior changes through this registration.
Shared stable-input checks cover schema/engine/CLI/export/plugin, generated schemas,
version boundaries and existing0.1 golden/bridge protocol paths before acceptance.
No live bridge rerun solely for unchanged transport. Source ownership follows finite
packets; this registration alone does not assign concurrent writers or start code.
Live collection, supplemental condition acquisition, new transform discovery,
generic diff and D05 working-memory enforcement remain separate obligations.

## Change record

`L01-ANALYSIS-001`: D03@1 unresolved local-result representation -> D03@2 plus
ANALYSIS/TYPES/VALIDATION@1. Mode Reconcile delegated technical shape/versioning;
normative authority is existing CLI/GEOMETRY/EXCHANGE/MODEL/PRIVACY and approved
ROADMAP, not a self-authorizing proposal. No released baseline or arithmetic change.
The five shared repairs and E02-R1/R2/R3 remain accepted and protected.
`L01-GEOMETRY-INPUT-001`: root selected compatible additive direct Observe input for
measure/check under approved read-only geometry goal; @2/registry18 reuses existing
local CLI snapshot loader. TYPES/VALIDATION@1, core0.1/analysis0.2, output/arithmetic,
inspect/diff/transport and original source records stay unchanged. Acceptance pending.
