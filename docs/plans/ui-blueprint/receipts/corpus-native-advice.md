# Native source advisor handoffs

## CN2 — existing debug routes, 2026-10-11

**CN2 завершён, read-only. Pin: `c7d8e3dcda9fb5c03fc8c0a96c29420d76e450be`.** Проверенные catalog/composition owners без локальных изменений. Готовность сборок и rendering запуском не проверялась.

**Главное: существуют три разных входа, их нельзя смешивать.**

| Вход | Что действительно подключено |
|---|---|
| Mac Debug `--playphrase-scenario ID` | Отдельный `AppScenario` с IDs `c01…c05` → `MacAppComposition` → настоящая Search-композиция с fixture dependencies |
| Именованные SwiftUI `#Preview` | Конкретные view owners, иногда с заглушками video/actions; не обычный app-launch маршрут |
| Release `preview/search` URL | Три существующих Search-компонентных состояния; это специальный разрешённый preview, не универсальный fixture selector |

Основание: [ACP launch contract](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-developer-agent-control-plane/acp-tool-003-playphraselaunchscenario.md), [Release boundary](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-developer-agent-control-plane/acp-rel-001-release-and-platform-invariants.md).

**Для CN-B3: 12 source-wired Debug-входов без нового wiring**

Общий существующий аргумент:

```text
--playphrase-scenario <ID>
```

Предпочтительный уже предусмотренный lifecycle — `playphrase.launch_scenario` с `{"scenario_id":"<ID>"}` на проверенном Debug artifact. Наличие работающего MCP-сервера сейчас не установлено. Эквивалентный launch-аргумент также разбирается самим приложением; не сохранять его в обычной схеме.

| ID | Реально заданное fixture-состояние / компонент |
|---|---|
| `c02.search.suggestions` | Открытые suggestions с обычными строками |
| `c02.search.suggestions-long-copy` | Две действительно длинные suggestion strings |
| `c02.search.suggestions-rtl` | Arabic suggestions и RTL presentation flag |
| `c02.search.loading-stable` | Новый запрос при сохранённых предыдущих результатах |
| `c02.search.results-paused` | Загруженные результаты, paused composition |
| `c02.search.results-long-copy` | `PhraseFixtures.longCopy` в результатах |
| `c02.search.results-rtl` | Arabic phrase и RTL presentation |
| `c02.search.empty-alternatives` | Пустые результаты с suggestions |
| `c02.search.offline` | Сохранённый контекст с typed offline failure |
| `c02.search.request-failed` | Сохранённый контекст с malformed-response failure |
| `c02.search.recovery-long-copy` | Failure поверх длинного сохранённого контента |
| `c02.search.recovery-rtl` | Failure поверх RTL-контента |

Цепочка подтверждена исходниками:

- [AppScenario IDs и parser](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/Packages/PlayPhraseScenarioCatalog/Sources/PlayPhraseScenarioCatalog/DeveloperAppScenario.swift)
- [Debug selection → makeC02Fixture](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/Application/Composition/MacAppComposition.swift)
- [конкретные состояния и synthetic transport](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/Packages/PlayPhraseFixtures/Sources/PlayPhraseFixtures/SearchJourneyScenarios.swift)
- [actual app root](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/MacApplicationRootView.swift)

Это **12 достижимых конфигураций входа, не доказательство 12 различных финальных картинок**. Консервативно здесь восемь содержательных семейств: suggestions, длинные suggestions, RTL suggestions, retained loading, paused results, длинные results, RTL results, empty alternatives; failure/recovery добавляют варианты, если текущая presentation действительно их показывает. После запуска нужно подтвердить фактическое состояние, а не принять имя scenario за результат.

**Дополнительные существующие пути**

- **Mac Release, три preview states:**
  `playphraseme://playphrase.me/preview/search?v=1&state=request-failed`
  Аналогично `state=no-results` и `state=search-prompt`.
  [Точный QA recipe](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/search-state-preview-links.md), [view owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Presentation/SearchStatePreviewView.swift). Retry здесь локальный; `request-failed-live` — другая, изменяющая текущий запрос операция. Эти три состояния уже были в CN1 — повторно не считать.
- **iPad Debug `--account-preview`:** подключён из `PlayphrasemeTabletApp`; host предлагает **Premium / Free / Load failure** и монтирует настоящий `TabletAccountView` с synthetic dependencies. [Host и flag](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Account/TabletAccountPreviewHost.swift). Это не production auth и не QR-login; согласовать с CN-B2, чтобы не дублировать.
- **Mac Debug `--playphrase-text-size large-200`:** parser и app entry подключены. Это специальный масштаб fixture, **не native Dynamic Type**. Один ID `…large-text` сам по себе не доказывает изменённый размер текста. [Parser](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/AppScenario.swift), [entry](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/PlayphrasemeMacApp.swift).

**Что реально использует именно FeatureFixtures**

| Fixture ID | Существующий renderer | Ограничение |
|---|---|---|
| `clip-normal` | [Mac grid: обычный/narrow previews](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Grid/ClipSearchGridView.swift), [viewer preview](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Viewer/ClipSearchViewerView.swift) | Preview callbacks частично no-op; viewer получает `content:nil` |
| `clip-normal` | [iPhone inspector preview](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/ClipSearchInspector.swift), Phone/Tablet grid/card previews | Xcode preview, не доказанный launch selector |
| `reels-loading` | [Mac Reels root preview](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/Features/Reels/Components/Root/ReelsRootView.swift) | Synthetic transport, не реальная сеть |
| `reels-paused` | [Mac slide preview](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/Features/Reels/Components/Slide/ReelsSlideView.swift) | Video — `Color.black`, action bar — `EmptyView`; нельзя считать полным Reel |
| `reels-normal` | [Tablet ordinary/fullscreen card previews](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Reels/Components/Slide/TabletReelsSlideView.swift) | Video/actions заменены preview content; только частичная композиция |

**Что пока только данные**

В выбранных Mac/iPad/iPhone owners не найден универсальный consumer `ComponentFixtureCatalog.descriptors`; `FeatureFixtures.search` не имеет прямого app/preview consumer. Большинство `clip-*` и `reels-*` loading/empty/error/long-copy/RTL IDs не подключены к generic launch selector. Нельзя передать `--playphrase-scenario clip-empty`: это другой каталог.

Более того, [developerCatalog](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/Tools/PlayPhraseMCP/Sources/PlayPhraseMCPServer/DeveloperScenarioMetadata.swift) строится из `AppScenario.allCases` и маркирует их `search_and_learn`; наличие enum surface `clip_search`/`reels` не создаёт соответствующих scenarios.

**Минимальный gap:** для неподключённых FeatureFixtures нужен отдельно разрешённый test-only selector/host, который выбирает существующий ID и передаёт его существующей production-композиции с полными безопасными dependencies. Для имеющихся `#Preview` сначала можно использовать текущий preview-host workflow, но маркировать заглушки и частичность. Универсальный renderer или новый benchmark framework для этого не требуется.

Для следующей партии рекомендую **сначала перечисленные 12 Debug-входов C02**, после вычитания дублей CN-B1. Это расширение состояний настоящей Search-композиции; пока **source-wired synthetic cases, не runtime acceptance и не measured geometry**. Файлы, runtime и QR-login не затрагивались.

## CN1 — source advisor handoff

Recorded by root from the terminal receipt of Спроектировать UI Blueprint, 2026-10-11. Discovery accepted; runtime and source-version limitations retained.

**CN1 завершён: 25 подтверждённых сценариев, две партии — 12 + 13.** Это 25 разных компонентных задач, а не 25 уже готовых blueprints. Build/run/UI/capture и изменения файлов не выполнялись.

Сначала важные границы:

- **G** — есть сохранённые canonical geometry + PNG.
- **V** — есть исторические PNG/AX-семантика, но внутренних measured bounds нет.
- **S** — сценарий подтверждён контрактом/исходниками; новый runtime dataset ещё нужен.
- Для будущего сбора: Mac — внешний AX в `pt` плюс независимый native PNG; iPhone/iPad — настоящий Simulator, семантика доступного developer-инструмента и native PNG. Поддержку мобильного collector это не доказывает.
- Probe PlayPhrase.me для всех перечисленных компонентов **не подтверждён**. Наличие SwiftUI anchors в отдельных owners не означает готовый экспорт geometry.
- Auth, покупка, экспорт, Favorites mutations, Reveal чувствительного контента и внешние ссылки не входят в безопасный проход. При gate — сохранить ограничение, не обходить его.

**Партия A — Mac Search и Clip Search, 12 сценариев**

Общий entry: canonical Mac, установленный исходный paused state; exit — закрыть созданную presentation, восстановить размер, выбранную вкладку и scroll. Не подменять другие действующие runtime owners.

| ID / состояние | Безопасный entry → exit | Компонентный вопрос / измерения | Точные основания и готовность |
|---|---|---|---|
| **A01 Search resize** | Paused Search → две поддержанные ширины → исходная рамка | Панели, transport, tabs, query band; ширины, оси, подписанные gaps; AX bounds отдельно от clipping | **G**: [RC03 dataset](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-resize/runtime-20261009.md). [Point sizing@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/point-sizing.md). Два original Observe по 160 узлов/partial; video-only frame и pixel transform неизвестны. |
| **A02 Toolbar и overflow** | Search → сужение → открыть системный overflow → закрыть/восстановить | Высоты и группировка controls, исчезнувшее из inline представления против удалённого; bounds overflow Surface | **S**, [toolbar@9](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/macos-learner-toolbar.md), [owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/Header/View/MacApplicationRootViewToolbar.swift). QA — Acceptance этого leaf. Не активировать Random/Refresh. |
| **A03 Common Phrases: форма + paging + список** | Выбрать вкладку → осмотреть фильтры/страницу → прежняя вкладка | Две строки фильтров, pagination, viewport списка и строки; label/value, scroll bounds | **S**, [Common Phrases@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/learner/common-phrases.md), [форма](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Components/MostCommonPhrases/SearchForm/CommonPhrasesSearchForm.swift). Не выбирать фразу или менять фильтры в первом проходе. |
| **A04 Vocabulary: фильтры и длинные определения** | Vocabulary → scroll существующей страницы → восстановить | Метаданные, определения, filter/pager rows; переносы, ширины колонок, граница viewport | **S**, [Vocabulary@2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/learner/vocabulary.md), [view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Components/CommonWords/View/VocabularyView.swift). Выбор слова меняет Search — исключить. |
| **A05 Grammar: home → таблица времени** | English → Grammar → существующая страница времени → home/прежняя вкладка | Табличные колонки, заголовки, строки и примеры; выравнивание, переполнение | **S**, [Grammar clauses@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/learner/favorites-grammar-statistics.md), [таблица](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Components/Grammar/View/GrammarTableView.swift). Не нажимать примеры, запускающие Search. |
| **A06 Settings: вложенная форма** | Command-comma → открыть один picker без выбора → Escape/Close | Outer shell/header/inner panel, label-control pairs, вложенное меню, scroll | **V**, [RC01](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-settings/README.md); текущая норма [service dialogs@11](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/macos-service-window-glass.md), [view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Components/Settings/View/SearchSettingsView.swift). QA `MAC-GLASS-SETTINGS`; значения не менять. |
| **A07 Genre popover** | Clip Search → Genre → Escape | Две колонки вариантов, trigger/anchor/popup; визуальный checkbox против AX role | **V**, [RC02](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-filters/README.md), [popover@1/r2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-08-26-macos-native-filter-popover-evolution.md), [picker](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Filters/PickerView.swift). Selection/Clear исключены. |
| **A08 Director: idle → lookup results** | Director → разрешённый поисковый draft → результаты без выбора → dismiss | Input, guidance, counted list rows, focus; popup placement относительно trigger | **V только idle**, те же [RC02 данные](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-filters/observations.json) и [контракт](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-08-26-macos-native-filter-popover-evolution.md); [search field](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Filters/PickerSearchField.swift). Lookup вызывает сеть; draft не applied filter. |
| **A09 Cast multiselect presentation** | Cast → осмотр текущих tags/вариантов → Escape | Несколько выбранных tags, remove affordance, список counts/checkmarks | **S**, [контракт](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-08-26-macos-native-filter-popover-evolution.md), [chip](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Filters/PickerChip.swift), [option row](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Filters/PickerOptionRow.swift). Если выбранных tags нет, не создавать их без отдельного setup. |
| **A10 Actor Voice single-select** | Voice → осмотр текущего состояния → dismiss | Отличие zero-or-one selection от Cast; label/count/selected semantics и row bounds | **S**, [filter behavior](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/user-visible-behavior-part-2.md), тот же [picker owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Filters/PickerView.swift). Не считать Cast доказательством говорящего. |
| **A11 Adaptive Clip grid и scroll** | Загруженная сетка → scroll/resize → восстановить | Rail/grid/query topology, 16:9 cards, колонки/gaps, viewport против offscreen rows | **S + историческая QA**, [lightweight cards@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/macos-lightweight-cards.md), [grid](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Grid/ClipSearchGridView.swift), [QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/runs/2026-09-07-macos-clip-adaptive-grid.md). Hover запускает preview — отдельный разрешённый state, не безусловно read-only setup. |
| **A12 Immersive Clip viewer** | Eligible card → viewer → явная Pause при необходимости → Close | Полноэкранная внутри окна композиция, media/metadata/transport/footer, границы действий | **S**, [viewer@3](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-08-20-macos-immersive-viewer-evolution.md), [owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Viewer/ClipSearchViewerView.swift). Возможен playback; не переходить через access boundary, не Download. |

**Партия B — help, дополнительные Mac-композиции и настоящие мобильные поверхности, 13 сценариев**

Каждая строка независимо стартует из записанного исходного состояния; партия не требует проходить A подряд.

| ID / платформа / состояние | Entry → exit и предмет измерения | Основания, риск и готовность |
|---|---|---|
| **B01 Mac Search component previews** | Открыть существующий preview URL для `request-failed`, `no-results` или `search-prompt` → Close/Escape. Card/icon/text/Retry и их отношения | **S**, [preview@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/state-preview-links.md), [точные URL/QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/search-state-preview-links.md), [view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Presentation/SearchStatePreviewView.swift). Это actual component preview, не настоящая network failure. `request-failed-live` исключить из безопасной первой партии. |
| **B02 Mac Guest Tour full map** | Ручной Tour → полная карта → штатное закрытие. Карточки, реальные anchors, connectors, группировка/пересечения | **S, отдельный подготовительный gate**, [Guest Tour@7](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/guest-tour.md), [полный обязательный plan/QA01–QA18](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/plans/guest-tour-full-map-spec-and-plan.md), [owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Components/Tooltip/View/PlayphrasemeGuestTourMap.swift). Геометрия недавно переоткрыта; перед запуском нужен полный closure с readable-connectors. Не выдавать старый Accepted record за текущий pass. |
| **B03 Mac Phrase Context** | Exact paused phrase → Context при доступе → Close. Strip, выбранный clip, перевод, Close и video region | **S**, [context@5](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/playback/context-safety-download.md), [view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Components/PhraseContext/View/PhraseContextView.swift). Gate не обходить; не выбирать соседей. Возврат должен сохранять исходную Search identity и паузу. |
| **B04 Mac Download form, без экспорта** | Download текущей фразы → форма/help → Close. Segmented modes, conditional rows, submit/disabled, inner panel | **S**, [download@4](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-shared-services/download-export-legal-and-support.md), [service@11](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/macos-service-window-glass.md), [view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/DownloadVideo/DownloadVideoView.swift). QA `MAC-GLASS-DOWNLOAD`; Submit/Save и изменение persisted options исключить. |
| **B05 iPhone Vocabulary detail** | More → Vocabulary → чтение/scroll → dismiss. Definition/translation/metadata и отдельные Search/Favorite/Copy | **S + историческая QA**, [Vocabulary@2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/learner/vocabulary.md), [owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Search/CommonWordsView.swift), [IPH-011 QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/runs/2026-09-08-iphone-vocabulary-iph011.md). Не Favorite/Copy/Search selection. Large-text state требует отдельного reversible setup. |
| **B06 iPad Common Phrases Filters** | Landscape → Filters → Done/Close. Полная форма, summary `+N`, anchor, paging | **V**, [RC04](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/ipad-filters/README.md), [filters@3](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/ipad-learner-filter-controls.md), [panel](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Search/Learner/Controls/TabletLearnerFilterPanel.swift). Историческая тройка условий уже есть; заново менять фильтры необязательно. Bounds пока неизвестны. |
| **B07 iPad navigation sidebar** | Sidebar toggle → раскрыть Learn → свернуть/закрыть. Overlay/list/disclosures против неизменного detail | **S**, [iPad behavior](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/ipad-behavior.md), [sidebar owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Navigation/TabletNavigationSidebarView.swift). Не выполнять Contact/Legal/account actions; раскрытие групп — presentation setup. |
| **B08 iPad Settings overlay** | Sidebar Settings → scroll без выбора → Close. Content-fitted outer/inner зоны, safe containment, rows | **S**, [popup-glass@12](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/ipad-popup-glass.md), [view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Settings/View/TabletSettingsView.swift). QA `IPG-SETTINGS`. Старый `form-sheet@2` не определяет нынешнюю topology. |
| **B09 iPad synthetic Account preview** | Только существующий DEBUG `--account-preview` → sample Account → Close/завершить owned run | **S, synthetic**, [разрешённый preview в popup contract](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/ipad-popup-glass.md), [точный host](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Account/TabletAccountPreviewHost.swift). Status, name/email layout, disclosure/scroll; не доказательство auth/entitlement. Реальный Account и QR-login исключены. |
| **B10 iPhone Search + keyboard** | Фокус текущего query без изменения текста → dismiss keyboard. Floating query, keyboard-safe region, active phrase/suffix list | **V для старых кадров**, [query Glass](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/iphone-query-glass.md), [phrase-list@2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/iphone-phrase-list.md), [root](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Search/SearchRoot.swift), [PNG/QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/runs/2026-10-07-iphone-glass-inspection/README.md). Keyboard focus меняет представление, не должен считаться query mutation. |
| **B11 iPhone More sheet** | More → scroll до низа → native swipe dismiss → reopen | **S**, [More@7](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/iphone-more-menu.md), [owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Search/PhoneAppMenuModifier.swift). Quick-action row, scroll content, detent/safe area. Нынешняя норма: large-first, без Close/header; старые схемы с крестиком не эталон. Не активировать строки. |
| **B12 iPhone Reels accessibility reflow** | Один paused Reel → ordinary/accessibility text → reading/action scroll → вернуть размер | **V**, [RC05](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/iphone-reels/README.md), текущий [aligned-column@7](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/iphone-reels-aligned-column.md), [page](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Reels/ReelsSlidePage.swift). Две action rows, selectors, video/reading split; не scroll по видео. Старый RC05 не закрывает long-description и trailing-selector reachability. |
| **B13 iPhone Clip study viewer** | Eligible result → paused viewer → Close | **S**, [viewer@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-09-01-iphone-compact-study-viewer.md), [actual composition](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/ClipSearchView.swift). Media/metadata/phrase/action row, query hidden, exact return. Возможен playback/access gate; не Next/Download/Copy в базовом проходе. |

**Готовые данные и targets**

Проверил наличие файлов, без нового запуска:

- **RC03, 9 октября:** две original canonical responses + два PNG существуют. Есть внутренние AX bounds, selected source keys и 16 результатов Measure. Это лучший первый positive geometry seed.
- **RC01/RC02:** Settings, Genre, Director — шесть original/crop PNG существуют; роли/значения есть, внутренней геометрии и точного AX↔CG mapping нет.
- **RC04:** пять iPad PNG существуют; **RC05:** три iPhone PNG существуют. Это исторические Simulator cases с семантикой, не мобильный geometry dataset.
- Старые версии контрактов внутри этих пакетов сохраняются как provenance. Новые captures сравниваются с текущими leaf-контрактами, а не с устаревшим оформлением.

Существующие цели определяет [platform-shells r7](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/platform-shells.md): `PlayphrasemeApp`, `PlayphrasemePhone` с отдельными Phone/Tablet compositions, `PlayphrasemeTablet` как development target. Фактическую установленность/готовность устройств CN1 не проверял.

[ComponentFixtureCatalog](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/Packages/PlayPhraseFixtures/Sources/PlayPhraseFixtures/ComponentCatalog.swift) содержит DEBUG descriptors для loading/empty/failed/offline/hidden/long-copy/RTL, включая `search-*`, `clip-*`, `reels-*`; [FeatureFixtures](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/Packages/PlayPhraseFixtures/Sources/PlayPhraseFixtures/FeatureFixtures.swift) содержит соответствующие данные. **Это не универсальная подтверждённая команда запуска каждого состояния.** Отдельно подтверждён source owner `learnerVocabularyFirstOpen`: [fixture](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Composition/C05VocabularyFirstOpenFixture.swift); его launch wiring нужно выбрать через действующий Debug route.

**Риски и порядок следующего исполнителя**

1. Начать с **A01**, затем A06–A08: есть реальные сохранённые входы, можно проверить полный путь dataset → blueprint без нового app setup.
2. Для остальных сначала установить exact build/Target/state. Общий QA route: [UI policy](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/ui-tool-policy.md) → [runtime](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-runtime-acceptance.md) → [reservations](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-runtime-lanes.md); для mobile дополнительно [Simulator ownership](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-simulator-runtime.md).
3. Партии независимы по данным, **не по desktop**. Mac canonical/input сериализуются; mobile можно вести на свободном существующем устройстве через адресованную isolated session.
4. iPad — landscape. Наличие mini в default pool не делает его заменой требуемому Pro 11 baseline. Нет подходящего разрешённого устройства — отдельный coverage gap, не повод создавать копии.
5. Screenshot pixels — `px`; AX — объявленные `pt`; declaration в SwiftUI — не measurement. Padding, hit region, material/radius и clipping не выводить из похожих рамок.
6. Для CORPUS-1000 не считать повторный кадр, другой timestamp или случайный текст новым независимым компонентом. Эти **25 seeds** дают содержательные семейства; 1000 уникальных готовых случаев CN1 не доказывает.

**Передача root:** маршруты и риски подготовлены; готовность runtime отдельно от наличия исходников отмечена. QR-login, N06 и пользовательские данные не затрагивались. Коммит не требовался.
