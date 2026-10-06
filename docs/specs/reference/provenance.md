# Исходные документы и исторические основания

- Node type: leaf; domain: `uib.provenance`.
- Contract: `UIB.PROVENANCE@1`; stable clause: `UIB.PROVENANCE.CONTENT`.
- Authority: Active / Stability: Evolving; norms + historical evidence; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: сверка происхождения норм или сценариев.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: none.
- Source mapping: TZ 1–12; TZ 812–867; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

Ссылки на другие проекты — provenance, не requires. Их содержимое не открывалось пакетом C00. Сохранённые утверждения об исследованиях относятся к исходному документу, не к новому запуску.

<a id="uib-provenance-content"></a>

# UI Blueprint общее техническое задание

Источник: [Page](https://chatgpt.com/space/page_5e1c4e973b0c819181b4a4513072b750). Копия от 6 октября 2026 года.

Техническое задание 1.4 от 6 октября 2026 года. Контракт `UIB.TZ@1.4`, Authority: Active, Stability: Evolving. Пользователь подтвердил документ как полноценное ТЗ и снял статус Draft в чате подготовки разработки 6 октября 2026 года. Документ объединяет веб- и Mac-перспективы и является нормативной основой планирования и реализации. Явно открытые инженерные решения D01–D07, предварительные имена и будущие этапы сохраняют обозначенные границы. Подтверждение ТЗ не запускает разработку, инструментирование или QA приложения: запуск следует согласованному плану. Редакция дополнена заданием для будущего агента-разработчика и каталогом обязательного чтения исходников. Добавлены Ui.Vision и UI Atlas, а также отдельное будущее направление атласа приложения. Расширен основной режим инженерного blueprint и связанное руководство; service host, MCP и dashboard описаны как будущие этапы.

UI Blueprint — локальный инструмент для наблюдения, измерения и управления реально работающим интерфейсом. Он превращает доступные runtime-данные в компактную семантическую и геометрическую модель, выполняет расчёты обычным кодом и отдаёт агенту выбранный участок или изменения. Человеку он готовит объяснение и промпт для ImageGen.

Собственные Rust-ядро, аналитика, кэш, diff и CLI работают без обязательной модели. Web, macOS, Android и iOS/iPadOS подключаются отдельными плагинами. Готовые blueprint-продукты служат источниками подходов и переносимой функциональности; они не становятся обязательной цепочкой внешних CLI. Playwright, браузерные протоколы и системные SDK допустимы как средства доступа.

Исходные документы сохраняются отдельно: [веб-видение 0.2](https://chatgpt.com/space/page_06b1be9161288191a37c8cd1512446b7) и [Mac-видение 0.1](https://chatgpt.com/space/page_f33493d6801881919fdb50a2de4c1888). При расхождениях новый документ отражает последние решения пользователя и согласование соавторов, не переписывая историю. Это ТЗ отдельного инструмента; его создание не изменяет приложения, не создаёт репозиторий и не подтверждает runtime-приёмку.

## Источники и принятые границы

Исходные видения: [Web 0.2](https://chatgpt.com/space/page_06b1be9161288191a37c8cd1512446b7) и [Mac 0.1](https://chatgpt.com/space/page_f33493d6801881919fdb50a2de4c1888). Они сохранены как самостоятельные документы. Совместная работа выполнена чатами «Research website UI blueprint» и «Спроектировать UI Blueprint» по прямому поручению пользователя.

Обязательность Rust, собственных аналитических модулей, платформенной плагинности, минимальных зависимостей, кэша/diff, model-free основы и ImageGen-промпта — решения пользователя. Равноправные обязательные пилоты отражают его последующее уточнение против веб-уклона. Детали schema, протокола и узкого native probe — совместное техническое предложение для проверки в проекте реализации.

Веб-источники зафиксированы исходным исследованием на ревизии 9cec209da010df4fe20a9fc028f4be02ba8186a7. Mac-соавтор повторно проверил N01–N06 и обновлённую policy на HEAD 48d0dfd1cd0fe46ac88c073e99ffab7bd1abe76e. Эти ссылки дают основания сценариев, а не отчёт о новом runtime QA.

### Веб контракты и QA

- [Фильтры и элементы управления](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/phrase-search-filters/surface-controls.md) и [мобильное поведение фильтров](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/phrase-search-filters/mobile-source-and-response.md) — W01–W06/W09.

- [Настройки](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/settings/overview-and-behavior.md), [инварианты](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/settings/invariants-and-failure-policy.md) и [состояние и QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/settings/state-qa-and-references.md) — W07.

- [Мобильная компоновка](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/mobile-layout/overview-and-behavior.md) и [инварианты/QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/mobile-layout/invariants-failure-route-and-qa.md) — W08/W10.

- [Локализация и направление](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/localization/translation-and-direction.md) — W06/W11.

- [Desktop filter QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-desktop-filters-and-suggestions-panel.md), [mobile filter QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-mobile-filters-navigation-panel.md), [locale filter QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-filters-follow-interface-locale-with-canonical-values.md).

- [Persistence QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-settings-modal-controls-persist.md), [mobile control size QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-mobile-settings-fixed-toggle-and-select-widths.md), [focus handoff QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-mobile-selection-blurs-input-and-attempts-playback.md).

- [Browser routing](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/browser-routing.md), [IAB workflow](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/in-app-browser-qa-workflow.md), [Simulator Safari workflow](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/mobile-safari-simulator-qa-workflow.md).

### Native контракты и QA

- N01: [query-binding@2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/input/query-binding.md) и [autofocus@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/input/destination-autofocus.md).

- N02: [macOS filter popover@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-08-26-macos-native-filter-popover-evolution.md), source r2.

- N03: [iPad filter popover@2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-08-26-ipad-native-filter-popover-evolution.md).

- N04: [learner filters@3](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/ipad-learner-filter-controls.md).

- N05: [point-sizing@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/point-sizing.md).

- N06: [aligned-column@5](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/iphone-reels-aligned-column.md).

- N07: [Runtime acceptance](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-runtime-acceptance.md) и [native-ui-tools@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/ui-tool-policy.md).

### Платформенные источники и аналоги

- [WAI-ARIA](https://www.w3.org/TR/wai-aria-1.2/), [Compose semantics](https://developer.android.com/develop/ui/compose/accessibility/semantics), [SwiftUI combine](https://developer.apple.com/documentation/swiftui/accessibilitychildbehavior/combine), [SwiftUI ignore](https://developer.apple.com/documentation/swiftui/accessibilitychildbehavior/ignore).

- [DOMSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/), [CSSOM View](https://www.w3.org/TR/cssom-view-1/), [Playwright actionability](https://playwright.dev/docs/actionability).

- [AXUIElement](https://developer.apple.com/documentation/applicationservices/axuielementref), [AX multiple attributes](https://developer.apple.com/documentation/applicationservices/1462051-axuielementcopymultipleattribute), [AX messaging timeout](https://developer.apple.com/documentation/applicationservices/1459345-axuielementsetmessagingtimeout), [ScreenCaptureKit](https://developer.apple.com/videos/play/wwdc2022/10155/).

- [Blueprint](https://github.com/popovanton0/Blueprint), изученная ревизия 09287ace7ea4a79cb5a1a08521cd69a365f7bdac; [Blueprint Compose Preview](https://github.com/GusWard/Blueprint-Compose-Preview), ревизия 95398528a6d5b1f4fa4ce92e1a73f830608eb3fb.

- [Galen](https://galenframework.com/docs/reference-galen-spec-language-guide/), [AccessKit TreeUpdate](https://docs.rs/accesskit/latest/accesskit/struct.TreeUpdate.html), [Peekaboo see](https://github.com/openclaw/Peekaboo/blob/main/docs/commands/see.md), [AXorcist](https://github.com/openclaw/AXorcist).

- [Jev](https://docs.typesafe.ai/introduction), [Screen2AX](https://github.com/MacPaw/Screen2AX), [OmniParser](https://microsoft.github.io/OmniParser/). Последние два — исследовательские направления, не обязательные зависимости.

Локальные ссылки требуют соответствующего checkout. Перед выполнением сценария проверяются текущая ревизия его контракта и разрешённый backend. Исследование и согласование документа не подтверждают скорость, работоспособность адаптеров или приёмку приложения.
