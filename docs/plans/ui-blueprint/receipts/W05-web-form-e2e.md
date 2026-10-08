# W05 — public Web form workflow and E2E qualification

2026-10-08. Restore / shipping qualification under approved PLAN.UIB@1 P5/B02
and Web E2E P7, [W05 packet](../packets/W05-web-form-e2e.md). This chat's dispatch
explicitly requested local plan followed immediately by implementation, own
headless runtime, repairs, documentation and current-master commit+push. No new
agent/chat, branch/worktree or real PlayPhrase.me project was used.

## Basis and scope

Traversal: global/local AGENTS → implementation governance; product-truth core,
routing, change, evidence, delivery and coordination; QA/operational safety;
docs/specs/README → product/README and acceptance/README. Complete selected
CONTENT@1 closure: ACTIONS, FORMS, IDENTITY, LIFECYCLE, CACHE, PRIVACY, MODEL,
BOUNDARIES, GEOMETRY, PROJECTIONS, ROADMAP, RUST-BOUNDARIES, PILOTS, WEB-PILOTS,
COMPLETION, GOLDEN, PERFORMANCE; Native pilots/NATIVE@2 as explicit dependency
only. EXCHANGE@2, CLI@15, CLI-ACTIONS@3, CLI-DIFF@2, CLI-GRAPH-DIFF@1,
CLI-GEOMETRY-DIFF@1, ANALYSIS@2/TYPES/VALIDATION@1; EXPORT and full DRAWING
PACKAGE/STYLE/GEOMETRY/PROMPT-A/PROMPT-B/REVIEW/EXAMPLE@1. Decisions branch →
D01@1/D02@2/D03@3/D04@1/D05@4/D06@1/D07@5, MEMORY@2/WORK@1/EVIDENCE@1;
RUST.md/DEV.RUST@2/REUSE@1. Old dependency labels resolve the current additive
revisions. Concurrent Native-session routing does not change the Web contract.
Applicable clauses are action delivery/verification/stop, CLI authority/output/
exits, scoped source identity, form-state attribution, analysis binding and
immutable paired-export facts. No semantic specification delta is made.

Supporting evidence: existing W02 Focus/Type and A05 application handoff sections,
W04/B03/E03 results, development CLI/collector/fixture/export guides; Web
worker/provider/native-button and fixed DOM reader, F01 controller and independent
expected.json, guarded-live launcher and CLI/export consumers. Native/shared
host/schema/engine/CLI/manifests/spec registry, new mechanisms/dependencies and
independent/P7 acceptance were excluded. No upstream algorithm was introduced.

Requirement: a complete public-input chain with independently expected results,
fresh significant-transition observations and truthful stop/unknown semantics.
Observed implementation already supplied the necessary bounded APIs. The existing
A03 chain began at fixture-filled L; A05 began at fixture-filled Lon. Technical
choice: reuse those shipping APIs in one finite harness starting empty, then
consume original observations through Rust diff/check/export. No scenario DSL,
parallel graph/parser, automatic input fallback or product-source edit was needed.

Declared before edits, final write set: tests/bridges/web/form-e2e.cjs,
tests/bridges/web/guarded-live.cjs, docs/development/fixtures-web.md and this receipt.
Other owners' dirty paths and the unrelated image remain untouched.

## Delivered result and observed evidence

The capability already exists in the product; W05 delivers reproducible composed
qualification and its developer entry point. Author-run34 public calls /31
per-call invariant checks passed on2026-10-08T13:19:38.349Z–13:19:46.497Z.
Independently authored literal expectations and controller state are separate
from the collector; this is not independent reviewer acceptance.

| Requirement | Actual public flow / result | Limit |
| --- | --- | --- |
| IDENTITY / ISOLATION | Exact own tab and original document; other same-title tab unchanged on every call. Portal relationship preserved; remounted Commit gets a new key; old prepared ref refuses4, NotDispatched/Failed and no after Snapshot. | No title/coordinate fallback or foreign target access. |
| FORMS / ACTIONS | Empty Observe → Semantic Focus → Type Lo → invalid readiness/Observe → Type n → suggestion readiness/Observe → Activate option → fresh selected Observe → Activate Commit → fresh applied Observe. Five Execute0 results have Confirmed/Succeeded and verification observations. | Type uses CDP Input.insertText/native ImeCommitText, not hardware key or IME-composition proof. Activate is untrusted native HTMLElement.click, userGesture=false. |
| SEMANTICS | Lo/Lon remain selected/applied empty, valid=false, deliveries0; selection yields draft/selected London, applied empty, valid=true; separate Commit yields applied London and exactly1 controller delivery. Authored event sequence has pending/invalid/pending/suggestion-ready/selected/applied. | DOM output alone is not application success; separate fixture controller confirms local applied state, never network/server/durable business success. |
| FRESHNESS / STOP | Fresh explicit Observe at significant transitions. Unexpected dialog + disabled Commit stop the positive chain. Separate disabled/private adversarial Execute probes refuse4 before delivery. Post-delivery private result produces Confirmed/action_outcome_unknown, stopped_at and stop_on_error=true; explicit reobserve is redacted and click count remains1. | No autonomous batch runner, rollback, hidden observe or retry. Independent refusal probes after the stop are not dependent positive-workflow steps. |
| GEOMETRY / ANALYSIS | Rust graph diff retains both original Snapshots,0 omissions and draft/applied value changes; Rust Check returns Pass, measured32css_px against independently authored Commit height32±0.01 from F01 CSS. | No inferred padding, common clock or atomic multi-command capture. |
| EXPORT / MODEL_FREE | Public paired export0 creates all six files; engine_recorded_graph/compared,0 omissions, two observed views with all6 scoped DOM/AX nodes each. Source intervals, last_verified, consistency, freshness and coverage preserved. | Image remains unverified, approval draft, generated_image=false. Values obey export policy; no ImageGen call/key or generated image. |
| PRIVACY / IMMUTABILITY | Private input canary absent from all34 canonical outputs and package; requested value is redacted. All saved request/expectation and response bytes remain unchanged after consumers. | Bounded known-hint canary, not universal secret detection or new cache-history proof. |

All13 Observe calls preserve honest partial/exit4; their useful data is not
promoted to complete. Geometry uses each source's actual css_px viewport Space.
Public Prepare is read-only and each actual Execute receives explicit trusted
caller authority plus an independently authored Expectation; no expected full
value is inferred from Type.text. UI setup never fills the positive form.
Portal opening/remount/disabled/private stimuli are separately labelled fixture
setup and not claimed as delivered product input.

## Checks, failures and tested sources

Product binaries were built exclusively from immutable git archive
260c7425d15b333b7f069e6977eded0e3b222b54, which contains W04/W03/B03/E03 and the
baseline reconciliation. Concurrent Native WIP was not consumed. Source and build
products used an own system-temp directory, not a worktree or persistent directory.

- Rust1.96.0 `cargo build --locked --offline -p uiblueprint-cli -p uiblueprint-host
  --features web --bins`: passed; no SDK/dependency change or Swift build.
- Node syntax checks for both changed harness files: passed.
- Own real Chromium145.0.7632.6, Playwright Core1.58.2, Node24.15.0, Darwin27.0.0
  arm64,800×600/DPR1; final full34-call scenario passed.32nodes/depth8/64KiB/250ms,
  256visited,120s overall and existing byte/transport limits unchanged.
- Public Diff/Check/Export are executed in that same chain; no separate synthetic
  replacement for the live pair. No Rust source changed, so unrelated logic suites
  and previous platform pilots were not rerun.
- Changed local links and exact-path diff checks: passed before checkpoint.

CLI SHA256:3f187d131b26544d514bc8bc8c68dcdfb51179dcf2129b199f87eb95539a59aa.
Worker SHA256:0d5aac77b9a8280681ae5c9de6b9bae55ac9f8293e2170a27456f2d92e703dd0.
Final scenario module SHA256:5c4c90c0a19f2a96c1f921869e3a3c6b5656e9e9ffd05a2b154edfe519237168.
The later launcher-only addition records that module hash in future run reports;
it does not change the executed scenario, source products or expected outcomes.

Initial setup rejected the bundled Playwright1.62.1 before browser launch; the
already-installed pinned1.58.2 was then used. First scenario assertion incorrectly
treated DOM native validity as ARIA invalidity; source inspection showed native
ValidityState.valid versus separately attributed AX aria-invalid. The harness
now checks the correct sourced AX property without changing product meaning.
One next full attempt stopped at selection Execute4/0stdout during severe host
load (observed load average237.65); unknown effect was never retried in that
session. The load is context, not a proven causal diagnosis. Subsequent fresh
isolated scenarios passed without relaxing limits; the last adds privacy and
paired-source preservation checks. No timing sample is a D06 pass or censored
benchmark result.

## Remaining obligations and cleanup

The W04 pre-Ready Attach classification dependency remains assigned to M02/shared
host: wrong target/old loader safe refusal must publish typed stale/resync rather
than generic exit1. W05 does not modify that owner or close full B01 semantics.
No additional Web implementation gap was found for this declared form workflow.
Independent W05 review, final coherent Native/Web P7 candidate and D06 gates remain
separate; this author check does not accept the overall product.

Every final-run CLI worker was reaped; owned browser/context/driver/server/profile
cleanup confirmed, other applications untouched. No images created or deleted.
After consuming results into this receipt, five run-owned non-image evidence
directories and the own source/build directory were removed; absence verified. The scoped checkpoint and
canonical master push run under fcntl.flock /tmp/ui-blueprint-master-git.lock;
the final chat supplies the successful SHA. No unrelated files are staged.
