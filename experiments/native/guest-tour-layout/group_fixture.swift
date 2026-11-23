import Foundation
import CoreGraphics

enum Card { case language, context }
struct Annotation {
    enum GroupID: String { case toolbar, tabs, modifiers, query }
    enum Section { case attached, header }
    enum Side { case top, bottom, leading, trailing }
    enum Alignment { case start, center, end }
    struct Shift {
        enum Distance { case points(CGFloat), fractionOfTarget(CGFloat) }
        var x: Distance = .points(0)
        var y: Distance = .points(0)
    }
    struct Group {
        let id: GroupID
        var section: Section = .attached
        var side: Side = .top
        var alignment: Alignment = .center
        var gap: CGFloat = 24
        var shift = Shift()
        var spacing: CGFloat = 20
    }
    let group: Group?
}
struct Modifier { let card: Card }
@inline(never) func groupRegistration(_ source: Modifier, description: Annotation) -> Int {
    return source.card == .context ? 1 : 2
}

let files = FileManager.default
let directory = URL(fileURLWithPath: files.currentDirectoryPath)
print("ready \(ProcessInfo.processInfo.processIdentifier)")
fflush(stdout)
let deadline = Date().addingTimeInterval(45)
var fired = false
while Date() < deadline {
    if files.fileExists(atPath: directory.appendingPathComponent("finish").path) {
        print("recovered")
        exit(0)
    }
    if !fired && files.fileExists(atPath: directory.appendingPathComponent("trigger").path) {
        fired = true
        for group in [Annotation.GroupID.toolbar, .tabs, .modifiers, .query] {
            _ = groupRegistration(Modifier(card: .language), description: Annotation(group: .init(id: group)))
        }
        _ = groupRegistration(Modifier(card: .context), description: Annotation(group: nil))
        print("phase complete")
        fflush(stdout)
    }
    Thread.sleep(forTimeInterval: 0.02)
}
exit(5)
