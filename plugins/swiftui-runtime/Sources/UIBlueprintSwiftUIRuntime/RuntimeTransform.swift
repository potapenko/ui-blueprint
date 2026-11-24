#if DEBUG
import CoreFoundation
import Foundation

/// Source-coordinate normalization only; measurements remain in the Rust engine.
struct RuntimeAffine {
    let values: [Double] // a, b, c, d, tx, ty

    func followed(by next: RuntimeAffine) -> RuntimeAffine? {
        let p = values, q = next.values
        let result = [q[0]*p[0]+q[2]*p[1], q[1]*p[0]+q[3]*p[1],
                      q[0]*p[2]+q[2]*p[3], q[1]*p[2]+q[3]*p[3],
                      q[0]*p[4]+q[2]*p[5]+q[4], q[1]*p[4]+q[3]*p[5]+q[5]]
        return result.allSatisfy(\.isFinite) ? RuntimeAffine(values: result) : nil
    }

    static func source(_ value: Any) -> RuntimeAffine? {
        // Only this known geometry value can reach Encodable. Never encode Data,
        // .value, environment or any arbitrary model/CustomReflectable value.
        guard String(reflecting: Swift.type(of: value)) == "SwiftUI.ViewTransform",
              let geometry = value as? any Encodable,
              let bytes = try? JSONEncoder().encode(geometry), bytes.count <= 65_536,
              let json = try? JSONSerialization.jsonObject(with: bytes) else { return nil }
        return decode(json)
    }

    static func decode(_ json: Any) -> RuntimeAffine? {
        guard let object = json as? [String: Any], Set(object.keys) == ["items", "positionAdjustment"],
              let adjustment = numbers(object["positionAdjustment"], count: 2),
              let items = object["items"] as? [[String: Any]], items.count <= 1024 else { return nil }
        var result = RuntimeAffine(values: [1, 0, 0, 1, -adjustment[0], -adjustment[1]])
        for item in items {
            if item.isEmpty { continue } // Source coordinate/scroll metadata, no affine effect.
            let step: RuntimeAffine
            if Set(item.keys) == ["translation"], let p = numbers(item["translation"], count: 2) {
                step = RuntimeAffine(values: [1, 0, 0, 1, p[0], p[1]])
            } else if Set(item.keys) == ["affineTransform"], let m = numbers(item["affineTransform"], count: 16),
                      [2, 3, 6, 7, 8, 9, 11, 14].allSatisfy({ m[$0] == 0 }), m[10] == 1, m[15] == 1 {
                step = RuntimeAffine(values: [m[0], m[1], m[4], m[5], m[12], m[13]])
            } else { return nil } // Perspective/3D or a new encoding is not an identity transform.
            guard let composed = result.followed(by: step) else { return nil }
            result = composed
        }
        return result
    }

    private static func numbers(_ value: Any?, count: Int) -> [Double]? {
        guard let array = value as? [NSNumber], array.count == count,
              array.allSatisfy({ CFGetTypeID($0) != CFBooleanGetTypeID() && $0.doubleValue.isFinite }) else { return nil }
        return array.map(\.doubleValue)
    }
}
#endif
