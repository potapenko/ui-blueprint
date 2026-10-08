# W05 — Web form workflow и E2E

Самостоятельный shipping_product task по PLAN.UIB@1 P5/B02 и Web E2E P7.
Outcome: законченный ограниченный workflow через shipping API/CLI: адресованный
Observe формы → публичный Focus/Type неполного draft → debounce/validation/
autocomplete → разрешённый выбор → независимое подтверждение applied value →
свежее наблюдение после значимого перехода → diff/check → полный compare-пакет.
Unexpected popup/remount/disabled/private/stale/неизвестный effect останавливают
зависимое исполнение. Доставка отдельно от проверки; DOM value не business success.
Не ограничиваться browser setup напрямую через Playwright вместо product input.

Пользователь одобрил полный цикл отдельного чата: постановка/план, source/read,
код нужных пробелов, synthetic/own headless runtime, исправления, документация,
commit+push и финальный результат. После плана реализация разрешена без root grants.
Не создавать подагентов или другие чаты. Если существующий production путь уже
покрывает часть результата, переиспользовать его, не переписывать ради активности.
Не создавать новый framework, сценарный DSL, daemon или universal macro engine.

## Основание и входы

AGENTS → specs/README → product/README → ACTIONS/FORMS/IDENTITY/LIFECYCLE/
CACHE/PRIVACY@1, CLI-ACTIONS@3, EXCHANGE@2 и явный MODEL/BOUNDARIES/GEOMETRY/
PROJECTIONS closure; acceptance/README → WEB-PILOTS@1 B02, COMPLETION@1 Web E2E,
PILOTS/GOLDEN/PERFORMANCE и explicit export/drawing closure. D02@2/D04@1/D05@4/
MEMORY@2/WORK@1 и D01@1/D03@3/D06@1/D07@5/RUST/DEV.RUST@2; EXECUTOR-SOURCES/
REUSE@1 для нового механизма. Root прочитал нормы; это Restore полного одобренного
сценария. Fresh source evidence не создаёт новых правил поведения.

Stable inputs: W04d3f4723/final951bc01, W03ffe33e4+85ea656, B03b43d0df,
E03c97c513+baseline reconciliationc6b2357. CLI@15/CLI-EXPORT@2; Native-session
routing текущего M02 task не меняет Web action requirements. Использовать coherent
saved sources + свои изменения, не захватывать Native WIP в проверенную сборку.

Каноническая история Web actions находится в одном существующем большом receipt:
receipts/W02-provider-handoff.md — релевантные Focus/Type и A05/application sections;
receipts/W02-provider-review.md, W04-observation-pilots.md, B03-popup-geometry.md,
E03-observed-compare.md. Не искать несуществующие отдельные A03/A04/A05 receipt-файлы.
Docs/development/cli.md, web-collector.md и fixtures-web.md; существующие
fixtures/web и tests/bridges/web/guarded-live.cjs уже содержат сценарии формы.

Установленные факты: прежняя A05 application цепочка имела7 public calls:
explicit option→draft London, Commit→applied London, surprise observation→zero
следующих Execute. Lon/readiness тогда были fixture setup, не новая product
Focus/Type proof. Focus/Type ранее проверялись отдельно. Задача закрывает цельную
композицию, не приписывает прежним срезам полный E2E. Semantic native HTMLButton
click остаётся untrusted/userGesture=false, не pointer/hardware/IME proof.
Actual expected state задаёт собственный fixture/profile/Expectation, не guess
по названию поля. Не вводить общий Apply/Cancel смысл для других платформ.

## Владение и ограничения

plugins/web; crates/host/src/worker_web.rs и Web-specific tests;
tests/bridges/web; fixtures/web; docs/development/web-collector.md,fixtures-web.md;
свой receipts/W05-web-form-e2e.md. Конкретные файлы/план объявить до правок.
CLI/engine/schema/plugin-api/shared host/worker_ops/worker_main/manifests/spec
registry принадлежат M02 или защищены. При настоящем API gap вернуть точную
зависимость, не создавать другую модель/парсер. M02 owns Native/общий host/CLI.

W04 established pre-Ready Attach classification gap передан M02 shared owner:
wrong target/old loader сейчас дают0stdout и safe clean refusal, но generic exit1
вместо typed stale/resync. Это отдельная зависимость общего host; не закрывать
B01 semantic acceptance, не чинить protected worker_main и не маскировать это как
свой successful expected product status. Независимые form/E2E части продолжаются.

Только свой headless Chromium/context; не desktop focus/pointer и не реальные
PlayPhrase.me проекты. Browser tools допустимы для fixture setup/oracle/наблюдения,
но не вместо проверяемой product delivery. Scope/deadline/bytes/fields сохранять;
никакого autoobserve между явными запросами, retry неизвестной mutation или Enter
по умолчанию. Redacted не empty; private values не выходят в stdout/error/cache/
export. Реальная сеть/сервер/durable business результат не заявляется по fixture.

Master без веток/worktrees. fcntl.flock /tmp/ui-blueprint-master-git.lock только
на короткий scoped commit+push canonical master без root grant. Собственные
build/run outputs system temp; images/их каталоги никогда не удалять. Нет новых
постоянных каталогов или зависимостей. Общие API выбирать из реальной реализации.

## Готовность

Один воспроизводимый цельный public workflow и необходимые negatives с independently
authored expected states. Точное соответствие target/result identity, input modality,
draft/selected/applied, awaited condition вместо network-idle, source/effect outcome,
stop_on_error, original snapshots/evidence и полный model-free compare-package.
Проверить relevant current save и обе scoped UI states без ложной atomicity/clock.
Прежние принятые проверки не повторять целиком, только затронутую композицию.

Если Source implementation уже достаточна, честно обозначить delivered qualification;
не создавать лишний product code. Если часть B02/E2E не достигнута — перечислить
конкретные missing capabilities/evidence, не выдать один click за full workflow.
Final: capability/qualification, actual checks/results, tested source, SHA+push,
cleanup/limitations/remaining dependency. Независимая review и итоговый общий
candidate с Native/D06 остаются отдельными обязательствами цели.
