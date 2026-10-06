# Browser collectors и sessions

- Node type: leaf; domain: `uib.web-sources`.
- Contract: `UIB.WEB-SOURCES@1`; stable clause: `UIB.WEB-SOURCES.CONTENT`.
- Authority: Active / Stability: Evolving; norms + historical evidence; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: R01; DOM/AX, refs/actionability, browser sessions.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.REUSE@1](reuse.md).
- Source mapping: TZ 874–921; TZ 964–967; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-web-sources-content"></a>

### agent browser

[Проект](https://github.com/vercel-labs/agent-browser), commit `6d3e22c673a44271d0c213c2fef722e0aeba627d`; [Apache-2.0](https://github.com/vercel-labs/agent-browser/blob/6d3e22c673a44271d0c213c2fef722e0aeba627d/LICENSE).

Начать с [cli/src/native/snapshot.rs](https://github.com/vercel-labs/agent-browser/blob/6d3e22c673a44271d0c213c2fef722e0aeba627d/cli/src/native/snapshot.rs), [cli/src/native/element.rs](https://github.com/vercel-labs/agent-browser/blob/6d3e22c673a44271d0c213c2fef722e0aeba627d/cli/src/native/element.rs), [cli/src/native/diff.rs](https://github.com/vercel-labs/agent-browser/blob/6d3e22c673a44271d0c213c2fef722e0aeba627d/cli/src/native/diff.rs) и [cli/src/native/daemon.rs](https://github.com/vercel-labs/agent-browser/blob/6d3e22c673a44271d0c213c2fef722e0aeba627d/cli/src/native/daemon.rs). В snapshot.rs есть проверки инвалидирования refs при смене iframe document; это полезная точка входа в тесты.

Что взять: уже существующий Rust-путь CDP/AX snapshot, compact/depth/interactive выборки, RefMap и document/session identity, долгоживущую сессию и экономный вывод. Это приоритетный референс для собственного Rust-кода; переводить уже Rust-код «с TypeScript» не требуется.

Ограничение: diff_snapshots здесь вычисляет строковый Myers diff через similar, а diff_screenshot — pixel diff. Они не являются нашим типизированным atomic graph delta и не подтверждают runtime-геометрию. Chromium/CDP-детали и возможные selector fallbacks не становятся общими платформенными инвариантами. Проверить своей fixture: повторный snapshot, iframe remount, скрытая перекрывающая цель и изменившийся ref.

### browser use

[Проект](https://github.com/browser-use/browser-use), commit `7be96ed8bafa8dfe1eef228b59cf5c884b8b2431`; [MIT](https://github.com/browser-use/browser-use/blob/7be96ed8bafa8dfe1eef228b59cf5c884b8b2431/LICENSE).

Начать с [browser_use/dom/enhanced_snapshot.py](https://github.com/browser-use/browser-use/blob/7be96ed8bafa8dfe1eef228b59cf5c884b8b2431/browser_use/dom/enhanced_snapshot.py) и [browser_use/browser/watchdogs/dom_watchdog.py](https://github.com/browser-use/browser-use/blob/7be96ed8bafa8dfe1eef228b59cf5c884b8b2431/browser_use/browser/watchdogs/dom_watchdog.py); продолжить модулями пакета browser_use/dom, которые они вызывают.

Что взять: разбор компактного DOMSnapshot, индексов строк, bounds/computed styles/paint order, предварительный lookup по backend node ID и обработку чувствительных inputs. Это пример объединения источников до передачи агенту.

Ограничение: внутренние эвристики видимости, DPR-преобразования и agent-oriented serialization требуют отдельной проверки. Python-агент, облачные функции и модельная orchestration не переносятся в обязательное ядро. Fixture: вложенный frame, transform/zoom, sensitive input и частично отсутствующий layout node.

### Playwright и его MCP backend

[Playwright](https://github.com/microsoft/playwright), commit `d0fd0f22ffad53804c0a326a692ee6b8e70ada3d`; [Apache-2.0](https://github.com/microsoft/playwright/blob/d0fd0f22ffad53804c0a326a692ee6b8e70ada3d/LICENSE), также [NOTICE](https://github.com/microsoft/playwright/blob/d0fd0f22ffad53804c0a326a692ee6b8e70ada3d/NOTICE).

Основные входы: [packages/injected/src/ariaSnapshot.ts](https://github.com/microsoft/playwright/blob/d0fd0f22ffad53804c0a326a692ee6b8e70ada3d/packages/injected/src/ariaSnapshot.ts), [packages/injected/src/roleUtils.ts](https://github.com/microsoft/playwright/blob/d0fd0f22ffad53804c0a326a692ee6b8e70ada3d/packages/injected/src/roleUtils.ts), [packages/playwright-core/src/server/dom.ts](https://github.com/microsoft/playwright/blob/d0fd0f22ffad53804c0a326a692ee6b8e70ada3d/packages/playwright-core/src/server/dom.ts). Здесь изучать роли/accessible names, refs, обход и actionability. DOM-геометрия и ARIA-структура — разные источники.

[Playwright MCP](https://github.com/microsoft/playwright-mcp), wrapper commit `f183dad4a52965583e3cc1d59b88cdc279e2e57d`, [Apache-2.0](https://github.com/microsoft/playwright-mcp/blob/f183dad4a52965583e3cc1d59b88cdc279e2e57d/LICENSE). Его [src/README.md](https://github.com/microsoft/playwright-mcp/blob/f183dad4a52965583e3cc1d59b88cdc279e2e57d/src/README.md) прямо указывает, что реализация находится в монорепозитории Playwright. Читать [packages/playwright-core/src/tools/backend/snapshot.ts](https://github.com/microsoft/playwright/blob/d0fd0f22ffad53804c0a326a692ee6b8e70ada3d/packages/playwright-core/src/tools/backend/snapshot.ts), [packages/playwright-core/src/tools/backend/form.ts](https://github.com/microsoft/playwright/blob/d0fd0f22ffad53804c0a326a692ee6b8e70ada3d/packages/playwright-core/src/tools/backend/form.ts) и каталог packages/playwright-core/src/tools/mcp.

Что взять: адресованный snapshot с depth/boxes, формы с typed fields и secret references, единый backend для tools. Не считать успешное fill завершённой бизнес-операцией и не переносить model-facing descriptions как наш контракт безопасности. Fixture: два одинаковых label, typed checkbox/select, секретное значение, stale target. Playwright разрешён как низкоуровневый сбор/ввод; собственные graph/geometry/diff остаются нашим кодом.

### Playwright CLI

[Проект](https://github.com/microsoft/playwright-cli), commit `b85c7a736bb473bf55b584e54a09ffa698d6d871`; [Apache-2.0](https://github.com/microsoft/playwright-cli/blob/b85c7a736bb473bf55b584e54a09ffa698d6d871/LICENSE).

Читать [playwright-cli.js](https://github.com/microsoft/playwright-cli/blob/b85c7a736bb473bf55b584e54a09ffa698d6d871/playwright-cli.js), [skills/playwright-cli/references/session-management.md](https://github.com/microsoft/playwright-cli/blob/b85c7a736bb473bf55b584e54a09ffa698d6d871/skills/playwright-cli/references/session-management.md), затем реальную программу [packages/playwright-core/src/tools/cli-client/program.ts](https://github.com/microsoft/playwright/blob/d0fd0f22ffad53804c0a326a692ee6b8e70ada3d/packages/playwright-core/src/tools/cli-client/program.ts) и каталог tools/cli-daemon в Playwright. Wrapper импортирует реализацию из playwright-core: отдельный repo не содержит весь движок.

Что взять: короткие команды, сохранение browser session и разделение client/daemon, работу с большим результатом вне модельного контекста. Не обещать универсальную экономию токенов без своего замера и не делать сторонний CLI обязательной прослойкой каждой операции. Fixture: серия запросов к одной сессии, завершение процесса клиента без потери target и явный cleanup.

### Stagehand

[Проект](https://github.com/browserbase/stagehand), commit `82ef425ee115dbd63bbff930f9f171fe892d231a`; [MIT](https://github.com/browserbase/stagehand/blob/82ef425ee115dbd63bbff930f9f171fe892d231a/LICENSE).

Проверенные входы: [packages/extension/understudy/a11y/snapshot/a11yTree.ts](https://github.com/browserbase/stagehand/blob/82ef425ee115dbd63bbff930f9f171fe892d231a/packages/extension/understudy/a11y/snapshot/a11yTree.ts), [packages/cli/src/lib/driver/commands/snapshot-format.ts](https://github.com/browserbase/stagehand/blob/82ef425ee115dbd63bbff930f9f171fe892d231a/packages/cli/src/lib/driver/commands/snapshot-format.ts), [packages/cli/src/lib/driver/commands/snapshot.ts](https://github.com/browserbase/stagehand/blob/82ef425ee115dbd63bbff930f9f171fe892d231a/packages/cli/src/lib/driver/commands/snapshot.ts).

Что взять: frame-scoped AX tree, обогащение ролей сведениями DOM, ограничение depth и фильтрацию с сохранением предков, отделение collector от форматирования ответа. В частности, имя/role AX может скрывать семантику file input.

Не переносить автоматически AI act/extract/observe, облачный сервис или кеширование модельных решений в model-free engine. Его форматирование дерева — не provenance-aware graph delta. Fixture: глубоко вложенная цель с нужным label/ancestor и frame-local identity.

### Браузерные и Android платформенные источники

[CDP DOMSnapshot](https://github.com/ChromeDevTools/devtools-protocol/blob/master/pdl/domains/DOMSnapshot.pdl): читать captureSnapshot, DocumentSnapshot, NodeTreeSnapshot, LayoutTreeSnapshot и индексы строк; [CSSOM View](https://www.w3.org/TR/cssom-view-1/) — getBoundingClientRect/getClientRects и координаты; [WAI-ARIA](https://www.w3.org/TR/wai-aria-1.2/) — роли и состояния. Это первичные протоколы/спецификации, не готовый общий движок. Версия CDP привязывается к выбранному браузеру в support matrix; master-документ не является проверкой совместимости Safari.
