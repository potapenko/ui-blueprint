#if DEBUG
import Foundation
import RuntimeUpdateScope

enum RuntimeUpdateContext {
    @MainActor static func perform<Value>(_ body: () throws -> Value) throws -> Value {
        #if !arch(arm64)
        throw RuntimeRefusal.unsupportedRuntimeProfile
        #else
        let os = ProcessInfo.processInfo.operatingSystemVersion
        #if os(iOS) && targetEnvironment(simulator)
        let supported = os.majorVersion == 27 && os.minorVersion == 2 && os.patchVersion == 0
        #elseif os(macOS)
        let supported = os.majorVersion == 27 && os.minorVersion == 0 && os.patchVersion == 1
        #else
        let supported = false
        #endif
        guard supported else {
            throw RuntimeRefusal.unsupportedRuntimeProfile
        }
        guard let token = UIBRuntimeUpdateBegin() else { throw RuntimeRefusal.updateContextUnavailable }
        defer { UIBRuntimeUpdateEnd(token) }
        return try body()
        #endif
    }
}
#endif
