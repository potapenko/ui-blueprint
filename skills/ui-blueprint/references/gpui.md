# Opt-in GPUI layout observations

Use a supplied original canonical Snapshot through [saved analysis](analysis.md).
An app owner may explicitly integrate the separate `uiblueprint-gpui` Debug source
package. Require its compatible revision, pinned gpui-kit/GPUI version and authorized
capture path. The installed CLI archive does not implicitly contain or inject it.
Source integration/builds require their own authorized scope and supplied SDK
instructions; this recipe alone does not authorize changing an app or installing it.

The qualified source path is gpui-kit0.9.1/gpui-pre0.3.12 on macOS ARM64, Rust1.100.0.
Only explicitly registered, native-ID-bearing elements are measured after layout.
Semantic marker IDs, current native mount generation, session/window incarnation,
capture sequence and layout pass are distinct. Parents/component membership are
declared and checked against marked runtime ancestry, not a recovered full tree.
Anonymous, unmounted or virtualized elements do not become known geometry.

Keep namespace `gpui.layout`, recorded SourceKeys and space `gpui-window-logical-px`.
Here `px` means GPUI logical Pixels, not DevicePixels. Source scale is separate.
Never substitute AX, hit, visible or painted bounds for layout; their unknown
status is intentional. Layout text frames are not glyph contours.

Capture is explicit and bounded. The app requests a redraw, waits for actual
measurement completion, obtains `Session::take` and calls `Capture::to_json` on
the same owning window/session. Publication rechecks frame and mounts; old IDs,
duplicates, missing selected ancestors, cancellation and limits refuse. Native
GPUI0.3.12 next-frame callbacks execute before drawing; use the SDK's qualified
host example rather than assuming a single callback means capture completed.

Saved output remains cache/unverified, even when just written. A layout pass is
not a GPU/presented-frame ID. Source window scale alone is not a screenshot map;
matching native window/state and separately verified transforms are required.
The SDK emits no screenshots or actions. Read-only acquisition never changes
focus, values, scroll or window size to improve coverage.

Inspect declared marker IDs to choose the actual returned keys, then use existing
`inspect --view design`, `measure` and `check`. There is no `import-gpui` command
and no need to rewrite geometry into a second JSON schema. Queries reference the
original geometry and coordinate spaces. Requirements need an independent source;
the example's8→18 gap is a diagnostic stimulus, not a client design rule.

Record real runtime checks and source-test evidence separately. Native input,
probe-on/off, resize, scrolling, remount and pixel pairing require their own
qualification; a synthetic snapshot or a green build does not supply it.
