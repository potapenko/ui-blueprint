# D05 working allocation and configured-bound evidence

This continues the [owned-capacity study](README.md). No production allocator,
parser, framing, session limits, model, validator or protected review behavior
changed. Inputs are either explicitly synthetic or exact files supplied by the
platform owners. No UI collection ran here. Full D05 remains open.

## Minimal measurement method and safety boundary

`crates/plugin-api/examples/resource_working.rs` is a dedicated single-process
diagnostic. [count_alloc.rs](count_alloc.rs) forwards the standard GlobalAlloc
operations to System and observes allocation-request layouts with allocation-free
atomics. Unsafe code is confined to that diagnostic forwarding boundary: pointers
and layouts are not changed/dereferenced; null failures do not change ownership;
successful realloc updates accounting; deallocation forwards the original layout.
It is never installed in the production library, CLI or another process.

Self-checks verify alloc/zeroed storage, growth accounting and release back to the
baseline. Every valid parsed document's retained allocation delta must equal the
independent exhaustive owned-capacity walk. Rejected inputs must retain zero parsed
object bytes. Measurements are sequential; no unrelated work may run while a peak
is reset. Baselines include already prepared input/report state and are reported
separately. Negative release deltas are intentional, not unsigned wraparound.

This counts Rust-requested live layouts, **not RSS**. Allocator headers/rounding,
fragmentation, internal realloc overlap, OS/C allocations, thread stacks, SDKs and
external pixel/payload_ref data remain excluded. Neither measured peak nor a frame
byte quota is asserted to bound the whole process. No profiler/framework dependency.

## Synthetic configured-bound cases

All generated nodes are authored synthetic records, never duplicated live nodes.
Trees have1/32/160 nodes with1/8/9 node-count depth (root=1), ordinary child edges,
and known/unknown/unsupported/redacted variants. No shared-descendant DAG or P1-R1
hang is constructed. Canonical member order, sorted members and escaped Unicode
deserialize to equal values but can have different allocation behavior.

| Input | Wire bytes | Retained requested heap | Parse peak above prepared-input baseline |
| --- | ---: | ---: | ---: |
| 32-node/depth8 canonical tree | 49,003 | 44,514 | 50,057 |
| Same values, sorted members | 49,003 | 42,486 | 265,208 |
| 160-node/depth9 canonical tree | 239,743 | 235,543 | 263,786 |
| Same values, sorted members | 239,743 | 205,675 | 1,318,009 |
| Valid dense text-list at64KiB, sorted | 65,534 | 501,775 | 1,553,488 |
| Valid dense text-list at128KiB, sorted | 131,072 | 1,026,079 | 3,126,368 |
| Valid dense text-list at512KiB, sorted | 524,288 | 4,197,487 | 12,589,232 |
| Wrong typed dense-array input below64KiB | 65,534 | 0 | 2,619,353 |

Large text, duplicate-property rejection and Unicode variants are also measured.
These are targeted cases, not an exhaustive upper-bound proof for every admitted
or malformed input. No limit was reduced to avoid P1-R1. In the unchanged64KiB
session configuration, the160-node response239,941 bytes rejects before parsing;
the1/32-node trees pass. A65,198-byte dense-list response is accepted under that
same frame cap, retaining500,480 additional bytes during receive and reaching
1,545,756 transient additional bytes. An encoded pending quota is not heap quota.

## Actual incoming/returned phases

| Provided sample/phase | Input bytes | DTO owned bytes (including16-byte root) | Parser peak above input baseline |
| --- | ---: | ---: | ---: |
| Web32-node actual returned Document | 78,241 | 102,043 | 103,403 |
| Same Web content, explicit diagnostic reorder | 78,241 | 67,959 | 376,106 |
| Native76-node exact incoming NDJSON | 337,500 | 286,841 | 1,427,270 |
| Native actual core-reserialized Document | 338,001 | 337,281 | 357,224 |

The Native inputs are distinct phases, not interchangeable byte copies. The Native
sample is partial, actual depth4 zero-based; it is not the160/depth9 ceiling.
The Web sample reaches32 unique source nodes/17 relations but actual depth1 and
partial coverage; maximal strings, depth and graph sharing remain unqualified.
The separate Native nonempty-canary run is not pooled with its main sample.
Reading these files in this probe allocated131,072/524,288-byte input buffers;
serialization allocated the same respective capacities. Those buffers overlap
the DTO while alive and must not be omitted or double-counted in a phase budget.

## Framing and session/process composition

The common reader uses Take(B+1), a new Vec per line, one buffered reader and a
capacity-one queue. At pinned Rust1.96, [read_until](https://github.com/rust-lang/rust/blob/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/io/mod.rs#L2155)
extends slices and [RawVec growth](https://github.com/rust-lang/rust/blob/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/alloc/src/raw_vec/mod.rs#L464)
requests max(double-capacity, required, minimum8 for bytes). Thus a bounded line's
growth request is at most max(2B,8), within this implementation/allocator model.
This is not a portable malloc/RSS guarantee. Chunked-input probes cover exact,
under- and over-limit lines at64/128/512KiB and chunk sizes1/31/8192/65536.
At64/128KiB, observed returned capacities reached126,976/253,952 and oversize peaks
131,072/262,144. The buffered reader was8,192 bytes. Capacity is not line length.

Up to three line Vecs can coexist: consumer, one queued frame and producer frame.
For byte payload reservations that gives6B+8,192 plus control/record storage, while
still excluding allocator internals. Queue/control creation requested672 bytes
on this host; send/receive of a payload-free event added zero, cleanup briefly64.
Thread/runtime/OS stack and collector ownership are separate costs, not zero.

With explicit one-in-flight synthetic sessions, attach retained213 bytes and begin
4,539 (including caller Ticket ownership). Cancel transferred the payload into
Completion while releasing262 bytes; dropping Completion released the actual
channel storage. Dropping idle session released4,453. After20 explicit in-memory
operations, post-cleanup idle allocation did not grow. Zero pending_encoded_bytes
therefore does not mean zero session/process heap. These figures are workload/
toolchain-specific observations, not ABI constants or production defaults.

Process accounting must include all active session owners, pending/returned
Completions, parser working allocations, framed buffers, encoding, thread/runtime
and retained storage; lifetimes determine overlap. Current ObservationSession
has no process-wide registry/quota. Caller ownership must enforce a process session
limit; summing independent per-session encoded counters cannot establish it.

## Exact remaining dependencies, without an imitation cache

K01 currently provides replay, not a retained storage/index owner. Before retention
and eviction charges can be implemented/proved, that owner must define logical
revision/channel grouping, payload/index/sharing representation, outstanding
borrow/Completion ownership and release point. An evicted ID must give
resync_required without claiming UI deletion, and memory cannot be considered
freed while another owner still holds it. A fake cache here would not prove that.

The measured parser cases are not a universal worst-case bound. Production policy
needs either a justified bound for the admitted decoder/input grammar or explicit
working-memory admission/enforcement at its real owner, plus process ownership.
Record the D05 revision before implementing such numbers. P1-R1 bounded-work and
the original four review repairs remain awaiting user authority; memory evidence
does not cure their behavior or justify easier request limits.

## Reproduction and retained evidence

```sh
cargo +1.96.0 run --locked --offline -p uiblueprint-plugin-api --example resource_working
cargo +1.96.0 run --locked --offline -p uiblueprint-plugin-api --example resource_working -- --document /absolute/input.ndjson --document /absolute/retained/channel-0.json
```

No arguments runs the authored synthetic experiment; explicit documents do not
run it again. Every assertion is local to this diagnostic, not live acceptance.
Final reports: application-state `UIBlueprint/development/P1/D05/accounting-798db1e302b5/`
contains synthetic.json and actual.json with compiler/source/input hashes and
phase results. Owner=root; consumers D05/P1/P7; retain through acceptance/discard.
Actual source bytes remain at their platform-owned locations; no raw capture/log
is committed. Earlier reports are preserved as earlier measurement revisions.
