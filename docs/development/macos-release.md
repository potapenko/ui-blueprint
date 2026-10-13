# Mac release preparation (not published)

The approved 2026-10-13 scope prepares an Apple Silicon combined Mac/Web release.
The [installation contract](../specs/development/macos-distribution.md) is routed
through D01. Publication and project licensing remain separate owner decisions.
No README, release/tag or remote tap changes during preparation. The local
`prepare` command never uploads to Apple; `notarize` is a separate explicit step.

## Prepare locally

`release.py` builds the existing `combined` selection from one committed revision,
including recipes, guides, Web caller, schemas and examples. It refuses an
invoked release/build recipe that differs from that revision. Checkpoint changes
first; unrelated working-tree changes are not included in the archive.
The release version is independent of schema/export package versions.
The conservative minimum macOS is the actual build host version (currently
27.0.1); this does not qualify older releases or Intel. The public URLs below
will not work until an authorized release has actually been published.

```sh
RELEASE_OUT="$(mktemp -d "${TMPDIR:-/tmp}/uib-release-candidate.XXXXXX")"
python3 -B release.py prepare --version 0.1.0 --revision HEAD --destination "$RELEASE_OUT"
python3 -B release.py verify --destination "$RELEASE_OUT"
python3 -B tests/bridges/release_check.py --candidate "$RELEASE_OUT"
```

The default creates an **unsigned local candidate**, not a public release.
For a signed candidate use `--sign-identity CERTIFICATE_SHA1` with an existing
Developer ID Application identity. No key/certificate is created or exported.
Signing uses Hardened Runtime and a secure timestamp; final bundle hashes follow
signing. The selected identity must actually be available.
`prepare` never uploads, tags, pushes or changes Homebrew. Outputs:

- `uiblueprint-vVERSION-aarch64-apple-darwin.tar.gz`, the flat combined bundle;
- `ui-blueprint-skill-vVERSION.tar.gz`, the portable skill from the same commit;
- rendered `install.sh`, with a pinned version and hashes;
- `SHA256SUMS` and rendered `uiblueprint.rb`;
- local `release.json`, recording source/platform/signing and pending gates.

Skill-capable candidates use `ui-blueprint-release-candidate-2`; verification also
accepts legacy candidate-1 archives. `SHA256SUMS` covers both archives, the rendered
installer and formula. The CLI archive and Homebrew installation remain unchanged.

Existing files cannot be overwritten. `release.json` is the last completion marker.
Keep a selected candidate only for its pending owner review; remove run-owned
non-image candidates after acceptance/discard. Never clean up retained images.

## Signing and later notarization

Notarization is an explicit upload of the signed candidate, separate from building:

```sh
python3 -B release.py notarize --destination "$RELEASE_OUT" --keychain-profile PROFILE
```

It verifies the candidate and signatures, creates an Apple-compatible temporary
ZIP of the same signed files, submits once, and records Apple's submission ID.
Use `xcrun notarytool info ID --keychain-profile PROFILE --output-format json`
to inspect it, or `wait ID --keychain-profile PROFILE --timeout 5m` for a bounded
wait. Require **Accepted** before public release; inspect rejected submissions
with `notarytool log`. Before upload the tool records `submission_outcome_unknown`;
a transport timeout retains that state and a subsequent command refuses to submit
again. Inspect Apple history and reconcile the record before any retry.
Standalone executables/tar.gz do not get
a stapled ticket; verify the downloaded/quarantined candidate under normal
Gatekeeper policy. Do not remove quarantine or disable system checks to pass QA.

## User installation after publication

```sh
curl -fsSL https://github.com/potapenko/ui-blueprint/releases/latest/download/install.sh | sh
```

The rendered installer pins its version, archive digest and full file hashes;
it never mixes latest metadata with a different version's download.
It uses HTTPS with bounded waits and only standard macOS tools. SHA-256 detects
changed bytes, not a compromise of the trusted installer source itself.
Default files live in `~/.local/share/uiblueprint/versions/VERSION`; `current`
selects the active set and `~/.local/bin` exposes CLI/validator. `--prefix ABS`
selects a different prefix. No sudo, shell-profile edit or UI access occurs.
If necessary, it prints a shell-quoted PATH command for the current shell;
add that path to your preferred shell configuration yourself for persistence.
Node24+ remains needed only for the optional Web first-use caller.

For the matching skill, pass `--skill-dir ABSOLUTE_SKILL_PATH` along with the prefix.
The installer has no agent-specific default or discovery mechanism. It records that
one authorized destination in `share/uiblueprint/.skill-link` and creates a symlink
to `current/skill`. Updates reuse it; both payloads validate before `current` switches.
Skill files live in `versions/VERSION/skill`, with a `.skill-sha256` receipt. Edited
or foreign files refuse replacement/removal. Older installers lacking skill support
are rejected during version dispatch when a skill is registered. See the
[agent installation guide](../install.md) for host paths and the one-prompt workflow.

Repeat the command to update, or use `sh install.sh --version VERSION` to select
an exact published release (including rollback). Stop sessions first and use
immutable version paths in long-lived trusted connections; do not mix helpers.
Older versions remain available. Installation locks serialize mutations; a stale
`.lock` requires inspection after proving no installer is running.
`sh install.sh --uninstall` validates all managed versions before removing owned
files/links. Foreign commands, symlinks, changed files and unknown version content
cause refusal. Unrelated files and shared prefix directories are never deleted.
There is no forced uninstall or recursive cleanup.

## Homebrew

The own-tap formula installs the **same archive** under Homebrew libexec and exposes
CLI/validator through bin links. Intended command after publication:
`brew install potapenko/tap/uiblueprint`; upgrade/remove belong to Homebrew.
Do not install both channels into overlapping command paths. The root
`uiblueprint.rb` is a template; use only the rendered candidate formula, and settle
the project's license before publishing it. No build-tool dependency is added.

## Verification and publication boundary

Run `shellcheck install.sh`, `ruby -c uiblueprint.rb`,
`python3 -B tests/bridges/release_check.py`, and the real candidate check above.
Installer tests use fake HTTPS transport and isolated system-temp prefixes.
The candidate check also runs the real installer/payload and Homebrew formula
install/test methods using the installed Homebrew API with an isolated prefix;
it does not register a tap or change the operator's Cellar.
Current export smoke follows [SHEET-PROMPTS@1](../specs/product/drawing-sheet-prompts.md)
for exact0.4 document/propose inventories; comparison remains0.2.
They are not proof of a published URL or downloaded Gatekeeper acceptance.
Public curl/Homebrew lifecycle, Apple Accepted status and a fresh downloaded
signature/Gatekeeper check remain publication gates. Broader product acceptance
is not replaced by successful packaging tests.
Prepare reviewed release notes with the exact source commit, architecture,
minimum OS, Node optionality and support limits before publishing. Do not create
tags, push, create a GitHub draft/release, or change the remote tap in preparation.

## Skill discovery verification — 2026-10-13

Local readiness: installer integration, installed saved-data smoke and isolated
host discovery are verified. Public release readiness remains pending. The operator
explicitly requested discovery checks in temporary directories without publication
or changes to real settings, then requested this documentation record and checkpoint.

The skill and installer tested here are unchanged from integration checkpoint
`fa7afc027358ab1d8d2168c531275c79dde33e95`; lifecycle test checkpoint is `5eb95d0d`.
The rendered production installer installed exact skill bytes into two temporary
prefixes, with host links under the isolated home's `.agents/skills/ui-blueprint`
and `.claude/skills/ui-blueprint`. Each link resolved through `current/skill` to
`versions/0.1.0/skill`. Local transport supplied the archives. This discovery fixture
used synthetic CLI payloads, which were not executed: real CLI installation and
the gap **8 css_px** / check **pass** smoke were established separately on the
unsigned `0.1.0-skill` candidate built from `fa7afc02` in the preceding check.

| Host and mode | Observed result |
| --- | --- |
| Codex bundled CLI `0.162.0-alpha.17.2`, isolated `app-server --stdio` | `skills/list` with the temporary project cwd and `forceReload: true` returned `ui-blueprint`, `scope: user`, `enabled: true`, resolved installed SKILL.md path, UI metadata and no skill-loading errors |
| Claude Code `2.1.296`, local stream-JSON initialization with `--setting-sources user,project,local` | The successful initialization response listed `ui-blueprint` with its description marked `(user)`, exposing `/ui-blueprint` |
| Same Claude version with `--bare` | Initialization succeeded, but its command list did not include this personal skill; do not generalize this observation to other versions or modes |
| Both hosts with the temporary discovery links removed, in fresh processes | `ui-blueprint` was absent; after installer-created links were present, fresh processes returned it again |

Codex used only `initialize`, `initialized` and `skills/list`, not `turn/start`.
Claude used `--print --input-format stream-json --output-format stream-json
--verbose --no-session-persistence`, an empty strict MCP configuration and disabled
hooks. Its only input was a control request with subtype `initialize`; no user
prompt or skill execution was submitted. This proves loader/command registration,
not model selection of the skill, instruction adherence or task completion.

Both probes ran under a macOS sandbox denying network access and writes outside
the task's system-temp tree. Child-process home/config/cache/temp paths were isolated;
real host settings and repository files were not changed. Claude Code was absent
from the checked local command locations, so its official npm package was downloaded
into temp for this test. No account login, credentials or model call was needed.
The initial Claude probe hit a denied `/tmp/claude-501` write; setting its
`CLAUDE_CODE_TMPDIR` inside the test tree allowed initialization without widening
write access. Codex's temporary-helper and blocked featured-plugin-cache warnings
did not produce skill-loading errors.

The production uninstaller removed both temporary managed installations and links.
Temporary Claude runtime, configuration, payloads, probe scripts and raw logs were
removed. Eight Codex-created system-skill PNG/SVG icons remain in the system-temp
test tree under the no-image-deletion rule; they are not discovery evidence. This
record summarizes the tool results retained in the requesting chat, not archived logs.

No public download, downloaded Gatekeeper check, real user installation, desktop
skill-picker interaction or model invocation was tested. Discovery is qualified only
for the versions/modes above. Publication, licensing and remaining signing/notarization
gates retain their separate authority; discovery success does not close them.
