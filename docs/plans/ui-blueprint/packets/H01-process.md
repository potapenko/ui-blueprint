# H01 owned process implementation

Class: shipping_product. Existing Native owner; inherit settings, no nested agents.
Authority: user-approved coordinated PLAN.UIB@1; adopted D02/D05 and D07 host binding
choice. Frozen foundation/API: `3abd8e5f9c054c12e910d4c8defc2f8a64a8b268`, pushed.
Current master, no branches/worktrees or real applications/UI/SDK input.
Consumer: Core's actual H01 supervisor and guarded worker; implement the real OS
owner, not another diagnostic design or placeholder.

## Basis and fixed API

AGENTS → specs/README registry10 → D02@2 all clauses, D05@3, D05-MEMORY@2,
D05-WORK@1, D06@1, D07@5/H01-PROCESS-001, D01/RUST/DEV.RUST@2 and their complete
explicit product/evidence closure. Same closure as H01-host packet; root read it
fully. Read new/missing contracts before source. Core's accepted libc0.2.190 archive/
license record and saved7b1489f are reused, not a reason for another general audit.
Inspect the exact public selected bindings/OS semantics needed for unsafe invariants.

Read the full saved docs/development/host.md process handoff and
crates/host/src/process_api.rs at the foundation SHA supplied by root. They pin
SpawnSpec, ProcessPlatform, OwnedProcess, WorkerPlatform, Transfer/Reap/PollInterest
and fixed descriptor semantics. Core owns this API; do not change it or infer an
alternative from the broad goal. Return an exact source/API dependency if necessary.
Mode Restore registered boundaries; no product/spec/number/permission changes.

Required facts: fixed absolute SpawnSpec (4096-byte root), no shell/inherited
environment/arbitrary argv collection; exact owned child and descriptors; at most36
poll interests with a bounded relative wait and no hidden EINTR deadline renewal;
Transfer::Bytes/WouldBlock/Closed; no successful cleanup before actual wait/reap.
Child descriptors3/4/5 are input/output/fatal;0/1/2 go to /dev/null. Parent descriptors
are nonblocking/CLOEXEC; unrelated session FDs do not leak into children. Source/
target FD collision in file actions must not corrupt ownership or close final lanes.
Worker setup must enforce/verify core-dump suppression and main stack, query the
actual watchdog stack, and issue one allocation-free nonblocking64-byte fatal status
write followed by _exit. Missing bootstrap status fd is explicit, not quota proof.

## Exact disjoint writes and collaboration

Own only crates/host/src/process.rs and private crates/host/src/process/** helpers,
crates/host/tests/process.rs, crates/host/tests/support/process_peer.rs, plus
docs/development/host-process.md and docs/plans/ui-blueprint/receipts/H01-process.md.
No process_api/lib/limits/buffers/protocol/supervisor/worker/allocator/Cargo/spec/
Web/Swift/old fixture edits. Core alone owns lib/module export and manifest wiring.
If the test peer needs a Cargo target or the process module needs its lib hook,
return the exact minimal lines/paths through root; do not add framework/dependency
or rewrite the API to avoid that coordination. Peer source remains test-only.

Implement a concrete platform type satisfying the pinned traits on
aarch64-apple-darwin with existing approved libc0.2.190/default-features=false.
Isolate justified unsafe locally and document/check pointer/layout/FD/PID/thread/
lifetime invariants. Preserve other crates' unsafe policies. No handwritten private
Apple ABI or inaccessible API; this is Rust/POSIX process work, not visible Mac UI.

Return only bounded HostError categories. Do not put paths, peer payload or raw
OS/private diagnostics into errors. Closed peers must yield bounded failure rather
than unexpectedly terminate the supervisor. Preserve global application behavior;
do not silently install broad signal handlers or close descriptors you do not own.
Partial spawn/setup failure cleans only resources acquired by this attempt.
Never signal a UI-supplied PID or a PID after its child has been reaped/ownership
lost. `terminate` is not proof of exit. Facts of Running/Exited/Signaled remain
distinct; uncertain cleanup fails closed. Core owns grants/quarantine/deadlines/
effect receipts and decides when they may be released; OS layer cannot free them.
Use fixed/bounded Rust control storage compatible with the parent inventory; report
exact inline/backing owners and any unavoidable C/OS allocation category separately.

## Required finite proof

Use small task-owned non-UI peers only, no browser/AX/pixels/real app or hostile
JSON input. Verify spawn and descriptor mapping, nonblocking IO and closed peer,
no unrelated-FD inheritance, partial setup cleanup, bounded poll/limit/interrupt
semantics, actual child exit/reap, repeated termination/reap without reused-PID risk,
and independent owned children. Exercise FD collision conditions without touching
user descriptors. Prove core-dump/main/watchdog stack behavior inside the proper
owned child/thread, not by mutating the operator's shell or another process.
Fatal-status/_exit checks run only in owned child; no callback allocation/format/
unwind. Missing status remains generic failure. Report unsupported proof precisely.

All waits/joins are bounded and resource ownership recorded; no blanket process
cleanup. Current source tests/builds consume shared API only after root/Core's
short stable-input handoff. Use Rust1.96 locked scoped checks/fmt/Clippy/process
tests; no workspace suite or unchanged Web/native capture QA. Raw run output and
compiled peer live in a unique task-temp outside repo/config; clean only own files
after accepted handoff. No additional persistent evidence system.

Return actual API implementation/owner inventory, exact paths/checks/hashes and
remaining H01 integration proof. Every coherent checkpoint gets root's short Git
grant, exact-path commit+push/SHA/release. No independent acceptance claim, total
host memory/D06/live qualification, or autonomous next packet. Core integration
and later independent host allocator/lifecycle review remain mandatory.
