# Real Mac resize reference pair

F03b / RC03, 2026-10-06. Two explicitly collected paused Search states, no
continuous observation. This is a real-app data pack, not collector or agent QA.

Inputs: prompt.md, observations.json, two AX excerpts and their PNG files.
Keep expected-answer.md separate. JSON is fixture data, not a competing production
schema. Exact content/per-control bounds are unknown.

## Captured sizes and residual

Native Window → Move & Resize → Left produced 961 × 1050 pt. Window → Move &
Resize → Return to Previous Size restored the original (0,30,1920,1050) pt.
Capture order is narrow then wide, not a claimed chronological wide→narrow edit.
Reference 1440 × 900 was attempted but not obtained with the used CUA drags;
the app's support for that exact size remains unverified.

The selected PNGs were captured independently via screencapture -x -o -l1277.
No CUA frame became a reusable image. Sources were retained losslessly and inspected
at original detail; no crop or resize was applied. Pixel/point ratios are 2x.

## Setup incident and restoration

One unsuccessful drag hit the video and started playback. The selection reached
the access gate. The gate was dismissed without login/purchase and Return restored
paused intent on the original first clip. Both selected captures were made after
this recovery. Query, source/counter, tab and paused state matched, and original
window frame was restored. Internal view/statistics side effects are unknown;
no app data was altered to erase them. No source or preferences were edited.

## Reproduction

Use a newly authorized bounded packet and current source runtime routes.
Acquire macos-product then desktop; rediscover canonical process/window IDs.
Establish one paused Search and record state before any resize. Prefer a proven
native sizing route; never claim requested dimensions without measurement.
Collect each chosen state once; restore geometry/state and release leases.
Do not reuse historical IDs. No scene bootstrap, SDK or source modification.

Raw evidence remains at `/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/real-cases/F03b/2026-10-06-1usid7xh`, owned by root for Q03/P7 until acceptance
or explicit discard. Build-source correspondence is unknown for the reused binary.
See mobile-prerequisites.md for the separate read-only mobile inventory.
