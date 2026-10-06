# F03c-build — PlayPhrase.me mobile build receipt

- Result: both canonical Release Simulator schemes built successfully (exit 0).
- Status: build-artifact preparation complete; checkpoint saved by this receipt’s commit.
- Root granted exclusive receipt-only Git lease; release immediately after checkpoint.
- Acceptance scope: installable artifacts/metadata only; advisor receives runtime handoff separately.
- Consumer: F03c RC04/RC05 product advisor; no runtime/product acceptance claimed.
- Only repository write: this receipt in UI Blueprint `master`.
- Source repository remained read-only; unrelated UI Blueprint/S01 changes untouched.

## Authority and traversal

[Packet](../packets/F03c-build.md) → UI Blueprint AGENTS and
[registry](../../../specs/README.md) → [approved plan's direct-user amendment](../../ui-blueprint-development.md#прямые-уточнения-пользователя-от-2026-10-06),
[task registry](../task-registry.md), [execution](../execution.md),
[product-context receipt](product-context.md),
[mobile prerequisites](../../../../fixtures/real-world/mac-resize/mobile-prerequisites.md).
Source AGENTS → docs/specs/README.md (behavior-neutral build; no feature leaf selected)
→ docs/agent-build-and-test.md + docs/agent-simulator-runtime.md + docs/agent-tooling.md.
Read helper/timeout and committed scheme/build-phase definitions for output isolation.
Global implementation/QA governance applied; ios-debugger-agent skill narrowed to
build-only as explicitly requested. Direct xcodebuild used for isolated storage,
fixed dependencies, generic destination and bounded execution. No source repair,
product choice, runtime lane, device ownership or desktop reservation was needed.

## Source identity and preservation

Source: `/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac`; branch `beta-01`; HEAD `48d0dfd1cd0fe46ac88c073e99ffab7bd1abe76e`.
Existing input change: ` M PlayphrasemeApp.xcodeproj/project.pbxproj`.
Working diff SHA-256: `8ce3a0a3fb4e3e6b7441dd8e98c52c39e8be3771a57df8ec0d54169a8d9ce85a`.
Project SHA-256: `7b26d3ad560da98ae06d406da7b7397238075009eb648e7b6df22ff89530ee75`.
Package.resolved SHA-256: `8489277d33ba538ab09d3c6807e020dff8ede668fb0776f61f78259f57041e87`.
Before/after branch, HEAD, full porcelain status, binary HEAD diff, index diff,
project bytes and lockfile bytes matched. Source index remained empty. These
products represent HEAD plus the existing project edit, not pristine HEAD.

## Build invocation and outputs

Xcode 27.0 (`27A266a`), installed `iphonesimulator27.0` SDK (`24A430`).
Task-owned marked root (created by source cleanup helper):
`/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/real-cases/F03c-build/playphrase-f03c-build.6vPRSB`.
[Manifest with exact invocations and metadata](</Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/real-cases/F03c-build/playphrase-f03c-build.6vPRSB/manifest.json>).
Both commands ran sequentially with the same isolated DerivedData/SourcePackages;
2400-second build deadline and 30-second destination deadline. Equivalent template:

```text
python3 /Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/scripts/with_timeout.py --timeout 2400 -- xcodebuild -project /Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp.xcodeproj -scheme SCHEME -configuration Release -sdk iphonesimulator -destination 'generic/platform=iOS Simulator' -destination-timeout 30 -derivedDataPath '/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/real-cases/F03c-build/playphrase-f03c-build.6vPRSB/DerivedData' -clonedSourcePackagesDirPath '/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/real-cases/F03c-build/playphrase-f03c-build.6vPRSB/SourcePackages' -disableAutomaticPackageResolution -onlyUsePackageVersionsFromResolvedFile -skipPackageUpdates ARCHS=arm64 build
```

- `PlayphrasemeTablet`: `playphraseme.PlayphrasemeTablet`, version 1.0 (1); families `[2]`.
  App: `/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/real-cases/F03c-build/playphrase-f03c-build.6vPRSB/DerivedData/Build/Products/Release-iphonesimulator/PlayphrasemeTablet.app`.
  Executable SHA-256: `2d28160327d369633e5d8bfd36053cc907c2d0a7e210a5a171c2889100a968fe`.
- `PlayphrasemePhone`: `playphraseme.Playphraseme`, version 1.2 (6); families `[1, 2]`.
  App: `/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/real-cases/F03c-build/playphrase-f03c-build.6vPRSB/DerivedData/Build/Products/Release-iphonesimulator/PlayphrasemePhone.app`.
  Executable SHA-256: `16450521953ae5ec0dbacdf521309cbf80ff0c43bc3a072eba26fa374e941ca2`.

Actual plist and Mach-O checks: both arm64, IOSSIMULATOR, minimum iOS 17.0,
SDK 27.0. `codesign --verify --strict` exit 0 for both; Release fixture bundle
absent for both. This is local bundle integrity, not device/distribution signing proof.
Tablet/Phone build logs contain 45/86 warning lines respectively (not unique
issue counts); no build errors. No tests or runtime scenarios executed.

## Handoff and cleanup

- Advisor must obtain runtime/device and desktop grants, choose exact owned arm64
  Simulator targets, and verify install/launch and scenario readiness under current
  source runtime routes. No UDID was selected, created, booted or used here.
- RC04 uses Tablet on iPad in landscape; RC05 uses Phone on an explicit iPhone
  in portrait despite its family list including iPad. Installed SDK is 27.0;
  runtime compatibility/launch, authentication, data and visible states remain unverified.
- No Simulator install/launch, app input, physical-device operation, SDK/runtime
  installation, project/lock/defaults/signing/credential change was performed.
- Raw logs and temporary runner: `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-f03c-build-qj7tg4al`.
- `Storage cleanup: deferred_handoff`. Owner=root; consumer=F03c advisor. Retain
  bundles/minimal manifest until F03c handoff/completion, then remove only this
  marked root when directed using source helper `remove-task-root EXACT-ROOT --apply`
  with `PLAYPHRASE_TASK_TMP_ROOT` set to its parent. Remove only this run’s temporary
  log directory after result acceptance. Never clean shared build or Simulator data.
- Receipt validation: local links exist, scope/routes agree, `git diff --check`
  passed for this file before the commit-only handoff. Runtime/product acceptance
  remains pending; no new builds, tests or runtime checks in the checkpoint step.
