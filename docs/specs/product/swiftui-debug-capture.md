# External SwiftUI debug-layout capture candidate

- Node type: leaf; domain: `uib.swiftui-debug-capture`.
- Contract: `UIB.SWIFTUI-DEBUG-CAPTURE@2`; Authority: Active; Stability: Evolving.
- Authority: the user's support assignment, directly verified Clip human turn
  `01a1e568-591d-7532-ba47-af6943b37d37`; finite fixture-only continuation from
  Reels `01a1e40e-5855-7ae2-9809-2729ab377ce2`. This registers the requested
  candidate, not product-app permission, release, runtime acceptance or B0.
- Read when: implementing or qualifying this external debug-layout candidate.
- Requires: [BOUNDARIES@1](boundaries.md), [MODEL@1](model.md),
  [IDENTITY@1](identity.md), [GEOMETRY@2](geometry.md), [PRIVACY@1](privacy.md),
  [EXCHANGE@2](exchange.md), [LIFECYCLE@1](lifecycle.md), [CLI@21](cli.md).
- Accepted/released baseline: none. Existing SDK/AX/GPUI paths are unchanged.

## DEBUG-CAPTURE.SCOPE

An explicit request identifies one already-running, operator-owned process by
PID, expected bundle and exact launch time, and one native window number.
An independent public process metadata check precedes attach and publication.
No launch, focus, resize, permission prompt, app configuration change, JIT,
expression evaluation, loaded helper library or executable-memory write occurs.
Root selection is exactly the selected window's existing NSView content tree;
other-window metadata may establish uniqueness but its content is not read.
The initial qualified profile is macOS27.0.1 arm64 / LLDB2103.0.34.407 only.

## DEBUG-CAPTURE.DATA

The existing `makeViewDebugData` result is transient input, never a file, log or
consumer payload. Admit its byte length before copying and decode with strict
duplicate/nonfinite/shape limits. Ignore reflected model/value/environment/text
properties; publish only structurally validated type, children, position and size.
Canonical core0.1 Snapshot uses a distinct `swiftui.debug.layout` namespace.
Type is structural metadata, not accessibility name or user-visible text.
Children preserve the reported hierarchy; keys are observation-scoped and confer
no cross-capture identity, action authority or declared component association.
An internal capture nonce is independent of the caller's correlation ID, so reusing
that ID cannot create continuity between two external observations.
Missing position/size yields unknown layout, not a zero rectangle.
Known position/size retain original numeric values in separate source-local pt,
top-left spaces per node. Inter-node/window/image transforms remain unknown;
do not put unqualified positions in a common space. Width/height are measurable;
cross-node placement and B0 alignment await a separately proven mapping.
Hit/paint/visible bounds remain unknown. Stored data is cache/unverified, with
unknown consistency and partial coverage, not an atomic or presented-frame claim.
The manifest separately reports the selected `NSWindow.isKeyWindow`,
`NSWindow.isMainWindow` and `NSApplication.isActive` booleans through their actual
getters in the same stopped-process interval. These are AppKit-reported flags,
not AXFocusedWindow, input focus, a foreground guarantee or a proof that external
OS events were processed while stopped. A failed capture publishes no such facts.

## DEBUG-CAPTURE.PRIVACY / LIMITS

Only sanitized canonical data and fixed-code metadata may reach disk/stdout/stderr.
No raw exception text, backtrace, model values, selectors supplied by UI, arguments
or process command line is published. Output files are exclusive0600 in an existing
private nonsymlink directory. Output and source inputs each cap at1MiB; max256
nodes/depth,32window candidates,256NSViews/calls and bounded cumulative reads.
Requests declare tighter positive limits and an acquisition budget≤10000ms from
successful attach, including post-attach identity checks and event waits. Cleanup
has a separate≤2000ms call allowance. One of the total256 calls is reserved for
pool cleanup; no ordinary budget refusal may consume that reserve.
The system getter's internal allocations are opaque; a post-return length check
does not claim a global target-process memory bound. This is an explicit residual.
Attach and synchronous debugger/OS stop-detach latency remain opaque; these budgets
are not an unqualified whole-attempt wall-clock guarantee. @2 reconciles the
reviewed deadline wording and rejects exponent overflow during JSON decoding.

## DEBUG-CAPTURE.LIFECYCLE

Require a quiescent main thread and hardware-only return breakpoints. Save and
restore supported register state and preserve the native stack red zone.
Only the capture's own temporary data/breakpoints/thread suspension is retired.
Normal completion or refusal after a completed call restores and detaches; it
never kills/restarts the target. Pre-dispatch deadline/identity failure has no
call effect. An interrupted in-flight call is not declared safely reversible:
publish no Snapshot, keep the process stopped, report owner recovery required
with fixed diagnostics, and never silently resume, retry or kill it.

## Acceptance

Required: actual canary model data is absent from all published output and errors;
malformed/oversize/stale/wrong-window inputs refuse; canonical validator/inspect/
measure consume real fixture data; known toggle/resize widths remain exact;
recoverable output refusal leaves the process usable for the next capture;
timeout effects and unknown transforms remain honest. Non-author privacy/lifecycle
review precedes consumer qualification or installation. The fixture-only predecessor
with destructive cleanup is not a supported consumer implementation.
