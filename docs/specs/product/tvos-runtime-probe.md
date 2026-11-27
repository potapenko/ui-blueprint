# tvOS Simulator runtime geometry
- Contract: `UIB.TVOS-RUNTIME@1`; domain: `uib.tvos-runtime`.
- Authority: Active; stability: Evolving; qualified local source; release acceptance recorded separately.
- Authority: direct user27Nov2026 message01a21d38-a0b4 in TV planning chat01a21d10.
- Requires: [Simulator](swiftui-simulator-probe.md), [UIKit](uikit-runtime-probe.md).

## TVOS.SOURCE
Extend the existing typed SwiftUI and explicit UIKit sources to arm64 tvOS27.2
Simulator, keeping their namespaces, bounds semantics and privacy filtering.
Keep macOS/iOS behavior unchanged. A tvOS compile floor17 is not physical-device
or older-runtime qualification. No source model/text/AX values or raw debug data.
Keep all identity, completeness, timeout/cancel and resource limits intact.

## TVOS.PAIR
Metadata reports `tvos-simulator`, never `ios-simulator`. The caller admits only
known supported Simulator platform tags, and verifies the exact descriptor's app
container belongs to the selected UDID. PNG must be native lossless, dimensionally
consistent with reported screen bounds/scale and orientation. Screenshot and
geometry remain sequential, not atomic. Model geometry does not prove focus
animation presentation bounds, pixel visibility or hit regions. Unqualified
transforms/fields remain unknown. Focus/held changes require fresh observations.

## TVOS.INTEGRATION
One retained opt-in Debug startup session in the actual TV App owner; no product
redesign, wrappers, new routes or changes to player/auth/access/business logic.
Release excludes the hook/library implementation. Launch recipe declares the
actual target/bundle and ordinary guest API origin. One immutable app can install
on distinct owned devices; data/container/session are never copied between them.

## TVOS.ACCEPT
Verify actual TV ordinary/actions/focus/held/fullscreen-return and Search or
Settings List/child, plus source-state change. Separate unsupported gesture/input
coverage from geometry results. Validate canonical bytes and measure with Rust.
Two concurrent independently owned TV sessions must reject swapped bindings and
produce their own fresh observations. Provide reusable local bundle/app/recipe,
source and diagnostic revision, exact device/input commands and retained evidence.
No Phone evidence alone closes this TV qualification.


## Release scope correction
Direct user27Nov2026 in this SDK chat: test Blueprint on one own TV instance,
publish the tool and return it; do not take over consumer provisioning or keep
walking the application. Experimental release includes the verified source and
pair mechanism. Settings/child-list, in-flight held/presentation bounds and
physical Apple TV remain unqualified. A coverage boundary is not an acceptance
claim. Existing adapter/privacy/limits remain protected.
