#if DEBUG && (os(iOS) || os(tvOS)) && targetEnvironment(simulator)
import UIKit

/// Read-only model geometry. No text, accessibility, descriptions or presentation layers.
@MainActor enum RuntimeUIKit {
    static func collect(_ window: UIWindow) throws -> [RuntimeNode] {
        var nodes: [RuntimeNode] = []
        var pending: [(UIView, Int?, Int)] = [(window, nil, 0)]
        while let (view, parent, depth) = pending.popLast() {
            guard nodes.count < RuntimeCollector.maxNodes else {
                throw RuntimeSourceLimit(kind: .nativeNodes, observed: UInt64(nodes.count) + 1)
            }
            guard depth <= RuntimeCollector.maxDepth else {
                throw RuntimeSourceLimit(kind: .nativeDepth, observed: UInt64(depth))
            }
            guard view === window || view.window === window else { throw RuntimeRefusal.staleWindow }
            let bounds = view.bounds
            guard [bounds.origin.x, bounds.origin.y, bounds.width, bounds.height].allSatisfy(\.isFinite),
                  bounds.width >= 0, bounds.height >= 0 else { throw RuntimeRefusal.invalidGeometry }
            let index = nodes.count
            nodes.append(RuntimeNode(index: index, parent: parent, hostingRoot: 0,
                type: try RuntimeCollector.sanitizedType(String(reflecting: type(of: view))),
                position: bounds.origin, size: bounds.size, positionSource: index, sizeSource: index,
                transform: mapping(view, to: window)))
            let children = view.subviews
            guard nodes.count + pending.count + children.count <= RuntimeCollector.maxNodes else {
                throw RuntimeSourceLimit(kind: .nativeNodes,
                    observed: UInt64(nodes.count + pending.count + children.count))
            }
            pending.append(contentsOf: children.reversed().map { ($0, Optional(index), depth + 1) })
        }
        return nodes
    }

    static func mapping(_ view: UIView, to window: UIWindow) -> RuntimeAffine? {
        // Check the full layer ancestry, including intermediary non-UIView layers.
        // A four-point coincidence alone cannot qualify a perspective transform.
        var layer: CALayer? = view.layer
        var reachedWindow = false
        var depth = 0
        while let current = layer {
            guard depth <= RuntimeCollector.maxDepth,
                  CATransform3DIsAffine(current.transform),
                  CATransform3DIsAffine(current.sublayerTransform) else { return nil }
            if current === window.layer { reachedWindow = true; break }
            layer = current.superlayer
            depth += 1
        }
        guard reachedWindow else { return nil }
        let origin = view.convert(CGPoint.zero, to: window)
        let x = view.convert(CGPoint(x: 1, y: 0), to: window)
        let y = view.convert(CGPoint(x: 0, y: 1), to: window)
        let m = [Double(x.x-origin.x), Double(x.y-origin.y), Double(y.x-origin.x),
                 Double(y.y-origin.y), Double(origin.x), Double(origin.y)]
        guard m.allSatisfy(\.isFinite) else { return nil }
        let b = view.bounds
        for point in [CGPoint(x: b.minX, y: b.minY), CGPoint(x: b.maxX, y: b.minY),
                      CGPoint(x: b.minX, y: b.maxY), CGPoint(x: b.maxX, y: b.maxY)] {
            let actual = view.convert(point, to: window)
            let expectedX = m[0]*point.x + m[2]*point.y + m[4]
            let expectedY = m[1]*point.x + m[3]*point.y + m[5]
            // Roundoff qualification only; preserve original bounds and coefficients.
            guard actual.x.isFinite, actual.y.isFinite,
                  abs(actual.x-expectedX) <= 1e-7, abs(actual.y-expectedY) <= 1e-7 else { return nil }
        }
        return RuntimeAffine(values: m)
    }
}
#endif
