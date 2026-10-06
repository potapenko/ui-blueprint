# D05 — limits, tolerances and unresolved calibration

- Domain: `uib.development.d05`; accepted/released baseline: none.
- Authority source: [C01-DEC-001](README.md#meaning-and-precedence).
- Node type: leaf; contract: `UIB.D05@1`; clause: `UIB.D05.CONTENT`.
- Authority: Active / Stability: Evolving; policy selected, numeric coverage bounded.
- Read when: S01 limits/validator or a platform/geometry acceptance test is prepared.
- Do not read when: no limits/precision/deadline choice is affected.
- Requires: [GEOMETRY@1](../../product/geometry.md), [PROJECTIONS@1](../../product/projections.md),
  [LIFECYCLE@1](../../product/lifecycle.md), [CACHE@1](../../product/cache.md),
  [PERFORMANCE@1](../../acceptance/performance.md), [evidence](evidence.md).
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

## S01 measurement status — policy unchanged

[Owned-model sizing](../../../../tests/bridges/resources/README.md) now measures
canonical P1 records and actual returned narrow Web/Native D02 samples. It counts
inline/owned container capacities, not JSON bytes or another process's RSS.
Largest observed P1 Snapshot:8,731 bytes; Web B03 Snapshot:46,184 bytes; selected
one-node Native AX Snapshot:6,694 bytes, capture metadata:5,768 bytes. Scope and
allocator/parser/pixel exclusions are explicit in the report. These are not
largest-platform/worst-permitted or total-process measurements.
No numeric production caps selected or implemented; D05-RES remains open pending
the named largest-example and working-memory evidence. This evidence/status
addition does not change CONTENT policy or advance its semantic revision.
Independent P1-R1 additionally identifies unbounded path expansion in the current
plugin depth check. It remains awaiting repair authority; small memory samples do
not close bounded-work/deadline acceptance or justify reducing limits to hide it.
