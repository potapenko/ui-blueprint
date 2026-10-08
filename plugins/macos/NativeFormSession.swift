import Foundation
import AppKit
import ApplicationServices
import Darwin

// Explicit fixture-only resident AX owner. No input is emitted from Observe or
// resolve. The parent broker authenticates the delivery nonce before forwarding.
enum NativeFormFailure: Error { case staleTarget, ambiguousTarget, unsupported, inputOwner, privateValue, focusUnavailable, keyboardUnavailable, setterUnavailable }

@MainActor final class NativeFormSession {
    let config: NativeConfiguration
    let configurationBytes: Data
    let epoch: UInt64
    let serial: UInt64
    let generation = UUID().uuidString
    let end: Double
    let application: AXUIElement
    let window: CFTypeRef
    let identifiers: [String]
    let handles: [CFTypeRef]
    var order: NativeFormOrder
    var inputOwner: pid_t?
    var ownerWindow: CFTypeRef?

    static func run(io: NativeDescriptorIO, first: NativeInbound) throws {
        let watchdog = DispatchSource.makeTimerSource(queue: .global())
        watchdog.setEventHandler { @Sendable in _exit(124) }
        watchdog.schedule(deadline: .now() + max(0, first.deadline - NativeDescriptorIO.now))
        watchdog.resume()
        let parent = DispatchSource.makeTimerSource(queue: .global())
        let fd = io.input
        parent.setEventHandler { @Sendable in
            var interest = pollfd(fd: fd, events: Int16(POLLIN), revents: 0)
            if poll(&interest, 1, 0) > 0 && interest.revents & Int16(POLLHUP | POLLERR | POLLNVAL) != 0 { _exit(125) }
        }
        parent.schedule(deadline: .now(), repeating: .milliseconds(10)); parent.resume()
        defer { parent.cancel(); watchdog.cancel() }
        let session = try NativeFormSession(first)
        var input = first
        while true {
            watchdog.schedule(deadline: .now() + max(0, input.deadline - NativeDescriptorIO.now))
            try session.process(input, io: io)
            watchdog.schedule(deadline: .now() + max(0, session.end - NativeDescriptorIO.now))
            input = try NativeHostProtocol.receiveInput(io, until: session.end)
        }
    }
    init(_ input: NativeInbound) throws {
        config = try NativeConfiguration.decode(input.configurationBytes)
        guard config.collection == "form", input.control.channel == 0,
              input.control.operationClass == 8, input.control.flags == 128,
              let ids = config.form_identifiers, let duration = config.form_session_ms,
              let serial = input.documents.last?["helper_serial"] as? UInt64
        else { throw NativeProtocolError.configuration }
        configurationBytes = input.configurationBytes
        epoch = input.control.epoch; self.serial = serial
        order = NativeFormOrder(epoch: input.control.epoch, serial: serial)
        end = input.started + Double(duration) / 1000
        identifiers = ids
        guard AXIsProcessTrusted() else { throw NativeProtocolError.configuration }
        var manifest = config.binding.manifest; manifest["identity_path"] = config.identity_path
        try NativeCurrentIdentity.verify(path: config.identity_path, expected: manifest)
        let admission = try NativeAcquisition(config.acquisition_limits, deadline: input.deadline)
        application = AXUIElementCreateApplication(config.binding.pid)
        window = try Collector.resolveWindow(application, identifier: config.binding.window_identifier, admission: admission)
        let selectedWindow = window
        handles = try Self.resolve(selectedWindow, ids: ids, admission: admission)
    }
    func current(_ admission: NativeAcquisition) throws {
        try admission.check()
        guard AXIsProcessTrusted(), let app = NSRunningApplication(processIdentifier: config.binding.pid),
              !app.isTerminated, app.bundleIdentifier == config.binding.bundle_id,
              app.launchDate?.timeIntervalSince1970 == config.binding.launch_time,
              Collector.windowOwnedBy(pid: config.binding.pid, window: config.binding.window_id)
        else { throw NativeFormFailure.staleTarget }
        do { try NativeCurrentIdentity.verify(path: config.identity_path, expected: config.binding.manifest) }
        catch { throw NativeFormFailure.staleTarget }
        guard CFEqual(window, try Collector.resolveWindow(application, identifier: config.binding.window_identifier, admission: admission))
        else { throw NativeFormFailure.staleTarget }
    }
    static func resolve(_ window: CFTypeRef, ids: [String], admission: NativeAcquisition, access: NativeAXAccess = .live) throws -> [CFTypeRef] {
        var queue: [(CFTypeRef, Int)] = [(window, 0)], seen: [CFTypeRef] = []
        var found = [CFTypeRef?](repeating: nil, count: ids.count)
        while !queue.isEmpty && seen.count < 160 {
            let (element, depth) = queue.removeFirst()
            if seen.contains(where: { CFEqual($0, element) }) { continue }
            seen.append(element)
            if let raw = try nativeAXAttribute(element, kAXIdentifierAttribute, admission: admission, access: access),
               let index = ids.firstIndex(of: try admission.text(raw)) {
                guard found[index] == nil else { throw NativeFormFailure.ambiguousTarget }
                found[index] = element
            }
            let capacity = depth < 9 ? max(0, 160 - seen.count - queue.count) : 0
            let omitted = try nativeAXElements(element, kAXChildrenAttribute, windows: false,
                capacity: capacity, admission: admission, access: access) { queue.append(($0, depth + 1)) }
            guard omitted == 0 else { throw NativeAcquisitionError.limit }
        }
        guard queue.isEmpty, found.allSatisfy({ $0 != nil }) else { throw NativeAcquisitionError.invalidValue }
        return found.map { $0! } // every requested identity was proved unique above
    }
    func validateHandles(_ admission: NativeAcquisition) throws {
        let fresh = try Self.resolve(window, ids: identifiers, admission: admission)
        guard zip(handles, fresh).allSatisfy({ CFEqual($0, $1) }) else { throw NativeFormFailure.staleTarget }
    }
    func key(_ index: Int) -> [String: String] {
        ["namespace": "macos.ax", "key": "native-\(generation)-\(epoch)-\(serial)-\(index)"]
    }
    func index(_ key: [String: String]) throws -> Int {
        guard let found = handles.indices.first(where: { self.key($0) == key }) else { throw NativeFormFailure.staleTarget }
        return found
    }
    func attribute(_ handle: CFTypeRef, _ name: String, _ admission: NativeAcquisition) throws -> CFTypeRef? {
        try nativeAXAttribute(handle, name, admission: admission)
    }
    func bool(_ handle: CFTypeRef, _ name: String, _ admission: NativeAcquisition) throws -> Bool {
        guard let value = try attribute(handle, name, admission), CFGetTypeID(value) == CFBooleanGetTypeID() else { return false }
        return CFBooleanGetValue(unsafeDowncast(value, to: CFBoolean.self))
    }
    func currentInputOwner(_ admission: NativeAcquisition) throws -> pid_t? {
        let system = AXUIElementCreateSystemWide()
        guard let focused = try attribute(system, kAXFocusedApplicationAttribute, admission),
              CFGetTypeID(focused) == AXUIElementGetTypeID() else { return nil }
        var pid: pid_t = 0
        guard AXUIElementGetPid(unsafeDowncast(focused, to: AXUIElement.self), &pid) == .success else { return nil }
        return pid
    }
    func snapshot(_ request: [String: Any], _ input: NativeInbound, _ admission: NativeAcquisition, _ json: NativeJSON) throws -> [String: Any] {
        try current(admission)
        guard let context = request["context"] as? [String: Any], let target = context["target"] as? [String: Any],
              let surfaces = context["surfaces"] as? [[String: Any]], surfaces.count == 1,
              target["id"] as? String == "f02-pid-\(config.binding.pid)", target["generation"] as? String == config.binding.target_generation,
              surfaces[0]["id"] as? String == "window-\(config.binding.window_id)", surfaces[0]["generation"] as? String == config.binding.surface_generation,
              context["scope_id"] as? String == config.scope_id,
              let fields = context["fields"] as? [String], fields.contains("enabled"), fields.contains("value"), fields.contains("focused"),
              let limits = request["limits"] as? [String: Any], let count = limits["max_elements"] as? Int, count >= handles.count, count <= 160,
              let depth = limits["max_depth"] as? Int, (1...9).contains(depth),
              let id = request["request_id"] as? String
        else { throw NativeProtocolError.request }
        let observation = "\(id)-native-\(input.control.operation)-\(input.control.flags)"
        try validateHandles(admission)
        var nodes: [[String: Any]] = []
        for index in handles.indices {
            let result = try collectWindowAX(handles[index], surface: surfaces[0], observationID: observation,
                maxNodes: 1, maxDepth: 1, deadline: input.deadline, admission: admission, fields: fields, json: json)
            guard result.nodes.count == 1 else { throw NativeProtocolError.request }
            var node = result.nodes[0]
            node["key"] = try json.borrowed(key(index)); node["children"] = try json.array { [Any]() }
            nodes.append(node)
        }
        var result = try Collector.snapshot(context: context, surface: surfaces[0], target: target, scope: config.scope_id,
            fields: fields, observation: observation, channel: "external_semantics", started: input.started,
            nodes: nodes, captures: [], json: json)
        let focused = try attribute(application, kAXFocusedUIElementAttribute, admission)
        let focusedIndex = try currentInputOwner(admission) == config.binding.pid
            ? focused.flatMap { focused in handles.indices.first(where: { CFEqual(handles[$0], focused) }) } : nil
        var focus = result["focus"] as! [String: Any]
        let evidence = try json.evidence(observation, "macos.ax", "current_keyboard_focus")
        if let index = focusedIndex {
            focus["keyboard"] = try json.borrowed(["status": "known", "target": key(index), "evidence": evidence] as [String: Any])
            if identifiers[index] != "f02.secret", let raw = try attribute(handles[index], kAXSelectedTextRangeAttribute, admission),
               CFGetTypeID(raw) == AXValueGetTypeID(), AXValueGetType(unsafeDowncast(raw, to: AXValue.self)) == .cfRange {
                var range = CFRange()
                if AXValueGetValue(unsafeDowncast(raw, to: AXValue.self), .cfRange, &range), range.location >= 0, range.length >= 0,
                   range.location <= Int.max - range.length {
                    focus["text_selection"] = try json.borrowed(["anchor": range.location, "focus": range.location + range.length, "units": "utf16", "evidence": evidence] as [String: Any])
                }
            }
        } else {
            focus["keyboard"] = try json.borrowed(["status": "unknown", "reason": "focus_outside_selected_controls"])
        }
        result["focus"] = focus
        try current(admission)
        try validateHandles(admission)
        return result
    }
    func action(_ request: [String: Any]) throws -> [String: Any] {
        guard let op = request["operation"] as? [String: Any], let action = op["action"] as? [String: Any] else { throw NativeProtocolError.request }
        return action
    }
    func validateAction(_ action: [String: Any], _ admission: NativeAcquisition, delivery: Bool) throws -> (Int, String) {
        try current(admission)
        guard let reference = action["backend_ref"] as? [String: Any], let key = reference["key"] as? [String: String],
              let intent = action["intent"] as? [String: Any], let name = intent["intent"] as? String else { throw NativeProtocolError.request }
        let i = try index(key), handle = unsafeDowncast(handles[i], to: AXUIElement.self)
        try validateHandles(admission)
        // Sensitive and unclassified inputs are never admitted to delivery.
        let roleRaw = try attribute(handle, kAXRoleAttribute, admission)
        let subroleRaw = try attribute(handle, kAXSubroleAttribute, admission)
        let role = try roleRaw.map { try admission.text($0) }
        let subrole = try subroleRaw.map { try admission.text($0) }
        guard try bool(handle, kAXEnabledAttribute, admission) else { throw NativeFormFailure.unsupported }
        if name != "focus" && (identifiers[i] == "f02.secret" || role == "AXSecureTextField" || subrole == "AXSecureTextField") { throw NativeFormFailure.privateValue }
        let owner = try currentInputOwner(admission)
        let focusedWindow = try attribute(application, kAXFocusedWindowAttribute, admission)
        guard (name == "focus" || owner == config.binding.pid), let focusedWindow, CFEqual(focusedWindow, window) else { throw NativeFormFailure.inputOwner }
        if delivery {
            guard owner == inputOwner, let old = ownerWindow, CFEqual(old, focusedWindow) else { throw NativeFormFailure.inputOwner }
        } else { inputOwner = owner; ownerWindow = focusedWindow }
        switch name {
        case "focus":
            var settable = DarwinBoolean(false)
            guard action["modality"] as? String == "semantic",
                  AXUIElementIsAttributeSettable(handle, kAXFocusedAttribute as CFString, &settable) == .success, settable.boolValue
            else { throw NativeFormFailure.focusUnavailable }
        case "activate":
            var raw: CFArray?
            guard action["modality"] as? String == "semantic", AXUIElementCopyActionNames(handle, &raw) == .success,
                  let raw, try admission.actions(raw).contains(kAXPressAction) else { throw NativeProtocolError.request }
        case "fill":
            var settable = DarwinBoolean(false)
            guard action["modality"] as? String == "setter", role == "AXTextField",
                  let text = intent["text"] as? String, text.utf8.count <= config.acquisition_limits.value_utf8_bytes,
                  !text.unicodeScalars.contains(where: { CharacterSet.controlCharacters.contains($0) }),
                  AXUIElementIsAttributeSettable(handle, kAXValueAttribute as CFString, &settable) == .success, settable.boolValue
            else { throw NativeFormFailure.setterUnavailable }
        case "type":
            guard action["modality"] as? String == "keyboard", role == "AXTextField",
                  let text = intent["text"] as? String, !text.isEmpty, text.utf16.count <= 20,
                  !text.unicodeScalars.contains(where: { CharacterSet.controlCharacters.contains($0) }),
                  let focused = try attribute(application, kAXFocusedUIElementAttribute, admission), CFEqual(focused, handle),
                  try bool(handle, kAXFocusedAttribute, admission) else { throw NativeFormFailure.keyboardUnavailable }
        default: throw NativeProtocolError.request
        }
        return (i, name)
    }
    func process(_ input: NativeInbound, io: NativeDescriptorIO) throws {
        let c = input.control, nextPhase = c.flags & 127
        guard input.configurationBytes == configurationBytes, c.epoch == epoch,
              input.documents.last?["helper_serial"] as? UInt64 == serial, c.channel == 0,
              let artifact = input.document["artifact"] as? [String: Any], artifact["kind"] as? String == "request",
              let request = artifact["data"] as? [String: Any], let limits = request["limits"] as? [String: Any],
              let duration = limits["deadline_ms"] as? Double, duration > 0,
              let output = limits["max_output_bytes"] as? Int, output > 1
        else { throw NativeProtocolError.control }
        try order.accept(c, serial: serial)
        let deadline = min(input.deadline, input.started + duration / 1000, end)
        let admission = try NativeAcquisition(config.acquisition_limits, deadline: deadline)
        let json = NativeJSON(config.acquisition_limits)
        let frame = try NativeJSONFrame(capacity: min(input.replyCap, output), deadline: deadline)
        do {
            if nextPhase == 2 {
                let act = try action(request)
                let (index, name) = try validateAction(act, admission, delivery: true)
                let handle = unsafeDowncast(handles[index], to: AXUIElement.self)
                try admission.check()
                let confirmed: Bool
                if name == "activate" { confirmed = AXUIElementPerformAction(handle, kAXPressAction as CFString) == .success }
                else if name == "focus" {
                    guard let app = NSRunningApplication(processIdentifier: config.binding.pid), app.activate(options: []) else { throw NativeFormFailure.inputOwner }
                    confirmed = AXUIElementSetAttributeValue(handle, kAXFocusedAttribute as CFString, kCFBooleanTrue) == .success
                }
                else if name == "fill" {
                    let intent = act["intent"] as! [String: Any]
                    confirmed = AXUIElementSetAttributeValue(handle, kAXValueAttribute as CFString, intent["text"] as! CFString) == .success
                }
                else {
                    let intent = act["intent"] as! [String: Any], text = intent["text"] as! String
                    guard let source = CGEventSource(stateID: .privateState),
                          let event = CGEvent(keyboardEventSource: source, virtualKey: 0, keyDown: true),
                          let release = CGEvent(keyboardEventSource: source, virtualKey: 0, keyDown: false) else { throw NativeProtocolError.request }
                    let units = Array(text.utf16)
                    event.flags = []; release.flags = []
                    event.keyboardSetUnicodeString(stringLength: units.count, unicodeString: units)
                    release.keyboardSetUnicodeString(stringLength: units.count, unicodeString: units)
                    event.postToPid(config.binding.pid)
                    release.postToPid(config.binding.pid)
                    // CG posting has no delivery acknowledgement. Fresh value alone
                    // cannot establish confirmed delivery, so preserve Accepted.
                    confirmed = false
                }
                try frame.encode(json.borrowed(["delivery": confirmed ? "confirmed" : "accepted"]))
            } else {
                let snapshot = try snapshot(request, input, admission, json)
                if nextPhase == 0 {
                    try frame.encode(json.response(request: request, ticket: c.ticket, channel: "external_semantics") {
                        try json.borrowed(["status": "observed", "data": snapshot] as [String: Any])
                    })
                } else if nextPhase == 3 {
                    try frame.encode(json.envelope("snapshot") { snapshot })
                } else {
                    var act = try action(request)
                    let (_, name) = try validateAction(act, admission, delivery: false)
                    guard input.documents.count == 4,
                          let expected = input.documents[2]["artifact"] as? [String: Any], expected["kind"] as? String == "expectation",
                          let expectedData = expected["data"] as? [String: Any], let targets = expectedData["targets"] as? [[String: String]], targets.count == 1
                    else { throw NativeProtocolError.request }
                    _ = try index(targets[0]) // snapshot independently validated every held object
                    var reference = act["backend_ref"] as! [String: Any]
                    reference["snapshot_id"] = snapshot["id"]
                    let observation = (snapshot["observations"] as! [[String: Any]])[0]["id"] as! String
                    reference["observation_id"] = observation
                    act["backend_ref"] = reference; act["unique_match"] = true
                    act["resolution"] = try json.borrowed(["available_intents": [name], "writable": try json.known("flag", name == "type" || name == "fill"),
                        "value_allowed": try json.known("flag", name == "type" || name == "fill"),
                        "evidence": try json.evidence(observation, "macos.ax", "exact_held_capability")] as [String: Any])
                    try frame.encode(json.envelope("action") { try json.borrowed(["snapshot": snapshot, "action": act]) })
                }
            }
        } catch {
            frame.reset()
            let failure = NativeJSON(config.acquisition_limits)
            // Fixed issues only; never serialize the request, value or SDK error.
            let code: String, recovery: String
            switch error {
            case NativeFormFailure.staleTarget: code = "stale_target"; recovery = "new_native_session"
            case NativeFormFailure.ambiguousTarget: code = "ambiguous_target"; recovery = "unique_native_identity"
            case NativeFormFailure.inputOwner: code = "interrupted"; recovery = "input_owner_changed"
            case NativeFormFailure.privateValue: code = "unsupported"; recovery = "public_enabled_control_required"
            case NativeFormFailure.focusUnavailable: code = "unsupported"; recovery = "focus_not_settable"
            case NativeFormFailure.setterUnavailable: code = "unsupported"; recovery = "value_not_settable"
            case NativeFormFailure.keyboardUnavailable: code = "unsupported"; recovery = "keyboard_preconditions"
            case NativeAcquisitionError.limit: code = "incomplete_scope"; recovery = "native_acquisition_limit"
            case NativeAcquisitionError.expired: code = "timeout"; recovery = "new_native_observation"
            default: code = "unsupported"; recovery = "new_native_observation"
            }
            try frame.encode(failure.envelope("error") {
                try failure.borrowed(["code": nextPhase == 2 ? "action_outcome_unknown" : code,
                    "scope_id": config.scope_id, "failed_step": "native-form", "recovery_class": recovery])
            })
        }
        try io.reply(frame, cap: min(input.replyCap, output), deadline: deadline)
    }
}
