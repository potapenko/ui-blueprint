# W01 transport — tungstenite 0.30.0 decision input

Recommendation: **adopt the named library as a codec, conditionally on the adapter
boundaries below**. Reject its convenience connector and default configuration for
W01. No alternate framework is needed by the inspected mechanism. This is a D07
source/API/license handoff, not dependency installation, runtime qualification,
D05 closure or permission to launch the live adapter. Root/Integration owns the
D07 revision, exact manifest/lock adoption and resource-policy prerequisites.

## Authority, basis and inspected identity

[W01-transport packet](../plans/ui-blueprint/packets/W01-transport.md), checkpoint
e5ed9c0, under approved PLAN.UIB@1/ROADMAP engineering authority. Traversal reused:
AGENTS → ROUTING@1 registry4 → product/decisions → D02@1/D07@2 and complete
current dependencies (D01/D03/D05, C01-EVIDENCE, DEV.RUST@2/RUST.md; normative
BOUNDARIES, EXCHANGE, LIFECYCLE, NATIVE, MODEL, IDENTITY, GEOMETRY, PROJECTIONS,
PRIVACY, FORMS, ACTIONS, CACHE, GOLDEN, ROADMAP, PERFORMANCE, RUST-BOUNDARIES,
REUSE and Web PILOTS CONTENT@1). Relevant full R01/F01/D02 context reused;
current D07 read. No broad upstream reread or new product contract.

Specified: Rust-owned scoped CDP, request-owned deadlines/resources, no periodic
recollection, preserved identity/partial results/privacy. Observed: exact upstream
source behavior below. Proposed: narrow adoption/integration conditions and future
checks; no numerical memory caps or product defaults selected here.

- Crate [0.30.0 metadata](https://crates.io/api/v1/crates/tungstenite/0.30.0), checked
  2026-10-06: not yanked, edition 2021, declared rust-version 1.85. Project pin 1.96.0
  satisfies that declared minimum; no compile/runtime or resolved transitive-MSRV
  claim was made in this packet.
- [Exact crate archive](https://static.crates.io/crates/tungstenite/tungstenite-0.30.0.crate)
  SHA-256 `e48ac77174b19c110a50ab2128b24215ac9cb40e0e12e093fb602d175c569d22`,
  recomputed and matched registry checksum.
- Archive `.cargo_vcs_info.json` pins upstream
  [`7f4aeaf0944992c5664c8fb8e0d54577c7e18020`](https://github.com/snapview/tungstenite-rs/tree/7f4aeaf0944992c5664c8fb8e0d54577c7e18020).
  Repository integration tests were retrieved at this same commit because crate
  include rules omit tests/. All relevant tests were **read, not executed**.

## Source ledger and license

| Selected source unit, under the pinned upstream | Mechanism actually inspected / nearest read checks |
| --- | --- |
| `Cargo.toml`, `Cargo.toml.orig`, `.cargo_vcs_info.json`, LICENSE-MIT/APACHE | Version/features/dependency intent and redistribution terms; no source copied |
| `src/client.rs`: connect/connect_with_config/connect_to_some, client/client_with_config, IntoClientRequest | Convenience path does DNS and blocking TcpStream::connect, optionally follows redirects; custom Read+Write stream path avoids those owners |
| `src/handshake/{mod,machine,client,headers}.rs`, `src/buffer.rs` | MidHandshake resumption, bounded hardcoded attack checks, parser/response validation, request serialization and tail buffer; inline formatting/header/buffer tests; `tests/handshake.rs` subprotocol cases |
| `src/protocol/mod.rs`: full config/WebSocket/Context read/write/flush/close paths | Buffer admission, automatic control reply slot, fragmentation/close states; inline receive_messages/size_limiting_text_fragmented/size_limiting_binary |
| `src/protocol/frame/mod.rs`; frame.rs header parse/length/format units | Length check before payload reserve, partial read/write buffer preservation; read_frames/from_partially_read/write_frames/size_limit_hit/overflow tests |
| `src/protocol/message.rs`: StringCollector/IncompleteMessage; `src/error.rs` error/capacity vocabulary | Cumulative fragmented-message limit, UTF-8 handling and payload-bearing errors |
| `tests/write.rs`, `auto_pong_flush.rs`, `no_send_after_close.rs`, `receive_after_init_close.rs`, `connection_reset.rs`, `wss_fails_when_no_tls.rs` | Write/flush accounting, pong WouldBlock recovery, closing/read semantics, reset and no-TLS rejection. connection_reset has TLS feature gating despite testing ws; not evidence the proposed feature set ran those tests |

License: MIT OR Apache-2.0; prefer the MIT option for the unmodified dependency
inventory. LICENSE-MIT names Alexey Galakhov 2017 and Jason Housley 2016 and requires
its copyright/permission text to accompany copies/substantial portions. Preserve
that file in distribution notices. Apache alternative includes its own notice/
change-marking terms. No NOTICE found in the exact crate archive or pinned upstream
tree. Selected source/header scan found no additional per-file license notice.
No borrowed fragments, assets, models or data are added to this repository.
This is not a transitive license audit; Integration must inspect actual locked
packages/features/notices before shipment, as D07 already requires.

Proposed manifest intent, for Integration to apply later:
`tungstenite = { version = "=0.30.0", default-features = false, features = ["handshake"] }`.
This enables base bytes/log/rand/thiserror and handshake data-encoding/http/httparse/
sha1 dependencies. No `url`, native-tls, rustls, native roots, vendored TLS or async
runtime feature. String/`http::Uri` request conversion is already available without
`url`. Declared semver ranges are not a resolved dependency inventory; this task
did not run Cargo resolution or install packages. Feature unification must be
checked when Integration creates the actual lock/feature graph.

## Concrete integration contract

1. Accept an already authorized numeric loopback SocketAddr and exact `ws://`
   endpoint/path for the selected CDP target. Validate endpoint/scheme/authority,
   bounded URI/headers and absence of userinfo before opening the connection.
   Use owned `TcpStream::connect_timeout` under remaining parent deadline; reject
   zero/expired budgets. Do not call tungstenite connect/connect_with_config:
   they introduce unbounded DNS/connect and redirect behavior. No automatic target
   discovery expansion or redirect follow. DNS/WSS/proxies require another scoped
   adoption/qualification; no claim of their support here.
2. Wrap the owned socket in a thin `Read + Write` deadline/cancel guard and call
   `client_with_config(request, stream, Some(config))`. Recompute nonzero remaining
   time before **each** underlying read/write/flush, set both socket timeouts and
   check parent deadline/cancel between protocol operations. A one-time socket
   timeout is per operation, not an end-to-end slow-drip deadline. Distinguish
   WouldBlock/TimedOut using the parent clock, not error text; never reset the
   request deadline on retry. A nonblocking route can retain MidHandshake and
   resume on readiness under the same deadline; no busy retry loop or async stack
   is required. Prefer the simpler guarded blocking socket for the initial slice.
3. Keep one IO owner per connection. If external cancellation must interrupt a
   blocked call, the owner may retain a cloned handle solely for shutdown(Both),
   never parallel reads/writes; join/drop all owned handles on teardown. Socket
   clone options affect the same socket. Actual macOS cancellation/close behavior
   still requires future proof; no user browser/process is terminated. On timeout
   or cancellation invalidate transport tickets, drop pending protocol state and
   suppress late results; no automatic resend/reconnect of the CDP command.
4. Build WebSocketConfig with explicit initial buffers and finite `Some` incoming
   frame/message caps plus finite max_write_buffer_size; validate host integers
   and `max_write_buffer_size > write_buffer_size` before the library's panic
   path. For latency-oriented request/response, `write_buffer_size(0)` is a proposed
   batching choice, not a memory cap. Set initial read size explicitly: set_config
   does not resize FrameCodec's existing input buffer. D05 owns numeric limits.
5. Pre-bound serialized outbound CDP JSON independently. Source write path does
   not enforce max_message_size/max_frame_size on outgoing Text even though error
   docs broadly mention a write capacity error. max_write_buffer_size includes
   frame header/mask, not just JSON payload; budget at most 14 header bytes per
   client frame plus existing buffered data/control reply. Host allocations exist
   before library admission. Reject invalid/oversize commands before dispatch.
6. `write` may queue/partially send; `send` is write then flush. On IO/WouldBlock,
   resume flushing the same pending bytes within deadline, never send the CDP
   message again. `WriteBufferFull` returns the unqueued frame; bound any host-side
   retry/pending queue too. Fatal IO/protocol/capacity errors close this owned
   transport and report affected request/partial channel state. Socket write
   success is not CDP response or verified application outcome.
7. Read complete Text messages, then bounded JSON parse and exact CDP correlation.
   Binary is not implicitly treated as equivalent CDP text; reject unsupported
   payload kinds. Ping/Pong/Close are protocol control, not CDP completion. The
   library queues automatic pong/close replies; drive flush/read only within an
   explicit operation/teardown budget. Do not add heartbeat, idle collector or
   periodic UI polling. Idle retained sockets may be found closed at the next
   explicit request; that is an explicit recovery condition.
8. `close` starts protocol closing; it does not guarantee immediate socket closure.
   Clients can await server EOF indefinitely without our deadline. Attempt close/
   flush only within bounded teardown, then shutdown/drop own socket. Do not send
   new commands after close/cancel; ConnectionClosed terminates use, AlreadyClosed
   is an ownership/programming error. Handle macOS repeated shutdown NotConnected
   as an idempotent cleanup condition, not a reason to touch other processes.

The standard-library evidence is the pinned
[Rust 1.96.0 TcpStream source](https://github.com/rust-lang/rust/blob/1.96.0/library/std/src/net/tcp.rs)
(connect_timeout, timeout setters, try_clone and shutdown docs/implementations),
not an executed host test. Current std docs resolve to 1.99.0; the 1.96 tag was read
separately to avoid claiming newer documentation as pinned runtime evidence.

## Library bounds versus caller responsibilities

| Source behavior | What it proves / what the adapter must add |
| --- | --- |
| `read_buffer_size` defaults 128KiB; FrameCodec reserves toward frame length | Read chunk/capacity setup, not a total input-memory maximum. BytesMut/Vec capacity slack and retained backing storage must be measured by D05 |
| `max_frame_size` defaults 16MiB, checks declared payload length in u64 before narrowing/reserving | Limits incoming frame payload, excluding header. Choose finite explicit limit; incoming invalid control payload may still allocate up to frame cap before control-size rejection |
| `max_message_size` defaults 64MiB, bounds final/unfragmented and cumulative incomplete message bytes | Bounds incoming decoded message content, not fragment count, stream duration, all live buffers or outbound data. Enforce time/received-byte/work budgets in caller |
| `max_write_buffer_size` defaults usize::MAX | Must override. Admission checks full frame+current buffer; automatic reply is a separate Option slot. Existing buffers, returned unqueued frame and caller queue also consume memory |
| Fragmented text/binary and UTF-8 split sequences are handled; control frames must be final and≤125 bytes; client rejects masked server/RSV frames | Reuse codec, do not implement another framing layer. No compression/extensions support is selected. A stream of tiny/empty fragments needs parent deadlines/work bounds |
| Handshake MAX_HEADERS 124; AttackCheck caps 65536 incoming bytes/512 successful reads and rejects too-small reads after 64 | Hardcoded mitigation, not configurable D05 handshake budget. Byte check occurs after appending each 4096-byte read chunk; parser/buffer/header allocations are separate. A guarded IO budget is still required |

No sum of these payload limits is advertised as allocator/process cap. Fragment
assembly, input buffer, message object, output buffer, HTTP header/tail data, control
reply, JSON parser, correlation map and kernel socket storage can coexist. No memory
cap or retention policy is invented from this source reading.

## Source caveats requiring explicit guards

- Client VerifyData checks status 101, upgrade/connection, accept key and subprotocol;
  **unsolicited Sec-WebSocket-Extensions validation is TODO**. Do not request an
  extension; inspect the returned HTTP response and reject any extension header
  before using the WebSocket. Do not rely on later RSV rejection as negotiation.
- Handshake Writing asserts that a successful nonempty write returns >0. Normalize
  an underlying `Ok(0)` into WriteZero in the IO guard rather than exposing this
  panic path. Read EOF/partial handshake is a failure; no retry as fresh request.
- Trace logs include whole request/frame/message data; close debug output and UTF-8
  errors may include peer text. Disable raw tungstenite-target diagnostics at the
  host logging boundary; never serialize Debug/Display of payload-bearing errors,
  HTTP responses, Message/Frame or URI-with-credentials. Emit sanitized error kind,
  scoped request ID and counts only. Canary checks below must prove this behavior.
- CDP remains adapter-owned: bind connection/session/target/surface generation and
  monotonically allocated IDs; match both CDP id and sessionId when used. Events
  are separate, bounded invalidation data, never request completion or automatic
  recollection. Ignore/reject unknown/late IDs without reusing an expired ticket;
  cap outstanding requests/event bytes and preserve completed channels on failure.
  No DOM graph/diff/analytics engine moves into this codec boundary.

## Focused future checks — proposed, not run

Use one owned finite fake peer/IO stub suite before W01 qualification, followed by
its authorized Chromium fixture slice only after D05 prerequisites:

- Slow-drip/never-finishing HTTP handshake; header limit; invalid accept key and
  unsolicited extensions; no DNS/redirect path; nonempty write→Ok(0) conversion.
- Frame cap before oversized payload allocation; fragmented aggregate cap,
  interleaved ping, empty-fragment flood, invalid UTF-8/mask/RSV/control frames.
- Short writes/WouldBlock, finite write backpressure and automatic pong retry:
  peer observes exactly one CDP command, never duplicate replay after partial IO.
- Deadline/cancel during read, write and handshake; bounded close with peer that
  never sends EOF; shutdown wakes owned blocked IO and another target progresses.
- Interleaved out-of-order CDP responses/events, wrong session/target generation,
  duplicate/late IDs and event overflow: no wrong result or scope widening.
- Payload/header/UTF-8/close canaries absent from all logs/errors; retained channel
  privacy preserved; no library read/collection work occurs between requests.
- Measure actual buffer/fragment/parser/map high-water allocations under selected
  D05 limits; do not treat maximum message bytes as total memory or runtime proof.

No test fixture/server, browser, build, library test or dependency install ran in
this packet. Source/API compatibility supports conditional adoption; fake-peer,
actual Chromium 145/macOS 27.0.1/Rust 1.96.0 and memory/cancel qualification remain
unverified. D05 live gate remains open. Root/Integration must retain these guards
when adopting the manifest, or return the exact incompatible condition before W01.
