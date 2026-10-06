# Rust workspace: T01 version boundary

Current concrete owner: [uiblueprint-schema](../../crates/schema/Cargo.toml).
Authority: [T01 packet](../plans/ui-blueprint/packets/T01.md), accepted C01
`42d2e6b059c5822476a39a59ac89a8a5c223db3a`, [DEV.RUST@2](../specs/development/rust.md)
and [handoff](../specs/development/decisions/handoff.md).
This is tooling for S01, not the product graph, plugin API, CLI or GOLDEN01.

## Setup and scope

[rust-toolchain.toml](../../rust-toolchain.toml) selects stable1.96.0 with rustfmt
and Clippy, minimal profile. [Workspace](../../Cargo.toml) selects edition2024,
resolver3, rust-version1.96 and centralizes exact serde/serde_json dependencies.
The [lockfile](../../Cargo.lock) records all actual resolved versions/checksums.
Initial checked host: aarch64-apple-darwin, macOS27.0.1. Compilation minimum is
the chosen policy1.96, not a claim that dependency source needs compiler1.96.
Older compilers/OS/architectures were not tested; no application runtime support
is implied. Global rustup default remains `stable`; the directory pin selects1.96.0.

If absent, install the named toolchain without changing global defaults:

```sh
rustup toolchain install 1.96.0 --profile minimal --component rustfmt --component clippy --no-self-update
```

There is one crate, no optional platform features or browser/native SDK, model,
async runtime or watcher. `publish=false`: no package publication authorized.
Build cache is ignored at `/target/`; no raw logs are committed. Dependencies and
their required notice files are inventoried in [dependencies](dependencies.md).

## Implemented boundary and independent examples

[Library](../../crates/schema/src/lib.rs) exposes `SchemaVersion`, `VersionDocument`
and payload-free `VersionError`. `VersionDocument::from_json(input,max_bytes)`
checks the caller's explicit byte cap before parsing one strict JSON object with
only `schema_version`. S01's actual resource caps remain D05-RES work; no universal
parser cap or cache policy is invented here. `to_json` serializes the typed value.
Serde is the JSON parser; lexical object-shape validation prevents its struct
sequence representation from admitting a JSON array.

| Authored input | Expected result |
| --- | --- |
| [current.json](../../crates/schema/tests/fixtures/current.json), `{"schema_version":"0.1.0"}` | Current typed identity; canonical identical JSON after round-trip |
| [incompatible.json](../../crates/schema/tests/fixtures/incompatible.json), `0.2.0` | IncompatibleVersion; never coerced to current |
| [malformed.json](../../crates/schema/tests/fixtures/malformed.json), `0.1` | MalformedVersion |
| Missing/unknown/duplicate key, null/bool/number, array, trailing document or truncated JSON | InvalidDocument |
| Byte limit equal to input length / one byte less | Accepted / InputTooLarge before decoding |
| Untrusted token/key/value in error path | Display/Debug/source chain retain no supplied payload |

The candidate version token grammar is three dot-separated unsigned ASCII decimal
components without leading zeros. Only exact0.1.0 is supported. Other canonical
numeric triplets are incompatible; suffixes/whitespace/Unicode digits are malformed.
This is not a general SemVer library or new compatibility promise. JSON string
escapes are decoded by serde; version identity is semantic, not raw byte spelling.
All rejection categories are local library errors, not finalized public CLI codes.

## Reproduce focused checks

From the repository root, after the committed lockfile exists:

```sh
cargo +1.96.0 check --locked -p uiblueprint-schema --all-targets
cargo +1.96.0 fmt --all -- --check
cargo +1.96.0 clippy --locked -p uiblueprint-schema --all-targets -- -D warnings
cargo +1.96.0 test --locked -p uiblueprint-schema
cargo +1.96.0 run --locked -p uiblueprint-schema --example version
```

Observed: all passed; six integration tests, no compiler/Clippy warnings. The
[executable example](../../crates/schema/examples/version.rs) exits0 and prints:

```text
{"schema_version":"0.1.0"}
incompatible schema version; supported version is 0.1.0
```

The example deliberately handles the rejected fixture; its successful process
exit does not mean the incompatible input was accepted. Do not substitute a
constant smoke test for [version contract tests](../../crates/schema/tests/version_contract.rs).
No unrelated suite or upstream suite was run. S01 extends this exact owner;
D02-PROOF, D05-RES, W01/M01/P01 and product compatibility freeze remain open.
