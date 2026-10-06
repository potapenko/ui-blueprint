# E02 proposal geometry repair

- Class: shipping_product; existing Export owner, inherit model/reasoning.
- Authority: approved PLAN.UIB@1 plus user's clarification «Какой ответ от меня
  нужен? Уже всё же обсудили уже.» Root removed the mistaken implementation wait;
  see execution.md. Reviewers remain read-only. Current master, no nested agents.
- Consumer: correct public imagegen-prompt proposal validation; same E02 reviewer
  performs affected recheck after the saved candidate and author receipt.
- Mode Restore, no product-spec delta or expansion. Basis: full E01/E01-cli closure,
  EXPORT/CLI/DRAWING/GEOMETRY/MODEL/PRIVACY@1 CONTENT, RUST/DEV.RUST@2, D03@1;
  root fully read closure already. Read changed D05 nodes only if applicable.

Repair the two findings in receipts/E02-candidate-review.md at artifactfe06537:
E02-R1 vertical anchors must honor declared origin; E02-R2 arithmetic roundoff
must not reject valid fractional geometry. No hidden UI measurement tolerance,
decimal truncation or blanket epsilon that accepts genuinely wrong dimensions.
Preserve explicit tolerance semantics, exact source values, all modes and statuses.

Writes only crates/export/src/proposal.rs, a narrow export-owned arithmetic helper
if genuinely needed, export proposal tests (new focused file or existing test file),
crates/cli/tests/export_binary.rs for actual-command regression, and
docs/plans/ui-blueprint/receipts/E02-repair.md. No changes to CLI production code,
schema/engine/plugin APIs, manifests/lock, existing fixtures, product specs or
unrelated compiler features. Broader dependency returns to root.

Focused checks: both origins and directions across unequal-height rectangles;
fractional x0.2,width0.1 and other magnitudes; truly incorrect dimensions still
reject; explicit tolerance remains separate. Verify valid proposal succeeds and
invalid fails through actual CLI without losing unknown/status/source facts.
Scoped Rust1.96 check/fmt/Clippy, export tests and affected binary cases. Shared
schema validation is being repaired by Integration: record input hashes, coordinate
final check barrier via root, do not claim unchanged checks against changed inputs.

Do not alter source snapshots or expected values to make tests pass. No model/UI
runtime, new dependencies, branch/worktree, other chat or broad hardening.
Git lease when ready, exact-path commit+push, SHA/checks/shared inputs/limits and
release. Preserve reviewer independence; no self-acceptance or next packet.
