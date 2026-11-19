// Synthetic SDK qualification. Pass an existing empty output directory, optionally followed by --panel-only.
import AppKit
import SwiftUI
import UIBlueprintSwiftUI

private struct ReferenceBounds: PreferenceKey {
    static let defaultValue: [String: Anchor<CGRect>] = [:]

    static func reduce(value: inout [String: Anchor<CGRect>], nextValue: () -> [String: Anchor<CGRect>]) {
        value.merge(nextValue(), uniquingKeysWith: { _, _ in preconditionFailure("Duplicate reference") })
    }
}

private extension View {
    @ViewBuilder
    func measured(_ id: String, parent: String, markers: Bool) -> some View {
        let reference = transformAnchorPreference(key: ReferenceBounds.self, value: .bounds) {
            $0[id] = $1
        }
        if markers {
            reference.blueprintMarker(id, parent: parent)
        } else {
            reference
        }
    }
}

private struct GrammarSearchHint: View {
    let id: String
    let width: CGFloat
    let markers: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text("Hint")
                .measured(id + "-title", parent: id + "-stack", markers: markers)
            Text("Example phrase for the grammar form")
                .fixedSize(horizontal: false, vertical: true)
                .measured(id + "-example", parent: id + "-stack", markers: markers)
        }
        .measured(id + "-stack", parent: id, markers: markers)
        .padding(8)
        .frame(width: width)
        .measured(id, parent: "grammar-panel", markers: markers)
    }
}

@MainActor
private final class Driver: ObservableObject {
    @Published var width: CGFloat = 110
    var reference: [String: CGRect] = [:]
}

private struct Form: View {
    @ObservedObject var driver: Driver
    let markers: Bool

    var body: some View {
        HStack(alignment: .top, spacing: 19) {
            GrammarSearchHint(id: "hint", width: driver.width, markers: markers)
            GrammarSearchHint(id: "neighbor", width: 145, markers: markers)
        }
        .padding(13)
        .measured("grammar-panel", parent: "root", markers: true)
        .frame(width: 424, height: 220, alignment: .topLeading)
        .overlayPreferenceValue(ReferenceBounds.self) { anchors in
            GeometryReader { proxy in
                let frames = anchors.mapValues { proxy[$0] }
                Color.clear.onChange(of: frames, initial: true) { _, result in driver.reference = result }
            }
        }
    }
}

private enum FixtureError: Error {
    case referenceNotReady, layoutChangedDuringCapture
}

@main
private struct Runner {
    @MainActor static func main() throws {
        let arguments = Array(CommandLine.arguments.dropFirst())
        guard arguments.count == 1 || (arguments.count == 2 && arguments[1] == "--panel-only") else { exit(2) }
        let output = URL(fileURLWithPath: arguments[0], isDirectory: true)
        let markers = arguments.count == 1
        let session = try BlueprintProbeSession(
            targetID: "grammar-fixture", targetGeneration: UUID().uuidString, surfaceID: "fixture",
            scopeID: "form", rootMarkerID: "root", requiredMarkerIDs: ["grammar-panel"], selection: .mountedSubtree
        )
        let app = NSApplication.shared
        app.setActivationPolicy(.accessory)
        let driver = Driver()
        let window = NSWindow(contentRect: CGRect(x: 0, y: 0, width: 424, height: 220),
                              styleMask: [.titled], backing: .buffered, defer: false)
        window.contentView = NSHostingView(rootView: Form(driver: driver, markers: markers).blueprintProbeRoot(session: session))
        window.orderFront(nil)
        Task { @MainActor in
            do {
                for (phase, width) in [("narrow", CGFloat(110)), ("wide", CGFloat(192))] {
                    driver.width = width
                    let deadline = ContinuousClock.now.advanced(by: .seconds(5))
                    while !session.isReady || driver.reference.count != 9 || driver.reference["hint"]?.width != width {
                        guard ContinuousClock.now < deadline else { throw FixtureError.referenceNotReady }
                        try await Task.sleep(for: .milliseconds(25))
                    }
                    let oracle = driver.reference
                    let snapshot = try await session.capture(requestID: phase, stateID: phase)
                    guard driver.reference == oracle, session.isCurrentMounts(snapshot) else {
                        throw FixtureError.layoutChangedDuringCapture
                    }
                    let rects = oracle.mapValues { [$0.minX, $0.minY, $0.width, $0.height] }
                    try JSONEncoder().encode(rects).write(to: output.appendingPathComponent(phase + "-oracle.json"), options: .withoutOverwriting)
                    try snapshot.data.write(to: output.appendingPathComponent(phase + "-snapshot.json"), options: .withoutOverwriting)
                }
                app.terminate(nil)
            } catch {
                FileHandle.standardError.write(Data("Fixture failed: \(error)\n".utf8))
                exit(1)
            }
        }
        app.run()
    }
}
