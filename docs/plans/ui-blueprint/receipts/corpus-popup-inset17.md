# CP17 — existing no-input popup inset selection

Finite Restore, 2026-10-13. Source `8f18a1e99e91fc889f1d185ec5089c16e60b570d`, pushed to master.
Authority: root CP17 dispatch in turn `01a133c0-04c3-7cc2-bd2f-183938dafb83`
under the user-authorized [CORPUS plan](../corpus-testing.md); same-turn additional
CW-B17-G witness is model-free only. No semantic contract delta or focus refinement.
Owned: `crates/export/src/compile.rs`, its existing `tests/compiler.rs`, this receipt.
No source app, live capture, collector, engine, schema, CLI, dependency or registry edits.

## Basis and diagnosis

Traversal: [registry41](../../../specs/README.md) → product route → EXPORT2 →
DRAWING-PACKAGE3/STYLE2/GEOMETRY2/PROMPT-A2/B2/REVIEW2/example1;
[HBP1.2 R2/R3](../../../engineering-blueprint-guide.md#hbp-human-001--клиентская-подача-2026-10-10), C2/C3;
SHEET-PROMPTS1 OWNERSHIP/BOUNDS, CLI-EXPORT3 INPUT/BOUNDS;
MODEL/EXCHANGE2/GEOMETRY/PROJECTIONS/IDENTITY/BOUNDARIES/PRIVACY,
ANALYSIS2/TYPES1/VALIDATION1, RUST/DEV.RUST2. Same epoch, no contract drift from CP16.
Protected: exact source facts, all old measurements/selections, anchors/full Space,
units/evidence, privacy/availability/focus attribution, clipping and output refusal.
Excluded: live Native/Web/actions/compare/new platforms and source-product design.

CT2 identified an existing useful known inset omitted from CW-B16-D's literal plan.
Observed cause: positive Measured left inset selection admitted only an input-group
container to its input. The measured Genres dialog has no such input group.
Contract requires a useful known inset, not a new inferred padding or capture frame.
Equivalent implementation choice: reuse the first existing positive same-frame
inset from a principal measured dialog to a reported direct child, only when no
input group owns the dialog. Existing input-group selection retains priority.
Existing deduplication keeps one inset per parent; arithmetic is unchanged.

## Exact replay

Accepted baseline source `fdaca3e34d0a856727f1589f38e01a922075739d` (CP16).
Baseline binary SHA-256 `adf52388cdcb1a634e2f5a25f68742d65e7a995798faa53c8398c4db5c8eec89`.
Candidate binary SHA-256 `896e15a8ffadd6d95830518bb748491e98d264596ac64b1eeb88dfc2a257e5e3`.
Retained root: `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CP17-90f_l0xz`.
[Neutral index](/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CP17-90f_l0xz/neutral-index.json)
SHA-256 `32bb11156bc54d80db0ffd47e2dfbbf3a34ec98b5b6f2780b04d55b699473575`.
It records exact commands/input/reference/package hashes, source blobs and binary.

| Case | Candidate vs CP16 | Existing inset selected | Prompt characters |
| --- | --- | --- | --- |
| CW-B16-D | prompt.txt only | M0245 N016.left → N017.left, 8.0234375 CSS px | 16354 → 16766 |
| CW-B13-D | entire package byte-identical | existing input inset unchanged | 16011 → 16011 |
| CW-B17-G | prompt.txt only; no image | M0229 N016.left → N017.left, 8.0234375 CSS px | 18337 → 18749 |

Both changed prompts add exactly the existing header anchor-context line and four
inset assignment lines; no old prompt line is removed. All machine files remain
byte-identical to CP16. No source facts, clip instructions or sheet ownership change.
Older producer → CP16 deltas are separate: B16 prompt only; B13 dimensions/brief/prompt;
B17 none. They are not CP17 changes. Original inputs/reference hashes reverified.
B16 native PNG reference390×538 at DPR2 is a capture union, not a new source frame;
B13 native PNG2560×1440 at DPR2 remains the qualified original. No recapture or crop.

## Verification and images

Passed with task-owned CARGO_TARGET_DIR: targeted `compiler` tests
`no_input_dialog` and `editable_combo_options`; export all-target Clippy with
`-D warnings`; locked CLI build; `cargo fmt --all -- --check`; `git diff --check`.
Regression covers one existing direct-child inset, native dialog role, unavailable
and incompatible Space geometry, non-dialog and missing-child refusal, prior input
priority, unchanged input bytes, exact aggregate byte limit and one-byte-under failure.
No full suite or repeated unchanged tests. No installed distribution was changed.

Exactly two built-in ImageGen first calls used literal candidate G01 prompts and
original qualified references. No wrappers, retouch, rerolls or generated references.
ImageIO/sips confirms both PNG1536×1024; all originals and temp copies retained.
- CW-B16-D SHA-256 `7605ce2178373ad195271e80588b7b499b68b77cfba7c41322fc502879d63a48`.
- CW-B13-D SHA-256 `f0773c2f675327d4ced5cef7737726ff5f7da11995e784ff8605c9e5aaa0ab23`.

Neutral index was published before [separate author assessment](/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CP17-90f_l0xz/author-review.json).
B16 now shows ≈8 CSS px between named dialog/container left boundaries. Full image
FAIL: Action selection cue missing, internal numbers printed, ≈5 duplicated,
trigger/popup width relationship distorted and overview Comedy focus tag absent.
B13 keeps its inset and content but full image FAIL: unassigned second-row sizes,
duplicate gap measurements, trigger width below rather than above and missing
main-view focus tag. Its unchanged prompt gives no evidence of CP17 code causality.
No general renderer-reliability or full visual acceptance claim. Existing CV2/CT2
own independent acceptance. Count: zero new corpus cases, two repair images.

Selected packages/binary/index/review remain for root and continuing critics.
Only own image-free build target and redundant scratch input index were removed;
no image or image-containing directory was deleted. Tool-managed originals also remain.
