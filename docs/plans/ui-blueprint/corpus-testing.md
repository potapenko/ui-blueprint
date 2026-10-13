# CORPUS-1000 — разнообразие интерфейсов и поиск дефектов

Дата: 2026-10-11. Статус: active после возобновления2026-10-12; текущее состояние
сохранено в [реестре](task-registry.md). Объём и критерии плана неизменны.
Пользователь прямо поручил создать помощников
по Mac и Web, тестировать широкий пласт интерфейсов, довести объём до тысячи
blueprints, находить закономерности и проблемы и улучшать продукт. Затем уточнил:
сначала составить план возможных проблем, затем выполнять тестирование.
Режим: coordinated, root только координация; исполнители — самостоятельные конечные
задачи с полным циклом. Новые продуктовые требования из тестовых догадок не выводятся.
Предыдущий source baseline: c63b600; registry37, DESIGN-ANALYSIS@1.

## Результат и честный счёт

Цель — 1000 выполненных и оценённых blueprint-кейсов на реальных источниках Mac и
Web, с полезными исправлениями обнаруженных дефектов. Кейсы различаются компонентом,
состоянием или значимым режимом layout; повторная генерация того же входа учитывается
отдельно. Тысяча картинок одной формы, число test assertions и количество узлов не
подменяют разнообразие. Числа ниже — размер конечных рабочих партий, не гарантия
доступности конкретного экрана и не новая норма продукта.

Для каждого засчитываемого кейса нужны source application/state, вход/выход сценария,
реальная геометрия и её источник, units/space/frame/coverage, согласованный screenshot,
actual CLI package и literal prompt, first generated image, проверка и disposition.
Число discovered, source-ready, numeric-checked, generated, evaluated и accepted
хранится раздельно. Negative/unsupported/failed captures не становятся successful
geometry/image cases; rerolls не скрывают первые ошибки. Старые три формы остаются
baseline, но сами по себе не закрывают новую цель. Недостаток корпуса сообщается
как факт, а не заполняется вымышленными состояниями.

## Какие проблемы ищем

| Слой | Риски и контрпримеры | Ожидаемая проверка |
| --- | --- | --- |
| Сбор/привязка | Другое окно/frame, stale ref, неполный AX, рассинхронизация изображения и bounds | Точная target/state binding; partial/unknown/refusal остаются явными |
| Геометрия | Layout vs visible/hit, clipping, scroll, popup за parent, safe areas, transforms, DPR/zoom, дробный остаток | Источник каждого размера; никакой оценки по растру или молчаливой смены единиц |
| Содержание | Длинный/перенесённый текст, локализация/RTL где реально поддержаны, пустое/заполненное поле, active vs applied, disabled | Реальный состав/состояние и подписи не теряются и не изобретаются |
| Закономерности | Совпадение округлённых чисел, 4/4/4/5, два разных семейства, отсутствие majority, дубликаты, другой viewport | Точные числа, явная область и основание; гипотеза не превращается в правило |
| Рекомендации | Намеренное исключение, tolerance, inequality, unknown applicability, нет реального expected_from | Объяснимый сигнал, отсутствие ложного fail, no automatic correction |
| Рисунок | Пропавшие controls/размеры, неверные anchors, screenshot fills, перегрузка текстом, неверная формула | Source-faithful blue/white outlines, читаемые краткие связи, неизменённые факты |
| Экспорт | Утечка private values/IDs, битые references, чужая revision, неограниченный ввод, неполный output | Existing privacy/binding/limits/refusal/совместимость; негативные tests отдельно |

Матрицу наполняют авторы источников: окна/панели, toolbars, списки/таблицы/сетки,
search/autocomplete, меню/popovers, sheets/dialogs, настройки, help/onboarding,
empty/error/loading только при настоящем воспроизводимом состоянии. Mac/iPhone/iPad
пример допустим лишь с реальным источником и честной платформенной маркировкой;
responsive browser не выдаётся за native iOS. Платформы/роли/состояния/viewport
пересекаются осмысленно, без декартова размножения одинаковой картинки.

## Цикл партии

1. Профильные владельцы дают конкретные маршруты, семантику, необходимые данные и
   ограничения. Сначала новые семейства, а не Director/Settings/Filters снова.
2. Исполнитель берёт конечную партию, получает реальные sources и сам выполняет
   observe/import → engine/CLI → literal prompt → fresh ImageGen → сверку.
3. Дефекты группируются по общей причине. Не делаем чат на каждый пиксель/команду.
   Подтверждённая ошибка получает воспроизводимый вход и регрессионный пример.
4. Владелец исправляет одну связную группу дефектов в продукте, тестирует затронутый
   путь и другое семейство для проверки обобщения, commit+push. Новые семантические
   решения предварительно сверяются с ТЗ/советником; тест не создаёт product intent.
5. Следующая партия использует сохранённый source revision. Старые failures и
   исходники остаются; итоговый результат не объявляется универсальной гарантией.

Три первых независимых пакета: Web-каталог/источники, Mac-каталог/источники,
детерминированная проверка analysis API на граничных случаях. Затем параллельные
партии генераций по готовым источникам. Один file owner; независимые source lanes
не ждут друг друга. Root принимает финальные результаты, не руководит командами.

## Пакеты первой волны

**CW1 / diagnostic, bounded.** Existing Web advisor «Research website UI blueprint».
Результат: не меньше20 отличающихся сценариев из существующего сайта с точными
spec/QA routes, safe entry/state/exit, компонентами и ожидаемой геометрической
пользой; реальные ограничения вместо догадок. Готовые datasets перечислить,
для новых указать воспроизводимый маршрут. Read-only source discovery, без UI,
без changes/login/subscription/server mutations. Consumer: первые Web партии.

**CN1 / diagnostic, bounded.** Existing Mac advisor «Спроектировать UI Blueprint».
Аналогичный каталог не меньше20 реальных Mac/имеющихся native mobile сценариев,
существующие debug/demo targets и наблюдение geometry/AX/probe; отличие runtime от
source-only рецепта обязательно. Read-only, без UI/сборки/изменений. Consumer: Mac
партии и выбор безопасного отдельного приложения/Simulator, если понадобится.

**CE1 / verification, bounded.** UI Blueprint worker: функционально разнообразные
детерминированные проверки DESIGN-ANALYSIS@1 на c63b600, в том числе arithmetic,
binding/unknown, unsafe recommendation и output attribution. Разрешён write set:
`crates/engine/tests/design_corpus.rs`, `crates/export/tests/design_corpus.rs`,
`docs/plans/ui-blueprint/receipts/corpus-analysis-wave1.md`. Product source read-only;
подтверждённые defects вернуть с минимальным repro, без правки expectations ради
green. Не считать synthetic vectors реальными UI-кейсами. Consumer: shipping
repair packets и будущий корпус; не строить фреймворк/новый crate.

Для всех: inherit model/reasoning, no nested delegation. Spec Basis UI: registry37 →
product route → DESIGN-ANALYSIS@1 → ANALYSIS@2/TYPES1/VALIDATION1/GEOMETRY1 +
MODEL/EXCHANGE2/IDENTITY/PRIVACY/PROJECTIONS/BOUNDARIES + EXPORT2 drawing closure
и CLI-EXPORT2. Source advisors дополнительно выбирают свой project spec/QA route;
они не меняют UI Blueprint контракты. Actions/new collectors/performance SLA — вне
первой волны. Economy basis: независимые платформенные знания и adversarial
проверки сразу питают готовые партии/репродукции, расширение только по реальным gaps.

## Выполнение и хранение

Взаимодействия с аккаунтом, покупками, отправкой сообщений и production-данными
не входят в геометрическое тестирование. Не трогать параллельные QR-login задачи.
UI capture packets выдаются отдельно с собственной целью и ресурсом; фактический
общий physical focus/input сериализуется. Root не использует Computer Use.
Не создавать branches/worktrees/постоянные output directories. Все изображения —
в system temp, никогда не удалять; tool originals копировать, сохранять оригиналы.
Обычные transient logs очищаются исполнителем; выбранный case package — явно
названный deliverable для corpus consumer, без commit сгенерированных receipts/logs.
Технические выводы и минимальные regression fixtures могут быть tracked.
Текущая ветка, exact-path checkpoint+push; в UI Blueprint Git flock
`/tmp/ui-blueprint-master-git.lock`. Чужой after-title-spacing.png не открывать.
Root owns этот план, task-registry и coordination-only source handoffs
`receipts/corpus-web-advice.md` / `receipts/corpus-native-advice.md`.
Worker reports objective/route/evidence,
checks, limitations, changed paths/commit/push и конкретный next dependency.
Waiting_resource освобождает lane и повторно проверяется до3мин; цель не закрывается
из-за ожидания. Принятые ненужные worker chats архивируются, advisors сохраняются.

## CP19 — bounded outline/assignment wording experiment, 2026-10-13

Direct user authority: improve actual generated prompts through image/critic cycles;
current CORPUS-1000 includes corpus-driven repairs. This is an agent-proposed bounded
presentation experiment, not a newly discovered arithmetic bug or a new product rule.
Existing HBP-HUMAN-001/R2/R3 requires blue interiors, white outlines and selected useful
anchored dimensions. CP18source2f90deb is accepted; B19still white-fills ON knobs,
adds an unsupported preference border and unassigned repeated dimensions. E04 shows
an inset transcription/anchor failure despite unchanged input. Prior failures remain.

Reuse original exporter owner. Objective: make these existing requirements clearer
in the literal compiler prompt through one coherent concise editorial candidate,
without changing source/measurements, assignments, CLI, flags, abstraction or source apps.
First compare the actual current per-object/overview/detail wording and applicable
ImageGen guidance; do not add another generic rule wall. If evidence supports no
specific alternative, return that result without a cosmetic code change. No semantic
contract evolution or generic renderer framework is authorized.

Writable scope: existing export prompt preparation plus directly affected tests and
one receipt corpus-outline-presentation19.md. Exact plan/files selected by worker.
Reuse the current fully read EXPORT2 drawing/HBP closure and common numeric/privacy
contracts from CP18; RUST/DEV.RUST checks apply only to actual changes. One independent
writer; master only, protected unrelated files and source repositories unchanged.

Use frozen CP18 B19 and E04 source inputs and qualified original PNGs. Freeze one
candidate before two first image calls. No wrappers, retouch, rerolls, new captures,
optional model calls or autonomous extra iterations. Machine files and selected
measurements must remain exact; focused affected checks only. Record each full-image
failure and any improvement separately; two samples cannot prove general reliability.
Report baseline/candidate pins, actual CLI text, checks, neutral index, separate author
assessment and scoped commit+push. Existing two critics assess the same frozen outputs.
These are repair-validation images, not additional primary corpus cases. No nested
agents. After that finite outcome return to root; future work requires a new decision
based on the result, not an automatic wording loop.

## CE2 — saved real-snapshot relationship verification, 2026-10-13

Existing user-authorized corpus verification, not a new feature or source-app edit.
After CE1's synthetic adversarial vectors, the real corpus expanded to194cases.
CP18 exposed a carrier-dependent omission (lowercase native select role), motivating
one finite actual-data verification of the existing DESIGN-ANALYSIS capability.
Immediate consumer: accurate mathematical explanations and grounded recommendations;
confirmed defects feed bounded implementation repairs, not speculative infrastructure.

Reuse CE1 owner. Select8–12qualified saved snapshots across at least6meaningful
families and both Web/Native from the ledger, recording unavailable/duplicate inputs.
Exercise existing equal/ratio/balance/pattern through actual engine/CLI with explicit
caller-selected families, public captions and bound evaluations. Preserve source
snapshots, compare independent source literals, exact facts/evidence and no-request
compatibility. Mixed observations/unknowns remain unavailable; no guessed intent,
tokens, global scale or normative check. Test-only mutations are never source cases.
Use current accepted da306a2, DESIGN-ANALYSIS1 and its current selected closure;
EXPORT2/HBP1.2R2R3, CLI18/CLI-EXPORT3, ANALYSIS2/TYPES1/VALIDATION1 and shared contracts.

Write authority: existing engine/export tests/design_corpus.rs only for genuinely
missing regression coverage, plus receipts/corpus-analysis-wave2.md. No product code,
source-app edits, specs, runtime/UI, modelcalls, new dependencies/crates/frameworks or
persistent dirs. Original datasets/images protected; task-temp inputs/build/evidence.
Return actual results and minimal confirmed defect repros; focused checks only,
scoped commit/push when files change. No new primary corpus cases or image counts.
One complete verification task; inherit settings, no nested agents. Root owns plan
and registry; CN16 human-authority dependency remains limited to source-app changes.

## CMATH1 — actual image explanations on four real families, 2026-10-13

Existing mathematical-prompt acceptance verification after CE2 passed11snapshots,
76measurements/47analysis requests; no new product semantics or generic wording loop.
Use CE2's recorded exact source operands for DirectorCW-B13-D, GenresCW-B16-D,
EpisodesCW-B15-M and iPadActorCN-B21. Reconstruct explicit public metadata in system
temp; original snapshots/images stay untouched. Verify frozen da306a2 CLI output and
source/data preservation, then generate one literal first output per required sheet
using each qualified original reference. No fresh runtime/captures/source-app changes,
wrappers/retouch/rerolls or compiler edits. Existing DESIGN-ANALYSIS1 and full current
export/analysis/common source/privacy closure apply. Unknown Native layout is not AX;
rounded equal labels do not mean exact equality; 2:2 widths create no recommendation.

One full task in existing exporter owner, no nested delegation. Only tracked write:
receipts/corpus-math-visual1.md; all generated packages/images in systemtemp, image
originals retained. Neutral index precedes author evaluation; existing critics verify
prepared math, source-fidelity and readable explanations. Source snapshots are reused,
so no new primarycase count; report actual validation-image count independently.
Stop after finite four-case outcome, report honest failures and scoped commit/push.
