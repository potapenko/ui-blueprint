# Changelog

The version covers the complete UI Blueprint distribution: CLI, collectors,
helpers and agent skill. Versions of recorded data formats are independent.

## 0.4.0

- Include the macOS SwiftUI cached-layout reader and its dependencies in native
  packages. A qualified owner-run on the paused Reels card returned 38 nodes,
  including a title and the measured video frame. The action-bar boundary was
  identified but its own bounds were unavailable.
- Keep scoped acquisition tied to the approved consumer, process generation,
  window and source types. Ordinary installation grants no debugging session.
- Read and measure the resulting canonical snapshots with the existing CLI.
  Local node coordinates require separate evidence before mapping to an image.

## 0.3.2

- Accept the current manifest5 native/combined distribution in `native-prepare.py`,
  retaining manifest4 compatibility and all file hash/mode and process guards.
- Exercise the shipped preparer during actual candidate verification so a bundle
  inventory update cannot silently break prepare before observing a real window.

## 0.3.1

- Resolve the installed CLI symlink before locating the matching native helper,
  so `capture-window` works through the normal `~/.local/bin/uiblueprint` entry.

## 0.3.0

- Add `capture-window` for an exact already-running macOS process/window: independent
  lossless PNG, source dimensions/scale, no-shadow convention and attributable
  mapping metadata without application instrumentation changes.
- Add scoped `--list-windows` numeric metadata for the selected PID and a matching
  installed skill recipe. Existing SDK Snapshots and geometry commands are unchanged.
- Local candidate only until separately authorized publication; older installed
  versions remain available for verified rollback.

## 0.2.0

Window-root geometry and matching agent installation. Install the source release
through the [tag-pinned guide](https://github.com/potapenko/ui-blueprint/blob/v0.2.0/docs/install.md).
No binary attachments; local preparation does not imply publication.

- Add opt-in SwiftUI and GPUI Debug producers for real runtime layout, using the
  existing canonical Snapshot inspection and Rust geometry analysis.
- Add root collection of mounted marked descendants with required-anchor checks,
  preserving explicit component selection and source identity/coverage limits.
- Route the agent skill from the selected window/root by default and include the
  complete SwiftUI/GPUI operating references in the installed matching package.
- Preserve update and rollback for historical installations while verifying the
  complete new skill inventory and its portable links.
- Add shared Rust lint policy and pull-request checks for supported CLI features.
- Pin source builds and the optional GPUI probe to stable Rust1.100.0, released
  2026-11-12. Existing dependency versions and data formats are unchanged.

Internal geometry requires explicit integration in the target application's Debug
build. It covers measured marked boundaries, not every private framework view;
layout, Accessibility, hit, paint and screen/image coordinates remain distinct.
Initial local build/runtime qualification remains Apple Silicon macOS27.0.1.

## 0.1.0

UI Blueprint source release under MIT. Install through the agent prompt in the
[README](https://github.com/potapenko/ui-blueprint/tree/v0.1.0#install-through-your-agent).
The [tag-pinned instructions](https://github.com/potapenko/ui-blueprint/blob/v0.1.0/docs/install.md)
build the utility locally and install its matching skill. No binary attachments.

- Local Rust inspection, geometry measurements, requirement checks and comparison
  of saved UI observations.
- Web and macOS observation, with explicit target selection, source attribution
  and partial/unknown coverage preserved.
- Opt-in Kotlin Compose geometry collection for Desktop and Android debug apps.
- Engineering blueprint packages and ImageGen prompts from observed geometry.
- Matching CLI and portable agent skill, with verified installation and rollback.
- Browser-session CDP support for Chrome's permission-based debugging, including
  the rooted-selection compatibility fix verified on Chrome154.
- One numeric product version, `uiblueprint --version`, and manual GitHub release
  verification from a pinned commit.
- Prompt-based installation builds the utility locally from the release tag and
  installs its matching skill. GitHub releases contain no binary attachments.

Local source builds are initially qualified on Apple Silicon macOS27.0.1, with
Git, Python3.11+, Rust1.96.0 and Xcode/Swift6. Node24+ is optional for Web observation.
Intel, Linux, Windows and older macOS support are not qualified.
