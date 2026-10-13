# USE-WEB-RESYNC2 — fixed consistency-guard diagnostics

Authority: root’s explicit USE-WEB-RESYNC2 approval in this chat, following
USE-WEB-RESYNC1’s concrete plan and two supplied CW-B22 failures. Second attempt
had all four collapse transitionend signals, no cancels/active animations, and
started17.44s after the signal. It still returned collector_resync_required/code0.
This establishes need for branch attribution, not an animation or geometry bug.

Spec Basis: active AGENTS/implementation/product-truth/QA/operational routes;
registry50→51/product→WEB-FIRST-USE4→CLI20 OBSERVE/diagnostics1→2; prior complete
BINDING/IDENTITY1/LIFECYCLE1/PRIVACY1/EXCHANGE2/BOUNDARIES1 and D02.2/D05.4/D07.5
closure retained. Exact diagnostic record/Host validation read; RUST/DEV.RUST2 and
supported Web matrix selected. No app/Native/export behavior route.

Contract delta: restore useful diagnostics by additive closed resync branch codes
0 unspecified,1 rect triple,2 per-node viewport,3 cross-node viewport,4 final viewport.
No guard, data schema, control layout, quota, deadline, privacy or lifecycle change.
Old Host rejects new codes; deliver matching CLI+worker, not cross-version compatibility.
Current helper863dcdb already validates/prints i32 codes without modification.

Approved write set: collector mod.rs/acquire.rs/io.rs; Host worker_web.rs/diagnostic.rs;
existing collector.rs/web_live.rs tests; CLI observe_diagnostic.rs test section only;
diagnostic contract and direct revision links; this receipt. No other source edits.

Register semantics first, then exact four branch tags and bounded Host validation.
Focused peer tests cover guard→typed site→control→CLI, zero publication, unknown0,
invalid−1/5, protected default output/other codes/privacy/cleanup. Build one matched
Web pair from frozen ea06095 plus exact source patch in system temp; no branch/worktree.
Deliver base/patch/binary pins and immutable neutral handoff to root for one real
source-owner attempt. No app access, guessed fixture geometry or independent acceptance.

## Implemented and verified

Contract registration was committed/pushed first: `1f548590` (registry51).
Production delta touches five files: closed ResyncSite,
optional Failure field, four branch constructors, two default-None constructors,
Host mapping and cause-specific0..4 validator. Four comparison predicates are
unchanged. CLI production and helper source are unchanged; CLI edits are tests only.

Frozen build: original `ea06095bf311e7e33af90e8d72fe822690a3223e` archive plus
`diagnostic.patch` SHA256 `d97f42c072875a27a686eb1bcbd2e2c1285e59b7c8f27c483c314834acbcb1a0`.
All eight patched source/test paths were identical to current task-owned files;
none differed between the frozen base and pre-task HEAD. Cargo.lock/dependencies,
features, guard fields/caps/deadlines and allocator settings were not changed.
This is a diagnostic composite, not a regular release bundle or current exporter.

Checks on that composite (Rust1.96.0, aarch64-apple-darwin):

- One `cargo build --locked -p uiblueprint-cli -p uiblueprint-host --features web
  --bin uiblueprint --bin session-worker` passed.
- Collector viewport_ four tests and popup_geometry_ two tests passed; actual
  failed comparisons carry sites2/3/4 and1 respectively, with no publication.
  Existing malformed/unsupported/sensitivity and positive geometry assertions stay.
- Existing lost-events/document-event test passed; unclassified resync keepsNone/0.
- Host diagnostic two tests passed:0..4 roundtrip, unchanged12-byte record and
  64-byte control, rejected−1/5, malformed/protocol/selection codes, counts and
  wrong operation/stage/envelope checks preserved.
- CLI observe_diagnostic two tests passed, including resync code4 and existing
  bounded fixed report/unavailable/privacy tests.
- Explicit `web_live::resync_sites_cross_real_worker_host_cli_without_source_output`:
  10 controlled-peer cases (stable baseline and four guards × structured/default).
  Actual matching CLI→worker→collector→Host→CLI transport; four failures each
  report correct code, stage collect/cause collector_resync_required, committed0,
  missing1, exit4, empty stdout, remote release and worker_cleanup=confirmed.
  Default stderr stays observe_unavailable; stable baseline remains canonical
  partial4/observe_incomplete. Private canary absent from both streams. Peer and
  child waits bounded; inputs/temp directory removed with absence assertion.
  Initial test-only TCP setup was corrected to blocking accepted socket on Darwin;
  the earlier handshake failures did not reach product guard verification.
- Focused Clippy passed with warnings denied for Web/Host/CLI libs/bins and
  collector/web_live tests. Changed-file rustfmt, Markdown links/route line limits
  and git diff --check passed. No full suite, app runtime or guessed F01 performed.

## Diagnostic consumer handoff

Retained system-temp root:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-resync2-qxh1hath`.
Only `bin/uiblueprint`, `bin/session-worker`, exact `diagnostic.patch` and immutable
`handoff.json` are deliverables for root and the same Web owner through the one
qualified attempt and its diagnostic/repair handoff. Build/source transients are
run-owned and removable after those deliverables are verified. Existing shared
bundles, source evidence, images and after-title-spacing.png remain untouched.

| Binary | SHA256 |
| --- | --- |
| uiblueprint | `c458a0a42ba288b1605e52ec7e80d32c3c5820634157fb345066fcff1e187dfc` |
| session-worker | `7dd1694cf65f702a84aa60cf4271f489e7c0a4c65f0e2e9c1e3f7efc23a1d8b4` |

Use unchanged helper863dcdb (SHA256
`26093cfe2e38f155326b3ad4127fbf2511b1cbdf98fa9a658d017d307e984758`) with both
explicit executable overrides. Never pair this worker with old CLI/Host: old Host
rejects nonzero resync codes. Fresh source connection and actual transition evidence
belong to the source owner; closed previous WebSockets are not reusable.

```sh
DIAG=/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-resync2-qxh1hath/bin
HELPER=/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-export1-h1iqyuk2/candidate/web-geometry.cjs
node "$HELPER" --connection "$CDP_BROWSER_WS" --url "$PAGE_URL" \
  --component 'aside.clip-search-suggestions-shell.is-collapsed' --json \
  --cli "$DIAG/uiblueprint" --worker "$DIAG/session-worker"
```

The caller may capture stdout/stderr separately in private system temp and must
retain actual exit and transition evidence. Code1..4 identifies only the failed
comparison. It does not prove animation, numerical precision, source instability
or a utility defect. Code0 remains unspecified. Root’s independent typed-boundary
review and the source owner’s one qualified actual attempt remain separate.
No collector repair, retry loop, scope reduction or quota change is included.
