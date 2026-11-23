import AppKit
let pid:Int32=32187, win:CGWindowID=1060
let expected="/private/tmp/playphraseme-guest-tour-runtime-axlhvjj9/PlayPhraseMe.app"
guard CommandLine.arguments.count==1,let a=NSRunningApplication(processIdentifier:pid),!a.isTerminated,
      a.launchDate?.timeIntervalSince1970==1795448825.752163,a.bundleIdentifier=="me.playphraseme.reference.guesttour.01a20845.g2",
      let path=a.bundleURL?.path,let canonical=realpath(path,nil) else{exit(4)}
defer{free(canonical)}
guard String(cString:canonical)==expected,let list=CGWindowListCopyWindowInfo(.optionIncludingWindow,win) as? [[String:Any]],
      list.count==1,let w=list.first,(w[kCGWindowOwnerPID as String] as? NSNumber)?.int32Value==pid,
      (w[kCGWindowNumber as String] as? NSNumber)?.uint32Value==win,(w[kCGWindowIsOnscreen as String] as? NSNumber)?.boolValue==true,
      (w[kCGWindowLayer as String] as? NSNumber)?.intValue==0,let raw=w[kCGWindowBounds as String] as? NSDictionary,
      let f=CGRect(dictionaryRepresentation:raw),[f.minX,f.minY,f.width,f.height].allSatisfy({$0.isFinite}),f.width>0,f.height>0 else{exit(4)}
guard let windows=CGWindowListCopyWindowInfo(.optionOnScreenOnly,kCGNullWindowID) as? [[String:Any]] else { exit(4) }
let owned=windows.filter { ($0[kCGWindowOwnerPID as String] as? NSNumber)?.int32Value==pid && ($0[kCGWindowLayer as String] as? NSNumber)?.intValue==0 }
guard owned.count==1, (owned[0][kCGWindowNumber as String] as? NSNumber)?.uint32Value==win else { exit(4) }
let id:[String:Any]=["pid":pid,"birth":1795448825.752163,"window_id":win,"bundle":a.bundleIdentifier!,"window_screen_pt":[f.minX,f.minY,f.width,f.height]]
print(String(decoding:try JSONSerialization.data(withJSONObject:id,options:.sortedKeys),as:UTF8.self))
