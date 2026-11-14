# UI Blueprint SwiftUI probe

`UIBlueprintSwiftUI` is an opt-in macOS14+ Swift package for selected internal
layout boundaries in an application's real SwiftUI view. All implementation is
`#if DEBUG`; Release builds expose no probe API. It has no external dependencies.
The app admits requests and owns any private output file. The SDK has no file,
network, Accessibility or AppKit access and never reads labels or input values.

## Connect a real app

Add this directory as a local Swift package and link its library to the explicit
Debug consumer. Gate imports, stored probe state and modifiers with `#if DEBUG`.
Do not change ordinary Run schemes, signing, entitlements or production endpoints.
Initialize one session in the window's retained Debug owner, never inside `body`:

```swift
let session = try BlueprintProbeSession(
    targetID: "me.example.app", targetGeneration: runGeneration,
    surfaceID: "learner-window-recovery", scopeID: "recovery",
    rootMarkerID: "recovery",
    requiredMarkerIDs: ["card", "icon", "text-group", "title", "detail", "retry"]
)
```

`runGeneration` is a separate UUID bound by the app to its current run. Never
export an ingress credential/launch token as an identity. All supplied IDs are
public ASCII `A-Z a-z 0-9 . _ -`, 1–96 bytes. The root ID is automatic; exclude it
from `requiredMarkerIDs`. The root plus children cannot exceed256 markers.

Apply `.blueprintProbeRoot(session: session)` to the existing full selected root.
Apply `.blueprintMarker("card", parent: "recovery")` to the actual card boundary,
and equivalent markers to its real inner parts. Declare title/detail's parent
as `text-group`, for example. Parents must form one tree reaching the root.
These are explicit marker relationships, not discovery of every SwiftUI view.
The marker adds no sizing, padding, hit area, focus or Accessibility element.

## Request and consume

Call on the main actor after the selected root appears (`session.isReady`):

```swift
let snapshot = try await session.capture(
    requestID: requestID, stateID: currentPublicStateID,
    expectedMounts: [:], timeoutMilliseconds: 1000, maxOutputBytes: 1_048_576
)
```

An explicit main-actor event reaches the existing SwiftUI geometry callback and
resolves selected bounds anchors without changing observable UI state. Mount
identity renewal also does not request rendering; old anchor generations refuse.
The app must check `session.isCurrentMounts(snapshot)` and its own selected state
again after awaiting capture and immediately before publication. This rejects
root/child invalidation that occurred after the SDK resumed the continuation;
it does not turn saved geometry into a current or atomic frame observation.
The returned `snapshot.data` is ready for the existing canonical core0.1 consumers:

```sh
uiblueprint inspect --snapshot captured.json --ref SOURCE_KEY_JSON --view design --max-input-bytes 1048576 --max-output-bytes 1048576
uiblueprint measure --snapshot captured.json --query query.json --space swiftui-root-pt --max-input-bytes 1048576 --max-output-bytes 1048576 --json
```

Use original reported source keys (`swiftui.layout`, semantic ID plus mount UUID).
No new Rust importer or handwritten geometry is needed. File writing is a separate
app-owned operation to its private system temporary directory; do not accept output
paths from untrusted URLs. Use `expectedMounts` to refuse a remounted target.

## Meaning and limits

- Layout is measured in `swiftui-root-pt`, top-left. Root origin is zero.
- Source display scale is recorded. `rootFrameInGlobalSpace` is SwiftUI global,
  not a screen coordinate or an established screenshot transform.
- `revision` counts successful captures. `layoutRevision` compares the sampled
  marker frames/mounts, global root frame, scale and public state ID. Neither is
  a presented/GPU frame ID; equal samples do not prove an unchanged interval.
- Saved observations retain source times, cache/unverified freshness and unknown
  consistency. Hit, visible and paint bounds are explicitly unknown.
- Duplicate/missing/invalid markers, stale mounts, invalid parents, timeout,
  cancellation and output overflow throw fixed errors without successful partial data.
- A session accepts one root and one pending request. Attach two simultaneous roots
  to one session and it becomes unavailable; create correctly scoped sessions.
- Real-app geometry, lifecycle and no-app-state-mutation require their own proof.
  Synthetic SDK tests alone do not accept an application integration. Pixel capture
  and screenshot association are separate capabilities, not this SDK's data source.

The governing contract is [SWIFTUI-PROBE@2](../../docs/specs/product/swiftui-probe.md).
The first native consumer is the real PlayPhrase.me Search recovery component;
its source owner retains the exact app/build/run identity and local fixture setup.
The controlled C02 failure scenario used to verify that component deliberately
avoids the real server. It is not an SDK requirement or a server-health test.
The verified installed `0.1.0-web.browser2` CLI already consumes this canonical
output; the SDK itself is supplied separately as this source package. No global
CLI/skill installation or public SDK release is implied by the local integration.

## Focused verification

Run `swift test --package-path plugins/swiftui --scratch-path "$TASK_TEMP/swift"`
from the repository root with `UIB_SWIFTUI_TEST_OUTPUT="$TASK_TEMP"` to export
the two synthetic test records. Build the existing CLI and schema validator into
the same task-owned temporary root, then run
`python3 -B tests/bridges/swiftui_probe_check.py --cli "$CLI" --validator "$VALIDATOR" --records "$TASK_TEMP"`.
The bridge check validates exact SDK bytes and independent8/18pt gap expectations.
Retain actual-app screenshots separately; remove only run-owned non-image outputs.
