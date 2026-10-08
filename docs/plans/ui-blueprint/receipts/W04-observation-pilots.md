# W04 — Web read-only observation pilots

2026-10-08. Restore / shipping_product plus bounded qualification. Authority:
explicit standalone W04 dispatch in this chat, [packet](../packets/W04-observation-pilots.md),
approved PLAN.UIB@1. Dispatch explicitly approves implementation after the local
plan and checkpoint+push, with no additional grants, agents or chats. Single-agent
work on the operator's existing master. No real PlayPhrase.me project operated.
Consumer: Q01 and public component/geometry inspection.

## Traversal and final basis

AGENTS → docs/specs/README registry28 → product/README and acceptance/README →
WEB-PILOTS@1 B01/B04/B06. Full CONTENT@1 closure read: BOUNDARIES, MODEL, IDENTITY,
GEOMETRY, PROJECTIONS, FORMS, PRIVACY, ACTIONS, LIFECYCLE, CACHE, PILOTS, GOLDEN,
PERFORMANCE, ROADMAP, RUST-BOUNDARIES, REUSE; current EXCHANGE@2/NATIVE@2 (Native
as dependency only). Decisions/README → D01@1/D02@2/D03@3/D04@1/D05@4/MEMORY@2/
WORK@1/D06@1/D07@5/EVIDENCE@1, including the complete worker/publication/lifecycle,
memory and guard clauses; RUST.md/DEV.RUST@2. CLI@15 CONTENT/INSPECT/OBSERVE,
CLI-DIFF@2, CLI-GEOMETRY-DIFF@1, CLI-GRAPH-DIFF@1 (read, not exercised), ANALYSIS@2
and TYPES/VALIDATION@1 complete closure. Global implementation, product-truth six
leaves, QA and operational safety read. Old dependency links resolve current
additive EXCHANGE/NATIVE revisions; no Web semantic drift. Concurrent registry29
adds only Native-session routing and was dispositioned outside this packet.
Excluded: Native implementation/acquisition, actions B02, new cache/B05 work,
B03 repetition, export, mobile/future stages and P7/D06 qualification. No spec edit.
Supporting evidence: R01 exact-source/no-copy ledger; F01 fixture/development guide;
W01 rooted-selection, W03 resync and B03 receipts by applicable sections; current
Web reader/wire/normalizer/collector tests, canonical ComponentMapping and public
consumer contracts. This is the selected finite context, not a new upstream audit.

Contract requirement: explicit source identities/mapping, truthful projections,
scoped read-only observation, attributed geometry/transforms, literal values and
honest unavailable/partial states. Observed implementation: DOM↔AX correspondence
already worked, but components was always empty; viewport/document mappings and
ref invalidation already existed. Technical choice under that requirement: report
explicit data-component-key + data-component-parts using the existing selected-ID
resolution mechanism, then canonical Rust normalization. No name/rect/ancestry
inference, schema/feature/constant authority, second graph or JS analytics.

## Local plan and own writes

Before implementation the chat declared the exact collector/normalizer/test,
fixture/harness, guide and receipt plan; the dispatch above is its actual immediate
approval source. Final write set:

- plugins/web/src/collector/{read-node.js,wire.rs,acquire.rs,action.rs}; action.rs
  only initializes the added optional private DTO component field to None.
- plugins/web/src/normalize/{mod.rs,components.rs}; plugins/web/tests/collector.rs;
  plugins/web/tests/fixtures/collector/script-check.cjs.
- fixtures/web/observation-pilots.html; tests/bridges/web/observation-pilots.cjs.
- docs/development/web-collector.md and this receipt.

No shared engine/schema/plugin-api/CLI/worker_ops/host supervisor/Cargo change.
Unrelated concurrent Native changes and after-title-spacing.png are untouched.
No directories created in the repository; all transient source/build data uses
one own system-temp directory. No images created or deleted.

## Delivered capability and actual qualification

Mapping requires every declared part to resolve uniquely within original selected
objects. Private endpoints/owner, incomplete lists and duplicate selected component
keys yield no association; malformed wire indices refuse. Rust preserves owner,
declaration part order, then separately sourced AX counterparts; source declarations
identify the DOM owner. No action ref is derived from component membership.

| Pilot | Expected → actual on owned Chromium | Owner/evidence |
| --- | --- | --- |
| B01 | Same labels in two exact tabs → separate Target/Surface identities and correct names. Current source ref works; original root and source ref after remount refuse4 without output; explicit new root works. Reload invalidates old loader; new binding works. Wrong target refuses before collection. Other tab unchanged. | Existing collector identity + W04 public harness; attach error precision gap below. |
| B04 | 16→24px font → authored 2em×1em box32×16→48×24 css_px, Rust dx0/dy−4/dwidth16/dheight8 (vertical centering). English→French changes AX Apply→Appliquer; London/empty/redacted values preserve meaning. Geometry unchanged across locale. Resize800→1000 moves responsive x660→860; scroll100 moves viewport y120→20 with document displacement0 and sourced affine ty100. | Existing Web geometry + Rust Measure/raw/G12 Diff; new text/locale qualification. Prior b234fff + Core140e53d resize/scroll evidence remains attributed. |
| B06 | One composite button → BUTTON/SPAN/SPAN DOM sources plus separate addressed AX sources, explicit apply-control mapping and reported declaration; design Inspect from AX shows parts, interaction Inspect preserves primary seed; ignored decorative icon role stays unknown. Missing declared parts yield no guessed component. | New Web component normalization; public Observe/Inspect in both projections; independent authored CSS/markup expectations. |

All nonempty observations remain partial/exit4, with unknown DOM semantic fields,
separate AX identity/role/name and original observation attribution. This does not
claim complete foreign-page design. All18 Observe calls check both pages' focus,
scroll, markup, live values, locale/direction, viewport and selected layout unchanged.
The earlier chat update's20 was a counting error; the final harness has18 Observe
checks and29 report entries. Four Rust measurements, four geometry diffs, one raw
locale diff and two Inspect pairs (compact+JSON) passed; original Snapshots/bytes
stay equal through consumers. Independent expectations are fixture literals, not
candidate-generated baselines. Text boxes are not glyph contours; no logical
leading/trailing, breakpoint, DPR-derived transform or action permission is inferred.

## Checks and build isolation

Candidate inputs: immutable git archive2f1dcfd (saved master after dispatch;
contains B03b43d0df, M03-C5fd4b6a, E03c97c513 and Replay85ea656) plus exact own Web
source/test overlays. Runtime builds never consume concurrent Native WIP. Source
and CARGO_TARGET_DIR are under the task's OS-temp directory. No dependency install,
Cargo update, Swift/native invocation or feature expansion.

Actual checks on Rust1.96.0 / aarch64-apple-darwin:

- `cargo test --locked --offline -p uiblueprint-web --test collector`:76 passed,
  including eight new component variants and existing identity/privacy/action/ref,
  partial/empty, viewport, clipping and bounded-publication regressions.
- `cargo clippy --locked --offline -p uiblueprint-web --all-targets -- -D warnings`:
  passed. Web CLI+host `cargo build --locked --offline -p uiblueprint-cli
  -p uiblueprint-host --features web --bins`: passed.
- Existing offline script suite plus nine explicit-component checks: passed.
  Source text is data; unknown/missing/out-of-scope/duplicate/private parts cannot
  cause extra traversal or fabricated membership.
- `observation-pilots.cjs --run-authorized`: final29 report entries passed on
  Node24.15.0/Playwright Core1.58.2/Chromium145.0.7632.6, DPR1, initial800×600.
  Bounds unchanged:32nodes/depth8/64KiB/250ms,256visits; the refusal case lowers
  only its own visit bound to1. Not a statistical performance run.
- Owned Rust formatting, Node syntax, changed local links and diff checks recorded
  before checkpoint. No claim of independent review or broader runtime support.

The first compile exposed one missing field initializer in checkbox_read; adding
component=None fixed it without changing action behavior. First live attempt
stopped on the wrong-target exit assertion; source tracing established the shared
Attach error-publication limitation below. The harness now expects that exact
existing failure only for wrong-target/old-loader Attach, with empty stdout,
read-only invariance and actual process retirement, not generic failure-as-pass.

CLI SHA256:40c73e47a0d15295232070e2f7a7cb0d7bc954a6d845888431fb4d7adf7d7f95.
Worker SHA256:18a9513a7f6b4710c00a052547e902046bf6a509dad681ce80505344a0b76c40.

## Limits, exact remaining dependency and cleanup

Common Attach error publication is outside W04's protected boundary. In pinned
crates/host/src/worker_main.rs, WebSession::attach returns before Ready via `?`;
worker_web already maps StaleTarget to ResyncRequired, but the parent/CLI sees
worker failure1 (`observe_worker_or_cleanup_failure`) instead of a typed stale/
resync refusal. Wrong target and old loader both publish0 and reap cleanly. The
minimum follow-up is shared-host startup failure control/CLI propagation and its
pre-Ready stale/invalid/permission tests, with Native startup protected. Do not
close full B01 error-semantic acceptance until that owner fixes/verifies it.
No extra host protocol/model or local workaround was introduced here.

B04/B06 positive author qualification is available within this exact declared
fixture/profile; independent review/acceptance (including pending B03 review),
integrated P7 and D06 remain separate. Arbitrary frames/OOPIF/shadow, unmarked
framework components, full visibility/hit/glyph geometry are not qualified.
No additional in-scope Web implementation dependency remains.

Each live run closed only its own CDP/context/browser/server and removed its own
non-image JSON temp files. Final harness confirms each CLI worker retired, browser
process exited, browser profile absent and run temp directory absent before success.
The original browser fixtures remain alive through product worker teardown.
The source/build temp tree is consumed after final input/hash checks and removed
only after confirming it contains no images; final cleanup/checkpoint evidence is
appended below. Existing task/other images are never included in cleanup.

## Saved candidate and final cleanup

Implementation checkpoint: d3f472361abc3c46bb3a8aa0f00b01607e651bd9. The final live
run AFTER that checkpoint passed all29 entries, including original unmodified
ChannelResponse byte storage and exact post-consumer byte checks. Eight compiled
Web source/test overlays and the runtime fixture/harness were compared byte-for-byte
with that saved commit; no concurrent shared source entered the candidate.
Eight-file path/hash fingerprint:
368396aee9834c4f3bf152daff4e4c710f2bd40b1e173a4e8912735e4c4f419a.
Fixture SHA256:1dcf3bfc89a24adb2471a1e15d9c27550adaa614d75192d65e35e983aba52d8c.
Harness SHA256:d5073b95fd8fab860bd69bc74b64907b60b3b701fbb385a4f956b59b26443d4d.
Owned rustfmt, Node syntax, both changed-document local links and diff checks passed.
One mistyped formatting invocation named nonexistent read-node.rs; corrected exact
Rust-file invocation passed, with no source change or waived check.

The task-temp source/build inventory contained no image/PDF/SVG files. After
consuming results and verifying no owned session-worker remained, all3700 run-owned
non-image files and empty directories were removed; the exact task directory's
absence was verified. Harness output separately confirms CLI workers reaped,
context closed, browser exited, profile absent, server closed and JSON temp absent.
No screenshot, crop, real application, existing image or unrelated source was touched.
Final receipt checkpoint and canonical master push use the prescribed shared flock;
actual final SHA/push result is returned in the chat, without another agent/message.
