# Mac and Web test-advisor handoff

Authority: direct user instruction to consult both original coauthors for product
and test questions. Both returned finite read-only advice; no files or runtime
changed. This receipt is coordination input for D02/W01/M01/Q03, not new product
intent, a new QA gate, or evidence that a runtime scenario passed.

## Web — Research website UI blueprint

Chat `01a1102f-e21d-7251-9597-c29a1c66d088`; site source basis
`9cec209da010df4fe20a9fc028f4be02ba8186a7`, UI Blueprint basis `6d1a1ba`.
Read PROFILE-SCENARIOS/WEB-PILOTS/FORMS/GEOMETRY/PROJECTIONS@1, D02–D05@1 and
the applicable closure. Current wire candidate is 0.1.0, not the old example 0.3-draft.

Three grounded future cases in PlayPhrase.me:
- Clip Search year draft 199 vs applied 1990–1999; URL/applied state separate from
  DOM text, debounce not success. Source: site docs/specs/discovery-playback/
  phrase-search-filters/surface-controls.md; desktop filters QA steps10–13.
- Director popup keyboard active option and outside-grid hit behavior. Same leaf
  plus mobile-source-and-response.md; desktop QA steps86–112. A DOM portal in the
  real site is not established by these behavior contracts.
- Settings reopen persistence, then mobile text-size reflow. Source: site
  docs/specs/surfaces-experience/settings/overview-and-behavior.md and
  mobile-layout/invariants-failure-route-and-qa.md; settings persistence QA.

Immediate consumer: S01-Web uses existing F01/B03 popup-open and overlay-on/off,
not a new fixture. Independent oracle: fixtures/web/expected.json, authored
separately and not read by collector/app; portal_parent=BODY, anchor=open-popup,
known hit targets and clipping. B02 and B04 cover later affected draft/reflow cases.
No new mandatory schema fields found. Do not run historical combined run.cjs as
current acceptance (it contains retired polling experiments), or count post-hoc
max-output truncation as bounded acquisition. No site code/runtime work dispatched.

## Mac — Спроектировать UI Blueprint

Chat `01a1102f-791c-7e91-bec3-1877ea004d51`; current source contracts:
- native-platform-composition/iphone-reels-aligned-column.md@5;
- native-reels/discovery-custom-search-and-phrase-chains.md@2;
- native-platform-composition/iphone-reels-summary.md@1;
- native-reels/translation.md r2;
- native-shared-services/translation-language-catalog.md@2.
Paths above are under PlayPhrase.me Mac docs/specs/features.

RC05 remaining evidence: long source metadata/translation and trailing-selector
reachability. Proposed bounded setup for a later packet: own fresh Phone, ordinary
guest continuation, Reels Search to a real long suggestion/existing corpus phrase,
pin first paused result, explicitly choose another available translation language,
compare ordinary/accessibility and reach/open/close trailing selectors without
changing values. Missing data remains open. This proposal has not been executed;
deliberate initial selection/language change needs the finite packet's scope.

Immediate S01-Native/M01 context from existing Mac examples:
- RC01 Settings is in-window; dialog text does not prove another NSWindow. Labels
  do not identify Close controls; crop ROI is not layout bounds.
- RC02 popup AX→CG link is inferred; anchor unmeasured. Genre AX role is button,
  visual checkbox does not establish checked. Director omitted value is unknown.
- F03a parent capture included popup, isolated F02 capture excluded it; missing
  popup pixels do not prove no Surface. Preserve independent channel coverage.

Inputs/expected stay separate in fixtures/real-world/mac-settings and mac-filters.
Existing F02 reads AXPosition/AXSize and capture metadata, but transfer to the real
app is unverified. Exact AX↔Surface↔pixel mapping, anchor/layout/hit/paint bounds,
occlusion and internal layout remain unknown where unmeasured; an external
collector may still need opt-in probe data. Simulator snapshot_ui is only a
candidate, not measured F03c bounds evidence. F02 capture-concurrency gate stays open.

Both platform owners received these bounded implications. Advisor chats remain
available for the user-designated continuing product/test role; they are not
running jobs merely because their historical context remains useful.

## Renewed product priority — 2026-10-07

After the user's direct criticism, root asked both named advisors for concrete
next useful scenarios. Both returned read-only handoffs, no new runtime or source
changes. Both select **Director popover in Clip Search first, Settings second**.
This prioritizes already-approved B03/M03/form/projection work; it does not expand
the goal or authorize arbitrary source-app changes. Advice is distinct from
runtime proof. W03 infrastructure is an actual dependency only if source evidence
shows it is needed for the next user capability, not a reason to defer that case.

Mac consumer: “Where do I type the director name and what is currently open?”
Existing [RC02 data](../../../../fixtures/real-world/mac-filters/observations.json)
and [independent expected](../../../../fixtures/real-world/mac-filters/expected-answer.md)
support one Director name field, Type a director placeholder, focused/settable
state and disabled Clear; Search by director name is explanatory text. Results
and exact anchor gap remain unobserved. Genre's AX button role without checked
does not prove false. Require sourced Target/Surface/time/anchor and measured
compatible bounds before geometry; parent pixels may omit a popup.

Web consumer: “Where is the popup field, which suggestion is active, and how is
it placed relative to its trigger and selected neighboring content?” Minimal
scope: trigger/popover/input/few visible options, one explicit nearby card only
as permitted context. Need roles/labels/expanded/focus/active_descendant/selected/
draft, known layout bounds/space/coverage and sourced relation. Applied filter is
separately observed through profile/URL; draft or active option never proves it.
Unknown hit testing does not block a truthful first answer, but rectangles alone
cannot prove hover accessibility. Advisor's example8/6css_px is illustrative,
NOT a measurement or expected value. Existing F01 B03 literals test mechanism;
they never define actual PlayPhrase.me dimensions or prove its DOM uses a portal.

Web source context supplied by advisor: playphraseme-site's surface-controls.md,
mobile-source-and-response.md, desktop-filter QA steps86–112 under the existing
phrase-search-filters route; PROFILE W02/W05 and B02/B03. Settings uses existing
settings/overview-and-behavior.md and persistence QA, PROFILE W07/W12. Root did
not operate that other project or treat source references as fresh UI evidence.

Settings follow-up distinguishes visible label/value, label→control and disabled
background/Close targets. Mac [RC01 expected](../../../../fixtures/real-world/mac-settings/expected-answer.md)
does not establish persistence or inner spacing. iPad filters/iPhone Reels remain
existing reference data, not mobile implementation authority. Immediate next
owners consume this handoff for useful scoped inspect/measure, not another audit.

### Known dimensions with unknown overall consistency

Web coauthor was asked about the actual four CLI unstable_state results. Read-only
answer: MODEL separates known property availability from stable/unstable/unknown
Observation consistency; GEOMETRY scopes stabilization; EXCHANGE promises no
global atomic snapshot; ANALYSIS-VALIDATION explicitly rejects unstable source.
Known width/height of one recorded known rect therefore need not be suppressed
solely for unknown overall consistency. Preserve that unknown and all original
evidence; it is not a claim that current UI is stable or final verification passed.
The advisor cautioned that no universal unknown-consistency operation table exists;
Core separately checked protected G01/accepted-analysis clauses and tests before
aligning the actual engine/schema guards. This records reconciliation of existing
meaning, not a new blanket permission for incoherent comparisons.

### Director selectors and addressed-root boundary

Web advisor read site source1954c9cd6c11464cd9529699552178854764e0d1 without
runtime/changes. Actual IDs: clip-search-filter-director-wrap, -trigger, -input,
-options (same prefix); popup has data-testid only and is a child of that wrapper,
not BODY portal or iframe. Options have source-generated IDs from canonical value.
Trigger has expanded/haspopup but no aria-controls to popup; input controls options.
aria-selected represents keyboard-active option; aria-pressed represents applied
choice in this source. No site dimensions or live availability follow from it.

Advisor recommends the already-supported conceptual Scope by observed ref: select
the actual wrapper/trigger via browser/DevTools picker, establish same target/frame/
document binding, then bounded child collection. ID/name/source structure checks
that chosen node; it does not prove global unique ID. A click is not an inspection
selection. Transfer of an opaque token from another browser backend into this
collector's CDP session is not demonstrated and must not be assumed. No executable
trusted profile or live selection mechanism was established by consultation.
Web's private rooted read-only seed connection is implementation work; actual
caller-origin binding and real Director execution remain separate qualification.

### Director action-state-result handoff, 2026-10-07

Consumer: future W02/B02 and M02 form verification after the primitive action
provider. Both original product advisors answered read-only; no runtime/action or
source application changes were performed. Their evidence does not grant a live
target, substitute actual field observations, or change the generic engine contract.

Web advisor source1954c9cd6c11464cd9529699552178854764e0d1:
draft query and keyboard-active option are distinct from applied membership.
Click/Enter toggles the chosen Director and closes the popup; there is no Apply.
For a positive add case choose a value absent from the original applied set and
preserve other filters. Actual selected tag plus decoded filters.director URL,
and aria-pressed on reopening, establish client filter membership; aria-selected
only means keyboard-active. Escape before selection closes without changing the
applied set; draft reset is not the acceptance condition. After possible delivery,
do not retry Enter: a second toggle could undo the first. Reobserve actual state.
Popup closure alone proves neither selection nor server search completion.
Sources attributed by the advisor: suggestions/view.cljs lines867/1056,
clip_search/ctrl.cljs lines328/397, filters/picker_view.cljs line405 under the
existing PlayPhrase.me site source. Root did not inspect that other project.

Mac advisor expectation is different: under macOS filter-popover@1/source r2,
activating a real Director applies it immediately, leaves the popup OPEN, shows
selected/checkmark and rail membership, and preserves existing multi-selection.
Escape dismisses without undoing the applied value; reopen verifies membership.
Search query preservation/reset is not established by this context. RC02 actually
proved only open and Escape, not typing/results/selection/application/persistence.
Sources supplied by advisor: native-clip-search/2026-08-26-macos-native-filter-popover-evolution.md
and native-clip-search/user-visible-behavior-part-2.md in PlayPhrase.me Mac docs;
local [RC02 observations](../../../../fixtures/real-world/mac-filters/observations.json)
and [expected](../../../../fixtures/real-world/mac-filters/expected-answer.md).
No popup geometry, AX-to-CG ownership, pointer mapping or server completion follows.

The platform-specific close/stay-open behavior is intentional context, not a
generic executor rule. This prevents inventing shared Apply/Cancel semantics.

## Mac own-fixture Snapshot setup — 2026-10-08

Same Mac advisor, finite read-only response after current failed shortcut setup
e698dd7. Source read: existing Fixture.swift f02.snapshot increments snapshotRequest
and off-build publish(), without explicit popup dismissal. Confirm/Compare-reset
do dismiss and are unsuitable. Parent-owned shortcut route does not establish
why popup vanished; foreground/key routing remains unknown.

Proposed smallest alternate: fresh exact own-app AX after popup open; if existing
f02.snapshot is actually present/enabled, semantic click its fresh index, no
coordinates/Compare/parent Raise. Require snapshot_request increase plus fresh AX
popup and manifest popup=true/current binding before Observe. This is setup, not
physical-hit/product delivery proof. If unavailable/closing, no guaranteed workaround;
fresh routing evidence required, not blind shortcut repetition. A new popup-local
Snapshot button would be a separate fixture change, not assumed existing behavior.
Advisor ran no app or edits. Root activated one alternate bounded attempt using
this evidence; actual outcome pending, source expectation is not runtime acceptance.

## Geometry-first cases from existing advisor context — 2026-10-08

Mac advisor nominates RC03 paused Search & Learn resize: left suggestions, central
video/transport, right learner panel and bottom query strip. Existing current-repo
fixtures/real-world/mac-resize/{observations.json,README.md,expected-answer.md}.
Actual outer windows1920x1050 and961x1050pt, origin0,30,2xPNG, same query/clip/paused
state. Inner content/panels/tab/text/clipping bounds are NOT measured. Desired question:
where does width run out, which container to change? Need attributed panel widths/
gaps/insets/transport alignment and before-after, not window-as-content substitution.

Web advisor nominates already-open Director popover in desktop Clip Search; wrapper
#clip-search-filter-director-wrap contains trigger/popup (not portal in inspected
source), input/listbox/visibleoptions. Need same CSSpx-space bounds, trigger→popup
gap, wrapper/trigger left offsets, input insets, input→list gap, option-left spread,
viewport overflow/known clipping. Code declares top100%+8px,left0,width258px,
max-width100vw-32px,padding10px; these are source declarations, NOT measured numbers
or universal expectations. Existing F01 B03/B04 oracle values belong only to F01.
Source handoff refers to playphraseme-site picker_view.cljs127/468, suggestions/
view.cljs1054 and qa/cases/regression/tc-clip-search-desktop-filters-and-suggestions-panel.md.
No new runtime/source edits by either advisor; no actual Director inner measurements.
Both cases are consumers for geometry utility, not an action-executor prerequisite.

## M05 public external-AX to owned-window bridge — 2026-10-08

Mac coauthor01a1102f-791c returned a read-only source answer after the in-process
SwiftUI sample traversal gap. No runtime/files/real PlayPhrase.me operation.
For an already identity-bound own window and fresh AX rectangle, candidate centre
pAX=(x+w/2,y+h/2) maps to AppKit screen (pAX.x,H-pAX.y), with
H=NSScreen.screens[0].frame.maxY, then exactWindow.convertPoint(fromScreen:).
Use primary/menu-bar screen, not NSScreen.main/current window screen/visibleFrame;
no Retina multiplier, titlebar correction or screenshot-derived scale.

Primary basis: [AXPosition](https://developer.apple.com/documentation/applicationservices/kaxpositionattribute),
[NSScreen.screens](https://developer.apple.com/documentation/appkit/nsscreen/screens),
[screen conversion](https://developer.apple.com/documentation/appkit/nswindow/convertpoint%28fromscreen%3A%29),
[event location](https://developer.apple.com/documentation/appkit/nsevent/locationinwindow).
This is a source-based coordinate proposal, not a hit proof. Require actual sample
membership in exact A, unchanged process/window generation/frame and display config
through dispatch. Round-trip arithmetic and AX rectangle containment do not prove
hit region; inside/outside Count outcomes remain necessary. No external CUA/global
WindowServer/physical-device claim. Root selected minimal trusted frame input to
existing own test seam, with Native source reconciliation and one changed pair.

## Q03 самостоятельные geometry/diff кейсы — 2026-10-08

Оба автора ответили read-only из уже имеющегося контекста, без новых запусков,
изменений или captures. Это выбор полезных вопросов и источников, не новая
runtime-приёмка и не разрешение менять/запускать реальные проекты PlayPhrase.me.
Mac: чат01a1102f-791c-7e91-bec3-1877ea004d51, turn01a11b4f-97f1-7ae0-9da6-a462dc8dec99.
Web: чат01a1102f-e21d-7251-9597-c29a1c66d088, turn01a11b4f-9d16-7212-a8b7-280e16ee287c.

| Кейс | Вопрос для агента / нужный результат | Основание и ограничения |
| --- | --- | --- |
| Mac RC03 Search resize, первый выбор | При сужении где перестают помещаться learner tabs? Bounds панелей/видео/transport/tab strip, gaps, оси и containment в pt; одинаковые query/clip/paused state | fixtures/real-world/mac-resize содержит пару внешних окон1920×1050/961×1050pt. Внутренние bounds/text extent/hit ещё unknown; наружное окно не подменяет content viewport |
| Mac RC02 Director | Остался ли popup рядом с trigger и помещаются ли input/list? Сourced anchored_to, отдельные Surface bounds/containment в pt | mac-filters содержит idle состояние, без ввода/results. AX↔CG исторически было inferred. Right-placement — предпочтение, допустимый system fallback не дефект; точные anchor/insets неизвестны |
| Mac RC01 Settings | После изменения строки сохранились axes labels/controls, widths/gaps и доступность Close? | mac-settings: семантика10controls и вложенная панель. Merged AX не раскрывает внутренние части; crop не равен dialog bounds. До/после правки ещё отсутствует |
| Web Director, первый выбор | Выровнены ли trigger/popup/input/list/options, какие gaps/insets/overflow изменились? Отделить перенос блока от внутреннего layout | Existing filter contract и tc-clip-search-desktop-filters-and-suggestions-panel; CSS только declaration. Полная occlusion/pointer доступность unknown. Не использовать спорный tag ellipsis как oracle |
| Web mobile Settings, дополнительный будущий профиль | Длинный текст не сжимает switch/select? Пересечения, scroll и viewport отдельно от layout | mobile-layout contracts и tc-mobile-settings-fixed-toggle-and-select-widths. Native content-size Simulator Safari нужен только для окончательного мобильного claim; этот случай не добавляет iOS implementation к цели. Положительный rect не доказывает читаемость |
| Web desktop Settings | При resize выровнены controls, локальны ли пояснения и сохранён ли порядок строк/значения? | settings/overview-and-behavior и tc-settings-modal-controls-persist. Отступ без принятого дизайна — факт, не fail; снимок не доказывает persistence |

Все пары сравнивать при явно известных state/environment/units и отдельных
наблюдениях до/после. Исторические numbers и F01 oracle не переносить на настоящий
сайт. Предпочтительные первые consumers Q03: Mac RC03 и Web Director. Runtime
разрешение и наличие нужных наблюдений проверяются перед Q03 отдельно.
