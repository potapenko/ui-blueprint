# RC02 expected answer and reviewer key

Withhold from the evaluated agent. This key is not an instruction to change
PlayPhrase.me and is not a production collector/schema acceptance claim.

## Genre

- Trigger: clipSearch.filter.genre-trigger, value Any genre.
- Popover: clipSearch.filter.genre-popover, visible title Genres.
- Clear is reported disabled. Fourteen options are reported, first Action focused.
- The native image shows two columns of checkbox-like options, seven rows.
  The supplied CUA representation reports the options as button, not checkbox,
  and does not expose checked values. Preserve both facts: do not silently
  replace reported roles or claim API-reported checked=false.
- The displayed squares look unmarked (visual annotation). No selection was made.

## Director

- Trigger: clipSearch.filter.director-trigger, value Any director.
- Popover: clipSearch.filter.director-popover, visible title Directors.
- Clear is disabled; one settable text field Director name is focused, placeholder
  Type a director. Search by director name is a text guidance node, not an input.
- No query was typed and no result rows were observed. The absence of result rows
  is not a proven request failure or empty result for a completed lookup.
- AX omits query value; visual placeholder appearance is a separate observation.

## Surfaces and geometry

Both appear right of their respective triggers with a pointer in the supplied
images (visual annotation). Narrow-width fallback was not exercised.
CG inventory reported same-process popup candidates 2950 (456 × 306 pt at 272,186)
and 2955 (436 × 348 pt at 272,239). Their association with AX popovers is supported
by time/appearance but not an exact AX-to-CG mapping API; do not claim otherwise.
These are native window bounds, not the inner content/padding/hit boundaries.

Whole-window screencapture included each open popover in this actual run.
Do not generalize to ScreenCaptureKit isolated-window capture or every backend.
Crops retain the relevant trigger and popover; their ROI is not measured layout.
Per-control geometry, material parameters, radii and exact hit regions are unknown.

The native window title remained Search & Learn / English while clipSearch.root
content was active. Title alone does not establish destination identity.
An inline locked-results/Unlock area existed in the background. It was not a
blocking access modal and its actions were not exercised.

## Actions actually observed

Sections → Clip Search; open Genre; Escape; open Director without typing; Escape;
return to Search & Learn. End-of-case triggers remained Any genre, Any director,
Any cast member and Any voice. No selection, Clear, upload/download/purchase,
outside click or grid hit-test occurred. Immediate commit/multi-select semantics
are contract expectations, not newly verified outcomes.

## Contract basis and evaluation

[macOS filter-popover@1/source r2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-08-26-macos-native-filter-popover-evolution.md)
owns these expectations: Genre two columns, entity search/counts, immediate choices,
filter-local Clear, Escape/outside dismissal, no Done, system-owned fallback.
Only the observed subset above can be marked verified.

Material mistakes: inventing a second Director input from the guidance;
assuming stored counts/results without a query; converting AX buttons into
reported checkboxes; inferring exact padding from native-window bounds;
claiming all parent-window capture excludes/includes popovers; treating the
stale title or historical ref as a current actionable identity.
Record criterion-level outcomes; do not let a fluent answer compensate for
unsupported geometry or unsafe action claims.
