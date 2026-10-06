# Local CLI candidate contract

L01 implements the existing [CLI contract](../specs/product/cli.md) over saved
canonical data and the [G01 geometry engine](geometry.md). These finite syntax,
format and exit choices are authorized engineering decisions under L01/PLAN.UIB@1,
recorded before implementation. No new wire schema or live observer is defined.

## Finite syntax

```text
uiblueprint check --snapshot FILE --expectation FILE --space SPACE_ID --max-input-bytes N --max-output-bytes N [--json]
uiblueprint measure --snapshot FILE --expectation FILE --space SPACE_ID --max-input-bytes N --max-output-bytes N
uiblueprint --help
```

All five value flags are required exactly once; numbers are positive byte bounds.
`max-input-bytes` bounds the aggregate bytes read from both files, before parsing.
Only explicitly named local regular files are read; stdin, URLs and files named
inside documents are not loaded. No defaults for budgets, tolerance or space.
`max-output-bytes` includes the final newline. Output is prepared within that
bound; overflow produces no partial stdout. OS write failure can leave a partial
write and is an IO failure, never a successful result. Help is fixed text.

Snapshot input is a canonical `Document` containing `Artifact::Snapshot`;
expectation input is a canonical `Document` containing `Artifact::Expectation`.
Both use the existing bounded parser/validator. The rule must be Geometry and
specify its normative source, conditions, targets, relation, expected value,
comparison, quantity kind, units and tolerance. `measure` uses the same supplied
relation to compute a fact without claiming its expectation passed. It does not
synthesize a normative expectation from a request, observed value or UI state.

SPACE_ID selects an existing unambiguous full Space from the expectation's
anchors or their requested geometry/direct transform destinations in the
Snapshot. It never creates a scale/origin mapping from a string. Geometry,
space handling and all arithmetic remain G01's responsibility.

## Output contract

Compact stdout is default. It identifies `saved_snapshot` analysis, command,
snapshot/revision, target/generation, scope, coverage/selected fields, explicit
byte budgets, source Observations and their original freshness/consistency.
It includes the expectation/source/conditions and all contributing engine
measurement evidence, plus measured quantity/details or unavailable reason.
Untrusted strings use escaped representations; no terminal control text is
interpolated raw. Input paths/private payloads are absent from errors.

Check JSON is exactly the existing versioned canonical `Document` with
`Artifact::Finding(FindingCase { snapshot, expectation, finding })`; source
Snapshot and Expectation are preserved intact. Thus coverage, selection,
Observations, source evidence, generations, units/frame kinds, applicability and
tolerance remain available without a parallel CLI envelope. The document is
bounded and validated before writing. JSON check requires every anchor source
space to equal the selected result space: FindingCase cannot record a separate
CLI-selected destination. Converted results remain available in compact; JSON
conversion returns `consumer_contract_gap` until the shared record is defined.
Its stored live/cache/freshness fields are
historical input facts, not a fresh collection claim. Check exit status agrees
with its Finding. JSON stdout never contains diagnostics; failures use only a
bounded constant code on stderr and leave stdout empty before write failure.

Standalone measure JSON is not yet representable: the schema has no canonical
measurement record retaining details, result space, all contributing evidence
and unknown reason without inventing a normative pass/fail. `measure --json`
returns `consumer_contract_gap` (5). Compact measure retains this evidence.
Additional transform chains or observed applicability context need a shared
validated input record; CLI accepts neither a private format nor a guess.
An unsupported canonical Finding combination likewise reports the consumer gap,
without mutating the source Snapshot or changing the shared validator.

## Exit mapping

| Exit | Meaning |
| --- | --- |
| 0 | Check pass; or measure known; or help |
| 1 | IO/internal failure, including allocation/output write failure |
| 2 | Invalid arguments/document/shape/rule/space or exceeded explicit byte limit |
| 3 | Check fail with a real measurement |
| 4 | Check/measure unknown, including missing required applicability evidence |
| 5 | Unsupported finite command/rule/output or missing shared consumer contract |

Diagnostics are stable bounded codes such as `invalid_arguments`,
`invalid_input`, `invalid_input_file`, `input_limit`, `output_limit`, `io_error`, `invalid_geometry`,
`unknown_space`, `ambiguous_space`, `unsupported_command`, `unsupported_rule`
and `consumer_contract_gap`. No raw serde, filesystem, argument or payload error
is printed. Future observe/action/export/plugin commands are unsupported and
cannot pretend to collect or act. This does not change schema validator exits.

## Scope and checks

Only CLI source/docs change. No engine/schema/validator fix, model/SDK/runtime,
cache owner, profile meaning, external action, export or release/install claim.
Canonical GEO fixtures provide independently authored expected calculations;
binary tests cover pass/fail/unknown, invalid versions/payloads, finite byte
bounds, escaped compact strings and stdout/stderr separation. Exact preliminary
and saved-state results belong in the [L01 receipt](../plans/ui-blueprint/receipts/L01.md).
