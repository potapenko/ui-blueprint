import AppKit
import SwiftUI
import Observation
import UIBlueprintSwiftUIRuntime

// An eager large tree plus a small component. No product application/state is used.
@MainActor @Observable final class SubtreeModel { var phase = 0 }
struct ScopeIcon: View { var body: some View { Image(systemName: "arrow.clockwise") } }
struct ScopeLabel: View { var body: some View { Text("Play again") } }
struct ScopeShortcut: View { var body: some View { Text("Return") } }
struct ScopeDialog: View {
    let model: SubtreeModel
    private let canary = "UIB_RUNTIME_MODEL_CANARY_317cce"
    var body: some View {
        GroupBox {
            Button(action: {}) { HStack { ScopeIcon(); ScopeLabel(); ScopeShortcut() } }
            Button("Close", action: {})
            Color.blue.frame(width: model.phase == 1 ? 180 : 120, height: 30)
        }.padding(8)
    }
}
struct ScopeRoot: View {
    let model: SubtreeModel
    var body: some View {
        VStack {
            if model.phase != 2 {
                ScopeDialog(model: model)
                if model.phase == 3 { ScopeDialog(model: model) }
            }
            ScrollView {
                VStack(spacing: 1) {
                    ForEach(0..<(model.phase == 4 ? 1 : model.phase == 5 ? 1100 : 450), id: \.self) { index in
                        HStack {
                            Text("Row \(index)")
                            Color.gray.frame(width: 20, height: 12)
                        }.padding(1).background(Color.white)
                    }
                }
            }.frame(height: 200)
        }.frame(width: 640, height: 400)
    }
}
@main struct SubtreeFixture: App {
    @State private var model = SubtreeModel()
    private let session: BlueprintRuntimeSession?
    init() { session = try! BlueprintRuntimeSession.startIfRequested() }
    var body: some Scene {
        Window("UI Blueprint subtree fixture", id: "subtree") {
            ScopeRoot(model: model).task {
                guard let value = ProcessInfo.processInfo.environment["UIB_FIXTURE_CONTROL"] else { return }
                let root = URL(fileURLWithPath: value)
                for _ in 0..<7200 {
                    if FileManager.default.fileExists(atPath: root.appendingPathComponent("quit").path) {
                        session?.stop(); NSApp.terminate(nil); return
                    }
                    if let text = try? String(contentsOf: root.appendingPathComponent("phase"), encoding: .utf8),
                       let phase = Int(text.trimmingCharacters(in: .whitespacesAndNewlines)), (0...5).contains(phase) {
                        model.phase = phase
                    }
                    try? await Task.sleep(for: .milliseconds(50))
                }
                session?.stop(); NSApp.terminate(nil)
            }
        }.windowResizability(.contentSize)
    }
}
