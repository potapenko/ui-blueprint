// Qualification-only CLI. --expect-trap deliberately reproduces the missing
// UpdateGroup guard observed in the real app; default mode must exit normally.
import Foundation
import SwiftUI
import RuntimeUpdateScope

@_silgen_name("$sSo11AGAttributea7SwiftUIE20syncMainIfReferences5flags19disqualifyAttribute2doxSo19AGChangedValueFlagsV_SbxyXEtlFZ")
func requireMainReferences<Value>(_ flags: UInt8, _ disqualify: Bool, _ body: () -> Value) -> Value

@main struct RuntimeScopeGuard {
    static func main() {
        let unscoped = CommandLine.arguments.contains("--expect-trap")
        let token = unscoped ? nil : UIBRuntimeUpdateBegin()
        guard unscoped || token != nil else { fatalError("source profile unavailable") }
        let result: Int = requireMainReferences(2, false) { 123 }
        if let token { UIBRuntimeUpdateEnd(token) }
        print("result=\(result),restored=\(UIBRuntimeUpdateCurrent())")
    }
}
