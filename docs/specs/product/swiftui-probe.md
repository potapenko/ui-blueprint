# Explicit SwiftUI layout probe

- Node type: leaf; domain: `uib.swiftui-probe`; contract: `UIB.SWIFTUI-PROBE@1`.
- Authority: Active; stability: Evolving; runtime acceptance pending.
- Authority source: direct 2026-11-14 user approval in chat
  `01a1dab2-2e7d-7f13-9400-8e42b0859137`: implement the first SwiftUI package
  against the real PlayPhrase.me app; work with its native owner, no GPUI work.
- Read when: integrating or verifying the opt-in SwiftUI SDK.
- Requires: [NATIVE@5](native.md), [MODEL@1](model.md),
  [GEOMETRY@2](geometry.md), [IDENTITY@1](identity.md), [PRIVACY@1](privacy.md),
  [EXCHANGE@2](exchange.md), [CLI@21](cli.md).

## SWIFTUI.COLLECT — Request-owned source measurement

The concrete Swift package `plugins/swiftui`, module `UIBlueprintSwiftUI`, uses
SwiftUI/Foundation/Observation on macOS14+. Its implementation is DEBUG-only.
The consuming app owns one session per explicitly selected root/window, provides
its real target/run identity and admits requests. No AppKit, reflection, networking,
file IO, model/text collection or input actions belong to the SDK.
Markers have public semantic IDs, declared parents and separate mount UUIDs.
They attach bounds anchors without adding a layout container, hit area, focus or
accessibility element. Resolve selected anchors only for an explicit capture,
on the main actor, in the measured root's local pt/top-left space. Source display
scale comes from that SwiftUI environment, not a process-global screen query.
Root-local frames are not AX, hit, paint or visibility bounds. A root frame in
SwiftUI global coordinates may be reported separately; it is not a screen or
capture transform. Missing mapping remains unknown.

## SWIFTUI.SNAPSHOT — Bounded and attributable

Configuration selects root plus required markers, at most256 total, with public
ASCII IDs of1–96 bytes. Capture has a unique request ID, public application state
ID, optional expected mount IDs, deadline1–10000ms and output cap1–1048576 bytes.
The ordinary default deadline is1000ms; caps are SDK limits, not enlarged host quotas.
One request per session is active. Duplicate/missing markers, invalid parents or
cycles, nonfinite/negative geometry, stale mount/root, cancellation, timeout and
overflow fail without a partial successful document or reuse of old geometry.
Detach invalidates in-flight work and the root generation; remount must not revive
old expected mounts. Stop retaining anchor handles after the capture callback.
Readiness, capture sequence and measured-layout revision are separate; none is a
GPU/presented-frame identifier. Public SwiftUI callbacks do not prove atomicity.

## SWIFTUI.OUTPUT — Existing canonical consumer path

Return immutable Data containing a canonical core0.1 Snapshot, plus small capture
metadata. The app alone owns any private system-temp write. Existing Rust
`inspect`, `measure` and `check` consume it without a new importer or schema.
Namespace `swiftui.layout`; channel `opt_in_layout_probe`; projection `design`.
Preserve original source times and root/mount identities. Saved output is
cache/unverified with unknown consistency; never restamp it as live/current.
Known layout properties have source evidence. Hit/visible/paint bounds remain
explicitly unknown. Component membership/parents are declared marker relationships,
not a reconstructed complete SwiftUI tree. Coverage is partial for the app.
Only IDs, geometry, scale and fixed diagnostic metadata are exported; no labels,
input values, credentials, paths, private views or media enter the document.
Swift collects/encodes; canonical validation and geometry arithmetic stay in Rust.
Old F02 probe, native_ax, Compose, CLI, wire versions and release behavior stay intact.

## SWIFTUI.ACCEPT — Real application, not fixture-only completion

The native owner integrates the SDK in the real PlayPhrase.me recovery component
under the existing local `c02.search.request-failed` Debug app composition.
Root/card/icon/text/title/detail/Retry measurements must validate in Rust and
produce useful numeric results, followed by real resize/content observations.
SDK tests cover identities, duplicate/missing/stale data, limits and cancellation.
Probe on/off must preserve the selected app's layout, focus, input and Retry.
Pair actual stationary-window screenshots with source geometry through a proved
transform and unchanged scene state. Unknown or changed pairing is not a pass;
exact animated-frame/pixel correspondence is not inferred from equal endpoints.
Headless tests support, but never replace, this actual app proof. Release exclusion
and owned-resource cleanup are required; screenshots remain in system temp.

## Change record

SWIFTUI-REAL-APP-001 adds a reusable producer under direct user approval. Existing
core0.1/analysis0.2 data consumers are unchanged. Native integration is owned by
chat `01a1daaf-dafe-7e63-b309-3afd2ddec599`; Blueprint owns SDK and numeric checks.
No GPUI source, deployment, global installation or production access is authorized.
