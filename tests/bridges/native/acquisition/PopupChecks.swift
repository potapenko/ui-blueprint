import Foundation
import ApplicationServices
import CoreGraphics
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
        func config(_ generation:String, budgets:[String:Int]=[:], artifact:String?=nil)throws->NativeConfiguration{
            let admitted=(profile as! [String:Int]).merging(budgets,uniquingKeysWith:{_,b in b})
            var data:[String:Any]=["binding":binding(window:789,key:"popup-a",generation:generation),"parent_binding":binding(window:456,key:"a",generation:parentGen),
                "identity_path":dir.appendingPathComponent("popup-a-identity.json").path,"parent_identity_path":dir.appendingPathComponent("a-identity.json").path,
                "scope_id":"popup-scope","collection":"popup-ax","acquisition_limits":admitted]
            if let artifact { data["artifact_directory"]=artifact; data["pixel_policy"]="owned_synthetic_fixture" }
            let bytes=try JSONSerialization.data(withJSONObject:data);expect(bytes.count<=4032);return try NativeConfiguration.decode(bytes)
        }
        let fields=["role","accessibility_name","placeholder","focused","enabled"]
        func command(_ generation:String,budgets:[String:Int]=[:],cap:Int=524288,channel:UInt8=0,artifact:String?=nil)throws->NativeCommand{
            let context:[String:Any]=["schema_version":"0.1.0","session_id":"popup-session","target":["id":"f02-pid-123","generation":"123:100.0"],
                "surfaces":[["id":"window-789","generation":generation],["id":"window-456","generation":parentGen]],"scope_id":"popup-scope","projection":"interaction","fields":fields,
                "plugin":["id":"macos","version":"0.1.0"],"environment_revision":"e1"]
            let req:[String:Any]=["clock_domain":"worker-clock","request_id":"popup-request","context":context,
                "limits":["max_elements":160,"max_depth":9,"max_output_bytes":524288,"deadline_ms":1000],"freshness_policy":"current_required",
                "operation":["operation":"observe","channels":[channel==1 ? "rendered_capture":"external_semantics"]]]
            var header=Data(repeating:0,count:64);header.replaceSubrange(0..<8,with:Data("UIBHST01".utf8));header[8]=3;header[9]=8;header[10]=channel
            for (offset,value) in [(16,UInt64(1)),(24,1),(32,1),(40,1),(48,1000)]{for i in 0..<8{header[offset+i]=UInt8(truncatingIfNeeded:value>>(i*8))}}
            return NativeCommand(configuration:try config(generation,budgets:budgets,artifact:artifact),document:["schema_version":"0.1.0","artifact":["kind":"request","data":req]],control:try NativeControl(header),replyCap:cap,deadline:ProcessInfo.processInfo.systemUptime+5)
        }
        let ids=[1:"a",2:"popup-a",3:"f02.popup",4:"f02.popup.owner.a",5:"f02.popup.confirm"]
        let children=[0:[1],1:[3,2],2:[4,5],3:[],4:[],5:[]]
        let access=NativeAXAccess(prepare:{_ in},attribute:{raw,name in
            let i=(raw as! NSNumber).intValue
            if name==kAXRoleAttribute{return (.success,(i==2 ? "AXPopover":"AXGroup") as CFString)}
            return name==kAXIdentifierAttribute ? (.success,ids[i]! as CFString):(.noValue,nil)
        },count:{raw,name in (.success,children[(raw as! NSNumber).intValue]!.count)},page:{raw,name,index,amount in
            let c=children[(raw as! NSNumber).intValue]!;return (.success,c[index..<min(c.count,index+amount)].map{NSNumber(value:$0)} as CFArray)
        },batch:{raw,names in
            let i=(raw as! NSNumber).intValue
            return (.success,names.map{name->Any in
                switch name{
                case kAXRoleAttribute:return i==2 ? "AXPopover":(i==3 || i==5 ? "AXButton":"AXGroup")
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
        // Physical popup789 is intentionally absent from AXWindows. Its reported
        // AXPopover lives under parent456; actual scoped resolver must not infer
        // physical ownership from that semantic ancestry.
        let scoped=try Collector.resolvePopover(NSNumber(value:1),ownerIdentifier:"f02.popup.owner.a",maxNodes:160,maxDepth:9,
            admission:NativeAcquisition(try config(popupGen).acquisition_limits,deadline:ProcessInfo.processInfo.systemUptime+5),access:access)
        expect(CFEqual(scoped,NSNumber(value:2)))
        let duplicate=NativeAXAccess(prepare:access.prepare,attribute:{raw,name in
            if (raw as! NSNumber).intValue==5 && name==kAXIdentifierAttribute{return (.success,"f02.popup.owner.a" as CFString)}
            return access.attribute(raw,name)
        },count:access.count,page:access.page,batch:access.batch,actions:access.actions,isElement:access.isElement)
        for (name,provider,nodes) in [("duplicate_marker",duplicate,160),("truncated_lookup",access,2)]{
            do{
                _=try Collector.resolvePopover(NSNumber(value:1),ownerIdentifier:"f02.popup.owner.a",maxNodes:nodes,maxDepth:9,
                    admission:NativeAcquisition(try config(popupGen).acquisition_limits,deadline:ProcessInfo.processInfo.systemUptime+5),access:provider)
                preconditionFailure("ambiguous/partial lookup cannot resolve")
            }catch{checks+=1}
            _=name
        }
        do{
            _=try Collector.resolvePopover(NSNumber(value:1),ownerIdentifier:"f02.popup.owner.b",maxNodes:160,maxDepth:9,
                admission:NativeAcquisition(try config(popupGen).acquisition_limits,deadline:ProcessInfo.processInfo.systemUptime+5),access:access)
            preconditionFailure("wrong popup cannot resolve")
        }catch{checks+=1}
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
        // Existing public capture call is substituted only inside this synthetic
        // owner check. No helper configuration, runtime flag or backend is added.
        let raw=Data(repeating:128,count:24)
        let image=CGImage(width:3,height:2,bitsPerComponent:8,bitsPerPixel:32,bytesPerRow:12,
            space:CGColorSpaceCreateDeviceRGB(),bitmapInfo:CGBitmapInfo(rawValue:CGImageAlphaInfo.premultipliedFirst.rawValue).union(.byteOrder32Little),
            provider:CGDataProvider(data:raw as CFData)!,decode:nil,shouldInterpolate:false,intent:.defaultIntent)!
        let captured=OwnedCapture(image:image,windowFrame:CGRect(x:0,y:0,width:3,height:2),filterRect:CGRect(x:0,y:0,width:3,height:2),scale:1,admissionWait:0)
        var captureCalls=0
        func runCapture(_ name:String,generation:String?=nil,cap:Int=524288,
            provider: @MainActor (UInt32,Int32,Double,NativeAcquisition) async throws->OwnedCapture)async throws->[String:Any]{
            let frame=try await Collector.popup(command:command(generation ?? popupGen,cap:cap,channel:1,artifact:dir.appendingPathComponent("images-"+name).path),
                access:access,publicBinding:{b in [456,789].contains(b.window_id)},application:{_ in NSNumber(value:0)},capture:provider)
            let bytes=frame.bytes{Data($0)}
            try bytes.write(to:out.appendingPathComponent(name+".json"),options:.withoutOverwriting)
            return (((try JSONSerialization.jsonObject(with:bytes) as! [String:Any])["artifact"] as! [String:Any])["data"] as! [String:Any])["result"] as! [String:Any]
        }
        let pixelResult=try await runCapture("popup_capture"){window,pid,budget,admission in
            expect(window==789 && pid==123 && budget>0 && budget<=2);captureCalls+=1;return captured
        }
        expect(pixelResult["status"] as? String=="observed")
        let pixelSnapshot=pixelResult["data"] as! [String:Any],pixelRecord=(pixelSnapshot["captures"] as! [[String:Any]])[0]
        let pixelContext=pixelSnapshot["context"] as! [String:Any],pixelSurfaces=pixelContext["surfaces"] as! [[String:Any]]
        expect((pixelRecord["capture_target"] as! NSDictionary).isEqual(pixelSurfaces[0]))
        expect((pixelRecord["included_surfaces"] as! NSArray).isEqual([pixelSurfaces[0]]))
        expect((pixelRecord["excluded_surfaces"] as! NSArray).isEqual([pixelSurfaces[1]]))
        expect((pixelRecord["unresolved_surfaces"] as! [Any]).isEmpty && pixelRecord["surface_coverage"] as? String=="partial")
        expect(pixelRecord["capture_kind"] as? String=="window_isolated" && pixelRecord["captures_audio"] as? Bool==false)
        expect((pixelRecord["crop_transform"] as! [String:Any])["status"] as? String=="unknown")
        expect((pixelSnapshot["nodes"] as! [Any]).isEmpty && (pixelSnapshot["relations"] as! [Any]).isEmpty)
        expect((pixelSnapshot["surface_records"] as! [[String:Any]])[0]["anchor"] is NSNull)
        let retained=dir.appendingPathComponent("images-popup_capture/capture")
        expect(try FileManager.default.contentsOfDirectory(atPath:retained.path).count==2)
        try NativeCurrentIdentity.verify(path:dir.appendingPathComponent("popup-a-identity.json").path,
            expected:binding(window:789,key:"popup-a",generation:popupGen));checks+=1
        let stalePixel=try await runCapture("capture_stale",generation:"old-generation"){_,_,_,_ in captureCalls+=1;return captured}
        expect((stalePixel["data"] as! [String:Any])["code"] as? String=="stale_target" && captureCalls==1)
        for (name,error,code) in [("capture_permission",OwnedCaptureError.permissionRequired,"permission_required"),
            ("capture_timeout",OwnedCaptureError.timeout("screenshot"),"timeout"),("capture_cancelled",OwnedCaptureError.cancelled,"interrupted"),("capture_unresolved",OwnedCaptureError.targetUnresolved,"target_unresolved")]{
            let failed=try await runCapture(name){_,_,_,_ in throw error}
            expect(failed["status"] as? String=="failed" && (failed["data"] as! [String:Any])["code"] as? String==code)
        }
        let limited=try await runCapture("capture_low_output",cap:512){_,_,_,_ in captured}
        expect(limited["status"] as? String=="failed" && (limited["data"] as! [String:Any])["code"] as? String=="incomplete_scope")
        var publicationChecks=0
        let publicationRoot=dir.appendingPathComponent("images-capture_publication_race")
        let publicationFrame=try await Collector.popup(command:command(popupGen,channel:1,artifact:publicationRoot.path),access:access,
            publicBinding:{b in
                if b.window_id==789 {
                    publicationChecks+=1
                    if publicationChecks==3 {
                        try! popup.close(pid:123,bundle:"local.uiblueprint.f02.off",launch:100,window:789)
                        return false
                    }
                }
                return [456,789].contains(b.window_id)
            },application:{_ in NSNumber(value:0)},capture:{_,_,_,_ in captured})
        let publicationBytes=publicationFrame.bytes{Data($0)}
        let publicationResult=((((try JSONSerialization.jsonObject(with:publicationBytes) as! [String:Any])["artifact"] as! [String:Any])["data"] as! [String:Any])["result"] as! [String:Any])
        expect(publicationResult["status"] as? String=="failed" && (publicationResult["data"] as! [String:Any])["code"] as? String=="stale_target")
        expect(try FileManager.default.contentsOfDirectory(atPath:publicationRoot.appendingPathComponent("capture").path).count==2)
        expect((publicationResult["data"] as! [String:Any])["captures"]==nil)
        try publicationBytes.write(to:out.appendingPathComponent("capture_publication_race.json"),options:.withoutOverwriting)
        popupGen=try popup.snapshot(pid:123,bundle:"local.uiblueprint.f02.off",launch:100,window:789)
        let raced=try await runCapture("capture_identity_race"){_,_,_,_ in
            try popup.close(pid:123,bundle:"local.uiblueprint.f02.off",launch:100,window:789);return captured
        }
        expect(raced["status"] as? String=="failed" && (raced["data"] as! [String:Any])["code"] as? String=="stale_target")
        expect(!FileManager.default.fileExists(atPath:dir.appendingPathComponent("images-capture_identity_race/capture/capture.png").path))
        print("{\"checks\":\(checks),\"actual_popup_collector_owner\":true,\"live_sdk\":false}")
    }
}
