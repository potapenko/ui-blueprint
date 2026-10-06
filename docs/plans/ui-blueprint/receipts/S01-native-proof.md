# S01-Native proof receipt

- packet_id / status: `S01-Native / done` (finite native proof; root acceptance remains separate).
- outcome / classification: finite native D02 verification with narrow test tooling.
  Live F02 AX/capture passed the common validator and actual common lifecycle;
  completed AX survived separately labelled injected timeout/cancel/detach.
  No production adapter, independent product acceptance or P1 freeze claimed.
- authority: PLAN.UIB@1 and explicit coordinated S01 bridge packet; root's renewed
  own-F02 runtime grant after common support dispatch. No nested delegation.
- Spec Basis: AGENTS → UIB.ROUTING@1 registry4 → UIB.DECISIONS@1 → D02/D03/D05@1
  and full explicit closure; current F02 CONTENT@1 retained. Missing GOLDEN/
  RUST-BOUNDARIES/D01/C01-EVIDENCE/D04/D06/C01-HANDOFF@1, RUST.md, DEV.RUST@2,
  D07@2 read. Details and the legacy DEV.RUST link-label note are in the
  [handoff/report](../../../development/interface-proof-native.md).
- pinned inputs: F02 `9a88b12b5855bac54bf04ba7b64d233df719ddec`;
  Stage A `9d2df153abd2a7d7567100e06d4260e5edda3bb3`;
  common support `73d772e97efcf550ea4a4d3e8480b56509ebc548`.
- prior checkpoints: preparation `27c413f6c4f4ee862805443add00920946e8555a` and
  collector WIP `3bdf34d48bd888c91e564ee82db8dda5143b48b1`, both pushed origin/master.
- final changed paths: `tests/bridges/native/Collector.swift`,
  `tests/bridges/native/build_support.py`, `tests/bridges/native/prove.py`,
  `tests/bridges/native/README.md`, `docs/development/interface-proof-native.md`,
  this receipt. prepare.py unchanged from its saved checkpoint.
- final checkpoint / push: commit containing this receipt on master, with exact
  SHA and origin/master push result in the terminal handoff. Root granted only
  the six paths above; preflight confirmed master and an empty index. Git lease
  released after the successful push. No runtime or unchanged check was repeated.

## Runtime proof and evidence distinction

Explicit setup only: prepared own F02-on bundle → observed Window B → Snapshot.
Actual bindings came from that fresh receipt: PID/launch incarnation, window3898,
fixture surface generation. No reused title/coordinate identity and no other app.
AX selected the unique sample within that exact fixture window; selected fields
role/accessibility_name/enabled/accessibility_bounds are observed, with raw AXButton
and pt screen frame preserved. Capture is independently observed, isolated window,
1100×1022 pixels, no audio/children, explicit partial surface coverage and unknown
transform. Actual filter/window metadata and capture-call interval are retained.

The common host created every Ticket using actual ObservationSession::begin and
its parent Instant. Collector used only the returned sequence. All canonical
Request/Session/live ChannelResponse documents passed the committed Rust validator.
Actual core-retained channel documents equal submitted observations structurally.
No second schema, protocol, parent clock or lifecycle implementation was introduced.

| Case | Actual source | Result | Explicit injection |
| --- | --- | --- | --- |
| live | current F02 AX and ScreenCaptureKit | Completed, both channels retained; host/helper exit0 | none |
| capture-timeout | current F02 AX | TimedOut, AX retained exactly, capture missing | no capture call; pending helper killed/reaped after2s; common parent expires at3s; late AX-success replay rejected StaleTicket |
| cancel | current F02 AX | Cancelled, AX retained exactly | control after first channel; helper reaped; late canonical AX replay rejected StaleTicket |
| detach | current F02 AX | Detached, AX retained exactly | control after first channel; helper reaped; late canonical AX replay rejected Detached |

Late frames are labelled replays of that case's valid live AX success, not late
OS callbacks or fabricated capture success. Injected capture timeout does not
repair/prove the historical concurrent ScreenCaptureKit continuation failure124.

## Checks, limits and scope

Host macOS27.0.1 build26A434, arm64, Swift6.4/SDK27; deployment compile target14.0.
Prepared F02 artifacts reused. Collector rebuilt only for changed metadata output;
no Swift warning/error. Common host+validator built locked/offline with Rust1.96.0
from immutable support Git archive; no mutable CLI/other-owner implementation was
consumed. The committed three Rust/eight generic support checks are reused for
wrong version, malformed/oversize frames and generic lifecycle negatives, not rerun.
The native prove.py live four-case run exited0. Changed Python syntax, local links,
whitespace and executed-code hash checks passed; index is empty. No unrelated
suite or review repair.

Literal finite test limits: frame65,536 bytes including newline; pending131,072
encoded bytes;1 in-flight;8 frames;3,000ms common parent deadline;1s AX/2s capture
waits;1s cleanup wait;45s whole-proof watchdog;8s helper last-resort watchdog.
These bound this known one-node proof and its fault cases, not D05 heap/cache/
retention defaults. Parent terminal observations301/3012/122/121ms are trace data,
not D06 evaluation. Shared parent clock and helper source domains stay separate.

Reader queues are capacity1, reads byte-bounded. Only spawned Popen helper/host
children were terminated/reaped; platform Python readers joined after reap before
stream close. The common wrapper's own stdin reader ends with its process exit,
as specified in its README.
All four proof.json files record actual PIDs, exit codes and cleanup. Live children
exited0; fault helpers exited-15 after owned SIGTERM; all hosts exited0. The test-
created fixture was stopped separately after proof, not by detach. CUA reported
no tree change after collection and the same focus. Native lane **released**.

## D05 samples and retention

Evidence root:
`/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/P1/D02/3642d7a8-c2a8-49b9-8d6a-4cdc7fe4b60d`

Minimal actual normalized examples for D05:
`proof/live/retained/channel-0.json` —4597 bytes, one AX node/four requested fields;
`proof/live/retained/channel-1.json` —3246 bytes, attributed capture. They are not
largest-native-graph or parsed-heap calibration. Each case includes canonical
request/session/submitted/retained documents and payload-free host receipts.
Live PNG and metadata establish actual capture. report.json/provenance.json/cleanup.json
pin checks, source/binary hashes, environment identities and owned teardown.
Owner root; consumers D02-PROOF/D05/P1/P7; retain until P1/P7 acceptance or explicit
discard. No raw command logs/captures are in Git or Codex/config/skill directories.
Task-temp remains `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-s01-native-vmy829z0`.

Remaining: root review of the finite proof; checkpoint+push is the saved candidate,
not full S01/P1 acceptance. Native runtime lane remains released.
Four P2 remain awaiting authority; D05 resource policy, real native concurrency,
input/privacy/isolation, current probe invariance and calibrated transforms retain
their named owners. No M01 production work or real PlayPhrase.me collection occurred.
