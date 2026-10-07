# Expose existing guarded observation through the product CLI

Class shipping_product; retained Core owner01a111a7-9887-7983-9aa0-c08dfa2d46bc,
inherit, no nested agents, master. Approved P2/P6 and CLI.CONTENT require usable
live observe; current observed gap is that only test callers compose the ready
host APIs. No new service, engine, browser/SDK discovery or background process.

Basis: current registry12/CLI@2 plus D02@2/D04@1/D05@4/MEMORY@2/WORK@1/D01@1/
D03@2/D07@5, EXCHANGE/MODEL/IDENTITY/PRIVACY/LIFECYCLE/BOUNDARIES/NATIVE and
explicit closure already read. RUST/DEV.RUST; before implementation read only
missing required details. Protect inspect/measure/check/export and all canonical
versions. Worker/Native resource caps and ownership do not change.

Concrete caller choices under ROADMAP delegated packaging/CLI authority:
- explicit operator-owned connection file, canonical Request file, worker executable
  path and aggregate input/output bounds; no secrets in argv or discovered targets;
- connection contains versioned local CLI configuration for exact existing
  SessionDescriptor, TargetLease identity, WebSetup/selection or Native helper
  executable/opaque config/channel mask. Use existing canonical types where defined;
  do not serialize private Rust memory layout or invent another graph;
- the connection is an explicit trusted operator capability, never read from UI,
  response payload or embedded reference. Its paths/endpoints must satisfy current
  host validation; reject unknown/duplicate fields, invalid versions and mismatches;
- after attach, bind request to actual Attached clock; preserve original target/
  scope/fields/channels/limits. Caller chooses an explicit attach deadline and
  existing valid HostLimits profile at/below D05; no hidden infinite waits;
- explicit --worker path until I01 installation chooses distribution placement.
  Native helper path is explicit in connection. No CARGO_BIN_EXE/test binary,
  automatic installation/permissions or implicit app/browser launch;
- output one unchanged committed canonical ChannelResponse per line (NDJSON),
  only at complete-frame boundary. Preserve prior ACKed channels on later failure;
  no invented combined Snapshot or parent graph parse;
- channel output plus newline fits caller's total cap; smaller remaining cap
  refuses before partial line publication. Setup/no-commit errors leave stdout empty.
  IO failure may leave partial physical writes and retains IO failure semantics;
- hold completions through publication; always shutdown/reap owned workers/helpers
  and report CleanupPending/quarantine truthfully. Never close user app/browser.

Register the exact finite command, connection fields/version, NDJSON behavior and
exit mapping in existing cli.md contract/routes BEFORE source edits in this same
task. Use existing exit classes0 success,1 IO/worker/cleanup failure,2 invalid/limit,
4 partial/unavailable observation,5 unsupported mode; document actual HostError
mapping explicitly, unknown-effect path cannot arise from read-only Observe.
No flags/settings beyond the concrete required inputs above. Root chose connection
and streaming boundaries; routine typed field spelling is implementation detail.

Implement in existing crates/cli/src/{arguments,input,output,main}.rs and a small
new observe.rs in that same existing directory if needed; existing binary tests
and support; crates/cli/Cargo.toml/root Cargo.lock only for local host dependency/
web feature wiring; docs/development/cli.md; existing CLI/root/product spec routes;
receipts/L01-live-observe.md. Declare exact subset. No host source/schema/engine/
Native/Web collector source, external deps, root manifest version changes or new
directories. Return concrete missing host API before editing its separate owner.

Native current support remains trusted own-fixture binding, not arbitrary PID/title
selection. Web connection requires established exact target/document; no arbitrary
site bootstrap. Core-only CLI commands must not launch workers; web-only build must
not require Swift. Runtime later uses already authorized owned fixtures only.

Focused binary checks via existing finite peers: actual attach/request/ACK output,
partial AX preservation, invalid config before dispatch, missing executable,
timeouts/cleanup and no partial line on admission failure. Test required chosen
feature combinations only, no broad suite/new framework. Prepare exact live CLI
handoff; do not launch real apps in this source task. Current-operation temp only,
remove/verify; no permanent outputs. Checkpoint/push each coherent piece under
short Git lease; no extra approval cycle between contract registration and code.

Actual caller dependencies: CLI may add direct workspace serde/derive using the
existing pinned version/features, member manifest and necessary local lock entry;
no new package/version or dependency update. Host Completed currently includes
delivered canonical Failed, so delivery alone cannot justify CLI success0. Core
may add only the minimal fixed precharged outcome metadata at existing worker
typed validation/publication and completion owners; record exact files/layout/API
before editing. No parent graph/JSON parse, second payload copy, larger cap/pool or
generic status framework. Canonical bytes, Commit/ACK and prior responses remain.
Web-owned web_config.rs/worker_web.rs stay protected; return an actual required
consumer-callsite change to that owner. Focused success/Failed/partial/cleanup
cases verify the new CLI exit distinction without another broad host audit.

Core returned exact metadata owners: publication.rs, host_types.rs,
worker_observation.rs, worker_native.rs, worker_main.rs, affected existing publication/
native tests and publication_probe.rs compile callsite. HostCompletion exposes
incomplete_channels for ACKed records only. Worker derives the flag from validated
Failed or partial/unknown coverage and transmits it in existing flags storage;
fixed parent inline metadata is charged, with no new pool/allocation. Frame/Commit/
ACK must agree and preserve correlation. Web owner alone performs the one
worker_web.rs receive-result→publish argument change; older immutable Web runtime
proof does not depend on this unpublished change. No unrelated host source opens.
