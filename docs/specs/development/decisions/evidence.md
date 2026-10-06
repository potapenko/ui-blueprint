# C01 evidence basis

- Domain: `uib.development.evidence`; accepted/released baseline: none.
- Authority source: [C01-DEC-001](README.md#meaning-and-precedence).
- Node type: leaf; contract: `UIB.C01-EVIDENCE@1`; clause: `UIB.C01-EVIDENCE.CONTENT`.
- Authority: Active / Stability: Evolving as an evidence index, not product intent.
- Read when: resolving provenance or a remaining proof obligation.
- Do not read when: a decision already provides sufficient applicable evidence.
- Requires: [decision authority](README.md#meaning-and-precedence).

## Accepted supporting inputs

| Input / checkpoint | Decision-grade observation | Limit |
| --- | --- | --- |
| [R01](../../../research/R01-web.md) `71ea44235cddf63381d1fea4acd121ad642e7ddf` | Chromium/CDP, DOM/CSSOM/AX; 48 author checks, repeated labels/remount/frame/target rejection | Whole DOMSnapshot is not bounded acquisition; no arbitrary frames or production privacy proof |
| [R02](../../../research/R02-native.md) `19cd1359ff4c239695f5da21f90d101d878c067d` | Public AX/ScreenCaptureKit and Swift measured probe feasible on actual host | Unknown transforms, expanded pixels and probe invariance incomplete |
| [R03](../../../research/R03-core.md) `26a19faed84b468dd21f77bf2c78516cd917d223` | Typed selected-field replacement counterexample; source algorithms/license limits | Only U1 executed; no graph engine/runtime |
| [F01](../../../development/fixtures-web.md) `a8368076cdf0d4a917e7c3c5448ca6b6d919cf64`, correction `585bcd90ad455110bcc2432f8f36f58e3c8d7ecf` | 58 fixture checks; per-request cold/warm baseline; independent oracle | Historical saturation/5 Hz/rAF excluded from product gates |
| [F02](../../../development/fixtures-native.md) `9a88b12b5855bac54bf04ba7b64d233df719ddec` | One-shot current probe, same-title/reopen cases and sequential API baseline | Concurrency failure 124; old/new probe evidence cannot be mixed |

Root accepted these as diagnostic/tooling evidence, not independent product QA.
Receipts link exact source/fixture hashes and retention; no original evidence
modified by C01. Documentary source reviews are not runtime tests.

## Retained records inspected directly

Under `/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/P0/`:

- `F01/run-fd3c8796-2914-45bc-94d3-830e2e465818/report.json`: environment,
  source hashes and individual per-request samples. SHA-256
  `e2af888ac0aa758edb6d9e141de8f261b1d3824e641aa8b6d50e06f671a9d123`.
- `F02/51b19df6-76de-4034-9949-933134c868d0/`: `run-summary.json`,
  `request-model-addendum.json`, `request-validation.json`,
  `final-off-cold/summary.json`, `final-on-cold/summary.json`,
  `final-off-warm/summary.json`, `final-on-warm/summary.json` and their
  `run-*/external-*.json` node-count/coverage/capture-size metadata.
- F02 `concurrent-summary.json`: both helpers exit 124 after 45-second watchdog,
  unresumed checked continuation diagnostic, completed AX not persisted. Last
  successful capture series was sequential `final-on-warm`. This is a real failure.
- F02 `normalized-comparison.json`: matching role/state/frame/focus but off
  active/key/main=true versus on=false. Pixel pair **not comparable**, not proof
  of interference or invariance. Current one-shot probe still needs P01 acceptance.

Additional precision over the F02 narrative: cold external records contain
**74–75 nodes**, warm records 75; all label AX coverage `partial`. Pixel buffers
are 1100×1050. Do not relabel this as complete native layout. Per-call API timings
precede the one-shot publisher correction and are exploratory, not current P7
candidate results. The current request-validation record proves two explicit
requests with three markers, no publication between them, and gap 8→18 pt.

Retention remains root-owned through P7 acceptance or explicit discard. C01
creates no copies of raw records or new permanent telemetry.

## Technical primary evidence, checked 2026-10-06

- Local `rustc --version --verbose`: 1.96.0, commit
  `ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96`, host aarch64-apple-darwin,
  LLVM 22.1.2. Local Cargo is 1.96.0 (30a34c682 2026-05-25).
- [Official 1.96.0 release manifest](https://static.rust-lang.org/dist/channel-rust-1.96.0.toml)
  confirms stable release date 2026-05-28 and aarch64-apple-darwin artifact.
- [Rust 2024 resolver](https://doc.rust-lang.org/stable/edition-guide/rust-2024/cargo-resolver.html)
  and [Cargo resolver](https://doc.rust-lang.org/cargo/reference/resolver.html)
  support edition 2024/resolver 3 and Rust-version-aware dependency resolution.
- [serde 1.0.229 metadata](https://crates.io/api/v1/crates/serde/1.0.229) and
  [serde_json 1.0.151 metadata](https://crates.io/api/v1/crates/serde_json/1.0.151),
  [Serde API](https://docs.rs/serde/1.0.229/serde/) and
  [JSON API](https://docs.rs/serde_json/1.0.151/serde_json/) support D07's bounded
  serialization dependency choice. Metadata was read; no dependency installed.

## Product context and discrepancy dispositions

[User/advisor consultation](../../../plans/ui-blueprint/receipts/product-context.md):
RC01 two-layer in-window Mac Settings; RC02 Genre/Director anchored popovers;
RC03 Mac resize; RC04 iPad visible/accessibility labels; RC05 iPhone accessibility
reflow. The sourced inventory suffices for C01 context; F03/Q03 own captures and
agent evaluation. Mobile implementation remains outside P0–P7.
During C01, [F03a case data](../../../development/real-world-cases.md) became
available and was read as additional candidate evidence, not accepted adapter QA.
Settings observations SHA-256 `16e5a2199168b21bfcf3a2de9c3b5b4bbc9c82dad1117c9833a05e26756eb8c0`;
filters observations `cd8ee7ce5261e9040373d63f7ef64c78653068bd43a0af9d1fd8504280a4e512`.
They retain unknown per-element bounds and binary/source correspondence; popup
AX-to-CG matching is temporal/visual inference. Genre's AX buttons do not expose
checked state; Director's omitted value is unknown, not known empty. These are
useful S01 evidence/availability examples, not permission to infer geometry or
relabel source roles. Exact AX times are absent, so no D06 timing use. No source
application, image capture or other project's files were operated/read by C01.
Cadence assumptions: stale/inapplicable evidence, excluded. Concurrent capture:
implementation defect in supporting helper, M01 repair/proof obligation.
Transforms/probe/popup proof: missing evidence with named owners, not product forks.
