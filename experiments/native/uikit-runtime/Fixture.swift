import SwiftUI
import UIKit

@main struct UIKitRuntimeFixture: App {
    @State private var selection = 0
    private let session: BlueprintRuntimeSession?
    init() { session = try? BlueprintRuntimeSession.startIfRequested() }
    var body: some Scene {
        WindowGroup {
            VStack(spacing: 24) {
                Text("UIKit geometry fixture").font(.title)
                Menu("Open system menu") {
                    Button("First option") { selection = 1 }
                    Button("Second option") { selection = 2 }
                }
                Picker("Ordering", selection: $selection) {
                    Text("Original").tag(0)
                    Text("Ascending").tag(1)
                    Text("Descending").tag(2)
                }.pickerStyle(.menu)
                Text("Selection: \(selection)")
                Text("Owned diagnostic fixture")
            }.padding().task { try? await runOracles() }
        }
    }
}

@MainActor private func runOracles() async throws {
    try await Task.sleep(for: .milliseconds(300))
    guard let scene = UIApplication.shared.connectedScenes.first as? UIWindowScene else { return }
    let window = UIWindow(windowScene: scene)
    window.bounds = CGRect(x: 0, y: 0, width: 402, height: 874)
    let scroll = UIScrollView(frame: CGRect(x: 20, y: 40, width: 240, height: 300))
    scroll.contentInsetAdjustmentBehavior = .never
    scroll.contentSize = CGSize(width: 800, height: 900)
    scroll.contentOffset = CGPoint(x: 13, y: 29)
    window.addSubview(scroll)
    let child = UILabel(frame: CGRect(x: 50, y: 60, width: 80, height: 30))
    child.bounds.origin = CGPoint(x: 7, y: 11)
    child.text = "UIB_UIKIT_PRIVATE_CANARY_7d9c"
    child.accessibilityValue = "UIB_UIKIT_PRIVATE_CANARY_7d9c"
    scroll.addSubview(child)
    let baseline = RuntimeUIKit.mapping(child, to: window)!
    // Independent arithmetic: parent location + child frame - scroll offset - local origin.
    let baselinePass = zip(baseline.values, [1, 0, 0, 1, 50, 60]).allSatisfy { abs($0-$1) < 1e-8 }
    child.transform = CGAffineTransform(rotationAngle: .pi / 6).scaledBy(x: 1.25, y: 0.75)
    let transformed = RuntimeUIKit.mapping(child, to: window)!
    let a = cos(Double.pi/6)*1.25, b = sin(Double.pi/6)*1.25
    let c = -sin(Double.pi/6)*0.75, d = cos(Double.pi/6)*0.75
    let expected = [a,b,c,d,97-a*47-c*26,86-b*47-d*26]
    let affinePass = zip(transformed.values, expected).allSatisfy { abs($0-$1) < 1e-8 }
    var perspective = CATransform3DIdentity
    perspective.m34 = -1/500
    scroll.layer.sublayerTransform = perspective
    let perspectivePass = RuntimeUIKit.mapping(child, to: window) == nil
    scroll.layer.sublayerTransform = CATransform3DIdentity
    let animation = CABasicAnimation(keyPath: "bounds.size.width")
    animation.fromValue = 20; animation.toValue = 80; animation.duration = 10
    child.layer.add(animation, forKey: "fixture")
    let nodes = try RuntimeUIKit.collect(window)
    let label = nodes.first { $0.type == String(reflecting: type(of: child)) }
    let modelPass = label?.size?.width == 80 && label?.position == CGPoint(x: 7, y: 11)
        && label?.parent.map { nodes[$0].type == String(reflecting: type(of: scroll)) } == true
    let observation = RuntimeObservation(id: "fixture-oracle", start: 1, end: 2,
        window: 99, windowGeneration: "fixture-generation", nodes: nodes, source: .uikit,
        windowMetadata: RuntimePlatform.uikitMetadata(window))
    let raw = try RuntimeEncoding.snapshot(observation, session: "fixture-session", bundle: "me.uiblueprint.uikitfixture",
        request: "fixture-request", sequence: 1)
    let privacyPass = !String(decoding: raw, as: UTF8.self).contains("UIB_UIKIT_PRIVATE_CANARY_7d9c")
    let root = FileManager.default.temporaryDirectory
    try raw.write(to: root.appendingPathComponent("uikit-oracle-snapshot.json"))
    let report: [String: Any] = ["status": baselinePass && affinePass && perspectivePass && modelPass && privacyPass ? "pass" : "fail", "nodes": nodes.count,
        "baseline_pass": baselinePass, "baseline": baseline.values, "affine_pass": affinePass,
        "affine": transformed.values, "expected_affine": expected, "model_pass": modelPass,
        "perspective_unknown": perspectivePass, "animation_model_width": child.bounds.width, "canary_absent": privacyPass,
        "screen_scale": window.screen.scale, "window_width": window.bounds.width]
    try JSONSerialization.data(withJSONObject: report, options: [.sortedKeys]).write(to: root.appendingPathComponent("uikit-oracle.json"))
}
