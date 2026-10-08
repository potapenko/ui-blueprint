# Единый реестр задач

- План: [PLAN.UIB@1](../ui-blueprint-development.md).
- Правила исполнения и восстановления: [ранбук](execution.md); этот реестр хранит
  текущее состояние, а не вторую копию правил. Прямой запрос пользователя требует
  сохранять правила push, чатов/параллельности и архивирования в файлах.
- Режим: `coordinated`, root coordination-only; цель активна в чате `01a11088-e608-7801-bdfb-db5c9383af9d` после прямого resume пользователя 2026-10-08.
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
- Пользователь 2026-10-07: «Продолжаю работу, я включаю цель.» Host confirms active; same approved scope and explicit parallel-chat authorization continue, without reapproval.
- Начальный checkpoint `358c757`; код продукта отсутствует; строки `queued`, если статус ниже не уточнён.

## Историческая пауза перед рестартом — 2026-10-08

Пользователь прямо запросил завершить текущие работы перед рестартом Codex,
не запускать новые чаты и затем поставить цель на паузу. Dispatch остановлен:
никаких новых пакетов, review или runtime. Scope P0–P7 не сокращён.

- Последний coordination checkpoint `92411e1749916dcfe73eb9704c133b49eb8172eb`
  успешно отправлен в canonical master; его Git lease освобождена.
- Core `01a111a7-9887-7983-9aa0-c08dfa2d46bc`: coherent WIP exact20
  `138d7bc7896537b82e18d0d2a57f7f83ab7daded` saved/pushed; idle, Git/runtime
  освобождены. Точный остаток — в A01-execution-handoff receipt: различение
  malformed input exit2 и fresh refusal exit4, оставшиеся metadata cases, review
  и actual public CLI proof. Не считать этот checkpoint завершением CLI acceptance.
- Web `01a110ac-2aae-7841-9c8b-12ff38c52d9d`: docs-only handoff
  `f0d241ef4dd6b32a723dab73ae9a8eae3042fc5d` saved/pushed; idle, Git lease
  освобождена, source не менялся. Mapping choices/initializer dependency остаются
  proposals в W02-provider-handoff receipt, а не принятой реализацией.
- Native `01a110ac-2da3-73d1-9bb2-273d4ff99e7a`: exact7 candidate
  `23f22fb74487d11ad6a04ca991837a3e7ff08081` saved/pushed; idle, index lease и
  runtime освобождены. Все 23 image paths сохранены. Source review/live capture
  остаются после рестарта; handoff в M03-popup-attribution receipt.
- Все три retained reviewer завершены; новых review-заданий нет. Desktop/runtime
  lane свободна. После сохранения каждый worker останавливается без auto-next.
- Drain завершён: все три чата idle, worker Git/write/runtime leases освобождены;
  собственные процессы отсутствуют по receipts. Root сохраняет только execution.md
  и этот реестр, затем устанавливает host goal paused. До явного сообщения
  пользователя не возобновлять. После рестарта сначала проверить host state и
  текущую ветку, прочитать эти три restart receipts и применимые spec routes;
  продолжить незавершённые пакеты тех же владельцев, не повторять принятые проверки.

## Возобновление — 2026-10-08

Прямое сообщение пользователя: «Продолжай работу.» Host goal подтверждён active;
временный запрет dispatch снят для прежней coordinated цели P0–P7. Ветка master,
чистый restart checkpoint b67461c; все три сохранённых owner chats были idle.
Новых чатов не создано; сохраняется исходное разрешение на parallel finite work.

- Core продолжает L01-actions-implementation от138d7bc: исправить установленное
  смешение malformed input2/fresh refusal4, закончить affected metadata checks.
  Source-backed proposal принят: internal ActionRefused/private terminal9, narrow
  lib.rs/diagnostic.rs additions; typed pre-Possible ACKed refusal only. Amendment
  в L01 packet; CLI-ACTIONS.EXITS/public schema/flags0..4 неизменны.
- Web продолжает W02-form-read-facts от docs-only f0d241e. Разрешена только
  механическая None initialization в collector/action.rs при расширении private
  DomRead; action behavior защищено. Mapping proposals сверить с полным contract
  и native API evidence до source; неизвестное не превращать в known.
- Native23f22fb accepted scoped source review тем же m01_acquisition_review:
  сначала source observations, затем author receipt и5source hashes matched.
  Prepared Swift23f22fb/Rust138d7bc без WIP. Активирован один actual AX+popup
  capture run с individual identity values,300s/unchanged D05; Native owns desktop
  lane до finally/reap, images retained. [Review](receipts/M01-acquisition-review.md).
- Git lease выдаётся только checkpoint-ready владельцу; desktop/runtime: Native
  current combined popup run, release after finally/reap.
  Следующий шаг: source-first review reconciliation → Native runtime activation;
  Core coherent checkpoint/review; Webc63b07a scoped source accepted;3path form_reads harness authorized on saveda53 after host-file release. Native setup alternate follows Mac advisor without repeating failed shortcut.

Traversal receipt: AGENTS/runbook/registry → spec registry17 → PRODUCT-ROUTES@1
→ CLI@6/CLI-ACTIONS@1, FORMS@1, NATIVE@1; full explicit closure MODEL/EXCHANGE/
IDENTITY/BOUNDARIES/GEOMETRY/PROJECTIONS/ACTIONS/LIFECYCLE/CACHE/PRIVACY@1,
D01@1/D02@2/D03@2/D04@1/D05@4/MEMORY@2/WORK@1/Native acquisition@2/D06@1/
D07@5/EVIDENCE@1, ROADMAP/RUST-BOUNDARIES/REUSE/GOLDEN/PILOTS/WEB-PILOTS/
NATIVE-PILOTS/PERFORMANCE@1, RUST/DEV.RUST@2. QA/operational/Apple/CUA routes read.
Selected contracts fully restored; no revision drift after pause. Current packets
and three restart receipts are evidence/ownership handoffs, not product authority.
Excluded: export/analysis serialization changes, mobile/future phases, real-app
changes and broad performance/QA wave. Restore existing intended behavior;
private mapping/encoding proposals remain proposals until source reconciliation.
Root writes only this registry, execution runbook and W02 packet scope amendment.

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
| Активные чаты/пакеты/ресурсы | Core исправляет concrete explicit-anchor aggregate coverage overconstraint (engine + mirrored schema guard). Web/Native geometry chains saved/pushed, runtime lanes released. Native next ordinary-window AX binding proposal ready; no new input work |
| Последний принятый результат продукта | G03 ordinary CLI geometry работает на Weba7dfcfd и Nativeea90baf: реальные bounds, размеры/gaps и recorded diff без ручного Snapshot extraction. Ограничения пространства/partial/clipping указаны; actual geometry examples delivered. Full P0–P7 не завершён |
| Следующий шаг | Сохранить P5 WIP; довести основной Web/Mac read-only geometry scenario до полезного agent-facing результата, используя существующий engine/collectors/probe и реальные component cases. Full P0–P7 scope сохраняется |
| Restart | проверить цель и разрешение; восстановить владельцев, epochs, ожидания и следующий готовый пакет |

## Активное исполнение

| Packet | Owner chat / host | Basis / scope | Status / receipt |
| --- | --- | --- | --- |
| [A02-single-step-forms](packets/A02-single-step-forms.md) | retained Core owner | existing ACTIONS/FORMS + source handoff; source3ebfafe accepted | candidate9bd5809 saved/pushed and scoped source review accepted;12kernel+5Web tests attributed,4hashes matched. Core continues Focus lane and host3-document Expectation composition; legacySetChecked2-document path preserved |
| [A01-execution-handoff](packets/A01-execution-handoff.md) | Core01a111a7-9887-7983-9aa0-c08dfa2d46bc / local | c3241ed diagnostic saved/pushed; ACTIONS/GOLDEN01; follow-on source grant | single-step API/internal Web edge ffe1166 saved/pushed,7 focused author tests/check/Clippy; source08a271f…0bccc. Scoped independent kernel review accepted through retained h01_producer_review; [receipt](receipts/A01-action-review.md), exact3 hashes match. Real provider/host/live composition still unaccepted |
| [L01-actions-contract](packets/L01-actions-contract.md) | retained Core owner | CLI@5 CONTENT/ACTIONS + approved P5/P6; actual source handoff | registrationc4bc255 saved/pushed, root read full leaf and delta; CLI-ACTIONS@1/CLI@6/registry16. Direct observed response input and chosen exact private flags registered before code. Actual public CLI implementation separately active |
| [A01-guarded-composition](packets/A01-guarded-composition.md) | retained Core owner | saved kernel ffe1166; Web provider in progress | coherent WIP80b7449 + remaining edge verification2f5c5eb saved/pushed. Declared6Prepare/3permit-fault/2cancel cases passed per author. Parent deadline repairc3967ca saved/pushed; same reviewer accepts composed source/peer boundary after11timing cases and affected regressions. Earlier not_verified resolved. Source/peer acceptance is not live action/P5 completion |
| [W02-provider-handoff](packets/W02-provider-handoff.md) | Web01a110ac-2aae-7841-9c8b-12ff38c52d9d / local | source handoff9c0af1b; A01 consumer | Source-only handoff complete: native checkbox Setter proposal, current F01 lacks actual checkbox, exact current resolver/effect/verification API still needed. Separate small actions.html fixture/README/receipt saved/pushed9bd9f66 with source/syntax/doc checks; existing F01 performance/layout untouched. Core compiling SetCheckedProvider/DeliveryPermit API received; provider728fa5b saved/pushed exact9, five focused synthetic provider/Core tests plus offline JS/check/Clippy attributed. Same Web reviewer accepts_with_residual finite provider boundary; [review](receipts/W02-provider-review.md). Truthful preparation1a8a602 saved/pushed: consuming prepare_exact from Snapshot/Prepare Request,8 focused tests/Clippy attributed; changed preparation review accepted_with_residual, no findings; actual host/live gates open. Core now has saved API for Prepare/Act composition; Web returns smallest live harness handoff. No synthetic facts accepted for live, no live input yet |
| [W02-focus-type-provider](packets/W02-focus-type-provider.md) | retained Web owner | source-backed Chromium handoff; A02 compiling API/save | running after Core explicitrelease9bd5809. DOM.focus/Input.insertText selected; Web owns plugins/web source/tests and JS live launcher. Necessary focus/selection normalization separated on real document focus. Runtime depends on realhostlane/Expectation composition, not a separate review of each file |
| [W02-form-read-facts](packets/W02-form-read-facts.md) | retained Web owner | FORMS/B02 and actual F01 action-state-result handoff | sourcec63b07a saved/pushed exact8; same reviewer accepts_with_residual changed getter/privacy/normalization boundary,7hashes matched. Author5new+4affected tests/JS/check/Clippy. Actual4b12d7b saved/pushed:4/4 Observe pass, UTF16 forward/backward/collapsed, output empty/London, private canary redacted/absent, state invariance and owned cleanup confirmed. Focus/Type/IME/business/fullB02 not claimed |
| [W02-live-actions](packets/W02-live-actions.md) | retained Web owner | accepted provider/preparation; Core guarded composition in progress | harness25bf999 saved/pushed exact3 after old80b7449 no-run/Clippy. Existing real barrier now saved/released by Core, no duplicate changes. Corrected Corec3967ca accepted;195-input2c037039… pins current. ONE actual headless actions run activated,120s and fixed32/depth8/64KiB/250ms; own loopback context, no desktop input. actual9d3a0a5 saved: all10 outcomes/12checks passed, source-derived Prepare→confirmed Setter→fresh Checkedtrue; readonly/remount/possible-before-delivery cancel and independent state/cleanup verified. No physical/business/delivered-loss/fullB02 claim |
| [L01-live-actions](packets/L01-live-actions.md) | retained Web owner | scoped accepted3ebfafe CLI plus actualAPI9d3a0a5 | actualec844f6 saved/pushed:6CLIcalls passed exits4/0/0/4/4/4, real Prepare/Execute expectedstate,disabledfreshrefusal/remountno-dispatch, unchangedcanonicalbytes/unrelatedstate and actualowncleanup. Source3ebfafe independently accepted, execution attributed. ReadonlyAPIproof9d3a0a5 separate; no fullforms/businessclaim |
| [L01-actions-implementation](packets/L01-actions-implementation.md) | retained Core owner | registered CLI-ACTIONS@1/c4bc255 + acceptedc3967ca/real Web9d3a0a5 | sourcea53b750 saved/pushed exact17 after WIP138; refusal2/4 resolved,22parent+8public CLI synthetic-CDP checks. Same reviewer accepted direct compact P2 repair3ebfafe after source-first/receipt+2hash match; real CLI/browser proof assigned separately. web_live.rs released Web |
| [M02-native-actions-handoff](packets/M02-native-actions-handoff.md) | retained Native owner | ACTIONS/FORMS/NATIVE and existing A01 kernel; source-only | source handoffec817e3 saved/pushed: ordinary checkbox setter not established, AXPress not substitute; exact kernel/helper/identity dependencies. actuale3a6880 saved/pushed: checkbox CFNumber0/Enabledtrue/settablefalse/AXPress; samplebutton Enabledtrue/settablefalse/Valueunavailable-25212/AXPress. No targetaction,identity/AX unchanged and27.8s own cleanup. heldAX/privateexchange proposal0e7fa32 saved/pushed. Root selected refs within one attached RuntimeHost session only; invalid on reap, no cross-CLI daemon/single-transaction substitute. Core private exchange/residency selection pending |
| [M03-popup-capture](packets/M03-popup-capture.md) | retained Native owner | own popup physical/AX/lifecycle facts; explicit all-image retention rule | acquisition@2/registry17 registeredaf8a1a4; exact7 writer/popup capture candidate23f22fb saved/pushed, author26writer/78popup checks and17canonical documents. Scoped source review accepted after source-first/receipt reconciliation and5hash match; prepared saved Swift23f22fb/Rust138d7bc. Initial3s combined configuration stopped BEFORE launch because helper inherits remaining budget. Root selected explicit1000ms combined Request/parent ceiling (both channels≤AX1s/capture2s maxima), no source change/quality reduction. actual8507aa4 saved/pushed: semantic AX Snapshot kept popup open; one combined Observe4 honest partial,2canonical validator0 channels,5AX nodes and362x228 retained PNG. Separate actual pre/post parent/popup identity values all matched, live AX equal,53.41s overall/finally cleanup and owned reap confirmed. Source23f22fb independently accepted; actual execution attributed. Prior interruptions preserved, fullM03/shared-parent/hit/transform/P7 open |
| [M03-direct-window](packets/M03-direct-window.md) | retained Native owner | actual unresolved mapper55a4ca81; supported nonvisual NSView.window candidate | source d5fd03f saved/pushed; actual open-popup Snapshot shows visible containing window9437 vs parent9430, equals_parent=false; operation51.7s with finally cleanup confirmed. No collector/capture acceptance; direct facts feed M03 connector, IDs are run-specific |
| [M03-popup-connector](packets/M03-popup-connector.md) | retained Native owner | actual direct-window evidence; existing source/identity/resource fixes | source21ac1d accepted; actual CLI popup AX validated partial/exit4,5 nodes/exact Surfaces/sourced anchor, inspect compact+JSON0. Full lifecycle hit300.15s watchdog before Confirm/stale/reopen, not accepted; cleanup confirmed/lane released. Cached-file equality does not prove live invariance. Native outcome b4a965e saved, remaining lifecycle-only actual close/stale/no-data/reopen positive passed59.3998s, live CUA AX equal. Separate combined post-current-file assertion unverified; Native inspects exact expression and minimal popup capture connection, no entire-chain repeat. Positive capture/shared-parent remain open |
| [M03-popup-attribution](packets/M03-popup-attribution.md) | Native01a110ac-2da3-73d1-9bb2-273d4ff99e7a / local | current identity source53e6e6e; existing F02 popup handoff | sourcec8d5287 saved/pushed, 19 focused author checks plus canonical/A controls. Same reviewer accepted repair71c82f47: resource refusal incomplete_scope, missing/stale identity preserved, no partial fallback. Author38 checks/8canonical validations. First own-popup interval interrupted before Snapshot, no mapping/CLI claim; finally cleanup153.39s confirmed. Shortcut a466051f worked in actual fresh run: Snapshot1 with popup still open; actual mapper unresolved/null, identity CLOSED. No CLI/pixels run; finally cleanup141.86s. Native source-only exact owner-attribution diagnostic follows outcome checkpoint. Positive capture remains open, B capture/pointer gaps protected |
| [M01-current-identity](packets/M01-current-identity.md) | retained Native owner | source53e6e6e; actual326622b, pushed | Scoped source review accepted through retained m01_acquisition_review. Actual identical-title A/B and CGWindowID reuse old-binding stale refusal/new-binding positive observed; total300s sequence failed at387.6s. Cleanup completed separately, temp absence verified. Do not claim whole bounded run accepted or repeat solely for paperwork; future cleanup precedes timing assertions |
| [L01-recorded-diff](packets/L01-recorded-diff.md) | retained Core owner | initial3d1c25e, correction9ab6a87 pushed; CLI@5/CLI-DIFF@2/registry15 | Existing before/after geometry32×16→48×24 reported through compact/JSON with separate original environments/Evidence; author focused checks pass. Root initial CACHE-style env restriction reconciled under BOUNDARIES/GEOMETRY and advisor evidence before code. Global CACHE/Delta compatibility unchanged; no fullgraph comparison |
| [W03-session-invalidation](packets/W03-session-invalidation.md) | retained Web/Core owners | Web24ed0e8/Core74d2e3b/peer3318662/harness9f03d52/result2085e065 pushed | Scoped source/peer review accepted. Actual B05 three Observe/Retain:120×32→150×32→150×48 css_px, original retained bytes/context/time unchanged, invariance/cleanup and temp removal confirmed. Live run does not expose internal invalidated flag or prove actual CDP loss; direct-state/peer evidence remains separate. Full W03/K02 open |
| [G02-recorded-diff](packets/G02-recorded-diff.md) | retained Core owner | 5476c2836748d156719a5d8c97155140a685b466 pushed5paths | borrowed exact-node/property comparison, content vs evidence-only and missing-side states;5focused tests/check/Clippy pass per author. No inferred deletion/Delta/cache mutation or full graph/CLI diff claim |
| [L01-platform-cli](packets/L01-platform-cli.md) | Web/Native existing owners | saved productCLI24ffcb, Nativebc2a264/Web57883cd outcomes pushed | actual own-platform observe partial4→inspect compact/JSON0 on both; Web known120css_px Measure0, complete Snapshot/evidence retained. Cleanup/temp removal confirmed. General targets/allpilots/distribution/P7 open |
| [P01-invariance](packets/P01-invariance.md) | retained Native owner | matched off/on e1c5755; stopped pointer attempt fd3f2fb, pushed | Actual76AX records and1100×1050RGBA equal under matched inactive state; cleanup/temp removal confirmed. Pointer hit remains waiting_evidence: CUA coordinate transform/foreground attribution not established, advisor confirms exact gap. No further blind coordinates; fullM05 remains open |
| [L01-live-observe](packets/L01-live-observe.md) | retained Core owner | 24ffcb7972fed6e22f868db54c22ce599725ede8 pushed22paths; CLI@3/registry13 | scoped independent accept: explicit authority, actualclock/budgets/ACKedNDJSON/outcome/cleanup;21digest matched. Actual peer/source checks passed per author. Separate platform actualCLI checks now required, no general/arbitrary-target/release claim |
| [P01-host-channel](packets/P01-host-channel.md) | retained Core owner | a1ad9143e4cc137535ef476b366e91795e524190 pushed7paths | scoped independent accept: same Ticket/ACK/reap, channel2 non-capture,2helper cap/quotas unchanged;5source-test hashes matched. Actual peer5cases/legacy regression author evidence, Native probe/SDK/off-on separate |
| [P01-probe-source](packets/P01-probe-source.md) | retained Native owner | source79a4853/db0154f; lifetime5223011; outcome b685a8c pushed | actual own explicit snapshots→H01 ACK→guarded Rust8→18pt passed with original cache/unverified time and unknown screen transform. Current validator replaced stale pre2491dec binary without recollecting baseline. Scoped source review accepts integration,5inputs matched; all temp/owned cleanup confirmed. Full off/on/M05 remains open |
| [L01-inspect-json](packets/L01-inspect-json.md) | retained Core owner | 8669ef339e37740b536d640019733d781f9c0dd4 pushed9paths | CLI@2/registry12 faithful chosen envelope registered before code; root read full leaf/routes. Four inspect tests/Clippy and actual Web/Native views pass per receipt, exact embedded Snapshot preserved. Compact/schema/analysis unchanged; no live revalidation claim |
| [W01-rooted-selection](packets/W01-rooted-selection.md) | retained Web owner | source1d83dc8+correction1568af3, harness40f4567 | actual corrected ownF01 rooted sequence passed9DOM+9AX/one39606B ACK/3binding-stale refusals/invariance/cleanup. Scoped source+receipt review accepted_with_residual; runtime/removal attributed. Real Director/foreignbackendseed/live-reparenting/fullpilot separate |
| [L01-inspect-compact](packets/L01-inspect-compact.md) | retained Core owner | 507383d946ebd2b42ea3aa6851f9b005a9ac602f pushed7paths | compact interaction/design passed3new+3existing CLI cases/check/Clippy and four actual retained Web/Native calls, source unchanged. Saved source hashes match author receipt; required JSON assigned next |
| [M01-form-context](packets/M01-form-context.md) | retained Native owner | source83247b9 and actual outcome b6811cb3fd76fe11b4d3544595192cc1b96c0c29 pushed | actual H01 field observation passed:76nodes/5fields, f02.name textbox, placeholder Name, enabledtrue/focusedfalse, name unsupported without fallback; ACK/validation/state invariance and own cleanup. Current temp removed/verified, no broader Director/pixels claim |
| [G02-live-data-measure](packets/G02-live-data-measure.md) | retained Core owner | correction2491dec9176ebe79e66f03ba4e13c4aa6ad133ff pushed5paths; earlier failure685651c | Scoped independent accept from retained h01_producer_review: source-first then receipt reconciliation,44input/source-test hashes match. Five focused checks and actual CLI32/16/48/24css_px exit0 remain author execution. Unknown consistency/Evidence intact; other refusals preserved. No full G02/fresh UI/import-recompute claim |
| [W03-source-handoff](packets/W03-source-handoff.md) | retained Web owner | full prepared W01 sequence passed; K01 retained source | completed with product steering: W03 is not prerequisite for current B03 consumer. Actual collector drops explicit relation evidence; current canonical types suffice. F01 City input is outside popup, not a Director replica |
| [W01-popup-relations](packets/W01-popup-relations.md) | retained Web owner | sourced11c077 and actual result09adc4b pushed | scoped independent accept_with_residual:3explicit relations, popup geometry and Close focus/invariance supported; runtime/fingerprints/temp removal author-attributed, removed artifacts not recreated. Full B03 hit/clip/frame/pixel and real Director remain open |
| Product next-case consultation | original Mac/Web advisor chats, retained | explicit user routing; latest action-state-result handoff in [advice](receipts/platform-test-advice.md) | Both read-only replies received. Web choice toggles applied membership then closes; Mac expected choice applies and stays open. Dismiss is not rollback; client membership is not server result. Mac RC02 selection/apply remains unexecuted. Future B02/M02 consumer; no real-app runtime grant inferred |
| [G02-source-handoff](packets/G02-source-handoff.md) | retained Core owner | current accepted G01/local analysis; existing projection contracts | completed: existing canonical relations/types support missing borrowed one-step neighbor selection; no source mutation/checks. Immediate implementation follows |
| [G02-scope](packets/G02-scope.md) | retained Core owner | 53c6f74a1acb7e7f2b24ea579ece926d8ad03af5 pushed5paths | finite pure slice accepted from [author evidence](receipts/G02-scope.md):6 focused tests/check/Clippy/self-review; root matched three source/test hashes and saved5path identity. Borrowed explicit neighbors only; full views/live G02 remains open. Core capacity error followed successful push/cleanup; no retry needed |
| W01 ReadException repair | Web; same /root/web_collector_review | d19d13018fb74c604ef9d7c2666a10ada3e0d270; actual result1acc4b8 | [scoped accept_with_residual](receipts/W01-collector-recheck.md): source-first protected-boundary observations then frame/report hash/receipt reconciliation; first actual response/invariance/cleanup supported. Broader prepared cases continue; no full Web/Native/D06 acceptance |
| [M01-host-observe](packets/M01-host-observe.md) | retained Native owner | sourcefd559df3, ownership docs7942d87, actual result33eecdb2873d9ed06f065d6e875691ecd8bbea9d pushed | First real A/AX/H01 passed after canonical-path probe correction: Activate sample/button/enabled/known pt bounds, matching identity/partial coverage, ACK+validator and all owned cleanup. No capture/permission changes; broader Native/SDK/pixels/D06 open |
| [W01-live](packets/W01-live.md) | retained Web owner | provider9b881e6 + repaird19d130; actual full-sequence receipt0b83f23 pushed | Six real responses, three expected stale/wrong-target refusals and11checks passed; immutable input/binary pins and cleanup confirmed. Three actual canonical frames retained. New B03 relations assigned separately; no active source/runtime hold or full B01–B06/D06 claim |
| H01 private diagnostic handoff | Core retained owner | 1810b1dabd6a2d07d4931606f3859dd96f614d44 pushed8paths | [receipt](receipts/H01-host.md): bounded cause/stage carrier saved, common paths/index clean and Git released. Focused checks pass; prior full-target timing residual remains explicit. Immediate consumer Web; no live acceptance |
| H01 publication allowance | Core / retained producer reviewer | f2af5a30226943412ae6c41dc7bb5c250a2f70c0, pushed5paths | [scoped accept](receipts/H01-producer-review.md):21inputs+7dependency hashes independently match; source closure and7predeclared modes close current pinned allowance gap, not whole-worker/live/SDK/D06 |
| [M01-acquisition-plan](packets/M01-acquisition-plan.md) | retained Native owner | proposal45c2667649c347202dfc64b6d5d978c3e47271f0, pushed | selected engineering handoff under ROADMAP/D05; public-header/source/old-sample evidence, no SDK/runtime claim |
| [M01-acquisition-registration](packets/M01-acquisition-registration.md) | retained Native owner | a0281dff74657b10baa4c7d137fb14572d88476f, pushed6docs | accepted faithful registration: D05@4/Native acquisition@1, registry11; all17ceilings/derived8192 preserve selection/common Rust/D06, no implementation acceptance |
| [M01-acquisition](packets/M01-acquisition.md) | retained Native owner | 9a5a8908 plus focused8d4f5016da767a5fa7b6497d350c6abbf37a232d, both pushed; registered a0281dff | [focused review accepted](receipts/M01-acquisition-review.md): all three offline gaps closed; source12/binaries/validator independently matched, author3suites/46assertions. SDK/H01/live/pixels/D06 remain open. Same chat preparing exact first-observation handoff, no repeat audit |
| H01 parent-death repair | Core same chat | 5f52cb5abd39fc088543b5bc3bd9504e05795386, pushed4paths; docs corrected3cba199 | [same-reviewer acceptance](receipts/H01-producer-review.md), independent89hash match; watchdog exits during held network read. Source assertion<2s from observed stall, no strict1s/orphan-reap/live claim |
| H01 direct-validator supplement | Integration / retained allocator reviewer | prepared5639cba, run5f52cb5, receipt6230b74ed692241a36222ae88737451018807c2f | [scoped review](receipts/H01-allocator-review.md) supports rejection/direct-validator refusals; callbacks/ownership and controls inspected; execution remains author evidence, not integrated phase/ACK/full H01 |
| [M01-host-helper](packets/M01-host-helper.md) | Native `01a110ac-2da3-73d1-9bb2-273d4ff99e7a` / local | `7c6e0780ac085c2a024281c2f8b0196ce536bfa0`, pushed9paths | [source review](receipts/M01-host-helper-review.md) accepts bounded connection, no findings; independent19/7 manifest and3binary hash equality. Author4builds/31offline cases, no SDK/live. Retained owner for next consumer |
| H01 parent allocation proof | Core same chat | `ff92c5111754c9a79b352a9207fd562325d69f97`, pushed3test/doc paths | [receipt](receipts/H01-host.md): actual setup backing+inline roots equals reported inventory; zero additional observed allocations on named paths, not OS/SDK/RSS or all-path runtime acceptance |
| H01 rejection allocation proof | Integration same chat | `da6330ee68c640d4b3687bf9ac46a0577eef2f30`, pushed2paths | [receipt](receipts/H01-allocation-proof.md): fixed1MiB key/3MiB quota, actual Decode fatal and prior ACK/reap; saved71 digestfc155032…48c36. Direct semantic-validation proof remains separately assigned |
| W01 guarded producer proof | retained Web owner | `74ea8e1ad9ea95eae6d0e491cae674f472095717`, pushed6paths; production c0abcff unchanged | [receipt](receipts/W01-guarded-worker.md):5worker+1config PASS; same producer reviewer accepts bounded-peer proof after source-first observation and independent91-input match8156d3ef…21265d. Live Chromium/SDK/D06 remain open |
| [H01-producer-review](packets/H01-producer-review.md) | `/root/h01_producer_review` | saved64cbec9 → c0abcff; two-stage read-only review | [accept_with_residual](receipts/H01-producer-review.md) for source connection; no findings; reviewer independently matched81/105/71 saved-input digests. Real producer/live/remaining allocation gates open |
| H01 Native deadline proof | Core same chat | `231882ad71e9d36194fbed621cf4120bdd30f12c`, pushed3test/receipt paths | [author receipt](receipts/H01-host.md): cancel and500ms request deadline after real AX ACK preserve bytes; delayed reply ignored, actual reap. No production change/SDK/live; separate from pinned c0abcff source review |
| H01 Replay allocation proof | Integration same chat | `1595a877a7c85d694e4b22d1b95195edcd79efb2`, pushed receipt only; test alreadyeea3c9f | [receipt](receipts/H01-allocation-proof.md): unchanged480KiB/4MiB−128KiB case proves real quota phase3, old ACK bytes/reap;71-input identity independently reconciled. Remaining validation/rejection attribution stays open |
| H01 connected producer provider | Core `01a111a7-9887-7983-9aa0-c08dfa2d46bc` / local | `c0abcffed886d0f33b34ddf3008a902b8e024676`, pushed16paths | [receipt](receipts/H01-host.md): Native broker/non-UI peer evidence; Web hooks compile. Author saved81 digest3b5d4412…c5a9e6 and105 digest68632cf4…ec4a363 match;18runtime+2isolated peers/default-web checks. Consumers and independent changed-caller acceptance remain open |
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
| [H01-host](packets/H01-host.md) | Core, excluding Native process files and assigned allocation-proof tests | stageC50c0c95f273c51120a088f68d556b27f57bd4195 pushed | [receipt](receipts/H01-host.md): actual reusable worker/GlobalAlloc/canonical path and R1 consumer connected;18paths,63-input digest7c192040…17bdcb saved equality reported;11tests plus isolated peer/check/Clippy/fmt. Core continues remaining lifecycle/helper/nonce; Integration owns allocator proof. Full H01 and independent acceptance remain open |
| [H01-process](packets/H01-process.md) | Native provider/peer; Core API/support owner | Nativebb69d4c1f9128975fcdce72a8329b0a637756037 pushed | [receipt](receipts/H01-process.md):7paths saved, index released;9prior tests +2affected new-API tests/check/build/Clippy,38 shared hashes matched per phase. Real-worker extent verification remains Core obligation. Exact saved Core hooks/support input checkpoint and independent review pending; Native temp retained |
| [H01-process-review](packets/H01-process-review.md) | collaboration `/root/host_process_review` | Nativebb69d4c, Core7b942ab saved38-input equality | [review](receipts/H01-process-review.md): reject one P1 H01-PROCESS-R1, SA_NOCLDWAIT/kernel auto-reap permits PID reuse before kill. Core confirms no implemented parent-signal guarantee; no other scoped finding. Same reviewer retained |
| [H01-reaping-repair](packets/H01-reaping-repair.md) | Native OS boundary/tests; Core host lifetime integration | Native54da088807e08b977562842b9095cf164accc310 + Core APIb22b05af72c29bae4bb3c72785f831e0dbfed8a2, both pushed | [same-reviewer recheck](receipts/H01-reaping-recheck.md) accepts finite Native boundary with residual: all38 saved inputs match, no scoped findings; four initial/two forwarding tests attributed. Actual connected Core lifetime/quarantine/ACK proof remains mandatory; no full R1/H01 acceptance |
| [H01-process-dependency](packets/H01-process-dependency.md) | existing Integration owner | 7b1489f5a23955978fdcba78c3d5131417768805 pushed | D07@5/registry10 faithful exact libc0.2.190/default-features=false adoption registered and accepted by root;4docs checked/saved, index released. Core adoption explicitly unblocked; unsafe/runtime proof still open |
| [W01-cdp-session](packets/W01-cdp-session.md) | existing Web source/member owner | 2f9ea77aeec6465e6d6d3a7d109d48840f1f8287 pushed; inputs3abd8e5 | [independent review](receipts/W01-cdp-review.md): accept_with_residual,19 author tests/check/fmt/Clippy and exact fingerprint reconciled. No source repair; own temp cleanup next. Semantic collector/live/host/D06 remain open |
| [W01-cdp-review](packets/W01-cdp-review.md) | collaboration `/root/web_cdp_review` | artifact2f9ea77 vs3abd8e5 | [review](receipts/W01-cdp-review.md): accept_with_residual/no findings; fresh source-first then receipt reconciliation, exact saved fingerprint and hashes matched; no reviewer execution or H01/live claim |
| [W01-collector](packets/W01-collector.md) | existing Web owner | 5bce7c6b71d3f2a495dab5826460a600e83533d4 pushed | [receipt](receipts/W01-collector.md):15paths saved;24Rust+11offline JS checks/check/fmt/Clippy, fingerprint834b7fe6…fd1eda5 matched. Source frozen for independent review, no live claim. Existing refs/root-frame/flat partial scope and single ExternalSemantics DOM+AX boundary explicit; temp retained |
| [W01-collector-review](packets/W01-collector-review.md) | collaboration `/root/web_collector_review` | source5bce7c6; registry10/source-stage scope | completed two-stage reject; [receipt](receipts/W01-collector-review.md): P1 selected-node continuity and P2 pre-acquisition reply cap. Hashes match; no related dirty drift. Same reviewer retained; author tests are not live proof |
| [W01-collector-repair](packets/W01-collector-repair.md) | existing Web chat | 4a45400d12a4aff9f02dd01930877333a4977df1 pushed | [same-reviewer recheck](receipts/W01-collector-recheck.md) accepts R1/R2 source repairs with residual;11paths match,28Rust/14offline JS/check/Clippy/fmt and aggregate fingerprint remain attributed. No live/H01/fullW01 acceptance |
| [H01-allocation-proof](packets/H01-allocation-proof.md) | retained Integration01a110ac-30da-7ab0-bed1-8d7a8e4de45e | Restore D02@2/D05@3/MEMORY@2/WORK@1; saved Core50c0c95 | running, three exact new test paths/one receipt, no production edits. Guard/fatal prerequisite before hostile parsing; Core retains production and lifecycle owners; stage-D supervisor changes require current input handoff for affected checks |
| [W01-bootstrap](packets/W01-bootstrap.md) | existing Web01a110ac-2aae-7841-9c8b-12ff38c52d9d | 8f770630a288c0fcddfa76ca2431c4b52d8839ed pushed | [fresh source review](receipts/W01-bootstrap-review.md) passes;22 inputs/fingerprint independently matched,37Rust/24offline JS/check/Clippy/fmt attributed. Bounded root light-DOM initial refs only; H01/live/broader modes/B01–B06/D06 remain open. Primary temp retained for immediate integration consumer |
| H01 lifecycle stage D / Core R1 consumer | Core; same host_process_review | 9424759e83296b5c3c622e92c5443a1d0cb8369c pushed;64-input digest7ba21b6d…8453ef | shutdown admission repair and6runtime cases plus isolated peer/check/Clippy/fmt saved. [R1 consumer review](receipts/H01-core-reaping-review.md) rejects returned-Lost child dispatch at prior50c0c95, unchanged by stage D; Core focused repair required |
| H01 returned-Lost repair | Core; same host_process_review | bf67c98b72ab73e3abb7babc20f8adfb98110391 pushed | [same-reviewer recheck](receipts/H01-core-reaping-review.md#repair-recheck--r1-accepted) accepts R1;64 saved inputs/digest independently reproduced, no remaining scoped findings.7runtime+2isolated tests attributed; continuous supported caller policy remains, full H01/allocator/live gates open |
| H01 initial allocator probes | Integration | 731a5248c6f43adf8f990d346fa321d861a402d7 pushed; shared input6523052 | [receipt](receipts/H01-allocation-proof.md):4/4 small method-level cases, Clippy/fmt, all probe children reaped; root reproduced16 saved-input digestb17be48b…53bd5b at6523052. Installed-worker hostile cases and injected System-null remain unexecuted; no full H01/source acceptance |
| H01 nonce/Cargo stage | Core | 6523052114a92c7262c29353ca2e6ce3311082a6 pushed;68-input digest31e9bfd0…77d788d | [receipt](receipts/H01-host.md):10paths saved,5 new fake-effect scenarios/12runtime+2isolated cases/check/Clippy/fmt attributed. One-use parent permit, Possible/no retry, Target/physical lane tested with fake peer only; production worker still rejects Mutation. No real-input/full H01 acceptance |
| H01 private null-proof seam | Core → Integration | 5e5da61004eb9d12b520e9e4b2181cbf5147b8a7 pushed | exact2paths; private allocate_with shared by production alloc/zeroed, shipping System forwarding unchanged. New allocator pin bfa9a61d…59d9aa replaces prior pin, other3 stable. Focused installed-quota/reap/check/Clippy/fmt attributed; Integration owns injected-null execution, not yet proved |
| [H01-native-handoff](packets/H01-native-handoff.md) | retained Native01a110ac-2da3-73d1-9bb2-273d4ff99e7a | 99c4c159c751024f339faf175f36bfbda6372fb7 pushed | [receipt](receipts/H01-native-handoff.md) accepted as finite source evidence: actual argv/stdin/stdout vs FD3/4/5 mismatch, Target/scope/clock/Ticket/redaction boundaries and Running/Tape/single-frame gaps. Channel-specific invocation is a proposal, not selected wire layout or adapter acceptance. Core consumes next |
| H01 combined null seam | Core → Integration | 00e282e36f509375bd33cc84b2e9a4997d6b6c57 pushed | exact2paths; private realloc forwarding added to alloc/zeroed seam; allocator pin2f1bf278…41e50e. Shipping fixed System and null failure accounting preserved; compile/Clippy/fmt attributed. Integration null runtime wave pending, no full guard acceptance |
| H01 combined null proof | Integration | 2a712aa36b826a772aa4ce6665daee912dfb8cb4 pushed | [receipt](receipts/H01-allocation-proof.md):5/5 bounded method-level cases/build/Clippy/fmt, all children reaped; injected alloc/zeroed/realloc failures preserve required accounting. Root reproduced16 saved inputs/digest4d717bc4…c21c53. No actual OS exhaustion or hostile/phase/full H01 acceptance |
| H01 guarded worker proof | Integration | bdeb87ef8ed0294e0aea14ad645159a7455b79ec pushed | [phase matrix](receipts/H01-allocation-proof.md#mandatory-failure-obligation-matrix):4/4 child-only cases/Clippy/fmt; real core0.1/analysis0.2 decode quota, malformed rejection, ACK survival through later quota/reap, normal Retain/Replay and post-encoding frame refusal. Saved64 digest3ab2314a…c78360a matches per author. No forced validation/replay/admission-allocation claim; next two concrete refusal cases assigned |
| H01 registered helper owner | Core | e11b44bd8840e443b6bf46f9f956ca0e5f585b38 pushed;71-input digestdc0be5f4…d3df57 | 10paths saved;16runtime+2isolated cases/check/Clippy/fmt attributed. Real direct child/capture/ingress/grant ownership works for single-frame helpers; active worker broker and actual Native composition remain missing, not full H01 acceptance |
| H01 helper-slot admission repair | Core | 8edf546103430f02595441b303850a886134b0dd pushed | 3paths saved; retained raw bytes no longer hide second vacant same-session slot. Focused regression/Clippy/fmt attributed, helper_runtime ec7dca71…aa59f and test b11e57e7…767da saved equality confirmed by Core. No allocator/worker pin change |
| H01 fatal-status timing repair | Core | 64cbec90de0bd1b46c8faea4efe01b2f9b3204c5 pushed | 4paths saved; available matching fatal record read before IO terminalization, missing status stays generic. Actual worker timing regression/control and17runtime+2isolated peer cases/Clippy/fmt attributed. Supervisor saved pin fbf181f3…581ee5; no worker/allocator/Cargo change |
| H01 writer/retained refusal proof | Integration | ba3c372a48b4c104e5e224f16368ad88113de3ef pushed | [receipt](receipts/H01-allocation-proof.md#existing-api-phase-refusal-wave):2/2 phase_ cases/build/Clippy/fmt, FixedOutput refusal and small retained allowance preserve prior ACK/base/replay. Saved64 hash eccadde84…eb63aa1 matches per author. Explicit resource refusal, not per-phase allocator OOM; leases/barrier released |
| H01 prepared Replay attribution | Integration | eea3c9f95e6d513145b85bc25c792247fe537703 pushed | 2paths saved as prepared WIP; fixed480KiB/empty-delta/ordinary4MiB−128KiB case and fixed64-byte fatal observer. Own fmt/whitespace only; no new compiler/runtime/acceptance claim. Waiting exact new producer baseline/ACK, no active freeze |
| [H01-allocator-review](packets/H01-allocator-review.md) | collaboration `/root/h01_allocator_review` | stable guard64cbec9, allocator2f1bf278…41e50e; packet8c159d1 | [review](receipts/H01-allocator-review.md) passes stable source scope, no findings; author5/4/2-case evidence reconciled without execution. Changed installation/producers, forced rejection/validation/Replay allocation, full parent/live/D06 remain open; not full H01 acceptance |
| [W01-guarded-worker](packets/W01-guarded-worker.md) | existing Web01a110ac-2aae-7841-9c8b-12ff38c52d9d | source draftd9de0a60a22b0551768e96474547548bb04ca4e3 pushed; shareda8e5c06 |4owned module/config/doc paths saved with own fmt only; compilation/runtime pending Core module/caller/parent hooks. Peer/tests preparation active; no fake Permit/ACK, no live grant. Common/Core paths protected |
| H01 shared producer boundary | Core | a8e5c06f54af8c1c955b18f6d7431455622b4f7c pushed |12paths saved,98-input digest6887caf0…4555006 matched per Core. Real begin/Ticket/ObservationRun, admission/publish primitives and compiled Native exchange; default/web checks and old17runtime+2isolated regressions attributed. Parent ObserveReady/HelperRequest dispatch/Web glue still incomplete |
| [W01-bootstrap-review](packets/W01-bootstrap-review.md) | collaboration `/root/web_bootstrap_review` | saved8f77063 vs4a45400, source-first then saved identity/author evidence | [review](receipts/W01-bootstrap-review.md) passes scoped source, no findings; protected owners unchanged, traversal vs DOM-read counts truthful. No reviewer execution or full W01 acceptance |
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

Allocation worker «UI Blueprint — Core R03»
`01a110ac-30da-7ab0-bed1-8d7a8e4de45e` archived after terminal completed/idle
confirmation and scoped accepted proof handoff. Closeout
`c3c653b9f349488d2cd4478d002387dedce210ca` pushed; index/owned paths clean and no
process/source/Git lease remains. Only its owned uib-h01-allocation-07xaavuw target
directory was removed after retention release; durable application evidence and
all saved source/receipts remain. No assigned immediate consumer requires that
worker now; history is recoverable if a later concrete task needs it. This archive
does not declare full H01/live/D06 complete. Core producer, Native acquisition and
Web live-preparation owners remain retained for their immediate next work.

## Current remote save condition

Ordinary pushes through the old origin redirect returned GitHub500. Root verified
origin and the GitHub-reported canonical git@github.com:potapenko/ui-blueprint.git
had identical remote HEAD/master, then pushed exact saved master SHA to that same
canonical repository. Pending commits through11fe45b and later worker checkpoints
were confirmed remotely. Git configuration, keys and history were not changed.
Use bounded same-master canonical pushes when the redirect fails; no force or new
branch. The earlier push_pending condition is resolved, not hidden work loss.

Resume outcome checkpoint: Native8507aa4 actual popup pixels shown inline in root;
retained final/staging system-temp paths are in M03 receipt and must never be deleted.
Desktop is released. Mac advisor handoff was consumed by successful direct AX setup;
no need repeat shortcut/initialInspect/close-stale-reopen. Root selected next M02
source handoff from reference branch EXECUTOR-SOURCES/NATIVE-SOURCES→REUSE, full
Native/action closure reused; no additional runtime/implementation authority inferred.

Latest form read runtime4b12d7b:4Observe/4oracle checks+browser survival passed on
saved4281c18/a53/c63 inputs; no retries/tuning, sessions/groups/pending0 and own
processes closed. Source reviewed independently; actual execution remains author
attributed. Selection source facts do not mean Focus/Type or business success.
Core3ebfafe compact repair independently accepted; next shared action expansion
is source handoff only until actual owner/verification signatures are reconciled.

A02 selection: existing canonical Intent/Expectation support is source evidence,
ACTIONS/FORMS verification is existing product authority. Core extends one executor
owner with explicit expected source property, no inferred Type final value/business
success. Native AXPress maps to Semantic Activate, never SetChecked substitution.
Existing Global focus lane remains a required host dependency; no new runtime grant.
Public CLI-ACTIONS stays SetChecked-only until a separately selected caller contract.

Nativecapability intermediatefailurefea4c98 preserved separately; successfule3a6880
fixed tempadapter error boundary but did not establish prior invalidValue cause.
No false setter support; chosen native consumer is explicit Semantic Activate.
Source identity/temporary-helper lifetime proposal must reconcile current broker
real owners before Native action exchange implementation; no daemon/ref fallback.

Current next dependencies: A02 compiling API candidate uses ActionExecution/
ActionProvider and explicit prepare_action Expectation, SetChecked aliases preserve
callers; Core owns mechanical Web action.rs until checkpoint/release. Twelve pure
tests passed per current progress, consumer compile/checkpoint/review pending. Web
prepares primary-source Focus/Type delivery handoff (Input.insertText focused-widget
and automatic-focus semantics must be explicit); no capability/atomicity invented.
Native awaits concrete shared helper exchange/lifetime after saved0e7fa32 proposal.
All prior runtime lanes released; no actual new Focus/Type/Activate input authorized.

## Согласованный рабочий порядок после вопроса о задержке

User2026-10-08 уточнил, что спрашивает о пользе ревью, а не требует безусловной
отмены. Root отозвал blanket no-review message всем3owners. [Ранбук](execution.md)
фиксирует обычные tests + законченный сценарий, risk-focused review интеграции
и отсутствие микросогласований. Исторические review verdicts не переписаны.

Current ownership envelope within approvedP5: Core task-wide crates/plugin-api и
crates/host source/tests/common integration; Web task-wide plugins/web source/tests
и tests/bridges/web launcher; Native plugins/macos и tests/bridges/native plus
соответствующие native developer docs. Core не пишет Web/Swift owner, они не
пишут Core. Existing schema/Cargo/public CLI/product fixture behavior остаются
защищёнными: реальную новую необходимость разрешить по выбранному spec contract
до affected edit. Каждый owner сохраняет source/tests и короткий result в своем
existing receipt; root владеет только coordination. Межфайловые механические
адаптации внутри owner не требуют повторного root grant; общий API синхронизируется
одним concrete handoff. Пауза/goal completion не объявлены.

## Приоритет пользователя: геометрия для разработки UI

2026-10-08: основной продуктовый смысл — быстрые реальные геометрические данные
для ИИ, Web/Mac. Root перечитал оригинальные тематические разделы UIB.TZ@1.4
и PLAN.UIB@1; смещение приоритета на P5 признано ошибкой исполнения. Scope P0–P7
не сокращён, goal active. Три owner получили drain только развития input: сохранить
текущий coherent code/tests (честный WIP допустим), не начинать следующий P5 участок.
После checkpoint: Core — полезный geometry CLI/engine путь; Web — содержательный
компонент/вложенная геометрия; Native — AX bounds и measured probe внутреннихчастей.
Каждый возвращает существующий callable путь и один ближайший недостающий шаг,
без общего аудита/новой платформы. Оба advisor запрошены о существующем конкретном
кейсе и полезном геометрическом ответе, read-only без нового runtime/fixtures.
Это приоритетная запись поверх прежних P5-next строк, не удаление истории.

Geometry ready work now follows [G03](packets/G03-geometry-cli.md). Native P5 WIP
38546350d267716f75ec27c9adbdecd146ea734b and Web source464b1d214e24071b2b95d2adce2c5a61c4064514
saved/pushed, not full action acceptance. Core P5 WIPbc2874e26f73c0ae8566ef72113ea60efee9b106 saved/pushed exact7; now fixes
confirmed measure/check direct-ChannelResponse input; independent Native source
inspection identified the same reusable read_snapshot boundary. Explicit additive
ANALYSIS@2/registry18 registration precedes code; other contracts/wire/arithmetic
remain protected. Geometry workers have standing authority for their one controlled
F01/F02 chain once the saved loader is ready, no repeated activation round.
Advisors' RC03/Director handoffs are saved in platform-test-advice; missing real inner
bounds remain unknown. Full product scope retained, main geometry utility prioritized.

G03 concrete outcomes: Nativeea90baf68a44f1887d33f3eae8d6648976f3ab0b ordinary
CLI Observe/Inspect/Measure/Diff: component173.5x48→231.5x62pt and icon/text gap8→18pt,
AX screen versus probe local frames preserved, transform unknown/probe unverified,
actual cleanup. Weba7dfcfd01d2cc3cc89c1b98dcff39ec8a84314c5: group360x133,input188x21,
popup200x60CSSpx; trigger-popup gap137,leftoffset-77.234375,input-listgap4, pairwise
left offsets0/0, resize800→640 markerx660→500; actual Observe100–120ms in this run,
not Q02 p95 qualification. Original first alignment expectation failure preserved;
later full chain returned honest aggregate/clipping unknown with numeric pairwise
measurements. No Director/RC03 inner-geometry claim or repeated UI run.

Concrete remaining geometry defect: engine global Complete coverage veto rejects
Aligned/EqualSpacing before examining explicit known anchors. Core reconciled with
ANALYSIS/GEOMETRY explicit-target contract; synthetic3rects reproducealignment0.
Schema analysis/results.rs::known_inputs mirrors this veto and currently rejects
publication. Root authorizes exactly coupled guard removal+focused tests in engine/
schema, preserving all property/evidence/binding/unknown checks and originalpartial
Snapshot. No public types/wire change. ActualUIoutputs alreadyconsumed/deleted;
no reconstruction of runtime IDs/provenance, no recapture required for this fix.

Native external-geometry usability handoff: fixture-only gate is currently coded
(F02bundle/a-b/current identityfiles). Existing bounded collectWindowAX already
provides public AX accessibility_bounds in screenpt without actionrefs/probe. Next
minimal candidate is explicit read-only AXFocusedWindow binding with PID/incarnation
and before/after sameAXobject checks; CG mapping remains unknown, no title/rect/order
substitute, ordinary AX cannot reveal hidden SwiftUI layout. Exact public binding/
CLI extension needs selection before implementation; no realapp launch granted.

Next Native implementation selected in [G04](packets/G04-native-ax-geometry.md):
read-only native_ax ordinary-window binding via explicit PID/incarnation/public
AXFocusedWindow, no fixture identity/probe dependency, no CG mapping/input claim.
Native owns Swift/docs plus narrow CLI connection branch/tests; Core notified,
engine/schema protected. Truthful per-observation identity must fit existing contract
or return exact dependency; no invented continuity. Test only ownF02 genericroute.

Alignment source256f2a27e88106a1987515dc3ee654b9cd725120 saved/pushed exact7 by root
from completed Core author-ready result: explicitknownanchors nowcompute despite
partialSnapshot, mirrorvalidator/recompute/CLI JSON regression passed perauthor; no
browserrerun. Core thread01a111a7-9887-7983-9aa0-c08dfa2d46bc completed/notLoaded;
send_message failed twice threadnotfound (including explicitlocal host). Active/
archived tool lists did not expose it; no actor replacement/restart yet. Existing
source/history retained; root onlycheckpointed, did notimplement or reruntests.

G04 source/contracts/actual qualification05a76f0f1f95f0a160b072329fdfe2e5a8993592
saved/pushed exact14; NATIVE@2/CLI@7/registry19, Core0.1/analysis0.2unchanged.
Grouped binding/privacy review accepted, source matches; tests/runtime attributed.
OrdinaryAX process/window gave75nodes and173.5x48pt withoutfixturefiles;262.11ms
singleObserve is notp95. Next Native task: previouslyauthorizedF03/Q03 readonly
existingMac process from advisor—PID13309/playphraseme.Playphraseme, precision
incarnation must be refreshed before use. Neverlaunch/change/focus/resize/terminate
userapp, neverinferAXbindingfromCG11489/title/outer1920x1050. CurrentAXwindow/inner
geometryonly, old961window andpausedstate notassumed. Scope Role/Name/AXbounds,
existinglimits, ownhelpercleanup. Ifunavailable return fact, no silentpreparation.
Webadvisor continuation currently also reports threadnotfound; existing casehandoff
retained, no new realwebsite target guessed. Geometry work/goal remain active.
