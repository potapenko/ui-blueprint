# Phone source geometry — local implementation

Owner: existing SDK chat01a20a46. Date:2026-11-26. No additional workers or Goals.
Authority: user messages01a21855-0622 and01a21855-96e1 in chat01a21811 explicitly
authorize research/implementation, diagnostic donor changes and evaluation of
a macOS host of the existing SwiftUI Phone layout. Android/Kotlin and GPUI
implementation are excluded. GPUI44af470 records the same permission.

## Outcome and sequence

First qualify Search root/toolbar/query and More open/closed, hierarchy and actual
layout bounds plus native PNG with sourced coordinate mapping, then a changed
viewport. Reuse one build for requests. Extend the same path to Reels4:3/actions,
Clip/Filters/viewer and Download only after that first source pair works. Do not
claim other surfaces from Search evidence or Mac-host output as iPhone runtime.

1. Inspect typed SDK source and run one small iOS Simulator probe. Completed:
   iOS27.2 arm64 iPhone17e,76nodes, Color120→180pt in one process, without debugger.
2. Compare unchanged Phone More modifier on macOS; system sheet semantics may
   prevent faithful hosting. Prefer actual Phone runtime when available.
3. Extend existing runtime collector's nonvisual platform adapter for Simulator;
   preserve Mac path, field allowlist, canonical graph/measure engine and limits.
4. Integrate one retained Debug session into the existing Phone App owner and
   build with its actual composition. Capture/measure/screenshot actual states.
5. Save local matching SDK/caller recipe and qualified evidence for consumers.
   Publication and GPUI changes are outside this new task.

## Write scope and protected behavior

Blueprint: runtime package/platform files, root/nested Package.swift, matching
Simulator launcher/capture caller, focused fixture/tests, runtime contract and
skill instructions. Native donor current beta-01 baseline5e396329: Phone startup
hook, target package membership, diagnostic recipe/contract only as needed.
No rewind to donor reference7de6680; preserve current source and unrelated work.
No new product layout, auth/store/media changes, production deployment or player edits.

Spec basis: registry→SWIFTUI-RUNTIME@8 and existing MODEL/IDENTITY/GEOMETRY/PRIVACY/
EXCHANGE/LIFECYCLE/CACHE clauses, plus Simulator extension below. Native registry→
platform shells/Phone compositionr28/More@7/query-glass@4 and runtime geometry@1.
Observed source: typed _UIHostingView getter exists in Xcode27.2 Simulator SDK;
0.11.0 currently only collects NSHostingView. Proposed change is one platform
adapter and Simulator endpoint ingress, not five screen-specific collectors.

## Current resources and completion

External task area: `/Volumes/Develop-2TB/Develop/tmp/uib-phone-capture-3saamkjh`.
Internal run area: `/private/tmp/uib-phone-runtime-qefjkw2p`.
Owned small Simulator initially Shutdown, recorded in runtime OWNERSHIP.md.
Retain final source pairs for five consumers; clean only own build/runtime after
readers release them. Tool-managed XcodeBuildMCP logs remain at its reported paths.
First milestone is actual Search+More evidence, not compile/probe success.
