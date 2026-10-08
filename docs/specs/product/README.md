# Продуктовые контракты

- Node type: branch; contract: `UIB.PRODUCT-ROUTES@1`; clause: `UIB.PRODUCT-ROUTES.ROUTE`.
- Authority: Active / Stability: Evolving; routing only; accepted/released baseline: none.
- Authority source: [registry](../README.md), C00 faithful routing of user-confirmed originals; L01-ANALYSIS-001 adds the delegated analysis route.
- Read when: выбор поведения/данных/экспорта.
- Do not read when: путь уже установлен и актуален.
- Requires: только выбранные ниже листья и их explicit closure.

Исходники и будущие этапы не входят в каждый closure. Экспорт выбирается через
`UIB.EXPORT@1`: он включает весь применимый DrawingBrief/стиль/шаблон/review.
Общие данные начинаются с MODEL/EXCHANGE; операции добавляют ACTIONS/CACHE.

| Контракт | Read when / роль |
| --- | --- |
| [UIB.BOUNDARIES@1](boundaries.md) | выбор архитектуры, platform/plugin/profile/extension boundaries; contract |
| [UIB.MODEL@1](model.md) | schema, нормализация, свойства, роли, provenance; contract |
| [UIB.EXCHANGE@2](exchange.md) | request/session/property/time/error/JSON envelopes and narrow explicit probe-wrapper composition; contract |
| [UIB.NATIVE@2](native.md) | Mac AX, capture, оконное matching, probe; contract |
| [UIB.GEOMETRY@1](geometry.md) | пространства, transforms, измерения, Expectation/check; contract |
| [UIB.PROJECTIONS@1](projections.md) | выбор области, coverage, проекции, many-to-many mapping; contract |
| [UIB.FORMS@1](forms.md) | поля, фокус, selection, IME, draft/applied; contract |
| [UIB.IDENTITY@1](identity.md) | Target/Surface/ref/locator, matching, revalidation; contract |
| [UIB.ACTIONS@1](actions.md) | prepare/resolve/act/verify, input ownership, cancel/teardown; contract |
| [UIB.LIFECYCLE@1](lifecycle.md) | cancel/dispatch, native handles, очереди Target и изоляция; contract |
| [UIB.CACHE@1](cache.md) | cache keys, invalidation, replay, upsert/removal; contract |
| [UIB.PRIVACY@1](privacy.md) | сбор, хранение, ввод секретов, logs/errors/pixels/export; contract |
| [UIB.CLI@14](cli.md) | CLI commands/output; compact component parts, [raw diff@2](cli-diff.md)/[selected-space diff@1](cli-geometry-diff.md)/[graph diff@1](cli-graph-diff.md), preserved JSON/OBSERVE and [CLI-ACTIONS@3](cli-actions.md) |
| [UIB.ANALYSIS@2](analysis.md) | local measurement/check0.2, direct observed input, result-space/evaluation, validation layers and protected0.1 compatibility; types/validation closure unchanged |
| [UIB.ROADMAP@1](roadmap.md) | пакеты реализации, сроки решений, compatibility freeze; contract |
| [UIB.RUST-BOUNDARIES@1](rust-boundaries.md) | engineering/toolchain proposals, Rust owners; contract |
| [UIB.CLI-NEIGHBORS@1](cli-neighbors.md) | bounded explicit recorded relation context; contract |
| [UIB.CLI-EXPORT@1](cli-export.md) | saved Snapshot/ChannelResponse → document package with explicit metadata; contract |
| [UIB.EXPORT@1](export.md) | E01/E02, imagegen-prompt, человеческий экспорт; contract |
| [UIB.DRAWING-STYLE@1](drawing-style.md) | компоновка документа и обозначения; contract |
| [UIB.DRAWING-GEOMETRY@1](drawing-geometry.md) | scene/dimensions, responsive/flow/state views; contract |
| [UIB.DRAWING-PACKAGE@1](drawing-package.md) | export compiler, manifest, ProposedLayout и validation; contract |
| [UIB.DRAWING-PROMPT-A@1](drawing-prompt-a.md) | сборка self-contained prompt; первая последовательная часть; contract |
| [UIB.DRAWING-PROMPT-B@1](drawing-prompt-b.md) | сборка self-contained prompt; вторая последовательная часть; contract |
| [UIB.DRAWING-REVIEW@1](drawing-review.md) | export QA, checked/approval, baseline и retention; contract |

Acceptance выбирается по [карте проверок](../acceptance/README.md), upstream — по
[каталогу](../reference/README.md). Они не добавляются все по факту этой ссылки.

## Пакеты после исследований

| Consumer | Выбор перед explicit dependency closure |
| --- | --- |
| C01/T01 | ROADMAP; RUST-BOUNDARIES; DEV.RUST@1 по trigger; REUSE и принятые source records |
| S01/L01 | MODEL, EXCHANGE, CLI, GOLDEN; schema решения D03; ANALYSIS для local analysis JSON/evaluation |
| G01/G02 | GEOMETRY, PROJECTIONS; NATIVE для probe mapping; platform PILOTS по проверке |
| W01/M01/P01 | соответствующий PILOTS; для Native NATIVE; его requires включает privacy/lifecycle |
| K01–K02/W03/M03 | CACHE, LIFECYCLE, GOLDEN; соответствующий PILOTS для живых событий |
| A01/W02/M02 | ACTIONS и соответствующий PILOTS; EXECUTOR-SOURCES перед новым executor mechanism |
| E01/E02 | EXPORT (полный closure, включая синтетический пример); CLI для CLI wiring |
| V01/I01/Q01/Q02 | COMPLETION; CLI и RUST-BOUNDARIES для поставки; PERFORMANCE для сравнения |

Имена здесь ссылаются на ссылки таблицы выше и [acceptance tree](../acceptance/README.md),
[reference tree](../reference/README.md). Точный пакет закрепляет выбранные revisions,
решения и evidence; эта навигация не назначает write lease или новый scope.
