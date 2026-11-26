#if DEBUG
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
            "capabilities": ["capture-subtree-type", "discover-types"],
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
                // Claim invalid requests too: publishing their error must not
                // retrigger the same input forever through directory events.
                let data = try RuntimeInput.claim(file, in: directory)
                if overloaded { throw RuntimeRefusal.busy }
                guard let shape = try JSONSerialization.jsonObject(with: data) as? [String: Any],
                      (Set(shape.keys) == ["version", "id", "run", "capability", "operation", "window", "windowGeneration"]
                       || Set(shape.keys) == ["version", "id", "run", "capability", "operation", "window", "windowGeneration", "subtreeType"])
                else { throw RuntimeRefusal.invalidRequest }
                let request = try JSONDecoder().decode(RuntimeRequest.self, from: data)
                if let value = shape["subtreeType"] {
                    guard request.operation == "capture", let type = value as? String else {
                        throw RuntimeRefusal.invalidRequest
                    }
                    try RuntimeSubtreeSelection.validate(type)
                }
                guard request.version == "1.0.0", request.id == id else { throw RuntimeRefusal.invalidRequest }
                guard request.window == nil || (request.window! > 0 && request.window! <= Int(Int32.max)) else {
                    throw RuntimeRefusal.invalidRequest
                }
                guard request.run == runID, request.capability == capability else { throw RuntimeRefusal.wrongSession }
                guard !isCancelled(id) else { throw RuntimeRefusal.cancelled }
                var response: [String: Any] = ["protocol": "1.0.0", "request_id": id, "run_id": runID]
                switch request.operation {
                case "windows": response["windows"] = collector.windows()
                case "capture", "discover":
                    let observation = try collector.capture(windowNumber: request.window, expectedGeneration: request.windowGeneration)
                        .selecting(type: request.subtreeType)
                    guard !isCancelled(id) else { throw RuntimeRefusal.cancelled }
                    let bytes: Data
                    let artifact: String
                    if request.operation == "discover" {
                        artifact = "discovery"
                        bytes = try RuntimeEncoding.discovery(observation, session: runID, bundle: bundle, request: id)
                    } else {
                        artifact = "snapshot"
                        guard sequence < UInt64.max else {
                            throw RuntimeSourceLimit(kind: .snapshotSequence, observed: sequence)
                        }
                        sequence += 1
                        bytes = try RuntimeEncoding.snapshot(observation, session: runID, bundle: bundle, request: id, sequence: sequence)
                    }
                    guard !isCancelled(id) else { throw RuntimeRefusal.cancelled }
                    let name = id + "." + artifact + ".json"
                    try publish(bytes, name: name)
                    response[artifact] = name
                    response["sha256"] = SHA256.hash(data: bytes).map { String(format: "%02x", $0) }.joined()
                    response["nodes"] = observation.nodes.count
                    if let selection = observation.selection {
                        response["selection"] = selection.metadata(observation: observation.id)
                    }
                    response["window"] = observation.window
                    response["window_generation"] = observation.windowGeneration
                    response["observation_id"] = observation.id
                default: throw RuntimeRefusal.invalidRequest
                }
                response["status"] = "ok"
                try writeJSON(response, name: id + ".response.json")
            } catch {
                let limit = error as? RuntimeSourceLimit
                let code = limit != nil ? "sourceLimit" : (error as? RuntimeRefusal)?.rawValue ?? "requestOrOutputFailure"
                var response: [String: Any] = ["protocol": "1.0.0", "request_id": id, "run_id": runID,
                                               "status": "failed", "reason": code]
                if let limit { response["source_limit"] = limit.metadata }
                try? writeJSON(response, name: id + ".response.json")
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
    let subtreeType: String?
}
#endif
