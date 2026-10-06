# Native pilot fixture

This is the canonical F02 synthetic fixture, not a shipping macOS plugin.
See [scenarios and measurements](../../docs/development/fixtures-native.md).
The historical [R02 experiment](../../experiments/native/README.md) remains intact.

The current workflow is **explicit request → one bounded measurement → response**.
There is no periodic publisher, polling loop or collection cadence. Changing UI
state does not publish measurements. Press **Snapshot** after establishing the
requested state, then invoke the external one-shot observer. The on build resolves
SwiftUI anchors after layout for that request; the off build returns no probe
measurements. Anchor declarations are not measured bounds. The transparent
measurement background is excluded from hit testing and accessibility.

```sh
fixtures/native/script/build.sh /absolute/task-temp/build
mkdir -p /absolute/task-evidence/live
open -n /absolute/task-temp/build/F02-on.app --args --run-dir /absolute/task-evidence/live
# In this owned app: choose A or B, establish state, then press Snapshot.
python3 fixtures/native/capture.py /absolute/task-temp/build/f02-observe \
  /absolute/task-evidence/live/a.json /absolute/task-evidence/request-1
```

Every output directory must be new. `a.json` and `b.json` are the last explicitly
requested fixture receipts, not live state or a cache freshness promise. Request
Snapshot again after an action, close/reopen or other relevant state change.
The helper always re-reads live AX/capture, validates PID/launch time/CG owner and
compares the supplied surface generation with the current explicit fixture receipt.
It does not expose actionable refs. It is not a production revalidation protocol:
without a fresh Snapshot, the file alone cannot certify an intervening reopen.

For a finite statistical experiment in an unchanged state, `--cold-runs 5` runs
five one-shot helper processes. `--samples 20` runs twenty explicitly requested
measurements in one helper, with no subscription or automatic continuation. Do
not use either as a background monitor. Cold/warm sample sets have distinct
statistics. `--request-timeout 0.001` is an explicit driver-timeout stimulus; it
kills and reaps only its owned read-only helper and records that fact.

Controls: Open A/B, Reset, Snapshot, Change layout, Resize, Move, Compare,
form inputs/Complete Ada/Apply, Edge popup, Scroll start/end and synthetic stimulus.
Move uses a small AppKit adapter for the exact own existing window. SwiftUI
initial/zoom placement APIs do not express this imperative operation on the
macOS 14 compile target; all visible UI and application state remain SwiftUI.
Compare resets state, expands the sample and requests own-window placement/key
activation. It is setup, not evidence: verify active/key/main/focus before comparing.
Close/reopen rotates the fixture generation even when SwiftUI retains the same
NSWindow and CGWindowID. Reset resets content; it does not promise a new generation.

Only the explicit own fixture bundle IDs are allowed. AX requests are bounded to
160 nodes, depth 9 and a 3-second traversal budget, with 0.2-second object timeout.
A 45-second process watchdog bounds an unresponsive capture; the driver also has
a 50-second default process deadline. These are diagnostic limits, not frozen
product gates. The current helper writes a combined result after capture; if
ScreenCaptureKit never resumes, that request's AX result is not persisted. This
limitation is recorded, not masked by older observations.

Secure values are absent from fixture manifests and not requested by the external
collector. The synthetic form is not a universal secret-detection test. Capture
uses an isolated exact window, audio off, child windows excluded. Popup semantics
may be in the parent AX tree while its pixels are outside this capture. Surface
mapping, image calibration and real OS permission/notification failure remain
adapter work. Never change TCC or display settings for this fixture.

No inputs are synthesized by the Python driver; use the granted Computer Use lane
for visible actions. Stop only the known owned process when finished. Outputs,
bundles and compiler logs belong outside Git. `check.py` checks the recorded F02
run and reports open gates explicitly; it is not a full M01–M06 release oracle.
