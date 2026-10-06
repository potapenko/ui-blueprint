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
