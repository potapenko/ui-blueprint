# AX, capture и SwiftUI semantics

- Node type: leaf; domain: `uib.native-sources`.
- Contract: `UIB.NATIVE-SOURCES@1`; stable clause: `UIB.NATIVE-SOURCES.CONTENT`.
- Authority: Active / Stability: Evolving; norms + historical evidence; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: R02; native identity/capture/notifications.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.REUSE@1](reuse.md).
- Source mapping: TZ 976–981; TZ 997–1004; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-native-sources-content"></a>

### Нативный сбор и действия

| Источник и код для чтения | Что полезно для UI Blueprint | Граница переноса |
| --- | --- | --- |
| [AXorcist](https://github.com/openclaw/AXorcist): [Element hierarchy](https://github.com/openclaw/AXorcist/blob/1b12b55d61c01363f913f17a2e1db732bbb203a3/Sources/AXorcist/Core/Element%2BHierarchy.swift), [Observer lifecycle](https://github.com/openclaw/AXorcist/blob/1b12b55d61c01363f913f17a2e1db732bbb203a3/Sources/AXorcist/Core/AXObserverCenter.swift), [geometry helpers](https://github.com/openclaw/AXorcist/blob/1b12b55d61c01363f913f17a2e1db732bbb203a3/Sources/AXorcist/Search/GeometryHelpers.swift) | Реальные AX-children, альтернативные источники потомков, адресные атрибуты, жизненный цикл уведомлений, геометрические помощники. Читать вместе с Element value/action owners и тестами observer lifecycle. | Альтернативный обход не должен расширять разрешённый scope; эвристика не доказывает идентичность. Переиспользовать идеи/допустимый код внутри macOS-плагина, не требовать внешний AXorcist CLI. Корневой LICENSE объявляет MIT. |
| [Peekaboo](https://github.com/openclaw/Peekaboo): [capture pipeline](https://github.com/openclaw/Peekaboo/blob/43b2fe2a72913e5f61f3996ef4c5873f15be8784/Apps/CLI/Sources/PeekabooCLI/Commands/AI/SeeCommand%2BCapturePipeline.swift), [snapshot validation](https://github.com/openclaw/Peekaboo/blob/43b2fe2a72913e5f61f3996ef4c5873f15be8784/Apps/CLI/Sources/PeekabooCLI/Commands/Shared/SnapshotValidation.swift), [mutation coordinator](https://github.com/openclaw/Peekaboo/blob/43b2fe2a72913e5f61f3996ef4c5873f15be8784/Apps/CLI/Sources/PeekabooCLI/Commands/Shared/SnapshotMutationCoordinator.swift), [exact-window fixture](https://github.com/openclaw/Peekaboo/blob/43b2fe2a72913e5f61f3996ef4c5873f15be8784/Apps/CLI/TestFixtures/ExactWindowCapture/main.swift) | Связь наблюдения и действия, отсутствие/актуальность UI-map, оконная атрибуция, capture pipeline и воспроизводимый стенд. | Наличие снимка само по себе не подтверждает свежесть каждого свойства. Не переносить весь agent/model stack, приложение-хост и разрешения как обязательные зависимости. Корневой LICENSE объявляет MIT; submodules проверяются отдельно. |

### Платформенные первоисточники

- [Apple AXUIElement](https://developer.apple.com/documentation/applicationservices/axuielementref), [batch attributes](https://developer.apple.com/documentation/applicationservices/1462051-axuielementcopymultipleattribute), [messaging timeout](https://developer.apple.com/documentation/applicationservices/1459345-axuielementsetmessagingtimeout), [observer notifications](https://developer.apple.com/documentation/applicationservices/1462089-axobserveraddnotification). Читать для точечных запросов, ошибок свойств и ограниченного ожидания; отсутствие уведомления не доказывает свежесть.

- [ScreenCaptureKit WWDC22](https://developer.apple.com/videos/play/wwdc2022/10155/), [sample capture](https://developer.apple.com/documentation/screencapturekit/capturing-screen-content-in-macos), [frame metadata](https://developer.apple.com/documentation/screencapturekit/scstreamframeinfo). Изучить capture filter, child-window exclusions и преобразование output buffer; не приравнивать isolated capture к видимому рабочему столу.

- [SwiftUI combine](https://developer.apple.com/documentation/swiftui/accessibilitychildbehavior/combine), [ignore](https://developer.apple.com/documentation/swiftui/accessibilitychildbehavior/ignore), [Accessibility Inspector](https://developer.apple.com/documentation/accessibility/inspecting-the-accessibility-of-screens). Нужны для границы семантики и visual layout и ручной проверки контролируемого fixture. Inspector не является обязательным runtime-продуктом UI Blueprint.
