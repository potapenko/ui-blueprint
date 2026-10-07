# Recorded comparison CLI output

- Node type: leaf; domain: `uib.cli.recorded-diff`.
- Contract: `UIB.CLI-DIFF@2`; clauses: `.BEHAVIOR`, `.DATA`, `.FAILURE`; supersedes @1.
- Authority: Active; stability: Evolving; accepted/released baseline: none.
- Authority source: approved PLAN.UIB@1/ROADMAP delegated representation, [selected L01 packet](../../plans/ui-blueprint/packets/L01-recorded-diff.md); no deletion or live-observation authority.
- Read when: implementing/using bounded recorded CLI diff.
- Do not read when: unchanged inspect/observe/analysis is sufficient.
- Requires: [CLI@5](cli.md), [CACHE@1](cache.md), [IDENTITY@1](identity.md), [MODEL@1](model.md), [EXCHANGE@1](exchange.md), [PRIVACY@1](privacy.md), [GEOMETRY@1](geometry.md), [BOUNDARIES@1](boundaries.md).
- Original CLI CONTENT/INSPECT/OBSERVE, strict CACHE/Delta compatibility and core/analysis wire protected.

## UIB.CLI-DIFF.BEHAVIOR

```text
diff --before FILE --after FILE --max-input-bytes N --max-output-bytes N --max-entries N [--json]
```

Both explicitly named files share one positive aggregate input-byte bound. Output
bound is positive and includes newline. max-entries is required/nonnegative;0
produces only omission information. Input accepts validated core0.1 Snapshot or
observed ChannelResponse containing its unchanged Snapshot, without reference IO.
Use existing engine compare_recorded, exact SourceKey/Field and matching session,
schema/plugin/Target/Surface generations, scope/projection/field set. Environment
revisions may differ for literal recorded comparison; preserve both environments,
geometry kinds/units/Spaces/transforms. No common transform, coordinate arithmetic
or normalized displacement is inferred. Different units/frame kinds remain two
attributed records. Schema contexts_compatible/CACHE/Delta stay strict and unchanged.
Preserve original source coverage/uncertainty/Evidence and both complete records.
Content/value/availability and Evidence/Observation-only changes are separate.
Missing-side classification means absent in that record, never deleted/created,
known empty or action authority. Known→Unknown does not inherit an old value.

This scope is node presence and properties only. Relations, children, node metadata,
focus and full graph comparison remain outside it; no Delta/Removal/changes stream,
cache mutation, heuristic identity or live revalidation. Compact states recorded
source and limited comparison scope, presents the same flags, source coverage and
before/after availability; unexamined metadata is not claimed unchanged.

## UIB.CLI-DIFF.DATA

JSON is a CLI-owned object with exactly output_version="1.0.0",
kind="recorded_difference", source="saved", live_revalidated=false,
comparison_scope="node_presence_and_properties", before/after (full unchanged
canonical Snapshots), entries (array), omitted_entries (nonnegative count).
Each entry contains exactly kind="node_presence"|"property", key (SourceKey),
field (canonical Field or null for node presence), before_present/after_present
(booleans), content_changed/evidence_changed (booleans). Node-presence entries
use content_changed=true/evidence_changed=false; property flags come from engine.
Original node/property/Evidence is resolved by exact key/field within before/after;
no second graph or copied normalization owner is encoded. Preserve engine order.
Serialize borrowed canonical data through bounded output, with one final newline.
output_version applies only to this local report; core0.1/analysis0.2 stay unchanged.
No imported-report parser/schema framework. Future output changes are versioned.

## UIB.CLI-DIFF.FAILURE

Complete report returns0 whether differences exist or not; it is not normative UI
pass/fail or proof of source continuity/atomicity. Truncated report returns4 with
actual omitted_entries independently of source coverage. Individually valid but
incompatible Context returns4/context_mismatch with no output/coercion. Invalid
arguments/input/byte limits2, IO/allocation1, unsupported modes5. Output overflow
has no prepublication partial stdout; physical write failure may be partial/IO1.
Diagnostics remain bounded constant codes on stderr, without raw input/path text.

Acceptance: literal32→48 + partial absent B, Known→Unknown, evidence-only updates,
namespace/context separation, explicit0/small entry cap, exact source Snapshot JSON
equality, aggregate input/output boundaries and preserved existing CLI routing.
Author runtime evidence is recorded separately; registration itself is not acceptance.

Change record: @1 copied CACHE/Delta environment equality into read-only comparison
and rejected the actual font-size change pair. Root/Core/original Web advisor
reconciled this technical overconstraint under [packet amendment](../../plans/ui-blueprint/packets/L01-recorded-diff.md#reconcile-environment-only-context-changes):
BOUNDARIES requires change→diff, GEOMETRY preserves resize/font contexts. @2 permits
environment differences only for recorded comparison with the protected bindings
above. No source restamping, deletion, cache/replay relaxation or wire migration.
