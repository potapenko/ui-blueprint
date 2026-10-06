# Контракты приёмки

- Node type: branch; contract: `UIB.ACCEPTANCE-ROUTES@1`; clause: `UIB.ACCEPTANCE-ROUTES.ROUTE`.
- Authority: Active / Stability: Evolving; routing only; accepted/released baseline: none.
- Authority source: [registry](../README.md), C00 faithful routing of user-confirmed originals.
- Read when: fixtures, QA и claims готовности.
- Do not read when: задача не меняет и не оценивает соответствующую capability.
- Requires: только выбранные ниже листья и их explicit closure.

Сценарии здесь — ожидаемый результат, фактическое evidence пока отсутствует.
Для общей приёмки выбрать COMPLETION; для одной платформы — её PILOTS.
Положительный gate не закрывается корректным unknown/error. Документационная
проверка C00 не является runtime acceptance или независимым review.

| Контракт | Read when / роль |
| --- | --- |
| [UIB.PILOTS@1](pilots.md) | fixtures и запрет замены positive gate отрицательным; contract |
| [UIB.NATIVE-PILOTS@1](native-pilots.md) | R02 и native fixtures/QA; contract |
| [UIB.WEB-PILOTS@1](web-pilots.md) | R01 и browser fixtures/QA; contract |
| [UIB.COMPLETION@1](completion.md) | интеграция, claims готовности, передача поставки; contract |
| [UIB.PERFORMANCE@1](performance.md) | baseline, latency/quality gates, сравнение; contract |
| [UIB.GOLDEN@1](golden.md) | P1 normalized fixtures, schema validation, full/delta oracle; contract |
