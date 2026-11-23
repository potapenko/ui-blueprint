import Foundation; import CoreGraphics
enum PublicCard: Int { case first = 7, sample = 19 }
@inline(never) func check(_ seed: Int) -> CGFloat {
    let card: PublicCard = seed == 1 ? .sample : .first
    let target = CGRect(x: 11, y: 22, width: 33, height: 44)
    let rect = CGRect(x: 55, y: 66, width: 77, height: 88)
    let start = CGPoint(x: 99, y: 111)
    return CGFloat(card.rawValue) + target.minX + rect.width + start.y
}
print("ready", getpid(), getuid(), geteuid())
fflush(stdout)
for _ in 0..<300 {
    Thread.sleep(forTimeInterval: 0.03)
    precondition(check(1) == 218)
    if FileManager.default.fileExists(atPath: "finish") { print("recovered"); exit(0) }
}
exit(5)
