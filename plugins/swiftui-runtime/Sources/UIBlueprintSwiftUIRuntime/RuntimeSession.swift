#if DEBUG
import AppKit
import CryptoKit
import Darwin
import Foundation

/// A development-only, process-owned geometry endpoint. Retain it for the app's lifetime.
@MainActor
public final class BlueprintRuntimeSession {
    public let directory: URL
    private let runID = UUID().uuidString
    private let capability = UUID().uuidString + UUID().uuidString
    private let bundle: String
    private let collector = RuntimeCollector()
    private var source: (any DispatchSourceFileSystemObject)?
    private var sequence: UInt64 = 0
    private var processing = false

    /// Activation is explicit and process-local; ordinary Debug/Release runs do not start a listener.
    public static func startIfRequested() throws -> BlueprintRuntimeSession? {
        guard ProcessInfo.processInfo.environment["UIBLUEPRINT_RUNTIME"] == "1" else { return nil }
        guard let bundle = Bundle.main.bundleIdentifier, !bundle.isEmpty else { throw RuntimeRefusal.invalidRequest }
        let session = try BlueprintRuntimeSession(bundle: bundle)
        try session.start()
        return session
    }

    private init(bundle: String) throws {
        self.bundle = bundle
        directory = FileManager.default.temporaryDirectory.appendingPathComponent("uiblueprint-runtime-\(UUID().uuidString)")
        try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: false,
                                               attributes: [.posixPermissions: 0o700])
        let ownership = "Owner: \(bundle), PID \(ProcessInfo.processInfo.processIdentifier), run \(runID)\nPurpose: diagnostic geometry requests\nState: active\nDisposable: acknowledged request/result files only; retained captures require consumer release.\n"
        try Data(ownership.utf8).write(to: directory.appendingPathComponent("OWNERSHIP.md"), options: .withoutOverwriting)
    }

    private func start() throws {
        let fd = open(directory.path, O_EVTONLY | O_NOFOLLOW | O_CLOEXEC)
        guard fd >= 0 else { throw RuntimeRefusal.outputFailure }
        let listener = DispatchSource.makeFileSystemObjectSource(fileDescriptor: fd, eventMask: [.write, .rename, .delete], queue: .main)
        listener.setEventHandler { [weak self] in Task { @MainActor [weak self] in self?.consume() } }
        listener.setCancelHandler { close(fd) }
        source = listener
        listener.resume()
        let descriptor: [String: Any] = ["protocol": "1.0.0", "source": "swiftui.runtime.layout",
            "run_id": runID, "capability": capability, "pid": ProcessInfo.processInfo.processIdentifier,
            "bundle": bundle, "directory": directory.path,
            "os": ProcessInfo.processInfo.operatingSystemVersionString,
            "profile": ProcessInfo.processInfo.environment["SWIFTUI_VIEW_DEBUG"] ?? "unset"]
        try writeJSON(descriptor, name: "session.json")
        try FileHandle.standardError.write(contentsOf: Data("UIBLUEPRINT_SESSION \(directory.appendingPathComponent("session.json").path)\n".utf8))
    }

    public func stop() {
        source?.cancel()
        source = nil
    }

    private func consume() {
        guard source != nil, !processing else { return }
        processing = true
        defer { processing = false }
        guard let files = try? FileManager.default.contentsOfDirectory(at: directory, includingPropertiesForKeys: nil) else { return }
        let requests = files.filter { $0.lastPathComponent.hasSuffix(".request.json") }.sorted { $0.lastPathComponent < $1.lastPathComponent }
        let overloaded = requests.count > 32 || files.count > 256
        for file in requests.prefix(32) {
            let id = String(file.lastPathComponent.dropLast(".request.json".count))
            guard UUID(uuidString: id) != nil else { continue }
            do {
                let values = try file.resourceValues(forKeys: [.isRegularFileKey, .isSymbolicLinkKey, .fileSizeKey])
                guard values.isRegularFile == true, values.isSymbolicLink != true,
                      let size = values.fileSize, size > 0, size <= 16_384 else { throw RuntimeRefusal.invalidRequest }
                let data = try Data(contentsOf: file)
                // Claim this request before source work; another event cannot dispatch it twice.
                try FileManager.default.removeItem(at: file)
                if overloaded { throw RuntimeRefusal.busy }
                guard let shape = try JSONSerialization.jsonObject(with: data) as? [String: Any],
                      Set(shape.keys) == ["version", "id", "run", "capability", "operation", "window", "windowGeneration"]
                else { throw RuntimeRefusal.invalidRequest }
                let request = try JSONDecoder().decode(RuntimeRequest.self, from: data)
                guard request.version == "1.0.0", request.id == id else { throw RuntimeRefusal.invalidRequest }
                guard request.run == runID, request.capability == capability else { throw RuntimeRefusal.wrongSession }
                guard !isCancelled(id) else { throw RuntimeRefusal.cancelled }
                var response: [String: Any] = ["protocol": "1.0.0", "request_id": id, "run_id": runID]
                switch request.operation {
                case "windows": response["windows"] = collector.windows()
                case "capture":
                    let observation = try collector.capture(windowNumber: request.window, expectedGeneration: request.windowGeneration)
                    guard !isCancelled(id) else { throw RuntimeRefusal.cancelled }
                    guard sequence < UInt64.max else { throw RuntimeRefusal.sourceLimit }
                    sequence += 1
                    let bytes = try RuntimeEncoding.snapshot(observation, session: runID, bundle: bundle, request: id, sequence: sequence)
                    guard !isCancelled(id) else { throw RuntimeRefusal.cancelled }
                    let name = id + ".snapshot.json"
                    try publish(bytes, name: name)
                    response["snapshot"] = name
                    response["sha256"] = SHA256.hash(data: bytes).map { String(format: "%02x", $0) }.joined()
                    response["nodes"] = observation.nodes.count
                    response["window"] = observation.window
                    response["window_generation"] = observation.windowGeneration
                    response["observation_id"] = observation.id
                default: throw RuntimeRefusal.invalidRequest
                }
                response["status"] = "ok"
                try writeJSON(response, name: id + ".response.json")
            } catch {
                let code = (error as? RuntimeRefusal)?.rawValue ?? "requestOrOutputFailure"
                try? writeJSON(["protocol": "1.0.0", "request_id": id, "run_id": runID,
                                "status": "failed", "reason": code], name: id + ".response.json")
            }
        }
    }

    private func isCancelled(_ id: String) -> Bool {
        FileManager.default.fileExists(atPath: directory.appendingPathComponent(id + ".cancel").path)
    }

    private func writeJSON(_ value: [String: Any], name: String) throws {
        try publish(JSONSerialization.data(withJSONObject: value, options: [.sortedKeys]), name: name)
    }

    private func publish(_ data: Data, name: String) throws {
        let temporary = directory.appendingPathComponent(".write-" + UUID().uuidString)
        let destination = directory.appendingPathComponent(name)
        try data.write(to: temporary, options: .withoutOverwriting)
        defer { try? FileManager.default.removeItem(at: temporary) }
        try FileManager.default.setAttributes([.posixPermissions: 0o600], ofItemAtPath: temporary.path)
        guard link(temporary.path, destination.path) == 0 else { throw RuntimeRefusal.outputFailure }
    }
}

private struct RuntimeRequest: Decodable {
    let version: String
    let id: String
    let run: String
    let capability: String
    let operation: String
    let window: Int?
    let windowGeneration: String?
}
#endif
