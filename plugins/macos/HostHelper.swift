import Foundation
import AppKit
import ApplicationServices
import Darwin

// Nonvisual public-system adapter. No app activation or permission prompt.
@main struct NativeHostHelper {
    @MainActor static func main() async {
        do {
            if CommandLine.arguments.count > 1 {
                guard CommandLine.arguments.count == 3, CommandLine.arguments[1] == "describe-process",
                      let pid = Int32(CommandLine.arguments[2]), pid > 0 else { _exit(2) }
                guard let app = NSRunningApplication(processIdentifier: pid), !app.isTerminated,
                      let bundle = app.bundleIdentifier, bundle.utf8.count <= 256,
                      let launch = app.launchDate?.timeIntervalSince1970 else {
                    FileHandle.standardOutput.write(Data("{\"status\":\"target_unresolved\"}\n".utf8)); _exit(4)
                }
                let process = NativeFocusedAX.Process(pid: pid, bundle_id: bundle, launch_time: launch)
                let value: [String: Any] = ["metadata_version": "1.0.0", "status": "known",
                    "process": ["pid": pid, "bundle_id": bundle, "launch_time": launch],
                    "target": ["id": "macos-pid-\(pid)", "generation": process.generation]]
                let bytes = try JSONSerialization.data(withJSONObject: value, options: [.sortedKeys])
                guard bytes.count < NativeHostProtocol.configCap else { _exit(2) }
                FileHandle.standardOutput.write(bytes); FileHandle.standardOutput.write(Data([10]))
                return
            }
            let io = try NativeDescriptorIO()
            let input = try NativeHostProtocol.receiveInput(io)
            if input.control.flags >= 128 {
                try NativeFormSession.run(io: io, first: input)
                return
            }
            if try NativeFocusedAX.isConfiguration(input.configurationBytes) {
                let command = try NativeFocusedAX.command(input)
                let remaining = command.deadline - NativeDescriptorIO.now
                guard remaining > 0 else { throw NativeProtocolError.expired }
                DispatchQueue.global().asyncAfter(deadline: .now() + remaining) { _exit(124) }
                let frame = try NativeFocusedAX.collect(command)
                try io.reply(frame, cap: command.replyCap, deadline: command.deadline)
                return
            }
            let command = try NativeHostProtocol.fixtureCommand(input)
            let remaining = command.deadline - NativeDescriptorIO.now
            guard remaining > 0 else { throw NativeProtocolError.expired }
            // A blocked SDK call cannot extend local life; parent still owns reap.
            DispatchQueue.global().asyncAfter(deadline: .now() + remaining) { _exit(124) }
            var bindingManifest = command.configuration.binding.manifest
            bindingManifest["identity_path"] = command.configuration.identity_path
            if command.configuration.collection == "popup-ax" {
                let frame = try await Collector.popup(command:command)
                try io.reply(frame,cap:command.replyCap,deadline:command.deadline)
                return
            }
            if command.control.channel == 2 {
                func identityFailure() throws -> NativeJSONFrame {
                    let request = (command.document["artifact"] as! [String: Any])["data"] as! [String: Any]
                    let json = NativeJSON(command.configuration.acquisition_limits)
                    let frame = try NativeJSONFrame(capacity: command.replyCap, deadline: command.deadline)
                    try frame.encode(json.response(request: request, ticket: command.control.ticket, channel: command.channel) {
                        try json.failure("stale_target", scope: command.configuration.scope_id, channel: command.channel)
                    })
                    return frame
                }
                do { try NativeCurrentIdentity.verify(path: command.configuration.identity_path, expected: bindingManifest) }
                catch { try io.reply(identityFailure(), cap: command.replyCap, deadline: command.deadline); return }
                let binding = command.configuration.binding
                guard let app = NSRunningApplication(processIdentifier: binding.pid), app.bundleIdentifier == binding.bundle_id,
                      app.launchDate?.timeIntervalSince1970 == binding.launch_time,
                      Collector.windowOwnedBy(pid: binding.pid, window: binding.window_id),
                      let path = command.configuration.probe_manifest_path else {
                    try io.reply(identityFailure(), cap: command.replyCap, deadline: command.deadline); return
                }
                let fd = open(path, O_RDONLY | O_CLOEXEC | O_NOFOLLOW)
                guard fd >= 0 else { throw NativeProtocolError.configuration }
                let file = FileHandle(fileDescriptor: fd, closeOnDealloc: true)
                defer { try? file.close() }
                let data = try file.read(upToCount: command.replyCap + 1) ?? Data()
                guard data.count <= command.replyCap else { throw NativeProtocolError.limit }
                let frame = try Collector.probe(data: data, command: command) { admission, json in
                    guard AXIsProcessTrusted() else { throw Collector.ProbeLinkError.permissionRequired }
                    let request = (command.document["artifact"] as! [String: Any])["data"] as! [String: Any]
                    let context = request["context"] as! [String: Any]
                    let surfaces = context["surfaces"] as! [[String: Any]]
                    let limits = request["limits"] as! [String: Any]
                    let requestID = request["request_id"] as! String
                    let start = ProcessInfo.processInfo.systemUptime
                    let root = AXUIElementCreateApplication(binding.pid)
                    let window = unsafeDowncast(try Collector.resolveWindow(root, identifier: binding.window_identifier, admission: admission), to: AXUIElement.self)
                    let result = try Collector.sample(window, identifier: binding.window_identifier, surface: surfaces[0],
                        observation: "\(requestID)-external_semantics-linked", maxNodes: (limits["max_elements"] as! Int) - 3,
                        maxDepth: limits["max_depth"] as! Int, deadline: command.deadline, admission: admission,
                        json: json, includeLayout: true)
                    guard CFEqual(window, try Collector.resolveWindow(root, identifier: binding.window_identifier, admission: admission)) else { throw NativeProtocolError.request }
                    return (result, start, ProcessInfo.processInfo.systemUptime)
                }
                do { try NativeCurrentIdentity.verify(path: command.configuration.identity_path, expected: bindingManifest) }
                catch { try io.reply(identityFailure(), cap: command.replyCap, deadline: command.deadline); return }
                guard NSRunningApplication(processIdentifier: binding.pid)?.launchDate?.timeIntervalSince1970 == binding.launch_time,
                      Collector.windowOwnedBy(pid: binding.pid, window: binding.window_id) else {
                    try io.reply(identityFailure(), cap: command.replyCap, deadline: command.deadline); return
                }
                try io.reply(frame, cap: command.replyCap, deadline: command.deadline)
                return
            }
            try await Collector.collect(document: command.document,
                manifest: bindingManifest,
                directory: command.configuration.artifact_directory.map { URL(fileURLWithPath: $0) },
                mode: command.configuration.collection == "window-ax" ? "window-ax" : "live",
                sequence: command.control.ticket, limits: command.configuration.acquisition_limits,
                evidence: command.configuration.acquisition_evidence == true, wireCap: command.replyCap, selectedChannel: command.channel,
                deadline: command.deadline) { bytes in
                    try io.reply(bytes, cap: command.replyCap, deadline: command.deadline)
                }
        } catch {
            // No raw error/UI data on stderr or private status. Missing FD5 status
            // is a generic helper failure; it must not mimic a Rust quota record.
            _exit(2)
        }
    }
}
