# UI Blueprint SwiftUI runtime (Debug candidate)

This package exposes existing SwiftUI hosting trees from inside an opted-in
macOS Debug process. It adds no markers to individual views and uses no Xcode
GUI, LLDB attach, raster-layer capture or input automation. Xcode/Apple SDK tools
are needed to build. The underscored Apple source is qualified per OS/toolchain.

## Integrate once

Link the local package `plugins/swiftui-runtime`, product
`UIBlueprintSwiftUIRuntime`, to the actual Mac application target. Retain one
session in the existing application startup owner, outside `body`:

```swift
#if DEBUG
import UIBlueprintSwiftUIRuntime
#endif

// In the existing App type:
#if DEBUG
private let runtimeGeometry: BlueprintRuntimeSession?
#endif

// In the existing init/do block:
#if DEBUG
runtimeGeometry = try BlueprintRuntimeSession.startIfRequested()
#endif
```

Use the app's existing startup error handling. The API does nothing without
`UIBLUEPRINT_RUNTIME=1`. The caller also supplies process-local
`SWIFTUI_VIEW_DEBUG=29`; the implemented collector allowlists only structural
type and position/size before encoding. This flag alone does not filter private
model values. Never serialize complete `_ViewDebug.Data` into app diagnostics.

Ordinary Release schemes and signing/sandbox stay unchanged. The module's
implementation is `#if DEBUG`. Compile and inspect the actual Release consumer
before accepting exclusion; do not embed developer frameworks in production.

## CLI workflow

The matching caller is `plugins/macos/swiftui_runtime.py` in the source checkout
until an accepted release packages it. Do not claim it exists in older bundles.
After the app is built and assembled on internal temp storage:

```sh
python3 -B "$CALLER" launch --app "$OWNED_APP" --run-root "$INTERNAL_TASK_ROOT"
python3 -B "$CALLER" windows --session "$SESSION_DESCRIPTOR"
python3 -B "$CALLER" capture --session "$SESSION_DESCRIPTOR" --window "$WINDOW" --out "$NEW_SNAPSHOT"
python3 -B "$CALLER" measure --session "$SESSION_DESCRIPTOR" --window "$WINDOW" \
  --type SwiftUI.Color --relation width --bundle "$UIB_BUNDLE"
```

Launch returns the private session descriptor and exact PID. The descriptor's
capability is not copied into reports. Capture/measure always acquires new data;
measurement uses the matching Rust engine, not Python geometry arithmetic.
One source instance serializes only its own reads; there is no desktop queue.
Different owned processes have separate endpoints. App-specific persistent
stores are not automatically isolated by copying its `.app`.

`prepare` accepts an app-owned JSON recipe and existing task-owned external build
and internal runtime roots. It uses CLI `xcodebuild` and LaunchServices, never
Xcode GUI. A recipe is source/configuration, not run evidence:

```json
{"recipe_version":"1.0.0","project_root":".","project":"Example.xcodeproj",
 "scheme":"Example","product":"Example.app","bundle":"me.example.app",
 "environment":{},"arguments":[]}
```

```sh
python3 -B "$CALLER" prepare --recipe "$PROJECT_RECIPE" \
  --build-root "$EXTERNAL_TASK_ROOT" --run-root "$INTERNAL_TASK_ROOT"
```

Reuse the returned `.app`/session for subsequent requests. `prepare` is not part
of ordinary capture. A shared artifact cache, isolation policy and application
recipe require their actual consumer checks; they are not inferred here.

## Current evidence and remaining acceptance

An ordinary owned SwiftUI App fixture returned188→212→188 source nodes in one
process after width/conditional-row changes. The real SwiftUI Color widths were
80→140→80pt; canonical output validated and Rust measured them. Eight concurrent
requests produced eight distinct observations in about0.125s on the test host.
Synthetic model canary was absent after allowlisting. These are fixture results,
not PlayPhrase.me acceptance or a cross-platform claim.

Source nodes without a reported position/size pair remain unknown. Transforms
are currently unqualified and source-local spaces remain separate: cross-node
gaps/window overlays are pending, not manufactured from coordinates. Real-app
coverage, repeated navigation/popup, Release, privacy/concurrency review and
installed-skill delivery remain open under
[SWIFTUI-RUNTIME@1](../../docs/specs/product/swiftui-runtime-probe.md).
