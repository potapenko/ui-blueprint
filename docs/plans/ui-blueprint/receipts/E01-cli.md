# E01-cli — public model-free export command candidate

## Authority, basis and ownership

Finite [E01-cli packet](../packets/E01-cli.md), saved at `ecde1ed`, under the
actual PLAN.UIB@1 approval `358c757` retained in the [registry](../task-registry.md).
Mode Restore; current master; one finite worker. Writes: `crates/cli/**`,
`docs/development/cli.md`, `docs/development/export.md`, this receipt. No agents,
other chats/worktrees, runtime/model calls, compiler/schema/engine or fixture edits.
Root lock belongs only to Integration. Core concurrently owns K01 design docs.

Reused the full current [E01 closure](E01.md#authority-and-traversal): registry4,
CLI/EXCHANGE/PRIVACY/MODEL/IDENTITY/BOUNDARIES@1 and full EXPORT/DRAWING@1 closure,
RUST.md, DEV.RUST@2, D03/D05@1, D07@2 and their selected explicit dependencies.
Current runbook and L01 contract/receipt read; operational-safety read for bounded
public diagnostics. No change to applicable AGENTS/spec/runbook paths since the
fully read basis. No semantic spec delta or new product-authority decision.

Required: a real public command, complete local package, explicit source/metadata,
independent statuses, finite IO, no reference dereference or overwrite. Observed:
E01 compiler `00b70cc` + saved membership `dbdccf6`, existing L01 binary and bounded
reader/writer. Finite syntax/output/exit engineering choices were documented in
[CLI contract](../../../development/cli.md#e01-public-imagegen-prompt-command)
BEFORE source implementation. They do not redefine normalized schema or intent.

## Shipping path implemented

`uiblueprint imagegen-prompt --brief FILE --out NEW_DIRECTORY` with all five
explicit input/output/component/view/detail limits calls the existing compiler
and package writer. DrawingBrief embeds canonical Snapshot or ProposedLayout;
metadata is not fabricated from a bare Snapshot. `--purpose` explicitly overrides
mode before compiler validation; default document and explain/propose/detail/flow/
compare use existing compiler gates. Blue Engineering is the only profile.

The six-file package and full A+B prompt are generated through the library.
Default stdout is compact. `--json` returns the documented export-owned
`result_version=0.1.0` package receipt, with tested exact field set, source/coverage,
counts, file list, bytes and independent statuses. This is not a second normalized
measurement/result record. `max-output-bytes` includes files and stdout including
newline; bounds are checked before writing. Existing destinations are refused.

The adapter shares L01 input/output machinery, changing only helper visibility
and adding export dispatch/output. Check/measure arithmetic, schema, error exits,
source JSON and existing consumer gaps are unchanged. Errors contain constant
codes only; no payload/path/serde error is printed. Invalid metadata, input limits,
sensitive content and failed writes cannot return success. If stdout fails after
package creation, exit1 reports IO error and the complete package is preserved.
No implicit references, pixels, source paths, URLs, model or runtime are accessed.

## Checks and exact inputs

Actual shared checkout, Rust1.96.0:

- `cargo check --locked -p uiblueprint-cli --all-targets`: passed.
- `cargo test --locked -p uiblueprint-cli`:19 binary tests passed:11 existing
  check/measure regressions and8 new actual-export-binary scenarios.
- One additional focused binary test for closed stdout passed after being added;
 20 distinct binary tests verified in total. Unchanged suites were not repeated.
- `cargo fmt -p uiblueprint-cli -- --check`: passed.
- `cargo clippy --locked -p uiblueprint-cli --all-targets -- -D warnings`: passed.

Export checks include full proposed and observed packages, versioned JSON shape,
all five modes/alias, document default, 32 observed nodes/17 relations, exact
97.296875css_px bound, partial/unknown status, full template, dense-sheet object
coverage, source/approval/image-status separation, invalid/missing/sensitive input,
sanitized diagnostics, aggregate output exact boundary, existing destination,
failed input/output IO and completed-package preservation after stdout failure.
The saved observed fixture remains historical F01 evidence; no new capture or
real PlayPhrase.me runtime acceptance. All test output is task-owned OS-temp and
cleaned by its case owner; no raw command evidence persisted.

Integration prepared exactly one local CLI dependency:
`uiblueprint-export = { path = "../export" }`. No external dependencies changed.
Root Cargo SHA256 `d23f77e04234ad8b193aefc7bec235a52d942c29fa5383ae1ca4da400c91abe3`;
lock SHA256 `15823a7d64777cff89bc6c38d013a57100aeb6d346b55ff8e124442d1a736c49`.
Shared input-set SHA256 `1018d7cb33c623f248fce9f314c384f0140ba285a897a0339cf4d3bea54c12c1`:
sorted compact JSON mapping root Cargo/lock plus every schema/engine/export `.rs`,
Cargo.toml and prompt-template.txt path to its file SHA256. Inspection HEAD was
`8103675f82104d547a98d3ae2d68c0ff22486100`. Protected shared source has no diff;
Integration-prepared lock still needs its separate checkpoint after this package.
Shared hash was rechecked unchanged after final scoped checks; changed local
links and scoped `git diff --check` passed.

## Residual and checkpoint handoff

The public **local brief → package** path is implemented and checked. Normal
live observe → stored Snapshot ID → export still depends on adapter/session/K01
owners and is not claimed complete. G02 attributed comparison, canonical
MeasurementResult/result-space integration, five shared review defects and D05
production-resource policy remain with their owners. E02 independent export
review, image generation/review and P6/P7 acceptance are separate and unclaimed.

Checkpoint: root granted the exact write set after `8103675`; master and empty
index verified. No Cargo/lock staging by E01. Shared hash equality was confirmed
immediately before the grant, with no intervening source changes;
package SHA, successful push and Git lease release return in the terminal receipt.

## G09-Export — direct observed file to document package

Current additive outcome, 2026-10-08: the earlier local --brief-only gap is closed
for an explicit canonical Snapshot or observed ChannelResponse file. Implicit
stored-ID lookup, live recollection, multi-channel merging and full P6 acceptance
remain outside this result. Earlier E01-cli checks/limits above are historical.

Authority: root's finite G09-Export dispatch under user-approved PLAN.UIB@1 P0–P7,
explicit parallel worker authorization and current read-only geometry priority.
Mode Restore EXPORT.CONTENT; `G09-EXPORT-INPUT-001` selects additive representation
under ROADMAP/P6 engineering authority. Before implementation, registered CLI@8,
CLI-EXPORT@1 and registry20; no new measurement/product intent or dependency.

Traversal: current AGENTS/work governance, registry19 → product branch → CLI@7,
EXPORT@1 and the complete previously read DRAWING/PROMPT/REVIEW/EXAMPLE closure;
GEOMETRY/PROJECTIONS/MODEL/EXCHANGE/IDENTITY/BOUNDARIES/PRIVACY dependencies reused
after saved no-diff check. ANALYSIS@2/TYPES/VALIDATION@1 read completely for direct
observed input and factual queries. RUST/DEV.RUST@2 remain unchanged. Updated
execution rules preserve current geometry priority and one review per finished
privacy/input boundary; no unrelated action/live/host route or full-suite preload.

Observed source: existing input::read_snapshot already validates Snapshot or
observed ChannelResponse through Document::from_json; it moves the unchanged
Snapshot. Existing export Metadata/ViewInput/SourceInput/compile/new-directory
writer already own document validation, privacy, geometry and output. Chosen
adapter: reuse that loader and compiler with one export-owned annotation record;
no second JSON parser, graph, compiler, geometry calculator or inferred labels.

Concrete command:

```sh
uiblueprint imagegen-prompt --snapshot observation.json --metadata metadata.json \
  --out "$TMPDIR/my-new-observed-package" --max-input-bytes 2000000 \
  --max-output-bytes 4000000 --max-components 256 --max-views 8 \
  --components-per-detail 12 --json
```

Metadata shape/example is in [export handoff](../../../development/export.md#observed-file-to-package).
It reuses full Metadata plus state/scope/environment/safe_source_reference,
not_depicted and public_text_fields; no source facts can be supplied through it.
Default direct purpose=document, explain alias; all other existing modes use
--brief. Missing metadata and mixed input forms reject. One input budget covers
both explicit files; existing compiler input cap and aggregate package/stdout cap
remain. User annotations are labelled; no date, fresh status, completeness,
padding, units conversion, arbitrary-ID meaning or accepted/checked image is guessed.
Canonical source is never rewritten. Six-file/versioned package and stdout shape
stay unchanged; no pixels or model calls, no persistent test directory creation.

Exact writes: crates/export/src/types.rs; crates/cli/src/{export,input,main}.rs;
crates/cli/tests/export_binary.rs; docs/specs/README.md, product/{README,cli,
cli-export}.md; docs/development/{cli,export}.md; this existing receipt.
No schema/engine/plugin/host/provider/fixture/Cargo/lock/root-coordination writes.

### Verification and source reconciliation

- `cargo check --locked -p uiblueprint-cli --all-targets`: pass.
- `cargo fmt -p uiblueprint-export -p uiblueprint-cli -- --check`: pass.
- `cargo clippy --locked -p uiblueprint-cli --all-targets -- -D warnings`: pass.
- `cargo test --locked -p uiblueprint-cli --test export_binary`:16 passed,0 failed;
  four new direct-input cases plus relevant existing --brief/modes/numeric repairs.
- Changed local links/route consistency and scoped whitespace checks pass.

First test pass exposed one historical assertion expecting every partial-scope
width/height to be unknown. Current saved known-anchor behavior (protected256f2a2)
retains explicit source geometry. Reconciled that stale assertion with independent
source width97.296875/height32 and contributing evidence; coverage/unknown-property
checks remain. No engine/source fixture or dimension was changed to obtain pass.

One actual public `cargo run ... -- imagegen-prompt --snapshot ... --metadata ...`
used the original saved F01 D05 overlay-on ChannelResponse, not a regenerated
DrawingBrief or fresh UI collection. Original bytes SHA256
`11e2a77cf6460636050e90ed0f6f63bcf3412c062bb3eb917d677be035858180`
remained unchanged. Output:32 source nodes,17 relations,32 explicit known dimensions,
partial coverage, unknown_count=null, unavailable properties retained,4 sheets,
all6 files and full A+B prompt. The particular caller-metadata run totalled629411
package bytes; image status unverified, approval draft. No machine path/payload_ref
entered the prompt. Source is our historical controlled F01, not real-site or
fresh PlayPhrase.me acceptance. Tests also cover direct Snapshot, failed/sensitive/
wrong-version/multiple responses, missing/private/geometry-injecting metadata,
text-policy refusal, unknown annotations, bounds, modes and baseline preservation.

All run output/metadata was under a unique system-temp directory. Removed only
its known6 text/JSON outputs and metadata after checking results, then its empty
directories; verified absence. No images created/deleted; old evidence untouched.

Checked relevant Rust-input set:72 files, SHA256
`a26df7203ee07b657c6e513915845d8205049eb4cfeab855031f99cddad29465`.
Recipe: sorted compact JSON path→file-SHA256 map of root Cargo.toml/Cargo.lock/
rust-toolchain.toml plus all .rs, Cargo.toml and prompt-template.txt under
crates/{schema,engine,export,cli}. Relevant provider source/Cargo diffs were empty;
inspection HEAD46ec1de includes other owner's disjoint saved Web work. No broad
workspace suite or new E02 numerical review was run. A focused finished input/
privacy review may follow; author checks do not claim independent acceptance.

Checkpoint-ready for short root Git grant on the12 exact paths above. Own code
is frozen pending save; existing shared owners and prior numerical fixes remain
protected. Saved SHA/push and Git release return in the terminal receipt.
