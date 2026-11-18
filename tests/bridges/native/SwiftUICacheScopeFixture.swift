// Tool-owned qualification only; never added to a product donor.
// Three source cards share their nominal type. Only the playing card mounts video.
import Foundation
import SwiftUI

struct ScopeCard: View {
    let playing: Bool
    let number: Int
    private let privateModel = ["token": "UIB_PRIVATE_MODEL_CANARY_86c2b1"]

    var body: some View {
        VStack(spacing: 12) {
            HStack(spacing: 8) {
                if playing { Color.blue.frame(width: 80) }
                VStack(spacing: 10) {
                    Text("Title").frame(width: CGFloat(40 + number))
                    if playing { ScopeVideo().frame(height: 90) }
                }
                .frame(maxWidth: .infinity)
            }
            Text("Actions").frame(width: CGFloat(90 + number))
        }
        .padding(16)
        .frame(width: 640, height: 220)

    }
}

struct ScopeVideo: View { var body: some View { Color.orange } }

@main
struct CacheScopeFixture: App {
    var body: some Scene {
        Window("UI Blueprint subtree fixture", id: "cache-transition") {
            VStack(spacing: 0) {
                HStack(spacing: 0) {
                    ForEach(0..<55, id: \.self) { _ in
                        Color.gray.frame(width: 1, height: 1)
                    }
                }
                .frame(height: 1)
                ScopeCard(playing: false, number: 1)
                ScopeCard(playing: true, number: 2)
                ScopeCard(playing: ProcessInfo.processInfo.environment["UIB_SECOND_VIDEO"] == "1", number: 3)
            }
        }
        .windowResizability(.contentSize)
    }
}
