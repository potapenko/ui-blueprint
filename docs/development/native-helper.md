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

Current fixture Snapshot uses the weak current public NSView.window attachment
from own popover content, requiring a visible positive own NSWindow. It publishes
that real physical window ID and independent popup generation; parent is never
renamed. Legacy NSApp marker scanning is not the binding authority. Missing weak
attachment/current owner remains unresolved. Shared native parent yields explicit
shared_native_window_requires_logical_surface_identity: current private IDs are
window-number based and cannot represent that case without a dedicated logical
Surface binding. It is refused, not assumed separate or silently broadened.

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
AX resolver is rooted in the authorized parent's actual AX window, and requires a
unique exact owner marker within a reported AXPopover ancestor. That semantic tree
placement is not physical-window evidence. The separate own direct content attachment
and live identity/CG owner checks supply physical attribution. Popup need not appear
as another AXWindows item. Duplicate/truncated/error/other-role markers refuse, with
no name/geometry/first-match fallback or unrelated-window content.
Actual SDK AX window/marker binding failure yields target_unresolved, closed/mismatched
current identities yield stale_target. Coverage remains partial; every requested
property retains known/unknown/unsupported states. Global focus/transform/layout gaps
are not inferred. Popup channel1 now uses the existing CaptureLifecycle with the exact
current popup windowID/PID and parent-owned capture lease. Both current records/public
bindings are verified before/after capture, PNG encoding and canonical publication.
Capture-only returns no AX nodes/anchor; explicit fixture metadata sources initiated_by.
Included Surface is popup, known parent is excluded, unresolved pair is empty; overall
coverage remains partial, crop transform unknown and audio off. Parent pixels are
never substituted. Source connection is not actual SDK/positive M03 acceptance.

Next finite setup is own F02 A Edge popup→explicit Snapshot, using the reported
popup_binding only if status bound; actual CLI connection names popup/parent surfaces
and exact identity paths. Observe/inspect popup and trigger; then close via existing
Confirm and require old popup binding refusal, reopen/new Snapshot/new binding positive.
No B capture, source-app permission changes or stale-generation repair in place.


## PNG image ownership — Native acquisition@2

Explicit user direction retains every system-temp image staging/partial/final path
and its containing directory on success/failure/stale/helper death. PNG writer requires
.png destination and system-temp image directory; exclusive completed link keeps the
original staging name. Failed Finalize/byte cap never produces an advertised final.
Descriptors close and helpers reap normally; non-image sidecars retain own-partial
cleanup. No retention flag, permission expansion or persistence directory is added.
Use separate task-temp image and non-image build/input destinations, and exclude all
image partials and containing directories from cleanup. Do not recursively remove
an operation root containing images. Synthetic test images also remain.

Focused writer check (real tiny CGImage/ImageIO, no AX/SCK): compile NativeAcquisition,
NativeJSON, NativeArtifacts and tests/bridges/native/acquisition/ArtifactChecks.swift;
run its binary with the existing profile.json and a new0700 system-temp image directory.
It checks retained same-inode staging/final, overflow/failed Finalize without final,
no overwrite, system-temp/name admission, descriptor release and non-image cleanup.
The updated monolithic Checks.swift PNG failure expectation remains compile-checked;
unchanged numeric/AX series need not be rerun for this source delta.

PopupChecks substitutes only the existing platform capture call inside the actual
Collector.popup path, alongside existing AX/public-binding test substitutions. It
checks exact window/PID dispatch, canonical capture/context/provenance/coverage,
stale-before-dispatch, identity race, permission/timeout/cancel/unresolved and low
output refusal. Test substitution is not exposed through helper config/runtime flags.
Compile with existing HOST_HELPER/CAPTURE_LIBRARY/IDENTITY_TEST source selection and
validate every returned canonical document with the current saved-source Rust validator.
Actual capture remains separately activated after source review: one fresh own popup
AX+capture Observe with separate before/after OPEN/generation/parent identity values,
inline retained PNG and exact worker/helper/fixture cleanup under fixed budgets.


## Native action held-object owner (integration pending)

NativeHeldAction retains only the first action's exact target and independent expected
result AX objects under one trusted window/current binding. It receives worker-owned
macos.ax keys from the originating action-capable observation, never reconstructs an
old ref from an identifier. validate resolves unique current connectivity and compares
CFEqual against the held window/target/result; any remount/generation loss refuses.
read returns the requested actual node/property evidence with the same canonical key;
identity validation surrounds the read. invalidate rejects later access; dropping the
session owner releases SDK references. Caller supplies explicit node/depth budgets and
current process/window/surface check, with existing acquisition/text/shape/deadlines.

The owner has no delivery/nonce/transport authority. Core's selected attached-session
exchange must hold it through Observe/Prepare/Act/result read and retire it on helper
reap. Legacy one-shot Observe keys remain non-actionable. HostProtocol/HostHelper still
need that concrete integration; no hidden daemon or cross-CLI process retention.
The target checkbox's measured non-settable AXValue remains Unsupported for Setter.
Semantic Activate on f02.sample.a requires actual parent permit and independent Count
Expectation result, not AXPress return alone. A02 supplies the common action kernel.

Focused synthetic owner check: compile existing Native acquisition/JSON/artifacts,
Observe/Collector/WindowAX/HostProtocol sources plus NativeHeldAction.swift and
HeldActionChecks.swift with CAPTURE_LIBRARY/HOST_HELPER. Invoke with profile.json.
It exercises stable target/result keys and source reads, same-ID target/result/window
remount refusal before property read, current-generation refusal and invalidation.
No AXPress/setter/runtime or production action acceptance follows from these checks.


## Ordinary CLI component geometry (G03)

A fresh F02-on explicit Snapshot provides the trusted current window identity and
three measured icon/text/container rectangles. Configure existing native_fixture
provider with channel mask4, collection sample, exact identity_path and probe manifest/
Snapshot request/source revision/uptime, registered acquisition_limits. Request
opt_in_layout_probe/layout_bounds/design/cached_allowed with160/depth9/512KiB/1s.
Probe import retains cache/unverified fixture clock and unknown screen transform.
AX sample uses a separate mask1/external_semantics/current_required Request with
role/accessibility_name/enabled/accessibility_bounds and collection sample. Do not
combine its ax-screen pt bounds with fixture-local layout bounds without a transform.

Once these ordinary trusted connection/Request files are prepared, use the public
binary, not the ignored native_fixture test caller or a graph-extraction script:

```sh
uiblueprint observe --connection probe-connection.json --request probe-request.json --worker /absolute/session-worker --max-input-bytes 2097152 --max-output-bytes 524288 > probe-observed.json
uiblueprint inspect --snapshot probe-observed.json --ref '{"namespace":"macos.swiftui.probe","key":"f02.sample.a.container"}' --view design --max-input-bytes 2097152 --max-output-bytes 524288
uiblueprint measure --snapshot probe-observed.json --query gap-query.json --space f02-fixture-local --max-input-bytes 2097152 --max-output-bytes 524288 --json
uiblueprint diff --before before-probe-observed.json --after after-probe-observed.json --max-input-bytes 2097152 --max-output-bytes 524288 --max-entries 32 --json
```

Observe exit4 is expected for honest partial coverage even when the channel is observed;
check its canonical status and real cleanup. Inspect/known Measure/complete Diff exit0.
ANALYSIS@2 directly accepts the original observed ChannelResponse, retaining the source
Snapshot. No manual Snapshot extraction, timestamp rewrite or mutation-ready refs needed.
Gap-query is analysis0.2 geometry_query, scope matching Request, targets
macos.swiftui.probe:f02.sample.a.icon and .text, operation gap/length/pt, anchors
layout_bounds/f02-fixture-local(local,pt,top_left)/x with fractions1 and0. Applies_when
fields null; no expectation/expected amount. Width/height queries select .container,
operation width/height, one layout anchor in that same space. Engine computes facts.
Existing Change layout then fresh Snapshot supplies after-state for recorded Diff.
Actual G03 run confirmed component173.5×48→231.5×62pt and measured gap8→18pt; these
are controlled F02 facts, not real RC03 inner geometry. No capture/input gate implied.


## Ordinary Mac AX geometry — native_ax

The existing public observe command also accepts trusted provider
{backend:"native_ax",helper_executable:"/absolute/native-helper",configuration:"..."}.
AX-only channels are fixed; no channels field, fixture identity_path/probe file or
F02 bundle/window identifier restriction. Build HostHelper with NativeFocusedAX.swift
alongside its existing sources. Old native_fixture configuration remains unchanged.

Private configuration is exactly collection="focused-ax", process={pid,bundle_id,
launch_time}, scope_id and acquisition_limits. Obtain incarnation from public
NSRunningApplication for the explicitly selected PID, not a UI label/title. Target
id is macos-pid-PID; generation is PID:launch_time using its full reported value.
For fresh Request.request_id R, its sole Surface is id=ax-focused-R/generation=R.
This names the public AXFocusedWindow observed during that request, not CGWindowID
or a stable cross-request/action identity. Request external_semantics/current_required,
explicit supported fields and existing160/depth9/512KiB/deadline limits. Session
capability describes partial AX read only. Common CLI trusted connection/profile and
Attach remain explicit; this extension does not discover or launch user applications.

Helper checks process/bundle/launch and public AX owner, holds the focused-window
object during collection and compares the same AXFocusedWindow before/after reply.
No activation/focus/scroll/input/TCC request, title/rect/order matching or other-window
fallback. Unresolved selection/currentness refuses, denied AX yields permission_required.
Secure AX values remain redacted through the existing WindowAX owner. Each requested
property has its real availability; no hidden layout/paint/hit geometry is fabricated.

Feed original observed response to existing inspect design and measure; select exact
SourceKey from that response. Accessibility bounds are ax-screen/screen/pt/top_left;
pixel transform unknown. NativeFocusedAX collects no images/probe/CG metadata and
exports no BackendRef. Optional own measured probe remains the separate fixture path.
Cross-request Diff requiring stable Surface continuity is not claimed by these ephemeral
window identities; this first generic route supports within-record bounds/relationships.

Actual generic-route qualification on own unmodified F02 required NO Snapshot action:
run directory stayed empty, no fixture/probe/current files existed. Observed75 AX nodes,
selected button rect801,364,173.5,48pt; ordinary Inspect and Measure returned173.5×48pt.
This is public AXFocusedWindow geometry via the generic route, not general Mac app
compatibility or a claim of stable identity/capture mapping. See P01-probe-source receipt.


## Developer first-use example: PID and component name

From the repository root, set TARGET_PID to the explicitly chosen already-running
Mac app and COMPONENT_NAME to its exact reported accessibility name. The example
never launches/activates/focuses/resizes the target or requests permissions. It gets
PID/bundle/full launch incarnation through helper describe-process, prepares private
canonical inputs in system temp, calls public Observe/Inspect/Measure, then removes
its named non-image JSON files. No fixture identity/probe file or hand-authored JSON.

Build the existing binaries once (pinned Rust/Swift/Xcode setup applies):

```sh
BUILD_OUT="$(mktemp -d "${TMPDIR:-/tmp}/uib-native-example.XXXXXX")"
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/host_observe.py build-geometry --output "$BUILD_OUT"
CLI="$BUILD_OUT/uiblueprint"
WORKER="$BUILD_OUT/session-worker"
HELPER="$BUILD_OUT/native-host-helper"
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/host_observe.py geometry --pid "${TARGET_PID:?Set an explicit running app PID}" --name "${COMPONENT_NAME:?Set the exact component name}" --cli "$CLI" --worker "$WORKER" --helper "$HELPER"
```

Typical controlled example output:

```text
component="Activate sample" source_key={...}
accessibility_bounds x=801 y=364 width=173.5 height=48.0 units=pt space=ax-screen
coverage=partial source=saved_observation transform=unknown observe_wall_ms=285.92
hidden layout/paint/hit/clipping unavailable; AX refs are observation-scoped
```

This is a developer example, not an installed-release CLI. Selects only from the
original returned Snapshot, never a new UI search; zero/multiple name matches return4
and report need for exact SourceKey (candidates for ambiguity), never choose by order
or coordinates. Such keys describe that recorded observation only; retain an original
Observe response when using public inspect/measure for further explicit selection.
Unavailable AX/selection/bounds returns its actual refusal without retry or dialog.
Default fields are role/accessibility_name/accessibility_bounds, no user Value/secret.
Width/height come from Rust Measure; x/y from reported AX rectangle, not inferred layout.
Own-F02 check used this mode with no Snapshot action/fixture files; real apps are not
launched by this procedure. Build products are temporary developer tools; clean owned
non-image products after use, preserve any images and their containing directories.


## Selected Web+Mac local build

One entrypoint builds the current committed HEAD with locked/offline dependencies:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/host_observe.py build-geometry --output "$BUILD_OUT"
```

BUILD_OUT must be an explicitly supplied existing absolute directory. The command
publishes only uiblueprint, session-worker and native-host-helper, refuses any existing
product file/symlink and preserves unrelated files. Build stages/module caches stay in
system temp and are cleaned as owned non-image files; image files/containing dirs are
retained if encountered. Failed publication removes only its own newly created products.
Committed-source archive ignores uncommitted WIP, prints source revision/product paths/
SHA256. This is reproducible source/recipe selection, not a bit-identical-binary guarantee.

Selected configurations are CLI features web,macos and worker feature web on supported
arm64 Mac; default/core-only feature semantics and manifests do not change. Requires
installed pinned Rust1.96.0, Xcode/Swift SDK and cached locked crates. rustup run plus
Cargo --offline never installs/downloads a toolchain or dependency; missing prerequisites
fail. No signing, global installation/PATH changes, app/browser launch or permissions.
Ordinary saved analysis works from this CLI without a model or browser session.

Use the produced paths for the Native developer example above. For existing Web
trusted connection/request inputs, retain the public observe grammar:

```sh
CLI="$BUILD_OUT/uiblueprint"
WORKER="$BUILD_OUT/session-worker"
HELPER="$BUILD_OUT/native-host-helper"
"$CLI" observe --connection "$WEB_CONNECTION" --request "$WEB_REQUEST" --worker "$WORKER" --max-input-bytes 2097152 --max-output-bytes 524288
```

The browser and explicit authorized CDP endpoint/tab/document are external prerequisites,
not discovered/installed by this build. No .npm cache executable path is embedded.
This local distribution preparation delivers selected geometry executables; it is not
full P5/P7/released-product, general OS compatibility or license/signing acceptance.


## M04 measured scroll component

Own F02-on Snapshot additionally records scroll viewport and authored row0 anchors
through the same GeometryProxy in f02-fixture-local pt/top_left. To select only these
two measured nodes, set existing trusted probe Request/config scope_id=f02.scroll.a
(or .b for explicit own B). No new flag/config field/public CLI grammar. All other
sample scopes retain the original icon/text/container map, keys and output. Private
scroll_source_declarations names f02.scroll.a and viewport/row.0; measured rectangles
live separately under probe.scroll_layout_bounds. Off/absent/stale import refuses.

Use original opt_in_layout_probe/cached_allowed/layout_bounds response in public
inspect/measure/diff. Keys macos.swiftui.probe:f02.scroll.a.viewport and .row.0 have
stable fixture generation/authored identity. Existing Scroll end changes actual row
layout position; Snapshot remains explicit. Rust inside/intersects compares measured
rectangles in the same local space. A viewport rectangle is not claimed visible/paint
clipping, glyph bounds or occlusion. Existing Move can compare the same local records
without treating screen translation as changed local offsets. Old sample and P01
seam remain unchanged.

M04-T adds a sourced `f02-fixture-local → f02-scroll-a-local` (or b) Transform
for this scroll scope. The destination is explicitly defined at the measured
viewport upper-left, local/pt/top_left. The same public GeometryProxy anchor
subscript supplies both rectangles; the authored fixture has no intermediate
rotation/scale. The translation `[1,0,0,1,-viewport.x,-viewport.y]` is derived
Evidence, not a modifier value or guessed screen/window origin. Rust performs
the actual conversion and measurements. Raw layout rectangles remain unchanged.

The explicit Snapshot's `probe.scroll_local_mapping` carries source method,
snapshot_request, source_revision, environment_revision, measured viewport and
actual SwiftUI display_scale. Set Request Context.environment_revision to that
exact mapping revision. Import checks matching Snapshot/revision/environment,
exact viewport equality, finite positive size/scale and existing process/Surface
binding. Missing or mismatched mapping emits Transform unknown while retaining
measured layout. Scale is context only: it never multiplies points into pixels.
Imported observations remain cache/unverified with original fixture clocks.

Select `--space f02-scroll-a-local` in existing measure or `diff --geometry`,
with ref `{"namespace":"macos.swiftui.probe","key":"f02.scroll.a.row.0"}`
and frame-kind layout_bounds. Each side uses its own original mapping/environment.
If mapping is absent, no destination Space is present: CLI returns2/unknown_space
before calculation, and the source Transform remains explicitly unknown. It does
not invent the missing transform or gain coordinate-action authority. Screen,
AX-to-local, pixel/capture mapping and cross-display qualification remain unknown.

Focused synthetic verification (executables built using the Native build recipes):

```sh
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/local_transforms.py --checks "$NATIVE_TASK_TMP/probe-checks" --validator "$NATIVE_TASK_TMP/target/debug/uiblueprint-validate" --cli "$NATIVE_TASK_TMP/target/debug/uiblueprint"
```

The test removes only its temporary JSON files. Actual owned-fixture move/scroll/
resize results and limitations are in the [M04-T receipt](../plans/ui-blueprint/receipts/M04-local-transforms.md).


## M05 explicitly linked AX and probe design observation

The existing native_fixture/sample Observe supports one composed design request:
provider channels=5; operation channels=[external_semantics,opt_in_layout_probe];
Context fields=[role,accessibility_name,enabled,accessibility_bounds,layout_bounds],
projection=design, freshness_policy=cached_allowed. Both session capabilities and
request channels must explicitly authorize both sources. Existing single-source
requests keep their shapes and capabilities; probe-only never performs AX reads.

Trusted binding/identity/probe manifest config is unchanged. A fresh explicit
fixture Snapshot declares sample_association with AX namespace/key, probe namespace,
parts icon/text/container, represents relation and f02_explicit_ax_probe_mapping.
Collector validates that exact declaration before acquiring the unique actual AX
sample through the existing bounded collector, and revalidates the same AX window.
No name/box equality creates an association. Missing/stale/ambiguous/unavailable
source publishes no fabricated mapping; separately requested AX remains its own slot.

Public Observe emits ordinary AX ChannelResponse and probe-wrapper ChannelResponse.
The latter carries four distinct nodes, one reported ComponentMapping and three
sourced represents relations. EXCHANGE@2 admits exactly this design AX/probe pair;
request-aware worker validates every nested channel's authority. AX retains live/
current helper clock; imported probe retains original cache/unverified fixture clock.
Unsupported per-source fields stay explicit. No new action refs/transform/calibrated
pixel/hit/visible-region claim. Use the unchanged composed response directly:

```sh
"$CLI" inspect --snapshot "$COMPOSED_RESPONSE" --ref '{"namespace":"macos.ax","key":"f02.sample.a"}' --view design --max-input-bytes 2097152 --max-output-bytes 524288
```

The developer fixture must have a positive public process launch time matching the
Snapshot before Observe. LaunchServices-backed app launch supplies that identity;
a direct-executable launch observed with launch_time=0 is refused, never repaired
by substituting a timestamp. Actual saved-source Observe/G11 passed; details and
initial refused setup are in the existing P01-probe-source receipt. This is own-
fixture explicit design linkage, not arbitrary-app SDK or full M05/P7 acceptance.

## M03-C known full-window capture mapping

The existing capture producer now emits a known crop_transform only when public
SCContentFilter.contentRect exactly matches SCWindow.frame and addressed window-server
bounds, the API's pointPixelScale produces the admitted natural pixel dimensions
without rounding, and window/display geometry is unchanged across capture. The
full-window filter excludes shadows/children/audio, explicitly preserves aspect ratio,
uses the complete destination rectangle and disables upscaling. Unsupported metadata
combinations retain unknown; no guessed titlebar/content origin or PNG-derived scale.

The directional canonical transform is ax-screen (screen/pt/top_left) → a unique
capture-OBSERVATION-pixels (surface/px/top_left), `[s,0,0,s,-x*s,-y*s]`, bound to
Target/Surface/environment and derived ScreenCaptureKit Evidence. Source geometry
remains unchanged. Popup rechecks current public geometry before/after PNG encoding
and immediately before returning its frame; change returns stale_target with no
advertised payload. Current process and both own parent/popup identities still
revalidate separately. Endpoint equality is not proof of atomic acquisition or
absence of an intervening move-and-return. Recorded mappings remain historical,
never current pointer/action authority. Observe itself creates no action refs.

Optional existing acquisition_evidence records raw window/filter rect, reported
scale, actual dimensions and helper-local interval in bounded capture-metadata.json.
AX and capture remain separate ChannelResponses with separate clocks and partial
coverage. Parent is explicitly excluded from isolated popup pixels. A known crop
transform does not add AX fields to capture or claim synchronized channels/occlusion.

The focused developer qualification consumes an explicitly saved own-popup pair:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/capture_mapping.py --pair "$OBSERVE_NDJSON" --cli "$CLI" --validator "$VALIDATOR"
```

It creates a temporary standalone Snapshot with the original AX nodes and actual
capture Observations/frame, retaining all original clocks/properties/coverage; it
is not a mixed-channel Observe response. An explicit EvaluationInput supplies the
recorded crop transform to the unchanged Rust measurement API. Rust checks popup
and button dimensions/insets and rejects missing, wrong-Surface, retired-generation
and wrong-environment mapping. No supplemental observation is invented in evaluation.
This is a saved-data test consumer, not a new CLI composition command or live sync
service. Real own-fixture evidence and retained PNGs: [M03-C receipt](../plans/ui-blueprint/receipts/M03-capture-mapping.md).

## Attached Native forms (M02-N)

The additive [Native session contract](../specs/product/native-session.md) exposes:

```sh
uiblueprint native-session --connection /absolute/connection.json --worker /absolute/session-worker --duration-ms 120000 --max-input-bytes 2097152 --max-output-bytes 524288
```

Build CLI with `macos`; the existing `native_fixture` connection has AX channels1.
Set helper configuration `collection:"form"`, explicit `form_session_ms`≤300000
and `form_identifiers` containing1–8 unique actual fixture AXIdentifiers. Other
binding/acquisition fields remain mandatory. No ordinary-process input is exposed.
Send one strict bounded JSON line per explicit operation on stdin:

```json
{"request":"/absolute/observe.json"}
{"request":"/absolute/prepare.json","source":"/absolute/observed.json","expectation":"/absolute/expectation.json"}
{"request":"/absolute/act.json","source":"/absolute/prepared.json","expectation":"/absolute/expectation.json"}
```

Requests, source and Expectation are existing canonical documents. The caller reads
stdout records and creates the next explicit request; this is not an automatic
scenario runner. Partial AX observations retain their coverage and can be used for
an exactly bound control. Every action needs a caller condition. IME/composition
and selection mutation remain unavailable. Secure values are
redacted; available selection is UTF-16. Known checkbox0/1 maps to Checked, but its
AXPress never substitutes for the unavailable SetChecked setter.

One parent-registered helper owns original CF handles and fresh UUID/session/helper
keys until shutdown. No refs survive CLI exit. Semantic Focus uses the parent Focus
lane, checks the prior owner, activates the exact process, then sets AXFocused.
Fill uses a reported settable public AXValue. Activate uses a reported AXPress and
an independently held caller result. Keyboard Type uses bounded public Unicode
key-down/key-up with clear modifier flags; the CG API gives Accepted, not a delivery
acknowledgement. Even an observed matching value cannot convert it to Confirmed;
the canonical outcome stays unknown and the session stops with4. No implicit Enter,
retry, rollback, locator repair or business-success inference occurs.

The existing parent permit is the only delivery authority. Phase0/1 are read-only;
phase2 requires the current exact one-use nonce; phase3 reads fresh source state.
Fixed parent ingress is returned after forwarding rather than newly allocated.
Operation deadlines and idle residency stay separate; parent EOF and per-operation
watchdogs stop the owned helper even during SDK work. Cleanup retains the existing
one-second bound/quarantine. The input application is never part of helper cleanup.

[Author result and remaining qualification](../plans/ui-blueprint/receipts/M02-native-workflow.md).

### Protected input (V02)

Existing core0.1 intent `{"intent":"fill_secret","secret_reference":"opaque-once"}`
with modality `setter` now has a bounded own-fixture path. A known secure AX
role/subrole AND current AXValue-settable capability are required. No raw Fill/Type
secret, keyboard fallback, clipboard, automatic submit or retry. Add the following
optional object to the trusted private helper configuration (not to ActionCase):

```json
{"protected_input":{"reference":"opaque-once","action_id":"protected-step","identifier":"f02.secret","path":"/absolute/caller-owned-input","trace":false}}
```

The actual Action.id must equal action_id. Source is a caller-owned same-uid0600
regular file with1..4096 UTF-8 bytes (tighter acquisition limits apply), no control
characters; final-component symlinks/special files refuse. Prepare never opens it.
Only permitted delivery reads it once, after exact session/ref/generation/window/
input-owner validation and parent nonce. Data+String copies charge the existing
Native acquisition budget; buffer clears on exit and core dumps are disabled before
read. The product does not create/delete the caller source. Swift/CF SDK copies are
bounded temporary delivery owners, not a universal memory-erasure claim.

Use an explicit Expectation on a DISTINCT public held result in the same Surface.
Own fixture `f02.secret-status` exposes AccessibilityName `Protected input: empty`
or `Protected input: received`; this proves presence only, not exact secret equality.
Available intent remains `fill`; canonical versions/shapes and old commands remain.
Optional trace emits only fixed stage codes to helper stderr (ordinary host discards
it). No raw errors, paths, secret values or command dumps. Consumed reference,
wrong binding, unsupported setter, stale owner and uncertain outcomes stop the session.
Resident snapshots now have increasing checked revisions, so existing CacheStore can
retain their actual history without conflicting revision1 records.

[Full canary results, reproduction and limits](../plans/ui-blueprint/receipts/V02-protected-input.md).
