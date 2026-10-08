# Самостоятельные задачи следующей группы

Основание: PLAN.UIB@1 в 358c757e7eab84a3989d150dbad57924d866601a,
прямое требование пользователя о самостоятельных чатах-задачах и активная цель.
Результаты ниже — части одобренного P0–P7, а не новые продуктовые требования.
Режим Restore существующего ТЗ; техническое представление, где оно ещё открыто,
выбирает исполнитель по делегированным ROADMAP решениям, фиксируя до кода.
Классификация каждой задачи: shipping_product. Модель/мышление: inherit.

## Общий договор

Один чат выполняет одну указанную ниже задачу целиком: читает требования и
существующую реализацию, записывает собственную постановку и короткий план,
реализует, проверяет, исправляет, документирует, сохраняет commit+push и возвращает
финальный результат. Немедленная реализация внутри назначенного outcome разрешена;
после локального плана нового согласования root/пользователя не требуется.
Старые source-only/prepare-only/runtime-activation/stop-for-save указания в
исторических packets не ограничивают этот полный цикл. Исторические protected
границы сохраняются. Не запрашивать очередное поручение после обычного этапа.

Root не пишет код, не определяет алгоритм и не проводит QA. Внутренних подагентов
не создавать. Не начинать соседние задачи, не расширять ТЗ, не обходить
неизвестность или отказ разрешений. При материальном конфликте/чужом owner вернуть
точную зависимость, продолжив независимую часть. Финальный ответ читается root
через статус чата; отдельная отправка сообщения другому чату не нужна.

Один checkout master. В task scope входят необходимые тесты и документация;
точный набор файлов уточнить в собственном плане до правок. Запрещены новые
ветки/worktrees, Cargo/dependency обновления, изменение чужих задач, реальных
проектов PlayPhrase.me и их запуск. Новые файлы допустимы в существующих каталогах;
новые постоянные каталоги не создавать. Системный temp для build/run outputs,
изображения и содержащие их каталоги никогда не удалять. Чужой
`after-title-spacing.png` не трогать и не включать в коммиты.

### Git и ресурсы без промежуточного одобрения

Вся группа, включая root, использует один временный lock-файл
`/tmp/ui-blueprint-master-git.lock`: Python fcntl.flock LOCK_EX на открытом файле,
удерживаемый одним процессом от проверки index до завершения commit+push.
Использовать неблокирующий захват; при занятости продолжать независимую работу.
Не удалять/заменять файл между сохранениями; root удалит его после завершения
всей группы. Не использовать этот lock для долгих сборок или всей разработки.
Пустой index проверяется уже под lock; чужой staging не трогать. Stage exact own
paths, commit coherent steps, push текущей master в canonical
`git@github.com:potapenko/ui-blueprint.git`, без force/history rewrite.
Нет отдельного root grant. При ошибке push не объявлять задачу завершённой.

У каждого чата собственный CARGO_TARGET_DIR в system temp. Не запускать общий
workspace fmt с записью. Shared manifests/schema/plugin-api защищены в этой группе.
Native имеет единственную desktop lane для своего fixture; Web использует только
свой отдельно адресованный headless context; Graph не запускает UI. Сохранять
точные owner/process identities и завершать только собственные ресурсы.

### Spec Basis и входы

Root traversal: AGENTS → specs/README (registry26) → product/README → выбранные
листья; acceptance/README → соответствующие pilots; development/decisions/README.
Полностью восстановлен общий closure: BOUNDARIES/MODEL/IDENTITY/GEOMETRY/
PROJECTIONS/FORMS/ACTIONS/LIFECYCLE/CACHE/PRIVACY@1, EXCHANGE@2, NATIVE@2,
CLI@13, CLI-DIFF@2, CLI-GEOMETRY-DIFF@1, ANALYSIS@2 с TYPES/VALIDATION@1,
PILOTS/WEB-PILOTS/NATIVE-PILOTS/GOLDEN/PERFORMANCE/ROADMAP/RUST-BOUNDARIES@1.
Пути этих листьев находятся в docs/specs/product и docs/specs/acceptance;
stable clauses CONTENT, плюс явно названные clauses у CLI/analysis.
Decision closure: D01@1,D02@2,D03@3,D04@1,D05@4,MEMORY@2,WORK@1,
Native acquisition@2,D06@1,D07@5,EVIDENCE@1; RUST и DEV.RUST@2.
Каждый чат читает только применимые своему outcome листья и их explicit Requires
полностью до исходников, уточняет receipt и текущие revisions. Старые ссылки @1
на EXCHANGE/NATIVE/CLI навигационные: текущие зарегистрированные версии выше.

Protected: core0.1/analysis0.2, исходные Snapshots/Observation/Evidence/clock domains,
unknown/redacted/empty, bounded acquisition/publication, identity/permissions,
позитивные критерии и frozen D06. Нет polling, автоматического сбора на событии,
нового daemon/model/framework или догадок о geometry/identity.
Export/mobile/future phases и новые actions исключены из этой группы.
Обычные нужные tests/runtime/checkpoints входят в задачу. Проверки выбирать по
риску изменения; unaffected accepted checks не повторять. Обязательное независимое
review помечать отдельным acceptance gap, не называть самопроверку независимой;
не останавливать код/свои тесты ради промежуточного review каждого шага.

## W03-R — Web: потеря событий, bounded resync и full/delta

Outcome: закончить Web-часть W03/B05: после изменения родителя/шрифта, потери
событий и ограниченного ответа следующий явный запрос корректно восстанавливает
актуальное состояние; подтверждена эквивалентность full/delta одного исходного
checkpoint. Исторический cache не становится свежим и не удаляет ненаблюдавшиеся
узлы. Другой Target продолжает работать, detach освобождает подписки/очередь.

Входы: packets/W03-session-invalidation.md и receipts/W03-session-invalidation.md,
receipts/W03-cache-provider.md. Уже есть Web24ed0e8/Core74d2e3b, peer3318662,
actual2085e065: parent/font данные получены, но фактическая loss/full-delta часть
ещё не принята. Не повторять завершённую часть вместо закрытия оставшейся.
Reuse существующий CacheStore/CanonicalSession/реальный worker; no parallel graph.

Владение: plugins/web; crates/host/src/worker_web.rs и worker_ops.rs;
crates/engine/src/cache и связанные cache tests; существующие Web host tests,
tests/bridges/web, tests/fixtures/web; docs/development/cache.md, web-cdp.md,
web-collector.md; собственный receipts/W03-resync-complete.md.
Core Graph не пишет cache, Native не пишет worker_ops. Shared schema/plugin-api,
общий host protocol/allocator, CLI и manifests закрыты; реальную зависимость
назвать, не строить обход. API уже существующих владельцев изучить самому.

Acceptance: реальные scoped Web requests через production path; loss/refusal →
bounded explicit recovery без autoobserve; unchanged historical values/times;
исключение cross-target invalidation; честная partial coverage; full/delta oracle
одного controlled source-state, включая unknown/redacted/empty, lost base и scope
mismatch. Синтетическую loss-инъекцию отличать от actual transport/event loss.
Команды/сценарии выбирает чат, готовность подтверждает итоговым receipt+SHA/push.
Immediate consumer: K02/P4 и безопасная свежесть последующих P5 запросов.

## M04-T — Native: проверенные локальные координаты

Outcome: завершить полезный M04 путь выбранный native компонент → sourced
преобразование в локальное пространство → Rust measure и diff при move/scroll/
resize, без ложного изменения внутренних отступов при переносе окна.

Входы: receipts/P01-probe-source.md (M04 actual24ae6d9, G11 consumptioneb2cb12),
receipts/G02-recorded-diff.md (G12), docs/development/native-helper.md и
native-acquisition.md. Уже измерены viewport и row своего F02, перенос окна не
изменил probe-local значения. Screen/pixel mappings ещё unknown; их нельзя
подменять вычитанием предполагаемого origin или общим scale. Исследовать публичные
источники actual transforms, затем реализовать поддержанное отображение.

Владение: plugins/macos (если существующий owner), tests/bridges/native,
tests/fixtures/native; crates/host/src/worker_native.rs, native_binding.rs,
native_broker.rs и соответствующие Native host tests при прямой необходимости;
docs/development/native-helper.md, native-acquisition.md, fixtures-native.md;
собственный receipts/M04-local-transforms.md. Общие schema/engine/CLI и worker_ops
защищены. Продуктовые нормы GEOMETRY/NATIVE не расширять; существующий canonical
Transform используется с actual Evidence и полной привязкой к источнику.

Acceptance: собственный разрешённый fixture, known transform по фактам API,
Measure/diff через имеющийся Rust CLI, move отдельно от local layout change,
scroll и resize; stale/environment mismatch и missing mapping дают явный unknown
без координатных действий. Сохранить AX/layout/hit/visible/pixels различия,
actual scale/units/origin. Не делать ненужный новый probe invariance run.
Cross-display проверять лишь при доступной подходящей среде без перестройки
дисплеев; неподтверждённую квалификацию явно оставить открытой. Никаких claims
полного M04/P7 только по этому ограниченному окружению.
Immediate consumer: реальная геометрия Mac и межплатформенный Q01 сценарий.

## G13 — Сравнение структуры, связей и фокуса

Outcome: закончить отдельный G02 результат — агент получает компактное и JSON
сравнение сохранённых source graphs, включая children, relations, component
mapping, node metadata и focus, с различением содержимого и evidence-only changes.
Это дополняет уже работающие property diff и selected-space rect diff.

Входы: packets/G02-recorded-diff.md, receipts/G02-recorded-diff.md,
docs/development/diff.md; существующий borrowed compare_recorded и G12.
Нынешний CLI-DIFF@2 явно ограничен node/property: его JSON1.0.0 и смысл не менять.
Техническое additive представление нового результата выбрать самому по
ROADMAP/CLI.CONTENT, зарегистрировать до кода без запроса root на каждый flag.
Не добавлять второй graph/cache; сохранять исходные canonical records.

Владение: crates/engine/src/diff.rs и непосредственно нужные engine exports/tests
(не cache); crates/cli/src и связанные CLI tests только для нового diff;
docs/development/diff.md и cli документация; docs/specs/product/cli.md,
cli-diff.md и при необходимости отдельный leaf в том же каталоге;
docs/specs/README.md для этой регистрации; receipts/G13-graph-diff.md.
Core schema/analysis wire и Web/Native/host/cache/export защищены.

Acceptance: независимые синтетические expected before/after для каждого вида
изменения, partial/missing не означает deleted, различение identity/namespaces/
generations и evidence-only, полнота/omissions/лимиты/санитизация в выводе;
старые raw JSON и G12 semantics неизменны. Сохранённые реальные данные использовать
только если они действительно доступны; не выдавать synthetic за live.
Одного понятного runnable CLI example достаточно для handoff, без нового UI run.
Immediate consumer: change→diff разработчика и будущий P6 compare.
