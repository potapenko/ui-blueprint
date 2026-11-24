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

    func testSubtreePreservesAncestorClosureAndRefusesAmbiguity() throws {
        let nodes = [
            RuntimeNode(index: 0, parent: nil, hostingRoot: 0, type: "Fixture.Root", position: .zero, size: .init(width: 100, height: 80)),
            RuntimeNode(index: 1, parent: 0, hostingRoot: 0, type: "Fixture.Sibling", position: nil, size: nil),
            RuntimeNode(index: 2, parent: 0, hostingRoot: 0, type: "Fixture.Dialog", position: .zero, size: .init(width: 100, height: 80), positionSource: 0, sizeSource: 0),
            RuntimeNode(index: 3, parent: 2, hostingRoot: 0, type: "Fixture.Label", position: .zero, size: .init(width: 100, height: 80), positionSource: 0, sizeSource: 0),
            RuntimeNode(index: 4, parent: nil, hostingRoot: 1, type: "Fixture.OtherHost", position: nil, size: nil)
        ]
        let full = RuntimeObservation(id: "o", start: 1, end: 2, window: 1,
            windowGeneration: "g", nodes: nodes, hostingRoots: 2)
        let selected = try full.selecting(type: "Fixture.Dialog")
        XCTAssertEqual(selected.nodes.map(\.index), [0, 2, 3])
        XCTAssertEqual(selected.nodes[1].positionSource, 0)
        XCTAssertEqual(selected.selection?.ancestors, [0])
        XCTAssertEqual(selected.selection?.subtree, [2, 3])
        XCTAssertEqual(selected.selection?.sourceNodes, 5)
        XCTAssertEqual(selected.scopeID, "window-1/subtree-n2-with-ancestors")
        func encode(_ value: RuntimeObservation) throws -> Data {
            try RuntimeEncoding.snapshot(value, session: "s", bundle: "fixture", request: "r", sequence: 1)
        }
        XCTAssertEqual(try encode(full), try encode(full.selecting(type: nil)))
        let bytes = try encode(selected)
        let doc = try XCTUnwrap(JSONSerialization.jsonObject(with: bytes) as? [String: Any])
        let data = (doc["artifact"] as! [String: Any])["data"] as! [String: Any]
        let output = data["nodes"] as! [[String: Any]]
        XCTAssertEqual((output[0]["children"] as! [[String: String]]).map { $0["key"]! }, ["o-n2"])
        XCTAssertFalse(String(decoding: bytes, as: UTF8.self).contains("Fixture.Sibling"))
        let decl = output[0]["source_declarations"] as! [[String: Any]]
        let count = decl.first { $0["name"] as? String == "excluded_direct_children" }!
        XCTAssertEqual(((count["state"] as! [String: Any])["value"] as! [String: Any])["value"] as? Int, 1)
        XCTAssertThrowsError(try full.selecting(type: "Absent")) { XCTAssertEqual($0 as? RuntimeRefusal, .subtreeNotFound) }
        var ambiguous = full
        ambiguous.nodes.append(RuntimeNode(index: 5, parent: nil, hostingRoot: 1, type: "Fixture.Dialog", position: nil, size: nil))
        XCTAssertThrowsError(try ambiguous.selecting(type: "Fixture.Dialog")) { XCTAssertEqual($0 as? RuntimeRefusal, .ambiguousSubtree) }
        for invalid in ["", "bad\nname", String(repeating: "é", count: 513)] {
            XCTAssertThrowsError(try full.selecting(type: invalid)) { XCTAssertEqual($0 as? RuntimeRefusal, .invalidRequest) }
        }
    }

    func testLargeWindowOutputLimitStillAllowsCompleteSmallSubtree() throws {
        var nodes = [RuntimeNode(index: 0, parent: nil, hostingRoot: 0, type: "Fixture.Root", position: .zero,
            size: .init(width: 500, height: 400), positionSource: 0, sizeSource: 0)]
        for index in 1..<10_000 {
            nodes.append(RuntimeNode(index: index, parent: 0, hostingRoot: 0,
                type: index == 1 ? "Fixture.Dialog" : "Fixture.Row", position: .zero,
                size: .init(width: 80, height: 20), positionSource: 0, sizeSource: 0,
                transform: RuntimeAffine(values: [1, 0, 0, 1, 0, 0])))
        }
        let full = RuntimeObservation(id: "fixture-observation", start: 1, end: 2, window: 1,
            windowGeneration: "g", nodes: nodes, hostingRoots: 1)
        func encode(_ value: RuntimeObservation) throws -> Data {
            try RuntimeEncoding.snapshot(value, session: "s", bundle: "fixture", request: "r", sequence: 1)
        }
        XCTAssertThrowsError(try encode(full)) { XCTAssertEqual($0 as? RuntimeRefusal, .outputLimit) }
        let inventory = try RuntimeEncoding.discovery(full, session: "s", bundle: "fixture", request: "r")
        XCTAssertLessThan(inventory.count, 1_000_000)
        let selected = try full.selecting(type: "Fixture.Dialog")
        XCTAssertEqual(selected.nodes.count, 2)
        XCTAssertLessThan(try encode(selected).count, 16_384)
    }

    func testDiscoveryPreservesTypeCountsTopologyAndUnknownTypesWithoutGeometry() throws {
        let nodes = [
            RuntimeNode(index: 0, parent: nil, hostingRoot: 0, type: "Fixture.Root", position: .zero, size: .init(width: 400, height: 300)),
            RuntimeNode(index: 1, parent: 0, hostingRoot: 0, type: "SwiftUI.Text", position: nil, size: nil),
            RuntimeNode(index: 2, parent: 0, hostingRoot: 0, type: "SwiftUI.Text", position: nil, size: nil),
            RuntimeNode(index: 3, parent: nil, hostingRoot: 1, type: nil, position: nil, size: nil)
        ]
        let observation = RuntimeObservation(id: "o", start: 1, end: 2, window: 7,
            windowGeneration: "g", nodes: nodes, hostingRoots: 2)
        let bytes = try RuntimeEncoding.discovery(observation, session: "s", bundle: "fixture", request: "r")
        let doc = try XCTUnwrap(JSONSerialization.jsonObject(with: bytes) as? [String: Any])
        XCTAssertEqual(doc["kind"] as? String, "swiftui_runtime_discovery")
        XCTAssertEqual(doc["discovery_version"] as? String, "1.0.0")
        XCTAssertNil(doc["artifact"])
        XCTAssertEqual(doc["observation_id"] as? String, "o")
        let types = doc["types"] as! [[String: Any]]
        XCTAssertEqual(types.map { $0["name"] as! String }, ["Fixture.Root", "SwiftUI.Text"])
        XCTAssertEqual(types.map { $0["count"] as! Int }, [1, 2])
        let output = doc["nodes"] as! [[String: Any]]
        XCTAssertEqual(output.map { $0["index"] as! Int }, [0, 1, 2, 3])
        XCTAssertEqual(output[2]["parent"] as? Int, 0)
        XCTAssertEqual(output[2]["type_index"] as? Int, 1)
        XCTAssertTrue(output[3]["type_index"] is NSNull)
        XCTAssertEqual(Set(output[0].keys), ["index", "parent", "type_index", "hosting_root"])
        let coverage = doc["coverage"] as! [String: Any]
        XCTAssertEqual(coverage["nodes_without_type"] as? Int, 1)
        XCTAssertEqual(coverage["status"] as? String, "partial")
    }

    func testDiscoveryKeepsTheOutputByteLimit() throws {
        let nodes = (0..<300).map { index in
            RuntimeNode(index: index, parent: nil, hostingRoot: 0,
                type: "Fixture." + String(repeating: "x", count: 65_000) + String(index), position: nil, size: nil)
        }
        let observation = RuntimeObservation(id: "o", start: 1, end: 2, window: 7, windowGeneration: "g", nodes: nodes)
        XCTAssertThrowsError(try RuntimeEncoding.discovery(observation, session: "s", bundle: "fixture", request: "r")) {
            XCTAssertEqual($0 as? RuntimeRefusal, .outputLimit)
        }
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
