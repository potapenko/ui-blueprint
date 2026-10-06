# Конечное задание исполнителю

Используется после запуска [плана](../ui-blueprint-development.md).
Пакет создаётся root только для готовой задачи; placeholders заполняются до dispatch.
Один worker не получает цель «реализуй весь проект» или весь root transcript.

## Обязательные поля

```text
packet_id / registry row:
work_classification: shipping_product | verification | diagnostic | tooling | coordination
objective: один конечный проверяемый результат
ready_because: принятые зависимости и revision
immediate_consumer: ближайшая capability/решение/приёмка
economy_basis: прямой путь; зачем отдельный контекст; когда расширять исследование
authority: исходное пользовательское разрешение + approved plan revision
execution_mode: coordinated; nested delegation: forbidden
model/reasoning: inherit
repo/current_branch/base_commit:
mode: Restore | Reconcile | Evolve | Discover | Behavior-neutral
spec_basis:
  traversal: AGENTS → registry → выбранные контракты
  complete_closure: пути, semantic revisions, stable clause IDs
  expectations/protected_behavior:
  excluded_siblings: почему не нужны
  evidence_inputs: source records/fixtures/revisions
  discrepancy/authority: подтверждённые правила отдельно от предложения
envelope: outcome; разрешённый contract delta; защищённые соседние домены
authority_mode: bounded
write_set: точные файлы/символы; нельзя писать вне набора
reuse: готовые schema/API/owners, запрещённые дубли
forbidden: новые frameworks/сервисы вне outcome, чужие проекты, branch/worktree
resource_lease: target/session/build/Git lane; owner; release condition
checks: конкретные команды/сценарии для этого изменения
acceptance: обязательные критерии; optional references отдельно
done_when: результат + checks + checkpoint commit + push + terminal receipt
waiting: точная dependency/permission/resource; не обходить границу
return: финальный ответ по формату ниже; следующую задачу не начинать
```

Worker читает полностью pinned closure до исходников и выводов. Если ревизия
изменилась, возвращает drift; не реконструирует смысл по коду/памяти.
C00 заранее выделяет короткие контракты из ТЗ с обратной mapping, чтобы каждый
worker не получал 1 400 строк общего документа. Это перенос норм, не сокращение
обязательных зависимостей. Пока такого closure нет, пакет не dispatch-ready.

## Пример задачи исследователя

R01 отвечает, как получить адресованное Web-наблюдение для B01/B03/B06 через
выбранные доступные средства. Читает pinned snapshot/ref/actionability owners
каталога ТЗ вместе с их types/tests. Возвращает конкретный source record,
кандидат интерфейса и проверяемый bounded prototype; не проектирует весь engine,
не принимает общую schema и не добавляет daemon. Немедленный потребитель — C01/W01.
Если исходник не подходит, «не брать» с объяснением — допустимый конечный результат.

## Возврат исполнителя

```text
packet_id / status: done | waiting_resource | waiting_evidence | awaiting_authority | failed
outcome / work_classification:
shipping_capability_delivered / supporting_work_delivered:
economy_basis:
spec_basis_read / specified_expectation:
observed_evidence / discrepancy_classification:
authority_used / authority_mode:
changed_paths / reused_owners / checkpoint_commit / push_result:
checks_run: command + relevant revision/environment + result
scope_check / semantic_scope_check:
deviations / residual / next_dependency:
runtime_or_visual_handoff: target/build/state/action/result/evidence or not applicable
```

`done` означает готовность конечного результата, не закрытие всей цели.
Root записывает receipt до освобождения слота и запуска зависимой задачи.

## Review packet

Проверяющий не автор и получает свежий контекст. Сначала только authority,
criteria, exact candidate revision/diff и нейтральные условия доступа/безопасности;
без verdict и объяснений автора. Он возвращает initial observations и coverage.
Затем root передаёт receipt автора для reconciliation в том же review-контексте.
Результат: accept / accept_with_residual / reject / not_verified, плюс
criterion → observed evidence → finding → repair owner → recheck.
Required criterion без evidence не превращается в residual или pass.
Reviewer не исправляет код сам; repair возвращается прежнему владельцу.
