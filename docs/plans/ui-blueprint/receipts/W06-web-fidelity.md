# W06 Web fidelity — delivered for Q02 qualification

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
