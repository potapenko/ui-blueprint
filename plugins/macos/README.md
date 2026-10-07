# Native H01 helper

A channel-specific entrypoint over the existing Swift collector. Core's saved
H01 protocol `c0abcff` provides the parent broker, guarded canonical consumer and
capture/process ownership. This source connection is fixture-specific; it is not
live Native or arbitrary-application acceptance.

Build from repository root into a caller-created task-temp directory:

```sh
xcrun swiftc -parse-as-library -swift-version 6 -D CAPTURE_LIBRARY -D HOST_HELPER \
  -target arm64-apple-macos14.0 \
  plugins/macos/NativeAcquisition.swift plugins/macos/NativeJSON.swift plugins/macos/NativeArtifacts.swift \
  fixtures/native/Observe.swift tests/bridges/native/Collector.swift \
  tests/bridges/native/WindowAX.swift plugins/macos/HostProtocol.swift \
  plugins/macos/HostHelper.swift -o "$NATIVE_TASK_TMP/native-host-helper"
```

There are no new packages or copied collector implementations. The same public
AX/ScreenCaptureKit source serves legacy fixtures. SwiftUI is not involved:
AppKit supplies only existing nonvisual process identity, never visible UI.

The executable expects H01-owned sockets on FD3/FD4, no argv/environment. Do not
invoke it as a standalone capture command. The trusted setup, bounds, offline
checks and explicit residuals are in [the consumer contract](../../docs/development/native-helper.md).

The mandatory `acquisition_limits` configuration implements the registered D05
Native profile through three shared source owners: NativeAcquisition (ranged arrays/
CF copying), NativeJSON (charged canonical construction/bounded existing codec),
and NativeArtifacts (checked images/exclusive capped files). Legacy fixture callers
link these same files and provide explicit limits; none is a default production cap.
Initial SDK returns and codec internals remain opaque. Synthetic checks and existing
record preservation do not replace actual SDK/H01/D06 acceptance.
