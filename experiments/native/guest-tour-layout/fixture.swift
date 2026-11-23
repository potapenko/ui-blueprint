import Foundation
import CoreGraphics

enum Card: CaseIterable { case language, actor, context }
enum Group { case toolbar, query }
enum Identity { case card(Card), group(Group) }
struct Attachment { let id: Identity }

@inline(never) func mapLayout(origin: CGPoint, viewport: CGRect, scale: CGFloat) -> CGFloat {
    return origin.x + viewport.width + scale
}
@inline(never) func connectionLayout(origin: CGPoint) -> CGFloat {
    return origin.x + origin.y
}
@inline(never) func cardGroup(card: Card, attachment: Attachment) -> Bool {
    return card == .context
}
@inline(never) func cardLayout(card: Card, target: CGRect, rect: CGRect, start: CGPoint) -> CGFloat {
    return target.width + rect.width + start.x
}

let files = FileManager.default
let directory = URL(fileURLWithPath: files.currentDirectoryPath)
print("ready \(ProcessInfo.processInfo.processIdentifier)")
fflush(stdout)
let deadline = Date().addingTimeInterval(45)
var observed = ""
var checksum: CGFloat = 0
while Date() < deadline {
    if files.fileExists(atPath: directory.appendingPathComponent("finish").path) {
        print("recovered")
        exit(0)
    }
    if let phase = try? String(contentsOf: directory.appendingPathComponent("trigger"), encoding: .utf8), phase != observed {
        observed = phase
        let expanded = phase == "expanded"
        let origin = CGPoint(x: 5, y: 7)
        checksum += mapLayout(origin: origin, viewport: CGRect(x: 10, y: 20, width: expanded ? 800 : 600, height: 400), scale: 1)
        checksum += connectionLayout(origin: origin)
        let cards: [Card] = expanded ? [.language, .context] : [.language, .actor, .context]
        for (index, card) in cards.enumerated() {
            let attachment = Attachment(id: card == .language ? .group(.toolbar) : card == .actor ? .group(.query) : .card(.context))
            _ = cardGroup(card: card, attachment: attachment)
            let target = CGRect(x: 220, y: 40 + index * 80, width: 40, height: 20)
            let rect = CGRect(x: 10, y: 40 + index * 80, width: expanded ? 160 : 100, height: 50)
            let start = CGPoint(x: target.midX, y: target.midY)
            checksum += cardLayout(card: card, target: target, rect: rect, start: start)
        }
        print("phase \(phase) \(checksum)")
        fflush(stdout)
    }
    Thread.sleep(forTimeInterval: 0.02)
}
exit(5)
