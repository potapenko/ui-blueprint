# P01 existing host probe-channel connection

Authority: [finite packet](../packets/P01-host-channel.md), approved P01/M05 under
PLAN.UIB@1. Root/Native identified the concrete channel2/bit4 admission gap.
Basis retained: D02@2/D04@1/D05@4/MEMORY@2/WORK@1 and NATIVE/EXCHANGE/
PROJECTIONS/IDENTITY/PRIVACY explicit closure; registry12/CLI@2 does not change
this boundary. RUST/DEV.RUST apply. No new schema/control/quota semantics.

Exact7 paths: crates/host/src/{native_binding,native_broker,supervisor}.rs;
crates/host/tests/support/{native_host,native_peer}.rs; docs/development/host.md;
this receipt. No Web-owned files, Native Swift, WorkerNative, helpers type,
allocator, Cargo, canonical source fixture or configuration changes.

## Concrete API and ownership

Existing NativeHelperBinding::authorized(exec, channels, opaque_config) now admits
mask bit4 as well as1/2; submit_native_observe accepts the same existing mask7.
Broker admits canonical channel2 and routes it through the unchanged Configure/
Submit/HelperReply/Ticket/NDJSON/receive/commit/ACK machinery. Public signatures
remain unchanged. Three previous guards were the actual gap; WorkerNative and
ObservationRun already support all three channels.

Only channel1 chooses HelperKind::Capture;0/2 choose the existing non-capture
ExternalSemantics resource classification. No third helper kind/pool/slot is added.
Parent retains opaque4032-byte config; Native owns probe_manifest_path,
probe_snapshot_request/revision/uptime and real source validation. Provider neither
parses them nor manufactures freshness. Probe request uses existing layout_bounds,
CachedAllowed and canonical OptInLayoutProbe as supplied by its trusted caller.

## Focused actual process proof

Rust1.96/aarch64-apple-darwin; all commands --locked --offline:

- cargo check -p uiblueprint-host --no-default-features --lib --bin session-worker:
  passed; compiling API returned before proof completion.
- cargo build -p uiblueprint-host --no-default-features --example native_peer
  --example process_peer: passed, existing finite non-UI peers only.
- cargo test -p uiblueprint-host --no-default-features --test runtime
  native::probe_channel_uses_existing_two_helpers_without_capture_and_keeps_ax_on_failure
  -- --exact --test-threads=1: passed five cases.
  Probe-only while same-session Capture helper remains registered succeeds with
  channel2 ACK, proving probe does not need capture lease. AX+probe succeeds with
  original AX ACK before the next producer. Probe mismatched Ticket and helper EOF
  fail without committing probe, while prior AX bytes/ACK remain. Two occupied
  helpers refuse the third/probe with zero new native_peer spawn. Invalid masks
  0/8/255 binding and8 submit refuse. Parent owned bytes do not grow.
  Probe peer returns existing canonical known geometry with cached/unverified
  Observation unchanged; it is explicitly synthetic, not a measured SDK claim.
  Every case finishes with sessions0/groups0/abandonedfalse after actual shutdown.
- Existing exact native::native_requests_are_collected_after_real_begin_and_ax_ack_precedes_capture
  regression: passed. cargo clippy -p uiblueprint-host --no-default-features
  --lib --bin session-worker --test runtime --example native_peer -- -D warnings:
  passed. Scoped rustfmt/diff/local links passed.

The forwarding process wrapper records actual ACK writes; it does not fabricate
controls/status. Parent operation2 differs from observation Ticket1, so the negative
correlation case is meaningful. Existing process_peer holds finite registered
resources; owned peers are reaped by normal host shutdown. No browser/Swift/UI or
new temporary/persistent output directory was created. No broad runtime/allocator
suite or unrelated audit. Masks/HelperKind classification do not weaken any cap.

Source/test SHA256:
native_binding.rs6ed613488d954c01289c3c9eb6a9bdfa7f39c1f86067a5314253b3ae60378fe9;
native_broker.rsb2221e701711f641460c9e5fa9a31b5857385e91f401e400789af7c2967cd0d4;
supervisor.rsd8eb2a4b4fca17b4ba5ecc4ccfb1f6b1bb4c798c647cdbcd4347f286654806ca;
native_host.rs0a4a12b0ade759814962434b6be5f3d8dac9075db6f3c9fa426957ec774dac85;
native_peer.rs6f18ee21286337936d95ca762d15d83c107ef4c77c79ef040db1f87f0c5bb87c.

Immediate consumer: Native's saved own-probe source/collector through this compiling
host. Actual manifest binding, fresh measured frames, Swift lifetime, off/missing/
stale marker outcomes and M05 off/on invariance remain Native/P01 live work; this
provider-only result does not claim those gates passed. Checkpoint follows root's
short Git grant, excluding concurrent Web/Native changes.
