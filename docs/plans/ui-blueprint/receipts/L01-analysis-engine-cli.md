# L01 factual analysis engine / CLI — checked candidate

Status: final coordinated migration checks passed; source frozen, checkpoint-ready.
This is author/cross-consumer proof, not independent or live/P1/P6 acceptance.

## Authority and pinned basis

[Packet](../packets/L01-analysis-engine-cli.md) `8b100c2`, approved PLAN.UIB@1 and
registered L01-ANALYSIS-001 `7a61a65`/handoff `8fdf608`; current master, no nested
agents/chats/branches/worktrees. Exact owned source/doc paths below; no scope growth.

Read registry6 → complete ANALYSIS/TYPES/VALIDATION@1 (all clauses), D03@2 and
concrete [schema API handoff](../../../development/schema.md#s01-analysis-api-slice--concrete-consumer-handoff).
Reused current full CLI/GEOMETRY/EXCHANGE/MODEL/PRIVACY/IDENTITY/BOUNDARIES and
D03 dependencies; D05@2/D05-MEMORY@1, RUST.md/DEV.RUST@2/D01 and required closure.
Read changed D07@3 entirely and registry7/decision-route delta for ANALYSIS-FLOAT-001.
Requirements are the registered contracts; provider code supplies concrete APIs;
no new product arithmetic, source facts or policy is inferred from implementation.
Mode Restore the registered representation/verification contracts.

Ready saved providers: initial types `67d7e49a0072eead590c65e379742554f5591fb8`, then
schema/fidelity `b68b5b718d0899f1bfd2d6ec78011c1ebcf49479`; Export adaptation `264a838`.
Integration owns canonical parsers/validators/accounting/fixtures/feature choice.
Core never edited schema/Cargo/lock/export/cache/replay/old oracles or specs.

## Concrete result

- measure_query accepts canonical factual GeometryQuery; measure(Expectation)
  faithfully extracts query fields from the actual requirement. No placeholders.
- Canonical Measurement/result/details/reason re-exports replace duplicate DTOs;
  schema owns arity/quantity/axis vocabulary, engine owns the sole arithmetic.
- Bound measure/check validate Snapshot ID/revision/full Context and supplied
  evaluation before borrowed adaptation. No fabricated/supplemental Observation.
- verify_analysis_result validates declarations then recomputes exact quantity,
  full Space/details/ordered evidence/reason/Finding semantics; only assigned
  Finding.id may differ. Contract-valid false declarations are not trusted results.
- Sensitive-redacted inputs keep attempted public Evidence without exposing value;
  distinct/unused inputs preserve first-use/full-equality provenance semantics.
- Actual CLI supports query/evaluation inputs, factual measure JSON0.2 and explicit
  converted/conditional check JSON0.2. Default/explicit check0.1 remains protected.
  Version/representation refusal is unsupported_result_version/5; no downgrade.
- Named files share one pre-parse aggregate byte limit; canonical validation and
  bounded encoding precede stdout. Exact0.2 round-trip equality detects fidelity
  failure without epsilon. Compact query mode invents no normative fields.

API details/examples: [geometry](../../../development/geometry.md) and
[CLI](../../../development/cli.md). The existing E01 command documentation and
implementation are preserved; its own owner migrated its observed measure caller.

## Final migration barrier — Core runner

Root confirmed saved providers and sole-runner permission after source readiness.
Final metadata HEAD `de742e6a0a66d33d522ae943a8c9f299249b5b17`, with Core candidate;
Rust1.96.0. Once, in order, all passed:

```sh
cargo check --locked --workspace --all-targets
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
```

| Package | Passed / failed | Coverage |
| --- | --- | --- |
| schema |32 /0 | legacy22 + analysis contract8/validator CLI2 |
| plugin-api |14 /0 | depth/common support/lifecycle |
| engine |52 /0 | analysis12, geometry14, replay11, cache13 + counter2 |
| export |21 /0 | compiler14/proposal regressions7 |
| CLI |31 /0 | analysis8/legacy11/export12 |

Total150 passed,0 failed/ignored, plus1 borrower compile-fail doctest. Original
K01 E0502/positive-control independent proof remains saved, not re-created here.
No source repair or repeated workspace suite occurred inside this barrier.

Before/after281-file SHA256 identical:
`6fe25292b369125b0c988de632aee77b69fa2de83d75a805234e682b4947f96e`.
Map includes root Cargo/lock/toolchain, all regular files under crates,
fixtures/{golden,golden-oracles,analysis,export}, schemas, tests/bridges/{common,
resources}, .cargo if present. Digest is SHA256 of compact sorted JSON path→SHA256.
Own12-source-file digest:
`02a18ba1c002a24a60221a855e4dbd5984cd8e22997daf8a0ab8a4c397ff0d80`.
Cargo.toml SHA256 `1df267dd297e776644e380192198ab919b8fff1f6ae8383841e589ef77cc821b`;
Cargo.lock `15823a7d64777cff89bc6c38d013a57100aeb6d346b55ff8e124442d1a736c49`.
Runtime float_roundtrip is provider-owned D07@3, same serde_json/version set. The
provider's production22-case probe was reused, not repeated; actual Core CLI bit
vector and exact parsed-result recomputation also passed. No tolerance workaround.

## Exact checkpoint / limits

Own15 paths: engine src/{lib,arithmetic,resolve,analysis}.rs; engine tests/{geometry,
analysis}.rs; CLI src/{arguments,input,output,main}.rs; CLI tests/{binary,
analysis_binary}.rs; docs/development/{geometry,cli}.md; this receipt.
Source unchanged after barrier; only docs/receipt updated. Links/route/whitespace
checks passed. Root granted exactly these15 paths after the common PASS; no suite
repeat. Terminal chat returns Core SHA/push, saved281-input equality and Git release.
Source remains frozen for independent review; author checks are not acceptance.
No cache/replay migration, live collection/condition acquisition/transform discovery,
new model/SDK/dependency, generic diff or D05 working-memory enforcement claimed.
Source Snapshot and core transport versions remain0.1; only analysis envelope is0.2.
