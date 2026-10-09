# D06 — frozen P0 per-request performance gates

- Domain: `uib.development.d06`; accepted/released baseline: none.
- Authority source: [C01-DEC-001](README.md#meaning-and-precedence).
- Node type: leaf; contract: `UIB.D06@2`; clauses: `UIB.D06.CONTENT`, `UIB.D06.REQUEST-INPUT`; supersedes @1 workload routing only.
- Authority: Active / Stability: Evolving; **P0 decision closed**, no candidate pass.
- Read when: instrumenting W01/M01 or performing Q02 comparison.
- Do not read when: proposing periodic monitoring or an unrelated workload.
- Requires: [PERFORMANCE@1](../../acceptance/performance.md),
  [PILOTS@1](../../acceptance/pilots.md), [D05](d05-limits.md), [evidence](evidence.md).
- Conditional Native request input: [UIB.D06-NATIVE-REQUEST@1](d06-native-request.md).
- Owner/deadline: C01 freezes thresholds before candidate evaluation; Q02 evaluates
  fixed post-Q01 candidate; adapter owners expose timing/quality evidence.

## Scope, rationale and frozen numbers

Flow: caller requests once, bounded current collection/calculation returns once.
No fixed Hz, saturated polling, rAF/animation or cadence gate. Historical such
numbers are retained but excluded. Baselines below are exploratory API/process
cost, not UI Blueprint implementation results. Adopt F01/F02's pre-candidate
per-request proposals as initial budgets: room for transport/normalization/check/
serialization, not a promised speedup or a statistically inferred percentile.

Both baselines: macOS27.0.1 (26A434), arm64, same operator development host; F01
records Apple M4 Pro/12 logical CPUs/24GiB. Ambient OS caches/load uncontrolled.
Only that environment/workload receives these gates; no hardware-general claim.

| Workload / baseline p95 | Frozen candidate threshold |
| --- | --- |
| Web F01: Chromium145.0.7632.6/CDP1.3, 800×600/DPR1, 2 documents/97 DOM nodes; warm addressed single-control AX 0.279ms, geometry 0.332ms (30 samples each) | Warm complete **requested** single-control semantic response p95≤20ms and geometry response p95≤20ms |
| Web cold: attach+first full fixture capture2.440ms; process launch through capture170.204ms (5 samples) | Attach+first equivalent requested response≤50ms p95; process-cold full fixture setup through first response≤500ms p95 |
| Native F02 expanded A, 550×525pt, isolated1100×1050px, AX depth9/cap160; cold74–75 nodes, warm75; AX coverage partial; Swift6.4/SDK27 | Same bounded channel workload, not a complete design graph |
| Native warm AX43.8ms, capture86.6ms, combined127.5ms worst off/on p95 (19 samples per mode) | AX stage p95≤100ms; capture stage p95≤200ms; combined request-to-normalized-response p95≤300ms |
| Native fresh helper process wall296.9ms worst off/on p95 (5 samples per mode) | Process-cold request through normalized response p95≤750ms |

Web full-fixture cold comparison is an explicitly authorized **whole fixture**
workload, never a substitute for bounded single-control acquisition. Stage timing
alone cannot pass an outer response gate. Native capture stage includes inventory,
image acquisition and first-image output, matching baseline; no free omission of
pixels/output cost. New Rust normalization/formatting must be separately timed and
included in the outer response. Native baseline precedes the one-shot publisher
correction, so it informs budgets only; candidate run uses current explicit-request
fixture. Partial native coverage is retained and cannot close full pilot gates.

## Fixed comparison procedure and non-negotiable quality

Q02 runs at least 20 process-cold and 100 warm explicit calls per selected workload,
one request at a time, caller-driven with no fixed-rate scheduler or work between
requests. Separate off/on and current/partial cohorts; do not pool them. Warm
reuses attachment, **collects current requested data**, and does not substitute a
cached graph. Cold includes process startup; report OS caches were not flushed.
Report raw sample counts, p50 and nearest-rank p95 without dropping outliers.
Timeout/failure is a failed request and quality result, never a censored fast sample.
Pin candidate/fixture hashes, environment/backend versions, source state/fields,
viewport/window/pixel sizes and limits before starting; match input/output coverage.
Unexpected node-count changes need explanation, not a silently cheaper sample.

Zero wrong targets/stale fallback, secret leakage, missing required known fields,
false pass on unknown or incorrect outcomes. Compare all applicable literal fixture
expectations before counting a timing as a successful response. Partial/degraded
cases have separate latency and quality reports; they cannot replace positive
Mac/Web/M05 gates. Full/delta equality uses one controlled checkpoint, not two live
captures. Read-only observation must not change focus/scroll/values/layout.

Report request API/transport/Rust normalization/match/diff/check/format intervals,
system-call counts, bytes, memory/cache high-water and resync count where measurable.
Report unavailable telemetry explicitly; helper RSS is not cache size. Snapshot
analysis has no fresh collection cost. No fabricated substage sum for overlapping
intervals or shared clock domain. No numeric Rust-stage/memory gate is derived from
absent measurements; D05-RES fixes resource caps before live implementation.

## Remaining evidence, not latitude to weaken these gates

F02 concurrent capture fails 124: M01 must fix/prove bounded independent-channel
progress before Q02; successful sequential numbers do not prove isolation.
Current probe overhead/invariance and calibrated transforms remain P01, not an
inferred percent overhead from historical reactive callbacks. Q02 compares the
available current workflow, UI Blueprint and screenshot-only where useful under
equal tasks; semantic+crop usefulness/agent tokens need F03/Q03 evidence, no made-up
speedup/accuracy percentage now. New workloads need a separately versioned baseline
and thresholds before their candidate evaluation; these gates cannot be silently
expanded or relaxed after failure. Any change records reason, authority and revision.

## UIB.D06.REQUEST-INPUT — Q02-NATIVE-REQUEST-001
Explicit user benchmark/launch instruction2026-10-09 and ROADMAP D06 register
[Native request-only@1](d06-native-request.md) BEFORE candidate evaluation.
Controlled017d85c proves a real76-node/Snapshot/OpenB/context delta; original75-node
evidence is preserved, not called equivalent. New input receives fresh direct API
baseline, complete off/on cohorts and ALL SAME100/200/300/750ms numeric gates.
This registration changes only selected workload/reference; protected CONTENT
quality/cost/privacy/bounds and all Web results remain. No runtime acceptance.
