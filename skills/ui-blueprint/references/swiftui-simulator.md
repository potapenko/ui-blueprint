# SwiftUI Phone geometry in iOS Simulator

Experimental source support introduced in0.12.0. Qualified locally on Apple
Silicon, Xcode27.2 and iOS27.2 Simulator. Physical devices, older/newer runtimes,
iPad behavior and a macOS-hosted Phone layout are not qualified by this result.
No Xcode GUI, debugger attach or per-element markers are required.

## Prepare the application once

Use the matching source tag's `UIBlueprintSwiftUIRuntime` SwiftPM product and
the [one retained Debug startup session](swiftui-setup.md#link-and-retain-the-session).
The root/nested package declares iOS17 as its compile floor; actual runtime
qualification remains narrower. Keep the existing Phone target, views, signing,
data ownership and Release behavior. Compile with the project's ordinary CLI
build recipe; Simulator geometry integration must be authorized by the app owner.
Do not add a new product screen, replace system sheets or rewrite the Phone UI
as a Mac app to make geometry accessible.

The application recipe has the same JSON fields as [SwiftUI setup](swiftui-setup.md#check-in-an-app-recipe),
with its real Phone scheme/product/bundle and supported local service environment.
`swiftui_runtime.py prepare` is the Mac build path; for Simulator build the
existing Phone target using `xcodebuild` or the project wrapper, then use the
Simulator launcher below. Do not invoke the Mac launcher for a Simulator app.

Select a free, existing, explicitly owned Simulator UDID. Respect the operator's
device pool; do not create/erase devices or replace another owner's installation.
Build intermediates follow the operator's external-storage policy. Copy the
self-contained `.app` into an owned internal runtime directory before install.
Use a fresh owned installation/container or the project's established isolation;
a different build directory alone does not isolate app data.

Resolve the installed matching `UIB_BUNDLE` as in [the skill](../SKILL.md).
Python3.11+ and Xcode CLI tools must already be available:

```sh
python3 -B "$UIB_BUNDLE/swiftui_simulator.py" launch \
  --simulator "$OWNED_SIMULATOR_UDID" --app "$INTERNAL_APP" \
  --run-root "$INTERNAL_RUN_ROOT" --recipe "$PHONE_RECIPE"
```

Launch installs that app on that device and starts it with process-local
`UIBLUEPRINT_RUNTIME=1` and `SWIFTUI_VIEW_DEBUG=29`. It binds a newly created private
endpoint inside that exact app container to the PID returned by simctl. It does
not kill a running application or reuse an old descriptor. Stop an old owned run
through its lifecycle before launching another; don't repeatedly relaunch after
`runtime_hook_not_ready`. Check the owned process and result first.

Keep the returned `session` path private; don't print its capability contents.
The app stays running for subsequent requests. Rebuild only when app/SDK source
or build configuration changes, not after opening More or for every measurement.

## Fresh requests and source/PNG pairs

The matching existing runtime caller supports `windows`, `discover`, `capture`,
`tree` and `measure` against this session. Follow
[query semantics](swiftui-runtime.md), including fresh reads, exact nominal type
selection, unknown properties and large-window limits. Window IDs here are
session-assigned UIKit witnesses, not macOS CGWindowIDs or permanent handles.

```sh
python3 -B "$UIB_BUNDLE/swiftui_runtime.py" windows --session "$SESSION"
python3 -B "$UIB_BUNDLE/swiftui_runtime.py" capture --session "$SESSION" \
  --window "$WINDOW" --generation "$GENERATION" --out "$NEW_SNAPSHOT"
python3 -B "$UIB_BUNDLE/swiftui_simulator.py" pair \
  --session "$SESSION" --simulator "$OWNED_SIMULATOR_UDID" \
  --window "$WINDOW" --out "$NEW_PAIR_DIRECTORY"
```

`pair` requires the selected app's foreground scene and an unchanged window
metadata envelope. It writes `snapshot.json`, native lossless `screen.png`, then
`pair.json` last. A directory without `pair.json` is incomplete. It first captures
PNG in internal temp, then copies it to the chosen evidence directory. It never
uses the browser mirror, a resized JPEG, or a screenshot estimate as geometry.
Simulator PNG covers the display, including system chrome; geometry covers the
selected exposed SwiftUI tree. These are related but different coverage scopes.

`pair.json` binds run/window/observation and hashes, native pixel dimensions,
reported screen point dimensions/scale/orientation and window-to-screen mapping.
It records a **sequential, non-atomic** pair; keep the state settled and do not
interact during the call. Missing geometry is not supplied from the screenshot.
Window metadata stability alone does not prove that every animated element was
stable. Different scenes/windows are not automatically merged.

Source rectangles remain in their own local pt spaces. Use their qualified
SwiftUI affine to the recorded hosting-root global space, then a separately
qualified host/global-to-screen relation and the reported screen-to-PNG scale.
The bundled initial profile was checked against public GeometryProxy observations
and exact PNG rectangle pixels at390×844/1170×2532 and440×956/1320×2868. Those cases
matched at3px/pt with less than1px raster rounding. This is not a universal proof
for transformed windows, external displays, every presentation or device runtime.
The pair deliberately keeps `swiftui_host_mapping` marked as requiring that
source-profile qualification; do not manufacture cross-host transforms.

Use the matching Rust validator/measure on original canonical files. Native type
names may omit custom Views; `discover` lists actual nodes, not source symbols.
For a semantic toolbar/query component, preserve the chosen node's ancestry and
record the source/UI attribution separately; don't rename an unrelated rectangle.

## Current evidence and boundaries

Actual PlayPhrase.me Phone source was built with only diagnostic startup/package
integration. Search/More open and close produced fresh complete exposed trees;
toolbar/query were measured at two Phone viewport widths. The same mechanism
captured Reels4:3/front/actions and Collections backface, Clip grid/Filters/viewer,
and the shared Download form from Search. Download from Clip showed its real
unavailable state; this does not qualify a ready Clip Download form. No export
was submitted, account changed or product layout repaired during capture.

Runtime type-name normalization admits long generic specializations by applying
the existing byte ceiling to the exported nominal name; generic arguments and
model values are not exported. Release compilation excludes the runtime hook.
These are bounded local scenario checks, not complete application/device QA.

If pairing refuses `foreground_scene_required_for_png_pair`, restore the intended
app through its authorized UI workflow, then make a new request. Wrong device,
changed window metadata, unsupported profile and source refusals remain distinct.
Do not bypass limits or retry an unknown source outcome automatically.

After consumers finish, stop only owned apps/helpers, uninstall run-owned test
installations and shut down owned devices as the project requires. Preserve the
accepted native PNG/canonical pairs and reusable app only under a named handoff.

## UIKit-only menus and popups

Use a release whose SDK descriptor advertises `capture-uikit`, and its matching
caller. SDK0.12.0 does not have this capability: update the app's tag-pinned
`UIBlueprintSwiftUIRuntime` dependency and build the diagnostic app once using
its owner's existing recipe. Updating only the CLI cannot change an old `.app`.
Keep that new app running for all subsequent popup changes/queries.

```sh
python3 -B "$UIB_BUNDLE/swiftui_runtime.py" windows --session "$SESSION"
python3 -B "$UIB_BUNDLE/swiftui_runtime.py" capture --source uikit \
  --session "$SESSION" --window "$WINDOW" --generation "$GENERATION" \
  --out "$NEW_SNAPSHOT"
python3 -B "$UIB_BUNDLE/swiftui_runtime.py" tree --source uikit \
  --session "$SESSION" --window "$WINDOW" --generation "$GENERATION"
python3 -B "$UIB_BUNDLE/swiftui_runtime.py" measure --source uikit \
  --session "$SESSION" --window "$WINDOW" --generation "$GENERATION" \
  --type "$OBSERVED_CLASS" --relation width --bundle "$UIB_BUNDLE"
```

`--source uikit` also works on Simulator `pair`. Default capture/discover remain
SwiftUI; there is no fallback or mixed tree. `discover --source uikit` is not an
operation: inspect the canonical UIKit `tree` instead. Existing exact
`--subtree-type` selection works on fresh nominal UIKit class metadata and refuses
multiple matches. Type names are framework observations, not stable app IDs.

A popup may live inside the main window's UIKit hierarchy. A new nonkey window
can instead be `UITextEffectsWindow`; neither `is_key` nor window enumeration
alone identifies a popup. Select each relevant window explicitly from the fresh
list and inspect its hierarchy; do not assume the highest/newest window is it.
In the owned Menu fixture, `_UIContextMenuView` was in window1 while window2 was
the text-effects window. This is evidence for selection, not a class-name rule.

Namespace `uikit.runtime.layout` reports UIView model `bounds`, parent-child
links, and verified affine mapping to the selected window in points. Nonzero
bounds origins are preserved. The qualified focus correction also admits a
3D matrix representation only when the entire layer ancestry preserves z=0
exactly with constant homogeneous denominator1; this includes inert m34 in
settled tvOS focus descendants. Effective perspective, tilt/depth under
perspective, nonfinite or unverified transforms remain unknown. Rust does not turn a rotated
bounding envelope into an exact rectangle relation. Do not substitute frame,
AX bounds, pixel estimates, SwiftUI global coordinates or a presentation frame.
No text, accessibility values, model values or raw descriptions are collected.
Complete coverage means this window's UIView hierarchy and requested bounds,
not every rendered glyph/layer, visible region or surface in another process.


## tvOS Simulator (0.14.0)

The TV-enabled SDK and matching `swiftui_simulator.py` admit
`tvos-simulator` explicitly on arm64 tvOS 27.2. Never relabel it as iOS or use the
unchanged 0.13.0 Simulator caller for pairs. Runtime caller and Rust wire remain
compatible. Use a supplied immutable TV `.app`, exact owned UDID, a fresh private
run root and the TV recipe. Launch once, then use the normal windows/discover/
capture/measure commands and `pair --simulator UDID --window WINDOW` repeatedly.
One `.app` may be installed in multiple devices; never copy another device's
container/session. A first launch may show the existing TV sign-in presentation;
close with the TV Back command to stay guest, never complete account activation.

The default source is SwiftUI; `--source uikit` selects UIView model geometry.
TV screen points, screen scale, safe area and window-to-screen matrix are reported;
PNG mapping and model-versus-presentation bounds need the supplied qualification.
Focus animations and held remote states require fresh reads, not a saved snapshot.
`pair` is sequential, not an atomic frame. Physical Apple TV is unqualified.
TV input via serve-sim currently provides a stream but remote-button support is
not qualified. Device Hub's explicitly selected TV has Back/Select/directions/
Play-Pause controls; physical desktop input requires its existing reservation.
No geometry request needs that desktop reservation. See the local handoff for
actual tested states and pending input coverage; one screen is not all-TV proof.


Qualified TV geometry: a public SwiftUI GeometryProxy oracle and native PNG agree
for a translated/scaled marker (global240,160,225×112.5pt → pixel480,320,450×225
at2x). This qualifies the tested single hosting root, not arbitrary cross-host
transforms or animated presentation bounds. Actual PlayPhrase.me TV ordinary,
actions-after-hold, changed focus, fullscreen and return captures validated on
one unchanged app; Rust player1200×675 →1920×1080 →1200×675pt. UIKit model capture
also validated. Device Hub Back, held Select, directional focus and double Select
worked in those cases. Settings/child-list and capture during an actively held
button were not qualified: Device Hub later timed out. Do not label them passed
or redesign the app to satisfy capture. Two independently addressed TV consumers
passed wrong-device refusal and stop/relaunch isolation; provisioning remains
with each consumer coordinator, not the Blueprint collector.


### Settled TV focus descendants

The general UIKit mapping correction is SDK-side: updating only this caller
cannot add transforms to a0.14 app or a saved unknown Snapshot. Build once with
the corrected SDK, then repeat the original `pair --source uikit` on the selected
owned window. A standard focused SwiftUI List fixture now reports verified
window transforms for its floating-content descendants. Preserve original node
keys/ancestry and bounds; do not infer a centered expansion from class names or
pixels. Consumer-specific focus screens still require their own fresh capture.

For a Rust query in window space, copy the target geometry's existing known
`transform.transform` record into `evaluation_input.transforms`, preserving its
source/target, surface and environment/observation evidence. Bind the original
snapshot ID/revision/context, use that transform's `to` as result_space and keep
the query anchor in its original local coordinate_space. `--space` alone supplies
no transforms. Do not construct a matrix from an assumed inset. The fixture's
window-width query is in `tests/bridges/uikit_focus_geometry_check.py`.

These remain model layout bounds, not focus shadow/ink, paint, visibility, hit
regions or animated presentation bounds. An affine mapping may rotate/skew the
rectangle; do not call its AABB an exact rectangle. A projective mapping is still
unsupported by this source; no four-corner-only qualification or arbitrary3D
projection fallback was added.


## Configured UILabel typography (explicit opt-in)

Use a matching SDK endpoint advertising `capture-uikit-label-style`. The old
layout-only snapshot does not contain fonts/colors; update the diagnostic SDK and
build once. Then reuse that build for all states. Request only when needed:

```sh
python3 -B "$UIB_BUNDLE/swiftui_runtime.py" capture --source uikit --label-style \
  --session "$SESSION" --window "$WINDOW" --generation "$GENERATION" --out "$NEW_SNAPSHOT"
python3 -B "$UIB_BUNDLE/swiftui_simulator.py" pair --source uikit --label-style \
  --session "$SESSION" --simulator "$OWNED_SIMULATOR_UDID" --window "$WINDOW" --out "$NEW_PAIR_DIRECTORY"
```

Each UILabel's existing node has `source_declarations` named `label_style_*`:
font_name/family/point_size(pt)/symbolic_traits and optional font_weight_trait,
text_color/highlighted_text_color extended-srgb RGBA, fit/highlight/enabled flags,
and actual interface-style/contrast/gamut/legibility traits. Keep node SourceKey,
snapshot/window/run and `label_style_observation_id`; metadata is one observation,
not an independent lookup to join onto an older snapshot. Availability is explicit;
nil highlighted color is absent, not black/transparent. Extended-sRGB conversion
may produce finite components outside0...1; don't silently clamp or rename them.
UIFont's numeric weight trait is not CSS weight. The full descriptor is not dumped.

These are CONFIGURED UILabel settings. `effective_paint_font/color` remain unknown:
no attributed runs, glyph fallback, autosizing or rendering effects are inspected.
`adjusts_font_size_to_fit_width=false` alone does not prove actual glyph font.
No label text, attributedText, accessibility values, model data or descriptions
are read. Default commands omit this data; SwiftUI and old SDKs refuse opt-in.
The screen PNG privacy boundary remains separate from metadata filtering.

SF Symbols: UIImage.SymbolConfiguration on the qualified27.2 SDK has public
constructors/removal/equality but no pointSize/weight getters. Image bounds or a
nil preferred configuration do not prove the resolved symbol settings. Keep those
fields unknown; never inspect descriptions/private ivars or infer a font size.


## Explicit SwiftUI font resolution probe

UIKit label metadata cannot resolve arbitrary SwiftUI Text. Where authorized,
add a Debug-only `BlueprintFontResolutionProbe(font: SAME_FONT, role: STATIC_ROLE)`
in the source Text's background immediately at its font boundary, preserving all
font/layout modifiers. The probe reads inherited `fontResolutionContext` from
that mount; it never reads Text content or uses a default EnvironmentValues.
This is instrumented source attribution, not universal automatic typography.
One qualified source build is reused. Keep the entire import/background inside
`#if DEBUG`; the public resolver requires26+, minimumdeployment need not change.

Launch that diagnostic build with both existing `UIBLUEPRINT_RUNTIME=1` (launcher)
and recipe environment `UIBLUEPRINT_FONT_RESOLUTION=1`. Without the latter the
probe mounts nothing. Ask explicitly using matching capability:

```sh
python3 -B "$UIB_BUNDLE/swiftui_simulator.py" pair --source uikit --font-resolution \
  --session "$SESSION" --simulator "$OWNED_SIMULATOR_UDID" --window "$WINDOW" --out "$NEW_PAIR_DIRECTORY"
```

The UIKit anchor node's `font_resolution_*` source declarations report role,
mount_id/context_revision, resolvedpoint_size(pt), publicCTFontPostScript/family/
traits and bold/italic/monospaced flags. Originalnode/window/run/observation binds
these values. The supplied Font is resolved on each requestedcapture using the
latest SwiftUI environment update at that mount; pendingUIupdates are not drained
and consistency remainsunknown. Do not rename the zero-sized anchor's rectangle
as the Text's geometry, or join it to another snapshot. Report source role/delta.
Unmount removes the anchor; a remount has a new mount identity. No globalregistry.

Before trusting an integration, compare samecomposition probeoff/on publicbounds
and nativePNG. ControlledTVqualification matched PNG exactly and resolved68→104→68
pt across actualDynamicTypecontextchanges, with removed/remountedprobe identity
checks. These fixture values are NOT the font of a different app/header. The
consumer must repeat in its actualmountedsourcecontext and verify appearance.
UIFontmetadata/glyphfitting/attributedstyles/ink/paint/SFconfig are distinct:
`effective_paint_font` remainsunknown. No Font.Contextdescription/fullEnvironment,
model/text/attributedruns or CTFontdescriptor dump is exported. Oldendpoints
refuse `--font-resolution`; oldsavedSnapshots cannot gain resolution metadata.
