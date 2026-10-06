# Приёмка первой цели

Основание: [ТЗ](../../ui-blueprint-spec.md) `UIB.TZ@1.4` и
[руководство](../../engineering-blueprint-guide.md) `UIB.DRAWING@1.1`.
Эта карта распределяет обязательства [плана](../ui-blueprint-development.md),
не создаёт новые продуктовые пороги. Все результаты сейчас `not_started`.

## Сценарии платформ

| Критерий ТЗ | Основные task owners | Доказательство на итоговой сборке |
| --- | --- | --- |
| M01 / B01 | M01,W01,M02,W02 | Повторяющиеся названия, точные окна/вкладки, закрытие/recreate/remount; ни одного чужого действия |
| M02 / B02 | M02,W02,A01 | Реальный ввод, focus/selection/IME при поддержке, draft/applied/outcome; stop на неожиданном переходе |
| M03 / B03 | M01,W01,M02,W02 | Popup/portal/frame, anchor/Surface, clipping, pixels/input где требуются, capture coverage |
| M04 / B04 | G01,M01,W01 | Известные transforms, resize/scroll/text-size/locale; local diff без ложного gap |
| M05 | P01,G02 | Реальные внутренние measurements; off/on probe не меняет layout/focus/hit/AX; source IDs сохраняются |
| M06 / B05 | K01,K02,M01,W01 | Partial errors, timeout, permission change, lost events, resync, независимость Target |
| B06 | W01,G02 | Interaction/design различаются, many-to-many mapping; decorative node не получает action ref |

Q01 интегрирует всю таблицу на одной ревизии; предыдущий module receipt не
заменяет сквозной сценарий. Negative/degraded результат не закрывает positive gate.
Если дополнительный cross-display случай не проверен, не заявлять его поддержку;
это не отменяет обычный M04 и не требует физического монитора для всех проверок.

## Общие семейства требований

| Семейство | Задачи | Обязательная проверка |
| --- | --- | --- |
| IDENTITY | S01,A01,W01,M01,V01 | Target/Surface generations, stale/ambiguous refs, no heuristic action |
| GEOMETRY | G01,G02,P01 | Units/space/transform, различие layout/AX/hit/visible, missing_transform, точные fixture числа |
| SEMANTICS | S01,G02 | false/empty/unknown/unsupported/redacted и selection=not_requested не смешаны |
| FRESHNESS | K01,K02 | Atomic full-node update; совместимость контекста; oracle одной source-state/revision; лимит не deletion |
| ACTIONS | A01,W02,M02,V01 | Delivery отдельно от verification; cancel до/после dispatch; нет повторного unknown submit |
| ISOLATION | M01,W01,K02,V01 | Bounded reads/queues/memory, timeout одного Target не блокирует другой; window-only scope не расширяется |
| EXPORT | E01,E02,V01 | DrawingBrief и self-contained prompt, scene/dimensions/sheets, document/propose/compare, unknown и provenance |
| MODEL_FREE | I01,Q01 | Базовый CLI и imagegen-prompt без ключей, модели, Jev и чужой CLI-цепочки |
| PRIVACY | W01,M01,A01,E01,V01 | Canary secret отсутствует в cache/history/logs/errors/stdout/prompt; redaction до сохранения; pixels отдельно |
| REUSE | R01–R03,C01,I01 | Upstream revision/files/условия/NOTICE + собственная fixture; нет ложного утверждения проверки всего upstream |

GOLDEN01 в S01 обязателен до живых адаптеров: valid/invalid envelopes,
validator exit status, observe→prepare→delivery→verify→delta→check→export и
варианты отказов из ТЗ. Моки доказывают engine contract, а не native/browser delivery.
Expected values задаются отдельно от результата коллектора; сам плагин не свой oracle.

## Человеческий экспорт

E01/E02 охватывают полный observed пример и proposed пример, перегруженный вид
с деталями, явные scope/environment/units/source/validation/approval, безопасные
references, согласованную арифметику и unknown. Document — основной режим.
Генерация изображения и его отдельная проверка не условие model-free CLI.
Предварительные имена файлов/флаги закрепляются в CLI/schema до реализации.

## Производительность и границы проверок

C01 фиксирует D05/D06 после разведочного baseline и до оценки кандидата:
hardware/backend/fixture sizes, cold/warm, p50/p95, memory, calls/resync/output,
ошибки целей/пропуски/false positives и влияние на UI. Q02 сравнивает с исходным
доступным процессом при одинаковых условиях; screenshot-only добавляется по пользе.
Числа здесь не выдумываются; gate нельзя подобрать после результата для pass.
Ускорение не покупается stale cache, потерей полей или непроверенным исходом.

Rust checks выбираются по [development contract](../../specs/development/rust.md).
Изменения identity/permissions/privacy/persistence/concurrency требуют независимого
review по [двухэтапному протоколу](packets.md). Для малого чистого helper достаточно
focused self-check, если он не затрагивает эти риски. Build не заменяет runtime/visual.
QA использует собственные разрешённые fixtures; работа в PlayPhrase.me и чужих
проектах не разрешается ссылками из ТЗ или запуском этого плана.

## Definition of done

P0–P7 закрыты по смыслу; обе живые платформы и P3 приняты; Q01/Q02 актуальны;
все обязательные review/QA имеют evidence. Поставка воспроизводится из текущей
ветки: lockfile/schema/examples, supported matrix, сборка/установка/первый fixture,
LICENSE/NOTICE, recovery/cleanup и честные ограничения. Schema compatibility
фиксируется после обоих P2/P5 и M05, а не после одного Web happy path.
Checkpoint создан; нет обязательного waiting gate. Root сверяет каждое семейство
и только тогда завершает цель. Количество пакетов, строк и зелёных tests не DoD.
