import ApplicationServices
import Foundation

struct WindowAXResult {
    let nodes: [[String: Any]]
    let metrics: [String: Any]
}
private enum AXReadIssue { case platform(AXError), shape, budget }

// Exact public AX call boundary. Tests provide bounded CF handles/pages; this
// internal executable type has no runtime selector, configuration or fault mode.
@MainActor struct NativeAXAccess {
    let prepare: (CFTypeRef) -> Void
    let attribute: (CFTypeRef, String) -> (AXError, CFTypeRef?)
    let count: (CFTypeRef, String) -> (AXError, Int)
    let page: (CFTypeRef, String, Int, Int) -> (AXError, CFArray?)
    let batch: (CFTypeRef, [String]) -> (AXError, CFArray?)
    let actions: (CFTypeRef) -> (AXError, CFArray?)
    let isElement: (CFTypeRef) -> Bool

    static var live: Self {
        Self(prepare: { AXUIElementSetMessagingTimeout(unsafeDowncast($0, to: AXUIElement.self), 0.15) },
            attribute: { raw, name in
                var value: CFTypeRef?
                let status = AXUIElementCopyAttributeValue(unsafeDowncast(raw, to: AXUIElement.self), name as CFString, &value)
                return (status, value)
            }, count: { raw, name in
                var count: CFIndex = 0
                let status = AXUIElementGetAttributeValueCount(unsafeDowncast(raw, to: AXUIElement.self), name as CFString, &count)
                return (status, count)
            }, page: { raw, name, index, amount in
                var values: CFArray?
                let status = AXUIElementCopyAttributeValues(unsafeDowncast(raw, to: AXUIElement.self), name as CFString, index, amount, &values)
                return (status, values)
            }, batch: { raw, names in
                var values: CFArray?
                let status = AXUIElementCopyMultipleAttributeValues(unsafeDowncast(raw, to: AXUIElement.self), names as CFArray, [], &values)
                return (status, values)
            }, actions: { raw in
                var values: CFArray?
                let status = AXUIElementCopyActionNames(unsafeDowncast(raw, to: AXUIElement.self), &values)
                return (status, values)
            }, isElement: { CFGetTypeID($0) == AXUIElementGetTypeID() })
    }
}

@MainActor func nativeAXAttribute(_ element: CFTypeRef, _ name: String,
    admission: NativeAcquisition, access: NativeAXAccess = .live) throws -> CFTypeRef? {
    try admission.check()
    guard access.isElement(element) else { throw NativeAcquisitionError.invalidValue }
    access.prepare(element)
    let (status, value) = access.attribute(element, name)
    return status == .success ? value : nil
}

@MainActor func nativeAXElements(_ element: CFTypeRef, _ attribute: String,
    windows: Bool, capacity: Int, admission: NativeAcquisition, access: NativeAXAccess,
    visit: (CFTypeRef) throws -> Void) throws -> Int {
    try admission.check()
    guard access.isElement(element) else { throw NativeAcquisitionError.invalidValue }
    access.prepare(element)
    let (status, count) = access.count(element, attribute)
    if status == .attributeUnsupported || status == .noValue { return 0 }
    guard status == .success else { throw NativeAcquisitionError.invalidValue }
    let omitted = try admission.array(count: count, windows: windows, capacity: capacity, read: { offset, amount in
        let (status, page) = access.page(element, attribute, offset, amount)
        guard status == .success, let page else { throw NativeAcquisitionError.invalidValue }
        return page
    }, visit: { raw in
        guard access.isElement(raw) else { throw NativeAcquisitionError.invalidValue }
        try visit(raw)
    })
    if windows {
        let (status, after) = access.count(element, attribute)
        guard status == .success, after == count else { throw NativeAcquisitionError.invalidValue }
    }
    return omitted
}

@MainActor func nativeAXArray(_ element: AXUIElement, _ attribute: String,
    windows: Bool, capacity: Int, admission: NativeAcquisition,
    visit: (AXUIElement) throws -> Void) throws -> Int {
    try nativeAXElements(element, attribute, windows: windows, capacity: capacity, admission: admission, access: .live) {
        try visit(unsafeDowncast($0, to: AXUIElement.self))
    }
}

func nativeAXScalar(_ value: Any?, json: NativeJSON) throws -> [String: Any] {
    guard let value else { return try json.unavailable("attribute_not_returned") }
    if let issue = value as? AXReadIssue {
        switch issue {
        case .platform(let error):
            return try json.unavailable("ax_error_\(error.rawValue)", status: error == .attributeUnsupported ? "unsupported" : "unknown")
        case .shape: return try json.unavailable("batch_shape_mismatch")
        case .budget: return try json.unavailable("native_acquisition_limit")
        }
    }
    if let text = value as? String { return try json.known("text", text) }
    let cf = value as CFTypeRef
    if CFGetTypeID(cf) == AXValueGetTypeID() {
        let ax = unsafeDowncast(cf, to: AXValue.self)
        if AXValueGetType(ax) == .axError {
            var error = AXError.failure
            AXValueGetValue(ax, .axError, &error)
            return try nativeAXScalar(AXReadIssue.platform(error), json: json)
        }
    }
    if CFGetTypeID(cf) == CFNullGetTypeID() { return try json.unavailable("ax_null_no_value") }
    if let number = value as? NSNumber {
        if CFGetTypeID(cf) == CFBooleanGetTypeID() { return try json.known("flag", number.boolValue) }
        guard number.doubleValue.isFinite else { return try json.unavailable("nonfinite_ax_value") }
        return try json.known("number", number.doubleValue)
    }
    return try json.unavailable("unsupported_ax_value_type")
}

func nativeAXGeometry(_ position: Any?, _ size: Any?, json: NativeJSON) throws -> [String: Any] {
    guard let position, let size else { return try json.unavailable("ax_bounds_not_returned") }
    let p = position as CFTypeRef, z = size as CFTypeRef
    guard CFGetTypeID(p) == AXValueGetTypeID(), CFGetTypeID(z) == AXValueGetTypeID() else {
        return try json.unavailable("ax_bounds_unavailable")
    }
    if AXValueGetType(unsafeDowncast(p, to: AXValue.self)) == .axError { return try nativeAXScalar(position, json: json) }
    if AXValueGetType(unsafeDowncast(z, to: AXValue.self)) == .axError { return try nativeAXScalar(size, json: json) }
    var point = CGPoint.zero; var extent = CGSize.zero
    guard AXValueGetValue(unsafeDowncast(p, to: AXValue.self), .cgPoint, &point),
          AXValueGetValue(unsafeDowncast(z, to: AXValue.self), .cgSize, &extent),
          point.x.isFinite, point.y.isFinite, extent.width.isFinite, extent.height.isFinite,
          extent.width >= 0, extent.height >= 0 else {
        return try json.unavailable("ax_bounds_unavailable")
    }
    return try json.knownBuilt("geometry") {
        try json.object(["frame_kind", "coordinate_space", "shape", "transform"]) {
            ["frame_kind": try json.scalar("accessibility_bounds"),
             "coordinate_space": try json.object(["id", "kind", "units", "origin"]) {
                ["id": try json.scalar("ax-screen"), "kind": try json.scalar("screen"),
                 "units": try json.scalar("pt"), "origin": try json.scalar("top_left")]
             }, "shape": try json.object(["shape", "value"]) {
                ["shape": try json.scalar("rect"), "value": try json.object(["x", "y", "width", "height"]) {
                    ["x": try json.scalar(point.x), "y": try json.scalar(point.y),
                     "width": try json.scalar(extent.width), "height": try json.scalar(extent.height)]
                }]
             }, "transform": try json.object(["status", "reason"]) {
                ["status": try json.scalar("unknown"), "reason": try json.scalar("ax_to_pixels_not_calibrated")]
             }]
        }
    }
}

@MainActor func collectWindowAX(_ root: CFTypeRef, surface: [String: Any], observationID: String,
    maxNodes: Int, maxDepth: Int, deadline: Double, admission: NativeAcquisition,
    fields: [String], json: NativeJSON, access: NativeAXAccess = .live) throws -> WindowAXResult {
    let selected = Set(fields)
    guard !fields.isEmpty, selected.count == fields.count,
          selected.isSubset(of: ["role", "accessibility_name", "description", "value", "placeholder", "enabled", "focused", "checked", "actions", "accessibility_bounds"]),
          access.isElement(root) else { throw NativeAcquisitionError.invalidValue }
    var handles = [root], depths = [0]
    var nodes: [[String: Any]] = try json.array { [] }
    var edges: [[Int]] = []
    var omitted = 0, unknownChildLists = 0, duplicateReferences = 0
    var availabilityCounts: [String: Int] = [:]
    func key(_ index: Int) throws -> [String: Any] {
        try json.object(["namespace", "key"]) {
            ["namespace": try json.scalar("macos.ax"), "key": try json.scalar("\(observationID)-handle-\(index)")]
        }
    }
    func batch(_ element: CFTypeRef, _ names: [String]) throws -> [String: Any] {
        try admission.check()
        guard names.count <= admission.limits.batch_values else {
            return Dictionary(uniqueKeysWithValues: names.map { ($0, AXReadIssue.budget) })
        }
        let (error, values) = access.batch(element, names)
        guard error == .success, let values else {
            return Dictionary(uniqueKeysWithValues: names.map { ($0, error == .success ? AXReadIssue.shape : .platform(error)) })
        }
        do {
            let admitted = try admission.batch(values, expected: names.count)
            return Dictionary(uniqueKeysWithValues: zip(names, admitted))
        } catch NativeAcquisitionError.expired { throw NativeAcquisitionError.expired }
        catch { return Dictionary(uniqueKeysWithValues: names.map { ($0, AXReadIssue.budget) }) }
    }
    func typed(_ value: Any?, expected: String) throws -> [String: Any] {
        // Avoid constructing a discarded known state merely to discover a type mismatch.
        if expected == "text", value is NSNumber { return try json.unavailable("ax_value_type_mismatch") }
        if expected == "flag", let value {
            let raw = value as CFTypeRef
            if value is String || (value is NSNumber && CFGetTypeID(raw) != CFBooleanGetTypeID()) {
                return try json.unavailable("ax_value_type_mismatch")
            }
        }
        return try nativeAXScalar(value, json: json)
    }
    while nodes.count < handles.count && nodes.count < maxNodes && ProcessInfo.processInfo.systemUptime < deadline {
        try admission.check()
        let index = nodes.count, el = handles[nodes.count]
        access.prepare(el)
        let identity = try batch(el, [kAXRoleAttribute, kAXSubroleAttribute, kAXIdentifierAttribute])
        let role = identity[kAXRoleAttribute] as? String
        let subrole = identity[kAXSubroleAttribute] as? String
        let identifier = identity[kAXIdentifierAttribute] as? String
        let secure = role == "AXSecureTextField" || subrole == "AXSecureTextField" || identifier == "f02.secret"
        func admittedIdentity(_ name: String) -> Bool {
            guard let value = identity[name] else { return false }
            if value is String { return true }
            // Public noValue/unsupported are explicit absence, not a dropped/oversize identity.
            if let issue = value as? AXReadIssue, case .platform(let error) = issue {
                return error == .noValue || error == .attributeUnsupported
            }
            let cf = value as CFTypeRef
            if CFGetTypeID(cf) == AXValueGetTypeID(), AXValueGetType(unsafeDowncast(cf, to: AXValue.self)) == .axError {
                var error = AXError.failure
                AXValueGetValue(unsafeDowncast(cf, to: AXValue.self), .axError, &error)
                return error == .noValue || error == .attributeUnsupported
            }
            return false
        }
        let safeIdentity = [kAXRoleAttribute, kAXSubroleAttribute, kAXIdentifierAttribute].allSatisfy(admittedIdentity)
        var names: [String] = []
        if selected.contains("description") || selected.contains("accessibility_name") { names.append(kAXDescriptionAttribute) }
        if selected.contains("placeholder") { names.append(kAXPlaceholderValueAttribute) }
        if selected.contains("enabled") { names.append(kAXEnabledAttribute) }
        if selected.contains("focused") { names.append(kAXFocusedAttribute) }
        if selected.contains("accessibility_bounds") { names += [kAXPositionAttribute, kAXSizeAttribute] }
        if (selected.contains("value") || selected.contains("checked")) && nativeMayReadValue(role: role, subrole: subrole, identifier: identifier, complete: safeIdentity) { names.append(kAXValueAttribute) }
        let values = names.isEmpty ? [:] : try batch(el, names)
        // An inadmissible Title must not discard the independently admitted
        // original fields. The same per-value/batch/aggregate checks still apply.
        let title = selected.contains("accessibility_name") ? try batch(el, [kAXTitleAttribute])[kAXTitleAttribute] : nil
        let roles = ["AXButton": "button", "AXCheckBox": "checkbox", "AXTextField": "textbox",
                     "AXStaticText": "text", "AXGroup": "group", "AXScrollArea": "scrollarea", "AXSlider": "slider"]
        let normalized = role.flatMap { roles[$0] }
        var properties: [[String: Any]] = try json.array { [] }
        func append(_ field: String, sensitive: Bool = false, state: () throws -> [String: Any]) throws {
            let property = try json.object(["selection", "field", "sensitivity", "evidence", "state"]) {
                ["selection": try json.scalar("requested"), "field": try json.scalar(field),
                 "sensitivity": try json.scalar(sensitive ? "sensitive" : "public"),
                 "evidence": try json.evidence(observationID, "macos.ax", "bounded_public_ax_attributes"), "state": try state()]
            }
            let status = (property["state"] as? [String: Any])?["availability"] as? String ?? "unknown"
            availabilityCounts[status, default: 0] += 1
            properties.append(property)
        }
        for field in fields {
            switch field {
            case "role":
                try append(field) { try normalized.map { try json.known("role", $0) } ?? json.unavailable("raw_role_has_no_selected_mapping") }
            case "description", "accessibility_name":
                try append(field) { try typed(values[kAXDescriptionAttribute], expected: "text") }
            case "value":
                try append(field, sensitive: secure) {
                    if secure { return try json.redacted() }
                    if !nativeMayReadValue(role: role, subrole: subrole, identifier: identifier, complete: safeIdentity) { return try json.unavailable("identity_classification_unavailable") }
                    return try nativeAXScalar(values[kAXValueAttribute], json: json)
                }
            case "checked":
                try append(field) {
                    guard role == "AXCheckBox", let number = values[kAXValueAttribute] as? NSNumber,
                          CFGetTypeID(number) == CFNumberGetTypeID(), [0.0, 1.0].contains(number.doubleValue)
                    else { return try json.unavailable("checked_state_not_exposed", status: "unsupported") }
                    return try json.known("flag", number.doubleValue == 1)
                }
            case "placeholder": try append(field) { try typed(values[kAXPlaceholderValueAttribute], expected: "text") }
            case "enabled": try append(field) { try typed(values[kAXEnabledAttribute], expected: "flag") }
            case "focused": try append(field) { try typed(values[kAXFocusedAttribute], expected: "flag") }
            case "actions":
                try admission.check()
                let (status, raw) = access.actions(el)
                let names = status == .success ? raw.flatMap { try? admission.actions($0) } : nil
                try append(field) {
                    if status != .success { return try nativeAXScalar(AXReadIssue.platform(status), json: json) }
                    if let names { return try json.known("text_list", names) }
                    return try json.unavailable("native_acquisition_limit")
                }
            case "accessibility_bounds": try append(field) { try nativeAXGeometry(values[kAXPositionAttribute], values[kAXSizeAttribute], json: json) }
            default: throw NativeAcquisitionError.invalidValue
            }
        }
        let extensions = try json.array {
            // Keep the raw title distinct from the existing AXDescription mapping.
            // It is selected name evidence, never a fabricated canonical Name.
            try ([kAXIdentifierAttribute, kAXSubroleAttribute] +
                 (selected.contains("accessibility_name") ? [kAXTitleAttribute] : [])).map { name in
                try json.object(["namespace", "name", "property"]) {
                    ["namespace": try json.scalar("macos.ax"), "name": try json.scalar(name),
                     "property": try json.object(["selection", "field", "sensitivity", "evidence", "state"]) {
                        ["selection": try json.scalar("requested"), "field": try json.scalar(name == kAXTitleAttribute ? "accessibility_name" : "description"),
                         "sensitivity": try json.scalar("public"),
                         "evidence": try json.evidence(observationID, "macos.ax", "bounded_public_ax_attributes"),
                         "state": try typed(name == kAXTitleAttribute ? title : identity[name], expected: "text")]
                     }]
                }
            }
        }
        nodes.append(try json.object(["key", "surface", "native_role", "properties", "children", "extensions", "source_declarations"]) {
            ["key": try key(index), "surface": try json.borrowed(surface),
             "native_role": try typed(identity[kAXRoleAttribute], expected: "text"), "properties": properties,
             "children": try json.array { [Any]() }, "extensions": extensions,
             "source_declarations": try json.array { [Any]() }]
        })
        var childIndices: [Int] = []
        let capacity = depths[index] + 1 < maxDepth ? max(0, maxNodes - handles.count) : 0
        do {
            omitted = try nativeAdd(omitted, nativeAXElements(el, kAXChildrenAttribute, windows: false,
                capacity: capacity, admission: admission, access: access) { child in
                    if let existing = handles.firstIndex(where: { CFEqual($0, child) }) {
                        duplicateReferences += 1
                        if !childIndices.contains(existing) { childIndices.append(existing) }
                    } else {
                        childIndices.append(handles.count); handles.append(child); depths.append(depths[index] + 1)
                    }
                })
        } catch NativeAcquisitionError.expired { throw NativeAcquisitionError.expired }
        catch { unknownChildLists += 1 }
        edges.append(childIndices)
    }
    var omittedReferences = 0
    for index in nodes.indices {
        omittedReferences += edges[index].filter { $0 >= nodes.count }.count
        // Initial empty array construction was charged too; cumulative work is explicit.
        nodes[index]["children"] = try json.array { try edges[index].filter { $0 < nodes.count }.map(key) }
    }
    var metrics: [String: Any] = admission.metrics
    metrics.merge(["visited_unique_nodes": nodes.count, "returned_nodes": nodes.count,
        "discovered_unique_handles": handles.count, "duplicate_handle_references": duplicateReferences,
        "known_unread_child_entries": omitted, "unreturned_child_references": omittedReferences,
        "unknown_children_lists": unknownChildLists, "remaining_queued_handles": handles.count - nodes.count,
        "maximum_observed_depth_zero_based": depths.prefix(nodes.count).max() ?? 0,
        "property_availability_counts": availabilityCounts, "coverage": "partial",
        "unexposed_native_or_visual_nodes": "unknown", "node_ceiling": maxNodes, "depth_ceiling": maxDepth,
        "identifier_policy": "observation_local_aliases_of_distinct_AX_handles_not_action_refs"], uniquingKeysWith: { _, new in new })
    return WindowAXResult(nodes: nodes, metrics: metrics)
}
