# Guarded Web producer composition

Status: concrete worker_web source exists; Core module/parent service wiring and
shared-input compilation/proof pending. No live browser, SDK, app or UI grant.
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

Core adds the feature-gated worker_web module only now that the actual file exists.
It owns worker_main storage/dispatch, trusted setup transport, parent ObserveReady/
Permit service and attach/submit API. Web does not change those files or synthesize
matching controls. web_config.rs adds bounded WebSetup::decode/WebSelection::decode;
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

## Pending concrete dependencies and proof

- Core module hook and real Web attachment/observe dispatch, reusing the existing
  worker guard, CanonicalSession and preallocated buffers.
- Actual parent ObserveReady/Permit and public Web attach/submit service; no shadow
  host loop or test-forged handshake can replace this dependency.
- Shared-input ready-to-run barrier before Cargo checks or real guarded-worker tests.
- Core-owned dev dependency handoff requested: tungstenite.workspace=true for bounded
  owned CDP server peers, using the already approved version/features, not a raw codec.

Once wired: prove first scoped request, reusable session/reference observation,
wrong target/ref/document, privacy, deadline/cancel and earlier committed data after
failure through actual RuntimeHost→guarded worker→Collector→parent ACK. Synthetic
peers do not qualify Chromium scripts, runtime invariance, pixels or B01–B06/D06.
No independent integration or full H01/live acceptance is claimed by this source file.
