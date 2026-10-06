# S01-Web common interface proof

State: fixture/setup prepared; Stage A pinned; shared executable test-host pending.
Consumer D02-PROOF/P1 boundary, then W01. No production adapter or pilot accepted.
Authority: [platform packet](../plans/ui-blueprint/packets/S01-web-proof.md) and
[shared handoff](../plans/ui-blueprint/packets/S01-bridges.md), approved PLAN.UIB@1
S01 obligation and direct parallel-work dispatch. No new product decision.

## Spec Basis

AGENTS → [registry](../specs/README.md) ROUTING@1/revision4 →
[decision route](../specs/development/decisions/README.md) DECISIONS@1/ROUTE →
D02@1, D03@1, D05@1 and complete dependencies. Read missing NATIVE@1,
RUST-BOUNDARIES@1, GOLDEN@1, D01@1, C01-EVIDENCE@1; reused unchanged full
BOUNDARIES, ROADMAP, MODEL, EXCHANGE, IDENTITY, GEOMETRY, PROJECTIONS, FORMS,
CACHE, ACTIONS, LIFECYCLE, PRIVACY, PERFORMANCE, REUSE, PILOTS and WEB-PILOTS
CONTENT@1 from R01/F01 after no-diff check. RUST.md, DEV.RUST@2, D07@2 and
C01-HANDOFF@1 with D04@1/D06@1 dependencies read for subsequent validator/build
integration. No Rust build or shared source inspection during this preparation.
Excluded: detailed export, mobile/ML/future products, real apps and native runtime.

## Owned setup and input fragment

Read-only F01 input `a8368076cdf0d4a917e7c3c5448ca6b6d919cf64`, documentation
correction `585bcd90ad455110bcc2432f8f36f58e3c8d7ecf`. The
[fixture handoff](fixtures-web.md) and [README](../../fixtures/web/README.md)
remain current. Historical polling/rAF runs are not used. Reuse F01 server and
frozen R01 foundation; do not copy or edit the fixture.

Preparation command (existing installation, no downloads):

```sh
S01_WEB_PLAYWRIGHT_CORE=/Users/eugenepotapenko/.npm/_npx/f88013d20c39cb98/node_modules/playwright-core \
node tests/bridges/web/prepare.cjs
```

The setup checks five fixture files against committed F01, Node 24.15.0,
Playwright Core 1.58.2 and installed Chromium executable; starts one ephemeral
loopback server, fetches only its fixture document and closes it. It does not
launch a browser, observe UI or validate an envelope. `fixture-host.cjs` provides
a callback-based owned headless setup for the later bridge; it is not invoked
until Stage A is pinned and the common driver's outer deadline/teardown is known.
Browser launch/navigation setup timeouts are fixture parameters, not D05 defaults.

Selected proof scenario, following root's user-designated Web coauthor advice:
existing F01/B03 `popup-open` plus `overlay-on/off`; no new fixture. The explicit
scope names `#open-popup`, `#portal`, its `#close-popup` control and authorized
neighboring `#left`, `#cover` when present, `#clip` and `#clip-child` context.
The portal is appended outside the form subtree; include it by its explicit
fixture anchor relationship, never by a broad document search or geometry guess.
The requested collection stays within these named nodes and needed relationships.

After Stage A is pinned, establish fixture states through the existing setup
operations and pointer APIs, separately from read-only collection. Obtain exact
tab/frame/loader/document identity; selected DOM/CSSOM bounds, parent/anchor
observations and addressed AX with fetchRelatives=false. Keep source identities,
raw roles, CSS pixels/viewport origin and each channel's actual monotonic interval.
Parent/collector clock domains remain separate. No full DOMSnapshot/full AX tree,
page traversal, frame/zoom inference, screenshots, polling or recollection between
explicit requests. Missing `#cover` in overlay-off is scenario state, not failure
of the entire scope; represent it only using the committed Stage A contract.

The collector never reads `fixtures/web/expected.json`. A separate assertion path
uses its B03 oracle: portal_parent=BODY, anchor=open-popup, blocked/visible hit
targets, clip bounds and intersection width. The latter is a derived rectangle
intersection plus bounded hit samples, not arbitrary occlusion proof. Expected
numbers are never inserted into observations. Preserve partial/unavailable
properties rather than inventing AX geometry or completeness.

D05 Web scenario parameters remain explicit caller limits: 32 nodes/depth 8,
64 KiB and 250 ms warm request deadline. Bound actual extraction over the named
nodes; wire field/channel identifiers come only from Stage A. The proof checks
request/scope/coverage preservation, not new product policy or schema fields.
B02 pending/invalid/selected/applied and B04 reflow are deferred to later affected
checks. Do not run the historical combined `fixtures/web/run.cjs` as acceptance.
Real-site Director popup is contextual only: its DOM portal structure is not
established here; no real-site launch, collection or changes are authorized.

## Committed Stage A and exact remaining dependency

Baseline supplied by root: `9d2df153abd2a7d7567100e06d4260e5edda3bb3`, committed
and pushed; packet/runbook checkpoint `553a4e9`. Read the current execution runbook
and committed [schema handoff](schema.md), [plugin handoff](plugin-interface.md),
`crates/plugin-api/Cargo.toml` and complete `crates/plugin-api/src/lib.rs`.
Shared working files match Stage A for those owners. No shared build or mutation.
This is a candidate, not acceptance; four known P2 findings stay untouched.

Observed API: `ObservationSession::attach/begin/receive/complete/cancel/expire/detach`
operates on canonical schema frames and a Ticket. It intentionally performs no IO,
clock reads or spawning. Committed plugin-api contains its library and authored
`tests/lifecycle.rs`; no binary/example/common bridge host. The available
`uiblueprint-validate` binary validates documents but does not exercise the session
state machine. Schema-only validation cannot satisfy the live plugin-boundary proof.

**One dependency request to Integration, through root:** provide a committed shared
executable test-host, usable by both platform bridges, that calls this existing
ObservationSession rather than introducing another protocol/lifecycle policy.
It must admit the canonical session/request with explicit test limits, return the
actual Ticket before collection, accept canonical channel frames, and expose
completion/cancel/expire/detach/late-response results. The host owns the real parent
monotonic clock and bounded framing; failure-injection mode must be labelled.
Document invocation/IO contract and existing negative-test reuse. Keep the same
schema validator and no platform SDKs in this owner. Integration chooses its exact
test command interface; this Web packet does not invent a competing one.

Next: root supplies the committed host revision/entry point; read that finite API,
then implement selected F01/B03 collection and wire binding in web paths, run live
request → channel results → shared Rust session/validator, plus assigned negatives.
Wrong-version, malformed/oversize, cancellation/detach and late replies must use
that common boundary. No second Web/Native driver, fabricated Ticket/clock or
mock-only pass. Existing preparation is checkpoint-ready but not D02-complete;
Git stage/commit/push still require root's short lease. No browser resources held.
