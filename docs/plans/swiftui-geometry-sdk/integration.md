# Native integration and build plan

Part of [PLAN.UIB.SWIFTUI-GEOMETRY@1](../swiftui-geometry-sdk.md). This is prospective scope; no app modification or launch was performed while writing the plan.

## Standard requirement for a consuming SwiftUI project

1. Link the version-pinned `UIBlueprintSwiftUI` and optional `UIBlueprintCaptureHost` products through SwiftPM.
2. Own a Debug capture host for the explicitly enabled process and register each actual scene/window and independently hosted presentation. Product models remain app-owned; the host observes only declared public state/generation.
3. Attach the collector to existing roots and mark meaningful component/part boundaries. Use the same registration API in shared components; do not duplicate each screen's collector or serializer.
4. Admit requests only during an explicitly enabled diagnostic run and write only to that run's private sandbox directory. Supply the session descriptor to the installed caller.
5. Keep a finite coverage contract: required parts, conditional parts, relevant states and unsupported system content. Adding a new measurable component includes its marker integration and one real capture check.

Instrumentation is part of Debug development support. It does not require a public preference, settings screen, debug button, overlay, custom layout container, changed Accessibility tree or production endpoint. A manifest saying `partial` is not a permanent exemption for required app-owned parts.

## Build configuration

Retain `#if DEBUG` and explicit activation rather than introducing another global build mode. Build the existing product targets/configurations with a bounded diagnostic launch helper. Ordinary committed Run schemes stay Release; no token, account, endpoint, device or diagnostic switch is saved into them. No new signing entitlement, get-task-allow override, sandbox disablement or LLDB backend is needed for the proposed primary path.

Release compile excludes collector/host implementations, ingress activation and IO. Dependency linkage may remain declared, but no capture facility or initialization may survive in Release. Shared source instrumentation uses additive Debug blocks and preserves the existing Release body/composition; follow the native prohibition on Debug/Release alternative product implementations.

Add macOS14 and iOS17 package platform declarations; keep Swift tools compatibility grounded in actual compiler checks. Add tvOS17 declaration only with a compile smoke if shared source needs it; tvOS runtime collection is not included in the current delivery claim. Do not raise app deployment floors or confuse iPad support with a separate SwiftPM platform.

Use root package URL + immutable release tag after publication. During co-development the existing local `../../potapenko-github/ui-bluprint/plugins/swiftui` reference can be retained with the exact commit recorded; it is not a version pin. Verify source equality before qualified builds. Migrate to remote resolution in one coherent native checkpoint, retaining the same module names.

The helper builds once per app/SDK revision and reuses the owned running instance for successive requests. It records app commit, SDK revision, CLI version, configuration, actual platform and instance/scene binding. Rebuild only after source/dependency changes; do not rebuild for every geometry query or restart another chat's app.

## Mac: existing source owners

Refactor reusable mechanics from `PlayphrasemeApp/Application/DeveloperControl/MacBlueprintProbeHost.swift` into the optional package host. Keep native activation/public-state policy in a thin application adapter. Replace the Search-only guard in `MacApplicationRootViewBlueprint.swift` with registration of the actual supported destination/scene; malformed/disabled geometry requests must still never fall through to authentication or public app routes.

Keep `MacBlueprintProbeScope` and toolbar hooks compatible. Add scopes at existing application, popup and overlay roots; use retained scene/session owners outside `body`. Actual scene occurrence, not a process-global selected destination, addresses multi-window requests. The initial app remains its current window composition; this work does not add windows or navigation.

## Mac coverage matrix

| Slice | Required geometry / relations | States proving integration |
| --- | --- | --- |
| Random completion (P1) | Card; Close, Replay, Next; applicable icon, label, keycap; actual parent links. | favorite-empty, favorite-one, common-next, word-next; resize; close/reopen. |
| Actors (P1) | Popup, header, Close, search field, list viewport, materialized row and label boundaries. | Empty/loading/results as existing UI supports; scroll/reuse; resize; close/reopen. |
| Practice | Outer presentation, header/footer, text/disclosure, selectors and materialized pills. | Empty lesson, collapsed/expanded, selection and resized presentation. |
| Guest Tour | Viewport, named card, target, group, measured spaces; declared anchor relations. | Selected card in wide/narrow windows; conditional card appears/disappears. Existing product anchors may inform registration without reading product model payloads into SDK output. |
| Access/Paywall preview | Existing card/CTA plus each mounted grid cell and grid root. | Loading, loaded/mixed placeholders, variable count and resize; no AX requirement for accessibility-hidden decorative cells. |
| Clip Search | Screen roots, filters/Genre trigger, app-owned popover content, grid/viewport. | Open/closed filter; actual parent/presentation binding; responsive grid and scroll. |
| Reels | Active card, media boundary, title/action parts and access overlay. | Active-card replacement, paused/playing capture, resize and mounted overlays. |
| Library/Grammar/Statistics | Existing panels plus required filter/control/detail boundaries and declared relations. | Relevant selection, empty/populated, shown detail/tooltip, scroll/resize. |
| Settings and service overlays | Existing settings markers completed to meaningful groups/controls; app-owned account/auth/subscription/download/notices where needed. | Open/close and conditional controls; geometry-only output with no values or sensitive strings. |
| Window and toolbar | Existing 12 named toolbar roots, content and safe-content boundaries; source relationships. | Wide/narrow, unavailable system hosts explicitly distinguished; supported mappings verified. |

This table is the integration scope, not an assertion that every state is present on every platform. Before editing a slice, select its existing product contract and inspect its concrete source owner. Do not invent UI states, change layout to expose a node, or add product actions to make QA convenient.

Each row names exact marker expectations before its first run. Shared button/card primitives should accept scoped diagnostic IDs once; use explicit local marker sites where no genuine shared owner exists. Do not build a universal UI wrapper or refactor product architecture merely to reduce marker lines.

## iPhone and iPad: shared infrastructure, separate app composition

Inspect actual target membership first. `PlayphrasemePhone` is the distribution target for iPhone and iPad; it launches the corresponding existing composition. `PlayphrasemeTablet` remains the standalone iPad development target, and `PlayphrasemeTabletShell` carries the Tablet sources without copying. Link Debug SDK/host to each actual compiling consumer, including framework targets where marked shared source is compiled.

Integrate at `PlayphrasemePhone/Application/PlayphrasemePhoneApp.swift`, `PlayphrasemeTablet/PlayphrasemeTabletApp.swift` and their existing scene roots. Preserve public/authentication URL handling. Reuse common host/protocol/serialization; only scene binding, explicit launch admission and sandbox transfer differ. Do not reuse Mac models or import AppKit into mobile sources.

Register existing Search, Clip Search, Reels, applicable learner panels, sheets/popovers and nested scroll content. Reuse marker declarations only where the actual source component is shared. Similar-looking Phone and Tablet implementations retain independent marker sites with compatible semantic meaning.

Initial acceptance uses the existing Simulator pool: one available iPhone and one landscape iPad. The shipped iOS distribution target must be checked on both idioms; the standalone Tablet target receives its affected build/smoke check. Do not create new persistent devices to execute this plan.

Simulator helper uses explicit UDID/bundle and `simctl get_app_container ... data`. It supplies the same committed request files and retrieves the same result manifest; it does not open a URL per sample. Device helper uses explicit development-device/bundle app-container transfer. App must already be in the authorized measurable state; locked/background/unpaired devices are concrete prerequisites, not reasons to change app entitlements or navigation automatically.

Exercise keyboard appearance, safe area, Dynamic Type, scrolling/reuse and applicable orientation/window-size changes. iPad interaction evidence stays landscape under the current repository rule. Geometry returns actual current units/space/context; Mac spacing is not a mobile design requirement.

## Storage and runtime ownership

Before heavy writes verify `/Volumes/Develop-2TB` is mounted and the destination writable. Build intermediates, DerivedData, compiler/package caches and retained evidence go to a uniquely owned external task area. Use supported build flags; TMPDIR alone does not relocate caches.

Assemble/run the self-contained Mac `.app`, local release packages and runtime/data/temp on the internal disk under one owned `/tmp` or native temp directory. Do not inherit external compiler TMPDIR into the app. Simulator/device sandbox is the app's runtime boundary; copy selected outputs to the external evidence area after publication.

Record small ownership/retention notes. Preserve old reference builds, accepted observations and peer runs. Clean only new owned disposable outputs after consumers exit; checkpoint source changes on the currently selected branches. No production service changes, store upload, host reboot or publication of the native app is part of this integration.
