# F01 Web fixture and exploratory baseline

## W05 public form workflow qualification

The finite [W05 harness](../../tests/bridges/web/form-e2e.cjs) composes the existing
shipping Observe/Prepare/Execute/Diff/Check/Export commands. It starts with an empty
field: Semantic Focus, Keyboard Type `Lo`, named invalid/debounce readiness, Type
`n`, named suggestion readiness, Semantic option selection, then separate Commit
and independent application-controller confirmation. All positive input uses the
public CLI; Playwright supplies exact target metadata, read-only oracles and the
explicit adversarial stimuli. No fixture fill is counted as product input.

Run from this repository with Node24.15.0, installed Playwright Core1.58.2 /
Chromium145.0.7632.6 and coherent saved Web CLI/worker products. No installation,
desktop input, model key or real application is needed. Set absolute paths and
their independently calculated SHA256 values:

```sh
S01_WEB_PLAYWRIGHT_CORE="$UIB_PLAYWRIGHT_CORE" \
UIB_WEB_LIVE_ALLOW=1 UIB_WEB_LIVE_CASE=form_e2e \
UIB_WEB_LIVE_TEST="$UIB_CLI" UIB_WEB_LIVE_TEST_SHA256="$UIB_CLI_SHA256" \
UIB_WEB_LIVE_WORKER="$UIB_WORKER" UIB_WEB_LIVE_WORKER_SHA256="$UIB_WORKER_SHA256" \
UIB_WEB_LIVE_EVIDENCE="$(node -e 'console.log(require("node:path").join(require("node:fs").realpathSync(require("node:os").tmpdir()),require("node:crypto").randomUUID()))')" \
node tests/bridges/web/guarded-live.cjs --run-authorized
```

The new UUID directory must not exist. The launcher writes its bounded report,
original canonical inputs/outputs and six-file compare package there. Consume the
report, then remove only those run-owned non-image files and empty directories;
never recursively remove a directory containing retained images. This harness
creates no screenshots. It closes its own browser/context/server/profile and
checks each CLI invocation has reaped its worker. Unknown effects stop the current
chain; a failed run is never silently retried or counted as a positive result.

The current34-call scenario verifies two scoped form states, draft/selected/applied
separation, authored32css_px Commit height, unchanged original observations,
attributed graph diff and full model-free compare package. Portal and remount are
explicit fixture stimuli; stale/disabled/private Execute refuse without delivery.
A separate result-private-after-delivery probe returns action_outcome_unknown,
reobserves redaction and performs exactly one click. Synthetic private canary
never appears in serialized command results or the package. DOM native validity
and AX aria-invalid remain separately attributed. Observe retains partial/exit4;
successful commands do not upgrade coverage or prove server/business success.

[W05 result and limits](../plans/ui-blueprint/receipts/W05-web-form-e2e.md) records
the tested source and actual run. This is author qualification of the bounded
workflow, not independent review, full P7 or D06 performance acceptance.

## Historical F01 basis

Tooling candidate for C01/W01/W02/W03/Q01/Q02. No production B01–B06 acceptance.
Authority: [approved F01 packet](../plans/ui-blueprint/packets/F01.md), PLAN.UIB@1
launch in [registry](../plans/ui-blueprint/task-registry.md), accepted R01
`71ea44235cddf63381d1fea4acd121ad642e7ddf`. Historical R01 source, receipt and report
are preserved; [fixture README](../../fixtures/web/README.md) explains composition.

The later [direct user clarification](../plans/ui-blueprint/receipts/product-context.md)
governs interpretation: explicit request → bounded current UI collection →
calculations/response, with no periodic recollection between requests. Local
inspect/measure/check can use stored snapshots; events may invalidate cache without
starting a new collection. Fixed 5 Hz, saturated polling and rAF experiments are
historical only and excluded from candidate product gates and C01 policy. Their
reports, source hashes and measured numbers remain unchanged.

## Basis and boundaries

Traversal: AGENTS → [UIB.ROUTING@1](../specs/README.md) →
[UIB.ACCEPTANCE-ROUTES@1/ROUTE](../specs/acceptance/README.md) →
[UIB.WEB-PILOTS@1](../specs/acceptance/web-pilots.md) and
[UIB.PERFORMANCE@1](../specs/acceptance/performance.md).
Complete selected `UIB.<ID>.CONTENT@1` closure: WEB-PILOTS, PILOTS, GEOMETRY,
MODEL, PROJECTIONS, IDENTITY, BOUNDARIES, FORMS, CACHE, EXCHANGE, LIFECYCLE,
PRIVACY, ACTIONS, PERFORMANCE, ROADMAP. Current full R01 content was reused;
unchanged contracts verified against the accepted checkpoint. New PERFORMANCE
and acceptance route were fully read. Source catalogs, native internals, export,
real products and future platforms are excluded. F01 packet links/closure agree;
no coordination-document correction is requested.

**Specified:** independent fixture states, exact identity, coverage/provenance,
real runtime actions separated from applied state, and per-request cold/warm
latency/cost. **Observed:** fixture/per-request evidence and explicitly historical
polling experiments below. **Proposed:** only the retained request/quality D05/D06
conditions, not new requirements or final engineering decisions.
Original F01 work evolved only its fixture/harness. This docs-only correction
removes an inapplicable experimental premise; no new product decision or spec delta.

## Scenario → expectation → observed evidence

Historical evidence command: `F01_PLAYWRIGHT_CORE=<runtime> F01_OUTPUT=<report> node fixtures/web/run.cjs`.
The unchanged combined harness also runs the historical saturation experiment;
this command is not prescribed as the current product acceptance procedure.
The [literal oracle](../../fixtures/web/expected.json) is not produced by the
collector. The retained report lists every expected/observed check and source-state
checkpoint. 58 focused checks passed, including one unchanged-state baseline check.

| Scenario | Independent expectation and actual result | Evidence boundary |
| --- | --- | --- |
| B01 | Two same-title targets have distinct CDP IDs; two main Apply names; remount changes backend ID and old handle rejects click; other target unchanged; navigation changes loader; reset reproduces semantic state with new generation | Real two-page/runtime fixture and CDP identity. Full production ref policy remains downstream; R01 separately covers cross-target/ref rejection. |
| B02 | Keyboard draft `Lo` remains unapplied during debounce; invalid commit records delivery but applied remains empty. Typing `Lon`, clicking London then Commit sets selected/applied=London. Unexpected dialog disables commit and driver performs zero dependent steps | Real keyboard/pointer API effects and separate application state; stop policy belongs to test driver, not production executor. No IME/cancellation proof. |
| B03 | Portal's parent BODY differs from form subtree; explicit anchor open-popup; focus returns on dismissal. Overlay owns hit and blocks actual pointer delivery; removing it permits exactly one delivery. Clipped child bounds `[120,400,60,30]`, intersection width 20, two hit samples agree; child-frame local box `[12,18,90,30]` | CSSOM, pointer and DOM hit-test evidence. Only same-process srcdoc; arbitrary occlusion, OOPIF and cross-frame transforms remain unverified/partial. |
| B04 | Resize 800→1000 moves responsive x660→860. Scroll 100 changes viewport y120→20. Font 16→24 changes explicit 2em×1em box 32×16→48×24. Locale fr changes label to Adaptatif while applied London persists | Explicit fixture setup for resize/scroll/font/locale and observed geometry/state; text box is not glyph contour. DPR 2 remains prior R01 evidence, not rerun here. |
| B05 | Parent width 240→300 changes child width 120→150; font 16→24 changes height 32→48. Rows become r1,r2=Deux,r4,r5,r6,r7=Seven. Source sequences 1,2,3; driver subscriber sees 1,3. Limit 2 returns partial with four omitted while six source rows remain. Same final checkpoint rereads equal | Controlled source states/journal for future replay; loss is deliberately simulated, not native event reliability. Response shaping happens after full row read; no bounded collector, cache or atomic delta implemented. |
| B06 | AX reports one button Apply for left; DOM has distinct left/icon/label backend IDs and explicit apply-control mapping. Icon is aria-hidden SPAN; action owner is left; pointer produces one delivery | Explicit DOM/AX source mapping, not inferred component identity. No normalized production graph or decorative action-ref implementation claimed. |

Useful source states: initial, draft-pending, draft-invalid, commit-rejected,
city-selected, city-applied, unexpected-transition, popup-open/closed,
overlay-on/off, text-large, locale-fr, parent-wide, font-large, rows-changed.
Each controller mutation advances revision; reset creates a fresh document and
fixture generation. B05 source-0 through source-3 and source-3-repeat give K01/W03
controlled before/after data. They do not equate two arbitrary live captures.

## Per-request baseline method and result

Run UTC 2026-10-06T10:28:08.416Z; macOS 27.0.1 build 26A434, Darwin 27.0.0 arm64,
Apple M4 Pro, 12 logical CPUs, 24 GiB RAM; Node 24.15.0, Playwright Core 1.58.2,
Chromium 145.0.7632.6 bundle 1208, CDP 1.3, browser revision
`47e20adcc15fc15f01825aa17e570c8f5492ac0f`. Fixed 800×600 CSS viewport, DPR 1.
Initial fixture: 2 documents / 97 DOM snapshot nodes / 10,788 re-serialized JSON response bytes.
No production Rust candidate existed. No screenshot/model comparison performed.

Five fresh browser/context cold samples; OS binary cache not flushed, so this is
process-cold, not machine/disk-cold. Launch/context/page setup, fixture load,
CDP attach/domain enable and first full DOMSnapshot capture timed separately.
Warm samples: 30 each, sequential semantic→geometry→full, fixed unchanged source
state, no discarded warm-up. Semantic=partial AX of #left; geometry=CSSOM rect
on resolved #left object; full=whole-fixture DOMSnapshot. Percentiles use nearest
rank; cold p95 is the maximum of only five, not a stable population tail estimate.

| Operation | p50 ms | p95 ms | JSON response bytes p50 |
| --- | ---: | ---: | ---: |
| Cold launch/context/page setup | 140.720 | 143.178 | n/a |
| Fixture load | 25.126 | 25.785 | n/a |
| Cold CDP attach/enable | 1.128 | 1.380 | n/a |
| First full capture | 1.023 | 1.439 | 10,788 |
| Cold attach+capture (excludes launch/load) | 2.114 | 2.440 | n/a |
| Total launch through first capture | 168.617 | 170.204 | n/a |
| Warm single-control semantic | 0.195 | 0.279 | 671 |
| Warm single-control geometry | 0.172 | 0.332 | 51 |
| Warm full-fixture capture | 0.404 | 0.589 | 10,788 |

Times are Node monotonic wall intervals around browser protocol calls, including
library transport; warm outer timings also include harness response accounting.
API/transport CPU cannot be independently separated here. No Rust normalize,
match/diff/check/format cost exists to measure. CDP request counts and re-serialized result JSON sizes per phase
are in report; byte counts are not network-wire measurements and exclude
Playwright driver's private protocol traffic.

## Historical saturation/rAF experiment — excluded from product gates

The following polling/animation measurements are retained facts from an unnecessary
experiment. They do not define product workload, ongoing monitoring or C01 gates.

Three paired 500ms off/on windows, order off/on, on/off, off/on. Both modes retain
the same AX/Performance domains and one rAF sampler. Off means no repeated capture;
on saturates serial **full** captures:1345,1410,1383 per window (4138 total).
This measures stress throughput/cost, not a normal polling rate or observer-free
browser. rAF gap p95 off 17.4ms / on 26.1ms; renderer TaskDuration per window
p50 off 2.864ms / on 227.659ms, p95 off 3.205ms / on 227.987ms. Aggressive full polling
has substantial measurable task cost even on this tiny fixture.

Metric provenance: [CDP Performance domain](https://github.com/ChromeDevTools/devtools-protocol/blob/d209a9a38897d2935a078a0bf00ca821811d21ed/pdl/domains/Performance.pdl)
and the tested Chromium revision's
[InspectorPerformanceAgent::getMetrics](https://github.com/chromium/chromium/blob/47e20adcc15fc15f01825aa17e570c8f5492ac0f/third_party/blink/renderer/core/inspector/inspector_performance_agent.cc)
were read. TaskDuration is timeTicks task duration (seconds converted to ms), not
CPU utilization. JSHeapUsedSize observed 2,097,700–2,150,416 bytes; Node driver RSS
178,552,832 bytes. These are distinct processes/measures, not retained graph memory.
No upstream code copied. Missing: total browser/OS RSS, product cache size/resyncs,
model tokens, actual product workload, independent transport CPU and Rust stages.

Full DOMSnapshot is explicitly whole-fixture acquisition. Filtering rows or
returning partial AX later does not fix its acquisition cost/scope. W01 must supply
bounded production acquisition or explicit unavailable coverage without widening
scope. Fast full-fixture numbers do not relax that requirement.

## Historical fixed 5 Hz supplement — excluded from C01 policy

Run UTC 2026-10-06T10:37:22.000Z, with runtime/hardware metadata checked equal to
the original run and all fixture/oracle hashes unchanged. The harness adds only
an isolated `--fixed-overhead-only` branch; original scenario/baseline function
text is byte-identical. No unchanged scenario/cold/warm/saturation checks rerun.
The original report is frozen at SHA-256
`e2af888ac0aa758edb6d9e141de8f261b1d3824e641aa8b6d50e06f671a9d123`.
Supplement links that report, retains the original harness whose hash it recorded,
and records the new harness hash; saturation data above remain historical evidence.

Method chosen before this run: four paired off/on windows, each 2,000 ms, order
off/on, on/off, off/on, on/off. On schedules 10 serial full captures at offsets
0,200,…,1800 ms; off performs none. Both retain the same AX/Performance domains
and one owned rAF sampler. Slots delayed by a full 200 ms would be skipped and
fail the schedule check, not delivered in a catch-up burst. Actual windows lasted
2,000.250–2,001.046 ms; all 40 captures occurred, maximum scheduling lag 1.525 ms.
Every response retained 2 documents / 97 nodes / 10,788 serialized JSON bytes.
18 named integrity checks passed, including unchanged source checkpoint, environment,
capture counts and rAF sample availability; additional fixture-size assertions pass.

| Measure | Off | Fixed 5 Hz on |
| --- | ---: | ---: |
| Pooled rAF gap p95, ms | 26.1 (530 intervals) | 26.2 (533 intervals) |
| Task duration normalized per 500 ms, median | 2.959 ms | 4.137 ms |
| Task duration normalized per 500 ms, p95 of 4 windows | 3.084 ms | 4.368 ms |
| Full capture round-trip p50/p95, 40 captures | n/a | 0.933 / 2.950 ms |

Within-pair rAF p95 differences were +0.1,+0.1,0,−0.1 ms. Paired task-duration
increases were 1.284,1.144,1.354,0.723 ms per 500 ms. Normalization uses measured
metrics-interval wall time, not assumed exact timer duration. Renderer JS heap
2,087,796–2,244,416 bytes; Node driver RSS 137,707,520 bytes. Missing telemetry
and TaskDuration interpretation remain as described above; report records call
counts, serialized response bytes, every schedule time and raw rAF interval.

These paired differences were below the then-proposed +5 ms rAF / +25 ms task
limits in this small exploratory run. Those proposals are withdrawn from product
gates and C01 policy under the direct user clarification; this comparison has
historical meaning only. No measured value is changed. It does not support a
monitoring requirement or demonstrate the requested per-request product behavior.
Off rAF p95 differs from the earlier saturation run's off baseline, so only paired
within-run differences support this comparison; ambient host load is uncontrolled.

## Retained per-request/quality proposals for C01 — not frozen gates

| Proposed condition | Rationale / remaining decision |
| --- | --- |
| D05: 0.5 css_px maximum absolute error for these authored axis-aligned rects, at specified viewport/DPR; unknown measurement fails positive gate | Current harness uses 0.01 arithmetic tolerance and exact literals. 0.5 is a proposed future cross-channel tolerance, not permission to guess bounds. Zoom/transforms need separate calibration. |
| D05:single-control scope plus declared context, max 32 output elements, depth 8, 250ms request deadline, 64KiB output cap | Small explicit envelope for this fixture's initial W01 slice. Acquisition must also be bounded; truncated response is partial with omissions. Proposed numbers need C01 ownership; memory retention remains unmeasured. |
| D06:warm single-control semantic/geometry response p95≤20ms each; process-cold total≤500ms and attach+capture≤50ms on this fixed environment | Measured direct baselines are below 0.34ms warm,171ms total and 2.5ms attach/capture. Extra budget accommodates future Rust/IPC/validation without assuming speedup. Evaluate only equal fresh fields and correct results. |
| D06:zero wrong targets, no stale fallback or lost required field | Correctness cannot be traded for per-request latency. The former polling/rAF conditions are withdrawn; they are not candidate gates. |

For final comparison, propose at least 20 process-cold and 100 warm samples with
the same fixture/backend/source state, measuring explicit individual requests.
Current small exploratory distribution is enough to expose mechanism cost, not
hardware portability or a promised acceleration percentage. Safari/Firefox/OOPIF,
real-product performance, privacy canaries, event recovery, cancellation and full
product pilot acceptance remain downstream.

[Terminal receipt](../plans/ui-blueprint/receipts/F01.md) records evidence retention,
exact paths, checks and checkpoint lease status. No D01–D07 was finalized.
