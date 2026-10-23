# Compose opt-in geometry collector

This source-included debug probe supports Kotlin Compose Desktop on macOS and
Android Phone/Tablet integration. It uses existing Compose UI dependencies;
no Kotlin Multiplatform migration, analytics library, service or polling is needed.
The initial API compile baseline is Kotlin2.3.20, Desktop UI1.12.0 and Android
UI1.9.4. Runtime qualification is specific to each tested app/device.

## Add the matching source

The matching UI Blueprint bundle contains `ComposeMarker.kt`, `ProbeSession.kt`
and `ProbeJson.kt` beside the CLI. Add that supplied directory to the existing
Kotlin source set **only for the explicitly enabled debug build**. For a checkout,
use `plugins/compose/src`. Keep all three files from the same committed bundle.

Example Android Gradle Kotlin DSL, inside the existing android configuration:

```kotlin
providers.gradleProperty("playphrasemeBlueprintSources").orNull?.let { path ->
    sourceSets.getByName("debug").java.srcDir(path)
}
```

For an existing JVM module, gate `kotlin.sourceSets.getByName("main").kotlin.srcDir(path)`
on the same explicit local property and use the app's development-only launch path.
Do not add this source to production builds or install another Compose version.
App-specific call sites also need a debug/development boundary; source inclusion
alone does not make unconditional production references compile.

## Mark existing layouts and capture explicitly

```kotlin
import dev.uiblueprint.compose.*

// Construct on the UI thread; remember one instance for this exact surface.
val probe = ProbeSession(
    targetId = "playphraseme-desktop", surfaceId = "main",
    rootMarkerId = "screen", platform = "desktop_macos"
)
// Android: actual public application identifier and platform = "android".
// Append to existing modifiers, without a new Box or semantics/testTag:
val screenModifier = existingModifier.uiBlueprintMarker(probe, "screen")
val videoModifier = existingVideoModifier.uiBlueprintMarker(probe, "video")

// Authorized debug callback on the SAME UI thread, after normal layout:
val bytes = probe.capture(ProbeRequest(
    requestId = "capture-1", scopeId = "player",
    markerIds = listOf("screen", "video"), maxNodes = 32,
    maxOutputBytes = 262144, deadlineMillis = 1000
))
// Write bytes only after success to an explicit private temporary file.
// Close on surface disposal, on the same UI thread:
probe.close()
```

In Compose use `remember` for the session and `DisposableEffect` for close. A
surface/root remount requires a new session. Never construct a new session for
each capture when comparing the same live mounted nodes. IDs must be static
public developer identifiers `[A-Za-z0-9._-]`,1–96 chars; no UI text, media data,
file paths, account IDs or secrets. Root must be an actual ancestor of every
selected marker and included in `markerIds`. The selected-root coordinate frame
is fixed across compatible captures. Choose a narrow explicit marker list.

Catch `IllegalArgumentException` and `IllegalStateException` at the debug trigger;
report only the fixed exception message, never a payload or stack with app data.
Failures include invalid request/ID/limits, wrong_thread, closed_session,
marker_limit, target_unresolved, ambiguous_target, stale_target, stale_surface,
outside_root, deadline and output_limit. Do not retry automatically or write partial
output. Unknown source geometry is encoded explicitly without an old-value fallback.
`expectedMounts` optionally maps selected marker IDs to mount IDs from a prior
raw response; a remount then refuses stale_target. Without it, a current remount
gets a new canonical key and is never silently matched to the previous instance.

No built-in keyboard binding, broadcast receiver, server, filesystem watcher or
background timer exists. The app owner supplies an explicit debug request trigger.
Desktop output belongs in system temp. Android uses its app-private cache, followed
by an explicitly authorized `adb exec-out run-as APPLICATION_ID cat EXACT_CACHE_FILE`
transfer to a system-temp host file. No broad storage permission or exported receiver.
Remove only run-owned nonimage data after its immediate consumers finish; images
always remain in system temp and are never agent-cleaned.

## Normalize and analyze

```sh
"$UIB_BUNDLE/uiblueprint" import-compose --input "$RAW" \
  --max-input-bytes 1048576 --max-output-bytes 1048576 > "$SNAPSHOT"
"$UIB_BUNDLE/uiblueprint-validate" --max-bytes 1048576 "$SNAPSHOT"
```

Check exit status before using the output. Import reads saved source only; it does
not attach, recollect or revalidate a live app. The producer transport is version1.0.0;
output is canonical core0.1.0 with namespace `compose.layout` and key
`MARKER_ID-MOUNT_ID`. Use the actual reported key, never the marker string alone.
`inspect`, `measure`, `check` and compatible `diff` use the ordinary Rust engine.
The portable skill's Compose/Analysis references give complete command examples.

Source rectangles are unclipped, transformed layout AABBs in `compose-root-px`.
Root Density supplies the explicit conversion to `compose-root-dp`; fontScale is
separate and is not another density factor. No screen/window/image transform,
AX/hit/visible region, measured padding, semantic tree or action authority follows.
Scroll is included in root-local positions. External overlays/occlusion are unknown.
Coverage stays partial for the app; only selected mounted markers were read.
No UI text is acquired. No source code declarations masquerade as measured bounds.

The app owner must verify enabled/disabled layout, focus, hit testing and playback
invariance and real resize/scroll/reflow on Desktop and Phone/Tablet. Headless
fixtures and compilation establish separate evidence, not that runtime acceptance.
