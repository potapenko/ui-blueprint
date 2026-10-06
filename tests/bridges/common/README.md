# D02 common Rust test support

Integration owns this directory and the `d02_host` example. Web/Native own their
collector/orchestration directories. This is finite test support, not a new product
transport, schema, service or mandatory executable gate. No platform SDK/dependency.
Canonical model and ObservationSession behavior are unchanged; four P2 remain open.

## Rust caller contract

Include [session_support.rs](session_support.rs) by path from the platform harness.
It imports the existing schema/plugin-api crates. The caller supplies canonical
`SessionDescriptor`, `Request` and explicit plugin-api `Limits`:

```rust,ignore
let mut harness = Harness::new(session, request, limits)?;
let ticket = harness.ticket(); // actual ObservationSession::begin result
// Only now invoke collection; send request + ticket.sequence to the collector.
harness.receive(&canonical_channel_document_bytes)?;
let completion = harness.complete()?;
let retained = harness.retained_documents(completion);
```

`Harness` owns a real parent `Instant`; caller does not author clock readings.
It calls the existing attach/begin/receive/complete/cancel/expire/detach methods.
`remaining()` supplies bounded IO wait from the same admission reading/deadline.
`cancel()`/`expire()` return completed channels; `detach()` drains this one-request
attachment. `retained_documents()` materializes those returned values as canonical
ChannelResponse Documents for equality/evidence/sizing, without a second DTO.
There is no second session state machine and no cache/geometry engine.

`read_frame()` bounds one newline-delimited JSON frame, including newline in the
cap; final nonempty EOF-terminated JSON is allowed. Oversize stops reading, without
unbounded draining. `pump_frames()` owns one reader and a capacity-one queue; its
frame-count bound stops unbounded input. For child stdout, the platform caller
closes/reaps its owned producer before joining the reader. A Rust reader thread
cannot interrupt an arbitrary OS read itself. Helper/app/browser cleanup stays
with the platform owner; no user application is terminated by common support.

## Optional Node-facing executable

```sh
cargo +1.96.0 build --locked --offline -p uiblueprint-plugin-api --example d02_host
target/debug/examples/d02_host SESSION_JSON REQUEST_JSON FRAME_BYTES PENDING_BYTES MODE LATE_MS MAX_FRAMES RETAIN_DIR_OR_DASH
```

Eight positional arguments are explicit test configuration. Session/request files
are canonical Documents with artifact kinds `session` and `request`. The request
must use Observe; its clock_domain names this host's parent monotonic domain.
The platform performs fixture setup/address resolution before preparing these
files, but does not collect requested UI fields until the **ticket** receipt.

stdin accepts **only canonical ChannelResponse Documents**, one compact JSON line
each. No JSON commands are accepted. stdout emits bounded proof receipts:
`ticket` → frame accepted/rejected → `terminal`; fault modes add `late_probe`.
Ticket receipt includes actual request_id/session_id/sequence and parent clock
domain/elapsed milliseconds. Feed those IDs/sequence into the collector; never
substitute an authored Ticket. Source Observation clock domains remain separate.
No UI labels/values are echoed in receipts. stderr does not carry source payloads.

Modes: `complete`, `cancel-after-first`, `detach-after-first`, `expire-after-first`.
Fault modes use real API transitions after the first admitted channel and label
the control as injected. Expire waits for the actual parent deadline. After its
terminal receipt, send one **canonical, correctly correlated** injected late frame
within LATE_MS; it must reject. Invalid/missing late input fails the proof instead
of masquerading as late-frame rejection. Caller labels whether source data were
live, recorded or synthetic; injected control does not make fake data live.

RETAIN_DIR must not exist (parent must exist), or use `-` for no retained files.
The wrapper writes only new `channel-N.json` canonical documents from actual core
Completion data. Compare these with submitted normalized data and retain them
only under the platform packet's application-state/evidence policy. Completed
does not mean every channel succeeded. A valid failed channel remains an outcome.

Host exits:0 intended test transition complete;2 rejected/incomplete proof such
as EOF/timeout/oversize or missing late probe;1 configuration/IO/internal failure.
These are test-host exits, distinct from public validator0/2/1 record validity.
Caller must enforce a whole-process watchdog and clean up its own collector.
The wrapper launches no process/app; terminal process exit releases its own stdin
reader. It never owns or kills an external collector/fixture.

## Literal test limits and checks

Synthetic checks use65,536 frame bytes,131,072 pending encoded bytes, one in-flight
request,8 input frames and1,000ms late grace. These reuse bounded small-fixture
test sizes and allow three channels plus injected negatives. They are not D05
memory defaults. Actual platform proof must pass explicit limits and record them.
The wrapper additionally refuses request deadlines>30s or late grace>5s as a test
safety ceiling; it does not replace D05's shorter request budgets/latency gates.
Queue accounting is not parsed-heap/process memory calibration.

```sh
cargo +1.96.0 test --locked --offline -p uiblueprint-plugin-api --test common_support
python3 tests/bridges/common/check_host.py --host target/debug/examples/d02_host
```

Three Rust checks and eight synthetic executable scenarios cover real Ticket/clock,
framing bounds, retained-data equality, complete/cancel/detach/expiry/late handling,
wrong request/response version, malformed and oversized frames. No live platform
proof follows from them. Consume only the committed support SHA supplied by root.
