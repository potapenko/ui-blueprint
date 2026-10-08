import Foundation
import ApplicationServices

// Q01's independent parent -> trigger -> popover graph, extended with real
// AXRole boundaries and an allowed parent result. No SDK calls or input.
@main struct PopupMembershipChecks {
    @MainActor static func main() throws {
        let limits = try JSONDecoder().decode(NativeAcquisitionLimits.self,
            from: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1])))
        var ids = [0: "parent", 1: "f02.popup", 2: "popover", 3: "f02.popup.confirm",
                   4: "f02.popup.owner.a", 5: "f02.result", 6: "parent-group",
                   8: "new-popover", 9: "f02.result"]
        var roles = [0: kAXWindowRole, 1: kAXButtonRole, 2: kAXPopoverRole,
                     3: kAXButtonRole, 4: kAXStaticTextRole, 5: kAXStaticTextRole,
                     6: kAXGroupRole, 8: kAXPopoverRole, 9: kAXStaticTextRole]
        var children = [0: [1, 6], 1: [2], 2: [3, 4], 6: [5]]
        var identifierReads = [Int](), childReads = [Int]()
        let access = NativeAXAccess(prepare: { _ in }, attribute: { raw, name in
            let id = (raw as! NSNumber).intValue
            if name == kAXRoleAttribute {
                return roles[id].map { (.success, $0 as CFString) } ?? (.noValue, nil)
            }
            guard name == kAXIdentifierAttribute else { fatalError("unexpected attribute") }
            identifierReads.append(id)
            return (.success, ids[id]! as CFString)
        }, count: { raw, _ in
            let id = (raw as! NSNumber).intValue; childReads.append(id)
            return (.success, children[id, default: []].count)
        }, page: { raw, _, offset, count in
            (.success, children[(raw as! NSNumber).intValue, default: []][offset..<(offset + count)]
                .map { NSNumber(value: $0) } as CFArray)
        }, batch: { _, _ in fatalError("no value read") }, actions: { _ in fatalError("no action") },
        isElement: { CFGetTypeID($0) == CFNumberGetTypeID() })
        var checks = 0
        func resolve(_ root: Int, _ names: [String]) throws -> [CFTypeRef] {
            identifierReads = []; childReads = []
            return try NativeFormSession.resolve(NSNumber(value: root), ids: names,
                admission: NativeAcquisition(limits, deadline: ProcessInfo.processInfo.systemUptime + 2), access: access)
        }
        func reject(_ names: [String]) throws {
            do { _ = try resolve(0, names); throw Failure.acceptedForeignSurface }
            catch NativeAcquisitionError.invalidValue { checks += 1 }
        }
        let popup = try resolve(2, ["f02.popup.confirm"])
        precondition(CFEqual(popup[0], NSNumber(value: 3))); checks += 1
        // Both the same actor and a different popup child must refuse as parent.
        try reject(["f02.popup", "f02.popup.confirm"])
        try reject(["f02.popup", "f02.popup.owner.a"])
        try reject(["f02.popup.confirm"]) // result-only lookup has the same boundary
        // Equal identifiers on genuinely distinct Surfaces remain legal. The
        // resolver must establish membership, not impose disjoint string lists.
        ids[5] = "f02.popup.confirm"
        let sameName = try resolve(0, ["f02.popup", "f02.popup.confirm"])
        precondition(CFEqual(sameName[1], NSNumber(value: 5)) && !CFEqual(popup[0], sameName[1])); checks += 1
        ids[5] = "f02.result"
        let parent = try resolve(0, ["f02.popup", "f02.result"])
        precondition(CFEqual(parent[0], NSNumber(value: 1)) && CFEqual(parent[1], NSNumber(value: 5))); checks += 1
        precondition(!identifierReads.contains(2) && !identifierReads.contains(3) && !identifierReads.contains(4)
            && !childReads.contains(2)); checks += 1
        // Full revalidation and phase3 use this same resolver. Closing the
        // popup does not prevent reading the original exact parent result.
        children[1] = []
        let after = try resolve(0, ["f02.result"])
        precondition(CFEqual(parent[1], after[0]) && !identifierReads.contains(2)); checks += 1
        // A new popup's same-identifier result must never repair a parent ref.
        children[1] = [8]; children[8] = [9]; children[6] = []
        try reject(["f02.result"])
        // Even the old held CF object is no longer a parent result if it moves
        // into the popup. Surface membership is independent of CF equality.
        children[1] = [2]; children[2] = [3, 4, 5]
        try reject(["f02.result"])
        children[6] = [5]; children[2] = [3, 4]
        let revalidated = try resolve(0, ["f02.popup", "f02.result"])
        precondition(zip(parent, revalidated).allSatisfy { CFEqual($0, $1) }); checks += 1
        // An unclassified container cannot prove an authorized Surface boundary.
        roles.removeValue(forKey: 6)
        try reject(["f02.result"])
        print("{\"membership_checks\":\(checks),\"value_reads\":0,\"sdk_input\":false}")
    }
    enum Failure: Error { case acceptedForeignSurface }
}
