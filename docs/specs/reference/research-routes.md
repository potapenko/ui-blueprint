# R01/R02/R03: точные маршруты

- Node type: branch; contract: `UIB.RESEARCH-ROUTES@1`; clause: `UIB.RESEARCH-ROUTES.ROUTE`.
- Authority: Active / Stability: Evolving; routing only; accepted/released baseline: none.
- Authority source: [registry](../README.md), C00 faithful routing of user-confirmed originals.
- Read when: подготовка/исполнение одного из R01/R02/R03.
- Do not read when: другой пакет уже имеет свой pinned closure.
- Requires: только выбранные ниже листья и их explicit closure.

Все перечисленные контракты закреплены на `@1`, clause `UIB.<ID>.CONTENT`.
Путь: AGENTS → specs/README → этот узел → листья ниже в указанном порядке.
Это полный transitive closure; оригиналы — provenance, не повторный full read.
Для новой материальной темы сначала расширить маршрут, не вывести норму из кода.

## R01

[UIB.BOUNDARIES@1](../product/boundaries.md), [UIB.REUSE@1](reuse.md), [UIB.WEB-SOURCES@1](web-catalog.md), [UIB.PILOTS@1](../acceptance/pilots.md).
[UIB.MODEL@1](../product/model.md), [UIB.GEOMETRY@1](../product/geometry.md), [UIB.IDENTITY@1](../product/identity.md), [UIB.PROJECTIONS@1](../product/projections.md).
[UIB.FORMS@1](../product/forms.md), [UIB.EXCHANGE@1](../product/exchange.md), [UIB.PRIVACY@1](../product/privacy.md), [UIB.LIFECYCLE@1](../product/lifecycle.md).
[UIB.CACHE@1](../product/cache.md), [UIB.ACTIONS@1](../product/actions.md), [UIB.WEB-PILOTS@1](../acceptance/web-pilots.md), [UIB.ROADMAP@1](../product/roadmap.md).

Browser collectors/refs/actionability/session: agent-browser, browser-use, Playwright injected/backend и CLI sessions, Stagehand, CDP/CSSOM/ARIA. Consumer: D01/D02/D04, W01; bounded prototype и source record, не новая модельная orchestration.

## R02

[UIB.BOUNDARIES@1](../product/boundaries.md), [UIB.REUSE@1](reuse.md), [UIB.NATIVE-SOURCES@1](native-catalog.md), [UIB.PROBE-SOURCES@1](probe-catalog.md).
[UIB.PILOTS@1](../acceptance/pilots.md), [UIB.MODEL@1](../product/model.md), [UIB.IDENTITY@1](../product/identity.md), [UIB.EXCHANGE@1](../product/exchange.md).
[UIB.GEOMETRY@1](../product/geometry.md), [UIB.PROJECTIONS@1](../product/projections.md), [UIB.PRIVACY@1](../product/privacy.md), [UIB.LIFECYCLE@1](../product/lifecycle.md).
[UIB.NATIVE@1](../product/native.md), [UIB.FORMS@1](../product/forms.md), [UIB.CACHE@1](../product/cache.md), [UIB.ACTIONS@1](../product/actions.md).
[UIB.NATIVE-PILOTS@1](../acceptance/native-pilots.md), [UIB.ROADMAP@1](../product/roadmap.md).

AX/capture/identity/notifications и реализуемость M05: AXorcist, Peekaboo, Apple APIs, Compose markers/Preview. Consumer: D01/D02/D04, M01/P01; measured probe и off/on invariants, не универсальный SDK.

## R03

[UIB.BOUNDARIES@1](../product/boundaries.md), [UIB.REUSE@1](reuse.md), [UIB.CORE-SOURCES@1](core-catalog.md), [UIB.PROBE-SOURCES@1](probe-catalog.md).
[UIB.MODEL@1](../product/model.md), [UIB.IDENTITY@1](../product/identity.md), [UIB.EXCHANGE@1](../product/exchange.md), [UIB.PROJECTIONS@1](../product/projections.md).
[UIB.PRIVACY@1](../product/privacy.md), [UIB.LIFECYCLE@1](../product/lifecycle.md), [UIB.CACHE@1](../product/cache.md), [UIB.GEOMETRY@1](../product/geometry.md).
[UIB.ROADMAP@1](../product/roadmap.md), [UIB.PILOTS@1](../acceptance/pilots.md).

Typed updates, projections/geometry/rules: AccessKit node/update/consumer, Galen/Extras, Compose anchors. Consumer: D03, G01/K01; строковый diff не graph delta, partial/unknown не pass.

## Общие исключения и границы

Из closure исключены CLI/export и DrawingBrief, product-specific N/W profiles,
Jev/ML, Android/iOS/Windows, F1–F4 и UI Atlas: у этих исследований нет такого
consumer. R03 не принимает живой adapter; GOLDEN01 выбирается в S01/K01 отдельно.
R01 не читает native internals, R02 не читает browser catalog. Листы общих
контрактов сохраняют только необходимые межплатформенные границы.

Upstream читать по выбранному механизму с соседними types/tests и source record
по REUSE. Каталог — исторический вход, не выполненное исследование. Разрешения
на mutation/prototype задаёт конкретный конечный пакет, а не этот маршрут.
D01–D07 здесь не решены; их deadlines/acceptance остаются в ROADMAP.
