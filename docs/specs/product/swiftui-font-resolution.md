# Mounted SwiftUI font resolution
- Contract: `UIB.FONT-RESOLUTION@1`; domain: `uib.swiftui-font-resolution`.
- Authority: Active / Evolving, user generalTVcapture/Debugintegration/tool-only
  scope, finite TV00-SWIFTUI-FONT-1 continuation28Nov2026.
- Requires: [UIKit](uikit-runtime-probe.md), [TV](tvos-runtime-probe.md).

## FONT.MOUNT
Explicitly integrated Debug-only `BlueprintFontResolutionProbe(font:role:)`
reads public Environment.fontResolutionContext at its actual SwiftUI mount.
The supplied token must be the same Font expression as the attributed source
role. The probe does not discover arbitrary Text fonts. Never use default
EnvironmentValues, reflect Font/Context/model or read text/attributed runs.
A transparent zero-sized noninteractive background UIView carries the current
SwiftUI update's resolver and mount/context revision. No global registry,
notification loop or layout mutation. Only process opt-in
UIBLUEPRINT_RUNTIME=1 plus UIBLUEPRINT_FONT_RESOLUTION=1 mounts the anchor.
Minimum app deployment stays17; public resolve API26+ is availability gated;
older OS mounts nothing. Qualified profile remains Simulator27.2arm64.

## FONT.CAPTURE
Add optional `fontResolution=true` to explicit UIKit capture, capability
`capture-swiftui-font-resolution`, callers `--font-resolution`. Default capture
never evaluates the resolver. Old endpoints refuse before dispatch. Reject other
sources/operations/nontrue wire values. Same file admission/identity/limits/cancel.
Fresh capture invokes Font.resolve(in:) using the most recent mounted SwiftUI
context; no guarantee of draining pending UI updates. Record consistencyunknown.
The diagnostic anchor node's original key/window/run/observation owns declarations
`font_resolution_*`: schema1, static role, mountID, contextRevision, contextbasis,
pointSizept, CTFont public PostScript/family names and symbolic traits, resolved
isBold/isItalic/isMonospaced. Bounded metadata strings/finite numbers only. Do not
export rawFont/Context/CTFontdescriptors, arbitraryenvironment, titles or modeldata.
Paint/glyph/fitting/attributedfont remainsunknown. Role attribution is an explicit
app integration, not a fabricated link to an independently captured Text node.
The anchor's geometry is not the Text rectangle. Unmount removes its live node;
no archived resolver is queried after it leaves the selected window.

## FONT.ACCEPT
Use oneownedfixture: same actualtitlecomposition/probeoff-on nativePNG/public
bounds agree, environmentchange updates resolvedfont, removal removesprobe,
olddefault/oldendpoint/cancel/stale/privacy/Release checks. Newprobe build has
explicit sourcepin+diagnosticdelta. Consumer restores its actualheader and verifies
freshmetadata plus unchangedlayout/appearance; fixture does not close that case.
No automatictypographyclaim for allSwiftUI or olderunqualifiedplatforms.
