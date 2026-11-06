# Numeric versions and GitHub release preparation

- Node type: leaf; domain: `uib.distribution`; contract: `UIB.RELEASES@2`.
- Authority: Active; stability: Evolving; released baseline: none.
- Authority source: this chat's five-point release plan (VERSION/CLI, changelog,
  workflow, tagged assets, installation guide), approved by the user on2026-11-06:
  “Да, план подтверждаю.” This authorizes local implementation and verification,
  not pushes, tags, remote builds or publication without the exact release gate.
  Later on2026-11-06 the user selected MIT and explicitly rejected binary release
  attachments: releases identify verified source; prompt installation builds locally.
- Read when: product version, release preparation/workflow or release instructions.
- Requires: [MAC-DISTRIBUTION@6](macos-distribution.md), [CLI@21](../product/cli.md),
  [AGENT-SKILL@6](agent-skill.md), [DEV.RUST@2](rust.md).
- Precedence: this approved scope permits the named README, CLI and workflow edits
  previously excluded by the older preparation-only contract. Other guards remain.

## RELEASES.VERSION

The root VERSION file is the single product release version: canonical numeric
MAJOR.MINOR.PATCH, beginning0.1.0. Bug fixes increment PATCH; new capabilities
increment MINOR while the product is pre1.0. No Web/native/browser suffixes in new
product releases. Internal crate and wire/schema versions remain independent.
All utility/worker/helper/skill assets in one release share this version and source
commit. `uiblueprint --version` prints `uiblueprint X.Y.Z` plus newline, exit0,
without opening a source/app or loading input. Other CLI behavior stays unchanged.

Release preparation reads VERSION and the matching CHANGELOG section from the
selected committed revision. An explicit --version must match it. Missing,
noncanonical or mismatched versions/changelog refuse before building. Historical
candidate verification and installed prerelease versions remain supported; do not
rename old directories, rewrite evidence or break rollback.

## RELEASES.WORKFLOW

`.github/workflows/release.yml` is manual-only, with exact numeric version and full
commit SHA. No push/tag trigger, automatic public release or mutable latest source.
The workflow validates the selected source/version, installs the pinned toolchain
and fetches locked dependencies in its ephemeral runner, then reuses release.py.
Build and temporary installation checks precede an optional source-only GitHub
draft. The draft names tag vX.Y.Z and the exact commit, with changelog notes and
prompt-install instructions; it has no uploaded assets. GitHub's automatic source
archives are source, not binary packages. An existing release/tag is refused.
The build job has read access; only an explicitly selected draft job receives
contents:write. Actions are SHA-pinned. No binary artifacts are uploaded by CI.
Public source publication requires the exact deployment confirmation. Apple signing,
notarization and Gatekeeper are not release gates for this source-only channel.
The older Mac binary distribution rules still govern separately requested binaries.
No signing credentials or Apple services are used for the source release.

## RELEASES.INSTRUCTIONS / ACCEPTANCE

Document one version bump, checkpoint, local build/verify, reviewed source and
manual workflow/draft sequence. Published installation resolves one exact release
tag and commit for its instructions, local build and matching skill. curl fetches
instructions/release metadata, not a prebuilt executable. The installer uses the
existing local-only candidate path; there is no published install.sh or binary
archive dependency. Before the first release, explicit developer pins remain valid.
Task writes: VERSION, CHANGELOG.md, release.py, distribution.py, CLI version code
and focused tests, .github/workflows/release.yml, release/installation docs and
these spec routes. Existing collection, data formats, installation ownership,
platform qualification, images and unrelated working changes are protected.

Acceptance: exact CLI/version output; reject malformed/mismatched versions before
build; current and historical candidate verification; workflow syntax and pinned
source/permissions/no-auto-publication checks; real local combined candidate and
temporary install smoke. Remote Actions/download acceptance remains pending until
explicit remote authorization. No claim of independent review or public release.

## RELEASES.LICENSE — direct user selection

Project source is MIT, Copyright2026 Eugene Potapenko. Preserve third-party notices.
Root LICENSE supplies GitHub's license; the existing bundled notice file and skill
entrypoint carry the complete same text so local package inventories/rollback do
not change. New candidate metadata and the optional local formula declare MIT.
License inclusion is verified from the selected source before local packaging.
Historical receipts retain their original licensing/preparation status.
