# RC04 — iPad learner filters

F03c reference material from PlayPhrase.me Release on two task-created iOS 27
Simulators: iPad Pro 13-inch (M5), then Pro 11-inch (M5), both landscape.
This is external collection for Q03/engine/export reasoning, not an iOS collector
implementation or application acceptance.

## Inputs and separation

Give the evaluated agent [prompt](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/ipad-filters/prompt.md),
[observations](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/ipad-filters/observations.json),
[AX excerpts](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/ipad-filters/ax-excerpts.txt) and the five PNGs
named in observations. Withhold [reviewer key](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/ipad-filters/expected-answer.md).
The observations file is case metadata, not the UI Blueprint production schema.
The key is authored from observed evidence and contracts, not independently
accepted. No agent answer has been produced or scored.

## Acquisition and interpretation

Pro 13: ordinary guest continuation → paused default Search → Filters →
Level from A2 → Only idioms → Only questions → open capture → visible X dismiss →
summary capture. The complete AX value survived visual truncation.
Pro 11: ordinary guest continuation → paused default Search → Filters.
Its default Filters caption was already truncated. No filters changed there.

Each simulator chose its own initial phrase; this is not a same-content
responsive pair. Common Phrases was used; Vocabulary, compact sheet adaptation,
VoiceOver operation and paging activation were not exercised.
All PNGs are original lossless simctl captures of the internal display, with no
CUA-derived assets, crop, resize or upscaling. Display scale/orientation came from
one-shot simctl IO enumeration. Display size is not an app content or hit-test rect.
AX and pixels were acquired separately, not atomically. AX role/availability
does not prove visibility, actual touch size or complete navigation coverage.

## Restore and reproduce

The coordinator interrupted F03c after Pro 13 captured its three states. Before
shutdown, Idioms/Questions were restored; A2 remained pending. On resumption,
the same owned simulator relaunched, rotated back to landscape, and A1 was
restored with the inspector showing both Booleans unselected and other values
unchanged. The app was then stopped/uninstalled and the temporary device deleted.
Pro 11 app/device was similarly cleaned after acquisition.

Reproduction needs a new explicit runtime packet, fresh owned UDIDs and the
[source Simulator route](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-simulator-runtime.md).
Do not replay historical CUA indices or reuse unrelated devices.
Raw evidence owner: root; consumer Q03/P7; retain until acceptance/discard.
Full build identity, device ownership and cleanup are in the
[F03c receipt](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/docs/plans/ui-blueprint/receipts/F03c.md).
