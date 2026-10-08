# W06 Web fidelity — srcset boundary repair ready for affected Q01 recheck

## Remaining P1 continuation — 2026-10-08

Q01 terminal88d0920 independently accepts P2 focusability and the original97-node
positive facts on9d715ee. Those scopes stay closed. Read its entire final recheck
section and the five exact reviewer reproducers under system-temp
uib-q01-w06-recheck-_cy5phij. Same W06 packet/WEB-DOCUMENTS@1 FACTS/BOUNDS and full
closure, reused after no-drift check. Mode Restore; no spec delta or new secret class.

Confirmed remaining defect: whitespace-only splitting hides the credential candidate
after `1x,` when no whitespace follows. Plan: candidate URL phase preserves internal
commas, descriptor phase recognizes its terminating comma; refuse unsupported
parenthesized descriptor ambiguity. Use identical finite scans in JS preflight and
Rust captured-table guard. Preserve original source bytes and currentSourceURL/originURL
checks. Basis: existing private credential-URL policy and [HTML srcset parsing](https://html.spec.whatwg.org/multipage/images.html#parsing-a-srcset-attribute)
URL/descriptor boundaries; original code, no upstream copy/new dependency/selection engine.

Write set: collector/document-check.js and snapshot_normalize.rs;
snapshot_privacy_tests.rs; document-privacy.cjs and shared srcset-cases.json;
new tests/bridges/web/srcset-privacy.cjs plus this receipt. Accepted focusability,
Q01/Q02-owned files, frozen fixtures, schema/engine/Cargo/Native and images protected.
Shared performance.cjs is currently Q02 WIP; do not stage, overwrite or consume it
for runtime until that owner's terminal saved release.

Initial execution state was waiting_resource for builds/tests/headless. Q02
01a11c77-25bf-7072-8cf6-a255fa4dc11c owns CPU/headless for accepted single-control
D06 cohorts. Source/test edits prepared without runtime or compilation. Recheck
compact wait_threads returned terminal completed/idle and final CPU/runtime release
on pushed b3c3a22; its full result/release receipt was read. The lane is now released
for focused checks/one affected live proof. No new root grant required.
All existing shared evidence/reviewer reproducers remain read-only and retained.

### Remaining P1 repaired result

Verified source `a7c04164df08441cfbbaa61b501aa64d29290732`. Both guards now scan
URL and descriptor phases separately. A descriptor comma starts a new candidate
even without following whitespace; URL-internal commas stay inside the URL.
Unsupported parenthesized descriptors refuse safely. This is a bounded privacy
classification pass, not a new browser source-selection algorithm or text rewrite.
All205 saved crate/Web/Cargo/toolchain inputs matched this pin byte-for-byte.

After Q02's terminal b3c3a22 release only:

* Shared20-case corpus passes in JS preflight and actual Rust snapshot normalization:
  tight/spaced density and width descriptors, first/second/third candidates,
  descriptor-less trailing delimiters, ASCII whitespace, Unicode, data URL commas,
  credential/token variants and ambiguous descriptor refusal. Safe native strings
  are compared verbatim; each private table uses a SAFE currentSourceURL.
* Exact Q01 comma-repro.cjs with repaired source/expectation returns private/count2.
  Exact comma-counterexample.rs with a repaired refusal assertion passes in a
  temporary test overlay. Original saved production bytes were restored before
  runtime compilation; reviewer files remain untouched. Existing captured-only
  currentSourceURL/originURL safe/private checks also pass in the same focused test.
* Separate affected headless driver srcset-privacy.cjs passes10 explicit Observes:
  baseline plus3 safe positives and6 private refusals. The actual comma-tight
  protocol-relative credential case with the real fixture port and safe selected
  currentSrc returns InvalidInput/0committed/0canonical bytes. Absolute credentials,
  third candidate, tab boundary, private candidate after a data URL and tight token
  query likewise refuse. Safe tight lists, base64 data URL and multiple internal
  data-URL commas retain exact source facts with raw/canonical parity.
* One necessary full positive sanity response still has97nodes/2documents/1102facts/
  19text boxes (443,370canonical bytes). Safe supplemental responses452,585 /
  452,695 /453,677bytes stay below the unchanged512KiB profile. Accepted P2 and
  semantic/geometry cohorts were neither changed nor rerun; no D06 timing campaign.
* Final Clippy Web lib -D warnings, selected Rust formatting, Node syntax, local
  receipt links and diff whitespace pass. No broad unrelated suites were run.

Actual Node24.15.0/Playwright1.58.2/Chromium145.0.7632.6, own headless800×600/DPR1;
original frozen F01 source hashes checked. All requests preserve state/focus/scroll.
No canary in canonical output, bounded diagnostics or saved report/config. Private
cases produce no Snapshot/publication for downstream retained/cache/history/export
consumers; no independent rerun of those unchanged consumers is claimed.
One actual host closure reports0sessions/0completion groups,192bytes ledger backing,
cleanup_confirmed=true, no abandoned/poisoned owner and caller exit0. Own browser,
context/server closed; exact task worker inventory empty. CPU/headless released.

Minimal sanitized report/config retained for the same Q01 affected recheck/Q02:
`/private/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-w06-srcset-proof-yi3pBs`.
SHA256 report `beeec335505776e8bdf6cea34759e0f13d1caf54160ae5a0531dcb052f36a21a`;
consumer `c6ff2485a688a83b8da06efa0db10ee613c539dd09732de5e64ecc2089f54cf6`;
worker `1b8f2689ccb661396d6eece44aefdc6eb9df0abf28f6887810954b8c5d65fd6c`;
driver `338b7dba25e521c01a45bfd8f1ed6e5c315d25f83959734d39047454885ee62d`.
Retain until both named consumers finish. Previous e020gN/Ln5Cz7, all Q01 proof
directories and both reviewer reproducer directories remain untouched. Own source/
build/log nonimages are cleaned after verification; no images created/deleted.
Shared performance.cjs changes belong exclusively to Q02's committed b3c3a22;
this task did not stage/edit its sources, documents or receipts.

This author result closes the reproduced behavior in its tested scope; the final
Documents privacy acceptance remains the same Q01 reviewer, then Q02's full-document
measurement gate. Existing accepted P2/positive facts retain their9d715ee acceptance.

## Q01 repair continuation — 2026-10-08

Candidate139d202 below was rejected by Q01 for confirmed W06-Q01-P1/P2; the
historical author proof does not establish independent acceptance. Same approved
W06/PLAN.UIB@1 scope and WEB-DOCUMENTS@1 FACTS/BOUNDS, mode Restore; no new semantic
contract delta. Read the entire Q01 final review and original W06 packet; reused
unchanged full closure after verifying no applicable spec/Web source drift from
139d202 to starting master8bcdaf4. Reviewer counterexamples were read without edits.

Observed: srcset and preserved native currentSourceURL/originURL bypassed URL
classification; focusable reused a converter accepting token/tristate strings.
Repair plan: classify srcset URL tokens plus observable currentSrc at bounded
preflight, classify captured URL facts before canonical nodes, and accept only
boolean-typed/boolean-valued focusable. Keep safe facts and legacy Checked mapping.
Write set: normalize/mod.rs; collector/document-check.js, snapshot_normalize.rs,
snapshot_privacy_tests.rs; collector.rs focused tests, document-privacy.cjs;
tests/bridges/web/fidelity.cjs and this author receipt. No Q01/Q02-owned files.
Checks: repaired independent repros, source type/URL matrices, original full97-node
headless workload plus safe/private srcset and existing binding/bound/cleanup cases.
The shared e020gN records and reviewer r0o_1c_n reproducers are retained untouched.

### Repaired result and evidence

Coherent repaired pin `9d715ee7bd8e566ad2b955024a68a8c3804966a4` (production
fixes05a25ff/f79eec7; later driver-only checkpoints). All204 saved crate/Web/Cargo/
toolchain inputs were byte-compared with that pin before cleanup: zero mismatches.
No spec meaning/threshold/API change and no protected-owner or frozen-fixture write.

P1: preflight classifies every whitespace-delimited srcset token, conservatively
including descriptors and preserving embedded data-URL commas. Observable native
IMG currentSrc is also checked. Captured currentSourceURL/originURL rare values
are classified after table validation and before canonical node construction;
token-bearing values refuse even when no private attribute remains. Protocol-relative
credential URLs are recognized by the same existing policy. Safe source strings,
attributes and URL facts remain byte-for-byte native values. No sanitizing rewrite,
new URL framework, general secret detector or canary-specific product rule.

P2: only native boolean/booleanOrUndefined kinds WITH a Scalar::Flag value become
known focusable. String/token/tristate/numeric/null/wrong-kind values remain unknown.
Actual Chromium145 F01 reports `{type:"booleanOrUndefined",value:true}`. The first
repair rejected that valid kind; the full live scenario exposed it and f79eec7
restored the original known fact while retaining strict value typing. Legacy ax_bool,
Checked tristate, Focused, field selection and evidence ownership remain unchanged.

Verification on saved source (Rust1.96.0, locked/offline):

* 77 collector tests +16 library tests pass. The focusability test now exercises15
  type/value/absence cases and independently checks legacy Checked=true and
  Focused=false. The URL table regression reproduces Q01's exact two-node input,
  then isolates srcset/currentSourceURL/originURL with safe/private pairs.
* Both original Q01 Rust counterexample modules ran with repaired assertions in
  temporary test-only overlays:2/2 pass. Overlays were removed and original saved
  production bytes restored before building the runtime. Exact JS check.cjs with
  only repaired source/expectation substitution now returns private/count2. Reviewer
  originals were neither changed nor deleted. The committed5-case JS matrix also
  covers later candidates, credentials, safe query strings and data-URL commas.
* Recorded original97-node replay +7 hostile table variants passes. Final Web
  all-target Clippy with -D warnings, changed rustfmt/Node syntax/local links and
  git diff whitespace checks pass. No unchanged Native/shared engine suite rerun.

Final own headless quality run:21 explicit requests,7 positives/14 expected
refusals,5 attachments/shutdowns. Original F01 remains2documents/97nodes,
1,102 checked native facts/19 text boxes; raw baseline10,788bytes. Canonical full
baseline/changed/restored443,379 /444,084 /443,381bytes; AX Apply→Changed→Apply
preserves true focusable on reused observed refs. Original frame/binding/bound,
password/token-URL, real extra-frame/navigation refusals remain zero-publication.
New private first/second srcset candidates each give InvalidInput,0committed
channels/0canonical bytes. Supplemental safe IMG scenario retains srcset and
known currentSourceURL and passes raw/canonical parity (453,695bytes); it does not
replace the original97-node workload. Canary absent from every response, bounded
caller diagnostics and every retained proof file. No rejected Snapshot exists for
downstream cache/history/export; those consumers were not separately reimplemented
or rerun. Captured-only originURL and malformed AX-kind evidence is deterministic
source/peer testing, not a claim Chromium emitted malformed protocol or that an
originURL-specific live leak was independently exercised.

All5 closures: cleanup_confirmed=true,0sessions/0completion groups,192bytes ledger
backing, no abandoned/poisoned owner, caller exit0. Own browser/context/server closed;
exact task worker process check empty. Profile ceilings/D06 gates unchanged; no
timing campaign. Independent acceptance remains with the same Q01 reviewer.

Minimal new public proof for Q01 recheck/Q02:
`/private/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-w06-proof-Ln5Cz7`.
Includes original full and supplemental safe-URL raw/canonical pairs, semantic
records, exact request/config and report. Retain until BOTH consumers finish.
Report SHA256 `f6de920702477944cf95157f8683e17586048fc43b644ffdf7e8b756e4ff7eef`;
canonical baseline `40f96c11c5a7ddbffd7543d700aaeaa52d5ba5ff9208f29112721159e890ecd2`.
Host consumer `647acd9cd8bb3eb0f614e4341287f1e2cc67b442f7d5e22b4206009497020bdf`;
worker `cc01ac31852335f5c6dbc48941269805bf7ad589ea18b7d8e2ab9b5ba2632b8e`;
driver `be85ecf4981d05603fd78d03c3fcd7873c71995c9afa6ef13d5d14eafbd4fa0c`.
Own failed-run nonimages and source/build temporary outputs are removed after
verification. Older shared e020gN and reviewer r0o_1c_n remain intact. No images
created/deleted; after-title-spacing.png untouched. Q02 harness/docs/receipt and
Q01 acceptance receipt are unchanged. Q01 recheck, then Q02 measurements remain
explicit acceptance gates; this author result does not self-accept them.

## Historical candidate139d202 — rejected by Q01, retained provenance

Authority: [packet](../packets/W06-web-fidelity.md), approved PLAN.UIB@1 original
358c757 and user approval preserved in [registry](../task-registry.md). This chat
executes its finite task directly, no agents/messages or D06 timing campaign.
Start master2d1eb5d; prior accepted Web source8e3dba2. Unrelated untracked image
after-title-spacing.png is protected and never touched/staged/deleted.

Traversal: global/repository AGENTS → implementation/product-truth/QA governance
→ specs/README → product/README and acceptance/README → MODEL/BOUNDARIES/FORMS/
PROJECTIONS/IDENTITY/GEOMETRY/PRIVACY/EXCHANGE@2, CACHE/ACTIONS/LIFECYCLE/NATIVE@2,
ROADMAP/RUST-BOUNDARIES; PERFORMANCE/WEB-PILOTS/PILOTS/GOLDEN@1; decisions branch
D01@1/D02@2/D03@4/D04@1/D05@4/MEMORY@2/WORK@1/D06@1/D07@5/evidence/REUSE;
RUST.md/DEV.RUST@2. Full selected leaves read; Native included only as explicit
shared-contract dependency, not runtime scope. Export/mobile/real sites excluded.
Supporting evidence: Q02 receipt/development recipe, F01 frozen fixture/handoff,
actual collector/normalizer/host Web owners and schema extension validator.

Mode Restore. Contract requirement: raw semantic fidelity and original full cold
workload. Observed defects: AX focusable discarded; rooted selection only Elements
in one document and refuses iframe. Technical choice W06-FIDELITY-001 is registered
before source edits in [WEB-DOCUMENTS@1](../../../specs/product/web-documents.md).
No protected wire/schema/engine/Cargo/Native change; no independent acceptance yet.

Write set: plugins/web/src/normalize/mod.rs; collector mod/wire/io and new document
collection/normalization modules in that existing directory; focused plugin tests;
crates/host/src/web_config.rs, worker_web.rs; new finite headless test in existing
tests/bridges/web; docs/development/web-collector.md, selected spec leaf/registry,
this receipt. CLI reuses existing typed WebSelection, no syntax change needed.
Verification: known true/false/unknown focusable; full original nodes/fields/bindings/
units; fresh changed/restored data; wrong/stale/unallowed frame; privacy and bounds;
saved-source focused compilation/tests/clippy, own headless proof and cleanup.

## Delivered API and representation

Production source `139d202b78a59d9017efaea1e55032f69f6468ac` on canonical master.
`Collector::attach_with_surfaces` takes the trusted descriptor's allowed Surfaces;
`observe_documents` takes explicit DocumentsScope with actual frame/loader/document
bindings and finite max_visited_nodes. Host WebSelection::Documents uses the same
guarded attach/Observe/Tape/Frame/Commit/ACK path. Existing initial/rooted/references
and Native/shared host/schema/engine/Cargo/frozen fixtures stay unchanged. Ordinary
attach Surface admission remains independent of a selected node allowance.
Existing CLI Web connections reuse this typed selection; no new CLI grammar owner.

`Focused` selects AX extension namespace `web.ax`, name `focusable`, requested
Field::Value/Value::Flag with source AX Observation/reported provenance. Known
true/false/unknown and unrequested selection are separate; Focused itself is not
fabricated. Semantic baseline/challenge/restoration reused actual observed DOM refs.

Full capture uses fields Value/LayoutBounds and original CDP DOMSnapshot tables.
The97 backend IDs/native names/types/node values, attributes, rare properties,
parents/children and frame Owns binding survive normalization. Original layout,
offset/client/scroll rectangles,19 text boxes/UTF-16 offsets and document metadata
remain attributed native facts. Source-native `-1` string index means known empty.
Document/css_px versus local/css_px stays explicit; transforms remain unknown.
Zero omitted DOM nodes does not make AX/visibility/paint/full design complete.

Source basis: the pinned [DOMSnapshot protocol](https://github.com/ChromeDevTools/devtools-protocol/blob/d209a9a38897d2935a078a0bf00ca821811d21ed/pdl/domains/DOMSnapshot.pdl)
and Chromium145 [InspectorDOMSnapshotAgent::AddString](https://github.com/chromium/chromium/blob/47e20adcc15fc15f01825aa17e570c8f5492ac0f/third_party/blink/renderer/core/inspector/inspector_dom_snapshot_agent.cc)
were read for native tables/rects/string semantics. The empty-string behavior was
also reproduced in actual F01. Own Rust decoder/normalizer, no upstream code copy,
dependency adoption, stylesheet inference or second graph. Prior attempts failed
closed on acronym field names and signed empty indices; those defects were fixed,
not accepted as degraded full-workload results.

## Actual finite quality proof

Final [driver](../../../../tests/bridges/web/fidelity.cjs) runs18 explicit requests
over5 owned attachments:6 positive responses and12 intended refusals. No sampling
cadence or D06 timing campaign. Node24.15.0, Playwright Core1.58.2,
Chromium145.0.7632.6,800×600/DPR1, unchanged original F01 files/a8368076.

* Baseline:88+9 DOM nodes,2 SurfaceRecords,57 layout entries,19 text boxes;
  1,102 individual native facts checked against the raw tables. Authored root
  button40/60/120/40 and child button12/18/90/30css_px also match literally.
* Native canonical channel sizes443,366 /444,084 /443,373bytes for full baseline,
  changed width121 and restored120. Same attached session, distinct fresh live
  Observations, matching original frame/document bindings and selected fields.
  Raw baseline is10,788bytes, matching the original frozen response size; SHA256
  b50436bb6e6b9f237f75cfea31697abec2b110f0ea20cf009244b5f2cb98d3d1.
  Canonical baseline SHA2565c00bddc56886a11324ee69837648564fbdb762ddd061ad1226b2d1f64108e57.
* AX focusable=true matches raw source; semantic name Apply→Changed→Apply uses
  the same live session and observed refs. Canonical sizes7,108 /7,104 /7,072bytes.
* Wrong document/loader, missing or unallowed Surface, actual extra frame and
  actual child navigation all yield ResyncRequired with zero published bytes.
  Wrong Target yields PermissionDenied. Node96/depth2/output4096 refusals retain
  their ResourceLimit; no ceiling was raised to pass those probes.
* Password canary and token-bearing URL refuse the whole native collection with
  InvalidInput/zero publication. Known private state is checked before capture;
  private nodes introduced in captured tables also refuse before canonical output.
  This is not universal detection of arbitrary secrets. Full native capture
  cannot safely redact all shared string/srcdoc aliases, so it does not pretend to.
* Every Observe preserves fixture state/focus/scroll. All5 real host shutdowns
  report cleanup_confirmed=true,0sessions/0completion groups, retained reservation
 192bytes (ledger backing), abandoned=false/reaping_poisoned=false, caller exit0.
  Own headless context/browser/server closed; exact task worker process check empty.

Full-proof parameters:128 nodes/depth16,512KiB canonical channel,32KiB protocol
message/256KiB cumulative replies,16KiB text/100methods,2s failure deadline.
They fit unchanged D05 ceilings; single-control32/64KiB/250ms and D0650/500ms are
not relaxed. Native SDK allocation/work stays opaque, not a browser RSS-cap claim.
Q02 must measure original20cold/100warm cohorts itself and report separate stages;
the quality driver does not establish latency acceptance or missing telemetry.

## Checks, pins and retention

Saved-source locked/offline compile passed. Collector77 tests and15 library tests
passed (recorded test initially opt-in); guarded Web worker12 tests passed. Explicit
F01 recorded replay subsequently passed with7 hostile native-table variants:
parent cycle, out-of-range string, wrong document, negative rect, duplicate backend,
new private node and bad text-box index. Empty/negative native-string focused check
passed. Final Web+host `clippy --all-targets --features web -- -D warnings` passed.
Changed Rust formatting, Node syntax, local links and diff whitespace are checked;
no unrelated workspace/Native suite or independent acceptance is claimed.

Final source139d202; driver checkpoint58f754c, SHA256
`26fb8a6063409ab84adaebfb24bfcb5e2fe462044b4b56cc338a0221c04846a1`.
Debug host consumer SHA256
`8ce2d4925422bdffb965b63eff4ed3a4cd516a48d2464b4ca983797c2b03e980`;
worker `9a09532a3944e2947bf7a6f75e01c20c20996cbc44515fc69abd89d086882af1`.
The unchanged Q02 consumer source was compiled and used, never edited. Debug
products are quality proof only; Q02 rebuilds its own saved release candidate.
All202 saved crate/Web/Cargo/toolchain input files were byte-compared with139d202,
with zero mismatches. Node/script fixture inputs stay separately pinned.

Retained minimal public raw/canonical/request/config/report records:
`/private/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-w06-proof-e020gN`.
Named consumer Q02/root independent review; remove run-owned nonimages after that
consumption. No evidence staged. Own intermediate nonimages and source/build temp
are cleaned after checks; no image was generated, moved or deleted, and unrelated
after-title-spacing.png remains untouched. No Native foreground/resource claim.

Residuals: D06 timing/stage reporting remains Q02; independent acceptance remains
Q01. No shared-model blocker remains for these two workloads. Arbitrary browser/
OOPIF/shadow/template/private-whole-document support and known cross-frame transforms
are not claimed. Overall coverage remains partial; original frozen quality and
performance requirements remain intact.
