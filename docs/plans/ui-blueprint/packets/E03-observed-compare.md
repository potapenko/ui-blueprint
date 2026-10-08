# E03 — Два наблюдения → полный compare-пакет

Статус: ready. G13 завершён:904667165703353e7c9cd27bd128367f51d5d6a3,
push подтверждён финальным ответом; CLI/spec ownership освобождено.
Класс shipping_product; PLAN.UIB@1 P3/P6; отдельный самостоятельный чат, inherit.
Пользователь одобрил полные tasks: постановка, собственный план, реализация,
проверки, исправления, документация, commit+push и финальный ответ без микрошагов
root. Немедленная реализация внутри outcome разрешена после локального плана.

## Результат

Агент передаёт два сохранённых наблюдения и явные разрешённые metadata; локальный
imagegen-prompt выдаёт полный compare-пакет с исходными фактами обеих сторон и
изменениями, вычисленными существующим Rust engine. Для известного изменения
контрола промпт называет именно подтверждённое изменение; не оставляет generic
unresolved_g02, когда engine уже умеет его установить. Изменение связей/структуры/
фокуса не теряется за геометрией. Document/propose/detail/flow сохраняются.
Пакет остаётся model-free, полным в объявленном scope, с отдельными статусами
source/validation/approval, неизвестными и ограничениями. ImageGen не запускать.

## Основание и готовность

Требование: EXPORT@1.CONTENT, DRAWING-PACKAGE/GEOMETRY/REVIEW@1.CONTENT,
CLI.CONTENT и GOLDEN.CONTENT export; источник — подтверждённые UIB.TZ@1.4,
UIB.DRAWING@1.1 и одобренный PLAN.UIB@1, а не пожелание compiler implementation.
Наблюдавшаяся реализация по E01/E01-cli receipts: --brief сравнивает пары views,
прямой --snapshot/metadata подключён только к document; полная attributed
comparison оставалась G02 consumer dependency. G13 её закрывает в своём scope.

Root traversal: AGENTS → specs/README → product/README → EXPORT@1 полный
closure (drawing-package/style/geometry/prompt-a/prompt-b/review и reference/
drawing-example@1), CLI-EXPORT@1 → EXCHANGE@2/PRIVACY@1/ANALYSIS@2 с
TYPES/VALIDATION@1; CLI/CLI-DIFF/новый G13 leaf после сохранения;
MODEL/BOUNDARIES/IDENTITY/GEOMETRY/PROJECTIONS/FORMS/CACHE/ACTIONS/LIFECYCLE@1;
acceptance/GOLDEN@1; ROADMAP/RUST-BOUNDARIES, RUST/DEV.RUST@2 и explicit Requires.
G13 pin: CLI@14/registry27, CLI-GRAPH-DIFF@1 INPUT/COMPARE/DATA/FAILURE;
полный новый leaf прочитан root, прежний closure сохранён. API/пример/проверки
в receipts/G13-graph-diff.md и docs/development/diff.md; кандидат9046671.
Snapshot metadata/surface_records/captures не входят в standalone graph comparison;
эту границу не скрывать в экспортном описании.
Остальные выбранные текущие решения: D01@1,D02@2,D03@3,D04@1,D05@4,
MEMORY@2/WORK@1/D06@1/D07@5, нормы и не новый повод менять host.
Публичную additive CLI форму выбирает исполнитель по ROADMAP/P6; регистрирует
конкретный вход/выход и совместимость до кода. Не менять исходное намерение.

Входы: receipts/E01.md, E01-cli.md, E01-analysis.md, E02-repair.md,
G02-recorded-diff.md, финальный G13 receipt; docs/development/export.md и diff.md.
Исполнитель сверяет источник и действующий API сам, не просит root проектировать
флаги/DTO/тестовые функции. Не повторяет уже принятую numerical/privacy проверку
целиком без изменения её входов; новые input/export boundaries проверяет.

## Владение и границы

crates/export с непосредственно нужными tests; export/input/dispatch owners в
crates/cli и соответствующие binary tests; fixtures/export для явно синтетических
примеров; docs/development/export.md и export-раздел cli.md; docs/specs/product/
cli-export.md, export-related routing в cli.md/README.md и docs/specs/README.md
только для принятого additive representation; собственный receipts/E03-observed-compare.md.
G13 должен освободить CLI/spec write ownership до старта. I01 не меняет эти файлы.
Engine/schema/plugin/host/cache/collectors/Cargo/lockfile защищены. Если готовый
engine API действительно не даёт необходимого результата — назвать конкретную
зависимость; не писать вторую аналитику в exporter или обходить evidence.

Сохранить raw diff1.0.0, G12/G13 outputs, core0.1/analysis0.2, существующий export
format где возможно; breaking representation только как явно зарегистрированная
техническая версия в разрешённых границах. Не менять observed Snapshots, times,
units/Spaces, coverage, availability, namespaces и source IDs. Отсутствие в записи
не доказывает deletion; heuristic совпадение не даёт action ref. Геометрическую
разницу вычисляет engine при известных transform/binding, без guessed space.
Metadata не вносят geometry или новую source authority. Public texts проходят
existing allowlist; paths/secrets/pixels не добавляются автоматически.

Git/runtime договор: autonomous-tasks-2026-10-08.md, текущая master, общий
/tmp/ui-blueprint-master-git.lock через fcntl.flock на короткий commit+push,
собственные exact paths, без root grant. Не создавать подагентов/новых постоянных
директорий, не запускать UI/модели/чужие приложения. Outputs в system temp,
изображения никогда не удалять. Не overwrite существующий destination/baseline.

## Готовность

Рабочий public CLI путь + полный six-file/self-contained compare package,
независимые expected pairs: changed/unchanged/unknown/redacted/partial,
children/relations/component/focus, evidence-only, несовместимые identities/
contexts, input/output bounds и сохранность legacy modes. Исходные факты обеих
сторон сохраняются; более сильные выводы из них не создаются. Контролируемые
synthetic данные не маркируются live; реальные сохранённые records использовать
лишь если доступны и применимы. Один runnable пример без новой коллекции UI.

Финальный ответ: capability, scopes, команды/результаты, SHA+push, exact residuals.
Независимая acceptance проверка input/privacy остаётся обязательной и может быть
сгруппирована с готовыми export изменениями; не создавать отдельные review rounds
на каждый внутренний шаг. Чат сам доводит код и свои проверки до результата.

## Закрыть оставшийся export acceptance gap

После c97c513 и независимого source/privacy acceptance остался конкретный
pre-existing test failure factual_query_migration_keeps_saved_packages_structurally_unchanged.
Он воспроизведён автором на e4bc256: historical F01 package ожидает unknown,
хотя ранее принятый256f2a2 и текущие G09/GEOMETRY semantics вычисляют известные
явные anchors даже при partial coverage. E03 доказал byte-equality document/propose
до/после своей правки; E03 regression не установлена.

Это продолжение исходному owner для P6 verification debt: полностью reconcile
текущие export expectations с нормативным поведением и закрыть failed test,
сохранив исторические observed/proposed package artifacts и raw evidence.
Нельзя просто удалить/skip тест, переименовать failure в pass, weaken privacy/
unknown requirements или вернуть engine к старой ошибке. Исполнитель сам выбирает
минимальные независимые expected assertions/current fixture representation и
проверяет все затронутые export modes. Никакого нового framework или каталога.

Норма/authority: GEOMETRY/ANALYSIS@2 known source anchor vs partial scope,
EXPORT/DRAWING full package/unknown/evidence/statuses; accepted source256f2a2,
G09/E03 receipts и unchanged public outputs как evidence. Новые продуктовые нормы
не вводятся. При реальном конфликте baseline authority вернуть точный факт.

Владение для этого остатка: crates/export/tests, fixtures/export (новые current
examples в существующих каталогах, НЕ overwrite исторических artifacts),
docs/development/export.md и E03 receipt. Product source/engine/CLI/spec registry/
Native/Web/host/manifests защищены, они принадлежат текущим M02/W04 или приняты.
Если выявится реальный source bug, сообщить точную dependency, не расширять
исправление молча. Цель — полный результат закрытия одного установленного gap,
не поручение на очередной промежуточный тест. Own plan→change→checks→commit+push;
общий flock, system temp и image retention прежние. UI/models не запускать.
Final: нормативное основание актуальных expectations, сохранность historical
артефактов, какой suite действительно прошёл, SHA/push и remaining gaps.
