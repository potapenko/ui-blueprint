# Отмена, teardown и конкуренция

- Node type: leaf; domain: `uib.lifecycle`.
- Contract: `UIB.LIFECYCLE@1`; stable clause: `UIB.LIFECYCLE.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: cancel/dispatch, native handles, очереди Target и изоляция.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.EXCHANGE@1](exchange.md), [UIB.PRIVACY@1](privacy.md).
- Source mapping: TZ 613–622; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-lifecycle-content"></a>

### Отмена и конкуренция

Перед первым живым adapter зафиксировать D02/D04/D05: maximum retained revisions, memory cap, deadlines, допустимые конкурентные чтения и единственного владельца mutation очереди каждого Target. Общий физический ввод дополнительно подчиняется единому host lane. Поддержанная изоляция вкладок или устройств не отменяет общего состояния, если оно действительно разделяется.

Cancel до dispatch удаляет ещё не отправленные шаги. Cancel во время системного вызова останавливает последующие шаги, но не обещает прервать уже принятый ОС input или откатить эффект. Запоздалый callback не возобновляет отменённый сценарий; его можно учесть только для диагностики исхода. При неизвестном эффекте возвращается action_outcome_unknown и сначала заново устанавливается состояние.

Detach/сбой плагина закрывает его подписки, помечает живые handles недействительными и прекращает новую отправку; пользовательское приложение не завершается ради очистки session. При активном неустановленном эффекте сохраняется минимальный redacted terminal receipt, а не ложное «ничего не произошло».

Вытесненная snapshot revision даёт resync_required при обращении к её базе; это не свидетельство исчезновения UI. Ссылки на payload в кэше имеют срок жизни и owner. Запрос, который может пережить teardown, не получает доступ к освобождённым native handles. Длительный syscall одного Target не удерживает глобальную очередь независимых сессий.
