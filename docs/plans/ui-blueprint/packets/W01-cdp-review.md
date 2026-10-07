# CDP correlation and ownership independent review

Class: verification; one fresh non-author reviewer, inherit settings; no nested
agents. User authorized coordinated PLAN.UIB@1 and independent reviews. Current
master, read-only. Source candidate `2f9ea77aeec6465e6d6d3a7d109d48840f1f8287`; base
3abd8e5 includes the saved dependency handoff. Review plugins/web/src/cdp/**,
src/lib.rs exports and tests/cdp.rs only, with the exact saved manifest graph.
Inspect related staged/unstaged/untracked drift; exclude concurrent H01/Native work.

## Neutral authority and scope

AGENTS → specs/README registry10 → product/decisions → D02@2, D03@2, D04@1,
D05@3/MEMORY@2/WORK@1, D07@5, D01/RUST/DEV.RUST@2 and complete explicit closure:
BOUNDARIES/EXCHANGE/IDENTITY/LIFECYCLE/PRIVACY/MODEL/GEOMETRY/PROJECTIONS/FORMS/
ACTIONS/CACHE/GOLDEN/ROADMAP/PERFORMANCE/PILOTS/REUSE/RUST-BOUNDARIES/evidence;
ANALYSIS/TYPES/VALIDATION+CLI where required. Root read the full current closure.
Approved scope is [W01-cdp-session](W01-cdp-session.md), including dependency
amendment3231928. No product/spec delta, browser/runtime or released baseline.

Accepted transport4106e04 is an unchanged dependency with narrow source audit
docs/research/W01-transport.md. Read its API source as necessary, not author verdicts.
R01/S01-Web source records are prior evidence, not current browser qualification.
Exact primary CDP envelope source may be read locally when necessary; no new
network retrieval or broad upstream survey. Unknown implementation details remain
evidence questions, not product invention or a reason to demand a new framework.

Review only the actual private CDP client layer. Total allocator/RSS enforcement,
guarded host publication, action permits, semantic method decoders/collector,
live Chromium/B01–B06 and D06 are later consumers. Their absence is not a codec
defect, and they must not be claimed delivered by this scope. Do not require
multi-outstanding support if its API is sequential; separate connections must not
be globally serialized. Accepted transport/analysis/cache/native source stays closed
unless a concrete new affected defect is demonstrated.

## Required criteria

1. Caller-provided canonical binding and private connection epoch/request IDs
   cannot be replaced by peer text. Register before send; exact int/session/context
   correlation rejects wrong/missing/duplicate/late/cancelled/unknown or old-epoch
   replies. Counters refuse overflow, not wrap/lossy coercion or ID reuse.
2. Preserve original absolute deadline across delayed dispatch and pending flush;
   no resend of possibly sent bytes or false success from socket progress. Cancel,
   expiry, detach/drop and later failure stop new dispatch without reviving tickets
   from buffered transport data. Already completed owned results survive correctly.
3. Explicit finite request/metadata/result/event bounds reserve before dispatch or
   retention. Actual String/container capacity and caller-held payload lifetime
   agree with permits; no uncounted retained copy, leak/early refund or whole-owner
   borrow that contradicts the promised independent progress. This is not by itself
   a global allocation bound; inspect claims at their actual level.
4. Shallow pinned-serde decoding validates required envelope shapes/IDs/fields,
   refuses ambiguous/malformed headers, separates protocol errors from method
   replies, and preserves bounded raw payload for its consumer. No second JSON/
   graph model, implicit-null substitute or forged normalized facts.
5. Events are bounded independent inputs, never replies or automatic recollection.
   Lost/overflowed continuity is explicit for consumer resync, not fabricated
   complete state. No silent target/session discovery expansion or retry.
6. Diagnostics/Debug/Failure/logs cannot leak peer payload/error/session/method
   canaries; raw data delivery is explicit. Existing upstream logging boundary and
   caller authority remain; CDP receipt never proves business/action success.
7. Tests/recorded proof cover actual exposed state transitions, their own finite
   peer/thread/socket cleanup and protected source/dependencies. Exact saved input
   identity must support author evidence; source review is not independent execution.

## Independent first observation and final reconciliation

First inspect pinned source/tests/contracts and source declarations, including exact
local dependencies if needed. Do NOT initially read docs/development/web-cdp.md,
receipts/W01-cdp-session.md, author test counts/verdicts or prior root conclusions.
Return initial observations/criterion coverage; then root supplies final author
receipt/hash to SAME reviewer. No replacement/parallel duplicate or implementation.

No file/Git mutation, builds/tests/scripts generating artifacts, runtime/peers/apps,
network/external communications or child agents. Missing proof is named precisely;
don't execute it or fabricate an issue. Return concise normal Markdown with scoped
accept/accept_with_residual/reject/not_verified, discrete actionable introduced
findings, scenario/impact, exact shortest file/line and owner/recheck. One
::code-comment per actionable inline issue; required title/body/absolute file,
optional lines/priority, visible verified material rule citation. No directives
when no issues; no JSON/schema output. Apply applicable instruction precedence and
deduplicate by location/defect. If review fails, report failure instead of findings.
