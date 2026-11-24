#if DEBUG
import AppKit
import Foundation
import SwiftUI

enum RuntimeRefusal: String, Error {
    case invalidRequest, wrongSession, busy, cancelled, sourceNotReady
    case ambiguousWindow, staleWindow, unsupportedSource, sourceLimit, invalidGeometry
    case outputLimit, outputFailure, launchProfileRequired
    case unsupportedRuntimeProfile, updateContextUnavailable
}

@MainActor
protocol RuntimeHosting {
    func runtimeGeometryTree() throws -> [_ViewDebug.Data]
}

extension NSHostingView: RuntimeHosting {
    func runtimeGeometryTree() throws -> [_ViewDebug.Data] {
        try RuntimeUpdateContext.perform { _viewDebugData() }
    }
}

struct RuntimeNode {
    let index: Int
    let parent: Int?
    let hostingRoot: Int
    let type: String?
    let position: CGPoint?
    let size: CGSize?
}

struct RuntimeObservation {
    let id: String
    let start: Double
    let end: Double
    let window: Int
    let windowGeneration: String
    let nodes: [RuntimeNode]
}

@MainActor
final class RuntimeCollector {
    static let maxNodes = 16_384
    static let maxDepth = 512
    static let maxBytes = 16 * 1024 * 1024

    private final class Witness {
        weak var window: NSWindow?
        let generation = UUID().uuidString
        init(_ window: NSWindow) { self.window = window }
    }
    private var witnesses: [Int: Witness] = [:]

    func windows() -> [[String: Any]] {
        witnesses = witnesses.filter { $0.value.window != nil }
        return NSApp.windows.filter { $0.isVisible && $0.windowNumber > 0 }.map { window in
            let generation = witness(window)
            return ["window": window.windowNumber, "generation": generation,
                    "width": window.frame.width, "height": window.frame.height,
                    "is_key": window.isKeyWindow]
        }
    }

    func capture(windowNumber: Int?, expectedGeneration: String?) throws -> RuntimeObservation {
        let profile = ProcessInfo.processInfo.environment["SWIFTUI_VIEW_DEBUG"]
        guard profile == "29" || profile == "287" else { throw RuntimeRefusal.launchProfileRequired }
        let candidates = NSApp.windows.filter {
            $0.isVisible && $0.windowNumber > 0 && (windowNumber == nil || $0.windowNumber == windowNumber)
        }
        guard !candidates.isEmpty else { throw RuntimeRefusal.sourceNotReady }
        guard candidates.count == 1 else { throw RuntimeRefusal.ambiguousWindow }
        let window = candidates[0]
        let generation = witness(window)
        guard expectedGeneration == nil || expectedGeneration == generation else { throw RuntimeRefusal.staleWindow }
        guard let content = window.contentView else { throw RuntimeRefusal.sourceNotReady }
        let start = ProcessInfo.processInfo.systemUptime
        var nodes: [RuntimeNode] = []
        var nativeCount = 0
        var hostCount = 0

        func visit(_ view: NSView, depth: Int) throws {
            nativeCount += 1
            guard nativeCount <= Self.maxNodes, depth <= Self.maxDepth else { throw RuntimeRefusal.sourceLimit }
            if let host = view as? any RuntimeHosting {
                let root = hostCount
                hostCount += 1
                try Self.collect(try host.runtimeGeometryTree(), root: root, nodes: &nodes)
                return
            }
            for child in view.subviews { try visit(child, depth: depth + 1) }
        }
        try visit(content, depth: 0)
        guard hostCount > 0, !nodes.isEmpty else { throw RuntimeRefusal.unsupportedSource }
        guard window.isVisible, window.contentView === content, witness(window) == generation else {
            throw RuntimeRefusal.staleWindow
        }
        return RuntimeObservation(id: UUID().uuidString, start: start,
            end: ProcessInfo.processInfo.systemUptime, window: window.windowNumber,
            windowGeneration: generation, nodes: nodes)
    }

    private func witness(_ window: NSWindow) -> String {
        witnesses = witnesses.filter { $0.value.window != nil }
        let key = window.windowNumber
        if let existing = witnesses[key], existing.window === window { return existing.generation }
        let value = Witness(window)
        witnesses[key] = value
        return value.generation
    }

    static func collect(_ roots: [_ViewDebug.Data], root: Int, nodes: inout [RuntimeNode]) throws {
        var pending = roots.reversed().map { ($0, Optional<Int>.none, 0) }
        while let (value, parent, depth) = pending.popLast() {
            guard nodes.count < maxNodes, depth <= maxDepth else { throw RuntimeRefusal.sourceLimit }
            let fields = Mirror(reflecting: value).children
            guard let data = fields.first(where: { $0.label == "data" })?.value as? [_ViewDebug.Property: Any],
                  let children = fields.first(where: { $0.label == "childData" })?.value as? [_ViewDebug.Data]
            else { throw RuntimeRefusal.unsupportedSource }
            // Do not reflect or encode value/environment/displayList/model payloads.
            let sourceType = (data[.type] as? Any.Type).map { String(reflecting: $0) }
            let type = try sourceType.map(sanitizedType)
            let position = data[.position] as? CGPoint
            let size = data[.size] as? CGSize
            if let position, !position.x.isFinite || !position.y.isFinite { throw RuntimeRefusal.invalidGeometry }
            if let size, !size.width.isFinite || !size.height.isFinite || size.width < 0 || size.height < 0 {
                throw RuntimeRefusal.invalidGeometry
            }
            let index = nodes.count
            nodes.append(RuntimeNode(index: index, parent: parent, hostingRoot: root,
                                     type: type, position: position, size: size))
            guard pending.count + children.count + nodes.count <= maxNodes else { throw RuntimeRefusal.sourceLimit }
            pending.append(contentsOf: children.reversed().map { ($0, Optional(index), depth + 1) })
        }
    }

    static func sanitizedType(_ name: String) throws -> String {
        guard name.utf8.count <= 65_536, !name.unicodeScalars.contains(where: { $0.value < 32 }) else {
            throw RuntimeRefusal.sourceLimit
        }
        return name.replacingOccurrences(of: #"\(unknown context at \$[0-9a-fA-F]+\)"#,
                                        with: "(private)", options: .regularExpression)
    }
}
#endif
