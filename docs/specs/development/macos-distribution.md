# Mac distribution

- Node type: leaf; domain: `uib.distribution`.
- Contract: `UIB.MAC-DISTRIBUTION@1`.
- Authority: Active; stability: Evolving; released baseline: none.
- Authority source: user approval in this chat on 2026-10-13 of the six-step
  Mac archive/curl/Homebrew plan, followed by “Подготовь всё до публикации,
  а лицензию и саму публикацию согласуем отдельно.”
- Read when: preparing release archives or installing/updating/removing them.
- Requires: [BOUNDARIES@1](../product/boundaries.md), [DEV.RUST@2](rust.md).
- Entry: [D01 support](decisions/d01-support.md); README files remain unchanged.

## DISTRIBUTION.SCOPE — Approved change

Evolve the local-only distribution into a prepared Mac Apple Silicon release:
one combined Mac/Web archive, shell installer and own-tap Homebrew formula.
Implementation stays in this chat on the current branch. Publication, license
selection, remote tap edits, tags and pushes remain outside preparation.
Developer ID signing uses existing credentials only. Notarization of the signed
candidate is an explicit preparation step after locating an existing profile;
it is never implicit in the local build command. The user's later request permits
read-only consultation with other project chats about the profile name only.
The original accepted plan and user approval remain in this chat, not inferred
from this contract. No README, product algorithms, CLI grammar, wire schemas,
runtime permissions, Linux/Windows/Intel support or automatic browser launch changes.

## DISTRIBUTION.ARCHIVE — One immutable set

Every distributed product, helper, recipe, guide and installer comes from one
committed revision. Release version is separate from schema/package versions.
Build the existing combined selection. Final file hashes follow code signing.
Preserve complete dependency notices; a missing project license is explicit
and prevents presenting preparation as a public release approval.
Use a flat tar.gz, a SHA256SUMS file and a version-pinned install.sh.
Record source revision, build host, conservative minimum OS and signature status.
The initial OS floor is the actual build/test host version; older OS support
requires its own qualification, never inference from Swift's deployment target.
No publish command or network upload runs implicitly. A prepared candidate is
not a notarized/public release. Existing local v1/v2 verification/removal stays valid.

## DISTRIBUTION.INSTALL — Ownership and lifecycle

Default prefix is ~/.local; --prefix selects an absolute alternative.
Own share/uiblueprint/versions/<version>, the current symlink and two bin links.
No sudo, shell-profile edit, Python/Node/Rust/Xcode prerequisite, UI collection,
permission grant or launch. The optional Web caller still needs Node24+ separately.
Detect unsupported OS/architecture, pin a release before fetching its archive,
check the embedded SHA-256 and exact regular-file inventory before activation.
Installation needs HTTPS and standard macOS shell tools. Checksums detect changed
bytes; they do not independently authenticate a compromised installer source.
Do not overwrite foreign paths, symlinks or another installation channel.
Serialize own mutations. Verify before switching current; retain previous versions.
Repeat installation of the same unchanged version is idempotent; --version selects
another published installer for upgrade/rollback. Stop sessions before switching;
long-lived trusted connections should use explicit immutable version paths.
--uninstall validates all managed versions before removing only owned files/links;
unknown/modified content is retained with an error. Never recursively delete.
Temporary extraction files are run-owned; retain images and image-containing
directories if encountered. Display a PATH instruction when the bin path is absent.

## DISTRIBUTION.BREW — Same release bytes

Prepare a formula for the proposed potapenko/homebrew-tap destination; editing
that remote repository is a separate publication step. Install the identical
versioned archive under libexec and expose only CLI/validator through bin links.
Homebrew owns upgrade/removal. Do not add build/runtime tool dependencies solely
for packaging. An unrendered template or unnotarized candidate is not publish-ready.

## DISTRIBUTION.ACCEPT — Preparation checks

Verify archive inventory/hashes, committed provenance, installed saved-data
commands and helper availability, paths with spaces, repeat install, update,
rollback, removal, collisions, symlinks, changed files, interrupted downloads,
bad hashes and unsupported platforms. Use isolated system-temp prefixes, not
the operator's real installation. Check formula syntax and run Homebrew lifecycle
when it is available within this scope; distinguish simulated transport from
actual published URL acceptance. Check real signatures for signed candidates.
Notarization, downloaded Gatekeeper acceptance, public curl and remote Homebrew
installation remain named publication gates; no bypass or false pass.
Broader native/Web product acceptance remains separate from packaging checks.

## Change record

MAC-DIST-001, 2026-10-13: user-authorized additive release preparation, from local
bundles to the archive and installation contract above. This contract's registration
precedes implementation through D01; it does not select a license or publish.
