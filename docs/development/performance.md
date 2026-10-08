# Q02 fixed D06 evaluation

Status: all four frozen Web numeric rows meet their gates on the qualified host.
Single-control semantic/geometry on9d715ee:100 warm each,p952.846/2.899ms≤20ms.
Full2-document/97-node capture on accepted a7c0416:20cold, attach+response
p9537.508ms≤50ms and process-cold264.900ms≤500ms. Full warm100,p9521.504ms is
supplemental, without a20ms gate. Zero cohort failures; all outliers retained.
Ordinary scope source is unchanged9d715ee→a7c0416, so prior evidence is reused,
not relabelled as rerun. Required Web Rust normalization/formatting is separately
measured by exact-owner saved-input diagnostics below. Native remains open; opaque
SDK/syscall/cache-peak counters remain unavailable without invented zeroes.
**No overall D06/P7 acceptance.** Historical8e3dba2 geometry2.688ms remains recorded.
2026-10-09 Native one-shot read-only preflight on accepted7709067 succeeded while
the retained fixture stayed inactive/off-screen:78 canonical nodes and1100×1050
pixels. Current source facts match the after witness, but original75-node workload
does not: extra controls and Open B x displacement−89.5pt. Exact AX tree invariance
also differs by one anonymous group; no Native timing or setup was performed.
Authority is [D06@1](../specs/development/decisions/d06-performance.md),
[PERFORMANCE@1](../specs/acceptance/performance.md) and the approved
[Q02 packet](../plans/ui-blueprint/packets/Q02-performance.md). Gates/quotas are
unchanged. [Current receipt](../plans/ui-blueprint/receipts/Q02-performance.md)
records pins, checks and resource dependencies.

## Separate Rust stages (diagnostic, not another latency cohort)

[rust-stages.py](../../tests/bridges/web/rust-stages.py) creates an immutable a7c0416
copy under an explicit system-temp destination. Its [input helper](../../tests/bridges/web/rust-stages-inputs.cjs)
performs only two public F01 Observes and saves the shipping read-node.js/AX inputs
at the same unchanged checkpoint. Full input reuses the original measured raw/
canonical pair. Every input and exact test-only delta is hashed before replay.

[Normalization test](../../tests/bridges/web/rust-stages-normalize.rs) is included
as a test child of the existing acquire owner, reusing its inert-peer helper solely
to own an unused client. It calls actual `Collector::normalize` and
`snapshot_normalize::snapshot`, not a copied normalizer. The entire normalization
call includes its own schema validation, relations/focus/component or native-table/
privacy work. `bounded_document` measures the real sizing-serialization pass.
Normalization runs in the library test process with System allocator; it does not
claim the worker's allocator overhead or a live collection cost.

[Worker format test](../../tests/bridges/web/rust-stages-format.rs) calls real
`FixedOutput` + serde_json::to_writer with actual GuardedAllocator/PublicationGuard.
It measures encoding and the publication guard, excluding input decode/validation,
buffer setup, syscall/ACK and output comparison. Normalization requires exact saved
Snapshot/Document equality; encoding requires byte-for-byte original canonical output.
No original production body, Cargo/features/dependency, public API or installation
changes. Only four conditional append-only test inclusions in task-temp.

Build via the helper's `prepare`, `build`, `run` actions, always with `--output`
under system temp. Prepare also takes `--pins` (saved accepted consumer manifest),
`--full-sample` and `--full-raw` from the original cohort; set the existing
S01_WEB_PLAYWRIGHT_CORE. Build is release/locked/offline for only the library test
and explicitly selected worker binary test. Run uses20 fresh processes×1 invocation
and100 invocations in one process per scope/stage; no warm-up/outlier is discarded.
Process startup and decode are OUTSIDE these stage spans: these are not cold outer
response numbers. Unique reports preserve failed runs; no new stage threshold.
The receipt contains all p50/p95/counts, exact input/delta/binary hashes and limits.
The mandatory Web stage-report gap is closed by these explicit diagnostics; do not
sum their percentiles or substitute them for the already accepted live latency rows.

## Recipe and timing boundary

[Rust consumer](../../crates/host/tests/performance.rs) is an ignored opt-in test
using the actual RuntimeHost, guarded worker and canonical Frame/Commit/ACK.
Each stdin line explicitly requests one Observe; idle stdin does no collection.
Its timer includes caller request decode/encode, configuration where requested,
dispatch, platform acquisition, worker normalization/validation/serialization,
transport and ACK. The driver's outer timer additionally includes canonical
stdout delivery. Source Observation intervals retain their own clock and units;
they must not be summed as an invented independent stage breakdown.

Build from a saved source archive in system temp, overlaying only the saved Q02
test. Use an isolated CARGO_TARGET_DIR and the pinned toolchain. No branch/worktree.
Before timing, rebuild against the final Q01 functional pin, record the source
tree and caller/worker/helper SHA256, profile and host load; finish all compilation.

```sh
cargo test --locked --offline -p uiblueprint-host --features web \
  --test performance --release --no-run --message-format=json
node tests/bridges/web/performance.cjs --check
node --check tests/bridges/native/performance.cjs
```

These are templates inside the task-temp archive, not commands to use the shared
repository target directory. Select the executable from Cargo's matching artifact.
Default test execution skips live work. No Cargo/dependency/source policy changes.

The [Web driver](../../tests/bridges/web/performance.cjs) requires explicit
`--run-authorized`, UIB_Q02_ALLOW=1, reviewed UIB_Q02_FUNCTIONAL_PIN, the saved
UIB_Q02_EXECUTABLE and matching UIB_Q02_EXECUTABLE_SHA256, new system-temp
UIB_Q02_OUTPUT and the established S01_WEB_PLAYWRIGHT_CORE. It launches only owned
headless F01 at800×600/DPR1 with the frozen Chromium145.0.7632.6/Playwright1.58.2/
Node24.15.0. No real site/profile, screenshots, cadence or background UI collection.
First run with `--preflight`: actual CDP protocol,2-document/97-node fixture shape,
raw addressed AX fields (including the reported focusable extension), independent
rect literals and changed/restored data. Documents additionally compares every
native fact through the saved W06 oracle:97nodes/2documents/1102facts/19text boxes.
Then supply UIB_Q02_PREFLIGHT pointing to that report. Only comparable workloads
with passed quality/freshness execute series, using the identical saved executable.
The original8e3dba2 semantic gap is retained historically; W06's extension must pass
new actual preflight rather than be assumed correct from its representation.
Current executable loop selects only Documents. It ran after terminal committed
Q01 Documents acceptance/release a1cae1a on a7c0416. Existing9d715ee
semantic/geometry results are reused: the source delta changes Documents privacy
only. Single-control rows are not rerun merely to populate this report.
Stdout uses streaming UTF-8 decoding so large canonical payloads preserve native
text when a multibyte character crosses a pipe-chunk boundary.
Each semantic/geometry cohort has20 process-cold single-control samples and one
reused session containing initial response plus100 explicit fresh warm requests.
First Observe resolves #left; subsequent calls reuse the observed ref and attachment.
Before the100 warm samples, a separate explicit fixture stimulus changes requested
name/width on that same held ref; a fresh response must report it, then another
response verifies restored baseline. These two checks are retained separately and
excluded from the latency cohort. Observation itself remains read-only.
Cold single-control is **supplemental**, never the D06 full-fixture cold gate.
The Documents cohort uses the actual explicit WebSelection::Documents API and
trusted two-Surface descriptor. Its128nodes/depth16/512KiB/2s failure bounds are the
W06 whole-document profile within unchanged D05, not changed latency thresholds.
Twenty process-cold full captures test50ms attach+response and500ms overall;100
reused-session full captures are reported separately without inventing a warm-full
threshold. Cold includes caller CDP setup, frame/document binding and configuration:
document IDs come from bounded metadata calls, never an untimed DOMSnapshot.
The raw comparison snapshot is requested only AFTER the timed candidate response
on the controlled unchanged checkpoint. Local oracle checking is outside response
latency and never becomes background UI work. First/startup failures remain rows;
quality failure never overwrites an already measured response time.
Partial source coverage remains partial. Requested known values are independently
checked against authored R01 literals; unknown/field loss cannot pass quality.
Semantic fidelity now retains raw focusable through the registered extension,
separately from Focused; Q01 acceptance and the new actual preflight close that
earlier gap. Numerical timing alone was never used as its quality proof.

Failures remain in samples; missing duration makes percentile unavailable. A broken
session stops and separately labels unrun requests, never silently restarts/censors.
Report median p50, nearest-rank p95, attempted and unrun counts, outer vs ACK intervals,
canonical bytes/node counts and source intervals. Cache/allocation high-water,
independent transport/normalization CPU, syscalls and model tokens are unavailable
until measured, never zero or estimated from bytes. No full/delta live equivalence
is inferred; that remains the controlled checkpoint proof accepted by Q01.
Available HostDomain parent-owned bytes, retained reservations, sessions/completion
leases and poison/abandonment flags are sampled at each ACK and after cleanup.
They are attributed reservations/owned layouts, not worker usage or SDK/RSS peaks.

## Native comparability before timing

The current authorized continuation uses ONLY Q01's already-running retained F02
after fresh process incarnation/executable/window/identity validation. Historical
manifest state is provenance, never current authority. No launch, activation,
Focus/Compare/Snapshot/reset/resize/close or historical bundle is allowed here.
[ReadonlyFacts.swift](../../tests/bridges/native/ReadonlyFacts.swift) is a bounded
nonvisual witness: same existing admissions/window resolution, independent scalar
AX reads, protected Value exclusion,160/depth9/1s and fixed512KiB output. It reads
process/CG metadata before and after; no setters or capture. The accepted shipping
helper/host performs the single canonical AX/capture operation. Raw before/after
properties/tree are compared offline with all canonical source properties.

The driver additionally requires UIB_Q02_WITNESS(+_SHA256) and the trusted retained
UIB_Q02_FIXTURE_EXECUTABLE. Context/report distinguishes current source fidelity,
historical baseline differences and strict tree invariance. An exact structural
insertion may explain index shifts for comparison only; it never creates persistent
identity or action refs. Mismatches are retained, not repaired by app setup or retry.
Current source quality/known fields can pass while the original workload remains
incompatible. See the current receipt for the one actual run and scoped result.

[Native preflight](../../tests/bridges/native/performance.cjs) takes the trusted
retained A manifest as provenance, baseline external JSON and saved helper, without starting,
activating or changing the app. `--baseline-only` is offline. `--preflight-runtime`
requires the same explicit functional/resource activation as Web, plus
UIB_Q02_MANIFEST, UIB_Q02_NATIVE_BASELINE, UIB_Q02_HELPER and its SHA256.
It requests all supported full-window fields,160/depth9,512KiB/channel, original
profile and an explicit3s parent deadline. AX/capture channel bounds remain existing
implementation bounds. `window-ax` with registered per-channel collection preserves
full-window AX plus isolated pixels. Its output is a comparability matrix, not D06
timing; even matching counts require field/value reconciliation before series.

Images/staging and their containing directories stay in system temp without agent
deletion. The helper creates a new private artifact directory; no existing image
is overwritten. The consumer can reconfigure the existing Native binding between
explicit calls after owned helpers reap, so later sampling can give each request
a distinct image destination while reusing the guarded Rust attachment. The current
preflight does not implement or claim completed Native statistical series.

## Established preparation discrepancies (original8e3dba2)

* F01 HTML/JS/server/oracle remain byte-identical to a8368076; the existing rooted
  collector refuses iframe boundaries. Thus the2-document/97-node whole-fixture
  cold DOMSnapshot baseline has no equivalent current candidate route. Measuring
  only #left or only the main document cannot close its50/500ms gates.
* Current F02 adds a Protected input status and a Result HStack relative to9a88b12b;
  current node/field/pixel equivalence is not established. Runtime preflight waits
  Q01's functional pin and explicit resource release. Never remove that functionality
  or lower160/depth9/pixel/field coverage to force comparability.
* Baseline75-node AX includes one known AXTitle. Current WindowAX explicitly reads
  AXDescription for accessibility_name/description and preserves AXIdentifier/
  AXSubrole extensions, but did not request/publish AXTitle at that historical pin.
  Accepted N04 repair7709067 and current actual preflight now preserve Title through
  its own bounded batch/extension; fixture equivalence remains a separate question.

These facts do not authorize new product collection features or revised D06 gates.
After actual preflight, return the smallest necessary compatibility decision within
Q02's packet before evaluating an incompatible workload.
