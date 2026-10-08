# B03-G — bounded popup clipping and hit facts

2026-10-08. Restore / shipping_product. Authority: direct autonomous B03-G dispatch,
[packet](../packets/B03-popup-geometry.md), approved PLAN.UIB@1 at
358c757e7eab84a3989d150dbad57924d866601a and
[autonomous task contract](../packets/autonomous-tasks-2026-10-08.md).
Immediate implementation/test/commit/push was authorized; no subagents or other
chats were created. Work stayed on the operator's master; no worktree/branch.

## Traversal and final Spec Basis

AGENTS → specs/README registry28 → product/README → GEOMETRY/PROJECTIONS/IDENTITY/
PRIVACY/MODEL/BOUNDARIES/EXCHANGE; acceptance/README → WEB-PILOTS/PILOTS; decisions/
README → D01–D07 and their explicit closure. Fully read CONTENT@1: BOUNDARIES,
MODEL, IDENTITY, GEOMETRY, PROJECTIONS, PRIVACY, FORMS, ACTIONS, LIFECYCLE, CACHE,
ROADMAP, RUST-BOUNDARIES, PILOTS, WEB-PILOTS, PERFORMANCE, GOLDEN, REUSE;
EXCHANGE@2 (including COMPOSITION), NATIVE@2 as shared dependency only;
D01@1/D02@2/D03@3/D04@1/D05@4/MEMORY@2/WORK@1/D06@1/D07@5/EVIDENCE@1,
including D02 WORKER/PUBLICATION/LIFECYCLE and MEMORY/WORK clauses. RUST.md,
DEV.RUST@2, global implementation/product-truth/QA/operational rules. Public saved
consumers additionally use CLI@15 CONTENT/INSPECT/OBSERVE, ANALYSIS@2 and
ANALYSIS-TYPES/VALIDATION@1. Old @1 links resolve to current EXCHANGE/NATIVE.
No B03 semantic revision drift; concurrently edited export routes are excluded.
Excluded: Native implementation/acquisition, actions, cache changes, export,
mobile, real PlayPhrase.me projects, future phases and full P7/performance claims.
Supporting evidence: R01 source ledger, current collector/normalizer/peer tests,
W01 rooted/popup and W03 resync receipts, platform advice and canonical DTOs.
These sources establish implementation/evidence, not new product requirements.

Required: sourced popup geometry/relations, clipping and available hit facts with
honest partial coverage, exact scope and no read-only focus/scroll/layout changes.
Observed defect: requested HitRegion/VisibleRegion were permanently unknown even
where native APIs could supply narrower facts. Existing Geometry/ExtensionProperty
can express those facts without changing wire/schema or inventing a second graph.
Chosen technical representation: exact single-point hit and separately named native
intersection facts; full visibility/paint remain unknown. No contract delta.

## Plan and exact write set

Before edits the chat declared source reads/normalization, focused tests, own
headless public path, documentation and scoped commit+push. Final necessary files:
`plugins/web/src/collector/{read-node.js,wire.rs,acquire.rs,action.rs}`;
`plugins/web/src/normalize/mod.rs`; `plugins/web/tests/collector.rs` and
`plugins/web/tests/fixtures/collector/script-check.cjs`;
`tests/bridges/web/{popup-geometry.cjs,guarded-live.cjs}`;
`fixtures/web/popup-geometry.html`; `docs/development/web-collector.md`; this receipt.
`action.rs` only initializes new private DTO fields to None; the old guarded test
now expects a point when the native hit sample matches. Shared engine/schema/CLI/
worker_ops/manifests and unrelated working-tree changes remain untouched.

API source record: [CSSOM View](https://drafts.csswg.org/cssom-view/#dom-document-elementfrompoint)
(elementFromPoint/client rectangles) and
[Intersection Observer](https://www.w3.org/TR/intersection-observer/), initial
entry/lifetime/intersection algorithm, read 2026-10-08. R01 pinned source-record
identity and no-action-trial decisions reused. No upstream implementation copied,
new dependency, general-purpose geometry engine or browser settings added.
The single observer is request-owned: zero margin, one target/result, bounded by
remaining time, disconnected before successful response. Timeout does no further
collection; prior hit evidence survives. Source/current/entry rectangle or viewport
changes refuse rather than publish mixed mapping. Existing remote-cleanup unknown
remains truthful after cancellation/disconnection or a suspended browser.

## Verification and actual capability

- `cargo test --locked -p uiblueprint-web --test collector`: **75 passed**.
  New cases cover typed partial facts, field gating, malformed numbers/shape and
  changed async geometry refusing publication; prior privacy/identity tests pass.
- `node plugins/web/tests/fixtures/collector/script-check.cjs`: passed, including
  one-shot disconnect/timeout/API refusal/late callback and no new reads on timeout.
- Web Clippy all targets with `-D warnings`, exact-file rustfmt checks, Node syntax
  checks, changed local Markdown links and `git diff --check`: passed.
- Web CLI + host/session-worker `cargo build --locked ... --features web --bins`:
  passed. Own CARGO_TARGET_DIR was system temp; no SDK/dependency changes.
- `popup-geometry.cjs --run-authorized`: **13 author-run checks passed** on Node
  24.15.0 / Playwright Core1.58.2 / Chromium145.0.7632.6, viewport800×600 DPR1.
  Public Observe open bounds=[120,120,180,80]; Inspect JSON preserves Snapshot;
  Rust Measure width=180css_px. Clip intersection=[120,120,80,80]; full clip has
  known empty rect, ratio0 and false; overlay preserves180×80 intersection but
  changes exact center hit to false. Positive sample=[210,160,0,0], never full area.
  Explicit selected trigger/popup preserve Controls/AnchoredTo and document Surface.
  Closed layout unknown; old remount identity refuses; explicit new identity works.
  Selected iframe and visit cap refuse2; wrong document/stale root refuse4, without
  a successful Snapshot or foreign-frame content. Positive Observe exits4 with
  honest partial coverage; Inspect/Measure exit0. All11 Observe invariance checks
  preserve focus/scroll/DOM/layout; saved original bytes remain unchanged.
- Existing guarded `popup_relations` live scenario passed: one committed channel,
  no missing channel, effect not_dispatched; DOM/AX/relations and browser survival.
  Driver/context/browser/server/profile/worker cleanup all confirmed.

Final runtime binaries SHA256: CLI
8ba29aa670c0db558304241d8de66b459331341b243d8f0ddb052c1ee624825e;
worker bf710a987a44f659b3a26054320e34a54f853884ba09bbb0b53da81296cf8672.
Runtime used the shared current checkout including unrelated concurrent CLI work;
this receipt qualifies only B03's unchanged public consumer interface. Temp JSON/
report files were consumed and removed with absence verified. No task images made.

Full hit area, paint/occlusion, arbitrary-site/frame/OOPIF/shadow support and D06
performance qualification remain outside this bounded capability. IO adds waiting
cost and browser-native work is opaque. Independent acceptance and integrated P7
remain separate; these are author checks with independently authored literal
expectations, not independent review. No implementation blocker remains in B03-G.
