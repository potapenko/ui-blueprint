import Foundation
import Darwin

// Charges the existing Foundation representation before construction. This is a
// small constructor boundary, not a second graph DTO, parser or JSON serializer.
final class NativeJSON {
    let limits: NativeAcquisitionLimits
    private(set) var slots = 0
    private(set) var stringBytes = 0
    init(_ limits: NativeAcquisitionLimits) { self.limits = limits }

    private func charge(slots count: Int, strings bytes: Int) throws {
        let nextSlots = try nativeAdd(slots, count)
        let nextStrings = try nativeAdd(stringBytes, bytes)
        guard nextSlots <= limits.response_slots, nextStrings <= limits.response_string_utf8_bytes
        else { throw NativeAcquisitionError.limit }
        slots = nextSlots; stringBytes = nextStrings
    }
    func object(_ keys: [String], _ build: () throws -> [String: Any]) throws -> [String: Any] {
        let bytes = try keys.reduce(0) { try nativeAdd($0, $1.utf8.count) }
        try charge(slots: nativeAdd(1, keys.count), strings: bytes)
        return try build()
    }
    func array<T>(_ build: () throws -> [T]) throws -> [T] {
        try charge(slots: 1, strings: 0)
        return try build()
    }
    func scalar<T>(_ value: T) throws -> T {
        try charge(slots: 1, strings: (value as? String)?.utf8.count ?? 0)
        return value
    }
    // Already-owned request Context/source keys may be reused only after charging
    // each output occurrence. No clone of that input is constructed by this walk.
    func borrowed<T>(_ value: T) throws -> T {
        if let object = value as? [String: Any] {
            _ = try self.object(Array(object.keys)) {
                for child in object.values { _ = try borrowed(child) }
                return object
            }
        } else if let array = value as? [Any] {
            _ = try self.array {
                for child in array { _ = try borrowed(child) }
                return array
            }
        } else { _ = try scalar(value) }
        return value
    }
    func known(_ type: String, _ value: Any) throws -> [String: Any] {
        try knownBuilt(type) { try borrowed(value) }
    }
    func knownBuilt(_ type: String, _ value: () throws -> Any) throws -> [String: Any] {
        try object(["availability", "value"]) {
            ["availability": try scalar("known"), "value": try object(["type", "value"]) {
                ["type": try scalar(type), "value": try value()]
            }]
        }
    }
    func unavailable(_ reason: String, status: String = "unknown") throws -> [String: Any] {
        try object(["availability", "reason"]) {
            ["availability": try scalar(status), "reason": try scalar(reason)]
        }
    }
    func redacted() throws -> [String: Any] {
        try object(["availability"]) { ["availability": try scalar("redacted")] }
    }
    func evidence(_ observation: String, _ source: String, _ method: String) throws -> [String: Any] {
        try object(["observation_id", "source_namespace", "provenance", "method", "uncertainty"]) {
            ["observation_id": try scalar(observation), "source_namespace": try scalar(source),
             "provenance": try scalar("reported"), "method": try scalar(method), "uncertainty": try scalar(NSNull())]
        }
    }
    func failure(_ code: String, scope: String, channel: String) throws -> [String: Any] {
        try object(["status", "data"]) {
            ["status": try scalar("failed"), "data": try object(["code", "scope_id", "failed_step", "recovery_class"]) {
                ["code": try scalar(code), "scope_id": try scalar(scope), "failed_step": try scalar(channel),
                 "recovery_class": try scalar(code == "permission_required" ? "explicit_permission" : "new_explicit_request")]
            }]
        }
    }
    func envelope(_ kind: String, data: () throws -> [String: Any]) throws -> [String: Any] {
        try object(["schema_version", "artifact"]) {
            ["schema_version": try scalar("0.1.0"), "artifact": try object(["kind", "data"]) {
                ["kind": try scalar(kind), "data": try data()]
            }]
        }
    }
    func response(request: [String: Any], ticket: UInt64, channel: String,
                  result: () throws -> [String: Any]) throws -> [String: Any] {
        guard let id = request["request_id"] as? String, let context = request["context"] as? [String: Any],
              let session = context["session_id"] as? String, let target = context["target"]
        else { throw NativeAcquisitionError.invalidValue }
        return try envelope("channel_response") {
            try object(["request_id", "session_id", "dispatch_sequence", "target", "channel", "result"]) {
                ["request_id": try scalar(id), "session_id": try scalar(session), "dispatch_sequence": try scalar(ticket),
                 "target": try borrowed(target), "channel": try scalar(channel), "result": try result()]
            }
        }
    }
}

// A single reserved destination, including LF. The Foundation codec's internal
// scratch is opaque; writeJSONObject is not a claim about its internal allocator.
final class NativeJSONFrame: OutputStream {
    private var storage: [UInt8]
    private var used = 0
    private(set) var failed = false
    private var status: Stream.Status = .notOpen
    private let deadline: Double
    private var finished = false
    var count: Int { used }

    init(capacity: Int, deadline: Double) throws {
        guard capacity > 1, deadline.isFinite else { throw NativeAcquisitionError.limit }
        storage = [UInt8](repeating: 0, count: capacity)
        self.deadline = deadline
        super.init(toMemory: ())
    }
    override func open() { status = .open }
    override func close() { status = .closed }
    override var streamStatus: Stream.Status { status }
    override var hasSpaceAvailable: Bool { !failed && !finished && used < storage.count - 1 }
    override func write(_ buffer: UnsafePointer<UInt8>, maxLength length: Int) -> Int {
        guard status == .open, !failed, !finished, length >= 0,
              ProcessInfo.processInfo.systemUptime < deadline,
              length <= storage.count - 1 - used else { failed = true; status = .error; return -1 }
        storage.withUnsafeMutableBufferPointer { target in
            target.baseAddress!.advanced(by: used).update(from: buffer, count: length)
        }
        used += length
        return length
    }
    func encode(_ object: Any) throws {
        guard !finished, used == 0 else { throw NativeAcquisitionError.invalidValue }
        open()
        var error: NSError?
        let written = JSONSerialization.writeJSONObject(object, to: self, options: [.sortedKeys], error: &error)
        guard written > 0, written == used, !failed, error == nil,
              ProcessInfo.processInfo.systemUptime < deadline else { throw NativeAcquisitionError.limit }
        storage[used] = 10; used += 1; finished = true; close()
    }
    func reset() {
        for i in 0..<used { storage[i] = 0 }
        used = 0; failed = false; finished = false; status = .notOpen
    }
    func write(to fd: Int32, deadline: Double) throws {
        guard finished, !failed else { throw NativeAcquisitionError.invalidValue }
        var offset = 0
        while offset < used {
            let remaining = deadline - ProcessInfo.processInfo.systemUptime
            guard remaining > 0 else { throw NativeAcquisitionError.expired }
            var interest = pollfd(fd: fd, events: Int16(POLLOUT), revents: 0)
            let ready = poll(&interest, 1, Int32(min(1000, max(1, ceil(remaining * 1000)))))
            if ready == 0 || (ready < 0 && errno == EINTR) { continue }
            guard ready > 0 else { throw NativeAcquisitionError.io }
            let n = storage.withUnsafeBufferPointer { Darwin.write(fd, $0.baseAddress!.advanced(by: offset), used - offset) }
            if n < 0 && (errno == EINTR || errno == EAGAIN) { continue }
            guard n > 0 else { throw NativeAcquisitionError.io }
            offset += n
        }
    }
    func bytes<T>(_ body: (UnsafeBufferPointer<UInt8>) throws -> T) rethrows -> T {
        try storage.withUnsafeBufferPointer { try body(UnsafeBufferPointer(start: $0.baseAddress, count: used)) }
    }
}
