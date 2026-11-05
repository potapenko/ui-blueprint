# Numeric versions and GitHub release preparation

- Node type: leaf; domain: `uib.distribution`; contract: `UIB.RELEASES@1`.
- Authority: Active; stability: Evolving; released baseline: none.
- Authority source: this chat's five-point release plan (VERSION/CLI, changelog,
  workflow, tagged assets, installation guide), approved by the user on2026-11-06:
  “Да, план подтверждаю.” This authorizes local implementation and verification,
  not pushes, tags, remote builds, uploads, license selection or publication.
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
Build, verify, installation checks and checksummed artifacts precede any optional
GitHub draft. The draft names tag vX.Y.Z, exact source and prepared assets; an
existing release/tag is refused, never overwritten. The build job has read access;
only an explicitly selected draft job gets contents:write. Actions are SHA-pinned.
Public publication remains a separate operator step after license, signing and
notarization requirements and exact deployment confirmation. A draft is not public
acceptance. No signing credentials or new signing identities are provisioned here.

## RELEASES.INSTRUCTIONS / ACCEPTANCE

Document one version bump, checkpoint, local build/verify, reviewed source and
manual workflow/draft sequence. Published installation resolves one exact release
tag for all downloads; until publication the existing pinned-source fallback stays
truthful. No broken claim that binaries are already on GitHub.
Task writes: VERSION, CHANGELOG.md, release.py, distribution.py, CLI version code
and focused tests, .github/workflows/release.yml, release/installation docs and
these spec routes. Existing collection, data formats, installation ownership,
platform qualification, images and unrelated working changes are protected.

Acceptance: exact CLI/version output; reject malformed/mismatched versions before
build; current and historical candidate verification; workflow syntax and pinned
source/permissions/no-auto-publication checks; real local combined candidate and
temporary install smoke. Remote Actions/download acceptance remains pending until
explicit remote authorization. No claim of independent review or public release.
