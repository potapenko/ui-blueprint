# Owned Web transport independent review

Class: verification; one fresh non-author reviewer, inherit settings, no nested
agents. Explicit user authorization: coordinated PLAN.UIB@1 and independent review.
Read-only current master; no files/Git mutation, builds/tests/runtime/peers/apps,
network/external communication or additional agents.

Artifact: saved `4106e04a91e141158dc46590778fb5fd2ee300a9`; basec20a54b.
Scope: plugins/web source/tests/manifest, its root Cargo/lock wiring and exact
D07@4/registry8 adoption. Concurrent H01 membership/source and D05 registration
are excluded except to preserve applicable current contract context. Inspect
staged/unstaged/untracked scope for drift, use pinned Git for the artifact.

## Neutral basis and required outcome

AGENTS → specs/README registry9 → product and decisions. Read D07@4 (all saved
source-audit guards), D02@2 CONTENT/WORKER/LIFECYCLE, D05@3/D05-MEMORY@2/D05-WORK@1,
D01@1/D03@2, DEV.RUST@2/RUST.md and complete explicit dependencies: BOUNDARIES,
EXCHANGE,LIFECYCLE,NATIVE,MODEL,IDENTITY,GEOMETRY,PROJECTIONS,PRIVACY,FORMS,ACTIONS,
CACHE,GOLDEN,ROADMAP,PERFORMANCE,RUST-BOUNDARIES,REUSE,C01-EVIDENCE@1;
ANALYSIS/TYPES/VALIDATION@1 + CLI only because the dependency closure names them.
Root has read this current closure. Reuse fully read current nodes; no product
intent from branch summaries. Public unknown/privacy/permission boundaries remain.

Decision authority: ROADMAP delegated D07 and root's
[implementation packet](W01-transport-implementation.md), with exact narrow logging
dependency amendmentcb90b7e. Pre-implementation source audit e5da7d6 at
docs/research/W01-transport.md gives selected pinned APIs/guards; this is eligible
neutral evidence, not a claim that the candidate works. No transport release baseline.
Registry9's new host is not implemented here and does not turn codec limits into
total allocation bounds. No browser/live collector/CDP graph/session owner is in
this transport artifact; those remain actual required next consumers.

## Mandatory criteria for this finite slice

1. Accept only explicit authorized numeric-loopback ws endpoints/paths with finite
   validated inputs; no DNS/redirect/TLS/proxy convenience fallback or unsolicited
   extensions. Authentication/target authority is caller-owned, never discovered
   from peer text. One actual IO owner; cancellation handle is shutdown-only.
2. Carry the same absolute monotonic operation deadline through connect, handshake,
   read/write/flush/close; enforce finite byte/work/payload/buffer/output limits
   before the operation they bound, including zero/overflow and library panic paths.
   Do not mislabel these as allocator/RSS/SDK or full D05 enforcement.
3. Partial/queued writes are resumed through the same codec/deadline without
   re-enqueueing or repeating a possibly sent command. Backpressure/control frames,
   fragmentation and read-ahead accounting preserve that invariant. Binary data
   cannot become CDP text; protocol control does not imply application completion.
4. Cancel/timeout/error/close/drop stop new sends, suppress late outcomes and clean
   only owned sockets/state. Pending operation lifecycle is explicit and another
   independent target can progress. No implicit reconnect/retry/heartbeat/collection.
   Source must accurately represent any bounded cancellation limitation rather
   than claiming instant interruption of an uncancellable platform call.
5. Error/Event Debug and ordinary diagnostics do not leak private peer bytes,
   headers/URI credentials/UTF8/close content. The actual tungstenite log boundary
   filters upstream payloads before host delegation; no silent takeover of existing
   logger or blanket suppression of unrelated host logs. Installation/refusal and
   actual canary evidence cover the implemented public boundary.
6. Dependency features/versions/MSRV/license adoption match D07@4; existing pins
   and float_roundtrip protected. No copied protocol implementation, async/TLS/url
   stack or platform SDK in unrelated core. Verify actual saved feature intent,
   not a newer online library's documentation.
7. Tests independently exercise public failure/state transitions, enforce cleanup
   of their own peers and distinguish executed checks from future live proof.
   Actual source/check identity must match final author evidence. Source review
   does not claim independent execution or validate a whole browser adapter.

Do not require CDP correlation/collection, host allocator integration, live Chromium
or full B01–B06 in this codec-level review. Those are explicit future obligations,
not permission for a false broader claim. Conversely, a defect that invalidates
this slice's deadline/privacy/cancel/no-repeat contract is actionable now.

## Independent two-stage result

First inspect pinned source/tests/contracts and necessary local exact dependency
source. Do NOT read author docs/development/web-transport.md or
receipts/W01-transport-implementation.md, author runtime-count/approval narratives,
or new final handoff until initial observations/criterion coverage have been sent
to root. Root then supplies the author receipt and exact saved evidence for SAME
reviewer reconciliation. No replacement, parallel duplicate or reviewer shopping.

Return concise normal Markdown: scoped accept/accept_with_residual/reject/
not_verified, actionable introduced defects with scenario/impact, exact shortest
file/line range, repair owner/recheck and remaining genuine evidence gaps. One
::code-comment per inline issue, absolute file/title/body, optional range/priority;
no directives when no issues. Verify applicable AGENTS.override/AGENTS precedence
and materially supporting rule references with smallest lines; don't invent
citations/findings or demand style-only changes. No JSON/structured schema.
If unable to perform review, report the failure instead of invented findings.
Keep unaffected accepted analysis/cache/native work closed; do not become builder.
