#if DEBUG
import SwiftUI

private struct ProbeSessionKey: EnvironmentKey {
    static var defaultValue: BlueprintProbeSession? { nil }
}

extension EnvironmentValues {
    var blueprintProbeSession: BlueprintProbeSession? {
        get { self[ProbeSessionKey.self] }
        set { self[ProbeSessionKey.self] = newValue }
    }
}

private struct ProbeAnchor {
    let id: String
    let mountID: String
    let mount: ProbeMount
    let parent: String?
    let anchor: Anchor<CGRect>
}

private struct ProbeAnchors: PreferenceKey {
    static var defaultValue: [String: [ProbeAnchor]] { [:] }
    static func reduce(value: inout [String: [ProbeAnchor]], nextValue: () -> [String: [ProbeAnchor]]) {
        mergeProbeMarkerGroups(&value, next: nextValue())
    }
}

private struct MarkerIdentity: Equatable {
    let id: String
    let parent: String
}

private struct ProbeMarkerModifier: ViewModifier {
    let identity: MarkerIdentity
    @State private var mount = ProbeMount()
    @Environment(\.blueprintProbeSession) private var session

    @ViewBuilder func body(content: Content) -> some View {
        if let session {
            content
                .transformAnchorPreference(key: ProbeAnchors.self, value: .bounds) { groups, anchor in
                    mergeProbeMarkerGroups(&groups, next: [session.sessionID: [
                        ProbeAnchor(id: identity.id, mountID: mount.id, mount: mount,
                            parent: identity.parent, anchor: anchor)]])
                }
                .onAppear { session.markerAttached(id: identity.id, mountID: mount.id) }
                .onChange(of: identity) { old, _ in
                    session.markerDetached(id: old.id, mountID: mount.id)
                    mount.renew()
                }
                .onDisappear {
                    session.markerDetached(id: identity.id, mountID: mount.id)
                    mount.renew()
                }
        } else {
            content
        }
    }
}

private struct ProbeRootModifier: ViewModifier {
    let session: BlueprintProbeSession
    @State private var mount = ProbeMount()
    @Environment(\.displayScale) private var displayScale

    func body(content: Content) -> some View {
        content
            .environment(\.blueprintProbeSession, session)
            .transformAnchorPreference(key: ProbeAnchors.self, value: .bounds) { groups, anchor in
                mergeProbeMarkerGroups(&groups, next: [session.sessionID: [
                    ProbeAnchor(id: session.configuration.rootMarkerID,
                        mountID: mount.id, mount: mount, parent: nil, anchor: anchor)]])
            }
            .backgroundPreferenceValue(ProbeAnchors.self) { groups in
                GeometryReader { proxy in
                    Color.clear
                        .onReceive(session.captureRequests) { value in
                            guard session.wantsCapture(token: value) else { return }
                            let anchors = groups[session.sessionID, default: []]
                            guard anchors.count <= 256 else {
                                session.fail(.resourceLimit, token: value)
                                return
                            }
                            let selected = anchors.filter {
                                session.configuration.selects($0.id)
                            }
                            // A retained mount reference may already have renewed
                            // while SwiftUI still holds an older preference value.
                            // Refuse that old anchor; never label it with the new ID.
                            guard selected.allSatisfy({ $0.mount.id == $0.mountID }) else {
                                session.fail(.staleMount, token: value)
                                return
                            }
                            // Resolve values only during this explicit request. Do not
                            // retain the GeometryProxy or anchors in the session.
                            let markers = selected.map {
                                ProbeMarker(id: $0.id, mountID: $0.mountID,
                                    parent: $0.parent, bounds: proxy[$0.anchor])
                            }
                            session.resolve(token: value, rootMountID: mount.id, markers: markers,
                                globalFrame: proxy.frame(in: .global), displayScale: displayScale)
                        }
                }
                .allowsHitTesting(false)
                .accessibilityHidden(true)
            }
            .onAppear { session.attach(rootMountID: mount.id) }
            .onDisappear {
                session.detach(rootMountID: mount.id)
                mount.renew()
            }
    }
}

public extension View {
    /// Mark one existing layout boundary. The parent is an explicit semantic
    /// relationship, not an inferred complete SwiftUI hierarchy.
    func blueprintMarker(_ id: String, parent: String) -> some View {
        modifier(ProbeMarkerModifier(identity: MarkerIdentity(id: id, parent: parent)))
    }

    /// Install one request-owned collector on the existing selected root.
    /// The root marker is automatic and must not be repeated by the caller.
    func blueprintProbeRoot(session: BlueprintProbeSession) -> some View {
        modifier(ProbeRootModifier(session: session))
    }
}
#endif
