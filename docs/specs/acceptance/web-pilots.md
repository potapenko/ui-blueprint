# Web пилоты B01–B06

- Node type: leaf; domain: `uib.web-pilots`.
- Contract: `UIB.WEB-PILOTS@1`; stable clause: `UIB.WEB-PILOTS.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: R01 и browser fixtures/QA.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.PILOTS@1](pilots.md), [UIB.GEOMETRY@1](../product/geometry.md), [UIB.PROJECTIONS@1](../product/projections.md), [UIB.FORMS@1](../product/forms.md), [UIB.CACHE@1](../product/cache.md), [UIB.ACTIONS@1](../product/actions.md).
- Source mapping: TZ 474–482; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-web-pilots-content"></a>

| Web пилот | Обязательный результат |
| --- | --- |
| B01 | Две вкладки/поверхности, повторяющиеся подписи, navigation/remount: точная адресация и инвалидирование refs |
| B02 | Форма с partial draft, debounce, validation и autocomplete: действительный ввод, applied value и остановка на неожиданном переходе |
| B03 | Popup/portal, overlay и frame boundary: anchor/Surface, clipping, hit testing и частичное покрытие |
| B04 | Resize, scroll, text-size и locale: локальная геометрия, transforms и сохранение смысла значений |
| B05 | Изменение родителя/шрифта, потеря событий и лимит ответа: корректный кэш, coverage и эквивалентность full/delta |
| B06 | Одна составная кнопка DOM/AX: разные узлы interaction/design, явные source identities и отсутствие ложных action refs |
