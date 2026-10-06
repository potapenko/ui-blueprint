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
