# Single-step action CLI

- Node type: leaf; domain: `uib.cli.actions`.
- Authority: Active; Stability: Evolving; accepted/released baseline: none.
- Contract: `UIB.CLI-ACTIONS@1`; supersedes: none.
- Authority source: approved PLAN.UIB@1 P5/P6, CLI.CONTENT/ACTIONS and [root's selected finite packet](../../plans/ui-blueprint/packets/L01-actions-contract.md) under ROADMAP; registration is not implementation acceptance.
- Read when: action prepare/execute commands, authority, output or exits.
- Do not read when: unchanged inspect/observe/diff or unrelated backend acquisition.
- Requires: [CLI@6](cli.md), [ACTIONS@1](actions.md), [IDENTITY@1](identity.md), [LIFECYCLE@1](lifecycle.md), [D02@2](../development/decisions/d02-boundaries.md); their explicit closure applies.

## UIB.CLI-ACTIONS.SCOPE — First concrete caller

Expose one SetChecked(bool) through the existing guarded worker and actual provider.
This is not full scenario/form/multi-step or Native delivery support. Unsupported
intent/backend is explicit; those remaining goal capabilities are not removed.
Core0.1/analysis0.2, existing commands, evidence, policy and feature defaults stay.
No new graph, scenario DSL, step loop, dependency, implicit discovery or source launch.

## UIB.CLI-ACTIONS.INPUT — Canonical records and bounds

```text
action prepare --connection FILE --snapshot FILE --request FILE --worker ABSOLUTE_PATH --max-input-bytes N --max-output-bytes N [--json]
action execute --connection FILE --plan FILE --request FILE --worker ABSOLUTE_PATH --max-input-bytes N --max-output-bytes N [--json]
```

Reuse strict connection1.0.0, explicit host limits, supported host/features and
established exact Web target/document from CLI.OBSERVE; defaults remain empty.
Three explicit regular files share one positive aggregate input budget. --snapshot
accepts canonical Snapshot or observed ChannelResponse containing it, as Inspect.
Failed/no-snapshot response refuses; full envelope validation and extraction stay
in the guarded worker, preserving the embedded Snapshot/evidence unchanged.
Plan is the existing single ActionCase Document.
Request is the existing core0.1 Request Document with Prepare or Act respectively.
Prepare uses Tape(Snapshot/observed ChannelResponse,Prepare Request); Execute uses Tape(ActionCase,Act
Request). Reject wrong type/count/order, binding, intent or unsupported backend.
Target/session/plugin/surfaces/scope/ref/intent must agree before dispatch; worker
validation remains authoritative. Output budgets include newline and respect the
tighter canonical Request/host bounds. No secret values in argv or diagnostics.

## UIB.CLI-ACTIONS.AUTHORITY — Explicit scope and revalidation

Prepare is read-only and confers no nonce, physical-input or mutation authority.
Explicit Execute invocation is the trusted caller mutation request only for the
exact validated connection target, subject to existing user/tool/project policy.
Plan, capability, response and UI data cannot authorize or widen that request.
No force bypass, additional arbitrary confirmation, retry or rollback follows.
After actual Attached, rebind only Request.clock_domain as in Observe. Preserve
saved Snapshot/Observation IDs, timestamps, clock domains, evidence and context.
Refuse unestablished session/document continuity; revalidate the exact live identity
and capabilities before permit. Saved rights/current flags are not permanent rights;
stale/remounted targets never redirect to a same-label or coordinate substitute.

## UIB.CLI-ACTIONS.OUTPUT — Delivery and verified outcome

JSON is one unchanged fully matching ACKed canonical core0.1 Document plus newline:
Prepare ActionCase/Error; Execute TransitionCase (Artifact::TransitionContext)/Error.
No new wrapper, public error or graph. Setup/no-commit failure leaves stdout empty;
preserve ACKed bytes after later failure, holding leases through publication.
Compact is bounded fixed preparation/delivery/verification/completeness status,
without raw UI/configuration text. It must distinguish delivery from verification.
Parent never parses graph bodies. A small fixed producer outcome derives solely
from the validated kernel result and matches Frame/Commit/ACK; it survives only a
matching ACK. Missing/corrupt/late metadata cannot establish success. Pre-Possible
refusal flags1 remains distinct from post-Possible non-success. Private encoding is
a separately selected implementation dependency, not a new public schema here.
Always clean/reap owned workers/helpers, never target browser/application. Physical
stdout IO failure may leave a partial write; no partial body is advertised complete.

## UIB.CLI-ACTIONS.EXITS — Truthful completion and recovery

Prepare0 requires a complete prepared ActionCase and successful cleanup. Execute0
requires verified requested source state, full matching ACK, protocol Completed,
confirmed delivery and successful cleanup; source-state success is not business
or application success. Known verified mismatch3; refused/uncertain/cancelled/
timeout/missing semantic evidence4. IO/internal/unconfirmed cleanup1 overrides
success; invalid input/limit2 and unsupported command/backend5 before Possible.
After Possible, absence of verified outcome is uncertainty4 rather than a safely
retryable pre-dispatch input failure, unless IO/cleanup1 applies. Earlier committed
results and effect uncertainty survive loss; establish actual state before retry.

## UIB.CLI-ACTIONS.ACCEPTANCE — Required evidence, currently pending

| Scenario | Required result |
| --- | --- |
| Confirmed delivery with Failed/Unknown check | Never Execute0; mismatch3 or uncertain4 |
| Corrupt/missing/late outcome metadata | No accepted verified-success metadata |
| ACK then worker loss/cancel/expiry | Original bytes retained; truthful effect/exit |
| Read-only target or stale/remounted plan | No unauthorized delivery or redirection |
| Valid Prepare / verified exact SetChecked | Prepare0 / Execute0 only with full ACK and cleanup |
| Output IO or unconfirmed owned cleanup | Exit1 overrides success |

## Change record

L01-ACTIONS-001: Evolve preliminary CLI action syntax into this first concrete,
unreleased slice under the selected packet. CLI@5→@6/registry15→16; protected
core/analysis/connection versions and existing commands unchanged. Source c3967ca
shows delivery-only HostCompletion; producer metadata, caller and actual CLI runtime
acceptance remain unimplemented gates, separate from ongoing Web host qualification.
