import XCTest
@testable import UIBlueprintSwiftUIRuntime
#if DEBUG
import RuntimeUpdateScope
import Darwin
#endif

#if DEBUG
final class RuntimeTests: XCTestCase {
    func testStaticGetterContextIsTheSwiftClassMetatype() throws {
        let metatype = try XCTUnwrap(_typeByName("SwiftUI.UpdateGroup"))
        let image = try XCTUnwrap(dlopen("/System/Library/Frameworks/SwiftUICore.framework/SwiftUICore", RTLD_LAZY))
        defer { dlclose(image) }
        let exported = try XCTUnwrap(dlsym(image, "$s7SwiftUI11UpdateGroupCN"))
        XCTAssertEqual(unsafeBitCast(metatype, to: UnsafeRawPointer.self), UnsafeRawPointer(exported))
    }

    @MainActor func testUpdateScopeRestoresContextOnNestedAndThrowingPaths() throws {
        let os = ProcessInfo.processInfo.operatingSystemVersion
        guard os.majorVersion == 27, os.minorVersion == 0, os.patchVersion == 1 else {
            throw XCTSkip("Runtime source profile is qualified separately for each OS")
        }
        let initial = UIBRuntimeUpdateCurrent()
        let value = try RuntimeUpdateContext.perform {
            XCTAssertEqual(UIBRuntimeUpdateCurrent(), 1)
            return try RuntimeUpdateContext.perform {
                XCTAssertEqual(UIBRuntimeUpdateCurrent(), 1)
                return 123
            }
        }
        XCTAssertEqual(value, 123)
        XCTAssertEqual(UIBRuntimeUpdateCurrent(), initial)
        XCTAssertThrowsError(try RuntimeUpdateContext.perform { throw RuntimeRefusal.cancelled })
        XCTAssertEqual(UIBRuntimeUpdateCurrent(), initial)
    }

    func testInvalidRequestsAreClaimedOnceWithoutFollowingSymlinks() throws {
        let root = FileManager.default.temporaryDirectory.appendingPathComponent("uib-input-" + UUID().uuidString)
        try FileManager.default.createDirectory(at: root, withIntermediateDirectories: false)
        defer { try? FileManager.default.removeItem(at: root) }
        let original = root.appendingPathComponent("original")
        try Data("preserve".utf8).write(to: original)
        let symlink = root.appendingPathComponent("symlink.request.json")
        try FileManager.default.createSymbolicLink(at: symlink, withDestinationURL: original)
        let empty = root.appendingPathComponent("empty.request.json")
        try Data().write(to: empty)
        let oversized = root.appendingPathComponent("large.request.json")
        try Data(repeating: 65, count: 16_385).write(to: oversized)
        for path in [symlink, empty, oversized] {
            XCTAssertThrowsError(try RuntimeInput.claim(path, in: root))
            XCTAssertFalse(FileManager.default.fileExists(atPath: path.path))
        }
        XCTAssertEqual(try Data(contentsOf: original), Data("preserve".utf8))
        XCTAssertEqual(try FileManager.default.contentsOfDirectory(atPath: root.path), ["original"])
    }

    func testValidRequestClaimPreservesExactBytes() throws {
        let root = FileManager.default.temporaryDirectory.appendingPathComponent("uib-input-" + UUID().uuidString)
        try FileManager.default.createDirectory(at: root, withIntermediateDirectories: false)
        defer { try? FileManager.default.removeItem(at: root) }
        let input = root.appendingPathComponent("valid.request.json")
        let bytes = Data("{\"id\":\"literal\"}".utf8)
        try bytes.write(to: input)
        XCTAssertEqual(try RuntimeInput.claim(input, in: root), bytes)
        XCTAssertFalse(FileManager.default.fileExists(atPath: input.path))
    }

    @MainActor func testTypeMetadataRedactsRuntimeAddresses() throws {
        XCTAssertEqual(try RuntimeCollector.sanitizedType("SwiftUI.(unknown context at $123abc).View"),
                       "SwiftUI.(private).View")
        XCTAssertThrowsError(try RuntimeCollector.sanitizedType("bad\nname"))
        XCTAssertEqual(try RuntimeCollector.sanitizedType("SwiftUI.ModifiedContent<App.View, SwiftUI.Padding>"),
                       "SwiftUI.ModifiedContent")
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
        XCTAssertTrue(String(decoding: bytes, as: UTF8.self).contains("source-transform-unavailable"))
    }

    func testTransformOrderAndUnknownEncoding() throws {
        let source: [String: Any] = ["positionAdjustment": [3, 4], "items": [
            ["translation": [3, 4]], ["affineTransform": [2, 0, 0, 0, 0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]],
            ["translation": [10, 20]], [:]]]
        XCTAssertEqual(try XCTUnwrap(RuntimeAffine.decode(source)).values, [2, 0, 0, 3, 10, 20])
        XCTAssertNil(RuntimeAffine.decode(["positionAdjustment": [0, 0], "items": [["projectionTransform": []]]]))
        XCTAssertNil(RuntimeAffine.decode(["positionAdjustment": [true, false], "items": []]))
        XCTAssertNil(RuntimeAffine.decode(["positionAdjustment": [0, 0], "items": [["affineTransform":
            [1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]]]]))
    }
}
#endif
