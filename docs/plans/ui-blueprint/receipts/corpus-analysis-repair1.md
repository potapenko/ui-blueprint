# CE1-FIX — duplicate size measurements and signed-zero display

2026-10-11. Finite `shipping_product` Restore packet under
[CORPUS-1000](../corpus-testing.md), following the confirmed
[CE1 findings](corpus-analysis-wave1.md). Consumer: subsequent corpus export batches.
The originating CE1-FIX dispatch explicitly authorizes these two fixes, focused
verification and current-master checkpoint/push; no further delegation or audit.
The original user mandate is to test diverse interfaces and fix discovered defects.
No semantic specification delta or new product requirement is introduced.

## Basis, evidence and executed plan

Traversal: global/repository AGENTS → implementation/product-truth/QA governance →
[registry37](../../../specs/README.md) → product/README →
[DESIGN-ANALYSIS@1](../../../specs/product/design-analysis.md), INPUT/TRUTH/OUTPUT/LIMITS.
Full selected closure (21 product/reference leaves): ANALYSIS@2, ANALYSIS-TYPES@1,
ANALYSIS-VALIDATION@1, GEOMETRY@1, MODEL@1, EXCHANGE@2, IDENTITY@1, PRIVACY@1,
PROJECTIONS@1, BOUNDARIES@1; EXPORT@2, CLI-EXPORT@2, CLI@16; DRAWING-PACKAGE,
STYLE, GEOMETRY, PROMPT-A, PROMPT-B, REVIEW@2 and DRAWING-EXAMPLE@1, plus
DRAWING@1.2 HBP-HUMAN-001/R2/R3. Older link labels resolve to the registry's
current leaves. No semantic revision drift. Engineering: RUST.md, DEV.RUST@2,
D01@1/D07@5, ROADMAP@1, RUST-BOUNDARIES@1, REUSE@1 and decision authority/evidence.
Global product-truth core/routing/change/evidence/delivery/coordination applied.
Excluded: collectors, actions, Native runtime, comparison-specific CLI contracts,
new design policy, source applications and real-image/visual acceptance.

Contract requirement: pattern operands are distinct measurements; full original
queries/results/evidence remain intact; human signed zero is rendered as 0.
Observed realization: engine pattern compared whole anchors, whereas Width/Height
arithmetic ignores fraction; export ratio_label preserved a minus sign on -0.0.
Source inspected: the two design owners, engine arithmetic and bound evaluation,
canonical Anchor fields, existing design tests and CE1 corpus repros. The original
CE1 receipt and its independently authored literal expectations remain protected.

Plan executed within the assigned five-file write set:

1. Enable the three ignored repros; add Height and valid distinct-anchor coverage.
2. Compare pattern anchor identity without fraction only for Width/Height.
   Preserve element, frame, full coordinate space and axis comparisons; Gap
   still compares fraction. No normalization/mutation of query inputs, no global
   deduplication, no change to compose or canonical geometry arithmetic.
3. Return human label `0` for either signed zero in ratio_label. Exact result
   remains IEEE -0.0; nonzero ratio formatting and arithmetic are unchanged.
4. Run focused design tests, affected-package Clippy and owned-file formatting;
   build an isolated model-free CLI for the next export consumer; checkpoint/push.

## Actual verification

Source at entry: master `ed6c60dba4df9053104d2f13bc03714e52f98397`.
Production design files still matched `c63b60029ec36a15865f8dcf11505241b80d9e45`.
Concurrent documentation checkpoint `d3ed404af597d352f7410e368f474a39ea6a0cc5`
changed no crates, Cargo manifests/lock or toolchain inputs.
Rust/cargo 1.96.0; default host aarch64-apple-darwin; unique system-temp target below.

- Before production edits: engine design_corpus **11 passed, 2 failed**, reproducing
  original Width and new Height false-majority cases; export design_corpus
  **9 passed, 2 failed**, reproducing false recommendation and signed-zero display.
- `cargo test --locked -p uiblueprint-engine -p uiblueprint-export --test design_corpus --test design`:
  exit 0, **36 passed / 0 failed / 0 ignored** (engine 13+8, export 11+4).
  All three former ignored repros now run normally and pass.
- Added checks: repeated Height with fractions 0/0.25/1 rejects; distinct Width
  and Height nodes with varying fractions retain exact values, original inputs,
  result spaces and full evidence. Same-element Gap anchors remain distinct
  when fractions matter; literal duplicate Gap still rejects. Export retains
  supplied fractions and safe aliases; machine -0.0 is checked by f64 bits while
  prompt.txt and drawing-brief.md both contain `0 times as large`.
- `cargo clippy --locked -p uiblueprint-engine -p uiblueprint-export --all-targets -- -D warnings`:
  exit 0, no warnings.
- Owned four Rust files: `rustfmt --edition 2024 --check`, exit 0.
- `cargo build --locked -p uiblueprint-cli --bin uiblueprint`: exit 0, default
  empty feature set. Binary built for local geometry/export; live adapters are
  not enabled or qualified by this build.
- Receipt local links and `git diff --check` pass. No workspace-wide tests/fmt,
  new schema/dependency/Cargo change, screenshot, ImageGen call or image cleanup.

## Binary handoff and scope protection

All Cargo commands used `CARGO_TARGET_DIR`:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uiblueprint-ce1-fix.SsmxEK`.
Fixed binary retained for subsequent corpus batches:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uiblueprint-ce1-fix.SsmxEK/debug/uiblueprint`.
SHA-256: `43eb93e7196de7be3ac889c55dc86f1e7d1e54ab372b58c629443abc08023717`.
Built source Git blobs: engine design `7a1a4ec9eade03f434fd7721c2e12a6197ab27a5`,
export design `c4603c24312a0de9dc123a99c7fb4a07eaddb058`; these are checkpointed
unchanged. The final chat receipt supplies the actual checkpoint/push identity.
Binary retention owner: CORPUS-1000 coordinator until subsequent export batches
finish or explicitly discard it. Other task-owned Cargo intermediates are removed.

Shared `target/debug/uiblueprint` is unchanged (SHA-256
`a4e9af497f060094cb3eca31a6afc362d1c7258e79d3837f7551aaaf024a55ed`).
`after-title-spacing.png` remains unopened and untouched. Concurrent documentation
changes were neither edited nor staged by this packet. Only the assigned files:
[engine source](../../../../crates/engine/src/design.rs),
[export source](../../../../crates/export/src/design.rs),
[engine tests](../../../../crates/engine/tests/design_corpus.rs),
[export tests](../../../../crates/export/tests/design_corpus.rs), and this receipt.
Git publication uses `/tmp/ui-blueprint-master-git.lock`, an empty-index gate and
exact-path staging on master; no branch/worktree change or force push.

Both confirmed defects are repaired in the shipping engine/export path. These
checks contribute zero real UI captures or accepted corpus images. CLI scenario,
fresh image acceptance, whole-workspace qualification and release acceptance are
not claimed; future batches can consume the attributed fixed binary above.
