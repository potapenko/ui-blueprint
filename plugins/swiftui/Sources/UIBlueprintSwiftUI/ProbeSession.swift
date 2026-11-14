#if DEBUG
import Foundation
import Observation

/// The app owns one session for one selected root. No IO or app state is owned here.
@MainActor @Observable
public final class BlueprintProbeSession {
    public private(set) var requestRevision: UInt64 = 0
    public var isReady: Bool { rootMountID != nil }
    let configuration: ProbeConfiguration
    let sessionID = UUID().uuidString
    @ObservationIgnored private var rootMountID: String?
    @ObservationIgnored private var rootCollision = false
    @ObservationIgnored private var surfaceGeneration = UUID().uuidString
    @ObservationIgnored private var pending: Pending?
    @ObservationIgnored private var timer: Task<Void, Never>?
    @ObservationIgnored private var sequence: UInt64 = 0
    @ObservationIgnored private var layoutRevision: UInt64 = 0
    @ObservationIgnored private var lastLayout: LayoutSample?

    private struct LayoutSample: Equatable {
        let markers: [ProbeMarker]
        let globalFrame: CGRect
        let scale: Double
        let stateID: String
    }
    private struct Pending {
        let captureID: UUID
        let requestID: String
        let stateID: String
        let expectedMounts: [String: String]
        let started: Double
        let deadline: Double
        let outputCap: Int
        let continuation: CheckedContinuation<BlueprintProbeSnapshot, any Error>
    }

    public init(targetID: String, targetGeneration: String, surfaceID: String,
                scopeID: String, rootMarkerID: String, requiredMarkerIDs: [String]) throws {
        configuration = try ProbeConfiguration(targetID: targetID, targetGeneration: targetGeneration,
            surfaceID: surfaceID, scopeID: scopeID, rootMarkerID: rootMarkerID,
            requiredMarkerIDs: requiredMarkerIDs)
    }

    public func capture(requestID: String, stateID: String, expectedMounts: [String: String] = [:],
                        timeoutMilliseconds: Int = 1000,
                        maxOutputBytes: Int = 1_048_576) async throws -> BlueprintProbeSnapshot {
        guard publicProbeID(requestID), publicProbeID(stateID),
              (1...10_000).contains(timeoutMilliseconds), (1...1_048_576).contains(maxOutputBytes),
              expectedMounts.allSatisfy({ configuration.requiredMarkerIDs.contains($0.key) && publicProbeID($0.value) })
        else { throw BlueprintProbeError.invalidRequest }
        guard rootMountID != nil else { throw BlueprintProbeError.unavailable }
        guard pending == nil else { throw BlueprintProbeError.busy }
        try Task.checkCancellation()
        let captureID = UUID()
        return try await withTaskCancellationHandler {
            try await withCheckedThrowingContinuation { continuation in
                guard requestRevision < UInt64.max else {
                    continuation.resume(throwing: BlueprintProbeError.resourceLimit)
                    return
                }
                let now = ProcessInfo.processInfo.systemUptime
                pending = Pending(captureID: captureID, requestID: requestID, stateID: stateID, expectedMounts: expectedMounts,
                    started: now, deadline: now + Double(timeoutMilliseconds) / 1000,
                    outputCap: maxOutputBytes, continuation: continuation)
                requestRevision += 1
                let token = requestRevision
                timer = Task { @MainActor [weak self] in
                    do { try await Task.sleep(for: .milliseconds(timeoutMilliseconds)) }
                    catch { return }
                    self?.fail(.timeout, token: token)
                }
            }
        } onCancel: {
            Task { @MainActor [weak self] in self?.cancel(captureID: captureID) }
        }
    }

    func attach(rootMountID: String) {
        guard !rootCollision else { return }
        guard self.rootMountID != rootMountID else { return }
        if self.rootMountID != nil {
            fail(.duplicateMarker)
            self.rootMountID = nil
            rootCollision = true
            return
        }
        fail(.staleMount)
        self.rootMountID = rootMountID
        surfaceGeneration = UUID().uuidString
        lastLayout = nil
    }

    func detach(rootMountID: String) {
        guard self.rootMountID == rootMountID else { return }
        fail(.staleMount)
        self.rootMountID = nil
        lastLayout = nil
    }

    func markerDetached(id: String, mountID: String) {
        guard configuration.requiredMarkerIDs.contains(id) else { return }
        if pending != nil { fail(.staleMount) }
    }

    func wantsCapture(token: UInt64) -> Bool {
        token == requestRevision && pending != nil && rootMountID != nil
    }

    func resolve(token: UInt64, rootMountID: String, markers: [ProbeMarker],
                 globalFrame: CGRect, displayScale: Double) {
        guard wantsCapture(token: token), let request = pending else { return }
        guard self.rootMountID == rootMountID else { return fail(.staleMount) }
        let end = ProcessInfo.processInfo.systemUptime
        guard end <= request.deadline else { return fail(.timeout) }
        do {
            guard displayScale.isFinite, displayScale > 0, finiteProbeRect(globalFrame)
            else { throw BlueprintProbeError.invalidGeometry }
            let selected = try validateProbeMarkers(markers, configuration: configuration,
                expectedMounts: request.expectedMounts)
            guard selected.first(where: { $0.id == configuration.rootMarkerID })?.mountID == rootMountID
            else { throw BlueprintProbeError.staleMount }
            guard sequence < UInt64.max, layoutRevision < UInt64.max
            else { throw BlueprintProbeError.resourceLimit }
            let layout = LayoutSample(markers: selected, globalFrame: globalFrame,
                scale: displayScale, stateID: request.stateID)
            let nextLayout = layout == lastLayout ? layoutRevision : layoutRevision + 1
            let data = try ProbeEncoding.snapshot(configuration: configuration, sessionID: sessionID,
                surfaceGeneration: surfaceGeneration, sequence: sequence + 1, layoutRevision: nextLayout,
                requestID: request.requestID, stateID: request.stateID, start: request.started, end: end,
                markers: selected, globalFrame: globalFrame, scale: displayScale, cap: request.outputCap)
            guard ProcessInfo.processInfo.systemUptime <= request.deadline else { throw BlueprintProbeError.timeout }
            sequence += 1
            layoutRevision = nextLayout
            lastLayout = layout
            pending = nil
            timer?.cancel()
            timer = nil
            request.continuation.resume(returning: BlueprintProbeSnapshot(data: data, revision: sequence,
                layoutRevision: nextLayout, mountIDs: Dictionary(uniqueKeysWithValues: selected.map { ($0.id, $0.mountID) }),
                surfaceGeneration: surfaceGeneration, rootFrameInGlobalSpace: globalFrame, displayScale: displayScale))
        } catch {
            pending = nil
            timer?.cancel()
            timer = nil
            request.continuation.resume(throwing: error)
        }
    }

    private func cancel(captureID: UUID) {
        if pending?.captureID == captureID { fail(.cancelled) }
    }

    private func fail(_ error: BlueprintProbeError, token: UInt64? = nil) {
        if let token, token != requestRevision { return }
        let request = pending
        pending = nil
        timer?.cancel()
        timer = nil
        request?.continuation.resume(throwing: error)
    }
}
#endif
