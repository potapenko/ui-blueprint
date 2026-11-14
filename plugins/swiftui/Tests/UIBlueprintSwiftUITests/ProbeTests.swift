#if DEBUG
import Foundation
import Combine
import Testing
@testable import UIBlueprintSwiftUI

private func configuration(selection: BlueprintProbeSelection = .requiredMarkers) throws -> ProbeConfiguration {
    try ProbeConfiguration(targetID: "example.app", targetGeneration: "run-1", surfaceID: "main",
        scopeID: "sample", rootMarkerID: "root", requiredMarkerIDs: ["icon", "text"], selection: selection)
}

private func markers(gap: Double = 8, textMount: String = "text-mount") -> [ProbeMarker] {
    [ProbeMarker(id: "root", mountID: "root-mount", parent: nil,
        bounds: CGRect(x: 0, y: 0, width: 240, height: 80)),
     ProbeMarker(id: "icon", mountID: "icon-mount", parent: "root",
        bounds: CGRect(x: 12, y: 12, width: 24, height: 24)),
     ProbeMarker(id: "text", mountID: textMount, parent: "root",
        bounds: CGRect(x: 36 + gap, y: 12, width: 100, height: 24))]
}

@Test func configurationRejectsAmbiguousOrUnboundedIDs() throws {
    #expect(throws: BlueprintProbeError.invalidConfiguration) {
        try ProbeConfiguration(targetID: "example.app", targetGeneration: "run", surfaceID: "main",
            scopeID: "scope", rootMarkerID: "root", requiredMarkerIDs: ["root"])
    }
    #expect(throws: BlueprintProbeError.invalidConfiguration) {
        try ProbeConfiguration(targetID: "private/value", targetGeneration: "run", surfaceID: "main",
            scopeID: "scope", rootMarkerID: "root", requiredMarkerIDs: [])
    }
    #expect(throws: BlueprintProbeError.invalidConfiguration) {
        try ProbeConfiguration(targetID: "app", targetGeneration: "run", surfaceID: "main",
            scopeID: "scope", rootMarkerID: "root", requiredMarkerIDs: (0..<256).map { "node-\($0)" })
    }
}

@Test func markerValidationRejectsMissingDuplicateStaleAndInvalidGeometry() throws {
    let c = try configuration()
    #expect(try validateProbeMarkers(markers(), configuration: c, expectedMounts: [:]).count == 3)
    #expect(throws: BlueprintProbeError.incompleteScope) {
        try validateProbeMarkers(Array(markers().dropLast()), configuration: c, expectedMounts: [:])
    }
    #expect(throws: BlueprintProbeError.duplicateMarker) {
        try validateProbeMarkers(markers() + [markers()[0]], configuration: c, expectedMounts: [:])
    }
    #expect(throws: BlueprintProbeError.staleMount) {
        try validateProbeMarkers(markers(), configuration: c, expectedMounts: ["text": "old-mount"])
    }
    var broken = markers()
    broken[1] = ProbeMarker(id: "icon", mountID: "icon-mount", parent: "root",
        bounds: CGRect(x: 0, y: 0, width: -1, height: 24))
    #expect(throws: BlueprintProbeError.invalidGeometry) {
        try validateProbeMarkers(broken, configuration: c, expectedMounts: [:])
    }
}

@Test func markerParentsMustReachTheSelectedRoot() throws {
    var broken = markers()
    broken[1] = ProbeMarker(id: "icon", mountID: "icon-mount", parent: "text", bounds: broken[1].bounds)
    broken[2] = ProbeMarker(id: "text", mountID: "text-mount", parent: "icon", bounds: broken[2].bounds)
    #expect(throws: BlueprintProbeError.invalidParent) {
        try validateProbeMarkers(broken, configuration: configuration(), expectedMounts: [:])
    }
}

@Test @MainActor func retainedChildStateGetsANewMountAfterDisappear() throws {
    let retained = ProbeMount()
    let firstAppearance = retained.id
    let initial = markers(textMount: retained.id)
    #expect(try validateProbeMarkers(initial, configuration: configuration(),
        expectedMounts: ["text": firstAppearance]).count == 3)
    // This is the same retained mount-state operation invoked by onDisappear,
    // not a hand-authored replacement ID. SwiftUI may keep the state instance.
    retained.renew()
    let reappeared = markers(textMount: retained.id)
    #expect(retained.id != firstAppearance)
    #expect(throws: BlueprintProbeError.staleMount) {
        try validateProbeMarkers(reappeared, configuration: configuration(),
            expectedMounts: ["text": firstAppearance])
    }
    #expect(try validateProbeMarkers(reappeared, configuration: configuration(),
        expectedMounts: ["text": retained.id]).count == 3)
}

@MainActor private func session(selection: BlueprintProbeSelection = .requiredMarkers) throws -> BlueprintProbeSession {
    try BlueprintProbeSession(targetID: "example.app", targetGeneration: "run-1", surfaceID: "main",
        scopeID: "sample", rootMarkerID: "root", requiredMarkerIDs: ["icon", "text"], selection: selection)
}

@MainActor private func capture(_ session: BlueprintProbeSession, id: String,
                                markers: [ProbeMarker] = markers(), cap: Int = 1_048_576,
                                expectedMounts: [String: String] = [:]) async throws -> BlueprintProbeSnapshot {
    let task = Task { try await session.capture(requestID: id, stateID: "ordinary",
        expectedMounts: expectedMounts, maxOutputBytes: cap) }
    await Task.yield()
    // Synthetic source callback exercises lifecycle/encoding. Real SwiftUI layout
    // and actual-app invariance are a separate required native integration proof.
    session.resolve(token: session.requestRevision, rootMountID: "root-mount", markers: markers,
        globalFrame: CGRect(x: 0, y: 28, width: 240, height: 80), displayScale: 2)
    return try await task.value
}

@Test @MainActor func captureKeepsLayoutRevisionAndRejectsOversizedOutput() async throws {
    let s = try session()
    s.attach(rootMountID: "root-mount")
    let first = try await capture(s, id: "capture-1")
    let same = try await capture(s, id: "capture-2")
    #expect(first.revision == 1 && same.revision == 2)
    #expect(first.layoutRevision == same.layoutRevision)
    let changed = try await capture(s, id: "capture-3", markers: markers(gap: 18))
    #expect(changed.layoutRevision == first.layoutRevision + 1)
    #expect(first.mountIDs == changed.mountIDs)
    await #expect(throws: BlueprintProbeError.resourceLimit) { try await capture(s, id: "small", cap: 100) }
    let next = try await capture(s, id: "capture-4", markers: markers(textMount: "remounted-text"))
    #expect(next.mountIDs["text"] != first.mountIDs["text"])
    if let directory = ProcessInfo.processInfo.environment["UIB_SWIFTUI_TEST_OUTPUT"] {
        try first.data.write(to: URL(fileURLWithPath: directory).appendingPathComponent("before.json"))
        try changed.data.write(to: URL(fileURLWithPath: directory).appendingPathComponent("after.json"))
    }
}

@Test @MainActor func detachedOrMultipleRootsCannotPublish() async throws {
    let s = try session()
    await #expect(throws: BlueprintProbeError.unavailable) {
        try await s.capture(requestID: "no-root", stateID: "ordinary")
    }
    s.attach(rootMountID: "root-mount")
    let task = Task { try await s.capture(requestID: "detach", stateID: "ordinary") }
    await Task.yield()
    s.detach(rootMountID: "root-mount")
    await #expect(throws: BlueprintProbeError.staleMount) { try await task.value }
    s.attach(rootMountID: "root-mount")
    s.attach(rootMountID: "other-root")
    #expect(!s.isReady)
}

@Test @MainActor func requestDeadlineAndCancellationAreBounded() async throws {
    let s = try session()
    s.attach(rootMountID: "root-mount")
    await #expect(throws: BlueprintProbeError.timeout) {
        try await s.capture(requestID: "timeout", stateID: "ordinary", timeoutMilliseconds: 10)
    }
    let task = Task { try await s.capture(requestID: "cancel", stateID: "ordinary") }
    await Task.yield()
    task.cancel()
    await #expect(throws: BlueprintProbeError.cancelled) { try await task.value }
    #expect(try await capture(s, id: "after-cancel").revision == 1)
}

@Test @MainActor func concurrentAndStaleCaptureRequestsRefuse() async throws {
    let s = try session()
    s.attach(rootMountID: "root-mount")
    let first = Task {
        try await s.capture(requestID: "first", stateID: "ordinary", expectedMounts: ["text": "old-mount"])
    }
    await Task.yield()
    await #expect(throws: BlueprintProbeError.busy) {
        try await s.capture(requestID: "second", stateID: "ordinary")
    }
    s.resolve(token: s.requestRevision, rootMountID: "root-mount", markers: markers(),
        globalFrame: CGRect(x: 0, y: 0, width: 240, height: 80), displayScale: 2)
    await #expect(throws: BlueprintProbeError.staleMount) { try await first.value }
    #expect(try await capture(s, id: "fresh").revision == 1)
}

@Test @MainActor func completedSnapshotCannotCrossLaterMountInvalidation() async throws {
    let s = try session()
    s.attach(rootMountID: "root-mount")
    let captured = try await capture(s, id: "before-detach")
    #expect(s.isCurrentMounts(captured))
    // The capture already completed, so invalidating only a pending continuation
    // would miss this race at the app's subsequent file-publication boundary.
    s.markerDetached(id: "text", mountID: "text-mount")
    #expect(s.isReady)
    #expect(!s.isCurrentMounts(captured))
    let fresh = try await capture(s, id: "after-detach", markers: markers(textMount: "new-text"))
    #expect(s.isCurrentMounts(fresh))
    s.detach(rootMountID: "root-mount")
    s.attach(rootMountID: "root-mount")
    #expect(!s.isCurrentMounts(fresh))
    let other = try session()
    other.attach(rootMountID: "root-mount")
    #expect(!other.isCurrentMounts(fresh))
}

@Test @MainActor func captureDeliversOneExplicitEventToTheExistingReader() async throws {
    let s = try session()
    s.attach(rootMountID: "root-mount")
    var delivered = [UInt64]()
    let subscription = s.captureRequests.sink { token in
        delivered.append(token)
        s.resolve(token: token, rootMountID: "root-mount", markers: markers(),
            globalFrame: CGRect(x: 0, y: 28, width: 240, height: 80), displayScale: 2)
    }
    let first = try await s.capture(requestID: "event-1", stateID: "ordinary")
    #expect(first.revision == 1 && delivered == [1])
    let second = try await s.capture(requestID: "event-2", stateID: "ordinary")
    #expect(second.revision == 2 && delivered == [1, 2])
    await Task.yield()
    #expect(delivered == [1, 2])
    withExtendedLifetime(subscription) {}
}

private func optionalBranch(mountID: String = "panel-mount") -> [ProbeMarker] {
    [ProbeMarker(id: "panel", mountID: mountID, parent: "root",
        bounds: CGRect(x: 12, y: 42, width: 216, height: 32)),
     ProbeMarker(id: "panel-row-0", mountID: "row-mount", parent: "panel",
        bounds: CGRect(x: 16, y: 46, width: 208, height: 24))]
}

@Test func nestedRootPreferenceBucketsAreIndependentAndBounded() throws {
    let c = try configuration(selection: .mountedSubtree)
    let foreign = (0..<300).map {
        ProbeMarker(id: "foreign-\($0)", mountID: "foreign-mount-\($0)", parent: "foreign-root", bounds: .zero)
    }
    var groups = [String: [ProbeMarker]]()
    mergeProbeMarkerGroups(&groups, next: ["nested-session": foreign, "selected-session": markers()])
    mergeProbeMarkerGroups(&groups, next: ["selected-session": optionalBranch(), "nested-session": foreign])
    let selected = try #require(groups["selected-session"])
    #expect(groups["nested-session"]?.count == 257)
    #expect(try validateProbeMarkers(selected, configuration: c, expectedMounts: [:]).count == 5)
    mergeProbeMarkerGroups(&groups, next: ["selected-session": foreign])
    #expect(groups["selected-session"]?.count == 257)
    #expect(throws: BlueprintProbeError.resourceLimit) {
        try validateProbeMarkers(groups["selected-session"]!, configuration: c, expectedMounts: [:])
    }
}

@Test func mountedSubtreeKeepsRequiredMinimumAndOptionalHierarchy() throws {
    let c = try configuration(selection: .mountedSubtree)
    let all = markers() + optionalBranch()
    #expect(try validateProbeMarkers(all, configuration: c, expectedMounts: [:]).count == 5)
    #expect(try validateProbeMarkers(markers(), configuration: c, expectedMounts: [:]).count == 3)
    #expect(throws: BlueprintProbeError.incompleteScope) {
        try validateProbeMarkers(all.filter { $0.id != "icon" }, configuration: c, expectedMounts: [:])
    }
    #expect(throws: BlueprintProbeError.invalidRequest) {
        try validateProbeMarkers(all, configuration: configuration(), expectedMounts: [:])
    }
    #expect(throws: BlueprintProbeError.staleMount) {
        try validateProbeMarkers(markers(), configuration: c, expectedMounts: ["panel": "panel-mount"])
    }
}

@Test func optionalNodesMustHaveValidUniqueRootedIdentityWithinTheLimit() throws {
    let c = try configuration(selection: .mountedSubtree)
    let all = markers() + optionalBranch()
    #expect(throws: BlueprintProbeError.duplicateMarker) {
        try validateProbeMarkers(all + optionalBranch(), configuration: c, expectedMounts: [:])
    }
    #expect(throws: BlueprintProbeError.invalidParent) {
        try validateProbeMarkers(markers() + optionalBranch().suffix(1), configuration: c, expectedMounts: [:])
    }
    let privateID = ProbeMarker(id: "private/value", mountID: "new-mount", parent: "root", bounds: .zero)
    #expect(throws: BlueprintProbeError.invalidRequest) {
        try validateProbeMarkers(all + [privateID], configuration: c, expectedMounts: [:])
    }
    let overflow = markers() + (0..<254).map {
        ProbeMarker(id: "optional-\($0)", mountID: "optional-mount-\($0)", parent: "root", bounds: .zero)
    }
    #expect(throws: BlueprintProbeError.resourceLimit) {
        try validateProbeMarkers(overflow, configuration: c, expectedMounts: [:])
    }
}

@Test @MainActor func optionalOccurrencesAreEncodedAndMountFenced() async throws {
    let s = try session(selection: .mountedSubtree)
    s.attach(rootMountID: "root-mount")
    let mount = ProbeMount()
    let all = markers() + optionalBranch(mountID: mount.id)
    let first = try await capture(s, id: "whole-root", markers: all)
    #expect(first.mountIDs.count == 5 && first.mountIDs["panel"] == mount.id)
    let document = try #require(JSONSerialization.jsonObject(with: first.data) as? [String: Any])
    let artifact = try #require(document["artifact"] as? [String: Any])
    let snapshot = try #require(artifact["data"] as? [String: Any])
    let nodes = try #require(snapshot["nodes"] as? [[String: Any]])
    #expect(nodes.count == 5)
    let panel = try #require(nodes.first { node in
        (node["key"] as? [String: String])?["key"] == "panel-\(mount.id)"
    })
    #expect((panel["children"] as? [[String: String]]) == [["namespace": "swiftui.layout", "key": "panel-row-0-row-mount"]])
    s.markerDetached(id: "panel", mountID: mount.id)
    #expect(!s.isCurrentMounts(first))
    mount.renew()
    let reappeared = markers() + optionalBranch(mountID: mount.id)
    await #expect(throws: BlueprintProbeError.staleMount) {
        try await capture(s, id: "old-optional", markers: reappeared, expectedMounts: first.mountIDs)
    }
    await #expect(throws: BlueprintProbeError.staleMount) {
        try await capture(s, id: "absent-optional", expectedMounts: first.mountIDs)
    }
    let current = try await capture(s, id: "current-optional", markers: reappeared,
        expectedMounts: ["panel": mount.id])
    #expect(s.isCurrentMounts(current))
    s.markerAttached(id: "new-control", mountID: "new-control-mount")
    #expect(!s.isCurrentMounts(current))
    if let directory = ProcessInfo.processInfo.environment["UIB_SWIFTUI_TEST_OUTPUT"] {
        try first.data.write(to: URL(fileURLWithPath: directory).appendingPathComponent("mounted-subtree.json"))
    }
}

@Test @MainActor func optionalLifecycleChangesRefusePendingSubtreeCaptureOnly() async throws {
    let s = try session(selection: .mountedSubtree)
    s.attach(rootMountID: "root-mount")
    let pending = Task { try await s.capture(requestID: "optional-arrives", stateID: "ordinary") }
    await Task.yield()
    s.markerAttached(id: "optional", mountID: "new-optional")
    await #expect(throws: BlueprintProbeError.staleMount) { try await pending.value }
    let exact = try session()
    exact.attach(rootMountID: "root-mount")
    let snapshot = try await capture(exact, id: "exact-subset")
    exact.markerAttached(id: "optional", mountID: "new-optional")
    exact.markerDetached(id: "optional", mountID: "new-optional")
    #expect(exact.isCurrentMounts(snapshot))
    await #expect(throws: BlueprintProbeError.invalidRequest) {
        try await exact.capture(requestID: "outside-subset", stateID: "ordinary",
            expectedMounts: ["optional": "new-optional"])
    }
}
#endif
