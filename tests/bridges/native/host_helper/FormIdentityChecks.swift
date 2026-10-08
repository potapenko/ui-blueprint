import Foundation
import ApplicationServices
@main struct FormIdentityChecks {
    @MainActor static func main() throws {
        let limits = try JSONDecoder().decode(NativeAcquisitionLimits.self, from: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1])))
        var duplicate = false
        let access = NativeAXAccess(prepare: { _ in }, attribute: { raw, name in
            guard name == kAXIdentifierAttribute else { return (.noValue, nil) }
            let id = (raw as! NSNumber).intValue
            return (.success, (id == 1 || (id == 3 && duplicate) ? "target" : id == 2 ? "result" : "other") as CFString)
        }, count: { raw, _ in (.success, (raw as! NSNumber).intValue == 0 ? 3 : 0) },
        page: { _, _, offset, count in (.success, Array([1,2,3][offset..<(offset+count)]).map { NSNumber(value:$0) } as CFArray) },
        batch: { _, _ in fatalError("identity lookup must not read values") }, actions: { _ in fatalError("no action lookup") },
        isElement: { CFGetTypeID($0) == CFNumberGetTypeID() })
        func resolve() throws -> [CFTypeRef] {
            try NativeFormSession.resolve(NSNumber(value:0), ids:["target","result"], admission: NativeAcquisition(limits, deadline:ProcessInfo.processInfo.systemUptime+1), access:access)
        }
        let held = try resolve()
        precondition(held.count == 2 && CFEqual(held[0], NSNumber(value:1)) && CFEqual(held[1], NSNumber(value:2)))
        duplicate = true
        do { _ = try resolve(); fatalError("duplicate identity accepted") }
        catch NativeFormFailure.ambiguousTarget { }
        print("{\"unique_binding\":true,\"ambiguous_refusal\":true,\"value_reads\":0,\"sdk_input\":false}")
    }
}
