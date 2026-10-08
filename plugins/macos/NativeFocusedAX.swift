import Foundation
import AppKit
import ApplicationServices

// Ordinary read-only AX window selection. No CG identity, fixture metadata,
// capture, mutation or reusable action handles are implied by this one reply.
enum NativeFocusedAX {
    private enum SelectionError: Error { case changed }
    struct Process: Decodable {
        let pid: Int32
        let bundle_id: String
        let launch_time: Double
        var generation: String { "\(pid):\(launch_time)" }
    }
    struct Configuration: Decodable {
        let collection: String
        let process: Process
        let scope_id: String
        let acquisition_limits: NativeAcquisitionLimits
    }
    struct Command {
        let configuration: Configuration
        let request: [String: Any]
        let control: NativeControl
        let replyCap: Int
        let deadline: Double
    }
    static func isConfiguration(_ data: Data) throws -> Bool {
        guard let object = try JSONSerialization.jsonObject(with:data) as? [String:Any] else { throw NativeProtocolError.configuration }
        return object["collection"] as? String == "focused-ax"
    }
    static func command(_ input: NativeInbound) throws -> Command {
        guard let object = try JSONSerialization.jsonObject(with:input.configurationBytes) as? [String:Any],
              Set(object.keys) == ["collection","process","scope_id","acquisition_limits"],
              let process = object["process"] as? [String:Any], Set(process.keys) == ["pid","bundle_id","launch_time"]
        else { throw NativeProtocolError.configuration }
        let config = try JSONDecoder().decode(Configuration.self,from:input.configurationBytes)
        try config.acquisition_limits.validate()
        guard config.collection == "focused-ax", config.process.pid > 0,
              !config.process.bundle_id.isEmpty, config.process.launch_time.isFinite, config.process.launch_time > 0,
              !config.scope_id.isEmpty, input.control.channel == 0,
              input.document["schema_version"] as? String == "0.1.0",
              let artifact = input.document["artifact"] as? [String:Any], artifact["kind"] as? String == "request",
              let request = artifact["data"] as? [String:Any], let rid = request["request_id"] as? String, !rid.isEmpty,
              let clock = request["clock_domain"] as? String, !clock.isEmpty,
              let context = request["context"] as? [String:Any], context["scope_id"] as? String == config.scope_id,
              let session = context["session_id"] as? String, !session.isEmpty,
              let target = context["target"] as? [String:Any], target["id"] as? String == "macos-pid-\(config.process.pid)",
              target["generation"] as? String == config.process.generation,
              let surfaces = context["surfaces"] as? [[String:Any]], surfaces.count == 1,
              surfaces[0]["id"] as? String == "ax-focused-\(rid)", surfaces[0]["generation"] as? String == rid,
              let plugin = context["plugin"] as? [String:Any], plugin["id"] as? String == "macos",
              let fields = context["fields"] as? [String], !fields.isEmpty, Set(fields).count == fields.count,
              Set(fields).isSubset(of:["role","accessibility_name","description","value","placeholder","enabled","focused","actions","accessibility_bounds"]),
              request["freshness_policy"] as? String == "current_required",
              let operation = request["operation"] as? [String:Any], operation["operation"] as? String == "observe",
              operation["channels"] as? [String] == ["external_semantics"],
              let limits = request["limits"] as? [String:Any], let nodes = limits["max_elements"] as? Int, (1...160).contains(nodes),
              let depth = limits["max_depth"] as? Int, (1...9).contains(depth),
              let output = limits["max_output_bytes"] as? Int, output > 1,
              let duration = limits["deadline_ms"] as? Double, duration.isFinite, duration > 0
        else { throw NativeProtocolError.request }
        let deadline = min(input.deadline,input.started + duration/1000)
        try NativeDescriptorIO.check(deadline)
        return Command(configuration:config,request:request,control:input.control,
            replyCap:min(input.replyCap,output),deadline:deadline)
    }
    @MainActor static func collect(_ command: Command, access: NativeAXAccess = .live,
        processCurrent: (Process) -> Bool = { p in
            guard let app = NSRunningApplication(processIdentifier:p.pid) else { return false }
            return app.bundleIdentifier == p.bundle_id && app.launchDate?.timeIntervalSince1970 == p.launch_time
        }, application: (Int32) -> CFTypeRef = { AXUIElementCreateApplication($0) },
        windowOwner: (CFTypeRef) -> Int32? = { raw in
            guard CFGetTypeID(raw) == AXUIElementGetTypeID() else { return nil }
            var pid: pid_t = 0
            return AXUIElementGetPid(unsafeDowncast(raw,to:AXUIElement.self),&pid) == .success ? pid : nil
        }, permitted: () -> Bool = { AXIsProcessTrusted() }) throws -> NativeJSONFrame {
        let config = command.configuration, request = command.request
        let context = request["context"] as! [String:Any], target = context["target"] as! [String:Any]
        let surface = (context["surfaces"] as! [[String:Any]])[0], fields = context["fields"] as! [String]
        let limits = request["limits"] as! [String:Any], scope = context["scope_id"] as! String
        let rid = request["request_id"] as! String, started = ProcessInfo.processInfo.systemUptime
        let admission = try NativeAcquisition(config.acquisition_limits,deadline:command.deadline)
        let json = NativeJSON(config.acquisition_limits), frame = try NativeJSONFrame(capacity:command.replyCap,deadline:command.deadline)
        func failure(_ code:String) throws -> [String:Any] {
            try json.response(request:request,ticket:command.control.ticket,channel:"external_semantics") { try json.failure(code,scope:scope,channel:"external_semantics") }
        }
        let unresolved = try failure("target_unresolved"), stale = try failure("stale_target")
        let incomplete = try failure("incomplete_scope"), permission = try failure("permission_required")
        guard processCurrent(config.process) else { try frame.encode(stale); return frame }
        guard permitted() else { try frame.encode(permission); return frame }
        let app = application(config.process.pid)
        func focused() throws -> CFTypeRef {
            guard let window = try nativeAXAttribute(app,kAXFocusedWindowAttribute,admission:admission,access:access),
                  access.isElement(window), windowOwner(window) == config.process.pid,
                  let raw = try nativeAXAttribute(window,kAXRoleAttribute,admission:admission,access:access),
                  try admission.text(raw) == kAXWindowRole
            else { throw NativeAcquisitionError.invalidValue }
            return window
        }
        let window: CFTypeRef
        do { window = try focused() } catch { try frame.encode(unresolved); return frame }
        func same() throws {
            try admission.check()
            guard processCurrent(config.process), CFEqual(window,try focused()) else { throw SelectionError.changed }
        }
        do {
            try same()
            let oid = rid + "-external_semantics"
            let collected = try collectWindowAX(window,surface:surface,observationID:oid,
                maxNodes:limits["max_elements"] as! Int,maxDepth:limits["max_depth"] as! Int,
                deadline:command.deadline,admission:admission,fields:fields,json:json,access:access)
            try same()
            var snapshot = try Collector.snapshot(context:context,surface:surface,target:target,scope:scope,
                fields:fields,observation:oid,channel:"external_semantics",started:started,
                nodes:collected.nodes,captures:try json.array{[]},json:json)
            snapshot["surface_records"] = try json.array { [try json.object(["identity","native_owner","initiated_by","anchor","evidence"]) {
                ["identity":try json.borrowed(surface),"native_owner":try json.known("identity",target),
                 "initiated_by":try json.scalar(NSNull()),"anchor":try json.scalar(NSNull()),
                 "evidence":try json.evidence(oid,"macos.ax","public_process_incarnation_and_AXFocusedWindow")]
            }] }
            try frame.encode(json.response(request:request,ticket:command.control.ticket,channel:"external_semantics") {
                try json.object(["status","data"]) { ["status":try json.scalar("observed"),"data":snapshot] }
            })
            do { try same() } catch { frame.reset(); try frame.encode(stale) }
        } catch {
            frame.reset()
            // Deadline/shape limits do not establish identity loss. The parent
            // keeps its authoritative timeout; no output is sent after expiry.
            guard processCurrent(config.process) else { try frame.encode(stale); return frame }
            if error is SelectionError { try frame.encode(stale) }
            else if let e = error as? NativeAcquisitionError, e == .limit { try frame.encode(incomplete) }
            else { try frame.encode(unresolved) }
        }
        return frame
    }
}
