# S01 Stage A — observation session boundary

[uiblueprint-plugin-api](../../crates/plugin-api/src/lib.rs) is a pure state machine
for explicit observation requests. It imports the canonical schema, not platform
SDKs. It performs no IO, clock reads, spawning, collection, polling or subscription.
The host owns the actual transport, unique attachment IDs, permission checks,
bounded acquisition and resource cleanup. This is not D02 live interface proof.

## Flow and responsibilities

1. `ObservationSession::attach` validates descriptor/version and explicit caller
   limits. Every new attachment must receive a fresh session_id from its owner.
2. `begin` accepts a bounded canonical Request frame. It checks the session,
   target/generations, allowed surfaces/scope, plugin and clock domain. Only
   `Observe` is admitted; local analysis and future action execution stay separate.
3. The host runs its authorized bounded collector and feeds canonical
   `ChannelResponse` frames to `receive`, using the returned Ticket sequence.
4. `complete` returns all requested channels; `cancel`, `expire` or `detach`
   return already completed channels plus missing-channel identities.

Every supplied `ClockReading` is a current reading of the parent's one monotonic
domain in milliseconds. Reversed/domain-mismatched readings reject. Source
Observation clocks retain their own domains; the state machine does not infer
cross-process alignment. The caller drives expiration; there is no background
timer or hidden continuation between requests.

`Terminal::Completed` means all requested channels replied, including channel
failures. It is not a whole-platform success or verified user action. AX data
survives a capture failure/timeout. Permission-required capability cannot be
upgraded by merely submitting an observed result. Reply context, channel and
ticket must match; duplicate/unrequested channels reject. Cancellation removes
the pending ticket; a later request with the same request_id gets a new sequence.
Detach is terminal for that object and releases pending state, not user apps.

The crate does not execute actions or prove real cancellation of OS calls.
Action/transition records, including unknown outcome after delivery, belong to the
schema; A01/W02/M02 own real execution. The four existing S01 review findings are
not repaired by introducing this observation-only state machine.

## Bounds are explicit, calibration is pending

`Limits` takes frame bytes, in-flight count and total pending encoded bytes.
Request output bytes bound the sum of accepted response frames; the input request
is charged to the session's pending budget but not its output allowance.
Oversize/over-budget data is rejected before insertion and does not erase already
accepted channels. Responses also respect requested node/depth limits and declared
current freshness. These checks do not prove the collector bounded its acquisition.

`pending_encoded_bytes()` accounts wire sizes, not allocator overhead, parsed Rust
heap, SDK buffers or process RSS. The tests use explicit synthetic limits only.
There are **no chosen production defaults or retained-revision/cache policy** here;
D05-RES must measure and freeze those before live adapters. The caller receives
owned completions and owns their later retention. K01 owns the cache/replay engine.

## Focused checks and remaining proof

[Lifecycle tests](../../crates/plugin-api/tests/lifecycle.rs) cover channel failure
preservation, cancellation and reused request IDs, deadline/detach/late replies,
session/scope/generation/version/clock rejection, malformed/oversize/duplicate
frames, response-budget boundaries, reply correlation and independent session state.
They use authored frames and injected clock readings, not actual Mac/Web backends.

```sh
cargo +1.96.0 check --locked -p uiblueprint-plugin-api --all-targets
cargo +1.96.0 test --locked -p uiblueprint-plugin-api
cargo +1.96.0 clippy --locked -p uiblueprint-plugin-api --all-targets -- -D warnings
```

D02 still requires the same wire exchange through current F01/F02 fixtures with
real channel attribution. Native proof requires a separate finite runtime grant.
Injected failure preservation here does not fix/prove F02's actual simultaneous
capture continuation failure, M01 session isolation or P01 probe invariance.

## Shared D02 caller support

[Common test support](../../tests/bridges/common/README.md) now supplies a reusable
Rust Harness with real parent Instant/Ticket and bounded reader. Native may call it
directly; Node may use the finite `d02_host` example wrapper. stdin remains canonical
channel documents, not a new command protocol. Platform owners retain setup,
collection, injection labeling and owned-process cleanup. A committed support SHA
is required before proof; synthetic support checks are not actual D02 acceptance.
