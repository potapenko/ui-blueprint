# DrawingBrief и локальный validator

- Node type: leaf; domain: `uib.drawing-package`.
- Contract: `UIB.DRAWING-PACKAGE@2`; stable clause: `UIB.DRAWING-PACKAGE.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.2, user confirmation 2026-10-06; C00 source plus HBP-HUMAN-001 user-authorized presentation evolution on 2026-10-10.
- Read when: export compiler, manifest, ProposedLayout и validation.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.PRIVACY@1](privacy.md), [UIB.DRAWING-GEOMETRY@2](drawing-geometry.md).
- Source mapping: DRAWING 131–157; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; HBP-HUMAN-001 presentation rules supersede the earlier exhaustive visible inventory rules; machine truth remains exact.

<a id="uib-drawing-package-content"></a>

## Документ DrawingBrief и пакет генерации

Команда imagegen-prompt должна формировать развёрнутый пакет, а не одну короткую строку. Ссылка на это руководство сопровождается его ID/revision и включёнными применимыми правилами: генератор не обязан уметь читать приватную Page по URL.

Предлагаемый состав экспортного каталога:

```text
drawing-package/
  manifest.json           происхождение, версия, файлы и статусы
  drawing-brief.md         человекочитаемое инженерное задание
  scene.json              выбранные объекты, bounds, состояния и связи
  dimensions.json         значения, anchors, единицы, источник, допуск
  sheets.json             G01/D01/... и размещение видов
  prompt.txt              полностью собранный текст для ImageGen
  references/             только разрешённые crops или исходные изображения
  output/                 результат генерации, если она отдельно запущена
  review.md               сверка изображения и статус документа
```

Обязательное смысловое содержание DrawingBrief: doc identity/revision; source_kind; scope и coverage; цель документа; аудитория; environment/state; пространство и единицы; components; dimensions; relations; unknowns; требования к листам и стилю; точные надписи; недопустимые изменения; условия проверки; ссылки на спецификацию и исходные данные.

Пакет валидируется локально: ссылки на объекты разрешаются, размерные anchors существуют, единицы совместимы, derived-суммы согласованы, proposal не помечен observed, sensitive content исключён. Ошибка этих условий возвращается до генерации. Неполный scope допустим только с явным coverage и отметкой того, что не изображено.

Для наблюдения источником служит разрешённый Snapshot/Observation. Для proposal используется отдельный ProposedLayout с требованиями и целевыми размерами; он не подмешивается в граф наблюдённых фактов. Изменения проекта не переписывают историческую evidence.

DrawingBrief является каноническим текстовым заданием визуализации. Поле style не может переписать измерения. В будущем те же scene/dimensions можно подать в SVG/PDF-экспорт; этот путь необязателен для ImageGen.

HBP-HUMAN-001: requires the [human presentation rules](../../engineering-blueprint-guide.md#hbp-human-001--клиентская-подача-2026-10-10), DRAWING@1.2. Full original machine records stay exact; visible inventories/long decimals are superseded.
