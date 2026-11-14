# UI Blueprint SwiftUI probe

`UIBlueprintSwiftUI` is an opt-in Swift package targeting macOS14+ for internal
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
    surfaceID: "learner-window-content", scopeID: "learner-window",
    rootMarkerID: "window-content",
    requiredMarkerIDs: ["search"],
    selection: .mountedSubtree
)
```

`runGeneration` is a separate UUID bound by the app to its current run. Never
export an ingress credential/launch token as an identity. All supplied IDs are
public ASCII `A-Z a-z 0-9 . _ -`, 1–96 bytes. The root ID is automatic; exclude it
from `requiredMarkerIDs`. The root plus collected children cannot exceed256 markers.
With `.mountedSubtree`, required IDs are a minimum: every current marker in this
session is collected, including conditional controls and materialized rows.
Missing optional branches are normal; missing required IDs refuse the capture.
Omitting `selection` preserves the original `.requiredMarkers` exact-subset mode.

Apply `.blueprintProbeRoot(session: session)` to the existing full selected root.
Apply `.blueprintMarker("search", parent: "window-content")` to the existing Search
workspace, `.blueprintMarker("video", parent: "search")` to its video container
and equivalent markers to its real sibling/query parts. For example, query-content
belongs to query, query-actions to query-content, and clear/favorite/counter to
query-actions. Parents must form one tree reaching the root.
These are explicit marker relationships, not discovery of every SwiftUI view.
The marker adds no sizing, padding, hit area, focus or Accessibility element.
Keep IDs unique in the selected root. Repeated rows may use public structural slot
names with their existing SwiftUI `ForEach` identity and SDK mount lifetime; a slot
name does not establish logical content identity. Never export private item IDs.
The app's public state and post-capture checks must account for content replacement.

Separate hosting roots, such as a toolbar, get distinct sessions and surface IDs.
Nested roots do not share marker buckets or charge each other's node limit. Capture
them through the app's window request and keep their canonical records separate;
the SDK does not invent a transform between their local origins. Declare any
unmeasured surfaces or uninstrumented regions as coverage gaps.

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
- Any selected optional marker's appearance, disappearance or changed identity
  invalidates publication across that lifecycle change. `expectedMounts` can bind
  optional occurrences too; a missing or remounted expected occurrence refuses.
- A session exceeding256 collected anchors refuses before selection. Overflow is
  never reported as a successful complete subset. Unmarked descendants are unknown.
- Real-app geometry, lifecycle and no-app-state-mutation require their own proof.
  Synthetic SDK tests alone do not accept an application integration. Pixel capture
  and screenshot association are separate capabilities, not this SDK's data source.

The governing contract is [SWIFTUI-PROBE@3](../../docs/specs/product/swiftui-probe.md).
Root collection follows [WINDOW-PROBE@1](../../docs/specs/product/window-probe.md);
its real runtime qualification is separate from SDK tests. On macOS27.0.1, the
real PlayPhrase.me learner window returns135 content nodes and12 named toolbar
surfaces from a request without a component selector. Actual panel changes,
scroll, Settings, playback and resize were checked with localhost API and ordinary
returned media. Narrow-window unavailable toolbar hosts and uninstrumented app
regions are explicit in coverage; cross-host coordinate transforms remain unknown.
The earlier [normal Search qualification](../../docs/specs/product/swiftui-normal-search.md)
records the smaller initial scope.
Its source owner retains exact app/build/run identity and the closed capture route.
The earlier C02 recovery example deliberately avoided the real server; it is
historical codec/geometry evidence, not normal service-operation proof.
The verified installed `0.1.0-web.browser2` CLI already consumes this canonical
output; the SDK itself is supplied separately as this source package. Matching
CLI/skill installation and public source publication have their own verification.

## Focused verification

Run `swift test --package-path plugins/swiftui --scratch-path "$TASK_TEMP/swift"`
from the repository root with `UIB_SWIFTUI_TEST_OUTPUT="$TASK_TEMP"` to export
the three synthetic test records. Build the existing CLI and schema validator into
the same task-owned temporary root, then run
`python3 -B tests/bridges/swiftui_probe_check.py --cli "$CLI" --validator "$VALIDATOR" --records "$TASK_TEMP"`.
The bridge check validates exact SDK bytes, independent8/18pt gaps and the optional
row's4pt inset in a mounted-subtree record.
Retain actual-app screenshots separately; remove only run-owned non-image outputs.
