# Кэш и atomic delta

- Node type: leaf; domain: `uib.cache`.
- Contract: `UIB.CACHE@1`; stable clause: `UIB.CACHE.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: cache keys, invalidation, replay, upsert/removal.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.EXCHANGE@1](exchange.md), [UIB.PROJECTIONS@1](projections.md), [UIB.LIFECYCLE@1](lifecycle.md).
- Source mapping: TZ 231–298; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-cache-content"></a>

## Кэш и инкрементальные обновления

Локальное состояние переиспользуется между запросами. Снимки, пространственные отношения и результаты проверок кэшируются отдельно. Ключи учитывают Target/Surface generations, schema/plugin version, scope, projection, fields и параметры среды. Pixel-кэш не выдаётся как свежая картинка по факту обновления семантики.

Нативные/браузерные события — сигналы инвалидирования, а не гарантированно полный журнал. Изменения родителя, шрифта, текста, safe area, zoom, viewport или scroll могут затронуть соседей. Минимальный пересчёт допустим при известной зависимости; иначе обновляется более широкая область. При потере событий выполняется bounded resync.

Кэш имеет лимиты памяти, срок жизни, владельца и cleanup на detach. Его вытеснение не означает исчезновение UI-элемента. TTL сам по себе не подтверждает актуальность; перед действием перепроверяется нужная цель. Неизвестное новое значение не заполняется старым значением под свежим timestamp. Исторический факт можно сохранить только как исторический с его временем.

Delta применяется атомарно к локальному графу. Это не обещание атомарного захвата постоянно меняющегося UI. Обязательная совместимость: Target/Surface generations, scope, schema/plugin version, projection, fields и base_revision. Изменившиеся условия требуют совместимого полного снимка.

Upsert заменяет полное представление source-узла в согласованных projection и fields; удаления задаются явно. Для каждого запрошенного свойства новое состояние известно либо обозначено unknown/unsupported/redacted. Поле вне fields имеет selection=not_requested: оно не удаляет отдельно сохранённый исторический факт, но этот факт не объявляется свежим или входящим в текущую проекцию. Изменения children, relations и focus согласованы в одной ревизии. Удаление подтверждается наблюдением в покрытой области; обрезка ответа, смена scope и виртуализация без известной идентичности не становятся безусловным deleted.

Иллюстрация сокращённого сообщения; идентификаторы условные:

```json
{
  "schema_version": "0.3-draft",
  "plugin": {
    "id": "web",
    "version": "0.1"
  },
  "target": {
    "id": "tab7",
    "generation": "g3"
  },
  "surfaces": [
    {
      "surface_id": "document7",
      "generation": "d5"
    }
  ],
  "scope_id": "filter-panel",
  "projection": "interaction",
  "fields": [
    "role",
    "name",
    "state.enabled",
    "state.focused"
  ],
  "base_revision": 41,
  "revision": 42,
  "coverage": {
    "status": "complete",
    "within": "requested_scope_and_fields"
  },
  "upsert": [
    {
      "element_key": "web.dom:e8",
      "surface_id": "document7",
      "role": "textbox",
      "name": "From",
      "state": {
        "focused": true,
        "enabled": true
      }
    }
  ],
  "removed": [
    "web.dom:e14"
  ],
  "focus": {
    "keyboard": "web.dom:e8"
  }
}
```

Полный протокол дополнительно содержит Observation, provenance и доступность свойств; в сокращённом примере перечисленные значения известны. Получатель с другой base_revision или несовместимым контекстом возвращает resync_required. Эквивалентность full/delta проверяется для одной исходной source-state/revision и одинакового покрытия через replay одного журнала или контролируемый checkpoint. Два последовательных live-захвата меняющегося UI не обязаны совпадать. Изменения текста, состояния, фокуса, структуры и геометрии можно выбирать раздельно.
