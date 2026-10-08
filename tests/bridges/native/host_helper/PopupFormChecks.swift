import Foundation
@main struct PopupFormChecks {
    static func main() throws {
        let profile = try JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1])))
        let parent: [String: Any] = ["pid": 123, "bundle_id": "local.uiblueprint.f02.off", "launch_time": 100.0,
            "window_id": 456, "window_identifier": "a", "target_generation": "g1", "surface_generation": "w1"]
        var popup = parent
        popup["window_id"] = 789; popup["window_identifier"] = "popup-a"; popup["surface_generation"] = "p1"
        let ordinary: [String: Any] = ["binding": parent, "identity_path": "/tmp/parent-identity", "scope_id": "form-1",
            "collection": "form", "form_identifiers": ["f02.name"], "form_session_ms": 1000, "acquisition_limits": profile]
        var paired = ordinary
        paired["binding"] = popup; paired["identity_path"] = "/tmp/popup-identity"
        paired["parent_binding"] = parent; paired["parent_identity_path"] = "/tmp/parent-identity"
        paired["form_identifiers"] = ["f02.popup.confirm"]; paired["parent_form_identifiers"] = ["f02.popup", "f02.result"]
        func decode(_ value: [String: Any]) throws -> NativeConfiguration {
            try NativeConfiguration.decode(JSONSerialization.data(withJSONObject: value))
        }
        _ = try decode(ordinary); _ = try decode(paired)
        var refused = 0
        func reject(_ mutate: (inout [String: Any]) -> Void) {
            var bad = paired; mutate(&bad)
            do { _ = try decode(bad); preconditionFailure("invalid popup authority admitted") }
            catch { refused += 1 }
        }
        reject { $0.removeValue(forKey: "parent_binding") }
        reject { $0.removeValue(forKey: "parent_identity_path") }
        reject { $0.removeValue(forKey: "parent_form_identifiers") }
        reject { $0["parent_form_identifiers"] = ["f02.result"] }
        reject { $0["parent_form_identifiers"] = ["f02.popup", "f02.popup"] }
        reject { $0["parent_form_identifiers"] = ["f02.popup", String(repeating: "a", count: 257)] }
        reject { $0["parent_form_identifiers"] = ["f02.popup"] + (0..<7).map { "id-\($0)" } }
        for (field, value) in [("pid", 124 as Any), ("launch_time", 101.0), ("window_id", 789),
                                ("window_identifier", "b"), ("surface_generation", "")] {
            reject { bad in var p = parent; p[field] = value; bad["parent_binding"] = p }
        }
        reject { $0["protected_input"] = ["reference": "once", "action_id": "step", "identifier": "f02.popup.confirm", "path": "/tmp/input", "trace": false] }
        reject { $0["collection"] = "window-ax" }
        var secure = ordinary
        secure["form_identifiers"] = ["f02.secret", "f02.secret-status"]
        secure["protected_input"] = ["reference": "once", "action_id": "step", "identifier": "f02.secret", "path": "/tmp/input", "trace": false]
        _ = try decode(secure) // unchanged single-Surface V02 configuration
        print("{\"positive_configurations\":3,\"popup_refusals\":\(refused),\"sdk_input\":false}")
    }
}
