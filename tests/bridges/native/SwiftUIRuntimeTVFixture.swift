// Synthetic tvOS public-geometry oracle; never used in a consumer application.
import SwiftUI
import UIBlueprintSwiftUIRuntime

@MainActor final class TVGeometryOracle {
    var rectangles: [String: [Double]] = [:]
    func record(_ name: String, _ frame: CGRect, scale: CGFloat) {
        rectangles[name] = [frame.minX, frame.minY, frame.width, frame.height]
        let payload: [String: Any] = ["rectangles": rectangles, "display_scale": scale]
        guard let data = try? JSONSerialization.data(withJSONObject: payload) else { return }
        try? data.write(to: FileManager.default.temporaryDirectory.appendingPathComponent("tv-oracle.json"),
                        options: .atomic)
    }
}

struct TVOracleContent: View {
    let oracle: TVGeometryOracle
    @Environment(\.displayScale) private var scale
    var body: some View {
        ZStack(alignment: .topLeading) {
            Color.black
            Rectangle().fill(Color.red)
                .frame(width: 180, height: 90)
                .onGeometryChange(for: CGRect.self) { $0.frame(in: .global) } action: {
                    oracle.record("marker", $0, scale: scale)
                }
                .scaleEffect(1.25, anchor: .topLeading)
                .offset(x: 240, y: 160)
        }
        .ignoresSafeArea()
        .onGeometryChange(for: CGRect.self) { $0.frame(in: .global) } action: {
            oracle.record("root", $0, scale: scale)
        }
    }
}

@main struct TVRuntimeGeometryFixture: App {
    private let oracle = TVGeometryOracle()
    private let session: BlueprintRuntimeSession?
    init() { session = try? BlueprintRuntimeSession.startIfRequested() }
    var body: some Scene {
        WindowGroup { TVOracleContent(oracle: oracle) }
    }
}
