# Приёмка: данные после каждого изменения
Часть [PLAN.UIB.SWIFTUI-GEOMETRY@3](../swiftui-geometry-sdk.md). Все пункты — требуемые проверки, не заявленные результаты.

## Сквозной сценарий на одной сборке
В одном owned процессе: tree/compound query → resize → live query → Search/Clip/Reels → popup → data change → scroll → close/reopen → повторные запросы. Между шагами нет source edits, новой instrumentation или rebuild. Используется установленный публичный workflow.

Проверять original source facts через Xcode Size inspector, controlled source fixture и original canonical bytes в Rust. Screenshot/source constants не подставляются вместо runtime layout. Старый sample не oracle нового размера окна.

## Обязательная матрица
| ID | Кейс | Условие прохода |
| --- | --- | --- |
| C01 | Nested stacks/Text/Image/Button | Настоящие SwiftUI nodes/frames, не только hosting wrappers. |
| C02 | Full root и заранее неизвестный leaf | Без marker edits; full scope не подменён demo subset. |
| C03 | Wide → narrow → wide | Новые actual frames/relations и viewport каждого live request. |
| C04 | Resize во время capture | Нет mixed old/new settled result; drift обозначен, следующий запрос работает. |
| C05 | Search → Clip → Reels → Search | Новые roots доступны той же сборке, старый экран не выдаётся как current. |
| C06 | Popup open/close/reopen | Root/owner/occurrence актуальны; stale ref не выбирает новый popup. |
| C07 | Conditional insert/remove | Текущий состав автоматически; absence только в covered scope. |
| C08 | ForEach insert/remove/reorder, repeated types | Exact instances/ancestry; не случайный индекс. |
| C09 | Lazy list/grid, scroll/reuse | Mounted frames актуальны; unmaterialized не имеет zero/old frame. |
| C10 | Text/locale/font/Dynamic Type | Actual reflow и соседние размеры обновлены при том же ID. |
| C11 | Async content/image load | Pending не назван final; после загрузки live query видит новый layout. |
| C12 | Animation/transition | Bounded settled; sample явно transient; видео не блокирует стабильный диалог. |
| C13 | Nested scroll/scale/rotation/clipping | Source spaces/transforms корректны; bounds не равны видимости. |
| C14 | Несколько окон/popovers | Exact target и origins; нет выбора по title/похожей рамке. |
| C15 | Другой display/scale | Source mapping обновлён, window-local не масштабирован искусственно. |
| C16 | Restart/scene recreation | Старые sessions/refs отказаны; fresh bind восстанавливает работу. |
| C17 | Два агента одновременно | Изолированные request/results, bounded очередь, нет чужого/старого response. |
| C18 | Width/height/gap/inside batch | Одна observation basis, не смесь разных layout epochs. |
| C19 | Большое дерево/pages | Локально полный доступный scope, bounded resources, no silent truncation. |
| C20 | Экран/элемент ещё не создан | Подтверждённый not-mounted/materialized/ready; после открытия без rebuild. |
| C21 | Cancel/busy/deadline/crash/disconnect | Terminal result, controlled cleanup, no late publish или kill чужого app. |
| C22 | Wrong version/token/path, oversized/partial files | Отказ до недопустимого чтения; no overwrite/leak. |
| C23 | Live против saved | Новый source read в live, original time у saved, no cache restamp. |
| C24 | Debug off/Release | Нет active collector/IO/internal-framework linkage; UI/input/playback сохранены. |
| C25 | iPhone/landscape iPad | Same semantics; actual composition/keyboard/safe area/scroll квалифицированы. |
| C26 | Новый обычный View/dynamic leaf | Появляется без дополнительных marker sites или специальных readers. |

Для обычных mounted C01–C11 нужны числа запрошенных layout properties. Unknown/unsupported не закрывает поддержку. Transparent node без layout box — подтверждённая source semantics, не оправдание пропуска измеряемых descendants.

Пройти весь [Mac inventory](integration.md), не только fixture. Именованные экраны — проверки общего механизма. Parameterized tests переиспользуют стандартные узлы вместо отдельной инфраструктуры на каждый screen.

## Что предотвращает повторение проблемы
1. C03–C09/C16/C17/C23 обязательны при изменениях collector/cache/identity/decoder.
2. Новые обычные View/compositions проверяются общей fixture/source inventory; не требуют отдельного integration заказа.
3. Перед сменой Xcode/OS/backend — compatibility smoke: SwiftUI leaves, resize, dynamic insertion, popup, repeated query.
4. App changes проверяют затронутые реальные экраны через тот же installed workflow; пробел не передаётся новому чату замера.
5. Defect закрывается исходным failing case и state-change regression, не увеличением лимита или partial dump.

CI: deterministic decoder/SDK/identity/queue/compatibility tests и applicable builds. Live Mac/Simulator/Xcode acceptance выполняется в доступной среде отдельно; headless tests не выдаются за runtime proof.

## Свежесть и источник
Xcode archive подтверждает content source. Автоматический runtime caller отдельно проверяется на повторяемость, freshness, lifecycle и реальные transitions. Совпавший формат не доказывает работоспособность capture.

Source profile показывает реальные OS/Xcode/backend versions. Desktop успех не доказывает mobile/device/older OS. Поддержка каждой платформы фиксируется по её evidence.

## Стоимость и безопасность
На small/large scopes замерить layout wait/acquisition/normalization/query/transfer, retained bytes/nodes. Повторные запросы не накапливают файлы, descriptors или observers без границ. Evidence не коммитить.

Canary проверяет отсутствие credentials/learner/model text/raw memory в normalized output и errors. Raw archive имеет private retention. Имя «geometry» не доказывает, что upstream не собрал посторонние свойства.

Проверить focus/input/scroll/playback и возобновление после debugger-mode acquisition. Quiet sampling не объявлять без проверки. Если backend требует паузы, отразить и измерить её; пригодность обычному workflow решить по evidence.

## Поставка
Installed CLI/skill делает live query без checkout experiments и ручного Xcode Export на каждое обращение. App/backend/CLI pins совместимы, legacy canonical records читаются, managed update/rollback сохранены.

Privacy/concurrency/compatibility review проводится в реально разрешённом объёме. Self-check/cold recipe не называются independent review. Source release ведёт существующий release owner; native production/App Store publication не входит.

Итог: одна интеграция/сборка, один live workflow, full tree/batch при необходимости, actual matrix и точные limits. Документы, tests, dump и tag не заменяют работающую цепочку.
