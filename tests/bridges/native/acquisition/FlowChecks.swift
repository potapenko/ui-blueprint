import Foundation
import ApplicationServices
import Darwin

// Only the actual AX calls are substituted. Traversal, type admission, identity
// schedule, canonical construction and final FD delivery stay in production code.
@MainActor final class SyntheticAX {
    enum WindowFault { case none, changedCount, countError, pageError, wrongType }
    enum Identity { case ordinary, secure, failed, oversized }
    var children: [Int: [CFTypeRef]] = [:]
    var windows = [1, 2]
    var identifiers = [1: "a", 2: "b", 3: "c"]
    var windowFault = WindowFault.none
    var identity = Identity.ordinary
    var countCalls = 0, pageCalls = 0, identifierReads = 0, valueDispatches = 0
    var batchSchedule: [[String]] = []
    static func handle(_ value: Int) -> CFTypeRef { NSNumber(value: value) }
    func access() -> NativeAXAccess {
        NativeAXAccess(prepare: { _ in }, attribute: { raw, name in
            self.identifierReads += 1
            let key = (raw as! NSNumber).intValue
            if name == kAXIdentifierAttribute, let value = self.identifiers[key] { return (.success, value as CFString) }
            return (.noValue, nil)
        }, count: { raw, name in
            if name == kAXWindowsAttribute {
                self.countCalls += 1
                if self.windowFault == .countError { return (.cannotComplete, 0) }
                return (.success, self.windows.count + (self.windowFault == .changedCount && self.countCalls > 1 ? 1 : 0))
            }
            return (.success, self.children[(raw as! NSNumber).intValue, default: []].count)
        }, page: { raw, name, index, amount in
            self.pageCalls += 1
            if name == kAXWindowsAttribute {
                if self.windowFault == .pageError { return (.cannotComplete, nil) }
                if self.windowFault == .wrongType { return (.success, ["not-an-element"] as CFArray) }
                return (.success, self.windows[index..<min(self.windows.count, index + amount)].map(Self.handle) as CFArray)
            }
            let values = self.children[(raw as! NSNumber).intValue, default: []]
            return (.success, Array(values[index..<min(values.count, index + amount)]) as CFArray)
        }, batch: { raw, names in
            self.batchSchedule.append(names)
            let key = (raw as! NSNumber).intValue
            if names.contains(kAXRoleAttribute) && self.identity == .failed { return (.cannotComplete, nil) }
            if names.contains(kAXValueAttribute) { self.valueDispatches += 1 }
            let values: [Any] = names.map { name in
                switch name {
                case kAXRoleAttribute: return "AXTextField"
                case kAXSubroleAttribute: return self.identity == .secure ? "AXSecureTextField" : ""
                case kAXIdentifierAttribute:
                    if self.identity == .oversized { return String(repeating: "x", count: 4097) }
                    return self.identity == .secure ? "f02.secret" : "f02.name"
                case kAXDescriptionAttribute: return "synthetic-\(key)"
                case kAXPlaceholderValueAttribute: return ""
                case kAXEnabledAttribute: return NSNumber(value: true)
                case kAXFocusedAttribute: return NSNumber(value: false)
                case kAXValueAttribute: return "ordinary-value"
                default: return NSNull()
                }
            }
            return (.success, values as CFArray)
        }, actions: { _ in (.success, ["AXPress"] as CFArray) },
        isElement: { CFGetTypeID($0) == CFNumberGetTypeID() })
    }
}

@main struct FlowChecks {
    @MainActor static func main() async throws {
        guard CommandLine.arguments.count == 5 else { exit(2) }
        let profile = try JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1]))) as! [String: Int]
        let requestDocument = try JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[2]))) as! [String: Any]
        let request = (requestDocument["artifact"] as! [String: Any])["data"] as! [String: Any]
        let savedSample = try JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[3])))
        let output = URL(fileURLWithPath: CommandLine.arguments[4], isDirectory: true)
        var assertions = 0
        func check(_ value: Bool) { precondition(value, "focused owner proof failed"); assertions += 1 }
        func refusal(_ body: () throws -> Void) {
            do { try body(); fatalError("expected owner refusal") } catch { assertions += 1 }
        }
        func limits(_ change: [String: Int] = [:]) throws -> NativeAcquisitionLimits {
            let result = try JSONDecoder().decode(NativeAcquisitionLimits.self,
                from: JSONSerialization.data(withJSONObject: profile.merging(change, uniquingKeysWith: { _, b in b })))
            try result.validate(); return result
        }
        func admission(_ change: [String: Int] = [:]) throws -> NativeAcquisition {
            try NativeAcquisition(limits(change), deadline: ProcessInfo.processInfo.systemUptime + 10)
        }
        let surface: [String: Any] = ["id": "window-1", "generation": "g1"]
        func collect(_ source: SyntheticAX, nodes: Int = 8) throws -> WindowAXResult {
            let quota = try admission()
            return try collectWindowAX(SyntheticAX.handle(1), surface: surface, observationID: "synthetic-observation",
                maxNodes: nodes, maxDepth: 4, deadline: quota.deadline, admission: quota,
                fields: ["role", "description", "value", "placeholder", "enabled", "focused", "actions", "accessibility_bounds"],
                json: NativeJSON(quota.limits), access: source.access())
        }
        func children(_ result: WindowAXResult, node: Int) -> [[String: Any]] { result.nodes[node]["children"] as! [[String: Any]] }
        func valueState(_ result: WindowAXResult) -> [String: Any] {
            (result.nodes[0]["properties"] as! [[String: Any]]).first { $0["field"] as? String == "value" }!["state"] as! [String: Any]
        }
        // 1. Actual traversal/type owners: duplicates/cycles, omissions, and the
        // binding owner on changing/error/type-invalid window enumeration.
        let graph = SyntheticAX()
        graph.children = [1: [2, 2, 3].map(SyntheticAX.handle), 2: [SyntheticAX.handle(1)], 3: []]
        let traversed = try collect(graph)
        check(traversed.nodes.count == 3)
        check(traversed.metrics["duplicate_handle_references"] as? Int == 2)
        check(traversed.metrics["known_unread_child_entries"] as? Int == 0)
        check(children(traversed, node: 0).count == 2)
        check((children(traversed, node: 1)[0] as NSDictionary).isEqual(traversed.nodes[0]["key"] as! NSDictionary))
        // This deliberately cyclic raw AX relation is bounded source data, not a
        // claim that a cyclic canonical graph passed Rust validation/publication.
        let limited = try collect(graph, nodes: 2)
        check(limited.nodes.count == 2)
        check(limited.metrics["known_unread_child_entries"] as? Int == 3)
        let wrongChild = SyntheticAX(); wrongChild.children = [1: ["bad-handle" as CFString]]
        let typed = try collect(wrongChild)
        check(typed.nodes.count == 1 && children(typed, node: 0).isEmpty)
        check(typed.metrics["unknown_children_lists"] as? Int == 1)
        let invalidRoot = SyntheticAX()
        refusal { _ = try Collector.resolveWindow("invalid-root" as CFString, identifier: "a", admission: admission(), access: invalidRoot.access()) }
        check(invalidRoot.countCalls == 0 && invalidRoot.identifierReads == 0)
        let windows = SyntheticAX()
        let bound = try Collector.resolveWindow(SyntheticAX.handle(0), identifier: "a", admission: admission(), access: windows.access())
        check(CFEqual(bound, SyntheticAX.handle(1)))
        check(windows.countCalls == 2 && windows.identifierReads == 2)
        for fault in [SyntheticAX.WindowFault.changedCount, .countError, .pageError, .wrongType] {
            let source = SyntheticAX(); source.windowFault = fault
            refusal { _ = try Collector.resolveWindow(SyntheticAX.handle(0), identifier: "a", admission: admission(), access: source.access()) }
            if fault == .wrongType { check(source.identifierReads == 0) }
            if fault == .countError { check(source.pageCalls == 0) }
        }
        let saturated = SyntheticAX()
        refusal { _ = try Collector.resolveWindow(SyntheticAX.handle(0), identifier: "a", admission: admission(["ax_windows": 1]), access: saturated.access()) }
        check(saturated.pageCalls == 0 && saturated.identifierReads == 0)
        let ambiguous = SyntheticAX(); ambiguous.identifiers[2] = "a"
        refusal { _ = try Collector.resolveWindow(SyntheticAX.handle(0), identifier: "a", admission: admission(), access: ambiguous.access()) }

        // 2. Actual acquisition -> attribute schedule, not a predicate-only test.
        for (identity, expectedDispatch, expectedAvailability) in [
            (SyntheticAX.Identity.ordinary, 1, "known"), (.secure, 0, "redacted"),
            (.failed, 0, "unknown"), (.oversized, 0, "unknown")] {
            let source = SyntheticAX(); source.identity = identity
            let result = try collect(source)
            check(source.valueDispatches == expectedDispatch)
            check(source.batchSchedule.filter { $0.contains(kAXValueAttribute) }.count == expectedDispatch)
            check(valueState(result)["availability"] as? String == expectedAvailability)
            check(source.batchSchedule.count == 2)
        }

        // 3. Collector's actual terminal path + real bounded FD receiver.
        func sockets(small: Bool = false) throws -> [Int32] {
            var fds = [Int32](repeating: -1, count: 2)
            guard socketpair(AF_UNIX, SOCK_STREAM, 0, &fds) == 0 else { throw NativeAcquisitionError.io }
            for fd in fds {
                let flags = fcntl(fd, F_GETFL)
                guard fcntl(fd, F_SETFL, flags | O_NONBLOCK) == 0 else { throw NativeAcquisitionError.io }
            }
            var yes: Int32 = 1
            guard setsockopt(fds[0], SOL_SOCKET, SO_NOSIGPIPE, &yes, socklen_t(MemoryLayout<Int32>.size)) == 0 else { throw NativeAcquisitionError.io }
            if small {
                var amount: Int32 = 1024
                guard setsockopt(fds[0], SOL_SOCKET, SO_SNDBUF, &amount, socklen_t(MemoryLayout<Int32>.size)) == 0 else { throw NativeAcquisitionError.io }
            }
            return fds
        }
        func receive(_ fd: Int32, cap: Int) throws -> Data {
            let until = ProcessInfo.processInfo.systemUptime + 2
            var result = Data(), bytes = [UInt8](repeating: 0, count: 1024)
            while ProcessInfo.processInfo.systemUptime < until {
                var interest = pollfd(fd: fd, events: Int16(POLLIN), revents: 0)
                if poll(&interest, 1, 50) <= 0 { continue }
                let n = Darwin.read(fd, &bytes, bytes.count)
                if n == 0 { return result }
                if n < 0 && (errno == EINTR || errno == EAGAIN) { continue }
                guard n > 0, n <= cap - result.count else { throw NativeAcquisitionError.limit }
                result.append(contentsOf: bytes.prefix(n))
            }
            throw NativeAcquisitionError.expired
        }
        func failure(_ json: NativeJSON) throws -> [String: Any] {
            try json.response(request: request, ticket: 7, channel: "external_semantics") {
                try json.failure("incomplete_scope", scope: "form-1", channel: "external_semantics")
            }
        }
        let sizing = NativeJSON(try limits()); _ = try failure(sizing)
        for mode in ["construction", "codec", "no-capacity"] {
            let json = NativeJSON(try limits(mode == "construction" ? ["response_slots": sizing.slots] : [:]))
            let fallback = try failure(json)
            let frame = try NativeJSONFrame(capacity: mode == "no-capacity" ? 2 : 2048,
                deadline: ProcessInfo.processInfo.systemUptime + 2)
            let pair = try sockets(); defer { close(pair[0]); close(pair[1]) }
            var sends = 0, didThrow = false, constructed = false
            do {
                try await Collector.finishChannel(frame: frame, failure: fallback, prepare: {
                    if mode == "construction" {
                        _ = try json.object(["not_admitted"]) { constructed = true; return [:] }
                    } else {
                        // The known old canonical body is larger than this output
                        // cap; this is actual codec refusal without new collection.
                        try frame.encode(savedSample)
                    }
                }, evidence: {}, send: { complete in
                    sends += 1
                    try complete.write(to: pair[0], deadline: ProcessInfo.processInfo.systemUptime + 2)
                })
            } catch { didThrow = true }
            shutdown(pair[0], SHUT_WR)
            let received = try receive(pair[1], cap: 4096)
            if mode == "no-capacity" { check(didThrow && sends == 0 && received.isEmpty) }
            else {
                check(!didThrow && sends == 1 && received.last == 10 && received.filter { $0 == 10 }.count == 1)
                let document = try JSONSerialization.jsonObject(with: received) as! [String: Any]
                check((document as NSDictionary).isEqual(fallback as NSDictionary))
                try received.write(to: output.appendingPathComponent(mode + ".ndjson"), options: .withoutOverwriting)
            }
            if mode == "construction" { check(!constructed) }
        }
        let json = NativeJSON(try limits()), fallback = try failure(json)
        let frame = try NativeJSONFrame(capacity: 524288, deadline: ProcessInfo.processInfo.systemUptime + 3)
        let pair = try sockets(small: true)
        defer { close(pair[0]) } // receiver owns/closes its end
        let receiverFD = pair[1]
        let receiver = Task.detached { () -> Int in
            defer { close(receiverFD) }
            let until = ProcessInfo.processInfo.systemUptime + 2
            var buffer = [UInt8](repeating: 0, count: 256)
            while ProcessInfo.processInfo.systemUptime < until {
                var interest = pollfd(fd: receiverFD, events: Int16(POLLIN), revents: 0)
                if poll(&interest, 1, 50) <= 0 { continue }
                let n = Darwin.read(receiverFD, &buffer, buffer.count)
                if n > 0 { return n } // close after a real prefix
                if n == 0 { return 0 }
            }
            return 0
        }
        var sends = 0, sendFailed = false
        do {
            try await Collector.finishChannel(frame: frame, failure: fallback, prepare: { try frame.encode(savedSample) },
                evidence: {}, send: { complete in
                    sends += 1
                    try complete.write(to: pair[0], deadline: ProcessInfo.processInfo.systemUptime + 2)
                })
        } catch { sendFailed = true }
        let prefixBytes = await receiver.value
        check(sendFailed && sends == 1)
        check(prefixBytes > 0 && prefixBytes < frame.count)
        print("{\"suites\":3,\"assertions\":\(assertions),\"partial_fd_prefix_bytes\":\(prefixBytes),\"send_attempts_total\":\(sends),\"live_ax_sck\":false}")
    }
}
