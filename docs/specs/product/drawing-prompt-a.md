# Полный prompt: основание и лист

- Node type: leaf; domain: `uib.drawing-prompt-a`.
- Contract: `UIB.DRAWING-PROMPT-A@1`; stable clause: `UIB.DRAWING-PROMPT-A.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: сборка self-contained prompt; первая последовательная часть.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.DRAWING-PACKAGE@1](drawing-package.md), [UIB.DRAWING-STYLE@1](drawing-style.md), [UIB.DRAWING-PROMPT-B@1](drawing-prompt-b.md).
- Source mapping: DRAWING 158–205; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

Шаблон разделён только по файлам: текстовые блоки A и B последовательно составляют один prompt. Обязательные placeholders заменяются данными или явным unknown.

<a id="uib-drawing-prompt-a-content"></a>

## Полный шаблон промпта для одного интерфейса

Шаблон собирается из проверенного DrawingBrief. Переменные должны быть заменены данными либо явным unknown; незаполненный обязательный placeholder делает пакет неготовым. Короткая поясняющая фраза агента не заменяет этот текст.

```text
ЗАДАЧА
Создай инженерный blueprint одного интерфейса для документации и обсуждения
с клиентом. Это основной полный вид выбранного scope, не сравнение «до/после».
Режим: {{document_or_propose}}.
Документ: {{document_id}}, ревизия {{revision}}, лист {{sheet_id}} из {{sheet_count}}.
Название: {{title}}. Аудитория: {{audience}}. Язык надписей: {{language}}.

ОСНОВАНИЕ
source_kind={{source_kind}}; validation={{validation_status}};
approval={{approval_status}}.
Источник: {{safe_source_reference}}.
Платформа и состояние: {{platform_environment_and_state}}.
Покрытие: {{coverage_statement}}.
Для observed используй только перечисленные наблюдённые сведения.
Для proposed обозначь все целевые параметры как проект и не называй их замерами.
Прикреплённые изображения — референс внешнего вида; численные подписи бери
из приведённой ведомости, а не оценивай самостоятельно по изображению.

ПРОЕКЦИЯ И ЛИСТ
Плоский ортогональный вид спереди. Без перспективы и 3D.
Формат {{page_format}}, ориентация {{orientation}}, разрешение {{output_size}}.
Покажи весь объявленный scope в центральном главном виде G01.
Оставь поля для размерных цепочек. Не обрезай части интерфейса.
Размести {{detail_views}} как увеличенные детали со ссылками на G01.
Ни один компонент не удаляй ради композиции. Если лист перегружен,
сохрани план разбиения на листы из sheets.json; не делай подписи нечитаемыми.
Граница UI и граница листа — разные рамки.

ГРАФИЧЕСКИЙ ЯЗЫК
Используй ровный синий/голубой фон {{background_color}}, белые контуры
и надписи, спокойную светло-голубую второстепенную сетку.
Основной контур заметнее вторичных; размерные/выносные линии тонкие.
Сетка не конкурирует с данными. Никакого свечения, объёмных карточек,
градиентного стекла, декоративных иконок, кодового дождя и состаренной бумаги.
Условные линии:
{{line_legend}}
Шрифт простой, чёткий; все цифры и кириллица должны читаться в итоговом размере.

ОБЪЕКТЫ И ИЕРАРХИЯ
Ниже приведена полная для этого листа ведомость.
Сохрани ID, состав, вложенность, относительное расположение и заданные надписи.
Не добавляй новые кнопки, окна, заголовки или декоративные элементы.
{{component_table}}
```
