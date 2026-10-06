# D05 policy handoff — awaiting storage ownership reconciliation

Status: finite design in progress under [D05-policy](../packets/D05-policy.md).
This receipt records proposals, not an accepted D05 revision or source changes.
Current normative policy remains UIB.D05@1. No further profiling is proposed.

## Authority and basis

ROADMAP.CONTENT delegates engineering D05 choices under the user-approved
PLAN.UIB@1; packet checkpoint56dc93b supplies this bounded decision assignment.
Current master only. Scope: D05 policy/route metadata and this receipt; no source,
wire/validator, Cargo, fixtures or five protected review repairs.

Recovered AGENTS -> spec registry UIB.ROUTING@1/registry4 -> decision route
D05@1 -> GEOMETRY/PROJECTIONS/LIFECYCLE/CACHE/PERFORMANCE@1 and C01-EVIDENCE@1.
Explicit closure MODEL/IDENTITY/EXCHANGE/PRIVACY/BOUNDARIES/ROADMAP@1 is current;
S01/handoff D01-D06@1/D07@2, RUST.md and DEV.RUST@2 remain current, with their
FORMS/ACTIONS/NATIVE/GOLDEN/PILOTS/RUST-BOUNDARIES/REUSE@1 dependencies.
Contract clauses remain CONTENT@1; no released baseline or semantic drift.
Runbook and finite K01-storage-design packet establish disjoint ownership.
Excluded: product UI, runtime/input, live collection, export internals and repairs.

Required: finite request/admission/storage/lifetime limits, completed-channel
preservation, independent sessions, eviction as resync rather than UI deletion.
Observed: existing Rust schema/session/replay APIs and measured allocation behavior.
Proposed below: quotas and enforcement ownership; measurements alone do not make
them requirements. The normative D05 delta follows the Core ownership handoff.

## Existing evidence is sufficient to choose an enforced ceiling

Saved evidence: owned sizing c6a5df6; actual Web fcf48be/Native51c7809; working
allocations2a5dfef. Exact methods, phases and exclusions are in
[working-memory.md](../../../../tests/bridges/resources/working-memory.md).
The actual Web32-node Document owns102,043 bytes; actual incoming Native76-node
Document owns286,841 bytes and peaks1,427,270 during parse above prepared input.
Both remain partial. Authored512KiB dense-list input owns4,197,487 heap bytes and
peaks12,589,232; an invalid under64KiB input peaks2,619,353 before releasing it.
These establish useful admission examples, not a universal amplification bound.

Framing has a code-based requested-layout reservation6B+8,192 for three possible
line buffers and the reader, before control/record storage. Encoded quotas do not
bound parser/replay heap. Pure replay additionally clones a candidate and builds
BTree maps/sets before publication. Existing safe ownership counting describes
retained DTO capacities; it does not intercept allocations before they happen.

## Decoder/host enforcement proposal

Reject a measured multiplier, post-parse size check, periodic RSS sampling or a
timeout as the sole memory enforcement. No universal decoder bound is established.
Retained storage needs its own checked layout ledger; transient decode/replay needs
allocation enforcement at a real execution owner before allocating.

The concrete candidate is an owned reusable Rust session-worker process inside
the same statically linked executable, with a hard allocation-request quota.
It is created per attached session, not per measurement, and is not a daemon or
second analytics implementation. The existing canonical parser/validator/engine
run there. Only that executable installs the narrow quota allocator; library
unsafe policy remains unchanged. Every System allocation is reserved before
forwarding; successful deallocation returns its charge. Realloc conservatively
reserves the full new layout while the old layout remains charged, then releases
the old charge on success or new reservation on failure. All arithmetic is checked.
This caps Rust-requested live/reserved layouts, not System metadata, RSS or OS/SDK
allocations. Allocator callbacks cannot allocate, lock, format or unwind.

Why isolation is material: [GlobalAlloc safety](https://doc.rust-lang.org/core/alloc/trait.GlobalAlloc.html)
forbids unwinding from its callbacks; ordinary [allocation failure](https://doc.rust-lang.org/std/alloc/fn.handle_alloc_error.html)
in a std binary normally aborts. A quota adapter cannot promise a recoverable
parser Result by panicking or returning null to arbitrary std/serde allocations.
The diagnostic counting allocator is evidence only, not the enforcement code.

An external session host must own deadlines, admission and worker cleanup. It must
receive and retain each validated/redacted completed channel before further channel
work, using bounded encoded buffers without reparsing an untrusted full graph in
the supervisor. On worker failure it preserves those completed channels, rejects
late success, marks lost cache bases resync_required and leaves other sessions
usable. It must not claim the failed channel returned a successful empty snapshot.
An allocation-free fixed control status must distinguish an explicit quota refusal
from an unexplained child failure; unavailable status is a generic worker failure.
Owned-child cleanup and suppression of payload dumps require focused proof.

This is a concrete implementation direction, not yet adopted packaging. It requires
reconciliation with D02 session ownership and Core's retained-store design. It is
not permission to add a transport framework, daemon, new wire schema or fallback
parser. If the owner cannot preserve completed channels across that boundary,
this proposal must be rejected before code, not weakened into post-parse checking.

## Proposed initial finite ceilings — not defaults or worst-case estimates

These are engineering ceilings for the first controlled implementation profile;
callers supply explicit values at or below them. A correctly rejected valid large
input is a resource refusal, not evidence that every frame/node limit combination
is supportable. Existing Web64KiB/deadline scenario parameters remain unchanged.

| Resource | Proposal | Rationale and actual enforcement owner |
| --- | --- | --- |
| Response frame |512KiB maximum | Accommodates the supplied Native337,500B frame; reader checks before parse |
| In-flight requests / frame queue |1 per session /1 waiting frame | Existing bounded D02 shape; host admission, no hidden backlog |
| Completed/pending encoded responses |2MiB per session request | Three512KiB channels fit; host counts actual cumulative wire bytes |
| Retained snapshots and derived cache storage |16MiB per session,64MiB aggregate | Multiple observed/synthetic payloads fit; Core must include containers/keys/indexes and preserve separate cache purposes |
| Rust worker allocation quota |64MiB per active session worker | Explicit ceiling allowing retained storage plus observed transient costs; no universal success multiplier |
| Active session workers |4 | At least two independent sessions; host reserves complete quotas before attach, maximum256MiB worker quota reservation |

The host's own memory, worker startup, control records, encoding, native helper,
pixel buffers and external payload owners are not free or silently included in
the256MiB reservation. Their budgets must be owned/enforced before the full live
D05 gate closes. The profile is not an RSS promise for a256MiB process. Exact
retention count/lifetime and aggregate-host allocation ceilings await the actual
owner layout; neither is inferred from frame size or helper RSS.

## Exact dependency and next finite implementation boundary

Core handoff must fix: full stream key/channel grouping, owner of each Snapshot,
entry/key/index/container capacity, separate derived-result storage, revision and
age semantics, outstanding borrowers, and where decode/replay candidates reside
relative to retained entries. In particular, determine whether one cache owner
manages multiple sessions or session stores are subordinate to a single quota
owner; never count shared payload twice or consider it freed on ID removal alone.

After that handoff, Integration can decide the parameterized K01 storage policy
and publish the D05 semantic delta before source work. Proposed source boundary:
canonical safe sizing in crates/schema/src/owned_size.rs plus its lib export and
existing sizing consumer imports/tests; actual storage in crates/engine/src/cache.rs
plus export, cache tests and its existing development handoff. No wire/validator
change, duplicate graph or new dependency is needed for that slice.

The worker/supervisor allocation enforcement is a separate named host packet,
not part of K01 retained storage. Its owner/API integration must be decided before
source dispatch; likely executable-private CLI modules plus focused subprocess
tests, without installing an allocator in shared libraries. Full live W01/M01
acceptance remains unavailable until framing, decoder/replay working allocations,
host aggregate quotas and completed-channel preservation have actual proof.

Required K01 checks: exact byte/count boundaries and overflow; reserve-capacity
charge; rejected admission leaves state intact; eviction/age/detach releases actual
ownership; missing base resync; cross-session accounting/isolation; unknown and
redacted/empty values survive without promotion. Actual allocation/control overhead
is checked against the implemented owner, not a simulated cache or another survey.
Host checks: forced quota exhaustion during decode/replay/encoding, conservative
realloc charge and release, fixed bounded failure signaling, completed-channel
survival, late-result suppression, independent session progress and owned cleanup.

No new runtime/build claims. Documentation checks and checkpoint follow completion
of this finite reconciliation under root's short Git grant. Five protected defects,
result/schema cutover and unrelated product contracts stay unchanged.
