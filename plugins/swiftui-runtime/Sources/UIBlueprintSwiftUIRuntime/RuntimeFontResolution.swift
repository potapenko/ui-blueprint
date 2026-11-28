#if DEBUG && (os(iOS) || os(tvOS)) && targetEnvironment(simulator)
import SwiftUI
import UIKit
import CoreText

/// Add only at an explicitly attributed source font site in a diagnostic build.
/// This probe has no visible content and is not a generic Text-style reader.
@MainActor public struct BlueprintFontResolutionProbe: View {
    private let font: Font
    private let role: String
    public init(font: Font, role: String) { self.font = font; self.role = role }
    public var body: some View {
        if RuntimeFontResolution.enabled {
            if #available(iOS 26, tvOS 26, *) {
                RuntimeFontResolutionAnchor(font: font, role: role)
                    .allowsHitTesting(false).accessibilityHidden(true)
            }
        }
    }
}

@MainActor enum RuntimeFontResolution {
    static let enabled = ProcessInfo.processInfo.environment["UIBLUEPRINT_RUNTIME"] == "1"
        && ProcessInfo.processInfo.environment["UIBLUEPRINT_FONT_RESOLUTION"] == "1"
    @available(iOS 26, tvOS 26, *)
    static func resolve(_ font: Font, context: Font.Context, role: String,
                        mount: String, revision: String) -> [String: RuntimeStyleValue] {
        let value = font.resolve(in: context)
        return ["schema": .number(1), "role": RuntimeLabelStyle.metadata(role),
                "mount_id": .text(mount), "context_revision": .text(revision),
                "context_basis": .text("latest-SwiftUI-environment-update-at-probe-mount"),
                "basis": .text("Font.resolve-in-mounted-fontResolutionContext"),
                "point_size": RuntimeLabelStyle.number(value.pointSize), "units": .text("pt"),
                "postscript_name": RuntimeLabelStyle.metadata(CTFontCopyPostScriptName(value.ctFont) as String),
                "family_name": RuntimeLabelStyle.metadata(CTFontCopyFamilyName(value.ctFont) as String),
                "symbolic_traits": .number(Double(CTFontGetSymbolicTraits(value.ctFont).rawValue)),
                "is_bold": .number(value.isBold ? 1 : 0), "is_italic": .number(value.isItalic ? 1 : 0),
                "is_monospaced": .number(value.isMonospaced ? 1 : 0),
                "effective_paint_font": .unknown("glyph-fallback-fitting-and-attributed-runs-uninspected")]
    }
}

@MainActor final class RuntimeFontResolutionView: UIView {
    let mount = UUID().uuidString
    var sample: (() -> [String: RuntimeStyleValue])?
    override var intrinsicContentSize: CGSize { .zero }
    override var canBecomeFocused: Bool { false }
}

@available(iOS 26, tvOS 26, *)
private struct RuntimeFontResolutionAnchor: UIViewRepresentable {
    let font: Font
    let role: String
    @Environment(\.fontResolutionContext) private var fontContext
    func makeUIView(context: Context) -> RuntimeFontResolutionView {
        let view = RuntimeFontResolutionView(frame: .zero)
        view.isUserInteractionEnabled = false
        view.isAccessibilityElement = false
        view.accessibilityElementsHidden = true
        view.backgroundColor = .clear
        return view
    }
    func updateUIView(_ view: RuntimeFontResolutionView, context: Context) {
        let font = font, fontContext = fontContext, role = role, mount = view.mount
        let revision = UUID().uuidString
        view.sample = { RuntimeFontResolution.resolve(font, context: fontContext, role: role,
                                                     mount: mount, revision: revision) }
    }
    func sizeThatFits(_ proposal: ProposedViewSize, uiView: RuntimeFontResolutionView,
                     context: Context) -> CGSize? { .zero }
    static func dismantleUIView(_ uiView: RuntimeFontResolutionView, coordinator: ()) { uiView.sample = nil }
}
#endif
