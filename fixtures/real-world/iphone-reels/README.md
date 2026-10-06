# RC05 — iPhone Reels Dynamic Type

F03c reference material from PlayPhrase.me Phone Release on a task-created
iPhone 17 Pro Simulator, iOS 27, portrait. The same paused Reel is observed at
ordinary large and accessibility-extra-extra-extra-large, then after scrolling
the reading/action region.

Agent inputs: [prompt](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/iphone-reels/prompt.md),
[observations](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/iphone-reels/observations.json),
[AX excerpts](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/iphone-reels/ax-excerpts.txt) and three PNGs.
Withhold [reviewer key](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/iphone-reels/expected-answer.md).
The key is a reference interpretation, not independent acceptance; no model was
scored. This is not a production snapshot schema or iOS collector implementation.

## Setup and bounded actions

Normal guest continuation → root destination selector → Reels. The selected
phrase was “Hi, it's nice to meet you.”, source Happy Death Day (2017), 1/40.
No Reel was deliberately advanced and observed transport remained Play.
The ordinary title occupied two lines; a separate long description and translated
text were unavailable in this state, so the complete long-text case is not met.

The documented simctl content_size command changed only the exact owned
Simulator from large to accessibility-extra-extra-extra-large. Computer Use
verified the visible result. A wheel scroll below the video did not move content;
a touch drag wholly below the video moved the reading area and revealed both
action rows. Their trailing selectors remained horizontally outside the view.
One horizontal row drag produced no visible change; selector reachability is
unverified, not a proven application defect or a pass.

The selected AX text for accessibility top and lower actions is identical while
the native images differ. AX text alone cannot establish current visibility.
Matching title/source/counter and paused affordance support same-Reel continuity;
they do not prove internal mounted-player identity or absence of remounts.

## Provenance and cleanup

Original internal-display simctl PNGs: 1206 × 2622 px, reported preferred UI
scale 3 and Portrait. Derived display extent: 402 × 874 pt. Safe-area/content
and per-control geometry are unknown. No crop, resize or CUA image asset.
AX and PNG requests are separate, with separately recorded timestamps.

Restored content_size=large and verified the same source, 1/40 and Play.
Stopped/uninstalled this run's app, shut down and deleted this run's device.
No auth, purchase, download, statistics inspection/reset or credential access.
Raw evidence owner=root, consumer Q03/P7, retention through acceptance/discard.
[F03c receipt](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/docs/plans/ui-blueprint/receipts/F03c.md) pins build and ownership.
