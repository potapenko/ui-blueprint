import AppKit
import ApplicationServices
import ScreenCaptureKit
import ImageIO
import UniformTypeIdentifiers
import Darwin

// Shared by the fixture helper and the canonical native test bridge.
// Public ScreenCaptureKit callbacks avoid the imported async overlay involved in
// the reproduced lost-continuation path. A local gate owns exactly one reply.
enum OwnedCaptureError: Error {
    case timeout(String), cancelled, permissionRequired, targetUnresolved, staleTarget, emptyCallback(String), resourceUnavailable
}
struct CapturePlatformFailure: Error {
    let stage: String
    let domain: String
    let code: Int
}
struct CaptureTransfer<Value>: @unchecked Sendable {
    // Immutable callback value is transferred once to the awaiting MainActor.
    // Late values are dropped; no mutable SDK object is accessed on both sides.
    let value: Value
}
final class CaptureReply<Value: Sendable>: @unchecked Sendable {
    private let lock = NSLock()
    private var continuation: CheckedContinuation<Value, any Error>?
    private var terminal: Result<Value, any Error>?
    private var ignored = 0
    func install(_ continuation: CheckedContinuation<Value, any Error>) {
        lock.lock()
        if let terminal { lock.unlock(); continuation.resume(with: terminal) }
        else { self.continuation = continuation; lock.unlock() }
    }
    @discardableResult func finish(_ result: Result<Value, any Error>) -> Bool {
        lock.lock()
        guard terminal == nil else { ignored += 1; lock.unlock(); return false }
        terminal = result
        let waiting = continuation; continuation = nil
        lock.unlock()
        waiting?.resume(with: result)
        return true
    }
    var pending: Bool { lock.lock(); defer { lock.unlock() }; return terminal == nil }
    var ignoredReplies: Int { lock.lock(); defer { lock.unlock() }; return ignored }
}
struct OwnedCapture {
    let image: CGImage
    let windowFrame: CGRect
    let filterRect: CGRect
    let scale: Float
    let admissionWait: Double
    var mapping: CaptureMapping? = nil
}

// Public window-server metadata is a geometry witness, not an identity source.
// The caller separately holds the fixture process/Surface generation binding.
struct CaptureGeometry: Equatable {
    let frame: CGRect
    let displays: [Double]
    static func read(windowID: UInt32, pid: Int32) throws -> CaptureGeometry {
        guard let rows = CGWindowListCopyWindowInfo(.optionIncludingWindow, windowID) as? [[String: Any]],
              rows.count == 1, let row = rows.first,
              row[kCGWindowOwnerPID as String] as? Int32 == pid,
              let bounds = row[kCGWindowBounds as String] as? [String: Any],
              let frame = CGRect(dictionaryRepresentation: bounds as CFDictionary)
        else { throw OwnedCaptureError.staleTarget }
        var ids = [CGDirectDisplayID](repeating: 0, count: 32)
        var count: UInt32 = 0
        guard CGGetActiveDisplayList(32, &ids, &count) == .success, count > 0, count < 32
        else { throw OwnedCaptureError.resourceUnavailable }
        var displays: [Double] = []
        for id in ids.prefix(Int(count)).sorted() {
            let bounds = CGDisplayBounds(id)
            displays += [Double(id), bounds.minX, bounds.minY, bounds.width, bounds.height,
                         Double(CGDisplayPixelsWide(id)), Double(CGDisplayPixelsHigh(id)), CGDisplayRotation(id)]
        }
        return CaptureGeometry(frame: frame, displays: displays)
    }
    func requireUnchanged(_ current: CaptureGeometry) throws {
        guard self == current else { throw OwnedCaptureError.staleTarget }
    }
}

struct CaptureMapping {
    let geometry: CaptureGeometry
    let affine: [Double]
    // Only the API-reported full-window, natural-resolution case is admitted.
    // Rounding/scaling/unknown content origin is not inferred from PNG dimensions.
    static func verified(before: CaptureGeometry, after: CaptureGeometry, window: CGRect,
                         content: CGRect, scale: Double, width: Int, height: Int) throws -> CaptureMapping? {
        try before.requireUnchanged(after)
        guard before.frame == window, content == window,
              [window.minX, window.minY, window.width, window.height, scale].allSatisfy({ $0.isFinite }),
              window.width > 0, window.height > 0, scale > 0,
              window.width * scale == Double(width), window.height * scale == Double(height)
        else { return nil }
        let affine = [scale, 0, 0, scale, -window.minX * scale, -window.minY * scale]
        guard affine.allSatisfy({ $0.isFinite }) else { return nil }
        return CaptureMapping(geometry: before, affine: affine)
    }
}
enum CaptureAdmission { case legacyRun, parentOwned }
@MainActor enum CaptureLifecycle {
    static func callback<Value>(stage: String, deadline: Double,
        begin: (@escaping @Sendable (Value?, (any Error)?) -> Void) -> Void) async throws -> Value {
        let gate = CaptureReply<CaptureTransfer<Value>>()
        let transferred: CaptureTransfer<Value> = try await withTaskCancellationHandler {
            try await withCheckedThrowingContinuation { continuation in
                gate.install(continuation)
                guard gate.pending else { return }
                let remaining = deadline - ProcessInfo.processInfo.systemUptime
                guard remaining > 0 else { gate.finish(.failure(OwnedCaptureError.timeout(stage))); return }
                DispatchQueue.global().asyncAfter(deadline: .now() + remaining) {
                    gate.finish(.failure(OwnedCaptureError.timeout(stage)))
                }
                // No gate/global lock is held over the platform call.
                begin { value, error in
                    if let error {
                        let native = error as NSError
                        gate.finish(.failure(CapturePlatformFailure(stage: stage, domain: native.domain, code: native.code)))
                    }
                    else if let value { gate.finish(.success(CaptureTransfer(value: value))) }
                    else { gate.finish(.failure(OwnedCaptureError.emptyCallback(stage))) }
                }
            }
        } onCancel: { gate.finish(.failure(OwnedCaptureError.cancelled)) }
        return transferred.value
    }
    static func validatedFault(_ fault: String?) throws -> String? {
        guard fault == nil || fault == "stall" || fault == "failure" else { throw OwnedCaptureError.resourceUnavailable }
        return fault
    }
    static func capture(windowID: UInt32, pid: Int32, budget: Double = 2,
                        admission: CaptureAdmission = .legacyRun, acquisition: NativeAcquisition) async throws -> OwnedCapture {
        try acquisition.check()
        guard budget.isFinite && budget > 0 && budget <= 2 else { throw OwnedCaptureError.resourceUnavailable }
        let fault = admission == .legacyRun ? try validatedFault(ProcessInfo.processInfo.environment["UIB_CAPTURE_FAULT"]) : nil
        if fault == nil { guard CGPreflightScreenCaptureAccess() else { throw OwnedCaptureError.permissionRequired } }
        let begin = ProcessInfo.processInfo.systemUptime
        let deadline = min(acquisition.deadline, begin + budget)
        var fd: Int32 = -1
        if admission == .legacyRun {
            guard let path = ProcessInfo.processInfo.environment["UIB_CAPTURE_LOCK_PATH"], path.hasPrefix("/") else {
                throw OwnedCaptureError.resourceUnavailable
            }
            fd = open(path, O_RDWR | O_CREAT | O_CLOEXEC | O_NOFOLLOW, mode_t(0o600))
            guard fd >= 0 else { throw OwnedCaptureError.resourceUnavailable }
            do {
                while flock(fd, LOCK_EX | LOCK_NB) != 0 {
                    guard errno == EWOULDBLOCK else { throw OwnedCaptureError.resourceUnavailable }
                    try Task.checkCancellation()
                    guard ProcessInfo.processInfo.systemUptime < deadline else { throw OwnedCaptureError.timeout("capture_admission") }
                    try await Task.sleep(for: .milliseconds(5)) // bounded resource wait, no UI recollection
                }
                try Task.checkCancellation()
            } catch { close(fd); throw error }
        }
        let admissionWait = ProcessInfo.processInfo.systemUptime - begin
        var completedPlatformCall = false
        defer {
            // On timeout/cancel the OS call may still run: keep the FD until this
            // owned one-shot helper exits/reaps. Never admit a replacement early.
            if fd >= 0 && completedPlatformCall { flock(fd, LOCK_UN); close(fd) }
            // parentOwned has no local lease/FD to release. The parent keeps its
            // CaptureLease until this exact helper is confirmed reaped.
        }
        if fault == "stall" {
            let _: Int = try await callback(stage: "injected_capture_stall", deadline: deadline) { _ in }
        }
        if fault == "failure" {
            throw OwnedCaptureError.emptyCallback("injected_capture_failure")
        }
        let geometryBefore = try CaptureGeometry.read(windowID: windowID, pid: pid)
        let content: SCShareableContent = try await callback(stage: "shareable_content", deadline: deadline) { complete in
            SCShareableContent.getExcludingDesktopWindows(true, onScreenWindowsOnly: false, completionHandler: complete)
        }
        try Task.checkCancellation()
        guard let window = content.windows.first(where: { $0.windowID == windowID && $0.owningApplication?.processID == pid }) else {
            throw OwnedCaptureError.targetUnresolved
        }
        let filter = SCContentFilter(desktopIndependentWindow: window)
        let configuration = SCStreamConfiguration()
        configuration.capturesAudio = false; configuration.showsCursor = false
        configuration.ignoreShadowsSingleWindow = true
        let dimensions = try acquisition.imageDimensions(width: Double(filter.contentRect.width),
            height: Double(filter.contentRect.height), scale: Double(filter.pointPixelScale))
        configuration.width = dimensions.0; configuration.height = dimensions.1
        configuration.scalesToFit = false
        configuration.preservesAspectRatio = true
        configuration.destinationRect = CGRect(x: 0, y: 0, width: dimensions.0, height: dimensions.1)
        if #available(macOS 15.0, *) { configuration.captureDynamicRange = .SDR }
        if #available(macOS 14.2, *) { configuration.includeChildWindows = false }
        let image: CGImage = try await callback(stage: "screenshot", deadline: deadline) { complete in
            SCScreenshotManager.captureImage(contentFilter: filter, configuration: configuration, completionHandler: complete)
        }
        try Task.checkCancellation()
        try acquisition.returnedImage(width: image.width, height: image.height, rowBytes: image.bytesPerRow, requested: dimensions)
        guard image.bitsPerComponent == 8, image.bitsPerPixel == 32, image.colorSpace?.model == .rgb
        else { throw NativeAcquisitionError.invalidValue }
        let mapping = try CaptureMapping.verified(before: geometryBefore,
            after: CaptureGeometry.read(windowID: windowID, pid: pid), window: window.frame,
            content: filter.contentRect, scale: Double(filter.pointPixelScale), width: image.width, height: image.height)
        completedPlatformCall = true
        return OwnedCapture(image: image, windowFrame: window.frame, filterRect: filter.contentRect,
                            scale: filter.pointPixelScale, admissionWait: admissionWait, mapping: mapping)
    }
    static func nativeError(_ error: any Error) -> (domain: String, code: Int) {
        if let platform = error as? CapturePlatformFailure { return (platform.domain, platform.code) }
        let value = error as NSError
        return (value.domain, value.code)
    }
    static func gateChecks() async -> [[String: Any]] {
        var result: [[String: Any]] = []
        for mode in ["complete", "timeout", "cancel", "detach"] {
            let gate = CaptureReply<Int>()
            let task = Task { () throws -> Int in
                try await withCheckedThrowingContinuation { gate.install($0) }
            }
            let initial: Result<Int, any Error> = mode == "complete" ? .success(7)
                : .failure(mode == "timeout" ? OwnedCaptureError.timeout("synthetic") : OwnedCaptureError.cancelled)
            let first = gate.finish(initial)
            let lateRejected = !gate.finish(.success(99))
            let value = try? await task.value
            result.append(["case": mode, "injected": true, "first_admitted": first,
                "late_rejected": lateRejected, "ignored_replies": gate.ignoredReplies,
                "outcome_matches": mode == "complete" ? value == 7 : value == nil])
        }
        let denied = issue(CapturePlatformFailure(stage: "recorded_unspecified_stage", domain: "com.apple.ScreenCaptureKit.SCStreamErrorDomain", code: -3801))
        result.append(["case": "recorded_permission_error_classification", "injected": true, "permission_required": denied.code == "permission_required"])
        var invalidRefused = false
        do { _ = try validatedFault("invalid") } catch { invalidRefused = true }
        result.append(["case": "unknown_fault_refused_before_platform_access", "injected": true, "outcome_matches": invalidRefused])
        return result
    }
    static func issue(_ error: any Error) -> (code: String, step: String) {
        if let acquisition = error as? NativeAcquisitionError {
            return (acquisition == .expired ? "timeout" : "incomplete_scope", "native_acquisition")
        }
        if let platform = error as? CapturePlatformFailure {
            // SDK27 SCError.h: -3801 is UserDeclined. Do not retry via another backend.
            if platform.domain == "com.apple.ScreenCaptureKit.SCStreamErrorDomain", platform.code == -3801 {
                return ("permission_required", platform.stage)
            }
            return ("incomplete_scope", platform.stage)
        }
        if let capture = error as? OwnedCaptureError {
            switch capture {
            case .timeout(let step): return ("timeout", step)
            case .cancelled: return ("interrupted", "cancelled")
            case .permissionRequired: return ("permission_required", "permission")
            case .targetUnresolved: return ("target_unresolved", "window_binding")
            case .staleTarget: return ("stale_target", "capture_geometry_changed")
            case .emptyCallback(let step): return ("incomplete_scope", step)
            case .resourceUnavailable: return ("incomplete_scope", "capture_resource_unavailable")
            }
        }
        if error is CancellationError { return ("interrupted", "cancelled") }
        return ("incomplete_scope", "platform_callback")
    }
}

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
    while !queue.isEmpty, nodes.count < (stimulus == "partial" ? 8 : 160), ProcessInfo.processInfo.systemUptime - start < 1 {
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
#if !CAPTURE_LIBRARY
@main
struct Observe {
    @MainActor static func main() async {
        // Independent process watchdog; system requests cannot hold a global queue.
        DispatchQueue.global().asyncAfter(deadline: .now() + 4) { _exit(124) }
        let args = CommandLine.arguments
        if args.count == 2 && args[1] == "--gate-checks" {
            let report = await CaptureLifecycle.gateChecks()
            if let data = try? JSONSerialization.data(withJSONObject: report, options: [.sortedKeys]) {
                try? FileHandle.standardOutput.write(contentsOf: data)
            }
            return
        }
        guard args.count == 4 || args.count == 5 else { exit(2) }
        let manifestURL = URL(fileURLWithPath: args[1])
        let out = URL(fileURLWithPath: args[2], isDirectory: true)
        let sampleCount = args.count == 5 ? (Int(args[4]) ?? 0) : 1
        guard (1...30).contains(sampleCount) else { exit(2) }
        do {
            let limitsFile = try FileHandle(forReadingFrom: URL(fileURLWithPath: args[3]))
            defer { try? limitsFile.close() }
            let limitsData = try limitsFile.read(upToCount: 4033) ?? Data()
            guard limitsData.count <= 4032 else { throw NativeAcquisitionError.limit }
            let acquisitionLimits = try JSONDecoder().decode(NativeAcquisitionLimits.self, from: limitsData)
            try acquisitionLimits.validate()
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
            // Persist completed AX before any capture wait or failure.
            try JSONSerialization.data(withJSONObject: result, options: [.prettyPrinted, .sortedKeys])
                .write(to: out.appendingPathComponent("ax-\(sampleIndex).json"), options: .atomic)
            let captureStart = ProcessInfo.processInfo.systemUptime
            var captureFailed = false
            if ProcessInfo.processInfo.environment["UIB_AX_ONLY"] == "1" {
                result["rendered_capture"] = ["selection": "not_requested"]
            } else if stimulus == "permission_denied" {
                result["rendered_capture"] = ["status": "permission_required", "injection": "synthetic", "prompted": false]
            } else if !CGPreflightScreenCaptureAccess() {
                result["rendered_capture"] = ["status": "permission_required", "prompted": false]
            } else {
                do {
                    let acquisition = try NativeAcquisition(acquisitionLimits, deadline: captureStart + 2)
                    let captured = try await CaptureLifecycle.capture(windowID: wid, pid: pid, acquisition: acquisition)
                    let image = captured.image
                    guard NSRunningApplication(processIdentifier: pid)?.launchDate?.timeIntervalSince1970 == launch else {
                        throw NSError(domain: "stale_target", code: 3)
                    }
                    if sampleIndex == 0 {
                    _ = try writeNativePNG(image, admission: acquisition, directory: out, name: "window.png")
                    }
                    result["rendered_capture"] = ["status": "observed", "capture_kind": "window_isolated",
                        "capture_target": wid, "pixel_buffer_size": [image.width, image.height],
                        "filter_content_rect_pt": rect(captured.filterRect), "window_frame_screen_pt": rect(captured.windowFrame),
                        "filter_point_pixel_scale": captured.scale,
                        "capture_admission_wait_seconds": captured.admissionWait,
                        "capture_serialization": "exclusive_run_owned_flock",
                        "crop_transform": "unknown_until_validated", "includes_children": false,
                        "included_surface_ids": [wid], "excluded_surface_ids": [], "unresolved_surface_ids": [],
                        "surface_coverage": "explicit_fixture_window_only_other_surfaces_not_included", "captures_audio": false,
                        "capture_filter": "desktopIndependentWindow", "desktop_visibility": "unknown"]
                } catch {
                    captureFailed = true
                    let issue = CaptureLifecycle.issue(error)
                    let native = CaptureLifecycle.nativeError(error)
                    result["rendered_capture"] = ["status": issue.code == "timeout" ? "timeout" : "error",
                        "code": issue.code, "failed_step": issue.step, "error_domain": native.domain,
                        "error_code": native.code, "requires_owned_helper_retirement": true,
                        "injected": ProcessInfo.processInfo.environment["UIB_CAPTURE_FAULT"] != nil]
                }
            }
            result["capture_duration_seconds"] = ProcessInfo.processInfo.systemUptime - captureStart
            result["total_duration_seconds"] = ProcessInfo.processInfo.systemUptime - begin
            result["observation_end_utc"] = ISO8601DateFormatter().string(from: Date())
            try JSONSerialization.data(withJSONObject: result, options: [.prettyPrinted, .sortedKeys])
                .write(to: out.appendingPathComponent("external-\(sampleIndex).json"), options: .atomic)
            if captureFailed { fputs("F02 capture failed; completed AX preserved\n", stderr); exit(2) }
            }
            print("F02 observations saved: \(sampleCount)")
        } catch {
            fputs("F02 observation failed: target identity or output unavailable\n", stderr)
            exit(1)
        }
    }
}

#endif
