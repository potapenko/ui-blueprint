# F3: dashboard и запуск

- Node type: leaf; domain: `uib.future-dashboard`.
- Contract: `UIB.FUTURE-DASHBOARD@1`; stable clause: `UIB.FUTURE-DASHBOARD.CONTENT`.
- Authority: Active / Stability: Evolving; future, вне P0–P7; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: только отдельно одобренный F3 или future client lifecycle.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.FUTURE-HOST@1](future-host.md).
- Source mapping: TZ 737–783; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-future-dashboard-content"></a>

### Dashboard для наблюдения работы

Первый dashboard ориентирован на просмотр. Он показывает только события, которые интегрированные агенты и engine реально публикуют, а не произвольную деятельность других программ или скрытые рассуждения модели.

- Сессии и задачи: агент/клиент, Target, текущая операция, прогресс, ожидание, ошибка и последнее подтверждённое состояние.

- Timeline: observe → prepare → delivery → verification с понятным описанием и evidence.

- UI inspector: дерево/граф, выбранный компонент, layout/AX/hit/visible geometry, freshness/coverage, разрешённый crop.

- Blueprint workspace: scope/state, document или propose, DrawingBrief, план листов, подробный prompt, проверка и экспорт.

- Документация и QA: source/validation/approval, ревизии, принятые reference и расхождения без автоматической смены baseline.

- Диагностика по раскрытию: latency, cache/resync, capabilities и bounded redacted errors.

Основной экран отвечает «что сейчас делается и что получилось». Сырые IDs и логи не занимают весь интерфейс. Live означает поток наблюдаемых событий с timestamp и статусом соединения; при потере связи показываются stale/disconnected.

«Подготовить blueprint» локально собирает пакет. «Сгенерировать изображение» — отдельная будущая интеграция с выбранным поставщиком, правами и оплатой, не условие работы панели. Предпросмотр и скачивание не означают approval. Управление stop/cancel и действиями использует общие policies.

### Предлагаемый способ запуска

Это проект интерфейса, не существующие команды. Имена и параметры фиксируются в CLI schema.

```text
# Первый самостоятельный этап
uiblueprint capabilities --plugin macos --target TARGET
uiblueprint observe --target TARGET --scope main-window --mode geometry
uiblueprint imagegen-prompt --snapshot S42 --purpose document --profile blue-engineering --out ./drawing-package

# Будущий локальный host
uiblueprint serve --listen 127.0.0.1:PORT --session review
uiblueprint observe --session review --target TARGET --scope form

# Будущий MCP adapter к той же сессии
uiblueprint mcp --transport stdio --session review

# Будущий dashboard как клиент host
uiblueprint dashboard --session review
```

PORT и TARGET — placeholders. Auth material не передаётся открытым argv. Dashboard открывает разрешённый локальный адрес уже работающего host либо явно сообщает, что нужно его запустить; не расширяет scope и не запускает скрытые приложения.

Документация поставки содержит поддержанные ОС/архитектуры, установку бинарного файла, права, первый fixture, запуск/подключение/остановку сессии, размещение временных данных и очистку run-owned ресурсов. «Нет AX permission», «backend не установлен» и «target отсутствует» имеют разные инструкции восстановления.

Acceptance F1–F3: клиенты видят одни revisions; reconnect не повторяет действие; read-only viewer не меняет UI; медленный клиент не блокирует engine; secrets отсутствуют в timeline; потеря потока вызывает resync; stop не завершает чужие процессы; standalone CLI работает без сервера.
