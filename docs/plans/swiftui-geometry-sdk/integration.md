# Одна интеграция для всех экранов
Часть [PLAN.UIB.SWIFTUI-GEOMETRY@3](../swiftui-geometry-sdk.md). Native владеет app changes и реальными замерами; Blueprint — общим инструментом и project fixtures.

## Требование к приложению
Один development-only package/hook выбранного backend, один вход запросов, actual scenes/windows и bounded output. Использовать существующего diagnostic owner. Приложение не пишет geometry engine или отдельный serializer каждого экрана.

Сбор не ограничивается Search & Learn и списком вручную выбранных проблемных controls. Windows/presentations обнаруживаются в runtime. Новая строка/диалог появляется в subsequent capture без collector edits. Отдельные roots — внутренняя деталь одного запроса с owner relations и source spaces.

Если выбранному источнику нужна app linkage, добавить её один раз всем фактическим target consumers. При доказанно работающем внешнем backend не внедрять библиотеку только ради первоначального предположения.

## Build
- Одна сборка на app/backend revision. Измерения, navigation и новые runtime elements не запускают rebuild.
- Mac/iOS binaries различаются по платформам, но не по запрашиваемому элементу или чату.
- Сохранить ordinary Release schemes, signing/sandbox, пользовательские stores и endpoints. Не ослаблять их незаметно ради collector.
- Исключить diagnostic API/activation/IO и internal-framework embedding/linkage из Release; одного guard вокруг вызова недостаточно.
- Dependency pin/tag/lockfile и совместимый CLI обязательны. Local override только для разработки с точным source pin.
- macOS14/iOS17 floors сохраняются; затронутый tvOS17 shared code должен компилироваться. Compile target не равен runtime qualification.

Native helper один раз подготавливает owned instance и private session descriptor. Потребителю не нужны source offsets, process allowlists или авторские extraction scripts.

## Inventory всего Mac-приложения
Разово сверить все destinations/window/presentation routes по исходникам. Таблица — обязательные семейства regression coverage, не allowlist collector.

| Семейство | Проверяемое содержимое |
| --- | --- |
| Window/toolbar | Content, toolbar roots/controls, safe-content и mappings. |
| Search & Learn | Query/suggestions/video/transport/study, active panels и overlays. |
| Library/Grammar/Statistics | Filters/details/rows/tooltips, empty/populated states. |
| Random/Actors/Practice | Полные открытые диалоги: containers, labels/icons/buttons/fields, lists/grids, headers/footers/keycaps/disclosure. |
| Guest Tour | Mounted cards/targets, viewport/groups, relations/transforms. |
| Clip Search | Filter roots/Genre popup, grid/scroll viewport/viewer. |
| Reels | Active card/media/title/actions, replacement и overlays. |
| Access/services | Paywall/CTA/preview cells, settings/account/auth/subscription/download/notices без вывода credentials/models. |

App-owned content и внешние system dialogs/menus различаются. System surface получает собственный source/binding; рамка родителя не доказывает её измерение. Неэкспонируемые internals перечислять по evidence, а не списывать весь SwiftUI как закрытый.

## Новые и динамические Views
Repeated elements собираются по фактическим mounted occurrences. Новый instance поддержанного View не требует static registration. Новый стандартный View в исходнике должен попадать тем же механизмом; semantic alias необязателен для frames.

Если fallback требует instrumentation исходников, выполнить её по всем реальным UI owners как одну интеграцию и автоматически выявлять пропуски. Не обещать compiler pass до проверки View identity/layout. Такой путь принимается только если последующие стандартные композиции не требуют ручных патчей под каждый вопрос.

Custom renderer/framework может требовать общего adapter extension. Это новый класс источника, а не новый collector каждого экрана. Потеря поддержанного SwiftUI узла считается регрессией.

## iPhone/iPad
PlayphrasemePhone — distribution target обеих idioms; существующие Phone/Tablet compositions остаются раздельными. PlayphrasemeTablet — standalone development target; PlayphrasemeTabletShell компилирует Tablet sources. Проверить target membership/dependency linkage без копирования исходников.

Переиспользовать request/response, normalization и Rust query. Отличаются developer connection, scene binding и доставка данных. Не переносить Mac модели/UI или spacing requirements в mobile.

Existing Simulator pool: доступный iPhone и landscape iPad, без новых persistent devices. Distribution target проверяется на обеих idioms; standalone Tablet — affected compatibility. Keyboard/safe area/scroll/size context входят в матрицу.

Physical device квалифицируется отдельно с exact device/process/developer prerequisites. Его отсутствие не задерживает Mac/Simulator, но не позволяет объявить physical support. Возможность devicectl file copy сама по себе не доказывает hierarchy capture.

## Ownership, storage, handoff
Native-чат согласует общие MacBlueprintProbeHost/MacApplicationRootViewBlueprint и окна с действующим Mac-владельцем. Отдельные чаты под замеры не создаются. Регрессии инструмента — на Blueprint fixtures.

Runtime/app/package — internal temp; compiler targets/caches/heavy scratch/retained evidence — verified mounted external volume. Ownership/retention notes обязательны. Не менять live peer instances или installed user apps.

Raw output сначала private app-owned, затем selected evidence копируется во внешний task area. Не рассылать debugger archive целиком. Удалять только owned disposable files после consumers; retained reference сохранять.

Handoff: app/backend/CLI pin, один preparation path, одна live-команда, actual matrix. Старые frozen builds остаются historical evidence, а не причиной поддерживать их частными читателями вместо разрешённой новой общей интеграции.
