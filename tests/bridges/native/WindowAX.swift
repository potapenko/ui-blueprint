import ApplicationServices
import Foundation

// Internal bounded acquisition result; emitted graph uses only Stage A records.
struct WindowAXResult {
    let nodes: [[String: Any]]
    let metrics: [String: Any]
}

@MainActor func collectWindowAX(_ root: AXUIElement, surface: [String: Any], observationID: String,
                               maxNodes: Int, maxDepth: Int, deadline: Double) -> WindowAXResult {
    var handles = [root]
    var depths = [0]
    var nodes: [[String: Any]] = []
    var edges: [[Int]] = []
    var omitted = 0, unknownChildLists = 0, duplicateReferences = 0
    var availabilityCounts: [String: Int] = [:]
    let evidence: [String: Any] = ["observation_id": observationID, "source_namespace": "macos.ax",
        "provenance": "reported", "method": "bounded_public_ax_attributes", "uncertainty": null]
    func key(_ index: Int) -> [String: String] {
        ["namespace": "macos.ax", "key": "\(observationID)-handle-\(index)"]
    }
    func failure(_ error: AXError) -> [String: Any] {
        ["availability": error == .attributeUnsupported ? "unsupported" : "unknown",
         "reason": "ax_error_\(error.rawValue)"]
    }
    func batch(_ element: AXUIElement, _ names: [String]) -> [String: Any] {
        var values: CFArray?
        let error = AXUIElementCopyMultipleAttributeValues(element, names as CFArray, [], &values)
        guard error == .success, let values = values as? [Any], values.count == names.count else {
            return Dictionary(uniqueKeysWithValues: names.map { ($0, error == .success ? unavailable("batch_shape_mismatch") : failure(error)) })
        }
        return Dictionary(uniqueKeysWithValues: zip(names, values))
    }
    func scalar(_ value: Any?) -> [String: Any] {
        guard let value else { return unavailable("attribute_not_returned") }
        if let failure = value as? [String: Any], failure["availability"] != nil { return failure }
        let cf = value as CFTypeRef
        if CFGetTypeID(cf) == AXValueGetTypeID() {
            let ax = unsafeDowncast(cf, to: AXValue.self)
            if AXValueGetType(ax) == .axError {
                var error = AXError.failure
                AXValueGetValue(ax, .axError, &error)
                return failure(error)
            }
        }
        if CFGetTypeID(cf) == CFNullGetTypeID() { return unavailable("ax_null_no_value") }
        if let text = value as? String { return known("text", text) }
        if let number = value as? NSNumber {
            if CFGetTypeID(cf) == CFBooleanGetTypeID() { return known("flag", number.boolValue) }
            return known("number", number.doubleValue)
        }
        return unavailable("unsupported_ax_value_type")
    }
    func typed(_ value: Any?, expected: String) -> [String: Any] {
        let state = scalar(value)
        guard let payload = state["value"] as? [String: Any] else { return state }
        return payload["type"] as? String == expected ? state : unavailable("ax_value_type_mismatch")
    }
    func geometry(_ position: Any?, _ size: Any?) -> [String: Any] {
        guard let position, let size else { return unavailable("ax_bounds_not_returned") }
        let p = position as CFTypeRef, z = size as CFTypeRef
        guard CFGetTypeID(p) == AXValueGetTypeID(), CFGetTypeID(z) == AXValueGetTypeID() else {
            return unavailable("ax_bounds_unavailable")
        }
        if AXValueGetType(unsafeDowncast(p, to: AXValue.self)) == .axError { return scalar(position) }
        if AXValueGetType(unsafeDowncast(z, to: AXValue.self)) == .axError { return scalar(size) }
        var point = CGPoint.zero; var extent = CGSize.zero
        guard AXValueGetValue(unsafeDowncast(p, to: AXValue.self), .cgPoint, &point),
              AXValueGetValue(unsafeDowncast(z, to: AXValue.self), .cgSize, &extent) else {
            return unavailable("ax_bounds_unavailable")
        }
        return known("geometry", ["frame_kind": "accessibility_bounds",
            "coordinate_space": ["id": "ax-screen", "kind": "screen", "units": "pt", "origin": "top_left"],
            "shape": ["shape": "rect", "value": ["x": point.x, "y": point.y, "width": extent.width, "height": extent.height]],
            "transform": ["status": "unknown", "reason": "ax_to_pixels_not_calibrated"]])
    }
    while nodes.count < handles.count && nodes.count < maxNodes && ProcessInfo.processInfo.systemUptime < deadline {
        let index = nodes.count, el = handles[nodes.count]
        AXUIElementSetMessagingTimeout(el, 0.15)
        let identity = batch(el, [kAXRoleAttribute, kAXSubroleAttribute, kAXIdentifierAttribute])
        let role = identity[kAXRoleAttribute] as? String
        let subrole = identity[kAXSubroleAttribute] as? String
        let identifier = identity[kAXIdentifierAttribute] as? String
        let secure = role == "AXSecureTextField" || subrole == "AXSecureTextField" || identifier == "f02.secret"
        var names = [kAXDescriptionAttribute, kAXPlaceholderValueAttribute, kAXEnabledAttribute,
                     kAXFocusedAttribute, kAXPositionAttribute, kAXSizeAttribute]
        if !secure { names.append(kAXValueAttribute) } // redact before requesting the value
        let values = batch(el, names)
        let roles = ["AXButton": "button", "AXCheckBox": "checkbox", "AXTextField": "textbox",
                     "AXStaticText": "text", "AXGroup": "group", "AXScrollArea": "scrollarea", "AXSlider": "slider"]
        let normalized = role.flatMap { roles[$0] }
        var properties: [[String: Any]] = []
        func append(_ field: String, _ state: [String: Any], sensitive: Bool = false) {
            availabilityCounts[state["availability"] as? String ?? "unknown", default: 0] += 1
            properties.append(["selection": "requested", "field": field,
                "sensitivity": sensitive ? "sensitive" : "public", "evidence": evidence, "state": state])
        }
        append("role", normalized.map { known("role", $0) } ?? unavailable("raw_role_has_no_selected_mapping"))
        append("description", typed(values[kAXDescriptionAttribute], expected: "text"))
        append("value", secure ? ["availability": "redacted"] : scalar(values[kAXValueAttribute]), sensitive: secure)
        append("placeholder", typed(values[kAXPlaceholderValueAttribute], expected: "text"))
        append("enabled", typed(values[kAXEnabledAttribute], expected: "flag"))
        append("focused", typed(values[kAXFocusedAttribute], expected: "flag"))
        var actions: CFArray?
        let actionError = AXUIElementCopyActionNames(el, &actions)
        let actionState: [String: Any]
        if actionError != .success { actionState = failure(actionError) }
        else if let names = actions as? [String] { actionState = known("text_list", names) }
        else { actionState = unavailable("ax_action_names_type_mismatch") }
        append("actions", actionState)
        append("accessibility_bounds", geometry(values[kAXPositionAttribute], values[kAXSizeAttribute]))
        // Raw AX identifier/subrole are runtime extensions, never code declarations.
        let extensions: [[String: Any]] = [kAXIdentifierAttribute, kAXSubroleAttribute].map { name in
            ["namespace": "macos.ax", "name": name, "property": ["selection": "requested", "field": "description",
                "sensitivity": "public", "evidence": evidence, "state": typed(identity[name], expected: "text")]]
        }
        nodes.append(["key": key(index), "surface": surface,
            "native_role": typed(identity[kAXRoleAttribute], expected: "text"), "properties": properties,
            "children": [], "extensions": extensions, "source_declarations": []])
        var childIndices: [Int] = []
        var count: CFIndex = 0
        let countError = AXUIElementGetAttributeValueCount(el, kAXChildrenAttribute as CFString, &count)
        if countError == .success {
            let capacity = depths[index] + 1 < maxDepth ? max(0, maxNodes - handles.count) : 0
            let requested = min(Int(count), capacity)
            omitted += Int(count) - requested
            if requested > 0 {
                var array: CFArray?
                let error = AXUIElementCopyAttributeValues(el, kAXChildrenAttribute as CFString, 0, requested, &array)
                if error == .success, let children = array as? [AXUIElement] {
                    for child in children {
                        if let existing = handles.firstIndex(where: { CFEqual($0, child) }) {
                            duplicateReferences += 1
                            if !childIndices.contains(existing) { childIndices.append(existing) }
                        } else {
                            childIndices.append(handles.count)
                            handles.append(child); depths.append(depths[index] + 1)
                        }
                    }
                } else { unknownChildLists += 1 }
            }
        } else if countError != .attributeUnsupported && countError != .noValue {
            unknownChildLists += 1
        }
        edges.append(childIndices)
    }
    var omittedReferences = 0
    for index in nodes.indices {
        omittedReferences += edges[index].filter { $0 >= nodes.count }.count
        nodes[index]["children"] = edges[index].filter { $0 < nodes.count }.map(key)
    }
    let metrics: [String: Any] = ["visited_unique_nodes": nodes.count, "returned_nodes": nodes.count,
        "discovered_unique_handles": handles.count, "duplicate_handle_references": duplicateReferences,
        "known_unread_child_entries": omitted, "unreturned_child_references": omittedReferences,
        "unknown_children_lists": unknownChildLists, "remaining_queued_handles": handles.count - nodes.count,
        "maximum_observed_depth_zero_based": depths.prefix(nodes.count).max() ?? 0,
        "property_availability_counts": availabilityCounts,
        "coverage": "partial", "unexposed_native_or_visual_nodes": "unknown",
        "node_ceiling": maxNodes, "depth_ceiling": maxDepth,
        "identifier_policy": "observation_local_aliases_of_distinct_AX_handles_not_action_refs"]
    return WindowAXResult(nodes: nodes, metrics: metrics)
}
