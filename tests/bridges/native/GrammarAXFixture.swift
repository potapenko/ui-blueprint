// Standalone public AppKit fixture. The two scroll areas deliberately repeat a name.
import AppKit

@main
enum GrammarAXFixture {
    @MainActor static func main() throws {
        let args = CommandLine.arguments
        guard args.count == 3, ["different-areas", "same-area"].contains(args[1]) else {
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
            let scroll = NSScrollView(frame: .init(x: 20 + 250 * index, y: 40, width: 210, height: 120))
            scroll.setAccessibilityIdentifier(index == 0 ? "searchLearn.learner.grammar" : "fixture.other")
            let content = NSView(frame: .init(x: 0, y: 0, width: 210, height: 120))
            scroll.documentView = content
            root.addSubview(scroll)
            let count = index == 0 && args[1] == "same-area" ? 2 : 1
            for row in 0..<count {
                let button = NSButton(title: "Fixture", target: nil, action: nil)
                button.setAccessibilityLabel("Present Simple")
                button.frame = .init(x: 12, y: 8 + row * 36, width: 100 + 50 * index, height: 30)
                content.addSubview(button)
                if selected == nil { selected = button }
            }
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
