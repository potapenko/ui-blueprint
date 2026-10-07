# L01 guarded live CLI — work in progress

Authority: [finite packet](../packets/L01-live-observe.md), approved P2/P6/PLAN.UIB@1.
Existing host composition is ready; the missing product caller is CLI observe.
Basis registry12/CLI@2, D02@2/D04@1/D05@4/MEMORY@2/WORK@1/D01@1/D03@2/D07@5
and full EXCHANGE/MODEL/IDENTITY/PRIVACY/LIFECYCLE/BOUNDARIES/NATIVE closure,
RUST/DEV.RUST. No new external dependency version, discovery, daemon or graph.

## Actual shared dependency selected before edits

Root authorized direct pinned workspace serde derive in CLI and minimal host
outcome metadata; delivery of canonical Failed must not imply observation success.
Exact shared source writes: crates/host/src/{publication,host_types,
worker_observation,worker_native,worker_main}.rs. Focused existing support:
tests/publication.rs, tests/support/native_host.rs and publication_probe.rs under
crates/host (last path only adapts its actual publish call signature).
Web owns worker_web.rs; its required callsite is separately returned to root.

Chosen API: HostCompletion::incomplete_channels()->u8, subset of committed mask.
Existing guarded typed receive computes one bool for canonical Failed or observed
Snapshot declared partial/unknown coverage (or nonzero omitted/unknown count).
No graph decode in parent. Frame/Commit/ACK use existing flag bit0, with exact
matching validation. Publication and CommittedFrames each add one inline u8;
parent runtime's existing size_of inventory charges any layout padding increase.
No pool/cap/Control64/payload copy change. Metadata becomes durable only after ACK,
survives later failure with prior canonical bytes, and is absent for uncommitted data.
Worker publish gets a final incomplete bool; non-observe uses false. Existing
live Native receive passes its validated result; Web must make the same callsite
change. This is a private paired worker/host update, not canonical wire migration.

Planned CLI set: existing arguments/input/output/main plus observe.rs in existing
src; tests/binary.rs; member Cargo.toml/root Cargo.lock for pinned local dependency
edges/features only; CLI development doc, three existing CLI spec routes and this
receipt. No new directories. Exact results/checkpoint will be appended after work.

## Implemented caller and registration

Before CLI edits, registered CLI@3 OBSERVE/registry13 in the same three existing
routes. Concrete syntax/strict connection1.0.0 fields, HostLimits and exit mapping
are normative there; [CLI guide](../../../development/cli.md) supplies build/use.
Features macos/web are explicit and defaults empty. Optional direct workspace
serde reuses pinned1.0.229/derive; optional host path dependency and its web feature
reuse existing packages. Cargo.lock changes only CLI dependency edges, no versions.
input.rs was not needed; existing bounded regular-file reader is reused.

CLI validates operator connection and canonical Request binding before dispatch,
uses exact worker/helper paths, applies the real Attached clock only to its owned
request, then composes attach/configure/submit/complete/shutdown. Native fixture
scope and established Web binding remain honest limitations. No graph from UI can
grant target/executable authority. Parent only reads trusted setup/Request; output
canonical frames remain opaque. Configuration/schema errors are sanitized.

Existing output::Bounded encodes trusted inputs before filling charged leases.
Request/frame/total limits reserve line-ending bytes before dispatch. Each returned
frame is streamed unchanged plus newline from completion leases. A failed operation
still publishes its earlier committed records; no empty replacement or combined
Snapshot. Cleanup runs before streaming and its failure overrides with1, while
leases retain ACKed bytes. A completion returned during emergency shutdown is also
preserved. Host deadlines govern active work; caller allows only existing cleanup
duration to collect the resulting terminal event, without extending collection.

Host metadata shared files stayed as declared above. Added actual test-only
native_peer.rs modes for partial coverage and delayed capture; no production fault
mode. Web independently saved its receive-bool→publish argument in e44f396,
SHA2562a49e5d3900525c3d6ab0734445937a45880cf49a671c4ebae4e97a9e3bd2372.
It is consumed by builds, not included in Core's checkpoint. Existing private
publish call in publication_probe.rs passes false for its unchanged non-live case.

## Affected evidence

On Rust1.96/aarch64-apple-darwin, Cargo --locked --offline:

- Host default lib/session-worker check: passed.
- CLI default, macos, web and combined macos,web bin checks (combined through
  Clippy): passed; no Swift runtime/build path. Default commands remain local.
- Host publication exact outcome_flags_must_match_through_ack_and_survive_later_failure:
  passed. Wrong commit/ACK flags reject; only fully ACKed incomplete bit survives
  later uncommitted data; non-Observe and unknown flag bits refuse.
- Existing exact host native_requests_are_collected_after_real_begin_and_ax_ack_precedes_capture:
  passed with added assertions: canonical Failed capture contributes incomplete2;
  successful AX alone remains complete after capture transport failure.
- Existing host worker/native_peer built; CLI binary exact
  observe_cli_uses_real_guarded_peer_and_preserves_failed_partial_and_timed_out_channels:
  passed. Five actual owned worker/helper scenarios: complete AX exit0/one line;
  delivered Failed capture exit4/two lines; declared partial AX exit4/one line;
  capture EOF exit1/prior AX retained; capture deadline exit4/prior AX retained.
  Every NDJSON line passes canonical decoding and byte equality with the peer's
  original compact serializer. Original Request file unchanged. Tests additionally
  prove invalid config/version/identity/unknown/duplicate/array before executable
  dispatch, missing worker1, output admission2/no stdout, unsupported backend5 and
  disabled Web backend5. Successful return requires actual host ShutdownComplete.
- Existing CLI finite_command_and_argument_contract default test: passed; observe
  is now an implemented command, so its malformed arguments give2 rather than5.
- CLI combined bin/binary-test Clippy and host web lib/bin/publication-probe/
  publication-test Clippy: passed with -D warnings. The large Web setup configuration
  is boxed in the CLI typed setup, not copied; no publication allocation added.

One concrete initial Web peer mismatch: exact host test
real_begin_permit_ack_and_reusable_first_and_reference_requests failed at existing
web_worker_peer.rs:98 (expected throwOnSideEffect=true, actualfalse), followed by
TimedOut. This occurred before publication; Web owner was notified. Root then
granted exactly web_worker_peer.rs's per-function expectation: READ_NODE false,
selection/continuity true, matching accepted d19d130. Only that assertion changed;
the exact previously failed case was rerun and passed. No collector/source behavior
change or unchanged broad retry. Real browser/Swift
acquisition through the CLI was not launched in this source packet; prepared caller
handoff is ready for platform owners after save. Existing old immutable runtime
proofs are not retroactively claimed to test this CLI.

Initial new partial test placed its mode branch in the wrong existing peer branch;
moved it to actual AX construction, then reran the affected test. Native CLI tests
were rerun only after subsequent cleanup/preflight changes or added negative cases.
No full suite, new audit, independent acceptance or model delegation. Existing
test Case owns its system-temp directory, deletes it on Drop and verifies absence;
no persistent result/evidence directories or real app/browser processes created.

Final actual write set (excluding concurrent Web/Native/root edits): Cargo.lock,
crates/cli/Cargo.toml, CLI src arguments/output/main/observe.rs, CLI tests/binary.rs;
host src publication/host_types/worker_observation/worker_native/worker_main.rs;
host tests publication.rs and support native_host/native_peer/publication_probe/web_worker_peer.rs;
docs/development/cli.md, specs README/product README/product cli.md, and this receipt.
Core0.1/analysis0.2 and inspect JSON1.0.0 stay unchanged. Full live release-path and
arbitrary target/distribution qualification remain outside this source handoff.

Checkpoint21 input digest (all exact-set files except this receipt; SHA256 of
compact sorted JSON path→SHA256):
bbfbf407bbc7843eb85abea29d4f0107099ac10bf29c5c2826d2197db3316b57.
observe.rs90d878847b884a93efb528cdc698642c4761678709e3d8f9a432a9a419ec45ff;
publication.rs5de5770ce07bdc3e1aa85aa101e1e79acb7136b123bf789a8879a71a40509789;
host_types.rs8f66180d916e2f8c40acc2cdd399891f4c2294661cf210027bd6b9521cdac9a4.
