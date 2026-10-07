# Connect explicit-request Web events to retained-session invalidation

Class shipping_product; existing Web/Core owners, no nested agents, master.
User-approved P4/W03; current source handoff identifies exact missing connection.
Use existing CACHE/LIFECYCLE/IDENTITY/EXCHANGE/PRIVACY/PROJECTIONS and current
D02/D04/D05 MEMORY/WORK explicit closure plus RUST/DEV.RUST. CLI@3 additions do
not alter cache meaning. No spec semantic delta or new subscription/monitor.

Known current behavior: bounded CDP queue detects loss, collector refuses identity
loss, CacheStore retains historical records and requires revalidation for current
reads. Explicit Retain stays explicit. Missing: event invalidation does not reach
retained contexts across the same session; detach retains queue until Drop.

Web independent ownership: collector/{mod,io}.rs accumulate one bounded pending
invalidation signal from already-received relevant events, including AX updates
and lost/changed document identity, including error paths. Do not decode UI payload
for a new graph or run collection automatically. cdp/session.rs releases queued
event buffers on actual detach using existing ownership; caller-held responses
remain owned. Nearest existing event/collector/CDP tests; docs/development/web-cdp.md
or web-collector.md if affected; receipts/W03-session-invalidation.md.

Core provider ownership after current diff checkpoint: crates/engine/src/cache/store.rs
adds same-session invalidation across retained contexts, with nearest existing
cache tests; crates/host/src/worker_ops.rs exposes it narrowly via CanonicalSession;
docs/development/cache.md and receipts/W03-cache-provider.md. Declare actual subset.
Existing context-only invalidate remains compatible. No Snapshot mutation, timestamp
refresh, deletion, replay change, eviction policy or other-session contamination.
Release/lifetime/grant policies stay intact; no new cache/index/pool/framework.

Web then owns worker_web.rs connection: after ObservationRun borrow ends, apply
pending signal through real Core hook, on collector/callback success AND failure;
acknowledge the signal only after successful invalidation. No guessed placeholder
API, fake retain/revalidation or swallowed error. Core returns compiling handoff
early; Web signal/queue work proceeds in parallel. No other host source opens.

Events are read during explicit CDP requests only. This slice does not promise
continuous observation or detection of all CSS/layout changes; fresh values still
require explicit observe. When event scope cannot be narrower, conservatively
invalidate retained contexts in that session; keep original source data historical.
Document loss still invalidates refs and requires resync, not silent rebinding.

Focused tests: pending signal survives failed operation until applied; no autoobserve;
two distinct contexts in same session invalidated; second session and old bytes/
times unchanged; loss/identity failure invalidates; detach drops actual queued
ownership. Reuse existing retained/cache/CDP controls and actual canonical types,
no broad suite/new test framework. Source/peer checks first; prepare exact integrated
live reuse/resync handoff after saved compatible provider, no new browser runtime
in this source step. Existing successful platformCLI checks remain accepted.

No schema/Cargo/CLI/Native/fixture/spec changes or new directories. Temp only for
current operation, remove/verify after use; no permanent event logs. Each coherent
owner change checkpoint/push exact paths under short Git lease. Current canonical
repo URL may be used after proven same identity; no remote/key/history alteration.
Full K02/live lifecycle, notifications outside explicit calls and performance stay
separate qualification, not implied by this source integration.
