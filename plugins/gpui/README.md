# GPUI layout probe

Optional Debug source package for selected GPUI runtime layout.
[Contract](../../docs/specs/product/gpui-probe.md): UIB.GPUI-PROBE@1.
Rust1.100.0, gpui-kit0.9.1/gpui-pre0.3.12; separate Cargo.lock keeps GPUI outside
core builds. No new importer, installed provider, global installation or client UI.

## Source and dependency decision

| Read source | Adopted / rejected |
| --- | --- |
| gpui-kit0.9.1, upstream longbridge/gpui-kit commit39ab849a93b5f15507b969164d49891db8d5b470 | Published GPUI re-export; no copied code |
| gpui-pre0.3.12 element.rs/window.rs/geometry.rs/elements/div.rs | Public Element delegation, prepaint bounds, exact window handle, logical Pixels and scale |
| gpui-base0.9.1 observe.rs/test_support.rs | Layout-transparent wrapping is applicable; its all-property observer is rejected because it reads text/values and retains last-paint data |
| gpui-kit0.9.1 test.rs/test/query.rs | Focused framework tests, not proof of native presentation |

GPUI-pre's LICENSE-APACHE (Zed Industries2022–2025) was read. kit/base manifests
are Apache-2.0; their cached packages contain no LICENSE/NOTICE. No upstream
implementation is copied. Local dependency use does not qualify redistribution
of assets/transitive dependencies. Exact resolved versions/checksums are in Cargo.lock.
Rust1.96 failed in gpui-pre-macos on Option::map_or_default; the subsequent direct
user request selected current stable1.100.0 for Blueprint and both GPUI projects.

## Integrate in an explicitly authorized app

Add this source package as an optional dependency, behind an application Debug
feature. Compile instrumentation with `cfg(all(debug_assertions, feature = "ui-blueprint"))`.
Do not inject it into another application or enable it in production by default.
`Session::new` also refuses Release collection with `Error::Disabled`.

```rust,ignore
let session = Session::new(Configuration::new(
    "public.app.id", "exact-build-id", "unique-app-run", "selected-component",
    vec![Marker::new("root", None), Marker::new("title", Some("root"))],
), window)?;
// Retain the session; use a Result-returning render helper for this snippet:
let title = session.mark("title", div().id("title").child("Example"))?;
let root = session.mark("root", div().id("root").child(title))?;
// App-owned explicit trigger; no periodic collection:
session.request(Request::new("request-1", "public-state-id", &["root", "title"],
    Duration::from_secs(2), 262_144), window)?;
// After the requested layout/paint completes, still on its owning window:
if let Some(result) = session.take(window) {
    let bytes = result?.to_json(&session, window)?;
    // The app owns any private system-temp write; SDK has no IO.
}
```

Every marker needs an existing native ElementId. A RenderOnce wrapper may erase
that ID; mark its actual ID-bearing element or an existing semantic parent frame.
Do not add a layout container merely to measure. Parents must match the nearest
marked runtime ancestor; unmarked/virtualized content is outside coverage.
Capture.mount(), surface_generation() and Request's expected-generation builders
support fresh binding. New sessions have distinct canonical IDs within one run.
On root removal create a new session; close/cancel retire pending work.

Native GPUI0.3.12 executes on_next_frame callbacks **before** drawing (window.rs:2008),
despite its doc wording. The example uses the following event for publication after
one requested redraw. It does not recollect or poll. See examples/support/view.rs.

## Run the native diagnostic example

From the UI Blueprint checkout, with installed prerequisites and cached dependencies:

```sh
UIB_GPUI_RUN=$(mktemp -d "${TMPDIR:-/tmp}/uib-gpui.XXXXXX")
export CARGO_TARGET_DIR="$UIB_GPUI_RUN/target"
cargo +1.100.0 build --manifest-path plugins/gpui/Cargo.toml --example native --locked --offline
python3 -B plugins/gpui/package-example.py --binary "$CARGO_TARGET_DIR/debug/examples/native" --destination "$UIB_GPUI_RUN/on"
```

Open the returned .app with authorized native tooling. Click **Capture**: its
captures/capture-1.json is actual canonical data. The package records the exact
binary SHA-256 in each Snapshot's public build identity. stderr/stdout are native.log.
Change gap, text and disclosure; scroll the list and use the window's Zoom action;
capture after each settled state. Type into the input, capture, then keep typing.
For on/off use the same binary and identical actions, package a NEW temp destination
with `--off`, and serialize the two app instances/input. Off emits no Snapshot.
The stdin commands are an optional app-owned diagnostic interface, not a live service.
Stop only the owned process. Keep all images; clean only owned non-image output.

## Qualification and limits

The local macOS ARM64 native run exercised11/12 markers, real8→18 logical-px gap,
long-text height22.5→67.5, scroll delta−256, child disappearance/new mount, and
viewport720×640→1920×1018→720×640. Existing CLI inspect/measure/check accepted the
original files; both gaps passed sourced zero-tolerance fixture requirements.
Input/focus survived capture and values were absent from Snapshots. Card and scroll
pixels matched on/off exactly in normal/expanded/scrolled states. The zoom pair's
cursor-overlay history differed; full zoom pixel equality is not claimed.
The ordinary titled-window pilot mapped logical content to PNG by source scale2
and verified content inset32; this is separate caller evidence, not an SDK transform.
Saved data remains cache/unverified; layout pass is not a presented GPU frame.
AX/hit/paint/visibility and arbitrary app-tree coverage remain unavailable.
Framework regressions cover adjacent-frame keyed replacement, same-run sessions,
stale publication, duplicate/missing IDs, limits, cancellation and deadlines.
Native qualification is scoped; no Linux/mobile, animation synchronization or
PlayPhrase.me product parity is claimed. B0 acceptance belongs to the consumer.
