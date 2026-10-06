# Минимальный контракт обмена

- Node type: leaf; domain: `uib.exchange`.
- Contract: `UIB.EXCHANGE@1`; stable clause: `UIB.EXCHANGE.CONTENT`.
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
