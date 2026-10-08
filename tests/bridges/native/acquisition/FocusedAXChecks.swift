import Foundation
import ApplicationServices

@main struct FocusedAXChecks {
 @MainActor static func main() throws {
  guard CommandLine.arguments.count==3 else{exit(2)}
  let profile=try JSONSerialization.jsonObject(with:Data(contentsOf:URL(fileURLWithPath:CommandLine.arguments[1])))
  let output=URL(fileURLWithPath:CommandLine.arguments[2],isDirectory:true)
  var checks=0, focusedID=1, secretValueCalls=0, switchDuringRead=false
  func expect(_ value:Bool){precondition(value);checks+=1}
  var config:[String:Any]=["collection":"focused-ax","process":["pid":123,"bundle_id":"ordinary.reader","launch_time":100.0],"scope_id":"selected-window","acquisition_limits":profile]
  let context:[String:Any]=["schema_version":"0.1.0","session_id":"ax-session","target":["id":"macos-pid-123","generation":"123:100.0"],"surfaces":[["id":"ax-focused-read-1","generation":"read-1"]],"scope_id":"selected-window","projection":"design","fields":["role","value","enabled","accessibility_bounds"],"plugin":["id":"macos","version":"0.1.0"],"environment_revision":"e1"]
  let request:[String:Any]=["clock_domain":"worker-clock","request_id":"read-1","context":context,"limits":["max_elements":160,"max_depth":9,"max_output_bytes":524288,"deadline_ms":1000],"freshness_policy":"current_required","operation":["operation":"observe","channels":["external_semantics"]]]
  var bytes=Data(repeating:0,count:64);bytes.replaceSubrange(0..<8,with:Data("UIBHST01".utf8));bytes[8]=3;bytes[9]=8
  for (offset,value) in [(16,UInt64(1)),(24,1),(32,1),(40,1),(48,1000)]{for i in 0..<8{bytes[offset+i]=UInt8(truncatingIfNeeded:value>>(i*8))}}
  let header=try NativeControl(bytes)
  func command(_ cfg:[String:Any],req:[String:Any]=request,cap:Int=524288)throws->NativeFocusedAX.Command{
   let wire = try JSONSerialization.data(withJSONObject:["schema_version":"0.1.0","artifact":["kind":"request","data":req]])
   let document = try JSONSerialization.jsonObject(with:wire) as! [String:Any]
   return try NativeFocusedAX.command(NativeInbound(configurationBytes:JSONSerialization.data(withJSONObject:cfg),document:document,control:header,replyCap:cap,deadline:ProcessInfo.processInfo.systemUptime+2,started:ProcessInfo.processInfo.systemUptime))
  }
  let access=NativeAXAccess(prepare:{_ in},attribute:{raw,name in
   if name==kAXFocusedWindowAttribute{return (.success,NSNumber(value:focusedID))}
   if name==kAXRoleAttribute{return (.success,"AXWindow" as CFString)}
   return (.noValue,nil)
  },count:{raw,name in(.success,(raw as! NSNumber).intValue==1 ? 2:0)},page:{raw,name,index,count in
   let values=(raw as! NSNumber).intValue==1 ? [2,3]:[]
   return (.success,Array(values[index..<min(index+count,values.count)]).map{NSNumber(value:$0)} as CFArray)
  },batch:{raw,names in
   let id=(raw as! NSNumber).intValue
   if switchDuringRead && id==2{focusedID=4}
   return (.success,names.map{name->Any in
    switch name{
    case kAXRoleAttribute:return id==1 ? "AXWindow":id==3 ? "AXTextField":"AXStaticText"
    case kAXSubroleAttribute:return id==3 ? "AXSecureTextField":""
    case kAXIdentifierAttribute:return id==3 ? "protected":"public"
    case kAXEnabledAttribute:return NSNumber(value:true)
    case kAXValueAttribute:if id==3{secretValueCalls+=1};return "public content"
    default:return NSNull()
    }
   } as CFArray)
  },actions:{_ in(.success,[] as CFArray)},isElement:{CFGetTypeID($0)==CFNumberGetTypeID()})
  func run(_ name:String,current:Bool=true,owner:Int32?=123,permission:Bool=true,cap:Int=524288)throws->[String:Any]{
   let frame=try NativeFocusedAX.collect(command(config,cap:cap),access:access,processCurrent:{_ in current},application:{_ in NSNumber(value:0)},windowOwner:{_ in owner},permitted:{permission})
   let data=frame.bytes{Data($0)};try data.write(to:output.appendingPathComponent(name+".json"),options:.withoutOverwriting)
   return (((try JSONSerialization.jsonObject(with:data) as! [String:Any])["artifact"] as! [String:Any])["data"] as! [String:Any])["result"] as! [String:Any]
  }
  let positive=try run("ordinary")
  expect(positive["status"] as? String=="observed")
  let snapshot=positive["data"] as! [String:Any]
  expect((snapshot["nodes"] as! [Any]).count==3 && (snapshot["captures"] as! [Any]).isEmpty)
  expect((snapshot["surface_records"] as! [[String:Any]])[0]["evidence"] is [String:Any])
  let serialized = try JSONSerialization.data(withJSONObject:snapshot)
  expect(secretValueCalls==0 && String(data:serialized,encoding:.utf8)!.contains("redacted"))
  for (name,current,owner,permission,code) in [("stale",false,Optional(Int32(123)),true,"stale_target"),("owner",true,Optional(Int32(999)),true,"target_unresolved"),("permission",true,Optional(Int32(123)),false,"permission_required")]{
   let r=try run(name,current:current,owner:owner,permission:permission)
   expect(r["status"] as? String=="failed" && (r["data"] as! [String:Any])["code"] as? String==code)
  }
  switchDuringRead=true
  let changed=try run("changed")
  expect((changed["data"] as! [String:Any])["code"] as? String=="stale_target")
  switchDuringRead=false;focusedID=1
  let limited=try run("limited",cap:512)
  expect(limited["status"] as? String=="failed")
  config["identity_path"]="/forbidden/fixture.json"
  do{_=try command(config);preconditionFailure("unknown config key")}catch{checks+=1}
  config.removeValue(forKey:"identity_path")
  var bad=request;var badContext=context;badContext["surfaces"]=[["id":"window-789","generation":"g1"]];bad["context"]=badContext
  do{_=try command(config,req:bad);preconditionFailure("false CG binding")}catch{checks+=1}
  bad=request;bad["operation"]=["operation":"observe","channels":["rendered_capture"]]
  do{_=try command(config,req:bad);preconditionFailure("capture not enabled")}catch{checks+=1}
  print("{\"checks\":\(checks),\"ordinary_ax\":true,\"fixture_files\":false,\"live_sdk\":false}")
 }
}
