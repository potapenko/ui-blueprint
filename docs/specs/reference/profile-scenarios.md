# Контекст сценариев PlayPhrase.me

- Node type: leaf; domain: `uib.profile-scenarios`.
- Contract: `UIB.PROFILE-SCENARIOS@1`; stable clause: `UIB.PROFILE-SCENARIOS.CONTENT`.
- Authority: Active / Stability: Evolving; norms + historical evidence; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: явно разрешённое подключение product profile; не базовые fixtures.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.FORMS@1](../product/forms.md), [UIB.GEOMETRY@1](../product/geometry.md).
- Source mapping: TZ 319–371; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

Это source context, не разрешение читать/менять другие проекты. Для живой работы нужен текущий маршрут самого продукта и отдельный разрешённый scope.

<a id="uib-profile-scenarios-content"></a>

## Сценарии веба

Эта матрица взята из исходного веб-ТЗ и его выбранных контрактов. Она определяет полезные задачи инструмента; существующие VERIFIED-метки QA не являются новым запуском.

| ID | Сценарий | Требуемое наблюдение и проверка |
| --- | --- | --- |
| W01 | Частичный год или IMDb | Draft отдельно от применённого filters; debounce и изменение URL проверяются отдельно |
| W02 | Автокомплит режиссёра | Фокус поля, active_descendant, выбор клавиатурой, закрытие попапа и applied value |
| W03 | Точный фильм | Сохранённые значения общих фильтров при disabled; после очистки восстановление редактирования |
| W04 | Удаление тега | Desktop remove без открытия popover; на mobile вся Cast-плашка — цель удаления |
| W05 | Попап над сеткой | Anchor, геометрия и hit testing вне попапа; отсутствие невидимого блокирующего слоя |
| W06 | Локализация | Отображаемые подписи меняются, канонические значения и язык корпуса не подменяются |
| W07 | Сохранение настройки | Pause Between Repeats после закрытия/открытия; подпись 1 sec отдельно от value 1000 |
| W08 | Большой текст | Доступность контролов, отсутствие overflow, реальный режим text size |
| W09 | Мобильная навигация | Filters открывается из Clip Search в секции с Back, отдельно от desktop rail |
| W10 | Выбор фразы после ввода | Смена draft/query, фокуса, экранной клавиатуры и состояния видео раздельными наблюдениями |
| W11 | RTL-текст | Направление текста отдельно от LTR-компоновки; отсутствие автоматического зеркалирования |
| W12 | До и после правки | Геометрические изменения при сохранённых ролях, значениях и действиях |

W01–W06 и W09 опираются на phrase-search-filters.surface-controls и phrase-search-filters.mobile-source-response, а также три QA-кейса desktop/mobile/locale filters. W07 — settings.overview-behavior и tc-settings-modal-controls-persist. W08/W10 — mobile-layout и связанные QA. W11 — interface-localization.translation-direction. W12 — общий сценарий инструмента, предложенный по этим задачам. Точные ссылки сохранены в приложении источников.

Два известных ограничения источников сохраняются: требование ellipsis для длинных тегов режиссёра расходится с Q1 desktop-кейсом; CSS zoom в старом кейсе мобильных настроек не заменяет native content size Simulator. Они не «исправляются» этим ТЗ и не импортируются как согласованные ожидания.

## Сценарии Mac и мобильных Apple приложений

Mac-соавтор повторно проверил выбранные leaf-контракты и актуальную QA-политику. Ниже спецификационные сценарии, а не отчёт о текущих кадрах.

| ID | Сценарий | Требуемое наблюдение и проверка |
| --- | --- | --- |
| N01 | Точный draft и продолжение ввода | Ввод/вставка/selection/IME; Tab completion; macOS 15+ caret-at-end отдельно от macOS 14 |
| N02 | Нативный filter popover Mac | Немедленный выбор, multi/single-select, локальный Clear, Escape/outside dismiss, допустимый fallback размещения |
| N03 | Фильтр iPad | Autofocus поиска, пояснение не input, Done только закрывает, адаптация popover к sheet, landscape Pro 13/11 |
| N04 | Короткая надпись и полный смысл | Visible label максимум два приоритетных значения и +N; полный accessibility-name; anchor всей кнопки; minimum hit target 44 pt |
| N05 | Resize Mac Search | Нативная перераскладка, перенос/сжатие/truncation/scrolling без требования масштабировать всё пропорционально |
| N06 | iPhone и accessibility text size | Условные правила колонки, действий, targets и scroll; Dynamic Type меняет композицию; формат видео в этой поверхности 4:3 |
| N07 | Происхождение QA-доказательств | Точный build/Target/backend/input; автоматизация, реальное взаимодействие и визуальная проверка отдельно; Simulator не физическое устройство |

N01: query-binding@2, destination-autofocus@1. N02: macOS filter popover@1, source r2. N03: iPad filter popover@2. N04: learner filters@3. N05: point-sizing@1. N06: aligned-column@5. N07: runtime acceptance и native-ui-tools@1. Точные ссылки — в приложении.

Числа и поведение этих сценариев принадлежат конкретным продуктовым контрактам. Они не становятся универсальными правилами UI Blueprint: например, 44 pt не вводится как единый глобальный порог для всех платформ, а 4:3 — как формат любого видео.

Кандидат T01: направленный TV-фокус, remote-команды и возврат на исходный элемент. Перед включением в приёмку потребуется отдельный полный маршрут TV-контрактов.

## Применение к PlayPhrase.me и актуальная QA политика

Web: текущая маршрутизация выбирает Codex in-app Browser, если он поддерживает сценарий. Generic Chromium может показать unsupported-browser surface. Сборщик подключается к разрешённой среде и её возможностям. Серьёзная мобильная веб-приёмка использует обычный iOS Simulator Safari с Web Inspector, native content size large/accessibility-large и реальной экранной клавиатурой для сценариев ввода; desktop mobile viewport — предварительный сигнал.

Native: native-ui-tools@1 от 6 октября 2026 года разрешает точечное AX/view inspection, bounded waits и обоснованную focused UI automation/snapshot-проверку. Старый blanket ban в Mac Draft исторический и не действует как правило нового документа. Канонические Computer Use action-state-visible-result и визуальная приёмка сохраняются.

Для native-результата отдельно отражаются automated_checks, real_app_interaction и visual_verification. Положительный результат первого не закрывает два остальных. Сохраняются build identity, environment, input modality, исходное состояние, действие и видимый результат.

Это локальные политики PlayPhrase.me; они не распространяются автоматически на другие проекты. Создание ТЗ не разрешает добавлять SDK, новые test targets, обходить ограничения инструментов или запускать приложение. Первые пилоты можно выполнять в отдельных контролируемых fixtures; применение к реальному продукту следует его действующему маршруту.
