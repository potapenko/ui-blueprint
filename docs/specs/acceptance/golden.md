# GOLDEN01 и validator

- Node type: leaf; domain: `uib.golden`.
- Contract: `UIB.GOLDEN@1`; stable clause: `UIB.GOLDEN.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: P1 normalized fixtures, schema validation, full/delta oracle.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.CACHE@1](../product/cache.md), [UIB.ACTIONS@1](../product/actions.md), [UIB.GEOMETRY@1](../product/geometry.md).
- Source mapping: TZ 633–652; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-golden-content"></a>

### GOLDEN01 Один полный путь

До подключения живых адаптеров разработчик создаёт версионированный набор goldens и validator. Это обязательный результат P1, а не замена живых пилотов. Сценарий ниже содержит синтетические данные и не выполняет действия в приложении.

Начальная база S10: session a1, target native-app:g1, surface window-1:w1, scope form-1. Есть source node macos.ax:check-1 с role checkbox, name «Пример», state.enabled=true и state.checked=false. Его backend ref относится к S10/w1. Источник AX reported; изображение и hit_region не запрашивались. Пустое unknown значение не заменяет checked=false.

| Шаг | Вход и ожидаемый результат |
| --- | --- |
| Observe | Запрос конкретных role/name/state полей возвращает S10, свой Observation interval, coverage для этого scope/fields и текущие capabilities. |
| Prepare/resolve | set_checked(true) указывает S10 ref, ожидает window generation w1, enabled=true и доступный setter/action. Все условия подтверждаются перед dispatch. |
| Act | Backend принял ввод: delivery подтверждена, verification ещё не завершена. Пользовательский успех пока не объявлен. |
| Observe/verify | S11 того же target/surface показывает checked=true. Если значение не наблюдается или target сменился, нет ложного succeeded. |
| Delta | К S10 применяется изменение на S11 с полным source-node представлением в выбранных fields, base_revision и всеми ключами совместимости. |
| Check | Expectation checked=true, expected_from=user_scenario, возвращает pass со ссылкой на S11 evidence. |
| Export | compare-промпт говорит только об изменившемся checkbox; геометрические размеры и pixels не выдумываются. |

Варианты того же набора: readonly/unsupported property, смена window generation до dispatch, исчезнувший target, две одинаковые подписи, отмена после принятия input, потеря S10 из кэша, пропуск delta revision и redacted поле. Validator обязан отвергать несовместимые IDs/units/version/required fields; отсутствие доказательства не превращается в pass.

Golden full/delta сравнивается на одном записанном источнике состояния или контролируемом checkpoint, а не на последовательных живых снимках. Платформенные goldens Web и Mac сохраняют свои raw roles и channels, проходя один общий schema validator. В P1 публикуются валидный и намеренно невалидный примеры для каждого обязательного envelope, а также документированный validator exit status.
