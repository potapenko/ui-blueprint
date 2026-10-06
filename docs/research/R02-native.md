# R02 — native mechanism and measured-probe feasibility

Diagnostic result on 2026-10-06; Discover, not shipping acceptance. Consumer:
C01 D01/D02/D04/D07, F02, M01 and P01. Authority is the user launch of PLAN.UIB@1
at `358c757e7eab84a3989d150dbad57924d866601a`, as recorded in the
[task registry](../plans/ui-blueprint/task-registry.md), and the bounded
[R02 packet](../plans/ui-blueprint/packets/R02.md).

## Spec Basis and traversal receipt

Read AGENTS → [registry](../specs/README.md) `UIB.ROUTING@1` →
[research route](../specs/reference/research-routes.md#r02)
`UIB.RESEARCH-ROUTES@1`, clause ROUTE → the R02 leaves in listed order:
BOUNDARIES, REUSE, NATIVE-SOURCES, PROBE-SOURCES, PILOTS, MODEL, IDENTITY,
EXCHANGE, GEOMETRY, PROJECTIONS, PRIVACY, LIFECYCLE, NATIVE, FORMS, CACHE,
ACTIONS, NATIVE-PILOTS, ROADMAP. All are `UIB.<ID>@1`, `.CONTENT`, read fully,
including their explicit transitive closure. C00 candidate
`92d2bf89ea9acab091fd166eef4432408e353744`; acceptance
`9bedeccbf4c5a92206d24d0794b299e54a2b22e1`; dispatch base `7c48392` on master.
Supporting routes: implementation, product-truth core/routing/evidence/worker
lifecycle, QA, Apple platform, Computer Use, operational safety. Skills:
macOS build/run/debug and SwiftUI expert. Root and Rust routes were not activated:
this is a finite worker with no Rust/Cargo changes. Context: 18 product/reference
leaves plus their two routing nodes; upstream sources below, not entire catalogs.
Excluded: browser internals, real-product profiles, mobile, ML, CLI/export,
DrawingBrief, F1–F4, generic SDK and shared production interfaces. No semantic
revision drift or product contract delta found.

Specified: channels retain independent permissions/provenance; AX bounds are not
layout bounds; exact window and process generations precede action; probe is
opt-in post-layout measurement and cannot establish production evidence. Full
M05 needs geometry/focus/hit/AX invariance. Observed evidence below is narrower.
Packaging/support choices below are proposals, not final D01–D07 decisions.

## Fresh upstream source records

Upstream archives were fetched read-only at the exact pinned commits; no upstream
program or test suite was run. Selected source headers were inspected. No source
fragment or third-party asset was copied into the fixture. The root license
markers are evidence of selected upstream terms, not clearance for all bundled
code/dependencies. No upstream runtime package was installed.

### AXorcist

[Source](https://github.com/openclaw/AXorcist/tree/1b12b55d61c01363f913f17a2e1db732bbb203a3),
revision `1b12b55d61c01363f913f17a2e1db732bbb203a3`.
Read `Sources/AXorcist/Core/Element+Hierarchy.swift`, `Element+ValueSetting.swift`,
`Element+Actions.swift`, `AXObserverCenter.swift`, `ObserverTypes.swift`,
`ObserverNativeWork.swift`, `Sources/AXorcist/Search/GeometryHelpers.swift`,
`Tests/AXorcistTests/ObserverNativeCleanupTests.swift`, `SetValueExecutionTests.swift`,
and `Package.swift`. Root LICENSE: MIT, copyright Peter Steinberger 2025;
selected headers contain no different license. No root NOTICE found.
Package uses ApplicationServices/Foundation, swift-log, and Commander for CLI;
its Swift 6.2/macOS 14 declaration is upstream's, not UI Blueprint's promise.

Observed algorithm: children merges direct and alternative attributes, deduplicates
Element identities and caps 50,000 per parent. Even strict traversal adds AXWindows
and focused application element. Value setter preserves CF numeric/Boolean types
and native errors; action owner invokes AXUIElementPerformAction. Neither an API
success nor a wrapper log proves application outcome. Observer keys include PID,
notification, element/process scope; tokens retain registration ownership. Native
work races timeout against completion, retaining capacity through late cleanup;
state epochs/process identity prevent obsolete registration commits. Cleanup tests
exercise a delayed remove and held admission slot. GeometryHelpers only serializes
points/sizes; it supplies no window/capture transform.

Decision: reimplement bounded per-target reads and observer ownership ideas against
our contract. Reject application/window expansion, verbose element logging,
50,000-child default and global singleton assumptions as product policy. Crucially,
`processUniqueIdentity` uses numeric proc_pidinfo flavor 17 from the private XNU
header; do not silently adopt this as supported public API. Our fixture uses its
own launch receipt and public NSRunningApplication launchDate. F02/M03 must test
late notifications, PID reuse/unknown identity and independent-session progress.
Own falsifier: bounded exact fixture window, unsupported attributes preserved,
stale process manifest refused. Native observer lifecycle is source evidence only.

### Peekaboo

[Source](https://github.com/openclaw/Peekaboo/tree/43b2fe2a72913e5f61f3996ef4c5873f15be8784),
revision `43b2fe2a72913e5f61f3996ef4c5873f15be8784`.
Read CLI `Commands/AI/SeeCommand+CapturePipeline.swift`,
`SeeCommand+PixelCapturePipeline.swift`, `Commands/Shared/SnapshotValidation.swift`,
`SnapshotMutationCoordinator.swift`, `TestFixtures/ExactWindowCapture/main.swift`,
`Tests/CLIAutomationTests/SnapshotMutationCoordinatorTargetTests.swift`; adjacent
AutomationKit `Strategy/ExactWindowSelectorResolver.swift` and
`Services/Support/SnapshotManagerProtocol+Mutation.swift`. Also inspected the
`DesktopTargetIdentity`/error definitions in `Strategy/DesktopTargetPlanning.swift`
and `ElementDetectionResult`, `DetectedElement`, `WindowContext` definitions in
`Services/Core/Protocols/ElementDetectionModels.swift`; not the whole SDK.
Root LICENSE: MIT, Peter Steinberger 2025. No selected-file override/NOTICE found.
Selected imports span PeekabooCore/Foundation, Commander, Algorithms, AppKit;
those internal and external dependencies are not copied or adopted.

Observed: SnapshotValidation checks UI-map presence, not every property's freshness.
Mutation coordinator delegates to a lease owner; unknown operation failure leaves
the lease pending, while failure after dispatch becomes indeterminate with target
attribution. Tests distinguish stale pre-dispatch refusal from post-dispatch lease
finalization failure. Exact selector rejects ambiguous titles; automatic selection
still uses a best-window heuristic and does not become our identity policy.
Pixel pipeline carries a deadline through prepare/publication and suppresses late
success; cleanup knows which output/snapshot it owns. The named legacy capture
entry redirects exact window/area paths to desktop observation. Its optional
web-content focus is mutation, not a read-only extraction trick.

Decision: adapt receipt separation, late-result suppression and exact-target
fixture strategy; no mandatory Peekaboo CLI/model/host stack. Do not adopt the
DetectedElement role-based actionable fallback. The upstream exact-window fixture
is AppKit-drawn parent/popup with a minimum display size; use its controlled
attribution concept, not its UI implementation or display prerequisite. Our
SwiftUI fixture establishes a public own-window path; F02 must add identical-title
siblings, popup/sheet capture, close/recreate and mutation outcome assertions.

### Compose Blueprint and Preview

[Blueprint](https://github.com/popovanton0/Blueprint/tree/09287ace7ea4a79cb5a1a08521cd69a365f7bdac),
revision `09287ace7ea4a79cb5a1a08521cd69a365f7bdac`: read commonMain
`BlueprintId.kt`, `Blueprint.kt`, `dsl/Anchor.kt`, `Dimension.kt`, `GroupScope.kt`,
`MeasureUnit.kt`, adjacent androidUnitTest `Dsl.kt`, `blueprint/build.gradle.kts`.
Apache-2.0 root LICENSE; NOTICE says Copyright 2023 Anton Popov. Compose
runtime/foundation and Android annotations are implementation dependencies.
Markers update LayoutCoordinates on global positioning, discard detached targets,
and resolve localBoundingBoxOf against the root. Anchors interpolate an edge or
center, with units separate. DSL tests assert anchor/unit construction.
However the visualization wrapper defaults to adding dimension padding and rounding
density; Sp is marked experimental/incorrect and RTL has explicit limitations.
Decision: use the measurement/explicit-marker idea, independently implement thin
SwiftUI reporting; reject the wrapper's layout/density mutation and drawing layer.

[Preview](https://github.com/GusWard/Blueprint-Compose-Preview/tree/95398528a6d5b1f4fa4ce92e1a73f830608eb3fb),
revision `95398528a6d5b1f4fa4ce92e1a73f830608eb3fb`: read
`preview/BlueprintPreview.kt`, `grid/logic/BlueprintLineCalculator.kt`,
`items/BlueprintItemData.kt`, corresponding `BlueprintLineCalculatorUnitTest.kt`,
and module build.gradle.kts under `blueprint-compose-preview`.
Additional ItemData directional-test examples were inspected, not claimed as a
full test-suite audit. Apache-2.0 root LICENSE; no root NOTICE found. Selected
headers have no additional license; Android/Compose/Material/runtime dependencies
remain excluded. Extraction tries unmerged semantics and reflects into outer
coordinates. It chooses inner bounds for some controls, suppresses children,
rounds to tenths and retains nonempty data in a 50-entry survivor cache. Line
calculator uses directional center tests, deduplicated pairs and visual line
placement; its tests cover horizontal/vertical pairs and obstructed parent lines.
Decision: do not port reflection, survivor freshness or suppression as graph
identity/removal. Optional projection ideas are useful, never measured paint
contours. Own falsifier: a merged AX control with separate opt-in icon/text bounds;
F02/P01 still need disappearing nodes, empty composition and explicit mapping
across remounts. Runtime Compose behavior was not evaluated.

### Apple platform contracts

Primary sources: [AX multi-attribute reads](https://developer.apple.com/documentation/applicationservices/1462051-axuielementcopymultipleattribute),
[AX timeout](https://developer.apple.com/documentation/applicationservices/1459345-axuielementsetmessagingtimeout),
[observer registration](https://developer.apple.com/documentation/applicationservices/1462089-axobserveraddnotification),
[ScreenCaptureKit WWDC22](https://developer.apple.com/videos/play/wwdc2022/10155/),
[SwiftUI combine](https://developer.apple.com/documentation/swiftui/accessibilitychildbehavior/combine).
Apple web AX pages exposed JS shells; Markdown fallback fetch failed. Exact SDK
27.0 AXUIElement.h declarations/comments were read for these APIs and prompt
preflight; ScreenCaptureKit SCStream/SCScreenshotManager headers establish
filter contentRect/pointPixelScale and availability (macOS 14; child inclusion
configuration 14.2). No undocumented alternative was substituted.

Batch errors can be per attribute; object messaging timeout is not a global
transaction deadline. Notifications can be unsupported; absence is not freshness.
WWDC single-window capture excludes other/child windows, can include occluded or
offscreen content, and scales into an output buffer. Audio policy differs, so our
capture explicitly disables it. Metadata is required for mapping, not a guessed
ratio. This sample uses a still-image API and filter metadata, not a complete
SCStream per-frame transform validation. SwiftUI combine is semantic merging;
our runtime confirms it does not expose these internal icon/text nodes externally.

## Owned experiment and falsifiable observations

Source: [fixture and reproduction](../../experiments/native/README.md).
Environment: macOS 27.0.1 (26A434), arm64, Xcode SDK 27.0, Apple Swift 6.4
(swiftlang-6.4.0.34.1). Compiled with Swift language mode 6 and macOS 14.0 target;
only the current host was run. No signing identity, system permission, display
configuration or other application was changed. No external SDK was required.
AX and ScreenCaptureKit preflights succeeded; no permission prompt was requested.

| Observation | Off baseline | On baseline | Off expanded | On expanded |
| --- | --- | --- | --- | --- |
| AX sample size, pt | 173.5×48 | 173.5×48 | 231.5×62 | 231.5×62 |
| AX sample origin, screen pt | 764,491 | 764,491 | 764,491 | 764,491 |
| AX focus / supported action | true / AXPress | true / AXPress | true / AXPress | true / AXPress |
| Sample activation count | 0 | 0 | 1 | 1 |
| Probe icon size, local pt | not requested | 26×24 | not requested | 26×24 |
| Probe text size, local pt | not requested | 115.5×19 | not requested | 151.5×26 |
| Derived icon→text gap, pt | unknown | 8 | unknown | 18 |

Gap is computed from reported text.left − icon.right, not copied from the HStack
spacing declaration. Local probe container origin is 24,60 in both states;
text origins are 70,74.5 and 86,78. Source declarations map the AX identifier to
container/icon/text; the relation is explicit one-to-many here, not evidence of
arbitrary many-to-many matching. AX remains partial and has no children under
`r02.sample`; no decorative child acquires an action ref.

Own-window binding initially refused because AXWindows included another service
window (`_NS:8`) as well as `primary`. The revised experiment reads only minimal
window identity metadata, selects the unique explicit `primary`, and independently
validates manifest PID/launch time and CG window ownership. It does not use title
or matching rectangles as generic identity. This demonstrates an own-fixture
mapping, not arbitrary external-window matching. A terminated off-fixture manifest
was refused with exit 1 before capture. Complete close/recreate/PID-reuse and
same-title adversarial cases remain F02 work.

All four captures are exact isolated windows, 880×624 pixels, reported filter
scale 2, 440×312 pt including title bar. AX screen top-left and AppKit screen
bottom-left are separately labeled. Probe→screen and image transforms are not
calibrated; do not use this sample for coordinate click or precise overlay.
The baseline PNGs are byte-identical (SHA-256 in evidence summary). Expanded PNGs
differ, visibly including focus/chrome presentation; its cause was not isolated,
so expanded pixel invariance remains unverified. AX focus parity does not prove
all focus rendering or hit-region invariants. Computer Use element activation
confirmed 0→1 for each build; no pointer-edge hit-region matrix was performed.

## Decision handoff and remaining acceptance

- D01 proposal: narrow native Swift bridge can build on the measured host with
  public AX + ScreenCaptureKit + SwiftUI. Oldest working OS, architecture matrix,
  signing/package behavior and per-caller permissions are unverified.
- D02 proposal: start with a thin Swift collector carrying bounded JSON records
  into Rust; keep analytics in Rust. A helper process provides a concrete hang
  lifetime boundary. This prototype does not choose production FFI vs IPC, a
  daemon, retained revisions or a complete memory policy.
- D04: require process/surface generations, explicit window binding and fresh
  target check; fail closed when own mapping is absent. Do not promote public
  launchDate plus fixture ID into a universal external identity guarantee.
- D07: reimplement against our contract; no copied source, runtime package or
  license obligation introduced by code copying. Recheck exact selected material
  and notices before any later borrowing, especially private/native bridges.

Minimum F02: two same-title windows with distinct explicit identities; fresh
move/resize/recreate and stale refusal; popup scope; secure form/focus outcomes;
scroll/clipping and lost-event/deadline fixtures; immutable off/on state oracle
including app active/key-window state, center/edge/outside hit checks, disappearing
nodes, empty composition and mappings. P01 must prove probe-off/on invariance on
the same fixed scenario, including expanded pixel/focus discrepancy, validated
transforms and release-vs-debug evidence separation. M01–M06/P01 are not accepted.
No numeric performance gate is selected from this sample; observed timings are
exploration only. Native lane released; next immediate resource is checkpoint
Git lease, followed by independent review and F02/C01.
