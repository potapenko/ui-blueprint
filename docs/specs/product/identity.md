# Идентичность и свежая привязка

- Node type: leaf; domain: `uib.identity`.
- Contract: `UIB.IDENTITY@1`; stable clause: `UIB.IDENTITY.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: Target/Surface/ref/locator, matching, revalidation.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.BOUNDARIES@1](boundaries.md).
- Source mapping: TZ 201–212; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-identity-content"></a>

## Идентичность и актуальность цели

Target включает процессную сессию, точное окно/вкладку/устройство и target_generation. Surface имеет свой surface_id и generation, например для отдельного popup или нового документа. Один PID, window ID, URL или заголовок недостаточен: они могут повторно использоваться.

Element имеет source namespace и session element_key для хранения; ref привязан к конкретным источнику, наблюдению и поколению. Locator служит повторному поиску. Координаты не являются устойчивым ID. Remount, перезапуск процесса, закрытие попапа и переиспользование виртуализированной строки инвалидируют соответствующие refs.

Перед действием проверяются Target, Surface, ref, единственность совпадения и необходимые свойства. При сомнении возвращаются stale_target или ambiguous_target. Устаревшая координата не применяется к соседней кнопке. Для повторного исполнения после повторного поиска создаётся актуальная привязка.

Сопоставление для diff допускает reported key или эвристические кандидаты с явной отметкой. Вероятное совпадение помогает анализу, но никогда само по себе не даёт права активировать элемент, в том числе если его предложил Jev.

На нескольких дисплеях сохраняются применимые пространства и преобразования; одного глобального scale недостаточно. При перемещении Surface обновляются её координаты, а внутренние изменения сравниваются в выбранном локальном пространстве.
