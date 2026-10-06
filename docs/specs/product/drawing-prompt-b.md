# Полный prompt: геометрия и результат

- Node type: leaf; domain: `uib.drawing-prompt-b`.
- Contract: `UIB.DRAWING-PROMPT-B@1`; stable clause: `UIB.DRAWING-PROMPT-B.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: сборка self-contained prompt; вторая последовательная часть.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.DRAWING-PACKAGE@1](drawing-package.md), [UIB.DRAWING-STYLE@1](drawing-style.md).
- Source mapping: DRAWING 206–251; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-drawing-prompt-b-content"></a>

```text
ГЕОМЕТРИЯ
Пространство: {{coordinate_space}}. Единицы UI: {{units}}.
Общие размеры выбранной поверхности: {{surface_dimensions}}.
Главные оси и выравнивания: {{alignment_rules}}.
Внешние отступы, ширины колонок и промежутки: {{primary_dimensions}}.
Детальные размеры, радиусы и baseline только там, где они заданы:
{{detail_dimensions}}
Отличай layout bounds, accessibility bounds, hit region и clip/safe region.
Показывай дополнительные рамки только по плану слоя/детали, не накладывай всё
в один нечитаемый контур. Не превращай расстояние до текста в padding.

НАНЕСЕНИЕ РАЗМЕРОВ
Проведи размерные линии между указанными anchors с выносными линиями.
Подписывай точные значения из ведомости. Не округляй и не пересчитывай их.
Не придумывай допуски, неизвестные радиусы и размеры декоративных деталей.
Не перекрывай число линией или стрелкой. Повторяющиеся размеры допускается
свести к типовой детали/таблице согласно плану листа.
Не измеряй ничего по pixels прикреплённого изображения.
Неизвестные значения: {{unknown_properties}} — пометь ? и примечанием.
Режим масштаба: {{scale_mode}}.
Если schematic, напиши «Размеры по подписям; не измерять по изображению».

СОСТОЯНИЯ И ДЕЙСТВИЯ
Этот главный вид фиксирует только {{state_name}}.
Список состояний контролов и краткая семантика:
{{state_and_action_table}}
Другие состояния показывай только в перечисленных отдельных видах.
Не изображай потенциальный переход как проверенный.
Не добавляй стрелки flow в геометрический вид без задания.

ОСНОВНАЯ НАДПИСЬ И ВЕДОМОСТЬ
Внизу справа помести title block:
{{title_block_fields}}
Рядом компактная легенда объектов/линий и примечаний:
{{notes}}
Не печатай секреты, machine paths или внутренние идентификаторы вместо
подготовленных безопасных ссылок.

РЕЗУЛЬТАТ
Выдай аккуратный инженерный лист, пригодный для обсуждения UI с клиентом.
Все размеры, ID и надписи должны соответствовать заданию.
Не выдавай изображение за проверенный CAD-файл или утверждённую спецификацию.
Не меняй статусы source/validation/approval.
```
