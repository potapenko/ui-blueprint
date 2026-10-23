# Explicit Compose layout probe
- Node type: leaf; domain: `uib.compose`; contract: `UIB.COMPOSE@1`.
- Authority: Active; stability: Evolving; accepted/released baseline: none.
- Authority source: human-approved Kotlin Compose plan2026-10-23, preserved in
  [task registry](../../plans/ui-blueprint/task-registry.md#kotlin-compose--active-goal-2026-10-23).
- Read when: Kotlin Desktop/macOS or Android Phone/Tablet probe/import/skill.
- Do not read when: unchanged Web, native AX, actions or pixel capture.
- Requires: [BOUNDARIES@1](boundaries.md), [NATIVE@5](native.md),
  [EXCHANGE@2](exchange.md), [GEOMETRY@2](geometry.md), [IDENTITY@1](identity.md),
  [PRIVACY@1](privacy.md), [CLI@21](cli.md), [ANALYSIS@2](analysis.md).

## UIB.COMPOSE.COLLECT — bounded opt-in runtime facts
Thin source-included Kotlin collector belongs to `plugins/compose`; dependencies
remain in the consuming application's existing Compose ui module. Separate JVM and
Android debug source sets need no commonMain, Gradle plugin or core SDK dependency.
Use public Modifier.Node lifecycle and LayoutCoordinates; no reflection, tree-wide
semantics collection, image capture, networking, polling, overlay or input actions.
Callbacks retain coordinates only; geometric reads occur solely in an explicit
bounded capture on the owning UI thread. No layout, density, semantics, focus,
hit testing or playback modifications. The app owns the request trigger and private
transport; integration must be opt-in and exclude production inclusion by default.
Only declared public marker IDs are collected; never labels, values, paths or media.

Session identifies one exact application surface with fresh session/target/surface
generations. Marker mount UUIDs change on detach/remount/reassignment. Duplicate
mounted IDs refuse ambiguity rather than overwrite. Missing/detached markers refuse;
expected mount mismatch refuses stale_target. Root remount invalidates the session.
Close clears handles; no target shutdown. Session/request limits bound registrations,
selected markers, bytes and elapsed monotonic time; no retries or old-data fallback.

## UIB.COMPOSE.GEOMETRY — attributed source rectangles
Public `root.localBoundingBoxOf(coordinates, clipBounds=false)` supplies unclipped
layout axis-aligned bounding boxes in selected-root local physical px. Same Compose
hierarchy is verified with bounded ancestry. These are layout bounds, including
source transformations; never paint contours, accessibility/hit bounds or visibility.
Source root density/fontScale remain exact finite positive facts. A sourced affine
px→dp scale uses the root density, never rounded density or fontScale. Per-marker
Density is retained separately; no window/screen/capture mapping is inferred.
Source read failure is unknown geometry, not zero. No unselected properties acquired.
Sequential UI-thread reads carry unknown atomicity; coverage is partial for the UI,
complete only for the selected mounted markers that return geometry.

## UIB.COMPOSE.IMPORT — local Rust entry
`uiblueprint import-compose --input FILE --max-input-bytes N --max-output-bytes N`
accepts one explicitly named bounded regular file, strict version `1.0.0` probe
record, no embedded path resolution or connection. It normalizes to unchanged core0.1
Snapshot through canonical validation and bounded encoding. Stored source timing,
identity, exact values and availability remain attributed; import is not live revalidation.
No new Observe provider/session capabilities, core schema or analysis engine.
stdout is one complete canonical JSON object plus newline; fixed sanitized stderr.
Exits:0 imported,1 IO/internal,2 invalid/limits; no partial prepublication stdout.
Snapshots use compose.layout namespace, per-mount source keys, source declarations
for public marker IDs/platform/density/fontScale, and no action refs or semantic roles.
Inspect/measure/check and compatible saved diff reuse their existing contracts.

## UIB.COMPOSE.ACCEPT — finite proof
Headless checks cover collector lifecycle, duplicate/missing/stale IDs, finite
geometry/density, limits and strict import; canonical validation and Rust numerical
measurement/check/diff verify the shared path. Existing CLI consumers remain unchanged.
Actual application-owner Desktop and Android Phone/Tablet captures and probe-on/off
layout/focus/input/playback checks remain required runtime acceptance. Build success
and synthetic fixtures do not qualify devices or Windows/TV. Skills provide portable
source integration and real capture recipes, never manually fabricated runtime JSON.

## Change record
KC-COMPOSE-001, Evolve: explicit human approval opens the new bounded Desktop/Android
collector and saved import path. Registry60→61, CLI20→21, AGENT-SKILL4→5; additive,
core0.1/analysis0.2/Web/native and action/capture authority preserved. Public release
and global installation excluded. Source decisions: [source record](../../research/compose-probe.md).
