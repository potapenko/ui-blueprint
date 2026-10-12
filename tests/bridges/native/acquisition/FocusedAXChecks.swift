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
  checks += try popupChecks(profile: profile, request: request, header: header, output: output)
  checks += try FocusedAXPublicationChecks.run(profile: profile, request: request, header: header, output: output)
  checks += try VisibleChildrenChecks.run(profile: profile, request: request, header: header, output: output)
  checks += try StaticTextChecks.run(profile: profile, request: request, header: header, output: output)
  print("{\"checks\":\(checks),\"ordinary_ax\":true,\"fixture_files\":false,\"live_sdk\":false}")
 }
 @MainActor static func popupChecks(profile: Any, request: [String: Any], header: NativeControl, output: URL) throws -> Int {
  var checks = 0
  func expect(_ value: Bool, line: UInt = #line) { precondition(value, "assertion at line \(line)"); checks += 1 }
  var config: [String: Any] = ["collection": "focused-popover-ax", "process": ["pid": 123, "bundle_id": "ordinary.reader", "launch_time": 100.0], "scope_id": "selected-window", "acquisition_limits": profile]
  var focus = 5, window = 1, parent: [Int: Int] = [:], role: [Int: String] = [:]
  var foreign: Set<Int> = [], contentReads = 0, foreignRoleReads = 0, secretReads = 0, focusReads = 0
  var mutation = "", mutated = false, processAlive = true, roleBudget = false
  func reset() {
   focus = 5; window = 1; parent = [5:4, 6:4, 4:3, 3:2, 2:1]
   role = [1:"AXWindow", 2:"AXGroup", 3:"AXPopover", 4:"AXGroup", 5:"AXButton", 6:"AXTextField", 9:"AXWindow", 90:"AXStaticText"]
   foreign = []; contentReads = 0; foreignRoleReads = 0; secretReads = 0; focusReads = 0
   mutation = ""; mutated = false; processAlive = true; roleBudget = false
   config["acquisition_limits"] = profile
  }
  let children = [1:[2,90], 2:[3], 3:[4], 4:[5,6]]
  let access = NativeAXAccess(prepare: { _ in }, attribute: { raw, name in
   let id = (raw as! NSNumber).intValue
   if name == kAXFocusedWindowAttribute { return (.success, NSNumber(value:window)) }
   if name == kAXFocusedUIElementAttribute {
    focusReads += 1
    // Initial bind + three complete checks, each followed by a final focus read.
    if mutation == "publication-focus" && focusReads == 7 { focus = 6 }
    if mutation == "precollection-focus" && focusReads == 2 { focus = 6 }
    return (.success, NSNumber(value:focus))
   }
   if name == kAXParentAttribute { return parent[id].map { (.success,NSNumber(value:$0)) } ?? (.noValue,nil) }
   if name == kAXRoleAttribute {
    if foreign.contains(id) { foreignRoleReads += 1 }
    if roleBudget && id == 5 { return (.success, String(repeating:"x",count:4097) as CFString) }
    return (.success, (role[id] ?? "AXGroup") as CFString)
   }
   return (.noValue,nil)
  }, count: { raw, _ in (.success,children[(raw as! NSNumber).intValue]?.count ?? 0) }, page: { raw, _, index, count in
   let values = children[(raw as! NSNumber).intValue] ?? []
   return (.success, values[index..<min(index+count,values.count)].map { NSNumber(value:$0) } as CFArray)
  }, batch: { raw, names in
   let id = (raw as! NSNumber).intValue
   contentReads += 1
   if !mutated {
    mutated = true
    switch mutation {
    case "focus": focus = 6
    case "window": window = 9
    case "parent": parent[4] = 2
    case "root": parent[4] = 7; parent[7] = 2; role[7] = "AXPopover"
    case "process": processAlive = false
    case "foreign": foreign.insert(4)
    default: break
    }
   }
   return (.success,names.map { name -> Any in
    switch name {
    case kAXRoleAttribute: return role[id] ?? "AXGroup"
    case kAXSubroleAttribute: return id == 6 ? "AXSecureTextField" : ""
    case kAXIdentifierAttribute: return "node-\(id)"
    case kAXEnabledAttribute: return NSNumber(value:true)
    case kAXValueAttribute: if id == 6 { secretReads += 1 }; return "public"
    default: return NSNull()
    }
   } as CFArray)
  }, actions: { _ in (.success,[] as CFArray) }, isElement: { CFGetTypeID($0) == CFNumberGetTypeID() })
  func command(_ req: [String: Any] = request, cap: Int = 524288) throws -> NativeFocusedAX.Command {
   let data = try JSONSerialization.data(withJSONObject:["schema_version":"0.1.0","artifact":["kind":"request","data":req]])
   return try NativeFocusedAX.command(NativeInbound(configurationBytes:JSONSerialization.data(withJSONObject:config),document:JSONSerialization.jsonObject(with:data) as! [String:Any],control:header,replyCap:cap,deadline:ProcessInfo.processInfo.systemUptime+2,started:ProcessInfo.processInfo.systemUptime))
  }
  func run(_ name: String, req: [String: Any] = request, cap: Int = 524288) throws -> [String:Any] {
   let frame = try NativeFocusedAX.collect(command(req,cap:cap),access:access,processCurrent:{_ in processAlive},application:{_ in NSNumber(value:0)},windowOwner:{ raw in foreign.contains((raw as! NSNumber).intValue) ? 999 : 123 },permitted:{true})
   let data = frame.bytes { Data($0) }
   try data.write(to:output.appendingPathComponent("popup-"+name+".json"),options:.withoutOverwriting)
   return (((try JSONSerialization.jsonObject(with:data) as! [String:Any])["artifact"] as! [String:Any])["data"] as! [String:Any])["result"] as! [String:Any]
  }
  func failed(_ name: String, _ code: String, noContent: Bool = true) throws {
   let result = try run(name)
   expect(result["status"] as? String == "failed")
   expect((result["data"] as? [String:Any])?["code"] as? String == code)
   expect(foreignRoleReads == 0 && secretReads == 0)
   if noContent { expect(contentReads == 0) }
  }
  reset()
  let positive = try run("positive")
  expect(positive["status"] as? String == "observed")
  let snapshot = positive["data"] as! [String:Any]
  let nodes = snapshot["nodes"] as! [[String:Any]]
  expect(nodes.count == 4 && focusReads == 7)
  expect((nodes[0]["native_role"] as! [String:Any])["value"] is [String:Any])
  let serialized = String(data:try JSONSerialization.data(withJSONObject:snapshot),encoding:.utf8)!
  expect(serialized.contains("AXPopover") && !serialized.contains("node-90") && serialized.contains("redacted") && secretReads == 0)
  expect((snapshot["surface_records"] as! [Any]).count == 1 && (snapshot["captures"] as! [Any]).isEmpty)
  reset(); role[2] = "AXPopover"
  let nested = try run("nearest-popup")
  expect(((nested["data"] as? [String:Any])?["nodes"] as? [Any])?.count == 4)
  reset(); focus = 3
  let selfPopup = try run("focus-is-popup")
  expect(((selfPopup["data"] as? [String:Any])?["nodes"] as? [Any])?.count == 4)
  reset(); focus = 2; try failed("no-popup","target_unresolved")
  reset(); parent.removeValue(forKey:4); try failed("missing-parent","target_unresolved")
  reset(); parent[3] = 4; try failed("cycle","target_unresolved")
  reset(); foreign.insert(5); try failed("foreign-focus","target_unresolved")
  reset(); foreign.insert(4); try failed("foreign-parent","target_unresolved")
  reset(); parent[2] = 9; try failed("other-window","target_unresolved")
  reset(); roleBudget = true; try failed("role-budget","incomplete_scope")
  reset(); var textLimited = profile as! [String:Any]; textLimited["copied_utf8_bytes"] = 1
  config["acquisition_limits"] = textLimited; try failed("initial-role-budget","incomplete_scope")
  reset(); focus = 100; role[105] = "AXPopover"; parent = Dictionary(uniqueKeysWithValues:(100..<118).map { ($0,$0+1) }); try failed("chain-limit","incomplete_scope")
  reset(); var small = profile as! [String:Any]; small["child_entries"] = 4; config["acquisition_limits"] = small; try failed("entry-limit","incomplete_scope")
  // Initial bind5 + precollection check6 leaves no hidden fresh allowance.
  reset(); small["child_entries"] = 11; config["acquisition_limits"] = small; try failed("revalidation-budget","incomplete_scope",noContent:false)
  reset(); mutation = "precollection-focus"; try failed("precollection-focus","stale_target")
  for change in ["focus","window","parent","root","process","foreign","publication-focus"] {
   reset(); mutation = change; try failed("changed-"+change,"stale_target",noContent:false)
  }
  reset(); let capped = try run("output-limit",cap:512); expect(capped["status"] as? String == "failed")
  reset(); var bad = request; var limits = request["limits"] as! [String:Any]
  for (key,value) in [("max_elements",161),("max_depth",10)] {
   var excessive = limits; excessive[key] = value; bad["limits"] = excessive
   do { _ = try command(bad); preconditionFailure("ceiling enlarged") } catch { checks += 1 }
  }
  limits["max_elements"] = 2; bad["limits"] = limits
  let bounded = try run("node-limit",req:bad)
  expect(((bounded["data"] as? [String:Any])?["nodes"] as? [Any])?.count == 2)
  reset(); config["collection"] = "focused-ax"
  let ordinary = try run("default")
  expect(ordinary["status"] as? String == "observed" && focusReads == 0)
  expect(((ordinary["data"] as? [String:Any])?["nodes"] as? [Any])?.count == 7)
  // Shared accounting preserves old arrays at selection=0 and charges selection first.
  let accounting = try NativeAcquisition(JSONDecoder().decode(NativeAcquisitionLimits.self,from:JSONSerialization.data(withJSONObject:profile)),deadline:ProcessInfo.processInfo.systemUptime+2)
  try accounting.reserveSelectionEntry()
  expect(accounting.selectionEntries == 1 && accounting.childEntries == 0)
  var four = profile as! [String:Any]; four["child_entries"] = 4
  let fourLimits = try JSONDecoder().decode(NativeAcquisitionLimits.self,from:JSONSerialization.data(withJSONObject:four))
  for reserved in [false, true] {
   let quota = try NativeAcquisition(fourLimits,deadline:ProcessInfo.processInfo.systemUptime+2)
   if reserved { try quota.reserveSelectionEntry() }
   var visited = 0
   let omitted = try quota.array(count:4,windows:false,capacity:4,read:{ offset,count in
    (offset..<offset+count).map { NSNumber(value:$0) } as CFArray
   },visit:{ _ in visited += 1 })
   expect(visited == (reserved ? 3 : 4) && omitted == (reserved ? 1 : 0))
   expect(quota.childEntries == visited && quota.selectionEntries == (reserved ? 1 : 0))
  }
  return checks
 }

}
