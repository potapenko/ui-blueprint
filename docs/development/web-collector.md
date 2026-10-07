# Addressed Web collector and first request

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
| DOM/CSSOM | Raw tag; applicable native form value/type/placeholder/flags; layout rect only with a fragment, css_px/viewport/top-left/LocalOnly |
| Addressed AX | fetchRelatives=false, exact backend/frame mapping; raw role plus finite mapping, name/accessibility_name/description, typed value/flags; mixed checked stays unknown |
| Unknown/unimplemented | DOM name/role inference, visible-text qualification, AX geometry, hit/visible/paint/baseline, actions, selection/IME, component/anchor discovery and cross-space transforms |

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
geometry, Close focus and honest unknown hit testing/active descendant. This
fixture does not establish actual PlayPhrase.me structure or dimensions.
[Preparation and exact evidence](../plans/ui-blueprint/receipts/W01-popup-relations.md).
