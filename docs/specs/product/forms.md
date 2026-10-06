# Семантика форм и ввода

- Node type: leaf; domain: `uib.forms`.
- Contract: `UIB.FORMS@1`; stable clause: `UIB.FORMS.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: поля, фокус, selection, IME, draft/applied.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.MODEL@1](model.md).
- Source mapping: TZ 185–200; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-forms-content"></a>

## Семантика форм и ввода

Собираются доступные name/accessibility_name, visible_text, description, value, placeholder, input_kind, required, enabled, readonly, checked, selected, expanded, focused, invalid и действия. Видимость, включённость в accessibility, возможность фокуса и активации независимы.

Отдельно хранятся keyboard_focus, accessibility_focus, active_descendant и, при поддержке, text_selection/caret и composition_state. Система не угадывает IME или положение каретки по конечной строке. Единицы offsets указываются явно.

Связи labelled_by, described_by, error_for, controls, owns, member_of, anchored_to и return_target сохраняют смысл. Tab-порядок, чтение и направленный фокус различаются. Неизвестный маршрут фокуса не заменяется порядком геометрических координат.

Намерения focus, activate, fill, type, select_option, set_checked, scroll, press, submit и dismiss имеют конкретный backend и способ ввода. set_checked(true) отличается от toggle. Семантическая активация, pointer, touch, keyboard и remote не взаимозаменяемы при проверке самого жеста. Прямой setter и ввод через IME имеют разные события.

Для автокомплита различаются введённый draft, активная подсказка и подтверждённый выбор. Для формы различаются DOM/native value, прикладное applied value и завершённый бизнес-результат. Их связь определяется profile/Expectation; имя поля не раскрывает её автоматически.

Dismiss может сохранить изменения, отменить их или быть недоступным. Enter может выбрать подсказку или отправить форму, поэтому не добавляется исполнителем по умолчанию. Проверка invalid=false не доказывает успешную серверную операцию.

Защищённые значения редактируются через ограниченный механизм ввода, но не возвращаются в диагностический граф. Отсутствие значения из-за редактирования обозначается redacted, а не пустой строкой.
