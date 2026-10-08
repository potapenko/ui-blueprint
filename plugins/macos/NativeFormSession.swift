import Foundation
import AppKit
import ApplicationServices
import Darwin

// Explicit fixture-only resident AX owner. No input is emitted from Observe or
// resolve. The parent broker authenticates the delivery nonce before forwarding.
enum NativeFormFailure: Error { case permissionRequired, staleTarget, ambiguousTarget, unsupported, inputOwner, privateValue, focusUnavailable, keyboardUnavailable, setterUnavailable, protectedSource }

// Delivery-only source reader. Public records carry an Id, never these bytes.
// No persistent store, path lookup from UI, or secret in an Error/description.
enum NativeProtectedSource {
    enum Stage: String { case sourceOpen = "source_open", sourceRead = "source_read", dispatch, error, released }
    static func trace(_ stage: Stage, enabled: Bool) {
        if enabled { fputs("native_protected:\(stage.rawValue)\n", stderr) }
    }
    static func withValue(_ source: NativeConfiguration.ProtectedInput, admission: NativeAcquisition,
                             body: (String) throws -> Bool) throws -> Bool {
        let deadline = admission.deadline
        trace(.sourceOpen, enabled: source.trace)
        defer { trace(.released, enabled: source.trace) }
        do {
            try NativeDescriptorIO.check(deadline)
            var core = rlimit(rlim_cur: 0, rlim_max: 0)
            guard setrlimit(RLIMIT_CORE, &core) == 0 else { throw NativeProtocolError.request }
            // Nonblocking prevents special-file open from hanging before fstat.
            let fd = open(source.path, O_RDONLY | O_NOFOLLOW | O_NONBLOCK | O_CLOEXEC)
            guard fd >= 0 else { throw NativeProtocolError.request }
            defer { close(fd) }
            var info = stat()
            guard fstat(fd, &info) == 0, info.st_mode & S_IFMT == S_IFREG,
                  info.st_uid == geteuid(), info.st_mode & 0o777 == 0o600,
                  info.st_size > 0, info.st_size <= min(4096, admission.limits.value_utf8_bytes)
            else { throw NativeProtocolError.request }
            let count = Int(info.st_size)
            try admission.reserveCopies(count + 1)
            var bytes = Data(count: count + 1)
            defer { bytes.resetBytes(in: 0..<bytes.count) }
            var used = 0
            while used < bytes.count {
                try NativeDescriptorIO.check(deadline)
                let n = bytes.withUnsafeMutableBytes { storage in
                    Darwin.read(fd, storage.baseAddress!.advanced(by: used), storage.count - used)
                }
                if n == 0 { break }
                if n < 0 {
                    if errno == EINTR { continue }
                    throw NativeProtocolError.request
                }
                used += n
            }
            guard used == count else { throw NativeProtocolError.request }
            bytes.removeLast()
            guard let text = String(data: bytes, encoding: .utf8),
                  !text.unicodeScalars.contains(where: { CharacterSet.controlCharacters.contains($0) })
            else { throw NativeProtocolError.request }
            try NativeDescriptorIO.check(deadline)
            trace(.sourceRead, enabled: source.trace)
            return try body(text)
        } catch {
            trace(.error, enabled: source.trace)
            throw NativeProtocolError.request // never propagate source/SDK payloads
        }
    }
}

@MainActor final class NativeFormSession {
    let config: NativeConfiguration
    let configurationBytes: Data
    let epoch: UInt64
    let serial: UInt64
    let generation = UUID().uuidString
    let end: Double
    let application: AXUIElement
    let window: CFTypeRef
    let parentWindow: CFTypeRef?
    let primaryCount: Int
    var parentResult: Int?
    let identifiers: [String]
    let handles: [CFTypeRef]
    var order: NativeFormOrder
    var inputOwner: pid_t?
    var ownerWindow: CFTypeRef?
    var protectedUsed = false
    var revision: UInt64 = 0

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
        primaryCount = ids.count
        identifiers = ids + (config.parent_form_identifiers ?? [])
        guard AXIsProcessTrusted() else { throw NativeProtocolError.configuration }
        var manifest = config.binding.manifest; manifest["identity_path"] = config.identity_path
        try NativeCurrentIdentity.verify(path: config.identity_path, expected: manifest)
        let admission = try NativeAcquisition(config.acquisition_limits, deadline: input.deadline)
        application = AXUIElementCreateApplication(config.binding.pid)
        if let parent = config.parent_binding, let path = config.parent_identity_path {
            try NativeCurrentIdentity.verify(path: path, expected: parent.manifest)
            let selectedParent = try Collector.resolveWindow(application, identifier: parent.window_identifier, admission: admission)
            parentWindow = selectedParent
            window = try Collector.resolvePopover(selectedParent, ownerIdentifier: "f02.popup.owner.\(parent.window_identifier)",
                maxNodes: 160, maxDepth: 9, admission: admission)
            handles = try Self.resolve(window, ids: ids, admission: admission)
                + Self.resolve(selectedParent, ids: config.parent_form_identifiers!, admission: admission)
        } else {
            parentWindow = nil
            window = try Collector.resolveWindow(application, identifier: config.binding.window_identifier, admission: admission)
            handles = try Self.resolve(window, ids: ids, admission: admission)
        }
    }
    func current(_ admission: NativeAcquisition, parentOnly: Bool = false) throws {
        try admission.check()
        let binding = parentOnly ? (config.parent_binding ?? config.binding) : config.binding
        guard AXIsProcessTrusted() else { throw NativeFormFailure.permissionRequired }
        guard let app = NSRunningApplication(processIdentifier: binding.pid),
              !app.isTerminated, app.bundleIdentifier == binding.bundle_id,
              app.launchDate?.timeIntervalSince1970 == binding.launch_time,
              Collector.windowOwnedBy(pid: binding.pid, window: binding.window_id)
        else { throw NativeFormFailure.staleTarget }
        do {
            if !parentOnly { try NativeCurrentIdentity.verify(path: config.identity_path, expected: config.binding.manifest) }
            if let parent = config.parent_binding, let path = config.parent_identity_path, let held = parentWindow {
                try NativeCurrentIdentity.verify(path: path, expected: parent.manifest)
                guard Collector.windowOwnedBy(pid: parent.pid, window: parent.window_id),
                      CFEqual(held, try Collector.resolveWindow(application, identifier: parent.window_identifier, admission: admission))
                else { throw NativeFormFailure.staleTarget }
                if !parentOnly {
                    guard CFEqual(window, try Collector.resolvePopover(held, ownerIdentifier: "f02.popup.owner.\(parent.window_identifier)",
                        maxNodes: 160, maxDepth: 9, admission: admission)) else { throw NativeFormFailure.staleTarget }
                }
            } else {
                guard CFEqual(window, try Collector.resolveWindow(application, identifier: binding.window_identifier, admission: admission))
                else { throw NativeFormFailure.staleTarget }
            }
        } catch NativeAcquisitionError.expired { throw NativeAcquisitionError.expired }
          catch NativeAcquisitionError.limit { throw NativeAcquisitionError.limit }
          catch { throw NativeFormFailure.staleTarget }
    }
    static func resolve(_ window: CFTypeRef, ids: [String], admission: NativeAcquisition, access: NativeAXAccess = .live) throws -> [CFTypeRef] {
        var queue: [(CFTypeRef, Int)] = [(window, 0)], seen: [CFTypeRef] = []
        var found = [CFTypeRef?](repeating: nil, count: ids.count)
        while !queue.isEmpty && seen.count < 160 {
            let (element, depth) = queue.removeFirst()
            if seen.contains(where: { CFEqual($0, element) }) { continue }
            seen.append(element)
            // A descendant popover is a separate Surface, even when AX nests it
            // beneath the parent trigger. The explicitly bound root stays in scope.
            if depth > 0 {
                guard let rawRole = try nativeAXAttribute(element, kAXRoleAttribute, admission: admission, access: access)
                else { throw NativeAcquisitionError.invalidValue }
                if try admission.text(rawRole) == kAXPopoverRole { continue }
            }
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
    func validateHandles(_ admission: NativeAcquisition, resultOnly: Int? = nil) throws {
        do {
            if let index = resultOnly, let parent = parentWindow {
                let fresh = try Self.resolve(parent, ids: [identifiers[index]], admission: admission)
                guard CFEqual(handles[index], fresh[0]) else { throw NativeFormFailure.staleTarget }
            } else {
                var fresh = try Self.resolve(window, ids: Array(identifiers.prefix(primaryCount)), admission: admission)
                if let parent = parentWindow {
                    fresh += try Self.resolve(parent, ids: Array(identifiers.dropFirst(primaryCount)), admission: admission)
                }
                guard zip(handles, fresh).allSatisfy({ CFEqual($0, $1) }) else { throw NativeFormFailure.staleTarget }
            }
        } catch NativeAcquisitionError.invalidValue { throw NativeFormFailure.staleTarget }
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
        let resultOnly = input.control.flags & 127 == 3 ? parentResult : nil
        try current(admission, parentOnly: resultOnly != nil)
        guard let context = request["context"] as? [String: Any], let target = context["target"] as? [String: Any],
              let surfaces = context["surfaces"] as? [[String: Any]], surfaces.count == (parentWindow == nil ? 1 : 2),
              target["id"] as? String == "f02-pid-\(config.binding.pid)", target["generation"] as? String == config.binding.target_generation,
              surfaces[0]["id"] as? String == "window-\(config.binding.window_id)", surfaces[0]["generation"] as? String == config.binding.surface_generation,
              context["scope_id"] as? String == config.scope_id,
              let fields = context["fields"] as? [String], fields.contains("enabled"), fields.contains("value"), fields.contains("focused"),
              let limits = request["limits"] as? [String: Any], let count = limits["max_elements"] as? Int, count >= handles.count, count <= 160,
              let depth = limits["max_depth"] as? Int, (1...9).contains(depth),
              let id = request["request_id"] as? String
        else { throw NativeProtocolError.request }
        if let parent = config.parent_binding {
            guard surfaces[1]["id"] as? String == "window-\(parent.window_id)",
                  surfaces[1]["generation"] as? String == parent.surface_generation else { throw NativeProtocolError.request }
        }
        let observation = "\(id)-native-\(input.control.operation)-\(input.control.flags)"
        try validateHandles(admission, resultOnly: resultOnly)
        let selected = resultOnly.map { [$0] } ?? Array(handles.indices)
        var nodes: [[String: Any]] = []
        for index in selected {
            let result = try collectWindowAX(handles[index], surface: surfaces[index < primaryCount ? 0 : 1], observationID: observation,
                maxNodes: 1, maxDepth: 1, deadline: input.deadline, admission: admission, fields: fields, json: json)
            guard result.nodes.count == 1 else { throw NativeProtocolError.request }
            var node = result.nodes[0]
            node["key"] = try json.borrowed(key(index)); node["children"] = try json.array { [Any]() }
            nodes.append(node)
        }
        var result = try Collector.snapshot(context: context, surface: surfaces[resultOnly == nil ? 0 : 1], target: target, scope: config.scope_id,
            fields: fields, observation: observation, channel: "external_semantics", started: input.started,
            nodes: nodes, captures: [], json: json)
        if parentWindow != nil && resultOnly == nil {
            // The binding source is the own fixture content attachment, not AX geometry.
            let bindingID = observation + "-binding"
            var bindingObservation = (result["observations"] as! [[String: Any]])[0]
            bindingObservation["id"] = bindingID
            bindingObservation["source_namespace"] = "macos.fixture.binding"
            result["observations"] = try json.borrowed((result["observations"] as! [[String: Any]]) + [bindingObservation])
            let evidence = try json.evidence(bindingID, "macos.fixture.binding", "explicit_fixture_popover_trigger_binding")
            // Configuration requires this exact parent anchor before session creation.
            let anchorIndex = identifiers.indices.first { $0 >= primaryCount && identifiers[$0] == "f02.popup" }!
            let parentRecord: [String: Any] = ["identity": surfaces[1], "native_owner": try json.known("identity", target),
                "initiated_by": NSNull(), "anchor": NSNull(), "evidence": evidence]
            let popupRecord: [String: Any] = ["identity": surfaces[0], "native_owner": try json.known("identity", target),
                "initiated_by": surfaces[1], "anchor": key(anchorIndex), "evidence": evidence]
            result["surface_records"] = try json.borrowed([popupRecord, parentRecord])
        }
        let (nextRevision, overflow) = revision.addingReportingOverflow(1)
        guard !overflow else { throw NativeProtocolError.limit }
        revision = nextRevision
        result["revision"] = try json.scalar(revision)
        let focused = try attribute(application, kAXFocusedUIElementAttribute, admission)
        let focusedIndex = try currentInputOwner(admission) == config.binding.pid
            ? focused.flatMap { focused in selected.first(where: { CFEqual(handles[$0], focused) }) } : nil
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
        try current(admission, parentOnly: resultOnly != nil)
        try validateHandles(admission, resultOnly: resultOnly)
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
        // Only explicit FillSecret may enter a known protected control.
        let roleRaw = try attribute(handle, kAXRoleAttribute, admission)
        let subroleRaw = try attribute(handle, kAXSubroleAttribute, admission)
        let role = try roleRaw.map { try admission.text($0) }
        let subrole = try subroleRaw.map { try admission.text($0) }
        guard try bool(handle, kAXEnabledAttribute, admission) else { throw NativeFormFailure.unsupported }
        let secure = identifiers[i] == "f02.secret" || role == "AXSecureTextField" || subrole == "AXSecureTextField"
        if name != "focus" && name != "fill_secret" && secure { throw NativeFormFailure.privateValue }
        if name == "fill_secret" {
            guard role == "AXSecureTextField" || subrole == "AXSecureTextField", let source = config.protected_input,
                  source.reference == intent["secret_reference"] as? String,
                  source.action_id == action["id"] as? String, source.identifier == identifiers[i],
                  !protectedUsed || delivery else { throw NativeFormFailure.protectedSource }
        }
        let owner = try currentInputOwner(admission)
        let focusedWindow = try attribute(application, kAXFocusedWindowAttribute, admission)
        guard (name == "focus" || owner == config.binding.pid), let focusedWindow, CFEqual(focusedWindow, parentWindow ?? window) else { throw NativeFormFailure.inputOwner }
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
        case "fill_secret":
            var settable = DarwinBoolean(false)
            guard action["modality"] as? String == "setter",
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
                else if name == "fill_secret" {
                    guard !protectedUsed, let source = config.protected_input else { throw NativeFormFailure.protectedSource }
                    protectedUsed = true // consume before source read, even on error/cancel
                    confirmed = try NativeProtectedSource.withValue(source, admission: admission) { text in
                        _ = try validateAction(act, admission, delivery: true)
                        try admission.check()
                        NativeProtectedSource.trace(.dispatch, enabled: source.trace)
                        return AXUIElementSetAttributeValue(handle, kAXValueAttribute as CFString, text as CFString) == .success
                    }
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
                    let (actorIndex, name) = try validateAction(act, admission, delivery: false)
                    parentResult = nil
                    guard input.documents.count == 4,
                          let expected = input.documents[2]["artifact"] as? [String: Any], expected["kind"] as? String == "expectation",
                          let expectedData = expected["data"] as? [String: Any], let targets = expectedData["targets"] as? [[String: String]], targets.count == 1
                    else { throw NativeProtocolError.request }
                    let resultIndex = try index(targets[0]) // independent held result
                    if parentWindow != nil {
                        guard name == "activate", actorIndex < primaryCount, resultIndex >= primaryCount else { throw NativeFormFailure.unsupported }
                        parentResult = resultIndex
                    }
                    if name == "fill_secret" {
                        guard targets[0] != act["backend_ref"].flatMap({ $0 as? [String: Any] })?["key"] as? [String: String],
                              identifiers[resultIndex] != "f02.secret" else { throw NativeFormFailure.privateValue }
                    }
                    var reference = act["backend_ref"] as! [String: Any]
                    reference["snapshot_id"] = snapshot["id"]
                    let observation = (snapshot["observations"] as! [[String: Any]])[0]["id"] as! String
                    reference["observation_id"] = observation
                    act["backend_ref"] = reference; act["unique_match"] = true
                    act["resolution"] = try json.borrowed(["available_intents": [name == "fill_secret" ? "fill" : name], "writable": try json.known("flag", name == "type" || name == "fill" || name == "fill_secret"),
                        "value_allowed": try json.known("flag", name == "type" || name == "fill" || name == "fill_secret"),
                        "evidence": try json.evidence(observation, "macos.ax", "exact_held_capability")] as [String: Any])
                    try frame.encode(json.envelope("action") { try json.borrowed(["snapshot": snapshot, "action": act]) })
                }
            }
        } catch {
            if config.protected_input?.trace == true { NativeProtectedSource.trace(.error, enabled: true) }
            frame.reset()
            let failure = NativeJSON(config.acquisition_limits)
            // Fixed issues only; never serialize the request, value or SDK error.
            let code: String, recovery: String
            switch error {
            case NativeFormFailure.permissionRequired: code = "permission_required"; recovery = "native_ax_permission_required"
            case NativeFormFailure.staleTarget: code = "stale_target"; recovery = "new_native_session"
            case NativeFormFailure.ambiguousTarget: code = "ambiguous_target"; recovery = "unique_native_identity"
            case NativeFormFailure.inputOwner: code = "interrupted"; recovery = "input_owner_changed"
            case NativeFormFailure.protectedSource: code = "unsupported"; recovery = "protected_source_binding_unavailable"
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
