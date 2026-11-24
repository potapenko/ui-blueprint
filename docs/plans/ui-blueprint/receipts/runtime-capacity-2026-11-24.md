# Complete large Mac runtime observations

Owner: [capacity implementation](codex://threads/01a20f35-c7e4-7490-9d62-2e06b6357999).
Review/architecture: [SDK owner](codex://threads/01a20a46-e5d0-7710-a5ce-9a93d8b82f02).
Status: full native capacity acceptance passed; graph-validation performance delta
and one live combined Word query remain under review/verification.

## Authority, basis and scope

New direct user authority in SDK chat01a20a46 on2026-11-24: finish full large-window
export and Word/Next generally; mobile later; remove the agent-selected restriction
(“мне похер на это ограничение… хоть гигабайт”). Its finite handoff explicitly
selects1048576 source/native nodes and1GiB artifacts, not another diagnosis-only
release. This supersedes the earlier no-cap-change stop recorded in
[the diagnosis receipt](runtime-source-limit-2026-11-24.md). Current branch master,
base a2fff6241f8723e8b0c697115bf341267f9f9e45; no branch/worktree or other app edit.

Reuse prior complete Runtime@7 closure, implementation/QA/product-truth governance.
Register Runtime@8 RUNTIME.CAPACITY/registry95 before capacity source edits.
Measured old Rust full-return cost justified the SDK owner's exact additional
lane: CLI arguments/main/output + existing focused integration tests. Read RUST.md,
DEV.RUST@5 and ANALYSIS@2 → TYPES@1/VALIDATION@1 closure; register ANALYSIS@3,
CLI@22/registry96 before Rust edits. No schema, engine, dependency or arithmetic change.

Contract requirement: complete fresh source tree with unique selected type and
original provenance/privacy. Observed implementation refused at16385 and16MiB;
old measurement JSON repeated the full source then Python discarded it. Approved
Evolve: larger resource admission plus optional typed result-only CLI envelope.
Write set: runtime Collector/Encoding, caller, Swift/Python/live tests, the agreed
four CLI files, runtime/analysis/CLI routes and README/skill references, this receipt.
Packaging/version/install/release remain with01a20929. Real windows/app builds and
acceptance belong to native owner01a20a56 and Random01a20844. No new chats/agents.

Protected: depth512/type65536/control guards, metadata16KiB/queue32, opt-in Debug,
private session/capabilities/window bindings, main UpdateGroup, field allowlist,
full source links/ancestors and unknowns, cancellation/no automatic retry, legacy
successful canonical documents. Node diagnostics admit old16384 and new1048576
ceilings; matching new SDK/caller is required for larger artifacts.

## Mechanism and measured critical path

SDK source and native counters now each allow1048576; canonical and discovery
admission allow1073741824bytes. Rust already accepts caller-supplied byte budgets;
there was no second16KiB/16MiB canonical admission cap to remove there.

An actual1100-row owned SwiftUI fixture gives17995 source nodes and41782430bytes
of canonical JSON. Initial full capture1.832s, validation0.583s; one full live
measurement4.832s. Old full Rust JSON on the SAME saved input returned42135415bytes
in2.703s plus0.260s Python decode versus compact text1.771s. These are individual
samples, not a p95 or a universal performance guarantee.

`measure --json --result-only` now serializes borrowed typed fields into a separate
CLI envelope1.0.0: source snapshot id/revision/context/observations/coverage,
query/evaluation/result. It retains clocks/times/freshness/coverage/evidence without
nodes. Complete original input and result validation stay. Old `--json` bytes on
that saved input are exactly identical between installed0.10.1 and candidate
(SHA256 ed3f1ded56951430ddbd2bb11dab3e047e9697600ab42236f5fa74025ab23b3e).
New result-only response3130bytes preserves exact values/evidence and metadata.
Second same-input samples: old JSON2.891s, new default JSON2.865s, result-only2.212s;
other owned fixture checks were running, so these are not isolated benchmarks.
Python checks Rust help support BEFORE source dispatch; no expensive fallback.

Eight concurrent large captures under the former5s default produced four
request_deadline_source_outcome_unknown refusals. All four later completed as
cancelled without artifacts; only their terminal owned files were acknowledged.
The new default30s/explicit maximum120s covers this measured serialized queue;
explicit shorter cancellation and source non-preemption remain. The full
capture-plus-measure shares one deadline with no hidden10s subprocess cap.
No1GiB artifact was generated merely to test a constant.

## Verification

- Swift13/13 with explicit profile29, copied internal XCTest bundle. Real getter
  collection at1048576 nodes passes;1048577 refuses. Type byte/control boundaries
  unchanged. Larger-than16MiB canonical and discovery encodings pass.
- Python14/14: old/new diagnostic profiles, malformed/private data refusal,
  result-only bundle support, binding/hash/cleanup and legacy request shape.
- Rust analysis16/16 and existing CLI binary21/21; focused Clippy all targets,
  fmt and diff checks pass. New mode preserves known/unknown result and metadata,
  exact f64 bits, malformed graph/wrong binding refusals, pre-input flag refusal,
  exact output budget including newline, and zero partial stdout on failure.
- Full17995-node/41.8MB capture and independent canonical validation pass. Rust
  measures an actual Color descendant selected through the unique container's
  source links FROM THE FULL Snapshot: width120pt. Scoped capture retains357nodes.
  Fresh owned transitions produce120→180→120pt; no cached geometry substitution.
- Final live full measurement3.874s; full capture1.936s, validation0.611s.
  Eight concurrent full captures all succeed with distinct observations and
  complete17995-node artifacts; longest10.421s. Canary absent. Cancellation and
  invalid input still publish no Snapshot.
- Updated scoped/discovery regression chains pass: full7595-node17.6MB export,
  explicit missing/ambiguous selectors, fresh state, four scoped concurrent reads,
  stale/cancel refusal, complete17995-node discovery. Actual0.10.1 caller still
  reads small411-node whole and357-node scoped snapshots from the new SDK.

The fixture's six-minute test lifetime ended before one suite started; its first
windows request correctly returned target_unavailable. Relaunched the same owned
binary with a fresh private binding; the complete later chain above passed.
No consumer process/window or private descriptor was used here.

Evidence: `/Volumes/Develop-2TB/Develop/tmp/uib-large-runtime-01a20f35-n29p8q2m/evidence`,
especially capacity/summary.json, subtree/summary.json, discovery/summary.json,
result-only-timing.json and concurrency-eight-baseline.json. Internal app/tests and
candidate Rust bundle: `/tmp/uib-large-runtime-01a20f35-kf70_nls`; these are task-owned
verification artifacts, not an installed release. Evidence retained through review
and actual consumer acceptance; builds/runtime cleaned only after consumers exit.

## Independent review delta

SDK owner reviewed1598658 capacity/compatibility/privacy and Rust borrowed result
projection without blocking findings there; independently reran Python14/14 and
checked saved timings/legacy SHA. One P2: slow Rust-help preflight could consume
the overall deadline while source request restarted the full duration. Correction
passes the same absolute deadline through preflight/request/measurement and checks
it again before source publication, including endpoint setup. Focused fake-clock
tests prove no dispatch after expiry and exact remaining budget. Python17/17 and
diff checks pass. SDK/Rust and their existing runtime evidence are unchanged;
SDK owner independently accepted7c764cda after Python17/17/diff checks. No redundant
SDK build/live matrix was repeated for that caller correction.

## Actual native capacity acceptance — 2026-11-25

Native owner01a20a56 ran one Debug binary0adf35edc7b48c26577377e47caedb0fa4da9a2a
with SDK7c764cda and matching candidate Rust/caller. Full Search, resize, Practice,
fresh post-close and original filled Word/Next all passed. Entire original
Snapshots validated with complete exposed layout coverage, omitted0/unknown0.
Practice appears once while open and is absent in the fresh closed graph.

| Phase | Nodes | Bytes | Rust size, pt |
| --- | ---: | ---: | --- |
| Search | 11844 | 28384535 | 1920×921 |
| Resize | 13191 | 31642245 | 1900×901 |
| Practice | 13839 | 33132513 | 560×355 |
| Closed | 13191 | 31642245 | 1900×901 |
| Word/Next | 34884 | 83808614 | 576×151 |

Word preserves all20 actual Vocabulary rows, Replay/Next/Enter; no markers, trimmed
lists, fixture state or subtree fallback. The closed Random feature chat was not
restarted to do product work; the SDK coordinator assigned this separate geometry
QA to the existing native owner. Exact source spaces/observations, full files and
consumer report are retained by that owner at
`/Volumes/Develop-2TB/Develop/tmp/playphraseme-blueprint-capacity-zyf80ld9/retained`.
Read result.md/summary.json for provenance and limitations, not a new app capture.

Word used default30 with no retry: capture30.8576s, source18.9999s, validation4.3296s,
separate saved width/height20.2028/20.2634s, total76.1975s. The external supervisor40s
did not change caller admission. Live combined measure was not run in that chain.
Full capacity passes stand independently of the remaining latency verification.

## Graph-validation performance correction

The next finite SDK-owner-authorized lane is only schema validation/graph.rs,
focused graph tests and this receipt. Mode: behavior-preserving; no API/schema,
Engine or semantic contract change. Read current graph_refs in full, including
its use in validate_delta. A temporary read-only optimized probe on the exact
saved Word Snapshot measured file read0.204s, JSON decode0.284s, one full semantic
validation3.986s. Repeated subtree traversals/cloned keys dominated, not JSON or
geometry arithmetic. All existing validation calls remain required.

Replace repeated ancestor walks with one iterative DFS using borrowed keys and
visiting/finished sets. Each edge is checked once; shared DAG children are valid,
back-edges/self/disconnected cycles and dangling keys retain DanglingReference.
Duplicate identities remain checked first, relations/focus afterward. No recursion,
untrusted fast flag, skipped branch, typestate framework or validation bypass.

Focused tests: all512 directed three-node graphs agree with independent Kahn
indegree elimination; diamond/multiple-parent/reordered nodes and a10000-node
chain pass; detached cycles, duplicates and dangling edges refuse. Delta cyclic
upserts, dangling upserts/removals, repaired-parent removal and remove/upsert
conflict retain their outcomes. Schema graph4/goldens4(including126 cases)/analysis12/
channel2, CLI analysis16/binary21 and affected Clippy all-targets pass; fmt/diff pass.

On the unchanged real Word input: one semantic validation now0.041319s;
validator CLI0.395607s; width0.515100s and height0.548148s. Both entire result-only
responses are exactly equal to the native owner's earlier responses, including
query/evaluation/context/observations/coverage/result/evidence and576×151pt.
These are bounded individual measurements, not a universal latency guarantee.
Evidence: word-validation-cost.txt, word-validation-cost-new.txt and
word-graph-performance.json in the implementation owner's existing evidence area.

## Remaining acceptance

Finite review of the Schema delta, then one native-owned live combined Word
measurement using the SAME retained app/SDK and updated matching Rust/caller.
The supported explicit --timeout120 is appropriate for this heavy full-window
query; default30 is not claimed to cover source capture plus measurement here.
Do not repeat the five accepted UI cases or rebuild unchanged SwiftUI SDK merely
for the Rust graph check. Native original data/app and both candidate bundles
remain retained until that consumer and the release owner finish. Packaging,
VERSION and publication stay with01a20929 after the accepted final pin.
