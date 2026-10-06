# F4: атлас и его source catalog

- Node type: leaf; domain: `uib.future-atlas`.
- Contract: `UIB.FUTURE-ATLAS@1`; stable clause: `UIB.FUTURE-ATLAS.CONTENT`.
- Authority: Active / Stability: Evolving; future, вне P0–P7; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: только отдельно одобренный atlas experiment.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.REUSE@1](reuse.md), [UIB.IDENTITY@1](../product/identity.md), [UIB.PRIVACY@1](../product/privacy.md).
- Source mapping: TZ 1031–1073; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-future-atlas-content"></a>

### UI Atlas как референс карты приложения

[UI Atlas](https://github.com/AI-Successors/ui-atlas), проверенная ревизия `8f86b4f51eddbb5d7748ee890bb7d87b99c974af`, — Windows/.NET-инструментарий записи и построения графа UI. Важен для нас путь от исходных наблюдений к структурной карте с сохранением доказательств.

Сначала читать [architecture.md](https://github.com/AI-Successors/ui-atlas/blob/8f86b4f51eddbb5d7748ee890bb7d87b99c974af/docs/architecture.md), [interaction-trace.md](https://github.com/AI-Successors/ui-atlas/blob/8f86b4f51eddbb5d7748ee890bb7d87b99c974af/docs/interaction-trace.md) и [known-limitations.md](https://github.com/AI-Successors/ui-atlas/blob/8f86b4f51eddbb5d7748ee890bb7d87b99c974af/docs/known-limitations.md). Они различают Raw Data Streams, Raw World и Semantic World, наблюдённые переходы и affordances с неизвестным результатом. Это структурная интерпретация, не доказанное понимание бизнес-намерения.

Точки входа в реализацию:

- [GraphContracts.cs](https://github.com/AI-Successors/ui-atlas/blob/8f86b4f51eddbb5d7748ee890bb7d87b99c974af/src/UiAtlas.Core.Contracts/GraphContracts.cs): Application/Window/Surface/State/Control, edges, EvidenceRef и graph metadata. Взять связь каждой сущности с исходным наблюдением.

- [RecordingGraphBuilder.cs](https://github.com/AI-Successors/ui-atlas/blob/8f86b4f51eddbb5d7748ee890bb7d87b99c974af/src/UiAtlas.Core.Build/RecordingGraphBuilder.cs): переход от записей к surface/control/state variants и transition-слоям. Изучить условия слияния и отвергнутые/неполные наблюдения.

- [StableIdentity.cs](https://github.com/AI-Successors/ui-atlas/blob/8f86b4f51eddbb5d7748ee890bb7d87b99c974af/src/UiAtlas.Core.Build/StableIdentity.cs) и [GraphSemantics.cs](https://github.com/AI-Successors/ui-atlas/blob/8f86b4f51eddbb5d7748ee890bb7d87b99c974af/src/UiAtlas.Core.Build/GraphSemantics.cs): детерминированные ключи и сравнение смыслового графа. Переносить с проверкой допущений: архивный structural ID не является актуальным actionable ref, а нормализация строк не всегда допустима для идентификаторов.

- [Consumer sample](https://github.com/AI-Successors/ui-atlas/blob/8f86b4f51eddbb5d7748ee890bb7d87b99c974af/samples/UiAtlas.Core.Consumer/Program.cs): независимый читатель, поиск контролов, selectors/actions, observed target states и число доказательств. Это полезная граница между сбором, картой и агентом-потребителем.

- [UIKG schema](https://github.com/AI-Successors/ui-atlas/blob/8f86b4f51eddbb5d7748ee890bb7d87b99c974af/schemas/uikg-v4.schema.json): пример версионированного interchange, не обязательный формат нашего протокола.

Что взять: раздельное сохранение наблюдений и производных слоёв, lineage, variants, негативные результаты переходов, reader без зависимости от визуального редактора, reproducible graph build и явное покрытие исследования.

Чего не обещать: полную карту приложения после одного прохода, автоматическое восстановление бизнес-логики, актуальность сохранённых selectors после обновления приложения и перенос Windows ownership/DPI правил на Mac. README описывает attended recording; подробные документы также описывают активные discovery-шаги hover/focus/menu. Поэтому нельзя считать весь upstream гарантированно пассивным наблюдателем: перед переносом конкретного механизма нужно проверить его реальные side effects.

Корневой [LICENSE](https://github.com/AI-Successors/ui-atlas/blob/8f86b4f51eddbb5d7748ee890bb7d87b99c974af/LICENSE) объявляет MIT; [THIRD-PARTY-NOTICES](https://github.com/AI-Successors/ui-atlas/blob/8f86b4f51eddbb5d7748ee890bb7d87b99c974af/THIRD-PARTY-NOTICES.md) и [provenance ledger](https://github.com/AI-Successors/ui-atlas/blob/8f86b4f51eddbb5d7748ee890bb7d87b99c974af/provenance/files.csv) проверяются для выбранных заимствований. Это не основание переносить весь .NET/WPF/SQLite стек в наше Rust-ядро.

## Атлас приложения как будущее развитие

Идея пользователя — взять готовое приложение и построить его атлас — сохраняется как отдельное направление после базового инструмента. Рабочая архитектурная граница: UI Blueprint поставляет свежие Observation/Snapshot/Transition и измерения; будущий atlas-потребитель хранит историю, группирует подтверждённые состояния, показывает маршруты и покрытие. Это может стать отдельным продуктом или пакетом, а не обязательной подсистемой первого MVP.

Предлагаемые сценарии:

| Сценарий | Польза | Что необходимо показать честно |
| --- | --- | --- |
| Обследование незнакомого приложения | Оператор проходит разрешённые экраны; агент получает карту окон, меню, форм и переходов | Исследованную область, версию приложения, условия и непосещённые ветви |
| Поиск функции | Найти, где наблюдались Export, Settings или нужное поле, и показать известный маршрут | Сохранённый маршрут — подсказка; перед действием нужны свежая цель и права |
| Сравнение версий | Увидеть появившиеся/исчезнувшие controls, новые состояния и изменения маршрутов | Эвристическое сопоставление и неполное покрытие не становятся доказанным удалением функции |
| Инвентаризация для переноса функциональности | Составить перечень видимых возможностей и сценариев для будущего нового приложения | Наблюдение поведения не раскрывает все бизнес-правила, серверные контракты и права |
| Документация и обучение | Генерировать описание пользовательского пути и ImageGen-промпт карты | Фактические состояния отделены от предполагаемых и предлагаемых |
| Планирование QA | Найти непроверенные переходы и повторить известные сценарии на новой сборке | Успешный старый маршрут не является новой приёмкой |

Для первого будущего atlas-эксперимента достаточно одного разрешённого приложения и ограниченного маршрута, например главное окно → настройки → вложенное меню → возврат. Принятие такого эксперимента: для каждого ребра есть исходное/результирующее состояние и evidence; неизвестная destination остаётся unknown; неуспешные действия сохраняются отдельно; rebuild из тех же очищенных записей воспроизводим.

Автономный crawler всего приложения, автоматический перенос функций в код и отдельный редактор атласа не добавляются в P0–P7. Расширение не может молча включить долговременную запись: нужна отдельная пользовательская опция retention и разрешённый scope. Исторические изображения, значения полей и structural IDs не получают права на автоматическое выполнение действий.
