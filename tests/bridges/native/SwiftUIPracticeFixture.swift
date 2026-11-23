// S15 synthetic geometry oracle, based on immutable native e19b20c47.
// Keeps the Practice layout operators; no history, clipboard, network or app SDK.
// Native cursor attachments reproduce source geometry witnesses only; they do
// not own visible content, actions or layout. Modal keyboard routing/localization
// are omitted. This fixture cannot qualify those parts or the application's scope.
import AppKit
import SwiftUI

// The donor already uses an actionless NSView overlay for pointing-hand cursor
// delivery. A pure SwiftUI replacement would remove the native layout witness
// being qualified. This test adapter intentionally has no cursor/input behavior.
final class S15CursorView: NSView {
    override func hitTest(_ point: NSPoint) -> NSView? { nil }
}

struct S15CursorArea: NSViewRepresentable {
    func makeNSView(context: Context) -> S15CursorView {
        let view = S15CursorView()
        view.setAccessibilityElement(false)
        return view
    }
    func updateNSView(_ view: S15CursorView, context: Context) {}
}

private struct S15CursorWitness: ViewModifier {
    @Environment(\.isEnabled) private var isEnabled
    func body(content: Content) -> some View {
        content.overlay {
            if isEnabled { S15CursorArea().accessibilityHidden(true) }
        }
    }
}

private struct PracticeBounds: PreferenceKey {
    static let defaultValue: [String: [Anchor<CGRect>]] = [:]

    static func reduce(value: inout [String: [Anchor<CGRect>]], nextValue: () -> [String: [Anchor<CGRect>]]) {
        value.merge(nextValue(), uniquingKeysWith: +)
    }
}

private extension View {
    func oracle(_ role: String) -> some View {
        #if S15_NO_ORACLE
        self
        #else
        transformAnchorPreference(key: PracticeBounds.self, value: .bounds) { values, anchor in
            values[role, default: []].append(anchor)
        }
        #endif
    }

    func cursorWitness() -> some View {
        modifier(S15CursorWitness())
    }
}

struct PracticeEntry: Identifiable {
    let id: Int
    let text: String
}

struct MacPracticeMaterial: View {
    let entries: [PracticeEntry]
    @Environment(\.colorScheme) private var colorScheme
    @Environment(\.dynamicTypeSize) private var textSize

    var body: some View {
        LazyVGrid(columns: [GridItem(.adaptive(minimum: textSize.isAccessibilitySize ? 260 : 160), alignment: .leading)],
                  alignment: .leading, spacing: 8) {
            ForEach(entries) { entry in
                Button {} label: {
                    Text(verbatim: entry.text)
                        .font(.callout.weight(.semibold))
                        .foregroundStyle(colorScheme == .dark ? Color.yellow : Color(red: 0.45, green: 0.32, blue: 0))
                        .multilineTextAlignment(.leading).lineLimit(1)
                        .fixedSize(horizontal: false, vertical: true)
                        .padding(.horizontal, 10).padding(.vertical, 3)
                        .background(Color.yellow.opacity(0.14), in: RoundedRectangle(cornerRadius: 18))
                        .contentShape(Rectangle())
                        .oracle("pill-label-\(entry.id)")
                }
                .buttonStyle(.plain)
                .cursorWitness()
                .oracle("pill-button-\(entry.id)")
            }
        }
        .oracle("material")
    }
}

@available(macOS 26, *)
private struct PracticeSurface: ViewModifier {
    func body(content: Content) -> some View {
        let shape = RoundedRectangle(cornerRadius: 20, style: .continuous)
        GlassEffectContainer(spacing: 12) { content.buttonStyle(.glass) }
            .controlSize(.large)
            .background { shape.fill(.clear).glassEffect(.regular.interactive(), in: shape) }
    }
}

@available(macOS 26, *)
struct MacPracticeRoot: View {
    let empty: Bool
    @State private var expanded: Bool

    init(empty: Bool, expanded: Bool) {
        self.empty = empty
        // Intentional launch-only fixture seed; no changing parent input is mirrored.
        _expanded = State(initialValue: expanded)
    }

    private var entries: [PracticeEntry] {
        empty ? [] : [PracticeEntry(id: 1, text: "Synthetic phrase one."),
                      PracticeEntry(id: 2, text: "Synthetic phrase two.")]
    }

    var body: some View {
        VStack(spacing: 0) {
            HStack(alignment: .center, spacing: 12) {
                ViewThatFits(in: .horizontal) {
                    HStack(alignment: .firstTextBaseline) { title; guide }
                    VStack(alignment: .leading) { title; guide }
                }
                Spacer(minLength: 0)
                Button("Close", systemImage: "xmark") {}
                    .buttonStyle(.glass).labelStyle(.iconOnly)
                    .font(.body.weight(.semibold)).buttonBorderShape(.circle).controlSize(.large)
                    .cursorWitness()
                    .oracle("close")
            }
            .padding(.horizontal, 24).padding(.vertical, 16)
            .oracle("header")
            ScrollView {
                VStack(alignment: .leading, spacing: 16) {
                    if !empty { Text("Synthetic next-step help").font(.footnote).foregroundStyle(.secondary) }
                    selectors
                    Text(empty ? "0 / 0" : "2 / 2")
                        .font(.title2.bold()).accessibilityAddTraits(.updatesFrequently)
                        .oracle("count")
                    Text("Synthetic selection help").font(.footnote).foregroundStyle(.secondary)
                    if empty { Text("Synthetic empty lesson").foregroundStyle(.secondary).oracle("empty") }
                    MacPracticeMaterial(entries: entries)
                    if !empty {
                        DisclosureGroup("Full prompt", isExpanded: $expanded) {
                            Text(verbatim: String(repeating: "Synthetic prompt line.\n", count: 12))
                                .font(.body).textSelection(.enabled)
                                .frame(maxWidth: .infinity, alignment: .leading).padding(.top, 8)
                                .oracle("prompt")
                        }
                        .oracle("disclosure")
                    }
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Synthetic language help")
                        Button("Settings") {}.buttonStyle(.plain).foregroundStyle(.tint)
                    }.font(.footnote).foregroundStyle(.secondary)
                    footer
                }
                .padding(16).frame(maxWidth: .infinity, alignment: .leading)
                .oracle("body-content")
            }
            .modifier(PracticeSurface()).padding(.horizontal, 8).padding(.bottom, 8)
            .oracle("scroll-surface")
        }
        .frame(width: 560).frame(maxHeight: 740)
        .fixedSize(horizontal: false, vertical: true)
        .buttonStyle(.glass).controlSize(.large)
        .background {
            RoundedRectangle(cornerRadius: 28, style: .continuous)
                .fill(.clear).glassEffect(.regular.interactive(), in: .rect(cornerRadius: 28)).opacity(0.72)
        }
        .oracle("popup")
    }

    private var title: some View { Text("Practice with ChatGPT").font(.largeTitle.bold()).oracle("title") }
    private var guide: some View {
        // No external navigation in a synthetic measurement fixture.
        Text("(Learn more)").font(.footnote).foregroundStyle(.tint).oracle("guide")
    }
    private var selectors: some View {
        HStack {
            Menu { Button("2026-11-23") {} } label: { Text(verbatim: "2026-11-23") }
                .menuStyle(.button).accessibilityLabel("Study date").oracle("date-selector")
            Spacer(minLength: 4)
            Menu { Button("Lesson 2") {} } label: { Text("Lesson 2") }
                .menuStyle(.button).oracle("lesson-selector")
        }.cursorWitness().oracle("selectors")
    }
    private var footer: some View {
        ViewThatFits(in: .horizontal) {
            HStack(spacing: 12) { newLesson; copyButton }
            VStack(spacing: 12) { newLesson; copyButton }
        }.padding(.top, 8).oracle("footer")
    }
    private var newLesson: some View {
        Button {} label: {
            Text("Start new lesson").font(.body.weight(.semibold))
                .frame(maxWidth: .infinity).padding(.vertical, 6)
        }.cursorWitness().disabled(empty).oracle("new-lesson")
    }
    private var copyButton: some View {
        Button {} label: {
            Label("Copy practice prompt", systemImage: "checkmark")
                .font(.body.weight(.semibold)).frame(maxWidth: .infinity).padding(.vertical, 6)
        }.cursorWitness().disabled(empty).oracle("copy")
    }
}

@available(macOS 26, *)
private struct PracticeFixtureRoot: View {
    let directory: URL
    let state: String

    var body: some View {
        ZStack {
            Color.black.opacity(0.18)
            MacPracticeRoot(empty: state == "empty", expanded: state == "expanded")
        }
        .frame(width: 900, height: 820)
        .oracle("fixture-root")
        #if !S15_NO_ORACLE
        .overlayPreferenceValue(PracticeBounds.self) { anchors in
            GeometryReader { root in
                let frames = anchors.mapValues { occurrences in occurrences.map { root[$0] } }
                Color.clear.onChange(of: frames, initial: true) { _, current in
                    // Keep every occurrence: ViewThatFits may materialize alternatives.
                    // Do not select by matching coordinates or overwrite duplicates.
                    let rectangles = current.mapValues { $0.map { [$0.minX, $0.minY, $0.width, $0.height] } }
                    do {
                        try JSONEncoder().encode(rectangles).write(
                            to: directory.appendingPathComponent("oracle-\(state).json"), options: .atomic)
                    } catch { preconditionFailure("Unable to save owned fixture oracle") }
                }
            }
        }
        #endif
    }
}

@available(macOS 26, *)
@main
struct SwiftUIPracticeFixture: App {
    private let directory: URL
    private let state: String

    init() {
        let arguments = CommandLine.arguments
        guard arguments.count == 3, ["collapsed", "expanded", "empty"].contains(arguments[2]) else {
            fatalError("Expected owned runtime directory and collapsed/expanded/empty")
        }
        directory = URL(fileURLWithPath: arguments[1], isDirectory: true)
        state = arguments[2]
    }

    var body: some Scene {
        Window("S15 Practice geometry fixture", id: "practice-fixture") {
            PracticeFixtureRoot(directory: directory, state: state)
        }.windowResizability(.contentSize)
    }
}
