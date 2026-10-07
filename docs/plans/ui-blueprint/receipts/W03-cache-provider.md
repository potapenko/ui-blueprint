# W03 bounded retained-session invalidation provider

Authority: Core section of [shared W03 packet](../packets/W03-session-invalidation.md),
approved P4/W03 under PLAN.UIB@1. Diff5476c2836748d156719a5d8c97155140a685b466 was
saved/pushed first; this is the next assigned finite provider, not resumed diff work.
Basis: existing CACHE/LIFECYCLE/IDENTITY/EXCHANGE/PRIVACY/PROJECTIONS and D02/D04/
D05 MEMORY/WORK explicit closure, RUST/DEV.RUST. CLI@3 does not change cache meaning.
Restore missing session-wide invalidation; no product/wire/cap or storage delta.

Exact5 paths: crates/engine/src/cache/store.rs, crates/engine/tests/cache.rs,
crates/host/src/worker_ops.rs, docs/development/cache.md and this receipt.
Web owns pending-signal/queue/callsite source separately. No schema/Cargo/CLI/
Native/fixture/registry edits, new index/pool/cache or directory.

## Actual API

`CacheStore::invalidate_session(SessionHandle)->Result<usize,CacheError>` first
checks store/ledger/session generation through existing session(). It marks every
occupied retained entry in that session's existing slots. Count includes previously
marked entries, like context-only invalidate; zero means empty session. No allocation,
Snapshot clone, timestamp/expiration refresh, eviction/deletion or charge release.
Foreign/retired handles reject before mutation; other sessions are not scanned.
Existing `invalidate(session, context)` remains unchanged and selective.

Guarded Web hook: `CanonicalSession::invalidate_retained_session(&mut self)
->Result<usize,HostError>`, cfg web, calls the same Store method using its own
cache_session. Store refusal maps to InvalidState, without payload. Call only
after ObservationRun borrow ends; adapter pending event is acknowledged only after
success. Provider never reads CDP, applies events as data or initiates observation.

## Focused author evidence

Rust1.96/aarch64-apple-darwin, Cargo --locked --offline:

- `cargo check -p uiblueprint-host --features web --bin session-worker` compiled
  actual provider/hook. At this intermediate handoff the hook has an explained
  unused-method warning until Web connected its owning callsite; no suppression or
  pretend use added. With saved Web24ed0e8 hookup, final `cargo clippy --locked
  --offline -p uiblueprint-host --features web --bin session-worker -- -D warnings`
  passed, confirming the integrated worker is warning-clean.
- `cargo test -p uiblueprint-engine --test cache
  session_invalidation_marks_all_contexts_without_touching_history_or_other_sessions
  -- --exact`: passed. Two scopes in one session invalidated; second session
  untouched; recorded canonical bytes/times/expiry unchanged; usage and ledger
  reservations unchanged; CurrentRequired still RevalidationRequired. Existing
  context-only invalidation remains selective. Repeated call, foreign/retired
  handle refusal and empty-session count checked.
- Existing exact clock_expiry_and_invalidation_do_not_relabel_source_freshness:
  passed, protecting expiration/clock/history behavior.
- Scoped engine Clippy --lib --test cache -- -D warnings plus formatting/diff/local
  links passed. No full suites/runtime/browser/SDK/source UI.

Source work recovered after host-reported model capacity interruption; task-scoped
model override came from root/user-informed host coordination, not application
default changes or extra agents. Existing WIP preserved; prior diff checks not repeated.
No temporary output or persistent directories created. Same-session historical
data remains accessible only with its stored disposition; no false current values.

Web saved its pending-signal/callsite hookup in24ed0e8; source compile is verified.
Integrated runtime proof remains Web-owned: pending signal across success/error
paths, ACK only after success, and detach queue ownership release. This provider
alone does not close live reuse/resync, continuous
event detection, K02 or performance gates. Saved checkpoint/push returned separately.

SHA256: store.rs4f6e1abc8092f0b518e22848bbd1292c0dc7805adbf395ab932a4490db9995c9;
cache test b6189ffa6708b8163a86cf5f9d7de73d60a8b2b71b6d69f5b3032b4c67fa29c1;
worker_ops.rsc024f1e768269ea4bea571d40a21be7c0d1109cca71291541afd28fd28ef43c3.
