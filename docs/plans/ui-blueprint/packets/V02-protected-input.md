# V02 — защищённый ввод и полный canary lifecycle

Самостоятельная shipping_product задача полного цикла по исходному PLAN.UIB@1
P5/V01 и PRIVACY.CONTENT. Источник — исходные нормы и Q01 initial observation,
а не новый продуктовый запрос. На94724df secure Native Fill/Type отвергаются;
M02 canary был введён через CUA setup и проверен только в canonical records.
Этот negative/read-only результат не закрывает нормативный input/error/cancel/
cache/history/prompt/enabled trace canary. Цель не сужать до уже сделанного.

## Результат

Реализовать минимальный поддержанный путь явно разрешённого защищённого ввода
на собственном Native fixture и доказать полный prescribed canary lifecycle.
Raw secret раскрывается только в delivery backend для конкретного разрешённого
действия; graph, snapshots/cache/history, diagnostics/errors/stdout и export prompt
его не содержат. Использовать opaque secret reference/существующий безопасный
механизм, без secrets в argv или публичных command records. Known empty и redacted
различаются. Delivery/verification, input owner, stop/unknown/no retry сохраняются.

Полный цикл в одном чате: local plan/source reconciliation → implementation →
focused tests/actual own fixture → исправления → docs/contracts → commit+push.
После своего плана выполнять без root grants на каждый шаг. Не создавать агентов
или другие чаты. Только синтетические секреты/canary; не запрашивать и не читать
реальные credentials пользователя. Никаких paid/external/account operations.

## Spec Basis и полномочия

AGENTS → specs/README → product/README → PRIVACY@1.CONTENT (включая обязательный
canary), ACTIONS/FORMS/IDENTITY/LIFECYCLE/CACHE/MODEL/BOUNDARIES/PROJECTIONS@1,
EXCHANGE@2/NATIVE@2 и explicit Requires; acceptance/NATIVE-PILOTS@1 M02,
COMPLETION@1 privacy/action criteria, GOLDEN/PILOTS; NATIVE-SESSION@1,
CLI-ACTIONS@3/CLI@16, D02@2/D03@3/D04@1/D05@4/MEMORY@2/WORK@1/Native
acquisition@2/D06@1/D07@5 и closure; RUST/DEV.RUST@2. Export/drawing closure
только для соответствующего canary consumer; product originals выше узкой leaf.

Режим Evolve технической Native input capability в уже одобренном outcome.
Сначала установить actual action-input representation/capability по коду и SDK,
не предполагать наличие setter у secure field. Существующее Unsupported — факт
кандидата, не новая норма запрета всей secure functionality. Новую конкретную
representation/допустимый typed reference зарегистрировать по ROADMAP/D03 ДО кода,
сохранив старые meanings/inputs/126legacy goldens/core0.1/analysis0.2 и все public
обычные workflows. Не применять строковые magic placeholders вместо настоящего
Action intent, не сериализовать secret в обычном ActionCase и не строить второй
parser/graph/секретное постоянное хранилище. Если без breaking shared-wire change
это невозможно, вернуть exact source-backed dependency/минимальный вариант;
не менять защищённый контракт молча и не объявлять задачу done через refusal.

Inputs: source6a5bec2/final94724df, NATIVE-SESSION leaf, current Native helpers/
resident host/effect gate, docs/development/schema.md/plugin-interface.md,
receipts/M02-native-workflow.md; PRIVACY and Q01 initial gap. Первым исследуется
конкретная существующая input/secret boundary, не всё upstream исследование заново.
Public Apple source/реальная capability определяют modality. Accepted CG posting
не становится Confirmed только ради теста; недостающий ack/неизвестный эффект
не вызывает auto-retry. Независимое наблюдение результата должно соответствовать
реальному разрешённому источнику без чтения/публикации защищённого значения.

## Владение и соседние задачи

Native helper/provider/form bridge/собственный fixture; прямо необходимые shared
host/CLI/plugin-api/action input owners и связанные tests; schema owner только
при обоснованной typed-reference/validation/accounting необходимости с сохранением
legacy; непосредственно нужные technical leaves/routing/docs; receipts/V02-protected-input.md.
До правок объявить exact paths и технический контракт. Нельзя менять unrelated
geometry/capture/Web/provider behavior или Cargo dependencies/frameworks ради удобства.
I02 owns distribution.py/README/notices/distribution docs/checks — не трогать.
Q01 owns integration harnesses/receipt и независимую QA, production не меняет.

Q01 сейчас имеет Native desktop lane ПРИ фактически доступной ownership. Начать
с code/offline verification; перед собственным actual UI дождаться освобождения
lane, продолжая независимое. Runtime ресурс сериализуется, а не все исходники.
Не устраивать скрытый concurrent input. После чужого input/явного handoff соблюдать
QA/CUA rules; elapsed time не разрешение обходить tool stop. Не запускать реальные
PlayPhrase.me проекты, менять TCC/displays или чужое состояние.

## Приёмка и сохранение

Реальный supported protected input + отдельно заявленные delivery/verification
факты; canary проходит backend error и cancellation, затем проверены все named
serialized channels до/после cache/history/export/включённой диагностики.
Не называть отсутствующий/неисполненный канал passed. Сохранять redacted/empty,
ref scope/generation, актуального владельца ввода, разрешение/nonce и bounded
время/память/копии. Pixels проверяются отдельно при включении, не выдавать text
redaction за pixel cleanup. Нет обещания универсального распознавания secrets.
Данные caller's secret source не удалять; собственные synthetic temp inputs —
по consumed-result правилу. Не публиковать canary/secret через тестовый failure dump.

Текущая master без worktree/branch; общий fcntl.flock
/tmp/ui-blueprint-master-git.lock для коротких scoped commit+push без root approval.
Coherent checkpoints+push по ходу длительной задачи; WIP не acceptance.
Build/output system temp; никакие images/содержащие их каталоги не удалять.
Final: capability, actual input/modality/verification и canary matrix, tests/results,
историческая совместимость, source/spec SHA+push, ресурс release и precise gaps.
Независимую privacy/shared-boundary acceptance выполнит Q01 после saved candidate;
самопроверка не называется независимой, full P0–P7 не объявляется этой задачей.
