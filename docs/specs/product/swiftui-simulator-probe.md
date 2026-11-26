# SwiftUI Simulator geometry
- Domain: `uib.swiftui-simulator`; contract: `UIB.SWIFTUI-SIMULATOR@1`.
- Authority: Active; stability: Evolving; local implementation, not released.
- Authority source: user26Nov2026 messages01a21855-0622/96e1, existing SDK owner
  tasked with Phone capture and permitted diagnostic donor integration.
- Requires: [runtime contract](swiftui-runtime-probe.md).
- Scope: iOS Simulator arm64 on the tested runtime; physical devices unqualified.

## Source and identity

Reuse typed `_UIHostingView._viewDebugData()` in the opted-in Debug process.
Read existing connected UIWindowScenes/windows/hosting views, no replacement UI.
Window numeric IDs are assigned by the session to weak window witnesses; they are
not CGWindowIDs or pointers. Each capture binds the exact run/window generation.
Use the same source field allowlist, inherited-context attribution, limits and
canonical engine. No raw value/environment/debug archive serialization.

The synchronization adapter verifies system SwiftUICore/SwiftCore symbols in the
Simulator runtime image and brackets the getter in the main UpdateGroup scope.
macOS retains its existing profile and behavior. Unsupported profiles refuse.
Do not infer iOS support from the package deployment floor; qualify iOS27.2 first.

## Transport and preparation

Simulator launch uses an explicitly selected owned device and actual app bundle,
with process-local UIBLUEPRINT_RUNTIME=1/SWIFTUI_VIEW_DEBUG=29. The existing private
file endpoint is reached through the owned app container/launch output. It shares
the existing protocol, capability binding, atomic output, cancellation and quotas.
No process scan, global environment, daemon or debugger injection. App integration
is once per lifetime, additive Debug only; Release and product state stay unchanged.

## Source pairs

Geometry is source-local points unless a mapping is independently established.
Native Simulator PNG belongs to an explicit device/display/orientation/viewport.
Record actual pixel dimensions and window/screen scale/bounds; qualify mapping
using public geometry observations in the fixture before claiming an app pair.
Capture before/after More and viewport change with new observations. Capture
timestamps delimit sequential observations, not an atomic GPU frame guarantee.
System-owned native surfaces without SwiftUI data remain explicitly uncovered.
Mac-hosted Phone evidence, if used, is separately labeled with platform changes.

## Acceptance

Small fixture then actual Phone Search/More composition with parent/child nodes,
layout bounds and screenshot mapping. Verify repeated reads, presentation roots,
unknown fields, stale binding, privacy and Mac compatibility. Missing Reels/Clip/
Download proof stays pending; no whole-platform qualification from one screen.
