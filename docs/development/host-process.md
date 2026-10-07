# H01 Darwin process owner

Packet [H01-process](../plans/ui-blueprint/packets/H01-process.md), frozen
process API/foundation `3abd8e5f9c054c12e910d4c8defc2f8a64a8b268`.
Real Rust/POSIX process/FD/stack/fatal implementation, no UI/Swift/capture/real apps.
No B capture permission operation, hostile JSON or production host acceptance.

## Basis and source boundary

AGENTS → registry10 → D02@2 all clauses, D05@3, D05-MEMORY@2, D05-WORK@1,
D07@5/H01-PROCESS-001, D01/D06@1 and RUST/DEV.RUST@2 with full applicable explicit
product/evidence closure. Current unchanged leaves reused; new contracts read
fully. D03@2 analysis closure is conditional on analysis serialization; this OS
owner neither parses nor serializes it. Fixed API and profile remain Core-owned.

DarwinPlatform implements ProcessPlatform and WorkerPlatform; DarwinChild owns
only the PID/FDs acquired by its successful exact-path posix_spawn. Core supplied
the cfg(macos) lib hook and test-only process_peer example, then froze the connected
foundation/publication/quota slice. Native did not edit lib/API/Cargo. Checks
excluded Core's unconnected worker/domain/allocator files and other WIP targets.

Public libc0.2.190 bindings were inspected for spawn/file actions/flags, socketpair,
fcntl/poll/read/send/close, wait/kill, rlimits, pthread queries and _exit. MSG_NOSIGNAL
is the public per-send alternative to a broad SIGPIPE handler. Installed Apple
SDK27 headers/local manuals and Apple's [file-action documentation](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/posix_spawn_file_actions_addclose.3.html)
provide OS context. Core's accepted archive/license audit7b1489f is reused. No ABI
layout/binding declarations or upstream source copied; no dependency added.

## FD and process invariants

All acquired socket/null sources move above FD5 before ordered file actions.
This prevents source/target collisions even if the caller's original0..5 were
closed. POSIX_SPAWN_CLOEXEC_DEFAULT excludes unrelated descriptors, including an
inheritable FD in the test. Sources are explicitly closed in the child after dup2.
Parent input/output/fatal lanes are nonblocking/CLOEXEC. Child3/4/5 are input/output/
fatal; child0/1/2 are verified /dev/null devices. Fatal5 is nonblocking.

No shell, extra argv or inherited environment is collected. SpawnSpec owns its
bounded absolute C string and stays borrowed through the synchronous C call.
OwnedFd and initialized file-action/attribute guards clean only this attempt's
resources after every setup failure. C action/attribute internals and OS buffers
are opaque allocations, explicitly outside Rust root accounting.

Read/write return Bytes/WouldBlock/Closed without hidden loops; empty slices are
not EOF. Per-send MSG_DONTWAIT|MSG_NOSIGNAL prevents blocking/global signal changes.
The fixed36-entry poll buffer rejects larger slices/overflowing wait conversion
before processing them. EINTR returns cleared readiness immediately; caller recomputes
its deadline. HUP can remain readable for draining buffered output.

The PID comes only from spawn and is never exposed as an input capability. Only
this owner may reap its child: Darwin offers no atomic pidfd kill if unrelated
same-process native code violates exclusive wait ownership. Cached actual exit/
signal results are stable; ECHILD marks ownership lost and returns CleanupPending.
No subsequent signal is sent in that state. terminate sends SIGKILL only while
unreaped, then still requires try_reap. It never means cleanup succeeded.

Drop is nonblocking best-effort, not a cleanup acknowledgement. Core must retain
its child/grant and quarantine until explicit reap; it cannot release reservations
because Drop ran or telemetry said exited. No background reaper/global PID list is
created by this layer. Parent-supervisor quarantine/destructor proof remains Core's.

## Stack and fatal behavior; concrete Core handoff

setup_main runs only on the calling main thread, disables and verifies RLIMIT_CORE
soft/hard0, sets/reads requested RLIMIT_STACK soft/hard, queries the actual pthread
extent and refuses an extent above the request. current_stack_bytes queries the
actual calling thread, not a cached/requested size. No operator shell limit changes.

Measured on Rust1.96/macOS27.0.1 arm64, page size16384:

- Main requested8MiB, actual8372224 bytes (below8388608).
- Initial std::thread::Builder request1048576 produced actual1060864, above the
  1MiB ceiling. This failed the first stack assertion; it is not hidden or accepted.
- Owned peer creation request1032192 (1MiB minus one host page) produced actual
  1044480 bytes, below the unchanged1048576 ceiling. The actual watchdog thread
  reports this size and rejects setup_main from that non-main thread.

Rust1.96's [public thread creation source](https://github.com/rust-lang/rust/blob/1.96.0/library/std/src/sys/thread/unix.rs)
passes the requested size to pthread attributes; OS padding means Builder input is
not proof of the actual extent. The public pthread query remains the proof. This
one-page headroom is observed on this host, not a universal unverified bound.

**Real Core watchdog is not repaired by changing a test peer.** Core owns its
creation. Root approved the following narrow addition; Core owns the signature/doc and
stable build handoff:
`WorkerPlatform::watchdog_stack_request(ceiling: usize) -> Result<usize, HostError>`.
The verified Darwin method uses public sysconf(_SC_PAGESIZE), checked subtraction and
page rounding/minimum-size validation to provide a creation request; the Core worker must pass
it to Builder, then query actual size inside the created thread and refuse readiness
if actual exceeds the original ceiling. Source owners: Core process_api.rs/worker
creation; Native process/worker.rs implementation. Native did not edit the shared signature. Core supplied API hash
`da5e278829b2b6b0d0d62c62b208cf838c8076bea117ee7b806fe49949878b56`;
provider/peer linkage, invalid/unrepresentable ceilings, conservative alignment
and actual watchdog query passed on that exact stable signature.
Alternatively Core may supply an explicitly qualified smaller creation request,
but must still verify actual extent; it cannot relabel Builder(1MiB) as cap proof.

fatal_exit performs one fixed64-byte best-effort send with DONTWAIT/NOSIGNAL, then
_exit. Negative status FD skips writing. No Rust allocation, formatting, panic,
unwind or retry exists in that path. Full status buffer and unavailable status
both still terminate; missing status is generic failure, not quota evidence.
Core's executable allocation guard and integrated fatal classification stay separate.

## Actual checks and fixed inventory

On the stable shared slice: scoped check/lib+process test+process_peer example,
peer build, Clippy -D warnings, own rustfmt and nine process tests passed. The peer
startup change affected every process scenario; the final nine-test rerun covers it.
Only the stack test was repeated for exact inventory reporting. After the agreed
trait addition, scoped check/peer build/Clippy plus two affected tests (new ceiling
validation and actual stack/mapping) passed. The other eight unchanged process
results are reused explicitly; no workspace suite or false full new-epoch run claim.

Tests cover mapping/env/flags, low-FD collisions in an owned nested peer, unrelated
FD exclusion, actual partial setup failure under child-only RLIMIT_NOFILE, failed
spawn cleanup, nonblocking/backpressure/closed IO, default-SIGPIPE survival in owned
peer,36-entry/relative/EINTR poll, independent children, repeated terminate/reap,
externally reaped ownership loss, actual core/main/watchdog checks and fatal/full/
missing status. No task-owned peer remained after the run.

Measured Rust inline owners: DarwinPlatform0, DarwinChild20, SpawnSpec4104 bytes;
fixed poll backing288 bytes (36×pollfd). Spawn's argv16/env8 bytes and two opaque
C-handle roots16 bytes are bounded local stack objects; no Vec/String/graph in the
production OS owner. Root SpawnSpec is4096 path bytes plus length/alignment, not a
fictional4096-byte total. Core must count its arrays/slots/borrows and grant owners
around these values. No total host memory, allocator or D06 claim follows.

All38 selected shared-input files were hashed before and after both check phases.
Each phase was unchanged; the only authorized inter-phase change was Core's new
process_api.rs signature/doc. Production Core watchdog creation/readiness still
needs to call the provider and check its actual thread; the peer is not that proof.
The exact set, full hashes, peer identity, commands and temp retention are in the
[receipt](../plans/ui-blueprint/receipts/H01-process.md). Root-owned wire/API/library
hooks were not altered by Native. H01 supervisor/worker integration and independent
unsafe/allocator/lifecycle review remain mandatory.
