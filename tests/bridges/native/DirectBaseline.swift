// Q02 direct public-API baseline. Explicit stdin requests only; no UI input.
import AppKit
import Foundation
import Darwin

@main struct Q02DirectBaseline {
    @MainActor static func main() async {
        do {
            let args = CommandLine.arguments
            guard args.count == 4 else { _exit(2) }
            let manifest = try JSONSerialization.jsonObject(with: Q02ReadonlyFacts.read(args[1], cap: 524288)) as! [String: Any]
            let limits = try JSONDecoder().decode(NativeAcquisitionLimits.self, from: Q02ReadonlyFacts.read(args[3], cap: 4032))
            print("@Q02 {\"kind\":\"attached\"}"); fflush(stdout)
            while let line = readLine() {
                let start = ProcessInfo.processInfo.systemUptime
                do {
                    guard line.utf8.count < 4096,
                          let command = try JSONSerialization.jsonObject(with: Data(line.utf8)) as? [String: Any],
                          let output = command["output"] as? String else { throw NativeAcquisitionError.invalidValue }
                    let before = try Q02ReadonlyFacts.identity(manifest, executable: args[2])
                    let json = NativeJSON(limits)
                    let axStart = ProcessInfo.processInfo.systemUptime
                    let facts = try Q02ReadonlyFacts.facts(manifest, limits: limits, json: json, deadline: axStart + 1)
                    let axEnd = ProcessInfo.processInfo.systemUptime
                    let captureStart = ProcessInfo.processInfo.systemUptime
                    let admission = try NativeAcquisition(limits, deadline: captureStart + 2)
                    let directory = try nativeChannelDirectory(URL(fileURLWithPath: output), channel: "capture")
                    let capture = try await CaptureLifecycle.capture(windowID: manifest["window_id"] as! UInt32,
                        pid: manifest["pid"] as! Int32, admission: .parentOwned, acquisition: admission)
                    let pngBytes = try writeNativePNG(capture.image, admission: admission, directory: directory, name: "capture.png")
                    let captureEnd = ProcessInfo.processInfo.systemUptime
                    let after = try Q02ReadonlyFacts.identity(manifest, executable: args[2])
                    let report: [String: Any] = ["kind": "sample", "status": "observed", "before": before, "after": after,
                        "facts": facts, "ax_ms": (axEnd-axStart)*1000, "capture_ms": (captureEnd-captureStart)*1000,
                        "request_ms": (ProcessInfo.processInfo.systemUptime-start)*1000,
                        "pixels": [capture.image.width,capture.image.height], "png_bytes": pngBytes,
                        "image": directory.appendingPathComponent("capture.png").path,
                        "capture_acquisition": admission.metrics]
                    let frame = try NativeJSONFrame(capacity: 524288, deadline: ProcessInfo.processInfo.systemUptime + 1)
                    try frame.encode(json.borrowed(report))
                    FileHandle.standardOutput.write(Data("@Q02 ".utf8))
                    try frame.write(to: STDOUT_FILENO, deadline: ProcessInfo.processInfo.systemUptime + 1)
                } catch {
                    print("@Q02 {\"kind\":\"sample\",\"status\":\"failed\",\"code\":\"baseline_acquisition_failed\"}"); fflush(stdout)
                }
            }
            print("@Q02 {\"kind\":\"closed\",\"cleanup_confirmed\":true,\"error\":null}"); fflush(stdout)
        } catch { _exit(4) }
    }
}
