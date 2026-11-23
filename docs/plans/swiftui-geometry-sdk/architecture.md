# Архитектура runtime-запроса
Часть [PLAN.UIB.SWIFTUI-GEOMETRY@3](../swiftui-geometry-sdk.md). Наличие candidate library не означает пройденную квалификацию.

## Постоянные роли
| Слой | Ответственность |
| --- | --- |
| App integration | Один opt-in на процесс, scenes/windows, доступ к источнику runtime tree и lifecycle. |
| Acquisition adapter | Новое наблюдение, discovery/selected или full capture, source semantics, deadline/cleanup. |
| Normalizer | Source tree/frames/transforms → canonical Snapshot с provenance; без геометрических догадок. |
| Request coordinator | Exact target, bounded очередь, freshness/readiness, selector resolution, terminal result. |
| Rust engine | Существующие size/gap/alignment/containment/diff/check. |
| CLI/skill | Find-and-measure/tree, compact output и optional retention исходного sample. |

Это логические роли, не требование шести модулей/сервисов. Расширять существующих владельцев; не создавать второй geometry engine или продуктовый state.

## Квалификация источника E1
| Кандидат | Проверка | Условие выбора |
| --- | --- | --- |
| Runtime Xcode hierarchy backend через development-only hook; кандидат LNViewHierarchyDumper | SwiftUI leaves, freshness после изменений, frames/space, bounded вызов, sandbox/signing и data minimization. | Основной кандидат, если работает без per-element markers и ручного GUI на каждый запрос. |
| Автоматизированный штатный Xcode capture/export | Reference data и repeatable automation, стоимость/пауза/cleanup. | Backend только после qualification; ручной экспорт — доказательство источника, не итоговый workflow. |
| Существующий SDK с общей instrumentation | Конкретные доказанные gaps первых источников, глобальное подключение и автоматический сбор новых instances. | Запасной путь; не ручные исправления под каждый поступивший вопрос. |

AX отвечает на явно запрошенные accessibility bounds и не заменяет SwiftUI layout. Не переключаться на другой источник после отказа незаметно и не смешивать frames разных источников.

Одна общая fixture: nested stacks, Text/Image/Button, повторения, conditional branch, scroll/lazy content, presentation. В одном процессе проверить baseline, resize, dynamic changes и popup open/close. Native-владелец повторяет на реальном приложении. Новая fixture для каждого экрана продукта не нужна.

## Одна интеграция и транспорт
Предпочитаемый режим — один debug-only app-level hook, вызывающий квалифицированный runtime collector. Пакет/зависимости и scene binding подключаются один раз; не генерировать новые диагностические данные вручную для каждого View.

Переиспользовать проверенные части текущего app-owned ingress/output, сняв Search-only ограничение. Выбрать один транспорт для repeatable quiet requests. Приватный файловый канал с completion marker допустим после sandbox/delivery proof; новый daemon/HTTP server/публичная настройка не требуются.

Если backend умеет только .viewhierarchy на диск, archive является временным внутренним результатом конкретного live request. Adapter читает, нормализует и освобождает eligible scratch после потребителей. In-memory response предпочтителен, если реально доступен. Постоянное сохранение каждого sample не обязательно.

Session descriptor сообщает app build, backend/SDK/CLI protocol/capabilities, run и реальные окна. Unsupported version/wrong generation отказывают до доступа к UI. Read-only request не запускает приложение и не захватывает чужие процессы.

## Свежесть и кеш
Первая реализация делает новый source read каждого live request по [контракту](request-contract.md). Корректность не зависит от полноты SwiftUI change events.

Кешировать можно decoder/type metadata, static mappings и historical observations. Geometry cache/deltas добавляются позже только с доказанной invalidation/revalidation и теми же регрессиями. Не рестампить старый sample.

Полное дерево доступно одним логическим запросом. Selected acquisition снижает стоимость при поддержке источником; иначе full capture выполняется локально, затем фильтруется. Агент получает требуемую часть, а не весь JSON по умолчанию.

## Geometry
Каждый frame сохраняет kind/units/origin/space/transform. Layout, AX, paint/visibility, native frame, glyph и screenshot pixels не взаимозаменяемы. Transparent group без собственной layout box не получает рамку соседа; её measured descendants сохраняются.

Relations считаются только в подтверждённом общем пространстве. General affine нельзя получить из отношения ширин bounding boxes. Scroll/rotation/scale/cross-window требуют source mapping; одинаковое имя space не доказывает одинаковый origin.

Перенос между дисплеями обновляет screen mapping/scale; window-local geometry не масштабируется искусственно. Compound request не смешивает старую кнопку с новым popup. GPU/presented-frame atomicity не обещается.

## Конкуренция и стоимость
Одна acquisition одновременно на target/run, независимые targets не блокируются. Очередь bounded, ожидание входит в deadline. Batch использует один sample. Независимые requests можно объединить только до начала общего сбора при совместимых условиях; поздний request не получает ранее начатый capture как новый.

Полное дерево хранится локально с bounded memory/bytes/depth/decompression; output pagination привязана к snapshot. Устаревший cursor не продолжается в новом дереве. Большой обязательный экран входит в capacity proof; тихая обрезка ради success запрещена.

Замерить layout wait/acquisition/normalization/query/transfer отдельно. Сначала корректный repeated path, затем оптимизация. Никакого периодического background capture без запроса.

## Приватность и отказ
Публичный output — разрешённые structural metadata и requested geometry. Raw Xcode archive может содержать images/text/models: private evidence, allowlist до публичного вывода, без передачи третьим сторонам.

У каждого capture новый owned destination; не использовать destructive overwrite из upstream без исправления. Публиковать только complete result с проверенным binding. Cancel/crash/timeout не разрешают убийство приложения или successful partial data.

При adoption библиотеки проверить pinned license/source, phases, broad property requests, loaded frameworks, bounded execution и cleanup. Internal framework dependency исключить из Release. Совместимость Xcode/OS/backend проверяется до распространения обновления.
