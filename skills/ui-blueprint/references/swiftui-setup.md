# Prepare a SwiftUI app for runtime geometry

Read this when integrating or preparing an app. For an already running integrated
instance, go directly to [runtime queries](swiftui-runtime.md). This is the
`UIBlueprintSwiftUIRuntime` source, distinct from the older marked
`UIBlueprintSwiftUI` SDK and external LLDB cache reader.

## What is prepared once

The app owner links one Debug library, retains one session at application startup,
and checks in one launch recipe. The library reads existing mounted SwiftUI
hosting trees. There are no per-screen markers, replacement view wrappers,
generated component lists or rebuilt binaries for individual measurements.

Build when source, dependency revision, build settings or qualified toolchain
changes. Reuse the resulting `.app` for further launches and the running session
for further queries. Resizing, opening a dialog, navigation and changing runtime
data require a new query, not a new build.

This path uses Apple's installed compiler/SDK tools and CLI `xcodebuild` for app
projects. It does not require Xcode's GUI, a debugger session, manual hierarchy
export, Accessibility permission or screen capture permission for geometry.
Other app behavior may have its own permissions; don't change them for Blueprint.

## Compatibility and authorization

Version 0.8.0 introduces this as **opt-in experimental Mac support**. The user
chose to make the current tool available before completing the full scenario
matrix. Use it for development and report reproducible defects; don't interpret
the release as a guarantee for every screen or OS. The scoped collector worked
in PlayPhrase.me; the newest inherited-layout/affine normalizer has fixture
qualification, with its full real-app sequence still pending.

Use the selected release's actual qualification matrix. The current source
profile targets arm64 macOS 27.0.1 with Xcode 27.2; its underscored SwiftUI API
is version-qualified. macOS 14 in Package.swift is a compile floor, not a runtime
support claim. iPhone, iPad, Intel and other OS versions are not qualified by this
Mac path. Do not bypass `unsupportedRuntimeProfile` or guess a different mask.

First confirm the task includes app integration/build/launch, or use the already
authorized project workflow. A read-only geometry request alone does not authorize
editing or restarting someone else's app. Existing authorization persists; do not
ask again for each routine step within it.

## Link and retain the session

Use the source revision matching the selected Blueprint release. The repository
root at `https://github.com/potapenko/ui-blueprint` is a Swift package; add product
`UIBlueprintSwiftUIRuntime` to the actual Mac application target and pin the exact
published tag/revision from that release. For a reviewed local checkout, the
nested package at `plugins/swiftui-runtime` also works. A dependency on the old
`UIBlueprintSwiftUI` product alone does not enable the runtime endpoint.

Add this to the existing startup owner, preserving its initialization and error
handling. Keep both import and retained property under `#if DEBUG`:

```swift
#if DEBUG
import UIBlueprintSwiftUIRuntime
#endif

// Inside the existing App type:
#if DEBUG
private let runtimeGeometry: BlueprintRuntimeSession?
#endif

// Once in its existing init, on the main actor:
#if DEBUG
runtimeGeometry = try BlueprintRuntimeSession.startIfRequested()
#endif
```

The last line belongs in the app's existing throwing/error-handling context;
do not paste a second App type or use `try!` to suppress startup errors. Keep
the session alive for the app lifetime. Do not create it in `body`, on every
screen appearance, or per request. Without `UIBLUEPRINT_RUNTIME=1`, startup
returns nil and starts no endpoint. No changes to individual View bodies are
required. Verify the ordinary Release app excludes the library's implementation;
leave signing, sandbox, deployment targets and normal schemes intact.

## Check in an app recipe

The app owner maintains a small JSON file, for example
`script/blueprint-runtime.json`. Paths are relative to this file:

```json
{
  "recipe_version": "1.0.0",
  "project_root": "..",
  "project": "Example.xcodeproj",
  "scheme": "Example",
  "product": "Example.app",
  "bundle": "me.example.app",
  "environment": {},
  "arguments": []
}
```

Use the real app scheme/product/bundle. `project` is relative to `project_root`.
This command supports `.xcodeproj` Mac Debug builds; other build systems need
their project's documented CLI build followed by `launch`. Do not fabricate a
workspace/scheme or change build signing to make a recipe pass.

`environment` and `arguments` contain only the app's supported development
configuration, such as a local API endpoint and a documented read-only test
session. Do not put secrets in the recipe, arguments or reports. The caller
sets `UIBLUEPRINT_RUNTIME=1`, `SWIFTUI_VIEW_DEBUG=29` and internal temporary
paths itself; do not change global environment or HOME/CODEX_HOME.

Copying an app, using a different PID or giving a build another directory does
not isolate UserDefaults, Keychain, databases, app groups or server state.
Describe the app's actual store/backend isolation in its development instructions.
Where an app lacks isolation, coordinate state-changing scenarios with its owner;
read-only geometry requests do not need a global desktop reservation.

## Prepare, launch and reuse

Resolve `UIB_BUNDLE` as in [the skill](../SKILL.md). The matching installed caller
is `$UIB_BUNDLE/swiftui_runtime.py` (Python 3.11+). Verify it exists and read
`--help`; don't silently use a helper from another checkout or older release.
Keep a candidate checkout workflow explicitly identified until release acceptance.

Set `PROJECT_RECIPE` to the absolute app-owned recipe path. Create task-owned,
private directories according to the operator's storage policy: `BUILD_ROOT` for
compiler output and `RUN_ROOT` for internal-disk app/runtime files. On this Mac,
verify the designated external volume is mounted and writable before creating
heavy build/evidence directories there. Running `.app` bundles and app temporary
data stay on internal system temp. Record owner, purpose and cleanup conditions.

```sh
CALLER="$UIB_BUNDLE/swiftui_runtime.py"
python3 -B "$CALLER" prepare --recipe "$PROJECT_RECIPE" \
  --build-root "$BUILD_ROOT" --run-root "$RUN_ROOT"
```

`prepare` builds Debug with CLI `xcodebuild`, copies the self-contained product
internally, and launches through LaunchServices in the background. Capture its
JSON result: `app`, `pid`, `session`, `run_directory`. Keep the exact source and
SDK revisions with the owned artifact; changing source makes the old artifact
historical. `build_identity` also records the Git-visible working input hash,
including dirty and untracked files, and the recipe hash. Inputs are checked
before and after compilation. Ignored/generated inputs and external local
packages still require their owner's separate provenance; the hash is not an
atomic filesystem snapshot. A descriptor means the hook started, not that every geometry case
passed. Verify `windows` and one real query before handing it to another agent.

If a suitable app already exists, launch that artifact directly, retaining the
same app recipe's required environment and arguments:

```sh
python3 -B "$CALLER" launch --app "$OWNED_APP" \
  --recipe "$PROJECT_RECIPE" --run-root "$RUN_ROOT"
```

If a suitable process is already running, do neither operation: use its owner-
supplied session descriptor. A new agent does not automatically need a new build
or process. Don't run `Contents/MacOS/...` directly in place of the launch recipe,
scan arbitrary processes for descriptors, or attach Xcode as a default fallback.

## Handoff and retirement

Give authorized consumers the caller/bundle revision, exact app revision and
artifact, PID, private descriptor path, selected window/generation, supported
configuration and owner. The descriptor contains a capability: share its path
only with authorized local consumers and never print its full JSON in reports.

Readers may share the endpoint. Independent scenarios may use separate instances
only when the app's state isolation supports them. Each instance serializes its
own short captures; unrelated instances do not wait on a shared geometry queue.
Physical mouse/keyboard actions are a separate shared resource.

After consumers finish, the application owner closes its instance normally and
removes only its disposable runtime/build files. Keep requested snapshots and
retained artifacts until their consumers release them. Never kill another owner's
app or remove a session because a request timed out. Continue with
[runtime queries](swiftui-runtime.md).
