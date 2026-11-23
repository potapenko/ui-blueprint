# Fixed backend for immutable SwiftUI capture

- Node type: leaf; domain: `uib.swiftui-debug-capture.backend`.
- Contract: `UIB.SWIFTUI-DEBUG-BACKEND@1`; Authority: Active; Stability: Evolving.
- Authority: S13 execution plan approved directly in chat
  `01a20922-f6ad-7651-997b-cef1b3f09b12` on2026-11-23; unchanged donor and no
  executable-memory writes remain protected requirements.
- Requires: [debug capture](swiftui-debug-capture.md), [PRIVACY@1](privacy.md),
  [LIFECYCLE@1](lifecycle.md).
- Acceptance: owned-fixture data/code write distinction and capture passed;
  independent review/product qualification remain open.

## BACKEND.BIND

Optional caller-owned `--backend-profile FILE` has version1.0.0, one absolute
regular debugserver path, its SHA256 and policy `rw-data-only-v1`. The profile is
not UI-derived. Reject malformed/oversized/duplicate fields and symlinks. Verify
file content and the executing peer's path/device/inode before any target PID is
sent. Use a capture-owned SBListener and drain its state events during connection,
attachment, fixed target calls and retained recovery; do not leave its primary
queue unconsumed while a secondary observer is polled. Use a private inherited socketpair; no listener address, reconnect or peer
substitution. The existing literal consumer target admission remains separate.
A product attempt additionally requires the exact backend SHA pinned in source
for that admission; an arbitrary caller-supplied hash is not qualification.

The backend disables software breakpoints and rejects memory writes unless every
covered region is writable and non-executable. It never changes VM protections
to admit a write. Existing fixed metadata/geometry calls may use their private RW
scratch and restored registers; no JIT, injected code or executable writes occur.
The normalizer/geometry limits and caller authority do not expand.

## BACKEND.OWNERSHIP

Connect and verify the idle peer before RemoteAttach. That API starts asynchronous
attachment: await the matching stopped process, not merely an accepted request.
Preserve exact ownership on an uncertain attach or in-flight call. The existing
retained-debugger owner recovery path remains mandatory. No automatic target kill,
resume/retry, timeout teardown or extra detach is introduced. Only after confirmed
detach or proven pre-target failure may the owned peer be retired. Attach/OS latency
retains the original opaque-boundary qualification; this is not a new universal
whole-attempt wall-clock guarantee.

## BACKEND.QUALIFY

An isolated owned fixture must reject a code-memory write, preserve those bytes,
permit/read back/release its private data allocation, then produce canonical
geometry with normal call restoration and detach. Test wrong hash/path/peer,
pre-target setup failure and ambiguous/unfinished attachment handling separately.
Source publication does not distribute a prebuilt debugger. Packaging/source
provenance and any product admission require their separate accepted revision.
