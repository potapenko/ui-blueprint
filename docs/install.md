# Install UI Blueprint

Install the local utility and the `ui-blueprint` skill together. The utility
performs geometry calculations; the skill explains observation, analysis,
comparison, and drawing workflows to the agent. One skill covers these tasks.

**Current status:** local installation and isolated Codex/Claude Code skill discovery
have been verified. Public release preparation remains incomplete; no public release
has been published. See [tested versions and evidence](development/macos-release.md#skill-discovery-verification--2026-10-13).
An agent following this guide must stop before installation when a matching
published CLI + skill release is unavailable. Source files and local candidates
are not published releases. Do not install only half of the requested set.

## Start with a prompt

Copy the appropriate [README prompt](../README.md#install-through-your-agent)
into local Codex or Claude Code. No manual download or terminal command is needed
from the person once the release is available. Normal host tool permissions still
apply; a web-only chat without access to the Mac cannot install local software.

The prompt authorizes UI Blueprint's managed files under `~/.local` and exactly
one skill destination. For custom paths, replace them in the prompt before sending
it. Reading this page alone does not authorize a local installation.

| Agent | Default personal skill destination | After installation |
| --- | --- | --- |
| Codex | `~/.agents/skills/ui-blueprint` | Read `SKILL.md`; invoke by name or `$ui-blueprint` where supported |
| Claude Code | `~/.claude/skills/ui-blueprint` | Read `SKILL.md`; invoke `/ui-blueprint` |

These paths follow [OpenAI's local skill documentation](https://developers.openai.com/codex/skills)
and [Claude Code's skill documentation](https://code.claude.com/docs/en/skills).
Existing custom/legacy locations must be identified from the host rather than
guessed. Do not install duplicate copies, change host configuration or migrate an
existing skill without the user's authorization. Discovery was verified with Codex's
bundled CLI 0.162.0-alpha.17.2 and Claude Code 2.1.296 in isolated local processes.
Claude used `--setting-sources user,project,local`; `--bare` did not list the skill in
this check. Discovery in the person's actual session must still be checked after
installation; successful file installation is a separate result.

## Instructions for the installing agent

### 1. Select a complete release before changing files

Use only [UI Blueprint releases](https://github.com/potapenko/ui-blueprint/releases)
from `potapenko/ui-blueprint`. Resolve the latest stable release to one exact tag;
use that tag for every subsequent download. Never mix assets from `latest` calls,
an arbitrary existing CLI, or a skill from a moving source branch.

Confirm that the release provides the Mac CLI archive, matching skill archive,
rendered `install.sh`, and checksums. The installer must support `--skill-dir` and
pin both payloads from one source revision. A missing release/asset or unsupported
installer is a prerequisite failure: report it without falling back to a source
build, installing a skill alone, or running this repository's unrendered template.

First support is Apple Silicon macOS. Check the release's OS floor; the initial
qualified host is macOS 27.0.1. Intel Macs, older macOS, Linux and Windows are not
qualified. End users do not need Rust, Xcode or Python. Node.js 24+ is needed only
for the optional Web first-use caller, not for installation or saved-data analysis.

### 2. Install the utility and this agent's skill

Identify the agent and the exact paths authorized in the prompt. Inspect an
existing installation first; preserve foreign or locally modified files and an
existing Homebrew installation. Do not silently switch installation channels.

Download the selected release's `install.sh` over HTTPS with a finite timeout to a unique system-temp directory,
inspect it, check the published checksum, then invoke it with absolute paths:

```sh
sh "$INSTALLER" --prefix "$HOME/.local" --skill-dir "$SKILL_DIR"
```

`INSTALLER` is the verified file from the selected tag; `SKILL_DIR` is the fully
expanded Codex or Claude Code destination authorized above. These are local shell
variables, not settings to add to the user's shell profile. The installer verifies
the payload hashes, inventories and existing managed files before activation.

The CLI lives under `~/.local/share/uiblueprint/versions/<version>`; `current`
selects the active set. Public commands live in `~/.local/bin`. The installed skill
points to that same active version's skill directory, keeping updates together.
Use the real version directory when supplying explicit worker/helper paths.

No sudo, shell-profile modification, UI permission grant, app/browser launch,
source build or automatic dependency installation is part of this workflow.
If the command directory is absent from PATH, use absolute paths for verification
and subsequent skill work. Do not make the person repair PATH just to finish setup.

### 3. Verify a useful result and load the skill

Read the installed `SKILL.md` and follow its relative
`references/analysis.md` → **Known-result smoke example**. Confirm each exit and
payload: validator **valid**, measured gap **8 css_px**, and check **pass**.
All referenced skill files must resolve outside the source checkout. Verify the
skill and CLI refer to the same installed version, not just the same schema number.

Load the skill in the current conversation when the host permits it. Check its
normal discovery/invocation mechanism; do not infer discovery from files alone.
If the host needs a refresh or a new session, report that as the remaining step.
Do not restart the person's application automatically.

Finish with the installed version, utility/skill locations, example result and
discovery status. Separate “files installed and example passed” from “the host
discovered the skill.” Remove only run-owned non-image download files and empty
temporary directories. Preserve user data and all images.

## Update with a prompt

```text
Update my existing UI Blueprint utility and skill together following https://raw.githubusercontent.com/potapenko/ui-blueprint/master/docs/install.md
Reuse their current locations, preserve my changes, and verify the bundled example.
```

Use the same installation channel and registered skill destination. Stop active
UI Blueprint sessions before switching versions. Repeating the same unchanged
installation is harmless; updates keep earlier versions available. A requested
rollback uses an exact compatible release version. An older installer that cannot
preserve the registered skill must be rejected rather than dropping the skill.

## Manual utility installation

The following commands become usable **after publication**. They install the
utility only; use the agent workflow above for the matching skill as well.

```sh
curl -fsSL https://github.com/potapenko/ui-blueprint/releases/latest/download/install.sh | sh
```

For an existing Homebrew setup:

```sh
brew install potapenko/tap/uiblueprint
```

Homebrew owns its own updates and removal. The shell installer's `--uninstall`
removes its verified managed installation and registered skill link; it does not
remove Homebrew files or a shared skills directory. Modified files cause refusal.

Maintainers: [release preparation](development/macos-release.md),
[distribution and compatibility](development/distribution.md#agent-skill-and-prompt-installation),
[skill source](../skills/ui-blueprint/SKILL.md).
