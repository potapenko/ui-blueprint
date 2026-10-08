import Foundation
import ApplicationServices
import Darwin

@main struct FormChecks {
    @MainActor static func main() throws {
        guard CommandLine.arguments.count == 3 else { exit(2) }
        let limits = try JSONDecoder().decode(NativeAcquisitionLimits.self,
            from: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1])))
        let output = URL(fileURLWithPath: CommandLine.arguments[2], isDirectory: true)
        let fields = ["role", "accessibility_name", "placeholder", "focused", "enabled"]
        let oldFields = ["role", "description", "value", "placeholder", "enabled", "focused", "actions", "accessibility_bounds"]
        var assertions = 0
        func check(_ value: Bool) { precondition(value, "form property control"); assertions += 1 }
        for mode in ["known_false_empty", "known_true", "unsupported", "unknown", "secure"] {
            var schedule: [[String]] = [], actionCalls = 0
            let access = NativeAXAccess(prepare: { _ in }, attribute: { _, _ in (.noValue, nil) },
                count: { _, _ in (.success, 0) }, page: { _, _, _, _ in (.success, [] as CFArray) },
                batch: { _, names in
                    schedule.append(names)
                    let identity = names.contains(kAXRoleAttribute)
                    if !identity && mode == "unsupported" { return (.attributeUnsupported, nil) }
                    if !identity && mode == "unknown" { return (.cannotComplete, nil) }
                    return (.success, names.map { name -> Any in
                        switch name {
                        case kAXRoleAttribute: return "AXTextField"
                        case kAXSubroleAttribute: return mode == "secure" ? "AXSecureTextField" : ""
                        case kAXIdentifierAttribute: return mode == "secure" ? "f02.secret" : "f02.name"
                        case kAXDescriptionAttribute: return "Name field"
                        case kAXPlaceholderValueAttribute: return mode == "known_false_empty" ? "" : "Name"
                        case kAXFocusedAttribute, kAXEnabledAttribute: return NSNumber(value: mode == "known_true")
                        case kAXValueAttribute: return "not_for_new_form_request"
                        default: return NSNull()
                        }
                    } as CFArray)
                }, actions: { _ in actionCalls += 1; return (.success, [] as CFArray) },
                isElement: { CFGetTypeID($0) == CFNumberGetTypeID() })
            let admission = try NativeAcquisition(limits, deadline: ProcessInfo.processInfo.systemUptime + 5)
            let json = NativeJSON(limits)
            let result = try collectWindowAX(NSNumber(value: 1), surface: ["id": "window-1", "generation": "g1"],
                observationID: "form-control", maxNodes: 160, maxDepth: 9, deadline: admission.deadline,
                admission: admission, fields: fields, json: json, access: access)
            let props = result.nodes[0]["properties"] as! [[String: Any]]
            check(props.map { $0["field"] as! String } == fields)
            check(schedule.count == 3 && schedule.last == [kAXTitleAttribute]
                && !schedule.flatMap { $0 }.contains(kAXValueAttribute) && actionCalls == 0)
            check(!schedule.flatMap { $0 }.contains(kAXPositionAttribute))
            let states = Dictionary(uniqueKeysWithValues: props.map { ($0["field"] as! String, $0["state"] as! [String: Any]) })
            if mode == "unsupported" || mode == "unknown" {
                let status = mode == "unsupported" ? "unsupported" : "unknown"
                for name in ["accessibility_name", "placeholder", "focused", "enabled"] { check(states[name]!["availability"] as? String == status && states[name]!["value"] == nil) }
            } else {
                check((states["placeholder"]!["value"] as! [String: Any])["value"] as? String == (mode == "known_false_empty" ? "" : "Name"))
                check((states["focused"]!["value"] as! [String: Any])["value"] as? Bool == (mode == "known_true"))
                check((states["enabled"]!["value"] as! [String: Any])["value"] as? Bool == (mode == "known_true"))
                check((states["accessibility_name"]!["value"] as! [String: Any])["value"] as? String == "Name field")
            }
            // Global focus is still not requested, independent of per-node focused.
            let context: [String: Any] = ["schema_version":"0.1.0", "session_id":"form-session", "target":["id":"form-target","generation":"g1"],
                "surfaces":[["id":"window-1","generation":"g1"]], "scope_id":"form-scope", "projection":"interaction", "fields":fields,
                "plugin":["id":"macos","version":"0.1.0"], "environment_revision":"e1"]
            let snapshot = try Collector.snapshot(context: context, surface: ["id":"window-1","generation":"g1"],
                target: ["id":"form-target","generation":"g1"], scope:"form-scope", fields:fields, observation:"form-control",
                channel:"external_semantics", started:ProcessInfo.processInfo.systemUptime, nodes:result.nodes,
                captures:try json.array { [] }, json:json)
            check(((snapshot["focus"] as! [String:Any])["keyboard"] as! [String:Any])["status"] as? String == "not_requested")
            let document = try json.envelope("snapshot") { snapshot }
            let frame = try NativeJSONFrame(capacity:512*1024,deadline:admission.deadline);try frame.encode(document)
            try frame.bytes { try Data($0).write(to:output.appendingPathComponent(mode+".json"),options:.withoutOverwriting) }
            // Existing eight-field request still includes its original field order.
            let old = try collectWindowAX(NSNumber(value:1),surface:["id":"window-1","generation":"g1"],observationID:"old-control",
                maxNodes:160,maxDepth:9,deadline:admission.deadline,admission:admission,fields:oldFields,json:NativeJSON(limits),access:access)
            check((old.nodes[0]["properties"] as! [[String:Any]]).map{$0["field"] as! String} == oldFields)
        }
        print("{\"assertions\":\(assertions),\"cases\":5,\"live_ax_sck\":false}")
    }
}
