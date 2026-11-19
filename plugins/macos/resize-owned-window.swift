// Explicit owner action, independent of layout observation. All sizes are window points.
import AppKit
import ApplicationServices

struct Refusal: Error { let code: String; init(_ code: String) { self.code = code } }

func attribute(_ element: AXUIElement, _ name: String) throws -> CFTypeRef {
    var value: CFTypeRef?
    guard AXUIElementCopyAttributeValue(element, name as CFString, &value) == .success,
          let value else { throw Refusal("ax_attribute_unavailable") }
    return value
}

func axFrame(_ window: AXUIElement) throws -> CGRect {
    var point = CGPoint.zero, size = CGSize.zero
    let position = try attribute(window, kAXPositionAttribute), dimensions = try attribute(window, kAXSizeAttribute)
    guard CFGetTypeID(position) == AXValueGetTypeID(), CFGetTypeID(dimensions) == AXValueGetTypeID(),
          AXValueGetValue(position as! AXValue, .cgPoint, &point),
          AXValueGetValue(dimensions as! AXValue, .cgSize, &size) else { throw Refusal("invalid_ax_frame") }
    return CGRect(origin: point, size: size)
}

func run() throws {
    let args = CommandLine.arguments
    guard args.count == 7, let pid = Int32(args[1]), pid > 0,
          let birth = Double(args[3]), birth.isFinite, let id = UInt32(args[4]), id > 0,
          let w = Int(args[5]), let h = Int(args[6]), (1...4096).contains(w),
          (1...4096).contains(h) else { throw Refusal("usage: PID BUNDLE NATIVE_BIRTH CGID WIDTH_PT HEIGHT_PT") }
    #if PINNED_OWNER_RESIZE
    // The owner builds a one-window executable with its own case-specific constants.
    guard Array(args.dropFirst()) == ownerArguments else { throw Refusal("owner_arguments_mismatch") }
    #else
    guard args[2] == "me.playphraseme.reference.reels.fgb7o3fd" ||
          args[2].hasPrefix("me.uiblueprint.external-layout.canary.") else {
        throw Refusal("unsupported_owner_copy")
    }
    #endif
    func frame() throws -> CGRect {
        guard let app = NSRunningApplication(processIdentifier: pid), !app.isTerminated,
              app.bundleIdentifier == args[2], app.launchDate?.timeIntervalSince1970 == birth else {
            throw Refusal("stale_process")
        }
        #if PINNED_OWNER_RESIZE
        guard let livePath = app.bundleURL?.path, let canonicalPath = realpath(livePath, nil) else {
            throw Refusal("owner_bundle_path_mismatch")
        }
        defer { free(canonicalPath) }
        guard String(cString: canonicalPath) == ownerBundlePath else {
            throw Refusal("owner_bundle_path_mismatch")
        }
        #endif
        guard let list = CGWindowListCopyWindowInfo([.optionOnScreenOnly,.excludeDesktopElements],
                                                     kCGNullWindowID) as? [[String:Any]] else {
            throw Refusal("window_inventory_unavailable")
        }
        let owned = list.filter { ($0[kCGWindowOwnerPID as String] as? NSNumber)?.int32Value == pid &&
                                  ($0[kCGWindowLayer as String] as? NSNumber)?.intValue == 0 }
        guard owned.count == 1, let window = owned.first,
              (window[kCGWindowNumber as String] as? NSNumber)?.uint32Value == id,
              let bounds = window[kCGWindowBounds as String] as? NSDictionary,
              let rect = CGRect(dictionaryRepresentation: bounds as CFDictionary) else {
            throw Refusal("single_onscreen_window_required")
        }
        return rect
    }
    let before = try frame()
    #if PINNED_OWNER_RESIZE
    guard before == ownerStartingFrame else { throw Refusal("owner_starting_frame_mismatch") }
    #endif
    guard AXIsProcessTrusted() else { throw Refusal("accessibility_permission_required") }
    let app = AXUIElementCreateApplication(pid)
    AXUIElementSetMessagingTimeout(app, 2)
    guard let windows = try attribute(app,kAXWindowsAttribute) as? [AXUIElement], windows.count == 1,
          let window = windows.first, try axFrame(window) == before,
          try attribute(window,kAXSubroleAttribute) as? String == kAXStandardWindowSubrole else {
        throw Refusal("ax_cg_binding_unresolved")
    }
    var settable = DarwinBoolean(false)
    guard AXUIElementIsAttributeSettable(window,kAXSizeAttribute as CFString,&settable) == .success,
          settable.boolValue else { throw Refusal("ax_size_not_settable") }
    var requested = CGSize(width:w,height:h)
    guard before.size != requested else { throw Refusal("size_already_matches") }
    guard let value = AXValueCreate(.cgSize,&requested), try frame() == before else {
        throw Refusal("window_changed_before_dispatch")
    }
    // One dispatch. A failed acknowledgement may still have changed the window.
    guard AXUIElementSetAttributeValue(window,kAXSizeAttribute as CFString,value) == .success else {
        throw Refusal("resize_dispatch_unconfirmed")
    }
    let deadline = ProcessInfo.processInfo.systemUptime + 2
    repeat {
        let after = try frame()
        if after.size == requested, try axFrame(window) == after {
            let record: [String:Any] = ["pid":pid,"bundle":args[2],"birth":birth,"window_id":id,
                "units":"pt","before":[before.width,before.height],"after":[after.width,after.height]]
            print(String(decoding:try JSONSerialization.data(withJSONObject:record,options:.sortedKeys),as:UTF8.self))
            return
        }
        Thread.sleep(forTimeInterval:0.05)
    } while ProcessInfo.processInfo.systemUptime < deadline
    throw Refusal("requested_size_not_confirmed")
}

do { try run() } catch {
    FileHandle.standardError.write(Data(((error as? Refusal)?.code ?? "resize_failed").utf8))
    exit(4)
}
