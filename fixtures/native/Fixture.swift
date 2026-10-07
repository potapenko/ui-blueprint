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
        .onChange(of: popup) { _, _ in popupGeneration = UUID().uuidString }
        .onReceive(NotificationCenter.default.publisher(for: NSWindow.willCloseNotification)) { notification in
            guard let window = notification.object as? NSWindow, window.identifier?.rawValue == role else { return }
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
        let frames = (try? JSONSerialization.jsonObject(with: JSONEncoder().encode(measurements.frames))) ?? [:]
        let processStart = app.launchDate?.timeIntervalSince1970 ?? 0
        let windows = NSApp.windows.filter { $0.isVisible }.map {
            ["window_id": $0.windowNumber, "identifier": $0.identifier?.rawValue ?? "unknown",
             "frame_appkit_screen_pt": ["x": $0.frame.minX, "y": $0.frame.minY, "width": $0.frame.width, "height": $0.frame.height]] as [String: Any]
        }
        let value: [String: Any] = [
            "environment": "task_owned_synthetic_debug", "role": role,
            "pid": app.processIdentifier, "bundle_id": app.bundleIdentifier ?? "unknown", "launch_time": processStart,
            "target_generation": "\(app.processIdentifier):\(processStart)",
            "surface_generation": measurements.surfaceGeneration, "window_id": window.windowNumber,
            "identity_path": runDirectory.appendingPathComponent("\(role)-identity.json").path,
            "live_manifest_path": runDirectory.appendingPathComponent("\(role).json").path,
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
