import Foundation
@main struct ReadonlyProtocolChecks {
 static func main() throws {
  let cfg=try JSONSerialization.jsonObject(with:Data(contentsOf:URL(fileURLWithPath:CommandLine.arguments[1]))) as! [String:Any]
  let configuration=(cfg["provider"] as! [String:Any])["configuration"] as! String
  let descriptor=((cfg["descriptor"] as! [String:Any])["artifact"] as! [String:Any])["data"] as! [String:Any]
  let context:[String:Any]=["schema_version":"0.1.0","session_id":descriptor["session_id"]!,"target":descriptor["target"]!,"surfaces":descriptor["surfaces"]!,"scope_id":(descriptor["allowed_scopes"] as! [String])[0],"projection":"interaction","fields":["role","accessibility_name","description","value","placeholder","enabled","focused","actions","accessibility_bounds"],"plugin":descriptor["plugin"]!,"environment_revision":"f02-expanded-a"]
  let request:[String:Any]=["schema_version":"0.1.0","artifact":["kind":"request","data":["request_id":"offline","clock_domain":"worker","context":context,"limits":["max_elements":160,"max_depth":9,"max_output_bytes":524288,"deadline_ms":3000],"freshness_policy":"current_required","operation":["operation":"observe","channels":["external_semantics","rendered_capture"]]]]]
  func input(_ flag:UInt8,_ channel:UInt8=0,_ kind:UInt8=8)throws->NativeInbound {
   var bytes=Data(repeating:0,count:64);bytes.replaceSubrange(0..<8,with:Data("UIBHST01".utf8));bytes[8]=3;bytes[9]=kind;bytes[10]=channel;bytes[11]=flag
   for(offset,value)in [(16,UInt64(1)),(24,1),(32,1),(40,1),(48,3000)]{for i in 0..<8{bytes[offset+i]=UInt8(truncatingIfNeeded:value>>(i*8))}}
   return NativeInbound(configurationBytes:Data(configuration.utf8),document:try JSONSerialization.jsonObject(with:JSONSerialization.data(withJSONObject:request)) as! [String:Any],control:try NativeControl(bytes),replyCap:524288,deadline:ProcessInfo.processInfo.systemUptime+3,started:ProcessInfo.processInfo.systemUptime)
  }
  var checks=0
  for flag:UInt8 in [0,128]{_ = try NativeHostProtocol.fixtureCommand(input(flag));checks+=1}
  for(flag,channel,kind):(UInt8,UInt8,UInt8) in [(1,0,8),(129,0,8),(128,1,8),(128,2,8),(128,0,9)]{
   do{_ = try NativeHostProtocol.fixtureCommand(input(flag,channel,kind));fatalError("invalid private command accepted")}catch{checks+=1}
  }
  print("{\"checks\":\(checks),\"live_calls\":0}")
 }
}
