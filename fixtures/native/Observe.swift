import AppKit
import ApplicationServices
import ScreenCaptureKit
import ImageIO
import UniformTypeIdentifiers

// This diagnostic only accepts the manifest of the one-window owned fixture.
// It never widens unresolved window scope to application content.
private func rect(_ r: CGRect) -> [String: Double] {
    ["x": r.minX, "y": r.minY, "width": r.width, "height": r.height]
}
private func attr(_ element: AXUIElement, _ name: String) -> CFTypeRef? {
    AXUIElementSetMessagingTimeout(element, 0.2)
    var value: CFTypeRef?
    return AXUIElementCopyAttributeValue(element, name as CFString, &value) == .success ? value : nil
}
private func property(_ value: Any) -> [String: Any] {
    let cf = value as CFTypeRef
    if CFGetTypeID(cf) == CFNullGetTypeID() { return ["availability": "unknown"] }
    if CFGetTypeID(cf) == AXValueGetTypeID() {
        let ax = unsafeDowncast(cf, to: AXValue.self)
        switch AXValueGetType(ax) {
        case .cgPoint:
            var p = CGPoint.zero; AXValueGetValue(ax, .cgPoint, &p)
            return ["availability": "known", "value": ["x": p.x, "y": p.y]]
        case .cgSize:
            var s = CGSize.zero; AXValueGetValue(ax, .cgSize, &s)
            return ["availability": "known", "value": ["width": s.width, "height": s.height]]
        case .axError:
            var e = AXError.failure; AXValueGetValue(ax, .axError, &e)
            return ["availability": e == .attributeUnsupported ? "unsupported" : "unknown", "ax_error": e.rawValue]
        default: return ["availability": "unsupported"]
        }
    }
    if let s = value as? String { return ["availability": "known", "value": String(s.prefix(128))] }
    if let n = value as? NSNumber { return ["availability": "known", "value": n] }
    return ["availability": "unsupported"]
}
private func semantics(pid: pid_t, identifier: String, stimulus: String) -> [String: Any] {
    if stimulus == "permission_denied" { return ["status": "permission_required", "injection": "synthetic", "prompted": false] }
    let channelStart = ProcessInfo.processInfo.systemUptime
    if stimulus == "slow" { Thread.sleep(forTimeInterval: 0.3) }
    guard AXIsProcessTrusted() else { return ["status": "permission_required", "prompted": false] }
    let application = AXUIElementCreateApplication(pid)
    let rawWindows = attr(application, kAXWindowsAttribute)
    let matches = (rawWindows as? [AXUIElement])?.filter {
        attr($0, kAXIdentifierAttribute) as? String == identifier
    } ?? []
    guard matches.count == 1, let window = matches.first
    else { return ["status": "target_unresolved", "reason": "single_owned_window_binding_failed",
                   "candidate_count": (rawWindows as? [AXUIElement])?.count ?? -1,
                   "candidate_identifiers": (rawWindows as? [AXUIElement])?.prefix(4).map {
                       attr($0, kAXIdentifierAttribute) as? String ?? "unknown"
                   } ?? []] }
    let start = ProcessInfo.processInfo.systemUptime
    var queue: [(AXUIElement, Int, Int)] = [(window, -1, 0)]
    var nodes: [[String: Any]] = []
    let baseFields = [kAXRoleAttribute, kAXIdentifierAttribute, kAXTitleAttribute, kAXDescriptionAttribute,
                  kAXPositionAttribute, kAXSizeAttribute, kAXEnabledAttribute, kAXFocusedAttribute, "AXF02Unsupported"]
    while !queue.isEmpty, nodes.count < (stimulus == "partial" ? 8 : 160), ProcessInfo.processInfo.systemUptime - start < 3 {
        let (el, parent, depth) = queue.removeFirst()
        AXUIElementSetMessagingTimeout(el, 0.2)
        let elementID = attr(el, kAXIdentifierAttribute) as? String ?? ""
        // Only explicitly known non-secret fixture fields request AXValue.
        let fields = baseFields + (["f02.name", "f02.result", "f02.count", "f02.enabled"].contains(elementID) ? [kAXValueAttribute] : [])
        var values: CFArray?
        let error = AXUIElementCopyMultipleAttributeValues(el, fields as CFArray, [], &values)
        var props: [String: Any] = [:]
        if let values = values as? [Any] {
            for (name, value) in zip(fields, values) { props[name] = property(value) }
        }
        var actions: CFArray?
        let actionError = AXUIElementCopyActionNames(el, &actions)
        let index = nodes.count
        nodes.append(["key": "ax:\(index)", "parent_index": parent, "properties": props,
                      "batch_error": error.rawValue, "actions": actions as? [String] ?? [],
                      "actions_error": actionError.rawValue])
        if depth < 9, let children = attr(el, kAXChildrenAttribute) as? [AXUIElement] {
            queue.append(contentsOf: children.prefix(max(0, 160 - nodes.count - queue.count)).map { ($0, index, depth + 1) })
        }
    }
    return ["status": "observed", "coverage": "partial", "nodes": nodes,
            "source": "AXUIElement", "provenance": "reported", "units": "pt",
            "coordinate_space": "ax_screen", "origin": "top_left",
            "binding": "own_pid_launch_and_explicit_fixture_identifier",
            "design_internals": "not_exposed_unless_reported",
            "duration_seconds": ProcessInfo.processInfo.systemUptime - channelStart, "injection": stimulus == "normal" ? "none" : stimulus]
}
@main
struct Observe {
    @MainActor static func main() async {
        // Independent process watchdog; system requests cannot hold a global queue.
        DispatchQueue.global().asyncAfter(deadline: .now() + 45) { _exit(124) }
        let args = CommandLine.arguments
        guard args.count == 3 || args.count == 4 else { exit(2) }
        let manifestURL = URL(fileURLWithPath: args[1])
        let out = URL(fileURLWithPath: args[2], isDirectory: true)
        let sampleCount = args.count == 4 ? (Int(args[3]) ?? 0) : 1
        guard (1...30).contains(sampleCount) else { exit(2) }
        do {
            let data = try Data(contentsOf: manifestURL)
            guard let manifest = try JSONSerialization.jsonObject(with: data) as? [String: Any],
                  let pid = manifest["pid"] as? Int32,
                  let bundle = manifest["bundle_id"] as? String,
                  ["local.uiblueprint.f02.off", "local.uiblueprint.f02.on"].contains(bundle),
                  let launch = manifest["launch_time"] as? Double,
                  let application = NSRunningApplication(processIdentifier: pid),
                  application.bundleIdentifier == bundle,
                  application.launchDate?.timeIntervalSince1970 == launch,
                  let identifier = manifest["window_identifier"] as? String,
                  ["a", "b"].contains(identifier),
                  let wid = manifest["window_id"] as? UInt32,
                  let rows = CGWindowListCopyWindowInfo(.optionIncludingWindow, wid) as? [[String: Any]],
                  rows.count == 1, rows[0][kCGWindowOwnerPID as String] as? Int32 == pid
            else { throw NSError(domain: "target_unresolved", code: 1) }
            guard let livePath = manifest["live_manifest_path"] as? String,
                  let live = try JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: livePath))) as? [String: Any],
                  live["surface_generation"] as? String == manifest["surface_generation"] as? String,
                  live["target_generation"] as? String == manifest["target_generation"] as? String
            else { throw NSError(domain: "stale_target", code: 6) }
            let state = manifest["source_state"] as? [String: Any] ?? [:]
            let stimulus = state["stimulus"] as? String ?? "normal"
            for sampleIndex in 0..<sampleCount {
            let begin = ProcessInfo.processInfo.systemUptime
            var result: [String: Any] = ["environment": "task_owned_synthetic_debug",
                "requested_target": ["pid": pid, "window_id": wid, "launch_time": launch,
                                     "target_generation": manifest["target_generation"] ?? "unknown",
                                     "surface_generation": manifest["surface_generation"] ?? "unknown", "window_identifier": identifier],
                "observation_start_utc": ISO8601DateFormatter().string(from: Date()),
                "sample_index": sampleIndex, "source_state": state,
                "external_semantics": semantics(pid: pid, identifier: identifier, stimulus: stimulus)]
            let captureStart = ProcessInfo.processInfo.systemUptime
            if stimulus == "permission_denied" {
                result["rendered_capture"] = ["status": "permission_required", "injection": "synthetic", "prompted": false]
            } else if !CGPreflightScreenCaptureAccess() {
                result["rendered_capture"] = ["status": "permission_required", "prompted": false]
            } else {
                do {
                    let content = try await SCShareableContent.excludingDesktopWindows(true, onScreenWindowsOnly: false)
                    guard let window = content.windows.first(where: {
                        $0.windowID == wid && $0.owningApplication?.processID == pid
                    }) else { throw NSError(domain: "capture_target_unresolved", code: 2) }
                    let filter = SCContentFilter(desktopIndependentWindow: window)
                    let config = SCStreamConfiguration()
                    config.capturesAudio = false; config.showsCursor = false
                    config.ignoreShadowsSingleWindow = true
                    config.width = Int((filter.contentRect.width * CGFloat(filter.pointPixelScale)).rounded())
                    config.height = Int((filter.contentRect.height * CGFloat(filter.pointPixelScale)).rounded())
                    if #available(macOS 14.2, *) { config.includeChildWindows = false }
                    let image = try await SCScreenshotManager.captureImage(contentFilter: filter, configuration: config)
                    guard NSRunningApplication(processIdentifier: pid)?.launchDate?.timeIntervalSince1970 == launch else {
                        throw NSError(domain: "stale_target", code: 3)
                    }
                    if sampleIndex == 0 {
                    let imageURL = out.appendingPathComponent("window.png")
                    guard let dest = CGImageDestinationCreateWithURL(imageURL as CFURL, UTType.png.identifier as CFString, 1, nil) else {
                        throw NSError(domain: "capture_write_failed", code: 4)
                    }
                    CGImageDestinationAddImage(dest, image, nil)
                    guard CGImageDestinationFinalize(dest) else { throw NSError(domain: "capture_write_failed", code: 5) }
                    }
                    result["rendered_capture"] = ["status": "observed", "capture_kind": "window_isolated",
                        "capture_target": wid, "pixel_buffer_size": [image.width, image.height],
                        "filter_content_rect_pt": rect(filter.contentRect), "window_frame_screen_pt": rect(window.frame),
                        "filter_point_pixel_scale": filter.pointPixelScale,
                        "crop_transform": "unknown_until_validated", "includes_children": false,
                        "included_surface_ids": [wid], "excluded_surface_ids": [], "unresolved_surface_ids": [],
                        "surface_coverage": "explicit_fixture_window_only_other_surfaces_not_included", "captures_audio": false,
                        "capture_filter": "desktopIndependentWindow", "desktop_visibility": "unknown"]
                } catch {
                    result["rendered_capture"] = ["status": "error", "error_domain": (error as NSError).domain,
                                                  "error_code": (error as NSError).code]
                }
            }
            result["capture_duration_seconds"] = ProcessInfo.processInfo.systemUptime - captureStart
            result["total_duration_seconds"] = ProcessInfo.processInfo.systemUptime - begin
            result["observation_end_utc"] = ISO8601DateFormatter().string(from: Date())
            try JSONSerialization.data(withJSONObject: result, options: [.prettyPrinted, .sortedKeys])
                .write(to: out.appendingPathComponent("external-\(sampleIndex).json"), options: .atomic)
            }
            print("F02 observations saved: \(sampleCount)")
        } catch {
            fputs("F02 observation failed: target identity or output unavailable\n", stderr)
            exit(1)
        }
    }
}
