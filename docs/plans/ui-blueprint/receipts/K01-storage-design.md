# K01 storage design handoff

Status: concrete documentation proposal ready for D05 consumer/root Git grant.
No production policy, storage implementation or runtime acceptance is claimed.

## Authority and recovered basis

Assigned [finite packet](../packets/K01-storage-design.md) at `56dc93b`, under
approved PLAN.UIB@1 `358c757` and the recorded explicit parallel-work authority.
Only `docs/development/cache.md` and this receipt are writable. Current master;
no nested agents/chats, source edits, dependencies, runtime or copied raw evidence.

Reused the complete current G01/L01/K01-replay closure: AGENTS → spec registry4
UIB.ROUTING@1 → product/decision/acceptance branches → CACHE/EXCHANGE/
PROJECTIONS/LIFECYCLE/MODEL/IDENTITY/PRIVACY/BOUNDARIES, D03/D04/D05,
GOLDEN/PERFORMANCE/GEOMETRY/FORMS/ACTIONS/NATIVE/ROADMAP/C01-EVIDENCE @1
CONTENT; RUST.md and DEV.RUST@2 → D01@1/D07@2 → RUST-BOUNDARIES/REUSE@1.
Read the changed execution-runbook coordination loop and D05 working-memory
status addition against their already-read complete content. No semantic drift:
D05@1 remains current; its limits/enforcement proposals are not accepted policy.
Mode: Restore existing requirements through a design-only implementation handoff.
Excluded: UI/runtime/input, exporters, mobile, projection heuristics and repairs.

## Requirements, observations, proposal

Required: separate cache purposes; full-context isolation; finite retained
revisions/bytes/lifetime; atomic replay, resync after eviction, detach cleanup,
privacy before persistence and no TTL freshness promotion.

Observed owners read: canonical Context/Snapshot/Delta/Expectation/Finding,
replay `e4ecee7`, relevant R03 AccessKit/U1/U2 ledger (reused), actual exhaustive
capacity walker and its focused capacity/overflow tests, plugin session Pending/
Completion transfer and detach, the allocation diagnostic's parser/session phases,
and [working evidence](../../../../tests/bridges/resources/working-memory.md)
at `2a5dfef`. No new profiling or allocation benchmark was run. Diagnostic code
is supporting evidence, not an adopted production API or a universal bound.

Root's later clarification and Integration [D05 draft](D05-policy.md) `8103675`
were incorporated: the reusable allocation-quota worker is a proposal only;
K01 must remain process-independent and cannot promise recoverable serde OOM.
No subprocess policy is selected by Core.

Concrete [proposal](../../../development/cache.md): one host aggregate quota
owner grants nonduplicated allowances to subordinate CacheStores; each exclusively
owns its local sessions/canonical snapshots. Fixed slot vectors, scalar generation
handles, full-key bounded scans and borrowed reads avoid hidden persistent index,
string-key copies and Arc retention. Exact Context/channel/revision keys and
separate future spatial-result/check owners are enumerated. Every slot/key/value/
capacity has a charge/release rule and checked sizing formula, including vacant
slots and historical capture metadata. No counts or byte defaults invented.

Admission moves an already charged Snapshot, validates and plans commit before
any eviction; failure returns/drops the caller-owned value without mutating stored
state. Replay candidate/maps/sets, parser rejection, encoding, framing and returned
Completions need a distinct host reservation/enforcement boundary. Normal replay
errors preserve base; worker death instead requires host resync/completed-channel
preservation and cleanup proof. These are different outcomes, not a local Result
promise for process abort.

## Exact D05 dependencies and next slice

Integration must pin: production canonical sizing owner/API (its proposed
`schema::owned_size` can reuse the existing exhaustive walker); explicit finite
slot/revision/session/aggregate retained limits and lifetime; quota-domain grant
and transfer/release contract; enforced decoder/replay/encoding working memory;
monotonic cleanup owner; and explicit collection-channel partition handoff.
If a worker is chosen, prove completed-channel survival outside the failed owner,
late-result suppression, lost-base resync, other-session isolation and actual
allowance release. Account nested retained/worker ceilings without double-counting
physical ownership. These are finite decisions, not another general survey.

First implementable slice after that policy: session/aggregate canonical Snapshot
retention, full-key borrowed reads, atomic replay admission, scoped eviction,
invalidation/expiry/detach and exact accounting tests. No raw pixel/serialized
history owner. Separate derived relation/check caches remain named subsequent
K01 work; slice A is not full K01. Minimal future code paths and focused acceptance
matrix are in cache.md; no code is authorized by this design-only packet itself.

Five protected review findings and JSON/result-space/external-context wire gaps
remain open. No second validator, public schema or policy weakening is proposed.

## Verification and checkpoint

Changed local links, route/requirement-vs-proposal consistency and scoped
`git diff --check` passed. Documentation-only: no Rust tests/builds/runtime run.
Root granted only these two documentation paths after coordination `d6f914d`.
Commit/push SHA and lease release return in the terminal chat receipt. No source
implementation begins before a separate packet with pinned D05 policy.
