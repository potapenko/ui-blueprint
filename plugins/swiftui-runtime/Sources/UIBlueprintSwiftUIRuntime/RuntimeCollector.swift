#if DEBUG
#if os(macOS)
import AppKit
#elseif os(iOS) || os(tvOS)
import UIKit
#endif
import Foundation
import SwiftUI

enum RuntimeRefusal: String, Error {
    case invalidRequest, wrongSession, busy, cancelled, sourceNotReady
    case ambiguousWindow, staleWindow, unsupportedSource, sourceLimit, invalidGeometry
    case outputLimit, outputFailure, launchProfileRequired
    case subtreeNotFound, ambiguousSubtree
    case unsupportedRuntimeProfile, updateContextUnavailable
}

/// Fixed, value-free evidence from the guard that refused acquisition.
struct RuntimeSourceLimit: Error {
    enum Kind: String {
        case nativeNodes = "native_nodes", nativeDepth = "native_depth"
        case sourceNodes = "source_nodes", sourceDepth = "source_depth"
        case typeNameBytes = "type_name_bytes", typeNameControls = "type_name_controls"
        case snapshotSequence = "snapshot_sequence"

        var limit: UInt64 {
            switch self {
            case .nativeNodes, .sourceNodes: return 1_048_576
            case .nativeDepth, .sourceDepth: return 512
            case .typeNameBytes: return 65_536
            case .typeNameControls: return 0
            case .snapshotSequence: return UInt64.max
            }
        }
    }
    let kind: Kind
    let observed: UInt64
    var metadata: [String: Any] { ["kind": kind.rawValue, "limit": kind.limit, "observed": observed] }
}

struct RuntimeNode {
    let index: Int
    let parent: Int?
    let hostingRoot: Int
    let type: String?
    let position: CGPoint?
    let size: CGSize?
    var positionSource: Int? = nil
    var sizeSource: Int? = nil
    var transform: RuntimeAffine? = nil
}

enum RuntimeSource: String {
    case swiftui, uikit
    var namespace: String { self == .uikit ? "uikit.runtime.layout" : "swiftui.runtime.layout" }
}

struct RuntimeObservation {
    let id: String
    let start: Double
    let end: Double
    let window: Int
    let windowGeneration: String
    var nodes: [RuntimeNode]
    var hostingRoots: Int = 0
    var emptyHostingRoots: Int = 0
    var selection: RuntimeSubtreeSelection? = nil
    var source: RuntimeSource = .swiftui
    var windowMetadata: [String: Any] = [:]

    var scopeID: String {
        let windowScope = "window-\(window)"
        return selection.map { "\(windowScope)/subtree-n\($0.root)-with-ancestors" } ?? windowScope
    }

    func selecting(type: String?) throws -> RuntimeObservation {
        guard let type else { return self }
        try RuntimeSubtreeSelection.validate(type)
        let matches = nodes.filter { $0.type == type }
        guard !matches.isEmpty else { throw RuntimeRefusal.subtreeNotFound }
        guard matches.count == 1 else { throw RuntimeRefusal.ambiguousSubtree }
        let root = matches[0].index
        let byIndex = Dictionary(uniqueKeysWithValues: nodes.map { ($0.index, $0) })
        var ancestors = Set<Int>()
        var parent = matches[0].parent
        while let index = parent {
            ancestors.insert(index)
            parent = byIndex[index]?.parent
        }
        // Collector order is parent-before-child; keep all actual descendants.
        var subtree: Set<Int> = [root]
        for node in nodes where node.parent.map(subtree.contains) == true {
            subtree.insert(node.index)
        }
        let retained = subtree.union(ancestors)
        let children = Dictionary(grouping: nodes.compactMap { $0.parent }, by: { $0 })
        var result = self
        result.selection = RuntimeSubtreeSelection(type: type, root: root, subtree: subtree,
            ancestors: ancestors, sourceNodes: nodes.count,
            originalChildCounts: children.mapValues(\.count))
        result.nodes = nodes.filter { retained.contains($0.index) }
        return result
    }
}

struct RuntimeSubtreeSelection {
    let type: String
    let root: Int
    let subtree: Set<Int>
    let ancestors: Set<Int>
    let sourceNodes: Int
    let originalChildCounts: [Int: Int]

    static func validate(_ type: String) throws {
        guard !type.isEmpty, type.utf8.count <= 1024,
              !type.unicodeScalars.contains(where: { $0.value < 32 || (127...159).contains($0.value) }) else {
            throw RuntimeRefusal.invalidRequest
        }
    }

    func metadata(observation: String) -> [String: Any] {
        ["type": type, "root_key": "\(observation)-n\(root)",
         "source_nodes": sourceNodes, "subtree_nodes": subtree.count,
         "ancestor_context_nodes": ancestors.count,
         "excluded_source_nodes": sourceNodes - subtree.count - ancestors.count]
    }
}

@MainActor
final class RuntimeCollector {
    static let maxNodes = 1_048_576
    static let maxDepth = 512
    nonisolated static let maxBytes = 1024 * 1024 * 1024

    private final class Witness {
        weak var window: RuntimeWindow?
        let generation = UUID().uuidString
        init(_ window: RuntimeWindow) { self.window = window }
    }
    private var witnesses: [Int: Witness] = [:]

    func windows() -> [[String: Any]] {
        witnesses = witnesses.filter { $0.value.window != nil }
        return RuntimePlatform.windows.map { window in
            let generation = witness(window)
            var value: [String: Any] = ["window": windowID(window), "generation": generation,
                    "width": window.frame.width, "height": window.frame.height,
                    "is_key": window.isKeyWindow]
            value.merge(RuntimePlatform.metadata(window)) { _, new in new }
            return value
        }
    }

    func capture(windowNumber: Int?, expectedGeneration: String?, source: RuntimeSource = .swiftui) throws -> RuntimeObservation {
        if source == .uikit, !RuntimePlatform.supportsUIKit { throw RuntimeRefusal.unsupportedSource }
        let profile = ProcessInfo.processInfo.environment["SWIFTUI_VIEW_DEBUG"]
        guard profile == "29" || profile == "287" else { throw RuntimeRefusal.launchProfileRequired }
        let candidates = RuntimePlatform.windows.filter { windowNumber == nil || windowID($0) == windowNumber }
        guard !candidates.isEmpty else { throw RuntimeRefusal.sourceNotReady }
        guard candidates.count == 1 else { throw RuntimeRefusal.ambiguousWindow }
        let window = candidates[0]
        let generation = witness(window)
        guard expectedGeneration == nil || expectedGeneration == generation else { throw RuntimeRefusal.staleWindow }
        guard let content = window.runtimeContent else { throw RuntimeRefusal.sourceNotReady }
        let start = ProcessInfo.processInfo.systemUptime
        var nodes: [RuntimeNode] = []
        var nativeCount = 0
        var hostCount = 0
        var emptyHostCount = 0

        func visit(_ view: RuntimeView, depth: Int) throws {
            nativeCount += 1
            guard nativeCount <= Self.maxNodes else {
                throw RuntimeSourceLimit(kind: .nativeNodes, observed: UInt64(nativeCount))
            }
            guard depth <= Self.maxDepth else {
                throw RuntimeSourceLimit(kind: .nativeDepth, observed: UInt64(depth))
            }
            if let host = view as? any RuntimeHosting {
                let root = hostCount
                hostCount += 1
                let before = nodes.count
                try Self.collect(try host.runtimeGeometryTree(), root: root, nodes: &nodes)
                if nodes.count == before { emptyHostCount += 1 }
            }
            for child in view.subviews { try visit(child, depth: depth + 1) }
        }
        var nativeRoot = content
        while let parent = nativeRoot.superview, parent.window === window { nativeRoot = parent }
        if source == .uikit {
            #if (os(iOS) || os(tvOS)) && targetEnvironment(simulator)
            nodes = try RuntimeUIKit.collect(window)
            #else
            throw RuntimeRefusal.unsupportedSource
            #endif
        } else {
            try visit(nativeRoot, depth: 0)
            guard hostCount > 0, !nodes.isEmpty else { throw RuntimeRefusal.unsupportedSource }
        }
        guard window.runtimeVisible, window.runtimeContent === content, witness(window) == generation else {
            throw RuntimeRefusal.staleWindow
        }
        return RuntimeObservation(id: UUID().uuidString, start: start,
            end: ProcessInfo.processInfo.systemUptime, window: windowID(window),
            windowGeneration: generation, nodes: nodes, hostingRoots: hostCount, emptyHostingRoots: emptyHostCount,
            source: source, windowMetadata: source == .uikit ? RuntimePlatform.uikitMetadata(window) : [:])
    }

    private var nextWindowID = 1
    private func windowID(_ window: RuntimeWindow) -> Int {
        #if os(macOS)
        return window.windowNumber
        #else
        if let existing = witnesses.first(where: { $0.value.window === window }) { return existing.key }
        let key = nextWindowID
        nextWindowID += 1
        witnesses[key] = Witness(window)
        return key
        #endif
    }

    private func witness(_ window: RuntimeWindow) -> String {
        witnesses = witnesses.filter { $0.value.window != nil }
        let key = windowID(window)
        if let existing = witnesses[key], existing.window === window { return existing.generation }
        let value = Witness(window)
        witnesses[key] = value
        return value.generation
    }

    static func collect(_ roots: [_ViewDebug.Data], root: Int, nodes: inout [RuntimeNode]) throws {
        var pending = roots.reversed().map { ($0, Optional<Int>.none, 0) }
        while let (value, parent, depth) = pending.popLast() {
            guard nodes.count < maxNodes else {
                throw RuntimeSourceLimit(kind: .sourceNodes, observed: UInt64(nodes.count) + 1)
            }
            guard depth <= maxDepth else {
                throw RuntimeSourceLimit(kind: .sourceDepth, observed: UInt64(depth))
            }
            let fields = Mirror(reflecting: value).children
            guard let data = fields.first(where: { $0.label == "data" })?.value as? [_ViewDebug.Property: Any],
                  let children = fields.first(where: { $0.label == "childData" })?.value as? [_ViewDebug.Data]
            else { throw RuntimeRefusal.unsupportedSource }
            // Metatype lookup is not reflection/encoding of the source value's fields.
            let metatype = (data[.type] as? Any.Type) ?? data[.value].map { Swift.type(of: $0) }
            let sourceType = metatype.map { String(reflecting: $0) }
            let type = try sourceType.map(sanitizedType)
            let index = nodes.count
            let ancestor = parent.map { nodes[$0] }
            let position = data[.position] == nil ? ancestor?.position : data[.position] as? CGPoint
            let size = data[.size] == nil ? ancestor?.size : data[.size] as? CGSize
            guard data[.position] == nil || position != nil, data[.size] == nil || size != nil else {
                throw RuntimeRefusal.unsupportedSource
            }
            let positionSource = data[.position] == nil ? ancestor?.positionSource : index
            let sizeSource = data[.size] == nil ? ancestor?.sizeSource : index
            let transform: RuntimeAffine?
            if let value = data[.transform] { transform = RuntimeAffine.source(value) }
            else { transform = ancestor?.transform }
            if let position, !position.x.isFinite || !position.y.isFinite { throw RuntimeRefusal.invalidGeometry }
            if let size, !size.width.isFinite || !size.height.isFinite || size.width < 0 || size.height < 0 {
                throw RuntimeRefusal.invalidGeometry
            }
            nodes.append(RuntimeNode(index: index, parent: parent, hostingRoot: root,
                                     type: type, position: position, size: size,
                                     positionSource: positionSource, sizeSource: sizeSource, transform: transform))
            guard pending.count + children.count + nodes.count <= maxNodes else {
                throw RuntimeSourceLimit(kind: .sourceNodes,
                                         observed: UInt64(pending.count + children.count + nodes.count))
            }
            pending.append(contentsOf: children.reversed().map { ($0, Optional(index), depth + 1) })
        }
    }

    static func sanitizedType(_ name: String) throws -> String {
        let nominal = String(name.prefix(while: { $0 != "<" }))
        guard nominal.utf8.count <= 65_536 else {
            throw RuntimeSourceLimit(kind: .typeNameBytes, observed: UInt64(nominal.utf8.count))
        }
        guard !name.unicodeScalars.contains(where: { $0.value < 32 }) else {
            throw RuntimeSourceLimit(kind: .typeNameControls, observed: 1)
        }
        return nominal.replacingOccurrences(
            of: #"\(unknown context at \$[0-9a-fA-F]+\)"#, with: "(private)", options: .regularExpression)
    }
}
#endif
