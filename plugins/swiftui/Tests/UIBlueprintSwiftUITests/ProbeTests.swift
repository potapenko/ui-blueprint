#if DEBUG
import Foundation
import Testing
@testable import UIBlueprintSwiftUI

private func configuration() throws -> ProbeConfiguration {
    try ProbeConfiguration(targetID: "example.app", targetGeneration: "run-1", surfaceID: "main",
        scopeID: "sample", rootMarkerID: "root", requiredMarkerIDs: ["icon", "text"])
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

@Test func retainedChildStateGetsANewMountAfterDisappear() throws {
    var retained = ProbeMount()
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

@MainActor private func session() throws -> BlueprintProbeSession {
    try BlueprintProbeSession(targetID: "example.app", targetGeneration: "run-1", surfaceID: "main",
        scopeID: "sample", rootMarkerID: "root", requiredMarkerIDs: ["icon", "text"])
}

@MainActor private func capture(_ session: BlueprintProbeSession, id: String,
                                markers: [ProbeMarker] = markers(), cap: Int = 1_048_576) async throws -> BlueprintProbeSnapshot {
    let task = Task { try await session.capture(requestID: id, stateID: "ordinary", maxOutputBytes: cap) }
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
#endif
