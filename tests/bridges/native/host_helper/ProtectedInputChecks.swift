import Foundation
import Darwin

// Tests production delivery reader, with injected backend errors/cancel; no SDK
// actions. Canary itself is caller-owned input and never printed, even on failure.
@main struct ProtectedInputChecks {
    struct BackendFailure: Error { let privateText: String }
    static func main() throws {
        let path = CommandLine.arguments[1]
        let source = NativeConfiguration.ProtectedInput(reference: "opaque-once", action_id: "protected-step", identifier: "f02.secret", path: path, trace: true)
        let profile = try Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[2]))
        func admission(_ limit: Int = 4096, _ deadline: Double = NativeDescriptorIO.now + 5, copyCap: Int = 262144) throws -> NativeAcquisition {
            var object = try JSONSerialization.jsonObject(with: profile) as! [String: Any]
            object["value_utf8_bytes"] = limit
            object["copied_utf8_bytes"] = copyCap
            return try NativeAcquisition(JSONDecoder().decode(NativeAcquisitionLimits.self, from: JSONSerialization.data(withJSONObject: object)), deadline: deadline)
        }
        if CommandLine.arguments.count == 4 {
            do {
                let _: Bool = try NativeProtectedSource.withValue(source, admission: try admission()) { _ in preconditionFailure("invalid source delivered") }
                preconditionFailure("invalid source admitted")
            } catch NativeProtocolError.request { print("{\"source_refused\":true}"); return }
        }
        let canary = try String(contentsOfFile: path, encoding: .utf8)
        var delivered = 0
        let result = try NativeProtectedSource.withValue(source, admission: try admission()) { value in
            precondition(value == canary, "delivery content mismatch")
            delivered += 1
            NativeProtectedSource.trace(.dispatch, enabled: true)
            return true
        }
        precondition(result && delivered == 1)
        var failures = 0
        // Backend is allowed temporary access; malicious/private exception data
        // MUST be replaced by a fixed error before returning across the boundary.
        for cancel in [false, true] {
            do {
                let _: Bool = try NativeProtectedSource.withValue(source, admission: try admission()) { value in
                    if cancel { throw NativeProtocolError.closed }
                    throw BackendFailure(privateText: value)
                }
                preconditionFailure("expected backend refusal")
            } catch NativeProtocolError.request { failures += 1 }
        }
        for limit in [1, canary.utf8.count - 1] {
            do {
                let _: Bool = try NativeProtectedSource.withValue(source, admission: try admission(limit)) { _ in preconditionFailure("pre-read refusal dispatched") }
                preconditionFailure("expected limit refusal")
            } catch NativeProtocolError.request { failures += 1 }
        }
        do {
            let expired = try admission(4096, NativeDescriptorIO.now + 0.002)
            Thread.sleep(forTimeInterval: 0.003)
            let _: Bool = try NativeProtectedSource.withValue(source, admission: expired) { _ in preconditionFailure("cancelled source read") }
            preconditionFailure("expired source admitted")
        } catch NativeProtocolError.request { failures += 1 }
        do {
            let _: Bool = try NativeProtectedSource.withValue(source, admission: try admission(copyCap: canary.utf8.count)) { _ in preconditionFailure("copy budget exceeded") }
            preconditionFailure("copy cap admitted")
        } catch NativeProtocolError.request { failures += 1 }
        precondition(failures == 6)
        print("{\"delivery_reader\":true,\"backend_error\":true,\"cancel_before_after_read\":true,\"limit_refusals\":3,\"trace_enabled\":true,\"sdk_input\":false}")
    }
}
