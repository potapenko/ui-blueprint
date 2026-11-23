# C04 offline candidate: Genre disclosure and popup

Owner: chat `01a20a04-5ef7-7350-9876-bcd66392126f`.
Authority: direct user approval on 2026-11-23: implement the proposed candidate
and checks; do not change SwiftUI e19b20c47 or run a new capture. This is not
permission for product admission, installation or release.

Spec Basis: [PAIR@1](../../../specs/product/swiftui-capture-pair.md),
DEBUG-CAPTURE@18, TRANSFORM@7, NATIVE-OWNER@2 and their selected dependencies.
The approved plan is the preceding C04 chat plan; the user's approval is the
authority. The source declaration, actual runtime observation and candidate
mechanism remain separate below.

## Delivered candidate

- `plugins/macos/swiftui_source_selection.py`: single-generic-parameter nominal
  selection and unique structural part inside the selected owner forest.
- `plugins/macos/swiftui_capture_pair.py`: pair validation, shared traversal/node
  budgets, two-window revalidation and canonical paired output with sourced
  relative window coordinates. Attachment is explicitly unknown.
- Two focused test modules under `tests/bridges/swiftui_capture_pair*_check.py`.
- [Finite shared glue](C04-pair-glue.patch), prepared against
  `d98f1865fe3dd27fde1df1ae72f6e5ffd230d642`, for `swiftui_ag_cache.py`,
  `swiftui_debug_snapshot.py`, `swiftui_debug_capture.py` only.

S13 owner `01a20922-f6ad-7651-997b-cef1b3f09b12` retains those shared files.
On 2026-11-23 the owner requested this patch and a separate coherent integration
checkpoint after the current S13 path is settled. Shared repository files have
not been edited by C04. Helpers and the tested patch are a reviewable candidate;
the checked-out capture entry does not support the new request until integration.
Existing S13/Actors admissions are preserved by the patch. Every product pair
refuses before debugger startup, even if its first window matches an existing
single-window admission. Only the existing fixture namespace reaches that path.

## Source basis and candidate selection

On immutable e19b20c47, `ClipSearchFilterPickerView<PopoverContent>` contains
`disclosureButton`; `.popover(attachmentAnchor: .rect(.bounds), arrowEdge: .leading)`
is attached there. Genre supplies a `LazyVGrid`; entity filters supply a different
content shape. The disclosure contains `MaterialExpandMoreShape`. These are
source declarations, not measurements or proof of runtime type availability.

Candidate primary scope:

```json
{
  "type": "PlayphrasemeApp.ClipSearchFilterPickerView",
  "owner_argument_type": "SwiftUI.LazyVGrid",
  "part": {
    "type": "SwiftUI.Button",
    "containing_type": "PlayphrasemeApp.MaterialExpandMoreShape"
  }
}
```

`paired_surface` adds exactly an explicit `window_id` and `scope`. A popup
candidate is `{"type":"SwiftUI.VStack","containing_type":"SwiftUI.LazyVGrid"}`.
Neither selector is live-qualified. Missing or duplicate runtime occurrences
refuse. No title, AX rectangle, model value or traversal index resolves ambiguity.
No old PID/window identity is supplied as a current runnable request.

The metadata decoder is an original narrow implementation informed by Swift's
[Metadata.h](https://github.com/swiftlang/swift/blob/main/include/swift/ABI/Metadata.h),
[GenericContext.h](https://github.com/swiftlang/swift/blob/main/include/swift/ABI/GenericContext.h)
and [MetadataValues.h](https://github.com/swiftlang/swift/blob/main/include/swift/ABI/MetadataValues.h).
No upstream source code was copied. The existing exact runtime image guard and
new header checks apply; published ABI descriptions are not C04 runtime proof.

## Verification and reproduction

The combined candidate was assembled from d98f1865 plus the finite patch and
these helper/test files in task-owned external scratch. Run after integration:

```sh
python3 -B tests/bridges/swiftui_capture_pair_check.py
python3 -B tests/bridges/swiftui_capture_pair_integration_check.py
```

The integration suite deliberately fails on the unintegrated collector; it does
not skip missing functionality. Regression selection: cache, cache-transform,
native-parts, debug-backend and debug-capture checks, excluding exactly
`test_long_no_target_command_uses_private_source_file_then_removes_it` and
`test_live_no_target_launcher_cancellation_preserves_debugger`, which start LLDB.
All remaining capture lifecycle tests use a substituted debugger/process API.
No app process, attach, native fixture or image capture was run by C04.
Final assembled-candidate result:113 tests passed, including18 C04 checks and
95 affected regression checks; the two real LLDB launcher tests were excluded.
`git apply --check` passed for the finite patch at preparation; new/changed
local documentation links and whitespace checks passed separately.

Synthetic paired source was consumed by installed UI Blueprint0.6.1
at223cf16808ffa9962ecc28018a8d69e372992c94: canonical validation and Rust measure
passed; the independently specified cross-window edge gap was38pt. This is
format/arithmetic evidence, not the real Genre geometry. Invalid/ambiguous
selection, shared limits, unmapped/dirty geometry and failed cleanup are covered.

## Remaining for C04 acceptance

1. S13 owner integrates the shared glue on the current branch and repeats the
   focused checks at that exact revision. The eventual bundle inventory must
   include both new Python modules. Installation/release remains separate.
2. Independent review of metadata/privacy, shared accounting and lifecycle.
   C04 self-review is not independent review; no reviewers were spawned.
3. A separately authorized source-shaped runtime fixture qualifies actual
   generic metadata, disclosure boundaries, popup host and native coordinate path.
4. A presentation/anchor association reader is still missing. The candidate
   always reports `presentation-anchor-reader-not-qualified` and emits no
   `anchored_to` relation. This is unfinished source-acquisition work, not a
   claim that the relation exists and merely awaits a test.
5. Only after that qualification and new explicit capture authority: owner-run
   evidence on unchanged e19b20c47 in hello + Comedy, with fresh identities and
   actual layout for disclosure/content/header/options. AX and GPUI stay separate.

The prior saved AX source has no layout values and no attachment relation. It
cannot close items3–5 offline. No waiting/polling job is scheduled by this packet.
