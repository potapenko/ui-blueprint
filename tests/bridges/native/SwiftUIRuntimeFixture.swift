import AppKit
import SwiftUI
import Observation
import UIBlueprintSwiftUIRuntime

@MainActor @Observable final class EndpointModel { var phase = 0 }
struct EndpointContent: View {
    let model: EndpointModel
    private let canary = "UIB_RUNTIME_MODEL_CANARY_317cce"
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack(spacing: 8) {
                Color.blue.frame(width: model.phase == 1 ? 140 : 80, height: 60)
                Text("Runtime query fixture")
                if model.phase == 1 { Color.green.frame(width: 40, height: 40) }
            }
            Button("Action") {}
            ForEach(0..<(model.phase == 1 ? 4 : 2), id: \.self) { Text("Row \($0)") }
        }
        .padding(16)
        .frame(width: model.phase == 1 ? 800 : 640, height: model.phase == 1 ? 450 : 400, alignment: .topLeading)
    }
}
@main struct EndpointFixture: App {
    @State private var model = EndpointModel()
    private let session: BlueprintRuntimeSession?
    init() { session = try! BlueprintRuntimeSession.startIfRequested() }
    var body: some Scene {
        Window("UI Blueprint runtime endpoint fixture", id: "endpoint") {
            EndpointContent(model: model).task {
                guard let value = ProcessInfo.processInfo.environment["UIB_FIXTURE_CONTROL"] else { return }
                let root = URL(fileURLWithPath: value)
                for _ in 0..<7200 {
                    if FileManager.default.fileExists(atPath: root.appendingPathComponent("quit").path) {
                        session?.stop(); NSApp.terminate(nil); return
                    }
                    if let phase = try? String(contentsOf: root.appendingPathComponent("phase"), encoding: .utf8),
                       let number = Int(phase.trimmingCharacters(in: .whitespacesAndNewlines)), (0...1).contains(number) {
                        model.phase = number
                    }
                    try? await Task.sleep(for: .milliseconds(50))
                }
                session?.stop(); NSApp.terminate(nil)
            }
        }.windowResizability(.contentSize)
    }
}
