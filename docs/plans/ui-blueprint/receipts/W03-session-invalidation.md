# W03 explicit-request invalidation signal

Status: Web source signal/queue slice and actual Core hook connection implemented;
independent checks passed, saved-provider integration verification remains next.
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
