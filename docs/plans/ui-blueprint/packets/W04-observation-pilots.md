# W04 — закончить Web read-only pilots B01/B04/B06

Ready после terminal B03-Gb43d0dfbe8b893d02fd6027e5d9f31331d4e4f57/push; Web owners освобождены. Один самостоятельный task
среднего размера по PLAN.UIB@1 P2/P3, shipping_product; оставшаяся Web observation
часть, не весь Web и не одно поручение на каждый тест. Immediate consumer Q01 и
практические geometry/diff вопросы агента. Уже сделанное повторно не реализовать.

После dispatch исполнитель сам проходит постановку/план, изучение нужных owners,
реализацию пробелов, необходимые synthetic/live checks, исправления, документацию,
commit+push и финальный отчёт. Никаких подагентов/новых чатов или остановок на
промежуточное root approval. Если capability уже работает, не добавлять лишний код:
показать достаточное подтверждение и отличить qualification от новой реализации.

## Результат

На собственном Chromium fixture работают оставшиеся B01/B04/B06 в поддержанном
профиле: две точные вкладки/поверхности с одинаковыми подписями, navigation/remount
инвалидируют старую привязку; resize/scroll/text-size/locale сохраняют смысл значений
и явно атрибутированные geometry/transforms; составной control имеет разные DOM/AX
source identities и truthful interaction/design projection/mapping.
Не превращать декоративный child в actionable ref и не угадывать компоненты по
именам/совпадению rect. Unsupported boundary остаётся явным ограничением, не pass.
Не расширять на Web actions B02, новый cache/resync B05 или повтор B03 целиком.

## Spec Basis и входы

AGENTS → docs/specs/README → product/README → BOUNDARIES/MODEL/IDENTITY/
GEOMETRY/PROJECTIONS/FORMS/PRIVACY@1, EXCHANGE@2; acceptance/README →
WEB-PILOTS@1 B01/B04/B06 и полный explicit Requires (PILOTS/CACHE/ACTIONS/
LIFECYCLE/GOLDEN); D01@1,D02@2,D03@3,D04@1,D05@4/MEMORY@2/WORK@1/D06@1/D07@5,
RUST/DEV.RUST@2, ROADMAP/RUST-BOUNDARIES/REUSE/EVIDENCE/PERFORMANCE@1.
Pin: registry28/CLI@15, B03b43d0df, E03c97c513, Native5fd4b6a;
Web-specific нормы и перечисленный closure неизменны. Independent review B03
пока pending; итоговую квалификацию выполнять на coherent saved inputs, не WIP.
CONTENT нормы не меняются; новый schema/feature/constant не создаёт свою authority.
Reference R01/Web source ledger использовать по механизму; новое upstream чтение
только при конкретном недостающем API, без повторения всего исследования.

Inputs: docs/development/web-collector.md, fixtures-web.md, existing fixtures/web,
receipts/W01-rooted-selection.md, W03-resync-complete.md, B03-popup-geometry.md
после завершения. Принятые B04 источники: b234fff actual viewport→document и G12
consumer140e53d; parent/font geometry и explicit resync есть в W03. G13 source
review9046671 + native-independent raw/G12 consumers доступны. Reuse исходные
receipts по их scope, не запускать новую такую же цепочку только для новой таблицы.

Недоказанные части определить по source/tests и существующим receipts внутри
трёх выбранных pilots; закрыть реализацией/проверкой, а не ещё одним source handoff.
Целевая новая evidence особенно text-size/locale и reported DOM↔AX/component
association. Не придумывать единый clock/новую freshness для сохранённых записей.
Сравнение геометрии выполняет Rust через existing CLI; source data не переписывать.

## Владение

plugins/web; Web-only host caller/worker_web и связанные tests; fixtures/web;
tests/bridges/web; docs/development/web-collector.md,fixtures-web.md,web-cdp.md;
собственный receipts/W04-observation-pilots.md. Точные файлы в своём плане.
Общие engine/schema/plugin-api/CLI/worker_ops/host supervisor/Cargo/lockfile и
spec semantics защищены. Native M02 будет отдельным владельцем общего Native
workflow; эти области не занимать. Реальный cross-owner/API gap вернуть точно,
не создавать shadow models или другую аналитику на JS.

Только свой headless browser/context, никакого desktop input. Между explicit
requests нет сбора/мониторинга. Не оперировать реальными PlayPhrase.me проектами,
не устанавливать browser/extensions/framework. Текущая master без worktree/branch;
общий fcntl.flock /tmp/ui-blueprint-master-git.lock на scoped commit+push,
без root grant. Build/output в system temp; images/их каталоги никогда не удалять.

## Готовность

Для каждого из B01/B04/B06 короткая связь expected → actual → owner/commit/test
и ограничения. Реальные bounded Observe/inspect/measure/diff там, где требуется
наблюдение; independent authored fixture expectations, source order/roles/values,
coverage/unknown/redacted/empty, read-only invariance, wrong/stale target refusal
и подтверждённый cleanup. Identity/capability проверка не является delivery proof.
Text metrics/visible/hit/layout и leading/trailing не смешивать; нет guessed glyph
bounds, scale или точного breakpoint из двух образцов. Если full positive
обязательство остаётся непроверенным, прямо назвать его и не закрывать pilot.
Final: готовая capability/qualification отдельно, checks/results, SHA+push,
remaining exact gaps. Не объявлять весь P7 или general browser qualification.
