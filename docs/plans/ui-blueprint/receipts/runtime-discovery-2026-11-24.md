# Bounded runtime discovery for Actors applicability

Owner: [runtime defect chat](codex://threads/01a20ed3-77b9-78c0-9ab3-ce714c6bb86c).
Status: fixture-qualified and independently reviewed; release and Actors acceptance pending.

## Scope and authority

The Actors continuation requests bounded discovery or a confirmed selector after
0.9.0 whole capture returned outputLimit and symbol-derived ActorPickerView returned
subtreeNotFound. The coordinator explicitly assigned the missing discovery work;
the SDK owner confirmed continued single-writer ownership and finite review.
This implements the original source-backed component-discovery outcome. It does
not change the application or infer a runtime node from symbols/marker geometry.

Basis: the [prior subtree closure](runtime-subtree-2026-11-24.md#basis-and-scope)
remains applicable. Read current Runtime@5 and implementation governance; register
[Runtime@6 RUNTIME.DISCOVERY](../../../specs/product/swiftui-runtime-probe.md#runtimediscovery--source-types-without-canonical-geometry-export)
before source edits. Additive Evolve from released0.9.0 a4d1c290; current branch
master, baseline dae31f3b. Protected: old full/scoped capture, core schemas,
geometry, privacy filtering, source/output limits, concurrency/cancel and app state.
Write set: RuntimeEncoding/Session, caller, Swift/Python tests and discovery harness,
runtime spec/registry/README/skill reference and this receipt. No release packaging.

## Verified mechanism

`discover` reads the selected window freshly, then writes a separate version1.0.0
source inventory. It includes bound observation/run/window identity, exact type
counts and compact parent/type-index topology for every admitted source node.
It exports no model/UI text, raw pointers, generic signatures or geometry. The
original opaque getter and source limits still apply. Output stays16MiB; stdout
is a compact saved-file receipt. Old SDKs refuse before request publication.

- Swift11/11 and Python11/11 passed; diff-check passed.
- Owned large fixture: full capture outputLimit; discovery7595 nodes,94 types,
  473733bytes. Discovered unique source root SwiftUI.SpacingLayout then yields
  a valid357-node scoped Snapshot containing Button/Image/Text parts.
- Between discovery and capture: removal refuses subtreeNotFound; duplication
  refuses ambiguousSubtree. Fresh discoveries report absent/type count2, with
  distinct observations. Topology is closed and counts exactly match occurrences.
-1100 rows still refuse sourceLimit; stale bindings and pre-dispatch cancellation
  publish no discovery file. Swift tests cover unknown types/partial coverage and
  output byte overflow. Python tests cover old endpoint capability refusal,
  request-derived names, hash verification and cleanup preserving unrelated files.
- The existing subtree live suite passed again: Rust width120→180→120, four
  concurrent distinct captures, canary absent, negative cases and canonical validation.
  Actual0.9 caller against new SDK:411-node full and357-node scoped capture pass.
- SDK source collection/geometry code unchanged; discovery stores the source's
  actual missing/custom-type boundary, not invented identities or stable handles.

Evidence (owned fixture only):
`/Volumes/Develop-2TB/Develop/tmp/uib-subtree-01a20ed3-x8x2x_d0/discovery/evidence`,
with discovery/summary.json and subtree/summary.json. The owned process stopped.
Internal candidate app/private runtime are under the same task's `discovery`
subdirectory in `/tmp/uib-subtree-01a20ed3-vg6wc94u`; descriptors are not handoff data.

Independent review by SDK owner01a20a46 found no blockers. Reviewer reran
Python11/11 and diff-check, checked four actual discovery artifacts for closed,
ordered, same-host topology and exact type/count/unknown/binding/canary properties,
and compared the411-node inventory to its canonical source counts. Reviewer did
not build or capture an app; Swift/live evidence remains author-run.
Reviewed SHA256 prefixes: Encoding33ee9860f2e2e087, Session474706d4cdb0784e,
caller068e52cd886a55a1.

## Consumer handoff

After matching SDK/caller release and owner-controlled rebuild:

```sh
python3 -B "$CALLER" discover --session "$SESSION" \
  --window "$WINDOW" --generation "$GENERATION" --out "$NEW_DISCOVERY"
```

Read `types` (name/count) and `nodes` (index/parent/hosting_root/type_index) in that
file. Use source hierarchy and the actual app state to attribute a candidate.
A count of1 establishes uniqueness in that observation only. A subsequent
`capture --subtree-type "$EXACT_SOURCE_TYPE"` reads again and revalidates uniqueness.
No ActorPickerView selector is confirmed here. If no attributable unique root
exists, report the remaining selection limitation, not success on another node.

Whole-window canonical outputLimit remains open. Discovery is not a Snapshot or
geometry pass and is not input to the Rust geometry validator. The Actors owner
alone operates its window and performs real component acceptance. Packaging,
installation, publication and version selection remain with the release owner.
