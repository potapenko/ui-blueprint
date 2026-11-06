# Install UI Blueprint through your agent

UI Blueprint releases identify verified **source versions**. There are no binary
release attachments to download. The agent builds the utility on your Mac and
installs it together with the matching `ui-blueprint` skill. curl retrieves these
instructions and release metadata; it does not install a downloaded executable.

## Start with a prompt

Use the [README prompt](../README.md#install-through-your-agent) in local Codex or
Claude Code with terminal access. The person need not run commands manually.
The prompt authorizes the managed utility under `~/.local` and the named skill
location; this page alone grants no permission. Preserve existing custom paths.

| Agent | Personal skill destination |
| --- | --- |
| Codex | `~/.agents/skills/ui-blueprint` |
| Claude Code | `~/.claude/skills/ui-blueprint` |

A web-only chat without filesystem/terminal access cannot perform local installation.
Normal host permissions apply. Do not create duplicate skill installations, change
shell profiles or restart the person's application merely to finish setup.

## 1. Resolve one release and its source

Fetch current instructions with bounded curl, then resolve the latest published
stable release in `potapenko/ui-blueprint` to a numeric tag `vX.Y.Z`. Pin its exact
commit for the entire installation. Never substitute moving master for a published
release or mix a skill from another commit. An explicit version request selects
that exact release instead. Before the first publication, only a user-selected
explicit developer commit may use the same build path; report it as unpublished.

Check prerequisites first: Apple Silicon macOS (initial qualified host27.0.1), Git,
Python3.11+, Rust/rustup1.96.0 with the locked Cargo dependency cache, and Xcode/Swift6.
Node24+ is optional for Web observation, not for saved-data geometry. Report a
concrete missing prerequisite; install additional development tools only within
separate user authority. The packaging recipe stays `--locked --offline`.

Run this sequence one step at a time, stop on errors, and use finite command timeouts:

```sh
UIB_SETUP=$(mktemp -d "${TMPDIR:-/tmp}/uib-setup.XXXXXX")
curl --fail --location --connect-timeout 10 --max-time 30 \
  https://api.github.com/repos/potapenko/ui-blueprint/releases/latest \
  --output "$UIB_SETUP/release.json"
UIB_TAG=$(python3 -c 'import json,re,sys; r=json.load(open(sys.argv[1])); t=r["tag_name"]; assert not r["draft"] and not r["prerelease"] and re.fullmatch(r"v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)",t); print(t)' "$UIB_SETUP/release.json")
git clone --depth 1 --single-branch --branch "$UIB_TAG" --no-checkout \
  https://github.com/potapenko/ui-blueprint.git "$UIB_SETUP/source"
UIB_REVISION=$(git -C "$UIB_SETUP/source" rev-parse HEAD)
test "v$(git -C "$UIB_SETUP/source" show "$UIB_REVISION:VERSION")" = "$UIB_TAG"
git -C "$UIB_SETUP/source" show "$UIB_REVISION:docs/install.md"
```

Read the selected commit's instructions before continuing. The no-checkout clone
avoids materializing repository images. Do not run the root install.sh template:
it is rendered by the local build, not a public download endpoint.

## 2. Build and install that exact version

Materialize only the two build recipes, then build the local matching package:

```sh
for UIB_RECIPE in release.py distribution.py; do
  git -C "$UIB_SETUP/source" show "$UIB_REVISION:$UIB_RECIPE" > "$UIB_SETUP/source/$UIB_RECIPE"
done
mkdir "$UIB_SETUP/candidate"
python3 -B "$UIB_SETUP/source/release.py" prepare \
  --revision "$UIB_REVISION" --destination "$UIB_SETUP/candidate"
python3 -B "$UIB_SETUP/source/release.py" verify --destination "$UIB_SETUP/candidate"
sh "$UIB_SETUP/candidate/install.sh" --local-release "$UIB_SETUP/candidate" \
  --prefix "$HOME/.local" --skill-dir "$SKILL_DIR"
```

Before installation, set SKILL_DIR to the exact authorized absolute host path and
inspect existing installation ownership. For updates, reuse the registered skill
location. Stop active UI Blueprint sessions before switching. Preserve foreign or
modified files and existing Homebrew ownership; do not switch installation channels.
These local archives are installation intermediates, never GitHub release assets.
No Apple certificate or notarization is required for this local source-build path.

VERSION supplies the product number. Do not invent suffixes, overwrite an installed
version with different bytes, or relabel historical builds. New source releases
must advance the numeric version. Earlier managed versions remain available for
verified rollback. CLI/worker/helper/skill must come from one source revision.

## 3. Verify the installed utility and load the skill

Run the installed `uiblueprint --version`; it must agree with the selected release.
Use its absolute path under `~/.local/bin` if PATH does not contain that directory.
Resolve the skill through the same installation's `current` link, read SKILL.md,
and follow `references/analysis.md`'s known-result smoke example: validator valid,
measured gap8 css_px, check pass. All references must resolve outside the checkout.

Load the skill in the current chat where supported. File installation and host
skill discovery are separate results. Report any remaining host refresh requirement
without restarting the application automatically. Live UI access is a separate
prerequisite; installation does not grant browser or macOS permissions.

Remove only this run's non-image source/build/download files and empty temporary
directories after verification. Preserve all images and unrelated existing files.
Finish with the installed numeric version, utility/skill paths, example results
and discovery status. Never describe a local source build as an Apple-notarized
binary download.

## Update with a prompt

```text
Update my existing UI Blueprint utility and skill using current instructions fetched with curl from https://raw.githubusercontent.com/potapenko/ui-blueprint/master/docs/install.md
Use the latest published source release, reuse their current locations, preserve my changes, and verify the bundled example.
```

The same flow resolves the new tag and builds locally. The installer verifies owned
files, keeps the matching skill with the CLI and retains prior versions for rollback.
Its --uninstall removes only verified managed files/links, not a shared skill parent
or Homebrew files. Modified installations refuse destructive replacement/removal.

Maintainers: [source release workflow](development/releases.md),
[local packaging](development/macos-release.md), [skill source](../skills/ui-blueprint/SKILL.md).
