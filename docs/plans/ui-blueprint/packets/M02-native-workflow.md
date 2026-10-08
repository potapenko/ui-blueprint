# M02-N — безопасная Native форма в одной живой сессии

Queued, не dispatch-ready до результатов B03-G/M03-C/E03 и исправления Replay.
Геометрический приоритет пользователя сохраняется: этот P5 task не задерживает
текущие geometry/export результаты. Объём P5/M02 уже одобрен в PLAN.UIB@1 и не
удалён прежней приоритетной паузой. Пакет готовит передачу, реализация ещё не выдана.

## Одна самостоятельная задача

После готовности — один видимый чат полного цикла: постановка/план, source/API
reconciliation, Rust+Native implementation, tests, собственный реальный fixture,
исправления, documentation, commit+push и конечный ответ. Не разделять на чаты
для Core signature, Swift glue, harness и runtime. Не создавать подагентов.
Повторного одобрения механических шагов внутри outcome нет.

Результат: рабочий Native observe → prepare → разрешённый input → независимое
наблюдение результата на форме собственного F02, в одной attached RuntimeHost
session. Чат доводит supported M02 form intents до наблюдаемого результата,
фиксируя отдельно capability, delivery, verification и честные unavailable cases.
Имеющийся Web путь и read-only Native geometry остаются рабочими.

## Требования и установленные решения

Нормативный путь root: specs/README → product/README → ACTIONS/FORMS/IDENTITY/
LIFECYCLE/CACHE/PRIVACY@1, NATIVE@2, EXCHANGE@2 и MODEL/BOUNDARIES/GEOMETRY/
PROJECTIONS closure; acceptance/NATIVE-PILOTS@1 M02/M06 + PILOTS/GOLDEN;
CLI-ACTIONS@3 (текущий Web контракт сохраняется, Native representation additive),
D02@2/D04@1/D05@4/Native acquisition@2/MEMORY@2/WORK@1 и их explicit Requires;
reference/README → executor-catalog/native-catalog/REUSE@1; RUST/DEV.RUST@2.
В момент dispatch закрепить актуальные revisions и сохранённую основу новой группы.
Root восстановил эти нормы полностью; технический design внутри уже разрешённого
Native исхода поручается исполнителю по ROADMAP, нормы он не изобретает.

Точные inputs: packets/M02-native-actions-handoff.md и соответствующий receipt;
NativeHeldAction.swift/HeldActionChecks.swift candidate3854635; существующие A01/A02
kernel и parent effect permit; текущие accepted Web Focus/Type/Activate как API
consumer evidence, не модель Native input events. Старые handoff-only/stop-for-Core
запреты относятся к предыдущему разделению владельцев, не будущему полному task.
Нельзя переносить чужой code/модальности без public-source/license проверки.

Уже принято: held Native refs живут через несколько explicit Observe/Prepare/Act
в одной attached RuntimeHost session, инвалидируются на reap/detach. Ссылки не
переживают завершённый CLI process; daemon и скрытый helper запрещены. Одна finite
transaction не подменяет эти session semantics. Конкретный bounded exchange,
residency и public invocation выбирает этот единый Rust+Native owner, записывает
техническое решение до кода и доказывает limits/lifecycle. Общие D05 ceilings и
требования cleanup не увеличивать ради pass; raw OS handles не сериализовать.

Actual e3a6880: f02.enabled AXValue CFNumber0, enabled=true, Value nonsettable,
AXPress available; sample button имеет AXPress. Поэтому AXPress нельзя выдавать
за идемпотентный SetChecked. Semantic Activate sample + fresh Count — существующий
положительный путь; поле/фокус/selection/secure behavior реализовать по реальным
capabilities каждого контрола. IME/composition только при подтверждённой поддержке;
не угадывать по конечной строке. Полная M02 acceptance требует её own positive/
negative coverage, один успешный click не закрывает форму.

Parent permit — единственная delivery authority. До Possible: отказ без ввода;
после возможного эффекта при потере certainty: unknown outcome, зависимые шаги
останавливаются, auto-retry/rollback/другая похожая цель запрещены. Target и
Expectation result удерживаются/перепроверяются независимо. Prepare read-only.
Смена владельца ввода, remount/закрытие, ambiguity и permissions ведут к нужному
refusal/revalidation. Secret values не попадают в argv/graph/логи/error/output.

## Владение при будущей выдаче

Целостная область Native action delivery: plugins/macos, Native fixtures/bridges,
Native-specific и прямо необходимые shared RuntimeHost/session/helper/effect
owners, существующий engine action kernel/provider boundary и plugin-api,
CLI Native session/action integration; соответствующие focused tests/docs и
технические spec leaves с регистрацией до semantic representation change.
Это будущая передача ownership, не право писать поверх running tasks.
Перед dispatch перечислить защищённые active owners; внутри области конкретные
файлы и план выбирает исполнитель. Web collector/provider behavior, schema/wire
и Cargo dependencies защищены; требуемый cross-domain change сначала обосновать
по существующим нормам, не строить обход или второе ядро.

## Проверка и завершение

Нужны реальные supported input/result на own F02, current focus/value/selection,
secure redaction, stale/remount/ambiguous/input-owner change refusal, forged/replayed/
wrong-session permit, cancel/deadline/EOF/detach и loss-after-Possible без retry.
К одному Target допустим bounded failure, другой не блокируется глобальным lock.
Reuse закрытые source/runtime проверки, повтор только затронутого поведения.
Shared boundary/input/privacy потребуют независимого итогового review законченного
изменения; code/test/runtime этапы внутри task не ждут отдельных root grants.

Общий execution/Git договор: current master; /tmp/ui-blueprint-master-git.lock
fcntl.flock только для scoped commit+push; CARGO_TARGET_DIR и run outputs в system
temp. Images и их каталоги никогда не удалять. Перед Native input нужен свободный
desktop lane и точная own-fixture identity; не оперировать реальными PlayPhrase.me
проектами, settings/TCC/displays или чужими процессами. Новых persistent dirs нет.
Финальный receipt: capability, сценарии/результаты, tested source, SHA+push,
ресурсы/cleanup, честные gaps. Не объявлять P0–P7 или M02 целиком без доказательств.
