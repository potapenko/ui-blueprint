# W01 rooted selection source handoff

## G06 combined Director request: exact limit — 2026-10-08

Classification: diagnostic / justified resource refusal, not a shipping defect.
The full request with role/accessibility_name plus layout_bounds/hit_region/
visible_region reaches `collector::observe::bounded_document` and exceeds its
65536-byte canonical ChannelResponse budget before publication. The counting
serializer accepted65530 bytes; the next write would exceed the cap.65530 is
the accepted prefix count, NOT the full required size (which remains unknown).
DOM+AX acquisition and normalization already returned; this refusal is not an AX
lookup failure, wrong root, method quota, timeout or source-app launch failure.

One source-backed diagnostic run03:30:56.747–03:30:59.200UTC on the same actual
local Director setup reproduced public `observe_invalid_or_limit`, exit2, zero
stdout,25 stderr bytes,111.24ms including worker lifetime. A temporary Web-only
diagnostic overlay wrote exactly one fixed record:
`{"code":"canonical_output","observed":65530,"cap":65536}`.
Instrumentation covered method admission/use, reply reservation/total, AX response
cardinality/properties and canonical output refusals. Only fixed literal codes
and two integers could be written to its task-temp file, capped4096B; no raw UI,
protocol, stderr or exception payload. It changed no refusal predicate, limit,
requested field, publication or cleanup behavior and is not shipped.

Selected contracts are reused from G05 with no drift: CLI@7 OBSERVE permits
invalid/limit exit2 and forbids partial publication; D05@4 and WORK@1 do not promise
that every node/field combination fits an independently bounded response. The
existing counting boundary enforces that contract. No product fix, larger quota,
field suppression or partial-as-complete result is justified by this finding.
The combined names/roles+geometry request is still unavailable for this component
at64KiB. The passing geometry-only result remains separate and unchanged; no CSS
discrepancy or general B03/P7 qualification was attempted.

Persistent write set: this receipt and `tests/bridges/web/guarded-live.cjs` only.
The latter adds `UIB_WEB_LIVE_CASE=director_semantics` using the same Director
setup and original five fields. It intentionally still reports failure on this
limit; it does not turn the result green. Use the G05 command/build preparation
below with that mode to reproduce the public refusal. Exact diagnostic stage
came from the temporary instrumented build, not a new public diagnostic API.
Existing `director` mode retains its original three fields and geometry chain.

Saved build basea9c2ff2ef42842b70333afc43543369354f447db; relevant Web/Core/spec
bytes equal cf9c071. Temporary overlay touched only Web collector mod.rs,
observe.rs,io.rs,acquire.rs. Locked/offline CLI+worker build and launcher syntax/
activation-refusal/diff checks passed. No Rust product source changed and no
unrelated test suite or unchanged live scenario was repeated.
Diagnostic CLI SHA256f0f4342da07660032652e1024b5defffd95eb6861ed8f645888f659b71a64008;
worker b57f2d37cf6310ce37367720080324e5518a0dd83b70742c92e9df8c97f8e1d0;
launcher de816751f1bef317e4410041c5cdb5d98faf0e6f0a78578e349a3c9d771962c0.
Report SHA2566d209beeee26a5c316da10bbc913cbdbb22993a9c0fc6364e849d8db32a6ccd1;
diagnostic a136f7a1a12462c8c57faa61d4ec27c1aedb36877f57302bd611b77658602b4d.

Browser/context/driver/profile and CLI cleanup confirmed; exact worker executable
inventory empty after the call. Site page/console and first-party request/error
counts all0. `worker_sessions=unconfirmed` is preserved separately: the harness's
successful-pipeline marker was never reached, and no internal host-session counter
was exposed. It is not recast as confirmed session telemetry. No owned process
remains; no source app/service, user browser or image was created/removed.
Post-use178 unmodified saved inputs, both binaries and launcher matched; four-file
diagnostic-overlay fingerprint2a7518327f5cccb9e095e24fd66f6cf26e88dce15155f74c18b49eaf6f92fd00.
Consumed run output3files and temporary diagnostic source/build2777files were
removed with absence verified. These hashes are historical identities, not downloads.

## G05 real PlayPhrase.me Director geometry — 2026-10-08

Finite verification delivered: existing public Observe → design Inspect → Rust
Measure on the real local Director popover. Immediate consumer: AI geometry work
and first-use instructions. No product contract delta, collector/engine/CLI/site
source change, action executor, login/profile reuse or source-app launch/build.
Write set: this receipt and `tests/bridges/web/guarded-live.cjs` only.

Basis: AGENTS → execution/worker packet → registry19 → PRODUCT-ROUTES@1 →
CLI@7 CONTENT/OBSERVE/INSPECT, ANALYSIS@2 SCOPE/CLI with TYPES/VALIDATION@1,
GEOMETRY/PROJECTIONS/MODEL/EXCHANGE/IDENTITY/BOUNDARIES/PRIVACY/LIFECYCLE@1;
Web PILOTS and explicit common D02@2/D04@1/D05@4/MEMORY@2/WORK@1/D07@5
closure. No semantic drift. Native/export/actions/mobile/full P7 excluded.
Reference route: named site's AGENTS → specs/discovery-playback → Clip Search
surface-shell/filter-controls/filter-context; source picker_view.cljs and
suggestions/view.cljs plus the named desktop QA case establish selectors and
`http://localhost:3000/#/clip-search?language=en`, not measured expectations.
The setup uses the packet's explicit isolated-headless permission.

HTTP preflight200; clean Chromium145.0.7632.6, Node24.15.0, Playwright1.58.2,
darwin arm64/27.0.0. New private context1280×900/DPR1, empty query, ordinary
trigger opening only. Same bound target/frame/document and actual backend root;
no synthetic ref, portal assumption, selection application or form submission.
Actual run03:22:48.066–03:22:50.840UTC passed13 public CLI calls: Observe4
(partial), Inspect0,10 known Measure0 and1 unknown Measure4. One original
25944B ChannelResponse supplied directly to every analysis command. Each result
preserves its full Snapshot exactly; read invariance passed before/after collection
and again after measurement. No independent acceptance or p95 claim.

All bounds below are reported CSSOM `layout_bounds`, same viewport/top_left
`css_px` space, local_only transform. They are real runtime values, not the
source's 8px/258px/10px declarations. Binary-to-site-source correspondence and
the cause of differing declaration/measurement values were not diagnosed.

| Part | Actual x,y,width,height css_px | Canonical web.dom key in this observation |
| --- | --- | --- |
| Wrapper |25.09375,278.109375,178.703125,42.90625|544|
| Trigger |25.09375,292.40625,178.703125,28.609375|17|
| Popup |25.09375,327.03125,194.25,85.9375|548|
| Input |33.609375,352.84375,177.21875,25.59375|18|
| List |33.609375,384.453125,177.21875,20|552|

Rust measured trigger-bottom→popup-top and input-bottom→list-top gaps6.015625;
wrapper→popup, trigger→popup and input→list left offsets0. Input within popup:
left8.515625/top25.8125/right8.515625/bottom34.53125. Insets are not padding.
Available option count was0 in this empty-query state; no option bounds/alignment
are fabricated. Clipping-aware width returns `unknown_property`; viewport overflow
is unmeasured because no canonical viewport geometry node was collected.

Snapshot `web-snapshot:1:1`, Observation `web.dom:1:1`, space/surface
`88FAE71C0A3D4A04B730A5681A94519B`; these identities belong only to this run.
16DOM/0AX nodes, coverage partial, omitted_count unknown, unknown_count33;
freshness current/live_read, consistency unknown/sequential-reads-not-atomic.
Limits unchanged32nodes/depth8/64KiB/250ms,256visited, collector16nodes and
120s overall. First broader request included role/accessibility_name and returned
exit2, zero stdout,25 stderr bytes in118.35ms. Its exact underlying limit was not
established. The only retry removed those unnecessary fields; no cap was raised,
scope widened or failure relabelled. It requested layout_bounds/hit_region/
visible_region. Public refusal diagnostics are now retained only by fixed-code
allowlist, never raw stderr. Broad AX+geometry remains an explicit limitation.

Single-run CLI acquisition119.92ms including worker lifetime; Inspect23.63ms;
local Measure24.59–26.34ms. Browser page/console errors and failed/error first-party
requests all0. Owned worker inventories empty after every CLI; context/driver/
browser/profile cleanup confirmed; no fixture server or source service was created.
Initial failed run also left no worker at its exact executable path; its harness
cleanup status stayed unconfirmed because the successful pipeline marker was absent.

Saved Rust base3a73ac792aa1effd570cfcf7b14609261b12cafb, exported to system temp
before build; concurrent Native WIP excluded. Locked/offline web CLI+worker build,
launcher syntax, activation refusal and diff checks passed; no old fixture suite.
CLI SHA256f01096d21fac591ded9d008083e28ca90aceac41c619113d765c222f964b98b2;
worker726557f3c66372f6bf37c920c5763d9c4032822e69e32fe5586447f3cd08d44d;
launcher b7ecd65d7ac1081076db17fcd4b49e7f9a2f1aaeb24f30882e15aaff5f01d35e.
Consumed Observe SHA2567d11e4a1152361ffa8b1eb147d3eb4a89e2c0dc5eb11dd764348c8eaf95adda3;
report350be6f43b5919fe9c1d0587b5613e1e8f81773fe98df3515765b3f7cb7ecb1c.
Post-use182 saved build-input files, both binaries and launcher hash matched.
No images were created. After immediate consumption, both run-owned output sets
(3+27 non-image files) and the separate2770-file temporary source/build tree were
removed with absence verified. No owned browser/worker/runtime resources remain.
Hashes identify historical bytes, not available downloads.

Runnable current command from this checkout (requires the existing local site;
does not start it; system-temp output must be consumed then removed):

```sh
UIB_G05_BUILD=$(mktemp -d "${TMPDIR:-/tmp}/uib-g05-web.XXXXXX")
git archive HEAD Cargo.toml Cargo.lock rust-toolchain.toml crates plugins/web | tar -xf - -C "$UIB_G05_BUILD"
cargo build --locked --offline --manifest-path "$UIB_G05_BUILD/Cargo.toml" -p uiblueprint-cli -p uiblueprint-host --features uiblueprint-cli/web,uiblueprint-host/web --bin uiblueprint --bin session-worker
export UIB_WEB_LIVE_TEST="$UIB_G05_BUILD/target/debug/uiblueprint"
export UIB_WEB_LIVE_WORKER="$UIB_G05_BUILD/target/debug/session-worker"
export UIB_WEB_LIVE_TEST_SHA256=$(shasum -a 256 "$UIB_WEB_LIVE_TEST" | cut -d ' ' -f 1)
export UIB_WEB_LIVE_WORKER_SHA256=$(shasum -a 256 "$UIB_WEB_LIVE_WORKER" | cut -d ' ' -f 1)
export UIB_WEB_LIVE_EVIDENCE=$(python3 -c 'import os,tempfile,uuid; print(os.path.join(os.path.realpath(tempfile.gettempdir()),str(uuid.uuid4())))')
S01_WEB_PLAYWRIGHT_CORE=/Users/eugenepotapenko/.npm/_npx/f88013d20c39cb98/node_modules/playwright-core UIB_WEB_LIVE_ALLOW=1 UIB_WEB_LIVE_CASE=director node tests/bridges/web/guarded-live.cjs --run-authorized
```

`report.json` contains bounds/measurements/timings/cleanup. `director-observe.json`
is the original canonical input; `director-*-query.json` plus the recorded CLI
command shape make individual Measure calls reproducible while those files exist.
Fixture geometry, whole Web pilot acceptance and full performance gates stay open.

## G03 actual ordinary CLI component geometry — 2026-10-08

Working path: original Observe ChannelResponse → design Inspect → Rust Measure
queries → recorded resize Diff. Authority: [G03](../packets/G03-geometry-cli.md),
user's geometry-first priority; registry18/ANALYSIS@2 direct source input plus
unchanged TYPES/VALIDATION@1, CLI@6, CLI-DIFF@2 and selected Web geometry closure.
Only this receipt and tests/bridges/web/guarded-live.cjs changed. No collector,
engine, Core CLI, fixture or input-executor work. Existing owned F01 setup exposes
City/London and Options popup; values below are actual F01, not real Director data.

Corrected actual run2026-10-08T01:11:24.272Z–01:11:26.428Z passed17CLI calls:
3Observe (partial4), design Inspect0,12measurements (10known0 +2unknown4), Diff0.
Rooted scope contained10DOM+10AX after the existing London option appeared; BODY
portal remained outside it. A separate explicit context collected8DOM+8AX including
popup/trigger/input/list/option and the existing responsive marker. No snapshots
were merged or reserialized for analysis. Every result embeds the original source
Snapshot unchanged. Bounds are reported CSSOM layout_bounds, viewport/top-left,
css_px; AX names are linked through reported corresponds_to, never AX-as-layout.

| Selected part | Actual x,y,width,height css_px |
| --- | --- |
| Fixture form |380,20,360,133|
| City input |380,42,188,21|
| Suggestions listbox |380,67,360,32|
| London option |380,67,62.484375,32|
| Open options trigger |477.234375,121,97.296875,32|
| Options popup |400,290,200,60|
| Close options |400,290,98.78125,32|

Rust results: trigger-bottom→popup-top gap137; popup-left relative to trigger-left
−77.234375; input-bottom→list-top gap4; input→list and list→option left offsets0/0.
Input within wrapper: left0/top22/right172/bottom90; these measured insets are NOT
declared padding. Wrapper360×133 and popup200×60 came through factual CLI queries.
Clipping-aware popup width remains unknown_property; no viewport geometry node
exists for viewport-overflow calculation. Aggregate Aligned remains incomplete_scope
because the current engine requires complete coverage for that group operation.
Partial scope and sequential/unknown consistency remain visible, not upgraded.

Real viewport resize800×600→640×600 changed the context marker bounds from
660,120,100,30 to500,120,100,30. Ordinary Diff returned that changed DOM layout
property with both complete original records/environments and omitted_entries0.
It is a recorded property comparison, not deletion, clipping or normalized motion.
Observed CLI acquisition times were120.42/102.58/100.30ms including worker lifecycle;
Inspect23.39ms, local measurements24.84–26.33ms, Diff27.37ms in this single run,
not a percentile/performance qualification. No source-CSS numbers became measurements.

One initial attempt stopped at the harness's mistaken expectation that Aligned
would be known on a partial snapshot. The engine correctly returned incomplete_scope.
Only the harness was corrected: retain that unknown and add two ordinary pairwise
Gap queries for useful left-offset facts. No engine/source/cap/tolerance change;
the corrected complete chain above passed. Initial artifact inventory26files/report
fd81fd5f269eb08fbae75101238b0d1b7a2dcab7fd01888daa703441a6d9ba0e preserved that failure
until consumption; worker absence was independently verified after it.

Immutable shipping base e275823c18351bc05a6ac68972221758220d9c32, with only launcher
overlay733ea5f02a771ef15db8950c554dcd5180d52a447b36249166cc3d311d0a05cc.
199-input fingerprint811f4c9f60f36d148d0ad0d2df11219d5ac2d3a44f1932da30b5dbff9a0ea8ec;
CLI373ea712c934e1c59bedcd5979c99db76d15a75c79a1bdc85c2038dabf8d674b;
worker7a2fd76528424f01908bdbd72e2635699788580f10d963fb8c48e905c9306dec.
Locked/offline web CLI/worker builds and launcher syntax/diff checks passed. Root's
Core loader checks were reused; no old action or unrelated full suite repeated.
Native/runtime: Chromium145.0.7632.6, Node24.15.0, Playwright1.58.2, darwin arm64.
Unchanged live32nodes/depth8/64KiB/250ms/256visited caps,120s overall; local Diff
explicit output budget200000B accommodates two unchanged snapshots (actual86268B).
No browser/model/measurement framework was added; arithmetic remains Rust-only.

Read invariance passed for all three observations. All CLI children closed without
signals; exact worker-path process inventories were empty after every call and
after the run. Browser survival and owned context/driver/browser/server/profile
cleanup confirmed; private host counters were not invented. Source/binary/output
hashes matched after use. Key consumed output pins: rooted44044B ea92782d3f740ce9b63592ef221d6d86d1da05a9047dc63b63295257b5ddb1a4;
context36408B f7e68fc9d581b0d50fe88600ee42f7d29439c85132131b5f8145be869bff80c5;
resized36408B b09576ae03901bae7e5392422f7a7efc745e040b074714570290e74d5f5bbfe3;
Diff86268B ecb76c3ae512109e04f8885d9b35de78333010db635360fe605d6416cc8dcbd2;
report ba6701b4408afac57df58b784d6bd40869ed0a35d7d797700dba8b0fdc1b66bc.
These are checked/deleted bytes, not available file links. After inline delivery,
both run-owned output sets26+36 non-image files and current uib-web-geometry-69gec8w1
source/build were consumed/removed with absence verified. No images produced or
deleted, older evidence unchanged. No runtime resources remain held.

## Earlier rooted collector qualification

Status: corrected rooted sequence PASSED:9DOM+9AX within selected section;
wrong-binding/document/remount refusals passed; cleanup/temp removal confirmed.
Authority: root's explicit implementation dispatch and [packet](../packets/W01-rooted-selection.md),
within user-approved PLAN.UIB@1. Restore bounded PROJECTIONS/IDENTITY requirement;
registry11 Web contract closure remains applicable; unrelated CLI routing additions
make no Web semantic delta. Reused full W01 MODEL/EXCHANGE/PRIVACY/BOUNDARIES/
LIFECYCLE/GEOMETRY/FORMS/WEB-PILOTS and D02@2/D04@1/D05@4/WORK@1 closure,
RUST/DEV.RUST@2 and applicable implementation/QA/operational governance.

Exact12 paths: plugins/web/src/collector/{mod.rs,bootstrap.rs,select-ids.js,
acquire.rs,observe.rs,io.rs}; crates/host/src/{web_config.rs,worker_web.rs};
plugins/web/tests/collector.rs; plugins/web/tests/fixtures/collector/script-check.cjs;
docs/development/web-collector.md; this receipt. No schema/engine/CDP/transport,
Native/site/fixture UI, new persistent directory or selector/cache framework.

## Actual API and ownership

Private WebSelection::Rooted { root: WebRootSeed, max_visited_nodes } passes via
existing Tape(Request, WebSelection), TargetLease and submit_web_observe. Seed
contains session_id, target/surface identities, real document_backend_id and
backend_node_id, sensitivity. No BackendRef/Snapshot/Observation provenance is
fabricated. Host decode rejects zero/invalid backend IDs, empty binding and zero
visit budget; collector checks exact attachment/session/target/surface/document
before any collection. Caller must actually obtain the selected root from runtime;
source IDs/labels/rectangles cannot manufacture it or confer authority.

Collector::observe_rooted consumes RootedScope with that seed. Existing resolve,
selection/property/description, field/AX reads, normalization/publication/cleanup
are reused. Original root becomes first selected node and must describe to the
seed's backend ID. The same isolated selector traverses only its light subtree;
no ID lookup or global uniqueness claim. Duplicate DOM IDs do not merge objects.
Each visited node counts; depth is relative to root; source/output caps determine
maximum selected Elements (two-source allowance with AX). Limit/depth/truncation
returns explicit Incomplete, never complete or empty success. Boundary detection
refuses Unsupported. No widened scope, raised caps or extra collection cadence.

Original root parent is held only inside the run's remote selection container.
Before each field read and at final publication boundary, verify root/document/
connectivity/parent plus bounded parent chains of every original selected object.
Moved-out/removed nodes or reparented root refuse; same-ID replacements do not
repair handles. Final target/document and original-node checks remain. No parent
fields are collected or serialized. Caller-sensitive root redacts descendants;
existing sensitive-type classification still applies to each node. Returned refs
exist only after canonical publication and bind actual Snapshot/Observation.

## Focused checks

Four rooted peer tests cover actual-root request/ref issuance, six invalid binding/
seed variants with no collection, incomplete/early or final stale/refused publication,
and sensitive descendants. Eight existing Initial tests plus first-request ref/ACK
regression cover unchanged callers.43 offline JS scenarios include11 rooted checks:
only subtree, duplicate-ID sibling exclusion, visit/depth/source caps, frame/shadow
boundaries, disconnected root, original child moved out, root reparented, removal
and same-ID replacement. No browser execution or source-state oracle is invented.
One private host configuration test covers valid seed and malformed/budget cases;
worker caller compiles. Final check outcomes/pins follow below.

## Prepared actual-root F01 operation boundary (not executed)

Use the existing owned headless F01 page and fixture setup, not a user browser or
PlayPhrase.me. In setup only, use that page's CDP session to resolve the fixed
fixture section #f01 with a verified single DOM.querySelectorAll result, describe
that actual node to obtain backendNodeId and obtain DOM.getDocument depth0 backend
ID plus actual target/frame/loader identities. This fixture-only uniqueness check
is not a product root-selection algorithm for arbitrary documents. Session ID
comes from the new host attachment; all seed values must be checked against it.

Then one explicit Rooted request uses that seed and unchanged256visited/depth8/
32output/64KiB/250ms plus source16/100methods/reply budgets. Observe only section
and its descendants; surrounding document and BODY portal stay outside. Setup
may separately open the existing popup to demonstrate that a BODY portal is not
silently included. Rooted traversal must not add an unselected popup even when an
in-scope trigger controls it. This is different from the future Director popup,
which advisor source places inside its wrapper; no source-app truth is inferred.

Verify selected membership/current properties, partial coverage, no forged refs,
read-only checkpoint and owned cleanup. No rooted runtime is authorized yet;
next finite runtime packet owns harness edits/activation and exact cases. Actual
product caller selection is still a separate dependency from the advisor. Only
system-temp output for current use, inline facts then verified removal; no archive.

## Final source check receipt

PASS:4rooted peer tests,8prior Initial tests and first-request ACK/ref test;1private
host seed configuration test;43offline JS scenarios; affected Web tests check and
Clippy -D warnings; host library/worker check and Clippy -D warnings. Mock-peer
verification override was corrected so negative rooted replies actually execute;
host nested-if lint fixed without suppression. No failed check remains.

Shared source base cadd345df929b374e0d9ddd37833c3871b1ae05c was exported into
current-operation system temp uib-web-rooted-hg_gv8w8; overlay only the10owned
source/test files above. Final90-input fingerprint (compact sorted JSON path→SHA256):
960ef2f43d90f9bf8b16df8dd71235f6e2bed06b595339cfed359b4541ff7f8e.
Input set: all files at that saved base under host/schema/engine/plugin-api src and
plugins/web/src, plus Cargo.toml/Cargo.lock/rust-toolchain.toml, those five member
Cargo.toml files and collector.rs/script-check.cjs. All shared bytes matched saved
base; all candidate bytes matched the checked copy. Temp source/build was removed
after check use and absence verified. Earlier root-owned evidence was untouched.
No new permanent directory, browser run, source hold or runtime artifact exists.

Immediate consumer: existing private host caller can construct Rooted only from
a real selected backend object and matching runtime binding; no fabricated refs.
Next activation must separately implement the finite actual-root fixture harness,
then pin its build and run; this source checkpoint alone claims no live acceptance.

## Prepared rooted live sequence

Root opened exactly existing crates/host/tests/web_live.rs,
tests/bridges/web/guarded-live.cjs and this receipt for the actual-root sequence.
Mode rooted uses existing popup setup; a fixture-only CDP session obtains actual
#f01 node/backend and document identity, with one unique root and8descendants
verified against literal fixture membership. Target/frame/loader comes from actual
page binding; seed session_id comes from actual host Attached. No fake prior refs.

One current rooted request asks Role/AccessibilityName/LayoutBounds/Focused/Expanded.
Expected9DOM nodes are section,label,draft,suggestions,validation,commit,applied,
open-popup,surprise; BODY portal and all siblings excluded. Check section authored
x380/y20/width360, actual input City/combobox and compatible viewport bounds,
trigger expanded, input not focused, input Controls suggestions. No false trigger
Controls unobserved popup or AnchoredTo. Scope/focus/coverage remain honest.
Then wrong seed surface generation and wrong document ID refuse; separate clone
remount makes original seed stale. All3refusals commit0; original ACKed bytes stay
equal. Invariance applies to each observation/refusal; setup/remount is separate.
Current32/depth8/64KiB/250ms/source16/256visited/100methods and all byte/cleanup/
whole120s bounds remain unchanged. No extra scenario, site or physical input.

Build root: /var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-rooted-live-v8km8tuc.
Exact export: saved providercadd345 Cargo/lock/toolchain/crates/plugins-web/fixture/
driver files, overlay source1d83dc8 eight changed production files and current two
harness files.98inputs use prior97 recipe plus saved provider's new engine source.
Final compact sorted JSON path→SHA256 fingerprint: 1a5eabf6789ccb55d7c41c5890bbf979e913d0ed11199353d84889e30b806e4e.
Test build/debug/deps/web_live-5d88e78afab66360 SHA256
c83d2557d958bf884620c9b8e6ac07cfc47fd03b9bc632a444817d575d6332f6;
worker build/debug/session-worker SHA256
03919847d82fde5be8ce38148ddbea849d47bb3a3d84bd1d732ca850ffca2c22.
No-run and affected consumer Clippy -D warnings PASS; launcher syntax PASS. Before runtime require saved
harness equality and these exact pins. Root auto-activated one sequence after
checkpoint/push and matching pins; no extra approval or shared-source hold.

Runtime command uses existing Node/Playwright path and guarded-live.cjs
--run-authorized with UIB_WEB_LIVE_ALLOW=1, UIB_WEB_LIVE_CASE=rooted,
UIB_WEB_LIVE_TEST/WORKER at these build paths, matching *_SHA256 and
UIB_WEB_LIVE_EVIDENCE=/private/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/0b406b15-a58f-4593-8bc6-783a08a3d939.
This exclusive fresh temp pathname is not created yet. Inline result delivery,
then remove only its report/rooted-context.json/emptydir and current build/source
temp and verify absence; no persistent archive or changes to earlier evidence.

## First activated rooted runtime result

Harness40f45673660ba185717637fdf5a27a0ea7c8322c saved/pushed; existing source1d83dc8
and providercadd345 composition ran once2026-10-07T16:32:43.789Z–16:32:45.305Z.
Separate worker_web callsite e44f396 for new Core incomplete metadata is NOT in
this immutable build; its integrated compile belongs to the current Core handoff.

Actual rooted-current terminal invalid_input, committed0/missing1/operation1;
Collect/CollectorMalformed code15 ContinuityException, count0, remote_cleanup1
Released, send_progress0 NotQueued. No canonical frame, pendingcase1, test exit101.
The first positive request stopped the sequence; wrong-binding/document/remount
cases did not execute. This identifies the new rooted verifier refusal, not an
assumed JavaScript/native cause. No unchanged retry, threshold or oracle adjustment.

Actual host cleanup sessions0/groups0/abandonedfalse, fixture survives worker reap;
test/context/driver/browser/server/profile/worker_sessions all confirmed closed.
After-run98-source/both-binary pins and checkout harness/fixture bytes matched.
Temporary report SHA256b2a2f62a4211c9ca903572758fe42af5a6450858c9fa0702f41e7385769af10c
was checked then removed along with its exclusive0b406b15...directory; absence
verified. Current uib-web-rooted-live-v8km8tuc source/build temp removed and absence
verified. No other evidence/build removed, no raw exception text retained. Compact
facts delivered inline in chat. No Git/source/runtime resource is held.

Next concrete owner is Web collector/bootstrap.rs::verify_rooted: distinguish and
repair its actual runtime exception without weakening original-root membership/
parent/document continuity. Positive rooted API remains unqualified despite passing
source/peer tests; actual Director/caller selection remains separate work.

## Exact verifier diagnosis and minimal correction

Root authorized one diagnostic on owned headless F01, using existing fixture-host
launcher/CDP and EXACT selectIds/verifyRooted source bytes, actual9root/child objects
and same side-effect/bounds. No alternative product collection or whole acceptance
rerun. Result: debugger_side_effect, lineNumber=-1/columnNumber=-1 (Chromium did
not locate an expression). No raw text/stack/UI data persisted or printed; only
static classifier/numeric location. Focus/scroll/checkpoint unchanged; remote group
released and fixture browser/server cleanup completed; zero output files created.
Node inspector with plain mock objects passed, so it was not Chromium proof.
Chromium145 Node.idl readonly parentNode and prior selector behavior did not justify
a guessed getter cause; exact live verifier rejection is the actual repair basis.

Correction changes ONLY verify_rooted's throwOnSideEffect true→false. The fixed
function text, all document/root/parent/membership checks and bounded parent chains
are unchanged; no app callback, DOM mutation, eval or locator is introduced.
Initial selection and original verifyNodes retain true. Peer assertion recognizes
this exact per-function guard split. No general diagnostic/API/protocol changes.
Changed retry is authorized after saved correction/harness/pins; no unchanged retry.
Repair writes: collector/bootstrap.rs, existing tests/collector.rs and this receipt.

Correction checks PASS:4rooted peer cases,43offline JS cases, affected Web Clippy
-D warnings and guarded live consumer no-run. Verifier function body is byte-equal
to1d83dc8, SHA256d2575539ad3bd93d5e53d86649c27afed6345f781afb4921752bcf3f48be21a0.
Changed build uses same providercadd345 + source1d83dc8 + harness40f4567, overlay
only bootstrap.rs correction (collector.rs peer change is check-only). Temporary
root /var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-rooted-repair-vq71eal1.
Same98 runtime-input fingerprint e844887b453f33a3765cb8a53169f44bcf1ca067b4d6d0a21af49d73ae986c0f.
Test deps/web_live-5d88e78afab66360 SHA256
5faf368033ce9e1bfbe4774d4679aec0d9399662e6ae1ee9bc75e5f956806664;
worker session-worker SHA256
403e81262ed3ba5ff27b3fe8a7f565b061214e08b9d436af7ad16e7f7ba1cf33.
One changed rooted retry uses same command/mode/caps with these paths/pins and
fresh system-temp pathname /private/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/c214febd-28a7-47f8-be6e-de4b42470bb3.
After saved correction and matching pins, root already authorizes this changed run.
No new host metadata dependencies consumed. Same inline delivery and verified
current-operation output/build removal apply; prior evidence untouched.

## Corrected rooted sequence — actual result

Correction1568af3cc19878c68c99902f290d4efb03c93c1b saved/pushed; exact unchanged
harness40f4567 and prior providercadd345/source1d83dc8 ran once with correction
overlay2026-10-07T16:46:04.791Z–16:46:06.990Z. Matching98source/binary pins above,
no rebuild/retry of unchanged acceptance or unrelated liveCLI metadata adoption.

Actual status passed, exit0, pending0: rooted-current completed committed1/missing0,
9DOM+9AX nodes and partial coverage. Exact fixture section membership excludes
BODY popup and all siblings. Observed City/combobox/input-controls-suggestions,
trigger expanded, compatible viewport geometry, authored sectionx380/y20/width360,
unknown global focus/active descendant and no unobserved popup relation passed.
Published root identity matches actual discovered backend/session/surface and
actual Snapshot/Observation provenance; no fabricated seed refs were used.

Wrong seed surface binding and document backend each returned resync_required,
collector StaleTarget, committed0/missing1 with no remote acquisition. Separate
fixture clone-remount made original seed stale; it returned resync_required with
remote Released, no replacement search or publication. First ACKed bytes remained
equal through all three refusals. All4read-only invariance checks and the browser-
survives-worker-reap check passed. No cap/oracle/continuity/body-check relaxation.

Worker cleanup confirmed sessions0/groups0/abandonedfalse; test/context/driver/
browser/server/profile all closed. Post-run98source/binary/harness/fixture hashes
match. Temporary canonical frame39606B SHA256
c3b16b471ebeb42e7f949588f932cd668936eb28f691296dfcb474c8b67b9494;
report dd18c7e427caf1a39be87e2ebaaaa524e5eca1035098b5e7110d40f57fc65604.
These are consumed/deleted-file identities, not retained download artifacts.
After inline factual delivery, report/rooted-context.json and exclusive c214febd...
directory were removed, absence verified. Current uib-web-rooted-repair-vq71eal1
source/build temp also removed and absence verified. Old evidence untouched;
no persistent output directory, source/Git/runtime hold or raw diagnostics retained.

This closes the finite actual-root fixture sequence, not general product root
selection, live PlayPhrase.me Director, full B01/B03, reparenting live timing,
performance/Q02 or current integrated liveCLI acceptance. Reparenting/limits/privacy
remain backed by the focused source/peer cases recorded above; do not relabel them
as additional browser scenarios. Caller selection origin remains separate work.
