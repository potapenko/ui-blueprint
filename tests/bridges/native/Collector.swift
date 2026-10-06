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
@discardableResult func emit(_ kind: String, _ data: [String: Any], limit: Int = 1_048_576) throws -> Int {
    var bytes = try JSONSerialization.data(withJSONObject: ["schema_version": "0.1.0", "artifact": ["kind": kind, "data": data]], options: [.sortedKeys])
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
@main struct Collector {
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
              doc["schema_version"] as? String == "0.1.0",
              let artifact = doc["artifact"] as? [String: Any], artifact["kind"] as? String == "request",
              let request = artifact["data"] as? [String: Any], let context = request["context"] as? [String: Any],
              let target = context["target"] as? [String: Any], let surfaces = context["surfaces"] as? [[String: Any]], surfaces.count == 1,
              let fields = context["fields"] as? [String], fields == ["role", "accessibility_name", "enabled", "accessibility_bounds"],
              let scope = context["scope_id"] as? String, let session = context["session_id"] as? String,
              let requestID = request["request_id"] as? String,
              let operation = request["operation"] as? [String: Any], operation["operation"] as? String == "observe",
              operation["channels"] as? [String] == ["external_semantics", "rendered_capture"],
              let limits = request["limits"] as? [String: Any],
              let maxNodes = limits["max_elements"] as? Int, (1...160).contains(maxNodes),
              let maxDepth = limits["max_depth"] as? Int, (1...9).contains(maxDepth),
              let deadlineMS = limits["deadline_ms"] as? Double, deadlineMS > 0,
              let outputLimit = limits["max_output_bytes"] as? Int, (1...1_048_576).contains(outputLimit),
              let manifest = try JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: args[1]))) as? [String: Any],
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
        let axAllowed = AXIsProcessTrusted()
        let captureAllowed = CGPreflightScreenCaptureAccess()
        func capability(_ channel: String, _ allowed: Bool) -> [String: Any] {
            ["channel": channel, "operation": "observe", "status": allowed ? "partial" : "permission_required",
             "reason": allowed ? "fixture_scope_partial" : "permission_not_granted"]
        }
        if sequence == nil {
        try emit("session", ["allowed_scopes": [scope], "session_id": session, "plugin": context["plugin"]!,
            "supported_versions": ["0.1.0"], "target": target, "surfaces": surfaces,
            "capabilities": [capability("external_semantics", axAllowed), capability("rendered_capture", captureAllowed)]])
        }
        if args[3] == "describe" { return } // No AX tree or pixels acquired.
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
        func reply(_ channel: String, _ result: [String: Any]) throws {
            if let sequence {
                responseBytes += try emit("channel_response", ["request_id": requestID, "session_id": session, "dispatch_sequence": sequence,
                    "target": target, "channel": channel, "result": result], limit: outputLimit - responseBytes)
            } else {
                // Standalone channel check, explicitly not a Ticket/lifecycle proof.
                guard let data = result["data"] as? [String: Any] else { exit(2) }
                responseBytes += try emit(result["status"] as? String == "observed" ? "snapshot" : "error", data, limit: outputLimit - responseBytes)
            }
        }
        func failed(_ code: String, _ channel: String) throws {
            try reply(channel, ["status": "failed", "data": ["code": code, "scope_id": scope,
                "failed_step": channel, "recovery_class": "new_explicit_request"]])
        }
        let axStart = ProcessInfo.processInfo.systemUptime
        if !axAllowed { try failed("permission_required", "external_semantics") }
        else {
            let root = AXUIElementCreateApplication(pid)
            let windows = (attribute(root, kAXWindowsAttribute) as? [AXUIElement] ?? []).filter {
                attribute($0, kAXIdentifierAttribute) as? String == identifier
            }
            guard windows.count == 1 else { throw NSError(domain: "ambiguous_window", code: 2) }
            var queue: [(AXUIElement, Int)] = [(windows[0], 0)]
            var matches: [AXUIElement] = []; var visited = 0
            while !queue.isEmpty, visited < maxNodes, ProcessInfo.processInfo.systemUptime - axStart < min(0.9, deadlineMS / 1000) {
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
        // AX frame is already flushed. Injected timeout never invokes capture APIs.
        if args[3] != "live" { try await Task.sleep(for: .seconds(6)); return }
        let captureStart = ProcessInfo.processInfo.systemUptime
        if !captureAllowed { try failed("permission_required", "rendered_capture"); return }
        let content = try await SCShareableContent.excludingDesktopWindows(true, onScreenWindowsOnly: false)
        guard let window = content.windows.first(where: { $0.windowID == wid && $0.owningApplication?.processID == pid }) else {
            throw NSError(domain: "capture_target_unresolved", code: 4)
        }
        let filter = SCContentFilter(desktopIndependentWindow: window)
        let config = SCStreamConfiguration(); config.capturesAudio = false; config.showsCursor = false
        config.ignoreShadowsSingleWindow = true
        config.width = Int((filter.contentRect.width * CGFloat(filter.pointPixelScale)).rounded())
        config.height = Int((filter.contentRect.height * CGFloat(filter.pointPixelScale)).rounded())
        if #available(macOS 14.2, *) { config.includeChildWindows = false }
        let image = try await SCScreenshotManager.captureImage(contentFilter: filter, configuration: config)
        guard NSRunningApplication(processIdentifier: pid)?.launchDate?.timeIntervalSince1970 == launch else { throw NSError(domain: "stale_target", code: 5) }
        let url = URL(fileURLWithPath: args[2]).appendingPathComponent("capture.png")
        guard let dest = CGImageDestinationCreateWithURL(url as CFURL, UTType.png.identifier as CFString, 1, nil) else { exit(1) }
        CGImageDestinationAddImage(dest, image, nil); guard CGImageDestinationFinalize(dest) else { exit(1) }
        let metadata: [String: Any] = ["observation_id": "\(requestID)-rendered_capture",
            "source": "ScreenCaptureKit filter_and_window_metadata", "units": "pt", "origin": "top_left",
            "filter_content_rect": ["x": filter.contentRect.minX, "y": filter.contentRect.minY,
                "width": filter.contentRect.width, "height": filter.contentRect.height],
            "filter_point_pixel_scale": filter.pointPixelScale,
            "window_id": wid, "window_frame": ["x": window.frame.minX, "y": window.frame.minY,
                "width": window.frame.width, "height": window.frame.height],
            "capture_call_start": captureStart, "capture_call_end": ProcessInfo.processInfo.systemUptime,
            "clock_domain": clock, "time_unit": "seconds", "transform_status": "unknown"]
        try JSONSerialization.data(withJSONObject: metadata, options: [.sortedKeys]).write(
            to: URL(fileURLWithPath: args[2]).appendingPathComponent("capture-metadata.json"))
        let capture: [String: Any] = ["observation_id": "\(requestID)-rendered_capture", "capture_target": surface,
            "capture_kind": "window_isolated", "pixel_width": image.width, "pixel_height": image.height,
            "crop_transform": ["status": "unknown", "reason": "frame_mapping_not_calibrated"],
            "included_surfaces": [surface], "excluded_surfaces": [], "unresolved_surfaces": [], "surface_coverage": "partial",
            "captures_audio": false, "filter_id": "desktopIndependentWindow-no-children-no-audio", "payload_ref": "capture.png"]
        try reply("rendered_capture", ["status": "observed", "data": snapshot("rendered_capture", "macos.screencapturekit", captureStart, [], [capture])])
    }
}
