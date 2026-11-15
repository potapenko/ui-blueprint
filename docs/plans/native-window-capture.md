# Native window capture delivery

Completed locally on2026-11-15; no remote publication performed.
Approved immediate execution: human01a1ddf8-8471-7e91-88d4-dc38e80cf501 and
01a1ddfa-0e60-74d2-b771-de13f9467652 in consumer01a1dcba; owner01a1db1e.
Mode Evolve, [NATIVE-CAPTURE@1](../specs/product/native-capture.md). Current master.
Reused Native/Geometry/Identity/Privacy/CLI, acquisition/image writer and local
release/install routes; screenshot-asset and QA governance applied.

## Delivered

Source9595d7652fd91bb9f0419bd8f9e31fc274297c77 adds capture-window/list-windows;
patch4bd6597d871318dfd48b0aa71c78306d7d168e01 resolves the installed CLI symlink
before locating the matching native helper. Local installed version0.3.1 uses
that exact source, with matching eight-file skill and existing .agents link;
the legacy .codex alias and prior0.2.0 installation remain intact.
No SDK, donor source, UI, permissions, core schema or geometry arithmetic changed.
No remote build, push, tag or release. Version0.3.0 remains an immutable earlier
local candidate with the discovered symlink-entry defect; use0.3.1.

## Verification

- 22 Swift boundary cases and6 helper command negatives passed: argument/identity,
 limits, changed geometry/display, destination collision/symlink and fixed errors.
- 26 existing native PNG writer/ImageIO checks passed, retaining images/partials.
- Default CLI binary tests21 + command tests2 passed. macOS binary tests22 passed;
 final command tests2 include a separate-bin symlink resolving its bundle helper.
- Clippy for all CLI targets/macOS, rustfmt, Python syntax and changed links passed.
- Local candidate inventory/hash and full existing installer update/rollback suite
 passed, including historical manifests1–5.0.3.1 reuses the unchanged installer;
 its package verification and real matching installation passed.
- Consumer01a1dcba reviewed the capture source/dispatch/privacy/binding scope with
 no actionable findings. Tool version wording was corrected: metadata versions
 its format; product source/version comes from the verified bundle manifest.
- Actual SwiftUI reference PID31317/window9313 produced3840×2032 and, after resize,
 1922×2100 lossless PNG at source scale2, with known screen/window transforms.
 Consumer checked original SDK geometry around both captures; donor unchanged.
- Actual GPUI example PID52804/window9396 produced1440×1344 PNG for720×672pt,
 source scale2 and known affine. Existing35-node SDK sample has the same PID/scale,
 content720×640. The titled-window content inset32pt is a separate verified
 consumer association, not an invented SDK field or atomic-frame guarantee.
- The consumer independently ran the ordinary installed0.3.1 entry: list and
 Swift capture passed with fresh3840×2034 PNG for the then-current1017pt window.
- The real installed ~/.local/bin/uiblueprint0.3.1 produced the same GPUI PNG hash
 as the candidate and returned correct list/capture metadata. Active skill bytes
 match committed source and contain the command recipe. Prior no-feature candidate
 and installed0.3.0 symlink failures were diagnosed, not counted as success.

## Ownership and limitations

The consumer owns its Swift process and source-reference artifacts. Our GPUI
process, desktop guard and caffeinate were stopped; other applications untouched.
All task PNG/staging images remain in system temp. Run-owned non-image build/log
artifacts retire after the consumer's acknowledged delivery copies.
Capture reports only the exact window, excluding shadow/cursor/audio/child windows.
It does not establish occlusion, animated GPU atomicity or unknown layout/hit/paint
facts. Metadata and existing SDK JSON may be associated by the consumer without
rewriting source records. Wider platform/pixel bounds remain unqualified.
