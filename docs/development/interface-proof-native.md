# S01-Native — preparation and Stage A handoff

Verification/minimal test tooling for D02-PROOF, not a production adapter. Authority:
PLAN.UIB@1 and the bounded [native packet](../plans/ui-blueprint/packets/S01-native-proof.md)
with its [shared handoff](../plans/ui-blueprint/packets/S01-bridges.md). Root explicitly
allows independent fixture/build preparation now; common binding waits for its
exact committed Stage A SHA. Root subsequently supplied Stage A
`9d2df153abd2a7d7567100e06d4260e5edda3bb3` and the own-fixture runtime grant;
shared-source no-diff checks confirmed that baseline. Runtime has not started.

## Spec Basis

AGENTS → [spec registry](../specs/README.md), UIB.ROUTING@1 registry revision4 →
[decision route](../specs/development/decisions/README.md), UIB.DECISIONS@1 →
D02@1, D03@1, D05@1 and complete explicit dependencies. Reused current full F02
CONTENT@1 closure: BOUNDARIES, ROADMAP, MODEL, EXCHANGE, IDENTITY, GEOMETRY,
PROJECTIONS, FORMS, CACHE, ACTIONS, LIFECYCLE, NATIVE, PRIVACY, PERFORMANCE,
PILOTS, NATIVE-PILOTS and REUSE. Loaded missing RUST-BOUNDARIES@1, GOLDEN@1,
D01@1, C01-EVIDENCE@1, RUST.md and DEV.RUST@2. Following the D01/development
handoff also resolved D04@1, D06@1, D07@2 and C01-HANDOFF@1. Clauses are each
UIB.<ID>.CONTENT; route clauses are ROUTE. Direct request-driven clarification
remains applicable; no periodic collection or recollection on events.

RUST-BOUNDARIES contains an old DEV.RUST@1 link label; its actual linked owner
is DEV.RUST@2, explicitly selected by current registry/packet/D01. This is a
navigation-label discrepancy, not permission to change a spec or choose a toolchain.
No semantic fork identified. Excluded: detailed export, browser internals, mobile,
ML, future products, real-app operation and the four pending review P2 repairs.
No new dependency, Rust protocol owner or D05-RES policy is selected here.

## Immutable fixture preparation

Input is F02 `9a88b12b5855bac54bf04ba7b64d233df719ddec`; its current source/setup
files have no diff from that checkpoint. [prepare.py](../../tests/bridges/native/prepare.py)
extracts only committed Fixture.swift, Observe.swift and script/build.sh into a
unique task-temp, then runs the original compiler script. No mutable neighboring
Rust file or Cargo manifest is read or compiled. No app/helper executable launches.

| Identity / source | What is established before runtime |
| --- | --- |
| Own bundle | local.uiblueprint.f02.off / local.uiblueprint.f02.on |
| Executable and compile flags | F02Fixture; Swift6, arm64-apple-macos14.0; on adds PROBE |
| Windows | SwiftUI identifiers a/b; both titles F02 Synthetic; title is not identity |
| Explicit setup | Open A/B; normal stimulus; Snapshot button publishes one requested fixture receipt |
| Runtime binding still required | actual PID, launch time, CGWindowID, target/surface generation and current own manifest |
| Measurement source | external AX, rendered capture, opt-in probe remain separate; probe is not needed to fabricate external layout |

All products, compiler output and preparation hashes are temporary. Compilation
only establishes availability on the recorded compiler/SDK; it does not qualify
runtime permissions, macOS14 operation, common protocol or a pilot.

## Existing public boundaries and their limitations

F02 `Observe.swift` supplies relevant public mechanisms: AXIsProcessTrusted without
prompt; application AXWindows metadata filtered to the exact fixture identifier;
AXUIElementSetMessagingTimeout; selected AX attributes/actions; CG owner validation;
NSRunningApplication launch identity; ScreenCaptureKit desktopIndependentWindow
capture with audio/cursor/children excluded. No private proc_pidinfo flavor or
private AX window-number bridge. Current declared AX coverage is partial. Bounds
are accessibility bounds in top-left screen pt, not layout/hit/paint geometry.
Capture rect/scale/buffer facts do not establish a calibrated image transform.

The original combined file writer withholds completed AX until capture returns.
Its 45-second watchdog and 3-second AX traversal setting are historical diagnostics,
not D05 defaults for this proof. Concurrent captures leaked continuations and both
exited124. This remains an M01 gate; the D02 injected test must not claim repair.
A fresh explicit Snapshot is needed after setup/state changes; its own identity
oracle is not a general external-app targeting protocol.

## Real-case cautions supplied by the Mac co-author

Read only the existing repo-local [RC01 key](../../fixtures/real-world/mac-settings/expected-answer.md)
and [RC02 key](../../fixtures/real-world/mac-filters/expected-answer.md) as supplied
supporting context; no linked other-project source or application was inspected.
These keys are not new product contracts and are not agent-evaluation inputs here.

RC01 Settings is observed inside the learner window; the word dialog does not
prove an NSWindow, repeated Close labels are not identities and crop ROI is not
layout bounds. RC02 popup AX-to-CG association is inferred; its anchor is not
measured. Genre retains reported AX button roles with checked unknown despite
checkbox-like appearance; Director's omitted value is unknown, not known empty.
F03a's parent capture included popup pixels while isolated F02 capture excludes
children: retain actual capture kind/scope, never infer Surface absence from pixels.
F02 public AXPosition/AXSize and ScreenCaptureKit metadata have evidence only on
the owned fixture. Transfer to real applications and exact AX-to-pixel transforms
remain unverified. These cautions constrain future normalization, not new collection.

## Executed native proof on committed common support

The WIP source checkpoint was `3bdf34d48bd888c91e564ee82db8dda5143b48b1`.
After Integration supplied common support `73d772e97efcf550ea4a4d3e8480b56509ebc548`,
[build_support.py](../../tests/bridges/native/build_support.py) built its host and
validator from immutable Git archive inputs. Mutable neighboring CLI work was
excluded. No generic shared check was repeated: the committed three Rust/eight
synthetic checks supply wrong-version, malformed/oversize and generic lifecycle
negative coverage. The native-specific live chain adds fresh channel evidence.

[prove.py](../../tests/bridges/native/prove.py) wires the common executable directly:
descriptor → real attach/begin Ticket → native canonical ChannelResponse frames →
actual receive/complete/cancel/expire/detach → canonical retained documents.
[Collector.swift](../../tests/bridges/native/Collector.swift) writes AX before
waiting for capture. The helper does not create Ticket/parent-clock values or a
lifecycle protocol. Its independent clock domain labels actual source intervals.
Canonical channel frames also pass the same Rust validator used by the Web proof.

| Case | Actual acquisition | Common terminal / retained evidence | Injected portion |
| --- | --- | --- | --- |
| Live | AX sample + own-window ScreenCaptureKit PNG | Completed; both observed channels retained exactly | none |
| Capture timeout | AX sample, flushed before wait | TimedOut at parent deadline; exact AX retained, capture missing | pending capture instead of calling ScreenCaptureKit; late AX-success replay rejected StaleTicket |
| Cancel | AX sample | Cancelled; exact AX retained, capture missing | cancel after first admitted channel; late AX-success replay rejected StaleTicket |
| Detach | AX sample | Detached; exact AX retained, capture missing | detach after first admitted channel; late AX-success replay rejected Detached |

The selected control was `f02.sample.b` in exact fixture window3898, generation
`2C9BB0E1-BD7E-419D-87A5-A3D3573A9BB9` for this run only. Raw role AXButton,
accessibility name Activate sample, enabled=true and accessibility bounds
(705,388,173.5,48) screen pt were reported. No layout/hit bounds substituted.
Both channels retain partial coverage and consistency=unknown with separate API
read intervals. The captured 1100×1022 px image visibly contains owned Window B;
filter/window metadata is 550×511 pt, scale2. Exact AX-to-pixel mapping remains
unknown. The whole-window image scope is explicit; it is not a sample-control crop.

Explicit setup was separate: launch the prepared own F02 bundle, observe Window B,
press Snapshot. No collection request moved/resized/focused the fixture. CUA after
the four cases reported no tree change and the same Open A focus. That is bounded
fixture evidence, not full no-side-effect or probe-invariance acceptance.

## Resource, timing and cleanup accounting

The literal proof limits reuse common small-frame accounting, widened in duration
for D05 native budgets: frame65,536 bytes including newline; pending131,072 bytes;
1 in-flight;8 frames;3,000ms parent deadline;1s AX wait;2s capture wait;1s cleanup;
45s whole-proof process watchdog. These are explicit test inputs, not D05 memory,
retention or production defaults. The collector has an independent 8s last-resort
watchdog. Its injected wait performs no capture call or periodic collection.

Observed host terminal elapsed values: live301ms, expiry3012ms, cancel122ms,
detach121ms. These single proof traces are not D06 latency evaluation. The exact
workload differs from the earlier 75-node F02 baseline: one normalized AX node
with four selected fields, plus an explicitly attributed window capture.

All four helper/host PIDs are recorded in each proof.json. Live helper/host exited0;
injected helpers were terminated by this owner with SIGTERM and reaped, host exited0.
The timeout helper was stopped after its2s capture budget before common parent
expiry. Platform capacity-one Python reader threads were joined after producer reap; the
common wrapper's own stdin reader ends at its process exit per its README. The separate
owned fixture exited after the proof; plugin detach did not terminate it. Native
lane released. No other process, Simulator, permission or display was changed.

Minimal D05 samples are actual core-retained canonical documents:
`proof/live/retained/channel-0.json` (4597 bytes, one AX node) and
`channel-1.json` (3246 bytes, capture). They are selected-fragment examples, not
largest-native-graph or parsed-heap calibration. D05 remains Integration-owned.
Retention/paths/hashes are in the [receipt](../plans/ui-blueprint/receipts/S01-native-proof.md).

## Remaining boundaries

This provides the finite native D02 interface evidence; it does not independently
accept S01/P1 or freeze compatibility. The four review P2 findings remain untouched.
Injected capture timeout proves preservation of completed AX through the actual
common lifecycle and bounded owned cleanup, **not** repair of F02's real concurrent
ScreenCaptureKit continuation leak/124. M01 still owns production concurrency and
session isolation; P01 owns current one-shot probe off/on invariance and transforms.
No real PlayPhrase.me application was collected. No cadence or background service
was added. Future native setup must derive fresh identities, not reuse run IDs here.

## D05-Native actual window sample

Follow-up authority: [D05 platform packet](../plans/ui-blueprint/packets/D05-platform-samples.md),
D05@1 and the existing S01 full closure; [resource sizing requirements](../../tests/bridges/resources/README.md)
read before scope selection. This follows Native88ac606 and common support73d772e.
Five review repairs remain untouched. No fixture, schema, engine or CLI changes.

The scope/limits were recorded before launch/request in scope-before-run.json:
one fresh exact F02 Window B, normal state, no popup, strict AXChildren only;
160-node/depth9 ceiling. Core fields: role, description, value, placeholder,
enabled, focused, actions and accessibility_bounds; raw AXIdentifier/AXSubrole
metadata for source binding/redaction. No application fallback or new pixels.
Literal frame/output512KiB, pending1MiB, parent deadline1s, collector admission900ms,
four frames/one request and whole-driver watchdog15s. These are test inputs, not
D05 memory defaults, and were not increased after a failure.

The first request completed through the actual common Ticket and validator:

| Measurement | Actual result |
| --- | --- |
| Visited / returned / discovered unique AX handles | 76 / 76 / 76 |
| Child edges / duplicate-handle references | 75 / 0 |
| Maximum observed depth | 4 zero-based, 5 levels; ceiling9 |
| Known unread child entries / unreturned refs / queued handles | 0 / 0 / 0 |
| Unknown child-list reads | 0; unexposed visual/native nodes still unknown |
| Selected property availability | 384 known,165 unknown,58 unsupported,1 redacted =608 (8×76) |
| Coverage / capture | partial; no new pixel capture |
| Exact returned wire bytes, including newline | 337500 |
| Common retained serialization bytes | 338001; structurally equal to returned document |
| Parent frame accepted / terminal | 142ms /296ms, one trace, not a latency gate |

One secure Value property was marked sensitive/redacted and AXValue was not
requested for it. The eight requested properties exist for each returned node;
raw roles and separate unknown transforms remain. All nodes reference the same
actual selected surface. The local aliases identify actual CFEqual-distinct AX
handles within this observation only; they are not action refs or global IDs.

This exhausts the available strict AX child list for the chosen current window,
not all painted internals or every possible fixture state. The160-node ceiling is
not reached:84 nodes and deeper configured-bound stress remain unrepresented.
No node duplication, long-value manufacture, popup opening or UI changes for size.
Artificial worst-permitted documents remain separate Integration work. Full D05
coverage remains waiting_evidence; no production cap or acceptance is inferred.

Use the exact incoming wire for Integration sizing because JSON member order may
affect deserializer capacities; do not substitute its common-host reserialization.
Request/session/limits and actual common completion are retained alongside it.
Existing D02 capture metadata remains a separate earlier-session document and was
neither recaptured nor transplanted into the new graph. Fixture/support products
were reused; only the changed native collector was compiled. The common generic
checks were not rerun. Owned helpers exited0/reaped and platform readers joined;
CUA after collection was unchanged, then the own fixture exited and lane released.

Exact paths, hashes, cleanup and checkpoint status are in the
[D05-Native receipt](../plans/ui-blueprint/receipts/D05-native-samples.md).

### Focused secure-value check for the expanded field scope

Root requested a targeted privacy check because whole-window sizing newly requests
Value across a window containing secure input. The collector checks actual
role/subrole and the known fixture identifier before requesting AXValue; secure
Value is sensitive/redacted. No setter/modality is inferred from a role; Actions
are the actual AXUIElementCopyActionNames result. Missing/unsupported/nil results
do not become empty strings or false values.

A separate own-fixture run entered a nonempty synthetic canary, then explicitly
requested Snapshot and the same field/limit scope. The API reported
AXSecureTextField subrole; the normalized Value stayed redacted. Canary was absent
from exact wire, submitted/retained documents and diagnostics (0 diagnostic bytes).
Known empty text, known false flags and unsupported properties remained distinct.
The driver checks the returned bytes before writing/submitting them; the one-use
canary environment value is removed before helper/host launch. No raw canary or
raw diagnostic log is retained, and no pixels were collected.

Privacy case:75 nodes,333119 wire bytes; original sizing case:76 nodes,337500 bytes.
One AXGroup differs and focus context changed; the mechanism of that difference
was not isolated. Both counts remain actual, never padded to match. The original
sizing sample and limits are immutable. privacy-validation.json and privacy-cleanup.json
record the added check; the scoped privacy case does not close configured-bound
coverage or production privacy acceptance.

## M01 capture prerequisite repair

Authority: [finite M01 packet](../plans/ui-blueprint/packets/M01-capture-repair.md),
Restore under D01/D02@1 and D05@2/D05-MEMORY@1; current native closure retained.
The new retained-memory decision is read, not implemented in this helper. No UI,
shipping adapter, shared Rust repair, native framework or permission policy change.

A bounded reproduction on the same own F02 a/b windows re-established the real
problem: two original helpers both emitted checked-continuation misuse, wrote no
AX result, and were killed/reaped at the declared3s parent bound. Source showed
unbounded imported SDK async awaits and combined-only persistence. Explicit public
callbacks with an owned one-result gate removed continuation misuse and preserved
AX, but both screenshot callbacks still failed to complete within2s. That locates
the observed callback loss at screenshot acquisition; SDK internal cause is unknown.

The repair shares CaptureLifecycle between Observe.swift and Collector.swift.
It uses the same public ScreenCaptureKit backend, bounds callback completion and
capture admission, serializes only capture with a caller-owned flock, and resumes
once across success/error/timeout/cancellation. No lock is held over AX. Completed
AX is flushed/persisted first. An uncertain capture retains its lease until the
one-shot helper exits/reaps, preventing a replacement while its OS call may still
be active. Late replies cannot revive the local gate; canonical late-frame rejection
still belongs to the actual common ObservationSession. No new lifecycle protocol.

Real simultaneous requests with capture-only serialization: A exited0 with an
attributed1100×1022 isolated Window A image, audio/children off, unknown transform;
B exited2 with SCStreamErrorDomain/-3801 (UserDeclined). Both preserved AX and had
no continuation misuse. Preflight succeeded on the entered capture path. The API
code is an operation permission-required residual, not evidence the user clicked
refusal or that the platform has no capture capability. No B retry, new pixel
attempt after denial, backend switch, permission dialog action or settings change.
System prompt presence was not observed; only the own fixture was inspected.

Final independent/fault checks: B AX completed in143ms while A held the capture
lease in an injected stall; AX A survived timeout, both helpers reaped and the
lease was available afterward. On pinned common support73d772e, current live AX
survived injected capture timeout/failure, cancel and detach; retained canonical
completion equalled submitted data, late success replay rejected, platform readers
joined. Gate terminal/late-reply and recorded permission-code classification checks
are explicitly injected. The modified capture.py failure path retained the AX file
and returned nonzero rather than claiming full success. No new pixels in fault tests.

Bounds fixed before attempts: AX1s, capture/admission2s, parent3s, cleanup1s;
helper4s legacy-diagnostic backstop and canonical collector8s backstop. The common
fault reader allows2.2s only for delivery around the2s callback deadline, while the
actual parent3s deadline remains authoritative. The focused whole driver is30s.
These are test bounds, not retained-memory/RSS defaults or a new performance baseline.

Positive A pixels used the callback+lease build; the unchanged successful path was
not rerun after B denial. Final changes classify/audit failures and validate fault
modes; final affected fault paths were compiled/checked. Integration uses saved
support73d772e, not concurrently edited Rust; a recheck against the eventual saved
shared repair remains separate. Positive B capture and full M01/D05/P7 acceptance
remain open. Exact builds/requests/condition/cleanup and retained evidence are in
[the M01 repair receipt](../plans/ui-blueprint/receipts/M01-capture-repair.md).

### Saved-validator compatibility after the capture repair

At root's request, validator9ca645aab6816425ee64bb20eeb1201750d0d2b0 was built from
an immutable Git archive, locked/offline. All20 stored Session/Request/submitted
ChannelResponse/retained documents in final-fault-proof passed. Exact document
and binary hashes are in compatibility-9ca645a.json under the repair evidence root.
No SDK, pixel, fixture application or full-suite execution occurred. This closes
only that offline compatibility check; it is not a fresh live integration test,
B permission resolution or full M01/D05/P7 acceptance.
