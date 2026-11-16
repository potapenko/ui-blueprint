# External SwiftUI capture candidate

**One source-pinned Services Settings qualification only.** The c89b277 path
remains withdrawn. The efd27a9 method has independent review closure for the
qualified image profile, not general product/B0 acceptance. The current candidate
admits only its source-pinned request ID/PID/bundle/birth/window plus owned fixtures;
other products refuse before debugger startup and at capture entry. A new identity
requires a fresh owner handoff; callers must not edit the gate or reuse old recipes.
No new installed version or production release is implied.
The [contract](../specs/product/swiftui-debug-capture.md) distinguishes this raw
field channel from the existing bounds-anchor SDK.

Current admitted request: `services-settings-qualification-1`, PID45908,
`me.playphraseme.reference.services.fgb7o3fd`, native birth1794870752.9841919,
window22500. The source owner revalidates this tuple before dispatch. This recipe
must not be reused after a restart or for another window/state without handoff.

The caller owns an already-running app and exact window. This command does not
launch it, set its environment, activate it or request OS permissions. Current
profile: macOS27.0.1 arm64, Xcode LLDB2103.0.34.407. A compatible installed native
bundle supplies the independently verified public process metadata helper.
The native search stops at outermost getter-capable hosting roots; it does not
scan all native controls below a selected host. Multiple outer roots still refuse.
Nested native/independently hosted content is not thereby declared measured.

An example request (replace the illustrative identity with independently verified
values; a PID or window title alone is insufficient):

```json
{
  "request_version": "1.0.0",
  "request_id": "owned-request-1",
  "pid": 123,
  "expected_bundle": "me.uiblueprint.external-layout.canary.example",
  "expected_launch_time": 1794836979.5,
  "window_id": 456,
  "limits": {
    "max_nodes": 256,
    "max_depth": 256,
    "max_output_bytes": 1048576,
    "deadline_ms": 10000
  }
}
```

The product case uses the exact frozen request supplied with its handoff, not
these illustrative fixture values. Run once using standalone Python in a retained
terminal/session; do not wrap it in a timeout that kills the process tree. Revalidate
identity immediately before dispatch. First refusal ends the case: no retry,
quota increase or blind SIGCONT. Preserve any running launcher/debugger on an
unsafe result and report the recovery receipt to the source owner.

Use an existing private system-temp output directory:

```sh
python3 -B plugins/macos/swiftui_debug_capture.py \
  --request "$UIB_REQUEST" --out "$UIB_OUTPUT" \
  --bundle /Users/eugenepotapenko/.local/share/uiblueprint/versions/0.3.2
```

Successful output is `snapshot-REQUEST.json` (canonical core0.1) and
`manifest-REQUEST.json` (manifest_version1.0.0), both exclusive0600. The manifest
names the output and cleanup outcome. The replacement reads only qualified type/geometry fields; it never calls the
full serializer or reads model/value Any payloads. Exit0 means a partial Snapshot was published;2 is invalid input or
budget refusal;4 is unavailable/stale/interrupted;1 is setup/IO failure.

`swiftui.debug.layout` nodes have observation-scoped keys and reported nominal short type names
(without generic arguments),
not persistent component IDs. The getter's fractional position/size is preserved;
it is not silently rounded to the SDK's resolved bounds-anchor channel.
Each node has a separate source-local space. `measure` can compute its width or
height; an inter-node gap returns `missing_transform` until a real mapping exists.
Semantic part correspondence, root/window/PNG transforms, hit/paint/visibility and
glyph/caret/selection are not supplied by this candidate.

`native_window_state` reports independent `is_key_window`, `is_main_window` and
`app_is_active` getters in the selected stopped-process interval. These are
AppKit-reported flags with unknown consistency, not Input focus, AXFocusedWindow,
an OS foreground guarantee or evidence that outside events were processed while
stopped. The application is never activated to obtain the values.

Recoverable refusal after a completed call yields no Snapshot and
`detached_restored`; this qualification nevertheless stops on its first refusal.
A further attempt requires its own explicit finite handoff.
The withdrawn implementation used `owner_recovery_required` after an interrupted
getter. It did not prove that the target stayed stopped after debugger teardown.
Do not use it for product recovery or treat a blind SIGCONT as safe restoration.
The acquisition budget starts immediately after successful attach and includes
post-attach identity checks. Cleanup has a combined2s allowance and two reserved
calls within the total256 for owned-array release and pool-pop. The framework getter's allocations and synchronous
debugger/OS latency are opaque; no global memory or whole-attempt wall-clock limit
is claimed.

This LLDB target rejects `Detach(True)`. An interrupted call now keeps its owning
LLDB process alive. `recovery-REQUEST.json` contains the exact PID, generation,
random owner token and command filename. Invalid/partial/foreign commands keep
holding. The checkpoint also fixes `termination_action`: fixtures use
`terminate-owned-fixture`, the admitted source uses `terminate-admitted-source`.
Observe never creates this command. For an unsafe product outcome, first present
the exact held PID/bundle/generation and possible loss of unsaved changes in this
copy, and obtain a separate explicit human confirmation to forcibly terminate it.
Diagnostic launch or reopening approval is not that confirmation. No advance
permission for hypothetical termination is required for ordinary capture.

Only AFTER that confirmation, the source owner writes exactly the trusted
checkpoint's PID/generation/owner plus `action: "terminate-admitted-source"` to
its indicated command filename, exclusive0600 in the same private directory.
The attached debugger executes SBProcess.Kill once and waits for an observed exit;
no external signal/reattach or debugger teardown substitutes for this operation.
The frozen handoff includes the exact conditional command for this target. If the
command fails or outcome is unknown, keep holding and report; never retry blindly.
The launcher uses an isolated child session and does not kill LLDB on ordinary
SIGINT/SIGTERM/SIGHUP cancellation; it waits for the owned debugger to finish.
Repeated wait, async-setup or backoff interrupts keep ownership. This means
cancellation is not an instruction to terminate a potentially attached debugger.
The final manifest distinguishes confirmed owner termination, external exit,
exit after a failed command and lost debugger ownership. Never kill the debugger
to simulate recovery; forced debugger/host loss has no safe-resume guarantee.

The field reader pins system-image UUIDs and checks target-resolved type metadata,
value witnesses, field offsets, native storage and exact dictionary tail ranges.
Unexpected profiles refuse. A completed getter return is owned before register
restoration; decoder/refusal cleanup releases that array before popping the pool.
This does not qualify arbitrary newer systems or prove a global allocation bound.

Focused checks:
`python3 -B tests/bridges/swiftui_debug_capture_check.py`,
`python3 -B tests/bridges/swiftui_debug_fields_check.py`, and
`python3 -B tests/bridges/swiftui_recovery_control_check.py`.
The synthetic SwiftUI source is `tests/bridges/native/SwiftUIDebugFixture.swift`.
The older serializer-based canary harness is historical; it does not exercise
or qualify the new field-only path. Current runtime evidence and pending review
are linked from [the active state](../plans/swiftui-external-layout.md).
