# CR1 — independent Native acquisition and export review

2026-10-11. **Native: reject; export: accept_with_residual.** One coherent review,
with initial independent observations recorded before the author handoff. This
does not accept the complete corpus, any generated image, or a release.

## Scope, authority and selected basis

Root assigned CR1 under the explicitly coordinated CORPUS-1000 work. Source was
read-only; the sole write is this receipt, on current `master`. No agents, live app
operation, source-app inspection, source fixes, branch or worktree was used.
Candidates: Native `8ebbd2c06b4ffe60e8bee20b41985ec2542d7465`; export
`1650d0c61a87427df7f1eb0e1c9703d60fcde827`. Reviewed source still matches both
candidates at reconciliation HEAD `e472f1846d80189b52ece0be1a538762b1f929ed`.

Traversal: global/repo AGENTS → implementation/truth/QA routes → registry38 →
product branch → [NATIVE@3](../../../specs/product/native.md) AX-READ/AX-POPOVER,
CLI@17 OBSERVE, Native acquisition@3/D02@2/D04@1/D05@4 and explicit dependencies;
[EXPORT@2](../../../specs/product/export.md) → full drawing closure and synthetic
example → DRAWING@1.2 HBP/R2/R3, CLI-EXPORT@2, DESIGN-ANALYSIS@1/ANALYSIS@2 with
TYPES/VALIDATION@1. Shared MODEL/IDENTITY/LIFECYCLE/PRIVACY/GEOMETRY/PROJECTIONS/
BOUNDARIES@1 and EXCHANGE@2; D01/D03/D06/D07, memory/work, forms/actions/cache,
roadmap, pilots/golden/performance, evidence/reuse, RUST/DEV.RUST dependencies read.
Historical revision labels in dependency links resolve to current additive leaves;
no material authority conflict found. Product requirements come from these clauses
and the packet, not tests or author success claims. Mobile, new actions/capture,
performance qualification, unrelated subsystems and image acceptance are excluded.
Resolved context comprises the selected contracts, two diffs/owners and focused
tests, original CW06/CW07 DOM/canonical inputs, then the two author handoffs below.

## CR1-F1 — P1: retain the binding through actual publication

**Implementation defect; reject this criterion.**
[NativeFocusedAX.swift](../../../../plugins/macos/NativeFocusedAX.swift):198–209
performs final selection validation, then returns only `NativeJSONFrame`.
`PopupBinding`, its complete retained chain, application and window are local to
`collect`. [HostHelper.swift](../../../../plugins/macos/HostHelper.swift):43–44
calls `io.reply` only after `collect` returns. `NativeJSONFrame` owns bytes and
stream state, with no retained binding or publication validator. Its write loop
may wait for a writable descriptor after that ownership has ended.

Independent reproduction used unchanged shipping sources with a task-temp Swift
harness: NSObject handles for application/window/popover/focus, injected public-AX
boundary, weak references to all four handles, successful collect, removal of the
fixture's external strong references, and `withExtendedLifetime(frame)` before
reply. Result: **0/4 handles retained; 4,405-byte observed frame still alive**.
The same result held after adding ten invalid-config cases. This proves the
ownership gap; it does not claim a live cross-window disclosure was observed.

NATIVE.AX-POPOVER requires retaining the complete chain/root for this Observe;
AX-READ and the CR1 packet require held identity through publication. The author
tests called publication-focus exercise the final check *inside collect*, not the
subsequent reply boundary. Positive live snapshots cannot discharge this gap.
Required correction: retain the authorized binding through successful/failed reply,
keep final validation at that publication boundary, and prove handle lifetime plus
no successful publication after an injected boundary identity change. No source
fix or contract relaxation was made here; no user product decision is needed.

## Independent checks and scoped decisions

| Criterion | Verdict | Evidence / limit |
| --- | --- | --- |
| Explicit selector, no fallback, prebinding scope | accept | Owner before role; structural-only bounded chain; missing/foreign/cyclic/other-window/no-popup refusal; ten additional malformed selector/extra-config rejections |
| Ancestry quotas and collection | accept | 16-chain, shared selection/child allowance, repeated text/entry charges; existing subtree collector; 160/depth9 ceilings unchanged |
| Changes before/after collection and after encoding | accept | Focus/window/parent/root/process/owner mutations refuse; initial/bounded failures contain no prebinding content |
| Held identity through reply | reject | CR1-F1 above |
| Default behavior/canonical compatibility | accept | Default has no focused-element reads; original whole-window path, core0.1 and limits preserved; 31 synthetic plus six retained live responses validate |
| CW06 label association | accept | Raw dom-134 SELECT belongs to dom-133 LABEL “Offensive :”; sibling dom-131 is “Only slang”. Own name, directed LabelledBy, direct LABEL parent, then neutral fallback replace proximity guessing |
| CW07 source formulas/numeric fidelity | accept | Four source Public VisibleText formulas survive exactly; dimensions/sheets unchanged, CW06 scene unchanged, CW07 changes only four text property states |
| Export privacy/input boundary | accept_with_residual | Exception restricted to caller-allowlisted VisibleText; sensitive/raw-value/unreviewed/name/metadata/IDs remain protected by existing rules; lexical ambiguity still requires actual caller review |
| General SDK lifetime, platform/corpus/image acceptance | not_verified | No new live experiment or universal secret/path detector claim; full generated-image acceptance remains zero |

Checks ran in unique system-temp `uib-cr1.EOTHUF3br4`, with exclusive
`CARGO_TARGET_DIR` and Swift module cache. Swift6/macOS14 compilation and the
98 existing focused assertions passed; the independent extension passed ten more
invalid selector/config assertions. All 31 emitted canonical responses validated.
`cargo test --locked --offline -p uiblueprint-export --test corpus_fidelity` passed3;
`--test compiler --test design --test protected_lifecycle` passed38+4, with the one
protected-lifecycle fixture test explicitly ignored for absent V02 records/secret.
That skip is not privacy acceptance for a new live secret-delivery campaign.

Original CW06/CW07 inputs were inspected before author results. Actual CLI replays
used the original 8,000,000 input /16,000,000 output /600 components /20 views /
150-per-detail limits. Both succeeded, at 8,299,255 and4,570,400 package bytes.
Earlier reviewer-selected smaller caps correctly refused before output creation;
they were not treated as product failures. No input, geometry or provenance was
changed to obtain success. Compared with the original packages, dimensions/sheets
remain exact, CW06 scene is identical and CW07 only restores the four formula states.

## Author evidence reconciliation

Second-stage handoff explicitly authorized reading
[CA3 receipt](corpus-native-popover-repair.md) (`d767d6c`) and
[CP2 receipt](corpus-export-fidelity-repair2.md) (`ac3d18a`) after initial findings.
CA3 retained root: `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CA3-live-z1t52z0d`.
`handoff.json`, `tools/distribution-manifest.json` and all six case requests,
connections, process records, invocations and original responses were reconciled.
Four executable hashes match the manifest; response hashes/bytes, request IDs,
Target/process binding, selector, fields and160/depth9 limits agree. The independent
validator accepts all six. Data confirms default-window160, Genre20 with14 named
options/known bounds, Genre-default160, Director-idle9, Director-results24 with10
named result rows/known bounds, and no-popup target_unresolved. All exit4 results
retain partial coverage/unknown counts; one parent Surface, AX screen/pt bounds and
unknown pixel mapping are preserved. This is retained evidence from an owned
PlayPhrase.me Release development instance, not a fresh independent live run.
Retired PID3568 was never operated. CR1-F1 remains despite these valid positives.

CP2 retained root: `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CP2-y2hhlpr7`.
Manifest binary hash and all four source blobs match. Each author's six-file package
is byte-for-byte identical to the independent matched-limit replay; prompt hashes
and27,210/14,040 character counts agree. The73-test author claim is attributed;
CR1 independently ran the45 focused Rust tests above, not all73. No image was
visually accepted by CR1; CP2 itself reports zero complete accepted images.

The notation guard is defense in depth, not a path/secret oracle. For example,
syntactically ambiguous `A/Accounts` can satisfy the symbolic rule; it is publishable
only if the source classification and caller's explicit public review are correct.
This residual is consistent with CLI-EXPORT.INPUT's reviewed-field policy and the
author's stated bound. It must not be represented as automatic safety for arbitrary
private UI text. Known private IDs remain aliased and path/URL/template/control-byte
canaries stay refused; no new permissions or metadata exception were found.

## Disposition and saved state

One actionable finding, CR1-F1; no export correction requested. Native acceptance
must remain open until the publication ownership fix and its focused proof return
to this same review. Existing positive evidence need not be repeated wholesale.
The final handoff provides this receipt's exact checkpoint SHA and push result.
Unrelated changes and every existing image were untouched. Run-owned non-image
build/check/replay files are removed after their results are recorded; author
handoff roots and original images remain under their existing retention ownership.
