# Local UI Blueprint distribution

Build a selected set from one **committed** source revision. This is a local
P6 delivery procedure, not a signed release or P7 acceptance. Saved-data analysis
and document/proposal/comparison export run without a model, API keys, browser, Node or Swift.

## Prerequisites and selection

The tested host is arm64 macOS 27.0.1. Install prerequisites yourself before
building: Python 3.11+, Git, rustup with Rust **1.96.0** already installed, a native
C linker, and cached packages from the committed Cargo.lock. The recipe uses
`rustup run 1.96.0` and Cargo `--locked --offline`; it never installs/downloads a
toolchain or dependencies. Missing tools/cache fail explicitly. An already prepared
Rust toolchain includes the standard library. Native additionally needs the selected
Xcode/Swift 6 toolchain and SDK; the helper uses public Apple frameworks.

| `--modules` | CLI features | Other executables | Additional build requirement |
| --- | --- | --- | --- |
| `core` | none | validator | No platform collector SDK |
| `web` | web | validator, worker with web | No Swift build; no Node/browser build tools |
| `native` | macos | validator, worker without web, Native helper | Swift/Xcode SDK |
| `combined` | web,macos | validator, worker with web, Native helper | Swift/Xcode SDK |

Defaults in Cargo manifests are unchanged. Core/Web compilation on a Mac still
uses its normal native linker/SDK; “no collector SDK” does not mean a Mac binary
can be linked without a system toolchain. Linux/Windows/Intel and older macOS
runtimes are unqualified. Native's macOS 14 deployment target is a compile
setting, not an older-runtime support promise. No iOS/Android/TV adapter is shipped.

## Build into an existing directory

Run from the checkout (or use an absolute script path). Choose an existing absolute
folder you own; unrelated files may be present. No named bundle file may exist,
including symlinks. The destination itself must not be a symlink. The script creates
no destination directory and never changes PATH, home settings or permissions.

```sh
BUNDLE="$(mktemp -d "${TMPDIR:-/tmp}/uib-local.XXXXXX")"
python3 distribution.py build --modules combined --revision HEAD --destination "$BUNDLE"
python3 "$BUNDLE/distribution.py" verify --destination "$BUNDLE"
```

Replace `HEAD` with a full saved commit to repeat source selection. All product
binaries, schemas and example inputs come from that revision, ignoring checkout
WIP. Builds use a private system-temp source archive, Cargo target and Swift module
cache; there is no branch/worktree change. The chosen release profile is optimized.
Build failure publishes no bundle. Exclusive publication rolls back its own files
on ordinary IO errors; `distribution-manifest.json` is written last as the completion
marker. A killed process/power loss can leave partial files without that marker:
never treat them as an installed bundle. Build again in a fresh existing folder;
manually inspect remnants before removing them. Do not run concurrent mutation or
remove while a bundle is in use. Do not replace individual executables.

The manifest records exact product source revision, feature sets, target, Rust/Swift
versions, lock hash, selected dependency checksums and all installed file hashes.
The invoked recipe and companion docs are copied with their own hashes; they may be
newer than `--revision`. Reproduce both the source pin and the recipe revision from
this repository. Reproducibility means pinned source, dependencies and procedure;
byte-identical compiler output across paths/SDKs/hosts is not promised.

I02 qualifies product source `94724dfd412f966d3d7a90db29aec8be7e35d650` with the
recipe last changed at `6a5bec2` (including Native form session helper sources).
Use that exact product pin to reproduce I02; `HEAD` selects whatever is committed
when you run. The [I02 receipt](../plans/ui-blueprint/receipts/I02-current-distribution.md)
records module checks and the separate documentation/checkpoint revision.

## Flat artifact layout

Everything is directly under the chosen directory; no package manager is needed:

- `uiblueprint`, `uiblueprint-validate`; selected `session-worker`, `native-host-helper`.
- `uiblueprint-0.1.0.schema.json`, `uiblueprint-analysis-0.2.0.schema.json`.
- `example-snapshot.json`, `example-query.json`, `example-evaluation.json`,
  `example-expectation.json`, `example-observed-brief.json`, `example-proposed-brief.json`.
- `RUN.md` (this guide), `distribution.py` (verify/remove), `THIRD_PARTY_NOTICES.md`,
  `DEPENDENCY_LICENSES.txt`, `RUST_LIBRARY_NOTICES.html`, `distribution-manifest.json`.

Examples are deterministic synthetic data, not current observations. Snapshot and
Expectation are extracted from the saved canonical analysis fixtures, without
changing source identity, values or evidence. No image or fixture application is
installed. Build intermediates are removed; any encountered images and their
containing system-temp directories are retained, never agent-cleaned.

## First run from any working directory

Keep BUNDLE as the absolute output printed by build. The following works after
`cd /`; binaries do not need the source checkout or a runtime Python/Rust toolchain.
Python is needed only for the distribution manager.

```sh
cd /
"$BUNDLE/uiblueprint" --help
"$BUNDLE/uiblueprint-validate" --max-bytes 131072 "$BUNDLE/example-snapshot.json"
"$BUNDLE/uiblueprint" measure --snapshot "$BUNDLE/example-snapshot.json" \
  --query "$BUNDLE/example-query.json" --evaluation "$BUNDLE/example-evaluation.json" \
  --space local-form --max-input-bytes 131072 --max-output-bytes 131072 --json
"$BUNDLE/uiblueprint" check --snapshot "$BUNDLE/example-snapshot.json" \
  --expectation "$BUNDLE/example-expectation.json" --space local-form \
  --max-input-bytes 131072 --max-output-bytes 131072 --json --result-version 0.2.0
EXPORT_PARENT="$(mktemp -d "${TMPDIR:-/tmp}/uib-export.XXXXXX")"
"$BUNDLE/uiblueprint" imagegen-prompt --brief "$BUNDLE/example-observed-brief.json" \
  --purpose document --out "$EXPORT_PARENT/document" --max-input-bytes 1048576 \
  --max-output-bytes 1048576 --max-components 100 --max-views 10 \
  --components-per-detail 10 --json
"$BUNDLE/uiblueprint" imagegen-prompt --brief "$BUNDLE/example-proposed-brief.json" \
  --purpose propose --out "$EXPORT_PARENT/proposal" --max-input-bytes 1048576 \
  --max-output-bytes 1048576 --max-components 100 --max-views 10 \
  --components-per-detail 10 --json
```

Expected: validator valid/0; measure **8 css_px** in local-form; check pass/0.
Each export writes six text/JSON files and reports `generated_image=false` with
unverified visual status. The local command never calls ImageGen or needs a key.
Example byte/count limits are explicit example parameters, not production defaults.
Keep exports separately; the distribution manager never deletes them. Remove only
your own non-image example outputs after use; retain any images and their folders.

For saved-pair comparison, supply two canonical Snapshot/observed ChannelResponse
files and explicit comparison metadata as described in [CLI export](cli.md).
Use the same export limits as above, replacing both the `--brief FILE` and
`--purpose document|propose` arguments with:

```sh
--before "$BEFORE" --after "$AFTER" --metadata "$COMPARE_METADATA" --purpose compare
```

The pair is analyzed locally. Comparison packages/receipts use version **0.2.0**;
other exports retain **0.1.0**. Source coverage, unknown values and independent
statuses remain explicit. No generated image or fresh observation is implied.

## Live use and matching components

The CLI keeps its explicit paths and trusted connection/request grammar:

```sh
"$BUNDLE/uiblueprint" observe --connection "$CONNECTION" --request "$REQUEST" \
  --worker "$BUNDLE/session-worker" --max-input-bytes 2097152 --max-output-bytes 524288
```

Use a trusted operator-authored connection for the **current** authorized target.
For Native set its `provider.helper_executable` to the absolute
`$BUNDLE/native-host-helper` path (expand the variable when authoring JSON).
For Web, establish an authorized numeric-loopback CDP target/document separately.
This build does not launch/discover an app/browser, grant AX/capture permission,
collect UI or prepare action authority. Workers and helpers use the private owned
FD protocol and are not standalone commands. Relocation requires updating explicit
paths in your trusted connection, then verifying the bundle again.

Native/combined also include the bounded `native-session` entry point:

```sh
"$BUNDLE/uiblueprint" native-session --connection "$CONNECTION" \
  --worker "$BUNDLE/session-worker" --duration-ms 120000 \
  --max-input-bytes 2097152 --max-output-bytes 524288
```

This requires an explicitly configured own-fixture `native_fixture` connection with
AX channels1, helper `collection:"form"`, bounded session duration and exact form
identifiers; see [Native session setup](native-helper.md#attached-native-forms-m02-n).
Bounded stdin records name canonical request/source/expectation files. EOF ends
owned resources; refs do not survive CLI exit. No UI access occurs merely by
building or requesting `--help`. Core/Web refuse this command as unsupported.
I02 checks entry/feature availability without attaching to UI; actual input,
privacy and lifecycle acceptance remain separate Q01 responsibilities.

Canonical input/version failures keep their existing exits: invalid/limit2,
IO/internal1, unknown/incomplete4, unsupported5; check mismatch3. Observe can return
partial NDJSON with a nonzero exit, so inspect both. Do not retry an action whose
effect is unknown. Full live setup remains in the source checkout's CLI/Native
helper documentation; no new discovery or configuration defaults are introduced.

## Verify, recovery and removal

```sh
python3 "$BUNDLE/distribution.py" verify --destination "$BUNDLE"
python3 "$BUNDLE/distribution.py" remove --destination "$BUNDLE"
```

Verify checks the complete expected inventory, hashes, file types and modes.
Remove first verifies **all** owned files, then removes only those named files and
the manifest. Unrelated files and the destination directory remain. Paths in a
manifest cannot add arbitrary removal targets: the allowed flat inventory is fixed.
No recursive deletion is used. Missing/changed files, symlinks or a malformed
manifest cause refusal before removal. This is accidental-mixing detection, not a
signature or protection against someone rewriting both manifest and manager.

If a worker/helper is incompatible or hashes differ, stop using that set. Build a
fresh complete bundle from one saved revision in another existing chosen directory,
verify, then update trusted connection paths together. Never copy just a worker from
a different revision. Keep a changed/foreign file until you decide how to recover it;
the tool does not force overwrite, repair or delete it. If the bundle verifies and
is no longer in use, `remove` followed by `build` reinstalls in the same directory.
A partial failure during removal is reported, never success; missing files then
require manual inspection or a fresh destination. Do not delete the surrounding
folder when it contains other data or images.

This procedure closes local build/install/recovery only. Positive Mac/Web pilots,
isolation/privacy V01, integrated Q01/Q02 and P7 release acceptance are separate.
No signed/notarized/archive release, global installation or project license grant
is implied. The source checkout remains necessary for rebuilding and full developer
fixtures; the delivered saved-data commands need only the selected executable set.
