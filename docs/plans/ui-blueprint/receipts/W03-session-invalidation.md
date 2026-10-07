# W03 explicit-request invalidation signal

Status: Web source/hook saved24ed0e8; finite reusable-worker event/loss integration
on saved provider74d2e3b passed. Direct cache-state proof remains separately attributed.
Authority: root implementation dispatch/[packet](../packets/W03-session-invalidation.md)
under approved P4/W03. Existing CACHE/LIFECYCLE/IDENTITY/EXCHANGE/PRIVACY/PROJECTIONS,
D02/D04/D05 MEMORY/WORK closure already read and current; CLI additions do not alter
it. Restore existing invalidation contract, no schema/CLI/new cache meaning.

Exact first7paths: plugins/web/src/collector/{mod,io}.rs;
plugins/web/src/cdp/{session,session_tests}.rs; plugins/web/tests/collector.rs;
docs/development/web-collector.md; this receipt. Core engine/worker_ops owns the
same-session cache hook separately. No other owners, fixture or site changes.

One bounded pending boolean is set by already-received relevant event methods,
existing document/identity loss, event overflow, detach/cancel/expiry and failed
exchange. Protocol/parse/callback failure never consumes an already-pending flag.
Inspection is non-consuming/no IO; explicit acknowledge is reserved for successful
cache apply. Original stale/resync outcomes remain. Unrecognized events are dropped
as before; no raw event content is decoded into a graph or retained diagnostically.
No extra domain enable, monitor, poll, thread, callback dispatch or autoobserve.

Actual CDP detach drops all still-queued WireEvent owners/permits, resets head/queued,
and closes its own transport. Caller-held events/replies keep their separate owners;
repeated detach cannot release them. Existing finite pools/storage remain unchanged.

Focused checks target relevant event coalescing on success/failure, pending survival
and acknowledgement without dispatch, identity/loss/cancel, and actual event quota
release before Client Drop with a caller-held event surviving. Source/peer proof is
not live cache integration or a new positive pilot. Historical Snapshot/time,
other-session isolation and cache-wide apply are Core provider obligations.

Next exact integration: consume Core's real compiling CanonicalSession hook in
worker_web.rs only, after ObservationRun ends, on success and failure; clear signal
only after successful apply. Do not fabricate an API or second cache. No live run
is authorized in this source step. Current operation temps only; old evidence intact.

Independent Web checks PASS: queued-owner detach test (slots and reserved bytes
released while caller-held event remains valid), content-event success/failure
coalescing, changed-document/cancelled connection, existing lost-event/refusal and
in-flight AX cancellation assertions. Five focused tests, no browser execution.
Affected Web library/collector Clippy -D warnings PASS. Peeking/acknowledging the
pending signal dispatches no RPC. Existing protocol/private-value errors stay
payload-free; no automatic collection or cache mutation claimed by these tests.

Five source/test candidate hashes are summarized by compact sorted JSON
path→SHA256 fingerprint 4cbbf04694c50952013d1506edd396c6c097b1bb38979562b874384791cbc5e6.
Shared compile uses existing unchanged Web/schema dependency configuration; no
Core engine source is a dependency of this independent slice. The existing task-owned temporary Cargo target was reused; no new directory,
persisted command log or browser/runtime evidence was created. Earlier retained
build/evidence ownership was not deleted or changed by a cleanup sweep.
Actual Core method now present is CanonicalSession::invalidate_retained_session
(&mut self)->Result<usize,HostError>, but its saved/compiling provider handoff is
still awaited before the worker connection is claimed. No placeholder call added.

## Actual Core hook connection

Root supplied compiling CanonicalSession::invalidate_retained_session()->
Result<usize,HostError>, wrapping CacheStore::invalidate_session(cache_session).
Returned count includes already-marked entries, no canonical data/time mutation.
Added exactly worker_web.rs as eighth write. Observation body is scoped in a local
closure so all success/collector/callback/early errors end the ObservationRun borrow
before apply. Pending signal calls actual hook, then acknowledge; apply error maps
through existing Finish diagnostic and returns explicitly, leaving signal pending.
Successful apply preserves the original operation result/diagnostic. No pool, new
wire/graph, replacement cache or additional CDP dispatch introduced.

Affected actual web session-worker cargo check PASS against the current real Core
hook. Core provider source save/equality and integration tests remain separately
owned/next; this compile does not claim a saved-provider or live cache acceptance.
Exact coherent8paths are the first7 above plus crates/host/src/worker_web.rs.
Worker callsite SHA256 070582052fcb457fb1068d9cc2b9aa453885fa08e383582ae0d1627441897620.

## Saved-provider reusable-worker integration

Root opened exactly crates/host/tests/web_worker.rs and its existing
support/web_worker_peer.rs, plus this result receipt. One test-only events_once
atomic emits1 relevant Accessibility.nodesUpdated event or5events (existing4-slot
queue loss) before the next explicitly requested reply, then resets to0. No
production event API, observer, new framework, cache inspection seam or browser.

The finite test creates two actual guarded workers with distinct canonical
session IDs. It observes A/B, explicitly Retains each actual Snapshot, then emits
event/loss during A's next explicit observe. Normal event completes; overflow
returns ResyncRequired without publication. B receives no CDP call from A's work
and subsequently observes successfully. Duplicate recorded Retain returns each
original Snapshot's exact canonical bytes and dispatches no CDP request. A's first
ACKed channel lease stays byte-equal across these operations and shutdown. Actual
shutdown releases sessions0/groups0, abandonedfalse. No fake Ticket/ACK/reap.

Observable limit: host Retain/Replay return Recorded data, not StoredRead.invalidated
or CurrentRequired cache reads. These results are not proof of an internal flag.
Combine saved24ed0e8 actual callsite with Core74d2e3b's direct cache-state test:
two contexts marked, other session unchanged, original bytes/time/expiry and
charges retained, CurrentRequired refuses revalidation. No new seam/protocol was
built merely to duplicate that check; full live cache lifecycle stays separate.

Exact provider74d2e3b7563031efbe3000b14c1429954c7bd981 includes Web24ed0e8 and
current saved metadata. Exported Cargo.toml/Cargo.lock/rust-toolchain.toml,crates,
plugins/web,fixtures/analysis,fixtures/golden into current-operation system temp
uib-w03-worker-ra6gx5zj, overlay only the two candidate test files.363 exported
source/support files matched saved base or exact candidate before build; compact
sorted JSON path→SHA256 fingerprint
61f926fee217b3800bb862c5ff9bc174e179fdc3c449c556f9f7a6ee95d11d94.
No current Core CLI/diff WIP consumed or held.

PASS: cargo test --locked --offline -p uiblueprint-host --features web --test
web_worker retained_history_survives_received_event_or_loss_and_other_session_progresses
-- --exact, one test/two event variants,1.34s. No old suite or accepted browser
scenario rerun. Affected test-target Clippy and temporary cleanup follow below.

Affected host web_worker test-target Clippy -D warnings PASS. Post-check363-input
fingerprint unchanged; both candidate test files equal the checked snapshot. Actual
worker executable SHA2564bda8248dd99ab8bf4f46d5131c0f7a3fa83e0705209553c7ef9ccc9b6bfad33 (consumed then removed).
Current uib-w03-worker-ra6gx5zj source/build temp removed and absence verified.
No raw run logs/canonical records persisted; previous evidence untouched. Exact
checkpoint set: the two host test files plus this human outcome receipt.

## Prepared finite F01 B05 current-geometry/history sequence

Root opened exactly crates/host/tests/web_live.rs, tests/bridges/web/guarded-live.cjs
and this receipt for preparation; no new browser activation yet. Mode b05 adds
only mutation-child scope and existing fixture parentWide/fontLarge stimuli.
Initial explicit Observe requests LayoutBounds, then explicit Retain stores its
actual Snapshot. ParentWide and fontLarge run as separate fixture actions; each
is followed by CurrentRequired Observe using the ORIGINAL canonical DOM ref.
Independent F01 B05 literals:120×32→150×32→150×48css_px; authored location40,550.
Same DOM source key/viewport Space required; revision/Observation change, each
request's actual environment_revision stays unchanged in its own result. No
rewriting old environment/context to satisfy cache or recorded comparison.

After each change, explicit duplicate Retain of the ORIGINAL Snapshot must return
its exact original canonical bytes/time/context. First ACKed channel lease remains
byte-equal throughout. Six reported outcomes:3observe+3Retain, with read-only
focus/scroll/checkpoint checks for each. Actual host shutdown/worker reap while
browser alive plus independent owned launcher cleanup remain mandatory.

This proves fresh geometry and recorded-history preservation only. No assertion
reads an internal invalidated flag or treats fixture revision/event counters as
actual CDP loss. Core direct-state tests/source hookup and previous peer loss flow
remain separate evidence. No new cache/API/fixture/product source or old sequence.
Same32/depth8/64KiB/250ms/source16/100methods/256visits, existing byte/IO limits,
800×600/DPR1/Chromium145 and120s whole-run bound remain fixed.

Prepared immutable source base3318662b1e70621060ba6162ffb6a3bfbb092c03 includes
saved W03 Web/Core and host peer integration. git archive of Cargo manifests/lock/
toolchain,crates,plugins/web,fixtures/web,experiments/web,tests/bridges/web plus
only two candidate harness overlays.188exported input fingerprint (compact sorted
JSON path→SHA256):33ed278d952652b580556b1c1c2f91c6f740da71626feb5e6380cac4c89e785c.
Current-operation build/source root:
/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-b05-o551kaii.
No source hold or current Core recorded-diff WIP consumed.

Scoped web_live no-run PASS. Test build/debug/deps/web_live-5d88e78afab66360 SHA256
224ad5579b4e6ff229246c196e25cf048f8ced4d2b8ff83594c9940b02ff6255;
worker build/debug/session-worker SHA256
ef2e5873c87a513b79383a297cfccd08aadafe6310dc0fa6573719feb4d259cd.
Both harness inputs match prepared source. Consumer Clippy -D warnings and
launcher syntax PASS; no runtime performed.

After saved handoff and runtime activation: existing S01_WEB_PLAYWRIGHT_CORE path,
UIB_WEB_LIVE_ALLOW=1, UIB_WEB_LIVE_CASE=b05, exact test/worker paths and SHA256 above,
UIB_WEB_LIVE_EVIDENCE=/private/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/579204b1-4d1a-4efe-864c-0a0c25e4f076,
node tests/bridges/web/guarded-live.cjs --run-authorized. Fresh output path remains
absent during preparation. Three unchanged ACKed frames b05-initial/parent/font.json
and compact report may exist only during this operation. Deliver factual outcome
inline, consume then remove own output/files/emptydir and build/source temp, verify
absence. Older evidence untouched; no persistent archive or screenshot links.
