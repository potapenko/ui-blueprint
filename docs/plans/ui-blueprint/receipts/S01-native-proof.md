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

## M05-COMPOSITION-001 — request-aware shared admission candidate

Finite Integration packet appended to S01-native-proof, root af24116, approved
P3/M05/PLAN.UIB@1. The earlier Native-only source exclusions are superseded solely
for this assigned shared change. Current master, no nested agents, real UI/capture,
new framework, graph, schema shape/version, dependencies or broad audit.

Basis before code: retained full D03/EXCHANGE/MODEL/IDENTITY/PROJECTIONS/GEOMETRY/
NATIVE@2/PRIVACY and native M05/PILOTS/FORM/CACHE/ACTIONS/GOLDEN closure; RUST/
DEV.RUST and current implementation/QA/operational governance. Read current packet,
actual schema/session/worker path, Native candidate0114fa1 and legacy tests.
The schema all(o.channel==wrapper) restriction was implementation-only beyond
source-composition norms; removing it alone would bypass nested-channel authority.
Read-only proposal accepted by root. Registered EXCHANGE@2.COMPOSITION and D03@3
plus decision README BEFORE code; Core owns spec-root/product route integration.
No authority is invented from candidate source or a passing synthetic test.

Exact10 paths: crates/schema/src/validation.rs; crates/plugin-api/src/lib.rs;
crates/schema/tests/channel_response.rs; crates/plugin-api/tests/lifecycle.rs;
crates/host/tests/support/native_host.rs and native_peer.rs;
docs/specs/product/exchange.md; docs/specs/development/decisions/d03-data.md and
README.md; this receipt. Packet's short native_host path was reconciled to its
existing support location. Core G12, Web B04, Swift/native collector, old fixtures,
root routes/coordination, JSON schema/types, Cargo and parent/host production untouched.

Schema preserves homogeneous responses and all full Snapshot/session/target checks.
Only added mixed case: wrapper OptInLayoutProbe, projection Design, observations
include both ExternalSemantics and OptInLayoutProbe and no other channel.
Missing own-wrapper, AX wrapper for mixed sources, wrong projection or capture/
third channel reject. Existing shape/strict field decoding/version negotiation
and standalone Snapshot semantics stay unchanged; no relabelling of AX as probe.

ObservationSession::check_response now verifies every nested source channel (and
wrapper) against BOTH Pending.requested and the session's observe Supported/Partial
capability. It runs before retained insertion/byte accounting publication and before
actual Native receive_channel returns to exchange.publish. Wrapper permission alone
cannot admit AX/probe data; unsupported/permission-required/missing/wrong-operation
capability or unrequested nested source refuses. Full context, node/depth/bytes,
freshness and correlation checks remain. Only wrapper slot is recorded, so probe
composition cannot complete AX or hide a failed raw AX response. Failed-channel
behavior and prior ACKed bytes remain intact. No host protocol/header/slot changes.

### Focused author proof

Rust1.96.0, locked/offline, system-temp target
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-m05-admission-9hyncslo`:
- schema channel_response:2 tests passed; allowed pair/raw homogeneous, projection/
  wrapper/third-channel negatives, source binding/evidence/privacy refusal.
- schema goldens:4 tests passed, including all126 existing independent oracle cases,
  structural/semantic parity, round-trip and unchanged published core schema.
- plugin-api lifecycle:11 tests passed, including3 new composite groups covering
  unrequested/missing/Unsupported/PermissionRequired/wrong-operation capabilities,
  Supported/Partial positives, source partial and separate missing AX slot,
  freshness/node/byte/context refusal and preservation of preceding accepted AX.
- production session-worker + native_peer/process_peer built; runtime selector
  native::probe_channel_uses_existing_two_helpers_without_capture_and_keeps_ax_on_failure
  passed9 matrix cases. Existing5 raw/helper cases retained;4 added composition
  cases cover successful AX+probe ACK, unrequested nested AX, capability-denied
  AX after an ACKed failed AX response, and wrong composite context after ACKed
  successful AX. Refused probe slot has no bytes/ACK; no capture/third helper pool;
  parent-owned bytes stable, shutdown/reap confirmed and all domain charges released.
  This is the real parent/production worker path with a finite synthetic producer,
  not an SDK/Native UI collection or live M05 acceptance.
- affected schema/plugin-api lib/tests and host lib/worker/native_peer/runtime
  Clippy -D warnings passed; affected three-package lib cargo check passed.
- Exact six Rust paths rustfmt check and scoped whitespace/local links passed.
No unrelated full workspace, Web/Native live suite, schema regeneration command,
image or measurement-tolerance change. Test commands use the single named temp
--target-dir; other worktree outputs are not task evidence.

Final inspected own6-source/test map SHA256:
`68c97fbb3d36ada056af0477816043a0ab55860a99a1684d7b3b286c5b02b614`.
Production validation.rs SHA256
`f4cda35449b8063051ae96f0eadb2a2d743e57ddce11ef48bbdb1f6dc32f3c28`;
plugin-api/lib.rs `668b86bca8bff8f6d073972aba1e77efcc3760e1ecc3ccdb8f4bb8646b059909`.
Final inspected relevant122-input Rust map
`55f7559afe8990bd4fb353b16e672009fae998e9819413cb70acde59318859b3`:
root Cargo.toml/Cargo.lock/rust-toolchain.toml plus every .rs/Cargo.toml under
schema/plugin-api/engine/host; sorted compact JSON path→SHA256 then SHA256.
This is a final source inspection pin, not an asserted before/after or saved-state
barrier: Core G12 engine WIP was present independently and is not accepted by this
proof. Root coordinates matching saved-source reconciliation; source/checkpoint
identity and push return in the terminal handoff. No other owner's WIP is staged.

Source-ready for exact10-path Git grant, grouped permission-boundary review and
Native's subsequent explicit saved Observe→G11 consumer. Root/product route updates
belong to Core and must be integrated before overall acceptance. Raw single-channel
behavior remains; no old126 fixture/oracle or schema file was changed. Author tests
do not establish independent review or real composite Native collection.

Consumed task-owned nonimage build target named above was checked for images and
removed after scoped proof completed; absence verified. Repository PNG and all
other owners/source/evidence were left untouched. No resources remain held.
