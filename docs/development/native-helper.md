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
  "pixel_policy": "owned_synthetic_fixture"
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

`artifact_directory` and `pixel_policy` are optional for AX, mandatory for capture.
Only explicit `owned_synthetic_fixture` policy is supported; it is not a general
masking policy or permission to export real-user pixels. The parent authorizes a
new directory under its run-owned location with an existing trusted parent.
Native creates it with mode0700, refuses an existing path (including a symlink),
and writes capture.png/capture-metadata.json there. `payload_ref` is capture.png
relative to that configured directory; no output path comes from Request/UI text.
The caller retains the directory-to-operation association, owns partial artifacts
and cleanup, and configures a fresh directory for subsequent capture requests
(after prior helpers reap). AX does not create that directory. No automatic export,
permission prompt, backend fallback or retry is implemented.

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

Swift/SDK allocations, AX strings/batches/window inventory and image buffers are
not a Rust heap/RSS bound. Further acquisition/string/pixel ceilings, arbitrary-app
identity, safe production pixel redaction/storage and real installable-adapter
qualification remain Native/M01 work. The recorded B ScreenCaptureKit−3801 outcome
stays permission_required; this packet does not retry pixels or alter permissions.

## Build and offline checks

[Helper build](../../plugins/macos/README.md). The two affected legacy builds are:

```sh
xcrun swiftc -parse-as-library -swift-version 6 -D CAPTURE_LIBRARY \
  -target arm64-apple-macos14.0 fixtures/native/Observe.swift \
  tests/bridges/native/Collector.swift tests/bridges/native/WindowAX.swift \
  -o "$NATIVE_TASK_TMP/legacy-collector"
xcrun swiftc -parse-as-library -swift-version 6 -target arm64-apple-macos14.0 \
  fixtures/native/Observe.swift -o "$NATIVE_TASK_TMP/legacy-observe"
```

Offline protocol peer replaces acquisition only; it invokes the actual FD reader
and common collector failure encoder, never Collector.collect or SDK APIs:

```sh
xcrun swiftc -parse-as-library -swift-version 6 -D CAPTURE_LIBRARY -D HOST_HELPER \
  -target arm64-apple-macos14.0 fixtures/native/Observe.swift \
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
