# Геометрия и проверочные отношения

- Node type: leaf; domain: `uib.geometry`.
- Contract: `UIB.GEOMETRY@1`; stable clause: `UIB.GEOMETRY.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: пространства, transforms, измерения, Expectation/check.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.MODEL@1](model.md).
- Source mapping: TZ 143–164; TZ 299–318; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-geometry-content"></a>

## Геометрия и координаты

| Поле | Значение |
| --- | --- |
| layout_bounds | Рамка системы раскладки |
| accessibility_bounds | Рамка, сообщённая accessibility API |
| hit_region | Подтверждённая или частично проверенная область взаимодействия |
| visible_region | Видимая область с учётом известной обрезки и перекрытия |
| paint_bounds при наличии | Отдельные сведения о границе рисования; не замена visible_region |

Неизвестные поля не заполняются другой рамкой без маркировки происхождения. Layout-рамка текста не равна контуру букв. Пересечение bounding boxes не доказывает визуальное перекрытие; z-index без контекста наложения недостаточен. Тени, прозрачность, blur, Canvas и системные эффекты могут остаться неизвестными.

Каждая геометрия указывает coordinate_space, units, origin и transform. Возможны screen, window, surface, viewport, document и local spaces; исходные px, css_px, pt и dp сохраняются. Поддерживаются rect, а при необходимости quad/polygon/fragments. Преобразования между окнами, frame, scroll-контейнерами и изображением имеют явный источник. Если нужный transform неизвестен, измерение возвращает missing_transform.

Window-local сравнение отделяет перенос окна от изменения внутреннего layout. Размер текста, display scale, zoom, ориентация, safe area, keyboard/visual viewport и scroll входят в контекст применимых измерений. Физические left/right и логические leading/trailing различаются; направление текста не означает направление всей композиции.

Базовые расчёты: размер, пропорция, расстояние между краями/центрами, доступная baseline, выравнивание, вложение, пересечение, overflow, равенство промежутков с допуском. Измеренный inset не называется padding без отдельного источника.

Viewport и visual viewport относятся к web extension. Для нативного UI используются display/window/content/safe-content пространства; конфигурация дисплеев имеет собственную ревизию. Смена этой конфигурации инвалидирует зависимые преобразования.

Стабилизация ограничена выбранной областью. Видео не должно задерживать измерение неподвижной формы до полной остановки всех пикселей. Время начала/окончания сбора и рассогласование дерева с crop сохраняются. Транзиентное состояние не принимается за финальную геометрию.

## Геометрические правила и прикладные ожидания

Rust-ядро реализует отношения gap, aligned, inside, intersects, width/height, ratio, equal_spacing и доступные baseline-проверки. Язык правил представляет эти операции, а не произвольный исполняемый код из страницы.

Условный пример:

```text
gap(icon.layout.right, label.layout.left) = 8 ± 1 css_px
aligned(menu_rows.layout.leading) tolerance 1 pt
inside(popup.layout, viewport) margin >= 12 css_px
```

Expectation содержит id, scope, targets, relation, параметры, units, tolerance, applies_when и expected_from. Последний ссылается на пользовательское решение, продуктовый контракт или проверенную fixture. Условие платформы, размера текста, input-mode и доступности данных входит в правило.

Check возвращает pass/fail/unknown с измерением, допуском, Observation и причиной. Отсутствие измерения не считается pass. Проверка существования элемента отличается от проверки его состояния. Локальный source profile не имеет права превратить старый QA-лог в новый контракт.

Profile PlayPhrase.me декодирует прикладной смысл вроде filters.year из URL и связывает его с черновиком поля. Такой смысл не вшивается в engine. Несогласованные спецификация и QA помечаются как conflict; спорная проверка не переопределяет продукт молча.

Геометрия поддерживает диагностику и приёмку, но не заменяет весь сценарий. Сохранение настройки требует повторного открытия; воспроизведение — наблюдаемого изменения media; успешная внешняя операция — отдельного результата, не одного снимка.
