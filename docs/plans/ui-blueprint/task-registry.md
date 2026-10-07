# Единый реестр задач

- План: [PLAN.UIB@1](../ui-blueprint-development.md).
- Правила исполнения и восстановления: [ранбук](execution.md); этот реестр хранит
  текущее состояние, а не вторую копию правил. Прямой запрос пользователя требует
  сохранять правила push, чатов/параллельности и архивирования в файлах.
- Режим: `coordinated`, root coordination-only; цель активна в чате `01a11088-e608-7801-bdfb-db5c9383af9d`.
- Одобренный план: `358c757e7eab84a3989d150dbad57924d866601a`; ветка `master`.
- Пользователь 2026-10-06: «Ну да, лучше, наверное, не писать код, только координация. Совсем согласен. Давай, это, начинай цель и делай по плану, по реестру и так далее. В остальном я согласен.»
- Объём: P0–P7, рабочие чаты и follow-up по плану; root не реализует и не проверяет продукт.
- Уточнение пользователя 2026-10-06: «Какой ответ от меня нужен? Уже всё же
  обсудили уже.» Исправления семи findings входят в уже одобренную реализацию;
  root ошибочно распространил read-only review restriction на дальнейшую работу.
  Текущее awaiting_authority снято; [ранбук](execution.md) закрепляет границу,
  [S01 repair](packets/S01-review-repair.md) и [E02 repair](packets/E02-repair.md)
  назначают владельцев. Старые receipts сохраняют историческое ожидание, а не
  действующий запрет. Reviewer сам код не меняет.
- Начальный checkpoint `358c757`; код продукта отсутствует; строки `queued`, если статус ниже не уточнён.

## Очередь

Классы: C=coordination, D=diagnostic, T=tooling, S=shipping_product, V=verification.
Зависимости означают необходимые принятые результаты. До dispatch каждая строка
получает конечный пакет с exact write set, contract revision и acceptance.

| ID / этап | Класс / роль | Зависит от | Конечный результат / непосредственный потребитель |
| --- | --- | --- | --- |
| C00 / P0 | C / Spec | запуск | Разнести текущие требования в короткие маршрутизируемые контракты со stable clause IDs и обратной картой к ТЗ; без новых смыслов; вход всех пакетов |
| R01 / P0 | D / Web | C00 | Код browser collectors/refs/actionability; source record и ограниченный prototype; вход D01/D02/D04 и W01 |
| R02 / P0 | D / Native | C00 | Код AX/capture/identity и реализуемость M05; source record и prototype; вход D01/D02/D04 и M01 |
| R03 / P0 | D / Core | C00 | Семантика update/geometry/projections; source records; вход D03 и G01/K01 |
| F01 / P0 | T / Web | R01 | Контролируемый Web fixture B01–B06 с независимыми expectations; baseline D05/D06 |
| F02 / P0 | T / Native | R02 | Mac fixture M01–M06, измеримый merged control и probe off/on режим; baseline D05/D06 |
| F03 / P0–P7 | T / product advisor | пользовательское уточнение, case inventory, runtime lane | Реальные Mac/iPhone/iPad case/data examples PlayPhrase.me; consumer C01/engine/agent/Q03 |
| C01 / P0 | C / Integration | R01,R02,R03,F01,F02 | Решения D01–D07 по срокам ТЗ, toolchain/MSRV/edition, support matrix и начальные frozen gates; вход P1/P2; root принимает receipt |
| T01 / P1 | T / Integration | C01 | Минимальная Cargo-сборка нужных owners, lockfile, выбранные host/feature проверки; вход первого исполнимого результата |
| S01 / P1 | S / Integration | T01 | Schema/plugin-api candidate, parser/serializer, validator и GOLDEN01 с valid/invalid envelopes; вход всех модулей |
| G01 / P1 | S / Core | S01,R03 | Типизированные пространства, transforms и geometry checks pass/fail/unknown; детерминированные fixtures |
| L01 / P1 | S / Integration | S01,G01 | Минимальный CLI вход в engine: bounded JSON/compact и выходы validator; первый проверяемый путь |
| H01 / P1–P2 | S / Core | accepted local analysis/K01; D05 registrationf9ff423 | Реальный bounded Rust host/worker, allocator/publication/cleanup proof; prerequisite live W01/M01 under adopted D02/D05 |
| W01 / P2 | S / Web | L01,F01 | Живой Web observe/inspect/measure, точные target/surface, channels/coverage, redaction до выхода |
| M01 / P2 | S / Native | L01,F02 | Живой Mac observe/inspect/measure, window matching, AX/capture provenance и redaction |
| P01 / P3 | S / Native | M01 | M05 measured probe и comparison off/on; никакого влияния на geometry/focus/hit/AX |
| G02 / P3 | S / Core | G01,W01,M01,P01 | Interaction/design, many-to-many, scope/neighbors и compare без превращения heuristic match в action ref |
| K01 / P4 | S / Core | S01 | Bounded cache, revisions, atomic delta и replay на GOLDEN01; кандидат до K02 |
| W03 / P4 | S / Web | K01,W01 | Web events/invalidation, bounded resync и subscription teardown через принятый cache API |
| M03 / P4 | S / Native | K01,M01,P01 | Native notifications/invalidation, permission changes, stale handles и subscription teardown |
| K02 / P4 | V / review | W03,M03,G02 | Live invalidation/resync/lifecycle обеих платформ; M06/B05, независимость Target и full/delta oracle |
| A01 / P5 | S / Core | K02 | Общий prepare/resolve/act/verify, cancel/timeout/unknown outcome; deterministic fake delivery tests |
| W02 / P5 | S / Web | A01,W01 | Реальный Web input B02 и stale/ambiguous/unknown negatives |
| M02 / P5 | S / Native | A01,M01,P01 | Реальный native input M02, popup lifecycle и attribution по modality |
| E01 / P6 | S / Export | S01,G01 | DrawingBrief validator/compiler, document/propose/detail/flow/compare и sheets plan на fixtures |
| E02 / P6 | V / review | E01,W01,M01,G02 | Observed/proposed примеры, поля/units/unknown, безопасный export; обновить проверку после final candidate |
| V01 / P5–P6 | V / review | W02,M02,K02,E02 | Независимая проверка identity/permissions/privacy/concurrency, canary по всем каналам; repairs владельцам |
| I01 / P6 | S / Integration | V01,P01,G02 | CLI integration, installable выбранный набор, recovery/docs, зависимости и NOTICE, никаких обязательных моделей |
| Q01 / P7 | V / review | I01 | Полные M01–M06, B01–B06 и два E2E на фиксированной сборке; совместимость schema после обоих пилотов |
| Q02 / P7 | V / performance | Q01,C01 | Cold/warm quality/latency и overhead против baseline; не подгонять gates |
| Q03 / P7 | V / real cases | I01,F03 | Проверить данные UI Blueprint и задачи агента на реальных кейсах; source/build/scope/expected/observed и ограничения |
| C02 / P7 | C / root | Q01,Q02,Q03 | Проверить всю матрицу, записать limitations и accepted baseline, сохранить checkpoint; цель complete только по результату |

Для защищённых рисков независимая V-проверка обязательна до accepted каждого
готового среза; final V01 объединяет актуальные доказательства. Q01 повторно
проверяет затронутые интеграцией I01 критерии на итоговом commit. Менять интерфейс для
нескольких владельцев — отдельная согласованная интеграция, не тихая правка.
M01/M02 здесь task IDs; одноимённые сценарии ТЗ явно писать как `pilot M01/M02`.

## Чтение внешнего кода

| Пакет | Релевантный маршрут каталога ТЗ | Ограничение результата |
| --- | --- | --- |
| R01 | agent-browser; browser-use; Playwright injected/backend и CLI sessions; Stagehand; CDP/CSSOM/ARIA | Точные revision/files/tests и переносимые механизмы; не новый агентный stack |
| R02 | AXorcist; Peekaboo; Apple AX/ScreenCaptureKit; Compose Blueprint/probe и Preview limitations | Проверить native identity, каналы и измерительный probe; не внешний CLI как зависимость |
| R03 | AccessKit node/update/consumer; Galen/Extras relations; Compose anchors | Typed delta не строковый diff; partial/unknown не pass |
| A01 | Ui.Vision command runner/player как поведенческий reference; refs/delivery из R01/R02 | Не копировать код при неразрешённых условиях; не переносить unsafe fallback/retry |

Каждый источник читается по механизму с нужными types/tests, не целиком ради
галочки. Ledger: URL+revision → entry files → вывод → adopt/reimplement/reject →
LICENSE/NOTICE выбранного материала → собственная fixture → потребитель.
Выводы старого каталога — вход, а не новое подтверждение runtime или лицензий.
Не изучать сейчас Jev, ML weights, мобильные runtime и UI Atlas без потребителя P0–P7.

## Состояние исполнения после запуска

У каждой строки появятся: `owner_chat_id`, `packet_path`, `epoch`, `write_lease`,
`base_commit`, `status`, `receipt`, `review`, `candidate_revision`, `residual`.
Переходы: proposed → queued → running → review → accepted; rejected → repair.
Ожидания: waiting_resource / waiting_evidence / awaiting_authority с точной причиной,
владельцем и следующим событием; resource recheck через три минуты.
`done` от исполнителя не равен accepted. Историю не раздувать логами.

| Текущие аренды / действия | Значение |
| --- | --- |
| Активные чаты/пакеты/ресурсы | H01 Core guard/publication and Native process implementation active against frozen API3abd8e5. CDP2f9ea77 accepted and own temp cleaned; Web receives actual scoped DOM/AX collector source packet, live calls still gated on H01/D05. Core owns shared host lib/Cargo; Git index free. Desktop released; B pixels remain stopped |
| Последний принятый результат продукта | scoped local analysis migration: factual measure/query JSON0.2, full check0.2 and recomputation with protected0.1 compatibility; no live adapter/full release acceptance |
| Следующий шаг | H01 реализует registered D02/D05 host boundary и конечные proof cases; Web завершает transport для independent review. При concrete API handoff выделить disjoint platform-process work Native owner. Live adapters/D06/RC05 и полная поставка остаются открытыми |
| Restart | проверить цель и разрешение; восстановить владельцев, epochs, ожидания и следующий готовый пакет |

## Активное исполнение

| Packet | Owner chat / host | Basis / scope | Status / receipt |
| --- | --- | --- | --- |
| [C00](packets/C00.md) | `01a1109a-5d6e-7792-b926-e767c7f63642` / local | candidate `92d2bf89ea9acab091fd166eef4432408e353744`; `UIB.ROUTING@1` | accepted; [receipt](receipts/C00.md) |
| [C00-review](packets/C00-review.md) | `01a110a5-8af7-7ef1-8fb6-fd989e78e666` / local | initial `f8d63cb`; final `9bedeccbf4c5a92206d24d0794b299e54a2b22e1` | accepted; [verdict](receipts/C00-review.md) |
| [R01](packets/R01.md) | `01a110ac-2aae-7841-9c8b-12ff38c52d9d` / local | `71ea44235cddf63381d1fea4acd121ad642e7ddf` | accepted diagnostic; [receipt](receipts/R01.md); bounded acquisition → W01 |
| [R02](packets/R02.md) | `01a110ac-2da3-73d1-9bb2-273d4ff99e7a` / local | `19cd1359ff4c239695f5da21f90d101d878c067d` | accepted diagnostic; [receipt](receipts/R02.md); full M05 proof → F02/P01 |
| [R03](packets/R03.md) | `01a110ac-30da-7ab0-bed1-8d7a8e4de45e` / local | `26a19faed84b468dd21f77bf2c78516cd917d223` | accepted diagnostic; [receipt](receipts/R03.md); proposals for C01 only |
| [F01](packets/F01.md) | Web chat R01 | `a8368076cdf0d4a917e7c3c5448ca6b6d919cf64` | accepted tooling/baseline; [receipt](receipts/F01.md); no product acceptance |
| [F02](packets/F02.md) | Native chat R02 | `9a88b12b5855bac54bf04ba7b64d233df719ddec` | accepted supporting fixture; current proof limits in [receipt](receipts/F02.md); M/P01 gates open |
| F01-doc | Web chat R01 | `585bcd90ad455110bcc2432f8f36f58e3c8d7ecf` | accepted docs correction; no new measurements |
| [F03a](packets/F03a.md) | `01a1102f-791c-7e91-bec3-1877ea004d51` / local | `5d1e0a53670fb8bfbd0c3982db5e164f4bf9bb6d` | accepted reference data; [receipt](receipts/F03a.md); agent/collector proof pending |
| [C01](packets/C01.md) | Core chat R03, Integration role | `42d2e6b059c5822476a39a59ac89a8a5c223db3a` | accepted engineering decisions; [receipt](receipts/C01.md); later proof gates remain open |
| [T01](packets/T01.md) | Core chat R03, Integration role | `28a08d34e66687cb5668608fe7ddd3429aa9807b` | accepted bounded Rust owner; [receipt](receipts/T01.md); S01 full review follows |
| [F03b](packets/F03b.md) | product advisor | `8f93735ed14e03380a76094c1370c32efd42552f` | accepted reference with explicit playback/statistics/geometry limits; [receipt](receipts/F03b.md) |
| [S01](packets/S01.md) | Core chat R03, Integration role | Stage A `9d2df15`; D02 support `73d772e97efcf550ea4a4d3e8480b56509ebc548`, pushed | [receipt](receipts/S01.md); D02 and platform samples saved, D05 retained policy@2; five shared repairs2828cd8 saved and under independent recheck; full live enforcement/compatibility freeze still open |
| S01-user-review | collaboration `/root/checkpoint_review` | `eb0edbf..1d12859` + changes saved in `61022a9` | completed static review; four findings, not accepted; [result](receipts/S01-user-review.md); no fixes authorized by review alone |
| [S01-oracles](packets/S01-oracles.md) | `01a11126-55ad-7d03-bd05-30e9feff0818` / local | `d539a0ec2bac243375256652fb75da7065b27350` | accepted oracle input; [receipt](receipts/S01-oracles.md); code/contract review still required |
| [F03c](packets/F03c.md) | product advisor | `492ac0075625ff7e9988294fc23d2bca6e0146f1`, pushed; F03c-build@eda0d77 | reference pack saved (8 PNG/12 scoped files), [receipt](receipts/F03c.md); RC05 long metadata/translation and trailing-selector reachability waiting_evidence, control bounds unknown; cleanup complete, lanes released; no collector/Q03 acceptance |
| [F03c-build](packets/F03c-build.md) | `01a11145-9a6a-7913-86d5-d9f0d483d961` / local | `eda0d7753f85f9e5c03b1d90613ea69586fd8016` | accepted build artifacts; [receipt](receipts/F03c-build.md); runtime still unverified |
| Mac-test-advice | `01a1102f-791c-7e91-bec3-1877ea004d51` / local | direct user request; F03c residual and D02/M01/Q03 | completed read-only; [handoff](receipts/platform-test-advice.md); implications sent to Native owner; reusable advisor retained |
| Web-test-advice | `01a1102f-e21d-7251-9597-c29a1c66d088` / local | direct user request; existing Web coauthor context | completed read-only; [handoff](receipts/platform-test-advice.md); F01/B03 reuse sent to Web owner; reusable advisor retained |
| [S01-Web](packets/S01-web-proof.md) | `01a110ac-2aae-7841-9c8b-12ff38c52d9d` / local | live proof `91475d3565683fa455a21008bd65dfdd90ee125f` pushed; F01/Stage A/support basis in [receipt](receipts/S01-web-proof.md) | saved verified candidate:3liveB03/6labelled negatives/8retained equalities, actual Ticket-before-collection; runtime cleaned; no S01/P1/Native acceptance claim |
| [S01-Native](packets/S01-native-proof.md) | `01a110ac-2da3-73d1-9bb2-273d4ff99e7a` / local | proof `88ac6062253092139c43ed9a14de9fe4b95a78ae` pushed; F02/Stage A/support basis in [receipt](receipts/S01-native-proof.md) | live AX/capture plus labelled timeout/cancel/detach verified; retained equality and owned cleanup, desktop released; one AX node, not largest graph; no M01/P01/full-S01 acceptance |
| [G01](packets/G01.md) | `01a111a7-9887-7983-9aa0-c08dfa2d46bc` / local | package `b1475c8476bdb43880fe56162f4bb76d26958a83` + membership `84a87a66bdb206a740301d97166b957b0eed2277`, pushed | saved controlled-data candidate; preliminary check/fmt/Clippy/14 tests, saved shared hashes match checked inputs; [receipt](receipts/G01.md); multi-hop/conditions/finding consumer residual and S01/four-review gates open |
| [L01](packets/L01.md) | existing Core chat `01a111a7-9887-7983-9aa0-c08dfa2d46bc` / local | initial c9555f6/5522966, now analysis candidatec30aa20 | factual query/measure JSON0.2 and explicit converted/conditional check0.2 implemented with legacy0.1 mode; [new receipt](receipts/L01-analysis-engine-cli.md). Independent migration acceptance pending; live/other CLI commands remain later work |
| [P1-review](packets/P1-review.md) | collaboration `/root/p1_candidate_review` | original `61022a9..8069710`; recheck repairs2828cd8 at9ca645a | original review completed with DAG-depth P2; same reviewer now covers coherent five-fix shared wave, author receipt withheld until initial recheck observations; no whole-P1 acceptance |
| [D05-platform-samples](packets/D05-platform-samples.md) | existing Web/Native owners | Webfcf48be / Native51c7809 saved | completed finite measurement inputs; actual32/76-node partial samples, no fabricated configured maximum or full D05 acceptance |
| D05-Web sample | existing Web owner | `fcf48be57c31d3e3cfa48f38fa1b16d1d6af9357`, pushed | [receipt](receipts/D05-web-samples.md):32 nodes/17 relations/160 properties/96unknown,78241 exact bytes; predeclared128KiB diagnostic budget, graphdepth1; input handed to Integration, no total memory/latency acceptance |
| D05-Native sample | existing Native owner | `51c780959907dc85f6e2c5998850c25964ce7296`, pushed | [receipt](receipts/D05-native-samples.md):76nodes/75edges, incoming337500B vs retained338001B, partial; separate75-node canary check passed, difference causeunknown; no pixels, cleanup/lane release; not160/depth9 coverage |
| D05 working allocation diagnostic | existing Integration owner | `2a5dfefdab219f4e76b8631854e39a897d209ec1`, push verified against origin/master | [receipt](receipts/S01.md); seven scoped paths saved, index released; parser/framing/session evidence only, not process RSS or production policy. K01 ownership/accounting handoff is a proposal for the next finite packet |
| [K01-replay](packets/K01-replay.md) | existing Core chat `01a111a7-9887-7983-9aa0-c08dfa2d46bc` | `e4ecee76f43662a3b6d29b4712b904a2877222d7`, pushed | checked pure replay candidate saved; [receipt](receipts/K01-replay.md),11tests/check/fmt/Clippy; no storage/eviction/D05 or protected-validator acceptance |
| [E01](packets/E01.md) | `01a11286-a187-7720-a452-41b6ea7b228b` / local | package `00b70cc41bb022c27b92f8598e1090430bb0c79a` + membership `dbdccf637c85d135f38faf9a4cab9c89e729a0a0`, pushed | saved candidate; [receipt](receipts/E01.md), check/fmt/Clippy/12 tests; Integration confirms saved shared hash 769ab201…a3e79c8 matches checked input. CLI wiring, G02 attribution and E02 acceptance remain open |
| [K01-storage-design](packets/K01-storage-design.md) | Core `01a111a7-9887-7983-9aa0-c08dfa2d46bc` / local | `0d40af78d2ae5f14b8fe0343a4a1c3007d40d3ca`, pushed | completed finite design; [receipt](receipts/K01-storage-design.md); transferred to Integration, no source implementation before pinned D05 policy |
| [D05-policy](packets/D05-policy.md) | Integration `01a110ac-30da-7ab0-bed1-8d7a8e4de45e` / local | `df998647bc73cd7d5d9cb5e1444a7b58f1dcbc71`, pushed; D05@2/D05-MEMORY@1, spec registry5 | retained engineering policy accepted by root under ROADMAP; [receipt](receipts/D05-policy.md). Actual storage and transient/host enforcement remain open; no subprocess/D02 change |
| [E01-cli](packets/E01-cli.md) | Export `01a11286-a187-7720-a452-41b6ea7b228b` / local | package9cd6666994f3314875403c143a07e82e9620a082 + lockfe0653752b85443e52db690400de6d161fdd9142, pushed | saved candidate; [receipt](receipts/E01-cli.md):20 distinct binary tests/check/fmt/Clippy; saved shared hash1018d7cb…4c12c1 matched, Git lease released; local brief→package only, live Snapshot-ID path and E02 remain open |
| [E02-candidate-review](packets/E02-candidate-review.md) | collaboration `/root/e02_candidate_review` | packet4e04161; artifactfe06537, export00b70cc and CLI delta afterdbdccf6 | completed two-stage review, reject; [receipt](receipts/E02-candidate-review.md): E02-R1 bottom-left vertical anchors and E02-R2 f64 exact equality; authority wait removed by current user clarification, same reviewer retained for recheck |
| [S01-review-repair](packets/S01-review-repair.md) | Integration `01a110ac-30da-7ab0-bed1-8d7a8e4de45e` / local | `2828cd8eb91382235bd327e68f708c3cfbad316f`, pushed | accepted five-fix scope by [independent recheck](receipts/S01-recheck.md) at9ca645a; check/fmt/Clippy/101tests tied to independently matched211inputhash; broader P1/live/JSON gates unchanged |
| [E02-repair](packets/E02-repair.md) | Export `01a11286-a187-7720-a452-41b6ea7b228b` / local | source `bd5270ae22b30c9730cc3fe0622b02e0cec72ca4`, proof receipt94ecbb4, both pushed | [independent recheck](receipts/E02-recheck.md) accepts R1/R2/R3; pinned Git211inputs/check/fmt/Clippy/19export+3CLI proof; only task-temp archive cleaned; no wholeE02/P6/P7 acceptance |
| [W01-transport](packets/W01-transport.md) | Web `01a110ac-2aae-7841-9c8b-12ff38c52d9d` / local | `e5da7d65247a24920c706f17ece8d15330527bf1`, pushed | finite source evidence accepted; [receipt](receipts/W01-transport.md), proposed tungstenite0.30.0 handshake-only with numeric-loopback ws and explicit guards; dependency adoption/runtime not performed |
| [K01-storage](packets/K01-storage.md) | Core `01a111a7-9887-7983-9aa0-c08dfa2d46bc` / local | sizingec0f35c + storef85f06f + proof9b12eba, pushed | retained Snapshot scope independently accepted; [receipt](receipts/K01-storage.md),40engine tests/check/fmt/Clippy, exactE0502/positive-control proof and saved156-inputhash; no fullK01/live/peak claim |
| [M01-capture-repair](packets/M01-capture-repair.md) | Native `01a110ac-2da3-73d1-9bb2-273d4ff99e7a` / local | `848ec6abe63653976b00c2abe1d686109efa865d`, pushed | [receipt](receipts/M01-capture-repair.md): known failure reproduced, bounded callbacks/capture serialization/AX preservation; A capture success, B-3801 permission-required;20saved records passed validator9ca645a offline; resources/lane released; review pending, no fullM01 |
| [L01-result-contract](packets/L01-result-contract.md) | Integration `01a110ac-30da-7ab0-bed1-8d7a8e4de45e` / local | `8fdf608f686586892799b9dfaba82db8878a50a4`, pushed | [receipt](receipts/L01-result-contract.md); exact0.2 local analysis records reusing unchanged0.1 source data; root accepts delegated representation decision; source/spec registration still pending |
| [L01-analysis-registration](packets/L01-analysis-registration.md) | Integration same chat | `7a61a65aefe22dfed9b9e041260c006912dcfabe`, pushed | [receipt](receipts/L01-analysis-registration.md), faithful technical registration accepted by root: D03@2, ANALYSIS/TYPES/VALIDATION@1, registry6; source work not yet claimed |
| [K01-store-review](packets/K01-store-review.md) | collaboration `/root/k01_store_review` | packet4a707e9, artifactf85f06f, proof9b12eba | [review](receipts/K01-store-review.md): accept after exact borrower diagnostic/control proof; no scoped findings; broader cache/live/peak obligations unchanged |
| [S01-analysis-schema](packets/S01-analysis-schema.md) | Integration same chat | API67d7e49 + remaining `b68b5b718d0899f1bfd2d6ec78011c1ebcf49479`, pushed | [receipt](receipts/S01-analysis.md):32 schema tests,48 new vectors,126 legacy cases/core parity/check/fmt/Clippy, production fidelity proof; saved214-inputhash72b5cd20…74e56f6 matches. Shared consumer barrier and independent acceptance pending |
| [M01-capture-review](packets/M01-capture-review.md) | collaboration `/root/m01_capture_review` | artifact848ec6, bounded own-fixture scope | [review](receipts/M01-capture-review.md): accept_with_residual support repair; no source findings. Positive final pixels/B permission/current-host live proof remain open; intermediate source and sixth test provenance limitations explicit |
| [D05-decode-bound](packets/D05-decode-bound.md) | Web `01a110ac-2aae-7841-9c8b-12ff38c52d9d` / local | `07b9715d94cee960288cdd9bd49cedd29d299bb0`, pushed | [receipt](receipts/D05-decode-bound.md): finite source audit accepted; local bounds/general-JSON admission needs identified, no end-to-end upper reserve. Typed-prefix/validation/float workspace still requires qualification/enforcement; own temp cleaned, no live claim |
| [L01-analysis-engine-cli](packets/L01-analysis-engine-cli.md) | Core `01a111a7-9887-7983-9aa0-c08dfa2d46bc` / local | `c30aa20091f7fc166b6ec0af63a39f74ceeecff7`, pushed | [receipt](receipts/L01-analysis-engine-cli.md): workspace check/fmt/Clippy,150tests+borrower doctest; saved281-inputhash6fe25292…47f96e matched; exact15paths saved, source frozen for independent review |
| [E01-analysis-adaptation](packets/E01-analysis-adaptation.md) | Export `01a11286-a187-7720-a452-41b6ea7b228b` / local | `264a838d440e687aa1eca988aeb7cd54ccc18083`, pushed | [receipt](receipts/E01-analysis.md): factual query caller replaces placeholder Expectation; saved packages unchanged,21 distinct tests/check/fmt/Clippy on recorded working provider; final saved migration proof pending |
| ANALYSIS-FLOAT-001 | Integration same chat | saved in b68b5b7; D07@3, registry7, same serde_json1.0.151 | default production204/2054 bit failures corrected to0/2054, production22-case proof passes; same17-package versions/unchanged lock, manifest1df267dd…cc821b. No epsilon or wire promotion; independent migration review pending |
| [L01-analysis-review](packets/L01-analysis-review.md) | collaboration `/root/analysis_integration_review` | packet259f6ff; base8bb5c02 → candidatec30aa20 | completed two-stage reject; [receipt](receipts/L01-analysis-review.md), R1 array decoding and R2 redacted oracle/manifest coverage; same reviewer retained; independent281-inputhash match |
| [L01-analysis-repair](packets/L01-analysis-repair.md) | Core sole decoder/fixture/test owner | testcffd8d3 + repair372aebb pushed; saved input4106e04 | [independent recheck](receipts/L01-analysis-recheck.md) accepts R1/R2 and scoped migration.49 affected tests/59manifest cases; reviewer reproduced exact278-inputhashbc410094…6c8e724 from saved inputs. No remaining scoped gaps; live/Web/D05/P1/P6/P7 unchanged |
| [D05-runtime-decision](packets/D05-runtime-decision.md) | Integration same chat | db629fc0e342ff567f5560344dffa4104d8fb8ef pushed | [receipt](receipts/D05-runtime-decision.md): concrete reusable guarded worker, parent completion pool and root Grant ownership selected as proposal; exact2docs saved/checked, index released. No source/runtime/normative acceptance; finite design review before registration |
| [D05-runtime-review](packets/D05-runtime-review.md) | collaboration `/root/d05_runtime_review` | proposed designdb629fc; D02@1/D05@2/D05-MEMORY@1 preservation | accepted design-registration scope; [receipt](receipts/D05-runtime-review.md), two-stage review/actual K01 source, no findings. Runtime/allocation/cleanup/D06 gates still open |
| [D05-runtime-registration](packets/D05-runtime-registration.md) | existing Integration owner | f9ff423a48952bc90078cc65682652b089fe4a8d pushed; registry9 | faithful D02@2/D05@3/D05-MEMORY@2/WORK@1 registration accepted by root after fullleaf/metadata/header review; exact8paths saved/checked, index released; D07@4 and all runtime/D06 gates preserved |
| [H01-host](packets/H01-host.md) | Core, excluding assigned Native process files | foundation3abd8e5f9c054c12e910d4c8defc2f8a64a8b268 pushed; D07@5 | [receipt](receipts/H01-host.md):12paths saved,4fixed-owner tests/check/fmt/Clippy; saved hash a1f6e56a22956d4a65d655078c2788045fb7561a2187710a9666ec373196f459 matched. Full supervisor/worker/guard/cleanup incomplete; Core continues, process_api frozen for Native |
| [H01-process](packets/H01-process.md) | existing Native owner | foundation/API3abd8e5, D07@5 exact libc0.2.190 | source/tests/peer authored; [handoff](receipts/H01-process.md), formatting only. waiting_resource: Core module/example hooks and stable shared inputs. Core confirmed active preparation at2026-10-07 01:50:05 UTC; recheck by01:53:05 UTC or earlier handoff/completed packet. Resume Native scoped compile/Clippy/owned-process tests after grant; no runtime success yet |
| [H01-process-dependency](packets/H01-process-dependency.md) | existing Integration owner | 7b1489f5a23955978fdcba78c3d5131417768805 pushed | D07@5/registry10 faithful exact libc0.2.190/default-features=false adoption registered and accepted by root;4docs checked/saved, index released. Core adoption explicitly unblocked; unsafe/runtime proof still open |
| [W01-cdp-session](packets/W01-cdp-session.md) | existing Web source/member owner | 2f9ea77aeec6465e6d6d3a7d109d48840f1f8287 pushed; inputs3abd8e5 | [independent review](receipts/W01-cdp-review.md): accept_with_residual,19 author tests/check/fmt/Clippy and exact fingerprint reconciled. No source repair; own temp cleanup next. Semantic collector/live/host/D06 remain open |
| [W01-cdp-review](packets/W01-cdp-review.md) | collaboration `/root/web_cdp_review` | artifact2f9ea77 vs3abd8e5 | [review](receipts/W01-cdp-review.md): accept_with_residual/no findings; fresh source-first then receipt reconciliation, exact saved fingerprint and hashes matched; no reviewer execution or H01/live claim |
| [W01-collector](packets/W01-collector.md) | existing Web owner | accepted CDP2f9ea77/transport4106e04/schema-analysis; source evidence R01/S01-Web | ready actual scoped collection/script/method decoding/canonical normalization plus bounded offline proof. No browser/UI/pixel/live calls before H01/D05 runtime packet; callback/channel API handoff to Core required |
| [W01-transport-review](packets/W01-transport-review.md) | collaboration `/root/web_transport_review` | source4106e04; finaldocsb8b8721 | [review](receipts/W01-transport-review.md): accept_with_residual/no findings, two-stage source/receipt reconciliation; exact fingerprint91f06b85…51e070. IPv6/blocked-write execution and future CDP/live/H01 proof remain explicit; no reviewer execution |
| [W01-transport-implementation](packets/W01-transport-implementation.md) | existing Web owner | source4106e04/finaldocsb8b8721 pushed; D07@4 | scoped accepted_with_residual through [review](receipts/W01-transport-review.md);17final tests/check/fmt/Clippy tied to saved fingerprint.2doc finalcheckpoint/index released; exact task-temp cleanup next. No CDP/live/D05/full W01 claim |

Current Web manifest handoff (saved in4106e04; transport itself remains WIP):
Cargo.toml SHA256 `a87adec8d6b011a88b1ccbb40191eeae0dc2964b52bc50e01a1d9b2a073a36c2`;
Cargo.lock `a3f321614b4551712fa44ee402f697f159dc43d0a613fb85e1e8ecae371d6173`.
Root independently matched these bytes; analysis reviewer verified saved equality. Web reports coherent package check,
tungstenite0.30.0 handshake-only, log0.4.29 no features, existing float_roundtrip
preserved. Core may check only affected schema/engine/CLI and pin hashes before/
after; new Web runtime/tests remain separately unverified. The saved checkpoint
closes the analysis278-input identity gap, not transport acceptance.

Saved integration barrier: at `9ca645a`, all211 checked inputs are committed and
match `db7792faeea43667960657a53560c90fd47aa4f947cf1b62daec896dc5d35afe`.
Input-set definition and101-test evidence are in S01-review-repair receipt; Export
verified saved HEAD equality after the three scoped checkpoints. Native changes
are outside that input set. No unchanged suites repeated. Review acceptance is
separate; Core's subsequent cache code has not inherited this test claim.

Initial research dispatch: `7c48392`; последующее состояние — в commit этого реестра.
T01 принят как ограниченный Rust owner; collector/full P1/runtime acceptance ещё отсутствуют.
Research packet версии проверены reviewer в `9bedecc`; P0 hypotheses остаются
предложениями для C01. Независимый review не подтверждает runtime прототипов заранее.
R03 подтвердил cleanup task-temp; durable verification.json сохранён для C01/P7.

## Уточнение пользователя и продуктовый контекст

2026-10-06 пользователь отверг monitoring/fixed 5 Hz: явный запрос → данные
интерфейса → ответ для отображения. Root добавил лишний эксперимент; cadence,
saturation и rAF proposals больше не используются как продуктовые gates.
Прошлые измерения не переписываются; per-request baseline и fixtures сохраняются.
Пользователь поручил продуктовые вопросы задавать чату «Спроектировать UI Blueprint»:
`01a1102f-791c-7e91-bec3-1877ea004d51` / local. Это advisory, без записи в другие проекты.
Первая [консультация](receipts/product-context.md) получена: основной путь on-demand;
широкая правка ТЗ не нужна. F02 получил коррекцию; cadence-направление закрыто.
Дополнение пользователя: «Можешь тестировать на нормальном приложении ... Точнее
не тестировать, а агента тестировать»; поручено внести реальные примеры в план.
Это разрешает F03/Q03 через названного advisor в PlayPhrase.me, включая Mac и
iPhone/iPad данные, при соблюдении конкретных project/runtime routes. Запрет
реальных приложений из исходного goal уступает этому более позднему узкому разрешению.
Inventory принят; F03a собрал RC01/RC02, F03b собирает RC03 по отдельным grants.
Код реального приложения и другие проекты не открыты для произвольных изменений.
Цель не остановлена; мобильные примеры следуют отдельным конечным пакетам.

## Advisor routing and chat lifecycle

Latest direct user instruction adds «Research website UI blueprint» as the Web
product/test advisor beside «Спроектировать UI Blueprint» for Mac. Both may be
asked finite questions within their respective source context; advice is separated
from verified runtime evidence and cannot silently change the Active local specs.
The user explicitly requests parallel ready work, status verification and archival
of completed chats that are no longer needed. Root checks actual thread handles;
idle with unfinished assigned work triggers a bounded continuation, not assumed
background progress. Shared code/Git/desktop ownership remains explicit.

Archived after authoritative terminal-status checks and accepted saved receipts:
C00 author `01a1109a-5d6e-7792-b926-e767c7f63642`, C00 reviewer
`01a110a5-8af7-7ef1-8fb6-fd989e78e666`, S01-oracles
`01a11126-55ad-7d03-bd05-30e9feff0818`, F03c-build
`01a11145-9a6a-7913-86d5-d9f0d483d961`. History and artifacts are preserved.
R01/F01 and R02/F02 chats remain for their next dependency-ready platform proof.
