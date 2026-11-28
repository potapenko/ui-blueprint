import UIKit

/// Project-owned public-API oracle. No application content or source introspection.
@MainActor enum FocusMatrixChecks {
    static func run() {
        guard let window = UIApplication.shared.connectedScenes.compactMap({ $0 as? UIWindowScene })
            .flatMap(\.windows).first(where: \.isKeyWindow) else { return }
        let parent = UIView(frame: CGRect(x: 200, y: 700, width: 500, height: 250))
        parent.bounds.origin = CGPoint(x: 17, y: 23)
        let child = UIView(frame: CGRect(x: 30, y: 40, width: 160, height: 80))
        child.bounds.origin = CGPoint(x: 7, y: 11)
        parent.addSubview(child); window.addSubview(parent)
        var records: [[String: Any]] = []
        func check(_ name: String, _ expected: Bool) {
            let result = RuntimeUIKit.mapping(child, to: window)
            var record: [String: Any] = ["name": name, "expected_known": expected,
                "actual_known": result != nil, "passed": (result != nil) == expected]
            if let result {
                let m = result.values
                let b = child.bounds
                var agrees = true
                for point in [CGPoint(x:b.minX,y:b.minY),CGPoint(x:b.maxX,y:b.minY),
                              CGPoint(x:b.minX,y:b.maxY),CGPoint(x:b.maxX,y:b.maxY),
                              CGPoint(x:b.midX,y:b.midY),CGPoint(x:b.minX+b.width*0.37,y:b.minY+b.height*0.61)] {
                    let actual = child.convert(point,to:window)
                    agrees = agrees && abs(actual.x-(m[0]*point.x+m[2]*point.y+m[4])) < 1e-7
                        && abs(actual.y-(m[1]*point.x+m[3]*point.y+m[5])) < 1e-7
                }
                record["affine"] = m
                record["public_points_agree"] = agrees
                record["passed"] = expected && agrees
            }
            records.append(record)
        }
        for (name, value) in [("nonfinite-perspective", CGFloat.infinity), ("nan-perspective", CGFloat.nan)] {
            var matrix = CATransform3DIdentity; matrix.m34 = value
            records.append(["name": name, "passed": !RuntimeUIKit.preservesModelPlane(matrix)])
        }
        var invalidW = CATransform3DIdentity; invalidW.m44 = 0
        records.append(["name": "zero-w", "passed": !RuntimeUIKit.preservesModelPlane(invalidW)])
        check("legacy-nonzero-bounds", true)
        var perspective = CATransform3DIdentity
        perspective.m34 = -0.001
        parent.layer.transform = perspective
        check("inert-parent-perspective", true)
        child.transform = CGAffineTransform(rotationAngle: 0.2).scaledBy(x: 1.2, y: 0.8)
        check("rotated-scaled-descendant", true)
        child.layer.zPosition = 20
        check("depth-under-perspective", false)
        child.layer.zPosition = 0
        child.layer.anchorPointZ = 1
        check("anchor-depth-under-perspective", false)
        child.layer.anchorPointZ = 0
        child.layer.transform = CATransform3DMakeRotation(0.2, 1, 0, 0)
        check("tilt-under-perspective", false)
        var tinyTilt = CATransform3DIdentity
        tinyTilt.m13 = 1e-14
        child.layer.transform = tinyTilt
        check("tiny-tilt-no-epsilon", false)
        child.layer.transform = CATransform3DMakeTranslation(0, 0, 10)
        check("matrix-depth-under-perspective", false)
        child.layer.transform = CATransform3DIdentity
        parent.layer.transform = CATransform3DIdentity
        parent.layer.sublayerTransform = perspective
        check("inert-sublayer-perspective", true)
        var projective = CATransform3DIdentity
        projective.m14 = 0.0001
        child.layer.transform = projective
        check("x-dependent-perspective", false)
        projective = CATransform3DIdentity; projective.m24 = 0.0001
        child.layer.transform = projective
        check("y-dependent-perspective", false)
        child.layer.transform = CATransform3DIdentity
        parent.layer.sublayerTransform = CATransform3DIdentity
        parent.layer.zPosition = 20
        check("legacy-affine-depth", true)
        parent.layer.zPosition = 0
        parent.layer.transform = CATransform3DMakeAffineTransform(CGAffineTransform(scaleX:1.25,y:1.25))
        parent.layer.sublayerTransform = perspective
        child.backgroundColor = .red
        check("visible-affine-marker", true)
        let points = [CGPoint(x:child.bounds.minX,y:child.bounds.minY),
                      CGPoint(x:child.bounds.maxX,y:child.bounds.maxY)].map {
            let p=child.convert($0,to:window);return [p.x,p.y]
        }
        let payload: [String: Any] = ["cases":records,"marker_window_corners":points,
                                     "screen_scale":window.screen.scale]
        if let data=try? JSONSerialization.data(withJSONObject:payload,options:.sortedKeys) {
            try? data.write(to:FileManager.default.temporaryDirectory.appendingPathComponent("matrix-checks.json"),options:.atomic)
        }
    }
}
