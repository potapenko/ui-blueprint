# W01 guarded worker composition receipt

Status: real worker_web module written; Core hooks/shared-input barrier and runtime
proof pending. Authority [packet](../packets/W01-guarded-worker.md), transfer25bdae2,
shared Core boundarya8e5c06 under approved parallel PLAN.UIB@1. No live browser/SDK/UI.
No nested agent or branch/worktree; master only. Web owns only the transferred paths.

## Basis and concrete consumer

Reuse current registry10 → product/development D02@2/D03@2/D04@1/D05@3/MEMORY@2/
WORK@1/D06@1/D07@5 and full existing Web/H01 closure, identity/scope/exchange/model/
privacy/lifecycle/publication, WEB-PILOTS/PILOTS, RUST/DEV.RUST@2/QA governance.
Guard acceptance64cbec9 is supporting evidence, not a reason to rerun its general
audit. Read only actual affected CanonicalSession/ObservationRun, worker admission/
clock/encoding/publication, TargetLease, Tape/config and existing runtime test setup.
Mode Restore; no spec, canonical wire0.1/analysis0.2, quota or feature-default delta.

Immediate implementation: executable-private WebSession::attach/observe in
crates/host/src/worker_web.rs; exact signatures/ownership in
[handoff](../../../development/host-web.md). Actual file exists for Core's module hook.
Attach consumes explicit trusted WebSetup and actual worker clock/TargetLease,
installs required log filtering and owns the existing Transport→CDP→Collector.
Observe decodes only Request+selection config, gets real begin/Ticket, waits for
actual parent permission, performs guarded collection, bounded encoding, real receive,
then actual Frame/Commit/ACK. No dummy success, pre-collected response or fake refs.

Encoding uses fixed preallocated output and publication allowance; canonical receive
runs under ordinary quota after original Document drop. Shared borrow/API boundary
is sufficient; no new publish/receive API needed. Source callback failure preserves
the specific HostError and unfinished ObservationRun Drop-cancels. No second graph,
client, JSON parser, parent decode or ref-cache/JSON envelope was introduced.
Bootstrap canonical refs remain represented by real emitted Snapshot/Observation IDs.

## Exact shared dependencies returned before crossing ownership

Core owns main/module/worker dispatcher and parent admission/ObserveReady/Permit;
those hooks were absent at the saved input. No synthetic substitute is used for a
runtime completion claim. Core must provide the actual Web attach/submit service and
a short ready-to-compile/run input handoff. Source preparation proceeds independently.
For real bounded test peers, requested Core add existing approved tungstenite.workspace
as host dev dependency; no new version/features/library or handwritten WS codec.
This Web worker has not edited any Cargo/module/common worker/parent/Native files.

## Current write set, checks and retention

Owned now: crates/host/src/worker_web.rs, crates/host/src/web_config.rs;
future tests/web_worker.rs and only support/web_worker_*; docs/development/host-web.md
and this receipt. web_config adds bounded config decoding; source signatures are
real implementation, not placeholder modules. Scoped rustfmt applied to owned files.
No Cargo or runtime success claimed before module hook and input barrier.

Initial shared pins from root: worker_main59b664c8…564ac5;
worker_ops45ae90c8…fc2bb; worker_observation12560324…92958;
web_config94fd7bc4…9d2d before Web-owned edits. Allocator2f1bf278/worker_io9700bfa
remain protected. Exact final inputs/checks/source hash follow the actual barrier.
No raw logs stored, no outside app/process operated. Existing primary temp remains
owned by Web for the immediate consuming source work; new runtime test artifacts
will use a named task temp, with cleanup after accepted evidence.

No full H01, live Web/B01–B06/D06 or independent acceptance. Root grants an exact-path
checkpoint+push once a coherent source/proof stage is ready; no autonomous next packet.
