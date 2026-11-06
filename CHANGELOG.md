# Changelog

The version covers the complete UI Blueprint distribution: CLI, collectors,
helpers and agent skill. Versions of recorded data formats are independent.

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
