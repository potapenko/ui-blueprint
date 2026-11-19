// Synthetic only. Oracle rectangles are SwiftUI bounds resolved at the fixture root.
import AppKit
import SwiftUI

private struct FixtureBounds: PreferenceKey {
    static let defaultValue: [String: Anchor<CGRect>] = [:]

    static func reduce(value: inout [String: Anchor<CGRect>], nextValue: () -> [String: Anchor<CGRect>]) {
        value.merge(nextValue(), uniquingKeysWith: { _, _ in
            preconditionFailure("Duplicate fixture role")
        })
    }
}

private extension View {
    func oracle(_ role: String) -> some View {
        transformAnchorPreference(key: FixtureBounds.self, value: .bounds) { bounds, anchor in bounds[role] = anchor }
    }
}

struct FormRow: View {
    let index: Int

    var body: some View {
        HStack {
            Text("Label").oracle("label-\(index)")
            Spacer()
            Text("Value").oracle("value-\(index)")
        }
        .frame(height: 40)
    }
}

struct SettingsForm: View {
    let directory: URL

    var body: some View {
        VStack(spacing: 0) {
            HStack {
                Text("Settings").oracle("title")
                Spacer()
                Text("Close")
            }
            .padding(12)
            .oracle("header")
            ScrollViewReader { scroll in
                ScrollView {
                    VStack(spacing: 6) {
                        ForEach(0..<3, id: \.self) { index in
                            FormRow(index: index)
                                .id(index)
                            if index < 2 {
                                Divider().oracle("divider-\(index)")
                            }
                        }
                    }
                    .padding(.horizontal, 10)
                }
                .frame(height: 86)
                .oracle("inner")
                .task {
                    while !Task.isCancelled {
                        if FileManager.default.fileExists(atPath: directory.appendingPathComponent("scroll").path) {
                            scroll.scrollTo(2, anchor: .bottom)
                            return
                        }
                        try? await Task.sleep(for: .milliseconds(150))
                    }
                }
            }
        }
        .padding(14)
        .frame(width: 360, height: 220)
        .overlayPreferenceValue(FixtureBounds.self) { anchors in
            GeometryReader { root in
                let frames = anchors.mapValues { root[$0] }
                Color.clear.onChange(of: frames, initial: true) { _, current in
                    let rects = current.mapValues { [$0.minX, $0.minY, $0.width, $0.height] }
                    do {
                        try JSONEncoder().encode(rects).write(to: directory.appendingPathComponent("oracle.json"), options: .atomic)
                    } catch {
                        preconditionFailure("Unable to save oracle")
                    }
                }
            }
        }
    }
}

@main
struct SettingsGeometryFixture: App {
    var body: some Scene {
        Window("Settings geometry fixture", id: "fixture") {
            SettingsForm(directory: URL(fileURLWithPath: ProcessInfo.processInfo.environment["UIB_FIXTURE_CONTROL"]!))
        }
        .windowResizability(.contentSize)
    }
}
