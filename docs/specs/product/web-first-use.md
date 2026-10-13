# Web URL and selected-component first use
- Node type: leaf; domain: `uib.web.first-use`; contract: `UIB.WEB-FIRST-USE@4`.
- Authority: Active; Stability: Evolving; accepted/released: none.
- Authority source: explicit URL+component request and USE-WEB-BOOTSTRAP1; USE-WEB-DIRECTOR2 authorizes safe diagnostics. @2 added typed diagnostics; @3 selects the measured component profile under USE-WEB-DIRECTOR2, before code. @4 adds root-selected exact canonical stdout under USE-WEB-EXPORT1.
- Read when: shipped Web helper URL/component preparation and external Observe/geometry.
- Do not read when: unchanged exact-ID mode, Native, actions or saved comparison.
- Requires: [Observe diagnostics@1](cli-observe-diagnostics.md), [CLI OBSERVE](cli.md), [BOUNDARIES@1](boundaries.md), [IDENTITY@1](identity.md), [MODEL@1](model.md), [EXCHANGE@2](exchange.md), [PRIVACY@1](privacy.md), [LIFECYCLE@1](lifecycle.md), [PROJECTIONS@1](projections.md), [GEOMETRY@1](geometry.md), [D02@2](../development/decisions/d02-boundaries.md), [D05@4](../development/decisions/d05-limits.md), [D07@5](../development/decisions/d07-reuse.md).
## UIB.WEB-FIRST-USE.INPUT
Shipped Node24+ web-geometry.cjs adds this mode; the existing full exact-ID invocation
remains compatible. No Rust CLI command, wire/schema, dependency or collector change.
`node web-geometry.cjs --connection WS --url URL --component SELECTOR [--json]`
Node24+ is required ONLY for this optional shipped caller; core and low-level guarded
Web CLI/worker stay Node-free. No package/runtime installation or browser setup.
CLI/worker default to sibling uiblueprint/session-worker in the verified bundle;
existing --cli/--worker may explicitly override both with matching absolute binaries.
No manual connection/request JSON or runtime target/frame/loader/backend IDs.
Help/-h alone is offline; missing connection reports connection_required plus a fixed
prerequisite hint before file/network activity. Unknown/duplicate/mixed-mode flags refuse.

WS is an already-authorized numeric-loopback ws browser DevTools endpoint with explicit
port and `/devtools/browser/ID` path, no credentials/query/fragment. It grants metadata
access to that caller-selected browser, not authority from the page URL. No guessed
port/default browser, registry, process/profile scan, launch, navigation or reconfiguration.
Unavailable connection is an explicit prerequisite failure, never an app-integration request.
URL is a caller-authorized existing http(s) page URL, at most4096 UTF-8 bytes, without
credentials/control characters or known credential query keys. Compare WHATWG URL.href
serializations exactly, INCLUDING query and fragment (SPA route); no substring/title/
query-order match. Scheme/host casing/default ports follow that standard serialization.
Only page targets qualify; zero exact matches→target_unresolved, multiple→ambiguous_target.

SELECTOR is explicit data, at most512 UTF-8 bytes: one compound CSS selector consisting
of optional type or *, followed by #id, .class and/or [attribute] / [attribute="value"]
terms. Simple ASCII CSS identifiers; quoted Unicode attribute values allowed. No escapes,
combinators, selector lists, pseudo classes/elements, :has, script or guessed labels.
Unsupported syntax refuses explicitly; never silently broaden or choose the first match.
Scope is the selected page's ROOT DOCUMENT light DOM only. Do not enter frame/shadow/
template content; a selected iframe/frame/template/slot or authored open/closed shadow host is unsupported.
CDP shadowRoots boundary metadata is checked too; user-agent internals of ordinary controls are not traversed and do not exclude their public host.
No app DOM IDs or other source-app changes are required.
## UIB.WEB-FIRST-USE.BINDING
Enumerate bounded Target.getTargets metadata on the authorized browser connection.
Read the uniquely selected target's actual ID/URL; form its page endpoint on that SAME
loopback DevTools origin and verify Target.getTargetInfo again on the actual page socket.
Obtain actual root frame/loader and document backend IDs via Page.getFrameTree and
DOM.getDocument(depth0,piercefalse). No source key is manufactured from URL/selector.
Caller session/generation nonces remain observation-scoped bookkeeping only.
Use a named isolated read-only world and an owned object group. Traverse light DOM under
explicit node/depth/time bounds; native Element.matches receives SELECTOR as a JSON
argument, never executable interpolation. Exactly one match must be proved after the
complete bounded traversal; one found before exhaustion is NOT unique. No full DOM dump,
querySelectorAll-then-truncate or browser-side geometry calculation.
Retain original document/root/parent objects and obtain its real backend ID. Recheck
page URL, target, frame/loader/document and root membership before handing off. Reuse
existing RootSeed/WebSelection, canonical connection/request and actual Rust Observe.
After Observe, before helper stdout/Inspect/Measure, revalidate exact URL/page uniqueness,
frame/loader/document, original connected root/parent and unique selector matching that
SAME root. Navigation/remount/reparent/changed selection refuses, never reacquires a node
or repairs an old ref. Identity checks do not assert atomicity or unchanged UI values.
Release owned remote handles/connections; never close the page/browser/application.
Only canonical Rust output supplies source facts, geometry, units, evidence and coverage.
No action rights, freshness beyond actual observations, full-page visibility or padding
is inferred. Partial/unknown/refusal meanings remain unchanged.
## UIB.WEB-FIRST-USE.BOUNDS
Bootstrap has a10s overall deadline including its Observe bracket,2s max per CDP call,
32 calls,128 enumerated targets,64KiB per text reply/256KiB total per connection, and
at most16 unsolicited events. Check byte bounds before JSON parse. Native WebSocket/
browser allocations are opaque tooling/SDK costs, not a Rust allocator or browser RSS cap.
Selection and final uniqueness walk each allow at most2048 visited DOM nodes, depth64
and1000ms, within the overall deadline; no automatic retries or quota increases.
An explicit --max-visited-nodes may lower this bound only (positive, <=2048).
URL mode requests32 output nodes/depth8/128KiB/1000ms, collector16 nodes/256 visits; the three layout/hit/visible fields and all transport/host caps stay unchanged. Legacy exact-ID remains64KiB/250ms.
Basis: unchanged14-node widget encodes69125bytes;16-node all-field observation spans258.74ms, typed250ms parent timeout, then complete78900bytes within the proposed bounds. Serialized per-node IntersectionObserver waits explain cost; this is a caller workload, not changed D06 gates.
No fields/nodes are pruned, guards weakened or requests retried. The finite component allowance is not a latency/completeness promise; oversized/unstable sources still refuse.
Every owned remote group/socket, child and temporary file has bounded cleanup; failed
cleanup exits1, never success. No idle polling or collection while awaiting commands.
## UIB.WEB-FIRST-USE.FAILURE
Help0; invalid/missing/unsupported arguments or quota2; missing/ambiguous target/root,
stale binding, unsupported source boundary or deadline4; IO/cleanup1. Existing Rust
exits retain their meaning; partial Observe4 can contain useful sourced dimensions.
Preparation/revalidation failures publish no helper result. URL mode requests --diagnostics-json with4096bytes and strictly validates the linked version/field/enum/count schema plus exit agreement; legacy mode stays compatible.
Render only validated enum/count facts on stderr, including partial observations; null Host detail stays unavailable. Invalid/missing reports fail safely without raw text or silent old-CLI fallback.
A limit/timeout category does not identify a particular budget or deeper cause; configured limits are context, not failure proof. No raw URI/selector/source/exception is emitted.
Metadata and temporary connection/request/observation files stay private task-temporary; no persistent config.
New explicit requests require current authority; no automatic retry or source stabilization.
## UIB.WEB-FIRST-USE.OUTPUT
URL mode --json is a unique valueless opt-in flag; default human Inspect/Measure and legacy mode stay unchanged.
Emit only the original canonical observed ChannelResponse bytes from Rust Observe, without reserialization, wrapping, added newline or human output. IDs/values/units/availability/privacy remain exact; no new schema, save-file API, cache or geometry adapter.
Publish only after observed-response/root admission, confirmed worker cleanup, final identity revalidation, remote close, child reaping and private temporary-file cleanup. Failed Observe, stale identity or cleanup failure emits no JSON. Useful observed exit4 retains its original bytes and exit4; empty refusal4 is not saved-data success.
Diagnostics stay validated safe stderr. Machine output skips Inspect/Measure. One stdout write has a5s bound; physical write failure/timeout is IO1 with a fixed diagnostic, may leave partial physical bytes and is never retried.
Shell redirection chooses a private caller-owned destination; use the unchanged response directly with CLI-EXPORT.INPUT/BOUNDS and explicit reviewed metadata. No state/environment/approval/reference is inferred.
## UIB.WEB-FIRST-USE.PROOF
Offline help/bad inputs cause no connection or temp IO; duplicate/missing pages and
components, selector eligibility, bounds/deadlines, stale loader/document/remount/parent,
credential/private diagnostics and release failure have focused proof. Owned F01 actual
URL+component→installed helper→canonical Rust Observe/Inspect/Measure checks independently
authored geometry, unchanged state, truthful coverage and owned cleanup. Consumer's real
Director retry remains independent acceptance. Machine proof adds exact-byte F01→validator/export and no-publication failure cases. No ImageGen, Native or source-app operation.
