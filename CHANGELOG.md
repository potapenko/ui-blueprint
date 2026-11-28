# Changelog

The version covers the complete UI Blueprint distribution: CLI, collectors,
helpers and agent skill. Versions of recorded data formats are independent.

## 0.15.0

- Add explicit `--label-style` for UIKit runtime captures, including Simulator
  pairing. Matching endpoints advertise `capture-uikit-label-style`; old endpoints
  refuse before dispatch. Omitted options preserve geometry-only behavior and
  do not read UILabel style. A matching Debug SDK rebuild is required.
- Attach allowlisted configured UILabel font names/family, point size in points,
  symbolic traits, optional numeric font weight, fit/highlight/enabled flags and
  numeric trait context to the original node and observation. Configured text and
  highlight colors are resolved against that label's traits into named extended
  sRGB RGBA; missing or unsupported colors remain unknown.
- This is configured metadata, not effective glyph/paint typography. Attributed
  overrides, fitting, fallback and rendering effects are not inspected; effective
  paint font/color always remain unknown. No text, attributed runs, accessibility
  values, models, full descriptor dumps or SF Symbol size/weight are collected.
- Preserve existing source identity, geometry, wire format, privacy and limits.
  Owned-fixture font/color, dynamic traits, refusal and compatibility checks pass;
  actual TV05 title acceptance remains with its application owner after rebuild.

## 0.14.1

- Qualify UIKit local-to-window model mappings when the complete layer ancestry
  provably preserves the z=0 plane, even if its 4×4 matrices contain an inert
  depth-perspective term. This corrects unknown mappings for planar focused
  descendants in the qualified tvOS Simulator profile.
- Require finite matrices, exact plane-preserving structural terms and zero
  layer depth/anchor depth. Preserve the existing all-affine path. Effective
  perspective, tilted/depth-displaced planes and unverified mappings remain
  unknown; no class-specific rule or epsilon relaxes those structural guards.
- Derive coefficients through the same public coordinate conversion and existing
  corner roundoff check. Original bounds, geometry, wire formats, privacy,
  limits, SwiftUI defaults and presentation/hit/visibility boundaries are unchanged.
- Owned fixture qualification covers 16 matrix cases, focused List descendants,
  fresh/stale/privacy behavior, canonical validation and exact Rust window-space
  measurement. Actual TV04 app replay remains with its owner. Previously saved
  unknown observations are not rewritten; use a fresh capture after SDK update.

## 0.14.0

- Add experimental tvOS 27.2 arm64 Simulator admission to the existing opt-in
  Debug SwiftUI and explicit UIKit sources. Root and nested SwiftPM packages
  declare a tvOS 17 compile floor; this is not older-runtime/device qualification.
- Report `tvos-simulator` metadata with fixed orientation 0. Simulator pairing
  accepts the explicit iOS/TV platform tags and preserves exact device/container,
  session/window binding, native lossless PNG hashes and sequential consistency.
  Existing Mac/iOS defaults, privacy, limits and Rust arithmetic stay unchanged.
- Qualify TV source geometry against a public GeometryProxy and native PNG oracle
  at 2× for one static translated/scaled hosting root. Actual ordinary,
  actions-after-hold, focus, fullscreen and return observations validated on one
  unchanged app; Rust player sizes were 1200×675 → 1920×1080 → 1200×675 pt.
  Independently addressed sessions passed wrong-device refusal and isolation.
- SDK Release compilation excludes runtime symbols. Full native app Release,
  Settings/child-list, capture during active hold, physical Apple TV and general
  focus-animation presentation bounds are not qualified by this release.
- Install the matching caller/skill and use the public SDK tag or a supplied
  immutable TV app. Each app owner retains its device, private session and data;
  geometry queries do not require rebuilding or a shared desktop-input queue.

## 0.13.0

- Add explicit `--source uikit` for runtime capture/tree/measure and Simulator
  source/PNG pairing. The new `uikit.runtime.layout` namespace exports the
  selected UIWindow's complete UIView hierarchy and original local bounds.
  Default requests remain SwiftUI, with no implicit source fallback.
- Qualify 2D local-to-window mappings from public point conversion, preserving
  original bounds and coefficients. Perspective/3D or unverified mappings stay
  unknown. The corner-agreement guard checks floating-point roundoff only;
  it does not change geometry or Rust rule tolerances.
- Report UIKit model/layout geometry, not presentation frames, visible/hit regions,
  clipping or occlusion. UIKit window spaces are distinct from SwiftUI spaces.
  No UI text, accessibility values, model descriptions or raw pointers are exported.
- Require the matching Debug SDK and caller. Older endpoints lacking
  `capture-uikit` refuse before collection; updating CLI alone is insufficient.
  Rebuild the owned app once, then reuse it for subsequent queries.
- Qualified support is iOS 27.2 arm64 Simulator. Owned Menu/Picker, affine/scroll,
  privacy and compatibility checks passed; original Sorting/VideoSize acceptance
  remains with Search and Services. A popup may be a subtree of the main window,
  not a separate window. Phone design, Mac behavior and existing limits remain.
  Physical devices, iPad and universal system-surface coverage are not claimed.

## 0.12.0

- Add experimental iOS Simulator support to the opt-in Debug SwiftUI runtime
  product, with the standalone `swiftui_simulator.py` launcher and source/PNG
  pairing caller alongside the existing runtime query caller.
- Bind launch to the explicitly owned Simulator, exact app container and returned
  PID. Pair canonical geometry with a native lossless Simulator PNG, preserving
  window metadata and hashes and declaring sequential, non-atomic consistency.
  SwiftUI-host-to-screen mapping still requires its qualified source profile.
- Include the portable Simulator setup/query guide in the matching eleven-file
  skill. Local bundle format 9 preserves exact formats 1–8 and verified rollback.
- Qualified local evidence uses iOS 27.2 arm64 Simulator on Apple Silicon:
  Search/More/close at 390×844 and 440×956 pt with 3× PNGs, Reels, Clip grid/
  Filters/viewer and the shared Search Download form. Clip Download's unavailable
  state is not qualification of a ready form. Debug/Release build and runtime
  exclusion were checked by the SDK/app owner; no full Phone matrix is claimed.
- The SwiftPM iOS 17 declaration is a compile floor, not runtime qualification.
  Physical devices, iPad, other runtimes and Mac-hosted Phone layouts remain
  unqualified. Existing Mac support and Rust geometry semantics are preserved.
- Apply the existing type-name byte guard to the exported nominal name after
  removing generic specialization, retaining control-character rejection and
  keeping generic arguments/model values out of the result.

## 0.11.0

- Support complete large Mac runtime observations with up to 1,048,576 source
  nodes/native views and 1 GiB canonical or discovery artifacts. Matching SDK,
  caller and Rust input budgets agree; source completeness, unique selection,
  privacy filtering, depth/type/metadata limits and cancellation remain protected.
- Resolve the previous large full-window output and filled Word/Next source-limit
  cases on the qualified macOS 27.0.1 arm64 profile. Actual full Search, resize,
  Practice, post-close and Word/Next observations validated and measured in Rust.
  The final single live query exported 36,517 nodes / 87,799,692 bytes and measured
  dialog width 576 pt in 5.6679 s, using explicit `--timeout 120`, without subtree
  fallback, application rebuild, markers or retry.
- Use one absolute deadline across preflight, capture and Rust measurement.
  Default timeout is 30 s; explicit values up to 120 s are supported. The heavy
  combined case above does not establish default-30 or universal latency bounds.
- Add opt-in `measure --json --result-only` to avoid returning the entire source
  Snapshot again while preserving identity, observations, coverage and result
  evidence. This CLI envelope is not a canonical Snapshot or standalone replay
  artifact; retain the original input. Existing `--json` output is unchanged.
- Validate source graphs with one iterative traversal, preserving all cycle,
  dangling-reference, duplicate, delta and other semantic checks. On the same
  accepted Word input, validation and scalar measurements became substantially
  faster with exactly equal measurement responses.
- This remains an opt-in Mac Debug source profile. Mobile support is deferred.
  The 1 GiB ceiling is not a peak-memory guarantee, and the opaque framework
  getter is not preemptible; preserve owner recovery and avoid automatic retries
  after an unknown outcome. Rebuild the app with the matching SDK.

## 0.10.1

- Explain SwiftUI runtime `sourceLimit` refusals with optional fixed diagnostic
  fields `source_limit.kind`, `limit` and `observed`. Only the guard that actually
  refused is reported, without source types, UI/model values, paths or raw errors.
- Preserve protocol 1.0.0, `reason=sourceLimit`, exit 4, all limits and refusal
  without a Snapshot or discovery artifact. Older callers retain the prior
  refusal; matching callers also accept old endpoints without diagnostic detail.
- Observed node counts can be a lower bound, not the total rejected tree size.
  The owned fixture's 16,384/16,385 boundary does not diagnose Word/Next.
  Supporting an oversized source scope and fixing Word/Next are not claimed.
- Update the matching skill with the consumer repeat and sanitized reporting
  procedure. Rebuild the opted-in app with the matching SDK; request only
  status/reason/source_limit from the same Word/Next state before attributing
  the actual cause. Existing successful geometry and marker acceptance stay separate.

## 0.10.0

- Add opt-in `discover` to the experimental SwiftUI runtime SDK and caller.
  It exports a bounded version 1.0.0 source inventory with observed nominal type
  counts and compact topology, bound to the selected run/window/observation.
  It contains no geometry, model values or UI text and is not a canonical Snapshot.
- Use actual discovered types and hierarchy to attribute a candidate for a later
  fresh `capture --subtree-type` request. Uniqueness is revalidated at capture;
  a symbol or Swift View name alone does not prove a runtime node exists.
  No ActorPickerView selector is confirmed by this release.
- Preserve full and scoped capture, source/output limits and privacy filtering.
  Older endpoints reject discovery before dispatch. Rebuild the opted-in Debug
  app with the matching SDK; updating the caller alone does not add the capability.
- Large whole-window geometry `outputLimit` and Vocabulary `sourceLimit` remain
  open. Owned-fixture discovery and compatibility checks do not establish Actors
  or other consumer interface acceptance.

## 0.9.0

- Add `--subtree-type` to the experimental SwiftUI runtime caller. The matching
  SDK selects one exact nominal source type before canonical serialization,
  retaining its descendants and attributed ancestor context within existing
  output, node and depth limits. Missing and duplicate matches refuse explicitly.
- Preserve default whole-window capture and the existing post-capture `--type`
  filter. New callers reject scoped requests to older endpoints before dispatch;
  rebuild the opted-in Debug app with the matching SDK to use component export.
- Large whole-window `outputLimit` remains open. Custom View names may not appear
  in the source tree, and common types can be ambiguous. No usable source root
  for MacPracticeRoot or RandomCompletionDialog has been demonstrated; choose a
  type from actual source evidence, not an assumed screen or dialog name.
- Owned-fixture qualification covers bounded component export, fresh changes,
  concurrent observations, refusal paths and legacy caller behavior. This does
  not establish consumer UI acceptance or general whole-window/mobile support.
- Add regression coverage for exact native capture launch-identity round trips.
  No production capture correction is claimed for the earlier transient refusal.

## 0.8.0

- Publish an experimental, opt-in Mac Debug SwiftUI runtime library and matching
  `swiftui_runtime.py` caller. Prepare and launch an owned integrated app, list
  windows and request fresh canonical geometry without Xcode GUI or LLDB attach.
  The root Swift package exposes both the marker and runtime library products.
- Include portable agent setup and runtime-query guides with the matching skill.
  Local bundle format 8 preserves verification and rollback of formats 1–7.
- Preserve source-local geometry and attributed inherited layout; qualified 2D
  affine facts remain scoped to each hosting tree. Cross-host/window/screen mapping
  is not inferred. Rotated/sheared rectangle relations can return
  `unsupported_shape`; perspective/3D mapping remains unknown.
- This is an experimental macOS 27.0.1 source profile, not full application,
  mobile or platform-matrix acceptance. The scoped predecessor captured 6,286
  real-app nodes; a later resize attempt timed out. The current normalization
  has owned-fixture evidence; complete real-app requalification remains pending.
  A timeout does not prove the framework call stopped: preserve owner recovery
  and do not automatically retry or terminate the app.

## 0.7.0

- Fix bounded SwiftUI owner association discovery using a metadata index and
  audited read intervals. The source-shaped stress fixture now retains the same
  geometry as the small fixture within the existing acquisition limits.
  Existing exact owner-run qualification and recovery gates remain mandatory;
  Random, Actors and Practice interface acceptance belongs to their consumers.
- Add the source-specific Guest Tour component diagnostic under
  `experiments/native/guest-tour-layout`, with canonical validation and Rust
  measurements, preserved connector origins and annotation scale. Use requires
  the separately verified immutable G2 profile and restricted backend. This is
  not a general SwiftUI capture path or acceptance of the full Guest Tour UI.
- Include the four required SwiftUI collector support modules in local bundle
  format 7, preserving verification, update and rollback of formats 1–6.
  No debugger binary or process authorization is distributed.
- Clarify the installed skill's separate AX discovery path for partially
  measured forms, including source identifiers and repeated Close controls.

## 0.6.1

- Fix macOS window capture on an external volume. `capture-window` can now
  save its PNG and metadata in a new capture directory under the caller’s
  approved temporary directory on that volume. Existing captures are protected
  against overwrite.

## 0.6.0

- Add `Session::mark_or_render` to the opt-in GPUI source SDK. An element
  whose native ID was erased stays interactive and reports an explicit
  unmeasured marker result. Strict `mark` and required-anchor checks remain
  available. Update the application's vendored SDK to use the new method.
  The source export includes the integration instructions from the same commit.
- Record observation-scoped branch provenance for successful direct SwiftUI
  subtree selection. Each selected branch identifies its emitted roots or an
  empty contribution. Earlier captures remain readable and product capture
  permissions are unchanged.
- Extend the source-only owner Accessibility examples and their qualification
  for Favorites and the public Statistics control. Document reproduction on a
  new Mac and include the Grammar hint and window-binding fixtures. These
  examples report geometry only for their verified public controls.

## 0.5.1

- Fix the native window helper's deadline handlers for window listing and PNG
  capture. When a watchdog expires, the helper reports `capture_deadline` and
  exits with code 4. Existing saved snapshots use the same formats and commands.

## 0.5.0

- Add fixture-only selection of a native view by its SwiftUI owner when several
  controls have the same native class. Ambiguous owners or bindings refuse.
  A cursor region describes that native view alone.
- Teams can validate, inspect and measure saved SwiftUI snapshots with the
  installed CLI. The previous Reels qualification covered one paused card and
  its one-shot admission has been consumed. Live product acquisition requires
  a separately qualified request and an exact handoff from the process owner.
- Include a copyable installation prompt pinned to the release tag in generated
  release notes.

## 0.4.0

- Include the macOS SwiftUI cached-layout reader and its dependencies in native
  packages. A qualified owner-run on the paused Reels card returned 38 nodes,
  including a title and the measured video frame. The action-bar boundary was
  identified but its own bounds were unavailable.
- Keep scoped acquisition tied to the approved consumer, process generation,
  window and source types. Ordinary installation grants no debugging session.
- Read and measure the resulting canonical snapshots with the existing CLI.
  Local node coordinates require separate evidence before mapping to an image.

## 0.3.2

- Accept the current manifest5 native/combined distribution in `native-prepare.py`,
  retaining manifest4 compatibility and all file hash/mode and process guards.
- Exercise the shipped preparer during actual candidate verification so a bundle
  inventory update cannot silently break prepare before observing a real window.

## 0.3.1

- Resolve the installed CLI symlink before locating the matching native helper,
  so `capture-window` works through the normal `~/.local/bin/uiblueprint` entry.

## 0.3.0

- Add `capture-window` for an exact already-running macOS process/window: independent
  lossless PNG, source dimensions/scale, no-shadow convention and attributable
  mapping metadata without application instrumentation changes.
- Add scoped `--list-windows` numeric metadata for the selected PID and a matching
  installed skill recipe. Existing SDK Snapshots and geometry commands are unchanged.
- Local candidate only until separately authorized publication; older installed
  versions remain available for verified rollback.

## 0.2.0

Window-root geometry and matching agent installation. Install the source release
through the [tag-pinned guide](https://github.com/potapenko/ui-blueprint/blob/v0.2.0/docs/install.md).
No binary attachments; local preparation does not imply publication.

- Add opt-in SwiftUI and GPUI Debug producers for real runtime layout, using the
  existing canonical Snapshot inspection and Rust geometry analysis.
- Add root collection of mounted marked descendants with required-anchor checks,
  preserving explicit component selection and source identity/coverage limits.
- Route the agent skill from the selected window/root by default and include the
  complete SwiftUI/GPUI operating references in the installed matching package.
- Preserve update and rollback for historical installations while verifying the
  complete new skill inventory and its portable links.
- Add shared Rust lint policy and pull-request checks for supported CLI features.
- Pin source builds and the optional GPUI probe to stable Rust1.100.0, released
  2026-11-12. Existing dependency versions and data formats are unchanged.

Internal geometry requires explicit integration in the target application's Debug
build. It covers measured marked boundaries, not every private framework view;
layout, Accessibility, hit, paint and screen/image coordinates remain distinct.
Initial local build/runtime qualification remains Apple Silicon macOS27.0.1.

## 0.1.0

UI Blueprint source release under MIT. Install through the agent prompt in the
[README](https://github.com/potapenko/ui-blueprint/tree/v0.1.0#install-through-your-agent).
The [tag-pinned instructions](https://github.com/potapenko/ui-blueprint/blob/v0.1.0/docs/install.md)
build the utility locally and install its matching skill. No binary attachments.

- Local Rust inspection, geometry measurements, requirement checks and comparison
  of saved UI observations.
- Web and macOS observation, with explicit target selection, source attribution
  and partial/unknown coverage preserved.
- Opt-in Kotlin Compose geometry collection for Desktop and Android debug apps.
- Engineering blueprint packages and ImageGen prompts from observed geometry.
- Matching CLI and portable agent skill, with verified installation and rollback.
- Browser-session CDP support for Chrome's permission-based debugging, including
  the rooted-selection compatibility fix verified on Chrome154.
- One numeric product version, `uiblueprint --version`, and manual GitHub release
  verification from a pinned commit.
- Prompt-based installation builds the utility locally from the release tag and
  installs its matching skill. GitHub releases contain no binary attachments.

Local source builds are initially qualified on Apple Silicon macOS27.0.1, with
Git, Python3.11+, Rust1.96.0 and Xcode/Swift6. Node24+ is optional for Web observation.
Intel, Linux, Windows and older macOS support are not qualified.
