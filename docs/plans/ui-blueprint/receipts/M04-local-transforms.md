# M04-T — sourced Native local transforms

Authority: immediate full-cycle M04-T assignment in
[autonomous tasks](../packets/autonomous-tasks-2026-10-08.md), approved PLAN.UIB@1.
Single chat, no delegation, current master; starting checkout f650b9c.
Mode Restore; shipping_product; no product/specification delta or full P7 claim.

Traversal: AGENTS → specs/README registry26 → product/README → NATIVE@2,
GEOMETRY@1, ANALYSIS@2/TYPES/VALIDATION@1, CLI@13/CLI-DIFF@2/
CLI-GEOMETRY-DIFF@1; acceptance/README → NATIVE-PILOTS/PILOTS@1;
development/decisions/README → Native acquisition@2/D02@2/D04@1/D05@4/D06@1.
Explicit closure read: BOUNDARIES/MODEL/IDENTITY/PROJECTIONS/LIFECYCLE/PRIVACY/
FORMS/CACHE/ACTIONS/ROADMAP/RUST-BOUNDARIES/PERFORMANCE/GOLDEN/REUSE@1,
EXCHANGE@2, D01@1/D03@3/MEMORY@2/WORK@1/D07@5/EVIDENCE@1, RUST/DEV.RUST@2.
CONTENT clauses plus selected analysis/binding/geometry-diff clauses govern.
Historical @1 links to EXCHANGE/NATIVE and old CLI links resolve to registered
current versions; no semantic conflict. Export/mobile/Web/action implementation
and coordination routes excluded. Supporting inputs: P01 receipt M04 source/live
sections, G02 receipt G12, Native helper/acquisition docs and current source.

Requirement: explicit sourced transform, no guessed screen origin/global scale,
immutable source/evidence, move separated from local layout change; missing mapping
unknown. Observed implementation: raw fixture-local anchors and unknown transform;
existing scroll/resize/move controls, Rust selected-space measure/diff consumers.
Technical selection under ROADMAP: define scroll-local at the actual measured
viewport upper-left, both pt/top_left. Existing public GeometryProxy anchor
subscript resolves viewport and row in the same local space; authored fixture has
no intervening rotation/scale. Encode the derived translation, never a screen or
pixel mapping. No new reader, UI state, modifier, framework or analytical engine.
Public API basis: [GeometryProxy](https://developer.apple.com/documentation/swiftui/geometryproxy)
and installed SDK27 SwiftUICore interface; bounds(of:) inspected as an alternative,
but a second reader/collection is unnecessary. No third-party source copied.

Task-owned writes: fixtures/native/Fixture.swift measured manifest only;
tests/bridges/native/Collector.swift probe conversion; acquisition/ProbeChecks.swift;
focused Native CLI check in tests/bridges/native; docs/development/native-helper.md;
this receipt. Additional test path is in the existing Native directory. Protected:
shared schema/engine/CLI/worker_ops/manifests, AX/layout/hit/visible/pixel distinctions,
P01 invariance seam, ordinary AX and sample/composed probe outputs, other tasks.

Plan: encode Snapshot-bound measured viewport mapping; validate before canonical
Transform construction; focused synthetic known/missing/stale/environment cases
through real Rust validator/measure/diff; one owned F02 move/scroll/resize run;
document actual results, checkpoint+push under shared nonblocking Git flock.
Tests use independent numeric literals. No renewed probe invariance run; no
cross-display claim without a suitable existing environment. Independent review
remains an acceptance gap and is not replaced by self-review.

## Source and focused checks

Changed only the six planned paths. Swift6/SDK27/macOS14 compile target: fixture-on,
production HostHelper and ProbeChecks compile cleanly. Source changed no visible
layout, focus/hit/AX modifier or source-state transition; no AppKit adapter added.
37 synthetic cases/74 assertions,35 emitted documents accepted by Rust validator.
Existing sample/composed-source checks retained. New independent literals resolve
viewport20/30 and row26/36 to local0/0 and6/6; synthetic translation invariance,
scroll dy−70 and resize dwidth100 pass public Rust diff. Inside measure6pt contains
both reported anchor and derived transform Evidence. Missing mapping, stale request,
environment mismatch, altered viewport and invalid scale preserve unknown; public
CLI rejects unknown destination with2/unknown_space and empty stdout. No fake
evaluation/transform is supplied to bypass destination discovery.

Command: `python3 tests/bridges/native/local_transforms.py --checks CHECKER
--validator VALIDATOR --cli CLI` (PYTHONDONTWRITEBYTECODE=1). All source inputs
remain byte-identical after consumption. Test-owned JSON directories are removed.
Test authoring corrected `axis=both` to canonical `xy` and Snapshot's `id` field;
then preserved actual unknown_space behavior instead of assuming EvaluationInput
alone registers a missing destination. No product contract/expected numeric result
was relaxed. Initial workspace Rust build included concurrently changing sources;
excluded it from acceptance, rebuilt CLI/worker/validator from immutable f650b9c
archive with locked/offline dependencies and reran the focused driver successfully.

## Actual production path — 2026-10-08

One owned F02-on process62030, A window13066; full public launch identity from
LaunchServices and exact own temp executable. CUA selected Window A by f02.owner.a;
explicit Snapshot → Move → Snapshot → Scroll end → Snapshot → Resize → Snapshot.
Each read used public Observe/native_fixture/channel4, scopef02.scroll.a/design/
layout_bounds, cached_allowed,160/depth9/512KiB/deadline1s, parent cleanup1s.
No AX/capture permission request, pixel collection, other user app or display change.
Observe returned4 because coverage is partial, with an actual observed payload.
All four original responses validate0. Public Rust Measure/geometry Diff returned0;
full source Snapshot equality checked in every result. Original cache/unverified,
fixture clock, environment revision and derived/reported Evidence remain intact.

| State | Row in scroll-local x/y/w/h (pt) | Minimum inset (pt) | Transition |
| --- | --- | ---: | --- |
| baseline |6/6/498/16|6|viewport510×90 |
| Move |6/6/498/16|6|window781/293→821/273, local diff all0 |
| Scroll end |6/−784/498/16|−784|row dy−790, viewport unchanged |
| Resize |6/−784/598/16|−784|window width550→650, row dwidth100; dx/dy/dheight0 versus scrolled |

Measured viewport origin20/301pt; source row y307→−483pt. SwiftUI display_scale2
recorded, never treated as screen/pixel transform. Four one-call Observe walls
175.05/218.19/113.24/135.41ms are observations, not p95 or D06 acceptance.
No visibility/paint/clipping or CSS-padding claim follows from these rect insets.
Only API-sourced local translation is delivered: screen↔local, AX↔probe, pixel
mapping, cross-display qualification and general SDK remain separate open work.
No suitable cross-display run was performed or display setup altered. Full M04/P7
and independent acceptance are not claimed.

Original response SHA256 (accepted consumer results, not persistent raw evidence):
baseline e85d28615b62aac164af68beca1e391dbca686ba4a9c87aa5810b9c3d1c475dd;
moved0f906b523d6c487ded60436938f4224f9a06841f82a2a69a232759964e74a1e9;
scrolledf70a86fc356ccdd33f6133f206699e533af032bd36be470858587b4af0e03144;
resized78552ba640a1d1cdea2aff20813461f8eb645375bfd0878cbf2ae81207e5ecee.
Owned fixture terminated by exact PID/executable verification after consumers;
helper/worker absent after final Observe. No images created. Cleanup and checkpoint
verification recorded below after execution; temporary files are not an archive.

Final verification: changed local Markdown links and scoped git diff --check pass.
Native compiled source hashes: Fixture.swift da96d5a07b1abe5abcae9a675910e0b93a4d6615a369483eb58713b2c19c3f63;
Collector.swift78025d90aa499e721012c1afad53ed7fd139b2aeb539069b152d08cafc97b0eb;
ProbeChecks.swift369e987c6ce8bb740aa29f37f14bbe61d014675870b358711008ceabd95decfe.
Concurrent CLI@14 registration only adds graph-diff routing and preserves this
selected geometry/analysis closure; inspected delta, no Native revision drift.
Exact own fixture/helper/worker absence verified. Task-temp build/source/input/
result nonimages removed and directory absence verified; images0, unrelated image
after-title-spacing.png untouched. Six own paths checkpointed under the shared
Git lock; final SHA/push outcome is reported by this chat after successful save.
