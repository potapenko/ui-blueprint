# Explicit GPUI layout probe

- Node type: leaf; domain: `uib.gpui-probe`; contract: `UIB.GPUI-PROBE@1`.
- Authority: Active; stability: Evolving; scoped local qualification recorded in
  [the package recipe](../../../plugins/gpui/README.md); no released baseline.
- Authority source: user approval of the presented package in chat
  `01a1dac3-2cc5-7023-9cfe-b94a139710de`, message
  `01a1db4b-9e3c-72e3-9f40-7babe4604262`, 2026-11-14; subsequent message
  `01a1db4c-a361-7683-9804-0461d38e25fb` permits parallel SwiftUI/GPUI work.
- Read when: integrating or qualifying the opt-in GPUI producer.
- Requires: [BOUNDARIES@1](boundaries.md), [MODEL@1](model.md),
  [GEOMETRY@2](geometry.md), [IDENTITY@1](identity.md),
  [PRIVACY@1](privacy.md), [EXCHANGE@2](exchange.md), [CLI@21](cli.md).

## GPUI.COLLECT — Explicit selected runtime layout

`plugins/gpui` is an optional, separate Rust package for gpui-kit0.9.1 and
gpui-pre0.3.12. It does not add GPUI to the core workspace or installed CLI.
The app owns a session for one exact native window/root and the request trigger.
Public Element hooks delegate the original layout, painting, identity and
accessibility behavior. No extra layout node, hitbox, overlay, text/value read,
network, file IO, background acquisition or action belongs to the producer.
Only declared public markers are considered; bounds are read in a requested
prepaint traversal. Every marker requires an existing stable native ElementId; erased/anonymous
identity refuses. Lightweight marker lifecycle invalidation is not geometry
collection. Capture may request a redraw, but must not change model state,
layout, focus, input or scroll. Runtime on/off proof is mandatory.

## GPUI.IDENTITY — Scope and freshness

The caller supplies public app/build/run identity; the session binds the actual
GPUI window handle and a unique session incarnation within the app run. Semantic
marker ID, current native-key/type mount generation, root/surface
generation, capture sequence and layout pass are distinct. Explicit parent
declarations are checked against the marked runtime hierarchy; anonymous and
virtualized/unmounted elements are not invented. A selected child requires its
selected declared ancestors. Duplicate IDs, missing markers, changed window,
stale expected mount/root, cancelled/closed session and expired request refuse.
Reappearance after a completed frame without a marker has a new generation.
Root removal invalidates its session; callers create a new session for a new root.
Publication rechecks the captured pass and mounts; no old-geometry fallback.

## GPUI.DATA — Existing canonical consumers

Return unchanged core0.1 Snapshot through the canonical schema and validator;
the existing Rust inspect/measure/check path remains the only analysis engine.
Use namespace `gpui.layout`, channel `opt_in_layout_probe`, projection `design`.
GPUI Pixels are logical units: preserve window-local top-left coordinates,
explicitly label their meaning and report source scale separately. They are not
DevicePixels; no implicit logical-pixel-to-image or screen transform is invented.
Known bounds are layout bounds only. Requested AX/hit/visible/paint data remain
unknown/unsupported. Declared component membership/parents remain attributed.
Coverage is partial for the application, never a universal internal-tree claim.
Saved documents are cache/unverified with unknown presented-frame consistency.
A completed layout pass or equal endpoint samples do not identify a GPU frame.
Only public identity, geometry, scale and fixed diagnostics cross this boundary.

## GPUI.LIMITS — Bounded local work

At most256 declared/selected markers, public ASCII IDs of1–64 bytes, bounded
ancestry, one request per session, explicit deadline1–10000ms and encoded output
cap1–1048576 bytes. Invalid/nonfinite/negative geometry, malformed declarations,
cycles, overflow and ambiguity fail before any successful partial document.
Use monotonic request time; explicit cancellation retires pending work. The app
alone owns private transport and system-temp output. Production dependency is
opt-in; Release collection is disabled and separately checked.

## GPUI.ACCEPT — Native qualification

The technical example belongs to UI Blueprint, not the PlayPhrase.me client.
It uses real native GPUI controls, text, nested disclosure and one scroll owner.
Required chains: initial capture → canonical inspect/measure/check;
resize and long text → new bounds; scroll → changed window-local bounds;
open/close/remount → current presence and rejected old generation;
controlled runtime gap8→18 → expected numeric delta; duplicate/stale/missing,
deadline/cancellation/limits → truthful refusal. These are diagnostic values,
not requirements for client UI. Test source values cannot be fabricated as live data.
Pair native PNGs with the same stationary window/state and verified scale/origin;
changed/unknown pairing is unverified, not success. Probe-on/off must preserve
layout, focus, input and the selected scenario. Another agent must reproduce
the documented connection and capture without hand-entered coordinates.
Headless/build checks support but do not replace native proof. Record actual
coverage, exact dependencies/features, evidence and residuals in the recipe.

## Change and scope record

GPUI-PROBE-001, Evolve: previously no GPUI producer; approved additive scoped
producer and native example. Owner: chat `01a1db1e-27f9-73a2-822e-0cd982b83ddb`.
Writes: plugins/gpui, this contract and routes, GPUI skill recipe; shared README
and skill links require handoff. SwiftUI, Compose, Web, core/schema/engine/CLI,
client/admin projects, global installation and release remain protected.
Basis: registry → product → selected Requires; DEV.RUST@4/D01@1/D07@5/REUSE@1,
RUST.md and GPUI Coding Guides govern engineering. Consumer visual-transfer@1
defines B0; both SwiftUI and GPUI must qualify before client UI transfer.
