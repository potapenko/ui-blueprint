# Real application reference cases

F03a supplies three requested states from the existing canonical macOS Release of
PlayPhrase.me: Settings, Genre open, Director idle/open. No iOS/iPadOS examples
or production UI Blueprint collector were created by this packet.

| Case | Agent input | Reviewer key | Evidence |
| --- | --- | --- | --- |
| RC01 Settings | [prompt](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-settings/prompt.md), [observations](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-settings/observations.json), AX excerpt | [key](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-settings/expected-answer.md) | Whole native window and dialog crop |
| RC02 Genre/Director | [prompt](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-filters/prompt.md), [observations](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-filters/observations.json), two AX excerpts | [key](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-filters/expected-answer.md) | Two whole native windows and bounded popover/anchor crops |
| RC03 Mac resize | [prompt](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-resize/prompt.md), [observations](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-resize/observations.json), two AX excerpts | [key](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-resize/expected-answer.md) | Two actual native window sizes; requested 1440 × 900 not obtained |

Give only the agent inputs and referenced PNGs to an evaluated agent. The answer
keys contain expected interpretation/spec references and must remain separate.
A model answer was not generated or scored in F03a; Q03 is the consumer.

These records support offline reasoning/normalization/export checks. Only a later
live run of a UI Blueprint platform collector can establish that collector's
capabilities. Manual visual annotations are reviewer evidence, not API-reported
layout. App observations never silently redefine its contract.

## Provenance and retention

- Source checkout: beta-01 at 48d0dfd1cd0fe46ac88c073e99ffab7bd1abe76e.
- Existing canonical app: PlayphrasemeApp Release product PlayPhraseMe.app,
  PID 74964. Its executable path/hash and build metadata are in each JSON.
- Exact binary-to-source commit correspondence remains unknown; no build was run.
- Capture: native screencapture to lossless PNG, no shadow/resizing; sips crop only.
  CUA screenshots were not saved or used as reusable assets.
- Native CG main-window bounds: (0,30,1920,1050) pt; source PNG: 3840 × 2100 px.
  Per-element AX/layout/hit/paint bounds, materials and radii remain unknown.
- Evidence store: `/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/real-cases/F03a/2026-10-06-8z711yn3`.
  Owner: root coordinator; consumers C01/Q03/P7; retain through acceptance/discard.
  Images and raw metadata stay outside Git; selected normalized case data is tracked.
- Exact AX call timestamps were not retained separately. Native PNG file times and
  a conservative enclosing run interval are provided, without atomicity/latency claims.
- Observe is on-demand only. No cadence, background monitor or animation sampling.

## Runtime scope

Opened Settings with Command-comma and closed with Escape. Used Sections to enter
Clip Search, opened Genre then Director separately and dismissed each with Escape.
No typing, selection, Clear, login, purchase, download or settings changes occurred.
Restored paused Search with its original query and current phrase. Preserved the
pre-existing user process. Released both macos-product and desktop; stopped the
exact F03a caffeinate session.

Follow [source runtime acceptance](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-runtime-acceptance.md),
[lanes](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-runtime-lanes.md) and
[UI tool policy](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/ui-tool-policy.md) before any reproduction.
An old screenshot, CUA index or CGWindowID is not a fresh runtime target.

## Open limits

These examples do not close canonical new-build QA, narrow-window fallback,
filter commit/results/Clear, preference persistence, all appearance/accessibility
modes, platform collector acceptance, agent quality or mobile implementation.
The current window title stayed Search & Learn / English in Clip content, and the
Genre AX text reported buttons while pixels showed checkbox-like controls; both
are retained as useful interpretation cases without fixing the application.

## F03b addition RC03

The [resize pair](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-resize/README.md) adds actual
961 × 1050 pt and restored 1920 × 1050 pt windows, captured narrow then wide.
Reference 1440 × 900 was not obtained with the used CUA sizing route; do not
report it as measured or infer that the app cannot support it. Content/control
bounds remain unknown. Both chosen captures show the same paused first clip.

A setup drag inadvertently started playback and exposed an access gate. Visible
query/item/pause were recovered before capture; internal view/statistics effects
were not inspected or reverted. This is disclosed, not part of the resize proof.
Window geometry was restored exactly; runtime leases were released.

F03b assets live at `/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/real-cases/F03b/2026-10-06-1usid7xh`, owner=root, consumer Q03/P7 through acceptance
or explicit discard. PNGs were native lossless captures without resampling.
See [mobile prerequisites](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-resize/mobile-prerequisites.md)
for read-only device/runtime findings. No mobile app was launched or configured.
