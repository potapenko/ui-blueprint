import AppKit
import ApplicationServices
import ScreenCaptureKit
import ImageIO
import UniformTypeIdentifiers

// Test-only binding to Stage A Document/Session/ChannelResponse JSON records.
// No separate public DTO/schema; Rust validates every emitted canonical frame.
let null = NSNull()
func known(_ type: String, _ value: Any) -> [String: Any] {
    ["availability": "known", "value": ["type": type, "value": value]]
}
func unavailable(_ reason: String) -> [String: Any] { ["availability": "unknown", "reason": reason] }
func canonicalBytes(_ kind: String, _ data: [String: Any]) throws -> Data {
    try JSONSerialization.data(withJSONObject: ["schema_version": "0.1.0", "artifact": ["kind": kind, "data": data]], options: [.sortedKeys])
}
func channelResponse(requestID: String, sessionID: String, target: [String: Any],
                     sequence: UInt64, channel: String, result: [String: Any]) -> [String: Any] {
    ["request_id": requestID, "session_id": sessionID, "dispatch_sequence": sequence,
     "target": target, "channel": channel, "result": result]
}
func channelFailure(code: String, scope: String, channel: String) -> [String: Any] {
    ["status": "failed", "data": ["code": code, "scope_id": scope, "failed_step": channel,
     "recovery_class": code == "permission_required" ? "explicit_permission" : "new_explicit_request"]]
}
@discardableResult func emit(_ kind: String, _ data: [String: Any], limit: Int = 1_048_576) throws -> Int {
    var bytes = try canonicalBytes(kind, data)
    guard bytes.count <= limit else { throw NSError(domain: "response_limit", code: 8) }
    let count = bytes.count
    bytes.append(10)
    try FileHandle.standardOutput.write(contentsOf: bytes)
    return count
}
func attribute(_ el: AXUIElement, _ key: String) -> CFTypeRef? {
    AXUIElementSetMessagingTimeout(el, 0.15)
    var value: CFTypeRef?
    return AXUIElementCopyAttributeValue(el, key as CFString, &value) == .success ? value : nil
}
#if !HOST_HELPER
@main
#endif
struct Collector {
    @MainActor static func main() async {
        DispatchQueue.global().asyncAfter(deadline: .now() + 8) { _exit(124) }
        do { try await run() } catch { fputs("native proof collector failed; no raw UI payload in diagnostics\n", stderr); exit(1) }
    }
    @MainActor static func run() async throws {
        let args = CommandLine.arguments
        guard args.count == 4 || args.count == 5 else { exit(2) }
        // Optional sequence must be supplied by the committed common host.
        // Without it only standalone Snapshot/Error artifacts are emitted.
        let sequence: UInt64? = args.count == 5 ? UInt64(args[4]) : nil
        if args.count == 5 && (sequence == nil || sequence == 0) { exit(2) }
        // Explicit finite test frame bound, before JSON parsing. One request only.
        var bytes = Data()
        while let byte = try FileHandle.standardInput.read(upToCount: 1), !byte.isEmpty {
            if byte[0] == 10 { break }
            bytes.append(byte)
            guard bytes.count <= 1_048_576 else { exit(2) }
        }
        guard let doc = try JSONSerialization.jsonObject(with: bytes) as? [String: Any],
              let manifest = try JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: args[1]))) as? [String: Any]
        else { throw NSError(domain: "invalid_request", code: 1) }
        try await collect(document: doc, manifest: manifest, directory: URL(fileURLWithPath: args[2]),
                          mode: args[3], sequence: sequence)
    }

    // The legacy wrapper and H01 entrypoint share acquisition/response code.
    // Selected H01 channels never perform the other channel's API/preflight.
    @MainActor static func collect(document doc: [String: Any], manifest: [String: Any],
        directory: URL?, mode: String, sequence: UInt64?, selectedChannel: String? = nil,
        deadline: Double? = nil, nativeEmit: ((Data) throws -> Void)? = nil) async throws {
        let windowMode = ["window-ax", "describe-window"].contains(mode)
        let expectedFields = windowMode
            ? ["role", "description", "value", "placeholder", "enabled", "focused", "actions", "accessibility_bounds"]
            : ["role", "accessibility_name", "enabled", "accessibility_bounds"]
        let expectedChannels = windowMode ? ["external_semantics"] : ["external_semantics", "rendered_capture"]
        func checkDeadline() throws {
            if let deadline, ProcessInfo.processInfo.systemUptime >= deadline { throw NSError(domain: "deadline", code: 1) }
        }
        try checkDeadline()
        guard doc["schema_version"] as? String == "0.1.0",
              let artifact = doc["artifact"] as? [String: Any], artifact["kind"] as? String == "request",
              let request = artifact["data"] as? [String: Any], let context = request["context"] as? [String: Any],
              let target = context["target"] as? [String: Any], let surfaces = context["surfaces"] as? [[String: Any]], surfaces.count == 1,
              let fields = context["fields"] as? [String], fields == expectedFields,
              let scope = context["scope_id"] as? String, let session = context["session_id"] as? String,
              let requestID = request["request_id"] as? String,
              let operation = request["operation"] as? [String: Any], operation["operation"] as? String == "observe",
              let channels = operation["channels"] as? [String],
              selectedChannel.map({ channels.contains($0) }) ?? (channels == expectedChannels),
              let limits = request["limits"] as? [String: Any],
              let maxNodes = limits["max_elements"] as? Int, (1...160).contains(maxNodes),
              let maxDepth = limits["max_depth"] as? Int, (1...9).contains(maxDepth),
              let deadlineMS = limits["deadline_ms"] as? Double, deadlineMS > 0,
              let outputLimit = limits["max_output_bytes"] as? Int, (1...1_048_576).contains(outputLimit),
              let pid = manifest["pid"] as? Int32, let bundle = manifest["bundle_id"] as? String,
              ["local.uiblueprint.f02.on", "local.uiblueprint.f02.off"].contains(bundle),
              let launch = manifest["launch_time"] as? Double, let wid = manifest["window_id"] as? UInt32,
              let identifier = manifest["window_identifier"] as? String, ["a", "b"].contains(identifier),
              let app = NSRunningApplication(processIdentifier: pid), app.bundleIdentifier == bundle,
              app.launchDate?.timeIntervalSince1970 == launch,
              target["generation"] as? String == manifest["target_generation"] as? String,
              surfaces[0]["generation"] as? String == manifest["surface_generation"] as? String,
              target["id"] as? String == "f02-pid-\(pid)", surfaces[0]["id"] as? String == "window-\(wid)",
              let inventory = CGWindowListCopyWindowInfo(.optionIncludingWindow, wid) as? [[String: Any]], inventory.count == 1,
              inventory[0][kCGWindowOwnerPID as String] as? Int32 == pid
        else { throw NSError(domain: "invalid_or_unresolved_target", code: 1) }
        let surface = surfaces[0]
        let wantsAX = selectedChannel == nil || selectedChannel == "external_semantics"
        let wantsCapture = selectedChannel.map { $0 == "rendered_capture" } ?? !windowMode
        let axAllowed = wantsAX ? AXIsProcessTrusted() : false
        let captureAllowed = wantsCapture ? CGPreflightScreenCaptureAccess() : false
        func capability(_ channel: String, _ allowed: Bool) -> [String: Any] {
            ["channel": channel, "operation": "observe", "status": allowed ? "partial" : "permission_required",
             "reason": allowed ? "fixture_scope_partial" : "permission_not_granted"]
        }
        if sequence == nil {
        try emit("session", ["allowed_scopes": [scope], "session_id": session, "plugin": context["plugin"]!,
            "supported_versions": ["0.1.0"], "target": target, "surfaces": surfaces,
            "capabilities": windowMode ? [capability("external_semantics", axAllowed)] : [capability("external_semantics", axAllowed), capability("rendered_capture", captureAllowed)]])
        }
        if mode == "describe" || mode == "describe-window" { return } // No AX tree or pixels acquired.
        let clock = "helper-\(ProcessInfo.processInfo.processIdentifier)-monotonic"
        let coverage: [String: Any] = ["status": "partial", "scope_id": scope, "fields": fields, "omitted_count": null, "unknown_count": null]
        func evidence(_ id: String, _ source: String, _ method: String) -> [String: Any] {
            ["observation_id": id, "source_namespace": source, "provenance": "reported", "method": method, "uncertainty": null]
        }
        func snapshot(_ channel: String, _ source: String, _ start: Double, _ nodes: [[String: Any]], _ captures: [[String: Any]]) -> [String: Any] {
            let end = ProcessInfo.processInfo.systemUptime
            let oid = "\(requestID)-\(channel)"
            return ["surface_records": [["identity": surface, "native_owner": known("identity", target),
                "initiated_by": null, "anchor": null, "evidence": evidence(oid, source, "public_owner_and_fixture_binding")]],
                "id": "\(oid)-snapshot", "revision": 1, "source_state": null, "context": context,
                "observations": [["id": oid, "source_namespace": source, "channel": channel, "start": start, "end": end,
                    "clock_domain": clock, "time_unit": "seconds", "freshness_basis": "live_read", "consistency_reason": "separate_api_reads",
                    "answer_source": "live", "freshness": "current", "last_verified": end, "consistency": "unknown", "coverage": coverage]],
                "nodes": nodes, "relations": [], "components": [],
                "focus": ["keyboard": ["status": "not_requested"], "accessibility": ["status": "not_requested"],
                    "active_descendant": ["status": "not_requested"], "text_selection": null,
                    "composition_state": ["selection": "not_requested", "field": "value"]],
                "captures": captures, "coverage": coverage]
        }
        var responseBytes = 0
        var responseCount = 0
        func publish(_ kind: String, _ data: [String: Any], limit: Int) throws -> Int {
            try checkDeadline()
            if let nativeEmit {
                guard responseCount == 0 else { throw NSError(domain: "duplicate_response", code: 1) }
                let bytes = try canonicalBytes(kind, data)
                guard bytes.count <= limit else { throw NSError(domain: "response_limit", code: 8) }
                try nativeEmit(bytes)
                responseCount += 1
                return bytes.count
            }
            return try emit(kind, data, limit: limit)
        }
        func reply(_ channel: String, _ result: [String: Any]) throws {
            if let sequence {
                responseBytes += try publish("channel_response", channelResponse(requestID: requestID, sessionID: session, target: target,
                    sequence: sequence, channel: channel, result: result), limit: outputLimit - responseBytes)
            } else {
                // Standalone channel check, explicitly not a Ticket/lifecycle proof.
                guard let data = result["data"] as? [String: Any] else { exit(2) }
                responseBytes += try publish(result["status"] as? String == "observed" ? "snapshot" : "error", data, limit: outputLimit - responseBytes)
            }
        }
        func failed(_ code: String, _ channel: String) throws {
            try reply(channel, channelFailure(code: code, scope: scope, channel: channel))
        }
        if wantsAX {
            let axStart = ProcessInfo.processInfo.systemUptime
            if !axAllowed { try failed("permission_required", "external_semantics") }
            else {
                let root = AXUIElementCreateApplication(pid)
                let windows = (attribute(root, kAXWindowsAttribute) as? [AXUIElement] ?? []).filter {
                    attribute($0, kAXIdentifierAttribute) as? String == identifier
                }
                guard windows.count == 1 else { throw NSError(domain: "ambiguous_window", code: 2) }
                if windowMode {
                    let collected = collectWindowAX(windows[0], surface: surface,
                        observationID: "\(requestID)-external_semantics", maxNodes: maxNodes, maxDepth: maxDepth,
                        deadline: min(deadline ?? .infinity, axStart + min(0.9, deadlineMS / 1000)))
                    if nativeEmit == nil, let directory {
                        try JSONSerialization.data(withJSONObject: collected.metrics, options: [.sortedKeys]).write(
                            to: directory.appendingPathComponent("acquisition.json"))
                    }
                    try reply("external_semantics", ["status": "observed", "data": snapshot("external_semantics", "macos.ax", axStart, collected.nodes, [])])
                    return
                }
                var queue: [(AXUIElement, Int)] = [(windows[0], 0)]
                var matches: [AXUIElement] = []; var visited = 0
                while !queue.isEmpty, visited < maxNodes, ProcessInfo.processInfo.systemUptime < min(deadline ?? .infinity, axStart + min(0.9, deadlineMS / 1000)) {
                    let (el, depth) = queue.removeFirst(); visited += 1
                    if attribute(el, kAXIdentifierAttribute) as? String == "f02.sample.\(identifier)" { matches.append(el) }
                    if depth < maxDepth, let children = attribute(el, kAXChildrenAttribute) as? [AXUIElement] {
                        queue.append(contentsOf: children.prefix(max(0, maxNodes - visited - queue.count)).map { ($0, depth + 1) })
                    }
                }
                guard queue.isEmpty, matches.count == 1 else { throw NSError(domain: "incomplete_or_ambiguous_sample", code: 3) }
                let el = matches[0]; let oid = "\(requestID)-external_semantics"
                func property(_ field: String, _ state: [String: Any]) -> [String: Any] {
                    ["selection": "requested", "field": field, "sensitivity": "public",
                     "evidence": evidence(oid, "macos.ax", "public_ax_attribute"), "state": state]
                }
                let nativeRole = attribute(el, kAXRoleAttribute) as? String
                let name = attribute(el, kAXDescriptionAttribute) as? String
                let enabled = attribute(el, kAXEnabledAttribute) as? NSNumber
                var bounds = unavailable("ax_bounds_unavailable")
                if let p = attribute(el, kAXPositionAttribute), CFGetTypeID(p) == AXValueGetTypeID(),
                   let z = attribute(el, kAXSizeAttribute), CFGetTypeID(z) == AXValueGetTypeID() {
                    var point = CGPoint.zero; var size = CGSize.zero
                    if AXValueGetValue(unsafeDowncast(p, to: AXValue.self), .cgPoint, &point),
                       AXValueGetValue(unsafeDowncast(z, to: AXValue.self), .cgSize, &size) {
                        bounds = known("geometry", ["frame_kind": "accessibility_bounds",
                            "coordinate_space": ["id": "ax-screen", "kind": "screen", "units": "pt", "origin": "top_left"],
                            "shape": ["shape": "rect", "value": ["x": point.x, "y": point.y, "width": size.width, "height": size.height]],
                            "transform": ["status": "unknown", "reason": "ax_to_pixels_not_calibrated"]])
                    }
                }
                let node: [String: Any] = ["key": ["namespace": "macos.ax", "key": "f02.sample.\(identifier)"], "surface": surface,
                    "native_role": nativeRole.map { known("text", $0) } ?? unavailable("role_unavailable"),
                    "properties": [property("role", nativeRole == "AXButton" ? known("role", "button") : unavailable("role_mapping_unavailable")),
                        property("accessibility_name", name.map { known("text", $0) } ?? unavailable("description_unavailable")),
                        property("enabled", enabled.map { known("flag", $0.boolValue) } ?? unavailable("enabled_unavailable")),
                        property("accessibility_bounds", bounds)], "children": [], "extensions": [], "source_declarations": []]
                try reply("external_semantics", ["status": "observed", "data": snapshot("external_semantics", "macos.ax", axStart, [node], [])])
            }
        }
        if !wantsCapture { return }
        try checkDeadline()
        // AX frame is already flushed. Injected timeout never invokes capture APIs.
        if selectedChannel == nil && mode != "live" { try await Task.sleep(for: .seconds(6)); return }
        let captureStart = ProcessInfo.processInfo.systemUptime
        if !captureAllowed && (selectedChannel != nil || ProcessInfo.processInfo.environment["UIB_CAPTURE_FAULT"] == nil) { try failed("permission_required", "rendered_capture"); return }
        guard let directory else { throw NSError(domain: "missing_owned_artifact_directory", code: 1) }
        if selectedChannel != nil {
            // Parent explicitly authorizes a new synthetic-fixture artifact directory.
            // Never overwrite existing output or accept a path from UI/request data.
            guard mkdir(directory.path, mode_t(0o700)) == 0 else { throw NSError(domain: "artifact_directory_unavailable", code: 1) }
        }
        let captured: OwnedCapture
        do { captured = try await CaptureLifecycle.capture(windowID: wid, pid: pid,
            budget: deadline.map { min(2, $0 - ProcessInfo.processInfo.systemUptime) } ?? 2,
            admission: selectedChannel == nil ? .legacyRun : .parentOwned) }
        catch {
            let issue = CaptureLifecycle.issue(error)
            let native = CaptureLifecycle.nativeError(error)
            if selectedChannel == nil {
                try JSONSerialization.data(withJSONObject: ["code": issue.code, "failed_step": issue.step,
                    "error_domain": native.domain, "error_code": native.code,
                    "injected": ProcessInfo.processInfo.environment["UIB_CAPTURE_FAULT"] != nil,
                    "requires_owned_helper_retirement": true], options: [.sortedKeys]).write(
                        to: directory.appendingPathComponent("capture-error.json"))
            }
            try failed(issue.code, "rendered_capture")
            return
        }
        let image = captured.image
        guard NSRunningApplication(processIdentifier: pid)?.launchDate?.timeIntervalSince1970 == launch else { throw NSError(domain: "stale_target", code: 5) }
        let url = directory.appendingPathComponent("capture.png")
        guard let dest = CGImageDestinationCreateWithURL(url as CFURL, UTType.png.identifier as CFString, 1, nil) else { exit(1) }
        CGImageDestinationAddImage(dest, image, nil); guard CGImageDestinationFinalize(dest) else { exit(1) }
        let metadata: [String: Any] = ["observation_id": "\(requestID)-rendered_capture",
            "source": "ScreenCaptureKit filter_and_window_metadata", "units": "pt", "origin": "top_left",
            "filter_content_rect": ["x": captured.filterRect.minX, "y": captured.filterRect.minY,
                "width": captured.filterRect.width, "height": captured.filterRect.height],
            "filter_point_pixel_scale": captured.scale,
            "capture_serialization": selectedChannel == nil ? "exclusive_run_owned_flock" : "parent_owned_capture_lease_until_reap", "capture_admission_wait_seconds": captured.admissionWait,
            "window_id": wid, "window_frame": ["x": captured.windowFrame.minX, "y": captured.windowFrame.minY,
                "width": captured.windowFrame.width, "height": captured.windowFrame.height],
            "capture_call_start": captureStart, "capture_call_end": ProcessInfo.processInfo.systemUptime,
            "clock_domain": clock, "time_unit": "seconds", "transform_status": "unknown"]
        try JSONSerialization.data(withJSONObject: metadata, options: [.sortedKeys]).write(
            to: directory.appendingPathComponent("capture-metadata.json"))
        let capture: [String: Any] = ["observation_id": "\(requestID)-rendered_capture", "capture_target": surface,
            "capture_kind": "window_isolated", "pixel_width": image.width, "pixel_height": image.height,
            "crop_transform": ["status": "unknown", "reason": "frame_mapping_not_calibrated"],
            "included_surfaces": [surface], "excluded_surfaces": [], "unresolved_surfaces": [], "surface_coverage": "partial",
            "captures_audio": false, "filter_id": "desktopIndependentWindow-no-children-no-audio", "payload_ref": "capture.png"]
        try reply("rendered_capture", ["status": "observed", "data": snapshot("rendered_capture", "macos.screencapturekit", captureStart, [], [capture])])
    }
}
