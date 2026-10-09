# I02 — current local distribution qualification

Current delivery: see [e641543 qualification](#accepted-ax-reuse-source-e641543).
The original94724df and a7c0416 records below remain historical evidence.

## Authority and plan

Finite single-chat full-cycle task under approved PLAN.UIB@1 P6 and the
[I02 packet](../packets/I02-current-distribution.md), dispatched from root chat
01a11088-e608-7801-bdfb-db5c9383af9d. The initiating instruction expressly permits
own plan followed by execution, documentation, scoped commit+push without further
root approval. No subagents, other chats, UI operation, branch/worktree creation,
signing, notarization or release publication. Mode Restore; qualification of
existing delivery, not a new product contract or implementation acceptance.

The plan was stated before edits: build four selections from committed94724df;
extend the owned offline check for Native session and current compare; verify
model-free saved-data paths and safe remove/reinstall; update exact owned docs;
clean own non-image temp resources; checkpoint and push under the shared Git lock.
Write set: README.md, THIRD_PARTY_NOTICES.md, docs/development/distribution.md,
docs/development/dependencies.md, tests/bridges/distribution_check.py and this
receipt. distribution.py was conditionally allowed only for a demonstrated defect;
none was found; the recipe is unchanged. Product source/manifests/specs and Q01 harnesses are protected.

## Spec Basis and evidence boundary

Traversal: applicable AGENTS → implementation governance, product-truth
core/routing/change/evidence/delivery/coordination and QA/operational safety →
[registry](../../../specs/README.md) revision29 → product/README and acceptance/README.
Selected CLI@16, NATIVE-SESSION@1, BOUNDARIES/RUST-BOUNDARIES/ROADMAP@1,
COMPLETION@1 and their full explicit closure; decisions branch → D01@1, D02@2,
D07@5 and DEV.RUST@2/RUST. Read dependencies: EXCHANGE@2, NATIVE@2,
MODEL/IDENTITY/GEOMETRY/PROJECTIONS/FORMS/ACTIONS/LIFECYCLE/CACHE/PRIVACY@1;
CLI-ACTIONS@3, CLI-EXPORT@2, CLI-DIFF@2, CLI-GRAPH-DIFF/CLI-GEOMETRY-DIFF@1;
ANALYSIS@2 with TYPES/VALIDATION@1; D03@3, D04@1, D05@4, MEMORY@2, WORK@1,
Native acquisition@2, D06@1, EVIDENCE@1; PILOTS/NATIVE-PILOTS/WEB-PILOTS/GOLDEN/
PERFORMANCE@1; EXPORT/DRAWING-PACKAGE/STYLE/GEOMETRY/PROMPT-A/PROMPT-B/REVIEW/
EXAMPLE and REUSE@1. Named CONTENT clauses plus CLI entry/export, D02/D05 lifetime,
publication, compatibility, privacy and limits remain unchanged.

Context is the finite packet, selected branch nodes and complete linked contracts;
no monolithic spec preload. Excluded: UI design, mobile/future features, independent
runtime/privacy acceptance and root coordinator-only routes. Cross-domain clauses
are protection/qualification requirements, not authority to edit their owners.
Old revision labels in Requires links resolve to the current files above; no new
contract delta or authority conflict was introduced.

Requirement: coherent installed modules, isolation from unnecessary tooling,
model-free analysis/export and safe recovery. Observed evidence: recipe, build and
feature manifests, CLI session dispatch, export output shapes, bundled fixtures,
I01 receipt and accepted source/safety review of3075239. I01's tested82342f0 is
historical; it does not prove current Native session/export integration.
Agent choice: reuse the sufficient recipe and add only affected smoke assertions
and operator documentation. Existing product/public formats are not redesigned.

## Exact inputs

- Initial checked-out master/task checkpoint: e0ce277388a6decde99a31355d8908ed0571b08b.
- Product source for every build: 94724dfd412f966d3d7a90db29aec8be7e35d650.
- Recipe last changed: 6a5bec23b8d15e4cc825aaff6105ac001a9a17bb; already includes
  NativeFormSession.swift. Recipe SHA-256:
  62fcf2f486d2b1ce8e69d87272089be20ab4bb52e53a7faaa077006c45027cd5.
- Cargo.lock SHA-256: 8084aeea8b0606cb11ac17a3293dc686f86c84f18809ac4f828e92c8fb672841.
- Recipe/docs provenance is distinct from product source; final checkpoint holds
  the documentation and verification changes, not new product binaries/source.

All Cargo manifests, lockfile and toolchain file are byte-unchanged between I01's
82342f0 and94724df. Selected features remain explicit/defaults-off. The existing
I01 license review and [independent source/safety acceptance](export-popup-distribution-review.md)
are reused within their stated scope; no universal license/security audit is claimed.
Concurrent commits and non-overlapping WIP are excluded by git archive of the exact
product pin. Existing after-title-spacing.png is untouched and never cleaned.

## Verification and completion

Builds ran with Rust1.96.0/ac68faa20, release/locked/offline,
aarch64-apple-darwin, macOS27.0.1 build26A434; Native helper used Apple Swift6.4
(swiftlang-6.4.0.34.1), SDK27.0, deployment target14.0. No older runtime is qualified.
Each install destination, source archive, Cargo target and Swift cache is owned
system temp. Two independent builds ran concurrently, without shared target dirs.

```sh
python3 distribution.py build --modules MODULE \
  --revision 94724dfd412f966d3d7a90db29aec8be7e35d650 --destination EXISTING_TASK_TEMP
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/distribution_check.py --bundle ABSOLUTE_BUNDLE
python3 ABSOLUTE_BUNDLE/distribution.py verify --destination ABSOLUTE_BUNDLE
python3 ABSOLUTE_BUNDLE/distribution.py remove --destination ABSOLUTE_BUNDLE
```

| Selection | CLI / worker features | Actual checks |
| --- | --- | --- |
| core | none / absent | Release build, verify, saved-data smoke PASS;17 external crates; no Swift invocation |
| web | web / web | Release build, verify, saved-data smoke PASS;39 external crates; no Swift invocation |
| native | macos / none | Release build including current helper, verify, saved-data smoke PASS;18 external crates |
| combined | web,macos / web | Release build including current helper, verify, saved-data smoke PASS;39 external crates |

Core/Web builds put refusing xcrun/swift/swiftc/xcodebuild sentinels ahead of the
system tools. Every build PATH contains only a temp rustup link and system tools;
no Node/Playwright/browser stack. All shipped executables have only system
/usr/lib or /System/Library dynamic dependencies (otool -L), no checkout/build paths.
The manifest source/features, recipe digest and common notices/docs match in all
four sets. Native helper and worker are built from the same archive as their CLI.
This confirms packaging provenance, not actual live private-protocol acceptance.

Smoke runs from cwd / with PATH=/usr/bin:/bin and no model credentials:
- Four canonical example inputs validate; measure returns literal8 css_px and
  check returns pass using analysis0.2.0.
- Document/propose each write six files, result_version0.1.0, generated_image=false.
- Direct saved-pair compare changes a synthetic width30→34; six files,
  result_version0.2.0, recorded layout change and displacement(0,0,4,0).
  This checks current E03 representation without claiming live observations.
- Help exposes native-session. Empty invocation gives invalid_arguments/2 in
  native/combined (entry present), unsupported_command/5 in core/web (feature absent),
  with empty stdout. No attach, helper invocation, permission prompt or UI input.
- Missing analysis input gives IO1 and no partial stdout.

Focused safety cases pass: late conflict rolls back only own files; injected fsync
failure leaves no manifest/partial publication; modified-file and symlink refusal
before removal; malicious manifest path refusal; foreign-file preservation; scoped
synthetic remove/reinstall; missing tool/revision/destination and relative destination.
Actual installed sets also refuse overwrite before compilation. Every real bundle
is removed by its own copied manager, preserving the foreign-I02.txt sentinel and
folder, then rebuilt into the same destination. All four reinstalled outputs passed the
same smoke, manifest/notice fingerprint and dylib checks before final removal.

### Dependency and notice fingerprints

Graph fingerprints are SHA-256 over each manifest dependencies array serialized
with Python json.dumps(sort_keys=True,separators=(',',':')); they include exact
versions, registry checksums, declared licenses and notice-file content hashes.
Shared package records agree across selections. The same committed manifests,
lock and features justify reusing I01's scoped source/license review; regenerated
fingerprints verify this run's material, not a nonexistent historical hash baseline.

| Selection | Graph fingerprint | DEPENDENCY_LICENSES.txt SHA-256 |
| --- | --- | --- |
| core | 04d150beeafd6219f2890248164dc5d948d08766e66d9c1802064c23727effa2 | 5d1a95506c3ea4d4d33e3f1e33feb62909c637eb3117d87e05906fdaa0a55b07 |
| native | 794e234264001d4c8852233fb9bd280643f98a8fe0902ec1b77acfd667c72401 | 7affb0885047cb61156bd13a1ba60851edd64d30b42c1bae9590606b02d87152 |
| web/combined | 5799e3427e446ab380d522860602ab6aa52b6fae01091145771ce43c374412d4 | 41feaf4df8cee2655d071e9f63ade5a062f6b112d2341bb2e773549278096f09 |

Pinned Rust standard-library notice SHA-256:
78c163fcec50e64bfd85fedb850c273595602fafa2b41f30f75d4e410b80ee83.
Unicode-3.0 and serde_json lexical/Alexander Huszagh attribution remain present;
jsonschema is absent from shipped graphs. No new licensing choice, dependency or
compiler policy. Package graph review remains distinct from a universal audit.

## Qualification and remaining limits

Classification: **verification**, with updated documentation and focused smoke
coverage. Existing recipe is sufficient; I02 makes no distribution.py or product
source change. Working deliverable is the reproducible four-selection procedure
and its checked operator examples, not another packaging framework or new runtime.

No compile/API/packaging blocker was found at94724df. Native form delivery,
Web runtime, privacy/isolation, full pilots, D06/P7 and release acceptance remain
Q01/Q02 or their designated owners' work. No UI test, live private-protocol proof,
independent review rerun, universal binary reproducibility, older macOS/Intel/
Linux/Windows/mobile qualification, signing or notarization is claimed. Existing
source/safety review is reused only for the unchanged manager, not labeled new
independent acceptance of this task's harness.

All eight actual builds (initial plus reinstall for four selections) succeeded.
All four reinstalled sets passed the final offline harness, including package and
receipt versions. Final removal used each bundle's own manager and preserved its
foreign sentinel. Afterwards only run-owned sentinels, tool guards and three
transient JSON result/fingerprint files were removed; the exact task-temp directory
was empty, removed, and its absence verified. Recipe-owned source/build stages and
smoke/safety non-image stages were cleaned by their owners. No images were created,
relocated or deleted; existing unrelated after-title-spacing.png remains untouched.
No raw command logs, builds or generated run evidence are staged/committed.

Python syntax, changed local Markdown file links, route consistency and git diff
--check pass. No Rust source changed, so no unrelated Rust logic/full-suite or UI
checks were run. The final exact-path checkpoint is made in the current master
under /tmp/ui-blueprint-master-git.lock (fcntl.flock), then pushed to the existing
canonical origin/master. The completing chat reports the resulting SHA; this
receipt is its own checkpointed record, avoiding a self-referential commit hash.



## Current candidate a7c0416

Same finite task, resumed by root's direct current-candidate instruction and
[packet continuation](../packets/I02-current-distribution.md#current-candidate-continuation--2026-10-08).
Initial current checkout51a09e, master; product pin
**a7c04164df08441cfbbaa61b501aa64d29290732**. The accepted original outcome and
write boundaries remain; user explicitly requests reuse of unchanged proof and
no eight-build replay. Mode Restore/verification, not new product design.

### Updated basis and plan

Recovered original full applicable AGENTS/governance/spec closure from this chat;
changed route evidence854037f→current: registry32, product/decision branch links,
CLI@16 Native requires reference, D03@4, NATIVE-SESSION@3, NATIVE-POPUP@1 and
WEB-DOCUMENTS@1. Read updated contracts and their changes fully; all other explicit
dependencies remain byte-unchanged and are reused. Protected input uses existing
FillSecret/core0.1 with one-use private source and same-Surface public result;
popup binding is explicit and excludes protected input. Documents adds explicit
whole-document selection, never expands ordinary scope. No contract delta here.

Requirement: deliver the coherent current saved source. Observed implementation:
recipe still archives all crates/plugins/web (including new JS/Rust collector
files), and existing Swift list already includes every changed Native source.
The only production deltas are host, plugin-api, Web and Native sources. Their
exact owners are protected. No missing packaging file or feature was found.
Agent technical choice: retain recipe; build Web/Native/combined once each after
Q02's terminal resource release; run affected installed checks and final removal.
Source-equivalent core evidence and unchanged full remove/reinstall/safety review
are reused instead of recompiling/replaying them. Scope stated before edits:
README, notices, distribution/dependencies docs, owned distribution_check and this
receipt. distribution.py only if a real packaging defect emerges; none observed.

Core equivalence was checked across complete production owners and manifests:
crates/cli/src (features off), schema/src, engine/src, export/src plus their manifests,
root manifest/lock/toolchain, schemas and all actually bundled analysis/export
inputs. No diff94724df→a7c0416. Core imports no host/plugin-api/Web/Native production
owner; changed export tests and analysis fixture generator/manifest are not shipped
core code or bundled example inputs. Prior core build/smoke/reinstall applies;
no freshly installed core/a7c0416 binary is falsely claimed.

Current CPU/headless dependency is Q02 chat01a11c77-25bf-7072-8cf6-a255fa4dc11c's
FULL-DOCUMENT task terminal release, not its historical/control-only release or
Q01's intermediate release. Preparation and disjoint docs/harness edits proceed
while waiting. No Native desktop or real application operation is authorized.

### New checks and reuse

Owned harness adds a no-UI Web Documents entry check: syntactically complete
configuration with an intentionally invalid connection version must reach
invalid_connection_version/2; an unknown selection must instead give
invalid_connection/2. Both stop before request IO, worker spawn, endpoint access or
host allocation. Zero caps/placeholder identities are intentionally inadmissible
test data, not documentation defaults. This proves deserialization availability,
not document acquisition, privacy or whole-document quality.

Original saved-data smoke (validator/measure/check/document/propose/compare and
Native entry gates) remains applicable and runs on each new installed set.
Existing manager publication/removal implementation and dependency/toolchain/
feature policy are unchanged. Reuse the prior detailed collision/fsync rollback/
changed-file/symlink/path traversal and actual four-set reinstall proof; freshly
verify and remove new sets with foreign-file preservation. No new package-manager
framework, dependency or broad audit. Graph and notice fingerprints must match
the historical table above; current source/recipe metadata must match the pins.

Saved Q01 acceptance a1cae1a on producta7c0416 was read:10 actual Documents cases,
original97-node/2-document baseline plus six private refusals and three safe
srcset cases, no leaked canary and confirmed owned cleanup. This is attributed
independent Q01 evidence; I02 does not rerun it or promote it to D06/P7/Native
acceptance. Q02 subsequently completed its full Documents campaign and explicitly
released CPU/headless in message msg_03fde7498d981145016ac7fba3f15081919a3a95cfd7ac9f5b.
Before compiling, I02 read its saved weba7-series-1/report.json under the Q02
system-temp root: no failure/cleanup_failure and21 confirmed clean closures.
This is the current full-document terminal release, not the earlier scoped one.
No additional root grant was requested. Timing results remain Q02 evidence.

### Current installed result

Classification remains **verification**: distribution.py did not need correction.
Recipe revision remains6a5bec23b8d15e4cc825aaff6105ac001a9a17bb and SHA-256
62fcf2f486d2b1ce8e69d87272089be20ab4bb52e53a7faaa077006c45027cd5.
Updated docs/harness are saved in this continuation checkpoint, separately from
producta7c04164df08441cfbbaa61b501aa64d29290732. No protected production owner,
manifest, lockfile, schema, specification or Q01/Q02 harness was changed.

Exactly THREE new release builds ran after the release above, using the existing
manager with --revision a7c04164df08441cfbbaa61b501aa64d29290732. Rust1.96.0,
locked/offline aarch64-apple-darwin, macOS27.0.1/26A434 and Swift6.4/SDK27.0 remain.
Private source/target/module-cache and install directories were system-temp only.
Web build used refusing Swift/Xcode sentinels; all three build PATHs contained only
an existing rustup link and system tools, without Node/browser tooling.

| Selection | Current evidence | Reuse / limit |
| --- | --- | --- |
| web | Current CLI/worker/validator build, verify, smoke, Documents parser, remove PASS | Unchanged manager reinstall/rollback and scoped license review reused |
| native | Current CLI/worker/Native helper/validator build, verify, smoke, remove PASS | Native session entry present; no protected input/popup UI delivery claimed |
| combined | Current matching full set build, verify, smoke, Documents parser, remove PASS | Same live-runtime boundary; no UI/browser launched |
| core | Complete production/input/dependency no-diff proof | Prior94724df build/smoke/reinstall, no new core build claimed |

Every new manifest has the exact current product pin and expected CLI/worker
feature sets. Recipe and documentation file hashes match across all three bundles.
Graph, DEPENDENCY_LICENSES and Rust standard-library notice fingerprints equal the
historical table above (Web/combined39, Native18). Cargo.lock hash also unchanged.
Every installed executable's otool -L dependencies resolve only to /usr/lib or
/System/Library, not the checkout or temporary build path.

Each new bundle passed the owned smoke from cwd / with no model credentials and
PATH=/usr/bin:/bin: four validators, gap8 css_px, check-pass, document/propose0.1,
compare0.2 with literal width30→34/displacement4, Native entry/feature refusal,
missing-input empty-stdout failure. Web/combined additionally passed the new
Documents version-vs-selection parse distinction. Invoked smoke directly to avoid
repeating the unchanged safety() campaign. No whole workspace suite, live session,
collection, secret read or fixture/browser/app launch occurred.

Fresh installed recovery checks on all three sets: build-over-existing refuses
before compilation; copied manager verify succeeds; copied manager remove preserves
the foreign sentinel and destination. Historical actual rebuild/reinstall and
injected failure proof remain applicable to the byte-unchanged manager. No claim
that those were repeated on this pin. All three new bundles were consumed/removed;
then only own sentinels, tool guards/link and transient results.json were removed.
Exact task root uib-I02-current-g434t6fc was empty, removed and confirmed absent.
Recipe/smoke owners clean their own non-image stages. No task images were created,
relocated or deleted. Shared Q01/Q02 proof and after-title-spacing.png untouched.

Updated README/distribution/dependency/notices explain the current pin and the
existing Native protected-input/popup and Web Documents setup routes. These are
source-checkout guides, not fake live identities or newly bundled fixture apps.
Python syntax, changed local links and git diff --check pass. Checkpoint/push uses
current master, exact six task paths and the shared fcntl Git lock; final chat gives
its actual SHA after remote verification.

No packaging/compile/API blocker remains for this finite continuation. Working
output is the current reproducible installed procedure and checked module choices.
Q01 privacy evidence and Q02 timing evidence retain their own pins and limitations;
Native foreground/comparability and full P7/release remain outside I02 acceptance.
No signed/notarized publication, global/home/PATH install, new persistent output
directory, arbitrary-app support or new platform qualification is claimed.



## Final current-source 063e709

2026-10-09 continuation of the same I02 P6 outcome, directly assigned through
[the final packet section](../packets/I02-current-distribution.md#final-current-source-delivery-reconciliation--2026-10-09).
Current selected source063e709cce4407aed2a2578620b645cf7b554f29 includes accepted
production Native770906798e1326fed3dd0edec40dc59b13772b3a. Initial master is063e709;
root's packet/task-registry WIP and after-title-spacing.png are protected.
No agents/chats/worktrees/branches, UI/SDK/input, real applications or shared cleanup.
Q02 terminal063e709 and the initiating instruction release CPU/build resources.

Full applicable AGENTS and the complete updated packet were read. Reused the
current full governance/spec closure above after confirming no spec delta from
a7c0416: registry32, CLI16, NATIVE-SESSION3/POPUP1, D03@4, WEB-DOCUMENTS1,
COMPLETION/BOUNDARIES/D01/D02/D07 and their explicit dependency closure remain.
No new material product choice or contract delta. Requirement: coherent current
installed helper/pair and honest current usage/limitations. Observed evidence:
shipping source delta is ONLY tests/bridges/native/WindowAX.swift; other changed
files are docs, test/benchmark consumers or I02's prior harness. Recipe already
archives that exact file. New ReadonlyFacts.swift is a Q02 witness, not a missing
shipping dependency. No packaging defect was found.

WindowAX adds selected raw Title extension distinct from Description and admits
Title in a singleton batch after the original fields. Q01's accepted7709067 and
terminal21e0abf reconcile field preservation and reusable bounded N03 whole-chain
functional evidence with explicit residuals. Read Q01's full terminal section,
Q02 actual readonly40f201d section and latest063e709 inactive-context result.
These are attributed acceptance/runtime inputs, never an I02 live rerun. Web
numeric evidence remains valid; Native source fidelity is not workload equivalence,
unchanged AX tree, verified input context or a D06 pass. Q03 remains separate/open.

Plan stated before edits: update exact four files README, distribution guide,
dependency inventory and this receipt; one fresh combined install from063e709;
existing installed smoke, exact source/features/recipe/notice/dylib checks,
no-overwrite/verify/remove/foreign preservation; own non-image cleanup; links/diff;
current-master exact-path checkpoint+push, then archive this completed chat.
No distribution.py or distribution_check.py correction is necessary.

Minimal verification basis: all Rust product sources/manifests, Web, schemas,
actually bundled examples, lock/toolchain and manager are unchanged a7c0416→063e709.
All ten Swift input files were compared; only WindowAX differs. Native-only and
combined invoke the IDENTICAL helper source list/flags, independently of Rust
feature selection. A new combined installation exercises the changed helper with
matching current-source CLI/worker. Prior native-only Rust/feature proof applies;
Core/Web and detailed rollback/reinstall/license evidence need no replay. No fresh
core/web/native-only installation on063e709 is claimed.

### Actual current installation and handoff

Classification **verification**; recipe/harness unchanged. Exactly ONE new build:
`python3 distribution.py build --modules combined --revision 063e709cce4407aed2a2578620b645cf7b554f29 --destination EXISTING_TASK_TEMP`.
It succeeded with the existing locked/offline Rust1.96.0 release profile and
Swift6/arm64-macos14 helper on macOS27.0.1. System-temp source/target/module cache
and installation only; PATH contained existing rustup plus system tools, no
browser/Node. No downloads, home/PATH installation or persistent output directory.

Manifest source063e709 and CLI[web,macos]/worker[web] matched. All shipped file
hashes/types/modes verified; otool -L on all four executables resolved only system
/usr/lib and /System/Library dependencies. The installed binary SHA-256 values:

| Binary | SHA-256 |
| --- | --- |
| native-host-helper | 80db2a0b2160a4c232d96b5d2c455ff46c64ed72fc7a11cf0feb10e5e93724a5 |
| session-worker | 442978474b00dff00d0ac189eaca59a688a7456ef9a56f6f5e33dd0bef9d3776 |
| uiblueprint | be7e22fa90ed580004ac3ceb9dec159f6f454d2d06d1e80f7dcf6a84fd9957b1 |
| uiblueprint-validate | 35e945ca310b91fd11f26e0416cdf7d7a916e0b1c3748acc8f247666bdfd300e |

WindowAX source at063e709 equals accepted7709067, SHA-256
15ed43c3c966b0156db8d76b670c8c64f921c3cf986c00d012dc7e6f8614406e.
Recipe remains6a5bec23b8d15e4cc825aaff6105ac001a9a17bb, hash62fcf2f4… as recorded
fully above. Graph39, Cargo.lock, DEPENDENCY_LICENSES and Rust notices fingerprints
exactly match the prior recorded values. This is current material verification,
not a new licensing audit or reproducible-bit-identical-build claim.

Existing smoke passed from cwd /, no model credentials/build-tool PATH: four
canonical validators; Native session entry, Web Documents strict parse-before-attach;
measure8 css_px; check-pass; document/propose0.1 and compare0.2 six-file packages
with literal width30→34/dwidth4; missing-input refusal/empty stdout. No SDK/AX/
UI/browser/secret/input runtime occurred. No harness correction was necessary.
No general Rust suite or unchanged injected-failure/reinstall campaign was replayed.

Fresh build-over-existing refused before compilation. Copied manager verify/remove
passed and left the foreign sentinel/destination intact. After consuming the
bundle, only own sentinel/tool link and empty task directories were removed;
uib-I02-final-j2nny0nx absence verified. Recipe/smoke owners cleaned their own
non-image stages. Every shared file, image and image-containing directory remained
untouched. No raw logs/build products were committed.

README and the guide now use/identify063e709 and give current build/install/use/
recovery steps, explicit trusted live setup links and limitation handoff. Scoped
Native/Web geometry and bounded functional evidence remain applicable; bounded
Native functional composition is accepted with Q01 residuals, not freshly rerun
here. Web numeric gates are measured on their exact workload/pins. Native original
benchmark structure/placement/context equivalence and eligible timed cohorts,
strict general AX-tree invariance, and Q03 real-case scope remain unresolved.
No background-input/IME/physical-pointer/arbitrary-app/other-platform or P7/release
promise was added. These are acceptance dependencies, not a packaging blocker.

All changed local Markdown targets AND anchors, route consistency and git diff
--check pass. Four exact task paths are checkpointed/pushed in current master under
/tmp/ui-blueprint-master-git.lock; final chat records actual SHA and remote check.
The finite I02 handoff is complete after that push; archive this chat as requested
by the packet. No persistent goal/P7 status is changed.


## Accepted AX-reuse source e641543

Same finite I02 resumed by the direct root instruction and
[accepted-source packet](../packets/I02-current-distribution.md#accepted-ax-reuse-source-delivery--e641543-2026-10-09).
Product pin e64154349bc93e9c7a4a91bab7698cef84b8eb7a; initial current masterb04b71a.
The previous checkpoint ef45577 successfully pushed063e709 qualification; archive
attempt was interrupted and is not represented as a confirmed archival outcome.
That prior binary is historical, not proof of the changed current host/helper.
No new agents/chats/worktrees/branches, UI/apps/model service or shared cleanup.

### Current basis, delta and bounded plan

Recovered applicable full governance/spec closure from the same task. Read the full
updated packet; registry33 and decision routes add Native acquisition@3, D06@2 and
request-only@1, all read with unchanged D02@2/WORK@1/MEMORY@2/NATIVE-SESSION@3 and
explicit closure retained. No product requirement is inferred from source. The
registered76-node workload retains all old facts and original100/200/300/750ms
thresholds; registration does not accept a candidate. No specification edit here.

Requirement: current coherent Host/Swift installed pair. Actual063e709→e641543
shipping delta: native_broker.rs exposes its channel internally; supervisor.rs adds
fixed epoch-bound capture bindings beside resident AX; HostHelper.swift adds the
bounded read-only window-ax loop; HostProtocol.swift admits only its exact flags128/
Observe/channel0 case. All four are already included in distribution.py. Test-only
performance.rs/diagnostic AXBoundary.swift must not become shipping inputs.
Existing recipe remains ten Swift inputs plus Cargo --bins; no framework change.

Read Q01's full focused acceptance9675c0b: ACCEPT exact e641543 source boundary,
independent protocol/binary evidence and attributed author execution, with explicit
v2/v3 provenance/cleanup residuals. This is not a fresh I02 live run or D06 pass.
Q03b014cd4 final Mac+Web outcome is completed bounded saved-data usefulness with
its own raw-import/live-ingestion/blind-scoring limits. No lingering Q03 dataset
dependency is invented. Timing/quality remains Q02-owned and outside this install.

Plan stated before edits: exactly README, distribution/dependencies docs and this
receipt; fresh native+combined builds only after actual Q02 CPU release; existing
model-free installed smoke, provenance/feature/notices/dylib verification and scoped
remove/foreign-file preservation; reuse unchanged manager safety/reinstall/license
and consumers; exact-path current-master commit+push under shared flock. No recipe
or harness correction presently needed. Product/spec/Cargo/Q02 paths protected.

Build selection rationale: changed shared Host is compiled with no features in
native and with web in combined; changed Swift helper has the identical native/
combined input list/flags. Both configurations need fresh installed proof. Core has
no Host dependency and its complete inputs are unchanged. Web CLI/collector are
unchanged; the current shared web-enabled Host/worker is built in combined. Prior
web-only feature-isolation and consumer evidence is reused, not called a fresh
web-only installation. Lock/manifests/toolchain/notice policy unchanged.

Resource gate: initial compact Q02 status remained active without a new release;
I02 did not compile. Root then explicitly transferred CPU to this prepared I02 task,
confirmed the last saved runtime release and stated Q02 was notified not to start
new timed cohorts until I02's actual release. This direct handoff satisfies the
packet's coordinator resource boundary; it is not inferred from silence. Two own
builds started only afterward. No additional internal grant is required.

### Actual installed result and released resources

Exactly TWO new complete installs passed, native and combined, from immutable
e64154349bc93e9c7a4a91bab7698cef84b8eb7a. Recipe remains unchanged at
6a5bec23b8d15e4cc825aaff6105ac001a9a17bb, SHA-256
62fcf2f486d2b1ce8e69d87272089be20ab4bb52e53a7faaa077006c45027cd5.
Classification is **verification**, with current handoff documentation; neither
shipping_product correction nor a new runtime capability was implemented by I02.

Built release/locked/offline with installed Rust1.96.0, aarch64-apple-darwin and
Swift6/macOS14 helper target on the existing macOS27.0.1 host. Each recipe build
used its own system-temp source archive, Cargo target and Swift cache. Build PATH
held an existing rustup link plus system tools, no Node/browser stack. All ten
Swift inputs were checked against063e709: only HostProtocol/HostHelper differ.
No AXBoundary.swift, Q02 witness, performance test executable or diagnostic flag
was added to shipping inputs. Q01's eleven-input diagnostic binary reproduction
is distinct provenance; I02 does not claim binary equality with that artifact.

Manifest product pin, features, file hashes/types/modes and complete inventory
passed. Native CLI[macos]/worker[] and combined CLI[web,macos]/worker[web] match the
selected build. All shipped executable dylibs resolve only to /usr/lib or
/System/Library. The two freshly built helper hashes are identical:

| Executable | Native SHA-256 | Combined SHA-256 |
| --- | --- | --- |
| native-host-helper | 0c0517fa9c3764220b07ce83ebdbd2d12f07e217edb22c40e145ba6f4e087f8c | 0c0517fa9c3764220b07ce83ebdbd2d12f07e217edb22c40e145ba6f4e087f8c |
| session-worker | 53784e16d9165b91c758202282975e47aaec987c340c71bbc05851c7eb2f57c5 | 442978474b00dff00d0ac189eaca59a688a7456ef9a56f6f5e33dd0bef9d3776 |
| uiblueprint | e7cc71153d9e7f81d9f6902b8b4acaba56cb2f83f6bcd62dd6c9d9397c0e4144 | c60be5d83b7f3ca272b7f73019b651e0f72fb4e5c3df9c9744ea58c4f124cdf0 |
| uiblueprint-validate | 35e945ca310b91fd11f26e0416cdf7d7a916e0b1c3748acc8f247666bdfd300e | 35e945ca310b91fd11f26e0416cdf7d7a916e0b1c3748acc8f247666bdfd300e |

Native18/combined39 graph fingerprints, complete dependency license hashes, Rust
standard-library notice and Cargo.lock hash exactly match the historical values
recorded above. No dependency/toolchain/license choice changed. This verifies
actual shipped material while reusing the scoped earlier license review.

Both installations passed existing smoke from cwd / with no model credentials:
four validators, Native entry/gate, measure8 css_px, check-pass, document/propose0.1,
compare0.2 and literal width30→34/dwidth4, missing-input empty-stdout refusal.
Combined also passed the strict Documents parse-before-attach check. Actual files
and receipt versions remain consistent. No app/browser/UI/AX/capture/input/secret
source/model call occurred. SDK-lifetime/fault evidence is attributed to Q01's
accepted exact source review and existing host tests, not claimed rerun by I02.

**CPU/build/check resource released at16:34 Europe/Podgorica,2026-10-09**
(clock confirmation14:34:04 UTC), explicitly announced in this chat immediately
after the two builds and all smoke/provenance checks. All compiler/test calls had
completed; no I02 runtime process remained. Remaining receipt/Git work held no quiet
CPU lane, so Q02 could continue without waiting for the final checkpoint.

Fresh affected install recovery: both build-over-existing attempts refused before
compilation; copied manager verify/remove succeeded while preserving each foreign
sentinel and directory. Historical fault injection/reinstall campaigns remain
applicable to the unchanged manager; no gratuitous rebuild cycle. After consumption,
only own bundles (via manager), sentinels, tool link, transient results/binary hashes
and empty directories were removed. Own root uib-I02-axreuse-09foqv_y absence was
verified. Recipe/smoke owners cleaned their non-image stages. No shared/Q02 artifact,
image, image-containing directory or after-title-spacing.png was altered.

README/guide now pin e641543 for reproduction and require the full matching set;
no standalone helper substitution or new CLI syntax. Q03b014cd4 is completed
bounded saved Mac/Web usefulness; Web numerical evidence remains on its exact
accepted workloads. Native D06/current quality, any future source repairs and full
P7/release remain separate. Builds and accepted source do not fix or excuse older
quality failures. No packaging/compile blocker remains within this exact pin.

Four-path current-master checkpoint/push under shared fcntl lock follows final
local-target/anchor, route-consistency and git diff --check verification. The final
chat reports the resulting SHA and remote confirmation. No product/spec/Cargo/Q02
performance file is staged; no broader goal or acceptance state is changed.
