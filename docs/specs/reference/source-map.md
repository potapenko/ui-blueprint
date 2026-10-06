# Обратная карта исходников

- Node type: branch; contract: `UIB.SOURCE-MAP@1`; clause: `UIB.SOURCE-MAP.ROUTE`.
- Authority: Active / Stability: Evolving; routing only; accepted/released baseline: none.
- Authority source: [registry](../README.md), C00 faithful routing of user-confirmed originals.
- Read when: проверка полноты/разницы routed норм.
- Do not read when: обычный пакет уже выбрал closure.
- Requires: только выбранные ниже листья и их explicit closure.

TZ = `UIB.TZ@1.4`; DRAWING = `UIB.DRAWING@1.1`. Диапазоны относятся к неизменным
исходникам C00 (approved source commit `358c757`), не заменяют stable clause IDs.
Каждый лист хранит весь указанный текст, включая нормы, примеры и оговорки.
Исходные заголовки/таблицы сохранены; только шаблон prompt разделён на A/B.
Historical/future материал не становится текущим обязательством от переноса.

| Источник / раздел / строки | Routed clause (`CONTENT`, @1) |
| --- | --- |
| [TZ: UI Blueprint общее техническое задание](../../ui-blueprint-spec.md#ui-blueprint-общее-техническое-задание), 1–12 | [UIB.PROVENANCE@1](provenance.md#uib-provenance-content) |
| [TZ: Назначение и обязательные решения](../../ui-blueprint-spec.md#назначение-и-обязательные-решения), 13–75 | [UIB.BOUNDARIES@1](../product/boundaries.md#uib-boundaries-content) |
| [TZ: Нативное наблюдение и изображения](../../ui-blueprint-spec.md#нативное-наблюдение-и-изображения), 76–105 | [UIB.NATIVE@1](../product/native.md#uib-native-content) |
| [TZ: Общая модель и происхождение данных](../../ui-blueprint-spec.md#общая-модель-и-происхождение-данных), 106–142 | [UIB.MODEL@1](../product/model.md#uib-model-content) |
| [TZ: Геометрия и координаты](../../ui-blueprint-spec.md#геометрия-и-координаты), 143–164 | [UIB.GEOMETRY@1](../product/geometry.md#uib-geometry-content) |
| [TZ: Выбор области и проекции](../../ui-blueprint-spec.md#выбор-области-и-проекции), 165–184 | [UIB.PROJECTIONS@1](../product/projections.md#uib-projections-content) |
| [TZ: Семантика форм и ввода](../../ui-blueprint-spec.md#семантика-форм-и-ввода), 185–200 | [UIB.FORMS@1](../product/forms.md#uib-forms-content) |
| [TZ: Идентичность и актуальность цели](../../ui-blueprint-spec.md#идентичность-и-актуальность-цели), 201–212 | [UIB.IDENTITY@1](../product/identity.md#uib-identity-content) |
| [TZ: Исполнитель действий и протокол результата](../../ui-blueprint-spec.md#исполнитель-действий-и-протокол-результата), 213–230 | [UIB.ACTIONS@1](../product/actions.md#uib-actions-content) |
| [TZ: Кэш и инкрементальные обновления](../../ui-blueprint-spec.md#кэш-и-инкрементальные-обновления), 231–298 | [UIB.CACHE@1](../product/cache.md#uib-cache-content) |
| [TZ: Геометрические правила и прикладные ожидания](../../ui-blueprint-spec.md#геометрические-правила-и-прикладные-ожидания), 299–318 | [UIB.GEOMETRY@1](../product/geometry.md#uib-geometry-content) |
| [TZ: Сценарии веба](../../ui-blueprint-spec.md#сценарии-веба), 319–371 | [UIB.PROFILE-SCENARIOS@1](profile-scenarios.md#uib-profile-scenarios-content) |
| [TZ: Консольный интерфейс](../../ui-blueprint-spec.md#консольный-интерфейс), 372–398 | [UIB.CLI@1](../product/cli.md#uib-cli-content) |
| [TZ: Инженерная документация и ImageGen](../../ui-blueprint-spec.md#инженерная-документация-и-imagegen), 399–416 | [UIB.EXPORT@1](../product/export.md#uib-export-content) |
| [TZ: Инженерная документация и ImageGen](../../ui-blueprint-spec.md#инженерная-документация-и-imagegen), 417–422 | [UIB.EXTENSIONS@1](extensions.md#uib-extensions-content) |
| [TZ: Собственная реализация и open source](../../ui-blueprint-spec.md#собственная-реализация-и-open-source), 423–442 | [UIB.REUSE@1](reuse.md#uib-reuse-content) |
| [TZ: Приватность и жизненный цикл данных](../../ui-blueprint-spec.md#приватность-и-жизненный-цикл-данных), 443–454 | [UIB.PRIVACY@1](../product/privacy.md#uib-privacy-content) |
| [TZ: Равноправные пилоты Mac и Web](../../ui-blueprint-spec.md#равноправные-пилоты-mac-и-web), 455–460 | [UIB.PILOTS@1](../acceptance/pilots.md#uib-pilots-content) |
| [TZ: Равноправные пилоты Mac и Web](../../ui-blueprint-spec.md#равноправные-пилоты-mac-и-web), 461–473 | [UIB.NATIVE-PILOTS@1](../acceptance/native-pilots.md#uib-native-pilots-content) |
| [TZ: Равноправные пилоты Mac и Web](../../ui-blueprint-spec.md#равноправные-пилоты-mac-и-web), 474–482 | [UIB.WEB-PILOTS@1](../acceptance/web-pilots.md#uib-web-pilots-content) |
| [TZ: Равноправные пилоты Mac и Web](../../ui-blueprint-spec.md#равноправные-пилоты-mac-и-web), 483–484 | [UIB.PILOTS@1](../acceptance/pilots.md#uib-pilots-content) |
| [TZ: Этапы реализации и границы поставки](../../ui-blueprint-spec.md#этапы-реализации-и-границы-поставки), 485–498 | [UIB.ROADMAP@1](../product/roadmap.md#uib-roadmap-content) |
| [TZ: Критерии приёмки](../../ui-blueprint-spec.md#критерии-приёмки), 499–519 | [UIB.COMPLETION@1](../acceptance/completion.md#uib-completion-content) |
| [TZ: Производительность и проверка пользы](../../ui-blueprint-spec.md#производительность-и-проверка-пользы), 520–539 | [UIB.PERFORMANCE@1](../acceptance/performance.md#uib-performance-content) |
| [TZ: Задание агенту разработчику и обязательные результаты](../../ui-blueprint-spec.md#задание-агенту-разработчику-и-обязательные-результаты), 540–576 | [UIB.ROADMAP@1](../product/roadmap.md#uib-roadmap-content) |
| [TZ: Минимальный контракт обмена для реализации](../../ui-blueprint-spec.md#минимальный-контракт-обмена-для-реализации), 577–596 | [UIB.EXCHANGE@1](../product/exchange.md#uib-exchange-content) |
| [TZ: Сквозные сценарии и доказательства готовности](../../ui-blueprint-spec.md#сквозные-сценарии-и-доказательства-готовности), 597–610 | [UIB.COMPLETION@1](../acceptance/completion.md#uib-completion-content) |
| [TZ: Отмена и конкуренция](../../ui-blueprint-spec.md#отмена-и-конкуренция), 613–622 | [UIB.LIFECYCLE@1](../product/lifecycle.md#uib-lifecycle-content) |
| [TZ: Очистка данных до сохранения](../../ui-blueprint-spec.md#очистка-данных-до-сохранения), 623–632 | [UIB.PRIVACY@1](../product/privacy.md#uib-privacy-content) |
| [TZ: GOLDEN01 Один полный путь](../../ui-blueprint-spec.md#golden01-один-полный-путь), 633–652 | [UIB.GOLDEN@1](../acceptance/golden.md#uib-golden-content) |
| [TZ: Отчёт чтения исходников](../../ui-blueprint-spec.md#отчёт-чтения-исходников), 653–660 | [UIB.REUSE@1](reuse.md#uib-reuse-content) |
| [TZ: Стартовый промпт для будущего разработчика](../../ui-blueprint-spec.md#стартовый-промпт-для-будущего-разработчика), 661–686 | [UIB.START-NAME@1](start-and-name.md#uib-start-name-content) |
| [TZ: Режимы запуска и дальнейшее развитие](../../ui-blueprint-spec.md#режимы-запуска-и-дальнейшее-развитие), 687–736 | [UIB.FUTURE-HOST@1](future-host.md#uib-future-host-content) |
| [TZ: Dashboard для наблюдения работы](../../ui-blueprint-spec.md#dashboard-для-наблюдения-работы), 737–783 | [UIB.FUTURE-DASHBOARD@1](future-dashboard.md#uib-future-dashboard-content) |
| [TZ: Дополнительные указания для Rust реализации](../../ui-blueprint-spec.md#дополнительные-указания-для-rust-реализации), 784–811 | [UIB.RUST-BOUNDARIES@1](../product/rust-boundaries.md#uib-rust-boundaries-content) |
| [TZ: Источники и принятые границы](../../ui-blueprint-spec.md#источники-и-принятые-границы), 812–867 | [UIB.PROVENANCE@1](provenance.md#uib-provenance-content) |
| [TZ: Каталог веб и общих исходников для разработчика](../../ui-blueprint-spec.md#каталог-веб-и-общих-исходников-для-разработчика), 868–873 | [UIB.REUSE@1](reuse.md#uib-reuse-content) |
| [TZ: agent browser](../../ui-blueprint-spec.md#agent-browser), 874–921 | [UIB.WEB-SOURCES@1](web-catalog.md#uib-web-sources-content) |
| [TZ: Galen и Galen Extras](../../ui-blueprint-spec.md#galen-и-galen-extras), 922–939 | [UIB.CORE-SOURCES@1](core-catalog.md#uib-core-sources-content) |
| [TZ: Blueprint для Compose](../../ui-blueprint-spec.md#blueprint-для-compose), 940–955 | [UIB.PROBE-SOURCES@1](probe-catalog.md#uib-probe-sources-content) |
| [TZ: Jev и официальные SDK](../../ui-blueprint-spec.md#jev-и-официальные-sdk), 956–963 | [UIB.EXTENSIONS@1](extensions.md#uib-extensions-content) |
| [TZ: Браузерные и Android платформенные источники](../../ui-blueprint-spec.md#браузерные-и-android-платформенные-источники), 964–967 | [UIB.WEB-SOURCES@1](web-catalog.md#uib-web-sources-content) |
| [TZ: Браузерные и Android платформенные источники](../../ui-blueprint-spec.md#браузерные-и-android-платформенные-источники), 968–971 | [UIB.EXTENSIONS@1](extensions.md#uib-extensions-content) |
| [TZ: Каталог нативных и исследовательских источников](../../ui-blueprint-spec.md#каталог-нативных-и-исследовательских-источников), 972–975 | [UIB.REUSE@1](reuse.md#uib-reuse-content) |
| [TZ: Нативный сбор и действия](../../ui-blueprint-spec.md#нативный-сбор-и-действия), 976–981 | [UIB.NATIVE-SOURCES@1](native-catalog.md#uib-native-sources-content) |
| [TZ: Нативный сбор и действия](../../ui-blueprint-spec.md#нативный-сбор-и-действия), 978–979 | [UIB.OTHER-PLATFORMS@1](other-platforms.md#uib-other-platforms-content) |
| [TZ: Нативный сбор и действия](../../ui-blueprint-spec.md#нативный-сбор-и-действия), 982–984 | [UIB.OTHER-PLATFORMS@1](other-platforms.md#uib-other-platforms-content) |
| [TZ: Пиксельные модели и исследовательские прототипы](../../ui-blueprint-spec.md#пиксельные-модели-и-исследовательские-прототипы), 985–996 | [UIB.EXTENSIONS@1](extensions.md#uib-extensions-content) |
| [TZ: Платформенные первоисточники](../../ui-blueprint-spec.md#платформенные-первоисточники), 997–1004 | [UIB.NATIVE-SOURCES@1](native-catalog.md#uib-native-sources-content) |
| [TZ: Дополнительные прототипы Ui Vision и UI Atlas](../../ui-blueprint-spec.md#дополнительные-прототипы-ui-vision-и-ui-atlas), 1005–1008 | [UIB.REUSE@1](reuse.md#uib-reuse-content) |
| [TZ: Ui Vision как референс исполнителя сценариев](../../ui-blueprint-spec.md#ui-vision-как-референс-исполнителя-сценариев), 1009–1030 | [UIB.EXECUTOR-SOURCES@1](executor-catalog.md#uib-executor-sources-content) |
| [TZ: UI Atlas как референс карты приложения](../../ui-blueprint-spec.md#ui-atlas-как-референс-карты-приложения), 1031–1073 | [UIB.FUTURE-ATLAS@1](future-atlas.md#uib-future-atlas-content) |
| [TZ: Рабочее имя и GitHub](../../ui-blueprint-spec.md#рабочее-имя-и-github), 1074–1080 | [UIB.START-NAME@1](start-and-name.md#uib-start-name-content) |
| [DRAWING: UI Blueprint руководство инженерной визуализации и ImageGen](../../engineering-blueprint-guide.md#ui-blueprint-руководство-инженерной-визуализации-и-imagegen), 1–30 | [UIB.EXPORT@1](../product/export.md#uib-export-content) |
| [DRAWING: Источники чертёжных принципов](../../engineering-blueprint-guide.md#источники-чертёжных-принципов), 31–90 | [UIB.DRAWING-STYLE@1](../product/drawing-style.md#uib-drawing-style-content) |
| [DRAWING: Размеры и геометрические отношения](../../engineering-blueprint-guide.md#размеры-и-геометрические-отношения), 91–130 | [UIB.DRAWING-GEOMETRY@1](../product/drawing-geometry.md#uib-drawing-geometry-content) |
| [DRAWING: Документ DrawingBrief и пакет генерации](../../engineering-blueprint-guide.md#документ-drawingbrief-и-пакет-генерации), 131–157 | [UIB.DRAWING-PACKAGE@1](../product/drawing-package.md#uib-drawing-package-content) |
| [DRAWING: Полный шаблон промпта для одного интерфейса](../../engineering-blueprint-guide.md#полный-шаблон-промпта-для-одного-интерфейса), 158–205 | [UIB.DRAWING-PROMPT-A@1](../product/drawing-prompt-a.md#uib-drawing-prompt-a-content) |
| [DRAWING: Полный шаблон промпта для одного интерфейса](../../engineering-blueprint-guide.md#полный-шаблон-промпта-для-одного-интерфейса), 206–251 | [UIB.DRAWING-PROMPT-B@1](../product/drawing-prompt-b.md#uib-drawing-prompt-b-content) |
| [DRAWING: Пример исходного задания без сравнения](../../engineering-blueprint-guide.md#пример-исходного-задания-без-сравнения), 252–276 | [UIB.DRAWING-EXAMPLE@1](drawing-example.md#uib-drawing-example-content) |
| [DRAWING: Проверка генерации и применение в QA](../../engineering-blueprint-guide.md#проверка-генерации-и-применение-в-qa), 277–300 | [UIB.DRAWING-REVIEW@1](../product/drawing-review.md#uib-drawing-review-content) |
| [DRAWING: Команды и критерии готовности](../../engineering-blueprint-guide.md#команды-и-критерии-готовности), 301–315 | [UIB.EXPORT@1](../product/export.md#uib-export-content) |

Единственный неперенесённый непустой source line — TZ 611, родительский
заголовок «Операционные условия и контрольный сценарий»: его полное содержание
распределено в LIFECYCLE, PRIVACY, GOLDEN и REUSE. Пустые строки не несут норм.
