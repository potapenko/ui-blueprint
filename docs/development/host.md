# H01 bounded host — implementation in progress

Norms: [D02@2](../specs/development/decisions/d02-boundaries.md), [D05@3](../specs/development/decisions/d05-limits.md), [D05-MEMORY@2](../specs/development/decisions/d05-memory.md), [D05-WORK@1](../specs/development/decisions/d05-working-memory.md). Source handoff: [reviewed working-memory design](working-memory.md). This foundation is real code, but is not a completed host/worker, allocation guard or live acceptance.

## Working foundation

`uiblueprint-host` statically depends on the existing schema/engine/plugin API and approved serde/json. No dependency versions/features changed. The narrow approved libc0.2.190 binding is available on Unix; no framework or new registry version was introduced.
`HostLimits` requires explicit values within D05 ceilings, including ordinary/publication split, root retained partition, fixed parent/input/ingress/result/control budgets and stack/cleanup bounds. No Default or measured amplification multiplier. Worker ordinary admission cannot exceed63MiB even when a caller selects a smaller publication reserve.
`ParentBuffers::new(limits, other_fixed_roots)` uses fallible fixed setup, counts actual root/backing capacities and drops partial construction on failure. Inputs, two separate ingress slots per worker and three output slots per completion group are disjoint fixed owners. ByteLease exposes only bounded slices/length and cannot grow a Vec. Exclusive borrows cover individual slots, not the entire host: a held output lease leaves another worker's input available. Logical bytes are cleared on release/reuse; this is not a forensic zeroization/RSS claim.
`other_fixed_roots` is an explicit inventory charge, NOT evidence that unimplemented supervisor/process owners are already closed. RuntimeHost must supply its actual complete control/spawn/lifecycle inventory before children. Completion-group admission, publication state machine and process ownership are still being implemented.
`Control` is a fixed64-byte private record: magic/version, closed kind/class, result slot, flags, epoch/operation and three numeric values. Encoding/decoding allocates nothing; invalid magic/reserved bytes/tags/slot reject. Body bytes are opaque. Message-specific state/flags, partial frame/commit/ACK and terminal/late checks belong to the supervisor; `matches` alone is not publication proof.

## Compiling platform API handoff

`src/process_api.rs` is the bounded substitution boundary, with no dummy success or placeholder implementation. Core owns it; a root-assigned platform worker can implement `src/process.rs` and its own tests/peer without touching Core's limits/pools/protocol/supervisor/worker/allocator files.

- `SpawnSpec::new(&Path) -> Result<SpawnSpec, HostError>` copies an explicit absolute executable path into a fixed4096-byte root, rejects interior NUL/overflow, and exposes a valid CStr. No shell, inherited environment or arbitrary argv collection.
- `ProcessPlatform::spawn(&mut self, &SpawnSpec) -> Result<Self::Child, HostError>`, associated Child: OwnedProcess. Use exact-path posix_spawn; only owned returned PID/FDs are cleanup targets.
- `ProcessPlatform::poll(&mut self, &mut [PollInterest], wait_ms:u32) -> Result<(),HostError>` handles at most36 interests (4×(1 worker+2 helpers)×3 lanes) with fixed backing. Relative bounded wait, no hidden EINTR retry/deadline extension.
- OwnedProcess: `write_input`, `read_output`, `read_fatal` → Transfer::{Bytes,WouldBlock,Closed}; `close_input`; `terminate` only the owned unreaped child; `try_reap` → Running/Exited{code}/Signaled{signal}; borrowed input/output/fatal descriptors. No PID supplied from UI/canonical input.
- Child protocol descriptors:3 input,4 output,5 fatal;0/1/2 go to /dev/null. Parent descriptors nonblocking/CLOEXEC; do not inherit other sessions' FDs. File actions must avoid source/target descriptor collision. Raw stderr/panic bytes are neither persisted nor interpreted.
- WorkerPlatform: `setup_main(main_stack_bytes) -> Result<actual_stack_bytes,HostError>` must set/verify core-dump suppression and main stack before untrusted input; `current_stack_bytes()` runs inside the actual watchdog to verify its explicit stack; `fatal_exit(fd, &[u8;64], code) -> !` performs one best-effort nonblocking fixed write then _exit, without Rust allocation/format/unwind. fd<0 skips unavailable bootstrap status; parent must not guess quota cause.

Native implementation errors are bounded HostError categories, never payload/path dumps. No successful cleanup claim before confirmed wait/reap; uncertain/lost ownership must fail closed rather than signal a reused/user PID. Core owns root Grant, quarantine/deadline/effect/publication state, and holds it until cleanup. The OS layer returns facts; it cannot release a grant based on exit telemetry.
Initial target is pinned aarch64-apple-darwin. Spawn/file-action C internals and OS buffers remain the explicit opaque platform category, not Rust parent-pool accounting or a security sandbox. Actual FD/stack/core-dump/reap/allocation proofs remain required.

## Dependency evidence

D07@5/H01-PROCESS-001 checkpoint `7b1489f5a23955978fdcba78c3d5131417768805` precedes adoption. libc EXACT0.2.190, default-features=false, existing lock checksum `ce5d3ddc6d3fa000eb1536d85e147bfe31aacaba692ed6a876f95cb7c855be78`. Selected Cargo/license/unix/bsd/apple installed files matched the crate archive. Full MIT read (Rust Project Developers); MIT OR Apache-2.0; no NOTICE; MSRV1.65. No copied binding declarations/source.
Read only posix_spawn/file-actions/flags, socketpair/fcntl/poll/read/write/close, kill/waitpid, getrlimit/setrlimit, _exit and relevant pthread stack queries. The selected dependency enables those real consumers; this audit is not successful runtime proof.

## Next implemented stages and proof boundary

Core: actual HostDomain/RuntimeHost and input/completion group lifecycle; trusted target/operation binding; parent fixed-frame publication/ACK; reusable worker invoking canonical APIs/K01; executable-only quota GlobalAlloc; deadlines, nonce, quarantine and failure handling. Platform process slice is disjoint only after root assigns its exact files.
First prove guard/fatal/owned cleanup on small deterministic processes. Only then run hostile2MiB/tag-order/array/numeric source-audit families inside that guard. No unguarded adversarial parsing, echo-only canonical-processing claim, caller-lease early release, SDK/pixel or full D06 assertion. Current checks/checkpoints live in [H01 receipt](../plans/ui-blueprint/receipts/H01-host.md).

## Connected publication/counter stage

ParentBuffers now reserves whole completion groups before dispatch. Group capacity
remains occupied while caller-held committed frames exist, even after worker teardown.
Publication copies directly into reserved slices, distinguishes body/commit/ACK,
rejects wrong/duplicate/partial/late/oversized controls, and retains only fully ACKed
channels on terminalization. Other worker/group slots remain usable. This is the
actual safe byte state machine, not yet a complete supervisor/canonical worker.
QuotaCounter provides checked atomic precharge/release/peak accounting, including
full-new-layout realloc reservations while old bytes remain live and no wraparound.
It does not install GlobalAlloc or prove real System/fatal enforcement by itself.

Native's actual Darwin process module and process_peer target are now linked.
The approved WorkerPlatform::watchdog_stack_request extension computes a checked
creation request only. Native measured main8,372,224 and watchdog1,044,480 bytes
below unchanged8MiB/1MiB ceilings in its peer; real Core watchdog must independently
query its created thread before untrusted work. See [process handoff](host-process.md).
Core's authority/domain/worker/allocator sources remain unconnected WIP; they are
not included in this stage's compile/runtime proof. Full H01 remains unfinished.
