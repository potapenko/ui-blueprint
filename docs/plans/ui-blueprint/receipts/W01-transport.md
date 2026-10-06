# W01 transport diagnostic receipt

Status: `done` (finite source-evidence/recommendation checkpoint; adoption separate).
Authority: W01-transport packet e5ed9c0 / approved PLAN.UIB@1 delegated D07 choice.
Consumer: Root/Integration D07 adoption and later shipping W01. No live adapter.

## Basis and outcome

Full current W01 packet closure reused from R01/F01/D02; current D07@2 read.
[Source ledger and integration handoff](../../../research/W01-transport.md) records
traversal, exact source identity, observed behavior and proposed conditions.
Recommendation: adopt tungstenite=0.30.0, default-features=false, handshake only,
as codec for an owned authorized numeric-loopback ws connection. Reject convenience
connect/default buffers for bounded W01. No async/TLS/url feature or alternate
framework proposed. Root/Integration retains manifest/spec/lock ownership.

Archive checksum recomputed and registry-matched:
`e48ac77174b19c110a50ab2128b24215ac9cb40e0e12e093fb602d175c569d22`.
Upstream commit from archive VCS metadata:
`7f4aeaf0944992c5664c8fb8e0d54577c7e18020`.
Declared MSRV 1.85/edition 2021, MIT OR Apache-2.0; MIT notice option recorded,
no NOTICE in exact archive/pinned tree. Selected headers/manifests/license terms
and neighboring source tests inspected; no source copied; no transitive audit claimed.

Important source findings: frame/message limits are incoming, outbound needs its
own bound; write buffer default is unlimited; handshake/connect need caller deadline;
client extension-header validation is TODO; raw trace/error values can leak data.
Prescribed guards: owned deadline/cancel Read+Write socket, no DNS/redirect,
explicit finite buffers/outbound limits, reject unsolicited extensions, sanitize
logging/errors, distinguish queued bytes from new sends, bounded close/shutdown,
exact CDP correlation and no between-request recollection. Numeric memory caps
remain D05's responsibility; feature-set/host behavior is not runtime-qualified.

## Evidence and checks

Read exact archive/API/source modules and inline/integration tests listed in the
ledger; test files fetched at same commit because crate package omits tests/.
Rust 1.96.0 TcpStream source consulted for owned IO/timeout/shutdown semantics.
All external retrievals used 30-second maximum waits. Sources lived only in an
owned task-temp directory outside repo/config/skills; removed after retaining
URLs/revision/checksum/decision. No raw source/download/log/evidence files committed.

Changed local links/route consistency and whitespace: passed. No Cargo/build,
library tests, fake server, browser/runtime, external message or other-app operation.
Future fake-peer/Chromium checks are proposals, not executed evidence.

Exact write set: docs/research/W01-transport.md and this receipt only. Existing
R01/F01/D02 evidence and all source/Cargo/spec files untouched. Root granted a
short two-path Git lease; master and empty index confirmed. This receipt is in
the scoped checkpoint; exact SHA/push result returned in terminal handoff, lease
released after push. No unchanged research/runtime checks repeated for checkpoint.
Root/Integration may then adopt D07/manifest under the stated guard contract;
D05 still gates live W01, with memory/runtime qualification and five review repairs
outside this packet. No general memory/latency/production acceptance claimed.
