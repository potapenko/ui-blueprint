# R03 selected-field replacement experiment

Diagnostic only; immediate consumers: C01/D03, S01 and K01. This is one private,
single-node in-memory counterexample, not a competing schema or production cache.
No external crates, Cargo workspace, live adapters, UI, input or secrets.

## Question and independently specified oracle

Can a selected-field update be merged into an older full node and the combined
node be called fresh? **No**, under `UIB.CACHE.CONTENT` and `UIB.EXCHANGE.CONTENT`.
The following expected states are specified from those contracts, not generated
by the experiment. All fixtures fix target/surface generations, scope,
projection and schema/plugin version; those compatibility axes are not tested.

| Fixture/input | Expected result |
| --- | --- |
| Revision 7: enabled=true, label="old"; same fields replace with false and "" | Revision 8: known(false), known(""); neither is absent |
| Revision 8: requested label becomes unknown / redacted / unsupported | Revision 9: corresponding status with no retained current value |
| Wrong base 6 against revision 8 | resync_required; whole current sample unchanged |
| Narrow delta fields=[enabled] against [enabled,label] | resync_required; whole current sample unchanged |
| Same declared fields but replacement omits label | Reject incomplete replacement; whole sample unchanged |
| New full revision 9 for fields=[enabled] | label selection=not_requested; revision-7 historical label stays revision 7 |
| Counterfactual: merge narrow full into revision 7 and stamp revision 9 | Incorrectly exposes the old label as requested/current; differs from oracle |

`Known` carries a typed value; unavailable states cannot carry one. Selection is
separate from availability. The map shape is only a fixture shorthand; it does
not propose dynamic property storage or public error names for S01. The helper
assumes a small fixed revision, canonical sorted field list and one node. It
does not implement removal, relations, focus, privacy canaries, bounded cache,
serialization, nullable values or concurrency. Assertions are the executable
experiment; no performance claim follows from them.

## Reproduce

Observed host: macOS arm64, installed `rustc 1.96.0 (ac68faa20 2026-05-25)`,
edition 2021. No toolchain installed or project toolchain selected; C01 owns that
decision. Commands run from the repository root with the same compiler version:

```sh
run_dir=$(mktemp -d /tmp/uib-r03-check.XXXXXX)
rustc --version
rustfmt --edition 2021 --check experiments/core/selected_update.rs
rustc --edition 2021 -D warnings experiments/core/selected_update.rs -o "$run_dir/selected-update"
"$run_dir/selected-update"
rm "$run_dir/selected-update"
rmdir "$run_dir"
```

Expected stdout is exactly:

```text
R03 selected-update: expected states verified; naive fresh merge falsified
```

Exit 0 means every listed assertion held. Changed local Markdown links and
`git diff --check` additionally cover the documentation. These checks do not
accept any runtime platform, full/delta graph replay, GOLDEN01, or product build.
