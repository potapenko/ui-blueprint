# Public saved relation-neighbor selection

- Node type: leaf; domain: `uib.cli.neighbors`; contract: `UIB.CLI-NEIGHBORS@1`.
- Authority: Active / Stability: Evolving; accepted/released implementation: none.
- Authority source: G10 section of [G02 packet](../../plans/ui-blueprint/packets/G02-scope.md), approved PLAN.UIB@1 P3/P6; delegated CLI representation of existing PROJECTIONS.CONTENT.
- Read when: public explicit-relation neighbors on saved canonical input.
- Do not read when: unrelated inspection/collection/geometry/action behavior suffices.
- Requires: [PROJECTIONS@1](projections.md), [MODEL@1](model.md), [IDENTITY@1](identity.md), [EXCHANGE@1](exchange.md), [PRIVACY@1](privacy.md).
- Precedence: [registry](../README.md); existing CLI commands, core0.1/analysis0.2 unchanged.

## UIB.CLI-NEIGHBORS.INPUT

```text
neighbors --snapshot FILE --ref SOURCE_KEY_JSON --max-relations N --max-input-bytes N --max-output-bytes N [--json]
```

All value flags required once. SourceKey JSON is the strict canonical
{namespace,key}; no name/box matching or action ref. Input is a local regular
core0.1 Snapshot or observed ChannelResponse, via the same loader as inspect.
Selector UTF-8 bytes and file bytes share a positive aggregate max-input-bytes.
Positive max-output-bytes includes newline and bounds complete publication.
max-relations is an explicit nonnegative usize cap, including zero; no hidden
selection default. Duplicate/unknown flags, invalid selector/source and failed
ChannelResponse reject2; missing exact seed returns target_unresolved/4; IO1.

Use existing borrowed scope::relation_neighbors with NeighborLimits only. Select
one step of source-order incident relations; preserve incoming/outgoing/self_loop,
repeated edges/counterparts, namespaces and Evidence. No spatial inference,
recursive expansion, identity merge, new graph, stable action authority or live
revalidation. Children/component membership do not silently become relations.
The unchanged original Snapshot remains separate from the bounded selection.

## UIB.CLI-NEIGHBORS.OUTPUT

Successful selection exits0 even for source partial/unknown or a truncated
selection: output explicitly states both; success does not mean full UI coverage.
Compact output includes saved source context/Observations/coverage, exact seed,
selection cap/returned/omitted/truncated, and each direction/Relation/counterpart
Node. Untrusted strings are escaped. No inferred UI labels or fresh timestamps.

JSON is a CLI-owned envelope with exactly output_version="1.0.0",
kind="relation_neighbors", source="saved", live_revalidated=false, selector
(canonical SourceKey), snapshot (full original canonical Snapshot), selection
{max_relations,returned_relations,omitted_relations,truncated}, neighbors
[{direction:"incoming"|"outgoing"|"self_loop",relation:Relation,counterpart:Node}].
Serialize borrowed canonical values; no second normalized wire schema or parser.
Repeated counterpart nodes preserve one-entry-per-relation semantics. Selection
counts apply only to incident relations already present, not omitted runtime UI.
The cap does not truncate the source Snapshot included for provenance; the output
byte cap can therefore reject an otherwise small selection with output_limit/2.

No input/reference/payload file loading beyond the explicit source, collection,
filesystem output, action or model call. Canonical privacy validation precedes
publication; errors are bounded constant codes and no partial stdout escapes
validation/encoding failure. Physical stdout failure remains IO1. Existing
inspect envelope1.0.0 and compact behavior, G09 export, analysis and diff unchanged.
This selection does not implement full interaction/design projection or heuristic
neighbors. Registration is not runtime or independent acceptance.

`G10-NEIGHBORS-001`: additive public bounded context caller for accepted G02 API;
no change to engine selection/validation, geometry, Snapshot meaning or collectors.
