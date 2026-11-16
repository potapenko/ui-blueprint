# External SwiftUI capture candidate

**WITHDRAWN from product captures.** The c89b277 consumer getter failed with
owner recovery required. Post-debugger-exit lifetime and safe detach are not proved.
Do not run this recipe on product apps; retained code is for controlled fixtures
and failure diagnosis only. A separate513-byte inline LLDB startup hang was also
reproduced without a target. No new version has been installed or released.
The current CLI also rejects product bundle namespaces before debugger startup.
Controlled experiments remain possible only in the dedicated synthetic namespace.
Its [contract](../specs/product/swiftui-debug-capture.md) preserves the distinction
between this raw debug getter and the existing bounds-anchor SDK.

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
  "expected_bundle": "me.uiblueprint.fixture",
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

Use an existing private system-temp output directory and a new request ID:

```sh
python3 -B plugins/macos/swiftui_debug_capture.py \
  --request "$UIB_REQUEST" --out "$UIB_OUTPUT" \
  --bundle /Users/eugenepotapenko/.local/share/uiblueprint/versions/0.3.2
```

Successful output is `snapshot-REQUEST.json` (canonical core0.1) and
`manifest-REQUEST.json` (manifest_version1.0.0), both exclusive0600. The manifest
names the output and cleanup outcome. Raw reflected values never become files or
debug output. Exit0 means a partial Snapshot was published;2 is invalid input or
budget refusal;4 is unavailable/stale/interrupted;1 is setup/IO failure.

`swiftui.debug.layout` nodes have observation-scoped keys and reported type names,
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
`detached_restored`; the same process can be captured again with a new request.
The withdrawn implementation used `owner_recovery_required` after an interrupted
getter. It did not prove that the target stayed stopped after debugger teardown.
Do not use it for product recovery or treat a blind SIGCONT as safe restoration.
The acquisition budget starts immediately after successful attach and includes
post-attach identity checks. Cleanup has a separate2s allowance and one reserved
call within the total256. The framework getter's allocations and synchronous
debugger/OS latency are opaque; no global memory or whole-attempt wall-clock limit
is claimed.

This LLDB target rejects `Detach(True)`; closing the
debugger may resume or lose the target. The current method must not be used for
product recovery. Owned-fixture work is replacing that teardown path with explicit
retained debugger ownership, and replacing full serialization with field-only reads.

Checks: `python3 -B tests/bridges/swiftui_debug_capture_check.py`; the synthetic
SwiftUI source is `tests/bridges/native/SwiftUIDebugFixture.swift`. The LLDB-only
`swiftui_debug_canary.py` harness proves a model canary is present in transient
getter input and absent from the canonical Snapshot/manifest. It is not a product
capture recipe and accepts only the named synthetic fixture namespace.
