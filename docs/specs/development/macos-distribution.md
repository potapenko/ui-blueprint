# Mac distribution
- Node type: leaf; domain: `uib.distribution`.
- Contract: `UIB.MAC-DISTRIBUTION@9`.
- Authority: Active; stability: Evolving; released baseline: none.
- Authority source: user approval in this chat on 2026-10-13 of the six-step
  Mac archive/curl/Homebrew plan, followed by “Подготовь всё до публикации,
  а лицензию и саму публикацию согласуем отдельно.”
- Read when: preparing release archives or installing/updating/removing them.
- Requires: [BOUNDARIES@1](../product/boundaries.md), [DEV.RUST@2](rust.md).
- Entry: [D01 support](decisions/d01-support.md); README files remain unchanged.
- Source-only releases: [RELEASES@2](releases.md) supersedes binary publication/signing gates for the user-selected source channel; binary packaging remains local installation machinery.
- Numeric versions/workflow: [RELEASES@1](releases.md) overrides the earlier README/CLI exclusions within its approved scope.
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
Use a flat CLI tar.gz, a SHA256SUMS file and a version-pinned install.sh.
Skill-capable candidates add a separate versioned skill tar.gz from the same commit;
legacy candidates remain verifiable. [AGENT-SKILL@3](agent-skill.md) governs opt-in installation.
Record source revision, build host, conservative minimum OS and signature status.
The initial OS floor is the actual build/test host version; older OS support
requires its own qualification, never inference from Swift's deployment target.
No publish command or network upload runs implicitly. A prepared candidate is
not a notarized/public release. Existing local v1/v2 verification/removal stays valid.
## DISTRIBUTION.INSTALL — Ownership and lifecycle
Default prefix is ~/.local; --prefix selects an absolute alternative.
Own share/uiblueprint/versions/<version>, the current symlink and two bin links.
Optional --skill-dir ABS selects one externally discoverable skill symlink through
current/skill, with destination ownership recorded under the install root. Updates
keep the registered skill and CLI together; uninstall verifies and removes both.
No sudo, shell-profile edit, Python/Node/Rust/Xcode prerequisite, UI collection,
permission grant or launch. The optional Web caller still needs Node24+ separately.
Detect unsupported OS/architecture, pin a release before fetching its archive,
check the embedded SHA-256 and exact regular-file inventory before activation.
Installation needs HTTPS and standard macOS shell tools. Checksums detect changed
bytes; they do not independently authenticate a compromised installer source.
Do not overwrite foreign paths, symlinks or another installation channel.
Serialize own mutations. Verify before switching current; retain previous versions.
Repeat installation of the same unchanged version is idempotent; --version selects
a fully verified already-installed version, or dispatches its published installer
when absent. Each version uses its recorded manifest1/2/3/4/5 inventory, with exact
receipt names, hashes and modes; skill inventory follows that same version. Stop sessions before switching;
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
MAC-DIST-002, 2026-10-13: explicit user continuation in the skill chat authorizes
matching skill packaging/installation and temp-only tests. Existing CLI archive and
Homebrew behavior stay unchanged; no publication, real installation or notarization.
MAC-DIST-003: user-authorized 2026-10-14 push/new-chat installation adds explicit
--local-release ABS for pinned locally built assets, with unchanged hash/inventory
guards. No network/version dispatch in that mode; no implicit prerequisite install.
KC-COMPOSE-001: user-approved manifest3 adds the four flat optional Compose source
files and references/compose.md; consumer Debug inclusion remains explicit.
KC-REVIEW-002: @5 selects verified installed-version rollback after the manifest2→3
receipt mismatch; older inventories, ownership and tamper refusal stay protected.
NATIVE-FIRST-USE-001: direct human directive adds native-prepare.py to manifest4;
Python3.11+ is optional. No runtime permission change or publication follows.
WINDOW-RELEASE-001: direct2026-11-14 instruction to finish the new capabilities,
skills and release preparation adds manifest5 with the same CLI inventory and
references/gpui.md in the eight-file skill. Formats1/2 retain six skill files;
3/4 retain seven. Verify those exact historical inventories for update, rollback
and removal. Candidate skill links must resolve within its packaged files.
SDK integration remains separate; source-only publication keeps its explicit gate.

Manifest6 adds the six committed cache-capture Python modules to native and
combined local packages after the first verified partial Reels capture. The
package carries no consumed process grant. Operator-bound qualification remains
a separate step. Existing CLI/wire and skill inventories retain their meaning.
Manifest1–5 verification selects the recorded historical inventory for rollback.

Manifest7 adds the native parts, cache transform, fixed backend and ancestor
cache Python modules required
by the selected committed collector. Release-owner packaging authority and S13/S03
module handoffs select this additive inventory; this does not qualify new product
captures. Preserve the exact six-module manifest6 inventory and all older formats
for verification, update, rollback and removal. Verify isolated installed imports
without attachment, plus format6→7→6 lifecycle and missing/foreign file refusal.
S14 component experiments remain source-only with their separately qualified
owner profile/backend; no debugger binary or process grant is bundled.
