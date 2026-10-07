import Foundation
import Darwin

@main struct ProbeChecks {
    @MainActor static func main() throws {
        let args=CommandLine.arguments; guard args.count==4 else {exit(2)}
        let profile=try JSONSerialization.jsonObject(with:Data(contentsOf:URL(fileURLWithPath:args[1])))
        let output=URL(fileURLWithPath:args[2],isDirectory:true)
        // Existing authored oracle is read only by this independent test consumer.
        let oracle=try JSONSerialization.jsonObject(with:Data(contentsOf:URL(fileURLWithPath:args[3]))) as! [String:Any]
        let gaps=oracle["gap_pt"] as! [String:Double]
        var assertions=0
        func check(_ b:Bool){precondition(b);assertions+=1}
        let binding:[String:Any]=["pid":123,"bundle_id":"local.uiblueprint.f02.on","launch_time":100.0,"window_id":456,"window_identifier":"a","target_generation":"g1","surface_generation":"w1"]
        let config:[String:Any]=["binding":binding,"scope_id":"probe-scope","collection":"sample","acquisition_limits":profile,
            "probe_manifest_path":"/tmp/owned-probe-manifest","probe_snapshot_request":1,"probe_source_revision":2,"probe_uptime":20.0]
        let configBytes=try JSONSerialization.data(withJSONObject:config);check(configBytes.count<=4032)
        let configuration=try NativeConfiguration.decode(configBytes)
        let context:[String:Any]=["schema_version":"0.1.0","session_id":"probe-session","target":["id":"f02-pid-123","generation":"g1"],
            "surfaces":[["id":"window-456","generation":"w1"]],"scope_id":"probe-scope","projection":"design","fields":["layout_bounds"],
            "plugin":["id":"macos","version":"0.1.0"],"environment_revision":"e1"]
        let request:[String:Any]=["clock_domain":"uib-worker-11-clock","request_id":"probe-request","context":context,
            "limits":["max_elements":160,"max_depth":9,"max_output_bytes":524288,"deadline_ms":1000],"freshness_policy":"cached_allowed",
            "operation":["operation":"observe","channels":["opt_in_layout_probe"]]]
        var header=Data(repeating:0,count:64);header.replaceSubrange(0..<8,with:Data("UIBHST01".utf8));header[8]=3;header[9]=8;header[10]=2
        for (offset,value) in [(16,UInt64(11)),(24,19),(32,100),(40,7),(48,1000)] {for i in 0..<8{header[offset+i]=UInt8(truncatingIfNeeded:value>>(8*i))}}
        let control=try NativeControl(header)
        func command(_ request:[String:Any]=request,cap:Int=524288)->NativeCommand{
            NativeCommand(configuration:configuration,document:["schema_version":"0.1.0","artifact":["kind":"request","data":request]],
                control:control,replyCap:cap,deadline:ProcessInfo.processInfo.systemUptime+5)
        }
        func sample(_ gap:Double)->[String:Any]{
            binding.merging(["snapshot_request":1,"source_state":["revision":2],"uptime_seconds":20.0,"collection_mode":"explicit_request_only","probe_enabled":true,
                "probe":["source":"swiftui.anchorPreference.explicit_snapshot","provenance":"reported","units":"pt","origin":"top_left","coordinate_space":"fixture_local","screen_transform":"unknown",
                    "layout_bounds":["icon":["x":10.0,"y":10.0,"width":20.0,"height":20.0],"text":["x":30+gap,"y":10.0,"width":80.0,"height":20.0],"container":["x":0.0,"y":0.0,"width":150.0,"height":40.0]]],
                "source_declarations":["logical_component_key":"f02.sample.a","represents":["icon","text","container"]]],uniquingKeysWith:{_,b in b})
        }
        func emit(_ manifest:[String:Any],name:String,request:[String:Any]?=nil) throws -> [String:Any]{
            let frame=try Collector.probe(data:JSONSerialization.data(withJSONObject:manifest),command:command(request ?? requestBase))
            let data=frame.bytes{Data($0)};try data.write(to:output.appendingPathComponent(name+".json"),options:.withoutOverwriting)
            return (try JSONSerialization.jsonObject(with:data) as! [String:Any])["artifact"] as! [String:Any]
        }
        let requestBase=request
        for name in ["baseline","expanded"] {
            let artifact=try emit(sample(gaps[name]!),name:name)
            let response=artifact["data"] as! [String:Any];check(response["dispatch_sequence"] as? Int==7)
            let snapshot=(response["result"] as! [String:Any])["data"] as! [String:Any]
            let nodes=snapshot["nodes"] as! [[String:Any]];check(nodes.count==3)
            let frames=nodes.map { (((($0["properties"] as! [[String:Any]])[0]["state"] as! [String:Any])["value"] as! [String:Any])["value"] as! [String:Any])["shape"] as! [String:Any] }
            let a=frames[0]["value"] as! [String:Double],b=frames[1]["value"] as! [String:Double]
            check(b["x"]!-a["x"]!-a["width"]! == gaps[name]!)
            let observation=(snapshot["observations"] as! [[String:Any]])[0]
            check(observation["start"] as? Double==20 && observation["end"] as? Double==20 && observation["freshness"] as? String=="unverified")
            check((snapshot["components"] as! [[String:Any]])[0]["logical_component_key"] as? String=="f02.sample.a")
        }
        for name in ["off","stale_generation","stale_request","stale_time","missing_marker","current_required"]{
            var manifest=sample(8);var req=request
            switch name{
            case "off":manifest["probe_enabled"]=false
            case "stale_generation":manifest["surface_generation"]="changed"
            case "stale_request":manifest["snapshot_request"]=2
            case "stale_time":manifest["uptime_seconds"]=19.0
            case "missing_marker":var probe=manifest["probe"] as! [String:Any];probe["layout_bounds"]=["icon":["x":0]];manifest["probe"]=probe
            default:req["freshness_policy"]="current_required"
            }
            let artifact=try emit(manifest,name:name,request:req)
            check((artifact["data"] as! [String:Any])["result"] as? [String:Any] != nil)
            check(((artifact["data"] as! [String:Any])["result"] as! [String:Any])["status"] as? String=="failed")
        }
        do{_ = try Collector.probe(data:Data(repeating:32,count:100),command:command(cap:99));fatalError("oversize")}catch{assertions+=1}
        do{_ = try Collector.probe(data:Data("{".utf8),command:command());fatalError("malformed")}catch{assertions+=1}
        print("{\"cases\":10,\"assertions\":\(assertions),\"live\":false}")
    }
}
