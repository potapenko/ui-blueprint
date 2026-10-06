# Граф и происхождение свойств

- Node type: leaf; domain: `uib.model`.
- Contract: `UIB.MODEL@1`; stable clause: `UIB.MODEL.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: schema, нормализация, свойства, роли, provenance.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.BOUNDARIES@1](boundaries.md).
- Source mapping: TZ 106–142; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-model-content"></a>

## Общая модель и происхождение данных

Внешний формат использует snake_case. Нативные имена сохраняются в namespaced extensions; новая схема не переименовывает существующие программные идентификаторы приложения.

| Сущность | Смысл |
| --- | --- |
| Target | Точное приложение/процесс, вкладка или устройство и его поколение |
| Surface | Окно, документ, frame, popup, sheet или встроенная поверхность |
| Observation | Интервал сбора, источники, среда, согласованность и покрытие |
| Snapshot | Материализованный граф наблюдённых данных с revision |
| Element | Узел конкретного источника с семантикой, состоянием и геометрией |
| Component при наличии | Логическая группа, подтверждённая явным mapping приложения |
| Region | Выбранная область или помеченная вычисленная группа |
| Relation | Семантическая, визуальная, компонентная, фокусная либо геометрическая связь |
| Action | Намерение, цель, способ ввода, предусловия и разрешённый scope |
| Transition | Действия и наблюдаемые результаты с привязкой к снимкам |
| Expectation | Требование со своим источником, условиями и допуском |
| Finding | Результат проверки или вывод с доказательствами и ограничениями |

Observation описывает получение данных, Snapshot — сохранённое состояние графа. Точность API не предполагается абсолютной. Для каждого свойства отдельно описываются:

- provenance: reported — сообщил API; derived — вычислено; estimated — эвристика или модель;

- availability: known, unknown, unsupported, redacted; not_requested описывает отдельную ось выбора полей (projection/fields), а не доступность запрошенного свойства;

- источник, время и Observation;

- точность/погрешность, когда она имеет измеримый смысл.

Декларативные сведения из кода/SDK хранятся отдельно в source_declarations: например заданный modifier, component ID или явный mapping. Это metadata с источником, а не измеренная геометрия и не автоматическое требование к продукту. Declared Expectation дополнительно содержит источник нормативного решения. Confidence не придумывается для каждого узла: она допустима при определённом методе оценки.

Источник ответа live/cache, свежесть current/stale/unverified с временем последней проверки, согласованность stable/unstable/unknown и покрытие complete/partial/unknown — независимые оси. Полнота относится к явно заданным scope и fields, а не ко всему приложению. Кэшированный ответ может быть актуальным и частичным. Даже новый live-сбор не гарантирует пригодность цели к более позднему действию.

Словарь ролей сохраняет необходимые различия: text, heading, textbox, searchbox, button, link, checkbox, radio, switch, combobox, option, list, listitem, menu, menuitem, tab, dialog, form, group, scrollarea, slider, image, media, custom, unknown. Исходная native_role обязательна при её наличии. Точные mapping-правила версионируются.

Семантический родитель, визуальный контейнер, владелец компонента, якорь попапа и поверхность не сводятся к одному дереву. Общий граф хранится локально; формат ответа не обязан включать его целиком.
