# Native helper connected to the guarded host

The `plugins/macos/HostHelper.swift` executable consumes the actual H01
[channel-specific FD contract](host.md#connected-native-parent-exchange-finite-producer-proof).
It delegates to the existing Collector/WindowAX/CaptureLifecycle owners. Core owns
spawn, TargetLease, fixed byte buffers, monotonic operation deadline, capture
admission and reap. The admitted Rust worker owns canonical decode/validation,
ObservationSession and Frame→Commit→ACK. No second schema or collector was added.
This is source/build/offline protocol qualification, not SDK/live acceptance.

## Trusted configuration and caller flow

The caller establishes the own-fixture Target/process incarnation and Surface,
then registers `NativeHelperBinding::authorized(SpawnSpec, channel_mask, bytes)`
through `configure_native_helpers` while Attached. These bytes are trusted setup,
never UI/request-derived authority. Maximum encoded configuration:4032 bytes.
No manifest file is opened by the H01 entrypoint; the needed manifest fields are
embedded in this private JSON configuration:

```json
{
  "binding": {
    "pid": 123,
    "bundle_id": "local.uiblueprint.f02.off",
    "launch_time": 100.0,
    "window_id": 456,
    "window_identifier": "a",
    "target_generation": "g1",
    "surface_generation": "w1"
  },
  "scope_id": "form-1",
  "collection": "sample",
  "artifact_directory": "/absolute/run-owned/new-capture-directory",
  "pixel_policy": "owned_synthetic_fixture",
  "acquisition_evidence": false,
  "acquisition_limits": {
    "ax_windows": 32, "array_page": 32, "child_entries": 1280,
    "value_utf8_bytes": 4096, "action_names": 32, "action_name_utf8_bytes": 256,
    "batch_values": 8, "batch_utf8_bytes": 16384, "copied_utf8_bytes": 262144,
    "response_slots": 65536, "response_string_utf8_bytes": 1048576,
    "image_width": 4096, "image_height": 4096, "image_pixels": 8388608,
    "image_bytes": 67108864, "png_bytes": 67108864, "sidecar_bytes": 16384
  }
}
```

Numbers/IDs above are illustrative, never discovery results. Binding must come
from the actual own F02 manifest and independent public process/window checks.
Only F02 off/on bundle IDs and explicit a/b windows are supported. PID, title,
CGWindowID and rectangle alone do not establish identity for other applications.
Unknown configuration keys, incompatible binding, scope or Surface generations
refuse before acquisition. Public launchDate and CG owner PID are revalidated by
the shared collector; AX additionally requires one exact window AXIdentifier.
Read-only collection does not focus/resize/change the target or request permissions.

`collection` is `sample` (the existing four-field sample control) or `window-ax`
(the existing eight-field whole-window AX collection). The original request must
supply the corresponding exact field list; there are no implicit defaults or
new arbitrary-subtree capability. A selected capture channel uses the original
Context even when AX's selected collection is window-ax. Canonical channel sets
can contain AX, capture or both; each helper performs only its selected channel.

All acquisition limits are mandatory explicit caller values, positive and at or
below [the registered profile](../specs/development/decisions/d05-native-acquisition.md).
The example uses the selected initial ceilings, not implicit defaults. Smaller
request and parent limits still govern. `acquisition_evidence` enables only bounded
private counters/metadata; ordinary canonical output gains no telemetry.

`artifact_directory` is a trusted private per-operation container. Capture requires
explicit `pixel_policy: owned_synthetic_fixture`; AX evidence only requires the
trusted directory, not pixel permission. No general real-user pixel policy exists.
The helper creates the container if absent and requires ownership/private0700/no
final symlink. Each selected channel creates a NEW0700 child (`ax` or `capture`),
so AX evidence cannot consume capture's destination and neither overwrites files.
Capture's canonical payload_ref is `capture/capture.png` relative to the configured
container. Legacy combined collector retains capture.png relative to its supplied
fixture directory. The caller retains the operation-to-container association and
owns cleanup/partial files after abrupt child death; every later operation needs
a fresh destination. No output path comes from Request/UI text.

H01 AX writes `ax/acquisition.json` only with evidence enabled. Capture writes
`capture/capture.png`, and optionally `capture/capture-metadata.json`; all use
exclusive/no-follow partial files and complete-only no-overwrite publication.
No permission prompt, backend fallback, automatic export or retry is implemented.

## Wire and deadline

FD3 receives exactly the saved64-byte Configure, admitted configuration bytes,
matching64-byte Submit, then the unchanged canonical Request bytes. Class is
Observe8; slot0 is AX and1 capture; flags/reserved bytes must be zero. Nonzero
session epoch, host operation and actual Ticket sequence must match both headers.
Ticket is distinct from host operation sequence. Configure bounds the reply
including LF; Submit supplies remaining duration. The Request itself keeps its
worker clock domain and Context; the helper never invents a Ticket or restamps an
Observation into that worker clock. Collection uses helper monotonic seconds;
parent Instant remains authoritative, with worker/helper local duration checks.

Input lengths are admitted before allocation/read/parse: config≤4032 bytes,
Request≤2MiB, reply≤512KiB including LF and smaller caller limits. Request JSON
whitespace is valid; input is length-framed, not NDJSON-scanned. The shared collector
retains max_elements≤160/depth≤9 and its explicit field sets. No response truncation
is passed off as acquisition bounds. The helper waits at most8s for bootstrap
(the legacy one-shot safeguard, not a request default); Submit starts its remaining
local duration, tightened by the request budget. Short transfers/EINTR do not
restart it. The SDK phase has an owned-process watchdog; parent still owns cleanup.

FD4 emits one UTF-8 canonical ChannelResponse and LF. Cap/UTF-8/no-unescaped-LF
checks occur before the first byte; bounded partial writes use the same deadline.
Errors never interpolate source/error payloads into diagnostics. Invalid control/
configuration/request or transport failure exits2 without a fabricated canonical
success or quota status on FD5. An absent private status remains generic failure.
Existing per-channel permission/capture failures use canonical Issue responses.
Neither early helper exit nor a completed callback proves parent cleanup/reap.

## Reuse, privacy and capture admission

Collector.swift now has a reusable selected-channel method and unchanged legacy
argv/stdin wrapper; WindowAX.swift is reused unchanged. Its secure-field branch
omits AXValue before reading it, preserving redacted versus empty/false. Native
role/unknown/partial coverage/provenance are retained. This is scoped known-secret
handling, not universal redaction. Rust validates canonical responses inside the
allocation guard; parent only frames/copies opaque bytes.

CaptureLifecycle uses explicit `parentOwned` admission for H01. It does not read
legacy environment fault switches or acquire/release a competing flock. The
parent's capture lease lasts until actual helper reap. Legacy fixture calls retain
the existing run-owned flock and injected diagnostic modes. Both share the same
bounded callback gate and late-result rejection. AX never waits for capture or
checks its permission; capture never queries AX permission/tree. Audio/cursor/
child-window exclusion and unknown AX-to-image transform remain unchanged.

NativeAcquisition now bounds ranged AX entries and additional copied strings/
actions/batches; NativeJSON charges object/string construction and writes through
the existing Foundation codec to a reserved LF-inclusive sink. NativeArtifacts
checks image dimensions/area/footprint and caps ImageIO callback bytes BEFORE file
writes. The UTF-8 conversion buffer and owned String are both charged; batch
admission includes that overlap. Requested/admitted/actual copied bytes are distinct.
Source-controlled bounds do not cover initial opaque CF objects, Foundation/ImageIO
internal scratch or SDK allocations and are not a Rust heap/RSS bound. Arbitrary-app
identity, safe production pixel redaction/storage and real SDK qualification remain. The recorded B ScreenCaptureKit−3801 outcome
stays permission_required; this packet does not retry pixels or alter permissions.

## Build and offline checks

[Helper build](../../plugins/macos/README.md). The two affected legacy builds are:

```sh
xcrun swiftc -parse-as-library -swift-version 6 -D CAPTURE_LIBRARY \
  -target arm64-apple-macos14.0 plugins/macos/NativeAcquisition.swift \
  plugins/macos/NativeJSON.swift plugins/macos/NativeArtifacts.swift fixtures/native/Observe.swift \
  tests/bridges/native/Collector.swift tests/bridges/native/WindowAX.swift \
  -o "$NATIVE_TASK_TMP/legacy-collector"
xcrun swiftc -parse-as-library -swift-version 6 -target arm64-apple-macos14.0 \
  plugins/macos/NativeAcquisition.swift plugins/macos/NativeJSON.swift plugins/macos/NativeArtifacts.swift \
  fixtures/native/Observe.swift -o "$NATIVE_TASK_TMP/legacy-observe"
```

Offline protocol peer replaces acquisition only; it invokes the actual FD reader
and common collector failure encoder, never Collector.collect or SDK APIs:

```sh
xcrun swiftc -parse-as-library -swift-version 6 -D CAPTURE_LIBRARY -D HOST_HELPER \
  -target arm64-apple-macos14.0 plugins/macos/NativeAcquisition.swift \
  plugins/macos/NativeJSON.swift plugins/macos/NativeArtifacts.swift fixtures/native/Observe.swift \
  tests/bridges/native/Collector.swift tests/bridges/native/WindowAX.swift \
  plugins/macos/HostProtocol.swift tests/bridges/native/host_helper/ProtocolPeer.swift \
  -o "$NATIVE_TASK_TMP/protocol-peer"
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_TARGET_DIR="$NATIVE_TASK_TMP/target" \
  cargo +1.96.0 build --locked --offline -p uiblueprint-schema --bin uiblueprint-validate
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/host_helper/check.py \
  --peer "$NATIVE_TASK_TMP/protocol-peer" --helper "$NATIVE_TASK_TMP/native-host-helper" \
  --validator "$NATIVE_TASK_TMP/target/debug/uiblueprint-validate"
```

The driver derives Request from ENV-REQUEST-VALID and validates emitted synthetic
permission Issues with the real canonical validator. Actual helper execution is
limited to malformed header/scope/missing pixel-policy refusal before SDK access.
These checks do not prove live collector success or parent integration with this
Swift binary. That needs its subsequent runtime packet, independent changed-source
review and the remaining positive M01/D05/D06 gates.

## Registered acquisition checks and legacy callers

The three shared files NativeAcquisition.swift, NativeJSON.swift and
NativeArtifacts.swift are linked by H01 and legacy builds. WindowAX and sample
collection use the same ranged-array/value admission. The standalone historical
Observe diagnostic retains its own noncanonical AX reporting; its shared capture
admission/PNG sink consumes the explicit profile. It is not relabelled a guarded
canonical H01 path.

Legacy Collector argv is `manifest output mode acquisition-limits.json [Ticket]`;
standalone Observe argv is `manifest output acquisition-limits.json [sample-count]`.
The pure `--gate-checks` path performs no acquisition and needs no limits. prove.py,
sizing.py and capture_lifecycle.py require `--acquisition-limits`; use their explicit
current source builds, not the historical Observe binary extracted by prepare.py.
Their expected outcomes/stimuli/fault semantics remain unchanged; these live drivers
were not executed for the source packet.

Compile and run the nonvisual synthetic suite (own CF values, tiny CGImage/ImageIO,
streams and files; no live AX/ScreenCaptureKit):

```sh
xcrun swiftc -parse-as-library -swift-version 6 -target arm64-apple-macos14.0 \
  plugins/macos/NativeAcquisition.swift plugins/macos/NativeJSON.swift \
  plugins/macos/NativeArtifacts.swift tests/bridges/native/WindowAX.swift \
  tests/bridges/native/acquisition/Checks.swift -o "$NATIVE_TASK_TMP/acquisition-checks"
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/acquisition/check.py \
  --checks "$NATIVE_TASK_TMP/acquisition-checks" --validator "$VERIFIED_VALIDATOR" \
  --sample "$RETAINED_NATIVE_SAMPLE" --output "$NATIVE_TASK_TMP"
```

The sample is the existing D05 Native returned-wire.ndjson identified in its receipt.
The runner proves unchanged canonical data through a bounded codec and the existing
validator; it does not pretend to reacquire that old UI or prove live AX node output.

## Focused actual-owner flow proof

The follow-up to the acquisition review exercises only its three missing boundaries.
WindowAX's internal NativeAXAccess substitutes exact AX calls with bounded synthetic
CF handles/pages; live callers use the same public APIs and validate element types
before casts. No config/CLI selector or production fault mode exists. The tests
run actual collectWindowAX traversal, Collector.resolveWindow and the actual batch
schedule, including failed/oversized identity with zero AXValue dispatch.

Collector.finishChannel is the production terminal path: construction/codec/evidence
failure can encode the pre-reserved whole failure, while final FD send stays outside
those catches. Tests inspect actual owned socketpair receivers for whole/zero output,
and close one receiver after a real prefix to prove a failed send has no second reply.
Cyclic synthetic raw AX relationships prove bounded traversal, not canonical graph
acceptance or real SDK behavior. Earlier137 numeric/codec and33 protocol tests are
not rerun as an unchanged broad suite for this focused proof.

```sh
xcrun swiftc -parse-as-library -swift-version 6 -D CAPTURE_LIBRARY -D HOST_HELPER \
  -target arm64-apple-macos14.0 plugins/macos/NativeAcquisition.swift \
  plugins/macos/NativeJSON.swift plugins/macos/NativeArtifacts.swift \
  fixtures/native/Observe.swift tests/bridges/native/Collector.swift \
  tests/bridges/native/WindowAX.swift tests/bridges/native/acquisition/FlowChecks.swift \
  -o "$NATIVE_TASK_TMP/flow-checks"
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/acquisition/flow_check.py \
  --checks "$NATIVE_TASK_TMP/flow-checks" --validator "$VERIFIED_VALIDATOR" \
  --sample "$RETAINED_NATIVE_SAMPLE" --output "$NATIVE_TASK_TMP"
```

The old saved canonical sample supplies a bounded large codec/FD input only; this
is not another sample-replay acceptance claim. Live AX/SCK/UI and SDK/H01/pixel/
latency qualification remain separate. The same reviewer reconciles the saved
focused result; author execution is not independent acceptance.
