# Runtime component export under the existing output limit

Owner: [outputLimit fix](codex://threads/01a20ed3-77b9-78c0-9ab3-ce714c6bb86c).
Status: released in0.9.0; Practice consumer acceptance passed; other consumer cases remain open.
Release owner: [builds/releases](codex://threads/01a20929-8e9a-7830-834d-3e558e9ff18d).

## Basis and scope

The defect assignment and explicit continuation “Продолжай реализацию уже
порученного исправления сейчас” authorize the presented SDK/caller subtree plan.
SDK owner01a20a46 handed off the shared runtime files and finite review.
Work remains on master; baseline c44f6495, unrelated later b587be92 preserved.
Mode: bounded Evolve of component selection, not a whole-window format rewrite.

Traversal: specs/README → SWIFTUI-RUNTIME@4 → BOUNDARIES@1, MODEL@1, IDENTITY@1,
GEOMETRY@2, PRIVACY@1, EXCHANGE@2, LIFECYCLE@1, CACHE@1, PROJECTIONS@1, CLI@21.
Implementation/QA and product-truth core/routing/change/evidence/delivery/coordination
contracts govern. Runtime@5 registers the additive selection before source edits.
Contract requires fresh bounded source facts; released implementation only selected
windows and post-filtered --type. The approved proposal adds explicit pre-encoding
selection, preserving old commands, core0.1, privacy and all limits.

Write set: runtime Collector/Encoding/Session, caller, Swift/Python tests and owned
fixture, runtime contract/registry, runtime README/skill reference, this handoff.
No consumer application, installed package, release version or packaging change.

## Verified outcome

- Swift runtime tests:9/9; Python caller tests:9/9.
- A real owned SwiftUI fixture produces7595 source nodes. Whole capture refuses
  outputLimit without a Snapshot; scoped capture retains301 subtree nodes and56
  explicit ancestors,1006435bytes. Canonical Rust validation passes.
- Actual selected root is SwiftUI.SpacingLayout, a unique observed container in
  the fixture GroupBox. It contains Button/Image/Text parts. GroupBox itself has
  two source occurrences; no first-match rule is used.
- One binary/process: Rust width120→180→120pt; removing the component refuses
  subtreeNotFound, duplicating it refuses ambiguousSubtree. Four concurrent
  requests have distinct observations and intact graphs.
- A1100-row source still refuses sourceLimit before selection. Pre-dispatch
  cancellation, stale window generation and malformed selectors publish no file.
- Actual0.8.0 caller against the new SDK still captures the whole411-node small
  window; --type does not change saved scope. New caller refuses old endpoints
  lacking the capability before publishing a scoped request.
- Source keys, local geometry, transforms, inherited defining ancestors and child
  order remain. Retained subtrees omit no children; excluded context branches are
  explicitly counted. Canary is absent from all retained canonical captures.

Evidence: external task directory
`/Volumes/Develop-2TB/Develop/tmp/uib-subtree-01a20ed3-x8x2x_d0/evidence/final`,
particularly summary.json and phase snapshots. It contains only owned fixture
observations. Internal app/runtime belongs to the same chat under
`/tmp/uib-subtree-01a20ed3-vg6wc94u`; private descriptors are not handoff inputs.
Reproducer source is tests/bridges/native/SwiftUIRuntimeSubtreeFixture.swift;
normal CLI test is tests/bridges/swiftui_runtime_subtree_check.py.

Independent compatibility/privacy review: owner01a20a46 found no blockers,
reran Python9/9 and diff-check, and independently checked four canonical snapshots
for301 reachable subtree nodes plus56 distinct ancestors, closed links and canary
absence. Swift/live checks remain author-run evidence, not reviewer-run captures.
Reviewed source SHA256 prefixes: Collector8d4efa6dce950f10,
Encodinga59e06b1676bd6fc, Session308f8b2981e03603, callerf02b1fb180e36e62.

## Consumer command and unresolved acceptance

After release-owner installation and matching app SDK rebuild, use:

```sh
python3 -B "$CALLER" capture --session "$SESSION" \
  --window "$WINDOW" --generation "$GENERATION" \
  --subtree-type "$EXACT_SOURCE_TYPE" --out "$NEW_SNAPSHOT"
"$UIB_BUNDLE/uiblueprint-validate" --max-bytes 16777216 "$NEW_SNAPSHOT"
```

Choose an exact nominal type from an actual source observation, including its
module. A source-code View name alone is not proof it exists as a runtime node.
When a smaller authorized state can be fully captured, its tree can establish a
candidate; the new request resolves uniqueness again in the desired state.
Do not hide rows or change the target scenario to claim acceptance. If the desired
root is absent or ambiguous, report that unsupported selection; no public bounded
discovery listing for an overflowing full tree is added by this package.

Whole-window outputLimit remains open. In the small whole fixture, repeated
properties occupy553844bytes and declarations253364 of roughly988KB; this supports
payload duplication as a cost, not a measured diagnosis of consumers' private trees.
Custom ScopeDialog is not a standalone source node even though its content is
present; such source omissions are not fixed or relabelled as not-mounted.
Actors, Random and the separate native resize/dialog chain remain consumer-owned
and unverified. Practice acceptance below does not cover full-window export.


## Practice consumer acceptance — 2026-11-24

[Practice S15 owner](codex://threads/01a20912-a8de-7040-b51f-6ae6df2d4585)
reported three validated real-app captures using the unique runtime selector
`PlayphrasemeApp.MacPracticeRoot`. This is consumer-run evidence, not another
capture by the tool owner. App source5223c1e25; SDK/caller release0.9.0,
commit a4d1c2906f2967fa30c3e0d8b87fc353bfde3702; same PID50264/window15518.

| Practice state | Subtree nodes | Ancestor context nodes | Layout size, pt |
| --- | ---: | ---: | --- |
| Empty | 1337 | 261 | 560 × 355 |
| 2/2, collapsed | 1616 | 261 | 560 × 413 |
| 1/2, expanded | 1633 | 261 | 560 × 740 |

For these states use `--subtree-type PlayphrasemeApp.MacPracticeRoot` in the
command above. Full export is no longer needed for this accepted component.
Other screens and the existing whole-window outputLimit remain outside acceptance.
