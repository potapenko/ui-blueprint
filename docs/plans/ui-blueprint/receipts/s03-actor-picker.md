# S03.A Actors picker — acquisition dependency

Status: **open; no picker geometry or fix accepted**. Updated 2026-11-23.

## Scope and authority

The operator's separate-bug-chat directive, human turn
`01a20921-a2b9-7dc0-ab2c-9106d7cd76aa` in coordinator
`01a1e565-14f6-7be0-aa02-cd648a1b0423`, assigns this finite investigation and
repair to `01a20923-023c-7cd0-8421-a2e20177df71`. Consumer:
`01a20911-330a-7fa2-aa96-980371b389a7`.

Required result: source-layout geometry of the open PlayPhrase.me actor picker,
including header, Close, search field, toggle, scroll area, suggestion row and
label, with explicit units and coordinate spaces. Donor source `e19b20c47`,
SDK `4e8a9968`, UI authority `7de6680a` remain immutable. Xcode Inspector,
AX rectangles and PNG estimates are not substitutes. The coordinator owns
versioning, managed installation and publication.

Basis: [registry](../../../specs/README.md) →
[SWIFTUI-PROBE@3](../../../specs/product/swiftui-probe.md),
[WINDOW-PROBE@1](../../../specs/product/window-probe.md), and their required
MODEL, GEOMETRY, IDENTITY, PRIVACY, EXCHANGE, NATIVE, CLI, BOUNDARIES,
PROJECTIONS and LIFECYCLE clauses. The external alternative is governed by
[SWIFTUI-DEBUG-CAPTURE@14](../../../specs/product/swiftui-debug-capture.md).
The installed `ui-blueprint` skill and its macOS/analysis recipes were read.
Restore is the intended repair mode; current work is bounded acquisition
diagnosis, with no semantic contract change. Write scope at this checkpoint is
this receipt only. Shared SDK/collector and S13/S14 files remain with their owners.

## First concrete result

Reviewed source at `91afbd661d4f791b121764bce3a33f81963aa5ae`:

- [ProbeModifiers.swift](../../../../plugins/swiftui/Sources/UIBlueprintSwiftUI/ProbeModifiers.swift)
  collects `ProbeAnchors` created by the existing root/marker modifiers and
  resolves selected anchors with `GeometryProxy`. It does not enumerate
  unmarked SwiftUI children. A changed CLI command or larger limit cannot
  supply missing anchors in an unchanged donor.
- The installed bundle manifest identifies version-directory `0.6.1` and
  source `223cf16808ffa9962ecc28018a8d69e372992c94`. An offline call to that
  bundle's `require_admitted_target` with a synthetic non-fixture identity
  returned `product_capture_withdrawn`. No process lookup or attach occurred.
  This tests the admission gate, not the live picker or the cache decoder.
- [The cache reader](../../../../plugins/macos/swiftui_ag_cache.py) has a
  structural body-owner selector. Its contract requires exact source ownership,
  bounded unique selection and qualification before consumer admission.
  Per-node cache positions do not establish shared coordinates. The current
  [runner](../../../../plugins/macos/swiftui_debug_capture.py) has
  `CONSUMER_ADMISSION = None`; this investigation did not alter it.
- Apple's [GeometryProxy](https://developer.apple.com/documentation/swiftui/geometryproxy)
  and [NSHostingView](https://developer.apple.com/documentation/swiftui/nshostingview)
  documentation describes local anchor resolution and the native hosting
  boundary. These checked public APIs do not supply a demonstrated external
  per-child layout route for this immutable donor. This is a bounded finding,
  not a claim that all external mechanisms are impossible.

## Pending consumer evidence

One consolidated request was sent to the consumer for the new open-state
manifest, exact command/scenario, source/SDK revision, live window identity and
available picker markers/roots. At this checkpoint the consumer's latest
reported collection is closed-state; it names
`actor-picker-and-authentication-overlays` as unmeasured. The consumer was
waiting for its desktop input lane. That result and the historical toolbar
89×28 frame do not measure the requested picker children.

Next event: the owner returns the open-state report and source-bound selector
facts. Analyze its original canonical records before choosing a correction.
If the same missing-anchor boundary persists, qualify an external scoped source
and its coordinate mapping with the existing collector owner; do not request
donor instrumentation, enable a generic product admission, repeat whole-window
scans, or relabel another geometry channel. Repeat the original scenario through
the consumer only after a qualified candidate exists. No release is warranted
by this checkpoint alone. Runtime, resize and consumer acceptance remain open.
