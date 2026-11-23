# External SwiftUI debug-layout capture candidate

- Node type: leaf; domain: `uib.swiftui-debug-capture`.
- Contract: `UIB.SWIFTUI-DEBUG-CAPTURE@18`; Authority: Active; Stability: Evolving.
- Authority: the user's support assignment, directly verified Clip human turn
  `01a1e568-591d-7532-ba47-af6943b37d37`; finite fixture-only continuation from
  Reels `01a1e40e-5855-7ae2-9809-2729ab377ce2`. This registers the requested
  candidate, not product-app permission, release, runtime acceptance or B0.
- Read when: implementing or qualifying this external debug-layout candidate.
- Requires: [BOUNDARIES@1](boundaries.md), [MODEL@1](model.md),
  [IDENTITY@1](identity.md), [GEOMETRY@2](geometry.md), [PRIVACY@1](privacy.md),
  [EXCHANGE@2](exchange.md), [LIFECYCLE@1](lifecycle.md), [CLI@21](cli.md).
- Conditional requires, mounted native parts: [NATIVE-PARTS@1](swiftui-native-parts.md).
- Conditional requires, native body ancestors: [NATIVE-OWNER@2](swiftui-native-owner.md).
- Conditional requires, fixed backend: [BACKEND@1](swiftui-debug-backend.md).
- Accepted/released baseline: none. Existing SDK/AX/GPUI paths are unchanged.
- Implementation status: c89b277 is withdrawn from product use after actual getter
  interruption and an unproved detach/post-debugger-exit lifetime. Source review
  and fixture tests did not qualify this path for the product. Further qualification
  received independent method/recovery review closure through8588025. The one
  admitted Settings attempt then entered retained owner recovery without Snapshot.
  The withdrawn getter admission remains disabled. After affected cache review,
  the one @8 Reels cache attempt refused tree_limit. Its admission is consumed;
  no further consumer is qualified without bounded subtree selection.
  Services completed owner recovery; target and debugger processes are absent.

## DEBUG-CAPTURE.SCOPE

The source-pinned admission identifies one Reels cache qualification request ID and exact
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

## DEBUG-CAPTURE.SUBTREE — bounded structural selection

@9 registers the owner-requested Reels/Clip correction after whole-window
qualification refused tree_limit. Optional request `scope` contains `type` and
optional `containing_type`: exact module-qualified nominal source types, without
generic arguments or model/text predicates. Owners supplied ClipSearchView and
ReelsSlideView containing ReelsVideoView from immutable e19b20c47 source. This is
structural attribution, not a claim that runtime uniqueness has been observed.

Discovery reads only graph-owned tree links and type metadata, at the same1024raw
records/256depth/2MiBread/deadline limits, across the selected window's graph.
No geometry/model-value payload is collected outside the selected subtree.
A match must be unique; absent/ambiguous selection refuses, including two active
Reels cards. Never select by traversal index, label text or assumed current page.
`containing_type` means an actual proper descendant in the same reported graph.
The chosen root and its descendants are normalized within the existing256view cap;
outside siblings do not consume emitted-view slots. Parent links within the subtree
remain, its external parent is omitted explicitly by the scoped context. Root and
window ownership checks stay mandatory. Inter-node/PNG transforms remain unknown.
No limit is increased and no raw scan limit is silently bypassed. Optional scope
is additive to request1.0; old unscoped inputs retain whole-host behavior. Product
admission remains closed until the selected mechanism is qualified and reviewed.

@10 covers the paused-card case requested by the Reels owner. On the qualified
profile a source view body can appear in several disjoint branches. Match the
unique body attribute whose outer occurrence contains the requested descendant,
then include every outer occurrence of that same attribute. Preserve parent links
within each branch and emit separate roots where the omitted ancestors differ.
The owner attribute is local to this observation. Two matching owner attributes
refuse as ambiguous. All selected branches share the original limits and must
pass the same graph, type and privacy checks.

Experimental anchor mode requires the exact mounted NSView type in the selected
window, a unique represented view, matching provider pointer, runtime ivar
offsets and a current weak layout attribute in the selected graph. The selector follows the recorded direct-parent link within the same subgraph and
crosses a subgraph root only to its exact uniquely associated parent element. The
first exact nominal card ancestor defines the bounded subtree. Parent traversal
shares the request depth limit. The recorded native layout attribute must occur
under the requested video type inside that card.
A selected card must contain exactly one recorded layoutComputer association
matching that native reference. Unknown, missing, expired or ambiguous bindings
refuse. The mode is fixture-only until independent qualification.

@11 adds an explicit geometry-bearing view selection for native-anchored
requests after the Reels owner reported tree_limit inside the selected card.
The root, native anchor, containing video type and records with position or
size fields are retained. Edges connect each included record to its nearest
included ancestor. Count omitted wrappers explicitly. Preserve model exclusion
for every encountered view including omitted wrappers before any geometry
payload read. All raw-record, emitted-record, depth, byte and call bounds apply.
A recorded geometry field whose payload is dirty remains unknown as before.
This mode does not establish semantic identities or guarantee that a large
control has only one geometry-bearing record.

@12 supports owner-declared leaf_types for a geometry_records request. Keep the
boundary record and stop before its children. Report the nominal body owner's
name and output node index for each cut; frame availability still comes from
that record. Require at least one boundary for each requested type. To prove
native membership, inspect at most64 records local to the weak attribute's
subgraph for a unique matching layoutComputer, then follow the graph-owned
parent path. Inside a cut video continue only along that verified witness path.
Other branches of a cut control do not consume the card traversal budget.
Only declared source boundaries may be requested. All previous ownership,
privacy and limit refusals remain. A request does not grant product admission.

## DEBUG-CAPTURE.NATIVE-WITHIN-OWNER

Library S12 and Grammar S12 request geometry where different controls mount the
same cursor NSView class. @13 registers a fixture-only experiment. An explicit
`scope.native_match = "within_owner"` first selects one structural SwiftUI body
owner by the SUBTREE rules for `type` and optional `containing_type`. Across all
branches of that owner require exactly one layoutComputer association to exactly
one live represented NSView of the declared exact class in the selected window.
Qualify every same-class candidate using the existing provider, graph and current
weak-attribute checks. An incomplete candidate search, ambiguous owner or binding,
stale reference, unsupported provider or exhausted budget refuses without output.
Selection by traversal index, coordinates, text or model values is forbidden.
Discovery and validation share the original cumulative budgets across candidates
and branches. Model payloads remain protected. This mode excludes `leaf_types`.
Omitting the option preserves the existing unique-native-anchor behavior.

Qualification requires same-class controls in distinct owners and refusals for
repeated matching owners, multiple matches within one owner, incomplete search
and stale bindings. Any reported native cursor rectangle describes only that
NSView in its stated native space. SwiftUI control layout requires independently
qualified SwiftUI geometry. Product admission and release require separate review.

## DEBUG-CAPTURE.SELECTION-PROVENANCE

For offline verification of normalized branches, @14 adds an optional top-level
manifest field `selection_provenance`. Emit it with a successful nonempty Snapshot
from direct unique structural selection, including `within_owner`. It contains:

- `observation_id` copied from the canonical observation, an opaque nonce-scoped
  `occurrence_id` for the one selected occurrence, and its `selection_source`
  (`body_owner` or `nominal_element`).
- `source_type`, the exact module-qualified nominal type matched in that source.
- `branches`, one entry per selected outer raw branch. Each has an opaque local
  `branch_id` and a `root_keys` array of canonical node keys. Use an empty array
  when normalization emitted no nodes for that branch.

Each emitted root belongs to exactly one entry. Its canonical child links identify
the other emitted nodes of that branch. The receipt must account for all selected
branches and emitted roots. Array position carries no semantic meaning. Keep raw
pointers and graph identifiers private. Tokens identify only this observation.

Omit the receipt for unscoped captures and native-ancestor selection without
direct structural selection. Ambiguous selection, an entirely empty result or a
failed capture supplies no mapping. Older manifests remain consumable with
unknown branch provenance. Construct the receipt only from existing selection
and normalization evidence within the same limits and acquisition permissions.

## DEBUG-CAPTURE.DATA

The @7 candidate replaces both debug getters with a bounded read of the existing
AttributeGraph tree and already-computed geometry cache. It must not invoke
_viewDebugData, makeViewDebugData, AGGraphGetValue, update_attribute, reflection
of model values or their copy witnesses. Ordinary admitted AppKit/runtime metadata
calls remain separate; no blanket claim of zero target instructions is made.
Exact image UUIDs, NSHostingView class/field binding, ViewGraph identity and root
Subgraph ownership must be established before tree payloads. Every tree/value/node
record must belong to the selected graph. Geometry requires target descriptor,
value-witness, field-offset, range and cache-validity checks before reading payload.
Read only position/size and the explicitly qualified inputs in
[TRANSFORM@7](swiftui-cache-transform.md); ViewSize.value is CGSize at0, not its proposal at16.
Dirty, stale, absent or unsupported cached values remain unknown; never evaluate
or numerically repair them. Sizes must be finite/nonnegative, positions finite.
Model value/body ranges are protected and audited on the canary; a constant boolean
or absence of evaluation calls alone is not privacy proof. Strings are read only
to their terminator, never as a wide neighboring payload dump.
Raw control records are distinct from emitted view nodes: normalize transparent
records according to the qualified source profile. Keep parent associations;
sibling order with equal source-order keys is unknown unless a complete valid
cached chain establishes it. Do not silently turn traversal order into semantics.
Affected consumer review of7d18685 closed the model-protection findings; cold
collection and same-window transitions passed. @8 admits only request
reels-cache-qualification-1 / PID19791 / me.playphraseme.reference.reels.fgb7o3fd /
birth1795038583.1458468 / CG25250, handed off by the Reels owner on2026-11-18.
The owner executes one attempt after fresh identity/state/hash verification;
first refusal ends qualification. This is no release/B0/transform acceptance.
The consumed8588025 getter admission is never reused.
Canonical core0.1 Snapshot uses a distinct `swiftui.ag_cache.layout` namespace.
Type is structural metadata, not accessibility name or user-visible text.
Children preserve the reported hierarchy; keys are observation-scoped and confer
no cross-capture identity, action authority or declared component association.
An internal capture nonce is independent of the caller's correlation ID, so reusing
that ID cannot create continuity between two external observations.
Missing position/size yields unknown layout, not a zero rectangle.
Known position/size retain original numeric values in separate source-local pt,
top-left spaces per node. Only [TRANSFORM@7](swiftui-cache-transform.md) qualifies an explicit mapping.
Other inter-node/window/image transforms remain unknown; never merge unqualified spaces. Width/height are measurable;
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
emitted views/depth,1024raw tree/control records and bounded metadata tables,
32window candidates,256NSViews/calls,2MiB cumulative reads. Metadata/control-record
units are not emitted view nodes; these bounds do not permit opaque getter allocation.
Requests declare tighter positive limits and an acquisition budget≤10000ms from
successful attach, including post-attach identity checks and event waits. Cleanup
has a separate≤2000ms call allowance. Two of the total256 calls are reserved for
cleanup; ordinary acquisition may not consume those reserves. Cache reading
allocates no returned Swift array; only the existing native pool needs a cleanup call.
Ordinary native getter/OS internals remain opaque; these limits do not claim a
global target-process memory bound. The withdrawn SwiftUI getter is not a fallback.
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

Observe never writes a recovery command. Product recovery requires authority distinct from Observe. The direct Services
human reply01a1e6c3 on2026-11-17 now grants standing lifecycle authority for our
isolated task-owned diagnostic copies; reuse it without repeated questions. Exact
owner/PID/generation binding and the explicit command still apply. This does not
cover unrelated processes, whole hosts, production or protected storage. Outside
that standing scope, obtain the missing actual authority before recovery.
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
