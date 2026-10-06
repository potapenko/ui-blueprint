import SwiftUI
import AppKit

// Diagnostic fixture only. All visible content, windows and state are SwiftUI.
// AppKit below is a read-only own-process window/launch identity adapter:
// SwiftUI's scene API does not expose the CGWindowID needed by ScreenCaptureKit.
@main
struct R02Fixture: App {
    var body: some Scene {
        Window("R02 Synthetic", id: "primary") {
            FixtureView()
        }
        .defaultSize(width: 440, height: 280)
        .windowResizability(.contentSize)
    }
}

private struct RectValue: Codable, Equatable {
    let x: Double, y: Double, width: Double, height: Double
    init(_ r: CGRect) { x = r.minX; y = r.minY; width = r.width; height = r.height }
}

private struct Measured: ViewModifier {
    let key: String
    let report: (String, CGRect) -> Void
    @ViewBuilder func body(content: Content) -> some View {
        #if PROBE
        content.onGeometryChange(for: CGRect.self) { $0.frame(in: .named("fixture")) } action: {
            report(key, $0)
        }
        #else
        content
        #endif
    }
}

private struct FixtureView: View {
    @State private var expanded = false
    @State private var activations = 0
    @State private var rects: [String: RectValue] = [:]
    @State private var generation = UUID().uuidString
    @FocusState private var sampleFocused: Bool
    @Environment(\.displayScale) private var displayScale
    private var runDirectory: URL {
        let a = CommandLine.arguments
        guard let i = a.firstIndex(of: "--run-dir"), a.count > i + 1 else {
            fatalError("Fixture requires --run-dir")
        }
        return URL(fileURLWithPath: a[i + 1], isDirectory: true)
    }
    private func measure(_ key: String) -> Measured {
        Measured(key: key) { k, frame in rects[k] = RectValue(frame) }
    }
    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            Text("Owned native fixture").font(.headline)
            Button { activations += 1 } label: {
                HStack(spacing: expanded ? 18 : 8) {
                    Image(systemName: "star.fill")
                        .font(.system(size: 20)).modifier(measure("icon"))
                    Text("Activate sample")
                        .font(.system(size: expanded ? 22 : 16))
                        .modifier(measure("text"))
                }
                .padding(expanded ? 18 : 12)
                .background(.blue.opacity(0.15), in: RoundedRectangle(cornerRadius: 8))
                .modifier(measure("container"))
            }
            .buttonStyle(.plain)
            .accessibilityElement(children: .combine)
            .accessibilityIdentifier("r02.sample")
            .focused($sampleFocused)
            Text("Activations: \(activations)").accessibilityIdentifier("r02.result")
            Button("Change layout") { expanded.toggle() }
                .accessibilityIdentifier("r02.change")
        }
        .padding(24)
        .frame(width: 440, height: 280, alignment: .topLeading)
        .coordinateSpace(name: "fixture")
        .task {
            // A bounded own-fixture manifest, never a general UI scanner.
            for _ in 0..<1200 {
                try? await Task.sleep(for: .milliseconds(500))
                if Task.isCancelled { return }
                save()
            }
        }
    }
    @MainActor private func save() {
        let app = NSRunningApplication.current
        let windows: [[String: Any]] = NSApp.windows.filter { $0.isVisible && $0.title == "R02 Synthetic" }.map {
            ["window_id": $0.windowNumber,
             "frame_appkit_screen_pt": ["x": $0.frame.minX, "y": $0.frame.minY,
                                         "width": $0.frame.width, "height": $0.frame.height],
             "identifier": $0.identifier?.rawValue ?? "unknown"]
        }
        #if PROBE
        let enabled = true
        #else
        let enabled = false
        #endif
        let frames = (try? JSONSerialization.jsonObject(with: JSONEncoder().encode(rects))) ?? [:]
        let report: [String: Any] = [
            "environment": "task_owned_synthetic_debug", "probe_enabled": enabled,
            "target_generation": generation, "pid": app.processIdentifier,
            "bundle_id": app.bundleIdentifier ?? "unknown",
            "launch_time": app.launchDate?.timeIntervalSince1970 ?? 0,
            "windows": windows, "expanded": expanded, "activations": activations,
            "sample_focused_reported_by_swiftui": sampleFocused,
            "observation_utc": ISO8601DateFormatter().string(from: Date()),
            "uptime_seconds": ProcessInfo.processInfo.systemUptime,
            "clock_domain": "fixture_process_uptime",
            "display_scale_reported_by_swiftui": displayScale,
            "probe": ["source": "swiftui.onGeometryChange", "provenance": "reported",
                      "coordinate_space": "fixture_local", "origin": "top_left", "units": "pt",
                      "screen_transform": "unknown", "layout_bounds": frames],
            "source_declarations": ["logical_component_key": "r02.sample",
                                    "represents": ["container", "icon", "text"],
                                    "ax_identifier": "r02.sample"]
        ]
        do {
            let data = try JSONSerialization.data(withJSONObject: report, options: [.sortedKeys, .prettyPrinted])
            try data.write(to: runDirectory.appendingPathComponent("manifest.json"), options: .atomic)
        } catch { /* The runner detects a missing/stale manifest; no UI values in errors. */ }
    }
}
