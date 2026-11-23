# SDK, host and component query architecture

Part of [PLAN.UIB.SWIFTUI-GEOMETRY@1](../swiftui-geometry-sdk.md). Everything marked proposed is an implementation choice awaiting plan approval; existing canonical semantics remain binding.

## Package and responsibility split

| Owner | Responsibility | Exclusions |
| --- | --- | --- |
| `UIBlueprintSwiftUI` (existing) | Markers, component/part hierarchy, bounds anchors, mounted occurrences, scoped measurement, canonical Snapshot encoding. | Files, network, application state, Accessibility, AppKit/UIKit, debugger and pixel capture. |
| `UIBlueprintCaptureHost` (proposed optional target) | Registered scenes/scopes, capabilities, bounded request admission, capture dispatch, completion manifest and private file transport. | Product navigation, opening controls, credentials/model inspection and geometry arithmetic. |
| Application adapter | Own the Debug host, bind actual scenes and public state generations, register components and explicit relations, choose private runtime directory and authorize activation. | A second collector or a screen-specific JSON converter. |
| Installed CLI | Address the selected session, list/query/capture, validate output, render compact hierarchy and invoke existing Rust measurements. | SwiftUI inference, UI mutations, automatic app relaunch or LLDB fallback. |

Keep API names/version changes explicit in the contract before coding. Preserve `.blueprintMarker(_:parent:)`, `.blueprintProbeRoot(session:)`, existing capture APIs and exact-subset behavior. Add only APIs exercised by the first real slice.

Add root `Package.swift` referencing the canonical sources under `plugins/swiftui/`; retain the nested manifest for existing local consumers and verify both export the same products/platforms. There is no second source copy. Remote SPM cannot be pointed at the current nested directory as though it were a repository package root. Use one immutable release tag and committed app `Package.resolved`; a local override is only for owned SDK development.

## Components and identities

A registered component has a public semantic identifier, declared parent/component relationship, actual scene/surface and mounted occurrence. Parts name the intended measurement boundary: outer control, icon, label, keycap, row, grid cell, header or footer. Place anchors at the actual boundary relative to padding/frame/overlay modifiers; do not infer those bounds from constants.

Use public semantic IDs and opaque instance/mount tokens. Repeated local names are qualified by component occurrence, avoiding hand-written globally unique strings for every row. Existing SwiftUI `ForEach` identity remains unchanged. Do not export URLs, account IDs, query text, captions or model identifiers as keys. A positional slot is observation-local, not a persistent item identity; content reuse invalidates its diagnostic generation even if SwiftUI preserves the view.

The catalogue reports registered components, mounted instances, current capability and explicit gaps. Declaration of a component is not proof it is mounted. Distinguish `not_instrumented`, `not_mounted`, `not_materialized`, `unsupported`, `stale`, `busy` and `limit_exceeded`; exact public codes are registered before implementation. Anonymous absence cannot establish which state applies.

Build actual `children` and component memberships from registration, not runtime type names, box proximity or AX. Many parts may belong to one component. Add explicit anchor/presentation relations from app ownership; an `anchored_to` relation does not itself establish a coordinate transform. Do not encode geometry in catalogue metadata.

## Bounded scopes without whole-window extraction

Keep the existing 256-node SDK bound for each measured source scope and 1 MiB per-Snapshot cap initially. A window is a catalogue of scopes, not an unlimited flat anchor bucket. Register natural screen/presentation/component scopes and resolve only the requested scope; default window capture accounts for all scopes with a shared deadline/byte budget and explicit per-scope outcomes.

Current overflow occurs before filtering, so adding a selector after collecting the whole window is insufficient. Implement registration/partitioning at the existing preference/session boundary. Test a small selected component with large mounted siblings before instrumenting the whole app. Selected capture must not resolve or serialize other scopes' anchors. Bound catalogue/registration storage too; do not replace an extraction limit with unbounded retained state.

Within one scope allow component subtree selection with its necessary declared ancestry. Preserve the source root/space and parent chain; never rebase or relabel old Snapshot bytes to create a fake scope. Discovery pagination binds to a catalogue revision and refuses stale cursors. Aggregate window reports may be partial; requested component completeness is judged against its declared parts.

## Coordinate model

Within a shared SwiftUI root, resolve bounds anchors into one named source space in points, top-left; report actual display scale, safe-area/viewport context where sourced, state and lifecycle generations. Dimensions, padding declarations, AX bounds, visible/paint bounds and pixel coordinates remain different facts.

For partitioned scopes in one hierarchy, qualify a parent-space boundary anchor alongside the child scope. Prefer directly resolving required anchors in their common ancestor space. A rectangle alone does not prove a general affine: rotation, scale and clipping must not be reconstructed from width ratios. Translation-only mapping requires explicit qualified source evidence; otherwise preserve separate spaces.

Toolbars, sheets/popovers and separate windows may have independent hosting roots. Register all of them, including presentation ownership. Prove each supported cross-root mapping using available source APIs and the existing external capture metadata; do not treat SwiftUI `.global` as screen coordinates. If the actual composition cannot expose a mapping through the permitted APIs, keep local dimensions usable and that cross-root acceptance criterion pending. Do not restructure product UI or add a native-view bridge to force a measurement.

No claim of an atomic GPU frame. Scope lifecycle/state is checked before capture and before publication. An animation sample is a sample; stable repeated geometry does not prove all rendered content is stationary. A changing video must not block an unrelated stable dialog.

## Request transport: one reusable diagnostic host

Proposed default: a private task-owned request directory inside the application's internal temporary sandbox, with an event-driven file listener. This is an opt-in Debug facility, no network listener, system daemon or background screen scan. Use public Dispatch file events; qualify real sandbox delivery in P1. The SwiftUI measurement code keeps its non-observable capture event and must not trigger UI redraw to answer.

1. Explicit launch activation establishes a random run generation/capability and host-owned directory; the launcher receives a private session descriptor. No token enters ordinary logs, source metadata or a public catalogue.
2. The caller names run, scene, scope/component, expected catalogue/mount generation, request ID and bounds. Requests accept no arbitrary filesystem paths, shell commands, selectors or model values.
3. Publish payload then a completion marker with exact length/digest. Host consumes only a complete matching request. Handle partial copies, duplicate/coalesced file events and directory replacement; never parse a half-written request as a valid command.
4. Host checks ownership/capability/generation and scope, then dispatches one bounded main-actor capture. Busy/timeout/cancel have explicit terminal outcomes; there is no hidden retry or unbounded queue.
5. Write original canonical Snapshot files, then the final result manifest with request/version/scope outcome and hashes. CLI waits only within the request deadline, validates identity and original bytes, then invokes Rust.
6. Bound completed-request retention and prevent replay without an ever-growing request-ID set. A monotonic request sequence per run plus bounded acknowledgements is the initial design to verify. Never delete a result before its consumer has retained or released it.

Host shutdown cancels owned requests, closes the file-event source and invalidates the session descriptor. IO completion rechecks run/scope generation before publication; late callbacks cannot publish after cancellation or scene detach. This lifecycle is independent of product app shutdown: capture failure never kills or restarts the app.

On Mac the tool uses the supplied private session descriptor. Simulator uses the exact UDID and bundle's data container. Physical development devices use qualified `devicectl device copy to/from --domain-type appDataContainer` with explicit device/bundle and timeouts. The inspected tool help supports these operations; successful real-device delivery is still unverified.

Legacy URL admission remains available with its original semantics and no schema widening by accident. Do not rely on URL routing for quiet sampling: SwiftUI routes incoming URLs according to active scenes, and opening one may affect app presentation. A sleeping/background mobile app may report unavailable; capture never activates it implicitly.

## CLI and agent workflow

Add a small SwiftUI entry to the existing CLI: list registered scenes/components; capture one selected component; display its attributed tree; measure selected nodes with existing Rust operations. Register exact grammar before source edits; these operation names are not commands available today.

Typical result: `random-completion → replay → icon / label / keycap`, each with identity, bounds, units, source space and coverage. Compact output includes dimensions and missing parts; full canonical files stay local. Component selectors resolve from the catalogue to exact observed keys. Existing `--ref`, query files, saved inspect/measure/check and their versions remain compatible.

Keep transport/platform helpers small and packaged with matching versions, following existing native-prepare precedent; there is one public command path and no copied per-chat Python extraction script. Negotiation checks SDK/host protocol/CLI capability compatibility, not only product version equality. Unsupported versions fail before app access.

## References checked for this proposal

- [Swift Package manifests/platforms](https://docs.swift.org/package-manager/PackageDescription/PackageDescription.html).
- [Apple anchors](https://developer.apple.com/documentation/swiftui/view/anchorpreference(key:value:transform:)) and [coordinate spaces](https://developer.apple.com/documentation/swiftui/coordinatespaceprotocol).
- [Apple file-event source](https://developer.apple.com/documentation/dispatch/dispatchsource/makefilesystemobjectsource(filedescriptor:eventmask:queue:)).
- [SwiftUI URL scene routing](https://developer.apple.com/documentation/swiftui/view/onopenurl(perform:)). These APIs establish available mechanisms, not runtime acceptance of this design.
