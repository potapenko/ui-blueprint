# D05 working-memory enforcement

- Node type: leaf; domain: `uib.development.d05-work`; contract: `UIB.D05-WORK@1`.
- Clauses: `UIB.D05-WORK.PROFILE`, `.GUARD`, `.SUPERVISOR`, `.PROOF`.
- Authority: Active / Evolving; implementation/runtime acceptance: none.
- Authority source: ROADMAP/PLAN.UIB@1, accepted designdb629fc and [registration packet](../../../plans/ui-blueprint/packets/D05-runtime-registration.md).
- Read when: host allocation/framing/admission/cleanup or bounded live integration.
- Do not read when: unchanged pure calculations need no runtime boundary.
- Requires: [LIFECYCLE@1](../../product/lifecycle.md), [PRIVACY@1](../../product/privacy.md), [D05-MEMORY@2](d05-memory.md), [D07](d07-reuse.md), [PERFORMANCE@1](../../acceptance/performance.md).
- Protocol/clock/effect owner: [D02@2](d02-boundaries.md); unchanged numeric quality gates: [D06@1](d06-performance.md).

## UIB.D05-WORK.PROFILE — explicit ceilings, not defaults

| Resource | Initial maximum |
| --- | --- |
| Active reusable Rust workers |4; reserve before spawn |
| Worker Rust requested/reserved layouts |64MiB/process; includes startup/shutdown |
| Ordinary work / reserved publication allowance |63MiB / final1MiB |
| Bootstrap before trusted fixed config |1MiB; raise only under reserved host lease |
| Parent owned buffers/control roots |32MiB total; fallible setup before children |
| Inbound operation |One/session,2MiB; caller fills reserved InputLease |
| Native ingress |Two registered helper streams/session,512KiB each, separate slots |
| Canonical output |512KiB/channel,3 channels,2MiB cumulative/request; smaller caller limits govern |
| Completion groups |8 total; charge active/returned leases until actual release |
| Cleanup allowance |1s after terminalization, then quarantine rather than false release |
| Worker threads/helpers |One operation thread+allocation-free watchdog; at most2 registered native helpers/session |

MiB=1,048,576 bytes. Constructors require explicit validated limits/profile; no
promise all admitted byte/node combinations succeed. Parent payload inventory is
8MiB inbound+4MiB native+12MiB completion; actual fixed control/header/identity/
child/spawn/backing inventory fits remaining8MiB or setup refuses. Controls≤4KiB.
Total managed Rust reservation≤256MiB workers+32MiB parent=288MiB. K01 retained
memory is NESTED in worker cap, not added again; encoded parent copies are separate.
Count worker input/DTO/Content/prefix/rejection/error/numeric scratch, validation
clones/maps, replay base/candidate, encoding and surviving Completions through the
actual allocator guard. Unknown overlap never justifies an unproved multiplier.
Reservation differs from usage/RSS. Exclude allocator headers/rounding/internal
bookkeeping, code/OS buffers, stacks, SDK/helper/pixels and user application's memory;
name separate owners, never zero cost. Request/prove supported stack ceilings
(initial target8MiB main/1MiB watchdog) separately. No address-space/RSS-cap claim.

## UIB.D05-WORK.GUARD — before allocation

Only worker executable installs quota GlobalAlloc; existing library unsafe policies
stay unchanged. Small private module forwards valid unchanged pointers/layouts to
System, using allocation-free checked atomics/fixed records. Charge layout.size
before alloc/alloc_zeroed. Realloc reserves FULL new layout while old is charged;
release old on success or new reservation on failure. Dealloc releases after forward.
This covers moving-layout overlap, not System internals. Overflow/underflow is fatal,
never wrapped allowance. No callback panic/unwind, formatting, logging, mutex or
allocator recursion; no promise that infallible std/serde OOM becomes local Result.
Quota exhaustion writes best-effort fixed status to a preopened nonblocking private
descriptor, then immediately terminates process. Distinguish quota, System allocation
failure and generic failure; missing status is not guessed. Private status/exit codes
are not new public CLI exits. Disable owned core dumps before input; raw stderr/
panic payloads are never forwarded, persisted or treated as control messages.
Ordinary work cannot consume the final publication allowance. Reserve reusable
publication buffer before decode/collection; prove bounded streaming/wrapper/error
work there. D02's complete-frame/commit/ACK defines completion before next risky work.

## UIB.D05-WORK.SUPERVISOR

Parent uses fallible fixed-pool creation and allocation-free bounded steady-state
framing/control; no graph/JSON/Value decode, proportional clone/grow or uncharged
formatting. Every Rust control allocation belongs to pool/precharged fixed roots.
Process setup uses bounded SpawnSpec and an owned-process wrapper, not unbounded
argv/environment collection. OS spawn internals remain explicitly opaque category.
Input/result leases, checked lengths/counters and no unbounded queue prevent
untrusted frame sizes from allocating before admission. Typed reuse happens in a
guarded worker; stdout byte streaming requires no parent graph decode. Prove the
closed parent inventory; merely measuring a control allowance is insufficient.
Root retained grant/child partition/reap follow D05-MEMORY@2; publication, nonce,
deadline, capture isolation and CleanupPending/quarantine follow D02@2. No second
cache, parser, daemon/network/async framework or permission expansion is selected.

## UIB.D05-WORK.PROOF — still required before live acceptance

Prove precharge/caps/overflow/zeroing/realloc/concurrent counters/fatal behavior,
including observable allocations rather than optimizer-elided work. Execute hostile
audit families only INSIDE a proven guarded child, including2MiB lower-bound excess,
core0.1/analysis0.2 malformed/tag-buffered/string/numeric inputs with current features.
Bounded invalid_input or quota refusal is truthful; tag order/limits are not weakened.
Force failures in decode/rejection, validation, replay, encoding/admission; compare
all ACKed canonical bytes, refuse partial output, verify leases/grant/capture release.
Prove slot saturation before dispatch, reusable worker, Target B progress during A
stall, cancel/late frames/lost-base resync, fake post-permit unknown outcome/no retry,
parent death/EOF/watchdog and quarantine without user-app cleanup. Web proves
acquisition/transport bounds; Native proves copied strings/pixels/helper/handle/thread
lifetime and capture permission/positive gates. Opaque SDK cost is not a Rust/RSS cap.
F01/F02 positives retain fields/coverage/deadlines; cold/warm IPC costs fit unchanged
D06 gates, never hidden in stage-only timings or substitute degraded results.
D05-WORK-001 registers reviewed mechanism under delegated authority. All allocator/
publication/supervisor/platform/D06 implementation evidence remains open; source
ownership and any narrowly needed OS dependency require separate finite packets.
