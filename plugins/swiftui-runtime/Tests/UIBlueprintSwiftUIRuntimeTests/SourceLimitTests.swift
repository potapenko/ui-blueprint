import XCTest
#if DEBUG
import AppKit
import SwiftUI
@testable import UIBlueprintSwiftUIRuntime

final class SourceLimitTests: XCTestCase {
    @MainActor func testTypeByteBoundaryAndControlPresenceKeepFixedEvidence() throws {
        let nominal = "Fixture." + String(repeating: "a", count: 65_528)
        XCTAssertEqual(try RuntimeCollector.sanitizedType(nominal), nominal)
        for (name, kind, observed) in [(nominal + "a", RuntimeSourceLimit.Kind.typeNameBytes, UInt64(65_537)),
                                        ("canary\nprivate", .typeNameControls, 1)] {
            XCTAssertThrowsError(try RuntimeCollector.sanitizedType(name)) { error in
                let limit = error as? RuntimeSourceLimit
                XCTAssertEqual(limit?.kind, kind)
                XCTAssertEqual(limit?.observed, observed)
                let bytes = try! JSONSerialization.data(withJSONObject: limit!.metadata)
                XCTAssertFalse(String(decoding: bytes, as: UTF8.self).contains("private"))
                XCTAssertFalse(String(decoding: bytes, as: UTF8.self).contains("canary"))
            }
        }
    }

    @MainActor func testActualSourceAtNodeLimitAndNextNodeRefusal() throws {
        guard ProcessInfo.processInfo.environment["SWIFTUI_VIEW_DEBUG"] == "29" else {
            throw XCTSkip("Run this source boundary fixture with SWIFTUI_VIEW_DEBUG=29")
        }
        _ = NSApplication.shared
        let host = NSHostingView(rootView: Color.blue.frame(width: 120, height: 30))
        host.frame = NSRect(x: 0, y: 0, width: 240, height: 100)
        host.layoutSubtreeIfNeeded()
        let roots = try host.runtimeGeometryTree()
        var sample: [RuntimeNode] = []
        try RuntimeCollector.collect(roots, root: 0, nodes: &sample)
        guard !sample.isEmpty else { return XCTFail("Owned source fixture must expose its tree") }
        func prefix(_ count: Int) -> [RuntimeNode] {
            (0..<count).map { RuntimeNode(index: $0, parent: nil, hostingRoot: 0,
                type: "Fixture.Context", position: nil, size: nil) }
        }
        var exact = prefix(RuntimeCollector.maxNodes - sample.count)
        try RuntimeCollector.collect(roots, root: 1, nodes: &exact)
        XCTAssertEqual(exact.count, RuntimeCollector.maxNodes)
        var over = prefix(RuntimeCollector.maxNodes - sample.count + 1)
        XCTAssertThrowsError(try RuntimeCollector.collect(roots, root: 1, nodes: &over)) { error in
            let limit = error as? RuntimeSourceLimit
            XCTAssertEqual(limit?.kind, .sourceNodes)
            XCTAssertEqual(limit?.kind.limit, 16_384)
            XCTAssertEqual(limit?.observed, 16_385)
        }
    }
}
#endif
