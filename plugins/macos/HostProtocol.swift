import Foundation
import Darwin

// Private configuration and the existing H01 byte protocol, not a graph schema.
// The admitted Rust worker remains the sole canonical validator/engine.
enum NativeProtocolError: Error { case control, configuration, request, closed, expired, io, limit }

struct NativeControl {
    let kind: UInt8
    let channel: UInt8
    let epoch: UInt64
    let operation: UInt64
    let length: UInt64
    let ticket: UInt64
    let auxiliary: UInt64

    init(_ bytes: Data) throws {
        guard bytes.count == 64, Array(bytes.prefix(8)) == Array("UIBHST01".utf8),
              bytes[9] == 8, bytes[10] <= 2, bytes[11] == 0,
              bytes[12..<16].allSatisfy({ $0 == 0 }), bytes[56..<64].allSatisfy({ $0 == 0 })
        else { throw NativeProtocolError.control }
        func integer(_ offset: Int) -> UInt64 {
            (0..<8).reduce(0) { $0 | UInt64(bytes[offset + $1]) << ($1 * 8) }
        }
        kind = bytes[8]; channel = bytes[10]
        epoch = integer(16); operation = integer(24); length = integer(32)
        ticket = integer(40); auxiliary = integer(48)
        guard epoch > 0, operation > 0, ticket > 0 else { throw NativeProtocolError.control }
    }
}

struct NativeConfiguration: Decodable {
    struct Binding: Decodable {
        let pid: Int32
        let bundle_id: String
        let launch_time: Double
        let window_id: UInt32
        let window_identifier: String
        let target_generation: String
        let surface_generation: String
        var manifest: [String: Any] {
            ["pid": pid, "bundle_id": bundle_id, "launch_time": launch_time,
             "window_id": window_id, "window_identifier": window_identifier,
             "target_generation": target_generation, "surface_generation": surface_generation]
        }
    }
    let binding: Binding
    let scope_id: String
    let collection: String
    let artifact_directory: String?
    let pixel_policy: String?
    let parent_binding: Binding?
    let parent_identity_path: String?
    let identity_path: String
    let acquisition_limits: NativeAcquisitionLimits
    let acquisition_evidence: Bool?
    let probe_manifest_path: String?
    let probe_snapshot_request: Int?
    let probe_source_revision: Int?
    let probe_uptime: Double?

    static func decode(_ bytes: Data) throws -> Self {
        guard let object = try JSONSerialization.jsonObject(with: bytes) as? [String: Any],
              Set(object.keys).isSubset(of: ["binding", "scope_id", "collection", "artifact_directory", "pixel_policy", "parent_binding", "parent_identity_path", "identity_path", "acquisition_limits", "acquisition_evidence", "probe_manifest_path", "probe_snapshot_request", "probe_source_revision", "probe_uptime"]),
              let binding = object["binding"] as? [String: Any],
              Set(binding.keys) == Set(["pid", "bundle_id", "launch_time", "window_id", "window_identifier", "target_generation", "surface_generation"])
        else { throw NativeProtocolError.configuration }
        if let parent = object["parent_binding"] as? [String:Any] {
            guard Set(parent.keys)==Set(["pid","bundle_id","launch_time","window_id","window_identifier","target_generation","surface_generation"])
            else { throw NativeProtocolError.configuration }
        }
        let config = try JSONDecoder().decode(Self.self, from: bytes)
        try config.acquisition_limits.validate()
        guard config.identity_path.hasPrefix("/"), !config.identity_path.utf8.contains(0),
              !config.identity_path.split(separator: "/").contains("..") else { throw NativeProtocolError.configuration }
        if config.acquisition_evidence == true && config.artifact_directory == nil { throw NativeProtocolError.configuration }
        guard config.binding.pid > 0, config.binding.window_id > 0,
              config.binding.launch_time.isFinite && config.binding.launch_time > 0,
              ["local.uiblueprint.f02.on", "local.uiblueprint.f02.off"].contains(config.binding.bundle_id),
              ["a", "b", "popup-a", "popup-b"].contains(config.binding.window_identifier),
              !config.binding.target_generation.isEmpty, !config.binding.surface_generation.isEmpty,
              !config.scope_id.isEmpty, ["sample", "window-ax", "popup-ax"].contains(config.collection)
        else { throw NativeProtocolError.configuration }
        if config.collection == "popup-ax" {
            guard let parent = config.parent_binding, let path = config.parent_identity_path,
                  ["a","b"].contains(parent.window_identifier),
                  config.binding.window_identifier == "popup-\(parent.window_identifier)",
                  parent.pid == config.binding.pid, parent.bundle_id == config.binding.bundle_id,
                  parent.launch_time == config.binding.launch_time, parent.target_generation == config.binding.target_generation,
                  parent.window_id != config.binding.window_id, path.hasPrefix("/"), !path.utf8.contains(0),
                  !path.split(separator:"/").contains("..") else { throw NativeProtocolError.configuration }
        } else if config.parent_binding != nil || config.parent_identity_path != nil || config.binding.window_identifier.hasPrefix("popup-") {
            throw NativeProtocolError.configuration
        }
        if let directory = config.artifact_directory {
            guard directory.hasPrefix("/"), !directory.utf8.contains(0),
                  !directory.split(separator: "/").contains(".."),
                  config.pixel_policy == nil || config.pixel_policy == "owned_synthetic_fixture"
            else { throw NativeProtocolError.configuration }
        } else if config.pixel_policy != nil { throw NativeProtocolError.configuration }
        if let path = config.probe_manifest_path {
            guard path.hasPrefix("/"), !path.utf8.contains(0), !path.split(separator: "/").contains(".."),
                  let request = config.probe_snapshot_request, request > 0,
                  let revision = config.probe_source_revision, revision >= 0,
                  let uptime = config.probe_uptime, uptime.isFinite, uptime > 0
            else { throw NativeProtocolError.configuration }
        } else if config.probe_snapshot_request != nil || config.probe_source_revision != nil || config.probe_uptime != nil {
            throw NativeProtocolError.configuration
        }
        return config
    }
}

struct NativeInbound {
    let configurationBytes: Data
    let document: [String: Any]
    let control: NativeControl
    let replyCap: Int
    let deadline: Double
    let started: Double
}

struct NativeCommand {
    let configuration: NativeConfiguration
    let document: [String: Any]
    let control: NativeControl
    let replyCap: Int
    let deadline: Double
    var channel: String { ["external_semantics", "rendered_capture", "opt_in_layout_probe"][Int(control.channel)] }
}

// Short transfers and EINTR never restart the local duration. Parent remains
// authoritative and reaps the directly registered helper on cancellation/expiry.
struct NativeDescriptorIO {
    let input: Int32
    let output: Int32
    init(input: Int32 = 3, output: Int32 = 4) throws {
        self.input = input; self.output = output
        for fd in [input, output] {
            let flags = fcntl(fd, F_GETFL)
            guard flags >= 0, fcntl(fd, F_SETFL, flags | O_NONBLOCK) == 0 else { throw NativeProtocolError.io }
        }
        // Only this owned socket changes; do not replace process SIGPIPE policy.
        var enabled: Int32 = 1
        guard setsockopt(output, SOL_SOCKET, SO_NOSIGPIPE, &enabled, socklen_t(MemoryLayout<Int32>.size)) == 0
        else { throw NativeProtocolError.io }
    }
    static var now: Double { ProcessInfo.processInfo.systemUptime }
    static func check(_ deadline: Double) throws {
        guard deadline.isFinite, now < deadline else { throw NativeProtocolError.expired }
    }
    private func ready(_ fd: Int32, _ events: Int16, _ deadline: Double) throws {
        while true {
            try Self.check(deadline)
            let wait = Int32(min(1000, max(1, ceil((deadline - Self.now) * 1000))))
            var interest = pollfd(fd: fd, events: events, revents: 0)
            let result = poll(&interest, 1, wait)
            if result > 0 { return } // read/write resolves EOF/error, including HUP
            if result < 0 && errno != EINTR { throw NativeProtocolError.io }
        }
    }
    func read(_ count: Int, deadline: Double) throws -> Data {
        var result = Data(count: count)
        var used = 0
        while used < count {
            try ready(input, Int16(POLLIN), deadline)
            let n = result.withUnsafeMutableBytes { storage in
                Darwin.read(input, storage.baseAddress!.advanced(by: used), count - used)
            }
            if n == 0 { throw NativeProtocolError.closed }
            if n < 0 {
                if errno == EINTR || errno == EAGAIN { continue }
                throw NativeProtocolError.io
            }
            used += n
        }
        return result
    }
    func reply(_ frame: NativeJSONFrame, cap: Int, deadline: Double) throws {
        guard frame.count > 1, frame.count <= cap, !frame.failed else { throw NativeProtocolError.limit }
        try frame.write(to: output, deadline: deadline)
    }

}

@MainActor enum NativeHostProtocol {
    static let configCap = 4032 // NATIVE_CONFIG_BYTES = 4096 - CONTROL_BYTES
    static let inputCap = 2 * 1_048_576
    static let replyCap = 512 * 1024

    static func receiveInput(_ io: NativeDescriptorIO) throws -> NativeInbound {
        // Same eight-second bootstrap safeguard as the existing one-shot helper;
        // not a request default. Parent cleanup also covers a missing Submit.
        let bootstrap = NativeDescriptorIO.now + 8
        let configure = try NativeControl(io.read(64, deadline: bootstrap))
        guard configure.kind == 1, configure.length > 0, configure.length <= configCap,
              configure.auxiliary > 1, configure.auxiliary <= replyCap else { throw NativeProtocolError.control }
        let configurationBytes = try io.read(Int(configure.length), deadline: bootstrap)
        let submit = try NativeControl(io.read(64, deadline: bootstrap))
        guard submit.kind == 3, submit.channel == configure.channel, submit.epoch == configure.epoch,
              submit.operation == configure.operation, submit.ticket == configure.ticket,
              submit.length > 0, submit.length <= inputCap, submit.auxiliary > 0
        else { throw NativeProtocolError.control }
        let start = NativeDescriptorIO.now
        let deadline = start + Double(submit.auxiliary) / 1000
        let request = try io.read(Int(submit.length), deadline: deadline)
        guard let doc = try JSONSerialization.jsonObject(with: request) as? [String: Any] else { throw NativeProtocolError.request }
        return NativeInbound(configurationBytes: configurationBytes, document: doc, control: submit,
            replyCap: Int(configure.auxiliary), deadline: deadline, started: start)
    }

    static func receive(_ io: NativeDescriptorIO) throws -> NativeCommand {
        try fixtureCommand(receiveInput(io))
    }

    static func fixtureCommand(_ input: NativeInbound) throws -> NativeCommand {
        let configuration = try NativeConfiguration.decode(input.configurationBytes)
        let doc = input.document, submit = input.control, start = input.started, deadline = input.deadline
        guard
              doc["schema_version"] as? String == "0.1.0",
              let artifact = doc["artifact"] as? [String: Any], artifact["kind"] as? String == "request",
              let data = artifact["data"] as? [String: Any],
              let requestID = data["request_id"] as? String, !requestID.isEmpty,
              let clock = data["clock_domain"] as? String, !clock.isEmpty,
              let context = data["context"] as? [String: Any], context["scope_id"] as? String == configuration.scope_id,
              let session = context["session_id"] as? String, !session.isEmpty,
              let target = context["target"] as? [String: Any],
              target["id"] as? String == "f02-pid-\(configuration.binding.pid)",
              target["generation"] as? String == configuration.binding.target_generation,
              let surfaces = context["surfaces"] as? [[String: Any]], surfaces.count == (configuration.collection == "popup-ax" ? 2 : 1),
              surfaces[0]["id"] as? String == "window-\(configuration.binding.window_id)",
              surfaces[0]["generation"] as? String == configuration.binding.surface_generation,
              let operation = data["operation"] as? [String: Any], operation["operation"] as? String == "observe",
              let channels = operation["channels"] as? [String], !channels.isEmpty,
              Set(channels).count == channels.count,
              Set(channels).isSubset(of: ["external_semantics", "rendered_capture", "opt_in_layout_probe"]),
              channels.contains(["external_semantics", "rendered_capture", "opt_in_layout_probe"][Int(submit.channel)]),
              let limits = data["limits"] as? [String: Any],
              let duration = limits["deadline_ms"] as? Double, duration.isFinite && duration > 0,
              let output = limits["max_output_bytes"] as? Int, output > 0
        else { throw NativeProtocolError.request }
        let composite = configuration.collection == "sample" && context["projection"] as? String == "design"
            && Set(channels) == Set(["external_semantics", "opt_in_layout_probe"])
            && context["fields"] as? [String] == ["role", "accessibility_name", "enabled", "accessibility_bounds", "layout_bounds"]
        guard let fields = context["fields"] as? [String], !fields.isEmpty, Set(fields).count == fields.count,
              composite || (submit.channel == 2 ? fields == ["layout_bounds"] : configuration.collection != "sample"
                ? Set(fields).isSubset(of: ["role", "accessibility_name", "description", "value", "placeholder", "enabled", "focused", "actions", "accessibility_bounds"])
                : fields == ["role", "accessibility_name", "enabled", "accessibility_bounds"]),
              let nodes = limits["max_elements"] as? Int, (1...160).contains(nodes),
              let depth = limits["max_depth"] as? Int, (1...9).contains(depth)
        else { throw NativeProtocolError.request }
        if submit.channel == 2 {
            guard configuration.probe_manifest_path != nil else { throw NativeProtocolError.configuration }
        }
        if submit.channel == 1 {
            guard configuration.artifact_directory != nil, configuration.pixel_policy == "owned_synthetic_fixture"
            else { throw NativeProtocolError.configuration }
        }
        let boundedDeadline = min(deadline, start + duration / 1000)
        try NativeDescriptorIO.check(boundedDeadline)
        return NativeCommand(configuration: configuration, document: doc, control: submit,
            replyCap: min(input.replyCap, output), deadline: boundedDeadline)
    }
}
