// UI Blueprint synthetic diagnostic fixture. Each mode creates one window.
import SwiftUI

final class Anchor: NSView {}

struct CursorOverlay: NSViewRepresentable {
    func makeNSView(context: Context) -> Anchor { Anchor() }
    func updateNSView(_ view: Anchor, context: Context) {}
}

struct Slot: View {
    var body: some View {
        Button("Slot") {}.frame(width: 74, height: 32).overlay { CursorOverlay() }
    }
}

struct ImportControl: View {
    var body: some View {
        Button("Import") {}.frame(width: 118, height: 42).overlay { CursorOverlay() }
    }
}

struct DoubleCursor: View {
    var body: some View { HStack { CursorOverlay(); CursorOverlay() }.frame(width: 94, height: 40) }
}

struct TripleCursor: View {
    var body: some View {
        HStack { CursorOverlay(); CursorOverlay(); CursorOverlay() }.frame(width: 160, height: 40)
    }
}

private struct PrivateCursorOverlay: NSViewRepresentable {
    func makeNSView(context: Context) -> Anchor { Anchor() }
    func updateNSView(_ view: Anchor, context: Context) {}
}

struct PrivateSlot: View {
    var body: some View {
        Button("Slot") {}.frame(width: 74, height: 32).overlay { PrivateCursorOverlay() }
    }
}

struct PrivateImport: View {
    var body: some View {
        Button("Import") {}.frame(width: 118, height: 42).overlay { PrivateCursorOverlay() }
    }
}

@main
enum NativeOwnerFixture {
    @MainActor static func main() throws {
        let args = CommandLine.arguments
        guard args.count == 3 else { throw CocoaError(.fileReadInvalidFileName) }
        let app = NSApplication.shared
        app.setActivationPolicy(.accessory)
        let window = NSWindow(contentRect: NSRect(x: 0, y: 0, width: 320, height: 200),
                              styleMask: [.titled], backing: .buffered, defer: false)
        switch args[1] {
        case "distinct":
            window.contentView = NSHostingView(rootView: HStack { Slot(); ImportControl() })
        case "private-distinct":
            window.contentView = NSHostingView(rootView: HStack { PrivateSlot(); PrivateImport() })
        case "same-owner-twice":
            window.contentView = NSHostingView(rootView: HStack { Slot(); Slot() })
        case "two-links":
            window.contentView = NSHostingView(rootView: DoubleCursor())
        case "three-links":
            window.contentView = NSHostingView(rootView: TripleCursor())
        default:
            throw CocoaError(.fileReadUnknown)
        }
        window.orderFront(nil)
        RunLoop.current.run(until: Date().addingTimeInterval(0.7))
        let me = NSRunningApplication.current
        guard let bundle = me.bundleIdentifier, let birth = me.launchDate, let root = window.contentView else {
            throw CocoaError(.fileReadUnknown)
        }
        var frames = [[CGFloat]]()
        func visit(_ view: NSView) {
            if view is Anchor {
                let rect = view.convert(view.bounds, to: nil)
                frames.append([rect.minX, window.frame.height - rect.maxY, rect.width, rect.height])
            }
            for child in view.subviews { visit(child) }
        }
        visit(root)
        let baseline: [String: Any] = ["pid": me.processIdentifier, "expected_bundle": bundle,
            "expected_launch_time": birth.timeIntervalSince1970, "window_id": window.windowNumber,
            "window_top_left_cursor_rects_pt": frames]
        try JSONSerialization.data(withJSONObject: baseline).write(to: URL(fileURLWithPath: args[2]),
                                                                    options: .withoutOverwriting)
        app.run()
    }
}
