// Tool-owned qualification only; never added to a product donor.
// A private control file drives ordinary SwiftUI state. No capture/debug getter.
import Foundation
import SwiftUI

struct CacheTransitionContent: View {
    @State private var phase = 0
    private let privateModel = ["token": "UIB_PRIVATE_MODEL_CANARY_86c2b1"]

    var body: some View {
        VStack(spacing: 12) {
            HStack(spacing: 8) {
                if phase == 0 { Color.blue.frame(width: 80) }
                VStack(spacing: 10) {
                    Text("Layout fixture")
                    Color.orange.frame(height: 90)
                }
                .frame(maxWidth: .infinity)
            }
            Text("Phase \(phase)")
        }
        .padding(16)
        .frame(width: phase == 2 ? 800 : 640, height: 400)
        .task {
            guard let directory = ProcessInfo.processInfo.environment["UIB_FIXTURE_CONTROL"] else { return }
            let root = URL(fileURLWithPath: directory)
            while !Task.isCancelled {
                if let text = try? String(contentsOf: root.appendingPathComponent("phase"), encoding: .utf8),
                   let requested = Int(text.trimmingCharacters(in: .whitespacesAndNewlines)),
                   (0...2).contains(requested) {
                    phase = requested
                    let acknowledgement = "\(ProcessInfo.processInfo.processIdentifier):\(phase)"
                    try? acknowledgement.write(to: root.appendingPathComponent("ack"), atomically: true, encoding: .utf8)
                }
                try? await Task.sleep(for: .milliseconds(200))
            }
        }
    }
}

@main
struct CacheTransitionFixture: App {
    var body: some Scene {
        Window("UI Blueprint cache transition", id: "cache-transition") {
            CacheTransitionContent()
        }
        .windowResizability(.contentSize)
    }
}
