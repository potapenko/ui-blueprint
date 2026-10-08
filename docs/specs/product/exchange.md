# Минимальный контракт обмена

- Node type: leaf; domain: `uib.exchange`.
- Contract: `UIB.EXCHANGE@2`; stable clauses: `UIB.EXCHANGE.CONTENT`, `UIB.EXCHANGE.COMPOSITION`; supersedes @1 additively.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: request/session/property/time/error/JSON envelopes.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.MODEL@1](model.md), [UIB.IDENTITY@1](identity.md).
- Source mapping: TZ 577–596; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-exchange-content"></a>

## Минимальный контракт обмена для реализации

Предложенная семантика ниже должна быть выражена в schema и контрактных tests P1; точное расположение типов определяется D03.

**Запрос.** Обязательны request_id, schema_version, operation, target/session identity, scope и deadline для внешнего обращения. fields/projection и freshness policy задаются явно либо получают документированные defaults. Запрос действия дополнительно содержит актуальный backend ref и условия, наблюдение — необходимые каналы. Ограничение глубины/числа узлов не должно молча становиться разрешением неограниченного обхода после ошибки.

**Сессия.** attach устанавливает plugin version, capabilities и точную разрешённую область. Каждый request принадлежит одной сессии; opaque handles непереносимы в другую. subscribe при поддержке только инвалидирует и обновляет известное состояние. detach прекращает подписки, ожидания и очередь ещё не отправленного ввода, освобождает run-owned ресурсы. Он не закрывает пользовательское приложение и не меняет его данные.

**Свойство.** У каждого запрошенного свойства availability обязательна. При known присутствует value, включая false или пустую строку; при unknown, unsupported или redacted value отсутствует. not_requested относится только к selection. Подлинное nullable значение допускается лишь при явном nullable-типе в D03 и не обозначает неизвестность. Evidence связывает его с source namespace, Observation, временем и методом; допустима общая запись evidence для группы свойств, если их происхождение одинаково. Source declarations и normative expectations отделены от измерений. JSON null не используется одновременно для «неизвестно», «очищено» и «не запрашивалось».

**Время.** Observation хранит начало/конец сбора и clock_domain. Внутри одного процесса для deadlines используется монотонное время; UTC служит корреляции и отчёту. Timestamp из изображения, другого процесса/устройства или удалённого backend нельзя сравнивать как общий monotonic clock без установленного преобразования. Проверка согласованности сообщает свой метод и неопределённость; глобальный атомарный snapshot не обещается.

**Snapshot и delta.** Snapshot имеет собственную revision и контекст target/surface generations, scope, projection, fields и schema/plugin version. Delta применяется целиком к совпадающей базе либо отклоняется с resync_required. Поле source_state идентифицирует воспроизводимую основу oracle, когда такая основа доступна; live observer без неё не выдумывает глобальную ревизию ОС. Снимки разных каналов могут иметь разные Observation, сохраняя связь и границы согласованности.

**Действие.** Ответ отдельно фиксирует фактическую доставку и проверку ожидаемого результата. Возврат «API принял запрос» не равен succeeded пользовательского сценария. Если эффект уже мог начаться к моменту cancel/timeout, сохраняются выполненные шаги и action_outcome_unknown для неопределённого эффекта; rollback не обещается. Request ID помогает сопоставлять ответы, но не делает внешнее действие идемпотентным. Повтор неизвестного mutation-запроса не выполняется без установления исхода.

**Ошибки.** Unsupported означает отсутствие возможности, permission_required — недостающие права, target_unresolved — неустановленную адресацию, stale_target — устаревшую привязку. Эти результаты не смешиваются с mismatch пользовательского ожидания. Каждая ошибка содержит affected scope, failed step при наличии и recovery class; она не требует раскрытия приватных значений.

**Вывод.** Один режим JSON имеет стабильную схему; диагностический текст не смешивается с машинным stdout. Secrets не передаются в argv. Compact output содержит полноту и статус, а не только удобные значения. Большой результат имеет bounded pagination с привязкой cursor к revision. Exit-status mapping фиксируется до публикации CLI и тестируется отдельно от внутренних error codes.


## UIB.EXCHANGE.COMPOSITION — explicit AX/probe design response

`M05-COMPOSITION-001`, selected under the [Integration packet](../../plans/ui-blueprint/packets/S01-native-proof.md#m05-design-composition-admission--2026-10-08)
and approved PLAN.UIB@1 P3/M05, adds one bounded semantic admission case without
changing core0.1 types, fields, generated schema, plugin negotiation or old fixtures.
Existing homogeneous ChannelResponse remains valid. The only mixed case is
channel=opt_in_layout_probe, Snapshot projection=design and the set of nested
Observation.channel exactly {external_semantics,opt_in_layout_probe}. No capture,
third-channel/general mixture or missing wrapper-channel observation is allowed.
Full Snapshot/evidence/privacy/session/target validation still applies.

Wrapper channel identifies its publication slot; nested Observations preserve their
actual source/channel, clocks, freshness and coverage. AX is never relabelled as
probe. The composed response does not satisfy a separate requested AX slot or
conceal its failure. Standalone Snapshot continues to support attributed source
composition. A schema-valid stored document is not request/session authorization.

Before accepting/publishing any observed ChannelResponse, the actual session owner
must check EACH nested Observation.channel against BOTH the original Observe
request and that session's observe capability status Supported/Partial. Missing,
Unsupported or PermissionRequired capability and any unrequested nested channel
refuse; an allowed wrapper alone is insufficient. Preserve context/generations,
correlation, freshness policy on all observations, total nodes/depth/bytes/work
bounds, duplicate-channel refusal, independent slot completion and earlier ACKs.
Failed-channel behavior is unchanged. No new collection/action/pixel authority.

This is an explicit additive relaxation of the former homogeneous-only semantic
validator, not a general multi-channel envelope or independent acceptance. Legacy
core0.1/analysis0.2 and126 existing golden results remain unchanged; Native
Observe→component inspection requires its own saved integration/runtime proof.
