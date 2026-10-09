# Q02 — фиксированные D06 gates и измеренные оптимизации

Самостоятельная задача по PLAN.UIB@1 Q02/P7, полный цикл в одном чате. Outcome:
воспроизводимый отчёт по ВСЕМ frozen D06 workloads/quality gates на согласованном
functional candidate и устранение доказанных in-scope performance defects, если
они обнаружены. Это не новый мониторинг, benchmark framework или выбор удобных
порогов. Численного выигрыша/готовности заранее не обещать.

Начать source/baseline/harness подготовку; timed серии ждут Q01 functional review
candidate и release Native/нагружающих verification resources. Это ресурсная
зависимость внутри целого task, не новый prepare-only packet. Q01 owns текущий
Native input; не оперировать одновременно. Root передаст подтверждённый release.
Не создавать подагентов/другие чаты. После собственного плана выполнять без
root approvals на внутренние шаги; coherent checkpoint+push по ходу работы.

## Основание и неизменяемые gates

AGENTS → specs/README → acceptance/PERFORMANCE@1 и decisions/D06@1 полностью,
D05@4/MEMORY@2/WORK@1/Native acquisition@2, D01@1/D02@2/D04@1/D07@5,
PILOTS/F01/F02 source/baseline handoffs и explicit Requires; RUST/DEV.RUST@2.
Current product contexts: NATIVE-SESSION@3/NATIVE-POPUP@1/D03@4, CLI@16 и
проверяемые observer/geometry/cache/serialization contracts по маршруту.
Root уже прочитал full D06; authority — его frozen P0 numbers, не новый benchmark.
Перед запуском закрепить current leaves/source pin; исходный candidatea40652a
(N03 source6ba7707), final review Q01 ещё running. Не использовать source WIP.

D06 requires минимум20 process-cold и100 warm explicit calls на selected workloads,
one caller-driven request at a time, no fixed Hz/rAF/фонового UI сбора. Warm reused
attachment получает СВЕЖИЕ requested данные; stale cache не ускорение. Cold включает
предписанный startup, stage-only timing не заменяет outer response. Off/on и
partial/full cohorts отдельны. Таймаут/failure записывается, не выбрасывается из
latency/quality; p50/nearest-rank p95, sample counts и условия явные.

Все конкретные thresholds, F01/F02 размеры/coverage/fields/версии/units и hardware
берутся из D06 и supporting actual baseline до оценки. Не уменьшать nodes/fields/
images, не менять deadlines/epsilon/threshold после failure. Источник fixture мог
получить функциональные дополнения: проверить сопоставимость по фактам, не молча
объявлять другой workload тем же. Материальную несовместимость с frozen profile
вернуть точной зависимостью/предложением, не изобретать новый более лёгкий gate.
Quality:0wrong targets/stale fallback/leaks/false pass/missing required known data.
Partial cases не закрывают положительные сценарии. Memory/SDK opaque costs и
недоступная telemetry называются, не становятся0. Запросы/bytes/resync/context costs
измерять где доступны. Не оценивать model tokens по байтам.

## Владение и workflow

Исполнитель сам выбирает минимальную воспроизводимую instrumentation/harness на
существующих APIs. Разрешены directly needed performance tests/bridges в существующих
каталогах, docs/development/performance.md и receipts/Q02-performance.md. Exact
write set объявить до правок. Tooling потребитель — этот measured gate, не future
observability subsystem. Own builds/data system temp, без permanent output dirs.

Если измерение выявило реальный product bottleneck, разрешены необходимые локальные
оптимизации owning observer/transport/host/engine/publication paths внутри исходного
поведения с исходными quotas/guard/identity/privacy/availability. Объявить exact
paths, убедиться что writer свободен; не менять schema/wire/feature policy/Cargo
или соседнее поведение без legitimate authority. Нет нового async/runtime/daemon,
скрытого cache/freshness downgrade или platform call между explicit requests.
После исправления affected correctness/consumers и новая полная сопоставимая серия,
с сохранением исходных неудачных результатов. Q01 отдельно rechecks affected risk;
самопроверка не independent acceptance. Новую инициативу за пределами выявленного
bottleneck не начинать. Если fix требует missing authority, вернуть точный scope.

Во время Q01 source/runtime review production сохраняется; source optimizations
только после его передачи writer/lane, чтобы не создавать конкурирующих изменений.
CPU/load и параллельную активность учитывать в отчёте. Не убивать/останавливать
чужие процессы и не менять power/display/TCC/системные настройки. Не смешивать
наш активный stress/compile с якобы idle cohort; timings начинаются после setup.
Реальные PlayPhrase.me проекты не оперировать; только свои fixed fixtures.

## Доставка результата

Current master/no branch/worktree; fcntl.flock /tmp/ui-blueprint-master-git.lock
только на short exact-path commit+push canonical remote, без grants.
Все images/staging/их каталоги в system temp остаются навсегда со стороны агента;
cleanup только own consumed nonimages с проверкой. Минимальные raw sample данные
можно удержать для named Q01/root consumer в system temp до потребления; никаких
коммитов сырых logs/изображений или нового persistent archive.

Final: source/recipe/fixture/environment pins, workload→expected gate→actual
p50/p95/counts/quality/memory/bytes/calls, failures без censoring, профилируемый
bottleneck/исправления/affected tests если были, limitations, SHA+push и resources.
Не выдавать exploratory single samples, raw SDK-only timing или другой workload
за D06 pass. Если обязательный gate fails, он остаётся открытым до достаточного
исправления/полномочий; не переопределять исходную цель P0–P7.

## Scoped Web release while Native foreground waits

Root scheduling update: Q01 independently passed Web functional on762f244;
`git diff --name-only 762f244..8e3dba2 -- crates/host/src crates/plugin-api/src
plugins/web/src crates/cli/src crates/schema/src crates/engine/src` is empty.
N03/repair affects Native Swift. Therefore Web quality/comparability preflight and
only matching frozen Web cohorts may run on8e3dba2 now. Q01 has been asked to avoid
concurrent heavy checks. This is a scoped domain release, not accepted Native or
whole D06. Unsupported full-fixture iframe/semantic rows remain explicit gaps;
no single-control substitution. Native still awaits Q01/operator ownership and
functional release; production timing optimizations follow the original packet.

## Complete the remaining Rust-stage report

The full Web numeric campaign on accepted a7c0416 now passes all four frozen
thresholds. Preserve these measured cohorts and their original source pins.
D06@1 separately requires timing the added Rust normalization/formatting; the
current receipt establishes that the existing guarded API does not expose those
intervals. That is an observed evidence gap, not permission to mark the requirement
complete or to invent a subtraction-based timing.

Continue the same full Q02 outcome with the smallest test-only measurement of the
actual owning Rust routines on pinned saved inputs. The existing Q02 harness/
bridge/recipe/receipt ownership covers this work. Choose the method after inspecting
its real source boundaries; do not duplicate the implementation as a toy benchmark.
If narrowly placed timing hooks in a system-temp source copy are necessary, preserve
exact pin plus the minimal instrumentation delta and label its results diagnostic;
it must not be reported as a new unmodified production latency cohort. Keep product
source, public formats, features/dependencies and the installed candidate unchanged.
No runtime telemetry service, profiler framework, source refactor or new threshold.

Report actual separately measured intervals and what they cover, including whether
they are exclusive or nested. Reuse accepted outer-response and unrelated quality
proof; rerun only what the instrumentation actually affects. Other SDK/internal
counters remain explicitly unavailable where D06 allows that; do not expand this
into a new observability project. If the required boundary cannot be measured under
these protections, return the exact source-backed necessary owner change rather than
a generic unavailability statement. Native foreground/runtime waits stay separate.
Complete harness, measurements, documentation, scoped commit+push and final result
without additional root grants for internal steps; CPU resources are now free.

## Independent Native read-only comparability continuation — 2026-10-09

Scope clarification grounded in NATIVE@2.CONTENT/AX-READ and the saved Q01 N03
foreground receipt: the pending operator activation is a runtime precondition for
input/whole popup-action composition, not a fresh implementation approval. Ordinary
one-shot AX/capture is independent of resident form/input ownership. The prior
blanket stop on every Native preflight was broader than those requirements.

After Q01 source/recorded acceptance of the N04 repair and CPU/resource release,
continue the existing Q02 task with a bounded read-only Native comparability
preflight against ONLY Q01's already running owned F02 instance, if still available.
Use Q01's trusted retained handoff at system temp uib-q01-final-5w3j1hkw as provenance,
not as proof the old PID68614/window/generation is still current. Validate exact
process incarnation and current own identity/window metadata before addressing it.
No title/geometry search, target substitution or old action refs. Missing/stale
ownership returns that exact condition; no replacement app launch or prompt.

Use current accepted helper/host and explicit bounded window-only AX/capture scopes,
requested fields and existing owned-synthetic pixel policy. Do not perform Focus,
Prepare/Execute, Compare/Snapshot setup, resize, reset, close, activation, input,
application launch or any TCC/display change. Do not alter or retire Q01's retained
fixture/resources. No actual PlayPhrase.me app or historical prepared off/on bundle
may be operated in this continuation. All task images remain in system temp without
agent cleanup. Read-only state/focus/geometry invariance must be checked.

The complete outcome is current source-fact preservation and actual original
field/value/tree/window/pixel comparability evidence, or a precise observed mismatch
or availability condition. N04's recorded mapping and pinned fixture proposal are
inputs, not live equivalence. Preserve every required field/node and explain extra
controls; never lower coverage, ignore state differences or replace frozen numbers.
If fixture setup/mutation is needed, keep that slice waiting for the pending Native
condition and return the exact needed setup, not an automatic workaround.

This authorizes the independent preflight, NOT unconditionally the Native timed
cohorts or N03 actions. Q01's ordinary-collection functional acceptance and matched
live comparison are still required before Native performance acceptance. Continue
only already-ready existing-task work; retain the still-open off/on/capture/cold
rows rather than narrowing the goal. Read relevant Apple/QA/runtime routes before
operations. Save the complete scoped result with checks, pins, invariance/cleanup,
commit+push and exact remaining dependency, without per-command root grants.

## Controlled request-only Native input comparison — 2026-10-09

Q01 terminal21e0abf independently accepts the bounded whole N03 functional chain
on current7709067 by explicit source-delta/evidence reuse. Repeating it or a separate
Focus command is not a new acceptance requirement. New mutations still require
fresh input ownership. The old operator activation question is not being answered
or used as permission for this task.

Authority for this distinct continuation is the original approved PLAN.UIB@1 P0/P7
own-fixture setup/Native Q02 execution, and ROADMAP's delegated engineering choices.
Root explicitly assigns Q02 the Native fixture/UI resource for this finite outcome.
Prior no-launch/no-setup restrictions governed only the completed retained-instance
read-only preflight; this paragraph replaces them ONLY within the scope below.
Real applications, TCC/displays, global installation, input-owner bypasses and new
branches/worktrees remain forbidden. Read applicable runtime/Apple/Computer Use
routes before operation; use allowed established setup methods, no new input backend.

Complete one autonomous outcome: qualify the already prepared historical
explicit-request53e6e6e F02 off/on as a Native D06 comparison input, or return its
exact measured incompatibility and a concrete justified workload proposal. This is
not a benchmark framework or a request to repeat Web/Rust-stage tests. Read the
Mac coauthor consultation in receipts/platform-test-advice.md and21e0abf's remaining
D06 criteria. Original reactive d33eac88 is historical evidence, not the collector
or fixture execution model to restore.

Resource handoff: Q02 may retire ONLY Q01's task-created retained F02-on process
under uib-q01-final-5w3j1hkw after fresh exact executable/PID/incarnation validation
against the trusted Q01 handoff (historical68614). Preserve every retained file and
image/containing directory; do not remove that shared tree or touch any foreign
process. This transfers lifecycle ownership of that synthetic process, not its old
refs or permission to interact with other apps. If identity is uncertain, stop that
operation and report it. No new input to this historical instance is needed.

Use N04's pinned source/bundles/provenance or reproduce them by its existing recipe
in task-owned system temp. Never modify/delete controls in current Fixture.swift,
use old observation IDs as live authority, launch a real project, or silently make
another input. Run the off/on fixture instances serially, one exact owned target
at a time; own launch/normal setup/cleanup are authorized. Establish the documented
expanded/Count1/name-empty/unchecked/Result-none/scroll-top state and actual window/
focus context with existing fixture controls. Every necessary action requires fresh
exact process/window/input ownership and its own allowed modality. Unknown delivery
or lost ownership stops dependent input; do not automatically retry or bypass it.
If setup cannot establish the needed state, report the exact precondition instead
of substituting data or claiming the old question was answered.

Before ANY candidate timing: compare each original known/unavailable fact, all
requested fields/actions/edges, layout/placement and context, natural550×525pt/
1100×1050px,160/depth9 and unchanged budgets. Keep and explain every extra node and
explicit-request cost. A75-node equality trick, field omission, proxy observer,
clipping/resizing input to fit, or changed epsilon/latency ceiling is forbidden.
Use current accepted7709067 shipping host/helper and independent source witnesses;
keep exact source/fixture/config/environment pins and no hidden polling.

Do not launch Native timed cohorts yet. Return the complete preflight/baseline input
result to the D06 owner: either demonstrated comparability under D06@1 with exact
proof, or the exact remaining mismatch and a concrete versioned workload proposal
(full source/layout/context/coverage/quality and same numeric gates) to be resolved
legitimately BEFORE evaluation. Do not edit D06/.PROOF or label new-workload success
as the old comparison inside this task. Original records, gates and all open Native
performance requirements remain. Q03's real-case authority question is separate.

Scope stays Q02's owned performance driver/bridges/recipe/receipt; product/fixture/
shared contracts and other owners are protected. Plan, implement the necessary
bounded driver adjustments, complete safe setup/preflight, record quality and
limitations, preserve images, clean only own consumed nonimages, commit+push and
return ONE terminal result/resource handoff. No internal-step root grants needed.

## Preserve original context before choosing a revised workload

Controlled017d85c proves complete field preservation off/on but not equivalence:
76nodes/extra Snapshot/OpenB shift plus active/key/main differs from the original
inactive/nonkey/nonmain. Root does not accept the proposed active context merely
for easier setup. Active input ownership for setup and inactive read-only collection
are distinct; the latter has already worked in the actual retained Native case.

Continue the same qualification using original F02 setup evidence and existing
allowed UI controls to establish the original context AFTER safe setup, if possible.
Keep all state/Name focus/fields/layout/window/pixels and extras; no fixture edits,
new backend, permission/display change or data reduction. Normal foreground return
to the application that was frontmost immediately before this own setup is allowed
only after exact PID/incarnation validation, without inspecting or otherwise
interacting with its content. No other real application operation is authorized.
If the required context cannot be established safely, return the concrete observed
precondition or conflict; do not manufacture a easier substitute or loop blindly.

One complete off/on comparison in the matched context, or that exact blocker, then
update the concrete workload proposal and save it. The real request-only structural
delta remains explicit. No candidate timing, D06/.PROOF change, new threshold, N03
rerun or unchanged Web/Rust-stage campaign is authorized by this continuation.

## Full Native benchmark continuation — user authorization 2026-10-09

This section supersedes the interim preflight-only/no-timing/fixtures-only and
manual activation restrictions above. User explicitly said to do the benchmark
and launch whatever applications are needed; exact quote is in execution.md.
Resume the original complete Q02 outcome, not another preparatory microtask.
Q01 accepted bounded Native functional composition in21e0abf and source7709067;
I02 qualified combined063e709 inef45577. All old own fixture processes are retired.
You hold Native desktop/quiet CPU now; other tasks prepare without UI/load until
release. Validate fresh ownership for new operations; old PIDs/refs are provenance.

Contract requirement: explicit requests, all data/cost, unchanged100/200/300/750ms
Native gates,20cold/100warm per off/on and full quality reporting.
Observed evidence: prepared53e6e6e request-only input adds Snapshot, shifts OpenB,
and controlled017d85c has76nodes with all original fields retained. These are real
input differences, not a proven old-baseline match. Prior inactive attempt063e709
failed to establish context, not a measured performance failure.
Selected engineering route under ROADMAP D06 and the user's instruction: separately
version this complete request-only workload BEFORE candidate evaluation, with exact
source/state/context/geometry/all fields and same thresholds; obtain a fresh direct
API baseline for exactly it, then measure the candidate. Preserve historical75-node
records and distinguish the new comparison. Use reproducible supported setup; active
context is allowed when explicitly pinned equally in baseline/candidate. Do not
spend another cycle forcing the old inactive context or disguising differences.

Reconcile mode: permitted contract delta is only this honest workload registration
and its acquisition.PROOF reference; same numerical gates, coverage preservation,
privacy/limits/identity/freshness/observer-invariance remain protected. In addition
to original ownership you own docs/specs/development/decisions/d06-performance.md,
its necessary evidence reference, d05-native-acquisition.md's baseline reference,
and docs/specs/README.md's corresponding registration/revision entry. Pin the exact
closure through current registry/decision routes, distinguish registration from
acceptance, checkpoint registration before candidate timing. Do not change unrelated
clauses or requirements. Original75 equivalence is not claimed by passing new input.

Complete setup, baseline, full off/on cohorts, affected self-checks, evidence and
any necessary in-scope repair under the original packet. Reuse accepted Web/Rust
results; no repeated full campaign unless their inputs change. A measured failure
stays visible. No new framework, fixture reshaping, field pruning or loosened gate.
Return one terminal result with counts/p50/p95/quality, source/conditions, actual
limitations, commit+push and CPU/desktop release. No nested agents or chats.

## Session-local AX process reuse boundary — 2026-10-09

Q02 observed both public CG lookups at similar30–45ms cold cost; source replacement
was not justified. Proposed next internal optimization: reuse the already existing
session-owned resident non-capture helper machinery for explicit read-only Observe,
with a separate one-shot capture helper. This is a proposal, not a performance pass.
Root reread D02@2, WORK@1, NATIVE-SESSION@3 and NATIVE@2 before this clarification.

Existing D02.CONTENT permits multiple explicit requests, no idle collection and
process ownership by session. WORK.PROFILE permits two registered Native streams/
helpers per session. The original Q02 outcome therefore permits this internal reuse
when the selected implementation preserves those contracts. It does not authorize
a new daemon, snapshot/cache layer, public CLI/schema/permissions or larger quotas.
NATIVE-SESSION@3's held form objects/action refs and1..8identifier profile remain
form-only; reuse of its machinery cannot silently broaden that public contract.
NATIVE.AX-READ's selected AX object stays Observe-local, with observation-scoped
Surface/refs and current owner validation before/after. Process reuse is not data,
handle, permission or identity reuse. All clocks/source attribution remain truthful.

Q02 remains sole source writer under the original observer/transport/host/publication
scope; declare exact paths/plan before edits and preserve adjacent form/PROTECTED,
one-shot, channel capabilities, cancellation/EOF/expiry, two-helper/ingress budgets,
ACK-before-capture and capture lease-until-reap. No permission gate for internal
steps. A demonstrated need to change a protected public/semantic contract is an
exact dependency, not implied authority. Coherent saved delta requires focused
independent Q01 review for changed lifetime/binding risks; unchanged work stays closed.
All original quality failures, full workload and numeric D06 gates remain mandatory.

## Remaining-phase ownership handoff — 2026-10-09

Source e641543 and focused Q01 acceptance9675c0b are saved; I02 current installed
qualification6ff4943 and Q03b014cd4 are complete. Root transfers remaining Native
measurement/quality execution to fresh visible task01a12127-5b94-7801-95cd-c35cf2c3ceab
under [Q02-R packet](Q02-native-final-execution.md), preserving the full P0–P7 goal.
Old Q02 must stop new source/runtime/timed work and finish/retain its exact state.
It is still treated active until actual terminal status; timeout is not a failure.
The new worker first stops ONLY the old Q02 turn through supported Codex UI if
necessary, verifies terminal/resource ownership and preserves all work. No competing
execution before that gate, no branch/worktree or process-kill workaround. No goals
are paused/cleared. Required gates and historical failures remain unchanged.
