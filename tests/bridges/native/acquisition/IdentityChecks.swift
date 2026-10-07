import Foundation
import Darwin

@main struct IdentityChecks {
    @MainActor static func main() throws {
        guard CommandLine.arguments.count==3 else {exit(2)}
        let dir=URL(fileURLWithPath:CommandLine.arguments[1],isDirectory:true)
        let limits=try JSONDecoder().decode(NativeAcquisitionLimits.self,from:Data(contentsOf:URL(fileURLWithPath:CommandLine.arguments[2])))
        let owner=FixtureIdentity(directory:dir,windowKey:"a");let path=dir.appendingPathComponent("a-identity.json")
        var checks=0
        func expect(_ value:Bool){precondition(value);checks+=1}
        func refuses(_ block:()throws->Void){do{try block();fatalError("expected refusal")}catch{checks+=1}}
        func binding(_ generation:String)->[String:Any]{["pid":123,"bundle_id":"local.uiblueprint.f02.off","launch_time":100.0,"window_id":456,"window_identifier":"a","target_generation":"123:100.0","surface_generation":generation,"identity_path":path.path]}
        refuses{try NativeCurrentIdentity.verify(path:path.path,expected:binding("absent"))}
        let first=try owner.snapshot(pid:123,bundle:"local.uiblueprint.f02.off",launch:100,window:456)
        let old=binding(first);try NativeCurrentIdentity.verify(path:path.path,expected:old);checks+=1
        // Last measured Snapshot need not change when identity invalidates.
        let measured=Data("unchanged Snapshot payload".utf8);try measured.write(to:dir.appendingPathComponent("a.json"))
        try owner.close(pid:123,bundle:"local.uiblueprint.f02.off",launch:100,window:456)
        expect(owner.generation != first)
        refuses{try NativeCurrentIdentity.verify(path:path.path,expected:old)}
        expect(try Data(contentsOf:dir.appendingPathComponent("a.json"))==measured)
        // Reopening same CGWindowID performs no Snapshot: closed file remains.
        refuses{try NativeCurrentIdentity.verify(path:path.path,expected:binding(owner.generation))}
        let fresh=try owner.snapshot(pid:123,bundle:"local.uiblueprint.f02.off",launch:100,window:456)
        expect(fresh != first);refuses{try NativeCurrentIdentity.verify(path:path.path,expected:old)}
        try NativeCurrentIdentity.verify(path:path.path,expected:binding(fresh));checks+=1
        var wrong=binding(fresh);wrong["window_identifier"]="b";refuses{try NativeCurrentIdentity.verify(path:path.path,expected:wrong)}
        wrong=binding(fresh);wrong["launch_time"]=101.0;refuses{try NativeCurrentIdentity.verify(path:path.path,expected:wrong)}
        let json=NativeJSON(limits);let frame=try NativeJSONFrame(capacity:524288,deadline:ProcessInfo.processInfo.systemUptime+5)
        let observed:[String:Any]=["status":"observed"]
        let stale:[String:Any]=["status":"failed","data":["code":"stale_target"]]
        try frame.encode(observed)
        try owner.close(pid:123,bundle:"local.uiblueprint.f02.off",launch:100,window:456)
        var received:Data?
        try Collector.sendCurrentIdentity(frame,stale:stale,manifest:binding(fresh)){received=$0.bytes{Data($0)}}
        expect((try JSONSerialization.jsonObject(with:received!) as! NSDictionary).isEqual(stale as NSDictionary))
        try Data("{".utf8).write(to:path);refuses{try NativeCurrentIdentity.verify(path:path.path,expected:old)}
        try Data(repeating:32,count:4033).write(to:path);refuses{try NativeCurrentIdentity.verify(path:path.path,expected:old)}
        try FileManager.default.removeItem(at:path);try FileManager.default.createSymbolicLink(at:path,withDestinationURL:dir.appendingPathComponent("a.json"))
        refuses{try NativeCurrentIdentity.verify(path:path.path,expected:old)}
        _=json // shared builder type compiled with the actual publication owner
        print("{\"checks\":\(checks),\"real_lifecycle_owner\":true,\"live_ui\":false}")
    }
}
