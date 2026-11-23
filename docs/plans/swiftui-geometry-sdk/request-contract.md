# Контракт живого запроса
Часть [PLAN.UIB.SWIFTUI-GEOMETRY@3](../swiftui-geometry-sdk.md). Это предлагаемые semantics; конкретное совместимое wire/CLI-представление регистрируется до реализации.

## Вход и ответ
Вход: exact app/run/device/window, selector или root, requested properties/relations, desired coordinate space, layout mode и deadline. Один find-and-measure выполняет новый сбор, поиск и вычисление; агент не готовит промежуточные JSON вручную.

Selector разрешается в свежем дереве: observed node, source-reported устойчивый ID либо однозначная structural/semantic привязка. Недостаточно определённый запрос возвращает ограниченный список кандидатов. Старый индекс или совпавший label не выбирает соседний элемент автоматически.

Ответ: request ID, target/run/window generations, observation ID, начало/конец сбора, backend/version, scope/coverage/consistency и значения с units/space/frame kind. Requested properties сохраняют known/unknown/unsupported/redacted. Compact output не скрывает пропуски.

## Режимы
| Режим | Значение |
| --- | --- |
| Live current, обычный | Новый source read после запроса: последняя доступная вычисленная layout-геометрия. При переходе это sample с явным временем/consistency, а не обещание финального кадра. |
| Live settled, явный | Дождаться готовности и квалифицированной стабильности выбранной области в пределах deadline, когда задача требует конечного размещения. |
| Saved observation, явный | Анализ указанного immutable Snapshot с исходным временем; не текущее состояние приложения. |

«Свежий» означает чтение в указанном интервале, не гарантию неизменности UI после ответа. Atomic GPU/presented frame не обещается. Unknown consistency не переименовывается в stable.

## Lifecycle одного запроса
1. Проверить protocol/capabilities, scope, run/window binding и аргументы до доступа к UI.
2. Получить место в bounded acquisition lane; ожидание входит в deadline.
3. Проверить наличие вычисленной geometry; при её отсутствии bounded ожидание layout-ready допустимо. Только явный settled дополнительно ждёт стабильность выбранной области. Не менять UI ради готовности.
4. Получить новое дерево и geometry с общей source basis; не склеивать старый tree с frames другой эпохи.
5. Разрешить selector, проверить уникальность, принадлежность и необходимые свойства.
6. Проверить context drift за acquisition. Доказанно смешанные epochs не использовать для compound geometry. Unknown consistency помечать, не превращая её автоматически в недоступность известного scalar frame. Backend без доказательства стабильности не квалифицирован только для явного settled.
7. Нормализовать, выполнить Rust relations и вернуть один terminal result.

Ожидание layout — ограниченная часть запроса, не бесконечные повторные attach/dump. После backend error/timeout нет скрытого retry. Новый запрос после обычного завершения перехода — новая acquisition. Cancel удаляет ещё не начатую работу; late result не публикуется.

## Реакция на изменения
| Событие | Обязательное поведение |
| --- | --- |
| Resize/reflow | Следующий запрос получает новые frames; drift во время сбора отражается явно. |
| Смена экрана/tab | Discovery текущих roots; прежний экран не выдаётся как текущий. |
| Popup/sheet появился | Автоматически обнаружить root и подтверждённое owner relation; не считать его частью content по умолчанию. |
| Popup close/reopen | Старый occurrence ref становится stale при совпавшем имени/type. |
| Dynamic insert/remove/reorder | Фактический новый состав без source edits; removal подтверждается только в covered scope. |
| Scroll/virtualized reuse | Новые позиции/occurrences; offscreen-mounted и не созданная lazy row различаются. |
| Text/font/locale/Dynamic Type | Новый layout и размеры соседей; тот же ID не оправдывает старую геометрию. |
| Keyboard/safe area/orientation | Новый context и mapping, без guessed insets. |
| Другой display/scale | Перепроверить transforms; не использовать один process-global scale. |
| Restart/scene recreation | Старый session/ref недействителен; fresh bind нового экземпляра. |

События — hints invalidation, не полный журнал. Поэтому первая реализация читает источник заново на каждый live request. TTL или отсутствие замеченных событий не доказывает свежесть.

## Экрана или элемента ещё нет
Не запущенное/не готовое приложение возвращает target-unavailable/not-ready, не размеры из исходника. Для отсутствующего узла различать not-mounted и not-materialized только при source evidence; иначе честно описать отсутствие в текущем scope.

При отсутствующей вычисленной geometry ждать только до deadline, затем вернуть конкретную причину. Наличие анимации само по себе не запрещает current sample. Blueprint не открывает экраны, не прокручивает и не создаёт offscreen views. После отдельно разрешённого обычного действия агента тот же запрос работает на той же сборке.

После монтажа новые nodes доступны автоматически. Отсутствие заранее написанного marker не закрывает доступ к реально обнаруживаемому source node.

## Identity и batch
Ref связан с observation/source/run/surface generation. Pointer, row index, label, type и structural path по отдельности не гарантируют continuity. Locator разрешается заново; неоднозначность возвращается, а не скрывается. Эвристическое соответствие для diff не является action identity.

Width/height/gap/inside в одном batch используют один sample. Ответы разных layout epochs не смешиваются без явного comparison. Несколько операций над saved Snapshot допустимы именно как исторический анализ.

Дополнительный semantic alias может улучшить поиск, но не должен быть обязательной ручной меткой для появления geometry каждого нового View.

## Полнота и причины отказа
Различать not-found, ambiguous, stale, unavailable, layout-pending, changing-layout, unsupported-field, missing-transform, permission-required, busy, deadline, resource-limit. Указывать scope/step и допустимое следующее действие, без произвольного «пересобери/попробуй другой PID».

Полный tree request сохраняет все доступные nodes выбранного scope локально. Выдача страниц одной immutable observation допустима; truncation не равна complete. Отсутствие в обрезанной странице не доказывает not-mounted. Required mounted case с unknown layout остаётся незавершённой поддержкой.
