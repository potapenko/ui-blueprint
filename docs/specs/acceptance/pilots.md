# Равноправные Mac/Web пилоты

- Node type: leaf; domain: `uib.pilots`.
- Contract: `UIB.PILOTS@1`; stable clause: `UIB.PILOTS.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: fixtures и запрет замены positive gate отрицательным.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.BOUNDARIES@1](../product/boundaries.md).
- Source mapping: TZ 455–460; TZ 483–484; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-pilots-content"></a>

## Равноправные пилоты Mac и Web

Первую схему нельзя принять по одному веб-сценарию. Mac и Web имеют отдельные обязательные результаты одинаковой значимости. Порядок реализации и распределение работ определяются в отдельном проекте, а не фиксируются как Web-first.

Пилоты выполняются на контролируемых разрешённых fixtures с известными исходными состояниями. Это не разрешение менять реальный PlayPhrase.me. Для положительных сценариев заранее выбирается среда, где доступны требуемые наблюдения и ввод. Успешный negative/degraded тест не закрывает отсутствующее положительное доказательство.

Результаты двух пилотов сопоставляются по общим свойствам и различиям. Если нужна новая сущность для native-поверхности, schema расширяется явно; она не маскируется под DOM или обязательный viewport.
