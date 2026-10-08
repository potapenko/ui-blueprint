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
