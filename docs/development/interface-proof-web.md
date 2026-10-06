# S01-Web common interface proof

State: independent live collector/normalizer checked; shared Rust support entrypoint pending.
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
integration. Initial preparation did not consume shared Rust; the subsequent focused check
built the unchanged Stage A schema validator as recorded below.
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
by a finite check/bridge with explicit outer timeout and cleanup.
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

## Independent collector/normalizer result

[collector.cjs](../../tests/bridges/web/collector.cjs) accepts supplied canonical
request/context and a remaining budget. It performs only fixed ID lookups, depth0
DOM descriptions, selected CSSOM reads, a bounded point hit-test and addressed AX
with fetchRelatives=false. No full DOM/AX snapshot or recursive collector traversal.
Its current test-supported request selects the five documented fields together;
other field sets reject before acquisition. At most seven DOM nodes and their AX
mappings are admitted, within the caller's 32-node limit; insufficient limits
reject. Frame-tree metadata checks document continuity on this fixed two-frame
fixture. This is not qualification of general W01 acquisition on arbitrary pages.

Stage A has `external_semantics`, `rendered_capture`, `opt_in_layout_probe` channels.
External Web DOM/CSSOM and AX use the first, with separate observations/namespaces,
source IDs, raw roles and times. DOM geometry is not mislabeled as an opt-in probe;
AX geometry stays unknown. Returned Snapshot has partial coverage, explicit
corresponds_to mappings from backend IDs and a sourced anchored_to relation.
Parent tag and point-hit samples remain bounded diagnostic evidence, not invented
normalized geometry. No full visible_region is claimed from rect intersection.

`collector-check.cjs` independently reads only the existing B03 oracle and sets up
popup-open, overlay-on and overlay-off. It checks the collector's observed data,
focus/scroll/application state invariance and injected wrong target/fields/limits/
expired-budget refusals. Supplied request context is explicitly **test-only**;
there is no Ticket, parent clock reading or lifecycle emulation in this test.
The collector never reads the oracle. The same function remains suitable for a
future real supplied request after common Rust admission.

45 focused checks passed, including six Rust file-validator exits (request and
snapshot for each state). Syntax checks passed. Canonical schema validator built
with `cargo +1.96.0 build --locked -p uiblueprint-schema --bin uiblueprint-validate`,
offline in a task-temp target directory; Cargo.toml/lock and schema sources matched
Stage A before/after build. No shared source changes, workspace suites or P2 repairs.
These are live collector + schema-format checks, **not D02 session proof**.

```sh
S01_WEB_PLAYWRIGHT_CORE=<absolute-playwright-core-path> \
S01_WEB_VALIDATOR=<pinned-built-uiblueprint-validate> \
S01_WEB_OUTPUT=<task-evidence-path> \
node tests/bridges/web/collector-check.cjs
```

## Committed Stage A and exact remaining dependency

Baseline `9d2df153abd2a7d7567100e06d4260e5edda3bb3`, committed/pushed; runbook
checkpoint `553a4e9` read. Inspected committed schema/plugin handoffs, canonical
model, relevant validation paths and plugin-api library. Four P2 findings untouched.
Preparation checkpoint `6029c7d6f912f826d9fd48633469a51b6b6e7496` was pushed to
origin/master; its short Git lease was released. The collector is saved in a
separate path-limited checkpoint identified in its terminal handoff/receipt history.

Root accepted one producer dependency and clarified its minimal form: Integration
owns reusable Rust **test support/entrypoint**, not necessarily an executable host
or new command protocol. Typed Rust orchestration can directly call the common
ObservationSession. A Node wrapper is needed only if chosen for this bridge.
Absence of a binary alone is not a product blocker. Platform-specific orchestration
may differ while the actual API, validation, Ticket/clock and lifecycle stay common.

Next: receive Integration's committed support revision/usage contract, bind the
collector to admitted canonical requests, return live channels using the real
Ticket and parent monotonic clock, and run complete/cancel/expire/detach/late-response
and malformed/oversize/version checks through that existing common owner. Do not
create a competing driver or policy. Until then the collector result is useful
independent preparation, not D02 acceptance. No browser resources held.
