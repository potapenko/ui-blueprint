# Changelog

The version covers the complete UI Blueprint distribution: CLI, collectors,
helpers and agent skill. Versions of recorded data formats are independent.

## 0.1.0

First numbered release candidate; not published yet.

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
  preparation from a pinned commit.

Initial binary target: Apple Silicon macOS. See the candidate's minimum OS and
signing status. Intel, Linux, Windows and older macOS support are not qualified.
