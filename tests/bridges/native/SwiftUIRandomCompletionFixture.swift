// Tool-owned source-shaped S13 fixture. Never added to a donor application.
import AppKit
import Foundation
import SwiftUI

// Reproduces the donor's existing actionless native cursor attachment only.
// Visible content, state and geometry are SwiftUI; the adapter supplies no layout.
final class S13CursorView: NSView {
    override func hitTest(_ point: NSPoint) -> NSView? { nil }
}
struct S13CursorArea: NSViewRepresentable {
    func makeNSView(context: Context) -> S13CursorView {
        let view = S13CursorView(); view.setAccessibilityElement(false); return view
    }
    func updateNSView(_ view: S13CursorView, context: Context) {}
}
private struct S13Bounds: PreferenceKey {
    static let defaultValue: [String: Anchor<CGRect>] = [:]
    static func reduce(value: inout [String: Anchor<CGRect>], nextValue: () -> [String: Anchor<CGRect>]) {
        value.merge(nextValue(), uniquingKeysWith: { _, _ in preconditionFailure("duplicate role") })
    }
}
private extension View {
    func oracle(_ name: String) -> some View {
        #if S13_NO_ORACLE
        self
        #else
        transformAnchorPreference(key: S13Bounds.self, value: .bounds) { value, anchor in value[name] = anchor }
        #endif
    }
}
private struct RandomCompletionActionButton: View {
    let role: String
    let label: String
    let shortcut: Bool
    var body: some View {
        Button {} label: {
            HStack(spacing: 8.470588) {
                Image(systemName: "arrow.clockwise").resizable().scaledToFit()
                    .frame(width: 16.941176, height: 16.941176).oracle(role+"-icon")
                Text(label).font(.system(size: 11.858824, weight: .bold))
                    .fixedSize(horizontal: false, vertical: true).oracle(role+"-label")
                Spacer(minLength: 0)
                if shortcut {
                    Text("Enter").font(.system(size: 8.470588, weight: .bold))
                        .padding(.vertical, 2.541176).padding(.horizontal, 5.082353)
                        .background(.white.opacity(0.12), in: RoundedRectangle(cornerRadius: 5.082353))
                        .oracle(role+"-keycap")
                }
            }
            .padding(.horizontal, 13.552941).padding(.vertical, 8.470588)
            .frame(minWidth: 211.764706, maxWidth: .infinity).frame(minHeight: 38.964706)
            .background(.blue.opacity(0.6), in: Capsule()).oracle(role+"-button")
        }
        .buttonStyle(.plain)
        .overlay { S13CursorArea().accessibilityHidden(true) }
    }
}
struct RandomCompletionDialog: View {
    let phase: Int
    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            Text("Synthetic random completion").font(.system(size: 18.635294, weight: .bold)).oracle("title")
            Text("Synthetic message for layout qualification.").font(.system(size: 11.858824))
                .padding(.top, 10.164706).oracle("message")
            ViewThatFits(in: .horizontal) {
                HStack(spacing: 6.776471) { actions }
                VStack(spacing: 6.776471) { actions }
            }
            .padding(.top, 18.635294).oracle("actions")
        }
        .padding(23.717647).frame(width: 540)
        .background(.black.opacity(0.8), in: RoundedRectangle(cornerRadius: 16.941176))
        .oracle("card")
    }
    @ViewBuilder private var actions: some View {
        if phase != 0 {
            RandomCompletionActionButton(role: "replay", label: "Play favorites again", shortcut: phase == 1)
        }
        if phase >= 2 {
            RandomCompletionActionButton(role: "next", label: "Play phrases from next page", shortcut: true)
        } else {
            Button("Close", systemImage: "xmark") {}.labelStyle(.iconOnly)
                .buttonStyle(.bordered).buttonBorderShape(.circle).oracle("close")
                .frame(minWidth: 211.764706, maxWidth: .infinity, minHeight: 38.964706)
                .overlay { S13CursorArea().accessibilityHidden(true) }
        }
    }
}
struct S13Root: View {
    @State private var phase = 1
    var body: some View {
        VStack {
            Text("Unrelated sentinel").frame(width: 37, height: 19)
            RandomCompletionDialog(phase: phase)
        }
        .frame(width: 760, height: 400)
        .overlayPreferenceValue(S13Bounds.self) { anchors in
            GeometryReader { geometry in
                let frames = anchors.mapValues { geometry[$0] }.merging(
                    ["root-global": geometry.frame(in: .global)]
                ) { _, new in new }
                Color.clear.onChange(of: frames, initial: true) { _, current in
                    guard let directory = ProcessInfo.processInfo.environment["UIB_S13_CONTROL"] else { return }
                    let values = current.mapValues { [$0.minX, $0.minY, $0.width, $0.height] }
                    let document: [String: Any] = ["pid": ProcessInfo.processInfo.processIdentifier,
                                                  "phase": phase, "frames": values]
                    if let data = try? JSONSerialization.data(withJSONObject: document, options: [.sortedKeys]) {
                        try? data.write(to: URL(fileURLWithPath: directory).appendingPathComponent("oracle.json"), options: .atomic)
                    }
                }
            }
        }
        .task {
            guard let directory = ProcessInfo.processInfo.environment["UIB_S13_CONTROL"] else { return }
            while !Task.isCancelled {
                if let text = try? String(contentsOfFile: directory+"/phase", encoding: .utf8),
                   let value = Int(text.trimmingCharacters(in: .whitespacesAndNewlines)), (0...2).contains(value) {
                    phase = value
                }
                try? await Task.sleep(for: .milliseconds(100))
            }
        }
    }
}
@main struct S13RandomFixture: App {
    var body: some Scene {
        Window("S13 random fixture", id: "fixture") { S13Root() }.windowResizability(.contentSize)
    }
}
