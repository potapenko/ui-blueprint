import Foundation
import ApplicationServices
import Darwin

@main struct PopupChecks {
    @MainActor static func main() async throws {
        guard CommandLine.arguments.count==4 else{exit(2)}
        let dir=URL(fileURLWithPath:CommandLine.arguments[1],isDirectory:true)
        let profile=try JSONSerialization.jsonObject(with:Data(contentsOf:URL(fileURLWithPath:CommandLine.arguments[2])))
        let out=URL(fileURLWithPath:CommandLine.arguments[3],isDirectory:true)
        var checks=0
        func expect(_ b:Bool){precondition(b);checks+=1}
        expect(FixturePopupAttribution.unique([(456,["f02.owner.a"]),(789,["f02.popup.owner.a"])],role:"a")==789)
        expect(FixturePopupAttribution.unique([(789,["f02.popup.owner.a"]),(790,["f02.popup.owner.a"])],role:"a")==nil)
        expect(FixturePopupAttribution.unique([(789,["f02.popup.owner.b"])],role:"a")==nil)
        expect(FixturePopupAttribution.unique([],role:"a")==nil)
        let parent=FixtureIdentity(directory:dir,windowKey:"a"),popup=FixtureIdentity(directory:dir,windowKey:"popup-a")
        let parentGen=try parent.snapshot(pid:123,bundle:"local.uiblueprint.f02.off",launch:100,window:456)
        var popupGen=try popup.snapshot(pid:123,bundle:"local.uiblueprint.f02.off",launch:100,window:789)
        func binding(window:Int,key:String,generation:String)->[String:Any]{["pid":123,"bundle_id":"local.uiblueprint.f02.off","launch_time":100.0,"window_id":window,"window_identifier":key,"target_generation":"123:100.0","surface_generation":generation]}
        func config(_ generation:String, budgets:[String:Int]=[:])throws->NativeConfiguration{
            let admitted=(profile as! [String:Int]).merging(budgets,uniquingKeysWith:{_,b in b})
            let data:[String:Any]=["binding":binding(window:789,key:"popup-a",generation:generation),"parent_binding":binding(window:456,key:"a",generation:parentGen),
                "identity_path":dir.appendingPathComponent("popup-a-identity.json").path,"parent_identity_path":dir.appendingPathComponent("a-identity.json").path,
                "scope_id":"popup-scope","collection":"popup-ax","acquisition_limits":admitted]
            let bytes=try JSONSerialization.data(withJSONObject:data);expect(bytes.count<=4032);return try NativeConfiguration.decode(bytes)
        }
        let fields=["role","accessibility_name","placeholder","focused","enabled"]
        func command(_ generation:String,budgets:[String:Int]=[:],cap:Int=524288)throws->NativeCommand{
            let context:[String:Any]=["schema_version":"0.1.0","session_id":"popup-session","target":["id":"f02-pid-123","generation":"123:100.0"],
                "surfaces":[["id":"window-789","generation":generation],["id":"window-456","generation":parentGen]],"scope_id":"popup-scope","projection":"interaction","fields":fields,
                "plugin":["id":"macos","version":"0.1.0"],"environment_revision":"e1"]
            let req:[String:Any]=["clock_domain":"worker-clock","request_id":"popup-request","context":context,
                "limits":["max_elements":160,"max_depth":9,"max_output_bytes":524288,"deadline_ms":1000],"freshness_policy":"current_required",
                "operation":["operation":"observe","channels":["external_semantics"]]]
            var header=Data(repeating:0,count:64);header.replaceSubrange(0..<8,with:Data("UIBHST01".utf8));header[8]=3;header[9]=8
            for (offset,value) in [(16,UInt64(1)),(24,1),(32,1),(40,1),(48,1000)]{for i in 0..<8{header[offset+i]=UInt8(truncatingIfNeeded:value>>(i*8))}}
            return NativeCommand(configuration:try config(generation,budgets:budgets),document:["schema_version":"0.1.0","artifact":["kind":"request","data":req]],control:try NativeControl(header),replyCap:cap,deadline:ProcessInfo.processInfo.systemUptime+5)
        }
        let ids=[1:"a",2:"popup-a",3:"f02.popup",4:"f02.popup.owner.a",5:"f02.popup.confirm"]
        let children=[0:[1,2],1:[3],2:[4,5],3:[],4:[],5:[]]
        let access=NativeAXAccess(prepare:{_ in},attribute:{raw,name in
            let i=(raw as! NSNumber).intValue
            return name==kAXIdentifierAttribute ? (.success,ids[i]! as CFString):(.noValue,nil)
        },count:{raw,name in (.success,children[(raw as! NSNumber).intValue]!.count)},page:{raw,name,index,amount in
            let c=children[(raw as! NSNumber).intValue]!;return (.success,c[index..<min(c.count,index+amount)].map{NSNumber(value:$0)} as CFArray)
        },batch:{raw,names in
            let i=(raw as! NSNumber).intValue
            return (.success,names.map{name->Any in
                switch name{
                case kAXRoleAttribute:return i==3 || i==5 ? "AXButton":"AXGroup"
                case kAXIdentifierAttribute:return ids[i]!
                case kAXSubroleAttribute:return ""
                case kAXDescriptionAttribute:return i==3 ? "Edge popup":"own popup content"
                case kAXEnabledAttribute:return NSNumber(value:true)
                case kAXFocusedAttribute:return NSNumber(value:false)
                default:return NSNull()
                }
            } as CFArray)
        },actions:{_ in (.success,[] as CFArray)},isElement:{CFGetTypeID($0)==CFNumberGetTypeID()})
        func run(_ generation:String,_ name:String)async throws->[String:Any]{
            let frame=try await Collector.popup(command:command(generation),access:access,publicBinding:{b in [456,789].contains(b.window_id)},application:{_ in NSNumber(value:0)})
            let data=frame.bytes{Data($0)};try data.write(to:out.appendingPathComponent(name+".json"),options:.withoutOverwriting)
            return ((try JSONSerialization.jsonObject(with:data) as! [String:Any])["artifact"] as! [String:Any])["data"] as! [String:Any]
        }
        let open=try await run(popupGen,"open");expect((open["result"] as! [String:Any])["status"] as? String=="observed")
        let snap=(open["result"] as! [String:Any])["data"] as! [String:Any]
        expect((snap["surface_records"] as! [[String:Any]]).count==2)
        let anchor=(snap["surface_records"] as! [[String:Any]])[0]["anchor"] as! [String:Any]
        expect((snap["nodes"] as! [[String:Any]]).contains{($0["key"] as! NSDictionary).isEqual(anchor as NSDictionary)})
        expect(((snap["relations"] as! [[String:Any]])[0]["kind"] as? String)=="anchored_to")
        try popup.close(pid:123,bundle:"local.uiblueprint.f02.off",launch:100,window:789)
        let closed=try await run(popupGen,"closed");expect((closed["result"] as! [String:Any])["status"] as? String=="failed")
        let oldGen=popupGen;popupGen=try popup.snapshot(pid:123,bundle:"local.uiblueprint.f02.off",launch:100,window:789)
        expect(popupGen != oldGen)
        let stale=try await run(oldGen,"stale");expect((stale["result"] as! [String:Any])["status"] as? String=="failed")
        let reopened=try await run(popupGen,"reopened");expect((reopened["result"] as! [String:Any])["status"] as? String=="observed")
        for (name,budgets,cap) in [("low_slots",["response_slots":512],524288),
                                    ("low_strings",["response_string_utf8_bytes":2048],524288),
                                    ("low_output",[String:Int](),512)] {
            let frame=try await Collector.popup(command:command(popupGen,budgets:budgets,cap:cap),access:access,
                publicBinding:{b in [456,789].contains(b.window_id)},application:{_ in NSNumber(value:0)})
            let bytes=frame.bytes{Data($0)}
            expect(bytes.count<=cap && bytes.last==10 && bytes.filter{$0==10}.count==1)
            let response=((try JSONSerialization.jsonObject(with:bytes) as! [String:Any])["artifact"] as! [String:Any])["data"] as! [String:Any]
            let result=response["result"] as! [String:Any]
            expect(result["status"] as? String=="failed")
            expect((result["data"] as! [String:Any])["code"] as? String=="incomplete_scope")
            expect(result["nodes"]==nil && (result["data"] as! [String:Any])["nodes"]==nil)
            try bytes.write(to:out.appendingPathComponent(name+".json"),options:.withoutOverwriting)
        }
        let missingTrigger=NativeAXAccess(prepare:access.prepare,attribute:{ raw,name in
            if (raw as! NSNumber).intValue==3 && name==kAXIdentifierAttribute{return (.success,"different-trigger" as CFString)}
            return access.attribute(raw,name)
        },count:access.count,page:access.page,batch:access.batch,actions:access.actions,isElement:access.isElement)
        let missingFrame=try await Collector.popup(command:command(popupGen),access:missingTrigger,
            publicBinding:{b in [456,789].contains(b.window_id)},application:{_ in NSNumber(value:0)})
        let missingBytes=missingFrame.bytes{Data($0)}
        let missingResponse=((try JSONSerialization.jsonObject(with:missingBytes) as! [String:Any])["artifact"] as! [String:Any])["data"] as! [String:Any]
        expect(((missingResponse["result"] as! [String:Any])["data"] as! [String:Any])["code"] as? String=="target_unresolved")
        try missingBytes.write(to:out.appendingPathComponent("missing_trigger.json"),options:.withoutOverwriting)
        do {
            _=try await Collector.popup(command:command(popupGen,cap:2),access:access,
                publicBinding:{b in [456,789].contains(b.window_id)},application:{_ in NSNumber(value:0)})
            preconditionFailure("unencodable fallback must not return a frame")
        } catch {checks+=1}
        let aOwner=FixtureIdentity(directory:dir,windowKey:"b")
        let aGeneration=try aOwner.snapshot(pid:123,bundle:"local.uiblueprint.f02.off",launch:100,window:457)
        try NativeCurrentIdentity.verify(path:dir.appendingPathComponent("b-identity.json").path,
            expected:binding(window:457,key:"b",generation:aGeneration));checks+=1
        let wrong=try await Collector.popup(command:command("wrong-generation"),access:access,publicBinding:{_ in true},application:{_ in NSNumber(value:0)})
        let wrongData=wrong.bytes{Data($0)};expect(((((try JSONSerialization.jsonObject(with:wrongData) as! [String:Any])["artifact"] as! [String:Any])["data"] as! [String:Any])["result"] as! [String:Any])["status"] as? String=="failed")
        print("{\"checks\":\(checks),\"actual_popup_collector_owner\":true,\"live_sdk\":false}")
    }
}
