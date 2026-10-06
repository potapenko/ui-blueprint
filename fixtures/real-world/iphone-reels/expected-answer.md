# RC05 — reviewer key (withhold from evaluated agent)

## Supported observations

- Ordinary large: title above video; source/counter and karaoke overlay media;
  summary/hint below; two action rows with trailing Translate/Manual visible.
- At maximum accessibility category, video is above the reading region; title
  wraps into three lines below it. Source is below title, and the counter is
  partially below the visible reading viewport in the top capture.
- After the bounded vertical touch drag, the same video stays in the upper
  region while lower hint and both action rows appear. Leading four controls
  and part of the next are visible per row. Trailing selectors are not visible.
  The top-clipped A1 is a scrolled text fragment, not evidence of data loss.
- AX exposes title, source Happy Death Day (2017), 1/40, Play and the action
  inventory before and after. Accessibility top/lower selected AX strings are
  identical despite changed pixels. Tree presence does not prove visibility,
  scroll offset, hit-test readiness or successful activation.
- A wheel scroll did not visibly move the content; the subsequent below-video
  touch drag did. A horizontal row drag also did not visibly move content.
  Do not claim complete selector reachability or diagnose the cause.
- The displayed source/counter/text and Play support continuity of the observed
  Reel. Exact underlying Reel ID, mounted player identity and remount behavior
  were not instrumented.
- The root Random control has no readable label/icon in the maximum-text frames.
  This is visible evidence to flag, not a new authorized source-app fix.
- Ordinary title is multiline but short in content; a separate long description
  and translated content were not captured. Other Dynamic Type sizes and
  physical devices are untested.

## Specification versus observation

[Aligned column@5](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/iphone-reels-aligned-column.md)
defines ordinary alignment, ACTIONS.FILL and ACCESSIBLE.REFLOW. It expects
readable wrapping, a video above scrollable reading content at accessibility
sizes, reachable two-row actions and stable mounted-player identity.
The evidence supports the observed reflow and vertical row exposure only.
It does not close that leaf's full acceptance matrix.

Native screen size is 1206 × 2622 px and reported UI scale 3; 402 × 874 pt is
a derived whole-display extent. No exact control, hit, safe-area or content
bounds were measured. No UI Blueprint collector or evaluated-agent run occurred.
