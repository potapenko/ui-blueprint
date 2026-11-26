#if DEBUG
import SwiftUI
#if os(macOS)
import AppKit
typealias RuntimeWindow = NSWindow
typealias RuntimeView = NSView
#elseif os(iOS)
import UIKit
typealias RuntimeWindow = UIWindow
typealias RuntimeView = UIView
#endif

@MainActor protocol RuntimeHosting {
    func runtimeGeometryTree() throws -> [_ViewDebug.Data]
}
#if os(macOS)
extension NSHostingView: RuntimeHosting {
    func runtimeGeometryTree() throws -> [_ViewDebug.Data] {
        try RuntimeUpdateContext.perform { _viewDebugData() }
    }
}
extension NSWindow {
    var runtimeContent: NSView? { contentView }
    var runtimeVisible: Bool { isVisible }
}
#elseif os(iOS)
extension _UIHostingView: RuntimeHosting {
    func runtimeGeometryTree() throws -> [_ViewDebug.Data] {
        try RuntimeUpdateContext.perform { _viewDebugData() }
    }
}
extension UIWindow {
    var runtimeContent: UIView? { self }
    var runtimeVisible: Bool { !isHidden && alpha > 0 && windowScene != nil }
}
#endif

@MainActor enum RuntimePlatform {
    nonisolated static var supportsUIKit: Bool {
        #if os(iOS) && targetEnvironment(simulator) && arch(arm64)
        let version = ProcessInfo.processInfo.operatingSystemVersion
        return version.majorVersion == 27 && version.minorVersion == 2 && version.patchVersion == 0
        #else
        return false
        #endif
    }
    nonisolated static var sourceMethod: String {
        #if os(macOS)
        "NSHostingView"
        #else
        "UIHostingView"
        #endif
    }
    static func metadata(_ window: RuntimeWindow) -> [String: Any] {
        #if os(macOS)
        return [:]
        #else
        let bounds = window.screen.bounds
        let origin = window.convert(CGPoint.zero, to: window.screen.coordinateSpace)
        let x = window.convert(CGPoint(x: 1, y: 0), to: window.screen.coordinateSpace)
        let y = window.convert(CGPoint(x: 0, y: 1), to: window.screen.coordinateSpace)
        return ["platform": "ios-simulator", "screen_width": bounds.width,
                "screen_height": bounds.height, "screen_scale": window.screen.scale,
                "window_to_screen": [x.x-origin.x, x.y-origin.y, y.x-origin.x, y.y-origin.y, origin.x, origin.y],
                "safe_area": [window.safeAreaInsets.top, window.safeAreaInsets.left,
                              window.safeAreaInsets.bottom, window.safeAreaInsets.right],
                "scene_state": window.windowScene?.activationState.rawValue ?? -1,
                "orientation": window.windowScene?.interfaceOrientation.rawValue ?? 0]
        #endif
    }

    static func uikitMetadata(_ window: RuntimeWindow) -> [String: Any] {
        var result = metadata(window)
        #if os(iOS)
        // No qualified screen transform is claimed by the UIKit model source.
        result.removeValue(forKey: "window_to_screen")
        result["window_bounds"] = [window.bounds.origin.x, window.bounds.origin.y,
                                   window.bounds.width, window.bounds.height]
        result["screen_bounds"] = [window.screen.bounds.origin.x, window.screen.bounds.origin.y,
                                   window.screen.bounds.width, window.screen.bounds.height]
        #endif
        return result
    }

    static var windows: [RuntimeWindow] {
        #if os(macOS)
        NSApp.windows.filter { $0.isVisible && $0.windowNumber > 0 }
        #else
        UIApplication.shared.connectedScenes.compactMap { $0 as? UIWindowScene }
            .flatMap(\.windows).filter(\.runtimeVisible)
        #endif
    }
}
#endif
