# Rust development

- Node type: leaf
- Authority: Active
- Stability: Evolving
- Contract: `DEV.RUST@3`; supersedes `DEV.RUST@2` development-check policy.
- Authority source: direct user instruction on 2026-11-14 to add shared lints,
  pull-request CI and current documentation; original setup follows
  [C01-DEC-001](decisions/README.md#meaning-and-precedence).
- Read when: changing toolchain, Cargo, dependencies, features, targets, or checks.
- Do not read when: no Rust development infrastructure is affected.
- Requires: [Rust engineering rules](../../../RUST.md), [D01](decisions/d01-support.md),
  [D07](decisions/d07-reuse.md).

## DEV.RUST.SCOPE — Current workspace

The workspace contains schema, plugin-api, engine, export, CLI, host and Web
packages. It pins Rust `1.96.0` with rustfmt/Clippy, edition `2024`, resolver `3`,
rust-version `1.96` and `aarch64-apple-darwin`. D01 remains the initial setup
decision; its T01/S01 pending statements are historical, not current build status.
This change preserves toolchain, dependencies, product behavior and release scope.

## DEV.RUST.TOOLCHAIN — Reproducible setup

- Keep a concrete stable version in `rust-toolchain.toml` with rustfmt and Clippy.
  The moving `stable` channel is not a version pin.
- Select the edition, workspace resolver when applicable, and `rust-version`
  from actual language/API and dependency requirements; record the MSRV policy.
- Document supported host/target and feature combinations. Check only valid
  combinations; `--all-features` is not automatically a supported product.
- Preserve the committed `Cargo.lock`; use `--locked` for dependency-resolving checks.
- Make dependency updates deliberate; do not run `cargo update` as routine QA.
- Keep optional platform SDKs and dependencies out of unrelated builds.
- Add packages, async runtimes, test runners, and benchmark tools only when
  needed. No multi-crate layout or particular third-party library is imposed.

## DEV.RUST.CHECKS — Select checks by the actual change

These commands apply to the existing workspace. Replace `PACKAGE` with an actual
package name, such as `uiblueprint-engine`; commands alone are not passing evidence.

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
cargo build --locked --workspace --bins --examples
cargo test --locked --workspace -- --test-threads=1
```

Use the supported subset instead when host-specific members cannot build on
one host, and report platform evidence still needed. Do not require all platform
SDKs for an unrelated package change. Do not repeat an unchanged passing check
without a relevant input, environment, or implementation change.

## DEV.RUST.CI — Shared lints and pull requests

All seven packages inherit `[workspace.lints]`: `unsafe_op_in_unsafe_fn` and
`unused_must_use` are denied; Clippy `dbg_macro` and `undocumented_unsafe_blocks`
warn. Keep existing crate unsafe prohibitions and narrow justified exceptions.
[PR CI](../../../.github/workflows/ci.yml) runs on GitHub-hosted `xcode-27`
(macOS ARM64), with read-only contents permission and no persisted credentials.
It checks fmt, Clippy with `-D warnings`, builds test peer binaries/examples,
then runs tests serially for each explicit CLI feature selection: none, `web`,
`macos`, and `web,macos`. For local reproduction add the same qualified
`--features uiblueprint-cli/web,uiblueprint-cli/macos` to build/check/test commands
for the combined selection; select either one alone for its single-feature job.
Live browser/native and performance tests remain explicitly ignored; CI does not
grant runtime permissions, qualify other platforms or publish a release.

## DEV.RUST.ACCEPTANCE — Evidence and limits

- Map behavior checks to the selected feature's clauses and plausible failures.
  Test observable results rather than mirroring implementation details.
- Use deterministic, bounded fixtures for pure logic. Adapter and integration
  evidence follows the feature contract; compiler success does not replace it.
- Optional watcher, dependency-audit, and benchmark tools are not prerequisites
  until a concrete need and policy establish them.
- Report actual commands/scopes and unresolved results; never imply checks ran
  before their build environment exists.
- Shared policy changes require the affected feature matrix, workflow validation,
  changed-link checks and a checkpoint. Local results do not prove a hosted run;
  source CI does not replace adapter runtime QA or establish release readiness.
