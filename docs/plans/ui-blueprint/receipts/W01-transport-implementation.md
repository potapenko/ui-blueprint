# W01 transport implementation receipt

Status: coherent WIP checkpoint; shipping transport candidate, not accepted.
Root granted exactly14 paths for an intermediate source/manifest save; final
focused checks and independent review remain open.
Authority: packet a21b69a, temporary exact-path Integration ownership and explicit
manifest/build grant after Core focused reproduction. D07@4/registry8 registered
before source/dependency mutation; runtime float_roundtrip preserved.

## Cargo barrier released

Initial `cargo +1.96.0 check -p uiblueprint-web --all-targets` passed; concrete
membership compiles. Dependency resolution finished; no further root Cargo/lock
mutation planned during source/tests. Core can resume affected package checks.
Cargo.toml SHA256 `a87adec8d6b011a88b1ccbb40191eeae0dc2964b52bc50e01a1d9b2a073a36c2`;
Cargo.lock SHA256 `a3f321614b4551712fa44ee402f697f159dc43d0a613fb85e1e8ecae371d6173`.
Every prior package/version/checksum is preserved; no existing-version drift.
Added registry packages: block-buffer 0.12.1, bytes 1.12.1, chacha20 0.10.2,
const-oid 0.10.2, cpufeatures 0.3.1, crypto-common 0.2.2, digest 0.11.3,
getrandom 0.4.3, http 1.5.0, httparse 1.10.1, hybrid-array 0.4.15, log 0.4.29,
r-efi 6.0.0 (target-conditional), rand 0.10.3, rand_core 0.10.1, sha1 0.11.0,
thiserror 2.0.21, thiserror-impl 2.0.21, tungstenite 0.30.0, typenum 1.20.1.
New workspace member uiblueprint-web0.1.0. Actual Web feature tree:
tungstenite handshake/data-encoding/http/httparse/sha1; log no features;
no TLS/url/default tungstenite feature or async framework. Existing serde_json
runtime float_roundtrip unchanged. This is not runtime/transport acceptance.

## Coherent intermediate implementation

Actual owned transport crate and operation API compile. Numeric-loopback ws only;
explicit payload/buffer/deadline/byte/work/outbound limits, one IO owner, cancellation
shutdown handle, no DNS/TLS/redirect/reconnect, bounded close. Queued application
writes resume through the same codec flush/deadline; no re-enqueue. Raw peer data
is excluded from Failure/Event Debug and upstream log targets are filtered before
host delegation. Logger installation refuses an existing foreign logger; no takeover.
Caller authorization, CDP correlation, graph collection and D05 working-memory
admission remain separate. Connecting cancel during TCP connect is observed after
std connect_timeout returns; its in-progress FD is not exposed for early shutdown.

Last complete check: package Clippy -D warnings and16 focused tests passed
(5 IO-stub /11 loopback+logger tests). Tests cover text/control, endpoint/extension/
malformed handshake, absolute slow-drip deadline, frame/fragment/outbound bounds,
byte/work limits, short-write/pong backpressure, read-ahead, pending-drop/cancel,
close-without-EOF, independent peer and real filtered logger canaries/refusal.
Subsequent source changes tighten invalid operation-limit admission before allocation
and add a handshake-zero-write unit test. Current all-targets compile is checked for
this checkpoint; that added test and final current-revision Clippy/test checks are
still pending. No claim that previous runtime results certify the final candidate.

D07@4 and registry8 metadata are stable. After this scoped commit+push, release
`docs/specs/README.md` and `docs/specs/development/decisions/README.md` to root/
Integration. D07 leaf, Cargo/plugin source retain Web ownership; manifests remain
frozen at the hashes above. No unrelated schema/engine/Core/native files staged.
Exact checkpoint SHA/push/index release is returned in the handoff. Independent
transport/lifecycle/privacy review follows the saved final result.

Raw source downloads/build output remain in owned task-temp
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-transport-impl-deislzp9/`
for remaining focused work; no raw evidence committed. No browser/UI/live adapter,
RSS/allocator/SDK total cap or completed W01/B01–B06 acceptance claimed.
