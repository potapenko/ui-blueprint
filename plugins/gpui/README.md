# GPUI window geometry

Opt-in Debug source package for gpui-kit0.9.1/gpui-pre0.3.12 and Rust1.100.0.
[GPUI-PROBE@3](../../docs/specs/product/gpui-probe.md) extends the existing producer;
core0.1 Snapshot and the existing Rust inspect/measure/check path are unchanged.
The separate Cargo.lock keeps GPUI out of normal CLI builds.

## Register the actual window root once

After `gpui_kit::init` and other Root plugins, before opening windows:

```rust,ignore
uiblueprint_gpui::register(
    Configuration::application("public.app.id", "exact-build-id", "unique-app-run")
        .required_anchors(&["toolbar", "content"]),
    cx,
)?;
```

Gate registration/instrumentation behind the consuming application's explicit Debug
feature. `Session::new`/registration also refuse Release collection. Required
anchors are a minimum, not a list of every optional descendant. Register before
creating windows: an already-created unregistered root is unavailable, not patched.

In the existing view's render/event callback, after Base Root creation:

```rust,ignore
let session = Session::for_window(window, cx)?;
let title = session.mark("title", div().id("title").child("Example"))?;
// A Result-returning render helper can compose marked existing elements.
session.request(Request::capture("request-1", "public-state-id",
    Duration::from_secs(2), 1_048_576), window)?;
```

An omitted component selector captures the actual window root and all admissible
mounted marked descendants. No repeated root ID, `selected()` list or special
window flag. Optional `.component("toolbar")` narrows the request; a missing or
ambiguous component refuses. All windows retain their own actual Root identity.

Markers need existing native ElementIds; apply builders before marking. Repeated
structural labels (such as `row`) are separate opaque occurrences. Use domain-stable
native keys, not row indexes; their raw values are never exported. A RenderOnce
wrapper may erase its ID: mark its actual native element or an existing parent
frame, without adding a layout node merely for instrumentation.

For optional instrumentation in a render function, keep the returned element
and report the separate marking result. Apply this after the element's builders
and before type erasure whenever its own geometry is required.

```rust,ignore
let (button, status) = session.mark_or_render("reels_action", button);
if let Err(unmeasured) = status {
    eprintln!("{unmeasured:?}");
}
toolbar.child(button)
```

An already-erased `AnyElement` still renders and handles input. Its public marker
appears in the unmeasured result with `InvalidIdentity` and has no bounds.
Invalid label text is omitted. A successful result here means the SDK attached
the marker. The app must request a capture to obtain measured bounds. Preserve
the marking result with consumer diagnostics if downstream users need the gap's
name. A missing required anchor still refuses capture. Older vendored SDK pins
expose only strict `mark`: update the source dependency to use this method.
Installing a newer saved-data CLI does not update the app's vendored SDK.

## Frame completion and saved data

The transparent Base Root decorator preserves original layout, focus and AX.
It collects normal and deferred prepaint, then a `window.defer` effect seals data
AFTER old-frame retirement. Requests force fresh callbacks through `window.refresh`;
last-paint cache data cannot silently become current geometry. Live missing
callbacks fail rather than publish a falsely complete marked set.
Read `Session::take` after completion, then `Capture::to_json` on that same session
and window. The latter validates every emitted occurrence. The app owns private
transport/output. The native example demonstrates the callback order: GPUI0.3.12
on_next_frame callbacks run BEFORE drawing, so one callback is not completion.

Only public IDs, geometry and fixed metadata are serialized. The namespace is
`gpui.layout`, space `gpui-window-logical-px`, with source scale separate. Saved
output is cache/unverified; layout pass is not a presented GPU frame. Native chrome,
unmarked internals, unmaterialized rows and other OS windows are not invented.
AX/hit/paint/visibility and general screen/image transforms remain unknown.

## Portable source dependency

`python3 plugins/gpui/package-source.py --revision FULL_COMMIT --out NEW_TEMP_DIR`
exports the unchanged SDK and required canonical schema library with standalone
manifests, exact locked runtime dependencies, MIT license and sdk-source.json hashes.
The consumer may vendor that directory in its authorized project; no dependency
on a sibling UI Blueprint checkout remains. This is an explicit source export,
not a registry publication or an automatic app integration.

## Run the native example

From the checkout, with the pinned toolchain/dependencies already available:

```sh
UIB_GPUI_RUN=$(mktemp -d "${TMPDIR:-/tmp}/uib-gpui.XXXXXX")
export CARGO_TARGET_DIR="$UIB_GPUI_RUN/target"
cargo +1.100.0 build --manifest-path plugins/gpui/Cargo.toml --example native --locked --offline
python3 -B plugins/gpui/package-example.py --binary "$CARGO_TARGET_DIR/debug/examples/native" --destination "$UIB_GPUI_RUN/on"
```

Open the returned app with authorized native tooling. **Capture window** writes
captures/capture-N.json with its exact binary hash. **Add row**, **Remove row**,
**Show details** and **Show overlay** change the mounted tree without changing the
capture call. All materialized row/control frames are marked. **Capture toolbar**
exercises explicit narrowing. Change gap8→18, text, scroll and window size.
For on/off package the same binary in a NEW temp destination with `--off`; use
identical actions and serialize native input/instances. Off emits no Snapshot.
Stop only the owned PID; all images remain, and only owned non-image output retires.

## Compatibility, source basis and qualification
`Configuration::new` / `Session::new` / `Request::new(...selected...)` retain the
exact-subset API. Do not double-instrument one native occurrence with two sessions.
The new source uses public RootPlugin::decorate, Window.defer/refresh and Element
hooks. gpui-kit0.9.1 source commit39ab849a93b5f15507b969164d49891db8d5b470 and
pinned gpui-pre/base sources were inspected; no upstream implementation was copied.
GPUI-pre LICENSE-APACHE was read; kit/base declare Apache-2.0. Local use is not a
redistribution audit of every asset/transitive dependency.
Eight integration tests cover legacy selection plus default root, dynamic/deferred
children, distinct repeated-row parents, component ambiguity, root replacement,
window/cross-App isolation, 140-generation churn, limits and fresh cached callbacks.
Native default capture returned35→41 nodes after new rows/branch/deferred overlay;
explicit toolbar narrowing returned8. Original JSON passed existing Rust analysis,
gap8→18, scroll/remount/resize and private-ID exclusion. Matched paste/capture/input
continuation passed on/off. Workspace/rows/footer/overlay pixels matched; CUA cursor
pixels prevent a full-toolbar bitmap equality claim. No universal renderer-tree,
Linux/mobile, client parity, installation or release claim follows from this SDK.
