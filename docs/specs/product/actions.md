# Исполнитель, отмена и конкуренция

- Node type: leaf; domain: `uib.actions`.
- Contract: `UIB.ACTIONS@1`; stable clause: `UIB.ACTIONS.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: prepare/resolve/act/verify, input ownership, cancel/teardown.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.EXCHANGE@1](exchange.md), [UIB.FORMS@1](forms.md), [UIB.LIFECYCLE@1](lifecycle.md).
- Source mapping: TZ 213–230; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-actions-content"></a>

## Исполнитель действий и протокол результата

Последовательность observe → prepare → resolve → act → verify → delta связывает каждый шаг с текущим наблюдением. Prepare описывает план и предусловия; он не создаёт новую авторизацию. Обычные уже разрешённые действия не требуют дополнительного пользовательского подтверждения только из-за наличия prepare.

Независимые заполнения допускаются в одном вызове, но выполняются упорядоченно. Исполнитель перепроверяет предусловия и результаты. Попап, асинхронная валидация, новая форма, навигация, изменение доступности или разрешений прерывают предсказуемый участок для свежего наблюдения.

Политика по умолчанию stop_on_error. Зависимые действия прекращаются после failed, interrupted или unknown_outcome. Продолжение независимых шагов допустимо только при явно выбранной политике сценария. Batch не является транзакцией и не обещает отката выполненных действий.

В Transition фиксируются запрошенное намерение, фактические backend/input_modality, цель, шаги, наблюдения до/после и результат. AX activate может подтвердить активацию, но не попадание указателя; setter не доказывает работу экранной клавиатуры; callback не доказывает доставку реального жеста.

Результат содержит completed_steps, outcome каждого запущенного шага, stopped_at и непройденное условие. Неизвестный исход отправки формы не повторяется автоматически. Для продолжения сначала устанавливается фактическое состояние приложения.

Единый ответ: schema_version, request_id, target, observation/snapshot, capabilities, coverage, data, issues. Ошибки имеют машинный code, scope и достаточную причину. Основные коды: unsupported, plugin_not_installed, incompatible_version, permission_required, target_unresolved, stale_target, ambiguous_target, unstable_state, incomplete_scope, missing_transform, resync_required, timeout, interrupted, action_outcome_unknown. external_surface_detected — диагностическое событие; оно становится причиной остановки только при неразрешённой или неустановленной цели/связи, а не при любом появлении известной поверхности.

Timeout ограничен конкретным условием и deadline. Ожидание относится к выбранному результату, а не к полной сетевой тишине. После отмены не продолжаются скрытые клики. Смена backend не обходит tool-policy или требуемую передачу действия пользователю.

Физические клавиатура, указатель и фокус имеют одного владельца на время воздействия. Параллельные адресуемые вкладки/устройства допустимы только при поддержанной изоляции и отсутствии конфликтующего состояния.
