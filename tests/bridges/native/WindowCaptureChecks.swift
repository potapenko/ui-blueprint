import Foundation
import CoreGraphics
import Darwin

@main struct WindowCaptureChecks {
    @MainActor static func main() throws {
        let root = URL(fileURLWithPath: CommandLine.arguments[1], isDirectory: true)
        var count = 0
        func expect(_ value: Bool) { precondition(value); count += 1 }
        func refuses(_ body: () throws -> Void) { do { try body(); preconditionFailure("expected refusal") } catch { count += 1 } }
        let valid = ["--pid","123","--window-id","456","--expected-bundle","public.app",
            "--out",root.appendingPathComponent("new").path,"--request-id","sample"]
        let discovery = try NativeWindowCapture.discoveryOptions(["--list-windows","--pid","123","--expected-bundle","public.app"])
        expect(discovery.pid == 123)
        refuses { _ = try NativeWindowCapture.discoveryOptions(["--list-windows","--pid","123","--out","/Users/unknown"]) }
        let options = try NativeWindowCapture.Options.parse(valid)
        expect(options.pid == 123 && options.window == 456 && options.deadlineMS == 5000)
        refuses { _ = try NativeWindowCapture.Options.parse([]) }
        refuses { _ = try NativeWindowCapture.Options.parse(valid + ["--pid","999"]) }
        refuses { _ = try NativeWindowCapture.Options.parse(valid + ["--unknown","private-canary"]) }
        refuses { _ = try NativeWindowCapture.Options.parse(valid + ["--deadline-ms","10001"]) }
        refuses { _ = try NativeWindowCapture.Options.parse(valid + ["--expected-launch-time","nan"]) }
        var changed=valid;changed[1]="0";refuses { _ = try NativeWindowCapture.Options.parse(changed) }
        changed=valid;changed[3]="4294967296";refuses { _ = try NativeWindowCapture.Options.parse(changed) }
        changed=valid;changed[7]="/Users/out";refuses { _ = try NativeWindowCapture.Options.parse(changed) }
        changed=valid;changed[9]="secret/value";refuses { _ = try NativeWindowCapture.Options.parse(changed) }
        try NativeWindowCapture.newDirectory(options.output, parentDirectory: options.parentDirectory)
        refuses { try NativeWindowCapture.newDirectory(options.output, parentDirectory: options.parentDirectory) }
        let link=root.appendingPathComponent("existing-link")
        expect(symlink(options.output.path,link.path)==0)
        refuses { try NativeWindowCapture.newDirectory(link, parentDirectory: options.parentDirectory) }
        let before = root.appendingPathComponent("before")
        let moved = root.appendingPathComponent("moved")
        expect(mkdir(before.path, 0o700) == 0)
        changed = valid
        changed[7] = before.appendingPathComponent("image").path
        let bound = try NativeWindowCapture.Options.parse(changed)
        expect(rename(before.path, moved.path) == 0 && symlink(root.path, before.path) == 0)
        refuses { try NativeWindowCapture.newDirectory(bound.output, parentDirectory: bound.parentDirectory) }
        expect(!FileManager.default.fileExists(atPath: root.appendingPathComponent("image").path))
        expect(unlink(before.path) == 0 && rmdir(moved.path) == 0)
        let frame=CGRect(x:120,y:40,width:1920,height:1050)
        let geometry=CaptureGeometry(frame:frame,displays:[1,0,0,1920,1080,3840,2160,0])
        let mapping=try CaptureMapping.verified(before:geometry,after:geometry,window:frame,content:frame,scale:2,width:3840,height:2100)
        expect(mapping?.affine == [2,0,0,2,-240,-80])
        let shifted=CaptureGeometry(frame:frame.offsetBy(dx:1,dy:0),displays:geometry.displays)
        refuses { _ = try CaptureMapping.verified(before:geometry,after:shifted,window:frame,content:frame,scale:2,width:3840,height:2100) }
        let resized=CaptureGeometry(frame:CGRect(x:120,y:40,width:961,height:1050),displays:geometry.displays)
        refuses { try geometry.requireUnchanged(resized) }
        refuses { try geometry.requireUnchanged(CaptureGeometry(frame:frame,displays:[2])) }
        expect(try CaptureMapping.verified(before:geometry,after:geometry,window:frame,content:frame,scale:2,width:1920,height:1050) == nil)
        let acquisition=try NativeAcquisition(NativeWindowCapture.limits(),deadline:ProcessInfo.processInfo.systemUptime+10)
        let dims=try acquisition.imageDimensions(width:1920,height:1050,scale:2)
        expect(dims.0==3840 && dims.1==2100)
        refuses { _ = try acquisition.imageDimensions(width:4096,height:4096,scale:2) }
        try FileManager.default.removeItem(at:link)
        try FileManager.default.removeItem(at:options.output)
        print("Native window capture boundary checks: \(count) passed")
    }
}
