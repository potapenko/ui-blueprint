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

## Next implementation after the committed Stage A SHA

The following is the bounded execution plan, not an implemented wire schema:

1. Read the committed Stage A schema/plugin API documentation and relevant exact
   types/tests at the supplied SHA. Confirm ancestor/current-file provenance before
   invoking a shared build. Never accept results from Integration's uncommitted files.
2. Bind a narrow Swift test collector to the canonical envelope, preserving
   request/session/target/surface IDs, selected fields, actual raw AX roles,
   per-channel capabilities, source/time/clock domain and partial coverage. Use
   actual observed properties only; missing requested values remain explicitly
   unavailable under the canonical type. No second protocol declaration.
3. Emit and flush the completed sanitized AX channel before capture begins. The
   Rust common boundary owns aggregation, monotonic deadline and terminal state.
   Transfer remaining duration to helper clock; never compare process clock readings
   as a shared monotonic timeline. Use D05's explicit 1s AX / 2s capture test budgets
   and Stage A's supplied outer/cleanup/frame limits, not new production defaults.
4. After a separate runtime grant, launch only the prepared owned fixture, establish
   exact A/B setup through Computer Use, explicitly request Snapshot, then perform
   one common live request. Keep setup mutation separate from read-only acquisition.
   Validate actual AX and independently attributed capture using the same committed
   Rust validator used by the Web proof. Preserve unknown transforms and capture scope.
5. Through the same available common boundary, exercise incompatible version,
   malformed/oversize framing, cancel/detach and late-response rejection. Reuse
   Stage A's focused tests where applicable and label live versus injected evidence.
6. Inject capture timeout **after real AX completion**. The parent must retain the
   emitted AX result, report capture timeout and reap only the owned hung helper.
   Deliver an injected late frame to the common boundary after cancel/detach and
   prove rejection. This is not a hung-OS/capture-concurrency repair claim.
7. Stop the proof's own resources, release native lane, preserve minimum evidence,
   then request the serialized Git lease for own-file commit and origin/master push.

Detach concerns session helpers/handles, not termination of the fixture or user
apps. Closing a test-created fixture is a separately owned harness cleanup action.
The bridge must not recollect between explicit requests or hold a global lock
across AX/capture. Collection/channel preservation are the finite D02 consumer;
input correctness, full privacy/lifecycle and probe invariance stay with later owners.

## Minimal consumer contract for Integration

The canonical ObservationSession API is available in Stage A; an executable is
not an additional product gate. Integration owns reusable framing/lifecycle test
support, as requested by root, whether module/harness or executable.

Native producer inputs: canonical Request Document as one bounded UTF-8 JSON line,
explicit own manifest/output paths, test mode and **actual** Ticket.sequence from
the common host. A descriptor-only invocation returns canonical Session Document
from public permission preflights and exact own-target binding, without acquiring
AX tree/pixels. Host attach+begin then supplies the real Ticket for collection.
Collector streams canonical ChannelResponse Documents, AX flushed before capture.
No worker invents Ticket values or parent clock readings.

Consumer operations needed: attach(descriptor, explicit limits/parent clock domain),
begin(request, current parent reading) returning Ticket; bounded line admission to
receive(ticket, frame, current parent reading); complete/cancel/expire/detach;
terminal channel/missing-channel summary and rejection of late canonical frames.
Host uses a real monotonic clock, reaps only its owned helper on timeout/cancel,
and runs shared wrong-version/malformed/oversize/late-response checks once. Native
provides launched child handles, live/injected frames and channel-specific evidence;
it does not duplicate the common orchestration. D05 channel tests use 1s AX/2s
capture; outer/cleanup/frame/admission bounds remain explicit shared test inputs.

## Exact dependencies requested from Integration/root

- Received: Stage A `9d2df153abd2a7d7567100e06d4260e5edda3bb3` and explicit
  own-F02 runtime lane grant. Runtime still requires checking current reservations.
  Fixture preparation alone has no Stage A acceptance claim.
- Integration: canonical request/channel/completion and attach/session types;
  documented validator invocation and plugin boundary entrypoint; one native-ready
  example with explicit outer deadline, cleanup bound and frame-size limit.
- Integration: actual common driver or test entrypoint for streaming channel
  completion, request cancellation/detach and stale/late-frame refusal. If the
  committed boundary cannot express these, return that exact missing operation to
  the shared owner. Do not build a competing lifecycle/protocol in this directory.

No live request, framing negative, channel timeout, cancellation or detach proof
has run in this preparation phase. Their status remains waiting for the reusable common test-support handoff,
not mock pass or P1 interface freeze. A module/API harness is sufficient; no
standalone binary requirement is invented.
