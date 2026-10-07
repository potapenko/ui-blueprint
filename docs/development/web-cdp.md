# Bounded sequential CDP client

`uiblueprint-web::cdp` owns request correlation over the accepted
[transport](web-transport.md). It supplies a callable library boundary for the
subsequent scoped W01 collector inside H01's guarded worker, not a live collector.
Authority: [W01 packet](../plans/ui-blueprint/packets/W01-cdp-session.md),
D02@2/D04@1/D05@3/D07@5. No specification, upstream source or normalized graph copied.

## Consumer flow

Install the transport log boundary, connect an explicitly authorized numeric-loopback
endpoint, then move that transport and caller-established `Binding` into `Client::new`.
`Binding` contains canonical target ID/generation and optional exact CDP session ID.
A missing session means an unflattened/direct connection; it is not a wildcard.
One connection serves one binding. No target discovery, attach, rebinding or multiplexing.
This layer cannot establish target authority or document/node freshness from a reply.

```rust,ignore
let mut client = cdp::Client::new(transport, binding, explicit_limits)?;
let cancel = client.cancellation(); // obtain before borrowing the client
let pending = client.prepare("Runtime.evaluate", &method_params, original_budget)?;
let ticket = pending.ticket().clone(); // provenance, never dispatch authority
let reply = pending.run()?;
match reply.kind() {
    cdp::ReplyKind::Result => method_decoder(reply.wire()),
    cdp::ReplyKind::Error { code } => handle_protocol_error(code),
}
while let Some(event) = client.pop_event() {
    consume_invalidation(event.method(), event.wire());
}
if let Some(loss) = client.take_event_loss() {
    require_resync_for_next_explicit_operation(loss.generation);
}
```

This is a composition outline, not a compiled host example or authorization for
Runtime.evaluate. Methods/parameters, allowlisting, redaction, decoding and effect
permits belong to their trusted caller. Parameters must serialize as an object;
empty arguments are `{}`, never null. The serializer is trusted in-process code.

`prepare` reserves a result slot/bytes, encodes under a finite cap and registers a
ticket before IO. `Pending` exclusively borrows the client: one outstanding request.
Other clients have independent sockets, quotas and progress. Process-wide checked
scalar counters allocate IDs 1..=i32::MAX and connection epochs without reset/wrap;
there is no global dispatch lock. Tickets are private, not portable across processes.
`owns_ticket` checks origin even after detach; it does not attest active/live state.

`run` calls send once, then resumes pending flush under the SAME transport Operation,
absolute deadline, read/write/work budgets. No retry/reconnect/re-enqueue. Every reply
must match the active ID and exact optional session. Wrong, unknown, duplicate or
old-epoch correlation closes this connection. Completed Reply owners survive it.
`cancel`, unfinished Pending drop and `detach` stop future dispatch; cancellation
handles also interrupt transport IO. No borrowed Pending survives client teardown.
Timeout/cancellation suppress late completion, preserving transport SendProgress.
Flushed/Result never means application success or an external action's verified effect.

## Bounded ownership and raw data

| Limit | Actual owner / admission |
| --- | --- |
| max_request_bytes | One preallocated bounded serde writer/String per Pending; overflow refuses before send |
| max_message_bytes | Incoming String **capacity** ceiling; full allowance reserved for each pending/returned Reply |
| max_results / result_bytes | Shared per-client slot/byte pool; Reply drop releases, client detach/drop does not release surviving Reply |
| max_metadata_bytes | Incoming decoded names/session and retained method byte ceiling; binding IDs also obey canonical 1..256 characters |
| max_events / event_bytes | Fixed queue slots plus caller-held WireEvents; reserve wire capacity + metadata ceiling before retaining an event |

All limits are explicit positive values, with checked arithmetic/fallible scaled
allocation. They are configurable library limits, not a selected production profile.
Event shortage drops that event and increments a checked persistent loss generation;
`take_event_loss` clears only the unreported count. Reply includes loss generation.
Popping an event does not release its allowance. No overflow triggers automatic reads,
subscription, collection or resync. Events are only received while a request runs.

Serde visitors inspect shallow envelope metadata and skip payloads via IgnoredAny.
The original full wire String is moved into Reply/WireEvent, with borrowed getters;
no persistent Value or payload clone. Reject duplicate/unknown envelope fields,
ambiguous result/error, missing/mistyped fields and floating/string/out-of-range IDs.
Result and event params must be objects. Remote error code/message types are checked;
message/data remain raw private payload, never diagnostic text. ID-less errors fail
as UncorrelatedError; they cannot complete a ticket. Escaped metadata uses bounded
parser staging; event retention reserves first, then moves/copies the method once.

These pools bound retained wire/method ownership, **not all allocations/RSS**. Fixed
client/Arc/Mutex/ticket/queue roots, transient serde/numeric/escape scratch, transport
buffers, encoding overlap, caller copies, stacks and OS buffers are additional costs.
H01/D05 must count/enforce worker allocations; no universal decoder multiplier or
infallible-OOM recovery is claimed. No host publication/ACK or cache grant implemented.
Binding/Reply/Event/Ticket/Client Debug and Failure expose only bounded codes/counts;
raw getters are explicit data access and must not be logged. No logger replacement.

## Evidence and remaining boundary

[Receipt](../plans/ui-blueprint/receipts/W01-cdp-session.md) pins 19 author tests,
inputs, commands and source evidence. Owned loopback peers plus CDP IO fakes only;
transport's unaffected 17-test proof is reused. This is not independent acceptance.
Next: independent correlation/privacy review, guarded-worker composition, scoped
method decoders/collector and real browser identity/coverage/D06 qualification.
