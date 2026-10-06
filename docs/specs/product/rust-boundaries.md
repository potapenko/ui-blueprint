# Границы Rust реализации

- Node type: leaf; domain: `uib.rust-boundaries`.
- Contract: `UIB.RUST-BOUNDARIES@1`; stable clause: `UIB.RUST-BOUNDARIES.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: engineering/toolchain proposals, Rust owners.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.BOUNDARIES@1](boundaries.md).
- Source mapping: TZ 784–811; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

Rust source дополнительно следует [RUST.md](../../../RUST.md); toolchain/Cargo — [DEV.RUST@1](../development/rust.md). Эти маршруты активируются по задаче, а не для переноса данного текста.

<a id="uib-rust-boundaries-content"></a>

## Дополнительные указания для Rust реализации

Аналитика, кэш, diff, schema и CLI остаются на Rust. Будущие transport/service/MCP crates используют публичный engine API. Web frontend может использовать JS/TS как UI-клиент: форматирование, выбор и отображение; канонические измерения, проверки и состояние операций остаются у engine.

Организация расширяется по необходимости: service-host для lifecycle/transport, mcp-adapter для MCP, отдельный dashboard frontend. Не создавать пустые пакеты ради roadmap и не включать их зависимости в обязательный CLI build до соответствующего этапа.

Перед implementation в отдельном проекте зафиксировать toolchain/MSRV, edition, workspace resolver, supported target matrix, features и правила Cargo.lock по фактическим API/зависимостям. [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html), [features/resolver](https://doc.rust-lang.org/cargo/reference/resolver.html).

- Отделить чистую геометрию и преобразования от IO, OS handles, clocks и transport.

- Использовать типизированные ошибки и явные generation/space/unit типы.

- Изолировать FFI/unsafe в узких платформенных модулях с описанными инвариантами; raw handles не входят в сериализуемый граф.

- Не удерживать глобальный lock во время AX/capture вызова; блокирующий API не становится отменяемым из-за async wrapper.

- Ограничивать retained revisions, очереди, память и concurrent work; перегрузка имеет явный результат.

- Проверять выбранные features на соответствующих target; web-only пользователь не обязан устанавливать Apple/Android SDK.

- Не вводить nightly, тяжёлый framework, dynamic ABI или persistence engine без доказанной необходимости.

- Engine проверять детерминированными tests, adapter — настоящими bounded fixtures, transport — lifecycle/reconnect contract tests.

- Документировать cargo fmt --check, cargo clippy и cargo test для выбранных пакетов/targets; не требовать сборки всех платформ на одном host.

Старт реализации — P0 и обязательное чтение upstream. Первый видимый результат — настоящий scoped capture и измерение на разрешённом fixture с понятным выводом CLI. Серверный framework и dashboard не заменяют этот результат.
