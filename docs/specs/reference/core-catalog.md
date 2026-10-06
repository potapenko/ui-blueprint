# Relations и согласованные updates

- Node type: leaf; domain: `uib.core-sources`.
- Contract: `UIB.CORE-SOURCES@1`; stable clause: `UIB.CORE-SOURCES.CONTENT`.
- Authority: Active / Stability: Evolving; norms + historical evidence; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: R03; geometry, rules, typed update.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.REUSE@1](reuse.md).
- Source mapping: TZ 922–939; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-core-sources-content"></a>

### Galen и Galen Extras

[Galen](https://github.com/galenframework/galen), commit `6c7dc1f11d097e6aa49c45d6a77ee688741657a4`; [Apache-2.0](https://github.com/galenframework/galen/blob/6c7dc1f11d097e6aa49c45d6a77ee688741657a4/LICENSE-2.0.txt). Читать [galen-core/src/main/java/com/galenframework/validation/specs/SpecValidationInside.java](https://github.com/galenframework/galen/blob/6c7dc1f11d097e6aa49c45d6a77ee688741657a4/galen-core/src/main/java/com/galenframework/validation/specs/SpecValidationInside.java), [galen-core/src/main/java/com/galenframework/specs/Range.java](https://github.com/galenframework/galen/blob/6c7dc1f11d097e6aa49c45d6a77ee688741657a4/galen-core/src/main/java/com/galenframework/specs/Range.java), [galen-core/src/main/java/com/galenframework/speclang2/specs/SpecAlignedProcessor.java](https://github.com/galenframework/galen/blob/6c7dc1f11d097e6aa49c45d6a77ee688741657a4/galen-core/src/main/java/com/galenframework/speclang2/specs/SpecAlignedProcessor.java) и [язык правил](https://galenframework.com/docs/reference-galen-spec-language-guide/).

Взять отношения inside/near/aligned и явные диапазоны/допуски. Не переносить Selenium/Java runner или трактовку отсутствия/видимости как универсальную истину: наш результат может быть unknown. Не доказывать paint occlusion одними rect. Fixture: частичное вложение, отрицательное расстояние, пограничный tolerance и единицы.

[Galen Extras](https://github.com/galenframework/galen-extras), commit `600fb9abc6bea91e6228a93929602f5433908448`; [Apache-2.0](https://github.com/galenframework/galen-extras/blob/600fb9abc6bea91e6228a93929602f5433908448/LICENSE-2.0.txt). Читать [galen-extras/galen-extras-rules.js](https://github.com/galenframework/galen-extras/blob/600fb9abc6bea91e6228a93929602f5433908448/galen-extras/galen-extras-rules.js), galen-extras/galen-extras-rules.gspec и examples.gspec из README.

Взять групповые отношения: равные интервалы, выравнивание ряда, таблица колонок. Не вводить его удобные значения допусков как продуктовые требования UI Blueprint. Fixture: ряд с одним выбивающимся промежутком и условная desktop/mobile раскладка.

### AccessKit

[Проект](https://github.com/AccessKit/accesskit), commit `466e24e252103f4d82bd6fbab98141b551e46e0c`. [Cargo.toml](https://github.com/AccessKit/accesskit/blob/466e24e252103f4d82bd6fbab98141b551e46e0c/Cargo.toml) объявляет MIT OR Apache-2.0; присутствуют LICENSE-MIT, LICENSE-APACHE и отдельный LICENSE.chromium. Перед переносом проверять условия конкретного файла.

Читать [accesskit/src/lib.rs](https://github.com/AccessKit/accesskit/blob/466e24e252103f4d82bd6fbab98141b551e46e0c/accesskit/src/lib.rs) — NodeId, Role, TreeUpdate, ActionRequest — и [accesskit_consumer/src/tree.rs](https://github.com/AccessKit/accesskit/blob/466e24e252103f4d82bd6fbab98141b551e46e0c/accesskit_consumer/src/tree.rs) — update, subtree/parent/focus и change processing. Папки common/src и consumer/src из старых описаний здесь уже не актуальны.

Взять схему семантики, идентичность дерева и атомарное применение согласованного обновления локального состояния. Не принимать AccessKit за внешний scanner и не копировать его wire schema механически: TreeUpdate заменяет полные nodes, а наш projection/fields/provenance требует дополнительных правил. Fixture: удаление поддерева, перенос parent, focus change и неверная base revision.
