# Acceptance, compatibility and release

Part of [PLAN.UIB.SWIFTUI-GEOMETRY@1](../swiftui-geometry-sdk.md). Checks below are planned, not executed results. Keep acceptance proportional to the changed boundary and reuse valid evidence.

## First usable slice: the stopping point for architectural experiments

P1 must deliver Random and Actors from the real instrumented Mac app through the installed command. Before adding the rest of the coverage, demonstrate: discover component → capture → inspect actual parts → measure size and gap → resize → capture again → close/reopen → refuse an old occurrence.

Use SDK anchors as the geometry source, with independent focused fixture expectations and real-app source/state attribution. Do not validate a copied expected rectangle against itself or use source constants as measured output. Compare the instrumented app's ordinary behavior with the same product revision without activation. No new LLDB qualification is a prerequisite.

The file transport gets one bounded real-app proof early: focus/active state, open popup, selection and playback unchanged by a capture; complete result on success; terminal refusal for partial request/timeout. Two failures of the same mechanism without outcome improvement require revisiting that mechanism. Do not expand into a daemon or network service by default.

## Required scenario matrix

| ID | Criterion |
| --- | --- |
| G01 | Catalogue identifies real scene, surface, component, occurrence and available parts. Duplicate labels/types do not cause guessing. |
| G02 | Required parent/child rectangles and component memberships are complete within the declared scope; correct modifier boundary is measured. |
| G03 | Resize/reflow updates source geometry; content-state changes and conditional parts have correct presence; no source constant is substituted. |
| G04 | Close/reopen, remount, row reuse and scene replacement invalidate old occurrences; stale capture/publication is rejected. |
| G05 | Selected small component works with a large sibling screen; source work/storage/output remain bounded and unrelated anchors are not resolved. |
| G06 | A separate presentation has its own root and owner relation; any claimed cross-root mapping is demonstrated at changed size/position/scale. Missing mapping stays a failed/pending mapping criterion. |
| G07 | One request cannot read another scene/run/device, escape its directory, overwrite another result or replay an old capability. |
| G08 | Cancel, busy, timeout, partial copy, malformed/oversized request, coalesced events, unsupported version and output failure terminate truthfully without partial successful publication. |
| G09 | Wrong/absent activation leaves normal Debug behavior unchanged; Release has no active host/ingress/collector or diagnostic IO. |
| G10 | Capture does not move focus/pointer, scroll, open/close UI, pause playback, mutate observable product state or request redraw to sample. |
| G11 | Public output/errors contain only approved structural metadata and geometry; canary credentials/query/model/media data never appear. |
| G12 | Original canonical bytes validate and existing Rust inspect/measure/check return correct values and expected unknowns; legacy saved records remain readable. |
| G13 | Mac coverage rows work in the real required states. A root frame or successful validation does not close missing inner parts. |
| G14 | Phone and landscape iPad use the same protocol, with correct safe area/keyboard/scroll context; distinct scenes and device IDs stay isolated. |
| G15 | A documented fresh invocation uses the installed package/skill and original app output without experimental scripts, manual JSON reconstruction or hidden source knowledge. |

Real screenshots/interaction observations support no-regression checks for affected app behavior. Internal geometry acceptance does not require ImageGen or pixel-identical rendering: this plan proposes no visual design changes. Any unintended visual change is a regression to repair; an intentional redesign requires its separate existing approval route.

## Checks by implementation layer

- Swift collector: focused identity/ancestry/scoped-selection/lifecycle/cancellation/cap tests; exact original Snapshot → canonical validator → Rust numeric checks. Include nested roots, repeated local IDs, large unrelated siblings and fractional coordinates.
- Capture host: deterministic admission/IO tests with temporary owned files; real Mac sandbox event delivery and complete-publication check; bounded request replay/retention and shutdown of listener. No test writes into user stores.
- CLI: selected component resolution, compact tree, version mismatch, scope ambiguity, original-byte validation, deadlines and exits; keep existing saved commands compatible. Use existing Rust command matrix only for affected packages/features.
- Native integration: structural source check, actual consumers' Debug/Release builds, focused configuration/route checks, then relevant real app scenarios. Do not run unrelated business suites solely for marker additions.
- Mobile: simulator compile and real flows on actual Phone/Tablet compositions; generic tvOS compilation only if shared changes affect it. Simulator proof is not physical-device proof or lowest-runtime proof.
- Packaging: root/nested SPM product agreement, exact tag resolution, recorded lockfile, bundled helper/skill inventory, installed command and historical installer update/rollback compatibility.

Record request-to-result latency and selected-scope size during the real runs, separately from device transfer and build time. Keep current request/deadline ceilings initially; report exhaustion rather than silently increasing them. No claim of low overhead follows merely from using SwiftUI preferences. Check idle activation performs no periodic capture and ordinary playback/input remains usable.

## Compatibility and scope changes

Preserve canonical core0.1/analysis0.2 meanings and the existing legacy Swift API. Represent new component relations/metadata using supported canonical structures where possible. If an existing structure cannot represent the required fact faithfully, register a versioned additive change and update affected readers before producing it; do not overload a field or silently patch saved files.

Version the host request/manifest protocol independently. Publish supported SDK/host/CLI capability ranges and capture provenance (app revision, SDK revision, build configuration, platform, run/scene/source state). Record source versions without exposing private paths/tokens. Build and SDK equality is verified rather than assumed from a nearby version string.

The historical immutable SwiftUI donor and its captures remain historical reference evidence. Newly instrumented captures get a new explicit app revision/configuration. Compare unchanged product behavior before using the new source as transfer evidence; never restamp the old baseline. Do not stop active owner processes or obsolete their experiments silently.

## Review and release

Review the final coherent SDK/host/API/IO boundary for privacy, compatibility and concurrency before broad distribution. The requirement for independent review does not authorize spawning workers; use an already authorized reviewer/owner or report that exact remaining review dependency. Routine local marker additions use focused self-checks plus runtime evidence.

The existing builds/releases owner handles packaging, managed installation, CI, source tag and GitHub release under the repository's standing source-release authority after the accepted revision is supplied. Native app publication remains outside this plan. A changed package inventory must retain support for prior manifests and rollback; adding a helper is not just copying a file into an installed bundle.

Deliver matched SDK source/tag, CLI/skill installation, app dependency revision and support matrix. Mac completion can be released while mobile remains explicitly a later milestone; never advertise mobile/device support solely from the package platform declaration. Fresh-use verification may be performed by the current executor from the installed recipe; do not call that independent/new-chat evidence.

## Handoff and completion

For each coverage row record only the current outcome, app/SDK/CLI pins, exact source observation/evidence path, scenarios checked, remaining unsupported fields and next owner. Link prior evidence instead of copying a running history. Retained evidence belongs in the authorized external task area, never in the installed package or repository as auxiliary logs.

The handoff must provide one reproducible path: build/launch the explicitly enabled app once, select its private session, list components, capture one component, inspect/measure, repeat after state changes. Preparation errors identify the missing capability/action; they do not send a new chat into arbitrary AX/debugger experimentation.

Complete P1/P2 only when the real Mac outcome exists. Complete mobile milestones only with their own actual evidence. The entire plan stays unfinished while a mandatory coverage or platform milestone is pending. Checkpoint task-owned changes in both current branches and retain accepted captures/builds with explicit ownership; remove only eligible owned scratch after its consumers finish.

## Execution-specific risks and disposition

| Risk | Required response |
| --- | --- |
| File watcher/copy races or sandbox refusal | Reproduce on owned diagnostic instance, correct completion/admission; keep existing URL compatibility, but do not call it quiet capture without proof. |
| Markers perturb view identity or lifecycle | Preserve existing composition/IDs, test activation on/off, repair marker placement; do not change expected product layout. |
| More than 256 nodes in a source scope | Partition at real boundaries and qualify needed source relationships; do not just raise the cap. |
| No public cross-host transform | Keep local sizes/relations, name the exact blocked mapping; no private API/native-view adapter or fabricated affine. |
| Missing real device or unsupported device container operation | Continue Mac/Simulator work, keep physical profile pending, identify exact device/tool prerequisite. |
| Existing writers modify shared files | Rebase the plan's implementation basis on current branch state and agree the actual file handoff; do not create a worktree or overwrite peer changes. |
