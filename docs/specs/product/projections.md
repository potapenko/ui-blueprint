# Scope и interaction/design

- Node type: leaf; domain: `uib.projections`.
- Contract: `UIB.PROJECTIONS@1`; stable clause: `UIB.PROJECTIONS.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: выбор области, coverage, проекции, many-to-many mapping.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.MODEL@1](model.md), [UIB.IDENTITY@1](identity.md).
- Source mapping: TZ 165–184; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-projections-content"></a>

## Выбор области и проекции

Scope задаётся по семантическому запросу, наблюдённому ref, устойчивому locator, точке или прямоугольнику. Ограничения fields, depth и max_elements задают стоимость ответа. По умолчанию не обходится всё приложение.

Контекст включает необходимые label/error, контейнер, соседей, scroll-owner, anchor и возможную блокирующую Surface. Попап может находиться в другом поддереве или окне. Выход за исходный scope допускается только как явно обозначенное включение зависимого контекста внутри разрешённого Target; чужие окна не добавляются молча.

Сначала используются явные связи. Пространственная кластеризация по близости — дополнительный derived/estimated результат, не доказательство структуры компонентов.

Две проекции:

- interaction: единый контрол с именем, состоянием и действиями;

- design: внутренние части, подписи, иконки и геометрические отношения.

Обе используют общий граф, но не требуют соответствия узлов один-к-одному. Идентичности разделены по источникам: AX-кнопка может представлять контейнер, иконку и текст debug-probe. Связи represents/corresponds_to допускают many-to-many и содержат происхождение. logical_component_key создаётся только при явном mapping, а не по совпадению рамок. Эвристические связи не склеивают source IDs и не разрешают действие. Actionable ref остаётся у узла соответствующего backend; декоративный design child не становится кнопкой.

Упрощение представления не удаляет исходные данные из графа. Подавленный проекцией узел не считается исчезнувшим. Внешний сбор может не раскрыть внутренние части: design возвращает partial/not_exposed, а точную причину combine/ignore сообщает только при отдельном подтверждении. Полная design-проекция любого чужого приложения не обещается.

Лимит ответа возвращает coverage, omitted_count или unknown_count и cursor. Cursor привязан к ревизии; после её несовместимого изменения требуется новый запрос. Невидимые, нераскрытые и виртуализированные элементы не перечисляются как полностью исследованные. Смена scope или фильтра полей сама по себе не означает удаление узлов.
