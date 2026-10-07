# H01 Native consumer handoff

Finite diagnostic packet: [H01-native-handoff](../packets/H01-native-handoff.md),
coordination21618b0; current master. Immediate consumer: Core H01 helper exchange,
then a separately assigned Native/M01 adapter entrypoint. Only this receipt is
written. Status: checkpoint_ready, awaiting root one-file Git grant; no acceptance
or next implementation authorization follows from this handoff.

## Basis and source pins

Traversal: AGENTS → specs/README registry10 → product/README and decisions/README →
D02@2 CONTENT/WORKER/PUBLICATION/LIFECYCLE, D04@1 CONTENT, D05@3 CONTENT,
D05-MEMORY@2 METRIC/LIMITS/OWNERSHIP/ADMISSION/LIFETIME/GATES, D05-WORK@1
PROFILE/GUARD/SUPERVISOR/PROOF. Full explicit closure read: D01/D06@1, D03@2,
D07@5, C01-EVIDENCE@1; CONTENT@1 BOUNDARIES/MODEL/IDENTITY/EXCHANGE/NATIVE/
GEOMETRY/PROJECTIONS/LIFECYCLE/PRIVACY/CACHE/ACTIONS/FORMS/ROADMAP/
RUST-BOUNDARIES/PERFORMANCE/GOLDEN/PILOTS/REUSE, plus RUST.md. Governance
core/routing/evidence/coordination and documentation QA apply. No revision drift.
These are requirements; source below is realization evidence, not new intent.
Excluded: analysis serialization, export, mobile, action delivery, SDK/runtime/UI,
pixels, tests/builds and any source edits. Context resolved to this closure and
only the bridge/host call chain listed below; no upstream re-audit or copied code.

Read source at HEAD `21618b0c8c5e09de8be8a973816966cb801ba013`:

| Saved source owner, equal to that HEAD | Last touching commit / inspected symbols |
| --- | --- |
| [Collector.swift](../../../../tests/bridges/native/Collector.swift) |848ec6a; main/run, emit/reply, target validation, AX/capture branches |
| [WindowAX.swift](../../../../tests/bridges/native/WindowAX.swift) |51c7809; collectWindowAX, bounded children, secure-value suppression |
| [Observe.swift](../../../../fixtures/native/Observe.swift) |848ec6a; CaptureReply/CaptureLifecycle, fixture semantics prefix |
| [prove.py](../../../../tests/bridges/native/prove.py) |88ac606; bounded Reader, descriptor/begin/sequence/collector wiring |
| [worker_ops.rs](../../../../crates/host/src/worker_ops.rs), worker_main.rs, worker_tape.rs |50c0c95; attach/observe, operation loop/publish, Tape |
| [protocol.rs](../../../../crates/host/src/protocol.rs), authority.rs, process_api.rs |3abd8e5 /50c0c95 /b22b05a; Control, TargetLease, SpawnSpec/FD contract |
| [plugin-api/lib.rs](../../../../crates/plugin-api/src/lib.rs) |2828cd8; Ticket, begin/check_request, receive/check_response |

Working-only Core evidence (SHA256 of bytes inspected; not saved or accepted):

| Path under crates/host/src | SHA256 |
| --- | --- |
| helpers.rs |43eb26c37163838756666f47fb0311d228a2341f97a3d1677af8c8565c7d9494 |
| helper_runtime.rs |a6aead9d741122640a62dcd5c5451933b5e3d9723f84d19a9e7b24e913eab8a9 |
| supervisor.rs |07cbbc24361ef536a5b7b7496296ffd6171e8ccd6d4aade627475bdf3362e06f |
| host_types.rs |4f09bf69303e7989132c4f0aca5ea3f67490105ddc90fc5a818ef045919e1d28 |

No claim of compilation here; the dispatch reports compilation separately.

## Actual producer and boundaries to carry forward

Existing executable is the test-only native-collector built from Collector.swift
+ WindowAX.swift + Observe.swift with CAPTURE_LIBRARY (see the
[native bridge README](../../../../tests/bridges/native/README.md)). There is no
shipping plugins/macos owner or H01-compatible Native entrypoint yet. Collector
requires manifest path, output directory, mode and optional host Ticket sequence
in argv; one request arrives on stdin, canonical records leave stdout. Describe
modes emit Session metadata; window-ax emits AX only; live sample mode emits AX
then capture. No capture-only mode exists. This is concrete reusable source, not
a request to introduce a second helper implementation or graph schema.

Target binding is fixture-specific: manifest PID, bundle off/on allowlist, public
NSRunningApplication launchDate, target/surface generations, CGWindowID owner PID,
and unique AXIdentifier a/b. Same titles are legal. The fixture manifest supplies
explicit continuity evidence; PID/title/CG ID/rectangle alone cannot generalize
it to arbitrary apps. TargetLease contains only target ID/generation/mutation
permission; it cannot supply native PID/incarnation/window/AX binding by itself.
Native's later trusted adapter configuration must carry the established binding,
authorized Surface/scope and cleanup-owned output destination independently of
UI text. Unresolved binding refuses window-only work; no application-wide fallback.
Session.allowed_scopes/surfaces and canonical Context remain the worker validator's
boundary. A scope string alone does not configure an arbitrary subtree collector.

Current sample fields are role/accessibility_name/enabled/accessibility_bounds;
window fields are role/description/value/placeholder/enabled/focused/actions/
accessibility_bounds. Collector requires these exact ordered lists and exact
channel sets. Actual limits: max_elements1..160, max_depth1..9, input before JSON
≤1MiB, max_output_bytes1..1MiB; window AX admission min(900ms, request duration),
per-element messaging timeout150ms. These are existing test bounds, not defaults
or full production acquisition proof. WindowAX uses count+bounded child reads and
observation-local CFEqual aliases, not action refs. Copied strings/batches and
legacy sample AXWindows/AXChildren reads still require the separate acquisition-
limit work; output-size checks after serialization do not bound SDK acquisition.

Framing is UTF-8 NDJSON, escaped newline inside strings is data. emit checks JSON
bytes before appending LF and accumulates JSON bytes across responses; transport
must count/admit the delimiter too. Input currently accepts EOF without LF; do not
mistake that convenience for verified complete framing. H01 ingress512KiB/helper,
output512KiB/channel and cumulative2MiB/request ceilings plus smaller caller limits
must govern before parse/copy. No JSON/graph decoding in the parent. A helper frame
is untrusted ingress until worker canonical validation and publication ACK.

Clock provenance: request.clock_domain must be the attached worker's returned
clock; ObservationSession::begin creates Ticket.sequence. Pass that real sequence,
request/session identity and original Context into collection; never fabricate a
Ticket or use host operation sequence as a substitute. Collector observations use
helper-PID-monotonic, systemUptime seconds; worker/session clocks use milliseconds.
Parent Instant deadline stays authoritative, passing remaining duration; helper
and worker enforce their local duration without claiming a shared Instant domain.
Keep helper observation intervals/provenance separate from parent latency.

Redaction occurs before serialization: WindowAX detects known secure role/subrole
or f02.secret and omits AXValue from acquisition, emits sensitive/redacted with no
value; unknown/unsupported/known-empty/false remain distinct. Other UI strings and
extensions are not a universal secret detector. General producer admission must
retain adapter redaction before any stdout/cache/trace/error persistence. Rust
must validate the existing canonical shape under its guard; no parent sanitizer
that reparses graphs, no parallel schema or Swift analytical engine. Pixel cleanup
is separate: current synthetic helper writes capture.png plus metadata/error JSON;
that does not authorize production unmasked storage or arbitrary payload_ref paths.
Output artifact ownership/crop/redaction remains a Native consumer dependency.

CaptureLifecycle owns one bounded callback result, explicit permission outcome,
window-isolated/no-audio/no-cursor/no-child capture and unknown crop transform. Its
current run-owned flock is configured by environment, retained on timeout until
helper retirement; H01 has a separate parent CaptureLease until actual reap.
Later entrypoint must reconcile those owners without an independent competing
admission mechanism or global AX lock. Do not copy test fault environment settings
into production. Preserve [M01 residuals](M01-capture-review.md): B error-3801 remains
permission_required, no pixel retry/backend/settings change; no new live proof here.

## Minimum missing Core hook — engineering proposal for its owner

Observed gap: CanonicalSession::observe requires Tape(Request, completed responses)
and calls begin only after those bytes exist. Controls have no helper-request/reply
kinds. Existing spawn_helper/write_helper/read_helper/take_helper_bytes provide
parent ownership, not a connected producer exchange. submit consumes InputLease;
reserve_input refuses Running. take_helper_bytes moves the sole ingress and starts
helper termination, so it cannot deliver AX and let the same combined collector
continue to capture. These facts rule out simply wrapping today's Tape API.

Proposed narrow connection, not a selected binary layout or new product contract:

1. Worker admits the existing Request, calls real begin, then requests collection
   through private fixed controls correlated by session epoch + operation + helper
   slot/serial + channel and real Ticket sequence. Payload is bounded existing
   canonical request bytes; executable/binding selection is a parent-authorized
   adapter handle, never a UI-provided path/PID/argv or arbitrary worker spawn.
2. Core services that request while the operation is Running: charged outbound
   bytes, partial-write offsets, opaque bounded ingress and delivery to the guarded
   worker, EOF/error/deadline/cancel disposition and no dispatch after terminal.
   Core must explicitly account buffer reuse/overlap; do not allocate another
   uncharged request Vec or try to reserve a second public operation InputLease.
3. For the smallest one-frame ownership fit, Native proposes channel-specific
   invocation of the same collector source: AX and capture in independently owned
   helper slots, each returning one ChannelResponse. This requires an explicitly
   assigned Native entrypoint/mode adaptation; current combined live mode cannot
   be relabelled channel-specific. Both helpers remain directly parent-owned, no
   Rust-worker grandchildren. Parent capture admission affects capture only.
4. Worker consumes each framed byte slice inside the existing allocation guard,
   validates Document and ObservationSession::receive, preserves provenance and
   emits the existing canonical channel through Frame→Commit→ACK before next risky
   channel work. Parent copies only admitted bytes. AX ACK survives later capture
   failure. Failed permission/timeout response must retain its canonical issue or
   an authorized bounded missing-channel disposition, not invented empty success.
   Current observe cancels/returns WorkerFailed on the first failed response; that
   behavior is not evidence of complete independent-channel/error publication.

Native entrypoint adaptation also must replace argv/stdin/stdout/env assumptions
with existing SpawnSpec exact executable + FD3 input/FD4 output/FD5 status and
trusted bounded configuration; 0/1/2 are /dev/null and environment is empty. Reuse
actual collector/CaptureLifecycle mechanisms. No signature or numeric encoding is
imposed on Core by this receipt; the necessary hook is begin→bounded helper exchange
while active→guarded receive→per-channel publication ACK, using its current owners.

Next consumer: Core chooses and supplies that concrete internal hook plus admitted
configuration ownership; root assigns Native adaptation against it. Remaining
Target binding for non-fixture apps, acquisition/string/pixel limits, redaction,
connected lifecycle and positive M01 gates are explicit work, not waived here.
Only documentation links/route consistency/whitespace are checked for this receipt;
no source mutation, build/test, SDK call, app launch or capture was performed.
