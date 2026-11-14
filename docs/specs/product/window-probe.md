# Useful geometry from a selected window root

- Node type: leaf; domain: `uib.window-probe`; contract: `UIB.WINDOW-PROBE@1`.
- Authority: Active; stability: Evolving; no released or runtime-accepted baseline.
- Authority source: direct user message `01a1dbc5-5599-7481-aed8-7eda49313fb1`,
  2026-11-14, requesting root/window geometry and completion of the matching release.
- Requires: [SWIFTUI-PROBE](swiftui-probe.md), [GPUI-PROBE](gpui-probe.md),
  [MODEL@1](model.md), [IDENTITY@1](identity.md), [GEOMETRY@2](geometry.md),
  [PRIVACY@1](privacy.md). Select only the platform being changed or verified.

## WINDOW.SCOPE — Root selection and coverage are separate

When the user selects a window without naming a smaller component, use its
authorized app-owned root as the acquisition scope. Collect the useful mounted
marked descendants, not only the root rectangle or a fixed demo subset. Explicit
component selection remains supported. The SDK does not discover unmarked private
framework internals. Instrumentation belongs to the app owner and remains opt-in.
The caller does not repeat a root ID or marker list for each request: an omitted
component selector selects the registered root of that exact window. Ambiguous or
unavailable window/root binding refuses; it never falls back to a small sample.

The integration names its meaningful regions and controls before qualification.
Report which are measured, absent in the selected state, unmaterialized, unsupported
or unverified. A marker count alone does not prove useful window coverage.
App-owned toolbar/content/overlay surfaces cannot silently disappear from that account.
Native chrome and system menus are not recast as internal layout measurements.

## WINDOW.COLLECTION — All mounted markers with bounded validation

Root collection uses all admissible mounted marker occurrences. Required anchors
define a minimum; optional/conditional branches do not become permanent required
nodes. Preserve an explicit exact-subset mode for established consumers.
Every emitted node obeys the existing identity, geometry, parent, privacy, lifecycle,
deadline and byte/node limits. Truncation never looks complete: refuse overflow or
return an explicitly supported partial outcome, never a silently filtered success.
Remount and virtualized-row reuse invalidate old occurrences. Structural labels
and opaque mount IDs are public; raw item, account, query or caption values are not.
Mounted layout is not proof of visibility, clipping, occlusion or painted bounds.

## WINDOW.SURFACES — One selection, attributable source records

An app-owned window request may obtain multiple bounded source-root records when
SwiftUI/GPUI hosts toolbar, sheet or overlay content separately. Preserve each
root's surface identity, local origin and canonical Snapshot bytes. Report each
requested surface's outcome. Do not merge unrelated origins or synthesize a screen
transform; cross-surface geometry stays unavailable until its mapping is proved.
Do not introduce another collector beside the existing platform producer.

## WINDOW.ACCEPT — Real useful window, installed route

For SwiftUI, qualify the real PlayPhrase.me learner Search window with localhost
API and ordinary returned media: window content, video workspace/transport/study
controls, query, suggestions, active learner panel and a shown in-content overlay.
Toolbar requires an explicit measured or unresolved surface outcome. Verify actual
resize and dynamic branch/row lifetime, current canonical geometry and refusal of
stale required occurrences. Preserve product input/playback and Release exclusion.
Unmaterialized rows and unvisited destinations remain gaps, not invented nodes.

For GPUI, verify the corresponding root behavior in the existing real native
technical example. Client application coverage is a separate app-owner integration.
Saved-data Rust analysis, SDK tests and actual runtime results are distinguished.
The installed skill must discover these routes and analyze actual source records;
source-only instructions in a checkout are not installed-skill completion.

WINDOW-ROOT-001 evolves selected-subset proof into useful root acquisition under
the direct user command. Existing wire formats, app behavior, permissions and
geometry meanings stay unchanged. Runtime acceptance and public release require
their separate evidence; this contract does not assert them.
