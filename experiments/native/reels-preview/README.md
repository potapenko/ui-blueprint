# R11 inline preview-cell qualification

Owner: chat `01a209c1-eece-70d1-a7c1-f002c18b5401`; consumer: Reels
`01a1f09e-6b6f-73a0-9ddb-87bf77c4b2a3`. The user approved fixture stages 1–3
on 2026-11-23, explicitly excluding donor changes and donor attachment.

## Basis and scope

Registry90 → DEBUG-CAPTURE@18, CACHE-TRANSFORM@7, BACKEND@1, GEOMETRY@2,
MODEL/IDENTITY/PRIVACY/LIFECYCLE@1, EXCHANGE@2, CLI@21 and their dependencies.
Governance: implementation, product truth, QA, Apple platform and Computer Use.
This is a finite fixture experiment, not a new supported product capture mode.
No public collector, schema, quota, product admission or donor code is changed.

Reels supplied the structural declaration from immutable `e19b20c474`,
`PlayphrasemeApp/Application/Access/Presentation/LearnerAccessGateSubscribePreview.swift:41–86`.
The donor has one `FeatureGateSubscribePreview` owner, separate loading/loaded
`LazyVGrid` branches and inline `ForEach` cells, with aspect-fit background,
conditional image overlay, rounded clip and border overlay. No cell marker or
nominal application cell type exists. Fixture sizes/colors/bitmap are synthetic;
this is no claim to reproduce product dimensions or media.

## Method

[SwiftUIReelsPreviewFixture.swift](../../../tests/bridges/native/SwiftUIReelsPreviewFixture.swift)
preserves those inline boundaries. Public bounds anchors resolve at its root;
a nonvisual NSView adapter converts them independently into full-window top-left
points. It does not consume cache-reader output. `R11_NO_ORACLE` removes the cell
anchor modifiers; the root/native adapter stays in both builds.

[capture_boundaries.py](capture_boundaries.py) accepts only the two exact R11
fixture bundle IDs and owner containing `SwiftUI.LazyVGrid`. It reuses the existing
leaf-cut mechanism in a private reader, qualifying the framework's private
`HiddenForReuseEffect` identity through its descriptor, symbol and framework image.
The unique grid's `ResettableLazyLayoutRoot → Tree` must expose all 21 distinct
child boundaries. Coordinates, labels, oracle IDs and traversal index select none.
Cell interiors are excluded explicitly. Original scan/read/depth/node limits,
model protection, cache validity and stopped-process ownership remain enforced.
The main reader and public request decoder are unchanged.

[check_capture.py](check_capture.py) selects the complete structural set before
an unordered bijection to the independent anchors. It preserves reported local
frames, sourced/derived native transforms, unknown ordering and target generations.
Public anchors snap to pixels; comparison allows one physical pixel (`1/scale` pt)
for this fixture only, reports the actual error and never asserts exact equality.
Canonical validation and Rust `measure` independently verify width/height in the
mapped window space. This is not semantic identity across snapshots.

## Reproduction

The retained task configuration is
`/private/tmp/uib-r11-preview-k1hqezx6/task.json`. It names an internal runtime
with the frozen collector `710ebd24ff75c5c03b042e21653a46a011443b42`, matched
0.6.1 CLI/helper bundle, hash-pinned data-only debugserver and two fixture apps.
The verified platform is macOS27.0.1 arm64 / LLDB2103.0.34.407 only.
Build with `xcrun swiftc -parse-as-library -g -Onone -module-name R11PreviewFixture`,
an external task-owned `-module-cache-path` and build TMPDIR, then ad-hoc sign only
the fixture with `com.apple.security.get-task-allow`. Compile the second app with
`-D R11_NO_ORACLE`. App resources/runtime/control/TMP stay internal; heavy build
scratch and retained evidence stay on the verified mounted external task volume.
Never reuse another task's instance, change a donor signature or increase quotas.

From the repository, with a new case name (existing evidence never overwrites):

```sh
python3 -B experiments/native/reels-preview/run_fixture.py TASK_JSON CASE 1
python3 -B experiments/native/reels-preview/check_capture.py CASE_DIRECTORY --bundle BUNDLE
python3 -B tests/bridges/swiftui_reels_preview_check.py
```

Phases: 0 loading; 1 mixed20+placeholder; 2 wider/displaced grid; 3 two owners
(refuse); 4 unsupported scale1.25 (unknown window transforms). `--no-oracle`
uses the separately compiled app. Each case uses a fresh process generation and
one attach, then requests normal fixture exit only after confirmed restored detach.
An uncertain capture keeps its debugger; its bound owner recovery is separate.

## Verified outcome and limits

Evidence: `/Volumes/Develop-2TB/Develop/tmp/uib-r11-preview-v68w3fai/evidence/`.
Loading, mixed and resized/displaced each supplied21 window-mapped cells, stable
before/after oracle, canonical validation and42 Rust measurements. Max anchor
difference:0.357142857pt at2× (not exact equality). Without cell anchors, all21
window rectangles matched the mixed case exactly. Scale left all21 transforms
unknown; two loading owners refused `cache_scope_ambiguous` without Snapshot.
Reels independently recalculated all four corners and confirmed the three
positive sets, no-oracle equality and both negative cases from original saved
data; that does not qualify collection permissions/lifecycle or product geometry.

The initial full mixed traversal refused `tree_limit`; no quota was raised.
One repeated attach to the old fixture entered retained recovery with0calls.
One bound owner command terminated only that fixture; exit was confirmed.
Fresh-process cases subsequently restored/detached and exited normally. Repeated
attach reliability and product acquisition remain unqualified. No donor was
contacted. Initial positive fixture source and per-file hashes are retained in
evidence; the final source additionally makes the ambiguity control use two
loading grids, keeping its full discovery under the unchanged scan limit.
