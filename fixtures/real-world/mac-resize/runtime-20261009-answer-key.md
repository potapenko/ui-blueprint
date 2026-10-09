# RC03 — reviewer key for the 2026-10-09 runtime pair

Withhold until the evaluated agent's answer is saved. This is a producer-authored
reference interpretation checked against original sources and actual Rust Measure
outputs, not independent product/collector/agent acceptance.

## Actual reported geometry

All values below are accessibility_bounds in ax-screen/screen/pt/top_left.
Rectangles are x,y,width,height. Match within each original response using exact
SourceKey; cross-record correspondence uses unique AXIdentifier, or for repeated
tab IDs the unique identifier+name+native role. No stable cross-request ref follows.

| Reported node | Narrow | Wide |
| --- | --- | --- |
| learner AXWindow | 0,30,961,1050 | 0,30,1920,1050 |
| searchLearn.root | 0,30,1018,1050 | 0,30,1920,1050 |
| suggestedCommonPhrases.overlay | 8.5,90.5,323.5,809 | 8.5,90.5,513,809 |
| learner.commonPhrases scrollarea | 555,133,463,766.5 | 1391.5,133,520,766.5 |
| transport | 304.5,383,352,374.5 | 784,383,352,374.5 |
| transport.toggle / Play | 409.5,383,142,142 | 889,383,142,142 |
| query.band | 0,1003,961,77 | 0,1003,1920,77 |
| tab Learn Common Phrases | 620.5,90.5,64.5,42.5 | 1391.5,90.5,155.5,42.5 |
| tab Vocabulary | 690,90.5,62,42.5 | 1552,90.5,93,42.5 |
| tab Favorites | 757,90.5,62,42.5 | 1650,90.5,83,42.5 |
| tab Grammar | 824,90.5,62,42.5 | 1738,90.5,83.5,42.5 |
| tab Statistics | 890.5,90.5,62,42.5 | 1826.5,90.5,85,42.5 |

Do not equate learner.overlay's broad group rect with the visible right panel:
it is 8.5,90.5,1009.5,809 narrow and 8.5,90.5,1903,809 wide.
The narrow root/right-scroll reported right edge reaches1018 while AXWindow
ends961. Preserve these real reported values; do not clamp or rewrite them.
This discrepancy warrants inspection, not an invented root-cause diagnosis.

## Actual existing Rust Measure results

| Relation | Narrow pt | Wide pt |
| --- | ---: | ---: |
| Left scroll width | 323.5 | 513 |
| Right scroll width | 463 | 520 |
| Transport width | 352 | 352 |
| First learner tab width | 64.5 | 155.5 |
| Left scroll right → transport left gap | -27.5 | 262.5 |
| Transport right → right scroll left gap | -101.5 | 255.5 |
| Transport/window horizontal center alignment deviation | 0 | 0 |
| First-tab right → Vocabulary left gap | 5 | 5 |

Negative gaps are intersections of reported bounding rectangles, not proof of
paint occlusion or failed pointer input. The transport's unchanged width and
center relation differ from the panels' width changes: not uniform bitmap zoom.
Gaps are not CSS/SwiftUI padding. Sixteen original MeasurementCase outputs retain
unchanged source Snapshots; queries and [summary](</var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-rc03-20261009-r775a559/measured-summary.json>)
are reviewer material. No cross-Surface geometry-diff identity was fabricated.

## Visual evidence and exact limits

Native narrow image shows wrapped suggestion text/counts and shortened/wrapped
right tab labels; wide shows longer labels and a broader composition. These are
visual annotations, not API-reported clipping or text intrinsic size.
Narrow titlebar is inactive-looking, wide active-looking; foreground state was
not instrumented. No pixel parity or unaffected-focus acceptance claim.
Both show Bridge to Terabithia (2007) [00:30:26], the same query and4/1,650, Play.
No action or playback was exercised except authorized window-size setup.

AX coverage is partial at160 nodes. Video-only bounds, layout, hit/paint/visible
regions, text extents and safe content remain unavailable/not established.
Image mapping unknown; 2x buffer/window dimension ratio alone is insufficient.
The [point-sizing@1 expectation](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/point-sizing.md)
is separate from these measurements; 1440×900 and all supported sizes not tested.
First five-field Observe failed incomplete_scope; preserve failure and do not
attribute its cause to a particular budget without evidence. Geometry-only
Observe succeeded twice. This closes measured-data availability for this pair,
not full Q03 usefulness scoring, D06 timing or product QA.
