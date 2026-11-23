import XCTest
@testable import UIBlueprintSwiftUIRuntime

#if DEBUG
final class RuntimeTests: XCTestCase {
    @MainActor func testTypeMetadataRedactsRuntimeAddresses() throws {
        XCTAssertEqual(try RuntimeCollector.sanitizedType("SwiftUI.(unknown context at $123abc).View"),
                       "SwiftUI.(private).View")
        XCTAssertThrowsError(try RuntimeCollector.sanitizedType("bad\nname"))
    }

    @MainActor func testEncodingDoesNotInventCommonCoordinates() throws {
        let observed = RuntimeObservation(id: "observation-1", start: 2, end: 3, window: 7,
            windowGeneration: "generation-1", nodes: [
                RuntimeNode(index: 0, parent: nil, hostingRoot: 0, type: "Fixture.Root", position: .zero, size: .init(width: 100, height: 80)),
                RuntimeNode(index: 1, parent: 0, hostingRoot: 0, type: "SwiftUI.Text", position: nil, size: nil)
            ])
        let bytes = try RuntimeEncoding.snapshot(observed, session: "session-1", bundle: "me.uiblueprint.fixture",
                                                 request: "request-1", sequence: 1)
        let doc = try XCTUnwrap(JSONSerialization.jsonObject(with: bytes) as? [String: Any])
        let artifact = try XCTUnwrap(doc["artifact"] as? [String: Any])
        let snapshot = try XCTUnwrap(artifact["data"] as? [String: Any])
        let nodes = try XCTUnwrap(snapshot["nodes"] as? [[String: Any]])
        let props = try XCTUnwrap(nodes[1]["properties"] as? [[String: Any]])
        XCTAssertEqual((props[0]["state"] as? [String: Any])?["availability"] as? String, "unknown")
        XCTAssertEqual((nodes[0]["children"] as? [[String: Any]])?.count, 1)
        XCTAssertTrue(String(decoding: bytes, as: UTF8.self).contains("runtime-transform-not-yet-qualified"))
    }
}
#endif
