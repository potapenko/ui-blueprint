# H01 allocator and guarded-input proof

Status: first four cases saved731a524; combined private-seam wave now passes five
cases, including controlled alloc/zeroed/realloc null handling. Three-path follow-up
checkpoint-ready. Hostile/RuntimeHost/remaining phase execution stays open.
No independent source acceptance, full H01 or live/SDK/RSS claim.

## Authority and source basis

[Packet](../packets/H01-allocation-proof.md)e11ca17 and hook amendmentd37c98c under
user-authorized coordinated PLAN.UIB@1. Master only, inherited settings, no nested
work. Own writes: new tests/allocator.rs, tests/hostile_worker.rs,
tests/support/allocator_probe.rs under crates/host, and this receipt. Core exclusively
owns production repairs/Cargo/example hook; other source/spec/fixture owners untouched.

Recovered active user AGENTS, implementation/QA/operational routes, repo registry10,
RUST/DEV.RUST@2; reused full current D02@2/D05@3/MEMORY@2/WORK@1 and packet's
D01/D03/D06/D07@5/C01-EVIDENCE/product/acceptance/reuse closure, with ANALYSIS/
TYPES/VALIDATION/CLI for0.2. Requirements are normative; Core's H01-host handoff is
attributed implementation evidence. Excluded drawing/export, live platforms/UI,
mobile/future, new benchmark/framework and source applications.

Four actual allocator/worker files match saved50c0c95 and packet hashes exactly.
Read actual quota allocator/counter, public Control/process APIs, worker operation/
tape/IO paths and the existing process/runtime proof pattern. Native provider and
Core's small installed-guard/reap prerequisite are attributed, not relabelled as
our execution. Existing safe counter overflow/underflow/concurrency evidence is
reused; no unchanged counter suite rerun. No hostile family has executed yet.

## Exact first proof boundary and checks

Core installed only the agreed allocator_probe example hook; current host Cargo
SHA2569a3f14090f6a5e1ed26eb19357d6ae07959a64d845e1fe30e3ed3676935854d9 was an
explicitly agreed WORKING input, not called a saved manifest. Root/Core froze the
named inputs for this finite run, then the barrier was released immediately after
the four results and before/after comparison. No freeze was held for hostile work.

The probe includes the unchanged production quota_allocator.rs. It does not install
an allocator in the operator/test runner, and deliberately does not install one
globally in this method-level child: explicit GlobalAlloc calls have exact isolated
counter ownership. Each pointer/layout is valid, small and owned; volatile writes/
reads verify actual storage. The real session-worker separately proves global coverage.
Fatal reporting is confined to disposable Darwin children with actual setup/reap.
ParentGone at probe tail is an explicitly invoked counter-reporting sentinel, not
a claim that parent EOF occurred. Direct fatal(System) is encoding proof only.

Passed four cases, zero ignored:
1. Actual System-backed alloc/alignment/zeroed bytes, growth preserving contents,
   dealloc/exact-cap reuse and two-thread bounded allocation/release balance.
2. Ordinary cap refusal and FULL new2048+old3072 realloc reservation even on shrink;
   private fatal record retains old live3072. Parent survives and reaps another child.
3. Publication reserve works only for its owning thread; live reserve escape,
   nested publisher and a different thread's borrowing attempt terminate distinctly.
4. Exact fatal correlation/phase/reason/exit and missing-status distinction. Exit
   code alone is not fabricated into a fatal record or System-null evidence.

Build and test commands used a unique task-temp target, not repository build logs:
`cargo +1.96.0 build --locked --offline -p uiblueprint-host --example allocator_probe --target-dir <task-temp>/target`;
`cargo +1.96.0 test --locked --offline -p uiblueprint-host --test allocator --target-dir <task-temp>/target -- --test-threads=1`.
Scoped Clippy for that test/example passed afterward; own rustfmt/diff checks pass.
No signal policy/global allocator change in the test runner or operator process.
Every spawned small probe was confirmed reaped; no test fatal ran in the runner.

Before/after16-file exercised-boundary digest, unchanged:
`b17be48b593e3116d05d4ae7a55b51aa37f8c8a972d2ad9fe9a947642b53bd5b`.
Recipe: compact sorted JSON path->SHA256 map then SHA256. Exact set: root Cargo.toml,
Cargo.lock,rust-toolchain.toml; host Cargo.toml; host src/{lib,quota,limits,protocol,
process_api,process,quota_allocator}.rs and src/process/**/*.rs; tests/allocator.rs
and tests/support/allocator_probe.rs. This identifies the exercised allocator/
counter/control/OS boundary; it does not claim complete supervisor/worker execution
or independent acceptance of every linked dependency. Locked build graph supplies
the current dependency context. Changing supervisor integration gets its own barrier.

Temporary target owner=Integration, immediate consumer=this finite packet; path
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-h01-allocation-07xaavuw`.
No raw logs persisted. Remove this run-owned target after its proof is accepted/
handoff no longer needs it; do not delete other existing evidence.

## Precise remaining gaps and next steps

System allocation failure: current concrete System calls have no safe bounded
failure injection. No huge real/VM request or exhaustion was attempted. Proposed
smallest Core seam: privately factor only System forwarding through a backend for
test-only inclusion; shipping always System. A bounded backend returns null for one
small request and delegates other calls. Verify precharge release on alloc/zeroed
null and old-layout retention/new-reservation release on realloc null before fatal.
This proves guarded null handling, not OS exhaustion. No public test API/command or
substitute guard/parser is requested; Core/root must approve/own the seam first.

Checked counter overflow/underflow evidence does not mean deliberately invoking
undefined invalid deallocation or enormous Layout requests. Actual fatal invariant
paths covered here are publication misuse/configuration, with bounded valid inputs.
Any unreachable mandatory branch stays explicitly unverified, not silently passed.

hostile_worker.rs is prepared but UNEXECUTED and excluded from this first save.
It requires a saved current RuntimeHost/supervisor input barrier; each run first
proves small installed guard failure and actual reap before constructing/executing
at-most2MiB core/analysis tag-order families, bounded escaped-string/number/map
cases and earlier ACKed canonical bytes surviving later channel quota failure.
Parent never decodes those hostile bodies. Further phase-specific allocation proof
must use actual reachable operations and observed phase attribution; missing
consumer/phase boundaries return to Core, not an alternate mock or raised limit.

## Coherent checkpoint request

Save only crates/host/tests/allocator.rs,
crates/host/tests/support/allocator_probe.rs and this receipt. Core saves Cargo
hook separately; do not stage prepared hostile tests or another owner's files.
Root short Git grant pending. Return SHA/push/input identity/release; then continue
the same finite packet after the next exact provider/barrier handoff. H01 remains
waiting_evidence for the named remaining proof; no new task is implied.

## Saved combined seam / bounded injected-null proof

Root amendments a6c1209/d9188b8 authorize Core's private factoring; no public test
API/command or allocator replacement. Alloc/zeroed seam saved5e5da61, combined
realloc seam saved00e282e36f509375bd33cc84b2e9a4997d6b6c57. Current source pin:
`2f1bf278b9265855ececed61ccb5b3f2e1904f17b1855adb6d659782b241e50e`.
Shipping calls still pass fixed System forwarders. Actual private signatures:
allocate_with(Layout, unsafe fn(Layout)->*mut u8), and
reallocate_with(pointer, Layout, new_size, unsafe fn(pointer,Layout,usize)->*mut u8).
No Integration production edit. The alloc-only bfa9 pin is superseded, not reused
as proof of realloc. Other worker pins remain unchanged.

Probe callbacks return null once for a64-byte request and delegate other calls
to System with valid layouts/ownership and zeroing where required. Fixed one-byte
markers prove exactly one forwarding call; no heap allocation, logging or unwind
occurs in a callback. Realloc callback inspects initialized old storage without
freeing/transferring it. Successful precharge then null returns System fatal102,
requested64/live32: only the new charge released, existing32 retained through fatal.
Separate over-cap alloc and old+new realloc cases prove zero forwarding calls.
No huge real/VM allocation or OS pressure. Direct fatal(System) remains separately
labelled reporting-only; injected callbacks prove real null-handling branches, not
an exhausted OS allocator.

After root's explicit short ACK, provider inputs matched saved helper checkpoint
e11b44bd8840e443b6bf46f9f956ca0e5f585b38. Same16 exercised input recipe above,
before/after identical:
`4d717bc422445f92b9efa482f32c6d85d658817658c22c45eaf35f54b2c21c53`.
Only the two Integration test/probe files were working inputs; all14 provider/
manifest/toolchain files matched e11b44b. Build/example, the affected allocator
test target (5 passed/0 failed/0 ignored), scoped Clippy -D warnings and own fmt
all passed once in the existing run-owned task-temp target. All children reaped.
The named barrier was released immediately after the before/after comparison;
no hostile execution/preparation was included in its duration.

This closes the identified bounded forwarding-null branch gap. Guarded worker
input/phase/ACK survival proof still awaits its exact current provider handoff;
prepared hostile_worker.rs remains unexecuted and excluded from this checkpoint.
Follow-up save set remains allocator.rs, support/allocator_probe.rs and this receipt
only. Core controls production/lifecycle changes; root grants the short Git lease.
