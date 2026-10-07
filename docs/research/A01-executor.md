# A01 finite executor source handoff

Consumer: the next A01 controlled-data implementation, then guarded Web/Native
executor composition. Authority: [finite packet](../plans/ui-blueprint/packets/A01-execution-handoff.md),
approved PLAN.UIB@1/P5. ACTIONS/IDENTITY/FORMS/LIFECYCLE/EXCHANGE/MODEL/PRIVACY/
CACHE plus D02/D04/D05 closure and EXECUTOR-SOURCES/REUSE were read. This is source
evidence/technical handoff, not permission for live input or a new scenario language.
Only two named documentation files are writable; no source/fixture/spec changed.

## Required pinned primary-source ledger

Ui.Vision revision `17b6302f1617b838efe2b26d3ef19c2face81350` was read in memory:

| Pinned source | Mechanism observed | Disposition for A01 |
| --- | --- | --- |
| [command.ts](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/src/common/command.ts) | Explicit browser/desktop command scope; action and verification vocabulary | Compare capability vocabulary; reuse our closed Intent/InputModality rather than importing a macro DSL |
| [command_runner.js](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/src/ext/content_script/command_runner.js) lines49–77,485–598,627–743,797–847,1051–1077 | Final retry may use secondary locators; check/type/select mutate and return acceptance; verify reads actual state/logs mismatch while assert throws | Reimplement delivery versus verification separation; reject secondary-target fallback and acceptance-as-user-success. No arbitrary script, hidden modality switch or retry after possible effect |
| [player.ts](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/src/services/player/player.ts) | Playing/paused/stopped/error states; completion/error/manual end reasons; doneIndices/errorIndex | Preserve completed steps and stopped reason in existing Transition/Step; do not adopt loop/runner state as product contract |
| [timer.ts](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/src/services/player/monitor/timer.ts), neighboring [types.ts](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/src/services/player/monitor/types.ts) | Timer accumulates Date-based pause/resume elapsed time through inspector lifecycle | Reimplement using host-authoritative monotonic deadlines and local clock domains, not wall-clock Date arithmetic |

[LICENSE.txt](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/LICENSE.txt)
declares AGPLv3 or commercial licensing; [package.json](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/package.json)
declares ISC. Treat as behavioral reference only. No code copied, dependency adopted,
repository cloned, upstream installed/run or full license audit claimed. No region
picker mechanism is required, so desktop_vision was excluded under the packet.
Pinned small-file SHA256: player.ts d8715bc4bb5e808978a816919a45eadab2502130f335cbd58c25f40897b4c4d2;
timer.ts ced79641bb4eb885b2a2fe7825fc2e3a11658cf620b92511237297a8410868a8;
LICENSE.txt9713a03495e632282de16a9ab8d113785e5e48af1b262fc3692b49518dc99fa4.

## Existing reusable product owners

Schema model owns Intent, Action/BackendRef/Resolution, Request Prepare/Act,
DeliveryStatus, Outcome, Step, Transition, ActionCase, ActionResult and TransitionCase.
There is no serialized scenario/plan or successful standalone ActionResult artifact;
ActionResult is currently a validated pre-dispatch refusal. Preserve these meanings.
schema validation/outcomes.rs::validate_action enforces full Context/ref/Observation
binding, unique_match, current Enabled, writable/value_allowed for setters, declared
available intent, sensitive-value handling and pointer hit-region/Space prerequisites.
validate_transition requires confirmed delivery and actual after/verification links
for Succeeded, Unknown delivery→ActionOutcomeUnknown and stop_on_error consistency.
These validate declarations; saved Current flags alone do not establish current UI.

plugin-api currently owns ObservationSession only, explicitly excluding actions.
host worker_ops::CanonicalSession::execute rejects Mutation in its default branch.
Parent supervisor/effects already owns TargetLease mutation permission, same-Target
claim and physical-input claim, one-use nonce and effect receipt. EffectReady sets
Possible before sending EffectPermit; no permit after authoritative expiry/cancel.
Loss after permit retains Possible; completion with matching nonce confirms the
reported delivery boundary, not verification/user success. Unconfirmed reap keeps
claims/grants quarantined. Frame/Commit/ACK preserve complete output bytes.

## Smallest next implementation proposal

Implement a bounded single-step SetChecked(bool) execution core over existing typed
ActionCase/Step/TransitionCase, with a finite fake backend supplied by tests. Likely
owner: a narrow action lifecycle module in existing plugin-api; guarded composition
later in worker_ops/worker action IO owner. This is a proposal for root's next packet,
not an invented wire format or authorization to add directories/frameworks.

1. Prepare validates canonical ActionCase and exact allowed scope/modality/capability;
   preparation grants no new authority. Platform resolver must independently attest
   current same Target/Surface/backend ref, uniqueness and actual preconditions.
   Stale/ambiguous/disabled/unwritable refuses before permit; do not rebind to a label.
2. Under trusted caller mutation authority and a finite monotonic deadline, request
   the existing correlated parent permit only after that fresh resolution. Consume
   its nonce once at delivery; recheck cancellation/deadline before new dispatch.
3. Record delivery separately from verification. Accepted/Confirmed delivery alone
   remains PendingVerification, not Succeeded. Reobserve exact current source for
   Checked==requested bool with honest availability/Observation links. Unknown,
   mismatch or changed binding cannot pass; do not infer checked from role/geometry.
4. Emit existing canonical TransitionCase/Step and validate before commit/ACK. Cancel/
   timeout before permit is NotDispatched/Interrupted; after possible effect retain
   completed delivery and ActionOutcomeUnknown/stopped_at, never rollback/retry.

Single-step first is concrete: host presently grants one nonce per Mutation operation
and rejects duplicate EffectReady. Multi-step execution/journal checkpoints are a
separate actual protocol/state choice; current fake-effect tests do not already
provide that functionality. Do not claim prior completed steps survive worker loss
without an outside-worker committed representation. Real platform delivery remains
unsupported until its narrowly authorized backend/current-state/permission gate.

Independent cases: ENV-ACTION-VALID SetChecked(true); G01-READONLY refusal;
stale generation/ref and two identical labels without unique binding; accepted
delivery with unavailable/mismatched after state; ENV-TRANSITION-VALID checked
verification; G01-CANCEL_AFTER_DELIVERY preserves confirmed dispatch but unknown
outcome. Existing effects_host/effect_peer prove one-use nonce, loss/duplicate permit,
before/after-permit cancellation and physical lane; they are fake delivery evidence.
No golden or test was executed here. Platform dependencies: exact current resolver,
capability/modality-specific delivery, reobserve verification and owned teardown.
Secret resolution, arbitrary application input and live action proof remain closed.
