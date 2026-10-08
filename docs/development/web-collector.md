# Addressed Web collector and first request

## W06 explicit full documents and AX focusability

[WEB-DOCUMENTS@1](../specs/product/web-documents.md) adds a separate explicit
whole-document request. Existing initial/rooted/references collection keeps its
original scope. `Focused` now also selects `web.ax` extension `focusable`, encoded
as a requested `Value` boolean with reported AX evidence. It is not the Focused
state or permission to act. Missing/nonboolean raw data is unknown; false remains
known false. No canonical Field, schema, engine, wire or dependency change.

The trusted SessionDescriptor lists BOTH actual Surface identities. WebSetup still
names the root Surface/endpoint. Existing RuntimeHost `attach_web` and
`submit_web_observe` accept the same Tape(Request, WebSelection), and the existing
CLI Web connection deserializes that same selection:

```json
{"selection":"documents","documents":[
  {"surface":{"id":"ACTUAL_MAIN_FRAME","generation":"ACTUAL_MAIN_LOADER"},"document_backend_id":1,"sensitivity":"public"},
  {"surface":{"id":"ACTUAL_CHILD_FRAME","generation":"ACTUAL_CHILD_LOADER"},"document_backend_id":2,"sensitivity":"public"}
],"max_visited_nodes":128}
```

IDs above illustrate the shape ONLY; all four frame/loader strings and both backend
IDs must come from the exact currently authorized documents. No synthetic ref,
Snapshot ID or Observation is needed. The canonical Request selects the exact
same ordered Surface list, scope, fields `["value","layout_bounds"]`, and only
external_semantics. The lower-level public API is
`Collector::attach_with_surfaces(client,binding,surfaces,limits,deadline)` followed
by `observe_documents(request,&DocumentsScope,dispatch_sequence,deadline,publish)`.
Old `attach` remains root-only. Additional descriptor Surfaces alone do no collection.

The F01 quality proof uses128 total node allowance/depth16,512KiB canonical output,
32KiB per protocol message/256KiB aggregate replies,16KiB text,100 methods and2s
failure deadline. These are explicit whole-fixture parameters within the unchanged
D05 profile, not replacements for the32-node/64KiB/250ms single-control scenario
or D06's50/500ms latency thresholds. Native capture internals are opaque browser
SDK work; bounded preflight/transport/worker/output do not imply a browser RSS cap.

Original97 source keys, children, frame `Owns` relation,97 native names/types/values,
attributes and rare fields are retained. Layout bounds use document/css_px;
native offset/client/scroll rectangles remain separate local-space extensions
with unknown transforms. Text boxes retain source bounds and UTF-16 offsets.
Empty native string index−1 maps to known empty, never unknown. Full known DOM
coverage has zero omitted nodes, but overall coverage stays partial (no full AX,
arbitrary visibility, paint or cross-frame transform claim).

All current frame loaders and exact document objects are checked before/after
bounded native-getter traversal and DOMSnapshot acquisition. Wrong/missing/extra/
stale/unallowed bindings refuse without publishing data. Known private documents,
password/private subtrees and credential-bearing or unclassifiable encoded URL
queries refuse the whole channel; no unsafe alias filtering of raw srcdoc/string
tables. Ordinary single-control redaction remains supported. Shadow/templates and
OOPIF/third-party support are unqualified; this is positive F01 same-process srcdoc
proof, not universal browser collection.

Finite author verification lives in [fidelity.cjs](../../tests/bridges/web/fidelity.cjs).
Build the unchanged Q02 generic host consumer from a coherent saved source revision:
`cargo test --locked --offline -p uiblueprint-host --features web --test performance --no-run --message-format=json`.
Use a system-temp CARGO_TARGET_DIR and the matching performance test executable;
set `S01_WEB_PLAYWRIGHT_CORE` to the pinned1.58.2 tooling, `UIB_W06_EXECUTABLE`, its
`UIB_W06_EXECUTABLE_SHA256`, `UIB_W06_PIN`, and `UIB_WEB_LIVE_ALLOW=1`, then run
`node tests/bridges/web/fidelity.cjs --run-authorized`. It performs finite quality
calls only, not D06 series, and saves public baseline/raw/canonical/report files
to a new system-temp directory for Q02. No screenshots; own contexts/workers close.
The [W06 receipt](../plans/ui-blueprint/receipts/W06-web-fidelity.md) pins actual
results and retention. Q02 owns timing and independent comparison acceptance.

## Developer first use: an explicitly selected existing component

The [geometry example](../../tests/bridges/web/geometry.cjs) is tooling, not a new
public CLI grammar or an installed release. It requires Node24 (built-in
WebSocket), existing Web-enabled `uiblueprint` and `session-worker` executables,
and an already accessible numeric-loopback CDP **page** WebSocket endpoint. Use
the shared [selected-module build](native-helper.md#selected-webmac-local-build)
to obtain executables; this example accepts their absolute paths and never builds,
installs, launches a browser/server, opens/navigates/focuses a page or reads a
browser profile. Playwright is needed only by the separate fixture test harness.

Select a component through an authorized caller's existing CDP node picker or
equivalent exact node selection. Supply its identity bundle from that SAME target
and document: target ID, root frame ID, loader ID, document backend ID and selected
element backend ID. `Target.getTargetInfo`, `Page.getFrameTree`,
`DOM.getDocument(depth:0,pierce:false)` and `DOM.describeNode(depth:0,pierce:false)`
provide those identities in the existing caller. This example does not search for
the component. It accepts no CSS/name/coordinate selector or first-match fallback.
IDs from another browser tool are not CDP backend IDs. Do not substitute a bare
old ID and current document metadata for the original bundle.

With those explicit values in shell variables, run from the repository root:

```sh
node tests/bridges/web/geometry.cjs \
  --cli "${UIB_CLI:?absolute Web-enabled CLI path}" \
  --worker "${UIB_WORKER:?absolute Web-enabled worker path}" \
  --endpoint "${UIB_CDP_PAGE_WS:?explicit ws://127.0.0.1:PORT/devtools/page/TARGET}" \
  --target-id "${UIB_TARGET_ID:?selected CDP target}" \
  --frame-id "${UIB_FRAME_ID:?selected root frame}" \
  --loader-id "${UIB_LOADER_ID:?original document loader}" \
  --document-backend-id "${UIB_DOCUMENT_BACKEND_ID:?original document backend ID}" \
  --root-backend-id "${UIB_ROOT_BACKEND_ID:?selected element backend ID}"
```

The command checks the caller-supplied target/frame/loader/document before passing
the exact root to the existing rooted collector. A missing element, foreign or
stale document refuses; duplicate flags are invalid. A backend identity addresses
one node, so no selector ambiguity is resolved by guessing. The collector still
checks ownership, connectivity, original root/parent/children and final document
continuity. This is a fresh read-only observation, not a stable cross-navigation
ref or action authorization. The run's target generation is observation-scoped.

It invokes public Observe → design Inspect → Measure(width,height), using the
original ChannelResponse bytes directly. Rust computes both dimensions. Output
includes compact inspection, reported component-part layout bounds with canonical
keys/space/units/frame kind/evidence, dimension results, coverage and observations.
No second graph or JavaScript geometry calculation is created. Names/roles are
explicitly not requested; canonical source keys are identifiers, not observed
human names. Unknown/redacted/unsupported geometry remains unavailable. Hit/visible
regions are requested but never replaced by layout bounds. The B03 facts below
add partial hit/clipping evidence; no full visibility or padding claim follows.

The fixed geometry profile matches G05:32 output nodes, depth8,64KiB,250ms,
256 visited, collector16nodes, and existing bounded transport/host settings.
Metadata setup is at most five sequential calls,2s each,8KiB per reply/64KiB total,
four events, no document traversal. Checks precede JSON decode; Node WebSocket's
opaque receive allocation is tooling and is not claimed to have the Rust worker's
memory guard. Local CLI calls allow5s each; the finite operation stays under120s.
No retry or increased quota after refusal. Limits do not promise every subtree or
field combination fits: G06 found that adding names/roles for the real Director
exceeded the64KiB canonical frame. This example does not claim that case solved.

Exit0 means commands completed without incomplete/unknown status; exit4 can still
contain useful measured bounds with honest partial coverage. Exit2 is invalid/limit,
exit1 is IO/validation/cleanup failure, and public CLI unsupported exits propagate.
The example prints bounded static refusal codes, never raw CDP/CLI errors. An
earlier printed inspection remains historical if a later measurement fails.
It closes only its own CDP connection and CLI children. Temporary JSON stays in a
unique system-temp directory and is removed after printing; the selected page and
browser remain open. No images or persistent profile/output files are created.
The [G08 receipt](../plans/ui-blueprint/receipts/W01-rooted-selection.md)
records the actual finite F01 proof separately from this generic invocation.

`uiblueprint-web::collector` owns actual CDP selection/reads and canonical DOM/AX
normalization. [Collector repair](../plans/ui-blueprint/receipts/W01-collector-recheck.md)
accepted source4a45400; [bootstrap receipt](../plans/ui-blueprint/receipts/W01-bootstrap.md)
records the new source/offline slice. No live, full W01 or D06 qualification.
Transport/CDP, schema and host are reused; no new client/framework/dependency.
## Concrete Core handoff

```rust,ignore
Collector::attach(client: cdp::Client, binding: collector::Binding,
                  limits: collector::Limits, deadline: Instant)
    -> Result<Collector, collector::Failure>
Collector::observe_initial(&mut self, request: &Request, scope: &InitialScope,
    dispatch_sequence: u64, deadline: Instant,
    publish: impl FnMut(Document) -> Publication) -> Result<BootstrapReport, Failure>
Collector::observe(&mut self, request: &Request, scope: &Scope,
    dispatch_sequence: u64, deadline: Instant,
    publish: impl FnMut(Document) -> Publication) -> Result<Report, Failure>
Collector::cancellation(&self) -> Option<transport::Cancellation>
Collector::detach(&mut self)
Collector::clock_domain(&self) -> &Id
```

Binding fixes canonical session/target/root-frame surface, exact optional CDP session,
allowed scope IDs, plugin identity and Clock { domain, origin }. Core passes its REAL
worker clock origin/domain and converts parent remaining duration to local Instant;
never serialize parent Instant or reuse a lost worker's session/clock incarnation.
Pass the actual admitted nonzero host dispatch_sequence; no fake host Ticket exists.

InitialScope supplies scope_id, ids: Vec<DomId { id: Id, sensitivity }>, and explicit
max_visited_nodes. It authorizes metadata selection in the bound root LIGHT DOM;
request max_depth also bounds traversal. Exact case-sensitive id attributes only:
no CSS/XPath/role/name expression or search in other frames/shadow roots. IDs are
serialized data to a fixed isolated function. Every declared ID must match uniquely;
no match, ambiguity, incomplete scan, detected boundary or local timeout returns
ErrorKind::Selection { status, visited_nodes }. A found candidate does not override
an unfinished scan. Detected selected open-shadow/frame/template/slot boundaries refuse.

The lookup retains original objects in an owned flat null-prototype container.
getProperties reads ONLY that container, ownProperties=true and preview=false;
describeNode(depth0,pierce=false) establishes each original object's real backend ID.
There is no prior ref/Observation/Snapshot placeholder. The same field collection,
normalizer and final R1 check produce the actual canonical Snapshot. BootstrapReport
contains Report, SelectionReport and canonical DOM BackendRefs tied to that emitted
Snapshot/DOM Observation; refs return only after confirmed publication/success.
They are not action authorization or a promise of freshness after return.

Scope remains the separate existing-ref path: scope_id plus NodeRef { reference:
BackendRef, sensitivity }. It validates session/target/surface/web.dom key and never
searches for a replacement. A NEW explicit observe_initial can resolve a new identity;
old refs are not rewritten. H01's remaining composition task is to provide the trusted
initial selection plan and actual admission/clock/deadline/callback, not fabricated refs.

## Publication and data ownership

Callbacks move OWNED core0.1 Document/ChannelResponse. Validate/encode in the guarded
worker; return Acknowledged only after real parent commit/ACK, Stop otherwise. Parent
must not parse a typed graph. Earlier callback-owned Documents survive later errors,
cancel/deadline or teardown. Bootstrap refs are not returned on a failed operation.
DOM and AX remain separate source namespaces/observations in ONE ExternalSemantics
completion. AX protocol refusal may preserve DOM as partial; this is not a separate
DOM ACK before AX. Requested capture/probe channels return Failed(Unsupported).

| Source | Reported data / explicit unknowns |
| --- | --- |
| DOM/CSSOM | Raw tag; applicable native form value/type/placeholder/flags; layout rect only with a fragment, css_px/viewport/top-left; sourced viewport→document mapping when qualified below |
| Addressed AX | fetchRelatives=false, exact backend/frame mapping; raw role plus finite mapping, name/accessibility_name/description, typed value/flags; mixed checked stays unknown |
| Unknown/unimplemented | DOM name/role inference, visible-text qualification, AX geometry, full hit area/visible/paint/baseline, actions, selection/IME, component/anchor discovery and screen/frame/capture mappings |

False/empty stay known; unrequested properties are not synthesized. Only sourced
backendDOMNodeId creates corresponds_to. Flat nonempty projections are partial;
explicit empty EXISTING scope is complete. Missing initial locator is an error.
No source_state, deletion, full tree or actionable refs are invented. Report counts
actual reads/replies/publications; bootstrap separately reports visited/selected nodes.
Caller sensitivity cannot be downgraded. Known password/OTP/card hints suppress strings
and AX; redaction precedes canonical retention. Raw metadata/handles/errors are not
normal diagnostic data. Debug/Failure are bounded; no UI text is evaluated as code.

## Bounds, continuity and remaining gates

One budget covers initial lookup PLUS subsequent reads: methods, reply/work bytes,
canonical output and original deadline are not reset between phases. No implicit
traversal default or automatic widening/retry. Both actual immutable codec frame/
message caps must fit max_reply_bytes at attach and remaining allowance before every
prepare; wire IO separately charges framing/control. Incompatible bounds refuse.
After all reads, target/document and ORIGINAL selected handles are checked again for
connectivity/ownership in one bounded call before group release/publication. Values
and timestamps are not reread/restamped; non-atomic consistency stays unknown.
One group owns document, result container and node handles; success confirms release.
Failure.remote_cleanup distinguishes Released/Unconfirmed/NotRequired; unknown cleanup
closes only the owned connection. No focus/scroll/input, background collection or app kill.

Native metadata/layout/AX/id-string work is opaque. describeNode may include attributes/
shadow/pseudo metadata, but the pinned depth0/null-frontend-map path does not force a
full descendant serialization. Frame/root metadata also has backend cost. Codec caps
are not browser CPU/RSS bounds; parser/normalizer/clone overlap still needs H01/D05 guards.
Next: source review, real guarded admission/ACK composition, fixed F01 first/current-ref
identity, limits, privacy and read-only-invariance cases under a separate runtime grant.
Broader locator/projection/frame/shadow modes, pixels, B01–B06 and D06 remain open.

## Selected popup/form relations

The fixed isolated reader receives the original selected DOM objects as CDP
arguments. Bounded aria-controls IDREFs and the explicit fixture data-anchor
convention resolve only to those objects: selected ID uniqueness plus the native
document ID lookup must agree. Missing, out-of-scope, duplicate selected IDs and
foreign/disconnected endpoints create no relation. This is partial relationship
coverage, not proof that no unreturned relation exists. No UI-derived expression,
selector, extra tree scan, raw IDREF string or unobserved endpoint is serialized.

Canonical DOM relations preserve direction: aria-controls → Controls;
fixture-declared data-anchor → AnchoredTo with separate reported Evidence methods.
They do not merge DOM/AX source IDs or generate action refs. Sensitive source or
endpoint suppresses the relationship before canonical retention. The original
handle/document verification still runs before publication, and the same method/
reply/output/working-memory budgets apply. Native ID lookup is opaque browser work;
selected-handle loops and bounded attribute splitting remain explicitly finite.

When Focused is requested, exactly one observed focused public selected node may
report an active descendant only if its aria-activedescendant resolves to another
observed public selected object. Otherwise active_descendant remains Unknown;
global keyboard focus remains Unknown. An active descendant is not selected or
applied filter state. Requested fields and privacy behavior are unchanged.

The opt-in popup_relations live case selects F01 open-popup, portal, close-popup,
draft and suggestions after the existing popup trigger. The City field is outside
the popup. It checks explicit controls/declared-anchor, compatible CSS viewport
geometry, Close focus, partial single-point hit testing and unknown active descendant. This
fixture does not establish actual PlayPhrase.me structure or dimensions.
[Preparation and exact evidence](../plans/ui-blueprint/receipts/W01-popup-relations.md).

## Rooted read-only bootstrap

`Collector::observe_rooted(request, &RootedScope, real_sequence, deadline, publish)`
uses `RootedScope { scope_id, root: RootSeed, max_visited_nodes }`. RootSeed holds
actual session/target/surface identities, document_backend_id, backend_node_id and
sensitivity. It is caller-observed input, not a BackendRef, action authority or a
claim of locator uniqueness. The seed must come from an actual selected browser
object under trusted caller authority; no Snapshot/Observation IDs are invented.
The existing host Tape carries `WebSelection::Rooted { root: WebRootSeed,
max_visited_nodes }` through `submit_web_observe`; no new public wire/operation.

The worker validates seed IDs/ranges and exact attached document/binding, resolves
the original root, then runs the existing bounded light-DOM traversal from that
object. IDs/labels are not consulted in this mode. Root is depth0, every traversed
DOM node consumes visit allowance; only Elements are source records. Source count
is at most min(collector.max_nodes, request.max_elements/source multiplier), with
two output sources reserved when AX is requested. Exceeding count/depth/visits
refuses Incomplete with no canonical publication; frame/shadow/template/slot
boundaries refuse Unsupported. Coverage stays Partial; no full-document claim.

Before each field read and again before publication, the original selection
container proves root/document/connectivity, unchanged root parent, and each
selected node's parent chain reaches root within max_depth. This prevents an
unannounced move outside the chosen root from becoming fresh scoped data. No
locator re-query repairs handles. Parent is identity-only remote metadata, never
a selected/serialized ancestor. Owned object-group cleanup includes the container.
Root sensitivity applies to all descendants; existing per-node private-value
classification and final original-handle checks remain. Method/reply/output/time
limits cover every added check, without resets or new default ceilings.

Only successful publication returns BootstrapReport references tied to the actual
Snapshot/Observation. Existing InitialIds and References retain their distinct
semantics. [Finite source checks and actual-root fixture preparation](../plans/ui-blueprint/receipts/W01-rooted-selection.md)
are not live Director or general browser-selection qualification.

## Sourced document-space geometry

On a requested layout read, the fixed isolated reader brackets the original client
rect with native Window/VisualViewport getters from its own bound root document.
[CSSOM View](https://drafts.csswg.org/cssom-view/) defines client rectangles and
scroll offsets in CSS pixels, with scrollX/Y locating the viewport against the
initial containing block. Rust constructs the existing canonical Known Transform:
viewport→document, affine `[1,0,0,1,scrollX,scrollY]`. The original viewport rect,
frame kind and property evidence stay unchanged. Transform evidence is separately
Derived/cssom-scroll-viewport-to-document with the actual DOM Observation and full
target, Surface/loader and caller environment binding. Space IDs are opaque and
document-generation-bound; use the returned transform.to.id, never invent one.

Finite positive viewport dimensions and DPR, visual scale1, zero visual offset and
matching native page/scroll offsets qualify this first mapping. Dimensions/DPR are
context guards, not inferred scale factors. Physical pixels/screen mapping, frames,
pinch/panned visual viewport and intrinsic pre-CSS-transform geometry are not claimed.
Missing/unconfirmed context retains original bounds with Unknown transform; old
private records without new facts retain LocalOnly. No missing value becomes zero.
Before/after and cross-node facts must agree; a final bounded selected-node context
read precedes the existing final identity/root checks. Changed context refuses with
resync_required, invalid facts refuse; no raised acquisition/deadline budget. The
DOM interval includes that context check but overall consistency stays sequential/
unknown, not atomic layout. Caller sensitivity is preserved on the final read.

Public Measure already accepts original Observe responses and `--space` equal to
the returned document Space ID. Rust alone transforms anchors and computes relations;
no JS rectangle conversion or new CLI flags. The `viewport` mode of
`tests/bridges/web/guarded-live.cjs` prepares the existing F01 baseline→resize→scroll
proof. Its canonical resized response also serves as before-scroll input. Results
and exact usage are recorded in the [W01 receipt](../plans/ui-blueprint/receipts/W01-rooted-selection.md).
Normalized-motion Diff is the separately owned Core G12 consumer, not this proof.

## Pending retained-session invalidation

`Collector::pending_invalidation()` reports one coalesced boolean, without IO or
consumption. `acknowledge_invalidation()` clears it only after the retained-session
owner successfully invalidates its contexts. No event text/node graph is exported.
Already received AX updates/loadComplete, DOM attribute/text/insert/style and CSS/
frame-resize notifications set the signal. Existing navigation/document/removal/
context-loss events still invalidate refs; document mismatch, event loss, detach,
cancel/expiry and failed CDP exchange leave the signal pending on failure paths.

Events are read only inside explicit CDP commands, with existing queue/byte/work
limits. No subscription expansion, event-pump thread, polling, autoobserve or new
freshness guarantee. Missing CSS/layout events remain possible; current data still
requires explicit observation. A cache owner must conservatively invalidate its
session's retained contexts without altering historical data/time or other sessions.
The worker ends the ObservationRun borrow on success/failure, calls actual
CanonicalSession::invalidate_retained_session, then acknowledges only successful
apply. Cache-apply failure remains explicit and leaves the signal pending.
Source/peer checks do not by themselves prove live retained-cache behavior. See [W03 handoff](../plans/ui-blueprint/receipts/W03-session-invalidation.md).

CDP detach now drops undelivered queued event buffers/permits immediately and
resets ring positions. Caller-held events and replies retain their independent
permits/data; no disappearance of an owner is treated as release of those borrows.

## W03-R: recovery after loss or a bounded refusal

Event queue overflow ends the current collection with `resync_required` and
closes that collector's transport; failed exchanges also invalidate retained
session data. No event handler collects automatically. Calls on that old owner
cannot silently repair references. The caller explicitly detaches/reaps it,
establishes the actual authorized Target/Surface binding again, attaches a new
session and requests a bounded `Initial` selection. Navigation requires a new
binding; neither labels nor old coordinates substitute for source identity.
A selection-budget refusal publishes no truncated success; a following explicit
request with the original permitted bounds can progress on a still-live session.
Recorded snapshots remain historical, and caller-held ACKed bytes survive detach.

The finite opt-in `UIB_WEB_LIVE_CASE=resync` mode in
[guarded-live.cjs](../../tests/bridges/web/guarded-live.cjs) uses the existing
`web_live` test and `session-worker` executable pins. It owns only its headless
Chromium context and loopback forwarding socket. Closing that socket is actual
transport loss, not fabricated CDP event loss. The independent peer test injects
five synthetic events into four queue slots and proves explicit recovery separately.
The [W03-R receipt](../plans/ui-blueprint/receipts/W03-resync-complete.md) records
commands, coverage and remaining acceptance limits. No continuous event detection,
automatic reconnect/retry, browser-wide collection or changed deadlines is implied.


## B03: one-shot clipping and hit facts

For requested `hit_region`, the isolated reader samples `Document.elementFromPoint`
at the original layout rectangle's center. It compares object identity only with
the selected node; it does not inspect the hit object's text, identity or subtree.
An exact match publishes a **zero-area point** as canonical HitRegion rect with
`cssom-elementFromPoint-single-exact-sample` evidence. This is partial sampled
coverage, never the full hit area or permission to click. A miss remains unknown
HitRegion: it does not prove the whole element is blocked. Descendant hits do not
count as an exact match. Pointer-events and visual opacity are distinct.

When `visible_region` is requested, a single-target IntersectionObserver obtains
one initial result with root=null, zero rootMargin and threshold0, only in the
bound top document. The timer uses remaining request time; callback, timeout or
API refusal disconnects the observer and clears the timer. No observer remains
after a successful response; no polling, page callback, focus, scroll or input.
Cancellation/transport loss preserves the existing unconfirmed remote-cleanup
status: a suspended browser cannot be promised to execute its cleanup timer.
Neither timeout nor a late callback performs new source reads. The independently
sampled hit fact survives unavailable clipping. Original/current/entry layout
rectangles and before/after viewport context must agree or collection refuses
`resync_required`; overall multi-node consistency still remains unknown/sequential.

The existing `web.dom` ExtensionProperty envelope carries these reported facts,
with the DOM Observation, viewport/css_px/top-left space and local-only transform:

| Extension name | Meaning |
| --- | --- |
| `hit_sample_point` | Tested point, represented by zero-size HitRegion rect; not a confirmed hit by itself |
| `hit_sample_matches` | Exact selected object matched at that point |
| `intersection_rect_not_occlusion` | Native browser intersection rectangle, including known ancestor/root clipping |
| `intersection_ratio_not_visibility` | Browser-reported intersection ratio |
| `is_intersecting_not_visible` | Browser-reported intersection flag, including known false |

Canonical VisibleRegion stays unknown: intersection does not establish paint,
opacity, filters or occlusion. PaintBounds stays unknown. Empty intersection with
ratio0/false is a known clipping result, unlike missing callback/API data. These
facts do not fabricate a separate popup Surface: the element belongs to the bound
document Surface; explicit selected `Controls` and fixture-declared `AnchoredTo`
relations retain their own source evidence. No schema/version or engine change.

The API basis is [Intersection Observer](https://www.w3.org/TR/intersection-observer/)
and [CSSOM View hit testing](https://drafts.csswg.org/cssom-view/#dom-document-elementfrompoint).
Browser intersection computation is opaque native work, not a bounded browser
CPU/RSS guarantee. Selected count, method/reply/output and worker budgets stay
unchanged; a large requested subtree can refuse within those bounds. IO waiting
adds latency; this is not a new D06 performance qualification.

Run the finite authored oracle using existing Web-enabled executables:

```sh
S01_WEB_PLAYWRIGHT_CORE=<absolute-pinned-playwright-core-1.58.2> \
UIB_WEB_LIVE_ALLOW=1 UIB_WEB_LIVE_CLI=<absolute-uiblueprint> \
UIB_WEB_LIVE_WORKER=<absolute-session-worker> \
node tests/bridges/web/popup-geometry.cjs --run-authorized
```

The [fixture](../../fixtures/web/popup-geometry.html) is setup data, not product
input. The [harness](../../tests/bridges/web/popup-geometry.cjs) checks public
Observe/inspect/measure, authored open/clipped/fully-clipped/covered geometry,
explicit popup relations, close/remount, exact document binding, frame refusal,
limits and read-only invariance. It closes only its own headless Chromium/server
and removes its non-image temporary files. Rooted scopes can observe a selected
popup beside an iframe; selecting the frame itself still refuses without reading
its content. Cross-origin/OOPIF/shadow support is not added. See the
[B03 receipt](../plans/ui-blueprint/receipts/B03-popup-geometry.md) for actual results.

## Explicit Web component declarations and observation pilots

An application can declare `data-component-key="apply-control"` and
`data-component-parts="icon label"` on its owner element. These are reported source
metadata, not inferred framework structure or action authority. The fixed reader
resolves each space-separated part ID only among the original selected elements,
with the same native ID lookup/uniqueness checks used for selected relationships.
It never widens the scope. Missing, duplicate, foreign or out-of-scope part IDs
withhold the mapping. Oversized declarations are bounded; malformed private DTO
indices refuse before publication. A sensitive owner/member or duplicate component
key within the selected records withholds the entire association.

Rust publishes the owner first, then explicitly declared DOM parts in declaration
order, then their separately observed AX counterparts via `cdp-backendDOMNodeId`.
`ComponentMapping` retains `reported` provenance and a declaration source tied to
the DOM owner; its `component_key` also appears in that owner's source declarations.
DOM and AX nodes keep distinct namespaces, native roles, properties and observations.
Both projections retain these source records. Existing public design Inspect can
show the parts from an AX seed; interaction Inspect keeps that seed primary.
A decorative span remains a span with unavailable action semantics. Mapping does
not create an actionable ref, full tree, source-state revision or layout measurement.
Unmarked components still have no declared parts; names/rectangles never substitute.
This opt-in attribute convention is qualified on the owned fixture only.

Run the finite B01/B04/B06 author qualification with existing pinned fixture tooling
and Web-enabled products (no install/browser-profile reuse):

```sh
S01_WEB_PLAYWRIGHT_CORE=<absolute-pinned-playwright-core-1.58.2> \
UIB_WEB_LIVE_ALLOW=1 UIB_WEB_LIVE_CLI=<absolute-uiblueprint> \
UIB_WEB_LIVE_WORKER=<absolute-session-worker> \
node tests/bridges/web/observation-pilots.cjs --run-authorized
```

The [fixture](../../fixtures/web/observation-pilots.html) and
[harness](../../tests/bridges/web/observation-pilots.cjs) check two same-label targets,
source refs/remount/navigation, text size and locale with value preservation,
resize/scroll, and reported DOM/AX component membership through public Observe,
Inspect, Measure and saved Diff. Rust performs geometry/transform comparisons.
The authored 2em text element is a layout box, never glyph bounds; locale does not
supply logical leading/trailing, a breakpoint or inferred scale. Coverage remains
partial and every unavailable property stays explicit. Observe preserves both
pages' focus, scroll, live values, markup and layout; setup mutations are separate.
Each CLI call checks worker retirement; the harness confirms owned browser/profile,
server and non-image temp cleanup before printing success. It creates no images.

Known common-host limitation on the pinned candidate: wrong-target or old-loader
failure during Attach gives CLI exit1 `observe_worker_or_cleanup_failure`, empty
stdout and confirmed cleanup. Existing-ref/remount refusal after Attach gives4.
The collector's precise stale failure is lost before common worker Ready; fixing
that startup publication path belongs to the shared host owner. These refusals
establish isolation, not complete error-code qualification. The
[W04 receipt](../plans/ui-blueprint/receipts/W04-observation-pilots.md) separates
actual capabilities, author checks, this gap and remaining independent acceptance.
