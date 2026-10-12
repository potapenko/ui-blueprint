import Foundation
import ApplicationServices
import Darwin

enum StaticTextChecks {
 @MainActor static func run(profile: Any, request: [String:Any], header: NativeControl, output: URL) throws -> Int {
  var assertions=0
  func expect(_ value:Bool,line:UInt = #line) {precondition(value,"static-text assertion at \(line)");assertions += 1}
  let canary="PRIVATE_STATIC_TEXT_CANARY_DO_NOT_PUBLISH"
  var role="AXStaticText", subrole="", identifier="public-static", fault="", window=1
  var payload:Any="public static content"
  var valueReads=0, identityReads=0, duplicateValueAttributes=0
  func axError(_ code:AXError)->AXValue {var code=code;return AXValueCreate(.axError,&code)!}
  func reset() {role="AXStaticText";subrole="";identifier="public-static";fault="";window=1;payload="public static content";valueReads=0;identityReads=0;duplicateValueAttributes=0}
  let access=NativeAXAccess(prepare:{_ in},attribute:{_,name in
   if name==kAXFocusedWindowAttribute{return (.success,NSNumber(value:window))}
   if name==kAXRoleAttribute{return (.success,"AXWindow" as CFString)}
   return (.noValue,nil)
  },count:{raw,_ in(.success,(raw as! NSNumber).intValue==1 ? 1:0)},page:{_,_,_,_ in(.success,[NSNumber(value:2)] as CFArray)},batch:{raw,names in
   let id=(raw as! NSNumber).intValue
   if id==2 && names.contains(kAXRoleAttribute) {identityReads += 1}
   if id==2 && names.contains(kAXValueAttribute) {
    valueReads += 1;duplicateValueAttributes += max(0,names.filter{$0==kAXValueAttribute}.count-1)
    if fault=="window-change" {window=9}
    if fault=="deadline" {usleep(30_000)}
   }
   if id==2 && names.contains(kAXRoleAttribute) && fault=="classification-failed" {return (.cannotComplete,nil)}
   return (.success,names.map{name->Any in
    if id==1 {
     switch name {case kAXRoleAttribute:return "AXWindow";case kAXSubroleAttribute,kAXIdentifierAttribute:return "";default:return axError(.noValue)}
    }
    switch name {
    case kAXRoleAttribute:
     if fault=="role-missing" {return axError(.noValue)}
     if fault=="role-unsupported" {return axError(.attributeUnsupported)}
     return role
    case kAXSubroleAttribute:return fault=="subrole-failed" ? axError(.cannotComplete) : subrole
    case kAXIdentifierAttribute:return fault=="identifier-failed" ? axError(.cannotComplete) : identifier
    case kAXDescriptionAttribute:return "semantic name"
    case kAXTitleAttribute:return "independent title"
    case kAXValueAttribute:return payload
    default:return axError(.noValue)
    }
   } as CFArray)
  },actions:{_ in(.success,[] as CFArray)},isElement:{CFGetTypeID($0)==CFNumberGetTypeID()})
  func run(_ label:String,fields:[String]=["visible_text"],changes:[String:Any]=[:],cap:Int=524288,deadline:Int=1000)throws->([String:Any],Data) {
   var limits=profile as! [String:Any];for (k,v) in changes {limits[k]=v}
   let config:[String:Any]=["collection":"focused-ax","process":["pid":123,"bundle_id":"ordinary.reader","launch_time":100.0],"scope_id":"selected-window","acquisition_limits":limits]
   var req=request,ctx=req["context"] as! [String:Any];ctx["fields"]=fields;req["context"]=ctx
   var bounds=req["limits"] as! [String:Any];bounds["deadline_ms"]=deadline;req["limits"]=bounds
   let doc=try JSONSerialization.jsonObject(with:JSONSerialization.data(withJSONObject:["schema_version":"0.1.0","artifact":["kind":"request","data":req]])) as! [String:Any]
   let command=try NativeFocusedAX.command(NativeInbound(configurationBytes:JSONSerialization.data(withJSONObject:config),document:doc,control:header,replyCap:cap,deadline:NativeDescriptorIO.now+2,started:NativeDescriptorIO.now))
   let frame=try NativeFocusedAX.collect(command,access:access,processCurrent:{_ in true},application:{_ in NSNumber(value:0)},windowOwner:{raw in fault=="foreign" && (raw as! NSNumber).intValue==2 ? 999:123},permitted:{true})
   let data=frame.bytes{Data($0)};try data.write(to:output.appendingPathComponent("static-"+label+".json"),options:.withoutOverwriting)
   let result=(((try JSONSerialization.jsonObject(with:data) as! [String:Any])["artifact"] as! [String:Any])["data"] as! [String:Any])["result"] as! [String:Any]
   return (result,data)
  }
  func property(_ result:[String:Any],_ field:String)->[String:Any] {
   let nodes=(result["data"] as! [String:Any])["nodes"] as! [[String:Any]]
   return (nodes[1]["properties"] as! [[String:Any]]).first{$0["field"] as? String==field}!
  }
  func state(_ result:[String:Any],_ field:String="visible_text")->[String:Any] {property(result,field)["state"] as! [String:Any]}
  func unavailable(_ name:String,_ availability:String,reads:Int=0)throws {
   let (r,data)=try run(name);expect(r["status"] as? String=="observed");expect(state(r)["availability"] as? String==availability)
   expect(valueReads==reads && duplicateValueAttributes==0);expect(!String(data:data,encoding:.utf8)!.contains(canary))
  }
  reset();let (known,_)=try run("known")
  expect((state(known)["value"] as? [String:Any])?["value"] as? String=="public static content" && valueReads==1)
  expect((property(known,"visible_text")["evidence"] as! [String:Any])["method"] as? String=="bounded_public_ax_static_text_value")
  expect(!((known["data"] as! [String:Any])["nodes"] as! [[String:Any]]).contains{($0["properties"] as! [[String:Any]]).contains{$0["field"] as? String=="value"}})
  reset();payload="";let (empty,_)=try run("empty");expect((state(empty)["value"] as? [String:Any])?["value"] as? String=="")
  reset();let (both,_)=try run("both",fields:["visible_text","value","accessibility_name"])
  expect(NSDictionary(dictionary:state(both)).isEqual(to:state(both,"value")) && valueReads==1 && duplicateValueAttributes==0)
  expect((state(both,"accessibility_name")["value"] as? [String:Any])?["value"] as? String=="semantic name")
  let ext=(((both["data"] as! [String:Any])["nodes"] as! [[String:Any]])[1]["extensions"] as! [[String:Any]]).first{$0["name"] as? String=="AXTitle"}!
  let titleState=(ext["property"] as! [String:Any])["state"] as! [String:Any]
  expect((titleState["value"] as? [String:Any])?["value"] as? String=="independent title")
  reset();let (old,_)=try run("old",fields:["role","accessibility_name"]);expect(valueReads==0 && property(old,"accessibility_name")["state"] != nil)
  reset();let (raw,_)=try run("raw-only",fields:["value"]);expect(valueReads==1 && (state(raw,"value")["value"] as? [String:Any])?["value"] as? String=="public static content")
  for name in ["role-missing","role-unsupported","classification-failed","subrole-failed","identifier-failed"] {reset();fault=name;payload=canary;try unavailable(name,"unknown")}
  reset();identifier=String(repeating:"x",count:4097);payload=canary;try unavailable("identity-oversize","unknown")
  for r in ["AXTextField","AXTextArea","AXSearchField","AXButton","AXMenuButton"] {reset();role=r;payload=canary;try unavailable("nonstatic-"+r,"unsupported")}
  for secure in ["role","subrole","identifier"] {
   reset();payload=canary
   if secure=="role" {role="AXSecureTextField"};if secure=="subrole" {subrole="AXSecureTextField"};if secure=="identifier" {identifier="f02.secret"}
   let (r,data)=try run("protected-"+secure,fields:["visible_text","value"])
   expect(state(r)["availability"] as? String=="redacted" && state(r,"value")["availability"] as? String=="redacted")
   expect(property(r,"visible_text")["sensitivity"] as? String=="sensitive" && valueReads==0 && !String(data:data,encoding:.utf8)!.contains(canary))
  }
  for (name,value) in [("number",NSNumber(value:7) as Any),("bool",NSNumber(value:true) as Any),("array",[canary] as Any),("object",["unreviewed":canary] as Any),("null",NSNull() as Any)] {
   reset();payload=value;try unavailable(name,"unknown",reads:1)
  }
  reset();payload=axError(.noValue);try unavailable("no-value","unknown",reads:1)
  reset();payload=axError(.attributeUnsupported);try unavailable("unsupported-value","unsupported",reads:1)
  reset();payload=canary+String(repeating:"x",count:4097);try unavailable("value-oversize","unknown",reads:1)
  reset();fault="foreign";payload=canary;let (foreign,data)=try run("foreign")
  expect(valueReads==0 && identityReads==0 && !String(data:data,encoding:.utf8)!.contains(canary))
  expect(((foreign["data"] as! [String:Any])["nodes"] as! [Any]).count==1)
  reset();fault="window-change";payload=canary;let (changed,changedBytes)=try run("window-change")
  expect(changed["status"] as? String=="failed" && (changed["data"] as! [String:Any])["code"] as? String=="stale_target" && !String(data:changedBytes,encoding:.utf8)!.contains(canary))
  for (label,changes,cap) in [("copy-budget",["copied_utf8_bytes":160],524288),("output-charge",["response_string_utf8_bytes":2000],524288),("wire-budget",[:],512)] {
   reset();payload=String(repeating:canary,count:40)
   do {
    let (r,bytes)=try run(label,fields:["visible_text","value"],changes:changes,cap:cap)
    expect(!String(data:bytes,encoding:.utf8)!.contains(canary))
    expect(r["status"] as? String=="failed" || state(r)["availability"] as? String=="unknown")
   } catch {assertions += 1}
  }
  reset();fault="deadline";payload=canary
  do {_ = try run("deadline",deadline:10);preconditionFailure("late text published")}catch {assertions += 1}
  return assertions
 }
}
