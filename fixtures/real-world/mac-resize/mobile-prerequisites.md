# Read-only prerequisites for RC04 and RC05

Inventory only, 2026-10-06. No Simulator was booted, installed, changed or captured.
Full bounded inventory with exact runtime IDs/UDIDs is in:
`/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/real-cases/F03b/2026-10-06-1usid7xh/mobile-inventory.json`.
Resolved source build settings are in mobile-build-settings.jsonl in the same root.
These environment IDs are not committed build configuration.

Source project:
[PlayphrasemeApp.xcodeproj](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp.xcodeproj).

- RC04: scheme PlayphrasemeTablet, bundle playphraseme.PlayphrasemeTablet,
  wrapper PlayphrasemeTablet.app, deployment target iOS 17.0, device family 2.
- RC05: scheme PlayphrasemePhone, bundle playphraseme.Playphraseme,
  wrapper PlayphrasemePhone.app, deployment target iOS 17.0; current queried
  device families are 1,2. Use an explicit iPhone destination for RC05.
- Available runtimes: iOS 26.5 and iOS 27.0.
- Both runtimes have shutdown iPad Pro 13-inch (M5) and 11-inch (M5).
- iOS 26.5 has shutdown iPhone 17 Pro / Pro Max / 17e / Air / 17, plus 16 Pro Max.
  iOS 27.0 has shutdown iPhone 18 Pro / Pro Max / 17e / Air / 17.
- A separate PlayPhrase.me Login iPhone was already Booted. Its ownership/state
  was not inspected or changed; do not reuse it implicitly.

Proposed next packet chooses existing supported shutdown devices or explicitly
owned new devices, records exact UDID and ownership, uses landscape-only iPad
(Pro 13 primary, Pro 11 secondary) and explicit iPhone portrait/text-size states.
Native size/appearance, app installation, authentication/data readiness, device
ownership and actual viewport remain unverified until that packet runs.
Availability in simctl is not proof of launch, installed app or physical-device QA.

Use the source [Simulator route](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-simulator-runtime.md),
[build route](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-build-and-test.md) and shared
[desktop lane](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-runtime-lanes.md). Simulator build storage/UDID
ownership is separate from physical input. Do not reserve macos-product solely
for an isolated Simulator build. Capture/input still needs exclusive desktop.
Mobile examples feed agent/engine/export evaluation; they do not add an iOS
collector implementation to P0–P7.
