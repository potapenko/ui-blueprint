# Native capture identity investigation — 2026-11-24

Owner: [capture investigation](codex://threads/01a20edf-d70e-7a70-954e-3f9aa616c088).
Reporter: [Search](codex://threads/01a20b28-3d04-7212-925d-274bfbd64622).
Status: normal operation qualified; original transient refusal not reproduced or
explained. No production fix or new release is claimed.

## Scope and basis

The delegated request authorized investigation and repair of a confirmed native
capture defect on project-owned fixtures, preserving exact identity and existing
files. The initial plan separated list/CLI identity round trips from late output
publication. No reporter application, window, directory or SDK was modified.

Mode Restore; source baseline 0.8.0 / c44f64957c709d999eb848d917d39bf511f07491.
Traversal: [registry](../../../specs/README.md) →
[NATIVE-CAPTURE@2](../../../specs/product/native-capture.md), CAPTURE.CALL,
IDENTITY, IMAGE, METADATA, LIMITS/FAILURE and ACCEPT; dependencies NATIVE@5,
IDENTITY@1, GEOMETRY@2, PRIVACY@1, CLI@21 and their EXCHANGE@2, PROJECTIONS@1,
LIFECYCLE@1, MODEL@1 and BOUNDARIES@1 closure. The contract permits retained PNG
artifacts after failure; it forbids advertising them as successful captures.
No semantic contract change, numeric tolerance, fallback or automatic retry.
Task writes are the focused WindowCaptureChecks regression and this receipt.

## Evidence and results

- Reported value `1795545649.1296577` retains binary64 bits
  `4745317872292088912` through JSON and CLI parsing. Its immediate binary64
  neighbors remain distinct. Six fixed values now exercise capture and discovery
  parsers through JSON → typed Double → round-trip string → argv.
- Current Rust dispatch passes argv unchanged to the matching helper. Production
  Swift source remains unchanged. Twelve own F02 process incarnations completed
  captures across source/debug and release-optimized diagnostic helpers. Four
  process reads per capture agreed, including after encoding and before stdout.
  The installed 0.8.0 CLI also passed on the owned fixture.
- Fifteen successful local captures have equal decoded stdout/sidecar objects and
  matching PNG SHA256, byte count and IHDR dimensions. Diagnostic helpers differed
  only by temporary identity stderr reporting; they are not release artifacts.
- Installed CLI rejects stale expected launch before capture (exit4, empty stdout,
  no directory). An occupied output directory refuses2 and preserves both existing
  PNG and metadata sentinels. Existing symlink/parent substitution guards pass.
- Terminating the owned fixture immediately after PNG creation gives exit4,
  empty stdout, retained PNG and no capture.json. A separate test-only helper
  injects owned-fixture termination after the real sidecar write; final validation
  again refuses4 and removes its sidecar without advertising success.
- Final optimized Swift boundary checks:39 passed. Existing external directory
  scope/symlink/permissions checks and six helper command negatives passed.
- An initial test caller used NSNumber.stringValue, which rounds this value to
  `1795545649.129658`. That is not the product argv path. The test now uses a typed
  Double and its round-trip string, matching the real Python caller semantics;
  the failed test is not evidence for the reported post-capture failure.

Reporter independently repeated on the original PID76997/window13212 without a
restart: all four diagnostic readings matched the original launch bits. A later
explicit installed0.8.0 capture succeeded with PNG2880×1968; the reporter checked
the complete JSON response, process/window/request identity, SHA256, bytes and
IHDR. The original failed PNG remains excluded from accepted evidence.

## Handoff and limits

Use the ordinary installed command with freshly selected PID/window/bundle and
the exact numeric launch value from --list-windows, plus a new output directory.
Accept only exit0, a complete stdout JSON response, matching capture.json and a
PNG matching its recorded hash, bytes and dimensions. Existence of PNG alone is
not acceptance. A new comparable refusal needs its own bounded investigation;
do not add a timestamp epsilon or hide the failure through automatic retry.

Local evidence owner directory:
`/Volumes/Develop-2TB/Develop/tmp/uib-capture-identity-d9po6_at`.
Final/reference images and summaries remain for handoff; compiler cache,
temporary diagnostic source/binaries and runtime fixture are disposable after
consumers exit. Reporter evidence remains under its owner's retention policy.
No source release is necessary for this test-only checkpoint. The builds owner
receives the revision and reporter qualification, with original cause explicitly
unresolved. No claim of atomic GPU state, broader platform coverage or repaired
transient process lookup is made.
