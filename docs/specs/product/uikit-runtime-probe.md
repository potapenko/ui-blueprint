# UIKit runtime geometry
- Domain: `uib.uikit-runtime`; contract: `UIB.UIKIT-RUNTIME@3`.
- Authority: Active; stability: Evolving; accepted/released baseline: none.
- Authority source: user26Nov2026 in SDK chat01a20a46-e5d0-7710-a5ce-9a93d8b82f02:
  add general UIKit popup geometry, release it and hand it to Search, without
  screen-specific fixes or Phone design changes.
- Requires: [SWIFTUI-SIMULATOR@2](swiftui-simulator-probe.md).
- Read when: collecting UIKit hierarchy or selecting the UIKit runtime source.

## UIKIT.SOURCE
Explicit `--source uikit` selects a distinct `uikit.runtime.layout` source through
one existing Debug endpoint. Default requests remain SwiftUI. Advertise
`capture-uikit` only on the qualified iOS27.2 arm64 Simulator profile. Unsupported
platform/profile or an old endpoint refuses before collection; never fall back.
Use capture/tree/measure and optional existing unique nominal subtree selection.
Discovery remains the existing SwiftUI operation; UIKit canonical tree supplies
its class hierarchy. No physical-device, iPad or universal system-surface claim.

Read public UIView subviews, bounds and coordinate conversion on the main actor
for the explicitly selected UIWindow, whether key or nonkey. Preserve every
UIView and parent-child link within that window, not just hosting trees. Do not
read UI strings, accessibility values/labels/identifiers, models or descriptions.
Only nominal class metadata passes the existing address/control redaction.
No mutation, forced layout, presentation, window replacement or screen-specific
class table. UIKit is a narrow diagnostic adapter, not a visible UI implementation.

## UIKIT.GEOMETRY
Original bounds are local top-left points, including nonzero bounds.origin.
Do not serialize UIView.frame as a precise transformed rectangle. For a qualified
2D layer ancestry, derive the local-to-window affine using public point conversion
and verify it against converted bounds corners. The1e-7pt corner-agreement guard
qualifies floating-point roundoff only: it does not alter bounds/coefficients or
change Rust rule tolerances. Additionally admit a finite 3D matrix chain whose
restriction to the model z=0 plane is provably affine: every transform and
sublayerTransform has exact m13=m23=m43=m14=m24=0, m44=1, and every layer has
zPosition=anchorPointZ=0. m34 may be nonzero because this invariant makes its
perspective denominator exactly1. No epsilon weakens these structural guards.
Keep the existing all-2D-affine path. Derive coefficients from public conversion
and verify all bounds corners as before. Any effective perspective, tilted plane,
nonfinite or unverified mapping remains unknown; local bounds remain useful. A window space
is separate from SwiftUI global spaces. No inferred cross-source mapping.

Record window/screen geometry, scale, orientation and scene state as attributed
source declarations. Window-to-screen metadata is not automatic PNG acceptance.
These are model/layout observations, not presentation frames, visible regions,
hit regions, ink or clipping/occlusion results. Consistency stays unknown and the
method explicitly says model geometry; an active animation does not become a
stable onscreen frame. Coverage complete means the selected UIView hierarchy and
requested bounds, not all rendered content or system processes outside the app.

## UIKIT.TRANSPORT
Reuse run/window generation witnesses, exact selection, private capability,
16KiB requests, queue32, node1048576/depth512/output1GiB bounds, cancellation,
correlation/hash validation and atomic publication. Add optional `source=uikit`
only to capture; omit it for legacy SwiftUI requests. Bind the returned source
before exposing a UIKit artifact. Canonical core0.1 and Rust arithmetic remain.
An SDK update requires one new diagnostic app build; later queries reuse it.
Release excludes SDK integration; preparation and consumer application changes
belong to their owners. Packaging/install/source publication belong to releases.

## UIKIT.ACCEPT
Project-owned SwiftUI Menu/Picker: open → capture selected-window UIKit popup subtree → close
→ reopen; fresh hierarchy/bounds, canonical validator and Rust measurement.
Fixture covers nonzero bounds origin/scroll, affine scale/rotation, unsupported
perspective and animated model values. Check stale/ambiguous window, cancellation,
source capability/binding, privacy canary and legacy SwiftUI/Mac compatibility.
Search repeats its original Vocabulary Sorting popup on its own updated instance;
fixture success does not claim that consumer's acceptance.

## Delta and basis
`UIKIT-001`, Evolve: previous Simulator contract explicitly excluded native
surfaces without hosting data; new source additively covers their UIView trees.
Protected: SwiftUI default/source semantics, Mac, core/analysis, UI design, limits,
identity/private transport and app ownership. Review compatibility/privacy once
on the coherent diff. Source observation at4aa09a7d confirms the collector visits
UIView but exports only RuntimeHosting data; this is evidence, not authority.
Selected basis: registry96 → SWIFTUI-SIMULATOR@1 → SWIFTUI-RUNTIME@8 and its
BOUNDARIES@1/MODEL@1/IDENTITY@1/GEOMETRY@2/PRIVACY@1/EXCHANGE@2/LIFECYCLE@1/
CACHE@1/CLI@22 closure; AGENT-SKILL@9 for the portable recipe. UIKit is the only
semantic addition. Remaining dependency: consumer-owned app rebuild and acceptance.


`UIKIT-FOCUS-002`, Restore/qualification under the existing general UIKit/TV
capture request and TV00-GEOMETRY-FOCUS-1: tvOS27.2 standard focused List fixture
reproduced m34=-0.001 with all content on z=0; the prior IsAffine-only guard
refused a planar-affine model mapping. New admission proves plane preservation
across the complete layer ancestry, not four-point coincidence alone. Original
saved unknown records stay unknown; only fresh captures can use qualification.
No new paint/presentation/hit/AABB channel or class-name exception is introduced.


## UIKIT.LABEL-STYLE — explicit configured metadata
TV00-TYPOGRAPHY-1 under the general UIKit/TV capture request adds optional
`labelStyle=true` to UIKit capture only (`--label-style` in callers/pair).
Capability `capture-uikit-label-style`; old endpoints refuse before dispatch.
Omission preserves existing geometry-only behavior and does not read UILabel
style. Reuse identity, quotas, privacy, snapshot hashes and cancellation.
On each UILabel retain source declarations on its original same-observation key:
configured UIFont fontName/familyName/pointSize(pt)/symbolicTraits and optional
finite descriptor weight; configured textColor/highlightedTextColor resolved
against label.traitCollection and converted to extended sRGB RGBA. Record numeric
interface style/contrast/gamut/legibility and configured fit/highlight/enabled
flags. Finite numeric values only; metadata strings max256UTF8bytes, no controls
or path separators. Missing/nonrepresentable values explicitly unknown. Pattern
colors unsupported; nil highlighted color absent, not transparent or black.
No text/attributedText/AX/model/descriptor dump/image description read/export.
Only exact public weight entry is extracted from the public traits dictionary.
Color conversion is attributed, not claimed to preserve an original colorspace.
Configured metadata is NOT effective glyph/paint typography; attributed override,
autosizing, fallback glyphs and rendering effects remain unqualified even if
fit flags are false. UIFont weight is its numeric trait, not an invented CSS
weight. No SF Symbol pointSize/weight inference; no public getters qualified.
Known declarations use UILabel.public-configured-style provenance plus original
snapshot observation/window/run; explicit style schema1 and effectivepaintunknown.
Default source/geometry/core0.1 remain compatible; no historical snapshot upgrade.
Fixture and consumer fresh captures must independently qualify this route.
