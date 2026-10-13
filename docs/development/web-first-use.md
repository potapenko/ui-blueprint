# First Web observation with UI Blueprint

Use this path for an **already authorized, existing CDP page and selected component**.
It is read-only. It does not launch a browser, change a profile, navigate, select a
page, focus anything, modify the site or require a model. Node.js 24+ is needed
only for this optional helper; the Rust CLI/worker do not require Node/Playwright.

## 1. Check the bundle and read help

Set BUNDLE to the absolute Web or combined bundle directory you built. The following
works from any cwd, including `/`, without the source checkout:

```sh
python3 "$BUNDLE/distribution.py" verify --destination "$BUNDLE"
node "$BUNDLE/web-geometry.cjs" --help
```

The help command exits 0 without connecting, validating executable paths or
creating temporary files. `-h` is equivalent. Keep CLI and worker from the same
verified bundle. Core/Native-only bundles do not include this Web helper.
Old `ui-blueprint-local-1` bundles do not contain it; build a new bundle with the
current distribution recipe rather than copying files into an old inventory.

## 2. Establish the existing connection and exact identity bundle

A browser being open, a URL, or an existing browser-tool session does **not** by
itself establish an accessible CDP page endpoint. Your authorized caller must
supply the following facts from one current page/document and selected element:

| Shell variable / helper flag | Required fact / supported source |
| --- | --- |
| CDP_PAGE_WS / --endpoint | Existing `ws://127.0.0.1:PORT/devtools/page/TARGET` (or numeric IPv6 loopback) endpoint for the exact authorized page; not a browser endpoint |
| TARGET_ID / --target-id | `Target.getTargetInfo` result `targetInfo.targetId`, type `page`, matching the endpoint |
| FRAME_ID / --frame-id | Root `Page.getFrameTree` result `frameTree.frame.id` |
| LOADER_ID / --loader-id | The same frame's original `loaderId` |
| DOCUMENT_BACKEND_ID / --document-backend-id | `DOM.getDocument` with `{"depth":0,"pierce":false}` → `root.backendNodeId`, nodeType 9 |
| ROOT_BACKEND_ID / --root-backend-id | An element chosen through the caller's existing authorized CDP node picker/selection; `DOM.describeNode` with `{"backendNodeId":ID,"depth":0,"pierce":false}` confirms it is an Element, nodeType 1 |

These are **protocol metadata requests**, not a manual geometry/snapshot adapter.
The helper does not take CSS selectors, names, screen coordinates, a browser-tab
index, or another tool's node IDs. The root must have been selected in that original
frame/loader/document. Never combine an old root with newly fetched loader metadata
or guess a backend ID from a DOM `id`. Navigation/remount requires fresh authorized
selection. The helper and worker revalidate continuity; they never repair stale refs.

**If your current caller cannot supply this identity bundle, stop here.** UI Blueprint
currently has no shipped first-connection/page/root discovery or picker command.
This is an explicit utility bootstrap dependency, not evidence that the collector
failed. Return the missing prerequisite to the UI Blueprint owner. Do not change
PlayPhrase.me, add application IDs, start a different browser/profile, expose a debug
port or install infrastructure merely to satisfy this guide. Such operations need
their own established scope and are not part of this recipe.

## 3. Run the existing supported helper

Once the trusted caller has supplied all six identity variables, run:

```sh
node "$BUNDLE/web-geometry.cjs" \
  --cli "$BUNDLE/uiblueprint" --worker "$BUNDLE/session-worker" \
  --endpoint "${CDP_PAGE_WS:?existing authorized CDP page endpoint required}" \
  --target-id "${TARGET_ID:?selected page target required}" \
  --frame-id "${FRAME_ID:?original root frame required}" \
  --loader-id "${LOADER_ID:?original document loader required}" \
  --document-backend-id "${DOCUMENT_BACKEND_ID:?original document backend ID required}" \
  --root-backend-id "${ROOT_BACKEND_ID:?selected element backend ID required}"
```

You do **not** write connection/request JSON or copy Rust structs. The helper reads
bounded identity metadata, creates those inputs in its own system-temp directory,
and invokes the real Rust `observe`, `inspect --view design`, and `measure` commands.
Its session/observation-scoped nonce is caller bookkeeping, not a fabricated browser
identity. Rust collects and computes geometry; the helper formats the actual result.
It removes its own temporary inputs after use, never the browser or application.
No helper setting expands the fixed small-component profile: 32 output nodes,
depth 8, 64 KiB output, 250 ms request deadline; collector cap 16 nodes and finite
metadata/visit limits. This is not a whole-form/page completeness promise.

## 4. Interpret the result and refusal

Successful reads print design inspection, reported component layout bounds,
width/height measurements with original units/evidence, and coverage/observations.
Names/roles are not requested by this geometry example. Layout bounds do not stand
in for hit/visible regions, inferred padding or unavailable transforms.

- Exit 0: complete result within the requested scope.
- Exit 4: partial/unknown/unresolved. Useful sourced measurements may still be
  present; inspect coverage. Never count missing facts as success.
- Exit 2: invalid arguments/identity format or a bound refusal.
- Exit 1: IO, connection setup timeout or unconfirmed owned cleanup.
- Other product exits are preserved; diagnostics contain fixed codes, not UI text.

`stale_document`, `stale_target`, `target_unresolved` and `root_unresolved` require
checking the exact current binding with the authorized caller; do not substitute
another page or retry with weaker checks. Connection availability and permission
must already exist. A new explicit request is a new operation, not automatic retry.
The helper performs no action and produces no durable observation file for replay;
for saved canonical data use the existing explicit Observe connection/request API.

## Current delivery boundary

Safe help and packaging are qualified separately from live application acceptance.
The existing helper has own-fixture first-use evidence; this guide does not claim
that PlayPhrase.me has been observed through the new installed bundle. The USE-WEB1
consumer stopped before Observe and still needs its authorized CDP identity handoff.
A future bootstrap command must retain exact target selection, stale/ambiguous
refusal, budgets/privacy and the existing Rust collector. Documentation cannot
create those missing runtime capabilities.
