# R01 — Web sources and bounded runtime result

Status: checkpointed diagnostic candidate; independent acceptance pending. Consumer: C01
D01/D02/D04/D07, then F01 and W01. No shipping capability or pilot accepted here.
Authority: [R01 packet](../plans/ui-blueprint/packets/R01.md),
[common boundary](../plans/ui-blueprint/packets/research-common.md), and actual
user launch of PLAN.UIB@1 at `358c757` recorded in the
[registry](../plans/ui-blueprint/task-registry.md). C00 candidate
`92d2bf89ea9acab091fd166eef4432408e353744` and acceptance
`9bedeccbf4c5a92206d24d0794b299e54a2b22e1` supplied by dispatch.

## Spec Basis and truth layers

Traversal, fully read before implementation evidence: AGENTS →
[UIB.ROUTING@1](../specs/README.md), registry revision 3 →
[UIB.RESEARCH-ROUTES@1 / ROUTE / R01](../specs/reference/research-routes.md#r01) →
[BOUNDARIES](../specs/product/boundaries.md), [REUSE](../specs/reference/reuse.md),
[WEB-SOURCES](../specs/reference/web-catalog.md), [PILOTS](../specs/acceptance/pilots.md),
[MODEL](../specs/product/model.md), [GEOMETRY](../specs/product/geometry.md),
[IDENTITY](../specs/product/identity.md), [PROJECTIONS](../specs/product/projections.md),
[FORMS](../specs/product/forms.md), [EXCHANGE](../specs/product/exchange.md),
[PRIVACY](../specs/product/privacy.md), [LIFECYCLE](../specs/product/lifecycle.md),
[CACHE](../specs/product/cache.md), [ACTIONS](../specs/product/actions.md),
[WEB-PILOTS](../specs/acceptance/web-pilots.md), [ROADMAP](../specs/product/roadmap.md).
All 16 leaves pin `UIB.<ID>@1`, clause `UIB.<ID>.CONTENT`; this is the complete
selected dependency closure, including native-neutral constraints.
Exclusions: native internals, real-product profiles, CLI/export/DrawingBrief,
mobile/ML and F1–F4. Supporting context: finite R01/common packet and global
implementation, product-truth, QA/operational rules; no contract drift found.

**Specified:** Rust owns graph/analytics; thin runtime collectors may use JS.
Refs retain target/session/frame/document identity and require fresh validation.
DOM layout, AX, hit and visible regions remain distinct. Delivery is separate
from verified application outcome; partial/unknown never implies pass.
**Observed:** source mechanisms below and own controlled Chromium run.
**Proposal:** adapter shape and baseline below; no spec delta or final D01–D07
choice. Discover applies to upstream; bounded fixture code is explicitly
permitted by the approved diagnostic packet, not shipping implementation.

## Source-reading ledger

All source archives were retrieved by exact commit on 2026-10-06 with bounded
network timeouts. Entries name the selected implementation units and nearby
checks actually read; they do not claim whole-repository audit. Upstream tests
were **read, not executed**. No upstream source was copied into this repository.

| Source / pinned revision | Selected code and tests read | Finding and disposition |
| --- | --- | --- |
| [agent-browser](https://github.com/vercel-labs/agent-browser/tree/6d3e22c673a44271d0c213c2fef722e0aeba627d) `6d3e22c673a44271d0c213c2fef722e0aeba627d` | `cli/src/native/snapshot.rs`: `document_identity_regressions`, `take_snapshot` DOM/AX scoping and ref allocation; `element.rs`: `RefEntry`, `DocumentRefs`, full `RefMap`, `resolve_element_object_id`, document/frame and cross-page tests; `cdp/types.rs`: AXNode/AXValue/AXProperty; `daemon.rs`: `handle_connection`; `diff.rs`: `diff_snapshots` and input/result types | Reimplement session+loader document buckets and monotonic refs, retaining separate observation binding. Unknown document breaks continuity. Node ID alone is insufficient. Reject default role/name/nth fallback for actions: ordinary refs permit re-query after backend resolution fails. The exact-backend flag is closer to our requirement, but still needs current connectivity/document validation. Selector scoping traverses a full DOM subtree and gets a full AX tree before filtering; compact output is not bounded acquisition. Daemon mutex preserves one command stream, but cannot define independent-target scheduling for us. String Myers/pixel diff is not atomic graph delta. Own proof: stale ref, detached handle, iframe loader replacement, same-title second tab. |
| [browser-use](https://github.com/browser-use/browser-use/tree/7be96ed8bafa8dfe1eef228b59cf5c884b8b2431) `7be96ed8bafa8dfe1eef228b59cf5c884b8b2431` | Full `browser_use/dom/enhanced_snapshot.py`; `dom/views.py`: DOMRect, EnhancedAXNode/Property, EnhancedSnapshotNode; `dom/service.py`: `_get_viewport_ratio`, capture factories and snapshot/AX lookups; `browser/watchdogs/dom_watchdog.py`: `_build_dom_tree_without_highlights` service entry; full `tests/ci/test_dom_live_input_value.py` | Reimplement string-table/rare-index parsing and backend lookup in Rust; retain missing layout and redaction explicitly instead of nullable conflation. Sensitive live input values are filtered before the enhanced node, but that is not proof for every raw channel. First layout occurrence and visibility/paint heuristics cannot represent all fragments or exact occlusion. The variable named DPR is actually deprecated/new viewport width ratio. Our run finds ratio=1 even at window DPR=2; no demonstrated upstream defect from that path. Reject blindly dividing by window DPR; test zoom separately. Python agent/model stack is excluded. Own geometry/missing-layout proof provided; sensitive-input test remains F01/V01 work. |
| [Playwright](https://github.com/microsoft/playwright/tree/d0fd0f22ffad53804c0a326a692ee6b8e70ada3d) `d0fd0f22ffad53804c0a326a692ee6b8e70ada3d` | `packages/injected/src/ariaSnapshot.ts`: types/options, full generateAriaTree/computeAriaRef/toAriaNode; `roleUtils.ts`: get/computeAriaRole and accessible-name cache entrypoints; `server/dom.ts`: full `_performPointerAction`; full `tools/backend/snapshot.ts`, `form.ts`; tests `mcp/snapshot-mode.spec.ts` boxes case and `page/elementhandle-click.spec.ts` detached/hidden cases | Use Playwright as a diagnostic low-level library. Injected ARIA is a computed DOM projection, not Chromium AX, and AI/default modes differ. Element-attached refs refresh for name/role changes; DOM node remount needs new binding. Actionability checks visible/enabled/stable and hit target, then scrolls/dispatches: do not run action trials inside read-only observation. Typed form delivery still needs our application outcome and stop policy. `toAriaNode` can collect input values; reject treating its raw snapshot as a privacy boundary. No accessible-name algorithm copied or claimed fully audited. Our prototype uses CDP AX, CSSOM and exact ElementHandle input, not the newer snapshot formatter. |
| [MCP wrapper](https://github.com/microsoft/playwright-mcp/tree/f183dad4a52965583e3cc1d59b88cdc279e2e57d) `f183dad4a52965583e3cc1d59b88cdc279e2e57d`; [CLI wrapper](https://github.com/microsoft/playwright-cli/tree/b85c7a736bb473bf55b584e54a09ffa698d6d871) `b85c7a736bb473bf55b584e54a09ffa698d6d871` | MCP `src/README.md` routes into the preceding monorepo backend; CLI `playwright-cli.js` entry/update path, session-management reference; Playwright `tools/cli-client/program.ts`: session resolution and start/run/cleanup functions; full `tools/cli-daemon/program.ts`; `tests/mcp/cli-session.spec.ts`: list, close, named close, idle shutdown cases | Reuse the concept of owned persistent runtime sessions across short requests. CLI is a wrapper, not a separate engine. Daemon/browser ownership matters for detach; never import close-all or kill-all behavior. Reject a mandatory foreign CLI or network daemon for every operation. Source tests show intended named-session cleanup; our run proves own context/browser cleanup only, not daemon persistence or client-crash recovery. D02 keeps transport open. |
| [Stagehand](https://github.com/browserbase/stagehand/tree/82ef425ee115dbd63bbff930f9f171fe892d231a) `82ef425ee115dbd63bbff930f9f171fe892d231a` | Full `packages/extension/understudy/a11y/snapshot/a11yTree.ts`, adjacent `a11yTree.test.ts` and `types/private/snapshot.ts`; full CLI driver `snapshot.ts`, `snapshot-format.ts` | Reimplement separation of collector, explicit mapping and formatting. DOM file-input decoration usefully supplements AX role, but preserve original role/provenance. Formatter retains ancestor lines for matching descendants; pruning is a projection, not source deletion. Reject scope-widening fallback: focus lookup failure may return all nodes, and frame error may retry unscoped AX. Tests verify deadline failures do not fall back and release objects. Frame-ordinal/backend IDs lack our document-generation contract. No cloud/AI orchestration adopted. F01 needs focused ancestor and missing-scope negatives; current proof only covers child document identity. |

License/dependency inspection: root LICENSE inspected for every pinned repo;
selected headers/imports/types inspected. agent-browser, Playwright and wrappers
are Apache-2.0; browser-use and Stagehand MIT (Gregor Zunic / Browserbase notices).
Playwright root NOTICE additionally records Puppeteer-derived code; same NOTICE
exists in installed 1.58.2. No root NOTICE found in the other five archives.
The inspected Rust collector uses serde/json, CDP client and Tokio; diff adds
similar/image. Browser-use imports cdp-use types and its DOM dataclasses;
Stagehand uses devtools-protocol and its progress/locator helpers; Playwright
injected code relies on internal role/DOM/distiller utilities, backend uses zod.
These are not approved shipping dependencies. Any later code transfer must retain
applicable license/notices and audit the chosen transitive code; this result
chooses original protocol-facing prototype code and an existing library runtime.

## Primary APIs consulted

[CDP domains at d209a9a38897d2935a078a0bf00ca821811d21ed](https://github.com/ChromeDevTools/devtools-protocol/tree/d209a9a38897d2935a078a0bf00ca821811d21ed/pdl/domains):
DOMSnapshot capture/DocumentSnapshot/NodeTreeSnapshot/LayoutTreeSnapshot use indexed
arrays; capture has no subtree or max-node parameter. Accessibility partial-tree
addresses a backend node; full-tree has depth/frame parameters. AXNode provides a
DOM mapping but no geometry field. Accessibility.enable stabilizes AX IDs between
calls and may add runtime cost. Page frame tree exposes loader identity; CSS
viewport metrics are preferred over deprecated metrics. These live docs were
pinned for reading; they are not a Safari support statement or a protocol-version
match for the tested browser.

[CSSOM View, 2025-09-16 draft](https://www.w3.org/TR/2025/WD-cssom-view-1-20250916/)
§6 defines client rects/bounding rect and transforms; union bounds do not establish
painted pixels or occlusion. [WAI-ARIA 1.2](https://www.w3.org/TR/wai-aria-1.2/)
name/role definitions describe semantics, not identity. Repeated accessible names
are valid. No new mapping vocabulary inferred from a library's display strings.

## Falsifiable runtime result

See [runnable probe and oracle](../../experiments/web/README.md).
macOS 27.0.1 (26A434), arm64; Node 24.15.0; installed Playwright Core 1.58.2,
Chromium 145.0.7632.6 (bundle 1208). This runtime version is deliberately recorded
separately from the newer pinned upstream source; no claim that its newer tools
were executed. Viewport 800×600; fresh headless contexts at DPR 1 and 2. API calls
and run have explicit deadlines. Same-origin srcdoc frame only.

| Independent expectation | Observation in both DPR modes |
| --- | --- |
| Main left/right bounds `[40,60,120,40]`, `[200,60,120,40]`; child-local `[12,18,90,30]` | DOMSnapshot and CSSOM equal those literals within preselected 0.01 css_px |
| Two main controls named Apply must not yield a unique name binding | AX exposes two; name-only selection classified ambiguous_target |
| Hidden control has no layout observation | unknown; CSSOM zero rect is kept separately, not substituted |
| Original node fresh; fabricated document rejected | current / stale_target respectively |
| Remount replaces node without changing name | old ref stale; old exact handle rejected; delivery count stays 1; new backend ID; right node ID retained |
| Child navigation changes document generation | loader changes; old child ref stale |
| Same-title second page cannot use first target ref | stale_target |
| Pointer delivery and later applied result are distinct checks | delivery event count 1; separately awaited application counter 1 |
| DPR parameter vs CDP viewport ratio | window DPR 1/2; viewport ratio 1/1; raw bounds unchanged |

48 checks passed. They are author-run fixture checks with independently specified
expected values, not independent reviewer acceptance. Each record includes source
state and source namespaces, target/session/frame/loader/document/backend IDs,
observation time interval, partial coverage and unknown/unsupported geometry.
Equal before/after frame trees do not prove atomic DOM/AX/layout capture; consistency
stays unknown. No screenshots, real applications, secrets or logged-in tabs used.

## Proposed adapter boundary and next gates

For C01: consider a Chromium/CDP baseline for F01/W01 on the measured environment;
Safari/Firefox/WebKit and OOPIF remain unverified, not supported by analogy. No
minimum browser version established. A Rust CDP collector is a candidate suggested
by source reuse; transport/library choice needs its own dependency audit at C01.
Keep Playwright 1.58.2 as the demonstrated fixture driver, not a required shipment.

Adapter owns attach/detach and capabilities; request carries exact target/session,
frame/document generation, scope, fields and monotonic deadline. Output carries
separate DOM/AX source IDs, per-property availability/provenance, coordinate spaces,
observation interval and actual coverage. Join only on explicit backend mapping;
logical component identity needs separate evidence. Host/generation keys wrap all
opaque IDs. Resolve checks target, document, connectivity and uniqueness; locator
search yields a new ref, never silently repairs an old action ref. Outcome verify
is a separate predicate from delivery. Unknown effects stop dependent actions.

F01 should turn these narrow cases into B01–B06 expectations: second tab plus
navigation/remount/unknown document; form draft/debounce/autocomplete and unexpected
transition; portal/overlay/OOPIF with explicit unavailable scope/transform; scroll,
zoom, resize and locale; parent/font changes, lost events and bounded output;
composite DOM/AX button without false decorative action refs. Add privacy canary,
redacted-vs-empty, cancellation and teardown checks before shipping claims.

Acquisition remains a concrete W01 gap: full DOMSnapshot then truncation is not
bounded observation. Evaluate selected DOM/CSSOM plus addressed partial AX, or
report unsupported/partial when an acquisition budget cannot be guaranteed.
Do not silently widen scope after failure. Geometry at page zoom, transforms,
scroll and cross-origin frames needs new calibration; current numbers are not
D05/D06 production gates. Rust graph/cache/delta and schema remain downstream.

Evidence: the [receipt](../plans/ui-blueprint/receipts/R01.md) names the minimal
non-repository report, owner and retention. No raw upstream archives or run logs
are part of this deliverable. No branch, worktree, shared index or external state
outside the owned fixture was modified.
