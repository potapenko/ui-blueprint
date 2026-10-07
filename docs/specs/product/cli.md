# CLI и bounded output

- Node type: leaf; domain: `uib.cli`.
- Contract: `UIB.CLI@6`; stable clauses: `UIB.CLI.CONTENT`, `UIB.CLI.INSPECT`, `UIB.CLI.OBSERVE`; routes DIFF@2/CLI-ACTIONS@1; supersedes @5.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 CONTENT preserved; INSPECT selected under ROADMAP/PLAN.UIB@1 by [L01 packet](../../plans/ui-blueprint/packets/L01-inspect-json.md); CLI-ACTIONS selected by [the action caller packet](../../plans/ui-blueprint/packets/L01-actions-contract.md).
- Read when: CLI commands, compact/JSON и публикация.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.EXCHANGE@1](exchange.md), [UIB.PRIVACY@1](privacy.md).
- Conditional requires, recorded diff only: [UIB.CLI-DIFF@2](cli-diff.md); existing clauses unchanged.
- Conditional requires, action caller only: [UIB.CLI-ACTIONS@1](cli-actions.md); selected first single-step syntax/output/authority/exits, no full scenario claim.
- Source mapping: TZ 372–398; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; INSPECT fixes its local representation; CLI-ACTIONS selects concrete syntax/authority/output/exits for its first single-step scope over preliminary CONTENT examples.

<a id="uib-cli-content"></a>

## Консольный интерфейс

Одна команда uiblueprint адресует собственный движок и выбранный плагин. Таблица сохраняет предварительный синтаксис CONTENT; конкретные интерфейсы inspect/observe/diff и [single-step actions](cli-actions.md) определены зарегистрированными контрактами ниже.

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

## UIB.CLI.OBSERVE — explicit guarded acquisition
Selected under ROADMAP/PLAN.UIB@1 by [live caller packet](../../plans/ui-blueprint/packets/L01-live-observe.md); CONTENT/INSPECT and canonical versions unchanged.
`observe --connection FILE --request FILE --worker ABSOLUTE_PATH --max-input-bytes N --max-output-bytes N` consumes two explicit regular files under one positive aggregate byte budget; no implicit discovery, source launch, permission request or secrets in argv.
Connection is a trusted operator capability (never UI/response-derived), strict JSON object with connection_version="1.0.0", target (canonical Identity), session (canonical SessionDescriptor), host_limits, attach_deadline_ms (positive), provider.
host_limits contains exactly existing HostLimits fields: workers, worker_bytes, publication_reserve, bootstrap_bytes, parent_bytes, input_bytes, ingress_bytes, output_bytes, request_output_bytes, completion_groups, control_bytes, cleanup_ms, retained_domain_bytes, retained_per_worker, main_stack_bytes, watchdog_stack_bytes. Validate all at/below D05; no hidden defaults.
provider is tagged by backend: native_fixture {helper_executable:absolute path, configuration:opaque UTF-8 string≤4032 bytes, channels:nonzero mask≤7}; web {setup:existing WebSetup, selection:existing WebSelection}. All records reject duplicate/unknown fields; no private Rust layout serialized. Native is trusted own-fixture binding only, not PID/title selection; Web requires established exact CDP target/document.
CLI features macos and web opt into existing host (web enables host/web); defaults empty. Supported live host is macOS; disabled/unsupported feature/platform refuses5. Core commands never spawn. Worker/helper placement is explicit until distribution selection; no installation or external dependency update.
Request is canonical core0.1 Request Document, operation Observe only. Session/target/plugin/surfaces/scope/channel authority must agree before dispatch. Only request.clock_domain is rebound after actual Attached; original fields/limits/freshness/identity remain. Parent monotonic request/attach deadlines and existing bounded cleanup apply.
Output is NDJSON: one unchanged ACKed canonical ChannelResponse Document per line, canonical channel order0/1/2. Parent never parses response JSON or builds a combined Snapshot. Reserve output bytes including line endings before dispatch; each full line is admitted before writing. Setup/no-commit failure leaves stdout empty; late failure preserves earlier ACKs. Physical IO failure may leave a partial write and returns1.
Worker validates typed channel outcome and carries a fixed incomplete bit through matching frame/commit/ACK metadata: Failed or Snapshot coverage partial/unknown/nonzero omitted/unknown count. Only ACKed bits survive; no graph parse/copy in parent or pool growth. Completed delivery alone does not mean observation success.
Hold completion leases through publication; always shutdown/reap owned worker/helpers, never target application/browser. CleanupPending/quarantine is failure, not confirmed shutdown. Do not invent rollback or retry.
Exits:0 Completed with all requested channels committed and none incomplete;4 canonical incomplete/unavailable, missing channels, cancellation, timeout. Host PermissionDenied/ResyncRequired/StaleOperation/Busy/DeadlineExpired→4; InvalidLimits/ResourceLimit/Overflow/InvalidInput→2; AllocationFailure/SystemAllocationFailure/Io/WorkerFailed/InvalidState/InvalidControl/CleanupPending→1. Output IO or unconfirmed cleanup overrides with1. Missing executables→1; malformed connection/version/binding→2; unsupported backend/feature→5.
Diagnostics are fixed codes/counts on stderr, never raw configuration/endpoint/UI text; no mutation/effect-unknown path. Private helper configuration validation remains its provider's responsibility. This caller exposes existing bounded APIs; it does not qualify arbitrary apps/sites or close live/platform gates.
