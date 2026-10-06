# S01-Web common-interface proof receipt

- status: `done` (finite Web proof candidate checkpointed; root acceptance separate).
  No S01/P1/product or Native acceptance claim.
- authority: S01-Web/S01-bridges packets and direct root follow-ups under approved
  PLAN.UIB@1. Scope F01/B03 popup/overlay only; no new product choice.
- basis: [complete traversal and implementation handoff](../../../development/interface-proof-web.md#spec-basis);
  current D02/D03/D05 and normative closure retained; execution runbook read.
- schema Stage A: `9d2df153abd2a7d7567100e06d4260e5edda3bb3`.
- common support: `73d772e97efcf550ea4a4d3e8480b56509ebc548`, committed/pushed.
- fixture: F01 `a8368076cdf0d4a917e7c3c5448ca6b6d919cf64`, docs correction `585bcd9`.
- previous saved checkpoints: preparation `6029c7d6f912f826d9fd48633469a51b6b6e7496`,
  collector `7813cd76b301aa0002b05818f42363d5d2df18a1`; both pushed, leases released.

## Outcome and checks

Supporting verification/tooling delivered: live isolated F01 request → actual
common Rust Ticket → bounded DOM/CSSOM/addressed AX → canonical channel frame →
ObservationSession/schema validation → returned Completion data. The three live
states popup-open/overlay-on/overlay-off retain 12/14/12 source nodes respectively,
partial coverage, raw roles, source times/clocks, identities and anchor evidence.
Actual returned channel Documents equal submitted normalized data structurally.
Parent terminal elapsed 32/29/24ms; this is deadline evidence, not Q02 acceptance.

Six additional Web-orchestration negatives: injected cancel/detach/expiry with
recorded live data marked stale/cache and correctly correlated late reply rejection;
wrong response version and malformed frame reject then canonical recovery succeeds;
oversize rejects with host exit2. All nine scenarios meet literal expectations;
eight retained-data equality comparisons pass. No fake Ticket or parent reading.
Live vs recorded/synthetic source and injected control are explicit in report.

Commands/checks:

- `cargo +1.96.0 build --locked --offline -p uiblueprint-plugin-api --example d02_host`:
  pass, separate task-temp target directory. Shared build inputs matched support
  commit before/after; exact host hash in retained build identity.
- `node --check tests/bridges/web/interface-proof.cjs`: pass.
- `S01_WEB_PLAYWRIGHT_CORE=<runtime> S01_WEB_HOST=<pinned-host> S01_WEB_PROOF_DIR=<new-dir> node tests/bridges/web/interface-proof.cjs`:
  pass 3 live/6 injected cases. Existing B03 oracle used by assertions, not collector.
- Common support 3 tests/8 synthetic scenarios reused from producer handoff;
  prior 45 collector checks not rerun. Wrong request-version coverage remains that
  labelled synthetic producer evidence; live response-version is tested here.
- Changed local links, scope and whitespace: pass; current source hashes match report.

Limits explicit: 32 nodes/depth 8,65536 frame/output bytes,131072 pending encoded bytes,
8 frames,250ms request,1000ms late grace,4s host watchdog,40s whole-run watchdog.
Node 24.15.0/Playwright Core 1.58.2/Chromium 145.0.7632.6,800×600/DPR1; same owned
headless fixture environment. All owned hosts/CDP/browser/localhost resources closed.
Queue/read-thread limits are not proof of OS-syscall cancellation. No physical focus,
real site, B02/B04, historical polling runner or production adapter used.

## Exact checkpoint scope and evidence

New file `tests/bridges/web/interface-proof.cjs`; updates only
`docs/development/interface-proof-web.md` and this receipt. Existing collector and
fixture files unchanged. No Cargo/schema/common-support/Native/root edits and
no review-P2 fixes. Root granted an exclusive three-path checkpoint lease;
master and empty index confirmed. This receipt belongs to that checkpoint; exact
SHA/push result is returned in the terminal handoff. Lease released after push.

Minimum durable proof directory:
`/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/P1/D02/web-proof-956d43cd-9297-41fb-ba24-ae5612e75e3a/`.
`report.json` contains actual host receipts, labelled cases, equality hashes and
collector/source/build identities; `build-identity.json` records pinned build.
For D05 use canonical returned normalized Documents:

- `popup-open/retained/channel-0.json`
- `overlay-on/retained/channel-0.json`
- `overlay-off/retained/channel-0.json`

Adjacent session.json/request.json retain exact capability/scope/field/budget inputs.
Injected-case retained data are labelled separately; they are not fresh samples.
Earlier collector-only evidence remains at
`/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/P1/D02/web-collector-0ec44061-82cd-4d96-a9cd-17729a63b563/`
(report snapshots[0..2].snapshot plus build identity), explicitly not full D02.
Owner=root; consumers D02/D05/P1/P7; retain through P1/P7 acceptance or explicit
discard. Narrow B03 samples are not the largest complete F01 graph or a memory
policy. No raw build logs or captures committed.

## Remaining condition

The shared API dependency is satisfied for this finite Web proof. The three-path
checkpoint and push save the candidate; root owns its acceptance. No unchanged
runtime checks repeated for checkpoint. All original proof evidence is preserved.
S01/P1 freeze, Native proof, D05 sizing and four P2 findings remain separate owners/
gates. Stop after this finite packet; do not start W01 or a host redesign.
