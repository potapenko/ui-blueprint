# W01 rooted selection source handoff

## B04 sourced viewport→document preparation — 2026-10-08

Authority: appended [B04 packet](../packets/W01-rooted-selection.md), approved P2/P3
geometry work. Selected registry23 WEB-PILOTS B04/GEOMETRY/PROJECTIONS/common closure
reused; Core registry24/CLI@12 work is unrelated and protected. No public schema,
engine/CLI/spec/manifest/fixture or action source change. Exact9 writes: collector
read-node.js/wire.rs/acquire.rs; normalize/mod.rs; existing collector.rs and
script-check.cjs tests; guarded-live.cjs; Web developer guide; this receipt.
During preparation, the additive EXCHANGE@2 M05 composition diff was read: its
Native mixed-channel case is outside this packet; Web's homogeneous external_semantics
observations and requested/allowed channel remain unchanged. No shared WIP consumed.

Primary reconciliation: [CSSOM View](https://drafts.csswg.org/cssom-view/#dom-window-scrollx)
defines API geometry/scroll in CSS px and viewport offset against the initial
containing block. Client rect remains original viewport/layout_bounds/top_left;
Rust builds existing Known Transform `[1,0,0,1,scrollX,scrollY]` to document CSS space,
with separate Derived evidence, actual DOM Observation and full target/Surface/
environment binding. No source declaration, screen size or DPR-derived scale.
Document Space ID binds frame+loader (length-delimited frame ID); it is opaque.

Private rect DTO embeds before/after native Window/VisualViewport facts without
changing DomRead constructor fields/legacy actions. Finite positive viewport and
DPR, visual scale1, zero visual offset and matching page/scroll offsets qualify;
scrollbar width need not equal visual width. Missing/unconfirmed zoom/frame context
is Unknown; legacy private records without facts remain LocalOnly. Invalid data
refuses, changed per-read/cross-node/final context gives resync_required. One extra
bounded original-node read brackets mapped collection; original identity/root
checks remain LAST. Caller sensitivity is retained, native values are not acquired
for this check. Sequential consistency remains unknown, not global atomicity.

PASS:4 focused viewport functions/12 cases (signed fractional source facts, binding,
original bounds, unsupported/missing/invalid contexts, intra-read/cross-node/final
change, sensitivity), existing canonical local-only literal case and4 rooted
functions. Offline JS suite plus native Window/VisualViewport getter mocks passed;
Web lib/collector check+Clippy -D warnings and host web lib/worker check passed.
Only a test's nonexistent Document.to_json call needed repair to serde_json; no
contract/expectation change. Owned rustfmt, harness syntax and diff checks passed.
No runtime or physical lane yet; checks use savedeb2cb12 plus owned overlays,
excluding Core/Native WIP. Source pins and consumed test-temp cleanup below.

Final184-input sorted path→SHA256 fingerprint
418114917cc31041d6dbee1ac6fddad6df3f0857bca0143499363198bcb62375;
reader e8d00a767dfa6e37e19d7f9b031da3d45088f8eed836c93e61fb0dfe05202db1;
wire f6d821327dd896f70ae8a7a130b7e1833120f68fa0103e8c0347a750a82c9b42;
acquire e4a08802768b5a6620409a162ac431cb7131d3359b2687aefafebfb310b217b9;
normalize264794c06055c840b5c3ba31de0b109231fa97d0bc3f15612bca77651bbd6ab2;
collector tests94eabf6e14ac204f704b8b759e2a8436a9cf229adcb439c4f5065a8207d68408;
script-check e3b94f3b5885c192e22a2d3e3df6620ae1e0ef4912ab5656f8d0afb6fbc4c68d;
harness092a007bd2c37b7539fb211e224e801e258617027e8b8715e73f4802fe4dca0f.
All checked inputs matched after use; own2884-file nonimage source/test tree
consumed/removed with absence verified. No images or older evidence touched.

Prepared mode `UIB_WEB_LIVE_CASE=viewport` reuses the current guarded launcher and
explicit CLI/worker products. One own F01 sequence captures baseline800×600/scroll0,
resized1000×600/scroll0, then scrolled1000×600/scroll100. Source-authored marker
rect expectations are existing B04 arrays; wrapper left380/top20 grounds independent
document-space edge-offset expectations x280→480→480, y100 throughout, width100.
Rust Measure computes every quantity from each original ChannelResponse using its
returned document Space ID and contributing transform evidence. No JS conversion.
Fixed32/depth8/64KiB/250ms/256visited/120s and original geometry/privacy remain.

Immediate Core G12 consumer will receive ONLY3 unchanged canonical responses:
resize compares baseline→resized; scroll compares resized→scrolled. The resized
response is also before-scroll, avoiding extra collection. Preserve every original
session/target/surface/environment/Observation; publish exact keys/Space/hashes after
the run. Web retains those files until Core consumes/root releases; other consumed
own nonimages are cleaned normally. Transform-aware Diff/full B04/P7 are not claimed.

### Actual B04 Observe→Measure and G12 handoff

Root saved/pushed exact9 asb234fff. Common `host_observe.py build-geometry --output
<own temp>` succeeded at committedd033667254eea4004b0d57f9e8809405fd4a7ba6; Web/harness
pins matched. CLI SHA25600c7b970b6522d6ee3031fa031768945b61e68070af049081501b21e34895ad6;
worker d969c7d47671982ee0597442f1376f6264fd8065a5543da5192d9c0a209cccc4.
No WIP input, install, repeat tests or Native runtime. One `UIB_WEB_LIVE_CASE=viewport`
guarded run2026-10-08T10:37:19.704Z–10:37:21.601Z PASSED:3Observe partial/4 and9
Rust Measure known/0; all original Snapshot equality and read invariance checks passed.

| Actual state | Marker viewport rect css_px | Rust document-space x/y left/top offsets from f01; width |
| --- | --- | --- |
|800×600, scroll0|660,120,100,30|280 /100;100|
|1000×600, scroll0|860,120,100,30|480 /100;100|
|1000×600, scroll100|860,20,100,30|480 /100;100|

Wrapper viewport rect is380,20,360,123 then380,20,360,123 then380,-80,360,123.
Original viewport/layout_bounds/css_px/top_left records remain unchanged; actual
Known matrices are `[1,0,0,1,0,0]`, same, then `[1,0,0,1,0,100]`. Measure used the
selected document Space and includes contributing Derived transform evidence.
No screen mapping, JS rectangle conversion or normalized-motion Diff was run.
Observe wall87.51/74.91/75.25ms; local Measure24.69–26.69ms including process lifetime,
one sample, not percentile/full B04/P7 acceptance. Existing bounds unchanged.

G12 retained directory:
`/private/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/95947cba-acee-46e9-a092-b4ab0d375a88`.
The3 ORIGINAL ChannelResponse files were retained through G12 consumption:
- viewport-baseline.json5337B SHA256ea8176746d2e4ac5eb2490e17704cfde4f0e5aec22cc5146159a6c305043cd8d
- viewport-resized.json5339B SHA25688de587eabfb79bfe5803dfa8b45e2ad21683a8146012203727992d002a6d3eb
- viewport-scrolled.json5350B SHA256a5b44fd6b368d5d70fb88b8ab495eb488b1582cfd0926057e075752039e4dad1
Resize pair baseline→resized; scroll pair resized→scrolled. SourceKey f01=
`{"namespace":"web.dom","key":"21"}`, responsive=`{"namespace":"web.dom","key":"23"}`.
Result Space ID `document:32:D4AE1D0B4F6FB761C9C00D0B89047D67:1347F1B49D80FB45B00897C94B573903`,
kind=document, units=css_px, origin=top_left. Session live-cli-viewport, scope
f01-viewport, projection design, fields[layout_bounds] are unchanged. Target ID
D4AE1D0B4F6FB761C9C00D0B89047D67/generationd4231094-239f-4d02-a9d9-7028c96299e4;
same Surface ID/generation1347F1B49D80FB45B00897C94B573903 in all3. Environments are
respectively f01-800x600-scroll0, f01-1000x600-scroll0, f01-1000x600-scroll100.
No session/environment/clock/identity was coerced; per-call worker Snapshot/Observation
IDs can repeat and do not imply a shared monotonic clock. G12 must retain attribution.

All CLI children ended without signals and exact worker inventories were empty;
owned context/driver/browser/profile/server cleanup confirmed. Public counters not
invented. Consumed report SHA256112afda23083a8383a10fff1aa348ca53a47c8cdbd31108d1a537bc202f49a75.
The3 responses were retained until Core consumption and root's release; other owned
nonimage output/products were consumed and cleaned, with the final inventory below.
No images created; unrelated images/source/WIP remain untouched.
Post-use3 product hashes matched, products/empty output directory removed with
absence verified. Exact26-name JSON inventory validated;23 consumed files removed,
only the3 named responses remained with matching hashes for the G12 consumer.

Root subsequently reported Core's actual public G12 comparisons passed exit0:
baseline→resized has document-space dx200css_px with other deltas0; resized→scrolled
has allzero document-space deltas. Original input hashes/context/partial coverage/
evidence were preserved. This is attributed to Core's comparison run, not a Web
rerun or independent/full B04/P7 acceptance.
After root released retention, Web verified the exact3-name inventory, regular JSON
types and all3 original hashes, removed only those files and the empty95947cba run
directory, and confirmed absence. No images or other evidence touched. The hashes
above now identify consumed historical bytes, not retained downloads or live refs.

## G08 generic explicit-target Web first use — 2026-10-08

Delivered tooling: `node tests/bridges/web/geometry.cjs` takes explicit CLI/worker,
page-CDP endpoint, target/frame/loader/document/backend-root identity and invokes
public Observe → design Inspect → Rust Measure(width,height). The complete
copyable invocation and prerequisites are in the [Web guide](../../../development/web-collector.md#developer-first-use-an-explicitly-selected-existing-component).
It never launches/navigates/focuses a page, scans the document, imports Playwright,
reuses profiles/cookies or builds executables. No Director/F01/URL/viewport/cache
path is embedded in the helper. Existing rooted collector and canonical records
are reused; no product source/schema/grammar/dependency/contract changed.

Basis: current registry19/G05–G06 CLI@7 OBSERVE/INSPECT, ANALYSIS@2 with full
TYPES/VALIDATION@1 and geometry/identity/privacy/lifecycle/Web/D02/D04/D05/WORK/D07
closure reused; no relevant spec/Rust/Web source drift. Mode tooling, immediate
developer geometry consumer. Caller-origin identity bundle is mandatory: the
helper compares target/frame/loader/document, describes exactly one backend node,
then the existing collector verifies same-document/connectivity/root continuity.
No stale ID repair or cross-navigation stable-ref/action claim. A numeric backend
identity has no selector ambiguity; duplicate arguments are refused, never chosen
by order. Missing, foreign and stale document paths were actually exercised.

Write set: new `tests/bridges/web/geometry.cjs`, existing guarded-live.cjs, Web guide
and this receipt. Native's common build command remains its sole owner. The G07
build-result file had already been removed when supplied, but both original binaries
still existed; SHA values from its owner receipt matched. Only CLI/worker were
copied to this task's system temp. G07 saved recipe5f185fe, actual product source
build02ba827; no duplicate build and no modification/removal of Native originals.

Actual smoke03:48:46.759–03:48:48.340UTC: owned existing F01, generic command
arguments, root web.dom:21. Current example exit4 retained partial coverage and
printed8216B including reported root rect380,20,360,123 css_px plus8 other DOM
parts. Rust width360/height123 were both known, same viewport/top_left space,
local_only; source CSSOM reported, current live_read, sequential consistency unknown.
Coverage partial, omitted_count unknown, unknown_count18. Names/roles not requested;
hit/visible availability is not replaced by layout. No JavaScript geometry arithmetic.
Each Measure output's original Snapshot equality and unchanged Observe bytes passed.
Entire positive command154.34ms; not p95, speed qualification or real Director proof.

Missing backend2147483647 returned `target_unresolved`/4, foreign document IDs
from the second owned target returned `stale_document`/4, and the original bundle
after owned-fixture reload returned `stale_document`/4. All three had zero stdout.
Each command preserved fixture state; no owned worker remained after it. Context,
driver, browser, profile, fixture server and CLI cleanup confirmed. The helper's
own non-image JSON directory is removed before its result exits. It leaves caller
pages open; only the fixture harness subsequently closes its owned pages/browser.

Invocation under existing finite harness: G05 environment/pinned executable paths
with `UIB_WEB_LIVE_CASE=first_use node tests/bridges/web/guarded-live.cjs --run-authorized`.
Caps stayed32/depth8/64KiB/250ms/256visited/120s; no cap tuning or old suite repeat.
Syntax/diff/local-link checks and three pre-network invalid-argument cases passed.
After the live proof, only cleanup-error handling changed to attempt every owned
file removal even if child cleanup fails. A focused mocked-metadata/missing-CLI
spawn-failure check confirmed no browser/network access, failure1 and no leftover
helper temp directory. Successful collection/measurement logic stayed unchanged.

CLI SHA2564324cae4ff918d08f3a1c1e321c49373be2887c9c1b0ad853e6c5e4dd09c77d8;
worker4cb36acbfeebeb9b6a0fd9bfd598ea7a2063dadb55623d9302500963eae7ab23.
Live helper2c86438c36751d96c21f4d6a6b02ddf9b28bebdc16ab1cd5708c3741d2536804;
final helper7360cd83e585e42035c1d123322280194a9f8fa629070cf27e1ce7fde788dff3;
harness cef2056ee1f48366c4ca8529dba9786b51cb70acd1729b17efb0feb92258d44a.
Report6f90dc9886e8d6a05091766ccca093b0b6fa45d0f806fc4df107132a65ef6dda;
human outpute27319f954dcd8d9452cf7ab45415f7cf277fae7f6bd0cc615945fc789e52a8c.
Both copied executable hashes matched after use and their worker process inventory
was empty. The two copied binaries and two consumed output/report files were removed
from their own temporary directories, with absence verified; originals untouched.
No image or persistent output directory created. G06 combined-field64KiB refusal,
general browser/platform support and full B03/P7 acceptance remain unchanged.

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
