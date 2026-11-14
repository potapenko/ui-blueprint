#if DEBUG
import Foundation

public enum BlueprintProbeError: String, Error, Sendable {
    case invalidConfiguration, invalidRequest, unavailable, busy, incompleteScope
    case duplicateMarker, staleMount, invalidGeometry, invalidParent, resourceLimit
    case timeout, cancelled, encodingFailed
}

/// Immutable source output. `revision` is a capture sequence, not a displayed frame ID.
public struct BlueprintProbeSnapshot: Sendable {
    let sessionID: String
    let lifecycleRevision: UInt64
    public let data: Data
    public let revision: UInt64
    public let layoutRevision: UInt64
    public let mountIDs: [String: String]
    public let surfaceGeneration: String
    public let rootFrameInGlobalSpace: CGRect
    public let displayScale: Double
}

struct ProbeMarker: Equatable, Sendable {
    let id: String
    let mountID: String
    let parent: String?
    let bounds: CGRect
}

/// Retained SwiftUI state can survive disappearance; mount identity must not.
struct ProbeMount: Sendable {
    private(set) var id = UUID().uuidString
    mutating func renew() { id = UUID().uuidString }
}

struct ProbeConfiguration: Sendable {
    let targetID: String
    let targetGeneration: String
    let surfaceID: String
    let scopeID: String
    let rootMarkerID: String
    let requiredMarkerIDs: Set<String>

    init(targetID: String, targetGeneration: String, surfaceID: String,
         scopeID: String, rootMarkerID: String, requiredMarkerIDs: [String]) throws {
        let ids = requiredMarkerIDs + [rootMarkerID]
        let selected = Set(ids)
        guard !requiredMarkerIDs.contains(rootMarkerID),
              selected.count == ids.count, selected.count <= 256,
              ([targetID, targetGeneration, surfaceID, scopeID] + ids).allSatisfy(publicProbeID)
        else { throw BlueprintProbeError.invalidConfiguration }
        self.targetID = targetID
        self.targetGeneration = targetGeneration
        self.surfaceID = surfaceID
        self.scopeID = scopeID
        self.rootMarkerID = rootMarkerID
        self.requiredMarkerIDs = selected
    }
}

func publicProbeID(_ value: String) -> Bool {
    (1...96).contains(value.utf8.count) && value.utf8.allSatisfy {
        (48...57).contains($0) || (65...90).contains($0) || (97...122).contains($0)
            || [45, 46, 95].contains($0)
    }
}

func finiteProbeRect(_ rect: CGRect) -> Bool {
    [rect.origin.x, rect.origin.y, rect.size.width, rect.size.height].allSatisfy(\.isFinite)
        && rect.size.width >= 0 && rect.size.height >= 0
}

func validateProbeMarkers(_ markers: [ProbeMarker], configuration: ProbeConfiguration,
                          expectedMounts: [String: String]) throws -> [ProbeMarker] {
    guard markers.count <= 256 else { throw BlueprintProbeError.resourceLimit }
    var byID: [String: ProbeMarker] = [:]
    var mounts = Set<String>()
    for marker in markers {
        guard configuration.requiredMarkerIDs.contains(marker.id),
              publicProbeID(marker.mountID) else { throw BlueprintProbeError.invalidRequest }
        guard byID.updateValue(marker, forKey: marker.id) == nil,
              mounts.insert(marker.mountID).inserted else { throw BlueprintProbeError.duplicateMarker }
        guard finiteProbeRect(marker.bounds) else { throw BlueprintProbeError.invalidGeometry }
        if let expected = expectedMounts[marker.id], expected != marker.mountID {
            throw BlueprintProbeError.staleMount
        }
    }
    guard Set(byID.keys) == configuration.requiredMarkerIDs,
          let root = byID[configuration.rootMarkerID] else { throw BlueprintProbeError.incompleteScope }
    guard root.parent == nil, root.bounds.origin == .zero else { throw BlueprintProbeError.invalidParent }
    for marker in markers where marker.id != root.id {
        var visited = Set<String>()
        var current = marker
        while current.id != root.id {
            guard visited.insert(current.id).inserted,
                  let parentID = current.parent, let parent = byID[parentID]
            else { throw BlueprintProbeError.invalidParent }
            current = parent
        }
    }
    return markers.sorted { $0.id < $1.id }
}
#endif
