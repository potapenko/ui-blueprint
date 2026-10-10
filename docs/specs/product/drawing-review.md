# Проверка генерации и хранение документа

- Node type: leaf; domain: `uib.drawing-review`.
- Contract: `UIB.DRAWING-REVIEW@2`; stable clause: `UIB.DRAWING-REVIEW.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.2, user confirmation 2026-10-06; C00 source plus HBP-HUMAN-001 user-authorized presentation evolution on 2026-10-10.
- Read when: export QA, checked/approval, baseline и retention.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.DRAWING-PACKAGE@2](drawing-package.md), [UIB.DRAWING-STYLE@2](drawing-style.md).
- Source mapping: DRAWING 277–300; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; HBP-HUMAN-001 presentation rules supersede the earlier exhaustive visible inventory rules; machine truth remains exact.

<a id="uib-drawing-review-content"></a>

## Проверка генерации и применение в QA

ImageGen создаёт графическую подачу и может ошибиться в цифрах, тексте, количестве элементов и расположении. Поэтому workflow разделён: validate DrawingBrief → generate отдельно по запросу → сверить изображение с brief → исправить/перегенерировать выбранные листы → зафиксировать статус. Локальная imagegen-prompt не требует модели; сама генерация является отдельным внешним действием.

Проверить все подписанные размеры и единицы, ID, состав объектов, положения основных зон, состояние и source_kind, размерные anchors, отсутствие добавленных контролов, полноту scope, читаемость в конечном размере и отсутствие утечки данных. Неизвестные значения должны оставаться неизвестными. OCR может помочь сверке, но отсутствие OCR-ошибок не доказывает правильную геометрию.

Для клиентского инженерного изображения численные подписи и смысл обязаны пройти проверку. Если нужна метрически точная координатная подложка или CAD-передача, ImageGen-only результат не объявляется точным: можно добавить проверенный overlay или воспользоваться опциональным детерминированным экспортом. Это не требует строить SVG-движок в основном MVP.

Сохранение документации и QA:

1. До разработки: создать propose-пакет с requirements и статусом draft; согласование человеком фиксирует authority принятого проекта.

2. После реализации: создать отдельный document-пакет из runtime и связать его с build/environment/Observation.

3. При QA: приложить DrawingBrief, machine data, изображение, review и результаты affected scenarios. Не заменять этим реальные interaction/visual checks продукта.

4. При принятии: сохранить immutable revision и именованный approval, а не перезаписать проект текущим кадром.

5. При изменении: создать новую ревизию; compare — дополнительный способ показать разницу, не основной вид документа.

Spec-first порядок сохраняется: accepted intent задаёт ожидание; runtime даёт факт; QA оценивает соответствие. Новый screenshot или generated blueprint не меняет намерение автоматически. При расхождении описать его и запросить нужное продуктовое решение, не обновлять baseline только ради pass.

Документационный export — явное исключение из временности сессии: пользователь выбирает место и срок хранения, документ имеет owner. Не сохранять каждый кадр работы агента как постоянный артефакт. В целевом проекте предложены docs/ui/<surface>/<revision>/ для принятых документов и отдельное application-state хранилище для временных run observations; точное размещение следует правилам проекта.

HBP-HUMAN-001: requires the [human presentation rules](../../engineering-blueprint-guide.md#hbp-human-001--клиентская-подача-2026-10-10), DRAWING@1.2. Full original machine records stay exact; visible inventories/long decimals are superseded.
