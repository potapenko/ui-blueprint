# S01-Web collector checkpoint receipt

- status: `waiting_evidence: committed reusable Rust support/entrypoint`.
- classification: verification with minimal test tooling; finite packet incomplete.
- authority: [S01-Web](../packets/S01-web-proof.md) and
  [shared S01 bridges](../packets/S01-bridges.md); approved S01 D02 obligation,
  explicit user parallel-work request. No new product choice.
- spec_basis: [full traversal/dependency receipt](../../../development/interface-proof-web.md#spec-basis).
- ready input: committed F01 `a8368076cdf0d4a917e7c3c5448ca6b6d919cf64` and docs
  correction `585bcd90ad455110bcc2432f8f36f58e3c8d7ecf`.
- supporting outcome: saved preparation plus independent bounded F01/B03 live
  collector/normalizer and schema file-validator evidence, explicitly test context.
- shipping/interface proof delivered: none. Shared source unchanged; D02 needs
  actual common-session admission, Ticket, parent clock and lifecycle evidence.

Write set: `tests/bridges/web/fixture-host.cjs`, `tests/bridges/web/prepare.cjs`,
`tests/bridges/web/collector.cjs`, `tests/bridges/web/collector-check.cjs`, `docs/development/interface-proof-web.md`, this receipt.
No fixture/schema/Cargo/shared-driver/coordination edits or review-P2 repairs.
Root granted a short collector checkpoint lease: only collector.cjs,
collector-check.cjs, interface-proof-web.md and this receipt. Master and empty
index confirmed; exact checkpoint SHA/push returned in terminal handoff.
Shared Rust changes belong to Integration.

Preparation checkpoint: `6029c7d6f912f826d9fd48633469a51b6b6e7496`, push to
origin/master succeeded and remote-tracking SHA matched; preparation lease released.
Collector checkpoint is the commit containing this updated receipt; its short
lease is released after push. Native/root/shared files remain untouched.

## Independent scoped checks

Stage A `9d2df153abd2a7d7567100e06d4260e5edda3bb3`; runbook `553a4e9` read.
Read canonical model, relevant validator and plugin API. Offline scoped schema
validator build used separate task-temp target dir; Cargo.toml/lock/crates/schema
matched Stage A before/after build. No P2 repair or shared write.

`node --check` for both new collector files passed.
`S01_WEB_PLAYWRIGHT_CORE=<approved-runtime> S01_WEB_VALIDATOR=<pinned-validator> S01_WEB_OUTPUT=<report> node tests/bridges/web/collector-check.cjs`
passed 45 checks: existing F01/B03 popup-open, overlay-on/off; BODY parent,
explicit anchor, hit targets, clipping oracle, separate DOM/AX source observations,
partial/unknown properties, read-only UI invariance and injected collector-argument
refusals. Six request/snapshot documents passed Rust file validator. This is NOT
ObservationSession/D02 proof and contains no fabricated Ticket or parent reading.
B02/B04, real site and historical combined polling runner were not executed.

Minimal retained evidence for D02/P1/P7 and Integration's example inspection:
`/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/P1/D02/web-collector-0ec44061-82cd-4d96-a9cd-17729a63b563/`:
`collector-report.json` plus `build-identity.json` with validator hash/basis.
Minimum normalized sample locations for D05 sizing are the report's
`snapshots[0].snapshot` (popup-open), `snapshots[1].snapshot` (overlay-on), and
`snapshots[2].snapshot` (overlay-off); each adjacent `request` records its selected
fields/context/limits. No duplicate sample files or new collection needed.
Owner=root; retain until P1/P7 acceptance or explicit discard. Narrow B03 samples
are not the largest complete F01 graph or a D05 memory policy. Raw build output
not retained/committed. Browser, CDP attachment and localhost resources closed.

## Exact remaining dependency

Root forwarded the common API need to Integration and clarified: reusable typed
Rust test support may directly call ObservationSession; no new executable protocol
is required merely because a binary is absent. Platform orchestration is allowed,
common API/validator/Ticket/clock/lifecycle must be reused. One shared support owner;
no competing host or policy in Web paths.

Next action: consume Integration's committed support entrypoint, bind existing
collector to an admitted canonical request and actual Ticket, then prove live
channels through the common Rust boundary and assigned version/framing/lifecycle
negatives. Full packet remains waiting_evidence after saving this collector
checkpoint. No prior checks rerun, new measurements or support tools added for
checkpoint; next work waits for committed Integration API handoff.
