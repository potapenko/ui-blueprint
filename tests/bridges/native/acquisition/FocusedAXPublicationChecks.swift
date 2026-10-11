import Foundation
import ApplicationServices
import Darwin

private final class PublicationHandle: NSObject {
    let id: Int
    init(_ id: Int) { self.id = id }
}
private final class WeakPublicationHandle {
    weak var value: PublicationHandle?
    init(_ value: PublicationHandle) { self.value = value }
}
// Only the injected AX state flag crosses to the socket-draining thread.
private final class PublicationChange: @unchecked Sendable {
    private let lock = NSLock()
    private var changed = false
    func set() { lock.lock(); changed = true; lock.unlock() }
    var value: Bool { lock.lock(); defer { lock.unlock() }; return changed }
}
@MainActor private final class PublicationAX {
    var strong = (0..<4).map(PublicationHandle.init)
    var weakHandles: [WeakPublicationHandle] = []
    let change = PublicationChange()
    init() { weakHandles = strong.map(WeakPublicationHandle.init) }
    var retained: Int { weakHandles.filter { $0.value != nil }.count }
    func handle(_ id: Int) -> CFTypeRef? { weakHandles[id].value }
    var access: NativeAXAccess {
        NativeAXAccess(prepare: { _ in }, attribute: { raw, name in
            let id = (raw as! PublicationHandle).id
            switch name {
            case kAXFocusedWindowAttribute: return (.success, self.handle(1))
            // A changed focus deliberately leaves the popup for its parent window.
            case kAXFocusedUIElementAttribute: return (.success, self.handle(self.change.value ? 1 : 3))
            case kAXParentAttribute: return id > 1 ? (.success, self.handle(id - 1)) : (.noValue, nil)
            case kAXRoleAttribute: return (.success, (id == 1 ? "AXWindow" : id == 2 ? "AXPopover" : "AXButton") as CFString)
            default: return (.noValue, nil)
            }
        }, count: { raw, _ in (.success, (raw as! PublicationHandle).id == 2 ? 1 : 0) },
        page: { _, _, _, _ in (.success, [self.handle(3)!] as CFArray) },
        batch: { raw, names in
            let id = (raw as! PublicationHandle).id
            return (.success, names.map { name -> Any in
                switch name {
                case kAXRoleAttribute: return id == 2 ? "AXPopover" : "AXButton"
                case kAXSubroleAttribute, kAXIdentifierAttribute: return ""
                case kAXEnabledAttribute: return NSNumber(value: true)
                default: return NSNull()
                }
            } as CFArray)
        }, actions: { _ in (.success, [] as CFArray) }, isElement: { $0 is PublicationHandle })
    }
}

enum FocusedAXPublicationChecks {
    @MainActor static func run(profile: Any, request: [String: Any], header: NativeControl, output: URL) throws -> Int {
        var checks = 0
        func expect(_ value: Bool, line: UInt = #line) { precondition(value, "publication assertion at \(line)"); checks += 1 }
        for mode in ["success", "changed-before-reply", "changed-while-waiting", "closed-peer"] {
            let source = PublicationAX()
            let config: [String: Any] = ["collection": "focused-popover-ax", "process": ["pid": 123, "bundle_id": "ordinary.reader", "launch_time": 100.0], "scope_id": "selected-window", "acquisition_limits": profile]
            var req = request
            var limits = req["limits"] as! [String: Any]; limits["deadline_ms"] = 3000; req["limits"] = limits
            let document = try JSONSerialization.jsonObject(with: JSONSerialization.data(withJSONObject: ["schema_version": "0.1.0", "artifact": ["kind": "request", "data": req]])) as! [String: Any]
            let command = try NativeFocusedAX.command(NativeInbound(configurationBytes: JSONSerialization.data(withJSONObject: config), document: document, control: header, replyCap: 524288, deadline: NativeDescriptorIO.now + 3, started: NativeDescriptorIO.now))
            var frame: NativeJSONFrame? = try autoreleasepool {
                try NativeFocusedAX.collect(command, access: source.access,
                    processCurrent: { _ in true }, application: { _ in source.handle(0)! },
                    windowOwner: { _ in 123 }, permitted: { true })
            }
            source.strong.removeAll()
            expect(source.retained == 4) // No fixture strong owner survives collect.
            var fds: [Int32] = [-1, -1]
            guard socketpair(AF_UNIX, SOCK_STREAM, 0, &fds) == 0 else { throw NativeAcquisitionError.io }
            let io = try NativeDescriptorIO(input: fds[0], output: fds[0])
            if mode == "changed-before-reply" { source.change.set() }
            if mode == "changed-while-waiting" {
                // Fill the real output descriptor so reply must wait in poll.
                let filler = [UInt8](repeating: 120, count: 4096)
                var pending = 0
                while true {
                    let n = filler.withUnsafeBytes { Darwin.write(fds[0], $0.baseAddress, $0.count) }
                    if n > 0 { pending += n; continue }
                    guard n < 0 && errno == EAGAIN else { throw NativeAcquisitionError.io }
                    break
                }
                let amount = pending, peer = fds[1], change = source.change
                expect(amount > 0)
                DispatchQueue.global().asyncAfter(deadline: .now() + 0.05) {
                    change.set() // Happens after collect, while real reply awaits readiness.
                    var left = amount, buffer = [UInt8](repeating: 0, count: 4096)
                    while left > 0 {
                        let n = buffer.withUnsafeMutableBytes { Darwin.read(peer, $0.baseAddress, min(left, $0.count)) }
                        if n <= 0 { break }
                        left -= n
                    }
                }
            }
            if mode == "closed-peer" { close(fds[1]); fds[1] = -1 }
            var refused = false
            do { try autoreleasepool { try io.reply(frame!, cap: 524288, deadline: command.deadline) } }
            catch { refused = true }
            expect(source.retained == 4) // Includes failed descriptor delivery.
            expect(refused == (mode == "closed-peer"))
            if !refused {
                var bytes = [UInt8](repeating: 0, count: 524288)
                let count = bytes.withUnsafeMutableBytes { Darwin.read(fds[1], $0.baseAddress, $0.count) }
                expect(count > 0 && bytes[count - 1] == 10)
                let data = Data(bytes.prefix(count))
                try data.write(to: output.appendingPathComponent("publication-" + mode + ".json"), options: .withoutOverwriting)
                let result = (((try JSONSerialization.jsonObject(with: data) as! [String: Any])["artifact"] as! [String: Any])["data"] as! [String: Any])["result"] as! [String: Any]
                expect(result["status"] as? String == (mode == "success" ? "observed" : "failed"))
                if mode != "success" { expect((result["data"] as! [String: Any])["code"] as? String == "stale_target") }
            }
            close(fds[0]); if fds[1] >= 0 { close(fds[1]) }
            weak let releasedFrame = frame
            frame = nil
            expect(releasedFrame == nil)
            expect(source.retained == 0) // No frame/validator/binding reference cycle.
        }
        // A changed binding after a real short write cannot replace an emitted
        // prefix, finish that observation, or revive the frame on a second reply.
        var fds: [Int32] = [-1, -1]
        guard socketpair(AF_UNIX, SOCK_STREAM, 0, &fds) == 0 else { throw NativeAcquisitionError.io }
        defer { close(fds[0]); close(fds[1]) }
        var bufferSize: Int32 = 4096
        guard setsockopt(fds[0], SOL_SOCKET, SO_SNDBUF, &bufferSize, socklen_t(MemoryLayout<Int32>.size)) == 0 else { throw NativeAcquisitionError.io }
        let io = try NativeDescriptorIO(input: fds[0], output: fds[0])
        let deadline = NativeDescriptorIO.now + 3
        let frame = try NativeJSONFrame(capacity: 524288, deadline: deadline)
        try frame.encode(["bounded_transport_payload": String(repeating: "x", count: 200000)])
        var validations = 0
        frame.publicationValidation = {
            validations += 1
            return validations == 1 ? nil : ["status": "failed"]
        }
        let readSignal = DispatchSemaphore(value: 0), readFlag = PublicationChange(), peer = fds[1]
        DispatchQueue.global().async {
            var bytes = [UInt8](repeating: 0, count: 4096)
            let count = bytes.withUnsafeMutableBytes { Darwin.read(peer, $0.baseAddress, $0.count) }
            if count > 0 { readFlag.set() }
            readSignal.signal()
        }
        var refused = false
        do { try io.reply(frame, cap: 524288, deadline: deadline) } catch { refused = true }
        expect(refused && frame.failed && validations == 2)
        expect(readSignal.wait(timeout: .now() + 1) == .success && readFlag.value)
        do { try io.reply(frame, cap: 524288, deadline: deadline); preconditionFailure("revived partial frame") }
        catch { checks += 1 }
        return checks
    }
}
