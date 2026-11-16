# External SwiftUI debug-layout capture candidate

- Node type: leaf; domain: `uib.swiftui-debug-capture`.
- Contract: `UIB.SWIFTUI-DEBUG-CAPTURE@6`; Authority: Active; Stability: Evolving.
- Authority: the user's support assignment, directly verified Clip human turn
  `01a1e568-591d-7532-ba47-af6943b37d37`; finite fixture-only continuation from
  Reels `01a1e40e-5855-7ae2-9809-2729ab377ce2`. This registers the requested
  candidate, not product-app permission, release, runtime acceptance or B0.
- Read when: implementing or qualifying this external debug-layout candidate.
- Requires: [BOUNDARIES@1](boundaries.md), [MODEL@1](model.md),
  [IDENTITY@1](identity.md), [GEOMETRY@2](geometry.md), [PRIVACY@1](privacy.md),
  [EXCHANGE@2](exchange.md), [LIFECYCLE@1](lifecycle.md), [CLI@21](cli.md).
- Accepted/released baseline: none. Existing SDK/AX/GPUI paths are unchanged.
- Implementation status: c89b277 is withdrawn from product use after actual getter
  interruption and an unproved detach/post-debugger-exit lifetime. Source review
  and fixture tests did not qualify this path for the product. Further qualification
  now has independent efd27a9 review closure. @5 admits one exact owner-run
  Services Settings qualification under the human diagnostic authorization
  01a1e59e and fresh owner handoff; no general product capability or release.

## DEBUG-CAPTURE.SCOPE

The source-pinned admission identifies one Services Settings request ID and exact
PID/bundle/birth/window. Other product identities refuse before debugger startup
and again at capture entry. Fixture admission remains separate. A consumer must
revalidate the pinned identity immediately before one attempt; first refusal stops
qualification with no retry/quota increase. Restart/new identity needs new handoff.
The source owner executes the capture; tool preparation grants no ownership of its
process, input or stores. No production installation is implied.

An explicit request identifies one already-running, operator-owned process by
PID, expected bundle and exact launch time, and one native window number.
An independent public process metadata check precedes attach and publication.
No launch, focus, resize, permission prompt, app configuration change, JIT,
expression evaluation, loaded helper library or executable-memory write occurs.
Root selection is exactly the selected window's existing NSView content tree;
other-window metadata may establish uniqueness but its content is not read.
Search this tree only until its outermost getter-capable hosting roots. Once one
is found, its native descendants are not independently scanned: that root's
getter is the selected SwiftUI graph. Multiple outer roots still refuse as
ambiguous. Native/independently hosted internals remain explicit coverage gaps.
@3 reconciles root selection after both real Clip/Reels attempts exhausted the
call budget while the old scan unnecessarily descended through hosting roots.
The initial qualified profile is macOS27.0.1 arm64 / LLDB2103.0.34.407 only.

## DEBUG-CAPTURE.DATA

The guarded correction reads the existing `_viewDebugData` return through
an exact system-image profile plus target-resolved metadata, value witnesses,
field offsets and native storage checks before payload access. It never invokes
the full model-reflecting serializer. Only type metadata, children, position and
size are admitted; model/value/environment/text Any payloads are not read.
Unsupported ABI/storage/representation refuses without a fallback serializer.
The returned owned array is released on every completed-call exit, including
all decoder refusals. Two calls within the total256 are reserved for its release
and pool cleanup; the combined cleanup deadline remains2s.
@4 restores the privacy/lifecycle requirements after the methodology review;
that fixture review is reused by @5; product acceptance remains separate.
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
has a separate≤2000ms call allowance. Two of the total256 calls are reserved for
array/pool cleanup; ordinary acquisition may not consume those reserves.
The system getter's internal allocations are opaque; a post-return length check
does not claim a global target-process memory bound. This is an explicit residual.
Attach and synchronous debugger/OS stop-detach latency remain opaque; these budgets
are not an unqualified whole-attempt wall-clock guarantee. @2 reconciles the
reviewed deadline wording and rejects exponent overflow during JSON decoding.

## DEBUG-CAPTURE.LIFECYCLE

Require a quiescent main thread and hardware-only return breakpoints. Save and
restore supported register state and preserve the native stack red zone.
Fixture correction: use the selected executable's existing entry point as the
return site and require the exact call-stack pointer. The interrupted original
PC can recur inside a nested call and is not a valid return witness by itself.
Only the capture's own temporary data/breakpoints/thread suspension is retired.
Normal completion or refusal after a completed call restores and detaches; it
never kills/restarts the target. Pre-dispatch deadline/identity failure has no
call effect. An interrupted in-flight call is not declared safely reversible:
publish no Snapshot and report owner recovery required with fixed diagnostics.
The withdrawn implementation did not meet the required stopped-state preservation:
this host rejects keep-stopped detach and debugger teardown can resume the target.
The replacement must retain debugger ownership until an explicit owner decision;
it may not silently resume, retry, terminate the target or claim safe teardown.
The retained debugger exposes a separate private command channel bound to the
captured PID, generation and random owner token. Fixture action is
`terminate-owned-fixture`; the single admitted product action is distinct:
`terminate-admitted-source`. A command with the wrong action/identity/owner or
partial/malformed input keeps holding. The selected action is fixed by the capture
owner, not supplied as new authority by the command. One matching command is
attempted once through the already attached SBProcess.Kill; failures never retry.

Observe never writes a recovery command. For product recovery, before writing the
command the source owner must obtain a separate explicit human decision to forcibly
terminate the exact held diagnostic copy. Diagnostic/reopen permission is not that
decision. Ask only if the unsafe outcome actually occurs; channel availability is
not permission to execute it. Report possible loss of unsaved state in that copy.
Normal capture does not require advance permission for hypothetical termination.
The owner then copies trusted checkpoint PID/generation/owner plus the fixed action
into the indicated0600 command file. The held debugger consumes it and reports
confirmed termination only on an observed exit; external exit and failed-command
exit remain distinct. External SIGKILL/debugger teardown are not this route.

@6 repairs the9407 handoff's non-executable product recovery boundary without
adding automatic recovery authority. Fixture qualification of this command route
and affected non-author review precede the product attempt. Receipt IO failure
keeps ownership; forced debugger/host loss remains outside safe-recovery guarantees.

## Acceptance

Required: actual canary model data is absent from all published output and errors;
malformed/oversize/stale/wrong-window inputs refuse; canonical validator/inspect/
measure consume real fixture data; known toggle/resize widths remain exact;
recoverable output refusal leaves the process usable for the next capture;
timeout effects and unknown transforms remain honest. Non-author privacy/lifecycle
review precedes consumer qualification or installation. The fixture-only predecessor
with destructive cleanup is not a supported consumer implementation.
