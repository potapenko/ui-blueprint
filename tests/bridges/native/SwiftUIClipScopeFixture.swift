// Synthetic source-owned window for the empty, expanded, focused Clip case.
import AppKit
import SwiftUI

private final class WindowProbeView: NSView {}
private struct SelectionAppearanceAdapter: NSViewRepresentable {
    func makeNSView(context: Context) -> NSView { WindowProbeView() }
    func updateNSView(_ view: NSView, context: Context) {}
}

private extension View {
    func recordSize(_ region: String) -> some View {
        onGeometryChange(for: CGSize.self, of: { $0.size }) { size in
            print("size", region, size.width, size.height)
            fflush(stdout)
        }
    }
}

struct PhraseQueryInput: View {
    @State private var text = ""
    @State private var mounted = false
    @FocusState private var focused: Bool
    var body: some View {
        TextField("Query", text: $text)
            .focused($focused)
            .background {
                if mounted || focused {
                    SelectionAppearanceAdapter().frame(width: 0, height: 0)
                }
            }
            .onAppear { focused = true }
            .onChange(of: focused) { _, value in
                if value { mounted = true }
                print("focused", value)
                fflush(stdout)
            }
    }
}

struct ClipSearchQueryView: View {
    var body: some View { PhraseQueryInput().frame(height: 40).recordSize("query") }
}

struct ClipSearchSuggestionsView: View {
    var body: some View { Color.gray.frame(height: 56).recordSize("suggestions") }
}

struct ClipSearchGridView: View {
    var body: some View { Color.clear.frame(height: 160).recordSize("grid") }
}

struct ClipSearchView: View {
    var body: some View {
        HStack(spacing: 0) {
            Color.gray.frame(width: 100)
            VStack(spacing: 8) {
                ClipSearchQueryView()
                ClipSearchSuggestionsView()
                ClipSearchGridView()
            }
            .frame(width: 300)
        }
        .frame(width: 400, height: 272)
    }
}

@main
struct ClipScopeFixture: App {
    var body: some Scene {
        Window("UI Blueprint Clip fixture", id: "clip-fixture") { ClipSearchView() }
            .windowResizability(.contentSize)
    }
}
