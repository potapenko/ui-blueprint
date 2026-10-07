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
  "identity_path": "/absolute/owned-fixture-run/a-identity.json",
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
(bounded whole-window AX using an explicit canonical requested subset). Original
sample requests still supply their exact four fields; original eight-field window
requests keep that order/semantics. Window requests can select existing role,
accessibility_name/description, placeholder, focused, enabled, value, actions or
accessibility_bounds. Only selected attributes are read; no implicit field defaults
or arbitrary-subtree capability is introduced. A selected capture channel uses the original
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

## First H01 fixture observation: prepared caller

`crates/host/tests/native_fixture.rs` is ignored and requires the explicit owned-F02
runtime marker. `tests/bridges/native/host_observe.py prepare` builds only saved
helper/descriptor sources and that consumer from an immutable Git archive plus the
owned new caller file. It creates no branch/worktree and consumes no moving G02/Web
working changes. `run` requires `--allow-live` from the separately activated runtime
packet; preparing or ordinarily running the test never launches AX/fixture work.

Preparation (task-temp only):

```sh
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/host_observe.py prepare \
  --output "$NEW_NATIVE_TASK_TMP/build"
```

First runtime setup, when assigned: reuse the prepared F02 off bundle9a88b12, launch
it with a new run-owned directory using the existing fixture mechanism. Select A,
normal stimulus/no popup, then explicitly Snapshot. Keep the exact manifest PID,
launch time, window ID and generations; shared titles are not binding. The launcher
never launches or closes a user app/fixture itself. The assigned Native runtime
worker owns fixture setup and exact-PID/path cleanup under root's activation.

The descriptor-only `describe-window` call supplies the existing eight-field metadata
request, with no tree/pixels or Ticket. SessionDescriptor has no field projection.
The actual H01 request has only the four sample fields: role/accessibility_name/
enabled/accessibility_bounds. Its clock is replaced with the real Attached clock
inside the Rust caller. The parent then performs real begin→selected helper→guarded
receive→Commit/ACK. Caller writes only committed bytes; graph assertions happen in
the bounded launcher after completion. Setup DTO/test-tooling memory is outside the
parent's steady-state inventory and is not claimed as a production memory proof.

Fixed before evaluation: nodes160/depth9, explicit registered profile.json,
512KiB reply including LF, AX/overall1s and host cleanup1s. Attach has a5s outer bound;
caller observation polling allows2s to receive the authoritative terminal; shutdown
polling allows2s and never upgrades CleanupPending. Launcher outer bound15s, descriptor
2s and validator3s; bounded stdout/stderr stays in memory, raw diagnostics are not
persisted. These are finite test controls, not new product defaults or D06 gates.

Runtime command, only after activation:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/host_observe.py run --allow-live \
  --prepared "$NATIVE_PREPARED/prepared.json" --manifest "$OWN_F02_RUN/a.json" \
  --validator "$VERIFIED_VALIDATOR" --output "$NEW_M01_EVIDENCE_RUN"
```

`NEW_M01_EVIDENCE_RUN` must be new under system task-temp or the existing application
state `~/Library/Application Support/UIBlueprint/development/P2/M01-H01/`; create
its parent separately. Root/M01 owns retention through acceptance or explicit cleanup.
Outputs are the selected canonical response, submitted request, fixed host/cleanup
receipt and bounded result. No raw manifest/UI/error logs are copied. Successful
oracle: f02.sample.a/button, known enabled=true, authored combined label containing
“Activate sample”, known AX bounds in pt, matching context and honest partial scope.
The launcher compares fixture state/source_state in memory to detect an observer
side effect. No old76-node count or pixels are forced into this one-control scenario.

AX denial is a canonical permission_required response with confirmed cleanup and a
negative launcher result; no retry, settings or alternative backend. Capture is not
requested, and B−3801 remains stopped. The dedicated host/worker/helper processes
and directories are independent of Web's own headless Chromium/profile/port. Only
brief F02 setup/Snapshot needs the shared physical-input lane; AX-only uses no capture
lane. Actual SDK/H01 observation, teardown and latency remain unverified by preparation.

## Native input context subset

Director's real reference question is where to type a name, distinct from explanatory
text. The same WindowAX owner now supports the explicit subset:
`role, accessibility_name, placeholder, focused, enabled`. `accessibility_name` uses
the existing public AXDescription mapping; placeholder uses AXPlaceholderValue;
focused uses AXFocused; enabled uses AXEnabled. Unknown/unsupported values remain
explicit, known false/empty remain known, and no missing name is filled from placeholder.
Per-node focused does not populate global keyboard/accessibility focus or active
descendant. No value/draft, anchor, relations or input delivery are added.

Runtime-ready own-F02 case, for a separately assigned run: fresh A normal Snapshot,
exact manifest binding, `collection: window-ax`, those five fields,160/depth9,
512KiB including LF, parent/AX1s and cleanup1s, same explicit acquisition limits.
Find the actual textbox by the reported AXIdentifier extension f02.name; expect
placeholder Name and enabled=true, report the actual per-node focused bool (typically
false after Snapshot) and actual name availability. Do not derive focus from selection
or text. No old node count, anchor gap, image transform or Director UI equality is
claimed. The F02 form is a mechanism/oracle case, not the real PlayPhrase.me popup.

Reuse the existing ignored Rust consumer: it already consumes caller-supplied
Session/Request/configuration files and replaces Request.clock_domain from Attached.
Generate those existing inputs in system task-temp using the existing descriptor-only
`describe-window` with the five-field Request, then run the same consumer with the
existing owned_f02_a opt-in marker and a new form-context helper binary. No new caller
flag/API is required; the sample-specific host_observe.py oracle remains unchanged.
The next assigned runtime worker checks returned form properties with the same Rust
validator and confirms exact helper/worker/fixture cleanup. Temporary inputs/results
are removed after use; any new persistent destination needs explicit user authorization
for that location and purpose. Source preparation here performs no live run.

Focused offline command:

```sh
xcrun swiftc -parse-as-library -swift-version 6 -D CAPTURE_LIBRARY -D HOST_HELPER \
  -target arm64-apple-macos14.0 plugins/macos/NativeAcquisition.swift \
  plugins/macos/NativeJSON.swift plugins/macos/NativeArtifacts.swift \
  fixtures/native/Observe.swift tests/bridges/native/Collector.swift \
  tests/bridges/native/WindowAX.swift tests/bridges/native/acquisition/FormChecks.swift \
  -o "$NATIVE_TASK_TMP/form-checks"
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/acquisition/form_check.py \
  --checks "$NATIVE_TASK_TMP/form-checks" --validator "$VERIFIED_VALIDATOR" \
  --output "$NATIVE_TASK_TMP"
```

## Explicit measured probe connection

Slot2/bit4 is opt_in_layout_probe, fields layout_bounds only. Native uses the same
helper executable/Configure+Submit/Ticket/bounded builder and canonical frame.
Core must admit channel2 as a non-capture helper; no capture resource is needed.
The shared parent broker remains the only process/ingress/publication owner.

Trusted opaque configuration adds probe_manifest_path, probe_snapshot_request,
probe_source_revision and probe_uptime. Path is absolute caller-owned fixture input;
no UI-request path or filesystem search. File is opened read-only/no-follow and
bounded by the admitted reply cap before parse. Expected binding matches PID/bundle/
launch/window/target+surface generations; exact snapshot request/source revision/
uptime must also match. Fresh explicit Snapshot caller supplies these values;
PID/title alone cannot authorize imported geometry. Existing4032-byte config cap
and registered response-building/output/node/deadline limits still apply.

Three icon/text/container layout_bounds come from the existing PROBE fixture's
SwiftUI anchorPreference measurement, fixture-local pt with top-left origin.
Source mapping uses the fixture's explicit component declaration; declarations
are not substituted for measured rectangles. Screen/pixel transform stays unknown.
Native does not calculate gap or read expectations.json. Rust consumes the actual
canonical Geometry and derives gap; the independent test oracle remains8/18pt.

Observation retains fixture uptime/clock provenance, answer_source cache and
freshness unverified; current_required refuses stale_target instead of restamping
an old Snapshot. Use cached_allowed for this explicit measured-record import.
Off probe returns failed unsupported; missing markers returns incomplete_scope;
stale binding/request/time returns stale_target. Malformed/oversized input refuses
before publication. No invented off markers or known transform.

Actual H01 measured-probe ACK and engine gap remain the next separately activated
run: F02-on fresh Snapshot, trusted current manifest/config, real worker Ticket,
then gap from canonical icon/text bounds. Full matched off/on invariance remains
another required condition, holding/checking app_active/window_key/window_main and
focus/content/size before and after acquisition. The historical mismatched pixel
pair cannot prove invariance. No UI/pixel/probe run is implied by source checks.

## Actual probe and guarded Measure caller

The existing fixture caller now accepts its supplied single AX or probe request;
request channel determines bit/slot without an added public flag. Existing launcher
uses own manifest probe_enabled to select the measured path. Probe requests use
layout_bounds/design/cached_allowed and opt_in_layout_probe, with exact expected
Snapshot request/source revision/uptime. Metadata-only descriptor reads the existing
AX window metadata path; caller-owned Session capability declares the connected
probe import, not a claim of general Native layout support. Current app/Surface
binding is still verified by the helper and trusted fixture manifest.

After a canonical probe ACK, that same guarded session executes existing Measure:
Tape(core Snapshot, analysis query, bound evaluation). Caller derives no arithmetic;
the worker runs the sole Rust engine in fixture-local pt, preserving the imported
source observations and unknown transform. The independent launcher checks known
Rust gap against fixtures/native/expectations.json's baseline8/expanded18. This is
not an Expectation copied into the collector or a schema migration.

Next one-operation setup: fresh F02-on A normal Snapshot, run the existing launcher;
then explicit Change layout and a fresh Snapshot, run again against the new exact
manifest, with unchanged160/depth9/512KiB/profile/parent1s/cleanup1s. Matched off/on
invariance is not this sequence. All current builds/module caches/inputs/results
belong in operation system-temp and are removed after inline facts; no persistent
archive/destination. Runtime begins only on saved matching Core channel2 provider
and this saved caller, with exact owned setup and cleanup as the activation states.

## Current own-fixture identity after close/reopen

Modern trusted configuration requires `identity_path`, the exact caller-owned
`a-identity.json` or `b-identity.json` in the existing temporary fixture run directory.
The fresh Snapshot manifest reports that path. File cap4032 bytes/read-only/no-follow;
no arbitrary UI path, title matching or filesystem discovery. Missing evidence from
older fixture builds refuses; there is no compatibility bypass or implicit default.

FixtureIdentity, within Fixture.swift, owns only fixed version/PID/bundle/launch/
window key/ID/target+surface generations/open|closed data. Explicit Snapshot atomically
publishes OPEN for the current window. Close rotates the existing surface generation
and atomically publishes CLOSED without calling measurement publish, collecting UI
or adding timers/polling. Reopen remains invalid until a fresh explicit Snapshot.
If close invalidation storage fails, the OWN debug fixture terminates fail-closed so
a stale OPEN record cannot coexist with its expected live process incarnation.
Visible SwiftUI content/layout/controls and independent expectations are unchanged.

Collector compares the current identity receipt with the trusted expected binding
before collection and immediately before sending the complete channel. Public launch/
CG owner and exact AX window matching remain independent; public process/window
binding is checked again at publication. Closed/mismatched/racing generation becomes
existing stale_target; unresolved public mapping remains target_unresolved. Probe
also checks identity before/after import, retaining cache/unverified measurement time.
Identity evidence never upgrades a stored Snapshot into current measured geometry.
Already ACKed channels stay governed by the parent publication/cleanup contract.

Next finite CLI proof must rebuild matching own off/on fixture/helper sources,
never reuse old9a88b12 fixture binaries as if they have identity receipts. Same-title
A/B AX-only cases use each fresh Snapshot's own identity_path. Keep A's old trusted
binding, close/reopen (even if CG ID reused), attempt old binding and require refusal;
then explicit Snapshot and fresh binding must succeed. B capture remains stopped.
This source step runs only synthetic lifecycle/reader checks, no UI/live proof.

## Own popup scope and anchor

Current fixture Snapshot attempts to attribute an open own popover window through
bounded public NSAccessibility identifier/children, matching explicit
f02.popup.owner.a/b. It excludes the parent A/B windows as popup candidates. A unique
marker-bearing own NSWindow supplies its actual windowNumber; title/rect/order do
not select it. Missing/ambiguous public mapping publishes popup_binding_status
unresolved and no binding. This mechanism still needs actual SwiftUI popup qualification.
Nonvisual explicit popup window identifier is assigned only after that attribution.

A/b popup presentation and close invalidate an independent popup-a/b identity receipt;
willClose also invalidates the attributed popup window. No measurements are collected
on lifecycle events. Explicit Snapshot alone can publish OPEN/current popup binding.
Existing parent A/B identity, measured probe semantics and UI layout remain unchanged.

Trusted `collection: popup-ax` config supplies binding for actual popup window,
identity_path, plus parent_binding and parent_identity_path. Both live records must
match before and after acquisition, with independent public process/window checks.
Context surfaces explicitly contain popup first and parent second; target stays the
same known fixture incarnation. Canonical popup Surface gets actual native owner,
initiated_by parent and anchored_to the actual collected f02.popup trigger node.
A separate fixture-binding Observation/Evidence marks the relationship's explicit
program source, not a measured gap/arrowEdge or geometric equality. Parent trigger
is authorized dependent context; unrelated window/process content is not returned.

The same bounded WindowAX/property owners collect popup nodes and one parent trigger.
Actual SDK AX window/marker binding failure yields target_unresolved, closed/mismatched
current identities yield stale_target. Coverage remains partial; every requested
property retains known/unknown/unsupported states. Global focus/transform/layout gaps
are not inferred. Popup capture consumer is not connected in this first source slice:
it returns explicit unsupported rather than substituting parent isolated pixels.
That exact remaining Native capture connection can reuse current CaptureLifecycle
only after attributable popup-window live proof; no new backend is needed.

Next finite setup is own F02 A Edge popup→explicit Snapshot, using the reported
popup_binding only if status bound; actual CLI connection names popup/parent surfaces
and exact identity paths. Observe/inspect popup and trigger; then close via existing
Confirm and require old popup binding refusal, reopen/new Snapshot/new binding positive.
No B capture, source-app permission changes or stale-generation repair in place.
