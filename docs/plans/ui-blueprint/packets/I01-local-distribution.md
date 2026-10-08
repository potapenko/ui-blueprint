# I01-L — воспроизводимая локальная поставка

Самостоятельная задача среднего размера, shipping_product, PLAN.UIB@1 P6.
Пользователь одобрил реализацию и отдельные чаты полного цикла. Root передаёт
результат и границы; исполнитель сам планирует, реализует, проверяет, исправляет,
обновляет документацию, делает commit+push и возвращает финальный ответ.
Повторного разрешения после плана не требуется. Подагентов/другие чаты не создавать.

## Результат

Пользователь может одной документированной процедурой собрать выбранный набор
UI Blueprint и получить согласованные CLI/worker/Native helper, необходимые
schema/examples и notices в явно выбранном существующем каталоге. Локальный
запуск не зависит от cwd исходного checkout, ключей модели или цепочки чужих CLI.
Есть точные инструкции запуска, ограничений, восстановления и удаления только
собственного установленного набора. Не устанавливать сейчас ничего в home/PATH,
не публиковать пакеты, не подписывать/нотаризовать и не менять настройки ОС.
Проверять поставку только в собственном system-temp output.

Это законченная install/build/recovery часть I01. V01, оставшиеся pilots и P7
по-прежнему нужны для release acceptance; успешная упаковка их не заменяет.

## Готовность и Spec Basis

Существуют рабочие saved CLI и worker, Native helper и build-geometry; текущая
CLI-документация оставляет installable placement открытым I01. Source-functional
часть G13 и платформенные задачи продолжаются независимо. Не ждать их финального
feature outcome для реализации упаковки; проверять свой bundle на coherent saved
revision и честно сообщать проверенный pin, не на незакоммиченном коде соседа.
Исторический rust-workspace.md описывает один ранний crate и не является текущим
инвентарём. Актуальные Cargo metadata/locked graph и executable owners изучает чат.

Traversal: AGENTS → docs/specs/README registry26 → product/README и
acceptance/README. Selected CLI@13, BOUNDARIES/RUST-BOUNDARIES/ROADMAP@1,
COMPLETION@1 (поставка/изоляция/model-free/recovery), D01@1,D02@2,D07@5,
DEV.RUST@2/RUST, REUSE@1 и explicit Requires. Полный COMPLETION closure включает
PILOTS/WEB-PILOTS/NATIVE-PILOTS/GOLDEN/PERFORMANCE@1, EXPORT@1 с
DRAWING-PACKAGE/STYLE/GEOMETRY/PROMPT-A/PROMPT-B/REVIEW/EXAMPLE@1. Общая
MODEL/IDENTITY/GEOMETRY/PROJECTIONS/FORMS/ACTIONS/LIFECYCLE/CACHE/PRIVACY@1,
EXCHANGE@2, NATIVE@2, D03@3,D04@1,D05@4,MEMORY@2,WORK@1,Native acquisition@2,
D06@1 — действующие ограничения; поведение этих доменов не менять.
Root прочитал применимый closure; исполнитель до исходников читает относящиеся
к задаче полные листья, фиксирует собственный traversal и текущее состояние.

Evidence inputs: docs/development/{cli,native-helper,dependencies,rust-workspace}.md,
plugins/macos/README.md, готовый build-geometry в tests/bridges/native/host_observe.py
как read-only reference, saved G09 exporter и CLI examples. Продуктовая норма —
воспроизводимость/выбранные модули; конкретный минимальный build/install способ —
технический выбор исполнителя в пределах ROADMAP. Не создавать framework.

## Владение

Разрешены необходимые root-level build/install script files (в существующем
корне, без нового каталога), root README.md для first-use при необходимости,
THIRD_PARTY_NOTICES.md, docs/development/distribution.md и dependencies.md,
собственный docs/plans/ui-blueprint/receipts/I01-local-distribution.md;
свои install-check файлы в существующих tests/bridges (не Web/Native подкаталогах).
Сначала объявить точные файлы в собственном плане. Не редактировать product source,
Cargo.toml/lockfile, helper/fixture source, cli.md или spec registry: там другие
владельцы либо защищённые интерфейсы. Если реально необходима такая правка,
вернуть точную зависимость, не создавать другую реализацию продукта.

Common resource/Git договор:
[самостоятельная группа](autonomous-tasks-2026-10-08.md#общий-договор).
Тот же /tmp/ui-blueprint-master-git.lock, Python fcntl.flock LOCK_EX без root grant;
stage только свои paths, push canonical master. Собственный CARGO_TARGET_DIR
и продукты в system temp. Не удалять любые images/их каталоги и чужие output.
Desktop/browser runtime не нужен и не разрешён этой задаче. Не создавать новые
постоянные каталоги. Build-time runtime dependencies и fixture tools различать.

## Проверка и завершение

Проверить реально выбранные supported feature combinations: core-only без
платформенных SDK; Web build не запускает Swift/Apple build toolchain; Native не
требует browser/model stack; для общей Mac поставки корректные matching binaries.
Использовать существующие features, не менять default policy/dependencies.
Не обещать Linux/Windows/Intel/старый macOS без проверки D01.

Получить bundle из committed revision, проверить запуск из другого cwd на
готовых deterministic examples (валидатор, measure/check, model-free export) без
сбора UI. Не выдавать эти smoke checks за оба E2E или live support qualification.
Проверить ошибки missing tool/input/destination, отказ перезаписывать чужие
файлы и отсутствие частично объявленной успешной поставки. Recovery объясняет
несовместимые worker/helper и переустановку своего набора без удаления чужого.
Notices собрать по фактически выбранному locked normal/build dependency graph,
проверяя LICENSE/NOTICE выбранного материала; не придумывать лицензию проекта
и не выполнять публикацию. Сырые command logs не коммитить.

Финальный ответ: готовая процедура и artifact layout, commit/push, проверенные
комбинации/команды/результаты, exact tested source revision, limitations и только
реальные оставшиеся dependencies. Дальнейшая Q01/V01/Q02 acceptance отдельна.
