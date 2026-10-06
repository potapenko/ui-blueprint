# Каталоги и сохранённые направления

- Node type: branch; contract: `UIB.REFERENCE-ROUTES@1`; clause: `UIB.REFERENCE-ROUTES.ROUTE`.
- Authority: Active / Stability: Evolving; routing only; accepted/released baseline: none.
- Authority source: [registry](../README.md), C00 faithful routing of user-confirmed originals.
- Read when: выбор upstream mechanism или provenance.
- Do not read when: обычная реализация уже имеет достаточный source record.
- Requires: только выбранные ниже листья и их explicit closure.

[R01/R02/R03](research-routes.md) задают точные замыкания для первых исследований.
Каталог сохраняет исходные revisions/entrypoints/ограничения без нового upstream
исследования. Будущие ссылки не являются разрешением начать их реализацию.
[Обратная карта](source-map.md) — для сверки полноты, не общий preload.

| Контракт | Read when / роль |
| --- | --- |
| [UIB.REUSE@1](reuse.md) | любой source record/borrowing decision; reference |
| [UIB.WEB-SOURCES@1](web-catalog.md) | R01; DOM/AX, refs/actionability, browser sessions; reference |
| [UIB.CORE-SOURCES@1](core-catalog.md) | R03; geometry, rules, typed update; reference |
| [UIB.PROBE-SOURCES@1](probe-catalog.md) | R02/R03; measured anchors и детализация; reference |
| [UIB.NATIVE-SOURCES@1](native-catalog.md) | R02; native identity/capture/notifications; reference |
| [UIB.EXECUTOR-SOURCES@1](executor-catalog.md) | A01; action runner, delivery/verification, unsafe fallback; reference |
| [UIB.EXTENSIONS@1](extensions.md) | только отдельно открытые extension/mobile исследования; future |
| [UIB.OTHER-PLATFORMS@1](other-platforms.md) | только отдельно открытое исследование Windows или iOS; future |
| [UIB.PROFILE-SCENARIOS@1](profile-scenarios.md) | явно разрешённое подключение product profile; не базовые fixtures; reference |
| [UIB.PROVENANCE@1](provenance.md) | сверка происхождения норм или сценариев; reference |
| [UIB.START-NAME@1](start-and-name.md) | проверка исходного старта/имён; не повседневная реализация; reference |
| [UIB.FUTURE-HOST@1](future-host.md) | только отдельно одобренные F1/F2; future |
| [UIB.FUTURE-DASHBOARD@1](future-dashboard.md) | только отдельно одобренный F3 или future client lifecycle; future |
| [UIB.FUTURE-ATLAS@1](future-atlas.md) | только отдельно одобренный atlas experiment; future |
| [UIB.DRAWING-EXAMPLE@1](drawing-example.md) | E01/E02; проверка достаточности scene и арифметики; reference |
