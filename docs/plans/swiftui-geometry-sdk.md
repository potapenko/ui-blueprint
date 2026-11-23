# SwiftUI geometry SDK: execution plan

- Plan: `PLAN.UIB.SWIFTUI-GEOMETRY@2`, 2026-11-23; status: method selection reopened by direct user correction. The whole-application outcome below supersedes the component-by-component delivery premise of revision 1.
- Blueprint plan/SDK owner: chat `01a20a46-e5d0-7710-a5ce-9a93d8b82f02`. The user designated [Добавь геометрию в сборку](codex://threads/01a20a56-736d-7751-a552-792eecdddb68) on 2026-11-23 as the native integration counterpart. This authorizes the named handoff, not new chats/workers or execution of the whole proposed plan.
- User authority: the 2026-11-23 reply explicitly says «разрешаю добавить какое-то дополнительно в Mac-приложение, чтобы мы нормально собирали информацию с него» and «подготовь план». It agrees with the audit and requests Mac/future iPhone/iPad coverage. This authorizes this plan, not its execution now.
- The former immutable-donor restriction is not a prerequisite for the proposed integration. Preserve old evidence/builds; qualify a new instrumented source revision instead of relabelling old observations.
- Source basis inspected: UI Blueprint `master` at `10cd1d8e`; PlayPhrase.me `beta-01` at `506527116a481617c9e75b816b1c07bd5c44a191`; installed/published CLI `0.6.1` at `223cf168`. Revalidate overlapping changes before implementation; stay on each current branch.

## Outcome

One application-wide diagnostic integration/build serves every requesting chat. One request captures the full available mounted hierarchy of the selected app/window, including its current presentation roots, with source geometry. Selection/measurement of individual components happens on that capture; it must not require new marker edits, a special app build or a new extraction chat. Rebuild only for actual source/dependency changes, never for each capture.

The user explicitly rejected delivery as separate Random/Actors/Practice instrumentation tasks and pointed to Xcode's Debug View Hierarchy. First qualify that existing whole-hierarchy source and its saved archive; do not assume manual marker coverage is required. Screens not instantiated yet have no current layout; they enter later captures when normally opened in the same build. This does not justify omitting mounted nodes or redefining full capture as a small sample.

## Revision 2: source selection before global instrumentation

The new primary route is [Xcode hierarchy qualification](swiftui-geometry-sdk/xcode-hierarchy.md). Apple documents SwiftUI hierarchy/size inspection and export to `.viewhierarchy`. Source review also identified `LNViewHierarchyDumper`, a reusable development-only caller of Xcode's internal hierarchy framework. Neither route has yet been qualified on this app/Xcode27.2. There is no evidence here that per-element annotation is necessary or that the library already supplies complete Mac SwiftUI output.

Required order: real owner-produced Xcode reference capture → inspect original archive and import its hierarchy/geometry → qualify repeatable whole-app acquisition → one integrated diagnostic build and installed consumer path → Mac-wide and mobile acceptance. Named dialogs are regression cases within that one delivery, not separate products or consumer prerequisites.

The remaining revision-1 SDK design below and its three detailed leaves are retained as a **conditional alternative**, not the selected implementation. Do not begin widespread manual annotations, a file transport, macros or a new host solely on that earlier proposal. Reuse existing SDK where useful; select further integration only against observed gaps in the Xcode route. Old decoder-specific bans/failures do not establish that Apple's supported Xcode UI capture fails; they also do not automatically authorize reusing the withdrawn decoder.

## Basis: requirement, implementation, proposal

| Kind | Basis |
| --- | --- |
| Current contract | [SWIFTUI-PROBE@3](../specs/product/swiftui-probe.md), [WINDOW-PROBE@1](../specs/product/window-probe.md): explicit Debug measurement, declared parents, bounded requests, truthful coverage, real-app proof. |
| Required closure | [BOUNDARIES@1](../specs/product/boundaries.md), [NATIVE@5](../specs/product/native.md), [MODEL@1](../specs/product/model.md), [IDENTITY@1](../specs/product/identity.md), [GEOMETRY@2](../specs/product/geometry.md), [PROJECTIONS@1](../specs/product/projections.md), [EXCHANGE@2](../specs/product/exchange.md), [PRIVACY@1](../specs/product/privacy.md), [LIFECYCLE@1](../specs/product/lifecycle.md), [CLI@21](../specs/product/cli.md). |
| Native contract | `playphraseme-mac/docs/specs/README.md` → developer control plane → `swiftui-geometry-probe.md` (`geometry@4`), lifecycle/Release leaves; `platform-shells.md` r8 preserves Mac14/iOS17/tvOS17 and distinct Phone/Tablet compositions. |
| Delivery contracts | [AGENT-SKILL@7](../specs/development/agent-skill.md), [RELEASES@2](../specs/development/releases.md), [MAC-DISTRIBUTION@8](../specs/development/macos-distribution.md), [DEV.RUST@5](../specs/development/rust.md); current AGENTS source-release authority takes precedence over older per-release gates. |
| Observed implementation | SDK code last changed at `4e8a9968`; the native host accepts Search & Learn only, has no component selector, and names major overlays as unmeasured. SDK already emits parent/child geometry; it does not discover unmarked internals. |
| Proposed evolution | Reusable capture host, component catalogue/selection, complete app-owned measurement scopes, portable package entry, mobile adapters and an installed end-to-end command. These are proposals here, not already Active contracts or delivered capabilities. |

Mode on approval: **Evolve** in diagnostic observation, SDK/host API, CLI and integration domains. Preserve application behavior, visual design, accessibility, focus, input, playback, authentication, storage, deployment floors, public routes, legacy snapshots and existing AX/Web/Compose/GPUI behavior.

Before source edits, register the approved deltas in the selected contracts. Add bounded leaves for capture-host and component-query responsibilities; link them through the existing registry. Do not copy this plan into specifications or load unrelated product contracts. Open each affected screen's existing route before touching its source or running its scenarios.

## Revision-1 alternative: not selected pending Xcode qualification

1. Extend `UIBlueprintSwiftUI`; do not add SwiftUI Introspect/ViewInspector, a second geometry engine, a new service or a macro framework.
2. Keep the collector pure SwiftUI/Foundation/Combine. Add one optional `UIBlueprintCaptureHost` target in the same package for reusable admission, request IO, catalogue and publication; app adapters retain product state and scene ownership.
3. Use explicit component/part markers on existing views. Component discovery means registered application components, not every private SwiftUI implementation view.
4. Use a private file request channel for repeated captures; existing URL capture remains compatible. Requests do not activate windows or open popups. Qualify this transport on Mac first.
5. Keep current source-local canonical geometry and Rust arithmetic. Prove mappings where available; never promise a universal screen transform from a root rectangle.
6. Ship a standard tag-pinned Swift Package Manager entry. Keep legacy local-package consumption working during migration.
7. Deliver a real Mac vertical slice before broad instrumentation, then finish the Mac coverage matrix, then qualify iPhone/iPad with the same SDK and protocol.

## Revision-1 implementation sequence: conditional alternative

| Step | Work | Exit condition |
| --- | --- | --- |
| P0 | Register approved contract deltas and finite component inventory; establish current writers and exact branches/builds. | Named scopes, parts, states, source owners and acceptance rows exist; no implicit product redesign. |
| P1 | SDK/host minimum plus Random and Actors integration and one installed capture command. | Actual component tree and selected sizes/gaps on both real dialogs, then resize and close/reopen, without debugger; legacy Search still works. |
| P2 | Scope partitioning, component queries, root relationships and remaining Mac instrumentation. | Every row in the Mac matrix is verified or names an exact remaining unsupported field; required missing fields keep that row open. |
| P3 | Package root, version-pinned dependency and automated integration checks; installed skill. | Fresh checkout resolves exact source, builds Debug/Release, and follows the documented command without repository experiments. |
| P4 | iOS package support, Phone/Tablet adapters and representative complete flows. | Real app on iPhone and landscape iPad Simulator uses the same component protocol; affected shared consumers still build. |
| P5 | Physical iPhone/iPad transport qualification when an authorized device is available. | Same app-container request/output route works on the selected development device; otherwise this device profile remains pending, with Mac/Simulator delivery usable. |
| P6 | Final compatibility/privacy/concurrency review, version, installation and source release via existing release owner. | Verified revision installed, tag and source release checked, consumer instructions name app+SDK+CLI versions and exact support matrix. |

P3 packaging can follow the first useful P1 slice; do not delay Mac outcome for mobile/device availability. Reuse coherent review evidence. Two comparable failures without outcome improvement trigger a new hypothesis, not another special-case reader or larger quota.

Within P1 first prove the missing parts by extending the existing app-owned SDK route. Reuse its current admission/output until real Random/Actors geometry exists; then extract the reusable host and qualify quiet file transport against the same cases. Do not postpone this first result for the full catalogue, remote SwiftPM packaging or the mobile adapter.

## Detailed implementation routes

- [SDK, host, protocol, geometry and CLI](swiftui-geometry-sdk/architecture.md).
- [Mac/iPhone/iPad integration, build and coverage](swiftui-geometry-sdk/integration.md).
- [Acceptance, compatibility, release and handoff](swiftui-geometry-sdk/verification.md).

## Write scope and ownership

This correction changes this plan and its linked planning documents only. It records the new user outcome and research evidence; no application source, package integration or live Xcode capture has been changed by the correction.

On implementation approval, UI Blueprint owns `plugins/swiftui/`, a root `Package.swift`, narrowly affected `crates/cli/` and existing engine scope helpers if required, focused Swift/bridge/CLI tests, selected specs, SDK/skill/install documentation and release inventory. Preserve old package entry paths, wire formats and source-capture experiments.

The designated PlayPhrase.me chat owns its existing DeveloperControl integration, actual screen marker sites and scene hooks in `PlayphrasemeApp/`, `PlayphrasemePhone/`, `PlayphrasemeTablet/` and actual shared consumers in `PlayPhraseLearner/`; its Xcode package links/resolution, bounded build helper and diagnostic specs/docs/tests. It owns all real-app measurements, windows and UI acceptance. Blueprint implementation/regression work uses project-owned fixtures. No new product UI, networking feature or learner state owner is authorized by instrumentation.

Use one writer per overlapping file and existing owners. The user's named native counterpart received this plan and the direct Blueprint chat link on 2026-11-23; no implementation start or interruption of active experiments was dispatched. The dedicated release chat retains packaging/publication ownership. Independent review required by privacy, compatibility and concurrency is a real gate, not permission to create reviewers automatically.

## Decisions still requiring evidence, not another broad planning cycle

Prove file-event delivery in the existing sandbox; selected-root request cost under a large sibling tree; mappings across actual presentation roots; native scene routing; device container delivery; lowest claimed platform support. These have explicit experiments and fallback boundaries in the detailed plan. A failure narrows that capability, not the whole SDK. New AppKit/UIKit bridges, private APIs, product redesign or relaxed evidence semantics are outside this proposal.

## Current state

The user now explicitly directs a global solution and allows required app additions. The concrete acquisition method is being corrected under the latest request to investigate Xcode documentation; revision 1 is not blanket approval for its extra architecture. Next evidence is the owner-produced full Xcode hierarchy and its actual structured contents. No runtime results are claimed for either proposed acquisition method.
