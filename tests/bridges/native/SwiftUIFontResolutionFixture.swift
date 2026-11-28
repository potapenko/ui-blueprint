// Project-owned font/environment/layout oracle, never part of a consumer product.
import SwiftUI
import UIKit
import CoreText

@MainActor @Observable final class FontFixtureState {
    var phase = 0
    var frame = CGRect.zero
    var pointSize: CGFloat = 0
    var name = ""
    var previousContext: Font.Context?
    var contextChanges = 0
    func record(_ context: Font.Context) {
        if let previousContext, previousContext != context { contextChanges += 1 }
        previousContext = context
        if #available(tvOS 26, iOS 26, *) {
            let resolved = Font.largeTitle.bold().resolve(in: context)
            pointSize = resolved.pointSize
            name = CTFontCopyPostScriptName(resolved.ctFont) as String
        }
    }
    func save() {
        let data: [String: Any] = ["phase":phase,"frame":[frame.minX,frame.minY,frame.width,frame.height],
            "point_size":pointSize,"postscript_name":name,"context_changes":contextChanges]
        if let bytes = try? JSONSerialization.data(withJSONObject:data,options:.sortedKeys) {
            try? bytes.write(to:FileManager.default.temporaryDirectory.appendingPathComponent("font-oracle.json"),options:.atomic)
        }
    }
}

private struct FixtureTitle: View {
    let state: FontFixtureState
    @Environment(\.fontResolutionContext) private var context
    var body: some View {
        Text("Font fixture canary 573")
            .font(.largeTitle.bold())
            .background {
                if state.phase != 2 {
                    BlueprintFontResolutionProbe(font:.largeTitle.bold(),role:"fixture.stable-title")
                }
            }
            .onGeometryChange(for:CGRect.self) { $0.frame(in:.global) } action: { state.frame = $0 }
            .frame(maxWidth:.infinity)
            .padding(.vertical,14)
            .accessibilityAddTraits(.isHeader)
            .onChange(of:context,initial:true) { _, new in state.record(new) }
    }
}

@main struct SwiftUIFontResolutionFixture: App {
    @State private var state = FontFixtureState()
    private let session: BlueprintRuntimeSession?
    init() { session = try? BlueprintRuntimeSession.startIfRequested() }
    var body: some Scene {
        WindowGroup {
            Color.black
                .navigationTitle("")
                .safeAreaInset(edge:.top,spacing:0) { FixtureTitle(state:state) }
                .dynamicTypeSize(state.phase == 1 ? .accessibility3 : .large)
                .task {
                    let root=FileManager.default.temporaryDirectory
                    for _ in 0..<6000 {
                        if let text=try? String(contentsOf:root.appendingPathComponent("phase"),encoding:.utf8),
                           let phase=Int(text.trimmingCharacters(in:.whitespacesAndNewlines)),(0...3).contains(phase) {
                            state.phase=phase
                        }
                        let trigger=root.appendingPathComponent("oracle.request")
                        if FileManager.default.fileExists(atPath:trigger.path) {
                            try? FileManager.default.removeItem(at:trigger);state.save()
                        }
                        try? await Task.sleep(for:.milliseconds(100))
                    }
                    session?.stop()
                }
        }
    }
}
