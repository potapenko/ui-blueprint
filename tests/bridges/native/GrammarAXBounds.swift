// Owner-run read-only qualification of one public AX button in Grammar Home.
// Build with plugins/macos/NativeAcquisition.swift. No bundle installation.
import AppKit
import ApplicationServices

private struct Refusal: Error {
    let code: String
    init(_ code: String) { self.code = code }
}

@main
private enum GrammarAXBounds {
    static let areaID = "searchLearn.learner.grammar"
    static let buttonName = "Present Simple"

    @MainActor static func run() throws {
        let a = Array(CommandLine.arguments.dropFirst())
        guard a.count == 5, let pid = Int32(a[0]), pid > 0,
              let birth = Double(a[2]), birth.isFinite, birth > 0,
              let cg = UInt32(a[3]), cg > 0, a[4].hasPrefix("/") else {
            throw Refusal("expected_PID_BUNDLE_BIRTH_CGWINDOW_ABSOLUTE_APP_PATH")
        }
        guard AXIsProcessTrusted() else { throw Refusal("accessibility_permission_required") }
        let appPath = URL(fileURLWithPath: a[4]).resolvingSymlinksInPath().standardizedFileURL
        let app = AXUIElementCreateApplication(pid)
        let start = ProcessInfo.processInfo.systemUptime
        let limit = try NativeAcquisition(.init(ax_windows: 1, array_page: 16, child_entries: 640,
            value_utf8_bytes: 1024, action_names: 1, action_name_utf8_bytes: 1, batch_values: 1,
            batch_utf8_bytes: 1024, copied_utf8_bytes: 65536, response_slots: 64,
            response_string_utf8_bytes: 4096, image_width: 1, image_height: 1, image_pixels: 1,
            image_bytes: 4, png_bytes: 1, sidecar_bytes: 1), deadline: start + 10)
        func prepare(_ e: AXUIElement) throws {
            try limit.check()
            var owner: pid_t = -1
            guard AXUIElementGetPid(e, &owner) == .success, owner == pid,
                  AXUIElementSetMessagingTimeout(e, 0.15) == .success else { throw Refusal("ax_owner_unresolved") }
        }
        func attribute(_ e: AXUIElement, _ key: String) throws -> CFTypeRef? {
            try prepare(e)
            var raw: CFTypeRef?
            let status = AXUIElementCopyAttributeValue(e, key as CFString, &raw)
            if status == .attributeUnsupported || status == .noValue { return nil }
            guard status == .success, let raw else { throw Refusal("ax_read_incomplete") }
            return raw
        }
        func text(_ e: AXUIElement, _ key: String) throws -> String? {
            guard let raw = try attribute(e, key) else { return nil }
            return try limit.text(raw)
        }
        func list(_ e: AXUIElement, windows: Bool, capacity: Int) throws -> [AXUIElement] {
            let key = (windows ? kAXWindowsAttribute : kAXChildrenAttribute) as CFString
            try prepare(e)
            var count = 0
            let status = AXUIElementGetAttributeValueCount(e, key, &count)
            if !windows && (status == .attributeUnsupported || status == .noValue) { return [] }
            guard status == .success, count <= capacity else { throw Refusal("ax_scope_incomplete") }
            var found = [AXUIElement]()
            let missing = try limit.array(count: count, windows: windows, capacity: capacity, read: { index, length in
                var page: CFArray?
                guard AXUIElementCopyAttributeValues(e, key, index, length, &page) == .success,
                      let page else { throw Refusal("ax_scope_incomplete") }
                return page
            }, visit: { raw in
                guard CFGetTypeID(raw) == AXUIElementGetTypeID() else { throw Refusal("ax_scope_incomplete") }
                found.append(unsafeDowncast(raw, to: AXUIElement.self))
            })
            var after = 0
            guard missing == 0, AXUIElementGetAttributeValueCount(e, key, &after) == .success,
                  count == after else { throw Refusal("ax_scope_changed") }
            return found
        }
        func rect(_ e: AXUIElement) throws -> CGRect {
            guard let p = try attribute(e, kAXPositionAttribute), let s = try attribute(e, kAXSizeAttribute),
                  CFGetTypeID(p) == AXValueGetTypeID(), CFGetTypeID(s) == AXValueGetTypeID() else {
                throw Refusal("ax_frame_unavailable")
            }
            var origin = CGPoint.zero, size = CGSize.zero
            guard AXValueGetValue(unsafeDowncast(p, to: AXValue.self), .cgPoint, &origin),
                  AXValueGetValue(unsafeDowncast(s, to: AXValue.self), .cgSize, &size),
                  origin.x.isFinite, origin.y.isFinite, size.width.isFinite, size.height.isFinite,
                  size.width > 0, size.height > 0 else { throw Refusal("ax_frame_unavailable") }
            return CGRect(origin: origin, size: size)
        }
        func window() throws -> (AXUIElement, CGRect) {
            try limit.check()
            guard let running = NSRunningApplication(processIdentifier: pid), !running.isTerminated,
                  running.bundleIdentifier == a[1], running.launchDate?.timeIntervalSince1970 == birth,
                  running.bundleURL?.resolvingSymlinksInPath().standardizedFileURL == appPath else {
                throw Refusal("stale_process")
            }
            guard let cgWindows = CGWindowListCopyWindowInfo([.optionOnScreenOnly, .excludeDesktopElements],
                                                              kCGNullWindowID) as? [[String:Any]] else {
                throw Refusal("cg_inventory_unavailable")
            }
            let candidates = cgWindows.filter { ($0[kCGWindowOwnerPID as String] as? NSNumber)?.int32Value == pid &&
                                                ($0[kCGWindowLayer as String] as? NSNumber)?.intValue == 0 }
            guard candidates.count == 1, let info = candidates.first,
                  (info[kCGWindowNumber as String] as? NSNumber)?.uint32Value == cg,
                  let bounds = info[kCGWindowBounds as String] as? NSDictionary,
                  let frame = CGRect(dictionaryRepresentation: bounds as CFDictionary) else {
                throw Refusal("cg_window_unresolved")
            }
            let windows = try list(app, windows: true, capacity: 1)
            guard let w = windows.first, try text(w, kAXRoleAttribute) == kAXWindowRole,
                  try text(w, kAXSubroleAttribute) == kAXStandardWindowSubrole, try rect(w) == frame else {
                throw Refusal("ax_cg_binding_unresolved")
            }
            return (w, frame)
        }
        func resolve(_ root: AXUIElement) throws -> (AXUIElement, AXUIElement) {
            var pending: [(AXUIElement, Bool, Int)] = [(root, false, 0)]
            var seen = [AXUIElement](), selectedArea: AXUIElement?, button: AXUIElement?
            while let (element, inherited, depth) = pending.popLast() {
                guard depth <= 32, seen.count < 160 else { throw Refusal("ax_scope_incomplete") }
                guard !seen.contains(where: { CFEqual($0, element) }) else { throw Refusal("ax_scope_not_a_tree") }
                seen.append(element)
                guard let role = try text(element, kAXRoleAttribute) else { throw Refusal("ax_role_unavailable") }
                if depth > 0 && (role == kAXWindowRole || role == kAXPopoverRole) { continue }
                var inside = inherited
                if role == kAXScrollAreaRole {
                    inside = try text(element, kAXIdentifierAttribute) == areaID
                    if inside {
                        guard selectedArea == nil else { throw Refusal("ambiguous_scroll_area") }
                        selectedArea = element
                    }
                }
                if inside, role == kAXButtonRole, try text(element, kAXDescriptionAttribute) == buttonName {
                    guard button == nil else { throw Refusal("ambiguous_button") }
                    button = element
                }
                let children = try list(element, windows: false, capacity: 160 - seen.count - pending.count)
                for child in children { pending.append((child, inside, depth + 1)) }
            }
            guard let selectedArea else { throw Refusal("scroll_area_unavailable") }
            guard let button else { throw Refusal("named_button_unavailable") }
            return (selectedArea, button)
        }
        let (firstWindow, firstWindowFrame) = try window()
        let (area, button) = try resolve(firstWindow)
        let bounds = try rect(button)
        let (secondArea, secondButton) = try resolve(firstWindow)
        let finalBounds = try rect(secondButton)
        let (lastWindow, lastWindowFrame) = try window()
        guard CFEqual(area, secondArea), CFEqual(button, secondButton), CFEqual(firstWindow, lastWindow),
              lastWindowFrame == firstWindowFrame, finalBounds == bounds else {
            throw Refusal("ax_target_changed")
        }
        try limit.check()
        let result: [String:Any] = ["source":"native_ax", "field":"accessibility_bounds",
            "role":kAXButtonRole, "scope_ax_identifier":areaID, "accessibility_name":buttonName,
            "name_source":kAXDescriptionAttribute, "pid":pid, "bundle_id":a[1],
            "launch_time":birth, "window_id":cg, "coordinate_space":"ax-screen", "units":"pt",
            "origin":"top_left", "rect":[bounds.origin.x, bounds.origin.y, bounds.width, bounds.height],
            "consistency":"revalidated_nonatomic", "uptime_start":start,
            "uptime_end":ProcessInfo.processInfo.systemUptime]
        print(String(decoding: try JSONSerialization.data(withJSONObject:result,options:.sortedKeys), as: UTF8.self))
    }
    @MainActor static func main() {
        do { try run() } catch {
            let code = (error as? Refusal)?.code ?? "acquisition_limit_or_invalid_data"
            FileHandle.standardError.write(Data((code + "\n").utf8))
            exit(4)
        }
    }
}
