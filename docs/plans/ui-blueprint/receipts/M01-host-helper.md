# M01 host helper source connection

Packet [M01-host-helper](../packets/M01-host-helper.md), f378334; Restore,
source/build/offline protocol only. Provider c0abcffed886d0f33b34ddf3008a902b8e024676;
Native source handoff99c4c159. Active/Evolving; no SDK/live/permission grant.

Actual write set declared before editing: tests/bridges/native/Collector.swift;
fixtures/native/Observe.swift (shared CaptureLifecycle only);
plugins/macos/HostProtocol.swift, HostHelper.swift, README.md;
tests/bridges/native/host_helper/ProtocolPeer.swift, check.py;
docs/development/native-helper.md; this receipt. No Rust/shared/spec/Git writes
outside the later exact-path root grant. Existing unrelated files remain untouched.

Traversal: AGENTS → specs/README registry10 → product/decisions/acceptance routes;
full current handoff closure reused, NATIVE-PILOTS@1 and DEV.RUST@2 loaded fully.
D02@2, D04@1, D05@3, MEMORY@2, WORK@1 and their explicit full CONTENT closure
remain as recorded in H01-native-handoff; no revision drift. Implementation,
product-truth change/delivery/evidence/coordination, QA, operational-safety and
Apple-platform governance read. AppKit remains only the existing nonvisual public
process-incarnation adapter; no visible UI or new Apple UI implementation.

Requirements: one selected channel, unchanged canonical Request/ChannelResponse,
real Ticket, trusted binding, bounded framing/duration, adapter redaction and
parent-owned process/capture cleanup. Observed provider chooses FD3/4 Control
Configure+Submit; source-specific trusted fixture configuration is an engineering
connection. No arbitrary-app identity, pixel permission or production privacy claim.

Status: checkpoint_ready; waiting for root exact-nine-path Git grant. Source
connection and assigned offline checks completed; independent review and actual
Swift/H01 live integration are not accepted by this receipt.

## Realized connection and scope

NativeHostHelper reads Core Configure+Submit on FD3 and emits exactly the selected
canonical ChannelResponse on FD4. Control magic/class/slot/flags/reserved bytes,
epoch/operation/real Ticket correlation, admitted lengths and remaining duration
are checked before collection. Boundaries:4032-byte config,2MiB Request,512KiB reply
INCLUDING LF, smaller caller limits. No request JSON newline scanning; short IO
and EINTR preserve the deadline. Socket-local NOSIGPIPE, no global signal handler.
Invalid setup/transport exits2 with no raw diagnostics or invented quota status.

The trusted private JSON configuration carries existing fixture manifest binding,
authorized scope and sample/window-ax selection. It is described exactly in the
[consumer contract](../../../development/native-helper.md). Optional capture
artifact directory and mandatory explicit owned_synthetic_fixture policy are
caller authority, never UI data. A capture path must be new, created mode0700;
caller owns directory-to-operation association/partial artifacts/cleanup and
reconfigures a fresh destination after prior helpers reap. No production unmasked
pixel policy is implied. There is no new Rust shared API dependency.

Collector.swift remains the single acquisition/canonical-encoding owner, now with
a reusable selected-channel method and its legacy wrapper. WindowAX is unchanged;
known secure AXValue is still unacquired/redacted. H01 calls shared CaptureLifecycle
with parentOwned admission, no legacy environment fault/lock settings. Parent
retains its capture lease until actual reap. Legacy calls retain flock/callback
behavior and compile against the same source. Exact process launchDate/window
owner/AX identifier binding, partial coverage, source clocks and unknown transforms
remain. No arbitrary-app identity or universal secret recognition is claimed.

## Checks actually run

Apple Swift6.4 (swiftlang-6.4.0.34.1), Swift6 language mode, arm64 compile deployment
macOS14.0. Four compiled targets: actual native-host-helper; legacy Collector with
CAPTURE_LIBRARY; standalone legacy Observe; offline ProtocolPeer using the actual
HostProtocol and shared canonical failure encoder. All passed, no diagnostics.
Build commands are in [plugins/macos](../../../../plugins/macos/README.md) and the
consumer contract. No Swift application bundle was launched.

Canonical validator: Rust1.96.0, locked/offline schema binary only, source/features
unchanged. First default-debug attempt failed with ENOSPC while writing full.rmeta;
no semantic/compiler failure was inferred. A direct retry with CARGO_INCREMENTAL=0
and CARGO_PROFILE_DEV_DEBUG=0 passed. This is functional validation, not production
profile/performance/D06 evidence. No cache or other owner's files were deleted.
An earlier mkdir also failed ENOSPC before source creation; work resumed after
available storage recovered. Neither incident authorized unrelated cleanup.

Offline check.py passed31 cases: valid selected AX/capture routing with Ticket7
versus host operation19, real canonical failure encoding/validation, exact LF cap,
one-byte-less refusal, one-byte transfers, escaped newline, truncated header/config/
body, invalid magic/class/channel/flags/reserved bytes, correlation mismatch,
zero/elapsed deadline, oversize lengths before body, trusted scope/binding/path/
generation/acquisition-cap refusal. Three actual helper cases stop before Collector
SDK access: scope mismatch, truncated header, missing pixel policy. Positive peer
outputs are explicitly synthetic permission_required Issues, not SDK observations.
Every generated response was accepted by the actual schema validator. All test-
created children reaped; no test app, AX call, capture, permission request or UI.

19 shared inputs (root Cargo/lock/toolchain, schema manifest/all src Rust files,
Core protocol/native_binding/native_broker) matched saved c0abcff before/after.
Manifest SHA256:
`54dab07d5e95718c3c3729ca7647a1acf3046c96445c914f84d68121eab84a03`.
No mutable neighboring Rust inputs were consumed as acceptance evidence.

Own seven-source manifest (includes unchanged WindowAX) SHA256:
`f359a0c37c0cbbd6bf1167c5bb6958efe0a4585c8f6fbb19d10b9be865b60f3d`.
Binary SHA256:
- native-host-helper: `3806ddfbfa503c9f7fb80c2ea2d3d88bbbb3b908b748511ac2ffe704fb4f3804`.
- protocol-peer: `80ba7ffa14fa218a33c2c5cf9e8e7809957b42e1b6013b1b67493f4d1b77559e`.
- validator: `119842ca9b12b7f2a9b0f2d4adbc1276f2c83e2eae5df983370fb015d14ddf15`.

Task-temp only:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-m01-host-helper-6jklkazj`.
Native owns binaries/source and shared manifests for this packet's root/reviewer
consumer. Retain until accepted handoff/review or explicit root cleanup; no new
permanent evidence system. No source lane, UI lane or running helper is held.

## Residual and next consumer

Core consumes the bounded trusted config and same saved FD protocol; M01's next
separately granted runtime packet must run the actual Swift executable through
H01 begin/receive/ACK and prove real SDK outcomes. Offline stub routing does not
prove AX/capture execution or parent lifecycle integration with this binary.
Independent changed-source/consumer review remains required, not self-acceptance.

SDK/string/window-list/batch/image acquisition costs remain separate from Rust
quota; this connection neither invents their limits nor claims RSS enforcement.
Full production privacy, safe pixels, arbitrary-app continuity and unchanged
positive M01–M06/D05/D06 gates remain open. The recorded B−3801 permission outcome
remains stopped: no new pixels, retries, alternate backend or settings changes.
No subsequent source/runtime packet was started. Documentation links, scoped
whitespace and route consistency checked; exact scoped commit/push follows grant.
