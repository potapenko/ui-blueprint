// Owner-run public AX measurements for Grammar Home and Favorites controls.
// Build with plugins/macos/NativeAcquisition.swift. Append a witnessed AX-screen x y
// to select one concrete scroll area by its public hit-test parent chain.
import AppKit
import ApplicationServices

private struct Refusal: Error {
    let code: String
    init(_ code: String) { self.code = code }
}

@main
private enum GrammarAXBounds {

    @MainActor static func run() throws {
        var a = Array(CommandLine.arguments.dropFirst())
        let names = ["library-empty-slot": "Favorites collection 1, empty", "library-import": "Import"]
        let selectedName = names[a.first ?? ""]
        let roleOnly = a.first == "library-slot-overlay-role"
        let library = roleOnly || selectedName != nil
        let pdfWitness = a.first == "library-statistics-witness"
        if library || pdfWitness { a.removeFirst() }
        let areaID: String
        if pdfWitness { areaID = "searchLearn.learner.statistics" }
        else { areaID = library ? "searchLearn.learner.overlay" : "searchLearn.learner.grammar" }
        let buttonName = selectedName ?? "Present Simple"
        if pdfWitness && a.count != 7 { throw Refusal("statistics_visual_witness_required") }
        if library && a.count != 7 { throw Refusal("library_overlay_witness_required") }
        guard (a.count == 5 || a.count == 7), let pid = Int32(a[0]), pid > 0,
              let birth = Double(a[2]), birth.isFinite, birth > 0,
              let cg = UInt32(a[3]), cg > 0, a[4].hasPrefix("/") else {
            throw Refusal("expected_PID_BUNDLE_BIRTH_CGWINDOW_ABSOLUTE_APP_PATH_optional_AX_SCREEN_X_Y")
        }
        var witness: CGPoint?
        if a.count == 7 {
            guard let x = Float(a[5]), let y = Float(a[6]), x.isFinite, y.isFinite else {
                throw Refusal("invalid_ax_screen_witness")
            }
            witness = CGPoint(x: CGFloat(x), y: CGFloat(y))
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
        // Test only the owner-declared public literal. Avoid converting unrelated identifiers
        // or descriptive strings to Swift values, including user-defined collection names.
        func matches(_ e: AXUIElement, _ key: String, _ expected: String) throws -> Bool {
            guard let raw = try attribute(e, key) else { return false }
            guard CFGetTypeID(raw) == CFStringGetTypeID() else { throw Refusal("ax_string_type_unavailable") }
            let value = unsafeDowncast(raw, to: CFString.self)
            let literal = expected as CFString
            return CFStringGetLength(value) == CFStringGetLength(literal) && CFEqual(value, literal)
        }
        func matchesButtonName(_ button: AXUIElement) throws -> Bool {
            if library { return try matches(button, kAXDescriptionAttribute, buttonName) }
            return try text(button, kAXDescriptionAttribute) == buttonName
        }
        func list(_ e: AXUIElement, windows: Bool, capacity: Int) throws -> [AXUIElement] {
            let key = (windows ? kAXWindowsAttribute : kAXChildrenAttribute) as CFString
            try prepare(e)
            var count = 0
            let status = AXUIElementGetAttributeValueCount(e, key, &count)
            if !windows && (status == .attributeUnsupported || status == .noValue) { return [] }
            guard status == .success else { throw Refusal("ax_array_count_unavailable") }
            guard count <= capacity else { throw Refusal("ax_children_over_capacity") }
            var found = [AXUIElement]()
            let missing = try limit.array(count: count, windows: windows, capacity: capacity, read: { index, length in
                var page: CFArray?
                guard AXUIElementCopyAttributeValues(e, key, index, length, &page) == .success,
                      let page else { throw Refusal("ax_array_page_unavailable") }
                return page
            }, visit: { raw in
                guard CFGetTypeID(raw) == AXUIElementGetTypeID() else { throw Refusal("ax_child_type_unavailable") }
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
            let selected = candidates.filter { ($0[kCGWindowNumber as String] as? NSNumber)?.uint32Value == cg }
            guard selected.count == 1, let info = selected.first else { throw Refusal("cg_window_unresolved") }
            func cgBounds(_ entry: [String:Any]) throws -> CGRect {
                guard let bounds = entry[kCGWindowBounds as String] as? NSDictionary,
                      let frame = CGRect(dictionaryRepresentation: bounds as CFDictionary),
                      frame.origin.x.isFinite, frame.origin.y.isFinite, frame.width.isFinite, frame.height.isFinite,
                      !frame.isEmpty else { throw Refusal("cg_frame_unavailable") }
                return frame
            }
            let frame = try cgBounds(info)
            let w: AXUIElement
            if let witness {
                // No global one-window assumption. The AX object is selected by a public
                // hit-test on this app and reports its containing window. Bind that window
                // only if its frame belongs to exactly the selected on-screen normal CG window.
                guard frame.contains(witness) else { throw Refusal("witness_outside_window") }
                for sibling in candidates where (sibling[kCGWindowNumber as String] as? NSNumber)?.uint32Value != cg {
                    if try cgBounds(sibling) == frame { throw Refusal("cg_window_frame_ambiguous") }
                }
                let hit = try hitAt(witness)
                if let raw = try attribute(hit, kAXWindowAttribute), CFGetTypeID(raw) == AXUIElementGetTypeID() {
                    w = unsafeDowncast(raw, to: AXUIElement.self)
                } else {
                    guard try text(hit, kAXRoleAttribute) == kAXWindowRole else {
                        throw Refusal("ax_hit_window_unavailable")
                    }
                    w = hit
                }
            } else {
                guard candidates.count == 1 else { throw Refusal("cg_window_unresolved") }
                let windows = try list(app, windows: true, capacity: 1)
                guard let first = windows.first else { throw Refusal("ax_cg_binding_unresolved") }
                w = first
            }
            guard try text(w, kAXRoleAttribute) == kAXWindowRole,
                  try text(w, kAXSubroleAttribute) == kAXStandardWindowSubrole, try rect(w) == frame else {
                throw Refusal("ax_cg_binding_unresolved")
            }
            return (w, frame)
        }
        func publicContainerRole(_ area: AXUIElement) throws -> String {
            let roles = [kAXGroupRole, kAXScrollAreaRole, kAXSplitGroupRole, kAXListRole, kAXLayoutAreaRole,
                         kAXLayoutItemRole, kAXTabGroupRole, kAXToolbarRole, kAXRadioGroupRole, kAXOutlineRole]
            guard let role = try text(area, kAXRoleAttribute), roles.contains(role) else {
                throw Refusal("recognized_public_container_role_unavailable")
            }
            return role
        }
        func hitAt(_ point: CGPoint) throws -> AXUIElement {
            try prepare(app)
            var hit: AXUIElement?
            guard AXUIElementCopyElementAtPosition(app, Float(point.x), Float(point.y), &hit) == .success,
                  let hit else { throw Refusal("ax_hit_unavailable") }
            return hit
        }
        func areaAtWitness(_ point: CGPoint, root: AXUIElement, startingAt original: AXUIElement? = nil) throws -> (area: AXUIElement, cost: Int) {
            guard try rect(root).contains(point) else { throw Refusal("witness_outside_window") }
            var element = try original ?? hitAt(point)
            var chain = [AXUIElement](), area: AXUIElement?
            while !CFEqual(element, root) {
                try limit.reserveSelectionEntry()
                guard chain.count < 32, !chain.contains(where: { CFEqual(element, $0) }) else {
                    throw Refusal("ax_hit_window_unresolved")
                }
                chain.append(element)
                if library {
                    // Compare the fixed public identifier on this parent path only.
                    if try matches(element, kAXIdentifierAttribute, areaID) {
                        guard area == nil else { throw Refusal("ambiguous_overlay_on_parent_path") }
                        area = element
                    }
                } else {
                    guard let role = try text(element, kAXRoleAttribute), role != kAXWindowRole,
                          role != kAXPopoverRole else { throw Refusal("ax_hit_window_unresolved") }
                    if role == kAXScrollAreaRole && area == nil {
                        guard try text(element, kAXIdentifierAttribute) == areaID else {
                            throw Refusal("ax_hit_scroll_area_mismatch")
                        }
                        area = element
                    }
                }
                guard let parent = try attribute(element, kAXParentAttribute),
                      CFGetTypeID(parent) == AXUIElementGetTypeID() else { throw Refusal("ax_hit_parent_unavailable") }
                element = unsafeDowncast(parent, to: AXUIElement.self)
            }
            guard let area else { throw Refusal(library ? "ax_overlay_unavailable" : "ax_hit_scroll_area_unavailable") }
            if library {
                guard let owningWindow = try attribute(area, kAXWindowAttribute),
                      CFEqual(owningWindow, root) else { throw Refusal("ax_overlay_window_unresolved") }
            }
            return (area, chain.count)
        }
        func resolve(_ root: AXUIElement) throws -> (AXUIElement, AXUIElement) {
            let hitArea = try witness.map { try areaAtWitness($0, root: root) }
            let nodeBudget = 160 - (hitArea?.cost ?? 0)
            var pending: [(AXUIElement, Bool, Int)] = [(hitArea?.area ?? root, hitArea != nil, 0)]
            var seen = [AXUIElement](), selectedArea = hitArea?.area, button: AXUIElement?
            while let (element, inherited, depth) = pending.popLast() {
                guard depth <= 32, seen.count < nodeBudget else { throw Refusal("ax_traversal_limit") }
                guard !seen.contains(where: { CFEqual($0, element) }) else { throw Refusal("ax_scope_not_a_tree") }
                seen.append(element)
                guard let role = try text(element, kAXRoleAttribute) else { throw Refusal("ax_role_unavailable") }
                if depth > 0 && (role == kAXWindowRole || role == kAXPopoverRole) { continue }
                var inside = inherited
                if hitArea == nil && role == kAXScrollAreaRole {
                    inside = try text(element, kAXIdentifierAttribute) == areaID
                    if inside {
                        guard selectedArea == nil else { throw Refusal("ambiguous_scroll_area") }
                        selectedArea = element
                    }
                }
                if inside, role == kAXButtonRole, try matchesButtonName(element) {
                    guard button == nil else { throw Refusal("ambiguous_button") }
                    button = element
                }
                let children = try list(element, windows: false, capacity: nodeBudget - seen.count - pending.count)
                for child in children { pending.append((child, inside, depth + 1)) }
            }
            guard let selectedArea else { throw Refusal("scroll_area_unavailable") }
            guard let button else { throw Refusal("named_button_unavailable") }
            return (selectedArea, button)
        }
        let (firstWindow, firstWindowFrame) = try window()
        if pdfWitness, let witness {
            func witnessedButton() throws -> (button: AXUIElement, area: AXUIElement, bounds: CGRect) {
                let hit = try hitAt(witness)
                guard try text(hit, kAXRoleAttribute) == kAXButtonRole else { throw Refusal("ax_hit_not_button") }
                let area = try areaAtWitness(witness, root: firstWindow, startingAt: hit).area
                guard let enabled = try attribute(hit, kAXEnabledAttribute),
                      CFGetTypeID(enabled) == CFBooleanGetTypeID() else { throw Refusal("ax_enabled_unavailable") }
                guard CFEqual(enabled, kCFBooleanTrue) else { throw Refusal("ax_button_disabled") }
                let bounds = try rect(hit)
                guard bounds.contains(witness) else { throw Refusal("witness_outside_button_bounds") }
                return (hit, area, bounds)
            }
            let first = try witnessedButton()
            let second = try witnessedButton()
            let (lastWindow, lastFrame) = try window()
            guard CFEqual(first.button, second.button), CFEqual(first.area, second.area),
                  first.bounds == second.bounds, CFEqual(firstWindow, lastWindow),
                  firstWindowFrame == lastFrame else { throw Refusal("ax_target_changed") }
            try limit.check()
            let bounds = first.bounds
            let result: [String:Any] = ["source":"native_ax", "field":"accessibility_bounds",
                "attribution":"owner_visual_witness", "role":kAXButtonRole, "enabled":true,
                "scope_ax_identifier":areaID, "pid":pid, "bundle_id":a[1], "launch_time":birth,
                "window_id":cg, "coordinate_space":"ax-screen", "units":"pt", "origin":"top_left",
                "rect":[bounds.origin.x, bounds.origin.y, bounds.width, bounds.height],
                "witness_ax_screen_pt":[witness.x, witness.y], "consistency":"revalidated_nonatomic",
                "uptime_start":start, "uptime_end":ProcessInfo.processInfo.systemUptime]
            print(String(decoding: try JSONSerialization.data(withJSONObject:result,options:.sortedKeys), as: UTF8.self))
            return
        }
        if roleOnly, let witness {
            let firstArea = try areaAtWitness(witness, root: firstWindow).area
            let firstRole = try publicContainerRole(firstArea)
            let secondArea = try areaAtWitness(witness, root: firstWindow).area
            let secondRole = try publicContainerRole(secondArea)
            let (lastWindow, lastFrame) = try window()
            guard CFEqual(firstArea, secondArea), firstRole == secondRole, CFEqual(firstWindow, lastWindow),
                  firstWindowFrame == lastFrame else { throw Refusal("ax_target_changed") }
            try limit.check()
            let report: [String:Any] = ["source":"native_ax", "field":"AXRole", "scope_ax_identifier":areaID,
                "role":firstRole, "pid":pid, "bundle_id":a[1], "launch_time":birth, "window_id":cg,
                "scope_selection":"unique_exact_identifier_on_ax_hit_parent_path_to_window",
                "scope_witness_ax_screen_pt":[witness.x, witness.y], "consistency":"revalidated_nonatomic"]
            print(String(decoding: try JSONSerialization.data(withJSONObject:report,options:.sortedKeys), as: UTF8.self))
            return
        }
        let (area, button) = try resolve(firstWindow)
        let firstRole = library ? try publicContainerRole(area) : nil
        let bounds = try rect(button)
        let (secondArea, secondButton) = try resolve(firstWindow)
        let finalBounds = try rect(secondButton)
        if let firstRole, try publicContainerRole(secondArea) != firstRole { throw Refusal("ax_target_changed") }
        let (lastWindow, lastWindowFrame) = try window()
        guard CFEqual(area, secondArea), CFEqual(button, secondButton), CFEqual(firstWindow, lastWindow),
              lastWindowFrame == firstWindowFrame, finalBounds == bounds else {
            throw Refusal("ax_target_changed")
        }
        try limit.check()
        var result: [String:Any] = ["source":"native_ax", "field":"accessibility_bounds",
            "role":kAXButtonRole, "scope_ax_identifier":areaID, "accessibility_name":buttonName,
            "name_source":kAXDescriptionAttribute, "pid":pid, "bundle_id":a[1],
            "launch_time":birth, "window_id":cg, "coordinate_space":"ax-screen", "units":"pt",
            "origin":"top_left", "rect":[bounds.origin.x, bounds.origin.y, bounds.width, bounds.height],
            "consistency":"revalidated_nonatomic", "uptime_start":start,
            "uptime_end":ProcessInfo.processInfo.systemUptime]
        if let witness {
            result["scope_selection"] = library ? "unique_exact_identifier_on_ax_hit_parent_path_to_window"
                                                : "nearest_scroll_area_on_ax_hit_parent_path_to_exact_window"
            result["uniqueness"] = library ? "all_ax_children_of_this_overlay_excluding_separate_surfaces"
                                          : "all_ax_children_of_this_scroll_area_excluding_separate_surfaces"
            if let firstRole { result["scope_ax_role"] = firstRole }
            result["scope_witness_ax_screen_pt"] = [witness.x, witness.y]
        }
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
