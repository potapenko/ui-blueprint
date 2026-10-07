# Addressed Web collector

`uiblueprint-web::collector` provides actual Rust CDP method owners, an isolated read-only function and canonical DOM/AX normalization. This is the source/offline stage of [W01](../plans/ui-blueprint/packets/W01-collector.md), not live qualification.
[Receipt](../plans/ui-blueprint/receipts/W01-collector.md) pins inputs/checks/residuals.
Transport/CDP behavior is preserved; [R2](../plans/ui-blueprint/packets/W01-collector-repair.md) adds only authorized read-only configuration getters.

## Concrete Core handoff

```rust,ignore
Collector::attach(client: cdp::Client, binding: collector::Binding,
                  limits: collector::Limits, deadline: Instant)
    -> Result<Collector, collector::Failure>
Collector::observe(&mut self, request: &Request, scope: &collector::Scope,
                   dispatch_sequence: u64, deadline: Instant,
                   publish: impl FnMut(Document) -> collector::Publication)
    -> Result<collector::Report, collector::Failure>
Collector::cancellation(&self) -> Option<transport::Cancellation>
Collector::detach(&mut self)
Collector::clock_domain(&self) -> &Id
```

`Binding` supplies canonical session/target/surface, optional exact CDP session,
allowed scope IDs, plugin identity and `Clock { domain: Id, origin: Instant }`.
Core supplies the SAME actual worker clock owner/origin, not just a copied label. Convert parent remaining duration into a local worker deadline; never serialize
parent Instant or claim equality of different process clock origins. Use a new
session/clock incarnation after worker loss. `clock_domain()` returns that owner.

`Scope { scope_id, nodes: Vec<NodeRef> }` contains `NodeRef { reference: BackendRef,
sensitivity: Sensitivity }`. Only existing canonical web.dom refs are accepted; session/target/surface must match, numeric backend key is exact positive int32,
duplicates refuse. Initial locator-to-ref bootstrap remains the caller/resolver
integration gap; this library does not invent prior Snapshot/Observation IDs or
silently discover targets. No new input wire syntax or fake host Ticket is supplied.

Call only after real H01 admission; pass its actual nonzero dispatch_sequence.
Callback receives an OWNED core0.1 Document/ChannelResponse, never a borrowed graph
or all-channel vector. Validate/encode in the guarded worker; return Acknowledged
only after the actual parent commit/ACK, or Stop on publication failure. Parent
must not parse this typed graph. Earlier callback-owned data survives later error,
cancel, deadline or collector drop. Host owns its retained leases, publication
allowances, terminal state and original deadline; callback timing cannot extend it.

DOM and AX are separate namespaces/observations in ONE ExternalSemantics channel.
It publishes once; DuplicateChannel semantics are preserved. A valid AX protocol
error stops further AX queries and permits DOM plus partial coverage after binding
revalidation. Cancel/timeout/malformed or lost identity abort this uncompleted
channel. DOM is not independently parent-ACKed before AX; that stronger boundary
would require separate admitted work/shared contract. Other requested canonical channels return Failed(Unsupported), after external publication, without acquisition.

## Acquisition and available fields

Attach verifies current target, root frame/loader and document backend ID, creates
an isolated world with universal access=false, enables stable AX IDs and rechecks.
Only root-frame attachment is implemented. Each explicit observe resolves the exact
bound document/nodes and passes the document handle into the fixed isolated reader.
After ALL source reads, target/document are checked again and the ORIGINAL selected
handles are revalidated together for connectivity/document ownership in one bounded
call, before group release/publication. Missing removal events cannot replace this check.
Values/timestamps are not reread/restamped; consistency remains non-atomic/unknown.
No locator fallback, descendant scan, full DOMSnapshot/AX tree or arbitrary expression.

| Source | Implemented reported data / remaining unknowns |
| --- | --- |
| DOM/CSSOM | Source tag; layout rect only with a reported client fragment, css_px/viewport/top-left/LocalOnly; native form value/type/placeholder and applicable required/enabled/readonly/checked/selected/expanded/focused/invalid |
| Addressed AX | fetchRelatives=false; bounded exact backend/frame mapping; raw role plus finite role mapping, name/accessibility_name/description, typed text/finite exact numeric/bool value and reported flags; mixed checked stays unknown |
| Unavailable | DOM accessible name/role inference, visible text qualification, AX geometry, hit/visible/paint/baseline, actions, selection/IME, component/anchor/declaration discovery and cross-space transforms |

Every requested property is present as known/unknown/redacted; false/empty remain known. Unrequested fields are not synthesized. Source IDs remain distinct and only
backendDOMNodeId supplies corresponds_to. Flat addressed projections are partial;
only explicit empty scope is complete. No children/deletion/source_state or action
refs are invented. Report includes visited DOM, queried/returned AX, returned DOM, unknown omissions, actual method/reply-byte counts and published canonical bytes.
Snapshot/Observation IDs include a checked process-local collector namespace; they are not portable source generations. Fields/schema/normalization use canonical types.

Caller sensitivity cannot be downgraded. Password and recognized password/OTP/card
hints suppress string reads in the function and skip AX. Sensitive strings are
cleared before canonical normalization; raw method errors/handles never enter it. No accessibility name from visible text, UI instructions executed, focus/scroll/input,
model call or background collection. Debug/Failure expose bounded categories only;
canonical Documents are explicit private data, never ordinary diagnostic logs.

## Bounds, cleanup and live handoff

Explicit Limits bound refs/methods (including final checks/cleanup), reply bytes,
text/handle/property lengths and transport IO/work. At attach both real codec frame/
message caps must fit max_reply_bytes; before EVERY prepare they must also fit the
remaining reply allowance. Incompatibility refuses before dispatch, never raises caps.
Transport::limits and Client::transport_limits read actual config; the latter returns
None for detached/cancelled owners. Wire IO separately counts framing/control; it may
refuse before a payload maximum is reached. Request fields/depth/nodes bound selection;
every method shares the original deadline. A counting serializer enforces TOTAL
canonical output across callbacks. No truncation, automatic retry/resync or polling.
One group owns resolved nodes/document; normal read results are by value. Success confirms releaseObjectGroup, not a browser heap measurement.
On failure, Failure.remote_cleanup distinguishes Released/Unconfirmed/NotRequired;
unknown release closes only the owned connection. No browser termination or rollback.

Native metadata/layout/AX/string-getter work is opaque: Page.getFrameTree includes
frame metadata, root DOM metadata includes counts/adopted sheets, CSSOM may compute
fragments. Byte rejection is not a browser CPU/allocation cap. Parser/normalizer/clone/
encoding overlap and OS/browser memory need their named H01/D05 owners; no RSS bound. Next guarded grant must compose this API with real admission/ACK and fresh canonical refs, then run fixed F01 positive/negative identity, fields, privacy, cleanup and
read-only-invariance cases. Browser/version, arbitrary frames/shadow/portals, pixel
capture, B01–B06 and D06 latency/quality acceptance remain open. No live grant here.
