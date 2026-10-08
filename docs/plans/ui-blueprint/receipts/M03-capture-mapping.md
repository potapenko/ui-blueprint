# M03-C — popup capture mapping

Assigned finite shipping_product task, immediate full cycle authorized by the
calling user message and [packet](../packets/M03-capture-mapping.md). Single chat,
no delegation; current master only. Restore existing requirements; no spec delta.
Starting fixed Rust source revision is recorded below with final results.

Traversal: AGENTS → specs/README registry27 → product/README → NATIVE@2.CONTENT,
GEOMETRY@1, EXCHANGE@2, MODEL/IDENTITY/PROJECTIONS/LIFECYCLE/PRIVACY@1;
acceptance/README → NATIVE-PILOTS/PILOTS/FORMS/CACHE/ACTIONS@1 closure;
development/decisions/README → D01@1/D02@2/D03@3/D04@1/D05@4/MEMORY@2/WORK@1/
Native acquisition@2/D06@1/D07@5/EVIDENCE@1 and explicit ROADMAP/RUST-BOUNDARIES/
PERFORMANCE/GOLDEN/REUSE/BOUNDARIES@1. Analysis consumer adds ANALYSIS@2,
TYPES/VALIDATION@1 and CLI@14 CONTENT/OBSERVE. CONTENT clauses and Native
acquisition METRICS/CEILINGS/ADMISSION/OWNERSHIP/OUTCOMES/PROOF are preserved.
RUST/DEV.RUST@2; implementation/product-truth/QA/Computer Use/Apple/operational
routes read. Historical @1 links resolve to current registered Native/Exchange;
no material spec drift. Other CLI modes/export/Web/mobile/actions and orchestration
excluded. Supporting resources: current Native source, installed SDK27 headers,
M03-popup-attribution final actual interval and M04-local-transforms receipt.
Packet's M03-popup-connector receipt path does not exist; historical source/live
attribution receipt is the available input, not invented acceptance.

Requirement: sourced mapping and separate surfaces; unknown never grants exact
coordinate authority; geometry/identity changes invalidate applicability.
Observed implementation always emitted unknown crop_transform despite collected
SCContentFilter metadata. Proposal selected under ROADMAP: qualify only full-window
natural-resolution no-shadow mapping, with public CG window/display metadata before
and after capture and before popup publication. Unknown/unsupported cases remain
unknown. No guessed titlebar or scale computed from PNG size, no new framework.
API source record: installed SDK27 SCStream.h SCContentFilter contentRect /
pointPixelScale and SCStreamConfiguration destinationRect/scalesToFit/
preservesAspectRatio/ignoreShadowsSingleWindow; SCShareableContent.h SCWindow.frame;
SCScreenshotManager.h captureImage. [Apple filter API](https://developer.apple.com/documentation/screencapturekit/sccontentfilter)
describes screen-point content rect and point-to-pixel scale. Own implementation;
no third-party code copied or dependency/license change. SCK is existing platform
framework; synthetic literals and real own popup are the qualification consumers.

Plan/write set: fixtures/native/Observe.swift; tests/bridges/native/Collector.swift;
tests/bridges/native/acquisition/PopupChecks.swift; focused Native consumer check;
docs/development/native-helper.md and native-acquisition.md; this receipt.
Public core/schema/engine/CLI/manifests/worker_ops and unrelated WIP protected.
Implement metadata-bound transform; focused synthetic boundaries/validator/Rust
consumer; one bounded owned F02 popup chain with move/resize/stale/reopen as relevant;
retain images, clean own nonimages/processes, exact-path commit+push under flock.
No renewed probe invariance/D06 performance campaign or full M03/P7 claim.
Independent review is a separate acceptance gap; these are author checks.

## Implementation and verification

Swift helper/descriptor and unchanged fixture compiled on Swift6/SDK27 with macOS14
compile target. Fixed-source Rust CLI(macos)/worker/validator build passed locked/
offline. First compiler error was a conditional CFDictionary bridge; corrected to
a typed dictionary bridge, no API/contract change. Popup synthetic suite91 checks,
18 canonical documents validated, before real SDK execution. Images remain in
system temp. Final runtime and cleanup are recorded below.


## Delivered capability and actual own-fixture evidence

Known crop_transform is now produced by the shipping Native helper, not just a
harness. CaptureMapping verifies API metadata, finite coefficients, unchanged
addressed geometry/display context and exact natural dimensions. Capture spaces
are unique per Observation; source AX rectangles and units remain intact.
Existing profile/identity/permissions/pixel policy and no-input Observe remain.
The caller gets separate AX/capture responses with explicit partial coverage;
parent excluded, popup included, unresolved known pair empty. This is not global
surface completeness or visibility/hit/occlusion evidence.

Actual2026-10-08 arm64/macOS27.0.1/SDK27.0, Swift6/macOS14 compile target.
Immutable Rust source ee752f9ff6fb2ad5aa4490059e5f3f8102d7ec3c contains ready
G13/M04-T inputs, built locked/offline CLI(macos), worker, validator. Shared WIP
was excluded by Git archive; only owned Native source overlays changed.
Fixture source unchanged throughout, no probe invariance rerun.

Primary own launch PID95890, launch1791461172.3526921, parent13180, popup13186:
CUA Edge popup → Snapshot → Observe; Move → Snapshot → Observe; Resize → Snapshot
→ Observe; Cancel → old Observe; reopen → old Observe; Snapshot → new Observe.
Actual parent control resize moves the popup anchor; popup intrinsic size remains
181×114pt. Different popup dimensions/in-flight resize are synthetic boundary
checks, not a claim of an actual resizable-popup gesture or cross-display run.

| Actual state | Popup origin pt | Crop affine | Result |
| --- | --- | --- | --- |
| baseline |1311,379|[2,0,0,2,-2622,-758]|AX5 + PNG362×228, known |
| moved |1351,399|[2,0,0,2,-2702,-798]|AX5 + PNG362×228, known |
| parent resized |1451,399|[2,0,0,2,-2902,-798]|AX5 + PNG362×228, known |
| closed / reopened with old identity |—|none|both channels stale_target, no image payload |
| reopened current13204 |1451,399|[2,0,0,2,-2902,-798]|new generation, AX5 + known PNG |

For each positive pair, every pre/post identity key equals the trusted expected
binding, state OPEN, including process launch/Target generation and both Surface
generations. Primary popup generation4C8B5A5D-50EA-4602-B1DB-1E1F12825908 → reopened
2ED41029-148F-4B9C-AA49-263EE7BB797A. The public helper additionally revalidates
actual owner/process/window/display metadata. No old frame is repaired/relabelled.
The four positive request walls455.74/825.66/617.37/752.10ms use unchanged1s parent
request deadline/1s cleanup and160/depth9/512KiB per channel. These are individual
observations, not warm/cold p95 or D06 acceptance. Initial cold call wall1062.46ms
returned preserved AX only and no committed capture; exact platform stage cause
was not established. It remains an incomplete request, not a censored success.

Focused final source candidate runtime: helper SHA256
b53dc83892a072fd7c66f6ad031a889d1d95f5117e5d85ac6a5e36acf5e435d2,
owned fixture PID4555/launch1791461742.0798712, parent13224, popup13242/generation
1B5D313E-8198-4F2C-A1DC-D0189D39ABAD. Actual AX5/PNG362×228 known, wall441.85ms;
pre/post expected current identity all match. CUA before/after tree unchanged with
Confirm focus. An earlier setup in this same launch had actually closed its popup
before Observe; current identity file CLOSED and refreshed CUA agreed, both channels
refused stale_target. No alternate backend or permission change; a fresh explicit
open/Snapshot established the positive precondition. Closure cause unknown.

After this runtime candidate, final source adds only defensive missing-context
refusal instead of force unwrap and preserves the old unknown reason string;
the affected valid mapping path is unchanged. Recompiled helper/descriptor and
actual PopupChecks on final source;96 checks,19 canonical documents validator0.
Checks include negative screen origins, known moved/resized metadata, size/scale/
rounding/overflow/missing-context refusal, changed geometry/display, retired identity,
publication race, permission/cancel/timeout, response bounds and retained PNGs.
No broad logic suite or another unchanged UI cycle was needed for that guard.

`capture_mapping.py` ran on all four primary pairs and the final candidate pair:
public Rust Measure produced popup362×228px, Confirm230×48px and insets
left66/top114/right66/bottom66px. Contributing transform Evidence is present.
Each run also proves missing transform/wrong parent Surface → missing_transform,
wrong environment/retired generation → invalid input. Original source bytes,
AX nodes/properties, clocks and all Observations remain unchanged. Canonical
standalone composition is explicit in this qualification consumer; it is not a
new shipping CLI join command, synchronized Observe response or live pointer proof.
AX/capture helper clock domains remain separate; consistency stays unknown.

Final source/binary SHA256:

- `fixtures/native/Observe.swift`: `ed035f97dbdac838486f5d8aabd2d7e52c8d097d4bfb588581e49c541df8e293`.
- `tests/bridges/native/Collector.swift`: `f8b96f5fae884d7780498ba5d39173c9b995bc6bf19540450bce8f537326ccd9`.
- `tests/bridges/native/acquisition/PopupChecks.swift`: `86d92d41eac0065b2935f1e7dd668557baa8f7e37c3ac5a71e74579381c7fb99`.
- `tests/bridges/native/capture_mapping.py`: `bdb90ac8dadc2f20574ae812ddfe83bd1692a0548d1965c6b1b02acf5cfa8b49`.
- `native-host-helper`: `3c2a7ba91e8fd8ae132300d943f2e95ec5d95e921b496aa3fb103f9bad24dc53`.
- `target/debug/uiblueprint`: `7f62639a45916dffb3b1cd7eb435452001c5dd1ab0bf81a5aef7d26a06750ee8`.
- `target/debug/session-worker`: `8280d5d7c2b31dd7083772d251bf6d43f9b5d6ecfcdc28e1c722c2313f35f21c`.
- `F02-off.app/Contents/MacOS/F02Fixture`: `1082e7a8c17309e1a019a21a72ee4fcc576898c123572845028242dac256ca1e`.

## Images, cleanup and limits

Actual final displayed PNG:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-m03c-final_open-images-sctasp0w/capture/capture.png`,
SHA256417d8da3ef8fefc031507a8f3663e04c1989076aca3db15aff8a5936f855522c.
Its same-inode `.native-A687F9E4-3272-42B5-863B-81C22F27942D.partial` also remains.
Earlier primary PNGs and staging originals remain under system-temp roots
uib-m03c-fixed-images-yhb9d72l, uib-m03c-moved-images-6q1lrf26,
uib-m03c-resized-images-nemw4ro8, uib-m03c-reopened-images-3zrh8gr2;
each capture/capture.png SHA256f72758f8d34f53c885edfced16d85a1cdb8c477b765b2e34f2f8185695538b87.
Synthetic image roots also remain: uib-m03c-tests-8aqbwqsf,
uib-m03c-final-tests-g3z8wng4, uib-m03c-boundary-tests-8zpcym_2.
All are under the same system-temp prefix above. Every staging/partial/final image
and containing directory is retained, including empty refused-operation image
containers; no agent image cleanup or lifecycle/deletion-time promise.

Both exact run-owned fixture processes terminated under bounded launchers; no
permission/display changes, no user app termination or real PlayPhrase.me operation.
Independent acceptance still required. Full M03/M04/P7, Native action provider,
arbitrary-app mapping/redaction, mixed-density cross-display qualification and
atomic cross-channel time mapping remain outside this delivered slice. The usable
known case is full-window no-shadow natural resolution with agreeing sourced metadata;
others stay unknown/refuse. Current SDK inventory/codec allocation is opaque.
Final cleanup and exact-path checkpoint follow; SHA/push is returned in chat.

Final cleanup: exact owned fixture/helper/worker absence verified from process
inventory. Build/source/cache/binaries, both live input/output roots, synthetic
JSON/identity files and metadata sidecars removed; non-image directory absence
verified. All28 image/staging paths remain with their containing directories.
No image, unrelated after-title-spacing.png, other task source, shared index or
historical evidence was cleaned. Scoped diff check, local Markdown links and
Python syntax pass. Final selected Native/Geometry/Exchange/acquisition contracts
have no revision changes since fixed source checkout. Seven exact task paths are
the checkpoint write set; no full workspace format/test or spec mutation.
