# B03-G — геометрия popup/portal и границы frame

Самостоятельная shipping_product задача по PLAN.UIB@1 P2/B03. Пользователь
разрешил полный цикл в отдельном чате: постановка/свой план, код, необходимые
тесты и живой own-fixture сценарий, исправления, документация, commit+push.
После плана выполнять без повторного согласования root; не создавать подагентов.

## Результат и основание

Закончить поддержанный Chromium B03 путь: агент адресует popup/portal и получает
его геометрию, sourced связь с trigger/Surface, сведения об известной обрезке и
доступные факты hit testing, с честным частичным покрытием и frame boundary.
Read-only observe не двигает focus/scroll/layout. Не оставлять доказуемые текущим
API факты навсегда unknown только потому, что прежний collector их не реализовал.
При этом layout bounds не доказывает hit/visible/paint и не заменяет эти поля.
Сэмпл hit point не объявлять полной hit area, clipping не доказывает полную
окклюзию, z-index без stacking context не создаёт порядок видимости.

Нормы: WEB-PILOTS@1.CONTENT B03; GEOMETRY/PROJECTIONS/IDENTITY/PRIVACY@1.CONTENT.
Traversal: AGENTS → docs/specs/README → product/README → названные листья и
MODEL/BOUNDARIES/EXCHANGE@2; acceptance/README → WEB-PILOTS/PILOTS@1 и explicit
FORMS/CACHE/ACTIONS/LIFECYCLE closure. D01@1,D02@2,D03@3,D04@1,D05@4,
MEMORY@2/WORK@1/D06@1/D07@5, RUST/DEV.RUST@2, ROADMAP/RUST-BOUNDARIES/
PERFORMANCE/GOLDEN/REUSE/EVIDENCE@1 и requires. CONTENT/worker/publication/
acquisition bounds остаются. Режим Restore; конкретные API/representation
исследует исполнитель по источникам, не придумывает продуктовые нормы.

Входы: docs/development/web-collector.md описывает уже действующие popup_relations
и rooted selection, неизвестные hit/visible/paint и отказы на boundary;
receipts/W01-rooted-selection.md, W03-resync-complete.md и source records R01.
Ранее actual popup-relations подтверждён, B04 viewport→document mapping сохранён
b234fff и G12 comparison140e53d; эти результаты переиспользовать.
W03-Rffe33e4 завершён/pushed, его owner освобождён. Для полезного сценария —
Web Director advice в receipts/platform-test-advice.md, но без запуска реального
PlayPhrase.me; численные ожидания собственного fixtures/web/expected.json
не переносятся на чужой сайт. Fixture controls/states — setup, не production input.

## Владение

plugins/web; crates/host/src/worker_web.rs и связанные Web host tests;
tests/bridges/web; fixtures/web; docs/development/web-collector.md, web-cdp.md,
fixtures-web.md; собственный receipts/B03-popup-geometry.md. Точные файлы в
собственном плане до правок. Schema/engine/CLI/export/worker_ops/Cargo/lockfile
и spec semantics защищены. Если canonical model не выражает нужный наблюдённый
факт или требуется общий engine API, вернуть конкретную зависимость, не писать
дублирующую аналитику на JS и не менять чужого owner без передачи владения.
JS/CDP получает факты среды; расчёты общего смысла остаются Rust.

Frame scope разрешать только по точной установленной target/document identity;
не расширять исходный root/allowlist молча, не искать другой frame по похожему
имени и не читать весь DOM для последующей фильтрации. Cross-origin/OOPIF/shadow
ограничения обозначать явно; успешный refusal не заменяет нужный positive gate.
Новые browser/permission settings/extension или запуск чужого сайта не разрешены.

Исполнитель владеет только своим headless browser/context; desktop у M03-C.
Общий договор autonomous-tasks-2026-10-08.md: master, fcntl.flock на
/tmp/ui-blueprint-master-git.lock для короткого exact-path commit+push без root
approval; no branch/worktree. Outputs/builds в system temp. Images/их каталоги
никогда не удалять. Никаких новых постоянных директорий/framework/dependencies.

## Готовность

Public scoped Observe/inspect/measure на собственном сложном popup/portal с
known положительными фактами и явно отделёнными неизвестными. Нужны независимые
expected геометрия/отношения, covered vs clipped/overlay состояния, boundary
отказ/допустимый exact scope, закрытие/remount/stale без чужого чтения, bounded
coverage/ошибки и read-only invariance. Точные API/команды/достаточные тесты
выбирает чат. Новое допустимое поддержанное поведение и ограничения документирует.
Не требовать universal arbitrary-site/frame support; не объявлять весь B03/P7
закрытым при недостающем обязательном positive evidence.
Финальный ответ: capability, проверенные сценарии/результаты, SHA+push, точные
remaining gaps; независимая итоговая приёмка отдельно от author self-check.
