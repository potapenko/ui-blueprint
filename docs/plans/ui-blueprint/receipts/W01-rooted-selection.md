# W01 rooted selection source handoff

Status: source slice implemented; source/peer checks only, no rooted browser run.
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
