// Tool-owned synthetic qualification; never compile this into a product donor.
// The oracle uses public SwiftUI geometry and is independent of the cache reader.
import Foundation
import SwiftUI
import AppKit

// Qualification of the donor's existing, actionless NSViewRepresentable anchor.
// Pure SwiftUI cannot provide this concrete NSView/weak-layout association.
// All visible content remains SwiftUI; this view draws nothing and accepts no hit.
final class ActorCursorFixtureView: NSView {
    override func hitTest(_ point: NSPoint) -> NSView? { nil }
}

private struct ActorCursorFixtureArea: NSViewRepresentable {
    func makeNSView(context: Context) -> ActorCursorFixtureView {
        let view = ActorCursorFixtureView()
        view.setAccessibilityElement(false)
        return view
    }
    func updateNSView(_ view: ActorCursorFixtureView, context: Context) {}
}

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
    @ViewBuilder
    func actorCursorFixture() -> some View {
        if ProcessInfo.processInfo.environment["UIB_ACTOR_CURSOR"] == "1" {
            overlay { ActorCursorFixtureArea().accessibilityHidden(true) }
        } else { self }
    }

    func actorOracle(_ name: String) -> some View {
        #if ACTOR_NO_ORACLE
        // Paired control: compile out the observer itself, not only its IO.
        self
        #else
        onGeometryChange(for: [CGRect].self) { proxy in
            [proxy.frame(in: .named("actor-fixture-root")), proxy.frame(in: .global)]
        } action: { frames in
            ActorOracle.record(name, frames[0])
            ActorOracle.record(name + "-global", frames[1])
        }
        #endif
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
    var body: some View {
        TextField("Search", text: $query)
            .accessibilityIdentifier("fixture.actor.query")
            .actorOracle("query")
    }
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
        Group {
            pickerContent
            if ProcessInfo.processInfo.environment["UIB_ACTOR_SPLIT_OWNER"] == "1" {
                // Same body owner, separate output branch: an ancestry-only
                // selector must not silently report the first branch as complete.
                Color.purple.frame(width: 31, height: 13).actorOracle("split-owner")
            }
        }
    }

    private var pickerContent: some View {
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
    @State private var unrelatedQuery = ""

    var body: some View {
        VStack(spacing: 17) {
            Color.orange.frame(width: 37, height: 19).actorOracle("outside")
            if ProcessInfo.processInfo.environment["UIB_ACTOR_NEIGHBOR_FIELD"] == "1" {
                TextField("Unrelated field", text: $unrelatedQuery)
                    .accessibilityIdentifier("fixture.outside.query")
            }
            if ProcessInfo.processInfo.environment["UIB_ACTOR_SOURCE_SHAPE"] == "1" {
                SourceActorPicker()
            } else {
                ActorPicker()
            }
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

// Structural control for immutable e19b20c47's ActorPickerView: only the search
// has its own View type; the header, toggle and repeated Button/Text rows do not.
// Synthetic strings and actions intentionally avoid all application/API state.
struct SourceActorSearch: View {
    @Binding var query: String
    @FocusState private var queryIsFocused: Bool
    var body: some View {
        HStack(spacing: 8) {
            Image(systemName: "magnifyingglass")
            TextField("Search", text: $query).textFieldStyle(.plain)
                .focused($queryIsFocused)
                .defaultFocus($queryIsFocused, true)
                .accessibilityIdentifier("fixture.source.query")
        }
        .font(.body)
        .padding(.horizontal, 12)
        .frame(minHeight: 36)
        .actorOracle("query")
    }
}

struct SourceActorPicker: View {
    @State private var query = ""
    @State private var possible = false

    var body: some View {
        VStack(spacing: 0) {
            HStack(spacing: 12) {
                Text("Synthetic actors").font(.largeTitle.bold()).actorOracle("header")
                Spacer()
                Button {} label: { Label("Close", systemImage: "xmark") }
                    .actorOracle("close")
            }
            .padding(.horizontal, 24)
            .padding(.vertical, 16)
            VStack(spacing: 12) {
                SourceActorSearch(query: $query)
                Button { possible.toggle() } label: {
                    Label("Include possible matches", systemImage: possible ? "checkmark.square" : "square")
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .padding(8)
                        .contentShape(Rectangle())
                }
                .actorCursorFixture()
                .accessibilityIdentifier("fixture.source.possible")
                .actorOracle("toggle")
                ScrollView {
                    LazyVStack(spacing: 0) {
                        ForEach(0..<20, id: \.self) { number in
                            Button {} label: {
                                Text("Synthetic option \(number)").actorOracle("label-\(number)")
                                    .frame(maxWidth: .infinity, alignment: .leading)
                                    .font(.body)
                                    .padding(.horizontal, 8)
                                    .padding(.vertical, 8)
                                    .frame(minHeight: 36)
                                    .contentShape(Rectangle())
                            }
                            .buttonStyle(.plain)
                            .actorCursorFixture()
                            .accessibilityIdentifier("fixture.source.row.\(number)")
                            .actorOracle("row-\(number)")
                        }
                    }
                }
                .frame(minHeight: 120, idealHeight: 360, maxHeight: 360)
                .actorOracle("scroll")
            }
            .padding(16)
            .padding(.horizontal, 8)
            .padding(.bottom, 8)
        }
        .frame(width: 440)
        .actorOracle("picker")
    }
}

@main
struct ActorScopeFixture: App {
    var body: some Scene {
        Window("UI Blueprint synthetic Actors", id: "actor-scope") { ActorFixtureContent() }
            .windowResizability(.contentSize)
    }
}
