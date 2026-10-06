# D02 — packaging and request-owned sessions

- Domain: `uib.development.d02`; accepted/released baseline: none.
- Authority source: [C01-DEC-001](README.md#meaning-and-precedence).
- Node type: leaf; contract: `UIB.D02@1`; clause: `UIB.D02.CONTENT`.
- Authority: Active / Stability: Evolving; chosen design, interface proof pending.
- Read when: S01 plugin boundary, W01/M01 adapters or lifecycle integration.
- Do not read when: only pure geometry is affected.
- Requires: [BOUNDARIES@1](../../product/boundaries.md),
  [EXCHANGE@1](../../product/exchange.md), [LIFECYCLE@1](../../product/lifecycle.md),
  [NATIVE@1](../../product/native.md), [PRIVACY@1](../../product/privacy.md),
  [D01](d01-support.md), [D03](d03-data.md), [D05](d05-limits.md), [evidence](evidence.md).
- Owner/deadline: S01 common interface proof before P1 freeze; W01/M01 realization.

## Requirement and evidence

Rust owns analytics and state; collectors are narrow and bounded. R01 demonstrates
direct addressed CDP operations; R02/F02 demonstrate a thin public-API Swift helper.
Neither prototype proves one shared plugin interface. F02 concurrent capture
actually failed and withheld AX results, so independent-session progress is open.

## Chosen approach

- Statically linked Rust schema/engine/plugin API/CLI and selected Rust adapters;
  no dynamic ABI, network daemon, service framework or mandatory model process.
- Web adapter: Rust session owns CDP connection and a small scoped DOM/CSSOM
  collector; protocol library approval belongs to D07 before W01 adoption.
  Addressed AX is a separate source. Full DOMSnapshot then filtering is excluded
  from the normal scoped path; unsupported boundaries stay partial/explicit.
- Native adapter: own thin Swift helper over bounded newline-delimited UTF-8 JSON
  pipes; not a foreign CLI invocation per measurement. Session may retain its
  helper between explicit requests, idle without collecting. One target/session
  owns its handles and process lifetime. Use length-limited framing before parsing;
  escaped JSON newlines are data; diagnostics never share machine stdout.
- Emit each completed channel result with request/session ID before waiting for
  another channel. Parent owns final aggregation/deadline, preserves completed
  sanitized data on capture timeout and rejects late success after cancellation.
  A hung native call has an owned-process termination boundary; async cancellation
  alone is not proof the OS call stopped. No shared lock across AX/capture waits.
- Capture serialization, if needed after diagnosis, is scoped to capture resource
  with bounded admission/deadline; it must not serialize unrelated AX or Rust work.
  This is a repair direction, not evidence that serializing fixes the continuation
  failure or that capture concurrency works.
- Attach negotiates plugin ID/version/schema range, target/generations and
  per-channel capabilities; observe carries scope/fields/limits/freshness policy.
  Detach clears subscriptions/handles and owned helpers, never closes user apps.
  Subscription remains optional; events only invalidate, never recollect.
- Request deadline uses parent monotonic time; transfer remaining budget to a
  helper with its own monotonic clock and keep parent deadline authoritative.
  Do not pretend independent process clock readings share a clock domain.
- Default CLI invocation can own a short session; reusable session API permits
  multiple explicit requests in an existing host. Snapshot-file analysis is local;
  no auto-started background service is needed to implement that path.

Rejected: linking Apple SDK into core; own accessibility-name engine copied from
Playwright; invoking a foreign full CLI chain per request; network daemon or
dynamic plugin ABI without a consumer; globally queued native calls; polling to
manufacture freshness. Alternatives add dependencies or contradict observed risks.

## Proof obligation, not marked passed

S01 owns `D02-PROOF` before P1 interface freeze: one common request/response envelope
round-trips **both** the F01 Web and current F02 native fixtures through narrow
test bridges. Exact target, requested fields, channel evidence, capabilities and
partial coverage must survive the same Rust validator. Check incompatible version,
oversize frame, malformed frame, timeout after AX completion, cancellation, detach
and late response rejection. Run explicit requests only; native lane needs root's
reservation. Existing scripts need a bounded integration packet from root if the
S01 write set lacks those bridges; mocks alone do not close the two-platform proof.

This obligation does not block starting T01 or S01's candidate schema. It **does**
block freezing P1 interface as verified. W01/M01 remain responsible for real
collector/input correctness; K02/V01 separately test lifecycle/privacy/isolation.
