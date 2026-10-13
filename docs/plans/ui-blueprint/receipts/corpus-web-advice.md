# Web source advisor handoffs

## USE-WEB1 — actual consumer first-use feedback, 2026-10-13

Terminal01a134b3-edfa-7c92-8059-85dc74114a4c. Web owner attempted a real task:
measure Director popup/input and list alignment in PlayPhrase.me without app changes.
Built modules=web at3e995ec5ab4fbf9160c6fc45e12c72fb32e5c0ed; build, distribution
verify and uiblueprint --help succeeded. Retained bundle:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-use-web1.OTL0dI`.
README's older a026584 pin was not claimed as this build.

Live observe did not run. Concrete first-use friction: caller must prepare connection/
request JSON and session/target/surface identities/limits, with docs pointing into
Rust source. tests/bridges/web/geometry.cjs requires CDP endpoint and target/frame/
loader/document/root IDs obtained elsewhere; helper is absent from flat bundle.
Safe repro `node tests/bridges/web/geometry.cjs --help` returns invalid_arguments
rather than parameter help. Consumer requests a documented supported first external
connection path and usable helper diagnostics; packaging/discovery design must be
reconciled in UI Blueprint, not imposed on the site.

This establishes onboarding/usability feedback, not a measured engine/collector bug
or inability to work externally. No manual geometry/JSON adapter was substituted.
No browser/app launch, source changes, images or new corpus cases. Both repositories
left unchanged by this consumer. Next consumer is the UI Blueprint owner; follow
through with a coherent first-use repair and return it to the same Web agent.


## CW13 — isolated storage does not isolate backend effects, 2026-10-13

Advisor terminal01a13423-0637-7740-8e90-acf2549e0697, sourcec23ef963; bounded
source-only revisit of prior shared-localStorage exclusions after B18/B19. Zero
newly ready candidates in this subset, not a claim the entire site is exhausted.
Reels query/source selection and desktop Actor selection can persist local state
safely in the owned context, but also start search/playback. Reels close-back-panel
sets flipped=false and passes playback-enabled=true; player lifecycle restarts
play-current on activation/item change. Pre-pausing is not a no-playback guarantee.
Qualified views may increment onboarding counters/send analytics-video-view;
closing a session cannot revoke prior requests. No source/UI/network changes made.
Source owners under src/cljs/playphraseme: views/reels/view.cljs:2128,
views/reels/views/video/view.cljs:1500, common/video_player.cljs:1524.
Contracts: existing Reels Discovery/Experience/shell and CW7 Search/persistence.
Remaining Settings selections only change labels/options in empty Clip Search,
not new dependent composition. B12 already covers unapplied Reels Search; B13–17
cover selected entity/Years/IMDb/Genres; B18 locale variants excluded; B19 complete.
CW44/CW46 access gates remain. No interception, capability spoof, clipboard,
account or source-app changes proposed. Exact new dependency for those specific
source/query transitions: qualify their ordinary playback/statistics scope or an
existing authorized runtime preventing those effects. CP18 proceeds independently.


## CW12 — backend rating is not a local Settings control, 2026-10-13

Advisor terminal01a13416-7c7d-7252-84b4-4d00799f81bd, sourcec23ef963,
read-only Settings/ContentSafety → Foundation/RuntimeConfig traversal. Zero candidates.
Content-rating arrives through bootstrap `GET /api/v1/configs?id=content-rating`;
there is no ordinary desktop/mobile Settings setter to lower it. The local nudity
and offensive flags do not change rating. Non-adult backend config hides the entire
17+ section, but that is not an available local UI transition. No backend changes,
response interception or hidden dispatch authorized by this finding.
Owners in the site repository: `common/rest_api.cljs:1480`, `core.cljs:523`,
`model.cljs:18` under `src/cljs/playphraseme/`; desktop Settings `view.cljs:446`,
mobile Settings `view.cljs:241`. Contract: `docs/specs/foundation/runtime-config/behavior.md:47`.
A separately authorized existing runtime with another backend rating would be a new
source prerequisite. This does not stop the independent corpus goal or reopen B19.


## CW-B18 — runtime isolation qualified, candidates excluded, 2026-10-13

Worker01a133fc-3106-75d2-8510-442d523b3260, terminal turn
01a133ff-674b-73f0-8afa-48651f16be75. `/tmp/uib18-b8w_lnv0/neutral-manifest.json`
pins @playwright/cli0.1.19/core1.63.0-alpha-2026-08-31 and existing Chrome for Testing
Chromium145.0.7632.6, headless/unmodifiedUA. New isolated context, no attach/imported
profile/operator cookies. Freshguest/emptyClipSearch/corpusen verified; only interface
locale changed through ordinary UI. Five loadedowner functions match local source,
not a full-bundle claim. CP17binary hash verified but no export executed.

Version-pinned implementation hooks: PWTEST_DAEMON_SESSION_DIR, PWTEST_SOCKETS_DIR,
PWTEST_SERVER_REGISTRY, PWTEST_CLI_GLOBAL_CONFIG point into owned `/tmp/uib18-b8w_lnv0`;
TMPDIR directs temporary profile/artifacts there, NO_UPDATE_NOTIFIER prevents notifier
cache/network. Resolver earlyreturn and inherited daemon env verified, actual daemon
files/sockets/registry/browser profile observed under that root. No HOME/CODEX_HOME,
package or global preference edits. This is not a public API compatibility promise
or all-host filesystem/zero-network claim. Exact session closed; owned process/profile/
sockets removed, seven QA/native images and their ancestors retained. Reuse requires
fresh ownroot/session and current path/version checks, not reused closed handles.

All3candidates excluded BEFORE imports/exports/ImageGen: Arabic Filters remained
English/LTR despite rootlocale ar; GermanSettings had no extra wrapping/reflow or
coverage difference (213nodes; changed rectangles text/header-only); Arabicpicker
matches existing172-node layout/group rows, only headertext/spacers change. Existing
native-name/English direction separation is not new. SourcePNG observations include
nativeDPR2 and one CSS-scale QA image, none promoted to generationreference.
Zero cases/prompts/modelcalls/rerolls; corpus count unchanged.

Startup included Firebase config/installationPOST and ordinary configs/common-content
reads; three analyticsPOSTs recorded. No search/filter-option/translation/media requests
seen. Only summarized paths/counters retained, no auth headers/body or operator storage.
Initial undocumented-hook prerequisite receipt remains at its original temp path;
root's later technical clarification superseded it without erasing history.

## CW11 — one mixed Settings switch state, 2026-10-13

Advisor turn01a1340a-2eb9-7de2-8ad9-ec5ccc7eb98c, c23ef963, source-only.
Route: Settings → Content Safety behavior/invariants/state-QA. Local preferences
are not security/age gates. No UI/storage/source changes or completed case.

Candidate: fresh qualified isolated guest, corpusen, empty ClipSearch/query/results/
viewer/playback → inputSettings. If BOTH17+content rows naturally exist and areOFF,
enable ONLY Hide explicit nudity. It becomes aria-checkedtrue/is-on with shifted
thumb while offensive remainsOFF. That meaningful checked-state difference is absent
from prior defaultSettings; it is not CW60subscriptiondemo or locale text substitution.
AI-note belongs only to nudity and is ALWAYS visible; don't claim toggle reveals it.
No select-value change creates new dependent controls/help/disabled sections in the
inspected Settings. No arbitrary switch combinations/mobile copies/scale variants.

Boolean setter changes app-db and model-storage in that owned isolated context only;
no direct account/server call. It affects filtering of existing cards and FUTURE
query flags, not a new query itself. Empty ClipSearch has no content consumer. Normal
Settings-open analytics/bootstrap remain. If rows absent, exclude without changing
rating/corpus or hidden state. Target switch by its row title, settle CSS transition,
capture one final state. Restore nudityOFF and verify botharia-checkedfalse before
closing Settings and exactbrowser session; closing form alone does not undo preference.

Owners: [switch](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/settings/view.cljs:320),
[model](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/settings/model.cljs:34),
[state CSS](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/settings/view.cljs:159),
[consumer](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/common/content_safety.cljs:39),
[future request flags](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/common/rest_api.cljs:1510),
[entry](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/input/view.cljs:273).
QA: [nudity note](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-settings-nudity-ai-note-visible-desktop-mobile.md),
[English offensive row](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-settings-hard-offensive-toggle-visible-only-for-english.md).

## CW10 — documented isolated browser route, 2026-10-13

Advisor turn01a133ed-34c3-7831-bbf0-32617dfae47a, sourcec23ef963. Read-only
tool/source feasibility; no contexts, tabs, storage or files created. IAB publishes
visibility/viewport, not storage-context creation; sessionName is not isolation.
Installed @playwright/cli0.1.19 documents independent per-session cookies/webstorage/
IndexedDB/cache/history and in-memory profile lifetime, with browser.isolated:true,
outputDir and outputMode:stdout. Site browser-routing permits generic Playwright
Chromium fallback for this missing IAB capability; unsupported-browser UI is a stop,
not permission to spoof UA. No attach/CDP endpoint/extension/persistent/operator
profile/storageState import. This is a fresh owned browser, not access to Chrome403.

Future entry: pinned CLI -s=<unique> --config=<system-temp-config> open own empty
ClipSearch URL. Close the SAMEsession browser, not just its tab; no close-all/kill-all.
Wrapper uses unpinned npx package, so exact inspected version must be preserved or
requalified. npx was found. Browser launchability and ALL daemon/session/output
locations were not established: qualify them against system-temp/no-persistent-dir
policy before launch. This documentation is not completed runtime isolation proof.
Images/tooloriginals/ancestors retained; only owned non-image transients cleaned.

Interface locale setter changes app-db and localStorage/model-storage, with no
separate account/server setter. Documentlang/scoped text direction change; shell
layout remainsLTR. Learner translation is separate but follows locale while unset,
so later translation consumers may change; use empty ClipSearch/no playback/content.
Settings opening has ordinary analytics. No user UI-scale setting found; responsive
scale is viewport-derived, not a reason to invent browser-zoom cases.

After runner qualification, proposed candidates: Arabic desktopFilters (RTL text
inside LTR grid), German mobileSettings (actual long-label wrapping/coverage only),
Arabic mobile interface-language picker (localized groups with mixed native/English
rows). Corpuslanguage remainsen; locale changes through normal UI in freshisolated
guest state only. RUfixedlocale/CNrestrictions remain, no override. Duplicate text-only
or linearly scaled states excluded. No operator cookies or profile state inspected.

References: [session docs](/Users/eugenepotapenko/.npm/_npx/31e32ef8478fbf80/node_modules/@playwright/cli/skills/playwright-cli/references/session-management.md),
[config schema](/Users/eugenepotapenko/.npm/_npx/31e32ef8478fbf80/node_modules/@playwright/cli/README.md:388),
[site routing](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/browser-routing.md),
[locale setter](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/model.cljs:114),
[Settings](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/settings/model.cljs),
[direction](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/common/localization.cljs:580),
[responsive scale](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/common/responsive.cljs:190).

## CW9 — Shortcuts has no wired entry, 2026-10-13

Advisor turn01a133d9-c3b0-7180-b7fc-a5d5fbd05913, source-only atc23ef963.
Route: Classic Search progressive-input/history/gate → Search overlays behavior;
mobile-shell only to distinguish desktop keyboard behavior. Existing contracts
describe shortcuts but do not establish an entry to this help component.

Shortcuts show! only sets modal-props; its local demonstration call is commented.
No caller/directmount found in inspected src/env/templates/client tests. Search
menu imports its namespace but exposes only Settings. Registered keyboard handlers
cover autocomplete/focus/playback/navigation/random actions, none opens this view.
Startup registry includes paywall/login only. Mobile random-shortcut dialog QA and
premium Context QA do not prove a help-dialog route. No keypress/console/network/UI
operation or file change occurred. Source-only negative, not runtime proof; no case.

Owners: [component](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/shortcuts/view.cljs:132),
[menu](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/menu/view.cljs:26),
[key handlers](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/ctrl.cljs:972),
[startup](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/common/startup_ui.cljs:158).
Do not test guessed keys or call show! from console to manufacture corpus coverage.

## CW8 — numeric filter states, 2026-10-13

Advisor turn01a132ff-1413-7490-94b3-840bc58e0fd5, c23ef963/new-banners-paywall,
source-only. Route: Phrase Filters surface-controls/mobile-source-and-response →
Clip Search filter-context and CW7 ctrl/storage chain. Years/IMDb use local drafts,
app-db filters and history.replaceState (not entity pushState). With empty main
query no phrase search/count or shared filter storage is invoked. Normal bootstrap,
logs and browser history remain; fresh tabs are not origin-storage isolation.

Years debounce350ms: each nonempty endpoint needs four digits, one-sided range
allowed, reversed values normalized min/max with no app error message. Incomplete
draft retains prior applied values. Empty removes range.0000 is a parser edge,
not a proposed valid-year or invented validation case. IMDb trims/lowercases and
accepts tt plus at least7digits; format only, no metadata/existence lookup. Invalid
nonempty draft retains prior ID; empty removes. Same350ms debounce; submit sameapply.
No separate app validation message exists for either field in inspected markup.

Ready candidate: own guest empty-query/filter Clip Search, Source=All, mobileFilters;
year-from1980/year-to1989 → settled year.min/max URL. A separate Clear row appears
below inputs and shifts following sections, unlike desktop's inline Clear. Capture
only if actual structure differs from prior empty mobileYears. Wait pending apply
BEFORE Clear; verify empty inputs/no Clear row/no year URL, Back and exact tabclose.
Clear/Back do not cancel old debounce; early exit could reapply stale values.

Desktop partialYears alreadyCW20; complete years without structural change are
not a newcase. IMDb is absent under Source=All and requires movie/episode context.
Two conditional candidates: desktop MovieIMDb tt7131622 with disabled generic
controls; mobile selected-source row/ID/removal and disabled fieldset, no duplicate
IMDb input. CW8 did not qualify switching the Clip Search Source filter and did not
authorize a runtime action. A worker may investigate that exact prerequisite and
use it only if proved to fit the user's existing safe own-tab state scope; global
corpus/source preferences remain protected. No prefilled URL injection or gate bypass.
Entity lookup drafts must stay empty to avoid incidental option refreshes.

Owners: [draft rules](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/filters/free_text.cljs),
[desktop apply/clear](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/suggestions/view.cljs:554),
[mobile apply/clear](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/mobile_filters/view.cljs:482),
[empty-query execution](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/ctrl.cljs:350),
[mobile QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-mobile-filters-navigation-panel.md).
Dedup evidence CW20/CW53/CW-B3/CW-B13. No runtime, changes or new completed cases.

## CW7 — empty-query Clip Search selection boundary, 2026-10-13

Advisor turn01a132b5-475f-7281-a320-d6c61799c4c9, source-only,
c23ef963e5be87195f80cef7173d537a5a146c74/new-banners-paywall. Route: Header/Actor;
Phrase Filters controls/state/lookup; Clip Search input/grid/route-data; Classic
Search discovery/history; Reels Discovery; daily-practice10 for persistence.
Contract requires route/filter continuity. Effects below are source findings;
three proposed compositions are QA scenarios, not new product behavior.

Clip Search entity selection/removal updates ordinary app-db filters and uses
history.pushState. With an EMPTY main query the execution branch resets results,
viewer and playback and does not request phrase search. Filters/selected IDs have
no persistence interceptor. This fits the user's authorized safe reversible UI
scope in a fresh own guest tab; it is not permission for nonempty-query searches,
account/preferences writes or a claim of zero side effects. Browser tab navigation
history may persist in browser/session restore after Close. Public option lookup,
normal bootstrap service storage/locale/analytics/server logs remain as before.

Three candidates, runtime qualification still required:
- Desktop Director: own empty-query/filter Clip Search tab; lookup tarantino,
  select returned Quentin Tarantino; lookup spiel, select returned Steven Spielberg.
  Capture multi-tag trigger/remove controls and open selected/applied picker as one
  composed state, not separate cases per name.
- Mobile Director: same two actual returned options through existing Filters UI;
  selected pills before input/Clear, count wrapping/growth only when really observed.
- Mobile Voice: own empty-filter tab, lookup arnold and select actual option;
  single selected pill/is-single differs from multi Director. Cast is not a
  separate case merely because names differ.

No main query input, source/preferences changes or persistent suggestions-collapse
toggle. Before and after each selection verify main query remains empty, no gate,
results/viewer absent and only own selected IDs. Exit: remove own values via existing
remove/pill controls, verify empty filters/results/viewer, then close exact own tab.
Desktop draft follows CW4 input-clear/settle/close-reopen check; mobile waits pending
lookup before Back/Close. Removal adds tab-history entries, does not erase them.

Actor desktop remains excluded: selection starts search after100ms, refresh after2s,
enables playback; qualified watch can write shared onboarding count/server statistics.
v1/v2 capability, gates and mobile paused semantics need separate qualification.
Reels selection remains excluded: load-feed/set-source-query writes shared model
storage via Common Phrases offset/filter events; custom search also persists source
query. Closing its tab cannot undo those writes; this needs additional authority.

Owners: [Clip Search execution](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/ctrl.cljs:330),
[selection](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/suggestions/view.cljs:867),
[mobile composition](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/mobile_filters/view.cljs:795),
[model](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/model.cljs:7),
[URL helpers](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/common/util.cljs:184),
[Actor](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/actor_select/view.cljs:274),
[Reels](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/view.cljs:984).
No runtime, changes or new completed cases from consultation. Source owners unchanged.

## CW6 — two remaining Reels Search states, 2026-10-13

Advisor turn01a132a2-795b-7df2-8751-a58bb36e6537, source-only,
c23ef963e5be87195f80cef7173d537a5a146c74/new-banners-paywall. Selected route:
site specs root → Discovery/Playback → Reels Discovery → behavior,
invariants-and-failure, route-state-data-qa-and-references. Contract supplies
fresh search input/public suggestions; local draft/callback behavior is observed
source evidence; the following two candidates are QA proposals, not new intent.

Own guest `http://localhost:3000/#/reels/en?source=common-phrases`, pause public
auto-started playback if needed, open `button[aria-label="Search phrase."]`,
fill `.reels-search-input` with exact public draft `hello` or `zzqxzzqx` in a
separate own tab for each state. R1 requires successful positive count and
nonempty suggestions: custom-query card plus Common Phrase rows/subtitles/counts.
R2 requires successful count0 and suggestions[]: disabled custom card/submit plus
no-suggestions message. Both differ from the already captured initially empty
panel. Runtime data, row count, overflow and uniqueness require fresh verification.
Transport failures cannot qualify as empty; actual duplicates are excluded.

Local atoms own input/options/count/loading. Each onChange, without debounce,
starts count then suggestions; request-id guards late results. Wait both completed
responses and stable output. No Enter/Submit/custom card/suggestion selection:
those apply feed/query and analytics/source-query persistence. Check unchanged
URL/source and no login/paywall; close whole exact own tab after capture.
Count may create browser-search-session-id in sessionStorage if absent; this is
ordinary service bootstrap, not evidence of zero writes or origin isolation.
Do not manually create/read out the identifier. A strict no-new-storage route
requires it to exist from ordinary bootstrap. login-md opens login on401/403;
exclude that outcome without bypass. Closing a tab does not revoke requests or
erase server logs/cache. No translate-to is supplied by this lookup.

Owners: [search component](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/search/view.cljs:179),
[REST](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/common/rest_api.cljs:1171),
[suggestion response](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/clj/playphraseme/common/common_phrases.clj:295),
[apply handler](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/view.cljs:1650).
QA: [empty input](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-search-panel-empty-focused-input.md),
[suggestion lookup](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-search-suggestion-opens-common-phrases.md)
without its selection step.

No additional safe family was established from the prior catalog. Actor ready is
owned by CW-B11; previous Director/mobile/Terms/demo variants are covered. Existing
gated/unavailable/non-novel CW candidates retain those dispositions. Shortcuts has
code but no current visible menu entry; do not invoke it from console. Account,
Favorites/export, filter application and preferences remain outside this packet.
No runtime, changes, captures or new counted cases came from the consultation.

## CW5 — Actor in a disposable own tab, 2026-10-13

Advisor turn01a13263-dd27-78f0-a156-3b5ecb6379ff, c23ef963/new-banners-paywall,
source-only. Route: Header/Corpus selectors/state QA → Actor/deeplink contract →
server-api.discovery-playback. Two candidates: Tom Hardy narrowed response and
zzqxzzqx successful empty[]. Each uses a new own guest desktop Search tab, no
selected Actor. Empty UI has no separate no-results message; do not invent one.

Actor query/results use ordinary reg-event-db assoc without model-store-md or the
explicit persistence interceptor. Global save-model persists locale fields, not all
app-db. No Actor storage-event/BroadcastChannel/shared-worker/unload persistence
was found in the selected source roots. Closing the entire own tab destroys that
JS context, so late responses cannot update its UI or another tab's Actor state.
It does not revoke server requests or remove normal logs/cache. Query is logged, so
only public test strings are permitted. Browser origin storage is NOT isolated by
new tabs: bootstrap locale normalization, value-for-test feature detection and
ordinary analytics remain existing page-start effects, not Actor persistence.

Entry localhost Search English → await initial Actor lookup → Select Actor → fill
whole public string once where supported → await successful matching response and
all prior pending Actor calls → capture → verify unchanged main query/Actor/URL →
close exact own tab. No option/Enter/Clear/checkbox/app-close restoration. If gate
or persistent/shared effect appears, exclude rather than bypass. Actual short/empty
composition must differ from old ready/open-empty; runtime remains a separate gate.

Owners: [Actor input](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/actor_select/view.cljs:192),
[Actor model](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/actor_select/model.cljs:15),
[persistence interceptor](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/common/localstorage.cljs:34),
[global save](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/model.cljs:70),
[REST](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/common/rest_api.cljs:1650),
[server lookup](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/clj/playphraseme/api/route_functions/search/search_actors.clj:39).
CW-B10 consumes these two recipes; consultation itself adds no cases or runtime proof.

## CW4 — temporary filter lookup states, 2026-10-12

Web advisor01a1102f-e21d-7251-9597-c29a1c66d088, source-only turns
01a131e8-315a-77a1-b35b-1559a7024267 and01a131eb-8db6-7593-a00f-dc89f6970531;
c23ef963/new-banners-paywall. No runtime/requests/changes. Specs root selects
Phrase Search Filters surface-controls/mobile-source-and-response/option-lookup/
gateway/QA, Header/Corpus and relevant external adapter/mobile shell/layout leaves.

| Recipe | Proposed public input | Candidate novelty |
| --- | --- | --- |
| D1 desktop Director | tarantino → zzqxzzqx, await each result | successful empty instead of counted rows; actual popover height change |
| D2 desktop Director | an, optional lower scroll after response | real limited-height inner-list overflow distinct from prior spiel |
| M1 mobile Director | tarantino → zzqxzzqx | inline empty-state moves following sections upward |
| M2 mobile Director | an | long inline result expands outer panel scroll and pushes Cast below |
| M3 mobile composite | Director tarantino, Cast brad, Voice arnold, sequential settled responses | simultaneous lists change shared composition; one case, not one per name/scroll |

Own guest Clip Search English tab, empty main query/no filters/initially empty
entity input; actual desktop/mobile layout, no settings override or auth bypass.
Queries are proposals, not guaranteed results. Failure/loading is not empty. Prior
single Cast/Voice results remain duplicates unless composition actually differs.
Input edits change local draft;250ms debounce, length≥2, GET public filter-options
with limit10. Gateway POST is a read-only Search Engine lookup, not filter apply.
Normal network reads/logs remain; no offline/no-network claim.

Desktop exit requires editing the field back to empty, waiting debounce and
under-threshold, Escape, then reopening. on-open synchronously increments request-id
and clears options/active/loading/error with no HTTP; stale responses are ignored.
Verify empty input, no option buttons/aria-activedescendant, unchanged URL/filters,
then Escape. The immediate minimum-length hint alone is insufficient proof.
Mobile waits for all current lookups, Back until unmounted, reopen to verify empty
entity inputs/no pills and unchanged URL/filter/badge, then Back. Back destroys local
query/options but does not cancel pending HTTP/debounce, hence waiting before exit.
Never use filter Clear/selected pills/options/Enter to restore. Web Actor lacks
late-response protection and its cross changes Actor/URL; excluded. Years/IMDb
auto-apply complete values; excluded. Native Phone lifecycle is not inferred from Web.

Source owners: [desktop draft/requests](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/suggestions/view.cljs:933),
[input](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/suggestions/view.cljs:1078),
[open/close](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/filters/picker_view.cljs:397),
[mobile query](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/mobile_filters/view.cljs:795),
[panel lifecycle](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/mobile_view/components/navigation_view/view.cljs:76),
[REST](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/common/rest_api.cljs:897).
QA: [desktop](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-desktop-filters-and-suggestions-panel.md),
[mobile](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-mobile-filters-navigation-panel.md).
CW-B9 consumes these five candidates after loaded-handler and source/novelty proof.

## CW3 — existing isolated preview discovery, 2026-10-12

Advisor01a1102f-e21d-7251-9597-c29a1c66d088, terminal turn
01a12d8b-f244-74d3-a394-97b32b790c04. Source-only, no runtime or new cases.
Source remains c23ef963e5be87195f80cef7173d537a5a146c74, branch new-banners-paywall;
preexisting env/dev/resources/config.edn untouched. No general component gallery.

One useful candidate: `http://localhost:3000/?demo=subscription`, two local
autorenewal states. [Runbook](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/ops/yookassa-subscription-demo.md),
[dev entry](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/env/dev/cljs/playphraseme/app.cljs),
[fixture](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/env/dev/cljs/playphraseme/subscription_demo.cljs).
Existing loopback guard/dev entry bypasses core/init!, renders actual subscription
content with fictional fixed data, and replaces actions with a local atom toggle;
no billing, clipboard, Favorites or preference writes from those handlers. Enabled
and disabled content differ meaningfully. Runtime/dev-build correspondence still
must be proved before any click; a normal-app remount ends the fixture scenario.
Shared home.html includes Google/Firebase analytics: this is not offline isolation.
Ordinary page telemetry is already within earlier Web capture authority; it does
not authorize any subscription/account operation. No network-bypass modification.

Other findings are not ready isolated previews: show_ui paywall/login follows
normal auth/preload/marketing; locale override persists preference. Search fixture
replaces runtime transport/capabilities and storage. CLJS renewal/checkout render
tests are data/code, not live gallery or captured geometry. No harness creation
authorized. Discovery covered env/dev, startup registry, package/project config,
templates, tests/fixtures and QA scripts, plus source spec/QA dependencies.
Consumer: finite CW-B7 on the two already-existing demo states, using own context
and source/geometry/image verification. No new release/billing acceptance claim.

## CW2 — additional families, 2026-10-11

Root received this read-only source handoff from Research website UI blueprint.
Advisor-local IDs CW25–32 are remapped below to corpus IDs CW43–50 to avoid
colliding with already captured CW-B3 and active CW-B4. No new runtime proof.
The mistyped username in one QA link was corrected after checking the file exists.

**CW2: `DONE_READ_ONLY`. Нашёл 8 дополнительных рецептов — примерно 7 семейств, не ещё 12 гарантированно доступных экранов.** Новых captures нет. CW01–24, нижние части прежних форм и текущая CW-B4 исключены.

Pin всех перечисленных site-source/spec/QA: **`c23ef963e5be87195f80cef7173d537a5a146c74`**. Это source basis, не подтверждение загруженного build.

База будущего сбора — существующий `http://localhost:3000`, собственный гостевой контекст. Канонический Reels URL: `/#/reels/en?source=common-phrases`. Старые QA-ссылки `/#/reels` — legacy entry, не новая рекомендация.

| ID | Новый случай; вход → выход | Геометрическая польза и отличие | Доступность / ограничения |
|---|---|---|---|
| **CW43** | Reels desktop, один активный reel → зафиксировать paused state → закрыть свою вкладку | **Однорядная action bar**: icon group, spacer, translation/auto-advance selects, описание и side navigation. Общие края и распределение остаточной ширины. Это другой shell, не Clip Search viewer | Нужен реально загруженный reel. Не нажимать speed/repeat/Favorites/download/copy. Возможны autoplay и telemetry |
| **CW44** | Тот же Reels в подтверждённой mobile layout → закрыть контекст | **Двухрядная action bar**: primary/secondary rows, правое выравнивание selects, video–description–toolbar spacing. Засчитывать только реальную смену композиции | Responsive browser, не native mobile. Не считать ещё одним случаем каждую ширину |
| **CW45** | Reels → `Search phrase.` → наблюдать пустую открытую панель → `.reels-backside-close` | **Search backside**: heading/subtitle, autofocus input, submit и suggestion area внутри отдельной panel composition. Отличается от inline autocomplete CW-B4 | Не вводить и не выбирать подсказку. Loading/empty/results отмечать только фактически наблюдёнными |
| **CW46** | Reels с source Common Phrases → `Open filters.` → закрыть крестиком **без Apply** | **Reels filter form**: pagination над grid полей, label/select pairs, footer. Отличается от learner-панели Search и mobile Clip Search Filters | **Условный рецепт:** QA применения требует authorized session; в прочитанном open-handler нет отдельного auth-gate. Доступность именно гостевого открытия не проверена. При gate — остановить, не логиниться |
| **CW47** | Reels desktop → hover `Select collection` → убрать pointer | **Один контекстный tooltip action bar**, его anchor и fit относительно нижней части reel. Это не полный Guest Tour cluster | Слабее по разнообразию: общий tooltip-механизм уже встречался. Учитывать как новое состояние, не как независимое семейство или серию по всем иконкам |
| **CW48** | Classic Search с доступным paused result → правый клик по player overlay → закрыть backdrop без выбора | **Video context menu**: две строки Speed/Repeat и группы option chips, выбранные состояния, позиционирование относительно точки открытия, viewport containment | Открытие и изменение значения разделены: premium-gate стоит на выборе speed/repeat. Ничего не выбирать. Это не modifier row из CW09 |
| **CW49** | Открыть `/terms-of-use.html` → один заранее выбранный viewport/scroll position → закрыть вкладку | **Настоящий long-form document**: ограниченная колонка, H1/H2, paragraphs/lists, line wrapping и scroll. Ранее были Legal menus, не документ | Public, без auth. Не превращать Terms/Privacy/Fair Use с одинаковой вёрсткой в три новых семейства. Не кликать contact links |
| **CW50** | Открыть существующий QA URL `/#/definitely-missing-route` → наблюдать SPA not-found → закрыть вкладку | **Dedicated route-fallback shell**: desktop header и отдельный heading/content region; placement/overlap/scroll. Это не пустая grid и не network-failure injection | Реальная предусмотренная route response, но runtime сейчас не проверен. Server 404 отдельно не предлагаю: в норме отмечен известный unresolved UX-дефект |

## Точные маршруты

**CW43–26**

- Spec: [reels-shell.behavior@3](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/reels-interaction-shell/behavior.md), [инварианты](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/reels-interaction-shell/invariants-and-failure.md).
- QA: [desktop/mobile action rows](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-action-bar-mobile-two-row-speed-repeat.md), [select alignment](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-action-bar-selects-right-aligned-with-spacer.md).
- Source: [action_bar/view.cljs](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/views/action_bar/view.cljs:822).
- Не переносить старый QA `force mobile` override как обязательный setup: сначала подтвердить естественную responsive composition.

**CW45–28**

- Spec: [reels-discovery.behavior](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/reels-discovery/behavior.md), [reels-experience.behavior](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/reels-experience/behavior.md).
- QA: [empty focused Search panel](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-search-panel-empty-focused-input.md), [Common Phrases filters](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-common-phrases-filters-apply.md).
- Source: [search panel](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/search/view.cljs:240), [filter form](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/collections/common_phrases/search_form.cljs:407), [panel mounting](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/view.cljs:1950), [Close](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/views/backside_close/view.cljs).

**CW47**

- Spec: `reels-shell.behavior@3`.
- QA: [desktop hover tooltips](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-action-bar-desktop-hover-tooltips.md).
- Source: [tooltip-wrapped action buttons](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/reels/views/action_bar/view.cljs:463).

**CW48**

- Spec: [shared playback controls](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/playback-runtime/overview-and-behavior.md).
- QA: [Reels action-bar case, right-click menu step](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-reels-action-bar-mobile-two-row-speed-repeat.md).
- Source: [Search mounts menu; gate applies to value changes](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/video/view.cljs:429), [menu rows/backdrop](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/video_menu/view.cljs:267).
- Файл `tc-video-context-menu-global-speed-repeat.md` сейчас проверяет premium gating; его название не означает готовый geometry oracle.

**CW49**

- Spec: [public-policy.overview-behavior](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/public-policy/overview-and-behavior.md), [routes/invariants](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/public-policy/invariants-failure-state-and-qa.md).
- QA: [public policy links](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-public-support-and-rights-links-expose-contact-and-fair-use-policy.md).
- Source: [terms-of-use.html](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/resources/public/terms-of-use.html:46).

**CW50**

- Spec: [route-resolution-and-not-found](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/route-resolution-and-not-found-spec.md).
- QA: [unknown hash route](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-unknown-hash-route-falls-back-to-dedicated-not-found-page.md).
- Source: [not_found/view.cljs](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/not_found/view.cljs).

## Что сознательно не добавлено

- **Download dialog:** существующий QA требует подписанную сессию; не подходит под запрет account/login.
- **Daily Practice:** `daily-practice@10` — иной сложный modal, но вход сразу копирует prompt и читает browser-local study history. Без отдельного разрешённого чистого material-состояния это не безопасный recipe.
- **Reels Collections:** source передаёт `favorites-query` и может раскрывать пользовательские коллекции; исключён при текущем запрете Favorites.
- **Contact/help:** контракт описывает outbound contact, а не отдельную help-форму. Новой панели придумывать нельзя.
- Новые слова, актёры, одинаковые жанровые chips, нижние части уже собранных панелей и очередные viewport не увеличивают число семейств автоматически.

**Практический порядок:** сначала CW45, CW48, CW49, CW50; затем CW43–26 при доступном публичном reel; CW46 — только при фактически доступном guest opening; CW47 — низкоприоритетный дополнительный state.

**Предел:** подтверждено ещё **8 source recipes**, из них один условный и один слабый по новизне. Ни один не captured в CW2. Вместе с прежними 24 это 32 каталожных сценария, а не 32 доказанно доступных состояния и тем более не основание обещать 1000 семантически разных кейсов одного сайта. Для дальнейшего расширения потребуется действительно новое семейство источников или отдельно разрешённый scope — не размножение этих форм.

Все будущие размеры — из runtime в указанном Space/units; ни приведённые CSS, ни старые QA-числа не являются новым measured oracle.


## CW1 — source advisor handoff

Recorded by root from the terminal receipt of Research website UI blueprint, 2026-10-11. Read-only discovery accepted; recipes are not runtime acceptance.

**CW1: `DONE_READ_ONLY`. Найдено 24 разных существующих сценария.** Два семейства имеют сохранённые screenshot+geometry datasets; остальные 22 — проверенные по specs/QA/source рецепты, требующие будущего runtime-сбора. Это не 24 выполненных blueprint-кейса.

Основа сайта: HEAD `c23ef963e5be87195f80cef7173d537a5a146c74`. Маршрут: `AGENTS.md → docs/specs/README.md → discovery-playback / surfaces-experience → выбранные листья ниже`. UI Blueprint `registry37/DESIGN-ANALYSIS@1` использован для понимания результата: точные измерения и гипотезы о закономерностях не становятся требованиями дизайна. Код, UI и файлы не менялись.

## Правила для обеих партий

База — существующий `http://localhost:3000`. `S` ниже означает `/#/search?language=en`, `C` — `/#/clip-search?language=en`.

Каждый будущий кейс сохраняет:

- actual target/frame/document identity, состояние, source/build correspondence либо `unknown`;
- screenshot и DOM/AX одного состояния, интервалы захвата, viewport/scroll/DPR/CSS zoom;
- bounds, родительские связи, label/control/anchor, доступные состояния;
- coverage и omitted; layout bounds отдельно от paint/hit;
- фактический путь входа и выхода.

**Глобально неизвестны без дополнительного evidence:** полная окклюзия, glyph contours, полные hit regions, причина layout из одних rects. CSS-декларации и старые QA-числа — не новые измерения.

## Партия CW-B1 — 12 сценариев Search и shell

Все — **recipe-only**, без готовой согласованной пары screenshot+geometry в этом чате.

| ID | Сценарий, безопасный вход → выход | Состояние и геометрический вопрос | Маршрут |
|---|---|---|---|
| CW01 | S → открыть corpus selector → закрыть без выбора | Trigger, dropdown, строки языка/count: ширина, выравнивание, clipping списка | H1 |
| CW02 | S → Select Actor → закрыть без выбора актёра | Готовый список, поле поиска, backdrop: inset поля, высоты строк, слои. Пустой список не подменять вымышленными актёрами | H2 |
| CW03 | S → правый interface-language selector → закрыть без смены locale | Билингвальные строки, native/English части, заголовок, секции списка: переносы и баланс ширин | H3 |
| CW04 | S → Legal → закрыть, не переходя по ссылкам | Компактное меню документов: icon/label, порядок, общие края и anchor | H4 |
| CW05 | S → Common Phrases tab → вернуть исходную вкладку | Фильтры, строки фраз и pagination: общий inset, выравнивание, границы scroll-area | L1 |
| CW06 | S → Vocabulary tab → вернуть исходную вкладку | Отличающаяся форма сортировки/фильтров и словарные строки: ширины select, интервалы, pagination | L2 |
| CW07 | S с English corpus → Grammar tab → вернуть исходную вкладку | Grammar cards/table: колонки, плотность, перенос текста, границы панели | L3 |
| CW08 | S → публичный draft с реально появившейся completion → восстановить draft | Составной input с невидимым prefix mirror и видимым suffix: baseline, граница typed/suggested text, clipping | O1 |
| CW09 | S с доступным результатом → paused overlay → восстановить исходное playback state | Центр Play и отдельная modifier row: центровка, gaps, наложение на видео. Не включать скрытый Progressive Repeat | O2 |
| CW10 | S → Tour → закрыть Tour | Несколько tooltip cards и SVG connectors: transformed bounds, viewport-fit, пересечения соединителей | T1 |
| CW11 | S в подтверждённой mobile layout → corpus selector → Back без выбора | Навигационный picker вместо desktop dropdown: header/Back, строки/counts, scroll и fixed header | H1 + M1 |
| CW12 | S mobile → overflow menu → Legal → Back | Отдельный navigation layer с документами: общие края Back/title/list, inset строк; не открывать документы | M2 |

CW05–07 доступны только если соответствующая learner-панель действительно показывается. Если текущая настройка скрывает её, **не менять настройки пользователя ради корпуса**: записать prerequisite gap. CW08 зависит от настоящего ответа suggestions, а не от заранее обещанной строки.

## Партия CW-B2 — 12 сценариев Clip Search и форм

| ID | Сценарий, безопасный вход → выход | Состояние и геометрический вопрос | Маршрут / статус |
|---|---|---|---|
| CW13 | C с пустым query → закрыть собственную вкладку | Empty-state, rail и нижний input: центрирование сообщения и распределение свободного пространства | C1; recipe |
| CW14 | C с публичным query, например `how are you` → завершить свою сессию | Обычная desktop grid с реальными карточками: колонки, gaps, media/meta/badges и нижний input | C1; recipe |
| CW15 | CW14 → hover одной карточки → убрать pointer и подтвердить остановку preview | Содержательно иной playing-preview state: вторичная chrome скрыта, download остаётся; сравнение занятых областей | C2; recipe |
| CW16 | CW14 → открыть карточку → pause → Close | Desktop fullscreen viewer: source/title, lower karaoke card, Play overlay, side navigation; alignment и overlap | C3; recipe |
| CW17 | C с результатами в подтверждённой mobile layout → закрыть вкладку | Двухколоночная мобильная grid с компактными badges и single-line source, не уменьшенная desktop grid | C1 + M1; recipe |
| CW18 | CW17 → карточка → pause → Close | Mobile viewer с квадратной stage и нижним plain-text overlay: ratio, gutters, copy/close alignment | C4; recipe |
| CW19 | C → Genre → закрыть без выбора | Popover с сеткой genre chips: переносы, равные семейства gaps, containment и anchor | F1; recipe |
| CW20 | C → Years: reversible draft `199` → восстановить исходный draft | Двухпольная строка с появившимся Clear: равенство полей, gap, alignment label/controls. Applied filters должны остаться прежними | F1; recipe |
| CW21 | C → hover/focus Source info → убрать hover/focus | Tooltip с публичным пояснением и ссылкой: trigger→tooltip gap, переносы текста и link containment. Ссылку не открывать | F1; recipe |
| CW22 | C → Director, draft `spiel`, без выбора → восстановить draft/Escape | Popup/input/list/options: внутренние интервалы, активная подсказка отдельно от applied membership | F1; **captured D1** |
| CW23 | C mobile → Filters → Back, без ввода/выбора | Полноэкранная прокручиваемая форма Source/Years/Genres/entities; clipped Cast и omitted нижние секции | F2; **captured D2** |
| CW24 | S → Settings → Close без изменения controls | Modal form: labels/selects/notes/Close, порядок строк, общие края и локальность пояснений | S1; recipe |

CW15–18 требуют реальных media results и могут запустить звук/telemetry. Будущий packet должен разрешать обычный preview/open/pause и закрепить ресурс. Autoplay refusal не обходить и не подменять playing-state. Для чистой геометрии фиксировать paused state там, где сценарий его допускает.

**Нельзя считать дополнительными кейсами:** десять карточек с одинаковой структурой, десять director names или несколько viewport без реально изменившейся композиции. CW11/CW17/CW18 отличаются от desktop отдельными нормативными layouts, поэтому обоснованы как самостоятельные сценарии.

## Точные spec / QA / source маршруты

QA здесь — навигация и историческое evidence. Его `VERIFIED` не означает новый pass; старые очистки storage, региональные overrides и числовые допуски не переносятся автоматически.

| Ключ | Spec | QA | Основной source |
|---|---|---|---|
| H1 | [header selectors](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/header-corpus/selectors.md) | [corpus popover](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-desktop-language-popover-selection-updates-route.md) | [languages_select](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/languages_select/view.cljs) |
| H2 | тот же `header-corpus.selectors` | [actor popover](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-desktop-actor-popover-backdrop-stays-above-search-surface.md) | [actor_select](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/actor_select/view.cljs) |
| H3 | тот же `header-corpus.selectors` | [locale header geometry](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-desktop-interface-locale-selector-shares-header-gap-distribution.md) | [interface_locale_select](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/interface_locale_select/view.cljs) |
| H4 | [header navigation](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/header-corpus/navigation.md) | [public document links](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-public-support-and-rights-links-expose-contact-and-fair-use-policy.md) | [header](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/header/view.cljs:365) |
| L1 | [learning panels](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/learning-panels/behavior.md) | [panel tabs](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-side-menu-tabs-open-functional-panels.md) | [most_common_phrases](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/most_common_phrases/view.cljs) |
| L2 | [grammar/vocabulary](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/grammar-and-vocabulary-spec.md) | [vocabulary filters](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-vocabulary-filters-pagination-reset.md) | [common_words](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/common_words/view.cljs) |
| L3 | тот же `grammar-vocabulary` + L1 | [panel tabs](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-side-menu-tabs-open-functional-panels.md) | [grammar](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/grammar/view.cljs) |
| O1 | [search overlays](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/search-overlays/user-visible-behavior.md) | [autocomplete](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-desktop-search-autocomplete-normalizes-whitespace.md) | [Search input](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/input/view.cljs) |
| O2 | тот же `search-overlays.behavior` | [paused overlay](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-desktop-search-paused-overlay-repeat-indicator-cycles.md) | [overlay](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/overlay/view.cljs) |
| T1 | [tour behavior](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/guest-tour/behavior.md), [invariants](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/guest-tour/invariants-and-failure.md) | [connectors](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-guest-tour-right-action-connectors-do-not-cross.md) | [tooltip](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/common/tooltip.cljs) |
| M1 | [mobile layout](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/mobile-layout/overview-and-behavior.md) | [mobile header](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-mobile-header-language-actor-select-fixed-equal-height.md) | [mobile header](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/mobile_view/components/header/view.cljs) |
| M2 | [mobile shell](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/mobile-shell/overview-and-behavior.md) | [Legal panel](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-mobile-menu-legal-opens-documents-navigation-panel.md) | [mobile Legal](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/mobile_view/components/legal/view.cljs) |
| C1 | [input/grid shell](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/clip-search/input-and-grid-shell.md), [grid results](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/clip-search/grid-results-and-gate.md) | [entry/grid/viewer](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-desktop-entry-grid-viewer-and-gate.md) | [grid](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/grid/view.cljs), [card](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/grid/card/view.cljs) |
| C2 | [grid preview](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/clip-search/grid-preview-and-entry.md) | [preview chrome](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-card-preview-playback-hides-secondary-chrome.md) | [preview_view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/grid/card/preview_view.cljs) |
| C3 | [viewer core](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/clip-search/viewer-core.md) | C1 QA | [viewer](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/viewer/view.cljs) |
| C4 | [mobile viewer](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/clip-search/mobile-viewer.md) | [mobile viewer QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-mobile-fullscreen-viewer-centered-layout-and-swipe.md) | [mobile_view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/viewer/mobile_view.cljs) |
| F1 | [filter controls](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/phrase-search-filters/surface-controls.md), [pointer/mobile/source](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/discovery-playback/phrase-search-filters/mobile-source-and-response.md) | [desktop filters](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-desktop-filters-and-suggestions-panel.md) | [picker](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/filters/picker_view.cljs), [suggestions/filter form](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/suggestions/view.cljs) |
| F2 | F1 mobile/source | [mobile filters QA](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-clip-search-mobile-filters-navigation-panel.md) | [mobile_filters](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/clip_search/components/mobile_filters/view.cljs) |
| S1 | [settings](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/docs/specs/surfaces-experience/settings/overview-and-behavior.md) | [settings modal](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/cases/regression/tc-settings-modal-controls-persist.md) | [settings view](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/src/cljs/playphraseme/views/search/components/settings/view.cljs) |

У этих site-leaves преимущественно нет отдельных числовых semantic revisions; закреплять их содержимое следует указанным Git HEAD и хешами выбранных файлов.

## Уже доступные реальные данные

**D1 — Director, 9 октября.** Каталог:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-q03-director-SYpdJ9`

[Manifest](/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-q03-director-SYpdJ9/manifest.json), `wide.jpg`, `narrow.jpg`, `wide-dom-detail.json`, `narrow-dom.json`, DOM backend identities, AX, metadata, task и отдельный answer key. Два состояния 1280×900/1024×768; изменение CSS zoom и геометрии действительно измерено. Это одно семейство; root отдельно решает учёт двух layout-состояний, не выдаёт их за разные формы.

**D2 — mobile Filters, 10 октября.** Каталог:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-mobile-filters-OBW93z`

[Manifest](/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-mobile-filters-OBW93z/manifest.json), `mobile-filters.jpg`, `dom-layout.json`, `node-identities.json`, `accessibility.json`, metadata/handoff. Один viewport 390×844, 53 видимых/частично видимых узла; Cast частично виден, нижние секции omitted. Поля пусты, raw values не сохранены.

**Сегодня проверено:** оба каталога существуют, все перечисленные manifest-файлы присутствуют, SHA-256 совпадают. Оба — browser-reference datasets, не самостоятельное доказательство canonical CLI import/export. Готового собственного Settings dataset у этого чата нет.

## Ограничения и передача workers

- Две **независимые партии рецептов по 12**: CW-B1 и CW-B2. Это задания будущим capture workers, а не уже готовые 12 source packages.
- Общий foreground/pointer сериализуется. IAB viewport override также нельзя одновременно менять двум workers; использовать раздельные поддержанные contexts либо очередь.
- Desktop ≥1000 CSS px; mobile shell подтверждать фактически. Responsive browser не обозначать native iOS acceptance. [Browser routing](/Users/eugenepotapenko/Projects/playphrase.me/playphraseme-site/qa/browser-routing.md).
- Без login, QR, покупок, отправки сообщений, download/copy действий и обхода gate. Favorites, Account, Statistics и региональные платёжные формы исключены.
- Только task-owned контекст; не очищать чужие cookies/storage и не менять preferences ради доступности панели. Временные draft/панели/playback восстанавливать.
- Для каждого сборщика pin фактического source/build/target обязателен; текущий checkout не доказывает соответствие загруженного bundle.
- Нулевой результат поиска, loading/error и locked states засчитывать только после настоящего воспроизведения; здесь они не добавлены как готовые дополнительные кейсы.
- Для DESIGN-ANALYSIS сравнимое семейство выбирается явно: например genre chips или строки одного dropdown. Округлённые совпадения, намеренные исключения и unknown не превращать в правило/дефект.

**Счёт CW1:** 24 discovered recipes; 2 captured families с 3 сохранёнными viewport-состояниями; 0 новых captures/generations/evaluations в этой задаче. До 1000 остаётся реальный corpus gap — его нельзя закрыть размножением размеров этих экранов. Следующая зависимость: конечные capture packets CW-B1/CW-B2 с выделенными ресурсами и фактической проверкой доступности.
