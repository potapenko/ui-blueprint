# Anchors и projection/probe ограничения

- Node type: leaf; domain: `uib.probe-sources`.
- Contract: `UIB.PROBE-SOURCES@1`; stable clause: `UIB.PROBE-SOURCES.CONTENT`.
- Authority: Active / Stability: Evolving; norms + historical evidence; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: R02/R03; measured anchors и детализация.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.REUSE@1](reuse.md).
- Source mapping: TZ 940–955; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-probe-sources-content"></a>

### Blueprint для Compose

[Проект Антона Попова](https://github.com/popovanton0/Blueprint), commit `09287ace7ea4a79cb5a1a08521cd69a365f7bdac`; [Apache-2.0](https://github.com/popovanton0/Blueprint/blob/09287ace7ea4a79cb5a1a08521cd69a365f7bdac/LICENSE), [NOTICE](https://github.com/popovanton0/Blueprint/blob/09287ace7ea4a79cb5a1a08521cd69a365f7bdac/NOTICE).

Читать [blueprint/src/commonMain/kotlin/com/popovanton0/blueprint/BlueprintId.kt](https://github.com/popovanton0/Blueprint/blob/09287ace7ea4a79cb5a1a08521cd69a365f7bdac/blueprint/src/commonMain/kotlin/com/popovanton0/blueprint/BlueprintId.kt), [blueprint/src/commonMain/kotlin/com/popovanton0/blueprint/Blueprint.kt](https://github.com/popovanton0/Blueprint/blob/09287ace7ea4a79cb5a1a08521cd69a365f7bdac/blueprint/src/commonMain/kotlin/com/popovanton0/blueprint/Blueprint.kt) и DSL Anchor/Dimension/GroupScope/MeasureUnit в том же commonMain/kotlin пакете.

Взять явные component markers, runtime LayoutCoordinates и отношения между якорями. Переносимо прежде всего представление измерений и алгоритмы, а не Compose runtime как обязательная зависимость Rust engine. Это встроенная разметка собственного приложения, не внешний сканер. Fixture: icon/text/container с изменяемым padding и размером текста; отсутствие влияния probe на layout. Старые README-ссылки на src/main/java не совпадают с текущим commonMain.

### Blueprint Compose Preview

[Проект GusWard](https://github.com/GusWard/Blueprint-Compose-Preview), commit `95398528a6d5b1f4fa4ce92e1a73f830608eb3fb`; [Apache-2.0](https://github.com/GusWard/Blueprint-Compose-Preview/blob/95398528a6d5b1f4fa4ce92e1a73f830608eb3fb/LICENSE).

Читать [blueprint-compose-preview/src/main/java/uk/co/gusward/blueprint/compose/preview/preview/BlueprintPreview.kt](https://github.com/GusWard/Blueprint-Compose-Preview/blob/95398528a6d5b1f4fa4ce92e1a73f830608eb3fb/blueprint-compose-preview/src/main/java/uk/co/gusward/blueprint/compose/preview/preview/BlueprintPreview.kt), [blueprint-compose-preview/src/main/java/uk/co/gusward/blueprint/compose/preview/grid/logic/BlueprintLineCalculator.kt](https://github.com/GusWard/Blueprint-Compose-Preview/blob/95398528a6d5b1f4fa4ce92e1a73f830608eb3fb/blueprint-compose-preview/src/main/java/uk/co/gusward/blueprint/compose/preview/grid/logic/BlueprintLineCalculator.kt) и items/BlueprintItemData.kt.

Взять автоматическое первоначальное обнаружение, размерные связи и переключение детализации. Исходник использует reflection, выбор inner/outer coordinates, подавление детей и survivor cache. Это эвристики инструмента визуализации, не точный paint contour и не правила нашего кэша. Не копировать сохранение старых непустых данных как fresh. Fixture: merged control, decorative children, disappearing node, empty composition и many-to-many AX/probe mapping.
