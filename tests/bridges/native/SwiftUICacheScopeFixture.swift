// Tool-owned qualification only; never added to a product donor.
// A private control file drives ordinary SwiftUI state. No capture/debug getter.
import Foundation
import SwiftUI

struct ScopeCard: View {
    @State private var phase = 0
    private let privateModel = ["token": "UIB_PRIVATE_MODEL_CANARY_86c2b1"]

    var body: some View {
        VStack(spacing: 12) {
            HStack(spacing: 8) {
                if phase == 0 { Color.blue.frame(width: 80) }
                VStack(spacing: 10) {
                    Text("Layout fixture")
                    ScopeVideo().frame(height: 90)
                }
                .frame(maxWidth: .infinity)
            }
            Text("Phase \(phase)")
        }
        .padding(16)
        .frame(width: phase == 2 ? 800 : 640, height: 400)

    }
}

struct ScopeVideo: View { var body: some View { Color.orange } }

@main
struct CacheScopeFixture: App {
    var body: some Scene {
        Window("UI Blueprint subtree fixture", id: "cache-transition") {
            VStack(spacing: 0) {
                HStack(spacing: 0) {
                    ForEach(0..<80, id: \.self) { _ in
                        Color.gray.frame(width: 1, height: 1)
                    }
                }
                .frame(height: 1)
                ScopeCard()
            }
        }
        .windowResizability(.contentSize)
    }
}
