# Kotlin Compose: Desktop/macOS and Android

Use the matching absolute `UIB_BUNDLE` from [the skill](../SKILL.md). The bundle
must contain `ComposeMarker.kt`, `ProbeSession.kt`, `ProbeJson.kt`, `COMPOSE.md`
and a CLI whose help includes `import-compose`. Read COMPOSE.md for integration
and the [analysis reference](analysis.md) for exact canonical query/check shapes.
Do not combine a new collector with an older installed CLI. No global installation
or app modification follows merely from invoking this skill.

## Obtain a real observation

Use an already authorized debug integration, or ask the application owner to add
this matching source to its existing JVM/Android debug source sets when integration
is within the task. Never invent runtime JSON or fill source rectangles by hand.
The source adds `Modifier.uiBlueprintMarker(session, "static-public-id")` to existing
layouts. One remembered ProbeSession belongs to the selected application surface;
its disposal closes the session. Existing Compose UI dependencies suffice.
No wrapper layout, semantics/testTag, overlay, density change or playback changes.

An explicit UI-thread debug callback calls `session.capture(ProbeRequest(...))` with
one request ID, scope ID, selected marker IDs including their marked root, node cap,
output cap and deadline. Use only public developer IDs, no UI text/values. The app
writes the returned bytes to its private temporary destination after success.
Desktop: system temp. Android: app-private cache; an authorized exact-file `adb
exec-out run-as` transfer produces a host system-temp file. The app's supported
trigger is caller-owned; do not add polling, an exported receiver or a network server.
No permission escalation or device switching to bypass a failure.

```sh
"$UIB_BUNDLE/uiblueprint" import-compose --input "$RAW" \
  --max-input-bytes 1048576 --max-output-bytes 1048576 > "$SNAPSHOT"
"$UIB_BUNDLE/uiblueprint-validate" --max-bytes 1048576 "$SNAPSHOT"
```

Budgets are finite examples. Check each exit. Import preserves source collection
timing/generations and unknowns; it is saved-data normalization, not live revalidation.
Raw producer version1.0.0 becomes unchanged core0.1.0 Snapshot, analysis stays0.2.0.

## Select and measure

Read Snapshot nodes and their `source_declarations.marker_id`; select the actual
`key` object. Namespace is `compose.layout`, key contains the unique mount identity.
Do not construct a key from a title or use a prior mount after remount. Duplicate
markers refuse ambiguity; missing/detached/stale selections refuse. `expectedMounts`
can bind an explicit request to previous raw mount IDs. Root remount needs a new
session and incompatible saved records must not be restamped to make diff pass.

```sh
"$UIB_BUNDLE/uiblueprint" inspect --snapshot "$SNAPSHOT" \
  --ref "$SOURCE_KEY_JSON" --view design \
  --max-input-bytes 1048576 --max-output-bytes 1048576
"$UIB_BUNDLE/uiblueprint" measure --snapshot "$SNAPSHOT" --query "$QUERY" \
  --space compose-root-dp --max-input-bytes 1048576 --max-output-bytes 1048576 --json
```

Construct QUERY from the canonical schema and reported source keys as described in
Analysis. Width uses one layout_bounds anchor in `compose-root-px` (`kind:local`,
`units:px`, `origin:top_left`), axis x/fraction0, operation width, quantity_kind length;
query units dp selects the density-backed result. Gap uses two appropriate edges.
The source-to-result transform comes from measured root Density; never divide or
round geometry in shell/Kotlin as a second analytics engine. FontScale is separate.
Use check only with an actual user/product expectation and explicit tolerance.

## Compare a resize or scroll

Capture once, perform the separately authorized normal app change, then explicitly
capture again with the same live session, marked root and selected list. Compare
stable mounted source keys; snapshots preserve different environment revisions.

```sh
"$UIB_BUNDLE/uiblueprint" diff --geometry --before "$BEFORE" --after "$AFTER" \
  --ref "$SOURCE_KEY_JSON" --frame-kind layout_bounds --space compose-root-dp \
  --max-input-bytes 2097152 --max-output-bytes 2097152 --json
```

Root-local position includes scroll. Rectangles are unclipped layout AABBs after
source transforms; they are not AX bounds, hit regions, painted contours or visible
regions. Other UI, unmounted lazy items, system overlays, pixels and screen mapping
remain unobserved. Partial coverage and sequential non-atomic reads stay explicit.
A snapshot cannot prove playback, accessibility or probe-on/off invariance: verify
those scenarios separately in the real application. Android Phone/Tablet and Desktop
claims apply only to observed consumers; no Windows/TV or general Android scanner.
