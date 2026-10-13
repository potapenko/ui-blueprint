# USE-WEB-LIMIT1 — narrow Register canonical-output refusal

Read-only diagnosis, 2026-10-14. Authority: root's finite USE-WEB-LIMIT1 packet;
only this receipt is writable. No production/spec/budget changes, runtime, browser,
Observe, trace, tests, build or app-source access. Shared spec ownership remains
with AUTO-DETAIL2. Current master/exact-path commit; unrelated files untouched.

## Basis and exact attribution

Traversal: active AGENTS and retained implementation/product-truth/QA/operational
routes → registry57 → product → WEB-FIRST-USE7/OBSERVE-DIAGNOSTICS3 and the existing
CLI20/IDENTITY/MODEL/EXCHANGE2/PRIVACY/LIFECYCLE/GEOMETRY2/D02/D05/D07 closure.
Registry55→57 changes concern export/installation; the selected Web leaves have
not changed since the fully read basis. No export/Native sibling investigation.

Inputs: `/tmp/uib-b28-yl69v8s8/narrow-register-handoff.json`, original
`register/commands.json`, `register/observe.err`, empty `register/observation.json`.
Actual bundle files were hashed and match supplied helper3bf0b562 and Rust
`abd88783221412f8e3f9d1da1f3c9a2b4733b665` pins. The command has no binary/profile
overrides. Closed historical endpoint was not reused. Source29/depth5 and nearby
successful response sizes are context only, never exhaustion proof.

Observed fixed record: exit2, stdout0, operation observe, terminal failed,
terminal_error resource_limit, requested1/committed0/missing1/incomplete0,
worker_cleanup confirmed, stage collect, cause collector_limit, code0,
selection_visited unavailable, remote_cleanup not_required, send_progress not_queued.

**Conclusion for this exact pinned configuration: the refusal is the collector's
canonical JSON counting guard in observe.rs::bounded_document, cap131072 bytes.**
It is not the64KiB CDP-reply sum and not the old109-vs100 method preflight. This is
an elimination proof from configuration, control flow, allocator semantics and
validated diagnostic ownership; code0 by itself remains unspecified.

## Guard map and exclusions (frozen abd8878 line numbers)

| Path / possible Limit | Reconciliation with this event |
| --- | --- |
| bootstrap.rs65–96 → io.rs295–325: rooted admission | Fields layout/hit/visible require no AX. min(nodes32,request32)=32;32≤32 and correct Observe operation. Node arithmetic cannot overflow. |
| observe.rs45–58,77–98: conversion/sequence/method preflight |131072 fits64-bit usize; fresh caller-owned worker's first Observe cannot overflow its sequence.3×32+13=109 equals granted109; the predicate is strictly greater-than. |
| acquire.rs63 → io.rs126–150: pre-group document verification | Fresh per-observe budget starts0. Exactly three fixed sends before group allocation. Each accepted CDP wire reply is≤8192; at most24576 total, well below65536. Methods≤3<109; remaining allowance stays≥49152 before the third call. No counter overflow/allowance shortage. Oversize CDP admission has a distinct CDP/transport cause. |
| acquire.rs91–118: all collect_nodes failures, including selection/handles/late method or reply limits | These errors are always decorated with remote_cleanup Released or Unconfirmed. A failed release itself becomes CleanupUnconfirmed. They cannot produce this Limit + NotRequired combination. NotRequired is not a claim that no earlier CDP calls occurred. |
| acquire.rs83–89,517–522 and observe.rs222–223: fallible Vec reservations outside that wrapper | Counts are bounded≤32 (AX0). Capacity-overflow is excluded for these finite sized types. Actual allocation quota/system failures in the shipping worker do not return TryReserveError: GuardedAllocator fatally exits instead, so they cannot produce this typed CollectorLimit completion. |
| acquire.rs573: canonical Snapshot validation | Failure maps to Malformed, not Limit. |
| observe.rs122–125 → bounded_document190–211 | Remains reachable after successful collection/release/normalization. It counts the complete ChannelResponse JSON against request131072 minus prior canonical bytes0. The fixed Count writer rejects the first write whose cumulative bytes exceed cap. |
| worker_web.rs316–354,436–441: later encoding/receive/publication | A callback error takes precedence as Host diagnostic with its actual Encode/Receive/Publish stage. This event instead carries collector_limit at the worker's fixed Collect stage. Thus Collect can include the collector's internal count serialization, without being the later Host encoder. |

CDP reply-size exclusion uses cdp/session.rs381–387 (accepted String capacity≤
max_message_bytes8192) and Reply::wire's original complete frame. Allocator exclusion
uses the worker global allocator in host/src/main.rs plus quota_allocator.rs reserve,
allocate_with/reallocate_with and fatal190–212: quota/null System allocation takes a
nonreturning fixed Fatal control path, never a returned collector error. Scope stays
at these directly necessary dependencies; no general allocator/transport audit.

The Document model uses derived Serialize over fixed records/enums/string IDs and
sequences, without custom fallible serializers or non-string map-key data. Snapshot
validation already succeeded. The pinned serde_json1.0.151 scalar/string/record
serializer forwards failures from its writer. Count::flush succeeds; its sole write
error is checked cumulative count over cap. Its small accepted prefix and bounded
worker storage rule out u64 overflow. Therefore this path identifies the byte-cap
predicate, not merely a generic serializer failure. No partial JSON was published.

All source reads used git show abd8878. Relevant current owners are unchanged from
that pin. No test result is claimed for this source-only reasoning.

## Missing facts and next decision

The failed serialized ChannelResponse exceeds131072 bytes (JSON body, before any
caller NDJSON newline). Its exact full size, first rejected chunk size, accepted
prefix length and actual selected node count are not retained. The counter stops at
its first rejection; neither29 static Elements nor Email124206bytes recovers them.
No animation/transient-node cause, required new allowance or32-visible-node guarantee
is inferred. No source graph should be reconstructed from the neighboring snapshots.

No new runtime or diagnostic implementation is needed to identify THIS guard. Root
can now reconcile the explicit canonical output profile with the whole-component
requirement; this receipt does not authorize changing that allowance or dropping data.
If a later sizing decision requires more numbers, the minimum separately authorized
one-run diagnostic is ONLY Count::write's existing failure: fixed site canonical_output,
accepted_bytes_before_write, incoming_chunk_bytes, attempted_total and cap. Record
these scalars before returning the SAME error, without serializing beyond the cap,
logging payloads/IDs/URLs or publishing a partial Snapshot. It gives an exact rejected
prefix/lower bound, not a full-size measurement. Do not add a broad Limit trace or
blindly repeat the current command. Existing DiagnosticRecord rejects nonzero code
and nonselection count for CollectorLimit, so any transport extension would itself
need an explicit narrow Host/diagnostic scope; no such change is made or assumed here.

Original evidence/images/shared bundles remain untouched; no task transient files or
browser/collector runtime processes were created. Only this authored receipt is saved. Immediate consumer is
root's bounded output-profile/fix decision. No canonical acquisition or corpus case
has been added by this diagnosis.
