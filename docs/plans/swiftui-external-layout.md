# External SwiftUI layout qualification

Status: active finite qualification; no shipped capability or version change.
Owner: support chat `01a1e565-14f6-7be0-aa02-cd648a1b0423`, current `master`.

## Authority and scope

The user assigned this chat UI Blueprint support for the PlayPhrase.me SwiftUI
to GPUI transfer. The direct Clip human turn `01a1e568-591d-7532-ba47-af6943b37d37`
was read: qualify/fix the external tool without changing the donor. Main's human
message `01a1e569-1b3b-75f1-b92b-68cd223eb149` also authorizes direct support
communication. All seven coordinators supplied concrete cases.

The requested result is actual runtime layout, hierarchy and source attribution,
then a compatible locally installed tool and consumer verification. AX bounds,
pixels, static declarations and renamed source records cannot substitute for it.
Product donor code/assets/configuration files, SDK markers, injected libraries/JIT and
runtime code patches remain prohibited. Consumer processes stay with their owners.
Production/public release needs the existing separate exact confirmation.

## Basis and distinct gaps

Registry75 → [SwiftUI probe@3](../specs/product/swiftui-probe.md),
[Native@5](../specs/product/native.md), [Boundaries@1](../specs/product/boundaries.md),
Model1 / Geometry2 / Identity1 / Privacy1 / Exchange2 / Projections1 / Lifecycle1 /
CLI21 establish existing behavior. Consumer `visual-transfer@4`, especially
VISUAL.B0/MEASURE/ACCEPTANCE, supplies the immutable-source acceptance boundary.
The source getter is observed implementation, not new product authority.
Private external collection would be an additive Evolve contract registered
before shipping implementation; the existing opt-in SDK contract is not changed.

- Shared SwiftUI gap: Clip, Reels, Settings, Favorites/Statistics, Grammar,
  Common/Vocabulary internals and compact Search suggestions are unmarked.
- Separate AppKit gap: actual NSButton toolbar layout in the GPUI client.
- Separate text gap: shaped glyph/caret/selection and hit/paint channels. A layout
  tree does not automatically provide these.
- Separate viewport request: exact native size and sourced content→PNG mapping;
  changing viewport is an explicit action, not an implicit capture side effect.

## Observed feasibility

On macOS27.0.1, Xcode27.2 / Swift6.4.2 / LLDB2103, a task-owned unmarked SwiftUI
fixture exposes the existing Objective-C `makeViewDebugData` getter. A temporary
host Python/LLDB driver calls existing functions through registers and hardware
breakpoints; it evaluates no expression, writes no executable memory and loads
no helper into the target. Successful calls restore68 saved registers and preserve
the checked return instructions/red zone. These checks are not a complete safety
proof for arbitrary application threads or newer architectural register state.

Ordinary launch returns `[]`. The installed SwiftUICore initializer reads
`SWIFTUI_VIEW_DEBUG`; its default properties word is zero. Relaunching the same
fixture binary with `SWIFTUI_VIEW_DEBUG=29` yields179 hierarchy nodes,25 with
position/size. After ordinary rail toggle:173 nodes/23 frames; neighboring width
520→608 and x104→16. Native Zoom changes root640×400→1920×1050 and the remaining
color region width608→1888. Original native PNGs show the corresponding states.
These are synthetic feasibility observations, not PlayPhrase.me B0 acceptance.

The first enabled-getter attempt failed with EXC_BAD_ACCESS during a stack write
using a separate256KiB stack. The driver adopted [LLDB's native-stack-minus-red-zone
method](https://github.com/swiftlang/llvm-project/blob/fb5f84d903a3b4e7aea15cd5666598a46f56c833/lldb/source/Target/ThreadPlanCallFunction.cpp#L54);
the successful getter/state/resize runs followed. Earlier failures are not passes.
No consumer application was attached, modified, restarted or captured here.

## Current candidate and next finite package

1. Direct user reply in this support chat on2026-11-16 authorizes the separate
   byte-identical product copy with `SWIFTUI_VIEW_DEBUG=29` and brief addressed
   LLDB layout capture. This does not authorize donor file/code changes.
2. Geometry-only extraction is implemented in `plugins/macos/swiftui_debug_snapshot.py`
   and the addressed `swiftui_debug_capture.py` candidate. The system serializer
   includes reflected property1/model values even with mask29. Never save/log its
   raw response from a product app. Scope, allocation/byte/node/depth budgets,
   cleanup and timeout effects have an explicit candidate contract; review is pending.
3. Resolve source identity and coordinate semantics. The returned tree has no
   stable semantic IDs; keys must be observation-scoped unless stronger evidence
   exists. Serialized transform items are incomplete. Do not infer arbitrary
   transforms or publish all positions in a universal window/image space.
4. [DEBUG-CAPTURE@2](../specs/product/swiftui-debug-capture.md) is registered.
   Ten focused checks and actual canary/output-refusal→recapture passed. Reels'
   non-author reviewer found call-budget, nonfinite and deadline defects in e6ecb1b;
   corrections and fresh final-shape canary/refusal evidence await affected re-review.
5. Qualify one real Clip root/rail/grid/query slice through its native owner, with
   state/resize and original PNG pairing. Then expand to the other named cases.
   Add the proven capability to a coherent local minor release/skill/install;
   only after this can consumers receive a supported command and exact pins.

The earlier temporary driver has destructive fixture cleanup and must never be
used on a consumer. The new candidate has no kill/restart path; uncertain in-flight
failure reports stopped/owner_recovery_required. Its [recipe](../development/swiftui-debug-capture.md)
states the exact interface and residuals. No new installed version, independent
review completion, complete transforms or glyph/paint support is claimed.

Fixture, driver and original observations are in system temp:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-external-layout-d6jl1rro/`.
This chat owns them through the next qualification/handoff; images and the
explicitly retained fixture app are not deleted. Non-image scratch retires only
after its named consumer accepts the result. No run logs are stored in Git.
