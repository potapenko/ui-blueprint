# Native source advisor handoffs

## CN12–CN14 — recovery choices and isolation boundary, 2026-10-13

CN12 did not recover the original phraseID from retained observations or passive
AX metadata. Current controls expose text/source/counter, not that ID. Normal lookup
for position121 is beyond the free batch and was not used without an authorized
client/credential route. No credential extraction or gate bypass. Copy Phrase copies
text, Copy Source copies source text, and internal copySourceLink is an external
source URL; no wired current-phrase-link action was found. SearchLinkCodec capability
alone is not a user command. No corresponding Release selection/restore log producer
was found; DEBUG CompositionTrace requires flags and was not enabled. Old CN-B19
stdout logs were absent; no broad system log search followed.

CN13 source-only check: ordinary Debug is production composition. Existing
--playphrase-scenario/PLAYPHRASE_SCENARIO choose fixtures with normally disabled
restoration and fixed/no-op prefs, but replace live services and retain shared window
persistence. Preview fixture selection does not protect @main bootstrap, which has
no XCODE_RUNNING_FOR_PREVIEWS guard. The scene always attaches
[window persistence](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/Platform/LearnerWindowFrameRestoration.swift:60).
DEBUG_SESSION_READ_ONLY protects only debug session bookkeeping. No existing live
isolated bootstrap found; UserDefaultsDataStore suiteName is not wired to launch
configuration. MemoryDataStore is a private test actor; MemoryDownloadVideoOptionsStore
is Download-only, and a general MemoryOptionsStore symbol was not found.

CN13's proposed source changes are NOT approved or implemented: explicit Debug-only
isolated entry; one memory store for relevant repositories, exclusion of production
credentials/library/statistics/analytics/shared caches; disable shared window saves;
protect app-target Preview bootstrap. Public lookup clients could remain real.
No new targets/bundle IDs/harness required by that proposal. Source app edits need
separate authority; neither the running goal nor this record authorizes them.

CN14 source-only lifecycle plan has no save-only action (standard Save is Download).
Activation has no direct Play command, but access restoration may update credentials,
account/bootstrap or Search. Deactivation pauses, may update translation, writes the
full restoration snapshot, checkpoints learner continuation and flushes pending
statistics/practice. This is NOT a two-field patch or no-network operation. An async
user choice is pending: leave data unchanged, or authorize one normal full live-session
save with passive identity/hash/state prechecks and postchecks, no automatic repeat.
No operation has been authorized by silence. The advisor's proposed return to Codex
is NOT adopted as automation: the user forbids Codex Computer Use. Any approved
deactivation must use an ordinary non-Codex surface or the user's own interaction.
Recovery snapshot remains private and unchanged; ordinary Debug still prohibited.

## CN11 — current saved/live discrepancy, 2026-10-13

Advisor turn01a13245-ca5e-7aa3-b157-86e720f5fd7e performed only authorized exact-key
and passive targeted AX reads. ReleasePID63105 original Oct11 03:29:03 launch and
executable revalidated; actual bundle playphraseme.Playphraseme, sandbox signing
identity matched the shared store route. Window stayed inactive, no screenshot/UI
dump, focus, clicks, writes or restart. Relevant container was identified from that
process's open paths; no broad preferences inspection.

The stored version1 snapshot is searchAndLearn, search.activeIndex4 and
activePhraseID67503d06a9e09c213cf104e1. Query matches known CN-B19 identity, English/
General/Grammar-off. Passive Release still shows original Tarzan121/426 and Play
toggle. Key bytes unchanged on repeated read. This matches Debug's final index5
and differs from live original selection, but absence of pre-run bytes prevents
proving the last writer or reconstructing all previous stored fields.

Only this key was backed up:633bytes, file0600/directory0700, SHA256
`9dd5b4c3328c50c11d435c5b963be7b306dd2114dbae0062ad911bc577d93219`.
Correct private recovery path:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CN11-recovery-q6ahofml/restoration-key.data`.
Root/user are retention owners until incident resolution; contents are not printed
or committed. The advisor's first hyperlink had a temp-prefix typo; only this
corrected exact path is used.

No rollback was performed. Proposed minimal correction would be activeIndex4→120
and exact original phraseID, preserving every other field and checking current key
hash immediately before any future write. The original ID is still unknown;121
alone is not enough, and a full Release checkpoint can overwrite sibling payloads.
CN12 now seeks exact identity from retained/passive evidence or a bounded supported
read-only application lookup. It cannot write or operate Release. Ordinary Debug
and app-target Preview remain subject to whole-bootstrap storage-isolation proof.

## CN10 — ordinary Debug shares restoration storage, 2026-10-13

Advisor01a1102f-791c-7e91-bec3-1877ea004d51, turn01a13237-b8ac-7f63-b3c8-5978130fc83e,
read source c7d8e3d/beta-01 and CN-B19 receipts only. Selected source-feed →
query-continuity@1, Search navigation@3/data contracts and persistence-registry@1.
No prefs, processes or UI were read/changed in that consultation.

The observed Reels→Search change is not only session-local by implementation:
[handoff](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayPhraseLearner/Application/AppModel/Search/AppModelSearchDestinationHandoff.swift:66)
saves Search restoration with activePhraseID and activeIndex. Destination switching
saves the entire NativeRestorationState, including Search/Clip/Reels, through
[AppCommands](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayPhraseLearner/Application/Commands/AppCommands.swift:311).
Source Debug and Release both use bundleID playphraseme.Playphraseme and the same
sandbox route; ordinary Debug has production composition. [Store injection](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/AppCompositionLearnerContinuation.swift:4)
uses [UserDefaultsDataStore](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/Packages/PlayPhrasePersistence/Sources/PlayPhrasePersistence/UserDefaultsDataStore.swift:7),
UserDefaults.standard, namespace PlayPhrase, key PlayPhrase.native-restoration-v1.

Thus Debug could overwrite the shared snapshot, including sibling state different
from the current Release. Serialization is only per repository instance. A later
Release checkpoint could overwrite it again. Actual signing/build overrides, exact
current stored value and whether the write completed are NOT established by source.
Save errors use try?. Stopping Debug does not roll back storage. Source's cached
lastRestoration and lack of an observed cross-process apply listener do not prove
the live Release UI changed automatically. The reported custom-search handoff does
not establish a catalog-offset/preferences/account/server mutation.

CN11 now performs narrowly authorized read-only comparison of the exact restoration
key and passively available Release identity/state, without activation or writes.
No ordinary Debug relaunch or blind snapshot reconstruction is allowed. Bare index121
is insufficient to reconstruct the original phrase. Any proposed correction needs
exact identity and must account for full-snapshot writes and concurrent Release state.
No user preference contents are published or committed; a minimal private recovery
copy of only this key may be retained in system temp for root/user until resolution.

Future Mac runtime must prove storage isolation for the entire bootstrap, not merely
for a view's MemoryOptionsStore. CN8 received this dependency before any Preview run.
Current checks do not authorize broad preferences, credentials or memory inspection.

## CN9 — transient lookup recipes, 2026-10-12

Mac advisor01a1102f-791c-7e91-bec3-1877ea004d51 completed source-only turns
01a131e8-2e83-7bc0-86dd-548503984b7d and01a131eb-8cd9-7cb3-be08-e164d17f9374,
c7d8e3d/beta-01. No runtime, requests or changes. Source route: Actor
scope-selection@11, Clip popovers Mac@1/iPad@2, Reels discovery@2,
native-ui-tools@1. Read-only callbacks supply restoration evidence; query strings
and candidate states below are QA proposals, not promises of actual results.

| Surface | Proposed temporary input | Required restoration |
| --- | --- | --- |
| Mac Actor | paused English Search, no Actor; tom → tom hardy → zzqvxxzzqvxx | Close clears picker query/candidates, cancels task and changes generation; reopen to verify fresh ready state |
| iPad Actor | same preconditions, landscape anchored popover | neutral tap-away dismiss; onDisappear closes model; verify query/scope/pause |
| iPhone Actor | same preconditions; More → Actors pushed view | Back closes picker; dismiss remaining More if needed, verify previous Search |
| Mac Director | paused Clip, initial empty draft; steven → zzqvxxzzqvxx | select text/Backspace to empty, wait under-threshold with no options, Escape |
| iPad Director | same initial empty draft, landscape Clip | restore empty and wait; visible Close only dismisses; no toolbar Clear/Return |
| Mac Reels Search | paused active item, new empty search session; how are → zzqvxxzzqvxx | restore empty, Close, wait flip, verify source/item/phrase/pause |

Actor input is isolated picker state, debounce300ms, minimum3normalized characters;
empty asks for ready list. Lookup reads public actor data. Close clears/cancels and
invalidates late responses. Do not select option/possible-matches or Clear, which
would change Actor scope. Refresh retains previous rows until the new result: wait
for the actual response, not just elapsed debounce.
Director editor debounces250ms, threshold2, at most10options. Empty input cancels,
bumps generation, clears options/active index and issues no lookup; stale responses
fail identity/cancellation checks. iPad's separate in-memory draft is bound to that
editor and restored too. Initial nonempty draft is excluded from these bounded
recipes. Plain dismiss alone is insufficient. Filter selection/clear writes are
separate operations and forbidden here. Reels search has a separate draft/session,
250ms read-only relatedPhrases lookup; empty cancels and changes identity, Close
clears results/phase and disables draft effects. User restoration projects source
and current item, not search draft. Internal generation counters are not rolled back.
Phone Clip Filters remains excluded because disappear applies drafts. No duplicate
Cast/Voice families added just to fill a quota. Names alone are not case novelty;
actual populated/narrowed/empty/overflow composition must differ from the corpus.

Source owners: [Actor model](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayPhraseLearner/Features/SearchLearn/Model/SearchControls/ActorPickerModel.swift:37),
[Actor state](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/Packages/PlayPhraseDomain/Sources/PlayPhraseDomain/Search/ActorPickerState.swift:146),
[Mac dismissal](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/MacApplicationRootView.swift:424),
[Mac view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ActorSelect/View/ActorPickerView.swift:44),
[iPad presentation](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Search/Actor/TabletActorPickerSearchOverlay.swift:44),
[phone callbacks](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Search/ActorSelectionView.swift:49).
Director: [editor](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayPhraseLearner/Features/ClipSearch/Model/ClipSearchFilterEditorModel.swift:213),
[iPad draft](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/ClipSearch/Model/TabletClipSearchFilterQueryDrafts.swift:4),
[binding](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/ClipSearch/Composition/TabletClipSearchFeatureComposition.swift:90).
Reels: [close](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/Features/Reels/Components/BacksideClose/ReelsBacksideView.swift:104),
[restoration projection](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayPhraseLearner/Features/Reels/Model/ReelsCatalogRouteProjection.swift:37).
CN-B19 consumes the Mac subset; CN-B20 consumes the phone/tablet subset. Standard
runtime/source qualification, isolated ownership and explicit restore still apply.

## CN-B17 — keyboard runtime qualification, 2026-10-12

The later authorized runtime packet resolves CN5B's scoped-control uncertainty
for the exact owned iPhone17e and iPadPro13M5 on iOS/iPadOS27.0 build24A434.
DeviceHub com.apple.dt.Devices → unique owned device/window → Device → Keyboard →
Toggle Software Keyboard was observed enabled and proved show/hide on all5surfaces.
No hardware-keyboard preference, globaldefaults, Done/Return or filterselection.
This is exact runtime evidence, not a universalresolution of Appleissue178432770.

Stale phone foreground interruption was discarded then retried with exactwindow
Raise and fresh AX menu indices. iPad mirrororientation was repaired through its
own supported Rotate before capture; a noWindowsAvailable coordinate action was
not accepted, and supported AX/addressedtouch supplied the operation. Own resources
were restored/released, including guarded termination of unresponsive task-launched
DeviceHub after normaltermination failed. No protected otherdevice was operated.

Consumer evidence: `playphraseme-cn-b17-7ekvw3ou/handoff.json`, manifest.json and
source-qualification.json under `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/`.
Five qualifiedreal keyboardcompositions produced43first images, no rerolls.
Sourcegeometry remains external AXscreen/pt with mapping/atomicity/owner unknown;
sourcePNGnative3xphone/2xiPad. Installedbuild hashesmatchB15, source revisionunknown.
The current batch is complete; new cases still require scoped ownership and their
own source qualification. Its successful keyboardroute need not be rediscovered.

## CN5B — keyboard candidates, 2026-10-12

Consultation owner01a12cf2-87fe-79a0-9d2e-0191367de38f completed resumed turn
01a1314c-6d75-75d1-aad1-747580509e5c. Read-only source inspection on
c7d8e3d/beta-01, not installed-build or runtime proof. No interfering process
or reservation was found. Previous host wait is no longer the task's current state.
The gap is observed device-scoped Toggle Software Keyboard show/hide capability;
it was never attempted because this consultation prohibited runtime interaction.

Five candidates: iPhone Search & Learn floating query; iPhone Clip Search query
over grid; landscape iPad Search & Learn query/action band; landscape iPad Clip
Search queryband/rail/grid; iPad Director entity-popover anchor/field/list clipping.
Source contract route: features→Platform composition iphone-behavior,
iphone-query-glass3,iphone-more-menu7,ipad-behavior; Clip Search
iphone-clip-query3,cross-destination-query1,ipad-filter-popover2.
Primary fields are focus-only with text/selections preserved; Done submits and
must not be used for dismissal. Director Close only dismisses, whereas Return
selects the active option. Exclude Phone Clip Filters: onDisappear applies drafts.
Keyboard dismissal for primary fields still requires a proven targeted operation.

Apple documents Toggle Software Keyboard separately from Connect Hardware Keyboard.
Consultant reports Xcode27RC issue178432770 for iPad/iOS27, not a demonstrated
failure of the owned runtime. UDID mirroring/HID metadata alone does not prove a
supported keyboard toggle. No global defaults, hardware-keyboard setting changes,
invented shortcuts/private APIs or protected ProMax operation. Exact device/runtime,
enabled targeted menu, visible effect and restoration must be qualified live.

CN-B17 assigns that missing bounded live qualification and the complete five-case
corpus cycle to the existing mobile owner, with normal source-project reservations,
current ownership and installed-build revalidation. Thus the former read-only
consultation limit is not a new user-approval requirement. This is a proposed
source batch, not five accepted or generated cases. No source-app edits authorized.

## CN7 — existing app-target previews, 2026-10-12

Mac advisor01a1102f-791c-7e91-bec3-1877ea004d51 terminal
01a12e22-e720-79c3-b2ca-e8bd2d220fb1, source-only on c7d8e3d/beta-01.
453Preview/PreviewProvider declarations:187Mac,117Phone,143Tablet,6Learner;
not453unique or rendered cases. No UI Preview in Packages or dedicated gallery
target found. App-target Xcode Canvas is an existing entry, not runtime proof.
Package-based simulator preview tooling does not directly run these app targets.

- [Phone Favorites results](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/PhoneFavoritesResultsView.swift:300):
  5states in3Preview declarations, fixed items/paging, empty callbacks, no AppModel.
  Some states share one VStack; neither wrappers nor grouped states count twice.
- [Tablet Grammar](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Search/Learner/Grammar/TabletGrammarPageView.swift:193):
  home+12tense pages, fixed English and empty navigation/search callbacks, often
  ScrollView520×760. Distinct geometry remains unverified; no double-counted tables.
- [Mac Download](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/DownloadVideo/DownloadVideoPreview.swift:60):
  4declared states, example.invalid, memory option store, unavailable service and
  no-op pause. No real download/export actions needed or authorized.
- Phone Download nil-model unavailable and Russian low-storage card are additional
  source candidates, not proof of actual system sheet/app-shell geometry.

Tablet Vocabulary filters/details use makeAppModel production composition and
UserDefaultsDataStore; excluded from this isolated-preview slice. Empty view
callbacks alone do not establish initializer/lifecycle isolation.
Source route: platform-shells r7, ACP-3/list_scenarios, learner/Favorites/state,
shared download@4, low-storage@1 and native-ui-tools@1. Original files/project dirty
state untouched. No build/UI/network/mutation or new corpus case.
Consumer CN8 qualifies one existing Mac unavailable Download Preview via supported
Xcode Canvas, without a new harness or source/project/settings changes.

## CN4 — additional Native mobile families, 2026-10-11

Root accepted this source-only handoff. Recipes CN4-1 through CN4-8 are new
candidates, not observed states or generated cases. One mistyped username in a
contract link is corrected using the advisor's explicit corrected link.

**CN4: восемь source-wired recipes, без новых запусков.** Pin: `c7d8e3dcda9fb5c03fc8c0a96c29420d76e450be`; проверенные owners без локальных изменений. Это кандидаты на следующую партию, **не восемь подтверждённых runtime datasets**. Установленные приложения/устройства сейчас не проверялись.

Общие предусловия: уже доступный paused контент, точный Target, отсутствие auth/access presentation. Только открытие, чтение/scroll и отмена; без выбора значения, Apply, Reset, смены source или воспроизведения. При недоступности — фиксировать её, не создавать состояние искусственно.

| Recipe | Воспроизводимый entry → exit | Новый геометрический вопрос и отличие от прежнего корпуса | Действующий маршрут и wiring |
|---|---|---|---|
| **1. iPad Reels → Collections backside** | На существующем Reel открыть Collections → видимый Close | Центрированный source chooser на обратной стороне карточки: content/scroll/Close, границы front/back accessibility. Это не sidebar и не обычный Reel | [iPad composition](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/ipad-behavior.md), [Reels source contract](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-reels/source-feed-and-active-item-behavior.md); [ActionBar entry](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Reels/Components/ActionBar/TabletReelsActionBarView.swift:80) → [RootContent](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Reels/Components/Root/TabletReelsRootContent.swift:125) → [Collections view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Reels/Components/Collections/TabletReelsCollectionsView.swift). **Только guest/неперсональные данные:** приватные Favorites collections не экспортировать. |
| **2. iPad Reels → пустая Search backside с клавиатурой** | Search action → свежий пустой editor → Close без ввода/submit | Keyboard-safe composition, editor/список/Close и отличие full back face от popup. Не повторяет обычный Search query | [Discovery@2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-reels/discovery-custom-search-and-phrase-chains.md), [Root.openBackPanel / cancelSearchSession](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Reels/Components/Root/TabletReelsRootView.swift:254), [actual Search view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Reels/Components/Search/TabletReelsSearchView.swift). [Существующая QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/runs/2026-09-10-ipad-reels-search-focus.md) **не закрыла** runtime focus. |
| **3. iPad Reels → current-source filter backside** | **Уже выбранный Common Phrases source** → Filters → Close без Apply/Reset | Длинная форма на обратной стороне, нижние действия и scroll containment. Это иной owner/commit flow, чем iPad Search Filters popover | [Reels component semantics](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-reels/c09-macos-component-behavior-part-3.md), [iPad composition](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/ipad-behavior.md); [filter dispatch](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Reels/Components/Root/TabletReelsRootContent.swift:169) → [Common Phrases form](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Reels/Components/CommonPhrases/TabletReelsCommonPhrasesFilterView.swift). Common Words — условный вариант **только если уже выбран**, не дополнительное семейство ради счётчика. |
| **4. iPhone Reels → Translation sheet, начало/конец списка** | Закрытый Translate selector → scroll до последнего языка → native Close | Fixed header + длинный inset-grouped list, отдельная nil-section, checkmark и нижняя safe area. Прежний B12 снимал front/reflow, не эту sheet | [Translation r7, `REELS.TRANSLATION.PHONE.SHEET`](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-reels/translation.md), [sheet owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Reels/PhoneReelsTranslationSheet.swift), [live presentation wiring](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Reels/ReelsView.swift:100). [Историческая QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/runs/2026-10-07-iphone-reels-translation-liquid.md) подтверждала открытие, конец списка и Close; не современный geometry dataset. **Язык не выбирать.** |
| **5. iPhone Reels → Auto-advance choices** | Текущий selector Manual/другой режим → список → X/штатное dismiss | Компактная прокручиваемая controls presentation с шестью вариантами и selected state; сравнение её реальной adaptation с full-height Translation sheet | [Auto-advance contract](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-reels/auto-advance-and-semantic-input-actions.md), [touch-overlays@6, CONTROLS](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/reels-touch-overlays.md); [entry](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Reels/ReelsSlideActionRows.swift:97) → [ControlsPanel](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Reels/PhoneReelsControlsPanel.swift:40). **Не выбирать режим:** это persisted preference и возможный premium gate. Не называть presentation sheet до фактического наблюдения. |
| **6. iPad Reels → fullscreen → return** | Тот же paused Reel → fullscreen action → labelled exit | Aspect-fit video в detail, exit над media, исчезновение ordinary title/action rows при сохранённом root toolbar. Это структурный mode switch, не ещё одна ширина front card | [touch-overlays@6, `IPAD.FULLSCREEN`, QA-REELS-TOUCH-4](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/reels-touch-overlays.md); [RootContent wiring](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Reels/Components/Root/TabletReelsRootContent.swift). Исправленная ссылка на контракт: [reels-touch-overlays.md](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/reels-touch-overlays.md). Player identity не доказывается одинаковой картинкой. |
| **7. iPhone Context: paused overlay + thumbnail grid** | Из доступной paused Search phrase открыть Context → читать/scroll → Close | Трёхколоночные thumbnails, отдельная Browse/hint полоса, fixed-height karaoke stack и paused source/controls. Это не Mac Context и не Clip viewer | [Context overlay@6](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/iphone-phrase-context-overlay.md), [browsing@2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/iphone-context-browsing.md); [presentation owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Search/SearchPhraseContextPresentation.swift), [grid owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Search/PhoneContextThumbnailGrid.swift). **Не включать Browse и не нажимать thumbnail:** оба пути запускают playback. Если корпус уже содержит именно Phone Context/grid — исключить как дубль. |
| **8. iPad Search → Actor picker** | Actor toolbar → существующий ready-list → outside dismissal, без typing/selection/clear | Heading/search/Include possible matches/list в anchored popover, compact adaptation и отсутствие Close. Отличается от Actor Voice metadata filter | [scope-selection@11](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/input/corpus-and-actor-scope.md), [wired root/content](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Search/Actor/TabletActorPickerSearchOverlay.swift:169). Открытие может загружать ready options; пустой/error результат принимать как фактическую доступность, не инициировать failure. Старый `TabletActorPickerMenuView` не использовать как доказательство современной topology. |

**Что не включаю в эту партию**

- **Practice review:** entry автоматически копирует материал; empty/long history нельзя создавать под видом безвредного просмотра.
- **Phone Clip Filters:** на dismiss вызывается `onApplyDrafts`, включая year/source-ID drafts. Без доказанного чистого editor state это не гарантированно нейтральное open/close. [Wiring](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/ClipSearchView.swift:430).
- Новые Reels empty/error states: их наличие в модели/`#Preview` не доказывает готовый безопасный app-launch route. Никакой failure injection.
- Auth/QR, Favorites contents, покупки и новые Guest Tour проверки не добавляю.

**Для следующего исполнителя:** сначала №2 и №4, затем №1/3/5; №6–8 — отдельные условные recipes. Все iPad — landscape, существующие разрешённые устройства по [Simulator route](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-simulator-runtime.md); ownership/input по [runtime lanes](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-runtime-lanes.md).

Новых готовых screenshot+geometry datasets или действующих installed targets CN4 не установил. Будущие данные должны сохранять source identity, units, AX-vs-layout/paint и partial coverage; source constants и эти recipes не являются measured `expected_from`. Файлы и runtime не менялись.


## CN2 — existing debug routes, 2026-10-11

**CN2 завершён, read-only. Pin: `c7d8e3dcda9fb5c03fc8c0a96c29420d76e450be`.** Проверенные catalog/composition owners без локальных изменений. Готовность сборок и rendering запуском не проверялась.

**Главное: существуют три разных входа, их нельзя смешивать.**

| Вход | Что действительно подключено |
|---|---|
| Mac Debug `--playphrase-scenario ID` | Отдельный `AppScenario` с IDs `c01…c05` → `MacAppComposition` → настоящая Search-композиция с fixture dependencies |
| Именованные SwiftUI `#Preview` | Конкретные view owners, иногда с заглушками video/actions; не обычный app-launch маршрут |
| Release `preview/search` URL | Три существующих Search-компонентных состояния; это специальный разрешённый preview, не универсальный fixture selector |

Основание: [ACP launch contract](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-developer-agent-control-plane/acp-tool-003-playphraselaunchscenario.md), [Release boundary](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-developer-agent-control-plane/acp-rel-001-release-and-platform-invariants.md).

**Для CN-B3: 12 source-wired Debug-входов без нового wiring**

Общий существующий аргумент:

```text
--playphrase-scenario <ID>
```

Предпочтительный уже предусмотренный lifecycle — `playphrase.launch_scenario` с `{"scenario_id":"<ID>"}` на проверенном Debug artifact. Наличие работающего MCP-сервера сейчас не установлено. Эквивалентный launch-аргумент также разбирается самим приложением; не сохранять его в обычной схеме.

| ID | Реально заданное fixture-состояние / компонент |
|---|---|
| `c02.search.suggestions` | Открытые suggestions с обычными строками |
| `c02.search.suggestions-long-copy` | Две действительно длинные suggestion strings |
| `c02.search.suggestions-rtl` | Arabic suggestions и RTL presentation flag |
| `c02.search.loading-stable` | Новый запрос при сохранённых предыдущих результатах |
| `c02.search.results-paused` | Загруженные результаты, paused composition |
| `c02.search.results-long-copy` | `PhraseFixtures.longCopy` в результатах |
| `c02.search.results-rtl` | Arabic phrase и RTL presentation |
| `c02.search.empty-alternatives` | Пустые результаты с suggestions |
| `c02.search.offline` | Сохранённый контекст с typed offline failure |
| `c02.search.request-failed` | Сохранённый контекст с malformed-response failure |
| `c02.search.recovery-long-copy` | Failure поверх длинного сохранённого контента |
| `c02.search.recovery-rtl` | Failure поверх RTL-контента |

Цепочка подтверждена исходниками:

- [AppScenario IDs и parser](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/Packages/PlayPhraseScenarioCatalog/Sources/PlayPhraseScenarioCatalog/DeveloperAppScenario.swift)
- [Debug selection → makeC02Fixture](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/Application/Composition/MacAppComposition.swift)
- [конкретные состояния и synthetic transport](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/Packages/PlayPhraseFixtures/Sources/PlayPhraseFixtures/SearchJourneyScenarios.swift)
- [actual app root](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/MacApplicationRootView.swift)

Это **12 достижимых конфигураций входа, не доказательство 12 различных финальных картинок**. Консервативно здесь восемь содержательных семейств: suggestions, длинные suggestions, RTL suggestions, retained loading, paused results, длинные results, RTL results, empty alternatives; failure/recovery добавляют варианты, если текущая presentation действительно их показывает. После запуска нужно подтвердить фактическое состояние, а не принять имя scenario за результат.

**Дополнительные существующие пути**

- **Mac Release, три preview states:**
  `playphraseme://playphrase.me/preview/search?v=1&state=request-failed`
  Аналогично `state=no-results` и `state=search-prompt`.
  [Точный QA recipe](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/search-state-preview-links.md), [view owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Presentation/SearchStatePreviewView.swift). Retry здесь локальный; `request-failed-live` — другая, изменяющая текущий запрос операция. Эти три состояния уже были в CN1 — повторно не считать.
- **iPad Debug `--account-preview`:** подключён из `PlayphrasemeTabletApp`; host предлагает **Premium / Free / Load failure** и монтирует настоящий `TabletAccountView` с synthetic dependencies. [Host и flag](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Account/TabletAccountPreviewHost.swift). Это не production auth и не QR-login; согласовать с CN-B2, чтобы не дублировать.
- **Mac Debug `--playphrase-text-size large-200`:** parser и app entry подключены. Это специальный масштаб fixture, **не native Dynamic Type**. Один ID `…large-text` сам по себе не доказывает изменённый размер текста. [Parser](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/AppScenario.swift), [entry](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/PlayphrasemeMacApp.swift).

**Что реально использует именно FeatureFixtures**

| Fixture ID | Существующий renderer | Ограничение |
|---|---|---|
| `clip-normal` | [Mac grid: обычный/narrow previews](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Grid/ClipSearchGridView.swift), [viewer preview](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Viewer/ClipSearchViewerView.swift) | Preview callbacks частично no-op; viewer получает `content:nil` |
| `clip-normal` | [iPhone inspector preview](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/ClipSearchInspector.swift), Phone/Tablet grid/card previews | Xcode preview, не доказанный launch selector |
| `reels-loading` | [Mac Reels root preview](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/Features/Reels/Components/Root/ReelsRootView.swift) | Synthetic transport, не реальная сеть |
| `reels-paused` | [Mac slide preview](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/Features/Reels/Components/Slide/ReelsSlideView.swift) | Video — `Color.black`, action bar — `EmptyView`; нельзя считать полным Reel |
| `reels-normal` | [Tablet ordinary/fullscreen card previews](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Reels/Components/Slide/TabletReelsSlideView.swift) | Video/actions заменены preview content; только частичная композиция |

**Что пока только данные**

В выбранных Mac/iPad/iPhone owners не найден универсальный consumer `ComponentFixtureCatalog.descriptors`; `FeatureFixtures.search` не имеет прямого app/preview consumer. Большинство `clip-*` и `reels-*` loading/empty/error/long-copy/RTL IDs не подключены к generic launch selector. Нельзя передать `--playphrase-scenario clip-empty`: это другой каталог.

Более того, [developerCatalog](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/Tools/PlayPhraseMCP/Sources/PlayPhraseMCPServer/DeveloperScenarioMetadata.swift) строится из `AppScenario.allCases` и маркирует их `search_and_learn`; наличие enum surface `clip_search`/`reels` не создаёт соответствующих scenarios.

**Минимальный gap:** для неподключённых FeatureFixtures нужен отдельно разрешённый test-only selector/host, который выбирает существующий ID и передаёт его существующей production-композиции с полными безопасными dependencies. Для имеющихся `#Preview` сначала можно использовать текущий preview-host workflow, но маркировать заглушки и частичность. Универсальный renderer или новый benchmark framework для этого не требуется.

Для следующей партии рекомендую **сначала перечисленные 12 Debug-входов C02**, после вычитания дублей CN-B1. Это расширение состояний настоящей Search-композиции; пока **source-wired synthetic cases, не runtime acceptance и не measured geometry**. Файлы, runtime и QR-login не затрагивались.

## CN1 — source advisor handoff

Recorded by root from the terminal receipt of Спроектировать UI Blueprint, 2026-10-11. Discovery accepted; runtime and source-version limitations retained.

**CN1 завершён: 25 подтверждённых сценариев, две партии — 12 + 13.** Это 25 разных компонентных задач, а не 25 уже готовых blueprints. Build/run/UI/capture и изменения файлов не выполнялись.

Сначала важные границы:

- **G** — есть сохранённые canonical geometry + PNG.
- **V** — есть исторические PNG/AX-семантика, но внутренних measured bounds нет.
- **S** — сценарий подтверждён контрактом/исходниками; новый runtime dataset ещё нужен.
- Для будущего сбора: Mac — внешний AX в `pt` плюс независимый native PNG; iPhone/iPad — настоящий Simulator, семантика доступного developer-инструмента и native PNG. Поддержку мобильного collector это не доказывает.
- Probe PlayPhrase.me для всех перечисленных компонентов **не подтверждён**. Наличие SwiftUI anchors в отдельных owners не означает готовый экспорт geometry.
- Auth, покупка, экспорт, Favorites mutations, Reveal чувствительного контента и внешние ссылки не входят в безопасный проход. При gate — сохранить ограничение, не обходить его.

**Партия A — Mac Search и Clip Search, 12 сценариев**

Общий entry: canonical Mac, установленный исходный paused state; exit — закрыть созданную presentation, восстановить размер, выбранную вкладку и scroll. Не подменять другие действующие runtime owners.

| ID / состояние | Безопасный entry → exit | Компонентный вопрос / измерения | Точные основания и готовность |
|---|---|---|---|
| **A01 Search resize** | Paused Search → две поддержанные ширины → исходная рамка | Панели, transport, tabs, query band; ширины, оси, подписанные gaps; AX bounds отдельно от clipping | **G**: [RC03 dataset](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-resize/runtime-20261009.md). [Point sizing@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/point-sizing.md). Два original Observe по 160 узлов/partial; video-only frame и pixel transform неизвестны. |
| **A02 Toolbar и overflow** | Search → сужение → открыть системный overflow → закрыть/восстановить | Высоты и группировка controls, исчезнувшее из inline представления против удалённого; bounds overflow Surface | **S**, [toolbar@9](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/macos-learner-toolbar.md), [owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/Header/View/MacApplicationRootViewToolbar.swift). QA — Acceptance этого leaf. Не активировать Random/Refresh. |
| **A03 Common Phrases: форма + paging + список** | Выбрать вкладку → осмотреть фильтры/страницу → прежняя вкладка | Две строки фильтров, pagination, viewport списка и строки; label/value, scroll bounds | **S**, [Common Phrases@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/learner/common-phrases.md), [форма](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Components/MostCommonPhrases/SearchForm/CommonPhrasesSearchForm.swift). Не выбирать фразу или менять фильтры в первом проходе. |
| **A04 Vocabulary: фильтры и длинные определения** | Vocabulary → scroll существующей страницы → восстановить | Метаданные, определения, filter/pager rows; переносы, ширины колонок, граница viewport | **S**, [Vocabulary@2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/learner/vocabulary.md), [view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Components/CommonWords/View/VocabularyView.swift). Выбор слова меняет Search — исключить. |
| **A05 Grammar: home → таблица времени** | English → Grammar → существующая страница времени → home/прежняя вкладка | Табличные колонки, заголовки, строки и примеры; выравнивание, переполнение | **S**, [Grammar clauses@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/learner/favorites-grammar-statistics.md), [таблица](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Components/Grammar/View/GrammarTableView.swift). Не нажимать примеры, запускающие Search. |
| **A06 Settings: вложенная форма** | Command-comma → открыть один picker без выбора → Escape/Close | Outer shell/header/inner panel, label-control pairs, вложенное меню, scroll | **V**, [RC01](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-settings/README.md); текущая норма [service dialogs@11](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/macos-service-window-glass.md), [view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Components/Settings/View/SearchSettingsView.swift). QA `MAC-GLASS-SETTINGS`; значения не менять. |
| **A07 Genre popover** | Clip Search → Genre → Escape | Две колонки вариантов, trigger/anchor/popup; визуальный checkbox против AX role | **V**, [RC02](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-filters/README.md), [popover@1/r2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-08-26-macos-native-filter-popover-evolution.md), [picker](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Filters/PickerView.swift). Selection/Clear исключены. |
| **A08 Director: idle → lookup results** | Director → разрешённый поисковый draft → результаты без выбора → dismiss | Input, guidance, counted list rows, focus; popup placement относительно trigger | **V только idle**, те же [RC02 данные](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/mac-filters/observations.json) и [контракт](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-08-26-macos-native-filter-popover-evolution.md); [search field](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Filters/PickerSearchField.swift). Lookup вызывает сеть; draft не applied filter. |
| **A09 Cast multiselect presentation** | Cast → осмотр текущих tags/вариантов → Escape | Несколько выбранных tags, remove affordance, список counts/checkmarks | **S**, [контракт](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-08-26-macos-native-filter-popover-evolution.md), [chip](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Filters/PickerChip.swift), [option row](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Filters/PickerOptionRow.swift). Если выбранных tags нет, не создавать их без отдельного setup. |
| **A10 Actor Voice single-select** | Voice → осмотр текущего состояния → dismiss | Отличие zero-or-one selection от Cast; label/count/selected semantics и row bounds | **S**, [filter behavior](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/user-visible-behavior-part-2.md), тот же [picker owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Filters/PickerView.swift). Не считать Cast доказательством говорящего. |
| **A11 Adaptive Clip grid и scroll** | Загруженная сетка → scroll/resize → восстановить | Rail/grid/query topology, 16:9 cards, колонки/gaps, viewport против offscreen rows | **S + историческая QA**, [lightweight cards@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/macos-lightweight-cards.md), [grid](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Grid/ClipSearchGridView.swift), [QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/runs/2026-09-07-macos-clip-adaptive-grid.md). Hover запускает preview — отдельный разрешённый state, не безусловно read-only setup. |
| **A12 Immersive Clip viewer** | Eligible card → viewer → явная Pause при необходимости → Close | Полноэкранная внутри окна композиция, media/metadata/transport/footer, границы действий | **S**, [viewer@3](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-08-20-macos-immersive-viewer-evolution.md), [owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/ClipSearch/Components/Viewer/ClipSearchViewerView.swift). Возможен playback; не переходить через access boundary, не Download. |

**Партия B — help, дополнительные Mac-композиции и настоящие мобильные поверхности, 13 сценариев**

Каждая строка независимо стартует из записанного исходного состояния; партия не требует проходить A подряд.

| ID / платформа / состояние | Entry → exit и предмет измерения | Основания, риск и готовность |
|---|---|---|
| **B01 Mac Search component previews** | Открыть существующий preview URL для `request-failed`, `no-results` или `search-prompt` → Close/Escape. Card/icon/text/Retry и их отношения | **S**, [preview@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/state-preview-links.md), [точные URL/QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/search-state-preview-links.md), [view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Presentation/SearchStatePreviewView.swift). Это actual component preview, не настоящая network failure. `request-failed-live` исключить из безопасной первой партии. |
| **B02 Mac Guest Tour full map** | Ручной Tour → полная карта → штатное закрытие. Карточки, реальные anchors, connectors, группировка/пересечения | **S, отдельный подготовительный gate**, [Guest Tour@7](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/guest-tour.md), [полный обязательный plan/QA01–QA18](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/plans/guest-tour-full-map-spec-and-plan.md), [owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Components/Tooltip/View/PlayphrasemeGuestTourMap.swift). Геометрия недавно переоткрыта; перед запуском нужен полный closure с readable-connectors. Не выдавать старый Accepted record за текущий pass. |
| **B03 Mac Phrase Context** | Exact paused phrase → Context при доступе → Close. Strip, выбранный clip, перевод, Close и video region | **S**, [context@5](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/playback/context-safety-download.md), [view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Components/PhraseContext/View/PhraseContextView.swift). Gate не обходить; не выбирать соседей. Возврат должен сохранять исходную Search identity и паузу. |
| **B04 Mac Download form, без экспорта** | Download текущей фразы → форма/help → Close. Segmented modes, conditional rows, submit/disabled, inner panel | **S**, [download@4](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-shared-services/download-export-legal-and-support.md), [service@11](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/macos-service-window-glass.md), [view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/DownloadVideo/DownloadVideoView.swift). QA `MAC-GLASS-DOWNLOAD`; Submit/Save и изменение persisted options исключить. |
| **B05 iPhone Vocabulary detail** | More → Vocabulary → чтение/scroll → dismiss. Definition/translation/metadata и отдельные Search/Favorite/Copy | **S + историческая QA**, [Vocabulary@2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/learner/vocabulary.md), [owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Search/CommonWordsView.swift), [IPH-011 QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/runs/2026-09-08-iphone-vocabulary-iph011.md). Не Favorite/Copy/Search selection. Large-text state требует отдельного reversible setup. |
| **B06 iPad Common Phrases Filters** | Landscape → Filters → Done/Close. Полная форма, summary `+N`, anchor, paging | **V**, [RC04](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/ipad-filters/README.md), [filters@3](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/ipad-learner-filter-controls.md), [panel](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Search/Learner/Controls/TabletLearnerFilterPanel.swift). Историческая тройка условий уже есть; заново менять фильтры необязательно. Bounds пока неизвестны. |
| **B07 iPad navigation sidebar** | Sidebar toggle → раскрыть Learn → свернуть/закрыть. Overlay/list/disclosures против неизменного detail | **S**, [iPad behavior](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/ipad-behavior.md), [sidebar owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Navigation/TabletNavigationSidebarView.swift). Не выполнять Contact/Legal/account actions; раскрытие групп — presentation setup. |
| **B08 iPad Settings overlay** | Sidebar Settings → scroll без выбора → Close. Content-fitted outer/inner зоны, safe containment, rows | **S**, [popup-glass@12](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/ipad-popup-glass.md), [view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Settings/View/TabletSettingsView.swift). QA `IPG-SETTINGS`. Старый `form-sheet@2` не определяет нынешнюю topology. |
| **B09 iPad synthetic Account preview** | Только существующий DEBUG `--account-preview` → sample Account → Close/завершить owned run | **S, synthetic**, [разрешённый preview в popup contract](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/ipad-popup-glass.md), [точный host](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeTablet/Account/TabletAccountPreviewHost.swift). Status, name/email layout, disclosure/scroll; не доказательство auth/entitlement. Реальный Account и QR-login исключены. |
| **B10 iPhone Search + keyboard** | Фокус текущего query без изменения текста → dismiss keyboard. Floating query, keyboard-safe region, active phrase/suffix list | **V для старых кадров**, [query Glass](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/iphone-query-glass.md), [phrase-list@2](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-search-and-learn/presentation/iphone-phrase-list.md), [root](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Search/SearchRoot.swift), [PNG/QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/runs/2026-10-07-iphone-glass-inspection/README.md). Keyboard focus меняет представление, не должен считаться query mutation. |
| **B11 iPhone More sheet** | More → scroll до низа → native swipe dismiss → reopen | **S**, [More@7](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/iphone-more-menu.md), [owner](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Search/PhoneAppMenuModifier.swift). Quick-action row, scroll content, detent/safe area. Нынешняя норма: large-first, без Close/header; старые схемы с крестиком не эталон. Не активировать строки. |
| **B12 iPhone Reels accessibility reflow** | Один paused Reel → ordinary/accessibility text → reading/action scroll → вернуть размер | **V**, [RC05](/Users/eugenepotapenko/Projects/potapenko-github/ui-bluprint/fixtures/real-world/iphone-reels/README.md), текущий [aligned-column@7](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-platform-composition/iphone-reels-aligned-column.md), [page](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/Reels/ReelsSlidePage.swift). Две action rows, selectors, video/reading split; не scroll по видео. Старый RC05 не закрывает long-description и trailing-selector reachability. |
| **B13 iPhone Clip study viewer** | Eligible result → paused viewer → Close | **S**, [viewer@1](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/native-clip-search/2026-09-01-iphone-compact-study-viewer.md), [actual composition](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemePhone/Application/Views/ClipSearchView.swift). Media/metadata/phrase/action row, query hidden, exact return. Возможен playback/access gate; не Next/Download/Copy в базовом проходе. |

**Готовые данные и targets**

Проверил наличие файлов, без нового запуска:

- **RC03, 9 октября:** две original canonical responses + два PNG существуют. Есть внутренние AX bounds, selected source keys и 16 результатов Measure. Это лучший первый positive geometry seed.
- **RC01/RC02:** Settings, Genre, Director — шесть original/crop PNG существуют; роли/значения есть, внутренней геометрии и точного AX↔CG mapping нет.
- **RC04:** пять iPad PNG существуют; **RC05:** три iPhone PNG существуют. Это исторические Simulator cases с семантикой, не мобильный geometry dataset.
- Старые версии контрактов внутри этих пакетов сохраняются как provenance. Новые captures сравниваются с текущими leaf-контрактами, а не с устаревшим оформлением.

Существующие цели определяет [platform-shells r7](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/specs/features/platform-shells.md): `PlayphrasemeApp`, `PlayphrasemePhone` с отдельными Phone/Tablet compositions, `PlayphrasemeTablet` как development target. Фактическую установленность/готовность устройств CN1 не проверял.

[ComponentFixtureCatalog](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/Packages/PlayPhraseFixtures/Sources/PlayPhraseFixtures/ComponentCatalog.swift) содержит DEBUG descriptors для loading/empty/failed/offline/hidden/long-copy/RTL, включая `search-*`, `clip-*`, `reels-*`; [FeatureFixtures](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/Packages/PlayPhraseFixtures/Sources/PlayPhraseFixtures/FeatureFixtures.swift) содержит соответствующие данные. **Это не универсальная подтверждённая команда запуска каждого состояния.** Отдельно подтверждён source owner `learnerVocabularyFirstOpen`: [fixture](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/PlayphrasemeApp/SearchLearn/Composition/C05VocabularyFirstOpenFixture.swift); его launch wiring нужно выбрать через действующий Debug route.

**Риски и порядок следующего исполнителя**

1. Начать с **A01**, затем A06–A08: есть реальные сохранённые входы, можно проверить полный путь dataset → blueprint без нового app setup.
2. Для остальных сначала установить exact build/Target/state. Общий QA route: [UI policy](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/qa/ui-tool-policy.md) → [runtime](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-runtime-acceptance.md) → [reservations](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-runtime-lanes.md); для mobile дополнительно [Simulator ownership](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-mac/docs/agent-simulator-runtime.md).
3. Партии независимы по данным, **не по desktop**. Mac canonical/input сериализуются; mobile можно вести на свободном существующем устройстве через адресованную isolated session.
4. iPad — landscape. Наличие mini в default pool не делает его заменой требуемому Pro 11 baseline. Нет подходящего разрешённого устройства — отдельный coverage gap, не повод создавать копии.
5. Screenshot pixels — `px`; AX — объявленные `pt`; declaration в SwiftUI — не measurement. Padding, hit region, material/radius и clipping не выводить из похожих рамок.
6. Для CORPUS-1000 не считать повторный кадр, другой timestamp или случайный текст новым независимым компонентом. Эти **25 seeds** дают содержательные семейства; 1000 уникальных готовых случаев CN1 не доказывает.

**Передача root:** маршруты и риски подготовлены; готовность runtime отдельно от наличия исходников отмечена. QR-login, N06 и пользовательские данные не затрагивались. Коммит не требовался.
