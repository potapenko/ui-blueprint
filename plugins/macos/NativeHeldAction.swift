import Foundation
import ApplicationServices

// Two held SDK objects for the first Native action: target and independently
// selected Expectation result. Canonical refs are worker-owned; this is not a graph.
// The session helper must keep this owner alive, and invalidate it before reap.
@MainActor final class NativeHeldAction {
    enum Slot { case target, result }
    let targetKey: [String: String]
    let resultKey: [String: String]
    private let target: CFTypeRef
    private let result: CFTypeRef
    private let window: CFTypeRef
    private let application: CFTypeRef
    private let windowIdentifier: String
    private let targetIdentifier: String
    private let resultIdentifier: String
    private let access: NativeAXAccess
    private let current: () throws -> Void
    private let maxNodes: Int
    private let maxDepth: Int
    private var live = true

    init(application: CFTypeRef, windowIdentifier: String,
         targetIdentifier: String, resultIdentifier: String,
         targetKey: [String: String], resultKey: [String: String],
         maxNodes: Int, maxDepth: Int, admission: NativeAcquisition, access: NativeAXAccess = .live,
         current: @escaping () throws -> Void) throws {
        guard (2...160).contains(maxNodes), (1...9).contains(maxDepth), admission.limits.ax_windows > 0,
              targetIdentifier != resultIdentifier,
              targetKey != resultKey,
              [targetKey, resultKey].allSatisfy({
                  Set($0.keys) == ["namespace", "key"] && $0["namespace"] == "macos.ax"
                      && !($0["key"]?.isEmpty ?? true)
              }) else { throw NativeAcquisitionError.invalidValue }
        try current(); try admission.check()
        let window = try Collector.resolveWindow(application, identifier: windowIdentifier,
            admission: admission, access: access)
        let target = try Collector.resolveElement(window, identifier: targetIdentifier,
            maxNodes: maxNodes, maxDepth: maxDepth, admission: admission, access: access)
        let result = try Collector.resolveElement(window, identifier: resultIdentifier,
            maxNodes: maxNodes, maxDepth: maxDepth, admission: admission, access: access)
        guard !CFEqual(target, result) else { throw NativeAcquisitionError.invalidValue }
        try current(); try admission.check()
        self.application = application; self.window = window
        self.target = target; self.result = result
        self.windowIdentifier = windowIdentifier
        self.targetIdentifier = targetIdentifier; self.resultIdentifier = resultIdentifier
        self.targetKey = targetKey; self.resultKey = resultKey
        self.access = access; self.current = current
        self.maxNodes = maxNodes; self.maxDepth = maxDepth
    }

    // Identifier lookup establishes unique current connectivity only. It never
    // replaces the stored object: remount with the same identifier must refuse.
    func validate(admission: NativeAcquisition) throws {
        guard live else { throw NativeAcquisitionError.invalidValue }
        try current(); try admission.check()
        let freshWindow = try Collector.resolveWindow(application, identifier: windowIdentifier,
            admission: admission, access: access)
        guard CFEqual(window, freshWindow) else { throw NativeAcquisitionError.invalidValue }
        for (identifier, held) in [(targetIdentifier, target), (resultIdentifier, result)] {
            let found = try Collector.resolveElement(window, identifier: identifier,
                maxNodes: maxNodes, maxDepth: maxDepth, admission: admission, access: access)
            guard CFEqual(found, held) else { throw NativeAcquisitionError.invalidValue }
        }
        try current(); try admission.check()
    }

    func read(_ slot: Slot, surface: [String: Any], observation: String,
              fields: [String], admission: NativeAcquisition, json: NativeJSON) throws -> [String: Any] {
        try validate(admission: admission)
        let element = slot == .target ? target : result
        let key = slot == .target ? targetKey : resultKey
        let collected = try collectWindowAX(element, surface: surface, observationID: observation,
            maxNodes: 1, maxDepth: 1, deadline: admission.deadline, admission: admission,
            fields: fields, json: json, access: access)
        guard collected.nodes.count == 1 else { throw NativeAcquisitionError.invalidValue }
        var node = collected.nodes[0]
        node["key"] = try json.borrowed(key)
        // Scope is the exact held node, not its unrequested descendants.
        node["children"] = try json.array { [Any]() }
        try validate(admission: admission)
        return node
    }

    func invalidate() { live = false }
}
