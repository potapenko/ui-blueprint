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

The user confirmed the product specification and visualization guide on
2026-10-06. They are Active requirements; implementation and release acceptance
remain separate. Explicitly open engineering choices and future phases retain
their stated scope. Use the registry for the current authority and revisions.

The [development plan](docs/plans/ui-blueprint-development.md) is proposed
execution scope until the user approves and launches it. For coordinated work,
follow its selected execution route and task registry. Planning does not start
workers, a persistent goal, or implementation.

Do not create empty crates merely to match a proposed directory tree. Introduce
concrete owners when an authorized implementation needs them.

## Documentation verification

For documentation-only changes, check changed local links, route consistency,
and `git diff --check`. Rust commands apply only once a Cargo project exists
and the change affects that layer. Do not claim builds or tests passed merely
because their commands appear in documentation.
