# Explicit GPUI layout probe

- Node type: leaf; domain: `uib.gpui-probe`; contract: `UIB.GPUI-PROBE@3`.
- Authority: Active; stability: Evolving; bounded native root-mode qualification recorded in the package recipe.
- Authority source: original approval message01a1db4b-9e3c-72e3-9f40-7babe4604262
  and parallel execution01a1db4c-a361-7683-9804-0461d38e25fb; direct root/default
  implementation message01a1dbc5-5599-7481-aed8-7eda49313fb1, 2026-11-14.
- Read when: integrating or qualifying the opt-in GPUI producer.
- Requires: [BOUNDARIES@1](boundaries.md), [MODEL@1](model.md),
  [GEOMETRY@2](geometry.md), [IDENTITY@1](identity.md),
  [PRIVACY@1](privacy.md), [EXCHANGE@2](exchange.md), [CLI@21](cli.md).
- Root outcome: [WINDOW-PROBE@1](window-probe.md); previous scoped evidence and
  integration instructions: [package recipe](../../../plugins/gpui/README.md).

## GPUI.ROOT — Default to the selected window's actual root

The existing `plugins/gpui` producer adds app-local Debug registration through
public gpui-kit0.9.1 Base Root hooks, before windows are created. No file IO,
network, actions or installation belongs to that registration. Each actual
Root/plugin instance owns one session bound to its exact WindowHandle/Root entity
and a private per-window runtime ownership witness (numeric IDs alone can repeat).
`Session::for_window(window, cx)` resolves only that registered root; unavailable,
changed or ambiguous binding refuses without choosing a small sample.
`Request::capture(id, state, deadline, bytes)` has no component selector by default:
collect the window root and all admissible mounted marked occurrences. Optional
`.component(public_id)` narrows to one current unambiguous marked subtree.
The caller never repeats a root ID or list of descendants for this normal path.
`Configuration::new` / `Session::new` / `Request::new(...selected...)` retain their
previous exact-subset contract, including ancestor and missing-marker refusals.

## GPUI.COLLECT — Runtime markers, one bounded producer

Application configuration supplies public app/build/run attribution and optional
required anchor names. Anchors are a minimum, not a whitelist of every descendant.
`Session::mark` in root mode accepts dynamic public structural IDs on existing
native-ID-bearing elements. Repeated structural labels are separate native
occurrences with opaque mount IDs; raw content/account/item IDs are never exported.
The root boundary uses actual framework-root identity, not an anonymous-child
identity exception. Ordinary anonymous/erased child identities still refuse.
The decorator forwards the existing layout ID, painting and accessibility behavior;
no new layout node, hitbox, focus boundary or visual surface is introduced.
Only requested geometry is read during the selected prepaint pass. Lightweight
registration/lifecycle metadata is not periodic geometry collection. No text/value
read, whole private-tree discovery, background acquisition or old-geometry fallback.
Deferred same-window marked layers participate in the same bounded frame collection.
Publication occurs after its complete layout/lifecycle boundary, not before deferred
prepaint or while a prior frame still owns removed occurrence state. Cached replay
cannot silently turn a missing current sample into successful full marked coverage.

The Reels integration at ed495f2 exposed an erased native identity during render.
For optional instrumentation, `Session::mark_or_render` returns the original
renderable element on refusal and a separate `UnmeasuredMarker` result. It
reports the fixed reason and the validated public label when available. A
malformed label is omitted. A successful marking result confirms attachment
only. Capture still validates required anchors and emits actual measured samples.
An erased child creates no synthetic identity, node or geometry. Keep named
render-time refusals with the consuming app's diagnostic results. The strict
`Session::mark` API and saved snapshot format retain their existing contracts.

## GPUI.IDENTITY — All emitted occurrences remain valid

Session/window/root incarnation, semantic label, native key/type mount, request
sequence and layout pass are distinct. Exact native ancestry/window hosting supplies
attributed occurrence parents; repeated labels must not join unrelated children.
Native-key replacement, root replacement, close/remount and virtualized occurrence
reuse invalidate previous bindings. Retire missing registrations; churn cannot
accumulate unlimited old IDs. Validate every emitted occurrence before publication.
Duplicate native registration, missing required anchor, stale expected mount/root,
changed window, cancellation, deadline and overflow fail without partial success.
Other OS windows need their own explicitly selected session; no app-wide fallback.

## GPUI.DATA — Existing canonical consumers and truthful coverage

Use unchanged core0.1 Snapshot, namespace `gpui.layout`, channel `opt_in_layout_probe`,
projection `design`; Rust inspect/measure/check remain the analysis engine.
Preserve GPUI logical Pixels, window-local top-left bounds and source scale; they
are not DevicePixels and do not create a screen/image transform. Layout is not AX,
hit, visible or paint data. Unknown/unsupported fields remain explicit.
Report root versus component selection and observed marked coverage. All mounted
markers does not mean every renderer element. Native chrome, unmarked internals,
unmaterialized virtual rows and unsupported separately hosted surfaces remain gaps.
Do not merge unrelated origins or pretend an unvisited surface was measured.
Saved documents remain cache/unverified; completed layout is not a presented GPU
frame. Only public identity, geometry, scale and fixed diagnostics are serialized.

## GPUI.LIMITS — Local lifecycle

At most256 live/selected occurrences, ASCII public IDs1–64 bytes, bounded ancestry,
one pending request per session, deadline1–10000ms, output cap1–1048576 bytes.
No silent truncation or limit expansion. Nonfinite/negative geometry, malformed
scope and ambiguity refuse. App owns private transport/system-temp output.
Release collection is disabled; closing a session retires only its own resources.

## GPUI.ACCEPT — Whole useful native example

Use the existing Blueprint native example, not the PlayPhrase.me client. One
unchanged default capture must include root, toolbar/content/control frames, all
materialized row markers, a dynamically added branch/row and shown same-window
DEFERRED overlay. Required anchors stay distinct from optional occurrences.
Verify component narrowing, identity/parent integrity, resize/scroll/cached content,
known runtime gap8→18, keyed replacement, disappearance, bounded churn, second-window
isolation and negative limits/lifecycle cases. No fabricated geometry JSON.
Actual native on/off preserves layout, focus and input; PNG pairing has its own
qualified mapping. Tests, native evidence and independent installed-skill use are
separate. Existing @1 scoped evidence does not itself accept this new root path.

GPUI-WINDOW-002 evolves only root selection/registration under the direct instruction.
Owner01a1db1e writes plugins/gpui, this leaf and its GPUI recipe; shared packaging,
CLI/schema/engine, Swift sources, client/admin and remote release are outside this
packet. Root owns installation/release and overall B0 disposition.
