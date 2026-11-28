#if DEBUG
import Foundation

enum RuntimeStyleValue {
    case text(String), number(Double), unknown(String)
    var state: [String: Any] {
        switch self {
        case .text(let v): ["availability":"known", "value":["type":"text", "value":v]]
        case .number(let v): ["availability":"known", "value":["type":"number", "value":v]]
        case .unknown(let reason): ["availability":"unknown", "reason":reason]
        }
    }
}

#if (os(iOS) || os(tvOS)) && targetEnvironment(simulator)
import UIKit

/// Public configured metadata only. Never accesses label text or attributed runs.
@MainActor enum RuntimeLabelStyle {
    static func collect(_ label: UILabel) -> [String: RuntimeStyleValue] {
        var result: [String: RuntimeStyleValue] = [
            "schema": .number(1), "basis": .text("UILabel-public-configured-style"),
            "effective_paint_font": .unknown("attributed-runs-fitting-and-glyph-fallback-uninspected"),
            "effective_paint_color": .unknown("attributed-runs-and-rendering-effects-uninspected"),
            "adjusts_font_size_to_fit_width": .number(label.adjustsFontSizeToFitWidth ? 1 : 0),
            "minimum_scale_factor": number(label.minimumScaleFactor),
            "is_highlighted": .number(label.isHighlighted ? 1 : 0),
            "is_enabled": .number(label.isEnabled ? 1 : 0),
            "trait_interface_style": .number(Double(label.traitCollection.userInterfaceStyle.rawValue)),
            "trait_accessibility_contrast": .number(Double(label.traitCollection.accessibilityContrast.rawValue)),
            "trait_display_gamut": .number(Double(label.traitCollection.displayGamut.rawValue)),
            "trait_legibility_weight": .number(Double(label.traitCollection.legibilityWeight.rawValue))
        ]
        if let font = label.font {
            result["font_name"] = metadata(font.fontName)
            result["font_family"] = metadata(font.familyName)
            result["font_point_size"] = number(font.pointSize)
            result["font_units"] = .text("pt")
            result["font_symbolic_traits"] = .number(Double(font.fontDescriptor.symbolicTraits.rawValue))
            let traits = font.fontDescriptor.object(forKey: .traits) as? [UIFontDescriptor.TraitKey: Any]
            if let weight = traits?[.weight] as? NSNumber, CFGetTypeID(weight) != CFBooleanGetTypeID() {
                result["font_weight_trait"] = number(weight.doubleValue)
            } else { result["font_weight_trait"] = .unknown("public-weight-trait-unavailable") }
        } else {
            for key in ["font_name", "font_family", "font_point_size", "font_units",
                        "font_symbolic_traits", "font_weight_trait"] {
                result[key] = .unknown("configured-font-unavailable")
            }
        }
        for (prefix, color) in [("text_color", label.textColor), ("highlighted_text_color", label.highlightedTextColor)] {
            for (key, value) in resolved(color, traits: label.traitCollection) {
                result[prefix + "_" + key] = value
            }
        }
        return result
    }

    static func metadata(_ value: String) -> RuntimeStyleValue {
        guard !value.isEmpty, value.utf8.count <= 256,
              !value.unicodeScalars.contains(where: { $0.value < 32 || (127...159).contains($0.value) }),
              !value.contains("/"), !value.contains("\\") else {
            return .unknown("font-metadata-not-admitted")
        }
        return .text(value)
    }

    static func number(_ value: Double) -> RuntimeStyleValue {
        value.isFinite ? .number(value) : .unknown("nonfinite-public-value")
    }

    static func resolved(_ color: UIColor?, traits: UITraitCollection) -> [String: RuntimeStyleValue] {
        func unknown(_ reason: String) -> [String: RuntimeStyleValue] {
            Dictionary(uniqueKeysWithValues: ["space","r","g","b","a"].map { ($0,.unknown(reason)) })
        }
        guard let color else { return unknown("configured-color-absent") }
        let cg = color.resolvedColor(with: traits).cgColor
        guard let model = cg.colorSpace?.model, model == .rgb || model == .monochrome,
              let target = CGColorSpace(name: CGColorSpace.extendedSRGB),
              let converted = cg.converted(to: target, intent: .relativeColorimetric, options: nil),
              let values = converted.components, values.count == 4, values.allSatisfy(\.isFinite) else {
            return unknown("configured-color-conversion-unsupported")
        }
        return ["space":.text("extended-srgb"), "r":number(values[0]), "g":number(values[1]),
                "b":number(values[2]), "a":number(values[3])]
    }
}
#endif
#endif
