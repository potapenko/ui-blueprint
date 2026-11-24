# Fresh SwiftUI geometry requests

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
The raw SwiftUI tree can contain logical wrappers without their own position/size.
An unknown leaf and a measured ancestor remain two different facts.

The current candidate reports known source-local layout rectangles in points.
Common coordinate transforms and full toolbar/nested-host coverage are still
under qualification. Do not claim window-wide completion from a large node count,
subtract unrelated local coordinates for gaps, substitute AX bounds, or infer
pixel visibility from layout. Follow the published release's verified capability
boundary; candidate evidence does not certify all screens or mobile platforms.

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
