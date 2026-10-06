# Repository instructions

This file contains local routing only. The operator's global `AGENTS.md` and
its conditional governance documents remain applicable; do not copy them here.

## Read only the applicable route

- Product behavior, APIs, state, compatibility, product QA, or specification
  work starts at [the specification registry](docs/specs/README.md).
- Rust source changes also require [RUST.md](RUST.md).
- Toolchain, Cargo, dependencies, features, build targets, or development-check
  changes also require [Rust development](docs/specs/development/rust.md).
- New product contracts use [the feature template](docs/specs/templates/feature-spec.md)
  and are registered before implementation. A template is not product authority.

Follow selected contracts and their explicit dependencies, not every sibling.
Product meaning belongs in the specification tree; Rust engineering rules
belong in `RUST.md`; development setup and command applicability belong in
`docs/specs/development/rust.md`.

## Bootstrap boundary

This repository currently contains documentation, including imported Draft
product proposals. Importing or indexing them does not approve their proposed
architecture, APIs, package layout, dependencies, or implementation stages.
Use the registry to distinguish their authority from active engineering rules.

Do not create empty crates or select frameworks merely to match a draft
directory tree. Introduce concrete owners when an authorized implementation
needs them.

## Documentation verification

For documentation-only changes, check changed local links, route consistency,
and `git diff --check`. Rust commands apply only once a Cargo project exists
and the change affects that layer. Do not claim builds or tests passed merely
because their commands appear in documentation.
