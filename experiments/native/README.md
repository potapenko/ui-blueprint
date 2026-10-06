# R02 native feasibility fixture

Diagnostic support for [R02](../../docs/plans/ui-blueprint/packets/R02.md),
not a shipping plugin or complete M01–M06 acceptance harness.
All visible content and state use SwiftUI. No upstream code is copied.

`Fixture.swift` builds twice from the same source, with and without `PROBE`.
The opt-in modifier reports post-layout frames in a named local space using
`onGeometryChange`. It inserts no overlay or hit target. The off binary has no
measurement modifier. Runtime output keeps measured frames separate from explicit
component/AX mapping declarations. It never infers paint bounds or padding.
The small AppKit adapter reads only own-process launch and window metadata;
SwiftUI scene APIs do not expose the CGWindowID needed for external capture.

`Observe.swift` is a read-only, disposable helper process. It validates the
fixture bundle, PID, launch time and exact CG window owner from the manifest.
AX binds only the unique `primary` identifier declared by this owned fixture;
other AX windows are excluded. This mapping is not a general AX-to-CG window-ID
solution. No private AX window API or private process identity flavor is used.
Both permission preflights suppress prompts. Denied channels remain independent.
The tree has a 64-node/depth-6 limit, 0.2-second per-object AX timeout, 3-second
traversal admission budget and a 15-second process watchdog. These are exploratory
bounds, not C01/D05 acceptance thresholds. No global lock exists. Timeout does not
promise cancellation of an already-running OS call; helper exit bounds lifetime.
The depth-limited AX result deliberately claims partial coverage.

Capture uses ScreenCaptureKit's exact `desktopIndependentWindow`, excludes child
windows, disables audio/cursor and records filter rect/scale and buffer size.
An image is written only for the synthetic fixture. Crop mapping remains unknown
until validated; width/height ratios are not presented as calibrated transforms.
Opaque AX refs are not exported for mutation. Values/secrets are not requested.
The fixture contains only bounded synthetic text; this is not a privacy canary.

Build with the available Apple SDK, writing all products outside the repository:

```sh
experiments/native/script/build.sh /absolute/task-temp/build
mkdir -p /absolute/task-evidence/off-baseline
open -n /absolute/task-temp/build/R02-off.app --args --run-dir /absolute/task-evidence/off-baseline
/absolute/task-temp/build/r02-observe /absolute/task-evidence/off-baseline/manifest.json /absolute/task-evidence/off-baseline
```

Use the granted native lane and Computer Use for the own app only. Freeze
`manifest.json` as `frozen-manifest.json` for each observation. For each build:
observe baseline → activate sample (counter 0→1) → Change layout → observe
expanded. Stop the off build before starting on. Never call an app selector
again after closing merely to check teardown: that selector can relaunch it.
Verify process exit through the known owned PID/path. The fixture manifest task
is bounded to ten minutes; callers must still close the owned fixture.

Validate the four frozen cases:

```sh
python3 experiments/native/check.py /absolute/task-evidence
```

The check compares AX geometry/focus/actions and measured frames, plus exact
baseline PNG equality for this run. Expanded pixel equality and full hit-region
invariance remain unverified. There is no support/release promise for the 14.0
compilation target; only the measured host and SDK in the source ledger were run.
Generated bundles, compiler logs and raw downloads never belong in Git.
