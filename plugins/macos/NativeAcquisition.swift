import Foundation
import CoreFoundation

enum NativeAcquisitionError: Error { case limit, invalidValue, expired, io }

// All fields are required trusted caller choices; these ceilings are validation,
// never a default profile. UIB.D05-NATIVE-ACQUISITION@1.
struct NativeAcquisitionLimits: Codable {
    let ax_windows: Int
    let array_page: Int
    let child_entries: Int
    let value_utf8_bytes: Int
    let action_names: Int
    let action_name_utf8_bytes: Int
    let batch_values: Int
    let batch_utf8_bytes: Int
    let copied_utf8_bytes: Int
    let response_slots: Int
    let response_string_utf8_bytes: Int
    let image_width: Int
    let image_height: Int
    let image_pixels: Int
    let image_bytes: Int
    let png_bytes: Int
    let sidecar_bytes: Int

    func validate() throws {
        let pairs = [(ax_windows,32), (array_page,32), (child_entries,1280),
            (value_utf8_bytes,4096), (action_names,32), (action_name_utf8_bytes,256),
            (batch_values,8), (batch_utf8_bytes,16384), (copied_utf8_bytes,262144),
            (response_slots,65536), (response_string_utf8_bytes,1048576),
            (image_width,4096), (image_height,4096), (image_pixels,8388608),
            (image_bytes,67108864), (png_bytes,67108864), (sidecar_bytes,16384)]
        guard pairs.allSatisfy({ $0.0 > 0 && $0.0 <= $0.1 }) else { throw NativeAcquisitionError.limit }
    }
}

func nativeAdd(_ a: Int, _ b: Int) throws -> Int {
    guard a >= 0, b >= 0 else { throw NativeAcquisitionError.invalidValue }
    let (n, overflow) = a.addingReportingOverflow(b)
    guard !overflow else { throw NativeAcquisitionError.limit }
    return n
}
func nativeMultiply(_ a: Int, _ b: Int) throws -> Int {
    guard a >= 0, b >= 0 else { throw NativeAcquisitionError.invalidValue }
    let (n, overflow) = a.multipliedReportingOverflow(by: b)
    guard !overflow else { throw NativeAcquisitionError.limit }
    return n
}

final class NativeAcquisition {
    let limits: NativeAcquisitionLimits
    let deadline: Double
    private(set) var copiedUTF8 = 0 // charged conversion-buffer + String copies
    private(set) var actualUTF8 = 0
    private(set) var requestedUTF8 = 0
    private(set) var childEntries = 0
    private(set) var arrayCalls = 0
    private(set) var maximumStringUnits = 0
    private(set) var refusedValues = 0
    private(set) var windowsCounted = 0
    private(set) var windowsRead = 0
    private(set) var maximumBatch = 0
    private(set) var maximumActions = 0

    init(_ limits: NativeAcquisitionLimits, deadline: Double) throws {
        try limits.validate()
        guard deadline.isFinite else { throw NativeAcquisitionError.invalidValue }
        self.limits = limits; self.deadline = deadline
        try check()
    }
    func check() throws {
        guard ProcessInfo.processInfo.systemUptime < deadline else { throw NativeAcquisitionError.expired }
    }
    // Inspect before bridging/copying. NULL-buffer CFStringGetBytes ignores its
    // maxBufLen: the UTF-16 guard MUST precede that bounded-work sizing pass.
    private func textSize(_ value: CFTypeRef, ceiling: Int) throws -> Int {
        try check()
        guard CFGetTypeID(value) == CFStringGetTypeID() else { throw NativeAcquisitionError.invalidValue }
        let string = unsafeDowncast(value, to: CFString.self)
        let length = CFStringGetLength(string)
        maximumStringUnits = max(maximumStringUnits, length)
        guard length >= 0, length <= ceiling else { throw NativeAcquisitionError.limit }
        var used = 0
        let converted = CFStringGetBytes(string, CFRange(location: 0, length: length),
            CFStringBuiltInEncodings.UTF8.rawValue, 0, false, nil, 0, &used)
        guard used >= 0 else { throw NativeAcquisitionError.invalidValue }
        requestedUTF8 = try nativeAdd(requestedUTF8, used)
        guard converted == length, used <= ceiling else { throw NativeAcquisitionError.limit }
        return used
    }
    func reserveCopies(_ bytes: Int) throws {
        // UTF-8 conversion buffer plus the resulting owned String. SDK-owned CF
        // storage is a separate opaque category, not disguised as zero cost.
        let total = try nativeAdd(copiedUTF8, nativeMultiply(bytes, 2))
        guard total <= limits.copied_utf8_bytes else { throw NativeAcquisitionError.limit }
        copiedUTF8 = total
    }
    private func copyAdmitted(_ value: CFTypeRef, bytes: Int) throws -> String {
        try check()
        let string = unsafeDowncast(value, to: CFString.self)
        var buffer = [UInt8](repeating: 0, count: bytes)
        var used = 0
        let converted = buffer.withUnsafeMutableBufferPointer {
            CFStringGetBytes(string, CFRange(location: 0, length: CFStringGetLength(string)),
                CFStringBuiltInEncodings.UTF8.rawValue, 0, false, $0.baseAddress, bytes, &used)
        }
        actualUTF8 = try nativeAdd(actualUTF8, max(0, used))
        guard converted == CFStringGetLength(string), used == bytes,
              let result = String(bytes: buffer, encoding: .utf8) else { throw NativeAcquisitionError.invalidValue }
        actualUTF8 = try nativeAdd(actualUTF8, bytes)
        return result
    }
    func text(_ value: CFTypeRef) throws -> String {
        do {
            let size = try textSize(value, ceiling: limits.value_utf8_bytes)
            try reserveCopies(size)
            return try copyAdmitted(value, bytes: size)
        } catch { refusedValues += 1; throw error }
    }
    static func item(_ array: CFArray, _ index: Int) -> CFTypeRef {
        // CFArrayGetCount validated by each bounded caller; retaining one object
        // avoids creating a second full Swift array before admission.
        Unmanaged<AnyObject>.fromOpaque(CFArrayGetValueAtIndex(array, index)).takeUnretainedValue()
    }
    func batch(_ array: CFArray, expected: Int) throws -> [Any] {
        var accepted = false
        defer { if !accepted { refusedValues += 1 } }
        try check()
        let count = CFArrayGetCount(array)
        guard expected > 0, expected <= limits.batch_values, count == expected else { throw NativeAcquisitionError.limit }
        maximumBatch = max(maximumBatch, count)
        var sizes = [Int](repeating: 0, count: count)
        var total = 0
        // Whole batch preflight before any String conversion.
        for i in 0..<count {
            let value = Self.item(array, i)
            if CFGetTypeID(value) == CFStringGetTypeID() {
                sizes[i] = try textSize(value, ceiling: limits.value_utf8_bytes)
                total = try nativeAdd(total, sizes[i])
            }
        }
        guard try nativeMultiply(total, 2) <= limits.batch_utf8_bytes else { throw NativeAcquisitionError.limit }
        try reserveCopies(total)
        var result: [Any] = []; result.reserveCapacity(count)
        for i in 0..<count {
            let value = Self.item(array, i)
            result.append(CFGetTypeID(value) == CFStringGetTypeID() ? try copyAdmitted(value, bytes: sizes[i]) : value)
        }
        accepted = true
        return result
    }
    func actions(_ array: CFArray) throws -> [String] {
        var accepted = false
        defer { if !accepted { refusedValues += 1 } }
        try check()
        let count = CFArrayGetCount(array)
        guard count <= limits.action_names else { throw NativeAcquisitionError.limit }
        maximumActions = max(maximumActions, count)
        var sizes = [Int](repeating: 0, count: count)
        var total = 0
        for i in 0..<count {
            sizes[i] = try textSize(Self.item(array, i), ceiling: limits.action_name_utf8_bytes)
            total = try nativeAdd(total, sizes[i])
        }
        guard total <= (try nativeMultiply(limits.action_names, limits.action_name_utf8_bytes)) else { throw NativeAcquisitionError.limit }
        try reserveCopies(total)
        var result: [String] = []; result.reserveCapacity(count)
        for i in 0..<count { result.append(try copyAdmitted(Self.item(array, i), bytes: sizes[i])) }
        accepted = true
        return result
    }
    // Callbacks are the narrow nonvisual synthetic seam as well as the AX path.
    // Count must cover the whole candidate list before any uniqueness decision.
    func array(count: Int, windows: Bool, capacity: Int,
               read: (Int, Int) throws -> CFArray, visit: (CFTypeRef) throws -> Void) throws -> Int {
        try check()
        guard count >= 0, capacity >= 0 else { throw NativeAcquisitionError.invalidValue }
        if windows {
            windowsCounted = count
            guard count <= limits.ax_windows else { throw NativeAcquisitionError.limit }
        }
        let allowed = windows ? count : min(count, capacity, limits.child_entries - childEntries)
        var offset = 0
        while offset < allowed {
            try check()
            let ask = min(limits.array_page, allowed - offset)
            let page = try read(offset, ask); arrayCalls += 1
            let returned = CFArrayGetCount(page)
            guard returned > 0, returned <= ask else { throw NativeAcquisitionError.invalidValue }
            if windows { windowsRead = try nativeAdd(windowsRead, returned) }
            else { childEntries = try nativeAdd(childEntries, returned) }
            for index in 0..<returned { try check(); try visit(Self.item(page, index)) }
            offset += returned
        }
        return count - offset
    }
    func imageDimensions(width: Double, height: Double, scale: Double) throws -> (Int, Int) {
        try check()
        guard width.isFinite, height.isFinite, scale.isFinite, width > 0, height > 0, scale > 0 else { throw NativeAcquisitionError.invalidValue }
        let w = (width * scale).rounded(), h = (height * scale).rounded()
        guard w.isFinite, h.isFinite, w > 0, h > 0,
              w <= Double(limits.image_width), h <= Double(limits.image_height) else { throw NativeAcquisitionError.limit }
        let wi = Int(w), hi = Int(h)
        try imageFootprint(width: wi, height: hi, rowBytes: nativeMultiply(wi, 4))
        return (wi, hi)
    }
    func imageFootprint(width: Int, height: Int, rowBytes: Int) throws {
        try check()
        guard width > 0, height > 0, width <= limits.image_width, height <= limits.image_height,
              try nativeMultiply(width, height) <= limits.image_pixels,
              rowBytes >= (try nativeMultiply(width, 4)),
              try nativeMultiply(rowBytes, height) <= limits.image_bytes else { throw NativeAcquisitionError.limit }
    }
    func returnedImage(width: Int, height: Int, rowBytes: Int, requested: (Int, Int)) throws {
        guard width == requested.0, height == requested.1 else { throw NativeAcquisitionError.invalidValue }
        try imageFootprint(width: width, height: height, rowBytes: rowBytes)
    }
    var metrics: [String: Int] {
        ["copied_utf8_bytes": actualUTF8, "admitted_copied_utf8_bytes": copiedUTF8, "requested_utf8_bytes": requestedUTF8, "child_entries": childEntries, "array_calls": arrayCalls,
         "maximum_string_utf16_units": maximumStringUnits, "refused_values": refusedValues,
         "windows_counted": windowsCounted, "windows_read": windowsRead,
         "maximum_batch_values": maximumBatch, "maximum_action_names": maximumActions]
    }
}

// Pure pre-dispatch gate shared with AX acquisition; absence caused by admission/
// platform uncertainty never authorizes a value read. Known secure is distinct
// from unknown classification and produces redacted, not a fabricated empty value.
func nativeMayReadValue(role: String?, subrole: String?, identifier: String?, complete: Bool) -> Bool {
    complete && role != nil && role != "AXSecureTextField" && subrole != "AXSecureTextField" && identifier != "f02.secret"
}
