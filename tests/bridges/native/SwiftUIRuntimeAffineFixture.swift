// Public SwiftUI geometry oracles. Only this synthetic fixture writes control data.
import AppKit
import SwiftUI
import Observation
import UIBlueprintSwiftUIRuntime

@MainActor @Observable final class AffineFixtureModel {
    var phase = 0
    @ObservationIgnored var rectangles: [String: CGRect] = [:]
    func record(_ name: String, _ rect: CGRect) { rectangles[name] = rect }
}

struct OracleText: View {
    let model: AffineFixtureModel
    private let canary = "UIB_RUNTIME_MODEL_CANARY_317cce"
    var body: some View {
        Text(model.phase == 1 ? "A changed multiline fixture label" : "Fixture label")
            .onGeometryChange(for: CGRect.self) { $0.frame(in: .global) } action: { model.record("text", $0) }
            .frame(width: 90).padding(17).opacity(0.7)
    }
}

struct AffineFixtureContent: View {
    let model: AffineFixtureModel
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 12) {
                Color.blue.frame(width: model.phase == 1 ? 140 : 80, height: 60)
                    .onGeometryChange(for: CGRect.self) { $0.frame(in: .global) } action: { model.record("color", $0) }
                OracleText(model: model)
                Button {} label: {
                    Text("Fixture action")
                        .onGeometryChange(for: CGRect.self) { $0.frame(in: .global) } action: { model.record("button", $0) }
                }
                ForEach(0..<(model.phase == 1 ? 4 : 2), id: \.self) { index in
                    Text("Row \(index)").accessibilityElement(children: .contain)
                        .accessibilityLabel("Synthetic row").accessibilityAddTraits(.isButton)
                        .accessibilityAction {}
                }
            }.padding(16).frame(height: 600, alignment: .topLeading)
        }
        .defaultScrollAnchor(.center).frame(width: 320, height: 160)
        .coordinateSpace(name: "fixture-scroll")
        .scaleEffect(model.phase == 2 ? 1 : 1.25, anchor: .topLeading)
        .rotationEffect(.degrees(model.phase == 2 ? 0 : 12), anchor: .topLeading)
        .offset(x: 17, y: 13)
        .frame(width: model.phase == 1 ? 800 : 640, height: model.phase == 1 ? 450 : 400, alignment: .topLeading)
        .accessibilityElement(children: .contain)
    }
}

@main struct AffineRuntimeFixture: App {
    @State private var model = AffineFixtureModel()
    private let session: BlueprintRuntimeSession?
    init() { session = try! BlueprintRuntimeSession.startIfRequested() }
    var body: some Scene {
        Window("UI Blueprint affine runtime fixture", id: "affine") {
            AffineFixtureContent(model: model)
                .toolbar {
                    ToolbarItem {
                        Text("Fixture toolbar")
                            .onGeometryChange(for: CGRect.self) { $0.frame(in: .global) } action: { model.record("toolbar", $0) }
                    }
                }
                .task {
                    guard let value = ProcessInfo.processInfo.environment["UIB_FIXTURE_CONTROL"] else { return }
                    let root = URL(fileURLWithPath: value)
                    for _ in 0..<7200 {
                        if FileManager.default.fileExists(atPath: root.appendingPathComponent("quit").path) {
                            session?.stop(); NSApp.terminate(nil); return
                        }
                        if let phase = try? String(contentsOf: root.appendingPathComponent("phase"), encoding: .utf8),
                           let number = Int(phase.trimmingCharacters(in: .whitespacesAndNewlines)), (0...2).contains(number) {
                            model.phase = number
                        }
                        let trigger = root.appendingPathComponent("oracle.request")
                        if FileManager.default.fileExists(atPath: trigger.path) {
                            try? FileManager.default.removeItem(at: trigger)
                            let payload: [String: Any] = ["phase": model.phase, "rectangles": model.rectangles.mapValues {
                                [$0.minX, $0.minY, $0.width, $0.height]
                            }]
                            if let data = try? JSONSerialization.data(withJSONObject: payload) {
                                try? data.write(to: root.appendingPathComponent("oracle.json"), options: .atomic)
                            }
                        }
                        try? await Task.sleep(for: .milliseconds(50))
                    }
                    session?.stop(); NSApp.terminate(nil)
                }
        }.windowResizability(.contentSize)
    }
}
