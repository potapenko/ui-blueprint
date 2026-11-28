// Synthetic fixture only: public UIKit matrices diagnose model-space qualification.
import SwiftUI
import UIKit

@MainActor enum FocusOracle {
    static func matrix(_ t: CATransform3D) -> [Double] {
        [t.m11,t.m12,t.m13,t.m14,t.m21,t.m22,t.m23,t.m24,
         t.m31,t.m32,t.m33,t.m34,t.m41,t.m42,t.m43,t.m44].map(Double.init)
    }
    static func save() {
        guard let window = UIApplication.shared.connectedScenes.compactMap({ $0 as? UIWindowScene })
            .flatMap(\.windows).first(where: \.isKeyWindow) else { return }
        var result: [[String: Any]] = []
        func visit(_ view: UIView, _ parent: Int?) {
            let index = result.count
            let b = view.bounds
            let points = [CGPoint.zero, CGPoint(x: 1,y: 0), CGPoint(x: 0,y: 1),
                          CGPoint(x:b.minX,y:b.minY),CGPoint(x:b.maxX,y:b.maxY)].map {
                let p=view.convert($0,to:window);return [p.x,p.y]
            }
            var layers: [[String:Any]]=[]
            var layer: CALayer?=view.layer
            while let current=layer {
                layers.append(["transform":matrix(current.transform),"sublayer":matrix(current.sublayerTransform),
                               "affine":CATransform3DIsAffine(current.transform),
                               "sublayer_affine":CATransform3DIsAffine(current.sublayerTransform),
                               "z_position":current.zPosition,"anchor_z":current.anchorPointZ])
                if current === window.layer { break }
                layer=current.superlayer
            }
            result.append(["index":index,"parent":parent as Any? ?? NSNull(),
                "type":String(reflecting:type(of:view)),"bounds":[b.minX,b.minY,b.width,b.height],
                "focused":view.isFocused,"points":points,"layers":layers,
                "mapping_known":RuntimeUIKit.mapping(view,to:window) != nil])
            for child in view.subviews { visit(child,index) }
        }
        visit(window,nil)
        if let data=try? JSONSerialization.data(withJSONObject:result,options:.sortedKeys) {
            try? data.write(to:FileManager.default.temporaryDirectory.appendingPathComponent("focus-oracle.json"),options:.atomic)
        }
    }
}

@main struct UIKitFocusGeometryFixture: App {
    private let session: BlueprintRuntimeSession?
    init() { session=try? BlueprintRuntimeSession.startIfRequested() }
    var body: some Scene {
        WindowGroup {
            NavigationStack {
                List {
                    Button("Synthetic focus row") {}
                    Button("Second fixture row") {}
                }.navigationTitle("Geometry fixture")
            }
            .task {
                try? await Task.sleep(for:.seconds(3))
                FocusOracle.save()
                FocusMatrixChecks.run()
            }
        }
    }
}
