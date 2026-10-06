# D04 — identity and on-demand freshness

- Domain: `uib.development.d04`; accepted/released baseline: none.
- Authority source: [C01-DEC-001](README.md#meaning-and-precedence).
- Node type: leaf; contract: `UIB.D04@1`; clause: `UIB.D04.CONTENT`.
- Authority: Active / Stability: Evolving; policy chosen, adversarial proof pending.
- Read when: adapter refs, cache invalidation or actions are implemented.
- Do not read when: only a stored immutable geometry calculation is affected.
- Requires: [IDENTITY@1](../../product/identity.md), [NATIVE@1](../../product/native.md),
  [CACHE@1](../../product/cache.md), [ACTIONS@1](../../product/actions.md),
  [PRIVACY@1](../../product/privacy.md), [evidence](evidence.md).
- Owner/deadline: W01/M01 binding before live refs; K02/A01/W02/M02 proof before P4/P5.

## Requirement and evidence

Source keys and action refs are different. Repeated titles/labels are legal.
R01 proves backend IDs change at remount and loaders change on navigation; old
handles refuse input. F02 proved a reopened window may reuse its CGWindowID.
Its own manifest is a fixture oracle, not a universal external identity source.

## Chosen policy

- A session owns target generation; target process/tab identity includes its
  incarnation. Each surface has its own generation. Browser binding includes
  target/session/frame/document-loader/backend node identity and connectivity.
  Unknown document continuity invalidates affected refs instead of guessing.
- Native matching uses public process incarnation and independently established
  AX/window owner binding. Own explicit IDs/receipts are accepted evidence only
  for an instrumented fixture; PID/title/CG ID/rectangle alone is insufficient.
  No private proc_pidinfo flavor from upstream. If identity remains unresolved,
  return target_unresolved/minimal candidates within authorized scope.
- Backend refs bind observation and generations. Fresh resolve checks target,
  surface, unique source node, capabilities and required action preconditions
  immediately before dispatch. A locator re-query returns a **new** ref; it never
  silently repairs an old ref by role/name/nth/coordinates.
- A new explicit observe collects requested current fields. Local inspect/check
  may read immutable snapshots and must show their freshness/coverage. TTL alone
  cannot make them current. No polling interval or automatic observe on events.
- Optional events invalidate affected state. Lost events require bounded resync
  when the next explicit operation needs current data. Scope/projection changes,
  pagination and cache eviction do not by themselves prove deletion.
- Diff may use marked heuristic matches but they never authorize an action.
  Explicit component mapping can link source IDs; geometric equality cannot.
- A known allowed popup can be an included surface with sourced anchored_to/
  initiated_by and separate owner. Unknown ownership/binding stops dependent
  action; it does not silently expand a window-only request to all applications.

Rejected: global IDs made from PID/title; survivor cache; old coordinate fallback;
private identity calls; full-app search after failed scoped lookup; fixed-rate
freshness. These contradict tested remount/reopen behavior or explicit scope.

## Required proof

W01/M01: same-label/title ambiguity, wrong target, remount/navigation,
close/recreate/reused ID, missing mapping and unknown process continuity, with
zero cross-target effects. W03/M03/K02: missing event, parent/font changes,
permission change, evicted base and scope switch; resync preserves other sessions.
A01/W02/M02: stale/ambiguous ref refuses before dispatch; loss of certainty after
possible delivery stops dependent actions and never repeats submit. Revalidate
input modality outcome, not just API return. These remain positive/adversarial
acceptance tasks, not completed by policy selection or fixture receipt review.
