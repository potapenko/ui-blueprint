# H01-process implementation and reaping repair receipt

- status: `H01-PROCESS-R1 checkpoint_ready / waiting_resource: Git lease`.
  Native author checks pass; same independent reviewer and real Core lifetime
  integration remain required. This does not close the defect or full H01.
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
- previous checkpoint / push: bb69d4c1f9128975fcdce72a8329b0a637756037 saved and
  pushed the original implementation; independent review rejected R1 afterward.
- current repair paths: process.rs, process/reaping.rs, process/spawn.rs,
  tests/process.rs, tests/support/process_peer.rs under crates/host;
  docs/development/host-process.md and this receipt. worker.rs/stack behavior unchanged.
- current repair checkpoint / push: pending root grant; index untouched.
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

## H01-PROCESS-R1 finite repair

Authority: [repair packet](../packets/H01-reaping-repair.md)ee5f95d, approved Restore
under D02.LIFECYCLE, and [same reviewer's finding](H01-process-review.md). No new
user approval, number/spec/API/dependency or operator signal change. Selected
D02@2/D05@3/MEMORY@2/WORK@1/D07@5 closure and RUST/DEV.RUST remain applicable.

Native now reads actual SIGCHLD disposition/flags before spawn-owned allocation,
immediately before posix_spawn, after successful spawn, before wait and before
signal. SIG_DFL without SA_NOCLDWAIT/SA_SIGINFO is the narrow supported profile.
Explicit SIG_IGN also causes Darwin auto-reap (public XNU source inspected) and is
refused, as are custom handlers. No library sigaction setter is used.

Before spawn, unsupported state returns InvalidState and creates no child. After
OS spawn, detected drift returns an owned wrapper latched Lost rather than a
failure that could release a live reservation. Subsequent policy/ownership loss
returns CleanupPending and prevents PID wait/signal; restoring policy cannot
re-enable authority. Confirmed cached reap remains valid. No Darwin pidfd invented.

Core/embedding must continuously retain compatible process-wide policy and exclusive
wait ownership through all managed/quarantined child lifetimes. Checks do not catch
an unsupported policy changed and restored wholly between calls; arbitrary same-
process native code is outside this supported contract/security guarantee. Core
owns actual HostDomain/RuntimeHost integration, refusal/quarantine/retained grants
and prevention of further dispatch/reuse. Existing spawn/lifecycle checks carry these outcomes. Root's f499bec coordinated
amendment now requires ProcessPlatform::validate_parent_reaping() for the actual
Core consumer; Core declares it first, Native forwards to the same predicate.
Core supplied the actual stable declaration; Native forwards to the real predicate
and ran only affected provider checks. Native did not edit the shared signature/file. This is not a claim Core's
lifetime guarantee has already been verified.

### Focused executed checks

On saved Core7b942ab1ddd5af334da18412c6b2d3ac7bd7a21b-compatible inputs:
scoped check/lib+process-test+peer, peer build and Clippy -D warnings passed.
Four focused process tests passed: incompatible SA_NOCLDWAIT/SIG_IGN/custom handler
refusal (three disposable cases); detected drift/latching after restore; repeated
owned terminate/reap with independent children; external-reaper lost ownership.
The drift test initially failed its automatic-reap flag because EOF preceded kernel
cleanup; its assertion was repaired to wait boundedly for actual ECHILD. Library
refusal/latching already passed in that first run. No PID was signalled after loss.
No mass PID-reuse stress, UI, apps, hostile JSON or operator/test-runner signal
policy mutation. Existing FD/stack/fatal/headroom results remain prior evidence.

Every unsupported-state test verifies no child, unchanged FD count and unchanged
signal policy across the attempted library spawn. Signal changes/restoration occur
only inside an owned test peer. The drift grandchild exits itself; the containing
peer confirms kernel status, restores its own policy and exits. All peers are gone.

All38 shared before/after hashes are identical and each also matches saved7b942ab.
Full files in new task-temp:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-h01-reaping-kghf73oo`.
Manifest SHA256 `564beff7b5dbebf3735dbf1853e079b7144f41ce89c488daf36374eaa04e6a7f`.
Current peer SHA256 `2a0a04304a1937051d36dae1a6d559249dafb34e89f8c83c22170f7b9813f749`;
compiled target reused the prior unique H01 process task-temp. Raw logs/downloaded
source stay task-temp; no new persistent evidence system. Root/Native retains own
run evidence until repair review/acceptance or explicit cleanup. No old evidence
or other-owner file was removed. Exact source hashes and final document checks are
recorded locally; root Git grant remains separate from independent acceptance.

### Provider linkage on the agreed working API

Root f499bec amendment and actual Core declaration:
`ProcessPlatform::validate_parent_reaping() -> Result<(), HostError>`.
API SHA256 `2f9a823aa14468a38ff40256ab20c259babda05465428e3c8fc34c767c00df91`.
Provider forwards directly to existing read-only OS predicate, no duplicated/dummy
policy. Existing spawn/wait/signal checks remain. Two affected peer regressions
passed, explicitly testing public validation refusal and compatible-policy restore
while child loss stays latched. Scoped check/peer build/Clippy -D warnings and own
rustfmt passed. Previous four focused reaping results remain phase-specific; no
unrelated FD/stack/fatal/workspace suite was repeated.

All38 provider-phase shared hashes match before/after. Only the authorized API
changed from saved7b942ab; current API is a hashed working input, not a claimed
saved Core checkpoint. Full provider-shared-before.json/after.json are in the same
repair task-temp; manifest SHA256:
`da4837e06106e88d21f41f761d5d28dca49e12c64a30ac3d72da4f2ed7f4270a`.
Provider peer SHA256 `661802a1b9ac5c55e05413571c48bc20df76c40138b5db60f5f30a5e58050c0e`.
Owned peers are gone; index is empty. Native repair scope remains seven paths;
Core shared declaration/host integration and other owners stay unstaged by Native.
Core must bind supported policy/exclusive reaper lifetime, actual validation,
poison/quarantine and grant/ACK preservation. No defect/full-H01 acceptance claim;
same reviewer rechecks sequential saved Native/Core results after checkpoint grants.
