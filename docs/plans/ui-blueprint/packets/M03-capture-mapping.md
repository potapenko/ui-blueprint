# M03-C — согласованная геометрия popup и его capture

Самостоятельный shipping_product task среднего размера, PLAN.UIB@1 P2/M03/P6.
Реализация одобрена; постановка, собственный план, код, необходимые проверки,
исправления, документация, commit+push и финальный ответ — единый цикл чата.
Не ждать root после каждого шага; не создавать подагентов или другие чаты.

## Результат

Для собственного разрешённого popup получить адресованный AX и window-isolated
capture с подтверждённым преобразованием между поддержанными пространствами,
чтобы Rust мог связать измеренную рамку с реальным кадром без догадок о scale.
Parent и popup остаются отдельными Surface с известными inclusion/exclusion и
явным partial coverage. Move/resize/close/reopen не оставляют пригодный устаревший
mapping. Никакой universal occlusion/hit/physical-pointer claim из этого не следует.

Это конечная capture/mapping часть M03: закрыть её реализацией и реальным
положительным сценарием, а не только очередным source handoff. Полный M03/P7,
Native action provider и cross-display qualification этим не объявляются.

## Основание и входы

Traversal root: AGENTS → specs/README → product/README → NATIVE@2.CONTENT,
GEOMETRY@1.CONTENT, EXCHANGE@2/MODEL/IDENTITY/PROJECTIONS/LIFECYCLE/PRIVACY@1;
acceptance/NATIVE-PILOTS@1 M03 (и explicit PILOTS/FORMS/CACHE/ACTIONS closure);
D01@1,D02@2,D03@3,D04@1,D05@4/MEMORY@2/WORK@1/Native acquisition@2/D06@1/D07@5,
PERFORMANCE/ROADMAP/RUST-BOUNDARIES/REUSE/EVIDENCE@1. RUST/DEV.RUST@2 для Rust,
действующие Apple/Computer Use/QA routes для собственных runtime операций.
Стабильные CONTENT clauses и Native acquisition METRICS/CEILINGS/ADMISSION/
OWNERSHIP/OUTCOMES/PROOF; не менять limits для pass. Полные применимые листья и
их explicit Requires прочитать до реализации. Режим Restore существующих норм.

Норма: capture metadata и sourced transforms, согласованная identity/time,
неизвестный mapping не даёт точного overlay/click. Наблюдавшийся результат:
receipts/M03-popup-attribution.md actual8507aa4 уже содержит popup AX5nodes и
ScreenCaptureKit PNG362×228, source-reviewed capture/identity, но crop_transform
unknown. receipts/M03-popup-connector.md содержит успешный close/stale/reopen;
не повторять неизменные проверки ради отчётности. Нужные изменённым mapping
сценарии повторяются по риску. Полезные owner/docs: native-helper.md,
native-acquisition.md, interface-proof-native.md, plugins/macos/README.md.

M04-T7478af3 завершил sourced fixture-local→scroll-local; receipt
M04-local-transforms.md — вход, не screen/pixel proof. P01/M05 invariance сохранять,
не запускать новый общий invariance цикл. Конкретные публичные API и mapping
выбирает исполнитель по фактам исходников/SDK и проверяет на реальном fixture;
размер PNG сам по себе не доказывает transform, нельзя вычитать guessed titlebar.
Источники разных clock domains не объявлять синхронными без установленной связи.

## Владение и защита соседей

fixtures/native; tests/bridges/native; plugins/macos; при прямой необходимости
crates/host/src/worker_native.rs,native_binding.rs,native_broker.rs и Native tests;
docs/development/native-helper.md,native-acquisition.md,fixtures-native.md,
interface-proof-native.md; собственный receipts/M03-capture-mapping.md.
Точный набор файлов уточнить в своём плане до правок. Общие engine/schema/CLI/
plugin-api/worker_ops/manifests/spec semantics защищены. Если текущий контракт
не представляет измеренный факт, вернуть конкретную зависимость вместо подмены.
Никакой второй движок анализа на Swift, private API или новые frameworks.

Текущая master, новый branch/worktree запрещён. Общий resource/Git договор из
packets/autonomous-tasks-2026-10-08.md: /tmp/ui-blueprint-master-git.lock через
fcntl.flock только на exact-path commit+push, без root grant. Native desktop lane
свободна после завершённого M04-T и закрепляется за этой задачей. Остальные чаты
не используют desktop. Не изменять/запускать реальный PlayPhrase.me или настройки
дисплеев/permissions. Отказ инструмента не обходить другим backend.

Каждое image/staging/partial image и его каталог только system temp и никогда
не удаляются; оригиналы и старые изображения не переносить/удалять. Показать
нужное изображение inline в своём чате. Для synthetic own-fixture capture применять
существующую разрешённую policy; новые реальные private pixels не собирать.
Nonimage resources/owned processes освобождать по текущему договору; не target user app.

## Готовность

Выполнены source/API reconciliation, необходимые boundary tests и реальная
цепочка own popup → addressed AX/capture → known mapping → Rust measurement
consumer при известных данных, плюс изменённые identity/resize/stale/refusal
сценарии. Положительный known gate не заменять правильным unknown. Если среда
не позволяет часть mapping, точно назвать факт, необходимые полномочия/данные
и выполненную часть; не объявлять задачу done по синтетическому substitute.
Нужны separate Surface attribution, truthful included/excluded/unresolved,
сохранные исходные данные/units, отказ stale, bounded capture и confirmed cleanup.

Финальный ответ: что работает, что проверено и на какой сборке, exact changes,
commit+push, retained image paths, ограничения и настоящие dependencies.
Не заявлять независимую review acceptance за свою проверку. Обязательная
итоговая проверка законченного блока остаётся отдельно, без промежуточных stop gates.
