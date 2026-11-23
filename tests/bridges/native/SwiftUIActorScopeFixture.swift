// Tool-owned synthetic qualification; never compile this into a product donor.
// The oracle uses public SwiftUI geometry and is independent of the cache reader.
import Foundation
import SwiftUI

@MainActor
private enum ActorOracle {
    static var frames: [String: [Double]] = [:]

    static func record(_ name: String, _ rect: CGRect) {
        let value = [rect.minX, rect.minY, rect.width, rect.height].map(Double.init)
        guard frames[name] != value,
              let path = ProcessInfo.processInfo.environment["UIB_ACTOR_ORACLE"] else { return }
        frames[name] = value
        let document: [String: Any] = [
            "coordinate_space": "actor-fixture-root", "units": "pt", "origin": "top_left",
            "pid": ProcessInfo.processInfo.processIdentifier, "frames": frames
        ]
        do {
            let bytes = try JSONSerialization.data(withJSONObject: document, options: [.sortedKeys])
            try bytes.write(to: URL(fileURLWithPath: path), options: .atomic)
        } catch {
            // A failed oracle cannot qualify a capture; no path/error content is published.
            fputs("actor_oracle_write_failed\n", stderr)
        }
    }
}

private extension View {
    func actorOracle(_ name: String) -> some View {
        onGeometryChange(for: [CGRect].self) { proxy in
            [proxy.frame(in: .named("actor-fixture-root")), proxy.frame(in: .global)]
        } action: { frames in
            ActorOracle.record(name, frames[0])
            ActorOracle.record(name + "-global", frames[1])
        }
    }
}

struct ActorHeader: View {
    var body: some View { Text("Synthetic actors").font(.headline).actorOracle("header") }
}

struct ActorClose: View {
    var body: some View { Button("Close") {}.actorOracle("close") }
}

struct ActorQuery: View {
    @Binding var query: String
    var body: some View { TextField("Search", text: $query).actorOracle("query") }
}

struct ActorPossible: View {
    @Binding var enabled: Bool
    var body: some View { Toggle("Include possible matches", isOn: $enabled).actorOracle("toggle") }
}

struct ActorSuggestionLabel: View {
    var body: some View { Text("Synthetic option").actorOracle("label") }
}

struct ActorSuggestionRow: View {
    var body: some View {
        HStack { ActorSuggestionLabel(); Spacer() }
            .padding(8)
            .frame(maxWidth: .infinity)
            .actorOracle("row")
    }
}

struct ActorSuggestions: View {
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 4) {
                ActorSuggestionRow()
                Text("Another synthetic option").padding(8)
            }
            .frame(maxWidth: .infinity)
        }
        .frame(height: 160)
        .actorOracle("scroll")
    }
}

struct ActorPicker: View {
    @State private var query = ""
    @State private var possible = false

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack { ActorHeader(); Spacer(); ActorClose() }
            ActorQuery(query: $query)
            ActorPossible(enabled: $possible)
            ActorSuggestions()
        }
        .padding(16)
        .frame(maxWidth: .infinity)
        .actorOracle("picker")
    }
}

struct ActorFixtureContent: View {
    @State private var narrow = false

    var body: some View {
        VStack(spacing: 17) {
            Color.orange.frame(width: 37, height: 19).actorOracle("outside")
            ActorPicker()
            if ProcessInfo.processInfo.environment["UIB_ACTOR_DUPLICATE"] == "1" {
                // Negative case: another source owner with the same native field class.
                ActorPicker()
            }
        }
        .padding(23)
        .frame(width: narrow ? 380 : 520)
        .coordinateSpace(.named("actor-fixture-root"))
        .task {
            guard let path = ProcessInfo.processInfo.environment["UIB_ACTOR_PHASE"] else { return }
            // Test-only control; ordinary SwiftUI layout owns the actual resize.
            while !Task.isCancelled {
                if let value = try? String(contentsOfFile: path, encoding: .utf8),
                   ["wide", "narrow"].contains(value) {
                    narrow = value == "narrow"
                }
                do { try await Task.sleep(for: .milliseconds(200)) }
                catch { return }
            }
        }
    }
}

@main
struct ActorScopeFixture: App {
    var body: some Scene {
        Window("UI Blueprint synthetic Actors", id: "actor-scope") { ActorFixtureContent() }
            .windowResizability(.contentSize)
    }
}
