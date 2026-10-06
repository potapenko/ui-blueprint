# RC03 reviewer answer key

Withhold this file from the evaluated agent. No product fix or new collector
acceptance is established by this reference pair.

## Reported facts

- Same canonical existing Release/PID/main CGWindowID; query, source label,
  first-item counter 1/40, paused Play, Common Phrases tab, speed/repeat match.
- Actual outer window frames: narrow 961 × 1050 pt, wide 1920 × 1050 pt.
  Both use screen origin (0,30). PNGs are 1922 × 2100 and 3840 × 2100 px: 2x.
- Width changes by 959 pt; height/origin do not change. These are outer-window
  measurements, not verified content or per-control sizes.
- Actual capture order is narrow then restored wide. The requested 1440 × 900
  reference was not obtained and must not be substituted for the measured data.
- Historical CUA indices differ; the retained source IDs/semantic values support
  correspondence within this case. They are not globally stable executable refs.

## Visual review annotations

The narrower image retains left suggestions, central transport/video, right learner
content and bottom query/actions. Suggestion text/counts wrap onto additional
lines; the visible number of rows changes. Labels/tabs in the right learner region
appear compressed/truncated/clipped. Do not turn this observation into a full
diagnosis of implementation causes or accepted narrow-layout quality.

These visual changes support reflow/compression, not merely shrinking the whole
wide bitmap uniformly. Exact font sizes, gaps, padding, content bounds and hit
regions remain unknown. No interaction/scroll reachability test was performed
on either captured size. Presence in AX does not prove visible clickability.

## Expectation source

[point-sizing@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/point-sizing.md)
expects intrinsic native-point sizing and container redistribution, preserves
topology/primary routes, and names paused 1440 × 900 as the reference. Its values
are requirements, not measurements of this run. This pair does not close the exact
reference-size scenario or prove every supported width/height.

## Limits and grading

Material errors: claiming 1440 × 900 was captured; window/content conflation;
invented per-control metrics; unqualified no-clipping/primary-action pass;
FPS/jank/animation claims; calling this a UI Blueprint collector test;
presenting the setup incident as unchanged app state throughout the run.

Accept criterion-level accurate / inaccurate / not-evidenced judgments. Before
these two captures, unintended playback during setup advanced to an access gate.
Visible first clip/query/pause were restored before capture; internal viewing/
statistics effects are not asserted unchanged. Binary source commit is unknown.
