# Windows/iOS источники

- Node type: leaf; domain: `uib.other-platforms`.
- Contract: `UIB.OTHER-PLATFORMS@1`; stable clause: `UIB.OTHER-PLATFORMS.CONTENT`.
- Authority: Active / Stability: Evolving; future, вне P0–P7; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: только отдельно открытое исследование Windows или iOS.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.REUSE@1](reuse.md).
- Source mapping: TZ 978–979; TZ 982–984; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-other-platforms-content"></a>

| Источник и код для чтения | Что полезно для UI Blueprint | Граница переноса |
| --- | --- | --- |

| [Terminator](https://github.com/mediar-ai/terminator): [element.rs](https://github.com/mediar-ai/terminator/blob/73a381c0c1c33eda55f2c0ecb1d918bf5ec7561a/crates/terminator/src/element.rs), [platform tree search](https://github.com/mediar-ai/terminator/blob/73a381c0c1c33eda55f2c0ecb1d918bf5ec7561a/crates/terminator/src/platforms/tree_search.rs) | Уже существующий Rust-код: сериализуемое описание элемента отдельно от объекта, выполняющего действия; платформенные границы и поиск. | На проверенной ревизии это Windows-ориентированный инструмент, не готовый Mac backend. Не переносить Windows assumptions или весь большой element owner в общий engine. Корневой LICENSE — MIT. Windows-плагин не добавляется в текущий объём автоматически. |
| [Appium XCUITest Driver](https://github.com/appium/appium-xcuitest-driver): [source.ts](https://github.com/appium/appium-xcuitest-driver/blob/b908435bed80f9720e14034e244326db20c35d6a/lib/commands/source.ts), [element.ts](https://github.com/appium/appium-xcuitest-driver/blob/b908435bed80f9720e14034e244326db20c35d6a/lib/commands/element.ts); [WebDriverAgent cache](https://github.com/appium/WebDriverAgent/blob/d17782422d55ff1e5e0ceb74eb1fd509cc0c35b6/WebDriverAgentLib/Routing/FBElementCache.m) и [resolve](https://github.com/appium/WebDriverAgent/blob/d17782422d55ff1e5e0ceb74eb1fd509cc0c35b6/WebDriverAgentLib/Categories/XCUIElement%2BFBResolve.m) | Источники iOS hierarchy, protocol/action boundary, element cache и явная обработка stale element. Полезно для будущего iOS-плагина и ошибок общего протокола. | Это test/developer-среда, не свободный доступ из любого iOS-приложения к соседним. У WebDriverAgent есть private headers: их не переносить в production SDK без отдельного решения. Driver root — Apache-2.0; WDA LICENSE содержит BSD-текст и дополнительные материалы, условия выбранных файлов проверить. |
