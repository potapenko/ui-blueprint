# F02 native fixture and exploratory baseline

Tooling for C01 D01/D05/D06, M01/M02/M03/P01/Q01/Q02 under the approved
[PLAN.UIB@1 packet](../plans/ui-blueprint/packets/F02.md). R02 accepted input:
`19cd1359ff4c239695f5da21f90d101d878c067d`. Preserve its
[ledger](../research/R02-native.md), receipt and application-state evidence.
Canonical source and commands: [fixtures/native](../../fixtures/native/README.md).
No product plugin, shared schema, SDK or Rust analytics is delivered by F02.

## Basis, authority and current execution model

Traversal: AGENTS → specs/README (`UIB.ROUTING@1`) → acceptance/README
(`UIB.ACCEPTANCE-ROUTES@1`) → NATIVE-PILOTS@1 and PERFORMANCE@1, all
`.CONTENT` clauses and explicit requires. Reused the fully read R02 closure:
PILOTS, NATIVE, FORMS, CACHE, ACTIONS, EXCHANGE, GEOMETRY, PROJECTIONS, LIFECYCLE,
PRIVACY, MODEL, IDENTITY, BOUNDARIES and ROADMAP @1. Catalogs remain source evidence;
no new borrowed code. Apple/Computer Use/QA/operational governance and SwiftUI,
build/run, window-management and narrow AppKit interop instructions applied.
Read packet links/requirements: valid; no coordination-document repairs needed.
Excluded browser internals, real products, mobile/ML, shared production APIs,
D01–D07 finalization and full pilot acceptance.

During F02, the user explicitly corrected the operating model: take an interface,
request its data, receive it and send it to the UI; no background monitoring.
That direct instruction governs the final harness and the product consultation
confirms the existing one-shot primary path; no broad normative rewrite or SC01
dependency is implied. The earlier diagnostic fixture published a manifest every
0.5 seconds.
That implementation was removed. The canonical fixture now has a Snapshot button
and resolves probe anchors once per explicit request, with no autonomous collector.
Old records remain historical. Per-call latency samples remain exploratory API
measurements under their recorded environment; they do not establish cadence,
subscription, 2 Hz/5 Hz or animation-overhead policy. Periodic-publication overhead
and earlier reactive-probe callback totals are **not used for C01 decisions**.

## Fixture, expected transitions and boundaries

[expectations.json](../../fixtures/native/expectations.json) was written before
runtime tests. It defines expected counter increments, names/form outcomes,
8/18 pt gaps, 40 rows, 550/650 pt widths and synthetic stimuli. Measurements are
read from runtime; the declared spacing is not serialized as measured geometry.
SwiftUI owns both same-title windows, all controls and their state. A read-only
AppKit adapter reports own window IDs/launch state. Move/Compare additionally
use a narrow explicit setup adapter: public SwiftUI initial/zoom placement does
not reposition an existing window on the selected macOS 14 compilation target.
No AppKit-authored UI, global installation, signing identity or display change.

| Scenario | Independent expectation | Observed result / evidence case | Limitation |
| --- | --- | --- | --- |
| M01 same-title targets | A and B have distinct IDs; activating B changes only B | `two-windows-a/b`: A=5, B=1, distinct CG IDs; real Computer Use activation | Exact synthetic IDs, not arbitrary-app matching |
| M01 resize | width 550→650 pt, same B identity | `resized-b`: runtime width 650; local sample unchanged | One host/display |
| M01 move | window translation must not change local sample | Two title drags had no effect; `final-before/after-move` verifies explicit Move +40,-20 pt and identical probe frames | Setup adapter success is not proof of physical titlebar drag |
| M01 close/reopen | old surface receipt rejected even with reused CG ID | Initial bug found; fixed willClose generation rotation. `stale-reopen-final` rejects old receipt, `reopen-final` succeeds, same CG ID 2731 | In final one-shot workflow request Snapshot after reopen; file alone is not live revalidation |
| M02 form | incomplete→invalid; typed/completed name + masked input + check→accepted-b | `form-accepted`: real typed Ad, Complete Ada, synthetic secure input, checked, visible accepted-b | App completion only; IME and adversarial input-owner interruption unverified |
| M03 popup | declared anchor B; confirm produces popup-b | `popup-open`, `scroll-end`: popover semantics and visible confirmed result | Parent isolated capture excludes separate popup pixels; full M03 pixel gate open |
| M04 clipping | scroll end displays final rows | CUA screenshot rows 36–39; `scroll-end` scrollbar ~0.992 and source state end | No calibrated capture transform or cross-display proof |
| M05 off/on | equal reported outer geometry; internal layout only on | `off/on-baseline`, `expanded`, `hit`: AX parity; gap 8→18, icon 26×24, text 115.5×19→151.5×26 | Earlier reactive-probe revision; final one-shot probe has separate checks below |
| M05 sampled hits | four inside points increment, two outside points do not | Both builds: count chain 1,2,3,3,4,5,5 at six recorded screenshot points | Expanded sample only; not full continuous hit-region or final anchor-background invariance |
| M05 normalized pixel repeat | same role/content/frame/focus/key context before comparing | `normalized-comparison.json`: A, state and 550×525 frame at 40,465 match; off active/key/main=true, on=false | Not comparable; not evidence of probe interference |
| M06 unsupported | failed attribute does not erase valid attributes | AXF02Unsupported reported unsupported, other properties retained | Synthetic unsupported attribute name |
| M06 slow/partial/loss/denied | bounded delay, truncated scope, source/event mismatch, channel refusals | `slow-b`, `partial-b` (8 nodes), `lost-event-b` (15 vs 14), `permission-injected-b` | Explicit synthetic stimuli, no real TCC change or lost OS notification proof |
| M06 timeout | owned request stops at deadline | `driver-timeout`: helper killed/reaped at requested driver deadline | Not a deliberately hung AX server |
| M06 concurrent requests | independent sessions complete within their bounds | `concurrent-summary.json`: both capture helpers emitted continuation-leak diagnostic and exited 124 by 45s watchdog | **Real unresolved concurrent capture failure**; no per-call AX output persisted before blocked capture |
| Explicit one-shot request | no output until Snapshot; no update between requests; next Snapshot reflects changed UI | `request-initial`, `request-updated`, `request-validation.json` | Fixture-only protocol, not frozen production transport |

First one-shot anchor attempt returned container only because the parent preference
replaced children; those `oneshot-*` records are rejected measurement evidence.
The correction uses transformAnchorPreference to merge markers. Current anchors
are resolved in a transparent, non-hit-testable/non-accessible background only
when Snapshot is requested. New full off/on pixel/hit acceptance remains P01;
previous onGeometryChange results are not silently promoted to the new mechanism.

The one bounded normalized pixel repeat requested by root matched all recorded
content/geometry/focus state but not active/key/main. Compare explicitly requests
own-window key activation, yet the captured intervals differed. The cause of that
focus ownership difference was not investigated outside fixture scope. P01 needs
same active/key interval held through frame acquisition and checked before/after;
otherwise return interrupted/not-comparable. No threshold was weakened and no
unbounded pixel retries were run.

## Per-request exploratory performance

macOS 27.0.1 (26A434), arm64, Apple Swift 6.4, SDK 27.0; compiled Swift mode 6,
macOS 14.0 target. Runtime support on older OS is unverified. The fixed measured
workload was expanded A: 75 AX nodes, depth/node bounds 9/160, 40 scroll rows,
550×525 pt window, isolated 1100×1050 px capture. No model or external CLI stack.

Five cold samples each launched a new helper; 19 warm samples followed its first
request in one explicitly bounded 20-request batch. Cold is process/first API
request, not a rebooted WindowServer or cold filesystem. Warm still performs
fresh AX traversal and ScreenCaptureKit inventory; it is not cached graph speed.
p50 is median, p95 nearest-rank (small-sample p95=max). Times below are milliseconds.

| Mode / metric | cold p50 / p95 (n=5) | warm p50 / p95 (n=19) |
| --- | --- | --- |
| off AX | 73.6 / 78.2 | 39.1 / 43.8 |
| on AX | 71.7 / 86.7 | 37.5 / 42.6 |
| off capture | 143.0 / 147.4 | 75.6 / 80.0 |
| on capture | 142.3 / 153.7 | 76.9 / 86.6 |
| off total in helper | 218.7 / 222.1 | 114.2 / 122.2 |
| on total in helper | 219.9 / 228.6 | 115.3 / 127.5 |

New-process end-to-end wall p50/p95: off 288.7/293.1 ms, on 286.9/296.9 ms.
JSON response ~101,448–101,451 bytes; helper peak RSS ~38.3–38.4 MB. Five cold
helper CPU times had medians 57.4 ms off and 56.2 ms on; the 20-request warm batch
used ~357 ms CPU in either case. RSS is Python RUSAGE_CHILDREN high-water for the
driver's children; it is not the fixture's memory or a retained-graph/cache budget.
Capture timing includes inventory, image acquisition and first-image PNG output;
not GPU-only timing. Separate Rust normalize/diff/format stages do not exist here.

Synthetic +300 ms AX delay: warm AX p50/p95 342.9/347.1 ms (n=5), capture
88.1/90.2 ms. Partial eight-node sample: AX 9.9/12.2 ms, capture 76.9/82.8 ms.
Reduced coverage is not a quality-preserving speed gain. Concurrent full capture
failed; do not infer independent-session support from these sequential numbers.
Only one host and small sample sets were measured. No statistically justified
probe-overhead percentage, UI frame-time claim or cadence policy follows.
These samples predate the explicit Snapshot publisher correction; their backend
per-call timings remain useful exploration, not final P7 candidate benchmarks.

### Proposals for C01, not frozen gates

- D05: use pt for local fixture geometry and px only for buffer geometry. Start
  with 1 pt tolerance for declared AX/probe local comparisons at the observed 2x
  context; unknown transforms remain unknown, never covered by tolerance. Keep
  160-node/depth-9 caps as an explicit partial window diagnostic, not app-wide scan.
- D05: retain explicit per-request deadlines. Consider a 1 s AX-channel and 2 s
  capture-channel exploratory budget with a separate hard process lifetime cap;
  validate slow/unsupported cases before adopting them. The current 45 s watchdog
  exposed a hang; it is not a proposed interactive latency target.
- D06: candidate one-shot p95 limits for this exact workload: cold process wall
  750 ms; warm AX 100 ms, capture 200 ms, combined 300 ms. These leave room above
  the observed baseline, need C01 freezing and later fixed-candidate remeasurement.
- C01/M01: keep AX progress independent from capture lifetime; publish/retain
  completed channel evidence before an unresponsive capture. Scope any capture
  serialization to that resource; do not hold a global queue across AX/capture.
  Investigate the observed ScreenCaptureKit continuation leak before promising
  concurrent capture. No backend switch or permission bypass was attempted.

## Checks and ownership

Focused off/on/helper builds passed, including the explicit-request source.
`capture.py` records driver timeout and helper failure without claiming success.
Its exit-124 error recording was tested with a task-owned stub, not presented as
native runtime proof. `check.py` validates only recorded fixture claims, redacted
JSON canary handling and explicit residual classification. Documentation links,
Python/shell syntax and whitespace are checked separately. No unrelated suite,
root Cargo, registry, original R02 file or real-product state was changed.

Runtime resources stopped; native desktop/focus lane released. The finite tooling
checkpoint is saved under root's serialized Git lease; product acceptance remains
separate. Evidence/retention details are in the
[F02 receipt](../plans/ui-blueprint/receipts/F02.md).
