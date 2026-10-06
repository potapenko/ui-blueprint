# M01 capture prerequisite repair receipt

- packet / status: `M01-capture-repair / done` (finite supporting repair/proof candidate).
  Finite repair/proof is ready; positive B capture remains permission-required
  waiting_evidence. No full M01/D05/P7 or shipping-adapter acceptance claimed.
- authority: approved PLAN.UIB@1, [finite packet](../packets/M01-capture-repair.md)
  at3a98c32, D01/D02/D05-NATIVE repair obligation, sole own-F02 native lane grant.
- Spec Basis: AGENTS → specs/README registry5 → decision D01/D02@1, D05@2,
  D05-MEMORY@1 (METRIC/LIMITS/OWNERSHIP/ADMISSION/LIFETIME/GATES), full explicit
  NATIVE-PILOTS/PILOTS/NATIVE/FORMS/CACHE/ACTIONS/EXCHANGE/GEOMETRY/PROJECTIONS/
  LIFECYCLE/PRIVACY/MODEL/IDENTITY/BOUNDARIES CONTENT@1 closure reused where current.
  QA, operational safety, Apple/Computer Use and prior macOS build/debug routes
  retained. D05 retained policy read, not implemented here. Mode Restore.
- inputs: F02 `9a88b12`, native proof `88ac606`, larger sample `51c7809`.
  Shared support/validator pinned to saved `73d772e97efcf550ea4a4d3e8480b56509ebc548`;
  no build against Integration's changing Rust source.
- exact_changed_paths: `fixtures/native/Observe.swift`, `fixtures/native/capture.py`,
  `fixtures/native/README.md`, `tests/bridges/native/Collector.swift`,
  `tests/bridges/native/capture_lifecycle.py`, `tests/bridges/native/README.md`,
  `docs/development/interface-proof-native.md`, this receipt.
  No Fixture.swift/UI/shared Rust/spec/other-project changes.
- checkpoint / push: commit containing this receipt on master; exact SHA and
  origin/master push result returned in terminal handoff. Root granted only the
  eight listed paths after the passing offline compatibility check. Branch/index
  preflight confirmed master and empty index; Git lease released after push.

## Established failure and repair

The old simultaneous one-shot helpers reproduced the recorded fault on owned A/B:
both emitted continuation misuse, no independent AX file, parent timeout3003ms,
both killed/reaped. Historical45s evidence was preserved, not rerun as a long wait.
Source cause under our ownership: imported async SDK awaits had no bounded reply
owner and the combined writer withheld completed AX while awaiting capture.

Explicit public callbacks plus first-result gate removed the continuation misuse
and retained AX, but both screenshot callbacks still timed out in2s (not injected).
That localizes observed non-completion to screenshot acquisition; no claim is made
about private SDK internals. The final repair adds capture-only flock admission,
bounded callbacks, terminal/late-result ownership, independently persisted/flushed
AX and owned helper retirement. No global lock spans AX or unrelated Rust work.
No different backend, SDK/framework, user UI or permission policy was introduced.

On uncertain timeout/cancel/error, the capture FD remains owned until one-shot
helper exit/reap; replacement capture cannot enter while that owned OS call may
still exist. On confirmed success it releases normally. Parent supplies one shared
absolute task-temp resource path. Unknown fault modes refuse before platform access.
Tests use explicit fault modes only; no periodic UI acquisition or monitoring.

## OS-observed versus injected evidence

| Case | Actual observation | Outcome / limitation |
| --- | --- | --- |
| Legacy concurrent | both real SDK requests | continuation misuse; no AX persisted; parent killed both at3s |
| Callback-only concurrent | both real SDK requests | no misuse; AX preserved; both timeout at screenshot stage |
| Serialized concurrent requests | real A/B requests, capture-only admission | A successful1100×1022 isolated Window A image, exit0; B API error-3801, exit2; AX preserved for both |
| Unrelated AX while capture stalls | current AX A/B; injected capture stall A | B completes143ms while A holds capture lease; AX A survives2s timeout; lease available after both reap |
| Common capture timeout/failure | current AX A; injected capture fault | canonical failed channel plus exact retained AX; common host accepts/retains both outcomes |
| Common cancel/detach | current AX A; injected pending capture/control | actual common transitions retain AX; correctly correlated late AX replay rejected; owned helper reaped |
| Reply-gate checks | injected first/late terminal results | complete/timeout/cancel/detach first result wins; late reply refused; recorded permission code and invalid fault classification checked |
| F02 capture.py failure | current AX A; injected capture failure | exit2 with preserved AX-file reference and labelled capture outcome, no false success |

Successful A PNG visibly contains Window A, attributed to its exact own window;
filter kind window_isolated, audio/children off, actual metadata and unknown
transforms preserved. The positive path used observe-serialized; unchanged success
logic was not rerun after denial. Final code changes classify/audit failures and
validate injected modes; affected final fault paths were compiled and verified.
No claim that true simultaneous screenshot calls now succeed.

B returned `com.apple.ScreenCaptureKit.SCStreamErrorDomain / -3801`, named
UserDeclined by SDK27 SCError.h, after the capture path's preflight passed.
This is an operation-level permission_required residual, not evidence the human
clicked a refusal or that the whole platform lacks capture capability. No B pixel
retry or further real pixel call after that result; no backend switch, permission
prompt action/settings/display change. System prompt presence was not observed;
only the own fixture was inspected. Final classification maps the recorded code
to permission_required and explicit_permission recovery.

## Bounds, checks and cleanup

Before attempts: AX1s; capture including admission2s; parent3s; cleanup1s.
Legacy/repaired diagnostic helper backstop4s; canonical collector existing backstop8s.
Fault-driver30s overall guard. A2.2s reader wait permits bounded delivery around
the2s callback deadline, without changing the actual common parent3s deadline.
These are scenario parameters, not D05 retained/working-memory or RSS defaults.

Swift6.4/SDK27/macOS27.0.1 arm64; focused standalone and CAPTURE_LIBRARY bridge
builds passed without warnings. Final gate checks and the Python fault/isolation
proof passed. Four actual common-host fault cases retained canonical documents
structurally equal to submitted data; platform readers joined after owned producer
reap. The common wrapper's stdin reader ends with its process, per pinned README.
Changed Python syntax, local links and whitespace checks passed; fixture UI inputs
remain unchanged and index was empty before scoped staging. No generic Rust suite, geometry/probe baseline
or unrelated tests were rerun.

Every request result records target identity, channel outcome and cleanup. Only
our Popen helpers were terminated/reaped. The test-created F02 process was stopped
separately after verifying its exact executable/PID, and exit was confirmed.
Capture lease reacquisition succeeded. CUA after proof showed unchanged own UI and
Open B focus. Native desktop lane **released**; no other app/Simulator was operated.

## Offline saved-document compatibility

Validator `9ca645aab6816425ee64bb20eeb1201750d0d2b0` was built locked/offline
from its immutable Git archive in task-temp. All20 saved canonical final-fault-proof
Session/Request/submitted ChannelResponse/retained documents passed (exit0).
`compatibility-9ca645a.json` records exact paths, input hashes, validator binary hash
and per-document results. No failed case, full Rust suite, new SDK/pixel/app call
or shared-source modification. This is **offline compatibility**, not a live
recheck or independent acceptance of capture behavior.

## Retention and remaining dependency

Evidence root:
`/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/P1/M01-capture/485fccb4-c772-4eec-9444-287697b7cb43`

bounds-before-run.json, legacy-concurrent/, callback-concurrent/,
serialized-concurrent/, final-fault-proof/, capture-driver-failure/ preserve the
minimum requests/results/AX/canonical retained records and one necessary own A
capture. provenance.json/cleanup.json and final-provenance.json pin build/source
identities and changes; raw stderr/build logs stay only in task-temp
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-m01-capture-aaay1hjv`.
Owner root; consumer M01/P7; retain through acceptance or explicit discard.
Old F02 failed evidence and other owners' files were not deleted or rewritten.

Next: root independent review of the saved candidate. Positive B capture needs its exact
permission condition resolved through a separately authorized path. A live/common
recheck remains separate from the passing20-document offline check on saved9ca645a;
no mutable neighbor build was consumed. Full adapter, general capture concurrency, calibrated
transforms, probe invariance and integrated M01/D05/P7 acceptance remain open.
