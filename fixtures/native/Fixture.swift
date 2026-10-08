import SwiftUI
import AppKit
import Darwin

// Identity-only invalidation, independent of UI measurements. The directory is
// existing run-owned setup; this owner creates no new directory or history.
final class FixtureIdentity {
    let directory: URL
    let windowKey: String
    private(set) var generation = UUID().uuidString
    private var windowID: Int?
    init(directory: URL, windowKey: String) { self.directory = directory; self.windowKey = windowKey }
    private func write(pid: Int32, bundle: String, launch: Double, id: Int, state: String) throws {
        let record: [String: Any] = ["identity_version": "1.0.0", "pid": pid, "bundle_id": bundle,
            "launch_time": launch, "window_id": id, "window_identifier": windowKey,
            "target_generation": "\(pid):\(launch)", "surface_generation": generation, "state": state]
        let bytes = try JSONSerialization.data(withJSONObject: record, options: [.sortedKeys])
        guard bytes.count <= 4032 else { throw NSError(domain: "identity_limit", code: 1) }
        try bytes.write(to: directory.appendingPathComponent("\(windowKey)-identity.json"), options: .atomic)
    }
    func snapshot(pid: Int32, bundle: String, launch: Double, window: Int) throws -> String {
        if windowID != window { generation = UUID().uuidString; windowID = window }
        try write(pid: pid, bundle: bundle, launch: launch, id: window, state: "open")
        return generation
    }
    func close(pid: Int32, bundle: String, launch: Double, window: Int) throws {
        generation = UUID().uuidString
        try write(pid: pid, bundle: bundle, launch: launch, id: window, state: "closed")
    }
}

// Minimal own-fixture window attribution. Only explicit marker presence chooses
// a candidate; same title/rectangle/order are not evidence. Synthetic tests use
// the same unique-candidate decision, while live public tree scanning is separate.
enum FixturePopupAttribution {
    static func unique(_ rows: [(window: Int, identifiers: [String])], role: String) -> Int? {
        guard rows.count <= 32 else { return nil }
        let matches = rows.filter { $0.identifiers.contains("f02.popup.owner.\(role)") }
        return matches.count == 1 && matches[0].window > 0 ? matches[0].window : nil
    }
    @MainActor static func window(_ windows: [NSWindow], parent: NSWindow, role: String) -> NSWindow? {
        guard windows.count <= 32 else { return nil }
        var rows: [(window: Int, identifiers: [String])] = []
        for window in windows where window !== parent && window.isVisible && !["a","b"].contains(window.identifier?.rawValue ?? "") {
            var queue: [(Any,Int)] = [(window,0)], seen: Set<ObjectIdentifier> = [], identifiers: [String] = []
            while !queue.isEmpty && seen.count < 160 {
                let (value, depth) = queue.removeFirst()
                let object = value as AnyObject
                guard seen.insert(ObjectIdentifier(object)).inserted, let accessible = value as? NSAccessibilityProtocol else { continue }
                if let key = accessible.accessibilityIdentifier(), key.utf8.count <= 4096 { identifiers.append(key) }
                if depth < 9, let children = accessible.accessibilityChildren() {
                    // The initial public AppKit return remains opaque; no second
                    // unbounded traversal/copy is retained by this fixture owner.
                    guard children.count <= 1280 else { continue }
                    queue.append(contentsOf: children.prefix(max(0,160-seen.count-queue.count)).map { ($0,depth+1) })
                }
            }
            rows.append((window.windowNumber,identifiers))
        }
        guard let id = unique(rows,role:role) else { return nil }
        return windows.first { $0.windowNumber == id }
    }
}

#if !IDENTITY_TEST
// Reusable synthetic pilot only. All visible content/windows/state are SwiftUI.
@main struct F02Fixture: App {
    var body: some Scene {
        Window("F02 Synthetic", id: "a") { PilotView(role: "a") }
            .windowResizability(.contentSize)
        Window("F02 Synthetic", id: "b") { PilotView(role: "b") }
            .windowResizability(.contentSize)
    }
}
struct RectRecord: Codable, Equatable {
    let x: Double, y: Double, width: Double, height: Double
    init(_ r: CGRect) { x = r.minX; y = r.minY; width = r.width; height = r.height }
}
@MainActor final class Measurements {
    var frames: [String: RectRecord] = [:]
    weak var popupContainingWindow: NSWindow?
    weak var popupCurrentIdentity: FixtureIdentity?
    func updatePopupWindow(_ value: NSWindow?) {
        if value == nil || popupContainingWindow !== value, let owner = popupCurrentIdentity {
            let app = NSRunningApplication.current
            do {
                // Attachment loss/change invalidates identity only, never publishes
                // measurement or changes visible UI state.
                try owner.close(pid:app.processIdentifier,bundle:app.bundleIdentifier ?? "unknown",
                    launch:app.launchDate?.timeIntervalSince1970 ?? 0,window:popupContainingWindow?.windowNumber ?? -1)
            } catch { _exit(2) }
        }
        popupContainingWindow = value
    }
    var callbackCount = 0
    var callbackNanoseconds: UInt64 = 0
    var windowID = -1
    var surfaceGeneration = UUID().uuidString
    func report(_ key: String, frame: CGRect) {
        let start = DispatchTime.now().uptimeNanoseconds
        frames[key] = RectRecord(frame)
        callbackCount += 1
        callbackNanoseconds += DispatchTime.now().uptimeNanoseconds - start
    }
}
// SwiftUI popover content has no public native handle. This nonvisual bridge
// records only its actual NSView.window; it measures/updates no UI or probe state.
@MainActor private struct PopupContainingWindowReader: NSViewRepresentable {
    let metadata: Measurements
    func makeNSView(context: Context) -> ReaderView {
        let view = ReaderView(frame: .zero)
        view.metadata = metadata
        view.setAccessibilityElement(false)
        return view
    }
    func updateNSView(_ view: ReaderView, context: Context) { view.metadata = metadata }
    static func dismantleNSView(_ view: ReaderView, coordinator: ()) {
        if view.metadata?.popupContainingWindow === view.window { view.metadata?.updatePopupWindow(nil) }
        view.metadata = nil
    }
    final class ReaderView: NSView {
        weak var metadata: Measurements?
        override func viewDidMoveToWindow() {
            super.viewDidMoveToWindow()
            metadata?.updatePopupWindow(window)
        }
        override func hitTest(_ point: NSPoint) -> NSView? { nil }
        override var acceptsFirstResponder: Bool { false }
        override var intrinsicContentSize: NSSize { .zero }
    }
}

private struct MarkerAnchors: PreferenceKey {
    static var defaultValue: [String: Anchor<CGRect>] { [:] }
    static func reduce(value: inout [String: Anchor<CGRect>], nextValue: () -> [String: Anchor<CGRect>]) {
        value.merge(nextValue(), uniquingKeysWith: { _, new in new })
    }
}
private struct Marker: ViewModifier {
    let key: String
    @ViewBuilder func body(content: Content) -> some View {
        #if PROBE
        content.transformAnchorPreference(key: MarkerAnchors.self, value: .bounds) { values, anchor in values[key] = anchor }
        #else
        content
        #endif
    }
}
// Narrow system-window setup, invoked only by explicit fixture buttons.
// Scene defaultWindowPlacement/windowIdealPlacement (macOS 15+) describe
// initial/zoom placement, not repositioning this existing macOS-14-target window.
@MainActor private enum WindowSetup {
    static func apply(role: String, comparison: Bool) {
        let matches = NSApp.windows.filter { $0.identifier?.rawValue == role && $0.isVisible }
        guard matches.count == 1, let window = matches.first, let screen = window.screen else { return }
        let visible = screen.visibleFrame
        let desired = comparison ? CGPoint(x: visible.minX + 40, y: visible.maxY - window.frame.height - 60)
            : CGPoint(x: window.frame.minX + 40, y: window.frame.minY - 20)
        window.setFrameOrigin(CGPoint(x: min(max(desired.x, visible.minX), visible.maxX - window.frame.width),
                                      y: min(max(desired.y, visible.minY), visible.maxY - window.frame.height)))
        if comparison { window.makeKeyAndOrderFront(nil); NSApp.activate() }
    }

    #if P01_SAMPLE_HITS
    // Test-only public event-queue adapter; all visible content remains SwiftUI.
    static func sampledPoints(role: String) throws -> (NSWindow, NSPoint, NSPoint) {
        let windows = NSApp.windows.filter { $0.identifier?.rawValue == role && $0.isVisible }
        guard windows.count == 1, let window = windows.first, window.windowNumber > 0,
              let content = window.contentView else { throw NSError(domain: "p01_window", code: 1) }
        var queue: [(Any, Int)] = [(window, 0)], seen: Set<ObjectIdentifier> = [], frames: [NSRect] = []
        var metadata: [[String: Any]] = []
        while !queue.isEmpty && seen.count < 160 {
            let (value, depth) = queue.removeFirst()
            guard seen.insert(ObjectIdentifier(value as AnyObject)).inserted else { continue }
            let object = value as AnyObject
            let full = value is NSAccessibilityProtocol
            let elementProtocol = value is NSAccessibilityElementProtocol
            var entry: [String: Any] = ["type": String(describing: type(of: object)), "depth": depth,
                "full_protocol": full, "element_protocol": elementProtocol]
            if let element = value as? NSAccessibilityProtocol {
                entry["modern_children_count"] = element.accessibilityChildren()?.count ?? -1
            }

            metadata.append(entry)
            guard let element = value as? NSAccessibilityProtocol else { continue }
            if element.accessibilityIdentifier() == "f02.sample.\(role)" { frames.append(element.accessibilityFrame()) }
            if depth < 9, let children = element.accessibilityChildren() {
                guard children.count <= 1280, queue.count + children.count + seen.count <= 160
                else { throw NSError(domain: "p01_children", code: 2) }
                queue.append(contentsOf: children.map { ($0, depth + 1) })
            }
        }
        guard queue.isEmpty, frames.count == 1, let screenFrame = frames.first,
              screenFrame.width > 0, screenFrame.height > 0 else { throw NSError(domain: "p01_sample", code: 3, userInfo: [
                "match_count": frames.count, "seen_count": seen.count, "remaining_queue": queue.count,
                "object_metadata": metadata, "frames": frames.map { ["x": $0.minX, "y": $0.minY, "width": $0.width, "height": $0.height] }]) }
        let inside = window.convertPoint(fromScreen: NSPoint(x: screenFrame.midX, y: screenFrame.midY))
        let outside = content.convert(NSPoint(x: content.bounds.minX + 5, y: content.bounds.minY + 5), to: nil)
        let sampleInWindow = window.convertFromScreen(screenFrame)
        guard inside.x.isFinite, inside.y.isFinite, outside.x.isFinite, outside.y.isFinite,
              content.bounds.contains(content.convert(inside, from: nil)),
              content.bounds.contains(content.convert(outside, from: nil)), !sampleInWindow.contains(outside)
        else { throw NSError(domain: "p01_coordinates", code: 4) }
        return (window, inside, outside)
    }
    static func postMousePair(window: NSWindow, point: NSPoint, number: Int) throws {
        guard NSApp.windows.contains(where: { $0 === window }), window.isVisible, window.windowNumber > 0
        else { throw NSError(domain: "p01_current_window", code: 5) }
        let now = ProcessInfo.processInfo.systemUptime
        guard let down = NSEvent.mouseEvent(with: .leftMouseDown, location: point, modifierFlags: [],
            timestamp: now, windowNumber: window.windowNumber, context: nil, eventNumber: number,
            clickCount: 1, pressure: 1),
            let up = NSEvent.mouseEvent(with: .leftMouseUp, location: point, modifierFlags: [],
            timestamp: now + 0.001, windowNumber: window.windowNumber, context: nil, eventNumber: number + 1,
            clickCount: 1, pressure: 0), down.window === window, up.window === window
        else { throw NSError(domain: "p01_event_binding", code: 6) }
        NSApp.postEvent(down, atStart: false)
        NSApp.postEvent(up, atStart: false)
    }
    #endif
}
private enum Field: String, Hashable { case name, secret, sample }
private struct PilotView: View {
    let role: String
    @State private var name = ""
    @State private var secret = ""
    @State private var checked = false
    @State private var applied = "none"
    @State private var activations = 0
    @State private var expanded = false
    @State private var wide = false
    @State private var popup = false
    @State private var popupGeneration = UUID().uuidString
    @State private var scrollEnd = false
    @State private var stimulus = "normal"
    @State private var sourceRevision = 0
    @State private var eventRevision = 0
    @State private var measurements = Measurements()
    @State private var identity: FixtureIdentity?
    @State private var popupIdentity: FixtureIdentity?
    @State private var popupWindowID = -1
    @State private var snapshotRequest = 0
    @FocusState private var focus: Field?
    @Environment(\.openWindow) private var openWindow
    @Environment(\.displayScale) private var displayScale
    private var other: String { role == "a" ? "b" : "a" }
    private var runDirectory: URL {
        let args = CommandLine.arguments
        guard let index = args.firstIndex(of: "--run-dir"), args.count > index + 1 else {
            fatalError("F02 requires --run-dir")
        }
        return URL(fileURLWithPath: args[index + 1], isDirectory: true)
    }
    private func marker(_ key: String) -> Marker { Marker(key: key) }
    private func changed() {
        sourceRevision += 1
        if stimulus != "lost_event" { eventRevision = sourceRevision }
    }
    private func reset() {
        name = ""; secret = ""; checked = false; applied = "none"; activations = 0
        expanded = false; wide = false; popup = false; scrollEnd = false; stimulus = "normal"
        focus = .name; changed()
    }
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Window \(role.uppercased())").font(.headline).accessibilityIdentifier("f02.owner.\(role)")
                Spacer()
                Button("Open \(other.uppercased())") { openWindow(id: other) }.accessibilityIdentifier("f02.open_other")
                Button("Snapshot") {
                    snapshotRequest += 1
                    #if !PROBE
                    publish()
                    #endif
                }.accessibilityIdentifier("f02.snapshot")
                .keyboardShortcut("s", modifiers: [.command, .shift])
                Button("Reset") { reset() }.accessibilityIdentifier("f02.reset")
            }
            HStack(spacing: 12) {
                Button { activations += 1; changed() } label: {
                    HStack(spacing: expanded ? 18 : 8) {
                        Image(systemName: "star.fill").font(.system(size: 20)).modifier(marker("icon"))
                        Text("Activate sample").font(.system(size: expanded ? 22 : 16)).modifier(marker("text"))
                    }
                    .padding(expanded ? 18 : 12)
                    .background(.blue.opacity(0.15), in: RoundedRectangle(cornerRadius: 8))
                    .modifier(marker("container"))
                }
                .buttonStyle(.plain).accessibilityElement(children: .combine)
                .accessibilityIdentifier("f02.sample.\(role)").focused($focus, equals: .sample)
                Text("Count: \(activations)").accessibilityIdentifier("f02.count")
            }
            HStack {
                Button("Change layout") { expanded.toggle(); changed() }.accessibilityIdentifier("f02.layout")
                Button("Resize") { wide.toggle(); changed() }.accessibilityIdentifier("f02.resize")
                Button("Move") { WindowSetup.apply(role: role, comparison: false) }.accessibilityIdentifier("f02.move")
                Button("Compare") {
                    reset(); expanded = true; activations = 1; focus = nil
                    // Allow SwiftUI layout to settle before the explicit setup action.
                    Task { @MainActor in
                        try? await Task.sleep(for: .milliseconds(200))
                        WindowSetup.apply(role: role, comparison: true)
                    }
                }.accessibilityIdentifier("f02.compare")
                Spacer()
                Button("Edge popup") { popup.toggle(); changed() }.accessibilityIdentifier("f02.popup")
                    .popover(isPresented: $popup, arrowEdge: .trailing) {
                        VStack {
                            Text("Popup \(role.uppercased())").accessibilityIdentifier("f02.popup.owner.\(role)")
                            Button("Confirm popup") { applied = "popup-\(role)"; popup = false; changed() }
                                .accessibilityIdentifier("f02.popup.confirm")
                        }.padding(20)
                        .background {
                            PopupContainingWindowReader(metadata: measurements)
                                .frame(width: 0, height: 0)
                                .allowsHitTesting(false)
                                .accessibilityHidden(true)
                        }
                    }
            }
            Divider()
            TextField("Name", text: $name).textFieldStyle(.roundedBorder)
                .focused($focus, equals: .name).accessibilityIdentifier("f02.name")
                .onChange(of: name) { _, _ in changed() }
            SecureField("Synthetic secret", text: $secret).textFieldStyle(.roundedBorder)
                .focused($focus, equals: .secret).accessibilityIdentifier("f02.secret")
                .onChange(of: secret) { _, _ in changed() }
            HStack {
                Toggle("Enabled", isOn: $checked).accessibilityIdentifier("f02.enabled")
                    .onChange(of: checked) { _, _ in changed() }
                Button("Complete Ada") { name = "Ada Lovelace"; focus = .name; changed() }
                    .accessibilityIdentifier("f02.complete")
                Button("Apply form") {
                    applied = name == "Ada Lovelace" && !secret.isEmpty && checked ? "accepted-\(role)" : "invalid"
                    changed()
                }.accessibilityIdentifier("f02.apply")
            }
            Text("Result: \(applied)").accessibilityIdentifier("f02.result")
            ScrollViewReader { proxy in
                ScrollView {
                    VStack(alignment: .leading, spacing: 6) {
                        ForEach(0..<40, id: \.self) { index in
                            Text("Row \(index)").frame(maxWidth: .infinity, alignment: .leading)
                                .id(index).accessibilityIdentifier("f02.row.\(index)")
                        }
                    }.padding(6)
                }.frame(height: 90).border(.secondary).accessibilityIdentifier("f02.scroll")
                Button(scrollEnd ? "Scroll start" : "Scroll end") {
                    scrollEnd.toggle(); proxy.scrollTo(scrollEnd ? 39 : 0, anchor: scrollEnd ? .bottom : .top); changed()
                }.accessibilityIdentifier("f02.scroll_to")
                .onChange(of: scrollEnd) { _, value in if !value { proxy.scrollTo(0, anchor: .top) } }
            }
            HStack {
                Text("Synthetic stimulus")
                Picker("Synthetic stimulus", selection: $stimulus) {
                    ForEach(["normal", "slow", "partial", "lost_event", "permission_denied"], id: \.self) { Text($0).tag($0) }
                }.labelsHidden().accessibilityIdentifier("f02.stimulus")
                Button("Mutate source") { changed() }.accessibilityIdentifier("f02.mutate")
            }
        }
        .padding(20).frame(width: wide ? 650 : 550)
        .coordinateSpace(name: "fixture")
        .defaultFocus($focus, .name)
        #if P01_SAMPLE_HITS
        .task { if role == "a" { await sampledMouseProfile() } }
        #endif
        .onChange(of: popup) { _, _ in
            if !popup { measurements.popupContainingWindow = nil }
            let app = NSRunningApplication.current
            let owner = popupIdentity ?? FixtureIdentity(directory:runDirectory,windowKey:"popup-\(role)")
            popupIdentity = owner
            measurements.popupCurrentIdentity = owner
            do {
                // Both close and a new presentation invalidate previous binding.
                // Only explicit Snapshot can establish a new measured/current record.
                try owner.close(pid:app.processIdentifier,bundle:app.bundleIdentifier ?? "unknown",
                    launch:app.launchDate?.timeIntervalSince1970 ?? 0,window:popupWindowID)
                popupGeneration = owner.generation
            } catch { _exit(2) }
        }
        .onReceive(NotificationCenter.default.publisher(for: NSWindow.willCloseNotification)) { notification in
            guard let window = notification.object as? NSWindow else { return }
            if window.identifier?.rawValue == "popup-\(role)", let popupOwner = popupIdentity {
                let app = NSRunningApplication.current
                do {
                    try popupOwner.close(pid:app.processIdentifier,bundle:app.bundleIdentifier ?? "unknown",
                        launch:app.launchDate?.timeIntervalSince1970 ?? 0,window:window.windowNumber)
                    popupGeneration = popupOwner.generation
                } catch { _exit(2) }
                return
            }
            guard window.identifier?.rawValue == role else { return }
            // SwiftUI Window may reopen the same NSWindow/CGWindowID with retained state.
            let app = NSRunningApplication.current
            let owner = identity ?? FixtureIdentity(directory: runDirectory, windowKey: role)
            identity = owner
            // Rotate and publish CLOSED only. No geometry/probe/manifest collection.
            do {
                try owner.close(pid: app.processIdentifier, bundle: app.bundleIdentifier ?? "unknown",
                    launch: app.launchDate?.timeIntervalSince1970 ?? 0, window: window.windowNumber)
                measurements.surfaceGeneration = owner.generation
            } catch {
                // A stale OPEN file must not survive failure of invalidation.
                try? FileManager.default.removeItem(at: runDirectory.appendingPathComponent("\(role)-identity.json"))
                // If storage cannot invalidate the receipt, fail the OWN debug
                // fixture closed: its public process incarnation is no longer live.
                _exit(2)
            }
        }
        #if PROBE
        .backgroundPreferenceValue(MarkerAnchors.self) { anchors in
            GeometryReader { proxy in
                Color.clear.onChange(of: snapshotRequest) { _, _ in
                    // Resolve measured bounds once per explicit Snapshot request.
                    // Anchor declarations alone are not collected geometry.
                    measurements.frames = [:]
                    for (key, anchor) in anchors { measurements.report(key, frame: proxy[anchor]) }
                    publish()
                }
            }
            .allowsHitTesting(false).accessibilityHidden(true)
        }
        #endif
    }

    #if P01_SAMPLE_HITS
    @MainActor private func sampledMouseProfile() async {
        var records: [[String: Any]] = []
        func state(_ phase: String, window: NSWindow, point: NSPoint? = nil) -> [String: Any] {
            let responder = window.firstResponder as? NSAccessibilityProtocol
            let accessibilityFocus = NSApp.accessibilityFocusedUIElement as? NSAccessibilityProtocol
            let declaredFocus = focus.map { field in
                field == .sample ? "f02.sample.\(role)" : field == .name ? "f02.name" : "f02.secret"
            }
            var value: [String: Any] = ["phase": phase, "count": activations,
                "swiftui_focus": focus?.rawValue ?? "none", "window_id": window.windowNumber,
                "surface_generation": measurements.surfaceGeneration, "app_active": NSApp.isActive,
                "window_key": window.isKeyWindow, "window_main": window.isMainWindow,
                "first_responder_identifier": responder?.accessibilityIdentifier() as Any? ?? NSNull(),
                "accessibility_focus_identifier": accessibilityFocus?.accessibilityIdentifier() as Any? ?? NSNull(),
                "swiftui_focus_declared_control": declaredFocus as Any? ?? NSNull(),
                "first_responder_class": window.firstResponder.map { String(describing: type(of: $0)) } ?? "none"]
            if let point { value["point_window_base_pt"] = ["x": point.x, "y": point.y] }
            return value
        }
        func save(_ status: String, error: String? = nil, metadata: [String: Any]? = nil) {
            var value: [String: Any] = ["profile": "own_intrawindow_synthetic_mouse", "status": status,
                "dispatch": "NSApplication.postEvent_normal_run_loop", "records": records]
            if let error { value["error"] = error }
            if let metadata { value["failure_metadata"] = metadata }
            if let bytes = try? JSONSerialization.data(withJSONObject: value, options: [.sortedKeys]) {
                try? bytes.write(to: runDirectory.appendingPathComponent("sampled-mouse.json"), options: .atomic)
            }
        }
        do {
            expanded = true; activations = 1; focus = .name
            try await Task.sleep(for: .milliseconds(200))
            WindowSetup.apply(role: role, comparison: true)
            try await Task.sleep(for: .milliseconds(200))
            publish()
            let (window, inside, outside) = try WindowSetup.sampledPoints(role: role)
            #if P01_FRAME_DIAGNOSTIC
            records.append(state("diagnostic_points", window: window, point: inside))
            save("diagnostic_ready")
            #else
            records.append(state("before_inside", window: window, point: inside))
            save("running")
            try WindowSetup.postMousePair(window: window, point: inside, number: 1)
            try await Task.sleep(for: .milliseconds(200)) // Return to the existing app event loop.
            publish()
            records.append(state("after_inside", window: window, point: inside))
            guard activations == 2 else { save("failed", error: "inside_count_not_plus_one"); return }
            let (current, newInside, newOutside) = try WindowSetup.sampledPoints(role: role)
            guard current === window, inside == newInside, outside == newOutside
            else { save("failed", error: "window_or_points_changed"); return }
            try WindowSetup.postMousePair(window: window, point: outside, number: 3)
            try await Task.sleep(for: .milliseconds(200))
            publish()
            records.append(state("after_outside", window: window, point: outside))
            save(activations == 2 ? "complete" : "failed", error: activations == 2 ? nil : "outside_count_changed")
            #endif
        } catch {
            save("failed", error: String(describing: error), metadata: (error as NSError).userInfo)
        }
    }
    #endif

    @MainActor private func publish() {
        let app = NSRunningApplication.current
        // Read-only adapter: SwiftUI doesn't expose CGWindowID/launch identity.
        guard let window = NSApp.windows.first(where: { $0.identifier?.rawValue == role && $0.isVisible }) else { return }
        let owner = identity ?? FixtureIdentity(directory: runDirectory, windowKey: role)
        identity = owner
        guard let currentGeneration = try? owner.snapshot(pid: app.processIdentifier,
            bundle: app.bundleIdentifier ?? "unknown", launch: app.launchDate?.timeIntervalSince1970 ?? 0,
            window: window.windowNumber) else { return }
        measurements.windowID = window.windowNumber
        measurements.surfaceGeneration = currentGeneration
        #if PROBE
        let probeEnabled = true
        #else
        let probeEnabled = false
        #endif
        var popupBinding: [String: Any]?
        if popup, let popupWindow = measurements.popupContainingWindow,
           popupWindow !== window, popupWindow.isVisible, popupWindow.windowNumber > 0,
           NSApp.windows.contains(where: { $0 === popupWindow }) {
            let popupOwner = popupIdentity ?? FixtureIdentity(directory:runDirectory,windowKey:"popup-\(role)")
            popupIdentity = popupOwner
            measurements.popupCurrentIdentity = popupOwner
            let popupKey = "popup-\(role)"
            // Direct own content attachment is the physical owner evidence.
            // A shared parent is never renamed or silently treated as separate.
            popupWindow.identifier = NSUserInterfaceItemIdentifier(popupKey)
            popupWindow.setAccessibilityIdentifier(popupKey)
            if let generation = try? popupOwner.snapshot(pid:app.processIdentifier,
                bundle:app.bundleIdentifier ?? "unknown",launch:app.launchDate?.timeIntervalSince1970 ?? 0,window:popupWindow.windowNumber) {
                popupGeneration = generation; popupWindowID = popupWindow.windowNumber
                popupBinding = ["pid":app.processIdentifier,"bundle_id":app.bundleIdentifier ?? "unknown",
                    "launch_time":app.launchDate?.timeIntervalSince1970 ?? 0,"window_id":popupWindow.windowNumber,
                    "window_identifier":popupKey,"target_generation":"\(app.processIdentifier):\(app.launchDate?.timeIntervalSince1970 ?? 0)",
                    "surface_generation":generation,"identity_path":runDirectory.appendingPathComponent("\(popupKey)-identity.json").path]
            }
        }
        let frames = (try? JSONSerialization.jsonObject(with: JSONEncoder().encode(measurements.frames))) ?? [:]
        let processStart = app.launchDate?.timeIntervalSince1970 ?? 0
        let windows = NSApp.windows.filter { $0.isVisible }.map {
            ["window_id": $0.windowNumber, "identifier": $0.identifier?.rawValue ?? "unknown",
             "frame_appkit_screen_pt": ["x": $0.frame.minX, "y": $0.frame.minY, "width": $0.frame.width, "height": $0.frame.height]] as [String: Any]
        }
        let containingEvidence: [String: Any]
        if popup, let containing = measurements.popupContainingWindow, containing.windowNumber > 0 {
            containingEvidence = ["status": "known", "window_id": containing.windowNumber,
                "equals_parent": containing === window, "is_visible": containing.isVisible,
                "source": "public_NSView_window_viewDidMoveToWindow", "ownership": "own_fixture_content_attachment"]
        } else { containingEvidence = ["status": "unavailable", "source": "public_NSView_window_viewDidMoveToWindow"] }
        let value: [String: Any] = [
            "environment": "task_owned_synthetic_debug", "role": role,
            "pid": app.processIdentifier, "bundle_id": app.bundleIdentifier ?? "unknown", "launch_time": processStart,
            "target_generation": "\(app.processIdentifier):\(processStart)",
            "surface_generation": measurements.surfaceGeneration, "window_id": window.windowNumber,
            "identity_path": runDirectory.appendingPathComponent("\(role)-identity.json").path,
            "live_manifest_path": runDirectory.appendingPathComponent("\(role).json").path,
            "popup_containing_window": containingEvidence,
            "popup_binding": popupBinding ?? NSNull(), "popup_binding_status": popupBinding == nil ? "unresolved" : "bound",
            "popup_binding_reason": popupBinding != nil ? "direct_content_attachment" :
                (popup && measurements.popupContainingWindow === window ? "shared_native_window_requires_logical_surface_identity" : "current_content_attachment_unavailable"),
            "popup_generation": popupGeneration, "popup_anchor_declared": role,
            "window_identifier": role, "windows": windows,
            "app_active": app.isActive, "window_key": window.isKeyWindow, "window_main": window.isMainWindow,
            "source_state": ["revision": sourceRevision, "event_revision": eventRevision, "stimulus": stimulus],
            "state": ["name": name, "secret": ["availability": "redacted"], "secret_present": !secret.isEmpty,
                      "checked": checked, "applied": applied, "activations": activations, "expanded": expanded,
                      "wide": wide, "popup": popup, "scroll_end": scrollEnd, "focus": focus?.rawValue ?? "none"],
            "snapshot_request": snapshotRequest, "collection_mode": "explicit_request_only",
            "probe_enabled": probeEnabled, "probe": ["source": "swiftui.anchorPreference.explicit_snapshot", "provenance": "reported",
                "units": "pt", "origin": "top_left", "coordinate_space": "fixture_local",
                "screen_transform": "unknown", "layout_bounds": frames,
                "callbacks": measurements.callbackCount, "callback_nanoseconds": measurements.callbackNanoseconds],
            "source_declarations": ["logical_component_key": "f02.sample.\(role)", "represents": ["icon", "text", "container"]],
            "display_scale": displayScale, "observation_utc": ISO8601DateFormatter().string(from: Date()),
            "uptime_seconds": ProcessInfo.processInfo.systemUptime]
        if let data = try? JSONSerialization.data(withJSONObject: value, options: [.sortedKeys]) {
            try? data.write(to: runDirectory.appendingPathComponent("\(role).json"), options: .atomic)
        }
    }
}

#endif
