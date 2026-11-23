# S15 Practice geometry

Owner: chat `01a2094f-9996-72d3-8230-3b1e4f1ec16d`; consumer
`01a20912-a8de-7040-b51f-6ae6df2d4585`. Branch: `master`.

## Authority and scope

The user approved the presented implementation plan on 2026-11-23 with
«Хорошо, делай по плану»: reuse S13's shared collector, qualify a separate
Practice fixture, then verify actual popup/header/selectors/pills/disclosure/footer
layout and layout-to-window mapping with the consumer. Deliver accepted fixes to
builds chat `01a20929-8e9a-7830-834d-3e558e9ff18d`.

Basis: registry90 → SWIFTUI-PROBE@3 / WINDOW-PROBE@1 (SwiftUI), NATIVE@5,
NATIVE-FIRST-USE@1, SWIFTUI-DEBUG-CAPTURE@15, CACHE-TRANSFORM@4,
MODEL/BOUNDARIES/IDENTITY/PRIVACY/PROJECTIONS/LIFECYCLE@1, GEOMETRY@2,
EXCHANGE@2, CLI@21, ANALYSIS@2 and its TYPES/VALIDATION@1 closure.
The fixture baseline uses committed transform@4; S13's native transform@5
working changes are not part of this result. Implementation/product-truth,
QA, Apple-platform and Computer Use governance apply. Mode: Restore the
requested measurement capability; this receipt creates no new product admission.

Shared collector/contracts and tests remain owned by S13 chat
`01a20922-f6ad-7651-997b-cef1b3f09b12`. Task-owned source:
[SwiftUIPracticeFixture.swift](../../../../tests/bridges/native/SwiftUIPracticeFixture.swift)
and this receipt. No donor, SDK integration, source model, clipboard, account,
signing entitlement or app input/lifecycle changes are authorized here.
Canonical formats, limits, privacy, unknown transforms and existing consumers stay
protected. The consumer owns its application and all real-app state transitions.

## Established source evidence

Immutable native `e19b20c47`, SDK `4e8a9968103d46a68bc06dbef67a06af9eae8690`.
Consumer-supplied source archive: external tmp
`playphraseme-guest-tour-01a20845-sy9r4vhv/playphrase-guest-tour-reference-g2.QO1uoP/src/playphrase.me/playphraseme-mac`.
Read only MacPracticeRoot.swift, MacPracticeMaterial.swift, ServiceWindowSurface.swift,
DesktopSearchView.swift, LearnerModalRoot.swift and the pointing-hand adapter.

Consumer evidence root:
`/Volumes/Develop-2TB/Develop/tmp/playphraseme-s15-practice-g3kbu4r7`.
The saved `ax-review/observation.ndjson` validates with installed0.6.1
(`223cf16808ffa9962ecc28018a8d69e372992c94`); inspect confirms62 AX nodes,
partial coverage and `layout_bounds` not requested. These are accessibility
rectangles in `ax-screen/screen/pt/top_left`, for the older1/1 state.
They are not layout or paint frames and have no qualified pixel transform.
The later `ax-two` belongs to the consumer's2/2 state; no fresh live claim is
made by reading either saved file.

Fresh `sdk-two/s15-two-material-coverage.json` explicitly includes
`download-and-practice-overlays` under unmeasured_app_regions. The separately
captured toolbar-practice Snapshot contains only one toolbar button. Cross-surface
mapping remains unknown. Repeating that SDK request cannot add unmarked parts.

## Fixture baseline and concrete failure

The fixture preserves the source layout operators: fixed560pt card, header/footer
ViewThatFits, nested ScrollView, adaptive LazyVGrid and DisclosureGroup, with
synthetic data. Public SwiftUI anchors are an independent oracle in the fixture
root's local pt/top-left space; repeated roles retain all occurrences.
Native cursor adapters, modal keyboard routing, localization, model loading and
actual app behavior are omitted and cannot be qualified by this fixture. Its
anchors and simplified guide also mean it is not the donor's exact runtime tree.

Built with `xcrun swiftc -parse-as-library -g -Onone -module-name UIBPracticeFixture`
and an external module cache; ad-hoc signed without get-task-allow. LaunchServices
launch uses process-local SWIFTUI_VIEW_DEBUG=29 and internal TMP/TMPDIR/TEMP.
The collapsed window was observed through Computer Use; independent oracle
contains21 named regions, including popup560×412.5pt. This is fixture geometry,
not measured product geometry. Expanded/empty startup states are provided but
have not yet been exercised.

One attempt against a staged immutable collector from `4be9a662` selected
`UIBPracticeFixture.MacPracticeRoot` at PID39384, launch1795453307.054801,
CG2412, bundle `me.uiblueprint.external-layout.canary.s15practice`.
At unchanged ceilings256 emitted nodes/depth,1MiB output and10s acquisition:
**exit2 / cache_scope_scan_limit / calls14 / detached_restored / snapshot=null**.
No quota increase, retry or consumer attach followed. Subsequent native metadata
still identified the same process/window; this is not an interaction pass.

Evidence: `/Volumes/Develop-2TB/Develop/tmp/uib-s15-practice-dhesfnl8`, including
`collapsed-request.json`, `oracle-collapsed.json` and
`collapsed-baseline/manifest-s15-fixture-collapsed-baseline.json`.
Owned runtime and pinned scripts: `/private/tmp/uib-s15-practice-bbrbr9r2`.
The exact failure and source fixture were sent directly to S13 and the consumer.

## Next event and acceptance

Waiting for S13's qualified bounded selector and native-window mapping revision.
That owner has the reproducible scan-limit failure; do not duplicate its reader
or select convenient branches while silently omitting others. A new mechanism
must first pass this fixture, retaining attribution and unknown/refusal behavior.
Only then qualify an exact consumer-owned Practice identity and command. Verify
collapsed2/2 plus an owner-performed state change, every requested part and its
coordinate transform. Missing parts/mapping keep acceptance open.

No actual Practice layout, installed fix, accepted release or complete S15 outcome
is claimed. Source-only fixture progress is separate from product acceptance.
