# CORPUS-1000 — case count reconciliation

2026-10-12. CCOUNT1 terminal reconciliation, updated by root after CN-B9 completion.
This counts source-qualified generated/evaluated cases, not visual acceptance.
No source, image, runtime or product code changed during the reconciliation.

## Conservative result

**107 proved eligible cases; range107–110 pending three named uncertainties.**
The planning remainder is893 cases, potentially890 if all three are established.
There are45 Web and62 Native cases in the proved minimum. Nineteen are explicitly
fixture/component-preview origin;88 are non-fixture observations. Fixture UI is
not relabelled as production data. Families and meaningful states remain distinct.

Image buckets:230 primary images =226 first outputs +4 rerolls;
40 separate repair-validation images;270 images in total. Extra sheets, repaired
prompts and repeated renders are not extra cases. A failed visual evaluation can
still complete its declared test protocol; missing source qualification cannot.

## Completed primary batches

| Batch | Proved eligible cases/states | Primary images | Limit or exclusion |
| --- | ---: | ---: | --- |
| CW-B1 |11|12|CW02 is an unqualified occlusion negative |
| CW-B2 |9|9|CW14 generation refused; unavailable/invalid candidates excluded |
| CN-B1 |1|2|A12 counted; A02 qualification unresolved; other initial generation gaps |
| CN-B2 |7|8|B07 partial sidebar qualification unresolved |
| CN-B3 |8|16|12 configurations include3 documented duplicates;4 rerolls; C04 novelty unresolved |
| CW-B3 |6|6|Qualified scrolled/lookup states; unrelated invalid CW-S1 excluded |
| CN-B4 |12|12|Partial qualified states, including3 component-preview cases |
| CW-B4 |9|13|All required per-sheet calls evaluated; gaps/no-op candidates excluded |
| CN-B5 |8|52|Negative old scrolled-grid pair excluded |
| CN-B6 |9|23|7 families,9 states; premium Context gap excluded |
| CN-B7 |1|7|New qualified retained CA5 visible-grid source, not a relabelled B5-09 |
| CW-B5 |5|5|Gated/no-novelty/unavailable candidates excluded |
| CN-B8 |7|11|5 families; no-op Collections excluded |
| CW-B6 |5|5|Natural guest dialog absent; qualifier verdict does not change execution count |
| CN-B9 |8|49|Actual distinct wired fixture compositions; one exact in-batch duplicate excluded |
| **Primary subtotal** |**106**|**230**| |

## One first completion in the repair bucket

**CP1/A06-settings adds1 case.** CN-B1 recorded generation_blocked with no image.
CP1 executed/evaluated its first literal combined-prompt attempt on954ecde; the
second output on4d6e0bf is a repeat, not another case. Other counted Settings
observations are Web/Simulator, not this Mac state.

Source: `uib-CN-B1-_djmb16k/A06-settings/observed.json`;
SHA256 `f0272408a6ff70b1c66142c96337eaa18c4370f5f7e38b1bceb0a9a68340309c`;
observation `be9654a4f0514ff9942d0cd9e23f622f-external_semantics`.
Qualification stays partial AX focused-window/pt/ax-screen, unknown AX→PNG mapping
and non-atomic pairing. Nested Repeat Pause menu completeness is not claimed.
Its images remain in the repair bucket; the unique case counts once.

CP1/A01-narrow-geometry adds0: it reuses historical RC03 from2026-10-09 with the
same clip/query/source screenshot; no new state novelty is established.
Source SHA256 `4ed4b8b1608be11246cb12e931bf4bdb7bbe3266dae4f9cf30cbaa5167039564`;
observation `d641a4242c384f85a53d58c7d7beb5e1-external_semantics`.
CP1 A01-wide and CW14 were exported but not generated. CP2–CP7 use already
represented source cases. Evidence: CP1 images.json/corpus-verified.json/review.md
and CN-B1 manifest. No case is excluded merely because its image is called a repair.

## Three unresolved cases, not counted in107

- **A02:** executed/evaluated, but parent image cuts off the separate menu and menu
  geometry is not covered. A bounded parent-window observation cannot silently
  become a qualified overflow-menu case.
- **B07:** two known AX frames and unknown internal sidebar geometry; any future
  inclusion must remain explicitly limited to the supported observed scope.
- **C04:** changed query/PNG without an observed loading indicator does not yet
  establish meaningful new state novelty.

## Original-protocol correction

The initial CCOUNT1 draft wrongly treated old sheets.json plans as independently
required image calls. Its91–93 total and claim of22 missing calls are withdrawn.
CW06/CW10/CW30/A02 and CN-B4 B4-08/10/11/12 used a declared single combined-prompt
attempt. Those calls occurred and were evaluated; missing depicted views were
rendering failures, not unperformed calls. A02 still has the separate source gap.

The independently executable per-sheet obligation becomes explicit in CP4:
registry38→39, SHEET-PROMPTS1/export0.4 and one attempt per resulting sheet.
Use each original task's actual protocol. Do not relax missing required calls or
retroactively impose a later protocol.

## Evidence locations

All short paths below start at:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/`.
Manifests are adjacent to handoffs unless their handoff points elsewhere.

| Batch | Terminal source |
| --- | --- |
| CW-B1 | `uib-cw-b1-gR3CYr/terminal-handoff.md` |
| CW-B2 | `uib-cw-b2-UhvhiK/handoff.json` |
| CN-B1 | `uib-CN-B1-_djmb16k/handoff.md` |
| CN-B2 | `playphraseme-cn-b2-00wlqx9r/handoff.json` |
| CN-B3 | `uib-CN-B3-3o25e72c/HANDOFF.txt` |
| CW-B3 | `uib-cw-b3-2k5Rlj/handoff.json` |
| CN-B4 | `uib-CN-B4-kllqoe62/handoff.md` |
| CW-B4 | `uib-cw-b4-14K1nY/handoff.json` |
| CN-B5 | `uib-CN-B5-6jmj42i_/handoff.md` |
| CN-B6 | `playphraseme-cn-b6-h7jzxeiz/handoff.json` |
| CN-B7 | `uib-CN-B7-_04jjev_/handoff.md` |
| CW-B5 | `uib-cw-b5-NTbzfs/handoff.json` |
| CN-B8 | `playphraseme-cn-b8-6no3fnyn/handoff.json` |
| CW-B6 | `uib-cw-b6-dRaUXk/handoff.json` |
| CN-B9 | `uib-CN-B9-hf8xty_p/HANDOFF.txt` |

Generation/submission records and literal prompts establish the old combined-call
protocol; individual package manifests/prompt hashes establish later per-sheet calls.
Repair buckets: CP1=4, CP2=2, CP3=12, CP4=13, CP6=8, CP7=1; CP5=0.
Accepted repository receipts retain their source revisions and artifact roots.

CN-B9 completed9 attempted configurations/8 distinct qualified fixture compositions
and49 evaluated first sheets,0rerolls. Its handoff verifies distinct actual
compositions and source hashes against CN-B3; component families may recur and
are not claimed as8 wholly new components. All582 required dimension IDs have one
owner.25scoped author sheet passes/24failures do not constitute full-case acceptance.
CW-S2/CW09 and CN5B remain source/host dependencies. CT3 changes interpretation of
two image verdicts, not the number of executed evaluations.
