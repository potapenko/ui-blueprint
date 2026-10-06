# Ui.Vision как поведенческий reference

- Node type: leaf; domain: `uib.executor-sources`.
- Contract: `UIB.EXECUTOR-SOURCES@1`; stable clause: `UIB.EXECUTOR-SOURCES.CONTENT`.
- Authority: Active / Stability: Evolving; norms + historical evidence; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: A01; action runner, delivery/verification, unsafe fallback.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.REUSE@1](reuse.md).
- Source mapping: TZ 1009–1030; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-executor-sources-content"></a>

### Ui Vision как референс исполнителя сценариев

[Ui.Vision](https://ui.vision/rpa) сочетает браузерные макросы, desktop-ввод, OCR/поиск изображений и интеграцию с агентами. Исходник браузерной части — [A9T9/RPA](https://github.com/A9T9/RPA), проверенная ревизия `17b6302f1617b838efe2b26d3ef19c2face81350`. Desktop-возможности используют дополнительные XModules; модельные функции и browser extension не следует считать одним неделимым обязательным backend.

Для чтения кода:

- [command.ts](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/src/common/command.ts): каталог команд и разделение browser/desktop scope. Полезен для сравнения нашего capability-контракта и понятного пользователю action vocabulary.

- [command_runner.js](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/src/ext/content_script/command_runner.js): адресация элементов и frame, ожидания, проверки, check/uncheck, type/select. Читать прежде всего отличие доставки действия от verify/assert.

- [player.ts](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/src/services/player/player.ts) и [timer.ts](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/src/services/player/monitor/timer.ts): состояния выполнения, причины окончания и учёт времени. В нашем исполнителе сохранить отдельные completed steps и неизвестный outcome, а deadlines строить на согласованном monotonic clock.

- [desktop_vision.ts](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/src/ext/common/desktop_vision.ts): выбор региона изображения и платформенные механизмы. Полезен как пример UX «укажи этот участок», но запуск системного picker является отдельным действием, не скрытой частью read-only observe.

Что взять: понятный жизненный цикл короткого сценария, различие проверки и действия, ограниченный поиск по области и объяснимую диагностику сбоя. Возможный пользовательский кейс — «заполни эту форму и покажи, какой шаг не прошёл», с наблюдением промежуточных состояний.

Что не переносить автоматически: fallback на вторичные locators, если первичная цель исчезла. В просмотренном runner такой fallback существует; UI Blueprint сначала обязан доказать актуальную единственную цель. Также не переносить произвольное выполнение script, автоматический retry неизвестной mutation и обязательную зависимость от всей Ui.Vision/XModules системы.

Лицензионное основание требуется проверить до переноса: [LICENSE.txt](https://github.com/A9T9/RPA/blob/17b6302f1617b838efe2b26d3ef19c2face81350/LICENSE.txt) объявляет AGPLv3 либо коммерческое лицензирование, тогда как package.json содержит ISC. Нельзя опираться на одно поле package.json или считать этот код permissive по умолчанию. До разрешения условий — поведенческий референс для собственной реализации, без копирования исходников. У отдельных компонентов могут быть дополнительные ограничения.

Контрольный кейс для нашего исполнителя: два одинаково названных поля, исчезновение выбранного элемента между observe и act, отклонённая валидация, отмена после dispatch. Ожидаем ограниченную ошибку/unknown, а не ввод в другую похожую цель.
