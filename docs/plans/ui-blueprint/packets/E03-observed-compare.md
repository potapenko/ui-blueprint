# E03 — Два наблюдения → полный compare-пакет

Статус: queued; dispatch только после финального G13 receipt и сохранённого API.
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
Перед dispatch закрепить actual G13 API/контракт/commit из финального receipt.
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
