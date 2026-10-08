# W03-R — explicit Web recovery and recorded full/delta

Mode Restore; shipping_product; finite autonomous task under PLAN.UIB@1 original
358c757e7eab84a3989d150dbad57924d866601a and the direct task dispatch in this chat.
[Task authority](../packets/autonomous-tasks-2026-10-08.md)
allows the complete implementation/test/commit/push cycle without a new grant.
Historical prepare-only packets are inputs, not the current execution boundary.

## Traversal and reconciled basis

AGENTS → specs/README registry26 → product/README → CACHE/LIFECYCLE/IDENTITY/
EXCHANGE/PRIVACY/PROJECTIONS; acceptance/README → WEB-PILOTS/GOLDEN/PILOTS;
development/decisions/README → D02/D04/D05 and explicit closure. Fully read:
BOUNDARIES/MODEL/IDENTITY/GEOMETRY/PROJECTIONS/FORMS/ACTIONS/LIFECYCLE/CACHE/
PRIVACY/ROADMAP/RUST-BOUNDARIES/PILOTS/WEB-PILOTS/GOLDEN/PERFORMANCE/REUSE@1,
EXCHANGE/NATIVE@2, D01@1/D02@2/D03@3/D04@1/D05@4/MEMORY@2/WORK@1/D06@1/
D07@5/EVIDENCE@1; CONTENT clauses plus D02 worker/publication/lifecycle and
MEMORY/WORK clauses. RUST.md and DEV.RUST@2; global implementation, product-truth
core/routing/change/evidence/delivery/coordination and QA rules. Native is read
only as a required shared-boundary dependency. Native acquisition, local analysis,
export, CLI and unrelated pilot/acceptance leaves excluded by their triggers.
Stale @1 links to EXCHANGE/NATIVE navigate to registered @2; no semantic drift
in the selected W03 meaning. Source evidence does not create new requirements.

Observed: existing collector closes on event loss/failed transport; existing
CanonicalSession invalidates recorded entries, never autocollects. Explicit
fresh attach + bounded Initial Observe is the existing safe recovery API. Existing
Replay uses actual retained CacheStore, exact base bytes and atomic replay. No
second cache or automatically rebound ref is needed. Prior parent/font runtime
and peer loss evidence remain attributed to their original receipts.

## Implementation plan / owned writes

Extend existing web_live.rs/guarded-live.cjs through finite helper files
crates/host/tests/support/web_resync.rs and tests/bridges/web/resync-transport.cjs;
focused crates/engine/tests/cache.rs test, docs/development/cache.md,
docs/development/web-collector.md and this receipt. No directory/manifests/schema/
protocol/CLI/Native changes. No nested agents or desktop input. Own system-temp
build/run root and headless browser only. Preserve unrelated checkout edits.

Scenario: actual owned TCP close while fixture changes → refusal and byte-equal
recorded history → independent Target B → explicit detach/attach/scoped current A;
bounded selection refusal → explicit current request; independently acquired full
canonical checkpoint → real worker Replay equality, missing-base and scope refusal.
Known empty, unknown and redacted are checked on actual Web data. Synthetic queue
overflow is separately attributed, never called actual browser-event loss.
Cache test protects unobserved historical nodes/times and atomic scope refusal.
Required evidence: focused cache/Web tests, affected Clippy/format, actual headless
scenario and confirmed owned cleanup; exact-path checkpoint+canonical master push.
Independent acceptance remains a separately reported gap, not self-review.

## Reproduced implementation defect

First live run passed transport/history/isolation/recovery/limit/private read, then
stopped at Replay. A fixed-code-only rerun established scope mismatch returned
InvalidInput from Document::from_json before CacheStore. CACHE/EXCHANGE require
ResyncRequired. Add crates/host/src/worker_ops.rs to the write set: only map canonical
IncompatibleContext/ResyncRequired decoder failures to HostError::ResyncRequired
inside Replay. Other malformed/private/invalid input and all other operations stay
unchanged. Shared Web/Native retained Replay has the same compatibility contract;
no protocol, schema or replay-engine change. This is Restore, not a spec delta.

## Final evidence and result

Owned writes additionally include crates/host/tests/web_worker.rs: extend the
existing synthetic event-loss test through old-owner refusal with zero extra CDP
calls, explicit detach/new attach and one successful bounded initial collection.
Other Target keeps progressing; shutdown releases sessions/groups and preserves
caller-held original bytes. CDP queued-owner cleanup is separately checked by
its existing focused unit test, including caller-held events after detach.

Build inputs: immutable git archive of82342f0e67761504bd643fe4d9d027af93481d0e
(saved master after the dispatch checkpoint) with only W03-R Rust test/source
candidates overlaid, in an owned OS-temp directory. Parallel Graph/Native WIP was
not consumed. Actual author checks on Rust1.96.0/aarch64-apple-darwin, locked/offline:

- Host web_live no-run; affected host session-worker/web_live/web_worker Clippy
  with Web feature and -D warnings: PASS.
- Cache `partial_replay_keeps_unobserved_history_and_scope_mismatch_is_atomic`:
  PASS; partial omission preserves original node/Observation, current read refuses,
  context mismatch is atomic and original entry stays invalidated.
- Cache `controlled_full_delta_oracle_preserves_unavailable_and_empty_values`:
  PASS, four authored variants (unknown/unsupported/redacted/false, each with empty
  name). Independent full source checkpoint from existing synthetic GOLDEN input
  equals admitted replay including complete canonical bytes; base is unchanged.
- Host `retained_history_survives_received_event_or_loss_and_other_session_progresses`:
  PASS, two synthetic variants; loss branch now includes explicit recovery.
- Web `detach_releases_only_queued_event_ownership_without_waiting_for_client_drop`:
  PASS; real queue charges release, independent caller-held event remains valid.

Actual `resync` headless scenario PASS with nine read-only invariant checks and
fixture survival after all worker reaps. Chromium145.0.7632.6, Playwright1.58.2,
Node24.15.0, 800×600/DPR1. Existing32nodes/depth8/64KiB/250ms and256visit profile;
refusal stimulus lowers only one call's selected visit bound to1. Actual owned
TCP close, then parent/font fixture stimuli, gives refusal/no publication; original
Snapshot Retain and first ACKed lease stay byte-equal. Independent Target B reports
120×32css_px; explicit recovered A reports150×48css_px. One bounded refusal then
one new explicit current request succeeds. Coverage remains partial; source_state
remains absent; current acquisition consistency remains unknown.

Real worker replay: lost base and mismatched scope return ResyncRequired with0
committed channels; valid recorded replay completes with1. Expected composite is
assembled from original source records before comparing output: newer selected
properties, one omitted historical web.ax node and both needed old observations,
plus historical Surface binding evidence. Sensitive draft omits AX acquisition;
its old public empty value remains historical, never relabelled current. Original
base can still be read byte-for-byte. All browser/server/profile/test/process/
forwarding resources report confirmed cleanup; sessions0/groups0/abandonedfalse,
no pending case. No screenshot, permanent raw output or real product was involved.

### Discrepancies and honest limits

The first live equality assertion incorrectly expected fresh Surface metadata and
an omitted AX node to be replaced/deleted by Delta. Corrected the test oracle to
CACHE/D03's historical metadata/absence semantics, without changing code/schema,
source timestamps or full captures. The strict full/delta equality proof is the
controlled synthetic source oracle above, not equality between two live captures.
Changing Surface metadata via Delta would require a separately authorized shared
schema/replay change; W03-R does not perform it. Full current Web observation is
already available through the scoped production path.

Actual transport loss is intentionally induced on the owned connection. Synthetic
queue overflow is not called naturally occurring Chromium event loss. Idle event
detection, performance/D06, full K02/P4/P7 and independent acceptance are not
claimed. These author checks are not independent review; that remains an explicit
acceptance gap for the coordinator. No additional agent/chat was created.

Final engine cache Clippy -D warnings, scoped rustfmt --check, both launcher Node
syntax checks, changed local documentation links and git diff --check: PASS.
All five task-owned Rust inputs equal the tested immutable source overlay. No
shared workspace formatter or Cargo/dependency update was run. Canonical run output
is transient and will be removed after this receipt records its consumed result;
no images were created. Checkpoint/push identity is returned in the chat result.

## Reviewer P2 correction — Delta-only recovery classification (2026-10-08)

Authority: direct correction dispatch in this chat, following independent source
review of ffe33e4. Restore the original requirement that inappropriate artifacts
remain InvalidInput; no renewed Web runtime or adjacent feature scope. Reused the
fully read W03-R CACHE/EXCHANGE/D02/D05-WORK/RUST/DEV.RUST basis above after verifying
those selected contracts unchanged since ffe33e4. Current AGENTS and receipt read.
Owned writes: worker_ops.rs, new crates/host/tests/replay_input.rs in the existing
tests directory, and this receipt. Web/worker_web/harness, schema, API, engine,
protocol, CLI and other owners' WIP remain untouched.

Reproduced BEFORE the correction with the actual guarded worker: first Replay
Tape segment ENV-SESSION-INVALID.json yields Failed(ResyncRequired), while the
regression expects Failed(InvalidInput). Canonical validation rejects this
session_context with IncompatibleContext before the old Artifact::Delta check.
The finding is an implementation defect; the intended error contract is unchanged.

Fix: Document::from_json remains the mandatory full validator. Only when it
returns IncompatibleContext or ResyncRequired, deserialize the same bounded bytes
with the existing canonical Document/serde decoder to establish artifact kind.
Only Artifact::Delta receives ResyncRequired; all other kinds and malformed inputs
receive InvalidInput. This diagnostic-only second decode never admits/reuses the
invalid record, adds no parser/types/schema/tag scanner and remains inside the
worker allocation guard. Successful requests and all other validation failures
keep their single decode; no parent parsing or validation bypass was introduced.

Focused regression `replay_recovery_is_reserved_for_delta_compatibility`: PASS
following the observed pre-fix failure. Real Retain/Replay covers invalid
session_context, valid non-Delta, malformed JSON, missing-property/private-value
Delta => InvalidInput; lost base, scope and revision mismatch => ResyncRequired;
compatible Delta => exact independent full canonical bytes. Refusals commit0;
original retained bytes survive refusals and successful replay. Worker shutdown
confirms sessions0/groups0/abandonedfalse. No browser/native target was attached.

Checked immutable source archive ef19ba5c67ce5f35f588339758b8c8f9cb8ba28c with only
this correction's two Rust files overlaid, isolated OS-temp Cargo target. Commands:
`cargo test --locked --offline -p uiblueprint-host --test replay_input -- --exact
replay_recovery_is_reserved_for_delta_compatibility`; affected session-worker and
replay_input Clippy with -D warnings; scoped rustfmt --check and git diff --check.
Prior successful Web/live evidence remains applicable and was not rerun. This
correction has author verification; the same independent reviewer receives the
saved result through the coordinator. No agents/chats or external messages created.

Correction Clippy, scoped rustfmt and diff checks all PASS; both Rust inputs match
the tested overlay. Temporary source/build files are run-owned, contain no images,
and are removed after recording these results. Checkpoint/push is returned in chat.
