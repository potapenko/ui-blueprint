# Whole SwiftUI hierarchy through Xcode: qualification route

Part of [PLAN.UIB.SWIFTUI-GEOMETRY@2](../swiftui-geometry-sdk.md), 2026-11-23. Research evidence and proposed next checks; no live result or adopted dependency is claimed.

## User correction and required outcome

The user requested one global solution: add necessary support across the app once, reuse one build, and collect the whole currently mounted hierarchy in one capture. They explicitly rejected planning around whichever component a particular chat happens to need and cited Xcode's existing hierarchy viewer.

The previous inference that absence of a public DOM-like SwiftUI library API requires manual per-component markers was premature. Xcode itself is a separate available source. A failed custom ABI/cache reader is not a failure of the supported IDE workflow.

## Verified documentation and source facts

1. Apple documents Debug View Hierarchy for SwiftUI/UIKit on Mac, iPhone, iPad and other Apple platforms. Its Size inspector shows SwiftUI size/placement resolution. [Apple appearance diagnosis](https://developer.apple.com/documentation/xcode/diagnosing-issues-in-the-appearance-of-your-running-app).
2. Apple's WWDC demonstration explicitly inspects SwiftUI stacks, properties and modifiers. [Debugging in Xcode 11](https://developer.apple.com/videos/play/wwdc2019/412/).
3. Apple documents saving and reopening captures with File → Export View Hierarchy as `.viewhierarchy`. [Xcode 12 release notes](https://developer.apple.com/documentation/xcode-release-notes/xcode-12-release-notes).
4. Installed Xcode is 27.2, build27B52. `xcodebuild -help` and Xcode's scripting dictionary did not expose a hierarchy-capture command in the inspected entries. This is not proof that no automation route exists.
5. [LNViewHierarchyDumper](https://github.com/LeoNatan/LNViewHierarchyDumper) declares macOS/Catalyst, Apple simulators and developer-image device support. It calls internal `DebugHierarchyFoundation` and `libViewDebuggerSupport`, is development/testing-only and is not App Store safe.
6. Inspected upstream pin: `64d1d66ac588c5e7369d0f0945069820fdcd7434`, version1.4.0, 2024-11-21. The checked package has no package dependencies. Its age/platform declarations do not qualify Xcode27.2 or full Mac SwiftUI coverage.
7. Its source sends phased requests through `DebugHierarchyTargetHub`, writes `RequestResponses/Response_*`, decodes JSON (with optional gzip), and writes metadata. This is structured archive data, not only a raster. The Mac phases request native frames/bounds and remaining properties; SwiftUI node coverage must be checked on a real capture. [Pinned implementation](https://github.com/LeoNatan/LNViewHierarchyDumper/blob/64d1d66ac588c5e7369d0f0945069820fdcd7434/LNViewHierarchyDumper/LNViewHierarchyDumper/LNViewHierarchyDumper.m).
8. Detox's capture operation uses this dumper, separately from its ordinary UIView XML traversal. That establishes an existing automated consumer, not successful qualification for PlayPhrase.me. [Pinned Detox caller](https://github.com/wix/Detox/blob/667efcb6e7dda59b390f1707ceeb546414869a97/detox/ios/Detox/DetoxManager.swift).

## Qualification sequence

| Step | Owner and action | Required result |
| --- | --- | --- |
| X1 | Native app owner obtains a normal Xcode Debug View Hierarchy capture of its exact owned app/window and exports it. | Real SwiftUI node/part coverage visible in Xcode; actual archive, app/build/OS/Xcode/state attribution. No changes per selected component. |
| X2 | Blueprint reads a bounded sanitized archive supplied by that owner. | Identify roots, parent/child links, node types, geometry fields, units and transformations; compare selected values with Xcode Size inspector. Record omissions rather than infer layout from pixels. |
| X3 | Blueprint implements/replays a reusable saved-archive importer on project-owned examples once the format is established. | Canonical hierarchy and exact source geometry; existing Rust measurements work. No need for custom markers to import a node present in Xcode's output. |
| X4 | Qualify reusable acquisition: supported Xcode export automation first, or a separately reviewed Debug-only dumper integration where justified. | Whole available tree captured repeatedly from one build, across different screens and presentations; no per-node instrumentation or rebuild. Current Apple framework compatibility and Release exclusion proved. |
| X5 | Native owner runs coverage/regression scenarios on the one integrated app; mobile owner qualifies the same route on iPhone/iPad. | Repeated captures after navigation, resize, scroll and popup changes. All captured nodes available to saved queries, not only initially selected controls. |

Random, Actors, Practice, Tour, Clip Search and Reels are coverage tests of this one route. They are not separate implementation deliverables. A small initial test can establish mechanism viability; it cannot redefine the final whole-tree scope.

## Important implementation checks

- Do not equate an exportable Xcode archive with a documented stable external schema. Version-pin the decoder and reject unsupported variants honestly.
- Identify actual SwiftUI nodes versus native NSView/CALayer records. Returning only hosting views is a failed SwiftUI coverage criterion.
- Preserve per-source spaces and transforms. A native view frame or layer rectangle is not automatically the corresponding SwiftUI layout rectangle.
- Xcode may pause/inspect the app. Attribute that acquisition mode; do not promise non-interference or continuous sampling without measurement.
- Raw Xcode exports can contain images, labels and inspected values. Keep them private in the owned evidence area, allowlist geometry/structural fields for Blueprint, and do not transmit/export raw app data to third parties.
- The inspected dumper deletes an existing output destination, has broad remaining-property requests, and loads internal frameworks. Do not adopt its implementation unmodified without checking preservation, bounded execution, data minimization and native project restrictions. A unique owned destination avoids overwriting retained evidence.
- No debugger attach, signing/sandbox change, private-library injection or framework deployment was performed during this research. Permission to add diagnostic support is not evidence that a particular internal-framework call works.
- Closed/unmaterialized UI has no current runtime rectangle. Opening it normally in the same build makes it eligible for the next full capture. Do not instantiate all application states at once or describe missing mounted nodes as merely closed screens.

## Selection criterion

Prefer the least invasive proven route that exposes the complete available mounted SwiftUI hierarchy and geometry to Blueprint. If the supported Xcode route is sufficient, add acquisition/import support rather than distributing markers throughout the app. If it has a concrete measured gap, show that gap before choosing global SDK instrumentation. Do not manufacture a choice between universal manual annotations and the already-failed private cache reader.
