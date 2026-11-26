# Owned UIKit runtime fixture

Only a project-owned diagnostic app. Never inject it into a consumer app.
Build with `python3 build.py --build-root EXTERNAL_OWNED_DIR --app-root INTERNAL_OWNED_DIR`.
Launch the assembled app with the normal `swiftui_simulator.py launch` caller on
an explicitly free existing iOS27.2.0 arm64 Simulator. No Xcode GUI or project
scaffolding is needed. This deliberately compiles the exact SDK sources, including
the Debug synchronization bridge, with the installed Apple CLI compiler.

The SwiftUI screen provides a real system Menu and menu-style Picker. Open,
capture the selected window with `--source uikit`, close and reopen; validate the
canonical bytes and measure an observed menu node with Rust. Inspect each window:
the nonkey text-effects window is not necessarily the menu's owner.

A separate nonvisible UIWindow fixture checks UIKit model geometry using known
construction arithmetic: nonzero bounds origin, scroll offset, rotation/scale,
perspective refusal and animated model width. It includes private label/AX-value
canaries. Its `tmp/uikit-oracle.json` must say `pass`; the corresponding canonical
Snapshot must validate. UIScrollView may add its own indicator views, so match
the unique label and its real parent rather than asserting a fabricated node count.
Copy evidence to the owned external evidence directory. Keep the session path
private, and terminate/uninstall only this bundle after consumers finish.

Source APIs: [UIView bounds](https://developer.apple.com/documentation/uikit/uiview/bounds),
[point conversion](https://developer.apple.com/documentation/uikit/uiview/convert(_:to:)-1xizt),
[transform/frame restriction](https://developer.apple.com/documentation/uikit/uiview/transform),
[model versus presentation](https://developer.apple.com/documentation/quartzcore/calayer/presentation()).
The adapter reads model geometry; it does not claim displayed animation positions.
