import Foundation
import ApplicationServices

@main struct HeldActionChecks {
    @MainActor static func main() throws {
        guard CommandLine.arguments.count == 2 else { exit(2) }
        let profile = try JSONDecoder().decode(NativeAcquisitionLimits.self,
            from: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1])))
        var checks = 0, windowID = 1, targetID = 2, resultID = 3, generationCurrent = true
        var calls = 0
        func admission() throws -> NativeAcquisition {
            try NativeAcquisition(profile, deadline: ProcessInfo.processInfo.systemUptime + 2)
        }
        func expect(_ b: Bool) { precondition(b); checks += 1 }
        func refuses(_ body: () throws -> Void) {
            do { try body(); preconditionFailure("expected refusal") } catch { checks += 1 }
        }
        let access = NativeAXAccess(prepare: { _ in }, attribute: { raw, name in
            let id = (raw as! NSNumber).intValue
            if name == kAXIdentifierAttribute {
                let text = id == windowID ? "a" : id == targetID ? "f02.sample.a" : id == resultID ? "f02.count" : "detached"
                return (.success, text as CFString)
            }
            return (.noValue, nil)
        }, count: { raw, name in
            if name == kAXWindowsAttribute { return (.success, 1) }
            return (.success, (raw as! NSNumber).intValue == windowID ? 2 : 0)
        }, page: { raw, name, index, count in
            let values = name == kAXWindowsAttribute ? [windowID] : (raw as! NSNumber).intValue == windowID ? [targetID, resultID] : []
            return (.success, Array(values[index..<min(values.count,index+count)]).map { NSNumber(value:$0) } as CFArray)
        }, batch: { raw, names in
            calls += 1
            let id = (raw as! NSNumber).intValue
            return (.success, names.map { name -> Any in
                switch name {
                case kAXRoleAttribute: return id == targetID ? "AXButton" : "AXStaticText"
                case kAXSubroleAttribute: return ""
                case kAXIdentifierAttribute: return id == targetID ? "f02.sample.a" : "f02.count"
                case kAXEnabledAttribute: return NSNumber(value:true)
                case kAXValueAttribute: return "Count: 0"
                default: return NSNull()
                }
            } as CFArray)
        }, actions: { _ in (.success, ["AXPress"] as CFArray) }, isElement: { CFGetTypeID($0) == CFNumberGetTypeID() })
        let targetKey = ["namespace":"macos.ax","key":"session-target"]
        let resultKey = ["namespace":"macos.ax","key":"session-result"]
        func owner() throws -> NativeHeldAction {
            try NativeHeldAction(application:NSNumber(value:0),windowIdentifier:"a",
                targetIdentifier:"f02.sample.a",resultIdentifier:"f02.count",
                targetKey:targetKey,resultKey:resultKey,maxNodes:160,maxDepth:9,admission:admission(),access:access,current:{
                    guard generationCurrent else { throw NativeAcquisitionError.invalidValue }
                })
        }
        let held = try owner(), surface:[String:Any] = ["id":"window-1","generation":"g1"]
        let a = try held.read(.target,surface:surface,observation:"before",fields:["role","enabled","actions"],admission:admission(),json:NativeJSON(profile))
        expect((a["key"] as! NSDictionary).isEqual(targetKey))
        let b = try held.read(.result,surface:surface,observation:"after",fields:["role","value"],admission:admission(),json:NativeJSON(profile))
        expect((b["key"] as! NSDictionary).isEqual(resultKey))
        expect((b["properties"] as! [[String:Any]]).contains { $0["field"] as? String == "value" })
        let before = calls
        targetID = 4
        refuses { _ = try held.read(.target,surface:surface,observation:"remount",fields:["enabled"],admission:admission(),json:NativeJSON(profile)) }
        expect(calls == before)
        targetID = 2; resultID = 5
        refuses { try held.validate(admission:admission()) }
        resultID = 3; windowID = 10
        refuses { try held.validate(admission:admission()) }
        windowID = 1; generationCurrent = false
        refuses { try held.validate(admission:admission()) }
        generationCurrent = true; held.invalidate()
        refuses { try held.validate(admission:admission()) }
        print("{\"checks\":\(checks),\"held_target_result\":true,\"mutation_calls\":0,\"live_sdk\":false}")
    }
}
