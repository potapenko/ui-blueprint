# Документирование и пакет ImageGen

- Node type: leaf; domain: `uib.export`.
- Contract: `UIB.EXPORT@2`; stable clause: `UIB.EXPORT.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.2, user confirmation 2026-10-06; C00 source plus HBP-HUMAN-001 user-authorized presentation evolution on 2026-10-10.
- Read when: E01/E02, imagegen-prompt, человеческий экспорт.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.DRAWING-PACKAGE@2](drawing-package.md), [UIB.DRAWING-STYLE@2](drawing-style.md), [UIB.DRAWING-GEOMETRY@2](drawing-geometry.md), [UIB.DRAWING-PROMPT-A@2](drawing-prompt-a.md), [UIB.DRAWING-REVIEW@2](drawing-review.md), [UIB.DRAWING-EXAMPLE@1](../reference/drawing-example.md).
- Source mapping: TZ 399–416; DRAWING 1–30; DRAWING 301–315; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; HBP-HUMAN-001 presentation rules supersede the earlier exhaustive visible inventory rules; machine truth remains exact.

Вхождение полного руководства заменяется указанным requires closure; ссылка на оригинал в перенесённом тексте — provenance.

<a id="uib-export-content"></a>

## Инженерная документация и ImageGen

Основной человекочитаемый результат — полный инженерный blueprint выбранного интерфейса для документации, согласования с клиентом и фиксации QA. Команда imagegen-prompt локально собирает развёрнутый DrawingBrief, scene/dimensions, план листов, подробный self-contained prompt и разрешённые references. Она не вызывает модель и не требует ключа. Полный контракт, чертёжные правила и большой шаблон заданы в [руководстве инженерной визуализации и ImageGen](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/docs/engineering-blueprint-guide.md), версия 1.2 (`UIB.DRAWING@1.2`).

Режим document — полный вид текущего интерфейса, основной по умолчанию; explain может сохраниться как alias. propose — будущая схема с явно заданными требованиями; detail — увеличенный узел; flow — карта подтверждённых состояний/переходов. compare до/после остаётся дополнительным режимом. Полнота относится к объявленному scope и состоянию: весь viewport, scroll-document и всё приложение не взаимозаменяемы. Для сложного scope формируется комплект общего и детальных листов вместо нечитаемой одной картинки.

Основной визуальный preset — Blue Engineering: ровный синий/голубой фон, белые контуры и надписи, плоский ортогональный вид, иерархия линий, размеры и выноски, оси, ID компонентов, легенда и штамп с ревизией/статусом. Это инженерное описание UI, без перспективы и декоративной фантастики. Точные размеры, padding/radius/hit regions показываются только по известному источнику. Полный шаблон включает scope, environment, ведомость компонентов, размерные anchors, unknown, состояния и правила проверки; ссылка на приватный документ сама по себе не заменяет включения этих сведений в prompt.

Предлагаемый вызов основного режима после реализации:

```text
uiblueprint imagegen-prompt --snapshot S42 --purpose document --scope main-window --profile blue-engineering --out ./drawing-package
```

Пакет содержит manifest.json, drawing-brief.md, scene.json, dimensions.json, sheets.json, prompt.txt и разрешённые references. Полный пример промпта и синтетическая схема целого интерфейса находятся в связанном руководстве; короткое пояснение агента не считается инженерным заданием.

ImageGen оформляет пакет по отдельному запросу. source_kind (observed/proposed), validation_status и approval_status независимы. После генерации проверяются численные подписи, состав, ID, геометрические связи, coverage и читаемость. Изображение может стать проверенным приложением к документации, но само не создаёт продуктовых требований и не доказывает CAD-масштаб. Для метрически точного вывода остаётся опциональный deterministic SVG/PDF/overlay. В spec-first работе отдельно сохраняются утверждённый проект, runtime-наблюдение и QA-результат; baseline не перезаписывается текущим кадром без принятого решения.

# UI Blueprint руководство инженерной визуализации и ImageGen

Источник: [Page](https://chatgpt.com/space/page_16e9c03ad3848191ad03b89fefa0b9ee). Копия от 6 октября 2026 года.

**Версия 1.1 · 6 октября 2026 года.** Контракт `UIB.DRAWING@1.2`, Authority: Active, Stability: Evolving. Пользователь подтвердил руководство вместе с ТЗ в чате подготовки разработки 6 октября 2026 года. Это нормативный контракт визуализации; подтверждение документа не означает готовности реализации или приёмки сгенерированных изображений. Руководство для подготовки инженерного изображения UI Blueprint, его машинного описания и подробного задания ImageGen. Основной результат — полная схема выбранного интерфейса для документации, проектного обсуждения и фиксации QA. Режим сравнения до и после дополнительный.

Связано с [общим ТЗ UI Blueprint](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/docs/ui-blueprint-spec.md). Это собственный профиль UI-документации, а не заявление о соответствии ISO, ЕСКД или AutoCAD. ImageGen остаётся основным каналом визуального оформления; его результат проверяется по исходным данным.

## Что фиксирует инженерный blueprint

Blueprint представляет интерфейс как систему поверхностей, компонентов, размеров, отношений, текстовых областей и доступных действий. Его можно показать клиенту как проект, сохранить как описание существующего экрана или использовать рядом со спецификацией и QA evidence.

Документ отвечает на вопросы: где проходят границы интерфейса и его областей; как элементы вложены и выровнены; какие размеры и интервалы заданы или измерены; что происходит при изменении доступного места; какие элементы интерактивны; какие состояния представлены; откуда получены сведения.

Полная карта означает полное покрытие объявленного scope и выбранного состояния. Она не означает автоматически полное приложение: скрытые меню, страницы, прокрутка и неоткрытые окна должны иметь отдельные наблюдения или явно обозначенные проектные виды. Viewport, полный scroll-document и карта маршрутов — разные виды документа.

## Режимы и статус

| Режим | Назначение | Источник |
| --- | --- | --- |
| document | Полный инженерный вид текущего интерфейса; основной режим | Реальное Observation/Snapshot с environment и coverage |
| propose | Проект будущего интерфейса для обсуждения с клиентом | Явные requirements и ProposedLayout; не выдаётся за runtime |
| detail | Увеличенный узел: кнопка, форма, popup, строка, отступы | Связанный фрагмент document/propose |
| compare | Изменения между двумя состояниями или ревизиями | Сопоставимые наблюдения либо два явно обозначенных проекта |
| flow | Карта состояний и наблюдённых переходов | Transition/evidence; предполагаемые связи отделены |

Статусы source, validation и approval независимы. source_kind различает observed и proposed. validation_status различает unverified и checked. approval_status различает draft, accepted и superseded. Красиво оформленная картинка не становится accepted автоматически. В штампе указываются фактические статусы, а не обобщённое «готово».

Для сравнения proposed → observed показывать разные основания обеих сторон: требование и измерение. Совпадение внешнего вида не доказывает поведение. Проект без runtime допустим и полезен, если целевые величины явно помечены как предложенные.

## Команды и критерии готовности

Предлагаемый CLI после реализации; это не существующие команды:

```text
uiblueprint imagegen-prompt --snapshot S42 --purpose document --scope main-window --profile blue-engineering --out ./drawing-package
uiblueprint imagegen-prompt --brief proposed-layout.json --purpose propose --profile blue-engineering --out ./proposal-package
uiblueprint imagegen-prompt --before S41 --after S42 --purpose compare --out ./comparison-package
```

document — основной режим; explain может сохраняться как совместимый alias. По умолчанию выводится полный план документа в заявленном scope, а не минимальный до/после prompt. detail и flow — дополнительные профили; при слишком большом scope формируется sheets plan.

Критерии P6: полный DrawingBrief и self-contained prompt; ссылка/ревизия этого руководства; согласованные scene/dimensions; один полный observed пример и один proposed пример; детали перегруженного вида; честные unknown; раздельные source/validation/approval; без модели работает подготовка пакета; compare не обязательный первый сценарий.

Генерация готовой картинки не становится условием model-free работы CLI. Если она выполнена в рамках отдельного запроса, её проверка обязательна перед маркировкой checked. Полноценный CAD-export, dashboard и хранение атласа остаются отдельными этапами.

HBP-HUMAN-001: requires the [human presentation rules](../../engineering-blueprint-guide.md#hbp-human-001--клиентская-подача-2026-10-10), DRAWING@1.2. Full original machine records stay exact; visible inventories/long decimals are superseded.
