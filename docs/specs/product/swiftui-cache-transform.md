# Qualified cached SwiftUI translation

- Node type: leaf; domain: `uib.swiftui-debug-capture.transform`.
- Contract: `UIB.SWIFTUI-CACHE-TRANSFORM@7`; Authority: Active; Stability: Evolving.
- Authority: direct approval of the S13 plan in chat
  `01a20922-f6ad-7651-997b-cef1b3f09b12`, 2026-11-23, to obtain source-attributed
  parts in a proved common window/native-frame space on the unchanged donor.
- Requires: [debug capture](swiftui-debug-capture.md),
  [GEOMETRY@2](geometry.md), [PRIVACY@1](privacy.md).
- Acceptance: bounded root-translation fixture verified; independent review,
  native-image mapping and S13 product qualification pending. No product admission.

## TRANSFORM.COLLECT

The selected graph's recorded `transform` property may be inspected alongside
position and size. Preserve the original geometry; never sum tree-parent origins.
Use the same exact image profile, graph ownership, cache validity, indirect-reference,
value descriptor, value-witness, field offset and protected-model-range checks.
The observed ViewTransform profile has size/stride48, alignment8 and four fields:
head0, spaces8, positionAdjustment16, pendingTranslation32. Validate the exact
SwiftUI nominal descriptor and metadata field layout before reading storage.
A metadata match alone is not semantic or runtime transform qualification.

The profile handles exact ViewTransform Element/BufferedElement chains, at most16
links and64 buffered items. `available` is free space, not allocation size.
Before any buffer header, query the fixed read-only allocator `malloc_size` for
that validated retained buffer pointer. Require a positive bounded extent, every
header/payload inside it, and final used+available equal to that extent. Cache the
extent only within the stopped observation; zero/foreign/invalid extents refuse.
This fixed libsystem_malloc call consumes the existing call/deadline budget;
no payload copying, graph evaluation or memory-protection change is added. Check metadata, superclass, field offsets, generic
arguments, item sizes, cycles, depth and retained linkage before numeric reads.
SizedSpaceElement, CoordinateSpaceElement and ScrollGeometryItem add no affine
operation; their payloads/names stay unread. TranslationElement supplies its
qualified finite CGSize offset. Unknown kinds leave mapping unknown. The spaces
registry is never dereferenced. No arbitrary heap/model values are collected.

An inherited transform has a recorded raw-parent path to its nearest property.
The owned fixture matches all11 public-anchor rectangles at initial, scrolled,
resized and translated states. A scale control keeps27 affected records unknown.
Canonical output may expose this qualified translation to the selected hosting-root
local space. Preserve source positions; affine translation is minus
positionAdjustment, pendingTranslation and recorded chain translations. Rust
consumers perform relations in that destination space.

A scoped root may read its nearest transform along the raw parent path, including
outside-scope ancestors as coordinate context only. Preserve parent/subgraph
ownership, original read/depth/record limits and model protection for that path.
Missing or ambiguous ancestry leaves mapping unknown. No outside position, size
or model payload is collected. Dirty/stale transform storage remains unknown;
never substitute a neighboring transform. Root mapping is not full owner coverage
or native-image qualification.

## TRANSFORM.QUALIFY

Before publishing a shared-space mapping, compare actual collected rectangles
against independently resolved SwiftUI root anchors on an owned fixture. Include
nested layout, nonzero placement, scrolling, resize and a nontranslation control.
An unsupported chain remains unknown even if numbers happen to match. Keep the
public source distinction between recorded transform inputs and derived conversion.
Root-local qualification does not establish a native-window or PNG transform.
Those require same-generation host/window membership, sourced native conversion,
origin/scale and exact capture metadata. Never use screenshot tracing or a fitted
constant to assert that mapping.

## TRANSFORM.NATIVE — owned-fixture qualification

The native candidate may obtain NSWindow.frame, the selected NSHostingView.bounds,
its isFlipped flag and convertPoint:toView:nil for (0,0), (1,0), (0,1). Require
that host.window is the exact selected window before/after these fixed getters.
Only zero-origin flipped host bounds are admitted by this initial profile.
Preserve FP registers under the existing call/restore protocol; no expression,
SwiftUI callback or graph evaluation is permitted. Existing deadlines/call limits
and owner recovery apply. Validate finite points/rectangles and matching bounds/frame, isFlipped and the
full three-point basis on recheck. Derive the affine basis into full-window top-left pt from those
reported points and the reported window height. Do not fit an image offset.
The owned fixture reports bounds360×252pt and native basis (0,252), (1,252),
(0,251); independent Blueprint PNG is720×504 at reported scale2 with known full-window
mapping and the same process/window. Root-to-window publication is qualified for
this fixed ABI/profile; independent review and product acceptance remain separate.
Compose that sourced affine with each known cached translation, preserving raw
rectangles and derived evidence. Unknown native mappings retain hosting-root
geometry; unsupported cached transforms stay unknown. PNG association remains a
separate same-state capture record, never a presented-frame atomicity claim.

## TRANSFORM.COMPATIBILITY

Canonical core0.1, existing position/size numbers, node identities, quota ceilings,
model exclusions, target admission and owner recovery remain unchanged. Unqualified
old data retains unknown transforms. Root-local mapping is separate from native
window or PNG qualification; do not publish either on this evidence. This registration authorizes the finite
experiment, not successful S13 acceptance or general private-API support.
