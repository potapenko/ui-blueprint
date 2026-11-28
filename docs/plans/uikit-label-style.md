# UIKit configured label style

User basis: general UIKit/TV capture and testability, tool-only role; finite
TV00-TYPOGRAPHY-1 packet28Nov fromexistingTVroot. No donor/product design changes.
Mode Evolve: opt-in public UILabel configuration observation, no text collection.
Basis registry→UIKIT-RUNTIME@2/TVOS-RUNTIME@1→Simulator/runtime/model/privacy.
Verified TV05common-display-uikit hashes; n41UILabel has layout_bounds only.
Public SDK27.2 UILabel.font/textColor/highlightedTextColor and UIFont name/family/
pointSize/descriptor.symbolicTraits/traits.weight support this configured data.
UIImage.SymbolConfiguration has constructors/equality/removal but no getters for
size/weight; no description/ivar extraction or inference from imagebounds.

Write set: RuntimeUIKit, new focusedRuntimeLabelStyle, RuntimeCollector nodefield,
RuntimeEncoding declarations, RuntimeSession admission, runtime/Simulatorcallers,
focusedfixture/tests, UIKITcontract/registry/Simulator skill. No Rust/corewire,
geometry/default source/API limits/privacy relaxation, player or donor changes.
Explicit --label-style requires sourceuikit/capture and capability; oldendpoints
refuse before dispatch; oldcommands omitfield and collect no style. Canonical
source_declarations on sameoriginalSourceKey bind snapshot/run/window/observation.
Only configuredfontmetadata+resolvedconfiguredcolors+fitflags; effectivepaintfont/
attributedoverride unknown. Colors normalized to named extendedsRGB or unknown.
Oneownedfixture positive/nil/pattern/dynamictrait/attributedcanary tests; olddefault,
negativeadmission/cancel/stale/sourcebinding/privacy/Release checks. Existingrelease
owner finite independentreview; TV05 owns actualreplay after oneSDKbuild.


Candidate checks: actualTVfixture3UILabels configured57/43/29pt; exactpublicfont
names/families/traits, dynamicdarkwhite/lightblack, fitflag/minScale(publicfloat),
highlightyellow, nilcolorunknown, patternunknown.8publicoracleprivacy/availability
checks; text+attributedcanaries absent; effectivepaintfont/color alwaysunknown.
Canonicalvalidatorpass; freshobservations/cancel/stale/malformedflag refused;
legacySwiftUI and actual0.14.1UIKitcalleromitstyles. Pair--label-style passes.
MacSwift14, runtimePython21, pairPython5; iOSDebugcompile/TVReleaseSDKsymbol
exclusion passed. Fixtureassertions corrected for UIKitminimumScale floatstorage
and extendedsRGBwhite1.000000119; outputvalues preserved, no clipping/rounding.
Evidence /Volumes/Develop-2TB/Develop/tmp/uib-label-style-3mjssllt/qualified-final.
PublicAPI research: Apple UILabel.font/textColor/adjustsFontSizeToFitWidth,
UIColor.resolvedColor(with:), UIFontDescriptor.TraitKey.weight plus selectedSDK
AppleTVSimulator27.2headers. SFsize/weight getters absent; remain unsupported.
TV05actualtitle remains pending consumerreplay with one updatedSDKbuild.
