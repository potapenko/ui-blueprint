import Foundation
import ApplicationServices

// Substitute only public AX responses; use the real provider, collector, quotas
// and canonical frames for both old and explicit-visible invocations.
enum VisibleChildrenChecks {
 @MainActor static func run(profile: Any, request: [String:Any], header: NativeControl, output: URL) throws -> Int {
  var assertions=0
  func expect(_ value:Bool,line:UInt = #line) { precondition(value,"visible assertion at \(line)");assertions += 1 }
  var config:[String:Any] = ["collection":"focused-visible-ax","process":["pid":123,"bundle_id":"ordinary.reader","launch_time":100.0],"scope_id":"selected-window","acquisition_limits":profile]
  var req=request, context=req["context"] as! [String:Any]
  context["fields"]=["role","value","accessibility_bounds"];req["context"]=context
  var fault="", window=1, fullReads=0, visibleReads=0, foreignValues=0, secretValues=0
  let full=Array(100..<477), visible=Array(128..<181)
  func children(_ id:Int,_ name:String)->[Int] {
   if id==1 { return [2] };if id==2 { return [3] }
   if id==3 { return name==kAXVisibleChildrenAttribute ? (fault=="empty" ? []:visible):full }
   return []
  }
  let access=NativeAXAccess(prepare:{_ in},attribute:{raw,name in
   if name==kAXFocusedWindowAttribute{return (.success,NSNumber(value:window))}
   if name==kAXRoleAttribute{return (.success,"AXWindow" as CFString)}
   return (.noValue,nil)
  },count:{raw,name in
   let id=(raw as! NSNumber).intValue
   if id==3 {
    if name==kAXChildrenAttribute {fullReads += 1}
    if name==kAXVisibleChildrenAttribute {
     visibleReads += 1
     switch fault {
     case "unsupported":return (.attributeUnsupported,0)
     case "no-value":return (.noValue,0)
     case "count-error":return (.cannotComplete,0)
     default:break
     }
    }
   }
   return (.success,children(id,name).count)
  },page:{raw,name,index,count in
   let id=(raw as! NSNumber).intValue
   if id==3 && name==kAXVisibleChildrenAttribute {
    if fault=="page-error" {return (.cannotComplete,nil)}
    if fault=="wrong-type" {return (.success,["not-an-element"] as CFArray)}
    if fault=="window-change" {window=9}
   }
   let values=children(id,name)
   return (.success,values[index..<min(index+count,values.count)].map{NSNumber(value:$0)} as CFArray)
  },batch:{raw,names in
   let id=(raw as! NSNumber).intValue
   return (.success,names.map{name->Any in
    switch name {
    case kAXRoleAttribute:
     if id==3 && fault=="missing-role" {return NSNull()}
     if id==3 && fault.hasPrefix("standard-") {return String(fault.dropFirst(9))}
     return id==1 ? "AXWindow":id==2 ? "AXGroup":id==3 ? "AXOpaqueProviderGroup":id==129 ? "AXTextField":"AXButton"
    case kAXSubroleAttribute:return id==3 ? (fault=="unclassified" ? "unclassified-source":"AXOpaqueProviderGrid"):id==129 ? "AXSecureTextField":""
    case kAXIdentifierAttribute:return "source-node-\(id)"
    case kAXValueAttribute:
     if id==128 && fault=="foreign" {foreignValues += 1};if id==129 {secretValues += 1};return "public"
    default:return NSNull()
    }
   } as CFArray)
  },actions:{_ in(.success,[] as CFArray)},isElement:{CFGetTypeID($0)==CFNumberGetTypeID()})
  func run(_ label:String,mode:String="focused-visible-ax",entryCap:Int?=nil,nodeCap:Int=160)throws->[String:Any] {
   window=1;fullReads=0;visibleReads=0;foreignValues=0;secretValues=0
   config["collection"]=mode;var selected=profile as! [String:Any]
   if let entryCap {selected["child_entries"]=entryCap};config["acquisition_limits"]=selected
   var request=req, limits=req["limits"] as! [String:Any];limits["max_elements"]=nodeCap;request["limits"]=limits
   let document=try JSONSerialization.jsonObject(with:JSONSerialization.data(withJSONObject:["schema_version":"0.1.0","artifact":["kind":"request","data":request]])) as! [String:Any]
   let command=try NativeFocusedAX.command(NativeInbound(configurationBytes:JSONSerialization.data(withJSONObject:config),document:document,control:header,replyCap:524288,deadline:NativeDescriptorIO.now+2,started:NativeDescriptorIO.now))
   let frame=try NativeFocusedAX.collect(command,access:access,processCurrent:{_ in true},application:{_ in NSNumber(value:0)},windowOwner:{raw in (raw as! NSNumber).intValue==128 && fault=="foreign" ? 999:123},permitted:{true})
   let bytes=frame.bytes{Data($0)};try bytes.write(to:output.appendingPathComponent("visible-"+label+".json"),options:.withoutOverwriting)
   return (((try JSONSerialization.jsonObject(with:bytes) as! [String:Any])["artifact"] as! [String:Any])["data"] as! [String:Any])["result"] as! [String:Any]
  }
  func nodes(_ result:[String:Any])->[[String:Any]] {(result["data"] as! [String:Any])["nodes"] as! [[String:Any]]}
  func source(_ result:[String:Any])->[String:Any]? {
   let grid=nodes(result)[2]
   return (grid["extensions"] as! [[String:Any]]).first{$0["name"] as? String=="AXChildrenSource"}.map{($0["property"] as! [String:Any])["state"] as! [String:Any]}
  }
  func identifiers(_ result:[String:Any])->[String] {
   nodes(result).compactMap {node in
    let item=(node["extensions"] as! [[String:Any]]).first{$0["name"] as? String=="AXIdentifier"}!
    return ((((item["property"] as! [String:Any])["state"] as! [String:Any])["value"] as? [String:Any])?["value"] as? String)
   }
  }
  let old=try run("old",mode:"focused-ax")
  expect(old["status"] as? String=="observed" && nodes(old).count==160 && fullReads==1 && visibleReads==0)
  expect(identifiers(old).dropFirst(3).first=="source-node-100" && identifiers(old).last=="source-node-256" && source(old)==nil)
  let current=try run("selected")
  expect(current["status"] as? String=="observed" && nodes(current).count==56 && fullReads==0 && visibleReads==1)
  expect(Array(identifiers(current).dropFirst(3))==visible.map{"source-node-\($0)"})
  expect(source(current)?["availability"] as? String=="known" && (source(current)?["value"] as? [String:Any])?["value"] as? String=="AXVisibleChildren")
  expect(secretValues==0 && foreignValues==0)
  expect(((current["data"] as! [String:Any])["coverage"] as! [String:Any])["status"] as? String=="partial")
  for role in ["AXList","AXTable","AXOutline","AXGrid"] {
   fault="standard-"+role;let selected=try run(fault)
   expect(nodes(selected).count==56 && fullReads==0 && visibleReads==1)
  }
  for mode in ["unsupported","no-value","count-error","page-error","wrong-type","foreign","unclassified","missing-role"] {
   fault=mode;let result=try run(mode)
   expect(result["status"] as? String=="observed" && nodes(result).count==3 && fullReads==0)
   expect(source(result)?["availability"] as? String=="unknown")
   expect(foreignValues==0 && secretValues==0)
  }
  fault="empty";let empty=try run("empty")
  expect(nodes(empty).count==3 && source(empty)?["availability"] as? String=="known" && fullReads==0)
  fault="";let budget=try run("entry-budget",entryCap:4)
  expect(nodes(budget).count==5 && Array(identifiers(budget).dropFirst(3))==["source-node-128","source-node-129"] && fullReads==0)
  let nodeBudget=try run("node-budget",nodeCap:3)
  expect(nodes(nodeBudget).count==3 && fullReads==0 && source(nodeBudget)?["availability"] as? String=="known")
  fault="window-change";let changed=try run("window-change")
  expect(changed["status"] as? String=="failed" && (changed["data"] as! [String:Any])["code"] as? String=="stale_target")
  return assertions
 }
}
