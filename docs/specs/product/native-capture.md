# Exact native window PNG

- Node type: leaf; domain: `uib.native-capture`; contract: `UIB.NATIVE-CAPTURE@1`.
- Authority: Active / Evolving; runtime acceptance pending.
- Authority source: direct user messages01a1ddf8-8471-7e91-88d4-dc38e80cf501 and
  01a1ddfa-0e60-74d2-b771-de13f9467652, chat01a1dcba-7955-7591-9bfb-4d0078eea8cf,
  2026-11-15: Blueprint itself captures the native window; donors remain read-only.
- Requires: [NATIVE@5](native.md), [IDENTITY@1](identity.md),
  [GEOMETRY@2](geometry.md), [PRIVACY@1](privacy.md), [CLI@21](cli.md).

## CAPTURE.CALL — One ordinary installed command

`uiblueprint capture-window --pid P --window-id W --expected-bundle B
--out NEW_SYSTEM_TEMP_DIRECTORY --request-id ID [--deadline-ms N]
[--expected-launch-time SECONDS]` uses the matching installed native-host-helper.
The macos feature and matching helper are required; unavailable builds refuse5.
The app must already be running and its selected window authorized for capture.
No application launch, focus, resize, scrolling, UI content search, system permission
prompt, install or retry is implied. PID/window are explicit; no alternate-window
fallback. The caller may pin a known process launch time; otherwise the command
records the currently resolved launch at entry and holds it through publication.

`capture-window --list-windows --pid P --expected-bundle B` resolves at most32
CGWindowIDs/frames/layers/on-screen flags for that PID with process launch identity.
No capture or automatic candidate selection occurs. It outputs no titles or foreign
window details; missing/overflowed inventory refuses. CG inventory allocation is
opaque, not a promised OS-side pre-filter. The optional expected launch also applies.

## CAPTURE.IDENTITY — Read-only external binding

Resolve public NSRunningApplication PID/bundle/launch identity and exact CG window
owner. Hold the selected ScreenCaptureKit window for the operation. Check the same
process incarnation and exact window geometry before/after acquisition and before
publishing success. Display-layout changes also invalidate mapping. Changed, missing
or wrong-owner targets refuse; IDs alone never become reusable mutation refs.
This is a bounded observation of a selected current window, not a proof that a CGID
never changes/reuses between separate requests. Caller-request IDs remain explicit.
No title or unrelated window content is output; SDK JSON is never modified.

## CAPTURE.IMAGE — Existing native acquisition

Reuse existing CaptureLifecycle, CaptureGeometry and capped NativeArtifacts PNG
encoding. Natural source scale comes from SCContentFilter.pointPixelScale, never
inferred only from buffer size. Full single-window frame, no shadow/cursor/audio,
SDR, no child windows, no resize/upscale or image recomposition. Lossless PNG only.
The caller supplies a new directory under system temp; create private exclusive
files, never overwrite. Original/staging/failure images remain and are never cleaned
by the agent. Advertise only a complete validated PNG; failed images stay unadvertised.
PNG is independent native capture, not a CUA frame or a renderer recreation.

## CAPTURE.METADATA — Association without a producer change

One bounded JSON success response plus capture.json beside window.png contains:
format_version1.0.0, request_id/capture_id, source tool identity, actual PID/bundle/
launch, window ID, request/acquisition timestamps and clock, window-server and
ScreenCaptureKit frames, filter content rect, source scale, actual pixel dimensions,
image format/hash/byte count, convention and source-qualified screen→image affine.
For the verified no-shadow full-window case, window-local→image is the same scale
with zero translation. Otherwise report mapping unknown with a fixed reason.
The matching verified bundle manifest supplies product version/source revision.
Origins and units are explicit. Content/layout-root→window mapping remains a
separate consumer fact: a producer's root-global metadata can support it when that
coordinate meaning and same process/window/state are verified. Empty captures=[]
in the original SDK Snapshot is not a prohibition on this external association.
Equal metadata endpoints do not prove an atomic GPU frame, unchanged animation or
visibility/occlusion. Layout/AX/hit/paint bounds are not inferred from PNG metadata.

## CAPTURE.LIMITS / FAILURE

Default operation deadline5000ms, caller range100–10000ms; monotonic watchdog and
existing capture callback deadline bound exit. No polling or retry. Fixed capture
admission ceilings4096×4096,8,388,608 pixels,64MiB raw/PNG,16KiB metadata; source
allocation internals remain opaque. Over-limit refuses instead of downscaling.
The output destination is validated before capture. Existing files/symlinks,
malformed/duplicate flags, invalid bounds/identity and unavailable permission fail.
stdout success is one full JSON object; errors are payload-free fixed diagnostics
on stderr. Exits0 success,1 IO/internal,2 invalid/limits,4 unavailable/stale/deadline,
5 unsupported platform/feature. Failed post-capture validation never publishes
success; retained image artifacts are not automatically accepted by their existence.

## CAPTURE.ACCEPT / protected boundaries

Focused negatives cover malformed args, destination collision/symlink/outside-temp,
wrong PID/bundle/window, stale expected launch and output/geometry limits. Existing
capture mapping/writer tests remain applicable. Actual SwiftUI reference and GPUI
native example must produce independent lossless PNG plus attributable metadata,
verified dimensions/scale and useful existing-layout association without donor edits.
Known resize between explicit captures yields new metadata; a changed identity or
geometry during capture refuses. No extra source/UI/SDK round-trip gate is invented.
Local candidate installation retains old version/rollback and matching skill recipe.
Independent review covers capture scope/privacy/binding. No remote publication.
