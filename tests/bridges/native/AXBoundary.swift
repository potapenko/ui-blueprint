// Finite Q02 diagnostic only. Metadata/codes, no UI values or raw pointers.
import Foundation
import ApplicationServices
@MainActor enum Q02Boundary {
    static var node = -1
    static var rows: [[String: Any]] = []
    static var parentChild: CFTypeRef?
    static var truncated = false
    static func reset() { node = -1; rows = []; parentChild = nil; truncated = false }
    // Test copy calls this before returning from collectWindowAX. The extra
    // reference never outlives the collector's own handles, including warm calls.
    static func release() { parentChild = nil; node = -1 }
    static func proof(observation: String, surface: [String: Any]) -> [String: Any] {
        ["method":"N05-AX-BOUNDARY@2", "observation_id":observation,
         "surface":surface, "complete":true, "truncated":truncated, "rows":rows]
    }
    static func add(_ operation: String, _ values: [String: Any]) {
        guard [28,70].contains(node) else { return }
        guard rows.count < 64 else { truncated = true; return }
        rows.append(["order":rows.count,"node_alias":node,"operation":operation,
            "uptime":ProcessInfo.processInfo.systemUptime].merging(values,uniquingKeysWith:{$1}))
    }
    static func bind(_ index: Int, _ handle: CFTypeRef) {
        node = index
        if index == 70 { add("visit",["same_as_parent28_child":parentChild.map{CFEqual($0,handle)} ?? false]) }
    }
    static func edge(_ alias: Int, _ child: CFTypeRef) {
        if node == 28 {parentChild = child;add("child_alias",["child_alias":alias])}
    }
    static func wrap(_ source: NativeAXAccess) -> NativeAXAccess {
        NativeAXAccess(prepare:source.prepare,attribute:{handle,name in
            let result=source.attribute(handle,name);add("attribute",["attribute":name,"status":result.0.rawValue]);return result
        },count:{handle,name in
            let result=source.count(handle,name);add("count",["attribute":name,"status":result.0.rawValue,"count":result.1]);return result
        },page:{handle,name,offset,amount in
            let result=source.page(handle,name,offset,amount);add("range",["attribute":name,"offset":offset,"asked":amount,"status":result.0.rawValue,"returned":result.1.map{CFArrayGetCount($0)} ?? -1]);return result
        },batch:{handle,names in
            let result=source.batch(handle,names);var errors:[Int32]=[]
            if [28,70].contains(node),let array=result.1,CFArrayGetCount(array)<=8 {
                for i in 0..<CFArrayGetCount(array) {
                    let value=NativeAcquisition.item(array,i);var code:Int32=0
                    if CFGetTypeID(value)==AXValueGetTypeID(){let ax=unsafeDowncast(value,to:AXValue.self);if AXValueGetType(ax) == .axError {var error=AXError.failure;AXValueGetValue(ax,.axError,&error);code=error.rawValue}}
                    errors.append(code)
                }
            }
            add("batch",["attributes":names,"status":result.0.rawValue,"entry_errors":errors]);return result
        },actions:{handle in
            let result=source.actions(handle);add("actions",["status":result.0.rawValue]);return result
        },isElement:source.isElement)
    }
}
