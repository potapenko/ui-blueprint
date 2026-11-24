# Fresh SwiftUI geometry requests

This is experimental Mac support introduced in 0.8.0. A prior scoped build
passed real PlayPhrase.me capture and concurrent reads; its resize measurement
timed out. The current normalization passed the general fixture, and the full
real-app state-change sequence remains pending. Report exact failures rather
than silently treating these scenarios as qualified.

Use an integrated app prepared through [SwiftUI setup](swiftui-setup.md).
All commands below use the matching installed
`CALLER="$UIB_BUNDLE/swiftui_runtime.py"` and the owner's private `SESSION`
descriptor path. Normal queries need no build, Xcode GUI, debugger or desktop
input lane.

## Bind the instance and window

```sh
python3 -B "$CALLER" windows --session "$SESSION"
```

Check success and choose the authorized returned `window` and `generation`.
Pass both on subsequent queries. These are this endpoint's window witnesses;
don't substitute an AX identifier, CUA index or title. A sole visible window can
be selected automatically, but explicit binding prevents accidental changes when
other windows appear. Multiple windows require a real choice.

```sh
python3 -B "$CALLER" tree --session "$SESSION" \
  --window "$WINDOW" --generation "$GENERATION" --type SwiftUI.Button

python3 -B "$CALLER" measure --session "$SESSION" \
  --window "$WINDOW" --generation "$GENERATION" \
  --type SwiftUI.Color --relation width --bundle "$UIB_BUNDLE"
```

Each `tree`, `capture` and `measure` call makes a **new runtime read** after the
request arrives. `--type` filters the returned nodes by exact nominal source
type (before `<...>`); it is not a text-label selector or a unique component ID.
Use actual reported types, for example `ExampleApp.SearchRoot`. Repeated types
can produce many matches. Preserve unknown results; don't choose the first
same-sized rectangle and call it the requested control.

Without `--type`, `tree` returns the captured tree with original child links.
`key` identifies a node in that observation only. A key from one call cannot be
used as a stable handle in another. Use the new tree's ancestry and source types
to distinguish repeated components. `measure` supports width/height and delegates
arithmetic to the matching Rust validator/engine. It does not turn missing source
geometry into an inferred size.

## Discover actual types when a full Snapshot overflows

With a matching SDK/caller advertising `discover-types`, save a new compact source
inventory without canonical geometry export:

```sh
python3 -B "$CALLER" discover --session "$SESSION" \
  --window "$WINDOW" --generation "$GENERATION" --out "$NEW_DISCOVERY"
python3 - "$NEW_DISCOVERY" <<'PYTHON'
import json, sys
with open(sys.argv[1]) as source:
    inventory = json.load(source)
for item in inventory["types"]:
    print(item["count"], item["name"])
PYTHON
```

This versioned **discovery artifact is not a Snapshot** and is not input to the
Rust geometry validator. It includes actual nominal types and occurrence counts,
plus source-order `nodes` with `index`, `parent`, `hosting_root`, `type_index`.
`type_index` addresses the `types` array; null means the source did not expose a
type. Follow parent links within this one artifact to understand where candidates
occur. No model values, UI text, raw pointers, full generic signatures or geometry
are exported. Stdout is only a compact saved-file receipt.

A count of1 makes a type unique in **that observation**, not a confirmed identity
of the requested dialog. Confirm its source hierarchy/meaning in the requested
app state, then pass its exact `name` to `--subtree-type`. The next capture reads
again and may return `subtreeNotFound` or `ambiguousSubtree` after a state change.
Discovery indices are not future capture handles. A symbol/source-code View name
absent from the list is not an exposed nominal node; don't guess an identity or
call an unrelated unique node the requested component. If no attributable unique
root exists, report that remaining selection limitation.

Discovery uses the same16384-node/depth512 source bounds and16MiB artifact cap.
It still calls the same opaque Apple getter; compact output is not a cheaper or
preemptible source-read guarantee. `coverage` reports source/unknown-type/hosting
root counts; completeness concerns the exposed source tree, not every application
View. `sourceLimit`, cancellation and stale-window refusals still apply. A legacy
SDK returns `discovery_unsupported_by_endpoint` before dispatch. Update the SDK
through the owning build workflow; no silent fallback to full capture is used.

## Select a component before export

A matching newer SDK/caller can select a subtree before serializing the canonical
Snapshot. This is separate from `--type`, which remains an output filter:

```sh
python3 -B "$CALLER" capture --session "$SESSION" \
  --window "$WINDOW" --generation "$GENERATION" \
  --subtree-type ExampleApp.SearchRoot --out "$NEW_SNAPSHOT"
```

`--subtree-type` also works with `tree` and `measure`. It requires one exact
**source-reported nominal type**, including its module. It does not search labels,
generic arguments or source-code view names that SwiftUI does not expose as nodes.
The SDK resolves it in this new observation; zero matches gives `subtreeNotFound`,
more than one gives `ambiguousSubtree`, without choosing the first match or
publishing a partial file. Some custom View bodies are exposed only as their
underlying framework nodes. `subtreeNotFound` does not prove the UI is unmounted;
report that applicability gap rather than adding app markers to manufacture a pass.

Output contains the complete selected subtree and its ancestor chain needed for
source geometry attribution. `selection` reports the source/subtree/context/excluded
node counts and selected observation key. Per-node declarations distinguish
`selected_root`, `descendant` and `ancestor_context`; `excluded_direct_children`
records excluded context branches. Original local spaces, transforms, inherited
source keys and retained child order remain. Context ancestors are not complete
exports of their other children; the Snapshot scope is the subtree with context.

The source still reads the selected window under the same 16384-node/depth512
bounds, and the response still has the 16MiB limit. The flag can resolve an
**output** overflow when the requested subtree is small; it cannot bypass a source
limit or guarantee that Apple's getter does less work. An oversized subtree still
refuses. No automatic fallback changes an ordinary whole-window request.
`subtree_selection_unsupported_by_endpoint` means the app has an older SDK;
update the SDK and matching caller through the owning build/release workflow.

## One observation for related measurements

For several measurements that must refer to the same layout, capture once and
run saved Rust queries on that original file. Otherwise two live commands can
legitimately see different states. Use a new, non-existing destination in the
task's evidence area:

```sh
python3 -B "$CALLER" capture --session "$SESSION" \
  --window "$WINDOW" --generation "$GENERATION" --out "$SNAPSHOT"
"$UIB_BUNDLE/uiblueprint-validate" --max-bytes 16777216 "$SNAPSHOT"
```

Then follow [factual queries](analysis.md#construct-a-factual-query), preserving
exact source keys, frame kinds, units and spaces. `tree` and `measure` also accept
`--out` to retain their own newly acquired canonical record. Output files are not
overwritten. A saved record preserves its collection interval and is historical;
validation or re-measurement does not make it current. Its canonical cache/
unverified label is intentional. The live command's request/observation identity
establishes the new read, not a perpetual freshness guarantee for the saved file.

## Resize, navigation, popups and lazy content

After the application's normal state change settles, make another query against
the same process. No rebuild or manual invalidation is needed. Newly mounted
nodes participate in the next read. Closed screens and rows not materialized by
SwiftUI have no runtime geometry yet; the tool does not render hidden screens or
scroll them into existence during collection. An authorized app workflow may
open/scroll them separately, followed by a new query.

A popup may belong to another window/hosting root. Enumerate windows after the
transition and bind the requested surface. If a window closes/reopens or the app
restarts, obtain its new generation/session. Don't reuse a stale binding or
silently switch to another window. Observe settled layout when the question asks
for final dimensions; a fresh sample during animation is not an atomic rendered
frame or proof that the animation finished.

## Read the result's limits

Source namespace is `swiftui.runtime.layout`; older marked snapshots use
`swiftui.layout`, while Accessibility is another source. The collector exports
structural types and allowlisted geometry, not view model values or UI text.
The debug tree records changed layout inputs. The qualified collector resolves
unchanged position/size through that same tree's context, marks inherited layout
as derived, and records `layout_position_source`/`layout_size_source` declarations
pointing to the defining nodes. This is source-specific inheritance, not copying
an arbitrary ancestor rectangle. A missing root context remains unknown. Nominal
types omit recursively repeated generic arguments; distinct occurrences retain
their keys and child links.

Known source-local layout rectangles use points. A qualified affine maps them to
that hosting tree's SwiftUI-global space, including offsets, scrolling, scale
and 2D rotation. Different hosting roots still have separate spaces; a toolbar's
global coordinates do not establish a mapping to the content root merely because
their values look compatible. The collector also visits toolbar and nested native
hosting views and reports empty roots as incomplete. Real-app coverage acceptance
remains separate from the general fixture qualification.

Rust can measure axis-preserving mapped rectangles. Rotated/sheared mapped shapes
currently return `unsupported_shape` for rectangle relations; their source-local
width/height remain available. Do not turn a bounding envelope into containment
or visibility truth, subtract unrelated local coordinates, or substitute AX
bounds. Follow the release's actual capability boundary; a large node count does
not certify all screens or mobile platforms.

Record the actual source/window/generation, request/observation ID, measured value,
units, frame kind, space and material unknowns. Width/height are layout dimensions,
not text ink, hit area or visible-region measurements. Image capture is separate.

## Failures and recovery

Read both the command exit and JSON. Successful transport can still contain
unknown properties or no matching type. No match is not proof that the source
examined every unmounted or unsupported part of the application.

| Result | Next step |
| --- | --- |
| `runtime_hook_not_ready` | Inspect the owned launch diagnostics and app recipe; verify Debug target membership and the retained startup hook. Don't repeatedly relaunch. |
| `launchProfileRequired` | Use the supported launch recipe on the next owner-authorized launch; setting an environment variable after startup does not initialize the source. |
| `unsupportedRuntimeProfile`, `updateContextUnavailable` | Report the actual OS/toolchain and source revision. Do not guess symbols, masks or call conventions in the consumer chat. |
| `sourceNotReady` | Check the app's intended window/state. A starting or closed window is not available geometry. Establish readiness through the normal app workflow before a new request. |
| `ambiguousWindow` | Enumerate and explicitly select the requested window. |
| `staleWindow`, `target_unavailable` | Re-establish the intended instance/window through its owner; discard old handles. |
| `busy` | Coordinate current readers and inspect completed/late request ownership; do not delete peers' files or widen limits. |
| `sourceLimit`, `outputLimit`, `unsupportedSource` | Preserve the exact failure and report the bounded source defect. Don't hide rows or add screen-specific markers to manufacture a pass. |
| `request_deadline_source_outcome_unknown` | Stop automatic retries. The client deadline cannot interrupt Apple's synchronous getter. Establish the owned process and late request outcome before another capture; don't assume cancellation stopped the call. |
| `measurement_deadline` | The bounded capture-plus-Rust query did not finish. Narrow the requested type/saved analysis scope when appropriate; don't increase the global limit blindly. |
| Unknown geometry or transform | Report that property as unavailable; a neighbor, ancestor or screenshot estimate is not the missing source fact. |

Normal default timeout is five seconds, with an overall CLI cap of thirty.
Do not turn a source failure into repeated rebuilds, polling captures, Xcode
sessions or a new application-specific extractor. Route a reproducible tool
defect to the Blueprint owner with a sanitized failure and exact version; the
application owner retains its windows and private evidence.
