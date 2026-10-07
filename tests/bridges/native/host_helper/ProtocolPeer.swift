import Foundation
import Darwin

// Offline transport peer; no Collector.collect or live platform calls.
@main struct ProtocolPeer {
    @MainActor static func main() {
        do {
            let io = try NativeDescriptorIO()
            let command = try NativeHostProtocol.receive(io)
            let request = (command.document["artifact"] as! [String: Any])["data"] as! [String: Any]
            let json = NativeJSON(command.configuration.acquisition_limits)
            let value = try json.response(request: request, ticket: command.control.ticket, channel: command.channel) {
                try json.failure("permission_required", scope: command.configuration.scope_id, channel: command.channel)
            }
            let frame = try NativeJSONFrame(capacity: command.replyCap, deadline: command.deadline)
            try frame.encode(value)
            try io.reply(frame, cap: command.replyCap, deadline: command.deadline)
        } catch { _exit(2) }
    }
}
