# Q01/V01 — интегрированная функциональная и privacy приёмка

Queued до terminal M02-N, сохранённого integrated candidate и освобождения desktop.
Одна самостоятельная verification-задача: собственный план проверки, выполнение,
полный отчёт по исходным обязательствам и точные repair dependencies. Не отдельные
чаты на команды/fixtures. User-authorized P0–P7 и отдельные чаты полного цикла;
root не выполняет QA. Model/reasoning inherit; подагенты и новые чаты запрещены.

## Результат

На одной согласованной сохранённой версии проверить все M01–M06/B01–B06,
GOLDEN01, оба E2E, privacy canary, model-free export/поставку/recovery и границы
идентичности/lifecycle. Сформировать requirement → implementation owner → scenario
→ independent expectation → actual result → evidence → limitation. Ни наличие
кода, ни old receipt/зелёный частный тест не закрывают full criterion.
D06/Q02 статистические измерения и Q03 agent usefulness — отдельные tasks после
функционального candidate; эта задача не меняет их thresholds и не заявляет pass.

## Нормативное основание

AGENTS → docs/specs/README → acceptance/README → COMPLETION/PILOTS/
NATIVE-PILOTS/WEB-PILOTS/GOLDEN/PERFORMANCE@1 и explicit Requires; product
ACTIONS/FORMS/IDENTITY/LIFECYCLE/CACHE/PRIVACY/GEOMETRY/PROJECTIONS/MODEL/
BOUNDARIES@1, EXCHANGE@2, NATIVE@2, EXPORT полный DRAWING closure;
CLI@16/CLI-ACTIONS@3/CLI-EXPORT@2/G13/G12/ANALYSIS@2 по проверяемым путям;
NATIVE-SESSION@1 и его closure; D01@1,D02@2,D03@3,D04@1,D05@4/MEMORY@2/
WORK@1/Native acquisition@2/D06@1/D07@5, RUST/DEV.RUST@2 и explicit deps.
Перед dispatch revalidate фактически сохранённые revisions и source pin.
Основное намерение — UIB.TZ@1.4/UIB.DRAWING@1.1 и PLAN.UIB@1, не новый checklist.

Узкая implementation leaf или выбранный unsupported не сокращает исходные
положительные обязательства. Различать допустимую границу платформы/модальности
от отсутствующей обязательной capability. Например Semantic setter не доказывает
Keyboard/IME; безопасный отказ в действии не доказывает positive form result.
Проверить фактический смысл masked/secure field и checkbox требований M02 и
privacy-canary input/error/cancel/cache/history/prompt/trace, а не объявлять их
закрытыми только из-за Unsupported. Не изобретать требования сверх ТЗ.

## Работа и evidence

Использовать существующие собственные fixtures и авторские harnesses как инструменты,
но проверять их expectations/coverage по нормам и независимым фактам. Продуктовую
доставку не подменять прямым Playwright/AX setup. Состояния и setup явно отделять.
Не запускать/изменять реальные PlayPhrase.me проекты. Desktop lane принадлежит
только этой задаче после M02 cleanup; Web можно адресовать собственным headless.

Особо восстановить целые цепочки: Native точное одноимённое окно → popup →
measurement/input/result → resize/local diff → stale old popup → compare,
плюс M05 actual internal measurements и неизменность off/on. Web exact target →
partial draft/validation/autocomplete → applied → portal/remount/stop → diff/check
→ compare. Переиспользовать достаточные unchanged proofs, но финальную композицию
на declared revision не заменять разрозненными module receipts.

Проверить typed pre-Ready Attach refusal dependency W04/M02; unknown effects без
retry; независимость второго Target; partial fields и event-loss resync; full/delta
oracle одного source checkpoint, не equality двух разных live snapshots.
Privacy-canary — все предусмотренные сериализуемые каналы, pixels отдельно;
не считать disabled/unexecuted trace или непроверенный путь автоматически passed.
Model-free обычные команды и export проверяются без API keys/модели; пакет
согласованных binaries собирается existing distribution.py из того же source pin.
Проверить current tests без skip прежнего E03 baseline (reconciledc6b2357).

## Независимость и границы

Production source/specs/manifests не менять. Разрешены необходимые integration
harness/tests в заранее объявленных существующих каталогах, собственный
receipts/Q01-integrated-acceptance.md; historical fixtures/baselines не переписывать.
Настоящие implementation defects вернуть сразу с exact reproducer/owner, продолжая
независимые проверки. Root передаст их owner, не создаёт конкурирующего implementer.
Самостоятельность задачи означает доведение всей verification scope, не разрешение
выдавать исправленную самим reviewer реализацию за независимую приёмку.

Для нового обязательного source review: сначала собственное наблюдение saved
artifacts и criteria coverage, не читать авторский verdict; затем root передаст
author receipts для reconciliation тому же чату. Старые принятые source reviews
использовать по неизменённым доменам, не повторять всё без причины.
Source, author runtime, собственный runtime и remaining gaps разделять.

## Ресурсы, сохранение, финал

Текущая master без branch/worktree; только coherent saved input, без чужого WIP.
Shared fcntl.flock /tmp/ui-blueprint-master-git.lock на exact-path commit+push,
без root grant. Own source/build/run files system temp. Все images/их каталоги
никогда не удалять, показывать необходимые изображения inline. Nonimage outputs
сохранять лишь до названного immediate consumer; не создавать постоянный archive.

Для последующего Q03 можно удержать минимальные original canonical responses и
соответствующие image references выбранных controlled complex cases в own system
temp, с точными pin/environment/scope и named Q03 consumer. Это не новые замеры
реального приложения и не источник новых product expectations. После потребления
удалить только свои nonimages; images/каталоги сохраняются. Не восстанавливать
удалённые observations из чисел прошлых receipts.

Финал: отдельное решение по каждому обязательному критерию и источнику evidence,
все failures/неточные claims/ограничения, source/harness commits и push, ресурсы.
Если задача неполна, точная оставшаяся проверка/внешняя зависимость, а не общий
«всё работает». P0–P7 завершает root только после Q02/Q03 и всех оставшихся gates.
