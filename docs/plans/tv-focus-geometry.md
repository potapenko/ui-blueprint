# TV focus geometry qualification

User basis: general UIKit capture request + direct TV capture/tool-only correction
27Nov in01a20a46. Finite continuation TV00-GEOMETRY-FOCUS-1 fromTV root28Nov.
Mode: investigate then restore/qualify general UIKit mapping; no product redesign.
Basis: registry → UIKIT-RUNTIME@1 / TVOS-RUNTIME@1 / SWIFTUI-SIMULATOR@2 /
SWIFTUI-RUNTIME@8 existing geometry/privacy/identity closure. Original0.14 pair
hashes verified: search-a-focused-uikit in TV04 retained tv04-remote-in98tvmw.
Observed: n37cell known mapping; n39/41/44/45focus descendants mapping unknown.
Collector requires CATransform3DIsAffine for every layer transform/sublayerTransform.
Hypothesis (not fact): depth-only/nonprojective3D representation may be stricter
than needed for an affine mapping of the model plane. Saved pair lacks matrices.
First reproduce on own standard TV List and inspect only public numeric transforms.
Then qualify a bounded general rule or preserve unknown. No class-specific rules.
Write set: RuntimeUIKit.swift; focused SDK tests/nativefixture and harness;
UIKIT contract/registry and Simulator skill. Existing endpoints/callers/core,
SwiftUI defaults/privacy/quotas and donor/player protected. No nativeapp edits.
Acceptance: public coordinate oracle, nonzero bounds/ancestor transforms, positive
focus reproduction and negative perspective/nonfinite; canonical/Rust, fresh reads,
Release exclusion. TV04 owns real-app repeating case. Existing release owner owns
finite independent review and packaging if correction qualifies. No new workers.


Observed cause on standard owned List: focused floating transform matrix has
m34=-0.001 with no tilt/depth; old0.14 refused. Qualified exact plane-preserving
chain: m13=m23=m43=m14=m24=0,m44=1; all finite; everylayerzPosition=anchorPointZ=0.
Existing all-affine branch retained. Coefficients still come from UIView.convert
and corners checked at existing1e-7 roundoff guard. No4pt heuristic/class rule.

Candidate proof: five native focus descendants map;39node fullcanonical valid;
Rust model windowwidth1768pt using original transform in evaluation_input.
16matrixcases including effectiveperspective/tilt/tinytilt/depth/anchor/sublayer/
nonzero bounds/nonfinite; fresh2concurrent unique captures/stale refusal/privacy;
legacySwiftUI230nodes valid. Mac Swift14pass; Python19+3pass; iOSDebugcompile and
TVReleaseSDK compile/symbol exclusion passed. Public numeric oracle and PNG red
marker agree at2x within0.5px rasteredge. Savedoriginalunknown remains unchanged.
Evidence: /Volumes/Develop-2TB/Develop/tmp/uib-tv-focus-_hp_gx33/qualified.
Testharness corrections: compare exported nominal type, not rawgeneric typename;
explicit evaluation must carry original transform (spaceflag alone returned
missing_transform); Release nm objectmember headers are not symbols. These were
fixture/harness fixes, not collector bypasses. ActualTV04 freshreplay pending.
Research: Apple UIView.frame/convert, CATransform3DIsAffine and installed27.2
CATransform3D.h. Float homogeneous plane proof is independent of pixel selection.
