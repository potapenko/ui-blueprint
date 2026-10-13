# Геометрия и проверочные отношения

- Node type: leaf; domain: `uib.geometry`.
- Contract: `UIB.GEOMETRY@2`; stable clause: `UIB.GEOMETRY.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing; USE-WEB-RECT2 explicitly authorizes the Web source-agreement clause below.
- Read when: пространства, transforms, измерения, Expectation/check.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.MODEL@1](model.md).
- Source mapping: TZ 143–164; TZ 299–318; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; Web source-agreement delta ниже ограничена одобренным USE-WEB-RECT2.

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

## UIB.GEOMETRY.WEB-SOURCE-AGREEMENT — USE-WEB-RECT2

Authority: root’s explicit USE-WEB-RECT2 acceptance under the user’s useful-geometry/testing request, MODEL per-property availability and no frame substitution. Actual private API evidence has equal CSSOM endpoints but different IO bounds; no Chromium-internal cause is inferred.
A is initial getBoundingClientRect, B is IntersectionObserver boundingClientRect, C is getBoundingClientRect in its callback; compare all four original x/y/width/height values exactly.
Retain every finite/size/ratio validation. A≠C refuses the whole observation with resync/code1 regardless of B. If A=C≠B, continue SAME identity, viewport and cleanup gates; do not reacquire, retry, change fields/caps/deadlines or numerically repair a rectangle.
On A=C≠B, layout_bounds preserves exact A and its existing Space/units/evidence/qualified transform. Hit sample keeps its independent source attribution. Requested visible_region remains unknown with reason intersection-bounds-disagree-with-layout.
The existing web.dom extensions intersection_rect_not_occlusion, intersection_ratio_not_visibility and is_intersecting_not_visible remain present but unknown with the same reason and original IO method/Observation evidence. Do not expose their unqualified geometry/ratio/boolean as known or leak raw B through new output.
A=B=C preserves prior canonical data and attribution. Missing IO data retains its prior behavior. No requested field/node is dropped; partial coverage and unknown/sequential-reads-not-atomic consistency remain. Equal endpoints are not proof of atomicity or a stable interval.
No schema, DTO, reader, Host, action authority, engine arithmetic or export contract change. This applies to Web source normalization, not a tolerance rule for other sources.
Acceptance: exact observed fractional sample yields useful partial canonical layout plus unknown IO; A≠C for any B, malformed IO, viewport drift, stale identity and cleanup failures still publish nothing. Matching control output stays byte-identical for fixed source/context/Observation inputs; same real source owner independently validates/measure/exports one qualified new observation.
