// Synthetic public-API oracle; no product UI changes or text reads by the collector.
import SwiftUI
import UIKit

@MainActor enum LabelStyleOracle {
    static func run() {
        guard let window = UIApplication.shared.connectedScenes.compactMap({ $0 as? UIWindowScene })
            .flatMap(\.windows).first(where: \.isKeyWindow) else { return }
        let traits = UITraitCollection(userInterfaceStyle: .dark)
        let normal = UILabel(frame: CGRect(x: 200,y: 300,width: 600,height: 80))
        normal.text = "UIB_STYLE_PRIVATE_CANARY_913"
        normal.font = .systemFont(ofSize: 57, weight: .semibold)
        normal.textColor = UIColor { $0.userInterfaceStyle == .dark ? .white : .black }
        normal.overrideUserInterfaceStyle = .dark
        window.addSubview(normal)
        let fitted = UILabel(frame:CGRect(x:200,y:420,width:600,height:80))
        fitted.attributedText = NSAttributedString(string:"UIB_ATTRIBUTED_PRIVATE_731",attributes:[.font:UIFont.italicSystemFont(ofSize:31),.foregroundColor:UIColor.red])
        fitted.font = .systemFont(ofSize:43)
        fitted.adjustsFontSizeToFitWidth = true; fitted.minimumScaleFactor = 0.4
        fitted.highlightedTextColor = .yellow; fitted.isHighlighted = true
        window.addSubview(fitted)
        let pattern = UILabel(frame:CGRect(x:200,y:530,width:600,height:80))
        pattern.font = .monospacedSystemFont(ofSize:29,weight:.regular)
        let image = UIGraphicsImageRenderer(size:CGSize(width:2,height:2)).image { context in
            UIColor.green.setFill(); context.fill(CGRect(x:0,y:0,width:2,height:2))
        }
        pattern.textColor = UIColor(patternImage:image);window.addSubview(pattern)
        let testValues: [String: Bool] = [
            "control_font_name_unknown": isUnknown(RuntimeLabelStyle.metadata("bad\nname")),
            "path_font_name_unknown": isUnknown(RuntimeLabelStyle.metadata("/private/font")),
            "oversized_font_name_unknown": isUnknown(RuntimeLabelStyle.metadata(String(repeating:"a",count:257))),
            "nonfinite_unknown": isUnknown(RuntimeLabelStyle.number(.infinity)),
            "pattern_unknown": isUnknown(RuntimeLabelStyle.resolved(pattern.textColor,traits:traits)["space"]!),
            "nil_unknown": isUnknown(RuntimeLabelStyle.resolved(nil,traits:traits)["space"]!),
            "dynamic_light_black": knownNumber(RuntimeLabelStyle.resolved(normal.textColor,traits:UITraitCollection(userInterfaceStyle:.light))["r"]!) == 0,
            "dynamic_dark_white": abs((knownNumber(RuntimeLabelStyle.resolved(normal.textColor,traits:traits)["r"]!) ?? -1) - 1) < 1e-6
        ]
        let expected: [[String:Any]] = [normal,fitted,pattern].map { label in
            ["origin":[label.bounds.minX,label.bounds.minY],"size":[label.bounds.width,label.bounds.height],
             "font_name":label.font.fontName,"family":label.font.familyName,"size_pt":label.font.pointSize,
             "traits":label.font.fontDescriptor.symbolicTraits.rawValue, "minimum_scale_factor":label.minimumScaleFactor]
        }
        let payload: [String:Any] = ["labels":expected,"checks":testValues]
        if let data=try? JSONSerialization.data(withJSONObject:payload,options:.sortedKeys) {
            try? data.write(to:FileManager.default.temporaryDirectory.appendingPathComponent("label-oracle.json"),options:.atomic)
        }
    }
    static func knownNumber(_ value:RuntimeStyleValue)->Double? { if case .number(let n)=value {return n};return nil }
    static func isUnknown(_ value:RuntimeStyleValue)->Bool { if case .unknown = value {return true};return false }
}
@main struct UIKitLabelStyleFixture: App {
    private let session: BlueprintRuntimeSession?
    init() { session=try? BlueprintRuntimeSession.startIfRequested() }
    var body: some Scene {
        WindowGroup {
            Color.black.ignoresSafeArea().task {
                try? await Task.sleep(for:.seconds(2))
                LabelStyleOracle.run()
            }
        }
    }
}
