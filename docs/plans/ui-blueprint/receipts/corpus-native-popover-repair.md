# CA3 — explicit focused-popover acquisition

Implementation and bounded live qualification complete, 2026-10-11; independent
review pending. Frozen product source:8ebbd2c06b4ffe60e8bee20b41985ec2542d7465.
No ImageGen, canonical PlayPhrase.me acceptance or whole-platform acceptance claim.

## Authorized envelope and contract delta

Root explicitly transferred CA1/CA2 diagnostic ownership to CA3 shipping_product
under the user's CORPUS-1000 repair mandate, accepting [CA2 binding proof](corpus-native-acquisition-gap.md).
Mode Evolve; change CA3-NATIVE-POPOVER-001. Before source edits registered registry38,
[NATIVE@3 AX-POPOVER](../../../specs/product/native.md#uibnativeax-popover--explicitly-selected-focused-popup) and [CLI@17 OBSERVE](../../../specs/product/cli.md).
Traversal: registry37 → product routes → NATIVE@2/CLI@16 and unchanged
IDENTITY/GEOMETRY/PROJECTIONS/EXCHANGE/MODEL/PRIVACY/LIFECYCLE closure; D05@4 /
Native acquisition@3 explicit dependencies reused. Global implementation, truth
change/evidence/delivery, QA/Computer Use/Apple/operational routes applied.
Native source-app route: AGENTS → specs → Clip Search → macOS filter popover@1/r2,
behavior part2/autofocus r6; UI-tools@1 → runtime/lanes → build/test/tooling.
Skill: build-macos-apps:build-run-debug, using existing wrapper and owned temp roots.
Protected: default focused-window behavior, core0.1/analysis0.2, existing4032-byte
private config shape,160/depth9 ceilings, permissions/errors/identity/privacy,
fixture providers, export/engine/renderers and source apps. No agents/worktrees.

## Shipping behavior and owners

Use existing provider native_ax; change only trusted configuration collection from
focused-ax to focused-popover-ax. Default remains whole-window, with no focus-element
read. Config still contains collection/process/scope_id/acquisition_limits only.
NativeFocusedAX resolves at most16 public parent-chain nodes from current focused
element to the held AXFocusedWindow and selects the nearest AXPopover. Owner checks
precede role reads; before complete binding there are no labels/values/geometry/
child reads. Missing/foreign/cyclic/no-popup chains refuse, never fall back.
Hold all original chain objects and compare process/window/focus/each chain/root
before collection, after collection and after encoding. Final focus is reread after
chain verification too. Changed identity gives stale_target; quota incomplete_scope.
Every selection focus/parent read is precharged against child_entries, separately
counted from returned children; role copies use existing text budgets. The shared
collector's remaining child allowance subtracts selection charge. No new quota.
Collect existing bounded AX data from popup root; preserve the parent window's sole
observation-scoped Surface. No fabricated anchor/Surface, stable ref, capture or action.
Owners: plugins/macos/NativeFocusedAX.swift and NativeAcquisition.swift; existing
WindowAX collector unchanged. Tests: acquisition/FocusedAXChecks.swift. Docs:
spec registry/product routes/native/cli plus native-helper ordinary-AX section.

## Verification

- Swift6/macOS14 target compilation passed;98 focused assertions passed.
-31 emitted synthetic canonical responses passed frozen uiblueprint-validate.
- Positives: nearest nested popup/self-popup, requested node cap, secure redaction,
  default whole-window with zero focus reads, unchanged shared array accounting.
- Negatives: missing/no-popup/cycle/foreign focus/foreign parent/other window,
 16-node chain cap, initial/per-role text cap, selection/revalidation entry caps,
 output cap, rejected161 nodes/depth10, focus/window/parent/root/process/owner
 changes, precollection focus change and final publication focus change. Failures
 publish only canonical failure; invalid prebinding cases perform no content reads.
- Local Markdown links/route revisions and git diff --check passed.
- Frozen native Release distribution built with distribution.py; manifests/hashes
 verified. No shared target/debug overwrite or dependency change.

## Actual CLI/live results

Owned PlayPhrase.me Release development build: source c7d8e3dcda9fb5c03fc8c0a96c29420d76e450be,
beta-01, existing modified project.pbxproj included unchanged (SHA256
7b26d3ad560da98ae06d406da7b7397238075009eb648e7b6df22ff89530ee75).
Build: ./script/build_and_run.sh macos build --development
/private/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/playphrase-ca3-popup-live.PYDK6U.
Exact contained Release/PlayPhraseMe.app launched by bounded open -n after realpath/
bundle validation, avoiding known wrapper lexical /var versus /private/var defect.
App SHA256 8a6e03a08577358043cc23af74d81e3f970bd9df5217f66588a1077416e4a957;
PID3568/start18:08:09 Europe/Podgorica; macOS27.0.1/26A434. Desktop token
d7a2fead6c4f4c9296f8819e40a20658 owned for entire CUA action/state/result chain.
Safe path: paused Search → Clip → Genre → Escape → Director idle → public Steven
draft →10 results → clear draft → Escape → original Search. No filter selected.

| Case | Actual result |
| --- | --- |
| No popup/new mode | target_unresolved,0 nodes; no fallback |
| Default Search window |160 nodes, original AXWindow root |
| Genre/new mode |20 nodes;14/14 named options with known AX bounds |
| Genre/default mode |160 nodes;0 interior option identifiers, preserving old scope |
| Director idle/new mode |9 nodes; popup/query/idle controls |
| Director Steven/new mode |24 nodes;10/10 result rows with known AX bounds |

All6 raw live responses passed canonical validation. Observe exits4 honestly:
coverage partial, omitted/unknown counts unknown; separate API reads/consistency
unknown. All selected option/row rectangles are accessibility_bounds in
ax-screen/screen/pt/top_left; pixel transform unknown. No grid/card descendants in
popup outputs. Existing inspect design succeeded for a Genre option and Director
row. Example reported sizes: Genre Action191×26pt; first Director row386×28pt.
These are source facts, not normative layout requirements or pixel measurements.

## Exact invocation and frozen handoff

B=/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CA3-live-z1t52z0d.
Actual invocation (each case retains exact connection/request/process/argv/output):
`$B/tools/uiblueprint observe --connection $B/genre/connection.json --request $B/genre/request.json --worker $B/tools/session-worker --max-input-bytes 2097152 --max-output-bytes 524288`.
Configuration collection is focused-popover-ax; fresh future calls require actual
current process incarnation/new request and observation Surface, never replay the
retired PID3568. tools/distribution-manifest.json pins all binaries/source; handoff.json
indexes six cases. Helper SHA256 f9851685cacd5a9bb81191632bba342769bb39d1c80ebca00edad8465287c75f;
CLI704a61945b144fad21d94a99222f28333ff524cd6e8a743b0a1d44aa93359b79;
worker e18b451a8b1aaa51ab51278e980fd898b823912a3ae7da6811be868a5fccb514.
Frozen bundle/canonical inputs/responses are explicit handoff to CORPUS root through
independent review and consumer acceptance. Two screenshots and containing dirs
remain system-temp and are never agent-cleaned. No generated evidence committed.
Restored empty Director draft, Any genre/director, Common Phrases, paused121/426.
Exact owned app terminated after path/start revalidation; absence verified. User
PID63105 unchanged. Desktop holder:null; own guard/caffeinate stopped. App build root
and synthetic compile/test root removed and absence verified. Storage cleanup:
completed, with named frozen consumer handoff retained. Source app changes untouched.
Residual: root-assigned independent review; Cast/Voice and broader platform coverage
unqualified. No general precision/performance gate or complete native design claim.
