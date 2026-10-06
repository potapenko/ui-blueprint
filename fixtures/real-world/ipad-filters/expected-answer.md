# RC04 — reviewer key (withhold from evaluated agent)

## Supported observations

- Five states: Pro 13 default, three-condition inspector and closed summary;
  Pro 11 default and default inspector, all landscape.
- Pro 13 default shows Filters. Three changes are lower CEFR bound A2
  (upper C2 retained), Only idioms and Only questions. The closed AX control has
  name Filters and value A2–C2, Only idioms, Only questions.
- Pixels show the selected caption truncated after A2–C2; a complete two-label
  plus +1 summary is not visible. Do not repair the screenshot in the answer.
- Pro 11 default caption is visibly F… and some tab labels wrap/truncate,
  while AX reports Filters and the full tab names.
- Both inspectors have native popover appearance and an arrow near the
  bottom-leading edge of the Filters button. This is a visual annotation,
  not a measured attachment point or verified framework ownership.
- Idiom/question controls are AX buttons with selected state on Pro 13;
  pixels show checked boxes. CEFR, Emotion, Polarity and Topic are AX pop-up
  controls. Topic is reported in AX but outside the visible captured form area.
- Reset and an X close affordance are visible. The selected AX excerpt does
  not expose them as separately named controls. Do not claim observed Done text.
  Pro 13 X was activated; Pro 11 exposed Cancel was invoked, without a
  retained post-dismissal visual proof.
- Default paging is 1–20 / 62,321, first/previous disabled. Three conditions
  yield 1–17 / 17 with all four navigation buttons disabled. These are reported
  values for this capture, not constant catalog sizes or paging execution proof.
- Selected source/query differ across devices. Pro 13 is The Good Night (2007),
  1/31; Pro 11 is Being There (1979), 1/215. Both have visible Play.

## Normative expectation, not measured evidence

The [iPad filter contract@3](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/ipad-learner-filter-controls.md)
requires the two-value +N summary, complete VoiceOver value, native
bottom-leading attachment, immediate changes, dismiss-only Done and 44-point
interaction regions. The caption observations expose a visual discrepancy
against that summary expectation. No source repair or product acceptance was
performed; exact hit bounds and VoiceOver operation remain unknown.

## Reject overclaims

Native display PNGs are 2752 × 2064 and 2420 × 1668; preferred UI scale is 2.
Dividing gives display extents 1376 × 1032 and 1210 × 834 pt, not app content
rectangles. AX element bounds, hit regions, padding, radii and materials were
not measured. Exact source app build is pinned in the manifest/receipt.
No Vocabulary, compact sheet, all-size, RTL, live collector or agent acceptance.
