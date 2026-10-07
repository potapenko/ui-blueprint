# Guarded Web producer composition

Status: real guarded worker/parent composition passed bounded synthetic-peer proof
on Core c0abcff. Independent integration/live browser/SDK/UI acceptance remains open.
Authority: [W01 guarded worker packet](../plans/ui-blueprint/packets/W01-guarded-worker.md),
shared boundarya8e5c06 and existing D02/D05 contracts, not a new host/parser/graph.

## Actual module handoff to Core

Executable-private crates/host/src/worker_web.rs exports:

```rust,ignore
WebSession::attach(raw_descriptor: &[u8], setup: WebSetup, target: TargetLease,
    clock: Id, origin: Instant, limits: HostLimits, deadline: Instant)
    -> Result<WebSession, HostError>
WebSession::observe(&mut self, session: &mut CanonicalSession<'_>,
    io: &mut WorkerIo, publication: &mut [u8], control: Control,
    input: &[u8], now: fn() -> u64) -> Result<(), HostError>
```

Core c0abcff supplies the feature-gated module, worker storage/dispatch, trusted setup
transport and real parent ObserveReady/Permit service. RuntimeHost::attach_web consumes
Tape(SessionDescriptor, WebSetup); submit_web_observe consumes Tape(Request, WebSelection).
No Native binding is required. Web did not edit those files or synthesize controls. web_config.rs adds bounded WebSetup::decode/WebSelection::decode;
existing serde/configuration types remain separate from canonical graph/request data.
Setup is immutable trusted attachment configuration, not an observation/UI-provided
endpoint. Canonical descriptor target must match TargetLease; configured surface
must belong to that descriptor. Numeric-loopback transport validation is reused.

Attach installs the mandatory upstream log boundary before Transport→Client→Collector.
The worker delegate emits no raw logs; bounded Core controls/errors remain diagnostics.
A foreign logger is refused, never replaced; max_level is not changed. Original
worker Clock origin/domain and attach deadline are supplied by Core, never invented.
The Web connection is retained in the worker; default/core/Native feature paths stay
Core-owned. No extra endpoint, registry, reconnect, model or browser launch exists.

## Real observation/publication path

Input Tape must have exactly two segments: existing canonical Request plus explicit
WebSelection configuration (Initial ids/visit cap or References). No prerecorded
ChannelResponse input is accepted by this producer. Observe calls actual
CanonicalSession::begin_observation and uses ObservationRun.ticket.sequence.
worker_main::admit_observation then waits for the actual parent permit and returns
the tightened worker deadline before any requested acquisition starts.

The Collector reuses the same instance, authority, clock and R1/R2 bounds. Its owned
Document callback encodes directly into the supplied fixed publication slice under
PublicationGuard. The original Document drops before canonical receive/validation,
which stays under the ordinary allocation quota. Only after real receive does
worker_main::publish send Frame/Commit and wait for matching ACK under the publication
allowance. The callback returns Acknowledged only on that success. Per-frame and
cumulative control limits are checked; there is no growing encoding Vec or parent
JSON/graph decode. Earlier ACKed bytes remain parent-owned on later failure.
ObservationRun::finish performs actual complete; all unfinished paths use Drop-cancel.
Callback failures remain the exact bounded HostError instead of being replaced by
Collector's generic PublicationStopped error. No unconditional test ACK is supplied.

Bootstrap convenience refs are dropped: actual source/Snapshot/Observation IDs already
exist in the canonical emitted document. No new completion JSON or uncharged ref
cache is invented. A later References configuration must use those actual IDs through
its canonical-data consumer; production supervisor code must never decode the graph.
Source limits map to ResourceLimit, source expiry to DeadlineExpired, stale/missing/
ambiguous continuity to ResyncRequired, malformed/unsupported configuration to
InvalidInput. Protocol/IO and cleanup refusals retain bounded HostError categories;
Core owns their existing terminal-control mapping and actual process cleanup.

## Focused guarded proof and remaining boundary

Source module/config d9de0a6 compiled and ran unchanged with Core provider c0abcff.
Five real RuntimeHost/guarded-worker cases plus one pure configuration case pass:

- A transparent Darwin wrapper withholds the REAL ObservePermit; no new CDP command
  occurs until it is forwarded. After prior Validate, host operation2 uses real
  Observation Ticket1. The canonical response and actual permit agree on that Ticket.
- First request has no prior refs. The real ACKed Snapshot supplies reference IDs
  for a later existing-ref request on the same CDP/worker session without new search.
- First Observe ACK is forwarded; the second actual ACK is withheld, then parent
  cancels. Only the first channel survives, still charged/readable after worker reap.
  The second channel is an explicit unsupported-capture response, not a capture test.
- Wrong canonical target refuses before Permit/source IO. Sensitive source/attribute
  canaries are absent from canonical data; no raw output/logger bridge was introduced.
- Wrong ref and changed document refuse while earlier bytes survive. A bounded
  stalled CDP read reaches the original parent deadline; owned worker cleanup completes.
- Configuration decoding rejects endpoint injection, oversized bytes and non-objects.

The wrappers only delay forwarding real controls; they never manufacture permit/ACK
or replace the allocator/process/ObservationSession. Production parent remains opaque;
known small canonical output is decoded only by the test client for assertions.
Every runtime case reaches zero reserved sessions through actual shutdown/reap; peers
close/join under bounds. No general guard, unrelated Native/Core or browser suites rerun.

Default/web scoped checks, Web target Clippy and owned formatting pass. Exact input/
binary hashes, commands and cleanup ownership are in the [receipt](../plans/ui-blueprint/receipts/W01-guarded-worker.md).
Synthetic peers do not execute Chromium scripts or qualify browser invariance, opaque
backend memory/cost, pixels or B01–B06/D06. No full H01/live acceptance is claimed.

## Live preparation only

[W01-live preparation](../plans/ui-blueprint/receipts/W01-live.md) has one failed first-observe run with no canonical frames. The four-file harness now exposes bounded terminal codes and explicit failure shutdown/reap; the changed-case run returned invalid_input with confirmed zero-session cleanup. Core1810b1d now supplies the fixed private carrier; Web maps original producer failures and harness captures only bounded numeric fields. Changed producer compilation passes; next one-case run retains identical caps/oracles. Source/backend/limits remain fixed; no live success is claimed.
