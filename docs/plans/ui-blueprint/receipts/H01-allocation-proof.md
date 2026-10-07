# H01 allocator and guarded-input proof

Status: small proof saved731a524 and combined null proof saved2a712aa; four real
guarded-worker cases and two existing-API phase-refusal cases now pass. Worker
test/receipt checkpoint-ready. Remaining
phase-specific allocator-exhaustion coverage stays explicit below; no full H01 claim.
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

## Saved combined proof and actual guarded-worker wave

Combined checkpoint2a712aa36b826a772aa4ce6665daee912dfb8cb4 was pushed; its exercised
hash4d717bc4…21c53 matched and the short Git lease was released. Root then explicitly
continued this packet on saved provider8edf546103430f02595441b303850a886134b0dd,
after Core fixed its separate held-ingress admission issue. No Integration production
repair or new parser/harness was introduced. Unconnected native_binding.rs was excluded.

Root/Core ACKed only the finite worker-run inputs. Before execution, every provider/
manifest/fixture matched saved8edf546. Full64-input before/after digest, unchanged:
`3ab2314a51bd92c0023991a2f5bf5ccc2a1ed247f56e9c6a7d166b4e1c78360a`.
Recipe remains sorted compact JSON path->SHA256 then SHA256. Set: root Cargo.toml,
Cargo.lock,rust-toolchain.toml; Cargo.toml and all TRACKED src/**/*.rs at8edf546 in
crates/{host,schema,engine,plugin-api}; own tests/hostile_worker.rs; six normal inputs:
fixtures/golden/{ENV-CAPABILITY-VALID,ENV-SNAPSHOT-VALID,ENV-REQUEST-VALID,ENV-DELTA-VALID}.json
and fixtures/analysis/{query-gap,measurement-gap}.json. New test was a working input;
provider was saved. No broader workspace/integration input claim is made.

Executed once in the existing uniquely owned task-temp target:

```sh
cargo +1.96.0 build --locked --offline -p uiblueprint-host --bin session-worker --target-dir <task-temp>/target
cargo +1.96.0 test --locked --offline -p uiblueprint-host --test hostile_worker --target-dir <task-temp>/target -- --test-threads=1
```

All4 passed, zero ignored,2.42s test runtime. Each entry first requires a OnceLock
prerequisite: an actual2MiB worker/1MiB ordinary quota refuses its fixed2MiB input
allocation, parent receives ResourceLimit without committed data, then actual Closed/
reap restores the reservation to QuotaLedger::backing_bytes(). No hostile body is
constructed/executed until this check passes. It was necessary for the changed real
supervisor baseline, not a reuse of a mock or unconfirmed old provider.

1. Core0.1 and analysis0.2 depth120 array families bounded by2MiB: kind-first rejects
   InvalidInput without quota loss; data-before-kind reaches real guarded quota
   refusal. Actual worker closure/reap releases session/root reservation. Parent
   only constructs/copies bytes, never deserializes hostile JSON/Value/Content.
2. Three malformed <64KiB inputs exercise escaped strings, long decimal digits and
   repeated map keys. All reject InvalidInput without output; the SAME worker then
   validates/publishes the unchanged normal query. No raw canary payload is published.
3. Real Observe/Tape path on a16MiB worker: first canonical external-semantics response
   is correlated/ACKed, then a512KiB data-before-kind malformed channel causes quota
   failure. Parent returns exact original first-frame bytes, committed mask1/missing2,
   no second frame/no possible effect. The caller's first bytes survive actual worker
   reap; releasing the result returns its completion group. No live AX/capture ran.
4. Real Retain/Replay path preserves the authored base and matches the independent
   full-source Snapshot at the new supplied ID. A subsequent Measure with16-byte
   output allowance returns ResourceLimit and zero partial frames, retaining the
   previously ACKed base lease. This is bounded encoding/publication refusal, NOT
   an allocator-OOM claim. All owned sessions/completion groups clean up.

The short source barrier was released immediately after result/hash comparison,
not held for receipt writing or further cases. Focused hostile_worker Clippy with
-D warnings and own rustfmt passed afterward; no runtime suite repeated. No process,
UI/browser/SDK/permission or operator signal policy was changed beyond owned children.

## Exact remaining phase-evidence boundary

Real guard/null behavior and decode-family quota exits are proved above. Retain/
Replay success and output quota refusal do not establish forced allocator exhaustion
inside every semantic phase. Current Document::from_json combines decode+semantic
validation while worker's phase marker is Decode; existing HostCompletion also does
not expose raw fatal phase. Guarded encoding streams into a preallocated buffer,
so output exhaustion is not evidence of System allocation failure there.
No fitted multiplier or arbitrary limit-tuning sweep was used to manufacture phase
coverage. Further validation/replay/admission allocation-failure claims require a
bounded actual source case plus trustworthy phase attribution (or a narrowly approved
private seam); Core/root must pin that exact additional scope. Unreachable cases
remain waiting_evidence, not passed or an environmental/tool failure. This does not
block saving the concrete completed checks or imply full H01/source acceptance.

Next coherent save set: crates/host/tests/hostile_worker.rs and this receipt only.
Root short Git grant pending. No other owner source/Cargo/fixture files are staged.

### Mandatory failure-obligation matrix

| Phase / obligation | Executed fact versus source boundary | Exact remaining consumer/case |
| --- | --- | --- |
| Installed setup guard | Forced2MiB fixed-input allocation against1MiB ordinary cap; ResourceLimit, no frame, confirmed reap and grant release | Closed bounded prerequisite on8edf546; not general setup/RSS acceptance |
| Typed decode | Both2MiB data-before-kind families force real guarded quota exits; kind-first rejects InvalidInput | Covered named core0.1/analysis0.2 families; no universal all-input upper coefficient |
| Rejection/error | Escaped string, long numeric and repeated-map cases reject normally; same worker then succeeds | OOM specifically while formatting rejection has not been isolated; do not infer it from ordinary InvalidInput |
| Semantic validation | Validation executes inside Document::from_json while phase remains Decode; normal canonical cases pass | Need one bounded source case plus trustworthy attribution separating validator scratch failure from decode; current completion lacks raw phase, so no forced-validation claim |
| Replay | Real normal replay plus the later predeclared480KiB case forces quota fatal with actual private Replay phase3; ACKed base bytes survive confirmed reap | Named Replay-pressure failure covered; do not promote it to every validation/all-input allocation path |
| Encoding/publication | Earlier16-byte OutputRequest refused in publish AFTER encoding; subsequent phase wave forces FixedOutput::write refusal with valid512KiB+1 query against512KiB slice, no partial frame and prior lease intact | Both bounded refusal boundaries covered; neither is claimed as allocator OOM |
| Encoding allocation | App-owned FixedOutput writes by checked slice copy; guarded_encode streams to_writer under PublicationGuard rather than constructing Value/Vec output | This source boundary is not a proof of all dependency allocations. No invented allocator OOM on an allocation-free writer; any claimed serde allocation failure requires an actual bounded reachable case |
| Retained admission | Subsequent phase wave uses predeclared32KiB retained allowance, ACKs small base, refuses canonical64KiB text candidate, then successfully replays old base and preserves caller lease | Retained-resource refusal/atomicity covered; not a GlobalAlloc-failure claim |
| System null | Bounded alloc/zeroed/realloc callbacks exercise true shared null branches and old/new charge preservation; exact status/reap | Injected-null handling closed; no claim of physical OS exhaustion |
| Overflow/underflow | Existing unchanged safe QuotaCounter boundary tests reused; real publication/configuration invariant fatalities tested | No undefined invalid deallocation or enormous allocation introduced merely to hit impossible-under-profile corruption branches |

The next writer/admission cases use existing API and bounded bytes, not a new
harness/public test command. Validation/replay-specific exhaustion needs only its
exact attribution/case handoff if existing APIs cannot identify the phase; it does
not reopen the whole mechanism or weaken expectations. Root coordinates the next
short saved-input barrier after this completed two-path step is saved.

## Existing-API phase refusal wave

After bdeb87ef8ed0294e0aea14ad645159a7455b79ec was pushed, root explicitly assigned
the two matrix cases above. Only hostile_worker.rs and this receipt changed.
Limits were fixed BEFORE the run: publication512KiB/input512KiB+1; retained32KiB/
candidate text64KiB; normal worker64MiB/ordinary63MiB. No post-failure adjustment.

Root/Core ACKed saved provider64cbec90de0bd1b46c8faea4efe01b2f9b3204c5, including
its separate fatal-lane race fix; supervisor pin
fbf181f340d8540a287b50ba5c1459e4b8d7ba654cd3eab7de3af6f034581ee5 matched.
All63 provider/manifest/fixture inputs matched that saved revision; own test was
the sole working input. Same tracked-source/six-fixture recipe as the previous
64-input wave, now pinned to64cbec9, before/after unchanged:
`eccadde8478ea57fabb1fe6767620d546f15d03b4ba69391c0da59c20eb63aa1`.
Unconnected native_binding.rs stayed excluded. The barrier was released immediately
after test/hash result, not held for receipt or further development.

Executed build session-worker, test hostile_worker filtered by phase_ with one test
thread, scoped hostile_worker Clippy -D warnings and own rustfmt, in the same owned
task-temp target.2 tests passed,0 failed/ignored;4 unchanged cases filtered out.
All affected checks passed; no full suite or earlier runtime wave repeated.

- Valid query plus trailing whitespace reaches the existing Validate path and its
  FixedOutput::write_all.512KiB+1 exceeds the configured output slice, so the writer
  refuses before copying/publishing; ResourceLimit, zero frames. Prior ACKed query
  bytes/root reservation stay intact; the same worker subsequently handles the
  ordinary query and cleans up. This differs from the earlier post-encode frame cap.
- A small base is actually retained and ACKed within the32KiB child retained domain.
  A new valid canonical candidate with64KiB name has measured owned size above that
  fixed allowance but encoded size below the frame cap. It is refused with
  ResourceLimit, zero frames and unchanged parent reservation/held bytes. Existing
  Replay then finds the old base and matches its independent full source, proving
  it was not evicted merely because the rejected candidate could not fit. Real
  shutdown returns session/completion counts to zero.

These are explicit quota/refusal outcomes, not System/GlobalAlloc exhaustion in
encoding or K01 admission. Source-backed allocation-free slice-copy boundary is
kept separate from unproved dependency scratch. Rejection/validation/replay-specific
allocation-failure obligations remain as the matrix states; no claim of full H01
or independent production source acceptance. Next save set: hostile_worker.rs and
this receipt only, under a new short root Git grant.

## Next predeclared Replay attribution case — prepared, not executed

Phase-refusal checkpoint ba3c372a48b4c104e5e224f16368ad88113de3ef is saved/pushed;
its saved64-input identity matches eccadde…63aa1 and the Git lease is released.
Root then authorized exactly one existing-API Replay pressure case, without a cap
sweep, new production seam or public HostCompletion change.

The test-local ProcessPlatform/OwnedProcess wrapper forwards the actual Darwin
predicate/spawn/poll/IO/termination/reap unchanged. read_fatal copies ONLY chunks
already returned to the real supervisor into a fixed64-byte record, then decodes
existing Control. It performs no second FD read, JSON parse, policy mutation or
fake phase/result. This is sufficient for an existing Replay phase marker; it does
not separate semantic validation hidden inside Document::from_json's Decode phase.

Fixed inputs before execution:480KiB unescaped text in one normal retained Snapshot,
compatible empty Delta, input2MiB/publication512KiB, ordinary quota4MiB−128KiB and
publication reserve1MiB. The fixed buffers plus retained text, decoded base text
and cloned candidate text exceed ordinary quota before other metadata. Actual
private fatal evidence, not this lower-bound reasoning alone, must establish phase.
Expected: successful base ACK, ResourceLimit with Replay class/phase3 and matching
operation, zero new committed frame, held original bytes surviving actual reap.
An earlier Decode failure or failed Retain prerequisite is reported unchanged;
the test may not tune the cap to obtain the desired result.

Ready command: build session-worker, then hostile_worker test filtered EXACTLY to
replay_clone_quota_has_private_phase_and_keeps_acked_bytes with one test thread,
plus affected Clippy/fmt. Before-run source recipe is the previous tracked-source/
manifest/toolchain/six-fixture set, recomputed on root's current saved provider;
new connected modules must be included. Runtime waits for a short Core ACK; no
freeze is held during this preparation. Only own hostile_worker.rs/receipt changed.

Prepared-case checkpoint note: only own rustfmt and scoped whitespace checks ran
for this new wrapper/case. No compiler or runtime pass is claimed against the
changing producer composition. Saved WIP is not acceptance or closure of the
remaining proof; wait for the exact new provider/ACK within this same packet.

## Fixed Replay pressure case — executed on saved default provider

Root explicitly continued the prepared eea3c9f case on coherent provider
c0abcffed886d0f33b34ddf3008a902b8e024676 with a fresh short source ACK.
No test/limit/shape edit occurred before this execution. Both worker build and
test used --no-default-features; no Web producer/collector or Native helper ran.
Existing host dev dependency tungstenite was compiled for the selected test target;
this is not evidence that feature=web was enabled or its source executed.

Executed in the same run-owned task-temp target, once:

```sh
cargo +1.96.0 build --locked --offline -p uiblueprint-host --no-default-features --bin session-worker --target-dir <task-temp>/target
cargo +1.96.0 test --locked --offline -p uiblueprint-host --no-default-features --test hostile_worker replay_clone_quota_has_private_phase_and_keeps_acked_bytes --target-dir <task-temp>/target -- --exact --test-threads=1
cargo +1.96.0 clippy --locked --offline -p uiblueprint-host --no-default-features --test hostile_worker --target-dir <task-temp>/target -- -D warnings
```

One exact test passed, zero failed/ignored, six unrelated tests filtered out;
test runtime0.53s. Worker build, scoped Clippy and own rustfmt passed. The fixed
480KiB base was retained/ACKed, the empty compatible delta reached an actual
ResourceLimit fatal with Replay class, matching operation and private flags=3.
No new result frame was committed. Original canonical bytes remained available
after real shutdown/reap; session/grant/completion ownership returned as asserted.
The wrapper copied only bytes already returned by the real read_fatal call; it
did not inject/delay status, consume the descriptor twice or change HostCompletion.
Thus this is observed Replay-phase allocation refusal under clone pressure,
not an inferred Decode failure or a fitted cap. Ordinary4MiB−128KiB and all other
predeclared sizes remained unchanged. It does not isolate an individual allocator
call site or prove all semantic-validation/rejection failure paths.

Before/after71-input hash, identical:
`c71cfa89a1875cd5823f8320e023636d12fe86d5b180d697df3417a4bf7cff40`.
All71 current inputs matched saved c0abcff BEFORE running, including the already
saved test. Hash recipe is compact sorted JSON path->SHA256 then SHA256. Exact set:
root Cargo.toml/Cargo.lock/rust-toolchain.toml; Cargo.toml and tracked src/**/*.rs
for host/schema/engine/plugin-api atc0abcff, EXCLUDING cfg(web) host web_config.rs
and worker_web.rs; host tests/hostile_worker.rs; the same six normal fixtures above;
workspace resolution manifests plugins/web/Cargo.toml, crates/cli/Cargo.toml and
crates/export/Cargo.toml. Newly connected native_binding/native_broker/worker_native/
worker_observation modules are included; old unconnected-module exclusions are not
silently reused. No Web production source or unrelated test source is claimed covered.

Source hold was released immediately after result/hash comparison. No new runtime
wave or source scope followed. This update only records completed proof: the test
was already saved and is unchanged. Next checkpoint is this receipt alone.
Separate validation/rejection attribution and full H01/source/live acceptance stay
open according to the matrix; successful normal/refusal paths retain their scope.
