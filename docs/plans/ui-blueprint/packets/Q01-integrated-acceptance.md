# Q01/V01 — интегрированная функциональная и privacy приёмка

Ready: M02-N terminal completed, candidate6a5bec2/final94724df pushed.
Product pin94724df (полный SHA разрешить через Git до запуска), registry29/CLI@16,
NATIVE-SESSION@1 прочитан root; остальные нормы/closure сохранены.
Одна самостоятельная verification-задача: собственный план проверки, выполнение,
полный отчёт по исходным обязательствам и точные repair dependencies. Не отдельные
чаты на команды/fixtures. User-authorized P0–P7 и отдельные чаты полного цикла;
root не выполняет QA. Model/reasoning inherit; подагенты и новые чаты запрещены.

## Результат

На одной согласованной сохранённой версии проверить все M01–M06/B01–B06,
GOLDEN01, оба E2E, privacy canary, model-free export/поставку/recovery и границы
идентичности/lifecycle. Сформировать requirement → implementation owner → scenario
→ independent expectation → actual result → evidence → limitation. Ни наличие
кода, ни old receipt/зелёный частный тест не закрывают full criterion.
D06/Q02 статистические измерения и Q03 agent usefulness — отдельные tasks после
функционального candidate; эта задача не меняет их thresholds и не заявляет pass.

## Нормативное основание

AGENTS → docs/specs/README → acceptance/README → COMPLETION/PILOTS/
NATIVE-PILOTS/WEB-PILOTS/GOLDEN/PERFORMANCE@1 и explicit Requires; product
ACTIONS/FORMS/IDENTITY/LIFECYCLE/CACHE/PRIVACY/GEOMETRY/PROJECTIONS/MODEL/
BOUNDARIES@1, EXCHANGE@2, NATIVE@2, EXPORT полный DRAWING closure;
CLI@16/CLI-ACTIONS@3/CLI-EXPORT@2/G13/G12/ANALYSIS@2 по проверяемым путям;
NATIVE-SESSION@1 и его closure; D01@1,D02@2,D03@3,D04@1,D05@4/MEMORY@2/
WORK@1/Native acquisition@2/D06@1/D07@5, RUST/DEV.RUST@2 и explicit deps.
Перед dispatch revalidate фактически сохранённые revisions и source pin.
Основное намерение — UIB.TZ@1.4/UIB.DRAWING@1.1 и PLAN.UIB@1, не новый checklist.

Узкая implementation leaf или выбранный unsupported не сокращает исходные
положительные обязательства. Различать допустимую границу платформы/модальности
от отсутствующей обязательной capability. Например Semantic setter не доказывает
Keyboard/IME; безопасный отказ в действии не доказывает positive form result.
Проверить фактический смысл masked/secure field и checkbox требований M02 и
privacy-canary input/error/cancel/cache/history/prompt/trace, а не объявлять их
закрытыми только из-за Unsupported. Не изобретать требования сверх ТЗ.

## Работа и evidence

Использовать существующие собственные fixtures и авторские harnesses как инструменты,
но проверять их expectations/coverage по нормам и независимым фактам. Продуктовую
доставку не подменять прямым Playwright/AX setup. Состояния и setup явно отделять.
Не запускать/изменять реальные PlayPhrase.me проекты. Перед desktop input подтвердить свежую доступность и владение lane: M02 сообщил
посторонний ввод ПОСЛЕ завершённых ACKed сценариев, остановил дальнейший input
и очистил собственный fixture. Не продолжать старую UI-сессию и не обходить
явный tool/user handoff. Новые действия только после восстановления разрешённой
ownership/начального состояния; при требуемом подтверждении вернуть точный blocker.
Номинальная Native lane после M02 освобождена; Web можно адресовать собственным headless.

Особо восстановить целые цепочки: Native точное одноимённое окно → popup →
measurement/input/result → resize/local diff → stale old popup → compare,
плюс M05 actual internal measurements и неизменность off/on. Web exact target →
partial draft/validation/autocomplete → applied → portal/remount/stop → diff/check
→ compare. Переиспользовать достаточные unchanged proofs, но финальную композицию
на declared revision не заменять разрозненными module receipts.

Проверить typed pre-Ready Attach refusal dependency W04/M02; unknown effects без
retry; независимость второго Target; partial fields и event-loss resync; full/delta
oracle одного source checkpoint, не equality двух разных live snapshots.
Privacy-canary — все предусмотренные сериализуемые каналы, pixels отдельно;
не считать disabled/unexecuted trace или непроверенный путь автоматически passed.
Model-free обычные команды и export проверяются без API keys/модели; пакет
согласованных binaries собирается existing distribution.py из того же source pin.
Проверить current tests без skip прежнего E03 baseline (reconciledc6b2357).

## Независимость и границы

Production source/specs/manifests не менять. Разрешены необходимые integration
harness/tests в заранее объявленных существующих каталогах, собственный
receipts/Q01-integrated-acceptance.md; historical fixtures/baselines не переписывать.
Настоящие implementation defects вернуть сразу с exact reproducer/owner, продолжая
независимые проверки. Root передаст их owner, не создаёт конкурирующего implementer.
Самостоятельность задачи означает доведение всей verification scope, не разрешение
выдавать исправленную самим reviewer реализацию за независимую приёмку.

Для нового обязательного source review: сначала собственное наблюдение saved
artifacts и criteria coverage, не читать авторский verdict; затем root передаст
author receipts для reconciliation тому же чату. Старые принятые source reviews
использовать по неизменённым доменам, не повторять всё без причины.
Source, author runtime, собственный runtime и remaining gaps разделять.

## Ресурсы, сохранение, финал

Текущая master без branch/worktree; только coherent saved input, без чужого WIP.
Shared fcntl.flock /tmp/ui-blueprint-master-git.lock на exact-path commit+push,
без root grant. Own source/build/run files system temp. Все images/их каталоги
никогда не удалять, показывать необходимые изображения inline. Nonimage outputs
сохранять лишь до названного immediate consumer; не создавать постоянный archive.

Для последующего Q03 можно удержать минимальные original canonical responses и
соответствующие image references выбранных controlled complex cases в own system
temp, с точными pin/environment/scope и named Q03 consumer. Это не новые замеры
реального приложения и не источник новых product expectations. После потребления
удалить только свои nonimages; images/каталоги сохраняются. Не восстанавливать
удалённые observations из чисел прошлых receipts.

Финал: отдельное решение по каждому обязательному критерию и источнику evidence,
все failures/неточные claims/ограничения, source/harness commits и push, ресурсы.
Если задача неполна, точная оставшаяся проверка/внешняя зависимость, а не общий
«всё работает». P0–P7 завершает root только после Q02/Q03 и всех оставшихся gates.

## Dispatch context

Новая обязательная source review в этой задаче касается M02 commits6a5bec2/94724df
и W04d3f4723 (декларативные component mappings), включая актуальную composition.
Предыдущие source/safety review groups уже закрыты по unchanged graph/export/
clipping/capture/distribution; не повторять их без изменённого consumer/риска.
До своей initial source assessment не читать author receipts этих двух изменений.
После initial observations root передаст их для reconciliation в этом же чате;
независимые проверки собственного источника/тестов продолжать по правилам.
Task owns integration tests/harness/receipt, не product code. Если W04 harness
ещё ожидает прежний generic Attach exit1, согласовать его с уже принятой typed
семантикой исправленного host; это correction устаревшего теста, не изменение нормы.
I02 параллельно владеет только distribution recipe/docs/packaging checks, без UI.
Не потреблять его незавершённые файлы; pin actual recipe при проверке установки.

## Native release quality diagnostic — finite parallel task, 2026-10-09

Classification: diagnostic. One outcome: determine precisely what the recorded
release-cohort quality failures prove under the unchanged Native quality contract,
and identify the smallest actual repair or missing-evidence dependency. Immediate
consumer: Q02 full performance outcome and root's final quality acceptance. This
is not another whole-source review, new runtime campaign or acceptance of timings.
User-authorized parallel chats under PLAN.UIB@1; root remains coordination-only.
No agents/nested chats, app operation, build, live API call or product/spec edit.
Model/reasoning inherit this existing chat. Q02 remains sole product writer.

Ready evidence, immutable original source770906798e1326fed3dd0edec40dc59b13772b3a:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-q02-native-full-lh3jw_lj/q01-quality-handoff.json`.
It lists exact release off/on baseline/candidate reports, hashes, individual sample
files/artifact directories, before/after raw witnesses, source/binary/fixture/config
pins and retention. Driver assertion output is data, not an authoritative verdict.
Inspect these actual records first; do not start from Q02 receipt/narrative or infer
cause from earlier anonymous-group observations. Source scopes are actual Native
AX mapping/publication and witness/quality predicate owners needed to explain these
records (pinned tests/bridges/native/WindowAX.swift, current ReadonlyFacts.swift,
cohorts.cjs and their directly relevant native dependencies). No broad source audit.

Spec Basis: registry33 → decision D06@2 plus D06-NATIVE-REQUEST@1.INPUT/METHOD/QUALITY,
D05-NATIVE-ACQUISITION@3.PROOF/OUTCOMES and explicit dependencies; PERFORMANCE@1,
NATIVE@2, MODEL/EXCHANGE@2/GEOMETRY/IDENTITY/PRIVACY and already-read applicable
Q01 closure. Known source versus unknown/unsupported, partial coverage, all requested
facts, full input/cost, read-only state and denominator/outlier requirements remain.
No changed threshold, field pruning, extra tolerance, silent quota relief, retry-to-pass,
reclassification of mandatory failure as a residual or fabricated same-time oracle.

Compare exact failed versus appropriate successful source/canonical records, keeping
collection times/clocks and baseline/candidate formats separate. Distinguish:
- demonstrated lost/misreported requested source facts or wrong ownership;
- actual explicit API/availability failure and its stated meaning;
- representation/state differences between separate observations, with causal limits;
- a driver assertion stricter/different than an actual contract criterion, if proven;
- missing evidence that prevents choosing among those explanations.
A raw witness at another time is not automatically a same-state ground truth. A
baseline failure does not excuse candidate failure. A count difference alone does
not establish either collector loss or benign source variability. Preserve complete
samples/denominators; a quality diagnosis cannot grant latency acceptance.

Economy basis: existing exact failed samples plus adjacent proof, no repeat live
run or new framework. Read-only lightweight file/source comparisons may run beside
Q02 offline optimization; no heavy build/stress/load. If a new live observation is
truly necessary, return the exact unanswered question/minimum evidence, not a broad
request to redo Native. Keep accepted unrelated Q01/Web/Q03 work closed.

Write only a distinct diagnostic section in receipts/Q01-integrated-acceptance.md.
Return one final compact result: per observed failure, contract criterion, what the
original bytes/source establish, what remains unproved, exact affected owner and
minimal next dependency. Do not label this diagnostic a fresh independent source
acceptance or full Q01/P7 pass. Ordinary new product deltas later retain their own
required focused review. Check changed links/whitespace, scoped commit+push under
shared Git lock; no root grant. Do not delete/rewrite any shared evidence or images.

## Focused AX SDK-reuse source acceptance — e641543, 2026-10-09

Classification verification. Finite outcome: independent acceptance or exact blocking
findings for the changed Native helper lifetime/binding implementation in immutable
`e64154349bc93e9c7a4a91bab7698cef84b8eb7a` against parent70c3ddb. Source and canonical
remote master were verified by root. Q02 is author/sole product writer; this Q01
reviewer is non-author. No nested delegation, new chats, new criteria or old-suite
requalification. Immediate consumer: final Q02 measured candidate and local delivery.
Economy basis: inspect the actual small changed boundary, reuse unchanged accepted
source evidence, execute only checks needed for its demonstrated risks.

Authority/Spec Basis: existing approved PLAN.UIB@1 plus D02@2.CONTENT/WORKER/
PUBLICATION/LIFECYCLE, D05-WORK@1 PROFILE/GUARD/SUPERVISOR/PROOF and its explicit
MEMORY@2/lifecycle/privacy/D07 closure; Native acquisition@3 admission/ownership/
outcomes; NATIVE@2, NATIVE-SESSION@3 (popup/PROTECTED only affected adjacency),
EXCHANGE@2/IDENTITY/PRIVACY and current RUST/DEV.RUST@2 for actual checks.
D06@2/request-only@1 applies to unchanged quality/latency obligations, not a claim
that those gates have passed. Root accepted internal process reuse only inside
these existing semantics, in coordination checkpoint70c3ddb: session-owned process
reuse, explicit fresh requests, no idle collection/result cache or public form/API
expansion, Observe-local AX objects/refs, separate capture isolation and all caps.
No changing contracts or grandfathering an implementation departure in this review.

Initial independent observation: inspect ONLY code/test changes and their necessary
callers/dependencies, not Q02 receipt, performance.md, builder chat or this task's
registry success narrative. Initial paths:
- crates/host/src/native_broker.rs and supervisor.rs;
- plugins/macos/HostHelper.swift and HostProtocol.swift;
- crates/host/tests/performance.rs and tests/bridges/native/host_helper/ReadonlyProtocolChecks.swift;
- directly affected existing binding/worker/collector/lifecycle/stream tests as needed.
Other test diagnostic files in the commit are supporting artifacts, not performance
acceptance; inspect only when needed. No broad source audit, formatting findings or
fresh review of unchanged Web/geometry/export/M05/action implementations.

Mandatory review dimensions: exact current identity/permission/source correlation;
observation-local objects/frames and per-call freshness; read-only phase/channel/
collection admission and no action/form/PROTECTED expansion; cross-channel binding
and reconfiguration authority/scope; original canonical values/limits; parent fixed
storage/precharge and bounded repeated operation; deadline/expiry/cancel/EOF/reap,
late response/ACK rejection, independent Target and capture lease-until-reap;
AX publication ACK before capture and unchanged legacy/ordinary/form consumers.
Production code stays read-only. Any material finding names criterion, actual
scenario/evidence, affected owner and minimum repair/recheck. No speculative verdict.

Two-stage independent protocol: first record and return initial observations and
criterion coverage BEFORE consulting builder narrative; this is intermediate,
not final acceptance. Root will then supply the matching builder receipt/evidence
to this SAME context. Reconcile those artifacts and issue ONE scoped final verdict
with actual independent versus author evidence, exact remaining gaps, commit+push.
Do not turn author tests into your own execution or infer runtime proof from a build.
Missing proof is not a demonstrated defect, but cannot be labelled accepted.

CPU is available for focused checks before Q02 starts its next timed campaign;
root coordinates that resource. No UI/SDK/app operation is assigned initially.
If concrete risk truly needs fresh live evidence beyond retained artifacts, state
the minimum operation/consumer first instead of rerunning Native/N03 by default.
Source/build/test nonimages use system temp; shared images/evidence are not deleted.
Only writable repository path: receipts/Q01-integrated-acceptance.md. Current master,
exact-path commit+push/shared Git lock; preserve all other writers and untracked PNG.
Model/reasoning inherit. Finish this one coherent acceptance task; no per-test grants.
