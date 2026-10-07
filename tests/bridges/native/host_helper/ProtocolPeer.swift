import Foundation
import Darwin

// Offline transport peer. Never invokes Collector.collect or any platform
// permission/AX/capture API. Reuses the actual collector's canonical error encoder.
@main struct ProtocolPeer {
    @MainActor static func main() {
        do {
            let io = try NativeDescriptorIO()
            let command = try NativeHostProtocol.receive(io)
            let request = (command.document["artifact"] as! [String: Any])["data"] as! [String: Any]
            let context = request["context"] as! [String: Any]
            let result = channelFailure(code: "permission_required", scope: command.configuration.scope_id,
                                        channel: command.channel)
            let frame = try canonicalBytes("channel_response", channelResponse(
                requestID: request["request_id"] as! String, sessionID: context["session_id"] as! String,
                target: context["target"] as! [String: Any], sequence: command.control.ticket,
                channel: command.channel, result: result))
            try io.reply(frame, cap: command.replyCap, deadline: command.deadline)
        } catch { _exit(2) }
    }
}
