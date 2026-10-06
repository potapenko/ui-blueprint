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
| Активные чаты/пакеты/ресурсы | S01 five repairs and E02-R1/R2/R3 independently accepted; Core K01 store + Integration L01-result-contract + Native capture repair active; Native owns own-F02 desktop lane, B pixels stopped on -3801; Export/Web retained for affected next work |
| Последний принятый результат продукта | нет |
| Следующий шаг | Принять affected recheck семи repairs либо вернуть точечный repair владельцу; завершить K01 store и Native capture prerequisite. D05 transient/host enforcement, D07 Web adoption/transport, canonical MeasurementResult и RC05 evidence остаются открытыми |
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
| [L01](packets/L01.md) | existing Core chat `01a111a7-9887-7983-9aa0-c08dfa2d46bc` / local | package `c9555f6e9d1e888231f43afdc9b16d332cb08e1b` + membership `5522966935cf878bc68fc12703400dafb15bc0a7`, pushed | check/fmt/Clippy/11binarytests on matching saved inputs; [receipt](receipts/L01.md); source frozen; full measure/conversion JSON awaits canonical result/evidence contract; not complete L01 |
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
| [K01-storage](packets/K01-storage.md) | Core `01a111a7-9887-7983-9aa0-c08dfa2d46bc` / local | sizing extraction `ec0f35c927e9b34fb06313cc0f6c9e4ecaf36f4f`, pushed; D05@2/D05-MEMORY@1 | [receipt](receipts/K01-storage.md); shared input proof saved, source barrier released; running actual Snapshot store; no live/peak/whole-K01 acceptance |
| [M01-capture-repair](packets/M01-capture-repair.md) | Native `01a110ac-2da3-73d1-9bb2-273d4ff99e7a` / local | packet3a98c32; D02@1/D05@2 known F02 failure | running; actual concurrent old-helper failure reproduced, repair/proof underway; exclusive own-F02 desktop lane until cleanup/wait; no real apps/production adapter/permission changes |
| [L01-result-contract](packets/L01-result-contract.md) | Integration `01a110ac-30da-7ab0-bed1-8d7a8e4de45e` / local | `8fdf608f686586892799b9dfaba82db8878a50a4`, pushed | [receipt](receipts/L01-result-contract.md); exact0.2 local analysis records reusing unchanged0.1 source data; root accepts delegated representation decision; source/spec registration still pending |
| [L01-analysis-registration](packets/L01-analysis-registration.md) | Integration same chat | pinned L01 handoff8fdf608; existing norms/ROADMAP authority | ready for dispatch; register D03@2/ANALYSIS@1, no source edits yet; Core engine/lib ownership remains until cache checkpoint |

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
