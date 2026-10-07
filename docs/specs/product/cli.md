# CLI и bounded output

- Node type: leaf; domain: `uib.cli`.
- Contract: `UIB.CLI@2`; stable clauses: `UIB.CLI.CONTENT`, `UIB.CLI.INSPECT`; supersedes @1.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 CONTENT preserved; INSPECT selected under ROADMAP/PLAN.UIB@1 by [L01 packet](../../plans/ui-blueprint/packets/L01-inspect-json.md).
- Read when: CLI commands, compact/JSON и публикация.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.EXCHANGE@1](exchange.md), [UIB.PRIVACY@1](privacy.md).
- Source mapping: TZ 372–398; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; INSPECT fixes only the local CLI representation below.

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

## UIB.CLI.INSPECT — saved canonical node inspection

CLI.CONTENT requires compact and JSON access. Observed compact507383d has no JSON
InspectResult artifact; this delegated technical choice supplies a CLI-owned
output envelope, not another graph or core/analysis wire change.

```text
inspect --snapshot FILE --ref SOURCE_KEY_JSON --view interaction|design --max-input-bytes N --max-output-bytes N [--json]
```

Selector is the existing strict canonical SourceKey JSON {namespace,key}; never
BackendRef, name/coordinate lookup or live action authority. Explicit positive
budgets include selector UTF-8 bytes plus file bytes, and complete output/newline.
Input is core0.1 Snapshot or observed ChannelResponse with unchanged Snapshot.
Missing exact node: target_unresolved/4; invalid/limit2; IO1; unsupported mode5.
Found node returns0 even when source coverage/properties are partial/unknown.
Compact preserves source context/evidence; views reorder presentation only.

JSON emits exactly these fields in one complete object plus newline:
output_version="1.0.0", kind="inspection", source="saved",
live_revalidated=false, selector (existing SourceKey), requested_view (existing
Projection), snapshot (full unchanged canonical Snapshot). The selector resolves
exactly within that Snapshot. No new freshness, projected graph or computed-result
claim; no payload reference IO. Canonical availability/coverage/consistency,
properties, relations, source declarations and Observation identities stay intact.
Serialize borrowed canonical data through bounded output; no unbounded Value or
second Snapshot owner. Prepublication errors emit no partial stdout; diagnostics
stay on stderr. OS write failure remains IO and may leave a partial physical write.

output_version versions this CLI envelope only; embedded Context.schema_version
stays0.1.0. Core Document negotiation/variants and analysis0.2 remain unchanged.
Future envelope changes require versioned compatibility. The product does not
import this envelope, so this choice adds no parser/schema-validation framework.
Registration is not runtime acceptance or completion of full projections/actions.
