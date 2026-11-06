# Release UI Blueprint source

A release is a verified source commit, numeric tag `vX.Y.Z` and changelog notes.
It has **no binary attachments**. Users install through the README prompt: the
agent resolves the release, obtains its instructions with curl, builds locally
and installs the utility and matching skill. GitHub automatically provides source
archives for tags; these are not uploaded executable packages.

## Prepare and verify

1. Set the root [VERSION](../../VERSION):0.1.0 →0.1.1 for fixes,0.2.0 for new
   capabilities. One number covers the complete utility and skill.
2. Update [CHANGELOG.md](../../CHANGELOG.md), including installation/support limits.
3. Keep the project [MIT license](../../LICENSE) and dependency notices with source
   and local packages. Do not change an existing release tag or version's bytes.
4. Run focused checks and checkpoint on the current branch. Review the complete
   delta, including previously unpushed commits, before proposing publication.
5. Verify that the selected source can actually build and install locally:

```sh
UIB_REVISION=$(git rev-parse HEAD)
UIB_CHECK=$(mktemp -d "${TMPDIR:-/tmp}/uib-source-release.XXXXXX")
python3 -B release.py prepare --revision "$UIB_REVISION" --destination "$UIB_CHECK"
python3 -B release.py verify --destination "$UIB_CHECK"
python3 -B tests/bridges/release_check.py --candidate "$UIB_CHECK"
python3 -B release.py notes --revision "$UIB_REVISION" > "$UIB_CHECK/RELEASE_NOTES.md"
```

The generated archives/install.sh are temporary local-installation inputs.
Do not attach them to GitHub Releases or upload them as CI artifacts. Remove
run-owned non-image files after verification; retain all images. No Apple signing,
notarization, credential lookup or Gatekeeper qualification is required for this
source-only release. Separately requested binary distribution is a different task.

## Verify on GitHub and prepare a source draft

The [workflow](../../.github/workflows/release.yml) has only a manual trigger.
It checks the numeric version and exact full40-character SHA, installs the pinned
build toolchain in the ephemeral runner, builds and tests local installation.
It uploads no binary artifacts. The build job has contents:read; the optional
source-draft job has contents:write. Action revisions are pinned. The xcode-27
runner checks Apple Silicon macOS source compilation, not Apple notarization.

Before a push, remote release workflow or publication by an agent, present the
reviewed version, source SHA and intended GitHub actions. Obtain the mandatory
explicit confirmation, with actual values:

«Разрешаешь выкладку UI Blueprint vVERSION из COMMIT на GitHub
potapenko/ui-blueprint: push текущего master, проверка и публикация исходников
с тегом vVERSION, без бинарных вложений?»

Wait for an unambiguous affirmative reply. This runbook and approval of a code
change are not publication authority. Changes to source, contents or targets need
a revised confirmation. No remote action runs implicitly during local preparation.

After authorization, push the reviewed current branch. In **Actions → Verify UI
Blueprint source release**, run the workflow with its numeric version and exact
commit SHA. `create_draft=false` only verifies. `create_draft=true` creates a source
release draft after successful checks; there are no asset uploads. Existing tags
or releases are refused instead of overwritten.

## Publish and check installation

Publish the exact reviewed draft after verification and within the user's explicit
publication approval. Check the tag points to the reviewed SHA, VERSION matches,
MIT is visible, and release assets are empty. The public notes must explain prompt
installation and link to the tag-pinned installation guide. Then verify that guide
resolves and installs the exact release source through the ordinary local flow.
Update current release status only from actual GitHub evidence. No binary download,
Apple signature, Homebrew tap publication or source-only tag alone is claimed as
proof of successful local installation.
