# In-process SwiftUI runtime geometry
- Node type: leaf; domain: `uib.swiftui-runtime`; contract: `UIB.SWIFTUI-RUNTIME@2`.
- Authority: Active; stability: Evolving; no app/release acceptance claimed.
- Authority: direct user continuation in chat01a20a46 on2026-11-23: continue until fresh query geometry works, without new plans, screen-specific fixes or GUI Xcode dependencies; PLAN.UIB.SWIFTUI-GEOMETRY@3 records scope.
- Requires: [BOUNDARIES](boundaries.md), [MODEL](model.md), [IDENTITY](identity.md), [GEOMETRY](geometry.md), [PRIVACY](privacy.md), [EXCHANGE](exchange.md), [LIFECYCLE](lifecycle.md), [CACHE](cache.md), [CLI](cli.md).
- Scope: separate opt-in Debug library and matching caller; existing marker SDK, external debugger/cache acquisition, snapshots and actions stay unchanged.

## RUNTIME.SOURCE
Use the installed SDK's typed `NSHostingView._viewDebugData()` inside the opted-in process on the main actor and within a verified SwiftUI main UpdateGroup scope. A protocol conformance locates existing hosting roots, including SwiftUI App-created roots, without replacing UI. This underscored development API is version-qualified, not a universal stable Apple API guarantee. No LLDB attach, JIT, absolute-address calls, getter ABI emulation or raster-layer encoding belongs to this path.

The unscoped getter candidate is withdrawn after real PlayPhrase.me trapped at a nil `UpdateGroup.current` in `AGAttribute.syncMainIfReferences`. A narrow Debug C bridge binds named exported UpdateGroup main/current/begin/end/isMain symbols from the system SwiftUICore image, using verified Swift calling convention/self context and balanced owned references. Unsupported symbols, non-main execution, incompatible scope or an unqualified OS refuse before calling the getter. The initial scope profile is macOS27.0.1; nested and throwing paths must restore the previous context. This synchronization does not grant UI mutation or raw model serialization authority. Actual app requalification remains required.

The Mac-only diagnostic adapter may read its own NSApplication/NSWindow/NSView identities and geometry; all visible content remains SwiftUI. The separate existing pure marker SDK does not acquire an AppKit dependency. Native app changes and real-app acceptance belong to the native owner.

Collection copies only source type metadata, position/size and independently qualified transform facts. Inspect only the known `_ViewDebug.Data` fields `data` and `childData`; never stringify/encode `.value`, `.environment`, model fields or raw complete Data. A canary found in JSONEncoder(Data) output proves that the launch property mask alone is not privacy filtering. Filter before serialization.

## RUNTIME.REQUEST
One opt-in session is created at Debug startup. A private internal-temp endpoint has owner-only permissions, a random run identity, a capability and an atomic descriptor. The CLI obtains it through the owned launch's diagnostic output, not a scan of unrelated user processes.

Operations: enumerate this instance's available windows, capture an explicitly selected window (or the sole unambiguous visible window), and inspect/measure its new canonical Snapshot. Requests identify session/capability/request and bounded fields; caller-supplied paths or arbitrary executable operations are forbidden. No operation opens, resizes, focuses or restarts product windows.

Each live capture performs a new source read. Node keys are observation-scoped. Window witnesses retain generation only while the exact weak window identity remains. Multiple agents use distinct request/result names; capture is serialized on this app's main actor, independent of the shared desktop-input lane.

Input publication is atomic. Consume each complete bounded request once; publish one complete response or fixed failure. Output is written atomically without overwrite. Cancel markers suppress geometry publication when cancellation arrives during the synchronous source call. A client timeout does not prove that the opaque framework call stopped; do not retry a still-active capture or kill the app. Queue/storage are bounded and caller-owned completed files are acknowledged/removed only by their consumer.

## RUNTIME.DATA
Source namespace `swiftui.runtime.layout`; plugin `swiftui-runtime-probe`; canonical core0.1 Snapshot and existing Rust analysis remain. Source-local positions are not declared window/screen coordinates without qualified mapping. Missing geometry remains unknown; known sizes/positions preserve numeric values. Original source hierarchy is retained, including unmeasured wrappers. Complete source-tree discovery and availability of every requested property are distinct.

Per-capture work admits at most16384 source nodes, depth512 and16MiB output; input16KiB, pending requests32. These are separate in-process bounds, not widened external-debugger quotas. No silent truncation. The synchronous Apple getter has opaque internal allocation/latency; elapsed/consistency and source outcomes remain explicit. Do not claim a hard whole-call execution bound from a post-call check.

## RUNTIME.PREPARE
One project recipe declares its source revision/build command, product, Debug activation and supported backend/toolchain. Prepare/build is separate from ordinary requests. Reuse the same verified artifact for owners; launch separate instances only when identity and app stores can actually be isolated. Shared artifact construction has a bounded build lock; independent captures need no Xcode GUI or desktop reservation. A build lock is not a global runtime queue.

Internal app/package/runtime storage and external compiler/evidence storage follow current user rules. Dependencies and the library are excluded from Release. No automatic signing/sandbox weakening, production service operation, dependency installation or user-store cloning.

## RUNTIME.ACCEPT
Qualify CLI-built fixture and actual app startup roots; repeated capture after resize, conditional rows, navigation/popup changes, stale window/run and simultaneous requests. Validate original canonical bytes and measure with Rust; prove canary exclusion before publication and Release exclusion. The six-field raw fixture example is not whole-app acceptance. Follow PLAN@3 C01–C26; unresolved mapped coordinates or required properties remain open.

## Source evidence
Installed Xcode27.2 SDK interfaces expose typed getter and `_ViewDebug.Data`; inspected paths are SwiftUI/SwiftUICore arm64e swiftinterfaces. No upstream implementation copied. Preliminary owned SwiftUI App fixture gives188→212→188 nodes and width80→140→80 without GUI/LLDB, with canary absent after field allowlisting. Runtime29/287 startup profiles were compared; neither bypasses filtering. Real PlayPhrase.me acceptance remains pending.

@2 is a Restore correction to the authorized general source: the actual fault offset856 resolves to the nil-current-UpdateGroup guard. A standalone fixture reproduces that guard's SIGTRAP without the scope and returns123 with it; nested geometry captures record current-context0→1→1→1→0 and unchanged field filtering. This is not yet proof of the corrected real-app scenario.
