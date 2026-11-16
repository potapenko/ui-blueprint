// Synthetic fixture only. No probe markers, capture getter or target-side bridge.
import Foundation
import SwiftUI

struct DebugCaptureFixtureContent: View {
    @State private var expanded = true
    private let privateModel = ["token": "UIB_PRIVATE_MODEL_CANARY_86c2b1"]

    var body: some View {
        VStack(spacing: 12) {
            HStack(spacing: 8) {
                if expanded { Color.blue.frame(width: 80) }
                VStack(spacing: 10) {
                    Text("Layout fixture")
                    Color.orange.frame(height: 90)
                }
                .frame(maxWidth: .infinity)
            }
            Button("Toggle rail") { expanded.toggle() }
        }
        .padding(16)
        .frame(minWidth: 320, minHeight: 240)
    }
}

@main
struct UIBlueprintDebugFixture: App {
    var body: some Scene {
        Window("UI Blueprint canary fixture", id: "fixture") {
            DebugCaptureFixtureContent()
        }
        .defaultSize(width: 640, height: 400)
    }
}
