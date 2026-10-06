# Исторический starter и рабочие имена

- Node type: leaf; domain: `uib.start-name`.
- Contract: `UIB.START-NAME@1`; stable clause: `UIB.START-NAME.CONTENT`.
- Authority: Active / Stability: Evolving; norms + historical evidence; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: проверка исходного старта/имён; не повседневная реализация.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: none.
- Source mapping: TZ 661–686; TZ 1074–1080; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

Starter — сохранённый пример, не дополнительный gate чтения целых исходников. Текущий запуск и approved scope задаёт [реестр задач](../../plans/ui-blueprint/task-registry.md); продуктовые маршруты — [корень](../README.md).

<a id="uib-start-name-content"></a>

## Стартовый промпт для будущего разработчика

Ниже текст для передачи после создания целевого проекта и разрешения реализации. Он не является автоматическим запуском.

```text
Реализуй UI Blueprint в подготовленном отдельном проекте по этому ТЗ.
Сначала прочитай его целиком, instructions целевого проекта и каталог аналогов.
Начни с P0: прочитай релевантный чужой код и тесты, закрепи revision/license,
прими D01–D07 по доказательствам и предложи конкретную реализацию первого
сквозного результата. Не подменяй этот шаг общим планом исследования.

Сохрани собственную Rust-аналитику, отдельные плагины, model-free основу,
ограниченный scope, честные unknown и равноправные Mac/Web acceptance.
Не подключай Jev или готовые blueprint-продукты как обязательный runtime.
Уже существующий Rust-код оцени на пригодность, не переписывай ради языка.

Реализуй общий контракт, затем оба живых vertical slices, включая узкий
native measured probe, freshness/delta, безопасное исполнение и ImageGen prompt.
Соблюдай правила целевого проекта о согласовании и checkpoint.
Проверяй текущие платформенные API и не выдумывай отсутствующие возможности.
Закрывай требования по фактическому пользовательскому результату, фиксируй
adversarial случаи и измеряй cold/warm latency отдельно.
Не выдавай benchmark, snapshot, debug SDK или Simulator за более сильное
доказательство. Не меняй PlayPhrase.me ради удобства инструмента.
```

## Рабочее имя и GitHub

Пользователь сохраняет рабочее название **UI Blueprint**. Рекомендация для репозитория — `ui-blueprint`; имя существующего в ТЗ CLI — `uiblueprint`. Такое разделение удобно: читаемый адрес репозитория и одна короткая команда. Предложение для GitHub description: `Runtime UI geometry, semantics, and actions for AI agents.`

Суффиксы `-ai`, `-rust` и `-rs` пока не нужны: модель необязательна, а проект включает платформенные мосты. Вариант `ui-blueprint-runtime` уместен, только если нужно дополнительно отличить инструмент от генератора макетов. Имя в аккаунте/организации пользователя этой работой не резервировалось; репозиторий не создавался.

GitHub поддерживает переименование репозитория с перенаправлением обычных ссылок и Git-операций. Имена опубликованных пакетов/CLI, GitHub Pages и ссылки на reusable actions требуют отдельной проверки при ребрендинге. [Правила GitHub](https://docs.github.com/en/repositories/creating-and-managing-repositories/renaming-a-repository).
