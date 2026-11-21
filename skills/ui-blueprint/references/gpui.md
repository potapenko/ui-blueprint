# GPUI window-root geometry

Use original canonical Snapshots through [saved analysis](analysis.md). Require the
app owner's authorized Debug capture route and matching source SDK. An installed
CLI does not inject instrumentation into another application. This recipe does not
authorize app changes, installation, permissions or input actions by itself.

## Ordinary selection

For an explicitly chosen window, omit the component selector to capture its
registered window root and available mounted marked subtree. Do not ask the user
for a root ID or a repeated marker list. An optional component selector narrows to
one unambiguous current subtree. Missing/ambiguous window, root or selector refuses;
never substitute an old small sample, another window or only a convenient region.

The source SDK registers a Debug Base Root plugin once before creating windows:
`register(Configuration::application(app, build, run).required_anchors(...), cx)`.
`Session::for_window(window, cx)` binds its real Root/WindowHandle. The normal request
is `Request::capture(id, state, deadline, bytes)`, with optional `.component(id)`.
The old `Request::new(...selected...)` is an explicit compatibility mode, not the
default route. Require the SDK's supplied revision and integration instructions;
source builds are qualified separately from the installed saved-data CLI.

## What is measured

Dynamic `Session::mark` registrations use public structural labels and real native
identities. Repeated rows have distinct opaque occurrences; never export content,
account or item values as their IDs. Required anchors are a minimum, not a complete
list of descendants. Native-key replacement, removal and root/window recreation
invalidate old bindings. The producer validates the entire emitted set.

When the vendored SDK provides `Session::mark_or_render`, keep its returned
element and inspect the accompanying result. `UnmeasuredMarker` identifies a
validated public label whose instrumentation failed. An erased element remains
interactive. Include that gap in the app's diagnostic report if consumers need
its name. A saved Snapshot contains measured occurrences only. Older SDK source
pins require marking the typed native element before erasure.

A root request collects admissible normal and deferred prepaint samples. Publication
waits for post-draw retirement; GPUI0.3.12 on_next_frame callbacks alone run before
drawing. Cached geometry cannot be re-labelled as a fresh sample. Requests and
live registrations are bounded; overflow must not look like complete collection.

Keep namespace `gpui.layout`, original SourceKeys and `gpui-window-logical-px`.
These are GPUI logical Pixels, not DevicePixels. Source scale is separate. Layout
is not AX/hit/paint/visibility, and text layout is not glyph contours. All marked
nodes does not mean every renderer element: account for meaningful regions and
controls, unmarked internals, unmaterialized rows, native chrome and other surfaces.
A marker count alone cannot satisfy useful window coverage.

Saved data remains cache/unverified. A layout pass is not a presented GPU frame;
source scale alone does not establish screenshot mapping. Do not merge separately
hosted roots with unrelated origins. Independent image/input evidence remains
separate from canonical geometry and source-test results.

## Existing analysis path

Inspect returned structural marker IDs and select their actual recorded keys.
Use `inspect --view design`, `measure` and `check`; there is no `import-gpui` command
or need to rewrite coordinates into another format. Requirements need their own
source. The native example's8→18 gap is a diagnostic stimulus, not client design.
Run root capture after meaningful state changes, then verify geometry, appearance,
remount and limits as applicable; do not poll to manufacture freshness.
