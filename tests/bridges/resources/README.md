# D05 owned-model sizing — partial evidence

This diagnostic measures materialized canonical Rust records, not JSON bytes,
another helper's RSS, or a substitute cache/engine. Model/validator behavior and
the four protected P2 findings remain unchanged.
The later static [P1-R1 depth-traversal finding](../../../docs/plans/ui-blueprint/receipts/P1-review.md#new-finding-p1-r1)
also remains awaiting authority: five review defects in total, with the original
four tracked separately in [the earlier review](../../../docs/plans/ui-blueprint/receipts/S01-user-review.md).
This sizer counts owned containers; it does not expand graph paths or repair/prove
the receiver's bounded-work behavior. No reported hang was executed here.

[owned_memory.rs](owned_memory.rs) counts inline layout once, then owned heap:
String capacity bytes, Vec capacity×element layout plus live nested allocations,
Box payload layout and descendants, and Option/array/enum contents. Allocation
blocks and unused capacity are reported separately. Checked sums/products reject
overflow. Zero-sized Vec elements allocate no storage even with usize::MAX capacity.
Exhaustive destructures/matches force a compile failure on canonical field/variant
changes. No unsafe code, global allocator replacement or new dependency.

Excluded: allocator headers/rounding, fragmentation, parser/serializer scratch,
input/output buffers, session/harness bookkeeping, OS/SDK memory and external
image/payload_ref data. These exclusions prevent calling this a total process
cap or completed D05-RES. Capacity is observed after canonical deserialization.

## Reproduce

```sh
cargo +1.96.0 run --locked --offline -p uiblueprint-schema --example measure_owned -- --max-input-bytes 4194304 --p1 fixtures/golden --document /absolute/returned/channel-0.json
cargo +1.96.0 test --locked --offline -p uiblueprint-schema --test owned_memory
```

Repeat --document for more canonical files. --web-report accepts the earlier
collector-report format, labeled web_b03_narrow, with a reconstructed wrapper.
Prefer exact returned wire bytes: JSON member order can change deserializer
capacity despite equivalent values. The explicit4MiB file-read ceiling is a
diagnostic limit, not a proposed production frame/memory default.

Three tests prove reserve-only changes increase storage without changing wire
data/node count; nested boxes/ZST do not double-count inline or invent allocations;
and impossible size sums reject rather than wrap. Core model source is untouched.

## Observed Rust1.96.0 / aarch64-apple-darwin results

59 valid P1 documents measured;67 intentionally invalid records excluded from
retained-state sizing. Largest whole validation bundle, GOLDEN01:18,002 bytes
(16 inline +17,986 heap;214 allocations;6,360 unused capacity). Its multiple
snapshots are not one cached graph revision. Largest P1 Snapshot is
GRAPH-MANY-TO-MANY:8,731 bytes,4 nodes/3 relations.

Actual returned Documents from the Web/Native D02 receipts:

| Source/state | Nodes / relations | Snapshot owned bytes | Allocation blocks | Unused capacity |
| --- | --- | ---: | ---: | ---: |
| Web popup-open | 12 / 7 | 41,052 | 451 | 13,668 |
| Web overlay-on | 14 / 8 | 46,184 | 518 | 14,788 |
| Web overlay-off | 12 / 7 | 41,053 | 451 | 13,668 |
| Native AX selected sample | 1 / 0 | 6,694 | 52 | 2,484 |
| Native capture metadata | 0 / 0 | 5,768 | 39 | 2,604 |

All observations are partial. Largest channel Document is Web overlay-on:46,532
owned bytes versus36,169 wire bytes. Native Documents are7,040 and6,114 owned
bytes versus4,597 and3,246 wire bytes; no screenshot buffer is counted.
These are narrow B03 and one-node F02 samples, not largest complete platform
graphs or worst permitted32/160-node responses. The earlier reconstructed Web
cohort gave31,407 Snapshot bytes for overlay-on; its missing deserializer slack
makes it unsuitable as the selected actual-return retention basis.

Minimum report:
`/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/P1/D05/sizing-e02cdb69af34/owned-memory.json`.
It retains compiler/source/input hashes and per-sample numbers without UI values.
Earlier `owned-8d7ef12e4f0a` and `returned-fb7be08591ff` reports remain historical.
Owner=root; consumers D05/P1/P7; retain through acceptance or explicit discard.
Original platform evidence is not copied or changed. No collection ran here.

## Bounds preparation — not selected policy or defaults

Two retained revisions are a minimal before/after candidate; one in-flight request
per session matches the finite path; at least two independent sessions are needed
for the slow-target isolation scenario. These are design candidates, not measured
concurrency/support promises. No byte cap is derived from helper RSS or only the
smallest examples. No retention/admission/cache policy is implemented by this tool.

As measured payload floors, two largest observed Web channel Documents alone
would occupy93,064 bytes; two current Native AX+capture-metadata pairs would
occupy26,308 bytes. These omit processing buffers/bookkeeping/pixels and cannot
serve as caps. In particular, a65,536-byte wire allowance is not the same thing
as retaining two normalized revisions within65,536 bytes of Rust memory.

Before final selection: obtain largest relevant normalized F01/F02 examples under
the declared field/node/depth scopes, or explicitly qualify their missing coverage;
measure parser/framing working allocations and session bookkeeping; exercise worst
permitted capacities. Session budget must cover retained model bytes plus pending
channels and owned IO working buffers. Process budget must additionally bound
session count/shared allocations. External pixels and helper processes need their
own accounted limits. K01 owns real cache eviction/resync; no imitation cache here.

Record a justified D05 revision before numeric policy implementation. Full D05
remains waiting_evidence, including largest/configured-bound and transient-memory
coverage; these measured payload capacities alone do not close the gate.

The later [working-allocation study](working-memory.md) adds explicitly synthetic
configured-bound inputs, a diagnostic-only System allocator counter, framing and
session ownership phases, and the larger actual32-node Web/76-node Native samples.
Its unsafe forwarding is isolated to that executable, not this safe ownership
walker or production libraries. Parser peaks/encoding order explain why the
earlier small DTO figures cannot become process caps. All remaining gaps and
K01 storage-owner dependencies are recorded there; production policy is still open.
