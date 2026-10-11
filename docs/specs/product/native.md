# Нативное наблюдение и capture
- Node type: leaf; domain: `uib.native`.
- Contract: `UIB.NATIVE@4`; stable clauses: `UIB.NATIVE.CONTENT`, `UIB.NATIVE.AX-READ`, `UIB.NATIVE.AX-POPOVER`, `UIB.NATIVE.AX-VISIBLE`; supersedes @3 additively.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: Mac AX, capture, оконное matching, probe.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.EXCHANGE@1](exchange.md), [UIB.GEOMETRY@1](geometry.md), [UIB.PROJECTIONS@1](projections.md), [UIB.LIFECYCLE@1](lifecycle.md).
- Source mapping: TZ 76–105; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.
<a id="uib-native-content"></a>
## Нативное наблюдение и изображения
Этот раздел разработан Mac-соавтором и согласуется с общим контрактом данных. Нативный плагин должен выдерживать неполную семантику, несколько поверхностей и потенциально медленные системные запросы.
Три независимых канала:
- external_semantics: доступные AX-атрибуты и действия;
- rendered_capture: изображение выбранного окна или композиции дисплея;
- opt_in_layout_probe: измерения после layout из собственного debug-приложения.
У каждого канала свои capabilities и разрешения. Отсутствие доступа к пикселям не отменяет доступный AX-ответ, а недоступный AX не запрещает разрешённый crop. Режим помечается degraded/partial, без объявления всей платформы unsupported. Read-only запрос не открывает системный диалог разрешений автоматически: при нехватке прав возвращается permission_required. Запрос разрешения — отдельный явный шаг допустимого сценария.
Capability уточняется на уровнях Target, Surface, Element и property. Возможность записи проверяется для конкретного атрибута перед setter; допустимость нового значения проверяется отдельно; действия берутся из доступного списка, а не выводятся только из role. Сбор использует перечень нужных атрибутов, пакетные чтения там, где доступны, ограниченный обход children и deadline. Ошибка одного атрибута не обнуляет другие. Один зависший Target не должен блокировать независимые сессии.
Оконная идентичность и принадлежность AX-данных проверяются отдельно. Если точное окно не сопоставлено, application_partial допустим только как явно более широкий диагностический результат с requested_surface_unresolved; он не выдаётся за снимок запрошенного окна и не содержит пригодных к мутации refs. Application scope должен быть уже разрешён задачей. Если разрешено только одно окно, ошибка matching возвращает target_unresolved и минимальные кандидаты без содержимого других окон. Нельзя расширять window-only scope ради удобного чтения. Ответ отдельно хранит requested_target, resolved_scope и unresolved_surfaces.
Popup, меню и системный ввод могут принадлежать отдельным Surface или процессам. native_owner/process identity хранится отдельно от initiated_by/anchored_to. Их owner/relationship должны иметь источник. Геометрическое пересечение не устанавливает владение. Включение системной поверхности требует явной адресации и допустимости по policy. external_surface_detected — диагностическое событие, не безусловный отказ: известная разрешённая поверхность может включаться штатно. Невозможность установить её цель или связь возвращает target_unresolved и останавливает зависимое действие, сохраняя исходную сессию; нехватка прав отражается отдельно. Встроенный SDK не экспортирует выдуманную структуру системного диалога.
Изображение содержит capture_kind: window_isolated или display_composited, фактический capture_target, pixel_buffer_size, crop_transform, время capture и доступные content-rect/scale сведения. Изолированный захват окна не доказывает видимость его содержимого на перекрытом рабочем столе. Frame также содержит included_surface_ids, excluded_surface_ids и unresolved_surface_ids: отсутствие попапа в кадре не означает отсутствие попапа в UI. Parent-window capture не считается автоматически содержащим дочерние popup-окна. При resize нельзя выводить scale только из размера pixel buffer: используется доступный frame-to-window transform и frame metadata. Для утверждений об окклюзии нужен соответствующий источник. Аудиозахват выключен по умолчанию и не включается вместе с UI capture. Capture_filter и применённые исключения сохраняются явно: display_composited с отфильтрованными окнами не считается неизменённым видом рабочего стола. Списки Surface перечисляют известное покрытие; их полнота имеет отдельный статус. Пустой список при неполном знании не доказывает отсутствия других окон или перекрытий.
Основание различия: Apple описывает desktop-independent single-window capture, которое может включать перекрытое/внеэкранное содержимое, исключать child/pop-up windows и масштабировать контент в заданный output buffer. [ScreenCaptureKit, WWDC22](https://developer.apple.com/videos/play/wwdc2022/10155/).
Связь AX bounds с capture frame проверяется по Target/Surface generation, времени и преобразованиям. При неизвестном mapping не выдаются точный overlay и координатный click. Несколько мониторов, разные scale contexts и перенос окна не сводятся к одному DPR. Отсутствие image mapping не запрещает отдельно разрешённое семантическое AX-действие по однозначно установленной актуальной native-цели; такой результат не используется как проверка попадания указателя.
Opt-in probe сообщает измеренный output, а не значение modifier из исходника. Связь с файлом и измеренные bounds — разные поля. Probe не меняет accessibility, focus, hit area, layout или продуктовые данные. Приёмка проверяет это сравнением instrumented/uninstrumented fixture; private API не становятся неоговорённым основанием. SDK-кадр не заменяет внешнее наблюдение канонической сборки.
Для обычной проверки нужен именно размер/состояние проверяемого сценария. Нет общего требования максимального окна, физического монитора или 2x. Поддержанный виртуальный дисплей пригоден там, где сценарий не требует конкретного физического свойства. Изменять конфигурацию дисплеев только ради удобного захвата не требуется.

## UIB.NATIVE.AX-READ — explicit ordinary-window geometry
Root selected G03 ordinary-window extension under approved PLAN.UIB@1/user geometry
priority. Existing CONTENT unchanged. `native_ax` is read-only external_semantics:
trusted operator names PID/bundle/process launch time and selector AXFocusedWindow.
Helper reads that public AX window without activating/focusing or searching by title,
rectangle/order. Hold the selected object only during this Observe; check process
incarnation/AX owner and same focused-window object before/after collection/publication.
Missing permission returns permission_required without prompt; changed/unresolved window
refuses. No F02 identity/probe file required. Native fixture provider remains unchanged.
Private config collection="focused-ax", process={pid,bundle_id,launch_time}, scope_id,
acquisition_limits. Request Target is macos-pid-PID with caller generation tied to
trusted process incarnation; Surface id="ax-focused-REQUEST_ID", generation=REQUEST_ID.
Surface is observation-scoped public AX binding, not CGWindowID or stable cross-request/
action identity. No capture/probe/action/ref authority from this path. Report actual
AX properties/bounds and known process owner, partial coverage and unknown transforms.
AX accessibility_bounds remain ax-screen/screen/pt/top_left; hidden layout/hit/paint
not inferred. Existing canonical0.1 shape/limits/privacy/deadlines/cleanup preserved.
Optional own measured probe is independent; ordinary windows gain no fabricated layout.
## UIB.NATIVE.AX-POPOVER — explicitly selected focused popup
`CA3-NATIVE-POPOVER-001`, Evolve: root's explicit shipping packet under the user's
CORPUS-1000 repair mandate, after accepted [CA2](../../plans/ui-blueprint/receipts/corpus-native-acquisition-gap.md), authorizes this narrow opt-in. Existing CONTENT/AX-READ and default collection remain unchanged.
Same native_ax config keys/provider; collection="focused-popover-ax" explicitly
selects the focused element's nearest AXPopover (including itself) inside the
held AXFocusedWindow. No arbitrary locator, other-window search or implicit fallback.
Resolve at most16 chain nodes through public AXFocusedUIElement/AXParent. Verify
each object's process owner BEFORE reading its role; read only structural roles
until ancestry reaches the exact retained window. Foreign owner, another window,
missing/cyclic/no-popover ancestry refuses target_unresolved; no content is acquired.
Each focus/parent selection read reserves one entry from the existing child_entries
allowance, separately counted from returned child entries. Repeated validation is
charged again; role copies use existing copied-value budgets. This only tightens
acquisition, adds no private config fields and preserves all Native/D05 ceilings.
Retain the complete chain/root for this Observe. Revalidate process incarnation,
focused-window/focused-element objects and every parent-chain object/root before
collection, after collection and after encoding before publication. Changes refuse
stale_target; resource exhaustion uses incomplete_scope, deadline/permission keep
existing meanings. No retry, focus change, actions, pixels or probe authority.
Reuse the existing bounded collector from the selected popup; max_elements≤160,
max_depth≤9 now bound that subtree. Parent window remains the single observation-
scoped Surface; no new popup Surface/anchor ownership, stable refs or CG mapping.
AX bounds remain ax-screen/screen/pt/top_left, capture transform unknown, coverage
honestly partial. Default focused-ax retains its exact behavior and wire versions.
Required proof: unchanged default, real Genre options/Director controls, bounded
entry/text/output limits, missing/foreign/cyclic/changed focus/window/ancestry/root,
no prebinding content reads and canonical validation. Independent review remains
separate; CA2 is binding evidence, not shipped geometry or whole-platform acceptance.
## UIB.NATIVE.AX-VISIBLE — explicit source-visible collection children
`CA5-NATIVE-VISIBLE-001`, Evolve: root selected this bounded repair under CORPUS-1000,
conditional on actual source support. [CA5 feasibility](../../plans/ui-blueprint/receipts/corpus-native-visible-children.md) proves the scrolled grid's public AXVisibleChildren; registration precedes implementation and is not acceptance.
Same native_ax config keys/4032-byte envelope; collection="focused-visible-ax" opts
into source-visible collection child lists inside the held AXFocusedWindow. Existing
focused-ax/focused-popover-ax, canonical0.1/analysis0.2 and all ceilings stay unchanged.
Applicable source roles: AXList/AXTable/AXOutline/AXGrid, or AXOpaqueProviderGroup
with reported AXOpaqueProviderList/AXOpaqueProviderGrid subrole. Public standard roles
and the witnessed opaque source roles establish this classification, never UI names.
Structural ancestors/other nodes retain AXChildren. Missing role or an unclassified
opaque group cannot establish that fallback: mark its child selection unknown.
For an applicable collection, read AXVisibleChildren via existing bounded count/page/
copy admission BEFORE enqueuing. Unsupported/noValue/error/type refusal is unknown,
never empty success or AXChildren fallback. A successful count0 reports only a
source-empty list, not absent pixels. Budget-limited lists remain partial/unknown.
Record requested source through macos.ax AXChildrenSource extension (Description
property, reported bounded_public_ax_visible_children evidence): known Text
AXVisibleChildren on a successful read, otherwise unknown with bounded reason.
Unclassified opaque/missing-role nodes use unknown child-source evidence too.
Existing actual entry/text/shape/deadline/output accounting applies; no larger node
or depth budget. A subset does not remove hidden nodes or create action authority.
Keep one parent-window Surface, actual AX pt bounds and unknown pixel transform;
no inferred visible_region/paint/occlusion or complete-view claim. The accepted
held-window/frame publication owner and validator remain through real descriptor IO.
Required proof: unchanged old mode; reported visible subset under same quotas;
unsupported/error/type/budget/window-change refusal; one actual scrolled-grid pair
with independent image and attributed partial scope. Independent review is separate.
