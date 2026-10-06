# Independent S01 semantic oracles

This is authored test data, **not the UI Blueprint wire schema**. Its immediate
consumer is S01 Integration, followed by G01 geometry and K01 replay. The author
read pinned product contracts without inspecting schema/plugin implementation,
tests, actual wire fixtures or real-case agent answer keys. Runtime is synthetic.

- [Inputs](inputs.json): one shared logical context and 97 stable cases.
- [Expected answers](expected.json): outcomes indexed by the same case IDs.
- [Sources and clause coverage](sources.json): exact contract paths/revisions.
- [Packet](../../docs/plans/ui-blueprint/packets/S01-oracles.md) and
  [receipt](../../docs/plans/ui-blueprint/receipts/S01-oracles.md).

## Interpretation and Integration boundary

The `uib-semantic-oracle/1` marker identifies this local test-data notation only.
Keys here are logical facts, not proposed serialization fields, public enum
variants, JSON Schema or an alternate parser. Names such as `value_present` and
`*_present` express a test condition, never wire members. Prose references to S10,
O11, context facts and another case must be expanded by Integration into the
single canonical schema. Case-specific facts override only the explicitly named
shared facts; all other relevant preconditions remain. `same_as_case` inherits
that input, never its answer. Omission lists and variant arrays are independent
subcases, each retaining the other baseline facts; exercise every member.

For `record_contract=invalid`, the **declared claim/record with its supplied
context** is invalid. The oracle JSON itself is well-formed, even when it models
an absent member, forbidden nonfinite number, private payload or contradiction.
Negative fixtures must not inject oracle metakeys into wire objects: remove or
alter the actual relevant members. Mapping failures return to the schema owner;
expected semantics must not be weakened to fit an implementation.

`record_contract=valid` accepts the logical facts, including properly represented
unknown/error outcomes. It does not promise successful action, current data,
complete coverage, actual delivery or product acceptance. A correct operation
error envelope is valid and receives validator exit 0; malformed or contradictory
input receives 2. IO/internal failure is exit 1 under D03, outside these data-only
cases. No public CLI action exit mapping is proposed. Sanitized diagnostics go to
stderr; machine results alone go to stdout. Contextual semantic checks are distinct
from structural JSON Schema checks and need all stated fixture context.

Every case has clause IDs, preconditions/input facts, an operation/declared record,
expected semantic result, evidence requirements and a protected distinction.
Expectations contain answers solely in expected.json. Input-side normative targets
(e.g. width equals 30) are scenario premises, not observed-result answer keys.

## Required coverage

- GOLDEN01 covers S10 → prepare/resolve → delivery → S11 verification → atomic
  delta → check → a restricted compare export **record**, not DrawingBrief code.
- G01-* covers every GOLDEN negative: readonly/unsupported, changed generation,
  vanished target, duplicate labels, cancel after input, lost base, skipped revision
  and redaction, plus missing/wrong-target verification evidence.
- ENV-*-VALID/INVALID covers request, capability, session, property, observation,
  snapshot, delta, action, transition, expectation, finding and error separately.
- PROP/SNAP/DELTA/COVERAGE cases preserve false/empty, unknown/unsupported/redacted,
  not_requested, required absence, history, full replacement, compatible context,
  justified removal and atomic children/relations/focus. No omission becomes pass.
- OBS/IDENTITY/GRAPH/FORMS cases protect clock domains, freshness, scope, generations,
  many-to-many provenance, action-ref ownership, raw identities and form distinctions.
- GEO-* gives literal sizes, centers, edge gaps, alignment, containment, intersection,
  overflow, equal spacing, baselines and sourced transforms; invalid/unknown variants
  preserve finite shape/unit/context requirements and missing_transform.

Geometry arrays mean `[x, y, width, height]`, origin top-left. Shared rectangles
and rule parameters are deliberate fixture choices, not new product defaults or
accuracy claims. Ratio and area are mathematical derived quantities; source frame
units remain css_px. Integration owns their canonical representation and must not
invent `dimensionless` or squared units as new geometry-length enum variants.
Full/delta equality uses the same recorded checkpoint and coverage, never two live
captures. No global OS revision is invented. All labels, targets and sensitive
canary data are synthetic; no real application was read or changed.

S01 validation/serialization, G01 arithmetic execution, K01 replay, D02 Mac/Web
interface proof, D05 resource calibration, live delivery/privacy and P7 acceptance
remain separate work. This corpus neither performs nor claims those checks.
