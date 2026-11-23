// R11 synthetic fixture. Shape follows e19b20c474 FeatureGateSubscribePreview;
// never linked into PlayPhrase.me. Images and identities are synthetic.
import AppKit
import SwiftUI

private struct CellBounds: PreferenceKey {
    static let defaultValue: [Int: Anchor<CGRect>] = [:]
    static func reduce(value: inout [Int: Anchor<CGRect>], nextValue: () -> [Int: Anchor<CGRect>]) {
        value.merge(nextValue(), uniquingKeysWith: { _, _ in preconditionFailure("duplicate cell") })
    }
}
private extension View {
    @ViewBuilder func cellOracle(_ id: Int) -> some View {
        #if R11_NO_ORACLE
        self
        #else
        transformAnchorPreference(key: CellBounds.self, value: .bounds) { values, anchor in
            values[id] = anchor
        }
        #endif
    }
}

struct FeatureGateSubscribePreview: View {
    let phase: Int
    let ids: [Int]
    private let columns = Array(repeating: GridItem(.flexible(), spacing: 6), count: 7)
    private static let spacing: CGFloat = 6
    private static let radius: CGFloat = 8
    // A local resource, with no media, networking, accounts or user data.
    private static let sample = NSImage(size: NSSize(width: 80, height: 48), flipped: false) { rect in
        NSColor.systemBlue.setFill(); rect.fill()
        NSColor.systemOrange.setFill(); NSRect(x: 20, y: 0, width: 20, height: 48).fill()
        return true
    }

    var body: some View {
        VStack(spacing: 8) {
            if phase == 0 { loadingGrid } else { previewGrid }
        }
    }

    private var loadingGrid: some View {
        LazyVGrid(columns: columns, spacing: Self.spacing) {
            ForEach(0..<21, id: \.self) { id in
                tileBackground.cellOracle(id + (ids.first ?? 0))
            }
        }
        .accessibilityHidden(true)
    }

    private var previewGrid: some View {
        LazyVGrid(columns: columns, spacing: Self.spacing) {
            ForEach(ids, id: \.self) { id in
                tileBackground
                    .overlay {
                        if id != 19 {
                            Image(nsImage: Self.sample).resizable().scaledToFill()
                        }
                    }
                    .clipShape(RoundedRectangle(cornerRadius: Self.radius, style: .continuous))
                    .overlay {
                        RoundedRectangle(cornerRadius: Self.radius, style: .continuous)
                            .strokeBorder(Color.white.opacity(0.05), lineWidth: 0.847059)
                    }
                    .cellOracle(id)
            }
        }
        .accessibilityHidden(true)
    }

    private var tileBackground: some View {
        RoundedRectangle(cornerRadius: Self.radius, style: .continuous)
            .fill(LinearGradient(colors: [Color.white.opacity(0.06), Color.white.opacity(0.14)],
                                 startPoint: .topLeading, endPoint: .bottomTrailing))
            .aspectRatio(1, contentMode: .fit)
    }
}

// Nonvisual oracle adapter: SwiftUI does not expose NSWindow's coordinate space.
// It converts independently resolved PUBLIC anchors, not collector/cache data.
private final class OracleView: NSView {
    override var isFlipped: Bool { true }
    override func hitTest(_ point: NSPoint) -> NSView? { nil }
    var publish: (() -> Void)?
    override func viewDidMoveToWindow() { super.viewDidMoveToWindow(); publish?() }
}
private struct NativeOracle: NSViewRepresentable {
    let frames: [Int: CGRect]
    let rootSize: CGSize
    let phase: Int
    let duplicate: Bool
    let scale: CGFloat
    let directory: URL
    func makeNSView(context: Context) -> OracleView {
        let view = OracleView(); view.setAccessibilityElement(false); return view
    }
    func updateNSView(_ view: OracleView, context: Context) {
        view.publish = { [weak view] in
            DispatchQueue.main.async {
                guard let view, let window = view.window, view.bounds.origin == .zero,
                      view.bounds.size == rootSize else { return }
                var values: [String: Any] = [:]
                for (id, rect) in frames {
                    let native = view.convert(rect, to: nil)
                    values[String(id)] = ["root": [rect.minX, rect.minY, rect.width, rect.height],
                        "window": [native.minX, window.frame.height - native.maxY, native.width, native.height]]
                }
                let document: [String: Any] = ["format": "r11-public-anchor-oracle-1",
                    "pid": ProcessInfo.processInfo.processIdentifier, "window_id": window.windowNumber,
                    "phase": phase, "duplicate": duplicate, "display_scale": scale,
                    "root_size": [rootSize.width, rootSize.height], "cells": values]
                do {
                    let data = try JSONSerialization.data(withJSONObject: document, options: [.sortedKeys])
                    try data.write(to: directory.appendingPathComponent("oracle.json"), options: .atomic)
                } catch { preconditionFailure("oracle write failed") }
            }
        }
        view.publish?()
    }
}

struct R11Root: View {
    @Environment(\.displayScale) private var displayScale
    @State private var phase = 0
    private let ids = Array(0..<21)
    private let directory = URL(fileURLWithPath: ProcessInfo.processInfo.environment["UIB_R11_CONTROL"]!)
    var body: some View {
        ZStack(alignment: .topLeading) {
            Color.black
            FeatureGateSubscribePreview(phase: phase == 3 ? 0 : phase, ids: ids)
                .frame(width: phase == 2 ? 574 : 490)
                .offset(x: phase == 2 ? 47 : 23, y: phase == 2 ? 61 : 31)
                .scaleEffect(phase == 4 ? 1.25 : 1, anchor: .topLeading)
            if phase == 3 {
                FeatureGateSubscribePreview(phase: 0, ids: ids.map { $0 + 100 })
                    .frame(width: 490).offset(x: 23, y: 280)
            }
        }
        .frame(width: phase == 2 ? 700 : 620, height: phase == 3 ? 600 : 400)
        .overlayPreferenceValue(CellBounds.self) { anchors in
            GeometryReader { root in
                NativeOracle(frames: anchors.mapValues { root[$0] }, rootSize: root.size,
                             phase: phase, duplicate: phase == 3, scale: displayScale, directory: directory)
                    .allowsHitTesting(false).accessibilityHidden(true)
            }
        }
        .task {
            // Fixture-only bounded control ingress; never installed in a donor.
            let end = ContinuousClock.now.advanced(by: .seconds(1800))
            while !Task.isCancelled && ContinuousClock.now < end {
                if FileManager.default.fileExists(atPath: directory.appendingPathComponent("finish").path) {
                    NSApplication.shared.terminate(nil); return
                }
                if let text = try? String(contentsOf: directory.appendingPathComponent("phase"), encoding: .utf8),
                   let requested = Int(text.trimmingCharacters(in: .whitespacesAndNewlines)),
                   (0...4).contains(requested) { phase = requested }
                try? await Task.sleep(for: .milliseconds(100))
            }
        }
    }
}

@main struct R11PreviewFixture: App {
    var body: some Scene {
        Window("R11 synthetic preview grid", id: "fixture") { R11Root() }
            .windowResizability(.contentSize)
    }
}
