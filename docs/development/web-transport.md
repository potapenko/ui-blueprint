# Owned WebSocket transport

`uiblueprint-web::transport` is the concrete Rust codec/socket boundary for the
future W01 adapter. It is not a CDP client, collector, graph engine or live-adapter
acceptance. Source checkpoint: `4106e04a91e141158dc46590778fb5fd2ee300a9`.
Authority: [implementation packet](../plans/ui-blueprint/packets/W01-transport-implementation.md),
[D07@4](../specs/development/decisions/d07-reuse.md), and the accepted
[source audit](../research/W01-transport.md). No upstream source copied.

At dispatch the current full Web Spec Basis was reused: registry7 → D02@1/D03@2/
D05@2/D05-MEMORY@1/D07@3 and explicit normative closure, RUST/DEV.RUST@2. D07@4/
registry8 were registered before source/dependency mutation. Later registration
f9ff423 was read/reconciled: D02@2, D05@3, D05-MEMORY@2 and D05-WORK@1 now govern
future guarded-worker placement; this transport owns IO only and does not bypass
that host boundary. No changes to these newer contracts. Runtime float_roundtrip,
core/analysis formats and all existing dependency pins remain preserved.

## API and ownership

1. Install `install_log_boundary(&'static dyn log::Log)` once at host/worker startup,
   before another logger. It delegates non-tungstenite records to that same host
   logger; it never changes max_level. See the refusal contract below.
2. Supply validated explicit `Limits` and `OperationLimits`; no Default exists.
   `Connecting::new(limits)` returns a single-use attempt. Obtain `cancellation()`
   before calling `connect(endpoint, handshake_budget)`. `Transport::connect` is
   the convenience path when no external handshake-cancel handle is needed.
3. An endpoint must be literal `ws://<numeric-loopback>:<nonzero-port>/<path>`.
   Reject DNS names, remote IPs, TLS, fragments, userinfo and whitespace before
   connecting. Caller still owns authorization for that exact endpoint; syntactic
   loopback validation is not application authority. No proxy/custom auth headers.
4. The attempt uses connect_timeout and a deadline/cancel guarded Read+Write socket,
   then `client_with_config`. It never uses convenience redirect/DNS connectors.
   Reject unsolicited extension headers; all original source-audit guards apply.
5. `transport.operation(budget)` exclusively borrows the IO owner. Calls within it
   share the same absolute local Instant deadline and byte/work counters. Send with
   `send_text`; if Pending, retain that Operation and call `resume_send` to flush
   the original bytes. A new send is refused while pending; no command re-enqueue.
6. `receive` returns bounded Text, Ping/Pong, PeerClose or WouldBlock. Binary/raw
   frame results refuse. Control replies use the same budget; pending automatic
   pong flush is resumed by the next receive, not a background task. Dropping an
   operation with pending application/control data shuts down instead of sending
   it later under a new deadline.
7. `close` drives a finite handshake then terminates its owned connection, or
   fails/forces shutdown. Dropping the transport also invalidates and shuts down.
   No reconnect, heartbeat, retry, polling, browser launch or helper process exists.

`SendProgress` distinguishes NotQueued, Queued, PossiblyWritten and Flushed.
Flushed means accepted by the underlying socket, never a CDP response/application
outcome. PossiblyWritten is conservative (control bytes can share the stream).
Failure progress describes historical delivery uncertainty; fatal failures close
and discard pending state. A valid Text is raw transport data, not verified CDP
correlation. Future W01/W02 owners retain request IDs/session/target generations,
redaction, operation authority and effect/unknown-outcome rules.

Cancellation permanently invalidates one connection. One shutdown-only cloned
TcpStream can wake blocked reads/writes/handshake; handles never perform IO.
The shutdown duplicate is taken/dropped so surviving handles do not retain that
FD. The actual IO owner closes its original descriptor on failure/drop. TCP
connect is bounded by std connect_timeout; std exposes no in-progress descriptor,
so cancellation during that call is observed when it returns, at latest its timeout.
No claim of instantaneous connect cancellation or rollback of delivered bytes.

## Explicit limits and their actual meaning

Limits cover endpoint bytes, handshake bytes, initial codec read buffer, write
batching threshold/write-buffer maximum, incoming frame/message and outbound text.
OperationLimits has a caller absolute deadline, maximum new socket-read bytes,
maximum socket-written bytes and maximum work steps. One step is a public protocol
call or underlying IO call, not every internal codec instruction. Wire-byte/message/
read-ahead bounds additionally bound finite fragment processing; no hard realtime
or total CPU claim. Read payload delivered from earlier codec read-ahead has its
own counter under the same read-byte ceiling, so cached input cannot bypass it.

Zero/overflow/inconsistent limits refuse before scaled allocation or dispatch.
Write cap must fit outbound payload +14-byte maximum header +139-byte control
headroom and exceed batching threshold. Incoming/outgoing limits are separate.
Header request size is checked before TCP connect; guarded handshake IO is also
capped, including any coalesced tail. Remaining time is recomputed before each
underlying read/write/flush; OS timeout error text is not the deadline authority.
Nonempty write returning zero becomes WriteZero, avoiding the upstream handshake
assertion. Errors have fixed categories only and invalidate the affected connection.

These are byte/work/deadline limits, **not total allocation/RSS limits**. Codec
capacity slack, fragmented message/input/output copies, fixed handshake scratch,
randomness/library state and OS buffers remain costs. D05-WORK's future host worker
must enforce actual allocations/ownership and parent deadlines. This packet does
not select another memory profile or claim its payload limits close the live gate.

## Logging and diagnostic boundary

Raw tungstenite traces contain frames/HTTP headers and its errors can contain peer
text. The filter suppresses exact target `tungstenite` and prefix `tungstenite::`
in both Log::enabled and Log::log **before** calling the host delegate. Filtering
only enabled would be insufficient because log! may call log directly. Runtime
max_level remains host-owned. Transport code emits no raw payload diagnostic.
Failure has no upstream error/source chain; Transport/Cancellation/Event Debug is
redacted (Text shows length only). Peer close reason/headers are not returned.
Raw Text data is intentionally available to the later CDP decoder, not to logging.

A preinstalled foreign logger is never replaced: installation returns
LoggingBoundary and connection construction refuses until this controlled boundary
is installed. An independent child test confirms the foreign logger still works.
Future CLI/worker composition must install this narrow delegate filter at startup;
this packet does not change CLI or another host's logger. Unsafe external replacement
of log's global logger is outside this safe API contract.

## Focused verification

Rust 1.96.0 (ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96), aarch64-apple-darwin,
macOS 27.0.1/26A434. Initial package check plus final locked/offline Clippy/test/fmt:

```sh
cargo +1.96.0 check --locked --offline -p uiblueprint-web --all-targets
cargo +1.96.0 clippy --locked --offline -p uiblueprint-web --all-targets -- -D warnings
cargo +1.96.0 test --locked --offline -p uiblueprint-web
cargo +1.96.0 fmt -p uiblueprint-web -- --check
```

17 tests passed: 6 IO-stub checks and 11 loopback/logger checks. Test IO limits are
explicit (typical frame 512/message 768/outbound 512 bytes, 8192 operation IO bytes,
512 steps; per-case 70/60ms or finite larger deadlines). They are test parameters.

| Proof | Actual checks |
| --- | --- |
| Basic/control/privacy | Text send/read, ping auto-pong, close; private canaries in text-debug, HTTP errors/headers and close text absent from product diagnostics/captured upstream logs; non-upstream host marker preserved |
| Admission/protocol | Endpoint/zero/overflow/budget rejection; unsolicited extension, malformed handshake, invalid frame opcode, frame-size/fragment aggregate overflow, binary refusal, outbound refusal/no phantom flush |
| IO/deadline | Slow-drip absolute handshake deadline, pending bytes flushed once after short write/WouldBlock, pong backpressure resume, zero write in frame and handshake paths, read-ahead budget, pending-drop and deadline refusal with no later send |
| Ownership/cleanup | Pre/during-handshake cancel, blocked-read cancellation, another peer's progress during a blocked target, close without server EOF, bounded listener/peer/isolated-logger-child cleanup |

Only ephemeral owned loopback listeners/sockets and IO stubs were used. Accepted
peer sockets explicitly restore blocking mode on macOS; an initial fixture setup
failure was corrected, not hidden by relaxed expectations. Peer helpers use bounded IO/accept/completion waits and finished-state joins;
cancellation workers were joined after bounded result receipts. All owned threads
completed in the recorded run; product transport spawns no threads. The isolated logging child is a
test-only process and is waited/killed/reaped under a 3s bound. No browser/SDK/UI.
No unchanged Core/whole-workspace suites were run. Independent review remains.

Tested source/config input fingerprint at 4106e04 is
`91f06b85481a306ddb5cee26e5010fe0ac93d4cf35ea36814793fee91c51e070`:
SHA256 over ordered `path + NUL + saved blob` for root Cargo.toml/Cargo.lock,
plugins/web/Cargo.toml, src/lib.rs, transport/{mod,io,logging,operation,tests}.rs,
and tests/transport.rs. Cargo.toml SHA256
`a87adec8d6b011a88b1ccbb40191eeae0dc2964b52bc50e01a1d9b2a073a36c2`;
Cargo.lock `a3f321614b4551712fa44ee402f697f159dc43d0a613fb85e1e8ecae371d6173`.
Those were stable throughout final checks; the manifest lane was then released.
Later disjoint host-member changes are not retroactive changes to that test input.

## Resolved dependency handoff

Every prior lock package/version/checksum survived. Twenty new registry packages
and the concrete Web member were added. Tungstenite features are exactly handshake
(data-encoding/http/httparse/sha1); no TLS/url/default or async framework. Direct
log 0.4.29 has no features and is the same resolved facade used by tungstenite.
Runtime serde_json float_roundtrip remains unchanged. No source was vendored.

New packages (manifest MSRV; MIT OR Apache-2.0 unless noted):

| Package | Version | Declared MSRV |
| --- | --- | --- |
| tungstenite | 0.30.0 | 1.85 |
| log | 0.4.29 | 1.68 |
| bytes (MIT) | 1.12.1 | 1.57 |
| http | 1.5.0 | 1.57 |
| httparse | 1.10.1 | not declared; tested on 1.96 |
| rand / rand_core | 0.10.3 /0.10.1 | 1.85 |
| getrandom / chacha20 | 0.4.3 /0.10.2 | 1.85 |
| sha1 / digest | 0.11.0 /0.11.3 | 1.85 |
| block-buffer / crypto-common | 0.12.1 /0.2.2 | 1.85 |
| hybrid-array / const-oid / cpufeatures | 0.4.15 /0.10.2 /0.3.1 | 1.85 |
| typenum | 1.20.1 | 1.41 |
| thiserror / thiserror-impl | 2.0.21 /2.0.21 | 1.77 |
| r-efi (MIT OR Apache-2.0 OR LGPL-2.1-or-later) | 6.0.0 | 1.68; target-conditional, not built on this host |

Actual package manifests and license texts were inspected. Retain each LICENSE-MIT/
LICENSE-APACHE (bytes LICENSE) and attribution for distribution; r-efi puts its
terms/copyrights in AUTHORS, which was inspected after verifying its archive hash.
No root NOTICE found in these new packages. Source hashes are pinned by Cargo.lock;
this inventory is not qualification of every target or a replacement for I01's
selected-distribution notice inventory. Existing shared packages (cfg-if/libc/itoa,
data-encoding and proc-macro dependencies) retained their lock versions. Their
selected manifest/license texts were also read: data-encoding 2.11.1 MIT/MSRV 1.48;
cfg-if 1.0.5 MIT-or-Apache/MSRV 1.32; libc 0.2.190 MIT-or-Apache/MSRV 1.65;
itoa 1.0.18 MIT-or-Apache/MSRV 1.68; proc-macro2 1.0.107, quote 1.0.47 and syn 3.0.6
MIT-or-Apache/MSRV 1.71; unicode-ident 1.0.26 additionally requires Unicode-3.0
terms (LICENSE-UNICODE), MSRV 1.71. Preserve those notices in the relevant source/
binary distribution inventory too. No new
backend/toolchain pin or certificate store was added.

## Remaining consumer boundaries

Place this API inside the newly registered D02/D05 worker when that host is proven;
keep its thread/process/publication ownership there. Add future CDP correlation and
scoped observation under their own packet. Live Chromium/other browser qualification,
W01/B01–B06, total memory/process admission, cancellation of unrelated OS APIs and
release acceptance are not established here. Syntax accepts IPv4/IPv6 numeric
loopback; the real-peer proof above used IPv4 only. No TLS/DNS/proxy support claimed.
