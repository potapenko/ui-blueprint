// Q02 nonvisual read-only witness. No launch, activation, setters or capture.
import AppKit
import ApplicationServices
import Foundation
import Darwin

@main struct Q02ReadonlyFacts {
    enum Refusal: String, Error {
        case invalid_trusted_input, process_missing, process_incarnation_changed,
             executable_changed, window_owner_changed, identity_record_changed
    }
    static func read(_ path: String, cap: Int) throws -> Data {
        let fd = open(path, O_RDONLY | O_CLOEXEC | O_NOFOLLOW)
        guard fd >= 0 else { throw NativeAcquisitionError.invalidValue }
        let file = FileHandle(fileDescriptor: fd, closeOnDealloc: true)
        defer { try? file.close() }
        let bytes = try file.read(upToCount: cap + 1) ?? Data()
        guard bytes.count <= cap else { throw NativeAcquisitionError.limit }
        return bytes
    }
    @MainActor static func identity(_ manifest: [String: Any], executable: String) throws -> [String: Any] {
        guard let pid = manifest["pid"] as? Int32, let bundle = manifest["bundle_id"] as? String,
              bundle == "local.uiblueprint.f02.on", let launch = manifest["launch_time"] as? Double,
              let window = manifest["window_id"] as? UInt32, manifest["window_identifier"] as? String == "a",
              let identity = manifest["identity_path"] as? String else { throw Refusal.invalid_trusted_input }
        guard let app = NSRunningApplication(processIdentifier: pid), !app.isTerminated else { throw Refusal.process_missing }
        guard app.bundleIdentifier == bundle, app.launchDate?.timeIntervalSince1970 == launch else { throw Refusal.process_incarnation_changed }
        guard let actual = app.executableURL,
              actual.resolvingSymlinksInPath().path == URL(fileURLWithPath: executable).resolvingSymlinksInPath().path else { throw Refusal.executable_changed }
        guard let rows = CGWindowListCopyWindowInfo(.optionIncludingWindow, window) as? [[String: Any]],
              rows.count == 1, rows[0][kCGWindowOwnerPID as String] as? Int32 == pid,
              let bounds = rows[0][kCGWindowBounds as String] as? [String: Any]
        else { throw Refusal.window_owner_changed }
        do { try NativeCurrentIdentity.verify(path: identity, expected: manifest) }
        catch { throw Refusal.identity_record_changed }
        return ["pid": pid, "bundle_id": bundle, "launch_time": launch, "window_id": window,
                "executable": actual.resolvingSymlinksInPath().path,
                "app_active": app.isActive, "app_hidden": app.isHidden,
                "frontmost_pid": NSWorkspace.shared.frontmostApplication?.processIdentifier ?? -1,
                "window_frame": bounds, "on_screen": rows[0][kCGWindowIsOnscreen as String] ?? false]
    }
    static func unavailable(_ code: AXError, _ json: NativeJSON) throws -> [String: Any] {
        try json.object(["availability", "ax_error"]) {
            ["availability": try json.scalar(code == .attributeUnsupported ? "unsupported" : "unknown"),
             "ax_error": try json.scalar(code.rawValue)]
        }
    }
    @MainActor static func property(_ node: CFTypeRef, _ name: String, _ admission: NativeAcquisition,
                                    _ json: NativeJSON) throws -> [String: Any] {
        try admission.check()
        NativeAXAccess.live.prepare(node)
        let (code, raw) = NativeAXAccess.live.attribute(node, name)
        guard code == .success, let raw else { return try unavailable(code, json) }
        func known(_ value: Any) throws -> [String: Any] {
            try json.object(["availability", "value"]) { ["availability": try json.scalar("known"), "value": try json.borrowed(value)] }
        }
        if CFGetTypeID(raw) == CFStringGetTypeID() {
            do { return try known(admission.text(raw)) }
            catch NativeAcquisitionError.limit { return try json.object(["availability", "reason"]) { ["availability":try json.scalar("unknown"),"reason":try json.scalar("acquisition_limit")] } }
        }
        if CFGetTypeID(raw) == CFBooleanGetTypeID() { return try known((raw as! NSNumber).boolValue) }
        if CFGetTypeID(raw) == CFNumberGetTypeID() {
            let value = (raw as! NSNumber).doubleValue
            guard value.isFinite else { return try unavailable(.failure, json) }
            return try known(value)
        }
        if CFGetTypeID(raw) == AXValueGetTypeID() {
            let value = unsafeDowncast(raw, to: AXValue.self)
            switch AXValueGetType(value) {
            case .axError:
                var code = AXError.failure; AXValueGetValue(value, .axError, &code)
                return try unavailable(code, json)
            case .cgPoint:
                var point = CGPoint.zero; guard AXValueGetValue(value, .cgPoint, &point), point.x.isFinite, point.y.isFinite else { return try unavailable(.failure, json) }
                return try known(["x": point.x, "y": point.y])
            case .cgSize:
                var size = CGSize.zero; guard AXValueGetValue(value, .cgSize, &size), size.width.isFinite, size.height.isFinite else { return try unavailable(.failure, json) }
                return try known(["width": size.width, "height": size.height])
            default: break
            }
        }
        return try unavailable(.failure, json)
    }
    @MainActor static func facts(_ manifest: [String: Any], limits: NativeAcquisitionLimits,
                                json: NativeJSON, deadline: Double) throws -> [String: Any] {
        guard AXIsProcessTrusted() else { return try json.object(["status"]) { ["status":try json.scalar("permission_required")] } }
        let admission = try NativeAcquisition(limits, deadline: deadline)
        let app = AXUIElementCreateApplication(manifest["pid"] as! Int32)
        let window = try Collector.resolveWindow(app, identifier: "a", admission: admission)
        var handles: [CFTypeRef] = [window], depths = [0], rows: [[String: Any]] = []
        var omitted = 0
        let names = ["AXRole","AXSubrole","AXIdentifier","AXTitle","AXDescription","AXPosition","AXSize","AXEnabled","AXFocused","AXPlaceholderValue","AXMain"]
        while rows.count < handles.count {
            try admission.check()
            let index = rows.count, node = handles[index]
            var props: [String: Any] = [:]
            for name in names { props[name] = try property(node, name, admission, json) }
            func text(_ name: String) -> String? { (props[name] as? [String: Any])?["value"] as? String }
            func classified(_ name: String) -> Bool {
                let p = props[name] as! [String: Any]
                if p["availability"] as? String == "known" { return p["value"] is String }
                return [AXError.attributeUnsupported.rawValue, AXError.noValue.rawValue].contains(p["ax_error"] as? Int32 ?? 0)
            }
            if text("AXRole") == "AXSecureTextField" || text("AXSubrole") == "AXSecureTextField" || text("AXIdentifier") == "f02.secret" {
                props["AXValue"] = try json.object(["availability"]) { ["availability":try json.scalar("redacted")] }
            } else if nativeMayReadValue(role: text("AXRole"), subrole: text("AXSubrole"), identifier: text("AXIdentifier"), complete: ["AXRole","AXSubrole","AXIdentifier"].allSatisfy(classified)) {
                props["AXValue"] = try property(node, "AXValue", admission, json)
            } else { props["AXValue"] = try unavailable(.failure, json) }
            NativeAXAccess.live.prepare(node)
            let (actionCode, rawActions) = NativeAXAccess.live.actions(node)
            let actions = actionCode == .success ? try rawActions.map { try admission.actions($0) } : nil
            var children: [Int] = []
            omitted += try nativeAXElements(node, kAXChildrenAttribute, windows: false,
                capacity: depths[index] + 1 < 9 ? max(0, 160 - handles.count) : 0,
                admission: admission, access: .live) { child in
                    if let old = handles.firstIndex(where: { CFEqual($0, child) }) { children.append(old) }
                    else { children.append(handles.count); handles.append(child); depths.append(depths[index] + 1) }
                }
            rows.append(try json.object(["index","properties","actions","actions_error","children"]) {
                ["index":try json.scalar(index),"properties":try json.borrowed(props),
                 "actions":try json.borrowed(actions ?? []),"actions_error":try json.scalar(actionCode.rawValue),"children":try json.borrowed(children)]
            })
        }
        guard CFEqual(window, try Collector.resolveWindow(app, identifier: "a", admission: admission)) else { throw NativeAcquisitionError.invalidValue }
        return try json.object(["status","nodes","omitted","acquisition"]) {
            ["status":try json.scalar("observed"),"nodes":try json.borrowed(rows),"omitted":try json.scalar(omitted),"acquisition":try json.borrowed(admission.metrics)]
        }
    }
    @MainActor static func main() {
        DispatchQueue.global().asyncAfter(deadline: .now() + 4) { _exit(124) }
        do {
            let args = CommandLine.arguments
            guard args.count == 5, ["identity","facts"].contains(args[4]) else { _exit(2) }
            let manifest = try JSONSerialization.jsonObject(with: read(args[1], cap: 524288)) as! [String: Any]
            let limits = try JSONDecoder().decode(NativeAcquisitionLimits.self, from: read(args[3], cap: 4032))
            let before = try identity(manifest, executable: args[2])
            let json = NativeJSON(limits), deadline = ProcessInfo.processInfo.systemUptime + 1
            let data = args[4] == "facts" ? try facts(manifest, limits: limits, json: json, deadline: deadline) : [:]
            let after = try identity(manifest, executable: args[2])
            let report = try json.object(["status","before","after","facts"]) {
                ["status":try json.scalar("validated"),"before":try json.borrowed(before),"after":try json.borrowed(after),"facts":try json.borrowed(data)]
            }
            let frame = try NativeJSONFrame(capacity: 524288, deadline: ProcessInfo.processInfo.systemUptime + 1)
            try frame.encode(report); try frame.write(to: STDOUT_FILENO, deadline: ProcessInfo.processInfo.systemUptime + 1)
        } catch {
            // No UI strings/raw SDK diagnostics on failure; no fallback/retry.
            let code: String
            if let refusal = error as? Refusal { code = refusal.rawValue }
            else if let issue = error as? NativeAcquisitionError {
                switch issue { case .expired: code = "read_timeout"; case .limit: code = "read_limit"; default: code = "acquisition_unavailable" }
            } else { code = "acquisition_unavailable" }
            FileHandle.standardOutput.write(Data("{\"status\":\"unavailable\",\"code\":\"\(code)\"}\n".utf8))
            _exit(4)
        }
    }
}
