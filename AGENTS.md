# Repository instructions

This file contains project-specific instructions. The operator's global `AGENTS.md` and
its conditional governance documents remain applicable; do not copy them here.

## Standing authority for UI Blueprint source releases

The user's instruction of 2026-11-23 (message `01a208f6-bbc6-7062-bf36-b0016c78c735`)
replaces per-release confirmation for the UI Blueprint source channel. Implement
requested fixes, select the version, verify the exact revision and install it
through the managed installer. Publish the verified revision promptly as a
source-only GitHub tag and release in `potapenko/ui-blueprint` without asking
again. Check the published tag and send waiting teams the version, commit and
tag-pinned installation or SDK update instructions. Existing approval rules
still govern other projects, production services, binary distribution and host reboots.

## Communication for bug-fix chats

Open a bug-fix chat when the request identifies a UI Blueprint failure to fix or explicitly asks to add missing tool support. Requests for an existing command, a screenshot, a measurement or a QA run belong in the requesting team's existing chat.

For each new defect, follow the [bug chat onboarding and handoff](docs/plans/ui-blueprint/execution.md#bug-chat-handoff).
Create a regular user-visible project chat, give it the context to prepare its
own implementation plan, and exchange direct chat links with the reporter.
The coordinator's assignment ends when both chats have each other's link.
Blueprint chats fix and regression-test the tool on project-owned fixtures. The reporting team runs all measurements and UI acceptance on its application, owns its windows, and changes its application code. Archive completed fix chats and measurement-only chats.

The builds and releases chat (`01a20929-8e9a-7830-834d-3e558e9ff18d`) owns
packaging, CI, local installation, publication and release verification for
all accepted fixes. It takes over existing release runs and notifies the
teams waiting for the fix once the published version is ready to use.

## Read only the applicable route

- Product behavior, APIs, state, compatibility, product QA, or specification
  work starts at [the specification registry](docs/specs/README.md).
- Rust source changes also require [RUST.md](RUST.md).
- Toolchain, Cargo, dependencies, features, build targets, or development-check
  changes also require [Rust development](docs/specs/development/rust.md).
- New product contracts use [the feature template](docs/specs/templates/feature-spec.md)
  and are registered before implementation. A template is not product authority.
- Coordinating the user-authorized goal or managing its worker chats starts at
  [the orchestration runbook](docs/plans/ui-blueprint/execution.md) and the
  [task registry](docs/plans/ui-blueprint/task-registry.md). The runbook records
  this goal's authorization; it does not authorize delegation in unrelated tasks.

Follow selected contracts and their explicit dependencies, not every sibling.
Product meaning belongs in the specification tree; Rust engineering rules
belong in `RUST.md`; development setup and command applicability belong in
`docs/specs/development/rust.md`.

## Bootstrap boundary

The user confirmed the product specification and visualization guide on
2026-10-06. They are Active requirements; implementation and release acceptance
remain separate. Explicitly open engineering choices and future phases retain
their stated scope. Use the registry for the current authority and revisions.

The [development plan](docs/plans/ui-blueprint-development.md), `PLAN.UIB@1`, was
approved and launched on 2026-10-06 for P0–P7. The actual approval, original
plan revision and active ownership are preserved in the
[task registry](docs/plans/ui-blueprint/task-registry.md). Root is coordination-only;
finite workers execute assigned packets through the selected execution route.
Future stages retain their separate scope. A plan or checkpoint is not acceptance.

Do not create empty crates merely to match a proposed directory tree. Introduce
concrete owners when an authorized implementation needs them.

## Documentation verification

For documentation-only changes, check changed local links, route consistency,
and `git diff --check`. Rust commands apply only once a Cargo project exists
and the change affects that layer. Do not claim builds or tests passed merely
because their commands appear in documentation.
