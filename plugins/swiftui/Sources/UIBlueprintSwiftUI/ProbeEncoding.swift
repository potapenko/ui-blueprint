#if DEBUG
import Foundation

enum ProbeEncoding {
    private static let namespace = "swiftui.layout"
    private static let fields = ["layout_bounds", "hit_region", "visible_region", "paint_bounds"]

    static func snapshot(configuration c: ProbeConfiguration, sessionID: String,
                         surfaceGeneration: String, sequence: UInt64, layoutRevision: UInt64,
                         requestID: String, stateID: String, start: Double, end: Double,
                         markers: [ProbeMarker], globalFrame: CGRect, scale: Double,
                         cap: Int) throws -> Data {
        let target = ["id": c.targetID, "generation": c.targetGeneration]
        let surface = ["id": c.surfaceID, "generation": surfaceGeneration]
        let observationID = "swiftui-\(sequence)"
        let coverage: [String: Any] = ["status": "partial", "scope_id": c.scopeID,
            "fields": fields, "omitted_count": NSNull(), "unknown_count": markers.count * 3]
        let evidence: [String: Any] = ["observation_id": observationID, "source_namespace": namespace,
            "provenance": "reported", "method": "SwiftUI.bounds-anchor.explicit-capture", "uncertainty": NSNull()]
        let byID = Dictionary(uniqueKeysWithValues: markers.map { ($0.id, $0) })
        let nodes = markers.map { marker -> [String: Any] in
            var declarations = [
                declaration("marker_id", text(marker.id)), declaration("mount_id", text(marker.mountID)),
                declaration("environment", text("instrumented_debug")),
                declaration("request_id", text(requestID)), declaration("state_id", text(stateID)),
                declaration("parent_mapping", text("explicit-marker-declaration"))
            ]
            if let parent = marker.parent { declarations.append(declaration("parent_marker_id", text(parent))) }
            if marker.id == c.rootMarkerID {
                declarations += [declaration("display_scale", number(scale)),
                    declaration("layout_revision", text(String(layoutRevision))),
                    declaration("process_id", number(Double(ProcessInfo.processInfo.processIdentifier))),
                    declaration("root_swiftui_global_x", number(globalFrame.origin.x)),
                    declaration("root_swiftui_global_y", number(globalFrame.origin.y)),
                    declaration("root_swiftui_global_width", number(globalFrame.width)),
                    declaration("root_swiftui_global_height", number(globalFrame.height)),
                    declaration("global_space_kind", text("SwiftUI-global-not-screen"))]
            }
            let geometry: [String: Any] = ["frame_kind": "layout_bounds",
                "coordinate_space": ["id": "swiftui-root-pt", "kind": "local", "units": "pt", "origin": "top_left"],
                "shape": ["shape": "rect", "value": rect(marker.bounds)],
                "transform": ["status": "unknown", "reason": "screen-capture-transform-not-established"]]
            let layout = property("layout_bounds", evidence: evidence,
                state: ["availability": "known", "value": ["type": "geometry", "value": geometry]])
            let unknowns = fields.dropFirst().map {
                property($0, evidence: evidence,
                    state: ["availability": "unknown", "reason": "not-exposed-by-layout-probe"])
            }
            return ["key": key(marker), "surface": surface,
                "native_role": ["availability": "unsupported", "reason": "layout-marker-not-accessibility"],
                "properties": [layout] + unknowns,
                "children": markers.filter { $0.parent == marker.id }.map(key),
                "extensions": [], "source_declarations": declarations]
        }
        guard let root = byID[c.rootMarkerID] else { throw BlueprintProbeError.incompleteScope }
        let observation: [String: Any] = ["id": observationID, "source_namespace": namespace,
            "channel": "opt_in_layout_probe", "start": start, "end": end,
            "clock_domain": "swiftui-\(sessionID)", "time_unit": "seconds",
            "freshness_basis": "live_read", "answer_source": "cache", "freshness": "unverified",
            "last_verified": NSNull(), "consistency": "unknown",
            "consistency_reason": "layout-sample-not-presented-frame", "coverage": coverage]
        let snapshot: [String: Any] = ["surface_records": [],
            "id": "swiftui-\(sessionID)-\(sequence)", "revision": sequence, "source_state": stateID,
            "context": ["schema_version": "0.1.0", "session_id": sessionID, "target": target,
                "surfaces": [surface], "scope_id": c.scopeID, "projection": "design", "fields": fields,
                "plugin": ["id": "swiftui-probe", "version": "1.0.0"],
                "environment_revision": "swiftui-\(surfaceGeneration)-\(layoutRevision)"],
            "observations": [observation], "nodes": nodes, "relations": [],
            "components": [["logical_component_key": c.scopeID, "members": [key(root)] + markers.filter { $0.id != root.id }.map(key),
                "declaration_source": "explicit-swiftui-markers", "provenance": "reported"]],
            "focus": ["keyboard": ["status": "not_requested"], "accessibility": ["status": "not_requested"],
                "active_descendant": ["status": "not_requested"], "text_selection": NSNull(),
                "composition_state": ["selection": "not_requested", "field": "value"]],
            "captures": [], "coverage": coverage]
        do {
            var data = try JSONSerialization.data(withJSONObject: ["schema_version": "0.1.0",
                "artifact": ["kind": "snapshot", "data": snapshot]], options: [.sortedKeys])
            guard data.count < cap else { throw BlueprintProbeError.resourceLimit }
            data.append(0x0a)
            return data
        } catch let error as BlueprintProbeError { throw error }
        catch { throw BlueprintProbeError.encodingFailed }
    }

    private static func key(_ marker: ProbeMarker) -> [String: String] {
        ["namespace": namespace, "key": "\(marker.id)-\(marker.mountID)"]
    }
    private static func rect(_ value: CGRect) -> [String: Double] {
        ["x": value.origin.x, "y": value.origin.y, "width": value.width, "height": value.height]
    }
    private static func text(_ value: String) -> [String: Any] { ["type": "text", "value": value] }
    private static func number(_ value: Double) -> [String: Any] { ["type": "number", "value": value] }
    private static func declaration(_ name: String, _ value: [String: Any]) -> [String: Any] {
        ["namespace": namespace, "name": name, "state": ["availability": "known", "value": value],
         "sensitivity": "public", "source": "explicit-swiftui-probe"]
    }
    private static func property(_ field: String, evidence: [String: Any], state: [String: Any]) -> [String: Any] {
        ["selection": "requested", "field": field, "sensitivity": "public", "evidence": evidence, "state": state]
    }
}
#endif
