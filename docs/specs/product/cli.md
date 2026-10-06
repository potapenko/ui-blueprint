# CLI и bounded output

- Node type: leaf; domain: `uib.cli`.
- Contract: `UIB.CLI@1`; stable clause: `UIB.CLI.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: CLI commands, compact/JSON и публикация.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.EXCHANGE@1](exchange.md), [UIB.PRIVACY@1](privacy.md).
- Source mapping: TZ 372–398; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-cli-content"></a>

## Консольный интерфейс

Одна команда uiblueprint адресует собственный движок и выбранный плагин. Синтаксис предварительный; команды не реализованы.

| Команда | Результат |
| --- | --- |
| plugins list | Установленные плагины, версии, совместимость |
| capabilities --plugin web --target … | Поддержка конкретных наблюдений/действий |
| observe --target … --scope … --mode semantic | Снимок семантики без обязательного изображения |
| observe --target … --scope … --mode geometry | Доступная геометрия и связи |
| observe --target … --scope … --mode pixels | Изображение при поддержке backend |
| inspect --snapshot … --ref … --view interaction | Компактный контрол для взаимодействия |
| inspect --snapshot … --ref … --view design | Детальная геометрия компонента |
| neighbors --snapshot … --ref … | Нужное окружение с ограничениями области |
| measure --snapshot … --from … --to … | Расстояние или отношение |
| diff --before … --after … | Сопоставление состояний с достоверностью |
| changes --since … | Совместимый поток delta или resync_required |
| check --snapshot … --expectations … | Проверки с происхождением требований |
| form describe --target … --scope … | Поля, отношения, draft/selection/focus |
| action prepare --scenario … | Ограниченный план действий |
| action execute --plan … | Выполнение разрешённого плана с поэтапным результатом |
| imagegen-prompt --snapshot … --purpose document --profile blue-engineering | Полный DrawingBrief, план листов и подробный ImageGen prompt |
| imagegen-prompt --before … --after … --purpose compare | Локальный промпт сравнения |
| analyze --extension jev --scope … | Явный вызов опциональной классификации |

Snapshot ID и Observation ID различаются; CLI может вернуть оба. Read-only команды не подразумевают focus/scroll/activate. MCP при необходимости предоставляет те же операции без второго движка. Полное состояние хранится локально; stdout по умолчанию короткий, JSON доступен явно.
