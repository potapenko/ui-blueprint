# Единый реестр задач

- План: [PLAN.UIB@1](../ui-blueprint-development.md).
- Режим: `coordinated`, root coordination-only; цель активна в чате `01a11088-e608-7801-bdfb-db5c9383af9d`.
- Одобренный план: `358c757e7eab84a3989d150dbad57924d866601a`; ветка `master`.
- Пользователь 2026-10-06: «Ну да, лучше, наверное, не писать код, только координация. Совсем согласен. Давай, это, начинай цель и делай по плану, по реестру и так далее. В остальном я согласен.»
- Объём: P0–P7, рабочие чаты и follow-up по плану; root не реализует и не проверяет продукт.
- Начальный checkpoint `358c757`; код продукта отсутствует; строки `queued`, если статус ниже не уточнён.

## Очередь

Классы: C=coordination, D=diagnostic, T=tooling, S=shipping_product, V=verification.
Зависимости означают необходимые принятые результаты. До dispatch каждая строка
получает конечный пакет с exact write set, contract revision и acceptance.

| ID / этап | Класс / роль | Зависит от | Конечный результат / непосредственный потребитель |
| --- | --- | --- | --- |
| C00 / P0 | C / Spec | запуск | Разнести текущие требования в короткие маршрутизируемые контракты со stable clause IDs и обратной картой к ТЗ; без новых смыслов; вход всех пакетов |
| R01 / P0 | D / Web | C00 | Код browser collectors/refs/actionability; source record и ограниченный prototype; вход D01/D02/D04 и W01 |
| R02 / P0 | D / Native | C00 | Код AX/capture/identity и реализуемость M05; source record и prototype; вход D01/D02/D04 и M01 |
| R03 / P0 | D / Core | C00 | Семантика update/geometry/projections; source records; вход D03 и G01/K01 |
| F01 / P0 | T / Web | R01 | Контролируемый Web fixture B01–B06 с независимыми expectations; baseline D05/D06 |
| F02 / P0 | T / Native | R02 | Mac fixture M01–M06, измеримый merged control и probe off/on режим; baseline D05/D06 |
| C01 / P0 | C / root | R01,R02,R03,F01,F02 | Решения D01–D07 по срокам ТЗ, toolchain/MSRV/edition, support matrix и начальные frozen gates; вход P1/P2 |
| T01 / P1 | T / Integration | C01 | Минимальная Cargo-сборка нужных owners, lockfile, выбранные host/feature проверки; вход первого исполнимого результата |
| S01 / P1 | S / Integration | T01 | Schema/plugin-api candidate, parser/serializer, validator и GOLDEN01 с valid/invalid envelopes; вход всех модулей |
| G01 / P1 | S / Core | S01,R03 | Типизированные пространства, transforms и geometry checks pass/fail/unknown; детерминированные fixtures |
| L01 / P1 | S / Integration | S01,G01 | Минимальный CLI вход в engine: bounded JSON/compact и выходы validator; первый проверяемый путь |
| W01 / P2 | S / Web | L01,F01 | Живой Web observe/inspect/measure, точные target/surface, channels/coverage, redaction до выхода |
| M01 / P2 | S / Native | L01,F02 | Живой Mac observe/inspect/measure, window matching, AX/capture provenance и redaction |
| P01 / P3 | S / Native | M01 | M05 measured probe и comparison off/on; никакого влияния на geometry/focus/hit/AX |
| G02 / P3 | S / Core | G01,W01,M01,P01 | Interaction/design, many-to-many, scope/neighbors и compare без превращения heuristic match в action ref |
| K01 / P4 | S / Core | S01 | Bounded cache, revisions, atomic delta и replay на GOLDEN01; кандидат до K02 |
| W03 / P4 | S / Web | K01,W01 | Web events/invalidation, bounded resync и subscription teardown через принятый cache API |
| M03 / P4 | S / Native | K01,M01,P01 | Native notifications/invalidation, permission changes, stale handles и subscription teardown |
| K02 / P4 | V / review | W03,M03,G02 | Live invalidation/resync/lifecycle обеих платформ; M06/B05, независимость Target и full/delta oracle |
| A01 / P5 | S / Core | K02 | Общий prepare/resolve/act/verify, cancel/timeout/unknown outcome; deterministic fake delivery tests |
| W02 / P5 | S / Web | A01,W01 | Реальный Web input B02 и stale/ambiguous/unknown negatives |
| M02 / P5 | S / Native | A01,M01,P01 | Реальный native input M02, popup lifecycle и attribution по modality |
| E01 / P6 | S / Export | S01,G01 | DrawingBrief validator/compiler, document/propose/detail/flow/compare и sheets plan на fixtures |
| E02 / P6 | V / review | E01,W01,M01,G02 | Observed/proposed примеры, поля/units/unknown, безопасный export; обновить проверку после final candidate |
| V01 / P5–P6 | V / review | W02,M02,K02,E02 | Независимая проверка identity/permissions/privacy/concurrency, canary по всем каналам; repairs владельцам |
| I01 / P6 | S / Integration | V01,P01,G02 | CLI integration, installable выбранный набор, recovery/docs, зависимости и NOTICE, никаких обязательных моделей |
| Q01 / P7 | V / review | I01 | Полные M01–M06, B01–B06 и два E2E на фиксированной сборке; совместимость schema после обоих пилотов |
| Q02 / P7 | V / performance | Q01,C01 | Cold/warm quality/latency и overhead против baseline; не подгонять gates |
| C02 / P7 | C / root | Q01,Q02 | Проверить всю матрицу, записать limitations и accepted baseline, сохранить checkpoint; цель complete только по результату |

Для защищённых рисков независимая V-проверка обязательна до accepted каждого
готового среза; final V01 объединяет актуальные доказательства. Q01 повторно
проверяет затронутые интеграцией I01 критерии на итоговом commit. Менять интерфейс для
нескольких владельцев — отдельная согласованная интеграция, не тихая правка.
M01/M02 здесь task IDs; одноимённые сценарии ТЗ явно писать как `pilot M01/M02`.

## Чтение внешнего кода

| Пакет | Релевантный маршрут каталога ТЗ | Ограничение результата |
| --- | --- | --- |
| R01 | agent-browser; browser-use; Playwright injected/backend и CLI sessions; Stagehand; CDP/CSSOM/ARIA | Точные revision/files/tests и переносимые механизмы; не новый агентный stack |
| R02 | AXorcist; Peekaboo; Apple AX/ScreenCaptureKit; Compose Blueprint/probe и Preview limitations | Проверить native identity, каналы и измерительный probe; не внешний CLI как зависимость |
| R03 | AccessKit node/update/consumer; Galen/Extras relations; Compose anchors | Typed delta не строковый diff; partial/unknown не pass |
| A01 | Ui.Vision command runner/player как поведенческий reference; refs/delivery из R01/R02 | Не копировать код при неразрешённых условиях; не переносить unsafe fallback/retry |

Каждый источник читается по механизму с нужными types/tests, не целиком ради
галочки. Ledger: URL+revision → entry files → вывод → adopt/reimplement/reject →
LICENSE/NOTICE выбранного материала → собственная fixture → потребитель.
Выводы старого каталога — вход, а не новое подтверждение runtime или лицензий.
Не изучать сейчас Jev, ML weights, мобильные runtime и UI Atlas без потребителя P0–P7.

## Состояние исполнения после запуска

У каждой строки появятся: `owner_chat_id`, `packet_path`, `epoch`, `write_lease`,
`base_commit`, `status`, `receipt`, `review`, `candidate_revision`, `residual`.
Переходы: proposed → queued → running → review → accepted; rejected → repair.
Ожидания: waiting_resource / waiting_evidence / awaiting_authority с точной причиной,
владельцем и следующим событием; resource recheck через три минуты.
`done` от исполнителя не равен accepted. Историю не раздувать логами.

| Текущие аренды / действия | Значение |
| --- | --- |
| Активные чаты/пакеты/ресурсы | C00 ready; root Git lease для startup checkpoint; остальные ресурсы свободны |
| Последний принятый результат продукта | нет |
| Следующий шаг | dispatch C00, scoped review нормализации, затем R01/R02/R03 |
| Restart | проверить цель и разрешение; восстановить владельцев, epochs, ожидания и следующий готовый пакет |
