# W01 transport implementation receipt

Status: `done` (finite author candidate saved); same-reviewer reconciliation and
acceptance remain separate. Source and 17-check result unchanged.
Authority: packet a21b69a, exact-path temporary Integration/manifest grants and
scoped Git grant for intermediate checkpoint. No nested workers or other projects.

## Saved source and final checks

Source/dependency/D07 checkpoint `4106e04a91e141158dc46590778fb5fd2ee300a9` committed
and pushed successfully; origin/master matched, index released. Exactly 14 paths
were saved. D07@4/registry8 metadata preceded source/manifest mutation; runtime
float_roundtrip and prior pins preserved. Both metadata READMEs were released to
Integration; its later registry9/D02/D05 changes are not edited by this worker.

Final tests ran against that saved source/config and passed:

- Package all-targets check (Rust 1.96.0).
- `cargo +1.96.0 clippy --locked --offline -p uiblueprint-web --all-targets -- -D warnings`.
- `cargo +1.96.0 test --locked --offline -p uiblueprint-web`: 17 tests, 6 IO-stub and 11
  loopback/logger tests. The previously pending handshake-zero-write case passed.
- `cargo +1.96.0 fmt -p uiblueprint-web -- --check`.

Environment macOS 27.0.1/26A434, aarch64-apple-darwin, rustc ac68faa. No code changes
after final pass; no unchanged/Core/workspace suites repeated for documentation.
Tested input fingerprint `91f06b85481a306ddb5cee26e5010fe0ac93d4cf35ea36814793fee91c51e070`
uses the ordered saved path+NUL+blob algorithm specified in the
[final API/dependency/proof handoff](../../../development/web-transport.md).

Cargo.toml SHA256 `a87adec8d6b011a88b1ccbb40191eeae0dc2964b52bc50e01a1d9b2a073a36c2`;
Cargo.lock SHA256 `a3f321614b4551712fa44ee402f697f159dc43d0a613fb85e1e8ecae371d6173`.
These inputs stayed frozen through final checks. Manifest/build lane was explicitly
released afterward for Core's concrete host member. Later host wiring need not
repeat unchanged transport checks; any actual affected feature/source delta must
be reconciled by its owner. No Web manifest mutation after release.

## Actual API, boundaries and dependencies

Concrete uiblueprint-web transport, single-use Connecting/cancel handle, one mutable
operation/IO owner, finite endpoint/handshake/frame/message/write/operation budgets,
absolute per-IO deadline, queued partial flush without duplicate send, bounded close
and owned shutdown/drop. Numeric-loopback ws only; no DNS/TLS/redirect/proxy/browser
or periodic collector. Public send progress never means CDP/business success.

Direct log 0.4.29 is the exact facade needed by tungstenite. Installation filters
upstream targets before host delegation, retains non-upstream logging and refuses
an existing foreign logger without replacement. Connection refuses without this
boundary. Real capture/canary/foreign-logger child tests passed. Future CLI/worker
must compose this filter at startup; no CLI/other logger source changed here.

Twenty registry packages plus the concrete member were added; every pre-existing
package/version/checksum remained. Exact feature tree/version/MSRV/license/notice
inventory is in the handoff. tungstenite 0.30.0 handshake only, no TLS/url/default;
log no features. New dependency license texts and shared graph notices were read,
including target-only r-efi AUTHORS and unicode-ident Unicode license. No vendored
source or raw registry/download evidence committed. Root handles final distribution.

Late normative revalidation: D02@2/D05@3/D05-MEMORY@2/D05-WORK@1 at f9ff423 fully
read after registration. This library supplies IO for the future guarded worker;
it does not implement/bypass worker allocation/publication/TargetLease enforcement.
No D02/D05 policy or foreign schema/engine/CLI/export/native source was modified.

## Proof and honest limits

Proof covers successful text/control, invalid endpoint/config/header/extension,
frame/fragment/outbound caps, byte/work/read-ahead budgets, slow drip, zero write,
short writes/WouldBlock/pong retry without duplicate application send, pending-drop,
cancel before/during handshake and pending read, independent peer progress, close
without server EOF, redacted errors/Debug/logs and actual owned resource cleanup.
All tests used only owned ephemeral loopback peers or IO stubs. Product transport
spawns no threads/processes. Peer helpers and isolated logging child have finite
completion/cleanup waits; all own threads/children/listeners/sockets were joined/
reaped/closed in the recorded run. Raw failing setup output was not retained.

TCP connect cancellation is observed after std connect_timeout returns (no exposed
in-progress FD). Work counts IO/protocol calls; internal codec work is additionally
bounded by admitted bytes/frame/message/read-ahead, not a hard realtime guarantee.
Payload limits do not cap allocator/RSS/SDK/kernel memory. No browser/Chromium,
W01/B01–B06, D05 live or release acceptance claimed. Independent reviewer source
observations/reconciliation remain root-owned; author's tests are not that review.

## Final checkpoint and cleanup

Final docs checkpoint contains only docs/development/web-transport.md and this
receipt. Root granted exactly those paths; master and empty index confirmed.
Exact SHA/push/release is returned in terminal handoff for reviewer reconciliation.
They record existing API/proof; source remains 4106e04 and checks were not repeated.
Both READMEs/Cargo are released and untouched. Retain task-temp until explicit
same-reviewer acceptance/cleanup handoff, then remove only named own directory
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-transport-impl-deislzp9/`.
Exact source/lock/checksums and dependency paths suffice for reproduction; no raw
command logs/downloads/builds need permanent storage. Stop at this packet.
