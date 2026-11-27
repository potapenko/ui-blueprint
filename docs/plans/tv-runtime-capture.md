# Local TV runtime capture

Authority: direct user message01a21d38-a0b4,27Nov2026, TV planning chat01a21d10:
implement and verify SwiftUI TV capture; each domain has its own Simulator.
The existing SDK owner executes directly, no new agents/chats/Goals.

User outcome: actual PlayPhrase.me TV tree/layout + native PNG, repeatable commands
on explicitly addressed isolated devices. Preserve Phone/Mac sources and TV design.
Observed gap:0.13.0 compiles/runtime-gates UIKit to iOS and pair accepts only
ios-simulator. Installed tvOS27.2 SDK exposes typed _UIHostingView._viewDebugData.
Proposal: extend the same adapter under a distinct tvos-simulator profile, no
platform-tag substitution, new UI, alternate collector or screen-specific marker.

Write scope: root/nested runtime Package.swift; RuntimePlatform/Collector/UIKit/
UpdateContext; Simulator caller; focused tests/contracts/skill. Native current
beta-01 baseline2a9225cf: TV startup hook, target membership and local diagnostic
dependency/recipe only. No GPUI/player/business logic edits or production deploy.

Steps: compile SDK for tvOS; integrate once in actual TV target; build internal
self-contained app with external intermediates; use assigned capture device.
Qualify ordinary/actions/focus/held/fullscreen-return and one system List/child;
record unsupported input states honestly. Coordinator owns actual two-session provisioning/acceptance after the first
ready build; this owner provides routing regression and uses only assigned
D39C49BF-AA6A-4B17-8480-6D6ECFF396F2. No peers' TV instances.

Evidence/storage: /Volumes/Develop-2TB/Develop/tmp/uib-tv-capture-qwb7_z3m and
/private/tmp/uib-tv-runtime-nckv54mo, ownership notes present. Retain final app,
native PNG/canonical pairs and recipe for five consumers; remove disposable build
output only after exact owned processes/consumers release. No public release is
needed for this local task. Actual app source and SDK deltas must be attributed.

Basis: registry → TVOS-RUNTIME@1 → SWIFTUI-SIMULATOR@2 / UIKIT-RUNTIME@1 /
SWIFTUI-RUNTIME@8 (unchanged privacy/identity/geometry/transport closure).
Native registry → features → developer control plane → runtime geometry@4,
ACP-REL-001; TV native action-bar/fullscreen/settings contracts govern scenarios.
Mode Evolve limited to TV source admission. Platform adapter shares existing
code; no UIKit presentation or source value collection is introduced.
First proof: typed SDK tvOS compile + actual native Debug build, first fresh
2208-node canonical capture valid. Runtime 27.2, arm64, window1920×1080pt,
screen scale2; input/state and mapping qualification still in progress.


Final user correction27Nov: Blueprint owns one test instance and tool release;
consumer provisioning/application work remains with the TV coordinator. Stop
expanding screen QA. Source candidate1a9b25e9 accepted finite compatibility review
by existing release owner (0 blockers). Public-oracle fixture proves translated/
scaled bounds and exact native PNG at2x; UIKit model capture valid. Actual native
ordinary/actions/focus/fullscreen/return valid on one app. Settings/child list and
active-held capture remain unqualified after Device Hub timeout/recovery failure.
SDK Release tvOS compile/symbol exclusion passed. Full native Release build was
not accepted: first MCP call timed out while leaving its build alive; stopped
owned build, bounded shell attempt later stopped under corrected tool-only scope.
No Release-app success claim. Parent independently verified two actual consumer
sessions, swapped-device refusal and stop/relaunch isolation; this owner did not
provision them. Evidence retained externally at the existing task path.
