# D05 — limits, tolerances and unresolved calibration

- Domain: `uib.development.d05`; accepted/released baseline: none.
- Authority source: [C01-DEC-001](README.md#meaning-and-precedence), [D05-RET-002 / PARTITION-001](d05-memory.md#change-record), [D05-WORK-001](d05-working-memory.md).
- Node type: leaf; contract: `UIB.D05@3`; clause: `UIB.D05.CONTENT`; supersedes @2.
- Authority: Active / Stability: Evolving; policy selected, numeric coverage bounded.
- Read when: S01 limits/validator or a platform/geometry acceptance test is prepared.
- Do not read when: no limits/precision/deadline choice is affected.
- Requires: [GEOMETRY@1](../../product/geometry.md), [PROJECTIONS@1](../../product/projections.md),
  [LIFECYCLE@1](../../product/lifecycle.md), [CACHE@1](../../product/cache.md),
  [PERFORMANCE@1](../../acceptance/performance.md), [evidence](evidence.md),
  [D05-MEMORY@2](d05-memory.md), [D05-WORK@1](d05-working-memory.md) for host ownership/enforcement.
- Owner/deadline: S01 sizing before W01/M01; W01/M01/P01 calibration before
  corresponding tests; K01 implements the established retention bounds.

## Chosen policy and evidence boundary

Every external request has explicit scope, fields, depth/node/output limits and
deadline. P1 does not hide unmeasured numeric defaults; missing mandatory budgets
are invalid requests. Adapter reports actual visited/returned/omitted/unknown
counts where known; post-hoc output truncation is not bounded acquisition.
Deadline uses monotonic time and cancellation stops new work. Saturation, sleeps
for a presumed stable UI and the F02 45-second watchdog are not product defaults.

| Bound or tolerance | Decision now | Evidence and scope |
| --- | --- | --- |
| Web initial single-control scenario | Caller requests at most 32 output nodes, depth8, 64KiB JSON and 250ms warm request deadline | F01 proposal with tiny known fixture; W01 must additionally bound acquisition and mark truncation partial. These are scenario parameters, not all-product defaults |
| Native fixture diagnostic scope | Caller requests at most 160 nodes, depth9, exact selected window | F02 observed 74–75 nodes; external semantics partial. No application-wide expansion or completeness claim |
| Native channel test budgets | Adopt explicit 1s AX / 2s capture as initial deadline-test parameters, with parent deadline specified separately | F02 proposal versus observed AX44ms/capture87ms warm p95; these bound failure, not latency gates. M01 must prove cleanup and preserve completed channels |
| Web known authored rect comparison | Keep existing 0.01 css_px arithmetic epsilon for the R01/F01 literal fixtures | A numeric comparison allowance, not a claim of general measurement accuracy; DPR1/2 observed, zoom/unresolved transforms excluded |
| Native pt/probe and frame px | Preserve units; **no cross-channel accuracy allowance frozen yet** | Proposed 1pt lacks final calibrated transform/current probe proof; unknown mapping cannot be absorbed by tolerance |
| Future 0.5css_px cross-channel tolerance | Not adopted as measured precision | F01 proposal exceeds literal epsilon but lacks zoom/transform calibration |

Tolerance in each Expectation is explicit, with expected_from and applies_when.
No Galen 2px containment allowance, ±1px spacing default or decimal truncation.
Missing required data is unknown and leaves a positive gate open. Frame kind is
part of the comparison: AX bounds do not stand in for layout/hit/paint geometry.

## Required numeric/proof decisions before dependent work

`D05-RES`, owner S01, **before first live W01/M01 adapter**: measure in-memory
representations of the largest P1/F01/F02 normalized examples, including relations
and unavailable properties; choose and record maximum retained revisions, per-
session/process byte caps, queue admission and response-frame byte caps. Include
overflow/oversize/eviction tests and a bounded resync. Raw F02 JSON ~101.5KB and
helper RSS38.4MB do not measure a Rust cache. Do not invent a cache cap from RSS,
make memory unbounded, or defer this decision until after live adapters start.
S01's initial executable examples carry explicit finite per-test limits; they are
not an accepted production memory budget. K01 uses the chosen policy rather than
selecting new numbers to accommodate its implementation.

`D05-NATIVE`, owner M01 before first capture pilot: explicit overall request and
owned-child cleanup bounds; timeout after AX completion must return preserved AX
and capture timeout, stop/reap only owned resources, suppress late success and
allow unrelated AX to progress. Reproduce/diagnose F02 continuation failure before
claiming concurrent capture. `D05-PROBE`, owner P01 before M05: calibrate transforms
and choose justified comparison tolerance before evaluating fixed candidate;
hold active/key/main and relevant focus state through matched off/on acquisition.

`D05-WEB`, owner W01 before affected B03/B04 checks: bound selected DOM/AX reads,
calibrate zoom/frame/scroll mappings; frame dimensions alone cannot prove mapping.
Each unresolved item needs named evidence/parameters before its test, not larger
tolerance or a timeout chosen after a failing product result. Existing positive
pilots and privacy requirements remain unchanged.

## Retained decision and remaining proof

[Owned-model sizing](../../../../tests/bridges/resources/README.md) and
[working-allocation evidence](../../../../tests/bridges/resources/working-memory.md)
distinguish owned capacities from wire bytes, parser/encoding/framing work and RSS.
Larger actual32-node Web/76-node Native inputs remain partial; configured synthetic
trees/list payloads are labelled synthetic. No universal decoder bound is claimed.
`D05-RET-002` selected retained limits/admission/lifetime before K01. Reviewed
designdb629fc now supplies D05-WORK-001/D05-PARTITION-001: actual worker allocation
guard, fixed parent pools and supervised retained partitions, through D05-WORK@1,
D05-MEMORY@2 and D02@2. This registers technical choices, not runtime acceptance.
Scenario scopes/deadlines/tolerances, core0.1/analysis0.2 shapes and product authority
are unchanged. Accepted shared repairs remain protected; small memory examples
cannot independently prove bounded work or justify easier request limits.

Full D05-RES remains open before live W01/M01: decoder/replay/encoding allocation
enforcement, framing and completed-channel owners, aggregate working-memory quotas,
actual cleanup and bounded resync proof. A host permit or retained counter alone
does not enforce transient allocations. The reviewed reusable-worker design is now
adopted under [registration authority](../../../plans/ui-blueprint/packets/D05-runtime-registration.md),
before source work; platform/allocator/publication and unchanged D06 gates stay open.
