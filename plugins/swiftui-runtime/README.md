# UI Blueprint SwiftUI runtime (experimental Debug support)

This package exposes existing SwiftUI hosting trees from inside an opted-in
macOS Debug process. It adds no markers to individual views and uses no Xcode
GUI, LLDB attach, raster-layer capture or input automation. Xcode/Apple SDK tools
are needed to build. The underscored Apple source is qualified per OS/toolchain.
The current arm64/macOS27.0.1 candidate brackets the getter in SwiftUI's main
UpdateGroup and restores it on every exit. The earlier unscoped call is withdrawn
after a real-app trap. The scoped call passed real PlayPhrase.me capture; the
newer normalization has fixture qualification and awaits its full app sequence.
The user explicitly chose an experimental 0.8.0 release now, with subsequent
defects reported by consuming agents.

## Integrate once

Link the repository-root Swift package at the selected published tag/revision,
or the local package `plugins/swiftui-runtime`, product
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

`discover --session ... --window ... --generation ... --out NEW_FILE` saves
source-reported nominal type counts and compact topology without serializing a
full geometry Snapshot. It requires the `discover-types` endpoint capability.
The separate versioned artifact preserves observation/run/window attribution and
all source nodes under the same limits; it contains no geometry or model values.
Use it to identify a source-backed candidate for a subsequent fresh subtree query,
not as a stable index handle or proof of a missing custom View's identity. See
[discovery instructions](../../skills/ui-blueprint/references/swiftui-runtime.md#discover-actual-types-when-a-full-snapshot-overflows).

With a matching SDK, `capture`, `tree` and `measure` also accept
`--subtree-type ExampleApp.SearchRoot`. This selects one exact source-reported
nominal type and all its descendants before serialization; ancestors are retained
and explicitly labelled as context. It does not change the existing post-capture
`--type` filter. Missing/ambiguous types refuse, and the caller checks the endpoint's
`capture-subtree-type` capability before dispatch. Source node/depth and output
capacity is1048576 nodes/native views and1GiB per artifact; depth512 remains. Some custom View
names are not exposed as standalone source nodes. See [scoped query semantics](../../skills/ui-blueprint/references/swiftui-runtime.md#select-a-component-before-export).

A matching SDK/caller preserves `sourceLimit` and exit4 while adding a bounded
`source_limit` diagnostic: `kind`, `limit`, `observed`. For example,
`{"kind":"source_nodes","limit":1048576,"observed":1048577}` means the collector
encountered at least1048577 nodes, not that it counted the entire tree. Other kinds
distinguish native/source depth, native nodes, type-name bytes/control presence
and exhausted snapshot sequence. No type names or model values are included.
New callers also accept historical16384-node diagnostics from0.10.1; older SDKs may omit detail. Use the matching updated SDK/caller for larger artifacts.
`--subtree-type` does not bypass source limits: uniqueness is established only
after the whole bounded source read. A hard source overflow publishes no artifact.

The runtime caller defaults to30seconds (explicit maximum120). Capture plus
measurement shares that budget; cancellation never claims to preempt the getter.
For measurements it checks the matching Rust CLI's `--result-only` support before
source dispatch. Rust still validates the original complete Snapshot and computes
the result; the versioned CLI reply preserves source context/observations/coverage
without returning the full graph a second time. The original canonical record
remains the replay source. Old ordinary Rust `--json` output is unchanged.

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

The scoped7b candidate now captures real PlayPhrase.me without the former trap:
6286 nodes, canonical validation and fresh Rust width/height queries passed.
The next normalization candidate resolves source-inherited layout inputs with
defining-node attribution, includes toolbar/nested hosting views and attaches
qualified 2D affine mappings within each hosting tree. Its public GeometryProxy
fixture covers resize/conditional content, text reflow, padding, button labels,
scrolling, scale/rotation and a separate toolbar root; the canonical snapshots
and Rust width/gap checks pass. Generic type arguments are omitted from nominal
labels to keep full trees readable and bounded; node identities/links remain.

Cross-host/window/screen mapping is not inferred. Perspective/3D remains unknown;
Rust rectangle relations reject rotated/sheared mapped shapes rather than use
their bounding envelope as containment. Source-local dimensions remain available.
Full real-app coverage/transitions remain incomplete. They do not block the
user-authorized experimental release; packaging/installation verification and
the documented capability boundary govern that delivery under
[SWIFTUI-RUNTIME@8](../../docs/specs/product/swiftui-runtime-probe.md).

The agent-facing preparation and query instructions live in
[setup](../../skills/ui-blueprint/references/swiftui-setup.md) and
[runtime queries](../../skills/ui-blueprint/references/swiftui-runtime.md).
