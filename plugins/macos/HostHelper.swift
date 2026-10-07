import Foundation
import AppKit
import Darwin

// Nonvisual public-system adapter. No app activation or permission prompt.
@main struct NativeHostHelper {
    @MainActor static func main() async {
        do {
            let io = try NativeDescriptorIO()
            let command = try NativeHostProtocol.receive(io)
            let remaining = command.deadline - NativeDescriptorIO.now
            guard remaining > 0 else { throw NativeProtocolError.expired }
            // A blocked SDK call cannot extend local life; parent still owns reap.
            DispatchQueue.global().asyncAfter(deadline: .now() + remaining) { _exit(124) }
            if command.control.channel == 2 {
                let binding = command.configuration.binding
                guard let app = NSRunningApplication(processIdentifier: binding.pid), app.bundleIdentifier == binding.bundle_id,
                      app.launchDate?.timeIntervalSince1970 == binding.launch_time,
                      Collector.windowOwnedBy(pid: binding.pid, window: binding.window_id),
                      let path = command.configuration.probe_manifest_path else { throw NativeProtocolError.configuration }
                let fd = open(path, O_RDONLY | O_CLOEXEC | O_NOFOLLOW)
                guard fd >= 0 else { throw NativeProtocolError.configuration }
                let file = FileHandle(fileDescriptor: fd, closeOnDealloc: true)
                defer { try? file.close() }
                let data = try file.read(upToCount: command.replyCap + 1) ?? Data()
                guard data.count <= command.replyCap else { throw NativeProtocolError.limit }
                let frame = try Collector.probe(data: data, command: command)
                try io.reply(frame, cap: command.replyCap, deadline: command.deadline)
                return
            }
            try await Collector.collect(document: command.document,
                manifest: command.configuration.binding.manifest,
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
