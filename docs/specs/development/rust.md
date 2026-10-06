# Rust development

- Node type: leaf
- Authority: Active
- Stability: Evolving
- Contract: `DEV.RUST@1`
- Authority source: user approval on 2026-10-06 of the five-file documentation plan.
- Read when: changing toolchain, Cargo, dependencies, features, targets, or checks.
- Do not read when: no Rust development infrastructure is affected.
- Requires: [Rust engineering rules](../../../RUST.md).

## DEV.RUST.SCOPE — Documentation foundation

This contract prepares Rust development. No Cargo package, toolchain pin,
supported target matrix, CI workflow, runtime, or framework is installed by it.
The initial implementation must resolve these choices against its approved
requirements. Do not infer them from the donor search service or product Drafts.

## DEV.RUST.TOOLCHAIN — Reproducible setup

When authorized Cargo setup begins:

- Record a concrete supported stable Rust version in `rust-toolchain.toml`,
  with rustfmt and Clippy. The moving `stable` channel is not a version pin.
- Select the edition, workspace resolver when applicable, and `rust-version`
  from actual language/API and dependency requirements; record the MSRV policy.
- Document supported host/target and feature combinations. Check only valid
  combinations; `--all-features` is not automatically a supported product.
- Commit `Cargo.lock` for the application/CLI workspace and use `--locked`
  for repeatable dependency-resolving checks once the lockfile exists.
- Make dependency updates deliberate; do not run `cargo update` as routine QA.
- Keep optional platform SDKs and dependencies out of unrelated builds.
- Add packages, async runtimes, test runners, and benchmark tools only when
  needed. No multi-crate layout or particular third-party library is imposed.

## DEV.RUST.CHECKS — Select checks by the actual change

The commands below are templates for a future Cargo project, not evidence of
checks already available or completed. Replace `PACKAGE` with a real package.
Use `--locked` after the initial authorized lockfile generation.

| Change / purpose | Command or evidence |
| --- | --- |
| Fast local edit loop | `cargo check --locked -p PACKAGE --all-targets` |
| Rust formatting | `cargo fmt --all -- --check` |
| Lint affected package | `cargo clippy --locked -p PACKAGE --all-targets -- -D warnings` |
| Local behavior | `cargo test --locked -p PACKAGE TEST_FILTER` plus affected scenarios |
| Package-wide behavior | `cargo test --locked -p PACKAGE` when the change warrants it |
| Shared or workspace risk | Expand to affected consumers; workspace commands below when needed |
| Changed benchmark target | `cargo bench --locked -p PACKAGE --no-run`, then relevant measurements |
| Documentation only | Changed local links, route consistency, `git diff --check` |

`--all-targets` means Cargo target kinds such as tests/examples/benches for the
selected platform, not every operating system or feature combination. Select
`--target` and feature flags only from the recorded supported matrix. Cross-build
success is not runtime acceptance on that platform.

When shared changes or a governing contract require workspace coverage, use:

```sh
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
```

Use the supported subset instead when host-specific members cannot build on
one host, and report platform evidence still needed. Do not require all platform
SDKs for an unrelated package change. Do not repeat an unchanged passing check
without a relevant input, environment, or implementation change.

## DEV.RUST.ACCEPTANCE — Evidence and limits

- Map behavior checks to the selected feature's clauses and plausible failures.
  Test observable results rather than mirroring implementation details.
- Use deterministic, bounded fixtures for pure logic. Adapter and integration
  evidence follows the feature contract; compiler success does not replace it.
- Optional watcher, dependency-audit, and benchmark tools are not prerequisites
  until a concrete need and policy establish them.
- Report actual commands/scopes and unresolved results; never imply checks ran
  before their build environment exists.
- This bootstrap is accepted through document/link checks and a checkpoint;
  it makes no claim about application build, runtime QA, or release readiness.

Adapted from the donor Rust infrastructure contract; search, database,
deployment, HTTP/gRPC, and mandatory full-suite policies are not transferred.
