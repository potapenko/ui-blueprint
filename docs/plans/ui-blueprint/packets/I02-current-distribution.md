# I02 — финальная сборка и установка текущей реализации

Самостоятельная задача полного цикла по PLAN.UIB@1 I01/P6: довести текущий
согласованный product candidate до воспроизводимого установленного комплекта.
Результат — рабочие core/web/native/combined build/verify/remove/reinstall на
текущем source94724df, matching CLI/worker/helper, актуальные docs/examples/notices,
проверенный model-free запуск из другого cwd. Не только повтор старого receipt:
I01 проверял82342f0, до Native resident session и последних Web/export changes.
Класс shipping_product при необходимых recipe corrections, иначе verification;
различить это в финальном результате. Публикация/подпись/notarization не входят.

Пользователь одобрил самостоятельные чаты: сам прочитай нормы/реализацию, зафиксируй
план, сделай необходимые packaging corrections, tests, документацию, commit+push.
После плана новое root approval не нужно. Не создавать подагентов/другие чаты.
Не менять продукт только для получения green bundle и не запускать desktop/UI.

## Основание и scope

AGENTS → specs/README → product/acceptance → CLI@16, NATIVE-SESSION@1 (поставка
entry point, не новая action реализация), BOUNDARIES/RUST-BOUNDARIES/ROADMAP,
COMPLETION@1 distribution/isolation/model-free/recovery и explicit Requires;
D01@1,D02@2,D07@5/REUSE/DEV.RUST@2/RUST. Текущие canonical core0.1/analysis0.2,
export compare0.2 и другие export0.1, private host/helper pair compatibility
сохранить. Root восстановил relevant full closure в предыдущих I01/Q01 routes;
исполнитель читает применимое полностью перед кодом, фиксирует current basis.

Inputs: distribution.py, docs/development/distribution.md/dependencies.md,
THIRD_PARTY_NOTICES.md, README.md, receipts/I01-local-distribution.md и ранее
закрытый source/safety review3075239 в export-popup-distribution-review.md.
Текущий candidate94724df включает M02/W04/W05/E03 и baselinec6b2357. Нет новой
dependency/toolchain policy; использовать locked/offline graph и установленный pin.
Не копировать вывод старой проверки вместо выполнения затронутых current checks.

Владение: distribution.py, README.md, THIRD_PARTY_NOTICES.md,
docs/development/distribution.md/dependencies.md, tests/bridges/distribution_check.py,
свой receipts/I02-current-distribution.md. Новые файлы только в существующих
каталогах при прямой необходимости; exact write set в своём плане. Product source,
CLI/host/Native/Web/schema/engine/manifests/specs защищены; реальный compile/API
bug вне recipe вернуть exact owner dependency, не чинить чужую реализацию.
Q01 независимо ведёт source/runtime/privacy приёмку и другие harnesses; не менять
его файлы и не занимать desktop. Не нужен новый packaging framework.

## Проверка и завершение

Только собственные system-temp destinations; никакой установки в home/PATH или
реальных проектов PlayPhrase.me. Все четыре supported selection на coherent saved
source, согласованность binaries/metadata, no Swift для Web/core, no browser tooling
для Native/core, model-free saved-data validation/measure/check/document/propose/
compare. Native session entry/help/feature availability проверить без input в UI;
реальный form runtime делает Q01, не дублировать. Проверить no overwrite/foreign
file preservation, verify/remove/reinstall и затронутые failure/recovery cases.
Условия/limits не ослаблять ради новой версии, unsafe deletion не добавлять.
Если actual graph/toolchain notices не менялись, проверить fingerprint и reuse
достаточную предшествующую проверку, не начинать новый universal license audit.

Результат хранить по governing output contract: raw build/log outputs transient.
Проверенные nonimage bundles удалить только своим ownership-aware manager после
потребления; images/содержащие их dirs никогда не удалять. Новые persistent dirs
не создавать. Main master и общий /tmp/ui-blueprint-master-git.lock fcntl.flock
на exact-path commit+push, без root grant. Готовая исправленная recipe может иметь
отдельный commit от product94724df — записать оба pins, не притворяться единым.
Final: рабочая процедура/комплект, изменения или отсутствие необходимости менять
recipe, actual source+recipe revisions, проверки/результат, SHA+push и remaining
limits. Final release/P7/D06 claims вне этой bounded installation задачи.
