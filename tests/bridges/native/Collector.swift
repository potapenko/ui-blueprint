import AppKit
import ApplicationServices
import ScreenCaptureKit
import Foundation
import Darwin

// Trusted current identity-only file, never last measurement publication. Public
// process/window/AX binding checks remain independent in the collection path.
enum NativeCurrentIdentity {
    static func verify(path: String, expected: [String: Any]) throws {
        let fd = open(path, O_RDONLY | O_CLOEXEC | O_NOFOLLOW)
        guard fd >= 0 else { throw NativeAcquisitionError.invalidValue }
        let file = FileHandle(fileDescriptor: fd, closeOnDealloc: true)
        defer { try? file.close() }
        let bytes = try file.read(upToCount: 4033) ?? Data()
        guard bytes.count <= 4032,
              let actual = try JSONSerialization.jsonObject(with: bytes) as? [String: Any],
              Set(actual.keys) == Set(["identity_version","pid","bundle_id","launch_time","window_id","window_identifier","target_generation","surface_generation","state"]),
              actual["identity_version"] as? String == "1.0.0", actual["state"] as? String == "open"
        else { throw NativeAcquisitionError.invalidValue }
        for name in ["pid","bundle_id","launch_time","window_id","window_identifier","target_generation","surface_generation"] {
            guard let a = actual[name] as? NSObject, let b = expected[name] as? NSObject, a == b else { throw NativeAcquisitionError.invalidValue }
        }
    }
}

#if !HOST_HELPER
@main
#endif
struct Collector {
    @MainActor static func main() async {
        DispatchQueue.global().asyncAfter(deadline: .now() + 8) { _exit(124) }
        do { try await run() }
        catch { fputs("native collector failed; no raw UI diagnostics\n", stderr); exit(1) }
    }
    @MainActor static func run() async throws {
        let args = CommandLine.arguments
        guard args.count == 5 || args.count == 6 else { exit(2) }
        let sequence: UInt64? = args.count == 6 ? UInt64(args[5]) : nil
        if args.count == 6 && (sequence == nil || sequence == 0) { exit(2) }
        // Mandatory explicit caller file, bounded before parsing. No implicit profile.
        let limitFile = try FileHandle(forReadingFrom: URL(fileURLWithPath: args[4]))
        defer { try? limitFile.close() }
        let limitsData = try limitFile.read(upToCount: 4033) ?? Data()
        guard limitsData.count <= 4032 else { throw NativeAcquisitionError.limit }
        let limits = try JSONDecoder().decode(NativeAcquisitionLimits.self, from: limitsData)
        try limits.validate()
        var bytes = Data()
        while let byte = try FileHandle.standardInput.read(upToCount: 1), !byte.isEmpty {
            if byte[0] == 10 { break }
            guard bytes.count < 1_048_576 else { throw NativeAcquisitionError.limit }
            bytes.append(byte)
        }
        guard let doc = try JSONSerialization.jsonObject(with: bytes) as? [String: Any],
              let manifest = try JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: args[1]))) as? [String: Any]
        else { throw NativeAcquisitionError.invalidValue }
        try await collect(document: doc, manifest: manifest, directory: URL(fileURLWithPath: args[2]),
            mode: args[3], sequence: sequence, limits: limits, evidence: true, wireCap: 1_048_576)
    }

    // Shared one-channel acquisition path. Legacy combined requests invoke it in
    // channel order with separate registered per-channel budgets.
    @MainActor static func collect(document doc: [String: Any], manifest: [String: Any],
        directory: URL?, mode: String, sequence: UInt64?, limits: NativeAcquisitionLimits,
        evidence: Bool, wireCap: Int, selectedChannel: String? = nil, deadline: Double? = nil,
        nativeEmit: ((NativeJSONFrame) throws -> Void)? = nil) async throws {
        try limits.validate()
        let windowMode = ["window-ax", "describe-window"].contains(mode)
        let sampleFields = ["role", "accessibility_name", "enabled", "accessibility_bounds"]
        let expectedChannels = windowMode ? ["external_semantics"] : ["external_semantics", "rendered_capture"]
        guard doc["schema_version"] as? String == "0.1.0",
              let artifact = doc["artifact"] as? [String: Any], artifact["kind"] as? String == "request",
              let request = artifact["data"] as? [String: Any], let context = request["context"] as? [String: Any],
              let target = context["target"] as? [String: Any], let surfaces = context["surfaces"] as? [[String: Any]], surfaces.count == 1,
              let fields = context["fields"] as? [String], !fields.isEmpty, Set(fields).count == fields.count,
              windowMode ? Set(fields).isSubset(of: ["role", "accessibility_name", "description", "value", "placeholder", "enabled", "focused", "actions", "accessibility_bounds"]) : fields == sampleFields,
              let scope = context["scope_id"] as? String,
              let session = context["session_id"] as? String, let requestID = request["request_id"] as? String,
              let operation = request["operation"] as? [String: Any], operation["operation"] as? String == "observe",
              let channels = operation["channels"] as? [String],
              selectedChannel.map({ channels.contains($0) }) ?? (channels == expectedChannels),
              let requested = request["limits"] as? [String: Any],
              let maxNodes = requested["max_elements"] as? Int, (1...160).contains(maxNodes),
              let maxDepth = requested["max_depth"] as? Int, (1...9).contains(maxDepth),
              let deadlineMS = requested["deadline_ms"] as? Double, deadlineMS.isFinite && deadlineMS > 0,
              let outputLimit = requested["max_output_bytes"] as? Int, (1...1_048_576).contains(outputLimit),
              let pid = manifest["pid"] as? Int32, let bundle = manifest["bundle_id"] as? String,
              ["local.uiblueprint.f02.on", "local.uiblueprint.f02.off"].contains(bundle),
              let launch = manifest["launch_time"] as? Double, let wid = manifest["window_id"] as? UInt32,
              let identifier = manifest["window_identifier"] as? String, ["a", "b"].contains(identifier),
              target["generation"] as? String == manifest["target_generation"] as? String,
              surfaces[0]["generation"] as? String == manifest["surface_generation"] as? String,
              target["id"] as? String == "f02-pid-\(pid)", surfaces[0]["id"] as? String == "window-\(wid)"
        else { throw NativeAcquisitionError.invalidValue }
        let end = min(deadline ?? .infinity, ProcessInfo.processInfo.systemUptime + deadlineMS / 1000)
        let chosen = selectedChannel.map { [$0] } ?? channels
        let wantsAX = chosen.contains("external_semantics"), wantsCapture = chosen.contains("rendered_capture")
        let app = NSRunningApplication(processIdentifier: pid)
        let identityCurrent = (manifest["identity_path"] as? String).map { path in
            (try? NativeCurrentIdentity.verify(path: path, expected: manifest)) != nil
        } ?? false
        let identityResolved = identityCurrent && app?.bundleIdentifier == bundle && app?.launchDate?.timeIntervalSince1970 == launch
            && windowOwnedBy(pid: pid, window: wid)
        let axAllowed = wantsAX && identityResolved ? AXIsProcessTrusted() : false
        let captureAllowed = wantsCapture && identityResolved ? CGPreflightScreenCaptureAccess() : false
        let surface = surfaces[0]
        var responseBytes = 0
        func send(_ frame: NativeJSONFrame) throws {
            guard frame.count <= outputLimit - responseBytes else { throw NativeAcquisitionError.limit }
            if let nativeEmit { try nativeEmit(frame) }
            else { try frame.write(to: STDOUT_FILENO, deadline: end) }
            responseBytes = try nativeAdd(responseBytes, frame.count)
        }
        if sequence == nil {
            guard identityResolved else { throw NativeAcquisitionError.invalidValue }
            let json = NativeJSON(limits)
            let descriptor = try json.envelope("session") {
                try json.object(["allowed_scopes", "session_id", "plugin", "supported_versions", "target", "surfaces", "capabilities"]) {
                    ["allowed_scopes": try json.borrowed([scope]), "session_id": try json.scalar(session),
                     "plugin": try json.borrowed(context["plugin"]!), "supported_versions": try json.borrowed(["0.1.0"]),
                     "target": try json.borrowed(target), "surfaces": try json.borrowed(surfaces),
                     "capabilities": try json.array { try chosen.map { channel in
                        let allowed = channel == "external_semantics" ? axAllowed : captureAllowed
                        return try json.object(["channel", "operation", "status", "reason"]) {
                            ["channel": try json.scalar(channel), "operation": try json.scalar("observe"),
                             "status": try json.scalar(allowed ? "partial" : "permission_required"),
                             "reason": try json.scalar(allowed ? "fixture_scope_partial" : "permission_not_granted")]
                        }
                     }}]
                }
            }
            let frame = try NativeJSONFrame(capacity: min(wireCap, outputLimit), deadline: end)
            try frame.encode(descriptor)
            // Legacy descriptor is attachment output, outside channel cumulative budget.
            try frame.write(to: STDOUT_FILENO, deadline: end)
        }
        if mode == "describe" || mode == "describe-window" { return }
        for channel in chosen {
            let admission = try NativeAcquisition(limits, deadline: end)
            let json = NativeJSON(limits)
            let frame = try NativeJSONFrame(capacity: min(wireCap, outputLimit - responseBytes), deadline: end)
            func envelope(_ result: () throws -> [String: Any]) throws -> [String: Any] {
                if let sequence { return try json.response(request: request, ticket: sequence, channel: channel, result: result) }
                // Historical standalone mode emits Snapshot/Error, not a forged Ticket.
                let value = try result()
                guard let data = value["data"] as? [String: Any] else { throw NativeAcquisitionError.invalidValue }
                return try json.envelope(value["status"] as? String == "observed" ? "snapshot" : "error") { data }
            }
            // Reserve a complete failure before risky construction. No new uncharged
            // error graph is needed when the ordinary builder reaches its ceiling.
            let failure = try envelope { try json.failure("incomplete_scope", scope: scope, channel: channel) }
            let unresolved = try envelope { try json.failure(identityCurrent ? "target_unresolved" : "stale_target", scope: scope, channel: channel) }
            let stale = try envelope { try json.failure("stale_target", scope: scope, channel: channel) }
            var proof: [String: Any] = [:]
            var proofDirectory: URL?
            func prepare() async throws {
                if !identityResolved { try frame.encode(unresolved); return }
                let started = ProcessInfo.processInfo.systemUptime
                let allowed = channel == "external_semantics" ? axAllowed : captureAllowed
                if !allowed && (selectedChannel != nil || ProcessInfo.processInfo.environment["UIB_CAPTURE_FAULT"] == nil || channel == "external_semantics") {
                    try frame.encode(envelope { try json.failure("permission_required", scope: scope, channel: channel) })
                    return
                }
                var nodes: [[String: Any]]
                var captures: [[String: Any]]
                let oid = "\(requestID)-\(channel)"
                if channel == "external_semantics" {
                    let root = AXUIElementCreateApplication(pid)
                    let window: AXUIElement
                    do {
                        window = unsafeDowncast(try resolveWindow(root, identifier: identifier, admission: admission), to: AXUIElement.self)
                    } catch {
                        try frame.encode(unresolved); return
                    }
                    if windowMode {
                        let collected = try collectWindowAX(window, surface: surface, observationID: oid,
                            maxNodes: maxNodes, maxDepth: maxDepth, deadline: min(end, started + min(0.9, deadlineMS / 1000)), admission: admission, fields: fields, json: json)
                        nodes = collected.nodes
                        if evidence { proof = collected.metrics }
                    } else {
                        let collected = try sample(window, identifier: identifier, surface: surface, observation: oid,
                            maxNodes: maxNodes, maxDepth: maxDepth, deadline: min(end, started + min(0.9, deadlineMS / 1000)), admission: admission, json: json)
                        nodes = collected.nodes
                        if evidence { proof = collected.metrics }
                    }
                    captures = try json.array { [] }
                } else {
                    nodes = try json.array { [] }
                    if selectedChannel == nil && mode != "live" { try await Task.sleep(for: .seconds(6)); return }
                    guard let artifactRoot = directory else { throw NativeAcquisitionError.invalidValue }
                    let directory = selectedChannel == nil ? artifactRoot : try nativeChannelDirectory(artifactRoot, channel: "capture")
                    proofDirectory = directory
                    let captured: OwnedCapture
                    do { captured = try await CaptureLifecycle.capture(windowID: wid, pid: pid,
                        budget: min(2, end - ProcessInfo.processInfo.systemUptime),
                        admission: selectedChannel == nil ? .legacyRun : .parentOwned, acquisition: admission) }
                    catch {
                        let issue = CaptureLifecycle.issue(error)
                        try frame.encode(envelope { try json.failure(issue.code, scope: scope, channel: channel) })
                        return
                    }
                    try admission.check()
                    guard NSRunningApplication(processIdentifier: pid)?.launchDate?.timeIntervalSince1970 == launch else {
                        try frame.encode(envelope { try json.failure("stale_target", scope: scope, channel: channel) })
                        return
                    }
                    let pngBytes = try writeNativePNG(captured.image, admission: admission, directory: directory, name: "capture.png")
                    if evidence {
                        func rect(_ r: CGRect) -> [String: Double] { ["x": r.minX, "y": r.minY, "width": r.width, "height": r.height] }
                        let metadata: [String: Any] = ["observation_id": oid,
                            "source": "ScreenCaptureKit filter_and_window_metadata", "units": "pt", "origin": "top_left",
                            "filter_content_rect": rect(captured.filterRect), "filter_point_pixel_scale": captured.scale,
                            "window_id": wid, "window_frame": rect(captured.windowFrame),
                            "pixel_width": captured.image.width, "pixel_height": captured.image.height,
                            "row_bytes": captured.image.bytesPerRow, "png_bytes": pngBytes,
                            "capture_serialization": selectedChannel == nil ? "exclusive_run_owned_flock" : "parent_owned_capture_lease_until_reap",
                            "capture_admission_wait_seconds": captured.admissionWait, "transform_status": "unknown",
                            "capture_call_start": started, "capture_call_end": ProcessInfo.processInfo.systemUptime,
                            "clock_domain": "helper-\(ProcessInfo.processInfo.processIdentifier)-monotonic", "time_unit": "seconds"]
                        proof = metadata
                    }
                    captures = try json.array { [try captureRecord(captured, surface: surface, observation: oid, payload: selectedChannel == nil ? "capture.png" : "capture/capture.png", json: json)] }
                }
                let complete = try envelope {
                    try json.object(["status", "data"]) {
                        ["status": try json.scalar("observed"), "data": try snapshot(context: context, surface: surface,
                            target: target, scope: scope, fields: fields, observation: oid, channel: channel,
                            started: started, nodes: nodes, captures: captures, json: json)]
                    }
                }
                try admission.check(); try frame.encode(complete)
            }
            func emitEvidence() throws {
                if evidence, let directory {
                    let output: URL
                    if let proofDirectory { output = proofDirectory }
                    else { output = selectedChannel == nil ? directory : try nativeChannelDirectory(directory, channel: channel == "external_semantics" ? "ax" : "capture") }
                    proof.merge(admission.metrics, uniquingKeysWith: { _, new in new })
                    proof["response_slots"] = json.slots
                    proof["response_string_utf8_bytes"] = json.stringBytes
                    proof["encoded_bytes_including_lf"] = frame.count
                    proof["publication_ack"] = "owned_by_parent_not_inferred"
                    _ = try writeNativeSidecar(proof, name: channel == "external_semantics" ? "acquisition.json" : "capture-metadata.json",
                        admission: admission, directory: output)
                }
            }
            try await finishChannel(frame: frame, failure: failure, prepare: prepare, evidence: emitEvidence, send: { value in
                guard NSRunningApplication(processIdentifier: pid)?.launchDate?.timeIntervalSince1970 == launch,
                      windowOwnedBy(pid: pid, window: wid) else {
                    value.reset(); try value.encode(stale); try send(value); return
                }
                try sendCurrentIdentity(value, stale: stale, manifest: manifest, send: send)
            })

        }
    }
}

extension Collector {
    @MainActor static func sendCurrentIdentity(_ frame: NativeJSONFrame, stale: [String: Any],
        manifest: [String: Any], send: (NativeJSONFrame) throws -> Void) throws {
        guard let path = manifest["identity_path"] as? String,
              (try? NativeCurrentIdentity.verify(path: path, expected: manifest)) != nil else {
            frame.reset(); try frame.encode(stale); try send(frame); return
        }
        try send(frame)
    }
    // Same production binding decision, with only the public AX calls replaceable
    // by bounded synthetic CF objects in this executable's offline tests.
    @MainActor static func resolveWindow(_ root: CFTypeRef, identifier: String,
        admission: NativeAcquisition, access: NativeAXAccess = .live) throws -> CFTypeRef {
        var window: CFTypeRef?
        var matches = 0
        _ = try nativeAXElements(root, kAXWindowsAttribute, windows: true,
            capacity: admission.limits.ax_windows, admission: admission, access: access) { candidate in
                if let raw = try nativeAXAttribute(candidate, kAXIdentifierAttribute, admission: admission, access: access),
                   try admission.text(raw) == identifier { matches += 1; window = candidate }
            }
        guard matches == 1, let window else { throw NativeAcquisitionError.invalidValue }
        return window
    }

    // Only construction/codec/evidence errors become the reserved whole failure.
    // Final FD delivery is deliberately OUTSIDE both catches: no retry after a
    // partial write. This is the production terminal path, not a test substitute.
    @MainActor static func finishChannel(frame: NativeJSONFrame, failure: [String: Any],
        prepare: @MainActor () async throws -> Void, evidence: @MainActor () throws -> Void,
        send: @MainActor (NativeJSONFrame) throws -> Void) async throws {
        do { try await prepare() } catch { frame.reset(); try frame.encode(failure) }
        do { try evidence() } catch { frame.reset(); try frame.encode(failure) }
        try send(frame)
    }
    // Inspect only the requested numeric owner in the returned CF container; do
    // not bridge/copy an entire metadata dictionary containing unrequested titles.
    static func windowOwnedBy(pid: Int32, window: UInt32) -> Bool {
        guard let rows = CGWindowListCopyWindowInfo(.optionIncludingWindow, window), CFArrayGetCount(rows) == 1 else { return false }
        let raw = NativeAcquisition.item(rows, 0)
        guard CFGetTypeID(raw) == CFDictionaryGetTypeID() else { return false }
        let row = unsafeDowncast(raw, to: CFDictionary.self)
        guard let pointer = CFDictionaryGetValue(row, Unmanaged.passUnretained(kCGWindowOwnerPID).toOpaque()) else { return false }
        let value = Unmanaged<AnyObject>.fromOpaque(pointer).takeUnretainedValue()
        return (value as? NSNumber)?.int32Value == pid
    }
    @MainActor static func sample(_ window: AXUIElement, identifier: String, surface: [String: Any],
        observation: String, maxNodes: Int, maxDepth: Int, deadline: Double,
        admission: NativeAcquisition, json: NativeJSON) throws -> WindowAXResult {
        var queue: [(AXUIElement, Int)] = [(window, 0)], matches: [AXUIElement] = []
        var handles = [window]
        var visited = 0, duplicates = 0
        while !queue.isEmpty, visited < maxNodes, ProcessInfo.processInfo.systemUptime < deadline {
            let (element, depth) = queue.removeFirst(); visited += 1
            if let raw = try nativeAXAttribute(element, kAXIdentifierAttribute, admission: admission),
               try admission.text(raw) == "f02.sample.\(identifier)" { matches.append(element) }
            if depth < maxDepth {
                let omitted = try nativeAXArray(element, kAXChildrenAttribute, windows: false,
                    capacity: max(0, maxNodes - visited - queue.count), admission: admission) { child in
                        if handles.contains(where: { CFEqual($0, child) }) { duplicates += 1 }
                        else { handles.append(child); queue.append((child, depth + 1)) }
                    }
                guard omitted == 0 else { throw NativeAcquisitionError.limit }
            }
        }
        guard queue.isEmpty, matches.count == 1 else { throw NativeAcquisitionError.limit }
        let element = matches[0]
        func text(_ name: String) throws -> String? {
            guard let raw = try nativeAXAttribute(element, name, admission: admission) else { return nil }
            return try? admission.text(raw)
        }
        let role = try text(kAXRoleAttribute), name = try text(kAXDescriptionAttribute)
        let enabled = try nativeAXAttribute(element, kAXEnabledAttribute, admission: admission) as? NSNumber
        let position = try nativeAXAttribute(element, kAXPositionAttribute, admission: admission)
        let size = try nativeAXAttribute(element, kAXSizeAttribute, admission: admission)
        func property(_ field: String, state: () throws -> [String: Any]) throws -> [String: Any] {
            try json.object(["selection", "field", "sensitivity", "evidence", "state"]) {
                ["selection": try json.scalar("requested"), "field": try json.scalar(field),
                 "sensitivity": try json.scalar("public"),
                 "evidence": try json.evidence(observation, "macos.ax", "public_ax_attribute"), "state": try state()]
            }
        }
        let nodes = try json.array { [try json.object(["key", "surface", "native_role", "properties", "children", "extensions", "source_declarations"]) {
            ["key": try json.object(["namespace", "key"]) {
                ["namespace": try json.scalar("macos.ax"), "key": try json.scalar("f02.sample.\(identifier)")]
             }, "surface": try json.borrowed(surface),
             "native_role": try role.map { try json.known("text", $0) } ?? json.unavailable("role_unavailable"),
             "properties": try json.array {
                [try property("role") { try role == "AXButton" ? json.known("role", "button") : json.unavailable("role_mapping_unavailable") },
                 try property("accessibility_name") { try name.map { try json.known("text", $0) } ?? json.unavailable("description_unavailable") },
                 try property("enabled") { try enabled.map { try json.known("flag", $0.boolValue) } ?? json.unavailable("enabled_unavailable") },
                 try property("accessibility_bounds") { try nativeAXGeometry(position, size, json: json) }]
             }, "children": try json.array { [Any]() }, "extensions": try json.array { [Any]() },
             "source_declarations": try json.array { [Any]() }]
        }] }
        return WindowAXResult(nodes: nodes, metrics: ["visited_unique_nodes": visited, "returned_nodes": 1,
            "discovered_unique_handles": handles.count, "duplicate_handle_references": duplicates,
            "node_ceiling": maxNodes, "depth_ceiling": maxDepth, "coverage": "partial"])
    }

    static func snapshot(context: [String: Any], surface: [String: Any], target: [String: Any],
        scope: String, fields: [String], observation: String, channel: String, started: Double,
        nodes: [[String: Any]], captures: [[String: Any]], json: NativeJSON) throws -> [String: Any] {
        let source = channel == "external_semantics" ? "macos.ax" : "macos.screencapturekit"
        let ended = ProcessInfo.processInfo.systemUptime
        func coverage() throws -> [String: Any] {
            try json.object(["status", "scope_id", "fields", "omitted_count", "unknown_count"]) {
                ["status": try json.scalar("partial"), "scope_id": try json.scalar(scope),
                 "fields": try json.borrowed(fields), "omitted_count": try json.scalar(NSNull()), "unknown_count": try json.scalar(NSNull())]
            }
        }
        func focusState() throws -> [String: Any] {
            try json.object(["status"]) { ["status": try json.scalar("not_requested")] }
        }
        return try json.object(["surface_records", "id", "revision", "source_state", "context", "observations", "nodes", "relations", "components", "focus", "captures", "coverage"]) {
            ["surface_records": try json.array { [try json.object(["identity", "native_owner", "initiated_by", "anchor", "evidence"]) {
                ["identity": try json.borrowed(surface), "native_owner": try json.known("identity", target),
                 "initiated_by": try json.scalar(NSNull()), "anchor": try json.scalar(NSNull()),
                 "evidence": try json.evidence(observation, source, "public_owner_and_fixture_binding")]
             }] }, "id": try json.scalar("\(observation)-snapshot"), "revision": try json.scalar(1),
             "source_state": try json.scalar(NSNull()), "context": try json.borrowed(context),
             "observations": try json.array { [try json.object(["id", "source_namespace", "channel", "start", "end", "clock_domain", "time_unit", "freshness_basis", "consistency_reason", "answer_source", "freshness", "last_verified", "consistency", "coverage"]) {
                ["id": try json.scalar(observation), "source_namespace": try json.scalar(source), "channel": try json.scalar(channel),
                 "start": try json.scalar(started), "end": try json.scalar(ended),
                 "clock_domain": try json.scalar("helper-\(ProcessInfo.processInfo.processIdentifier)-monotonic"),
                 "time_unit": try json.scalar("seconds"), "freshness_basis": try json.scalar("live_read"),
                 "consistency_reason": try json.scalar("separate_api_reads"), "answer_source": try json.scalar("live"),
                 "freshness": try json.scalar("current"), "last_verified": try json.scalar(ended),
                 "consistency": try json.scalar("unknown"), "coverage": try coverage()]
             }] }, "nodes": nodes, "relations": try json.array { [Any]() }, "components": try json.array { [Any]() },
             "focus": try json.object(["keyboard", "accessibility", "active_descendant", "text_selection", "composition_state"]) {
                ["keyboard": try focusState(), "accessibility": try focusState(), "active_descendant": try focusState(),
                 "text_selection": try json.scalar(NSNull()), "composition_state": try json.object(["selection", "field"]) {
                    ["selection": try json.scalar("not_requested"), "field": try json.scalar("value")]
                 }]
             }, "captures": captures, "coverage": try coverage()]
        }
    }
    static func captureRecord(_ capture: OwnedCapture, surface: [String: Any], observation: String,
                              payload: String, json: NativeJSON) throws -> [String: Any] {
        try json.object(["observation_id", "capture_target", "capture_kind", "pixel_width", "pixel_height", "crop_transform", "included_surfaces", "excluded_surfaces", "unresolved_surfaces", "surface_coverage", "captures_audio", "filter_id", "payload_ref"]) {
            ["observation_id": try json.scalar(observation), "capture_target": try json.borrowed(surface),
             "capture_kind": try json.scalar("window_isolated"), "pixel_width": try json.scalar(capture.image.width),
             "pixel_height": try json.scalar(capture.image.height), "crop_transform": try json.object(["status", "reason"]) {
                ["status": try json.scalar("unknown"), "reason": try json.scalar("frame_mapping_not_calibrated")]
             }, "included_surfaces": try json.borrowed([surface]), "excluded_surfaces": try json.array { [Any]() },
             "unresolved_surfaces": try json.array { [Any]() }, "surface_coverage": try json.scalar("partial"),
             "captures_audio": try json.scalar(false), "filter_id": try json.scalar("desktopIndependentWindow-no-children-no-audio"),
             "payload_ref": try json.scalar(payload)]
        }
    }
}

#if HOST_HELPER
extension Collector {
    // Explicit own-fixture snapshot import; its measurement clock/time are retained.
    // No UI recollection, expected.json oracle or derived-gap calculation here.
    @MainActor static func probe(data: Data, command: NativeCommand) throws -> NativeJSONFrame {
        let config = command.configuration
        let json = NativeJSON(config.acquisition_limits)
        let frame = try NativeJSONFrame(capacity: command.replyCap, deadline: command.deadline)
        guard let artifact = command.document["artifact"] as? [String: Any],
              let request = artifact["data"] as? [String: Any],
              let context = request["context"] as? [String: Any], let requestID = request["request_id"] as? String,
              let scope = context["scope_id"] as? String, let target = context["target"] as? [String: Any],
              let surfaces = context["surfaces"] as? [[String: Any]], surfaces.count == 1,
              context["fields"] as? [String] == ["layout_bounds"],
              let limits = request["limits"] as? [String: Any], let maxNodes = limits["max_elements"] as? Int,
              data.count <= command.replyCap else { throw NativeProtocolError.request }
        func failed(_ code: String) throws -> NativeJSONFrame {
            try frame.encode(json.response(request: request, ticket: command.control.ticket, channel: "opt_in_layout_probe") {
                try json.failure(code, scope: scope, channel: "opt_in_layout_probe")
            })
            return frame
        }
        guard let manifest = try JSONSerialization.jsonObject(with: data) as? [String: Any],
              let state = manifest["source_state"] as? [String: Any],
              manifest["pid"] as? Int32 == config.binding.pid,
              manifest["bundle_id"] as? String == config.binding.bundle_id,
              manifest["launch_time"] as? Double == config.binding.launch_time,
              manifest["window_id"] as? UInt32 == config.binding.window_id,
              manifest["window_identifier"] as? String == config.binding.window_identifier,
              manifest["target_generation"] as? String == config.binding.target_generation,
              manifest["surface_generation"] as? String == config.binding.surface_generation,
              manifest["snapshot_request"] as? Int == config.probe_snapshot_request,
              state["revision"] as? Int == config.probe_source_revision,
              manifest["uptime_seconds"] as? Double == config.probe_uptime,
              manifest["collection_mode"] as? String == "explicit_request_only"
        else { return try failed("stale_target") }
        guard manifest["probe_enabled"] as? Bool == true else { return try failed("unsupported") }
        // An imported explicit Snapshot is historical until an actual continuity
        // consumer proves current state; PID/Snapshot equality alone is insufficient.
        guard request["freshness_policy"] as? String == "cached_allowed" else { return try failed("stale_target") }
        guard let measured = manifest["probe"] as? [String: Any],
              measured["source"] as? String == "swiftui.anchorPreference.explicit_snapshot",
              measured["provenance"] as? String == "reported", measured["units"] as? String == "pt",
              measured["origin"] as? String == "top_left", measured["coordinate_space"] as? String == "fixture_local",
              measured["screen_transform"] as? String == "unknown",
              let bounds = measured["layout_bounds"] as? [String: Any], Set(bounds.keys) == Set(["icon", "text", "container"]),
              let declarations = manifest["source_declarations"] as? [String: Any],
              declarations["logical_component_key"] as? String == "f02.sample.\(config.binding.window_identifier)",
              declarations["represents"] as? [String] == ["icon", "text", "container"], maxNodes >= 3,
              let time = config.probe_uptime, time.isFinite
        else { return try failed("incomplete_scope") }
        let admission = try NativeAcquisition(config.acquisition_limits, deadline: command.deadline)
        let source = "macos.swiftui.probe", oid = "\(requestID)-opt_in_layout_probe"
        let surface = surfaces[0]
        func key(_ marker: String) throws -> [String: Any] {
            try json.object(["namespace", "key"]) { ["namespace": try json.scalar(source), "key": try json.scalar("f02.sample.\(config.binding.window_identifier).\(marker)")] }
        }
        let nodes = try json.array {
            try ["icon", "text", "container"].map { marker -> [String: Any] in
                try admission.check()
                guard let rect = bounds[marker] as? [String: Any], Set(rect.keys) == Set(["x", "y", "width", "height"]),
                      let x = rect["x"] as? Double, let y = rect["y"] as? Double,
                      let width = rect["width"] as? Double, let height = rect["height"] as? Double,
                      [x,y,width,height].allSatisfy({ $0.isFinite }), width >= 0, height >= 0 else { throw NativeAcquisitionError.invalidValue }
                return try json.object(["key", "surface", "native_role", "properties", "children", "extensions", "source_declarations"]) {
                    ["key": try key(marker), "surface": try json.borrowed(surface), "native_role": try json.unavailable("probe_not_ax", status: "unsupported"),
                     "properties": try json.array { [try json.object(["selection", "field", "sensitivity", "evidence", "state"]) {
                        ["selection": try json.scalar("requested"), "field": try json.scalar("layout_bounds"), "sensitivity": try json.scalar("public"),
                         "evidence": try json.evidence(oid, source, "swiftui_anchorPreference_explicit_snapshot"),
                         "state": try json.knownBuilt("geometry") {
                            try json.object(["frame_kind", "coordinate_space", "shape", "transform"]) {
                                ["frame_kind": try json.scalar("layout_bounds"),
                                 "coordinate_space": try json.object(["id", "kind", "units", "origin"]) {
                                    ["id": try json.scalar("f02-fixture-local"), "kind": try json.scalar("local"), "units": try json.scalar("pt"), "origin": try json.scalar("top_left")]
                                 }, "shape": try json.object(["shape", "value"]) { ["shape": try json.scalar("rect"), "value": try json.borrowed(rect)] },
                                 "transform": try json.object(["status", "reason"]) { ["status": try json.scalar("unknown"), "reason": try json.scalar("fixture_screen_transform_unverified")] }]
                            }
                         }]
                     }] }, "children": try json.array { [Any]() }, "extensions": try json.array { [Any]() }, "source_declarations": try json.array { [Any]() }]
                }
            }
        }
        var snapshot = try self.snapshot(context: context, surface: surface, target: target, scope: scope,
            fields: ["layout_bounds"], observation: oid, channel: "opt_in_layout_probe", started: time,
            nodes: nodes, captures: try json.array { [] }, json: json)
        // Replace generic helper observation with actual fixture clock/time/source.
        var observation = (snapshot["observations"] as! [[String: Any]])[0]
        for (name, value) in ["source_namespace": source, "clock_domain": "fixture-\(config.binding.pid)-monotonic",
                              "freshness_basis": "unverified", "freshness": "unverified", "answer_source": "cache",
                              "consistency_reason": "explicit_fixture_snapshot_not_atomic_os_state"] {
            observation[name] = try json.scalar(value)
        }
        observation["end"] = try json.scalar(time); observation["last_verified"] = try json.scalar(NSNull())
        snapshot["observations"] = try json.array { [observation] }
        snapshot["surface_records"] = try json.array { [try json.object(["identity", "native_owner", "initiated_by", "anchor", "evidence"]) {
            ["identity": try json.borrowed(surface), "native_owner": try json.known("identity", target),
             "initiated_by": try json.scalar(NSNull()), "anchor": try json.scalar(NSNull()),
             "evidence": try json.evidence(oid, source, "fixture_manifest_binding")]
        }] }
        snapshot["source_state"] = try json.scalar("fixture-source-revision-\(config.probe_source_revision!)")
        snapshot["components"] = try json.array { [try json.object(["logical_component_key", "members", "declaration_source", "provenance"]) {
            ["logical_component_key": try json.scalar("f02.sample.\(config.binding.window_identifier)"),
             "members": try json.array { try ["icon","text","container"].map(key) },
             "declaration_source": try json.scalar("f02_explicit_component_mapping"), "provenance": try json.scalar("reported")]
        }] }
        try frame.encode(json.response(request: request, ticket: command.control.ticket, channel: "opt_in_layout_probe") {
            try json.object(["status", "data"]) { ["status": try json.scalar("observed"), "data": snapshot] }
        })
        return frame
    }
}

#endif
