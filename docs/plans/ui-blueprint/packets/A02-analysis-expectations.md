# A02-F — актуальные analysis fixture expectations

Конечная задача по P1/P7 после независимого Q01: на94724df падает
`cargo test --locked --offline -p uiblueprint-engine --test analysis every_manifest_engine_expectation_is_verified_at_its_contract_layer`.
Q01 source/runtime established: unstable-source.json и unstable-conditions.json
содержат consistency=unknown (OG1/OC1), ожидают unstable_state и engine=match.
Текущий принятый engine отличает Unknown от Unstable и вычисляет known8css_px;
результат импорта даёт ResultMismatch. Это mismatch ожиданий, не установленный
product defect и не разрешение вернуть неверную Unknown→Unstable трактовку.

Сразу после необходимого чтения выбери минимальную правку ожиданий/test integration,
выполни её, проверь затронутый analysis contract слой и checkpoint+push. Полный
цикл в одном чате, без агентов/других чатов и root approvals на шаги.
Исторические source documents/данные не переписывать ради pass. Не skip/delete
проверки, не ослаблять recomputation/validation, сохранить independent known8 и
явное Unstable→unknown coverage. Если соответствующие positive tests уже достаточны,
не добавлять зеркальные тесты или framework. Изменять только доказанно устаревшие
expectations, с объяснением schema-valid vs engine-recomputed различия.

## Основание и владение

AGENTS → specs/README → product/ANALYSIS@2/TYPES/VALIDATION@1, GEOMETRY@1 и
explicit MODEL/EXCHANGE@2/IDENTITY/PRIVACY/BOUNDARIES/CLI closure; RUST/DEV.RUST@2.
Норма factual known anchors при partial/unknown consistency и rejected inconsistent
imported result не меняется. Evidence: Q01-integrated-acceptance.md пункт1,
accepted256f2a2/2491dec, E03 baseline reconciliationc6b2357 — метод сохранения
истории, не причина копировать его expected без собственного анализа.

Владение: существующий analysis fixture manifest/expected metadata и
crates/engine/tests/analysis.rs/непосредственно нужные analysis tests; synthetic
current examples в существующих fixtures/analysis, если нужны; соответствующий
README/development schema guidance и собственный receipts/A02-analysis-expectations.md.
Точные paths объявить до правок. Production engine/schema/CLI/host/Native/Web,
public formats, Cargo/spec norms и historical JSON documents защищены.
V02 owns protected-input implementation, Q01 verifier ожидает исправленные данные;
они не пишут названный test/manifest слой. Genuine source/authority conflict вернуть
точно, не подгонять ожидаемое значение под текущий вывод.

## Завершение

Показать исходный reproducible failure и точную классификацию/разрешение, затем
проходящий analysis manifest/affected suite с отсутствием skip и сохранностью
historical artifacts. Более широкие untouched suites/runtime не запускать.
Текущая master, общий fcntl.flock /tmp/ui-blueprint-master-git.lock на exact-path
commit+push, без grants. Source/build temp из saved94724df + свои изменения,
не V02 WIP. No UI/network/model calls и новые persistent dirs; images не трогать.
Final: changes/authority, tests/results, hashes/preservation, SHA+push и limitations.
