import Foundation
@main struct FormOrderChecks {
    static func control(_ phase: UInt8, _ op: UInt64, epoch: UInt64 = 11, nonce: UInt64 = 7, kind: UInt8 = 9) throws -> NativeControl {
        var bytes = Data(repeating: 0, count: 64)
        bytes.replaceSubrange(0..<8, with: Data("UIBHST01".utf8))
        bytes[8] = 3; bytes[9] = kind; bytes[11] = 128 | phase
        for (offset, value) in [(16, epoch), (24, op), (40, nonce)] {
            for i in 0..<8 { bytes[offset+i] = UInt8(truncatingIfNeeded: value >> (i*8)) }
        }
        return try NativeControl(bytes)
    }
    static func main() throws {
        var order = NativeFormOrder(epoch: 11, serial: 5)
        var checks = 0
        func refuse(_ c: NativeControl, serial: UInt64 = 5) {
            do { try order.accept(c, serial: serial); fatalError("unexpected acceptance") } catch { checks += 1 }
        }
        refuse(try control(2, 1))
        try order.accept(control(0, 1, kind: 8), serial: 5)
        refuse(try control(0, 1, kind: 8))
        refuse(try control(1, 2, epoch: 12))
        refuse(try control(1, 2), serial: 6)
        try order.accept(control(1, 2, kind: 10), serial: 5)
        refuse(try control(2, 2))
        try order.accept(control(1, 3), serial: 5)
        refuse(try control(3, 3))
        try order.accept(control(2, 3, nonce: 8), serial: 5)
        refuse(try control(2, 3, nonce: 8))
        refuse(try control(3, 4))
        try order.accept(control(3, 3), serial: 5)
        try order.accept(control(1, 4), serial: 5)
        refuse(try control(2, 4, nonce: 8))
        try order.accept(control(2, 4, nonce: 9), serial: 5)
        try order.accept(control(3, 4), serial: 5)
        precondition(checks == 9 && order.lastNonce == 9)
        print("{\"order_refusals\":9,\"valid_sequences\":2,\"sdk_input\":false}")
    }
}
