#if DEBUG
import Foundation

enum RuntimeEncoding {
    static let namespace = "swiftui.runtime.layout"

    static func snapshot(_ observation: RuntimeObservation, session: String, bundle: String,
                         request: String, sequence: UInt64) throws -> Data {
        let target = ["id": bundle, "generation": session]
        let surface = ["id": "window-\(observation.window)", "generation": observation.windowGeneration]
        let evidence: [String: Any] = ["observation_id": observation.id, "source_namespace": namespace,
            "provenance": "reported", "method": "NSHostingView.typed-viewDebugData.geometry-allowlist",
            "uncertainty": NSNull()]
        let fields = ["layout_bounds"]
        let missing = observation.nodes.filter { $0.position == nil || $0.size == nil }.count
        let coverage: [String: Any] = ["status": missing == 0 && observation.emptyHostingRoots == 0 ? "complete" : "partial",
            "scope_id": "window-\(observation.window)", "fields": fields,
            "omitted_count": observation.emptyHostingRoots == 0 ? 0 : NSNull(), "unknown_count": missing]
        let childMap = Dictionary(grouping: observation.nodes.compactMap { node in
            node.parent.map { ($0, node.index) }
        }, by: { $0.0 })
        func key(_ index: Int) -> [String: String] {
            ["namespace": namespace, "key": "\(observation.id)-n\(index)"]
        }
        let nodes: [[String: Any]] = observation.nodes.map { node in
            let inherited = (node.positionSource != nil && node.positionSource != node.index)
                || (node.sizeSource != nil && node.sizeSource != node.index)
            var geometryEvidence = evidence
            if inherited {
                geometryEvidence["provenance"] = "derived"
                geometryEvidence["method"] = "NSHostingView.debug-layout-input-context"
            }
            var state: [String: Any] = ["availability": "unknown", "reason": "source-has-no-position-size-pair"]
            if let position = node.position, let size = node.size {
                let localSpace = ["id": "\(observation.id)-local-\(node.index)",
                                  "kind": "local", "units": "pt", "origin": "top_left"]
                var transform: [String: Any] = ["status": "unknown", "reason": "source-transform-unavailable"]
                if let affine = node.transform {
                    var mappingEvidence = evidence
                    mappingEvidence["provenance"] = "derived"
                    mappingEvidence["method"] = "SwiftUI.ViewTransform.qualified-affine-context"
                    transform = ["status": "known", "transform": ["from": localSpace,
                        "to": ["id": "\(observation.id)-swiftui-global-\(node.hostingRoot)",
                               "kind": "local", "units": "pt", "origin": "top_left"],
                        "affine": affine.values, "target": target, "surface": surface,
                        "environment_revision": observation.id, "evidence": mappingEvidence]]
                }
                let geometry: [String: Any] = ["frame_kind": "layout_bounds",
                    "coordinate_space": localSpace,
                    "shape": ["shape": "rect", "value": ["x": position.x, "y": position.y,
                        "width": size.width, "height": size.height]],
                    "transform": transform]
                state = ["availability": "known", "value": ["type": "geometry", "value": geometry]]
            }
            let role: [String: Any] = node.type.map {
                ["availability": "known", "value": ["type": "text", "value": $0]]
            } ?? ["availability": "unknown", "reason": "source-type-not-exposed"]
            var declarations = [declaration("hosting_root", .number(Double(node.hostingRoot)))]
            if let source = node.positionSource, source != node.index {
                declarations.append(declaration("layout_position_source", .text(key(source)["key"]!)))
            }
            if let source = node.sizeSource, source != node.index {
                declarations.append(declaration("layout_size_source", .text(key(source)["key"]!)))
            }
            if node.parent == nil {
                declarations += [declaration("request_id", .text(request)),
                                 declaration("hosting_root_count", .number(Double(observation.hostingRoots))),
                                 declaration("empty_hosting_root_count", .number(Double(observation.emptyHostingRoots)))]
            }
            return ["key": key(node.index), "surface": surface, "native_role": role,
                "properties": [["selection": "requested", "field": "layout_bounds", "sensitivity": "public",
                    "evidence": geometryEvidence, "state": state]],
                "children": childMap[node.index, default: []].map { key($0.1) }, "extensions": [],
                "source_declarations": declarations]
        }
        let observed: [String: Any] = ["id": observation.id, "source_namespace": namespace,
            "channel": "opt_in_layout_probe", "start": observation.start, "end": observation.end,
            "clock_domain": "runtime-\(session)", "time_unit": "seconds", "freshness_basis": "live_read",
            "answer_source": "cache", "freshness": "unverified", "last_verified": NSNull(),
            "consistency": "unknown", "consistency_reason": "main-actor-layout-sample-not-presented-frame",
            "coverage": coverage]
        let snapshot: [String: Any] = ["id": observation.id, "revision": sequence, "source_state": NSNull(),
            "surface_records": [], "context": ["schema_version": "0.1.0", "session_id": session,
                "target": target, "surfaces": [surface], "scope_id": "window-\(observation.window)",
                "projection": "design", "fields": fields,
                "plugin": ["id": "swiftui-runtime-probe", "version": "1.0.0"],
                "environment_revision": observation.id],
            "observations": [observed], "nodes": nodes, "relations": [], "components": [],
            "focus": ["keyboard": ["status": "not_requested"], "accessibility": ["status": "not_requested"],
                "active_descendant": ["status": "not_requested"], "text_selection": NSNull(),
                "composition_state": ["selection": "not_requested", "field": "value"]],
            "captures": [], "coverage": coverage]
        let envelope: [String: Any] = ["schema_version": "0.1.0", "artifact": ["kind": "snapshot", "data": snapshot]]
        let data = try JSONSerialization.data(withJSONObject: envelope, options: [.sortedKeys])
        guard data.count <= 16 * 1024 * 1024 else { throw RuntimeRefusal.outputLimit }
        return data
    }

    private enum Value {
        case text(String), number(Double)
        var json: [String: Any] {
            switch self {
            case .text(let value): ["type": "text", "value": value]
            case .number(let value): ["type": "number", "value": value]
            }
        }
    }
    private static func declaration(_ name: String, _ value: Value) -> [String: Any] {
        ["namespace": namespace, "name": name, "state": ["availability": "known", "value": value.json],
         "sensitivity": "public", "source": "in-process-runtime-geometry"]
    }
}
#endif
