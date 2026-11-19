// The two public AX groups repeat a slot label; identifiers are shared by other buttons.
import AppKit

@main
enum LibraryAXFixture {
    @MainActor static func main() throws {
        let args = CommandLine.arguments
        guard args.count == 3, ["unique", "duplicate", "missing"].contains(args[1]) else {
            throw CocoaError(.fileReadInvalidFileName)
        }
        let app = NSApplication.shared
        app.setActivationPolicy(.accessory)
        let window = NSWindow(contentRect: NSRect(x: 80, y: 140, width: 520, height: 200),
                              styleMask: [.titled, .closable, .resizable], backing: .buffered, defer: false)
        let root = NSView(frame: .init(x: 0, y: 0, width: 520, height: 200))
        window.contentView = root
        var selected: NSButton?
        for index in 0...1 {
            let overlay = NSView(frame: .init(x: 20 + index * 250, y: 20, width: 220, height: 160))
            overlay.setAccessibilityElement(true)
            overlay.setAccessibilityRole(.group)
            overlay.setAccessibilityIdentifier(index == 0 ? "searchLearn.learner.overlay" : "fixture.other")
            root.addSubview(overlay)
            let count = index == 0 && args[1] == "duplicate" ? 2 : 1
            for row in 0..<count {
                let button = NSButton(title: "Fixture", target: nil, action: nil)
                let match = index != 0 || args[1] != "missing"
                button.setAccessibilityLabel(match ? "Favorites collection 1, empty" : "Other")
                button.setAccessibilityIdentifier("searchLearn.learner.favorites")
                button.frame = .init(x: 12, y: 8 + row * 36, width: 100 + 40 * index, height: 30)
                overlay.addSubview(button)
                if selected == nil { selected = button }
            }
            let other = NSButton(title: "Other", target: nil, action: nil)
            other.setAccessibilityIdentifier("searchLearn.learner.favorites")
            other.frame = .init(x: 15, y: 100, width: 80, height: 25)
            overlay.addSubview(other)
        }
        window.orderFront(nil)
        RunLoop.current.run(until: Date().addingTimeInterval(0.6))
        let identity = NSRunningApplication.current
        guard let bundle = identity.bundleIdentifier, let date = identity.launchDate, let selected,
              let path = identity.bundleURL?.path, let screen = NSScreen.screens.first else {
            throw CocoaError(.fileReadUnknown)
        }
        let frame = selected.accessibilityFrame()
        let data: [String:Any] = ["pid":identity.processIdentifier, "bundle":bundle, "birth":date.timeIntervalSince1970,
            "cg":window.windowNumber, "path":path,
            "expected":[frame.minX, screen.frame.maxY - frame.maxY, frame.width, frame.height]]
        try JSONSerialization.data(withJSONObject:data).write(to:URL(fileURLWithPath:args[2]), options:.withoutOverwriting)
        app.run()
    }
}
