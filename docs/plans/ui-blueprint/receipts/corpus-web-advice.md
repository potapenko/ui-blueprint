# Web source advisor handoffs

## CW3 — existing isolated preview discovery, 2026-10-12

Advisor01a1102f-e21d-7251-9597-c29a1c66d088, terminal turn
01a12d8b-f244-74d3-a394-97b32b790c04. Source-only, no runtime or new cases.
Source remains c23ef963e5be87195f80cef7173d537a5a146c74, branch new-banners-paywall;
preexisting env/dev/resources/config.edn untouched. No general component gallery.

One useful candidate: `http://localhost:3000/?demo=subscription`, two local
autorenewal states. [Runbook](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/ops/yookassa-subscription-demo.md),
[dev entry](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/env/dev/cljs/playphraseme/app.cljs),
[fixture](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/env/dev/cljs/playphraseme/subscription_demo.cljs).
Existing loopback guard/dev entry bypasses core/init!, renders actual subscription
content with fictional fixed data, and replaces actions with a local atom toggle;
no billing, clipboard, Favorites or preference writes from those handlers. Enabled
and disabled content differ meaningfully. Runtime/dev-build correspondence still
must be proved before any click; a normal-app remount ends the fixture scenario.
Shared home.html includes Google/Firebase analytics: this is not offline isolation.
Ordinary page telemetry is already within earlier Web capture authority; it does
not authorize any subscription/account operation. No network-bypass modification.

Other findings are not ready isolated previews: show_ui paywall/login follows
normal auth/preload/marketing; locale override persists preference. Search fixture
replaces runtime transport/capabilities and storage. CLJS renewal/checkout render
tests are data/code, not live gallery or captured geometry. No harness creation
authorized. Discovery covered env/dev, startup registry, package/project config,
templates, tests/fixtures and QA scripts, plus source spec/QA dependencies.
Consumer: finite CW-B7 on the two already-existing demo states, using own context
and source/geometry/image verification. No new release/billing acceptance claim.

## CW2 — additional families, 2026-10-11

Root received this read-only source handoff from Research website UI blueprint.
Advisor-local IDs CW25–32 are remapped below to corpus IDs CW43–50 to avoid
colliding with already captured CW-B3 and active CW-B4. No new runtime proof.
The mistyped username in one QA link was corrected after checking the file exists.

**CW2: `DONE_READ_ONLY`. Нашёл 8 дополнительных рецептов — примерно 7 семейств, не ещё 12 гарантированно доступных экранов.** Новых captures нет. CW01–24, нижние части прежних форм и текущая CW-B4 исключены.

Pin всех перечисленных site-source/spec/QA: **`c23ef963e5be87195f80cef7173d537a5a146c74`**. Это source basis, не подтверждение загруженного build.

База будущего сбора — существующий `http://localhost:3000`, собственный гостевой контекст. Канонический Reels URL: `/#/reels/en?source=common-phrases`. Старые QA-ссылки `/#/reels` — legacy entry, не новая рекомендация.

| ID | Новый случай; вход → выход | Геометрическая польза и отличие | Доступность / ограничения |
|---|---|---|---|
| **CW43** | Reels desktop, один активный reel → зафиксировать paused state → закрыть свою вкладку | **Однорядная action bar**: icon group, spacer, translation/auto-advance selects, описание и side navigation. Общие края и распределение остаточной ширины. Это другой shell, не Clip Search viewer | Нужен реально загруженный reel. Не нажимать speed/repeat/Favorites/download/copy. Возможны autoplay и telemetry |
| **CW44** | Тот же Reels в подтверждённой mobile layout → закрыть контекст | **Двухрядная action bar**: primary/secondary rows, правое выравнивание selects, video–description–toolbar spacing. Засчитывать только реальную смену композиции | Responsive browser, не native mobile. Не считать ещё одним случаем каждую ширину |
| **CW45** | Reels → `Search phrase.` → наблюдать пустую открытую панель → `.reels-backside-close` | **Search backside**: heading/subtitle, autofocus input, submit и suggestion area внутри отдельной panel composition. Отличается от inline autocomplete CW-B4 | Не вводить и не выбирать подсказку. Loading/empty/results отмечать только фактически наблюдёнными |
| **CW46** | Reels с source Common Phrases → `Open filters.` → закрыть крестиком **без Apply** | **Reels filter form**: pagination над grid полей, label/select pairs, footer. Отличается от learner-панели Search и mobile Clip Search Filters | **Условный рецепт:** QA применения требует authorized session; в прочитанном open-handler нет отдельного auth-gate. Доступность именно гостевого открытия не проверена. При gate — остановить, не логиниться |
| **CW47** | Reels desktop → hover `Select collection` → убрать pointer | **Один контекстный tooltip action bar**, его anchor и fit относительно нижней части reel. Это не полный Guest Tour cluster | Слабее по разнообразию: общий tooltip-механизм уже встречался. Учитывать как новое состояние, не как независимое семейство или серию по всем иконкам |
| **CW48** | Classic Search с доступным paused result → правый клик по player overlay → закрыть backdrop без выбора | **Video context menu**: две строки Speed/Repeat и группы option chips, выбранные состояния, позиционирование относительно точки открытия, viewport containment | Открытие и изменение значения разделены: premium-gate стоит на выборе speed/repeat. Ничего не выбирать. Это не modifier row из CW09 |
| **CW49** | Открыть `/terms-of-use.html` → один заранее выбранный viewport/scroll position → закрыть вкладку | **Настоящий long-form document**: ограниченная колонка, H1/H2, paragraphs/lists, line wrapping и scroll. Ранее были Legal menus, не документ | Public, без auth. Не превращать Terms/Privacy/Fair Use с одинаковой вёрсткой в три новых семейства. Не кликать contact links |
| **CW50** | Открыть существующий QA URL `/#/definitely-missing-route` → наблюдать SPA not-found → закрыть вкладку | **Dedicated route-fallback shell**: desktop header и отдельный heading/content region; placement/overlap/scroll. Это не пустая grid и не network-failure injection | Реальная предусмотренная route response, но runtime сейчас не проверен. Server 404 отдельно не предлагаю: в норме отмечен известный unresolved UX-дефект |

## Точные маршруты

**CW43–26**

- Spec: [reels-shell.behavior@3](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/reels-interaction-shell/behavior.md), [инварианты](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/reels-interaction-shell/invariants-and-failure.md).
- QA: [desktop/mobile action rows](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-action-bar-mobile-two-row-speed-repeat.md), [select alignment](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-action-bar-selects-right-aligned-with-spacer.md).
- Source: [action_bar/view.cljs](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/views/action_bar/view.cljs:822).
- Не переносить старый QA `force mobile` override как обязательный setup: сначала подтвердить естественную responsive composition.

**CW45–28**

- Spec: [reels-discovery.behavior](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/reels-discovery/behavior.md), [reels-experience.behavior](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/reels-experience/behavior.md).
- QA: [empty focused Search panel](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-search-panel-empty-focused-input.md), [Common Phrases filters](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-common-phrases-filters-apply.md).
- Source: [search panel](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/search/view.cljs:240), [filter form](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/collections/common_phrases/search_form.cljs:407), [panel mounting](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/view.cljs:1950), [Close](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/views/backside_close/view.cljs).

**CW47**

- Spec: `reels-shell.behavior@3`.
- QA: [desktop hover tooltips](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-action-bar-desktop-hover-tooltips.md).
- Source: [tooltip-wrapped action buttons](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/views/action_bar/view.cljs:463).

**CW48**

- Spec: [shared playback controls](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/playback-runtime/overview-and-behavior.md).
- QA: [Reels action-bar case, right-click menu step](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-action-bar-mobile-two-row-speed-repeat.md).
- Source: [Search mounts menu; gate applies to value changes](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/video/view.cljs:429), [menu rows/backdrop](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/video_menu/view.cljs:267).
- Файл `tc-video-context-menu-global-speed-repeat.md` сейчас проверяет premium gating; его название не означает готовый geometry oracle.

**CW49**

- Spec: [public-policy.overview-behavior](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/public-policy/overview-and-behavior.md), [routes/invariants](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/public-policy/invariants-failure-state-and-qa.md).
- QA: [public policy links](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-public-support-and-rights-links-expose-contact-and-fair-use-policy.md).
- Source: [terms-of-use.html](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/resources/public/terms-of-use.html:46).

**CW50**

- Spec: [route-resolution-and-not-found](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/route-resolution-and-not-found-spec.md).
- QA: [unknown hash route](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-unknown-hash-route-falls-back-to-dedicated-not-found-page.md).
- Source: [not_found/view.cljs](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/not_found/view.cljs).

## Что сознательно не добавлено

- **Download dialog:** существующий QA требует подписанную сессию; не подходит под запрет account/login.
- **Daily Practice:** `daily-practice@10` — иной сложный modal, но вход сразу копирует prompt и читает browser-local study history. Без отдельного разрешённого чистого material-состояния это не безопасный recipe.
- **Reels Collections:** source передаёт `favorites-query` и может раскрывать пользовательские коллекции; исключён при текущем запрете Favorites.
- **Contact/help:** контракт описывает outbound contact, а не отдельную help-форму. Новой панели придумывать нельзя.
- Новые слова, актёры, одинаковые жанровые chips, нижние части уже собранных панелей и очередные viewport не увеличивают число семейств автоматически.

**Практический порядок:** сначала CW45, CW48, CW49, CW50; затем CW43–26 при доступном публичном reel; CW46 — только при фактически доступном guest opening; CW47 — низкоприоритетный дополнительный state.

**Предел:** подтверждено ещё **8 source recipes**, из них один условный и один слабый по новизне. Ни один не captured в CW2. Вместе с прежними 24 это 32 каталожных сценария, а не 32 доказанно доступных состояния и тем более не основание обещать 1000 семантически разных кейсов одного сайта. Для дальнейшего расширения потребуется действительно новое семейство источников или отдельно разрешённый scope — не размножение этих форм.

Все будущие размеры — из runtime в указанном Space/units; ни приведённые CSS, ни старые QA-числа не являются новым measured oracle.


## CW1 — source advisor handoff

Recorded by root from the terminal receipt of Research website UI blueprint, 2026-10-11. Read-only discovery accepted; recipes are not runtime acceptance.

**CW1: `DONE_READ_ONLY`. Найдено 24 разных существующих сценария.** Два семейства имеют сохранённые screenshot+geometry datasets; остальные 22 — проверенные по specs/QA/source рецепты, требующие будущего runtime-сбора. Это не 24 выполненных blueprint-кейса.

Основа сайта: HEAD `c23ef963e5be87195f80cef7173d537a5a146c74`. Маршрут: `AGENTS.md → docs/specs/README.md → discovery-playback / surfaces-experience → выбранные листья ниже`. UI Blueprint `registry37/DESIGN-ANALYSIS@1` использован для понимания результата: точные измерения и гипотезы о закономерностях не становятся требованиями дизайна. Код, UI и файлы не менялись.

## Правила для обеих партий

База — существующий `http://localhost:3000`. `S` ниже означает `/#/search?language=en`, `C` — `/#/clip-search?language=en`.

Каждый будущий кейс сохраняет:

- actual target/frame/document identity, состояние, source/build correspondence либо `unknown`;
- screenshot и DOM/AX одного состояния, интервалы захвата, viewport/scroll/DPR/CSS zoom;
- bounds, родительские связи, label/control/anchor, доступные состояния;
- coverage и omitted; layout bounds отдельно от paint/hit;
- фактический путь входа и выхода.

**Глобально неизвестны без дополнительного evidence:** полная окклюзия, glyph contours, полные hit regions, причина layout из одних rects. CSS-декларации и старые QA-числа — не новые измерения.

## Партия CW-B1 — 12 сценариев Search и shell

Все — **recipe-only**, без готовой согласованной пары screenshot+geometry в этом чате.

| ID | Сценарий, безопасный вход → выход | Состояние и геометрический вопрос | Маршрут |
|---|---|---|---|
| CW01 | S → открыть corpus selector → закрыть без выбора | Trigger, dropdown, строки языка/count: ширина, выравнивание, clipping списка | H1 |
| CW02 | S → Select Actor → закрыть без выбора актёра | Готовый список, поле поиска, backdrop: inset поля, высоты строк, слои. Пустой список не подменять вымышленными актёрами | H2 |
| CW03 | S → правый interface-language selector → закрыть без смены locale | Билингвальные строки, native/English части, заголовок, секции списка: переносы и баланс ширин | H3 |
| CW04 | S → Legal → закрыть, не переходя по ссылкам | Компактное меню документов: icon/label, порядок, общие края и anchor | H4 |
| CW05 | S → Common Phrases tab → вернуть исходную вкладку | Фильтры, строки фраз и pagination: общий inset, выравнивание, границы scroll-area | L1 |
| CW06 | S → Vocabulary tab → вернуть исходную вкладку | Отличающаяся форма сортировки/фильтров и словарные строки: ширины select, интервалы, pagination | L2 |
| CW07 | S с English corpus → Grammar tab → вернуть исходную вкладку | Grammar cards/table: колонки, плотность, перенос текста, границы панели | L3 |
| CW08 | S → публичный draft с реально появившейся completion → восстановить draft | Составной input с невидимым prefix mirror и видимым suffix: baseline, граница typed/suggested text, clipping | O1 |
| CW09 | S с доступным результатом → paused overlay → восстановить исходное playback state | Центр Play и отдельная modifier row: центровка, gaps, наложение на видео. Не включать скрытый Progressive Repeat | O2 |
| CW10 | S → Tour → закрыть Tour | Несколько tooltip cards и SVG connectors: transformed bounds, viewport-fit, пересечения соединителей | T1 |
| CW11 | S в подтверждённой mobile layout → corpus selector → Back без выбора | Навигационный picker вместо desktop dropdown: header/Back, строки/counts, scroll и fixed header | H1 + M1 |
| CW12 | S mobile → overflow menu → Legal → Back | Отдельный navigation layer с документами: общие края Back/title/list, inset строк; не открывать документы | M2 |

CW05–07 доступны только если соответствующая learner-панель действительно показывается. Если текущая настройка скрывает её, **не менять настройки пользователя ради корпуса**: записать prerequisite gap. CW08 зависит от настоящего ответа suggestions, а не от заранее обещанной строки.

## Партия CW-B2 — 12 сценариев Clip Search и форм

| ID | Сценарий, безопасный вход → выход | Состояние и геометрический вопрос | Маршрут / статус |
|---|---|---|---|
| CW13 | C с пустым query → закрыть собственную вкладку | Empty-state, rail и нижний input: центрирование сообщения и распределение свободного пространства | C1; recipe |
| CW14 | C с публичным query, например `how are you` → завершить свою сессию | Обычная desktop grid с реальными карточками: колонки, gaps, media/meta/badges и нижний input | C1; recipe |
| CW15 | CW14 → hover одной карточки → убрать pointer и подтвердить остановку preview | Содержательно иной playing-preview state: вторичная chrome скрыта, download остаётся; сравнение занятых областей | C2; recipe |
| CW16 | CW14 → открыть карточку → pause → Close | Desktop fullscreen viewer: source/title, lower karaoke card, Play overlay, side navigation; alignment и overlap | C3; recipe |
| CW17 | C с результатами в подтверждённой mobile layout → закрыть вкладку | Двухколоночная мобильная grid с компактными badges и single-line source, не уменьшенная desktop grid | C1 + M1; recipe |
| CW18 | CW17 → карточка → pause → Close | Mobile viewer с квадратной stage и нижним plain-text overlay: ratio, gutters, copy/close alignment | C4; recipe |
| CW19 | C → Genre → закрыть без выбора | Popover с сеткой genre chips: переносы, равные семейства gaps, containment и anchor | F1; recipe |
| CW20 | C → Years: reversible draft `199` → восстановить исходный draft | Двухпольная строка с появившимся Clear: равенство полей, gap, alignment label/controls. Applied filters должны остаться прежними | F1; recipe |
| CW21 | C → hover/focus Source info → убрать hover/focus | Tooltip с публичным пояснением и ссылкой: trigger→tooltip gap, переносы текста и link containment. Ссылку не открывать | F1; recipe |
| CW22 | C → Director, draft `spiel`, без выбора → восстановить draft/Escape | Popup/input/list/options: внутренние интервалы, активная подсказка отдельно от applied membership | F1; **captured D1** |
| CW23 | C mobile → Filters → Back, без ввода/выбора | Полноэкранная прокручиваемая форма Source/Years/Genres/entities; clipped Cast и omitted нижние секции | F2; **captured D2** |
| CW24 | S → Settings → Close без изменения controls | Modal form: labels/selects/notes/Close, порядок строк, общие края и локальность пояснений | S1; recipe |

CW15–18 требуют реальных media results и могут запустить звук/telemetry. Будущий packet должен разрешать обычный preview/open/pause и закрепить ресурс. Autoplay refusal не обходить и не подменять playing-state. Для чистой геометрии фиксировать paused state там, где сценарий его допускает.

**Нельзя считать дополнительными кейсами:** десять карточек с одинаковой структурой, десять director names или несколько viewport без реально изменившейся композиции. CW11/CW17/CW18 отличаются от desktop отдельными нормативными layouts, поэтому обоснованы как самостоятельные сценарии.

## Точные spec / QA / source маршруты

QA здесь — навигация и историческое evidence. Его `VERIFIED` не означает новый pass; старые очистки storage, региональные overrides и числовые допуски не переносятся автоматически.

| Ключ | Spec | QA | Основной source |
|---|---|---|---|
| H1 | [header selectors](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/header-corpus/selectors.md) | [corpus popover](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-desktop-language-popover-selection-updates-route.md) | [languages_select](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/languages_select/view.cljs) |
| H2 | тот же `header-corpus.selectors` | [actor popover](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-desktop-actor-popover-backdrop-stays-above-search-surface.md) | [actor_select](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/actor_select/view.cljs) |
| H3 | тот же `header-corpus.selectors` | [locale header geometry](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-desktop-interface-locale-selector-shares-header-gap-distribution.md) | [interface_locale_select](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/interface_locale_select/view.cljs) |
| H4 | [header navigation](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/header-corpus/navigation.md) | [public document links](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-public-support-and-rights-links-expose-contact-and-fair-use-policy.md) | [header](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/header/view.cljs:365) |
| L1 | [learning panels](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/learning-panels/behavior.md) | [panel tabs](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-side-menu-tabs-open-functional-panels.md) | [most_common_phrases](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/most_common_phrases/view.cljs) |
| L2 | [grammar/vocabulary](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/grammar-and-vocabulary-spec.md) | [vocabulary filters](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-vocabulary-filters-pagination-reset.md) | [common_words](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/common_words/view.cljs) |
| L3 | тот же `grammar-vocabulary` + L1 | [panel tabs](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-side-menu-tabs-open-functional-panels.md) | [grammar](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/grammar/view.cljs) |
| O1 | [search overlays](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/search-overlays/user-visible-behavior.md) | [autocomplete](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-desktop-search-autocomplete-normalizes-whitespace.md) | [Search input](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/input/view.cljs) |
| O2 | тот же `search-overlays.behavior` | [paused overlay](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-desktop-search-paused-overlay-repeat-indicator-cycles.md) | [overlay](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/overlay/view.cljs) |
| T1 | [tour behavior](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/guest-tour/behavior.md), [invariants](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/guest-tour/invariants-and-failure.md) | [connectors](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-guest-tour-right-action-connectors-do-not-cross.md) | [tooltip](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/common/tooltip.cljs) |
| M1 | [mobile layout](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/mobile-layout/overview-and-behavior.md) | [mobile header](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-mobile-header-language-actor-select-fixed-equal-height.md) | [mobile header](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/mobile_view/components/header/view.cljs) |
| M2 | [mobile shell](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/mobile-shell/overview-and-behavior.md) | [Legal panel](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-mobile-menu-legal-opens-documents-navigation-panel.md) | [mobile Legal](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/mobile_view/components/legal/view.cljs) |
| C1 | [input/grid shell](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/clip-search/input-and-grid-shell.md), [grid results](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/clip-search/grid-results-and-gate.md) | [entry/grid/viewer](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-desktop-entry-grid-viewer-and-gate.md) | [grid](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/grid/view.cljs), [card](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/grid/card/view.cljs) |
| C2 | [grid preview](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/clip-search/grid-preview-and-entry.md) | [preview chrome](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-card-preview-playback-hides-secondary-chrome.md) | [preview_view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/grid/card/preview_view.cljs) |
| C3 | [viewer core](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/clip-search/viewer-core.md) | C1 QA | [viewer](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/viewer/view.cljs) |
| C4 | [mobile viewer](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/clip-search/mobile-viewer.md) | [mobile viewer QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-mobile-fullscreen-viewer-centered-layout-and-swipe.md) | [mobile_view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/viewer/mobile_view.cljs) |
| F1 | [filter controls](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/phrase-search-filters/surface-controls.md), [pointer/mobile/source](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/phrase-search-filters/mobile-source-and-response.md) | [desktop filters](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-desktop-filters-and-suggestions-panel.md) | [picker](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/filters/picker_view.cljs), [suggestions/filter form](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/suggestions/view.cljs) |
| F2 | F1 mobile/source | [mobile filters QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-mobile-filters-navigation-panel.md) | [mobile_filters](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/mobile_filters/view.cljs) |
| S1 | [settings](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/settings/overview-and-behavior.md) | [settings modal](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-settings-modal-controls-persist.md) | [settings view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/settings/view.cljs) |

У этих site-leaves преимущественно нет отдельных числовых semantic revisions; закреплять их содержимое следует указанным Git HEAD и хешами выбранных файлов.

## Уже доступные реальные данные

**D1 — Director, 9 октября.** Каталог:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-q03-director-SYpdJ9`

[Manifest](/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-q03-director-SYpdJ9/manifest.json), `wide.jpg`, `narrow.jpg`, `wide-dom-detail.json`, `narrow-dom.json`, DOM backend identities, AX, metadata, task и отдельный answer key. Два состояния 1280×900/1024×768; изменение CSS zoom и геометрии действительно измерено. Это одно семейство; root отдельно решает учёт двух layout-состояний, не выдаёт их за разные формы.

**D2 — mobile Filters, 10 октября.** Каталог:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-mobile-filters-OBW93z`

[Manifest](/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-mobile-filters-OBW93z/manifest.json), `mobile-filters.jpg`, `dom-layout.json`, `node-identities.json`, `accessibility.json`, metadata/handoff. Один viewport 390×844, 53 видимых/частично видимых узла; Cast частично виден, нижние секции omitted. Поля пусты, raw values не сохранены.

**Сегодня проверено:** оба каталога существуют, все перечисленные manifest-файлы присутствуют, SHA-256 совпадают. Оба — browser-reference datasets, не самостоятельное доказательство canonical CLI import/export. Готового собственного Settings dataset у этого чата нет.

## Ограничения и передача workers

- Две **независимые партии рецептов по 12**: CW-B1 и CW-B2. Это задания будущим capture workers, а не уже готовые 12 source packages.
- Общий foreground/pointer сериализуется. IAB viewport override также нельзя одновременно менять двум workers; использовать раздельные поддержанные contexts либо очередь.
- Desktop ≥1000 CSS px; mobile shell подтверждать фактически. Responsive browser не обозначать native iOS acceptance. [Browser routing](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/browser-routing.md).
- Без login, QR, покупок, отправки сообщений, download/copy действий и обхода gate. Favorites, Account, Statistics и региональные платёжные формы исключены.
- Только task-owned контекст; не очищать чужие cookies/storage и не менять preferences ради доступности панели. Временные draft/панели/playback восстанавливать.
- Для каждого сборщика pin фактического source/build/target обязателен; текущий checkout не доказывает соответствие загруженного bundle.
- Нулевой результат поиска, loading/error и locked states засчитывать только после настоящего воспроизведения; здесь они не добавлены как готовые дополнительные кейсы.
- Для DESIGN-ANALYSIS сравнимое семейство выбирается явно: например genre chips или строки одного dropdown. Округлённые совпадения, намеренные исключения и unknown не превращать в правило/дефект.

**Счёт CW1:** 24 discovered recipes; 2 captured families с 3 сохранёнными viewport-состояниями; 0 новых captures/generations/evaluations в этой задаче. До 1000 остаётся реальный corpus gap — его нельзя закрыть размножением размеров этих экранов. Следующая зависимость: конечные capture packets CW-B1/CW-B2 с выделенными ресурсами и фактической проверкой доступности.
