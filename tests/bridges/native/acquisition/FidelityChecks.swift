import Foundation
import ApplicationServices
import Darwin

// Replay original F02 raw facts through the production collector. No AX/CG/SCK
// call occurs: only the NativeAXAccess boundary receives recorded CF values.
@main struct FidelityChecks {
    @MainActor static func main() throws {
        guard CommandLine.arguments.count == 4 else { exit(2) }
        let limits = try JSONDecoder().decode(NativeAcquisitionLimits.self,
            from: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1])))
        let raw = try JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[2]))) as! [String: Any]
        let original = (raw["external_semantics"] as! [String: Any])["nodes"] as! [[String: Any]]
        let output = URL(fileURLWithPath: CommandLine.arguments[3], isDirectory: true)
        let fields = ["role", "accessibility_name", "description", "value", "placeholder", "enabled", "focused", "actions", "accessibility_bounds"]
        let surface: [String: Any] = ["id": "recorded-window", "generation": "recorded-generation"]
        var assertions = 0
        var emptyTitleCopiedUTF8: Int?
        func check(_ condition: Bool) { precondition(condition, "F02 fidelity"); assertions += 1 }
        func errorValue(_ code: Int32) -> CFTypeRef {
            var error = AXError(rawValue: code)!
            return AXValueCreate(.axError, &error)!
        }
        func cfValue(_ name: String, _ property: [String: Any]?) -> Any {
            guard let property else { return errorValue(AXError.attributeUnsupported.rawValue) }
            guard property["availability"] as? String == "known" else {
                return errorValue((property["ax_error"] as? NSNumber)?.int32Value ?? AXError.noValue.rawValue)
            }
            let value = property["value"]!
            if name == kAXPositionAttribute {
                let xy = value as! [String: Double]; var point = CGPoint(x: xy["x"]!, y: xy["y"]!)
                return AXValueCreate(.cgPoint, &point)!
            }
            if name == kAXSizeAttribute {
                let wh = value as! [String: Double]; var size = CGSize(width: wh["width"]!, height: wh["height"]!)
                return AXValueCreate(.cgSize, &size)!
            }
            return value
        }
        var children = [Int: [NSNumber]]()
        for (index, node) in original.enumerated() where index > 0 {
            children[node["parent_index"] as! Int, default: []].append(NSNumber(value: index))
        }
        for mode in ["recorded", "empty", "unsupported", "unknown", "oversized", "wrong-type", "unselected"] {
            var schedule = [[String]](), secureValueReads = 0
            let access = NativeAXAccess(prepare: { _ in }, attribute: { _, _ in (.noValue, nil) },
                count: { handle, _ in (.success, children[(handle as! NSNumber).intValue, default: []].count) },
                page: { handle, _, offset, count in
                    let list = children[(handle as! NSNumber).intValue, default: []]
                    return (.success, Array(list[offset..<min(list.count, offset + count)]) as CFArray)
                }, batch: { handle, names in
                    schedule.append(names)
                    let props = original[(handle as! NSNumber).intValue]["properties"] as! [String: [String: Any]]
                    if (props[kAXIdentifierAttribute]?["value"] as? String) == "f02.secret", names.contains(kAXValueAttribute) { secureValueReads += 1 }
                    return (.success, names.map { name -> Any in
                        if name == kAXTitleAttribute {
                            switch mode {
                            case "empty": return ""
                            case "unsupported": return errorValue(AXError.attributeUnsupported.rawValue)
                            case "unknown": return errorValue(AXError.cannotComplete.rawValue)
                            case "oversized": return String(repeating: "x", count: 4097)
                            case "wrong-type": return NSNumber(value: 7)
                            default: break
                            }
                        }
                        return cfValue(name, props[name])
                    } as CFArray)
                }, actions: { handle in
                    let node = original[(handle as! NSNumber).intValue]
                    return (AXError(rawValue: (node["actions_error"] as! NSNumber).int32Value)!, (node["actions"] as! [String]) as CFArray)
                }, isElement: { CFGetTypeID($0) == CFNumberGetTypeID() })
            let selected = mode == "unselected" ? ["role", "description"] : fields
            let admission = try NativeAcquisition(limits, deadline: ProcessInfo.processInfo.systemUptime + 10)
            let json = NativeJSON(limits)
            let result = try collectWindowAX(NSNumber(value: 0), surface: surface, observationID: "recorded-f02",
                maxNodes: 160, maxDepth: 9, deadline: admission.deadline, admission: admission,
                fields: selected, json: json, access: access)
            check(result.nodes.count == original.count && result.metrics["coverage"] as? String == "partial")
            check(secureValueReads == 0 && schedule.allSatisfy { $0.count <= 8 })
            check(schedule.flatMap { $0 }.contains(kAXTitleAttribute) == (mode != "unselected"))
            check(schedule.filter { $0.contains(kAXTitleAttribute) }.allSatisfy { $0 == [kAXTitleAttribute] })
            if mode == "empty" { emptyTitleCopiedUTF8 = admission.actualUTF8 }
            if mode == "oversized" {
                // Rejected Title contributes no copied bytes; all sibling text
                // still has exactly the same copy accounting as empty Title.
                check(admission.actualUTF8 == emptyTitleCopiedUTF8)
                check(admission.copiedUTF8 == admission.actualUTF8)
                check(result.metrics["refused_values"] as? Int == original.count)
            }
            for node in result.nodes {
                let properties = node["properties"] as! [[String: Any]]
                check(properties.map { $0["field"] as! String } == selected)
                check(!properties.contains { $0["field"] as? String == "name" })
                let extensions = node["extensions"] as! [[String: Any]]
                let title = extensions.first { $0["name"] as? String == kAXTitleAttribute }
                if mode == "unselected" { check(title == nil); continue }
                let property = title!["property"] as! [String: Any]
                let state = property["state"] as! [String: Any]
                check(property["field"] as? String == "accessibility_name")
                let evidence = property["evidence"] as! [String: Any]
                check(evidence["provenance"] as? String == "reported")
                check(evidence["source_namespace"] as? String == "macos.ax" && evidence["observation_id"] as? String == "recorded-f02")
                if mode == "empty" {
                    check(state["availability"] as? String == "known")
                    check((state["value"] as! [String: Any])["value"] as? String == "")
                } else if mode != "recorded" {
                    check(state["availability"] as? String == (mode == "unsupported" ? "unsupported" : "unknown"))
                    check(state["value"] == nil)
                }
            }
            let context: [String: Any] = ["schema_version": "0.1.0", "session_id": "recorded-session",
                "target": ["id": "recorded-target", "generation": "recorded-generation"], "surfaces": [surface],
                "scope_id": "recorded-f02", "projection": "interaction", "fields": selected,
                "plugin": ["id": "macos", "version": "0.1.0"], "environment_revision": "recorded-boundary-replay"]
            let snapshot = try Collector.snapshot(context: context, surface: surface,
                target: context["target"] as! [String: Any], scope: "recorded-f02", fields: selected,
                observation: "recorded-f02", channel: "external_semantics", started: ProcessInfo.processInfo.systemUptime,
                nodes: result.nodes, captures: try json.array { [] }, json: json)
            let document = try json.envelope("snapshot") { snapshot }
            let frame = try NativeJSONFrame(capacity: 524288, deadline: admission.deadline)
            try frame.encode(document)
            try frame.bytes { try Data($0).write(to: output.appendingPathComponent(mode + ".json"), options: .withoutOverwriting) }
        }
        print("{\"cases\":7,\"assertions\":\(assertions),\"live_ax_sck\":false}")
    }
}
