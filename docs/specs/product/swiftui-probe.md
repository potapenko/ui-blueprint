# Explicit SwiftUI layout probe

- Node type: leaf; domain: `uib.swiftui-probe`; contract: `UIB.SWIFTUI-PROBE@3`.
- Authority: Active; stability: Evolving; prior bounded macOS acceptance below
  does not accept the new window mode, public release or whole-application visuals.
- Authority source: direct 2026-11-14 user approval in chat
  `01a1dab2-2e7d-7f13-9400-8e42b0859137`: implement the first SwiftUI package
  against the real PlayPhrase.me app; subsequent native-chat human correction
  `01a1db4f-dad2-7133-b3d7-da15922655b3` makes internal geometry the acceptance
  outcome, not screenshot/display investigation. Direct user message
  `01a1dbc5-5599-7481-aed8-7eda49313fb1` authorizes @3 root collection.
- Read when: integrating or verifying the opt-in SwiftUI SDK.
- Requires: [NATIVE@5](native.md), [MODEL@1](model.md),
  [GEOMETRY@2](geometry.md), [IDENTITY@1](identity.md), [PRIVACY@1](privacy.md),
  [EXCHANGE@2](exchange.md), [CLI@21](cli.md).

## SWIFTUI.COLLECT — Request-owned source measurement

The concrete Swift package `plugins/swiftui`, module `UIBlueprintSwiftUI`, uses
SwiftUI/Foundation/Combine on macOS14+. Its implementation is DEBUG-only.
The consuming app owns one session per explicitly selected root/window, provides
its real target/run identity and admits requests. No AppKit, reflection, networking,
file IO, model/text collection or input actions belong to the SDK.
Markers have public semantic IDs, declared parents and separate mount UUIDs.
They attach bounds anchors without adding a layout container, hit area, focus or
accessibility element. Resolve selected anchors only for an explicit capture,
on the main actor, in the measured root's local pt/top-left space. Source display
scale comes from that SwiftUI environment, not a process-global screen query.
An explicit event reaches the existing SwiftUI geometry reader; capture must not
mutate observable UI state to request redraw. Mount identity is non-observable;
an old anchor with a renewed mount token refuses instead of acquiring the new ID.
Root-local frames are not AX, hit, paint or visibility bounds. A root frame in
SwiftUI global coordinates may be reported separately; it is not a screen or
capture transform. Missing mapping remains unknown.

## SWIFTUI.SNAPSHOT — Bounded and attributable

Configuration selects root plus required markers, at most256 total, with public
ASCII IDs of1–96 bytes. Default `requiredMarkers` preserves exact-subset callers;
`mountedSubtree` emits all mounted markers for that session, with required IDs as
a minimum. Optional nodes obey the same parent/identity/lifecycle validation;
appear, disappear and identity change invalidate publication across that change.
Missing expected optional mounts refuse as stale. Preferences are session-scoped;
each session's overflow refuses before selection, never silently truncates.
Capture has a unique request ID, public application state
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

Ordinary localhost Search follows [NORMAL-SEARCH@1](swiftui-normal-search.md);
useful window-root qualification follows [WINDOW-PROBE@1](window-probe.md).
The initial profile below covers the real PlayPhrase.me recovery component under
`c02.search.request-failed`; its numeric/resize proof is not normal service proof.
SDK tests cover identities, duplicate/missing/stale data, limits and cancellation.
The SDK must not mutate app state, focus, input or layout to measure it. Verify
actual bounds, identities, parent links, state/reflow/resize and stale refusals
through the real app's internal producer and Rust consumers. Headless tests alone
do not replace this proof. Release exclusion and owned-resource cleanup remain.
Screenshots are optional separate visual evidence, never a geometry source or a
blocking gate for this internal-data package. Exact presented frames, pixel
neutrality and universal screen transforms are not accepted by this package.

## Change record

SWIFTUI-REAL-APP-001/002 qualify the earlier SDK and internal-data acceptance.
WINDOW-ROOT-001 adds explicit mounted-subtree collection under the direct user
instruction above; canonical formats and exact-subset defaults remain unchanged.
Native owner `01a1daaf-dafe-7e63-b309-3afd2ddec599` owns app integration/runtime;
Blueprint owns SDK and numeric checks. Prior screenshots remain unaccepted.
Separate hosting roots retain separate sessions/records and unknown transforms.
## Prior bounded profile

SDK30523aba / nativea5228dd01 produced7 recovery nodes accepted by canonical
validation/inspect/measure/check; source gaps18/7/18pt pass,19pt fails, hit unknown.
SDK10 tests and native27 checks passed. This does not qualify @3 window collection,
external closed-app introspection, unmarked descendants, screenshots or other platforms.
