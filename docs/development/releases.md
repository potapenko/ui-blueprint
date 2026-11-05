# Release UI Blueprint

Current preparation: **0.1.0**, Apple Silicon macOS. No public release exists yet.
The executable remains `uiblueprint`. One release version covers its matching
worker, native helper, optional callers and agent skill. Recorded-data schema
versions and internal Cargo package versions are separate.

## Bump and prepare locally

1. Update the root [VERSION](../../VERSION):0.1.0 →0.1.1 for fixes,0.2.0 for new
   capabilities. Use three numeric components; no feature/browser suffixes.
2. Add that version's section to [CHANGELOG.md](../../CHANGELOG.md). Describe the
   user-visible result and relevant support limits. Never replace an existing
   release with different bytes under the same number.
3. Complete focused checks and checkpoint on the current branch. Inspect the
   complete source delta before including previously unpushed work in a release.
4. Build one exact committed source, verify it and test its temporary installation:

```sh
UIB_REVISION=$(git rev-parse HEAD)
UIB_RELEASE_OUT=$(mktemp -d "${TMPDIR:-/tmp}/uib-release.XXXXXX")
python3 -B release.py prepare --revision "$UIB_REVISION" --destination "$UIB_RELEASE_OUT"
python3 -B release.py verify --destination "$UIB_RELEASE_OUT"
python3 -B tests/bridges/release_check.py --candidate "$UIB_RELEASE_OUT"
python3 -B release.py notes --revision "$UIB_REVISION" > "$UIB_RELEASE_OUT/RELEASE_NOTES.md"
```

Stop on any failure. The CLI's `--version`, archive name, installer version and
skill archive must agree. Preparation is local and unsigned by default. The
existing [Mac release guide](macos-release.md) covers signing, notarization and
installation ownership. Keep review artifacts in system temp; remove run-owned
non-image outputs after review, retaining all images.

## GitHub Actions and draft

The [release workflow](../../.github/workflows/release.yml) has only a manual
`workflow_dispatch` trigger. It never starts on a push or tag. Before any remote
release build, upload or publication, obtain the operator's explicit confirmation
for the exact version, source SHA and GitHub repository. Approval of implementation
or a Git push alone is insufficient. Present this exact question with real values:

«Разрешаешь выкладку UI Blueprint vVERSION из COMMIT на GitHub
potapenko/ui-blueprint (сборка и загрузка артефактов / создание draft / публикация)?»

Name only the actions actually requested. Wait for an unambiguous affirmative
answer. This runbook does not itself authorize a remote action.

After that authorization and after the workflow/source are present on GitHub,
open **Actions → Prepare UI Blueprint release → Run workflow**. Supply the numeric
VERSION and reviewed full40-character SHA. The default `create_draft=false` builds
and verifies the combined bundle and keeps the artifacts for7days. The runner is
the GitHub `xcode-27` Apple Silicon image; toolchain1.96.0 is explicitly installed
and locked dependencies fetched before the offline packaging recipe runs.

With separately authorized `create_draft=true`, a second job downloads only this
run's artifacts, verifies checksums/source/version, refuses an existing tag and
creates a **draft** titled `UI Blueprint X.Y.Z` for tag `vX.Y.Z`. It attaches the
CLI archive, matching skill archive, install.sh, SHA256SUMS, release.json and
Homebrew formula. Release notes come from the selected commit's changelog.
GitHub also refuses a duplicate release; the workflow never overwrites assets.
The build job has read-only repository access. Only the optional draft job gets
contents:write. Action implementations are pinned to verified commit hashes.

## Public release

Draft creation does not publish. Before public release, resolve the project
license and existing Mac signing/notarization/Gatekeeper requirements, review the
exact final assets, and obtain the required explicit publication confirmation.
Do not publish the default unsigned candidate as a qualified Mac binary release.
Any changed source, contents or targets require a revised review/confirmation.

Publish the reviewed draft only after those gates. Then verify the public download
and installation from that exact tag, and update the README's release-status text.
The [installation guide](../install.md) resolves the latest stable release once
and uses its exact tag for all downloads. Until the first publication, it retains
the explicit pinned-source fallback and does not claim public binaries exist.
