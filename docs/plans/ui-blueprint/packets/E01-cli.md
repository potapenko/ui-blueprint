# E01 public imagegen-prompt CLI integration

- Class: shipping_product; existing Export owner, inherit model/reasoning.
- Outcome: the real `uiblueprint imagegen-prompt` command produces the saved
  compiler's complete package from explicitly selected local input, without a
  model/key/network. This is the next P6 release-path capability, not a demo wrapper.
- Authority: approved PLAN.UIB@1 (`358c757`) P6/early CLI and direct parallel-chat
  authorization; current master only. No nested agents or other chats/projects.
- Ready basis: Export `00b70cc` + saved membership `dbdccf6`, shared hashes matched;
  L01 `c9555f6` + membership `5522966`. These are candidates, not accepted releases.
- Economy: reuse the existing compiler and CLI IO/errors; no second package writer,
  graph, validator, measurement engine or generic command framework.

## Spec Basis and protected behavior

Root traversal: AGENTS → specs/README@UIB.ROUTING1/registry4 → product branch →
CLI@1 → EXCHANGE/PRIVACY/MODEL/IDENTITY/BOUNDARIES@1; EXPORT@1 → complete
DRAWING-PACKAGE/STYLE/GEOMETRY/PROMPT-A/PROMPT-B/REVIEW@1 and reference
DRAWING-EXAMPLE@1 → GEOMETRY/PROJECTIONS@1. All CONTENT clauses fully read and
current, with no semantic drift. Rust and D03/D05 closure is pinned in E01 and
K01-storage-design; DEV.RUST@2/D07@2, other selected decisions/leaves @1.
Read current runbook, L01 receipt/CLI contract and E01 receipt/API handoff.
Operational-safety applies to bounded diagnostics. Reuse current read contracts;
do not reload unrelated source/spec siblings. No product-spec semantic delta.

Mode Restore: required full local export command, explicit observed/proposed
bases and independent statuses, full selected scope/unknowns, safe client text,
finite input/output and existing-file preservation. Observed: compiler and local
example exist; public CLI wiring is missing. Exact syntax/error mapping are
delegated implementation choices, documented before source edits, not new intent.

Preserve existing measure/check behavior, canonical output and known consumer
gaps. Five protected review fixes, MeasurementResult schema change and D05 policy
remain outside this packet. Do not claim the new command closes those gates.

## Finite implementation and acceptance

Wire document (default)/propose and the compiler's existing detail/flow/compare
modes through the public command. Use canonical DrawingBrief and Snapshot inputs;
no UI facts, normative requirements, safe labels or permission are guessed to make
an underspecified invocation work. Support explicit local inputs and document
their real syntax; stored-ID resolution awaits K01. Missing required metadata
returns a useful bounded error. No filesystem/URL dereference from payload refs.
Keep unresolved G02 comparison attribution visibly unresolved.

Produce all six required files and full A+B prompt through the existing library,
with only permitted references. Existing exports/baselines must not be overwritten.
Default stdout is compact and truthful; any JSON mode uses one documented stable
export-owned result, never a private replacement normalized schema. Document exits
and limits before implementation. Invalid/oversize/sensitive input and failed writes
must not appear successful or dump raw payload/private paths to diagnostics.
No model invocation, generated images, capture or new external dependencies.

Writes: `crates/cli/**` (existing check/measure semantics protected),
`docs/development/cli.md`, `docs/development/export.md` for real CLI examples,
`docs/plans/ui-blueprint/receipts/E01-cli.md`. Only Integration updates root lockfile;
request the exact existing export path dependency/lock integration through root.
No compiler/schema/engine/fixture/spec edits; return a precise dependency if needed.
Core owns only K01 design docs concurrently and has no CLI source lease.

Focused tests exercise the actual binary: full proposed and observed packages,
default document and explicit mode handling, fractional/unknown/status preservation,
full template and sheet scope, invalid/sensitive input, limits, existing destination,
sanitized errors and exit/stdout behavior; existing CLI tests protect regressions.
Use saved examples and independent expected assertions, no invented live evidence.
Scoped Rust1.96 locked check/fmt/Clippy/CLI tests; shared inputs fixed and hashed.
No unrelated suite or repeated unchanged export suite. E02 independent review follows;
author checks do not establish privacy/release acceptance.

Checkpoint-ready → root Git lease → scoped commit+push → exact SHA/push/checks/
source revisions/residual/lease release. Stop after this result. Missing shared
dependency gets a precise waiting condition while independent work continues.
