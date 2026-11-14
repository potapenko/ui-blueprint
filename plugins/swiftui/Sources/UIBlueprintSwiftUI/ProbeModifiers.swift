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
    let parent: String?
    let anchor: Anchor<CGRect>
}

private struct ProbeAnchors: PreferenceKey {
    static var defaultValue: [ProbeAnchor] { [] }
    static func reduce(value: inout [ProbeAnchor], nextValue: () -> [ProbeAnchor]) {
        // Retain one sentinel beyond the limit so overflow cannot look complete.
        let remaining = max(0, 257 - value.count)
        if remaining > 0 { value.append(contentsOf: nextValue().prefix(remaining)) }
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

    func body(content: Content) -> some View {
        content
            .transformAnchorPreference(key: ProbeAnchors.self, value: .bounds) { records, anchor in
                if session != nil, records.count < 257 {
                    records.append(ProbeAnchor(id: identity.id, mountID: mount.id,
                        parent: identity.parent, anchor: anchor))
                }
            }
            .onChange(of: identity) { old, _ in
                session?.markerDetached(id: old.id, mountID: mount.id)
                mount.renew()
            }
            .onDisappear {
                session?.markerDetached(id: identity.id, mountID: mount.id)
                mount.renew()
            }
    }
}

private struct ProbeRootModifier: ViewModifier {
    let session: BlueprintProbeSession
    @State private var mount = ProbeMount()
    @Environment(\.displayScale) private var displayScale

    func body(content: Content) -> some View {
        let token = session.requestRevision
        content
            .environment(\.blueprintProbeSession, session)
            .transformAnchorPreference(key: ProbeAnchors.self, value: .bounds) { records, anchor in
                if records.count < 257 {
                    records.append(ProbeAnchor(id: session.configuration.rootMarkerID,
                        mountID: mount.id, parent: nil, anchor: anchor))
                }
            }
            .backgroundPreferenceValue(ProbeAnchors.self) { anchors in
                GeometryReader { proxy in
                    Color.clear
                        .onChange(of: token, initial: true) { _, value in
                            guard session.wantsCapture(token: value) else { return }
                            let selected = anchors.filter {
                                session.configuration.requiredMarkerIDs.contains($0.id)
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
