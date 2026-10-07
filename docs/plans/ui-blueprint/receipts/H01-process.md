# H01-process implementation receipt

- status: `saved_candidate / finite process packet complete` for the concrete
  process implementation. Real Core watchdog creation/integration remains separate.
- authority: H01-process packet7ffe5df under coordinated PLAN.UIB@1; frozen API
  `3abd8e5f9c054c12e910d4c8defc2f8a64a8b268`, then explicit stable Core build handoff.
- basis: AGENTS → registry10 → D02@2 all clauses, D05@3/MEMORY@2/WORK@1/D07@5
  full applicable closure, D01/D06/RUST/DEV.RUST@2. Saved process_api.rs/host.md
  read in full; current prior product leaves reused. libc audit7b1489f reused;
  selected public bindings, installed SDK/man semantics and narrow stack source
  references inspected. No copied/private ABI or new dependency.
- exact own paths: `crates/host/src/process.rs`, `crates/host/src/process/spawn.rs`,
  `crates/host/src/process/worker.rs`, `crates/host/tests/process.rs`,
  `crates/host/tests/support/process_peer.rs`, `docs/development/host-process.md`,
  this receipt. Core supplied lib/Cargo hooks; Native did not edit shared owners.
- result: actual DarwinPlatform ProcessPlatform/WorkerPlatform and DarwinChild,
  exact owned spawn/FD mapping/nonblocking IO/poll/reap/core-stack/fatal hooks.
  No UI, Swift, AX, capture, real apps or hostile JSON executed. B capture residual
  was neither operated nor bypassed. No independent/full-H01 acceptance claim.
- checkpoint / push: commit containing this receipt on master; exact SHA and
  origin/master push result returned in terminal handoff. Root granted only the
  seven own paths; branch/index preflight confirmed master and empty index.
  Core hooks/API/publication/quota and Web changes are excluded. Git lease released
  after successful push; unchanged tests were not repeated.
- acceptance basis: shared-input proof is tied to the fully hashed stable working
  Core bytes above, not a claimed saved Core implementation. Root will save that
  shared slice separately before acceptance. Own temp remains for review/acceptance.

## Actual checks and failures

Rust1.96.0, aarch64-apple-darwin, current macOS27.0.1 development host. Commands used
CARGO_TARGET_DIR in the unique task-temp below, locked/offline:

- `cargo check -p uiblueprint-host --lib --test process --example process_peer`: pass.
- `cargo build -p uiblueprint-host --example process_peer`: pass.
- Pre-extension `cargo test -p uiblueprint-host --test process -- --test-threads=1`:9/9 pass.
- New stable signature: scoped check/peer build/Clippy pass; targeted watchdog
  request validation and actual stack/mapping tests2/2 pass. Other eight unchanged
  process results reused; no full new-signature suite run is implied.
- `cargo clippy -p uiblueprint-host --lib --test process --example process_peer -- -D warnings`: pass.
- Own five Rust files `rustfmt --edition2024 --check`: pass. No workspace suite.

Initial process run passed8/9. Builder requested1048576 watchdog bytes but public
query reported1060864, exceeding the unchanged1MiB ceiling. Only the failing stack
case was repeated to establish that exact number. The peer startup then requested
1032192 (1MiB minus host page16384), and actual watchdog extent was1044480, below
1048576. Main actual8372224 is below8388608. The startup change affects every peer
scenario, so the final9-test run covers it; a targeted stack/inventory output also
passed. No number was fabricated, rounded down or used to relax the profile.

This is an owned test-peer creation result only. **Core's real watchdog has not
been repaired or qualified by it.** Root approved the narrow generic API extension; Core owns its signature/doc:
 `WorkerPlatform::watchdog_stack_request(ceiling: usize) ->
Result<usize, HostError>`. Native implemented checked public page-size headroom, conservative page rounding and
minimum/profile validation;
Core uses the request at Builder creation and must query/reject oversize actual
extent before readiness. process_api.rs and real worker creation are Core-owned.
The new method and peer migration compiled and passed their two affected tests
on Core's supplied stable signature; Native did not edit process_api.rs. Exact source path/sequence is in
[host-process.md](../../../development/host-process.md). The existing queried-extent API
remains truthful; previous9/9 applies to the pre-extension slice, and new2/2 to
provider/peer/actual-extent linkage. No ceiling changed. Other OS/compiler/creation
mechanisms need their own qualification; real Core watchdog integration stays open.

## Process invariants and coverage

FD0/1/2 verified /dev/null;3/4/5 protocol/fatal mapping; parent nonblocking/CLOEXEC;
empty env/one argv; unrelated inheritable FD exclusion; low-FD source/target collision
inside an owned nested peer; partial setup cleanup under child-only RLIMIT_NOFILE;
failed spawn descriptor balance; closed/empty/WouldBlock/backpressure distinctions;
default SIGPIPE in an owned peer survives per-send suppression; max36 poll bounds,
relative wait and no EINTR renewal; independent children; actual cached exit/signal
and repeated terminate/reap; externally reaped child returns CleanupPending and
cannot be signalled again. Core/main/watchdog query and fatal/full/missing status
exit cases passed. No owned peer remained after checks.

Unsafe is isolated to this process module/test peer, with pointer/FD/C-handle/PID/
thread lifetime comments. Production fatal path is one best-effort64-byte
DONTWAIT|NOSIGNAL send then _exit; no Rust allocation/format/panic/unwind/retry.
No status is inferred when bootstrap fd is unavailable. No broad signal handler
is installed by the library. Terminate is not reap. Drop is nonblocking best effort,
not cleanup proof; Core must retain/quarantine owner+grant until confirmed wait.
External same-process reapers violate exclusive-child ownership; Darwin has no
atomic pidfd kill to mask that violation. Lost wait ownership fails closed.

## Fixed owner inventory and shared input provenance

Measured inline: DarwinPlatform0, DarwinChild20, SpawnSpec4104 bytes. Fixed poll
backing288 bytes. Bounded spawn locals include argv16/env8 and two opaque C-handle
roots16 bytes. No production Vec/String/graph or Rust heap allocation in this OS
owner. C file-action/attribute allocations, OS buffers and stack mappings remain
explicit separate opaque categories; this is not total host/RSS accounting.

All38 selected shared files were hashed before/after each phase and match exactly
within each phase. The only authorized inter-phase change is process_api.rs. The full
set includes root Cargo/lock/toolchain, host manifest/lib/API/limits/buffers/protocol/
publication/quota and dependency schema/engine/plugin-api manifests+source. Core's
unconnected worker/domain/allocator and Web WIP were not consumed as proof inputs.
Key exact hashes match the delivered handoff:

- Cargo.toml `299c8a44d63290ecf126fda4c0bfc6170883a2c5164d0657b1f198c0448758f0`
- Cargo.lock `af468c06f884e8fa07996bdcbc39785d0c005004fb59f1a9f7e74a40cb6c8623`
- host Cargo `cbd51a8e013013b9a1b70b922d14de21674ecc1b0f486de3fda62de7897ccd01`
- host lib `0ec828b36f1e22fe5e93cafd5ee2ef53b16fefebcc727a58a44f79230f4faaa9`
- Initial process_api `cf2ba1f7abfa4d100f067efc428cb8f45e3ad991cf8057b67faeb109dc9cf72a`
- Agreed extended process_api `da5e278829b2b6b0d0d62c62b208cf838c8076bea117ee7b806fe49949878b56`

Full before/after JSON and raw logs/compiled peer are task-temp only:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-h01-process-v2hfxco_`.
shared-before.json/after.json SHA256:
`af1f38247056c61ef88205919a527903064c676e982d0d92179cf8bf283ada6e`.
Initial peer binary SHA256 `6e5161749019cf8ab2abc74ef53737f2fe67ed0143d33d2ff30c2014f8311c59`.
Extended phase full manifest files shared-extension-before.json/after.json SHA256:
`564beff7b5dbebf3735dbf1853e079b7144f41ce89c488daf36374eaa04e6a7f`.
Current peer binary SHA256 `a93d68f09c32e5d27e3e9bc776de57b71ddbb10e574fe07416138f89f00f2722`.
Owner Native for this finite run, consumer Core/root H01 handoff; cleanup only these
run-owned files after accepted handoff or explicit root cleanup. No new persistent
evidence system and no logs under Codex/skills/config. Source/docs only in Git.

Next: Core/root chooses watchdog creation/API handoff, integrates this OS owner
with actual supervisor/grant/quarantine/worker/allocator, and performs independent
lifecycle/unsafe review. Those are not accepted by the prior9/9 plus current2/2 scoped process evidence.
This checkpoint saves only the finite Native-owned slice; no autonomous next packet.
