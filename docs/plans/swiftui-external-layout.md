# External SwiftUI layout qualification

Status: c89b277 WITHDRAWN; reviewed method, one exact Settings qualification pending.
No shipped capability or version change. Existing installed0.3.2 is unchanged.
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

Registry76 → [SwiftUI probe@3](../specs/product/swiftui-probe.md),
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
This initial fixture experiment did not operate a consumer. A later explicitly
leased read-only Reels diagnostic occurred after its failed consumer capture;
it did not establish safe detach or post-debugger-exit lifetime.

## Current coherent fixture-only candidate

Human reply in this support chat on2026-11-16 authorizes a separate unchanged
product copy with process-local `SWIFTUI_VIEW_DEBUG=29` and brief addressed LLDB.
General product retries remain withdrawn; the single Settings admission below
uses closed method review and a fresh exact consumer ownership handoff. No donor code/assets/config files, JIT,
injected libraries or executable writes are authorized.

The original serializer path failed on real consumers: both a4 captures exhausted
256calls; c89 outer-host pruning then failed inside the Reels getter. The read-only
Reels diagnostic did not establish safe detach/lifetime. Clip independently hung
before command execution;513-byte inline startup input reproduces the LLDB pipe
failure without a target. Both owners closed those failures; no product PID is
leased to this chat now. Existing installed0.3.2 remains unchanged.

Current package restores DEBUG-CAPTURE.DATA/PRIVACY/LIFECYCLE under contract@5,
registry78. Task-owned writes: capture/field-reader/snapshot/recovery Python
modules, their three focused checks, this state, candidate contract, registry and
candidate recipe. Existing SDK/AX/GPUI paths and unrelated work remain protected.
The exact-image/target-ABI checks are implementation of existing privacy/lifecycle
requirements; source feasibility does not create broader product authority.

- Command-file LLDB startup avoids the observed inline-input deadlock.
- Calls require the distinct executable entry return site, exact SP and BP/thread.
- Field-only collection pins SwiftUI/SwiftUICore/libswiftCore UUIDs, target type
  metadata, value witnesses and field offsets; native storage identity precedes
  headers, exact tail ranges precede payloads, Any metadata precedes geometry.
  Model/value slots and full serialization are excluded. Types are nominal short
  names; generic arguments remain unreported. Canonical coordinates stay separate
  source-local spaces with unknown transforms, no B0/window/PNG claim.
- Getter return ownership is recorded before register restoration. Every completed
  decoder refusal releases the array; release/pool share2s and2reserved calls.
- Interrupted calls retain LLDB. Malformed/partial/foreign commands keep holding;
  one explicit command binds PID/generation/random owner and can terminate only
  the owned fixture. External exit and failed-command exit have distinct outcomes.
  Forced debugger/host loss is not safe recovery; no product termination enabled.

Runtime evidence in the retained system-temp root below:
`integrated-run-path.txt`, `integrated-refusal-recapture.json`,
`integrated-final-path.txt`, `lifetime-integrated-summary.json`.
Final allowlist instrumentation:167model slots,0slot reads,77068read bytes,
canary absent from reads/output. Actual167-node capture uses45calls; max_nodes1 refuses with37calls and restored
detach, then same-generation recapture succeeds. Canonical validator/inspect pass;
Rust measure returns exact Color width1888pt. Public metadata after debugger exit
confirms the original fixture generation33990:1794853157.923368 remains alive.
Controlled infinite-call fixture stays stopped with LLDB alive through partial,
null, foreign-generation and foreign-owner commands. Explicit matching termination
then produces target exit-9/debugger exit0 and target absence after debugger exit.
This is fixture evidence, not product qualification.

Affected review of d65fe4f found two remaining cancellation escapes: Python
subprocess.run kills its child on KeyboardInterrupt, and recovery setup/backoff
could throw outside the protected loop. Corrections use explicit isolated Popen
ownership, parent cancellation handlers and a protected wait/setup/backoff loop.
`lifetime-cancellation-summary.json` records a real held fixture surviving double
SIGINT/SIGTERM/SIGHUP to its launcher, then the existing invalid-command sequence
and exact owner termination. No product process participated.

Final review confirmed only those2P1 findings; ABI/storage/Any and owned-array
cleanup were closed for the exact image profile. The correction passes28focused
checks in embedded Python3.9, including repeated wait/SetAsync/backoff interrupts,
unrestorable native-handler preflight and real no-target launcher cancellation.
The new held-fixture harness traverses run_lldb; the older lifetime harness did not.

SWIFTUI-METHODOLOGY@3 final turn01a1e699-d456-7f80-8637-6cd8ebcfacf7 closes
both cancellation P1 on efd27a9, no new findings; author personally read verdict.
One bounded consumer qualification is now allowed by the existing human diagnostic
authority, with explicit code admission rather than a consumer bypass.
Services owner@11 handed off fresh45908 / me.playphraseme.reference.services.fgb7o3fd /
exact native birth1794870752.9841919 / window22500, English Settings open,
menus closed,1/39/Common17/Ready-paused. Public native helper independently agrees.
Old72842 is absent and is not reused. Owner's unchanged-copy/env29/signature receipt:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/playphraseme-services-debug-source-11-kp12lrk3/run-identity.json`.
Request ID is services-settings-qualification-1. Other consumer identities remain
closed. Product hold cannot parse termination commands or call Kill; receipt command
is null and a separate owner recovery decision is required. Capture is owner-run,
one attempt, initial refusal→stop, no retry/quota increase/blind SIGCONT.

Next: affected check of this narrow admission/no-product-kill delta, frozen5-module
candidate handoff to Services root, actual Settings outcome, then compatible local
delivery only if qualification succeeds. Reels/Clip/Library await that outcome.
No new agents, donor modifications, product termination or production release.

Fixture, driver and original observations are in system temp:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-external-layout-d6jl1rro/`.
This chat owns them through the next qualification/handoff; images and the
explicitly retained fixture app are not deleted. Non-image scratch retires only
after its named consumer accepts the result. No run logs are stored in Git.
