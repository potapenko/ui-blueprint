# Native form session
- Node type: leaf; domain: `uib.native.session`; contract: `UIB.NATIVE-SESSION@1`.
- Authority: Active / Evolving; implementation/acceptance separate.
- Authority source: approved PLAN.UIB@1 P5/P6 and [M02-N packet](../../plans/ui-blueprint/packets/M02-native-workflow.md), explicitly assigned full execution on 2026-10-08.
- Requires: [NATIVE@2](native.md), [CLI-ACTIONS@3](cli-actions.md), [D02@2](../development/decisions/d02-boundaries.md), [Native acquisition@2](../development/decisions/d05-native-acquisition.md) and their closure.
- Read when: attached Native form observation/preparation/delivery; excludes ordinary AX geometry.

## UIB.NATIVE-SESSION.LIFETIME
M02-N chooses one explicit resident non-capture helper in the existing registered
slots. Caller supplies session duration 1..300000ms, bounded by the retained-age
profile, and 1..8 unique exact fixture AX identifiers (each at most256 UTF-8 bytes).
This is an additional tighter Native residency profile, not enlarged D05 quotas.
Keep original CF objects and session/helper-scoped keys from first Observe until
expiry/detach/reap. Repeated Observe revalidates these objects; same-identifier
replacement refuses, never repairs old refs. No idle collection or daemon.
Each operation retains its own authoritative parent deadline. Cancel, EOF,
deadline or transport loss stops/reaps the helper and invalidates every ref.
Other sessions remain independent; ordinary one-shot AX/capture paths stay intact.

## UIB.NATIVE-SESSION.ACTIONS
Existing canonical core0.1 Snapshot/ActionCase/Expectation/TransitionCase and A02
kernel remain authoritative. Prepare freshly reads held target/result without input.
Semantic Focus records the current input owner during resolve, refuses intervening owner changes, and only after the parent Focus permit activates the exact selected process then sets its held AXFocused attribute when settable. Fresh verification requires that process to own keyboard focus. Fill(Setter) uses settable AXValue on public enabled AXTextField with an explicit full-value Expectation. Semantic Activate uses reported
AXPress. Type(Keyboard) is restricted to public, enabled, focused text fields and
the exact current frontmost process/window/focused AX object. A Type request is at most20 UTF-16 units with no control characters; CG posting is Accepted, not Confirmed. Unicode keyboard
delivery must be attributed as keyboard, never setter or IME proof. Secure input,
composition, selection mutation and nonsettable SetChecked refuse Unsupported;
secure observation remains redacted. Available selection offsets are UTF-16.
Every action uses explicit caller Expectation, with separate held result when needed.
Target and result continuity are independently revalidated. No implicit Enter,
toggle-for-SetChecked, fallback, retry or business-success inference.

## UIB.NATIVE-SESSION.EXCHANGE
Reuse private fixed HelperRequest/Reply and Configure/Submit controls. Phase0 is
Observe; 1 is fresh resolve; 2 delivery; 3 independent post-read. Only parent may
forward phase2 after matching current Possible nonce, once. Correlation binds
session/operation/helper, and helper enforces phase order and nonce monotonicity.
Parent transports the already charged original operation payload; no graph parse.
Existing Tape carries action input. Helper emits canonical records or a fixed
delivery status; worker validates records and runs existing kernel/ACK publication.
Parent ingress is reused only after the previous reply has been forwarded; no
additional stream/pool/cap. Swift SDK allocations remain the documented opaque
category, bounded logical copied values/response construction remain mandatory.

## UIB.NATIVE-SESSION.CLI
An explicit `native-session` invocation owns one attached RuntimeHost and accepts
bounded sequential commands from stdin. Each command explicitly names its request,
source and expectation files; Observe/Prepare never confer action authority.
The connection remains connection1.0.0 native_fixture; an explicit session duration
and form configuration opt into this path. JSON output is unchanged canonical NDJSON.
EOF/session expiry closes owned resources; references never survive CLI exit.
Each stdin line is a strict JSON object {"request":"FILE","source":"FILE","expectation":"FILE"} with a4096-byte ceiling. Source/expectation are required together for actions and absent for Observe. Fields/files share a per-command positive aggregate input budget. Per-command failure stops the session, preserving already ACKed bytes. Partial observation is explicitly marked and may continue; EOF0 means completed session operations, not complete AX coverage. Verified mismatch exits3; uncertainty/refusal4; IO/cleanup1; invalid/limit2; unsupported command/backend5. Existing
action/observe commands and exit meanings stay unchanged. No secrets in argv,
command records, graph, diagnostic or errors; unsupported secure input is explicit.

## Required evidence
Repeated Observe/Prepare/Act in one session, no Prepare effects, current focus/value/
selection, secure redaction, remount/ambiguity/input-owner refusal; forged/replayed/
wrong-session permit, timeout/cancel/EOF/detach, post-Possible loss without retry,
independent Target progress, actual F02 supported delivery/result and owned cleanup.
Independent shared-boundary review remains required; self-checks do not replace it.
