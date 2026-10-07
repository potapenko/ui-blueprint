import Foundation
import CoreFoundation
import CoreGraphics
import Darwin

@main struct AcquisitionChecks {
    static func main() throws {
        let args = CommandLine.arguments
        guard args.count == 5 else { exit(2) }
        let data = try Data(contentsOf: URL(fileURLWithPath: args[1]))
        let profile = try JSONSerialization.jsonObject(with: data) as! [String: Int]
        let root = URL(fileURLWithPath: args[2], isDirectory: true)
        let deadline = ProcessInfo.processInfo.systemUptime + 30
        var passed = 0
        func limits(_ changes: [String: Int] = [:]) throws -> NativeAcquisitionLimits {
            let value = profile.merging(changes, uniquingKeysWith: { _, b in b })
            let result = try JSONDecoder().decode(NativeAcquisitionLimits.self, from: JSONSerialization.data(withJSONObject: value))
            try result.validate(); return result
        }
        func expect(_ value: Bool) { precondition(value); passed += 1 }
        func refuses(_ body: () throws -> Void) {
            do { try body(); fatalError("expected bounded refusal") } catch { passed += 1 }
        }
        func owner(_ changes: [String: Int] = [:]) throws -> NativeAcquisition { try NativeAcquisition(limits(changes), deadline: deadline) }
        for (key, maximum) in profile {
            refuses { _ = try limits([key: 0]) }
            refuses { _ = try limits([key: maximum + 1]) }
        }
        refuses { _ = try nativeAdd(Int.max, 1) }
        refuses { _ = try nativeMultiply(Int.max, 2) }
        let text = try owner()
        expect(try text.text("" as CFString) == "")
        expect(try text.text(String(repeating: "x", count: 4096) as CFString).utf8.count == 4096)
        expect(try text.text(String(repeating: "😀", count: 1024) as CFString).utf8.count == 4096)
        let textReject = try owner()
        refuses { _ = try textReject.text(String(repeating: "x", count: 4097) as CFString) }
        refuses { _ = try textReject.text(String(repeating: "😀", count: 1025) as CFString) }
        refuses { _ = try textReject.text(NSNumber(value: 9)) }
        let units: [UniChar] = [0xD800]
        let invalid = CFStringCreateWithCharacters(nil, units, 1)!
        refuses { _ = try textReject.text(invalid) }
        expect(textReject.copiedUTF8 == 0)
        let aggregate = try owner(["copied_utf8_bytes": 16])
        expect(try aggregate.text("12345678" as CFString) == "12345678")
        refuses { _ = try aggregate.text("x" as CFString) }
        expect(aggregate.copiedUTF8 == 16)
        let aggregateShort = try owner(["copied_utf8_bytes": 15])
        refuses { _ = try aggregateShort.text("12345678" as CFString) }
        expect(aggregateShort.copiedUTF8 == 0)
        let batch = try owner()
        let excessive = Array(repeating: String(repeating: "x", count: 4096), count: 5) as CFArray
        refuses { _ = try batch.batch(excessive, expected: 5) }
        expect(batch.copiedUTF8 == 0)
        expect(try batch.batch(["a", "b", NSNumber(value: false)] as CFArray, expected: 3).count == 3)
        expect(batch.copiedUTF8 == 4)
        let exactBatch = try owner()
        let exactBatchValues = Array(repeating: String(repeating: "a", count: 2048), count: 4) as CFArray
        expect(try exactBatch.batch(exactBatchValues, expected: 4).count == 4)
        expect(exactBatch.copiedUTF8 == 16384)
        let shortBatch = try owner(["batch_utf8_bytes": 16383])
        refuses { _ = try shortBatch.batch(exactBatchValues, expected: 4) }
        expect(shortBatch.copiedUTF8 == 0)
        expect(try owner().batch(Array(repeating: "a", count: 8) as CFArray, expected: 8).count == 8)
        refuses { _ = try owner().batch(Array(repeating: "a", count: 9) as CFArray, expected: 9) }
        refuses { _ = try batch.batch(["a"] as CFArray, expected: 2) }
        let actions = try owner()
        expect(try actions.actions(Array(repeating: String(repeating: "a", count: 256), count: 32) as CFArray).count == 32)
        let actionReject = try owner()
        refuses { _ = try actionReject.actions(Array(repeating: "a", count: 33) as CFArray) }
        refuses { _ = try actionReject.actions([String(repeating: "a", count: 257)] as CFArray) }
        refuses { _ = try actionReject.actions(["a", NSNumber(value: 5)] as CFArray) }
        expect(actionReject.copiedUTF8 == 0)
        expect(!nativeMayReadValue(role: "AXTextField", subrole: nil, identifier: "f02.name", complete: false))
        expect(!nativeMayReadValue(role: nil, subrole: nil, identifier: nil, complete: true))
        expect(!nativeMayReadValue(role: "AXSecureTextField", subrole: nil, identifier: nil, complete: true))
        expect(!nativeMayReadValue(role: "AXTextField", subrole: "AXSecureTextField", identifier: nil, complete: true))
        expect(!nativeMayReadValue(role: "AXTextField", subrole: nil, identifier: "f02.secret", complete: true))
        expect(nativeMayReadValue(role: "AXTextField", subrole: nil, identifier: "f02.name", complete: true))
        let arrays = try owner(["array_page": 2, "child_entries": 5])
        var asked: [Int] = [], visited = 0
        let omitted = try arrays.array(count: 8, windows: false, capacity: 8, read: { _, amount in
            asked.append(amount); return Array(repeating: NSNumber(value: 1), count: amount) as CFArray
        }, visit: { _ in visited += 1 })
        expect(omitted == 3 && visited == 5 && asked == [2, 2, 1])
        expect(try arrays.array(count: 2, windows: false, capacity: 2, read: { _, _ in fatalError("no remaining entry allowance") }, visit: { _ in }) == 2)
        var reads = 0
        refuses { _ = try arrays.array(count: 33, windows: true, capacity: 33, read: { _, _ in reads += 1; return [] as CFArray }, visit: { _ in }) }
        expect(reads == 0)
        let fullWindowList = try owner()
        var windows = 0
        expect(try fullWindowList.array(count: 32, windows: true, capacity: 32,
            read: { _, amount in Array(repeating: NSNumber(value: 1), count: amount) as CFArray },
            visit: { _ in windows += 1 }) == 0)
        expect(windows == 32)
        let malformed = try owner()
        refuses { _ = try malformed.array(count: 1, windows: false, capacity: 1, read: { _, _ in [1,2] as CFArray }, visit: { _ in fatalError("bad page cannot be retained") }) }
        let scalarBudget = NativeJSON(try limits())
        let falseState = try nativeAXScalar(NSNumber(value: false), json: scalarBudget)
        expect((falseState["value"] as? [String: Any])?["value"] as? Bool == false)
        let emptyState = try nativeAXScalar("", json: scalarBudget)
        expect((emptyState["value"] as? [String: Any])?["value"] as? String == "")
        expect(try nativeAXScalar(NSNumber(value: Double.nan), json: scalarBudget)["availability"] as? String == "unknown")
        expect(try scalarBudget.redacted()["value"] == nil)
        let json = NativeJSON(try limits(["response_slots": 2]))
        var constructed = false
        refuses { _ = try json.object(["a", "b"]) { constructed = true; return [:] } }
        expect(!constructed && json.slots == 0)
        let strings = NativeJSON(try limits(["response_string_utf8_bytes": 3]))
        expect(try strings.scalar("abc") == "abc")
        refuses { _ = try strings.scalar("x") }
        expect(strings.stringBytes == 3)
        let value: [String: Any] = ["text": "a\nb", "flag": false, "empty": ""]
        let expected = try JSONSerialization.data(withJSONObject: value, options: [.sortedKeys])
        let frame = try NativeJSONFrame(capacity: expected.count + 1, deadline: deadline)
        try frame.encode(value)
        expect(frame.count == expected.count + 1)
        let bytes = frame.bytes { Data($0) }
        expect(bytes.dropLast() == expected && bytes.last == 10)
        let small = try NativeJSONFrame(capacity: expected.count, deadline: deadline)
        refuses { try small.encode(value) }
        expect(small.failed)
        refuses { try small.write(to: -1, deadline: deadline) }
        small.reset(); try small.encode(["a": 1]); expect(!small.failed)
        let expired = try NativeJSONFrame(capacity: 100, deadline: ProcessInfo.processInfo.systemUptime - 1)
        refuses { try expired.encode(value) }
        let pixels = try owner()
        let dimensions = try pixels.imageDimensions(width: 550, height: 525, scale: 2)
        expect(dimensions.0 == 1100 && dimensions.1 == 1050)
        try pixels.returnedImage(width: 1100, height: 1050, rowBytes: 4400, requested: dimensions); passed += 1
        refuses { try pixels.returnedImage(width: 1099, height: 1050, rowBytes: 4400, requested: dimensions) }
        try owner(["image_pixels": 6, "image_bytes": 24]).imageFootprint(width: 3, height: 2, rowBytes: 12); passed += 1
        refuses { try owner(["image_pixels": 5]).imageFootprint(width: 3, height: 2, rowBytes: 12) }
        refuses { try owner(["image_bytes": 23]).imageFootprint(width: 3, height: 2, rowBytes: 12) }
        for width in [Double.nan, Double.infinity, 0, -1, 1e100] {
            refuses { _ = try pixels.imageDimensions(width: width, height: 2, scale: 2) }
        }
        refuses { _ = try pixels.imageDimensions(width: 4096, height: 4096, scale: 1) }
        try pixels.imageFootprint(width: 4096, height: 2048, rowBytes: 32768); passed += 1
        refuses { try pixels.imageFootprint(width: 4096, height: 2048, rowBytes: 32769) }
        refuses { try pixels.imageFootprint(width: 1, height: 1, rowBytes: 3) }
        func folder(_ name: String) throws -> URL {
            let path = root.appendingPathComponent(name)
            try FileManager.default.createDirectory(at: path, withIntermediateDirectories: false, attributes: [.posixPermissions: 0o700])
            return path
        }
        let raw = Data(repeating: 128, count: 4 * 3 * 2)
        let provider = CGDataProvider(data: raw as CFData)!
        let image = CGImage(width: 3, height: 2, bitsPerComponent: 8, bitsPerPixel: 32, bytesPerRow: 12,
            space: CGColorSpaceCreateDeviceRGB(), bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedFirst.rawValue).union(.byteOrder32Little),
            provider: provider, decode: nil, shouldInterpolate: false, intent: .defaultIntent)!
        let pngFolder = try folder("png")
        let pngSize = try autoreleasepool { try writeNativePNG(image, admission: pixels, directory: pngFolder, name: "capture.png") }
        expect(pngSize > 0 && FileManager.default.fileExists(atPath: pngFolder.appendingPathComponent("capture.png").path))
        let exactFolder = try folder("exact")
        expect(try autoreleasepool { try writeNativePNG(image, admission: owner(["png_bytes": pngSize]), directory: exactFolder, name: "capture.png") } == pngSize)
        let rejectedFolder = try folder("rejected")
        refuses { try autoreleasepool { _ = try writeNativePNG(image, admission: owner(["png_bytes": pngSize - 1]), directory: rejectedFolder, name: "capture.png") } }
        expect(try FileManager.default.contentsOfDirectory(atPath: rejectedFolder.path).count == 1)
        expect(!FileManager.default.fileExists(atPath: rejectedFolder.appendingPathComponent("capture.png").path))
        let oversizedFolder = try folder("oversized-return")
        refuses { _ = try writeNativePNG(image, admission: owner(["image_width": 2]), directory: oversizedFolder, name: "capture.png") }
        expect(try FileManager.default.contentsOfDirectory(atPath: oversizedFolder.path).isEmpty)
        let rgbProvider = CGDataProvider(data: Data(repeating: 127, count: 18) as CFData)!
        let rgbImage = CGImage(width: 3, height: 2, bitsPerComponent: 8, bitsPerPixel: 24, bytesPerRow: 9,
            space: CGColorSpaceCreateDeviceRGB(), bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.none.rawValue),
            provider: rgbProvider, decode: nil, shouldInterpolate: false, intent: .defaultIntent)!
        refuses { _ = try writeNativePNG(rgbImage, admission: pixels, directory: oversizedFolder, name: "capture.png") }
        let saved = try Data(contentsOf: pngFolder.appendingPathComponent("capture.png"))
        refuses { _ = try writeNativePNG(image, admission: pixels, directory: pngFolder, name: "capture.png") }
        expect(try Data(contentsOf: pngFolder.appendingPathComponent("capture.png")) == saved)
        let linkFolder = try folder("link")
        try FileManager.default.createSymbolicLink(atPath: linkFolder.appendingPathComponent("capture.png").path,
            withDestinationPath: pngFolder.appendingPathComponent("capture.png").path)
        refuses { _ = try writeNativePNG(image, admission: pixels, directory: linkFolder, name: "capture.png") }
        expect(try Data(contentsOf: pngFolder.appendingPathComponent("capture.png")) == saved)
        let partialFolder = try folder("partial")
        do {
            let writer = try NativeArtifactWriter(directory: partialFolder, destination: "done", cap: 3, deadline: deadline,
                output: { fd, p, n in Darwin.write(fd, p, min(1, n)) })
            expect(raw.withUnsafeBytes { writer.put($0.baseAddress!, count: 3) } == 3)
            expect(try writer.finish(encoderSucceeded: true) == 3)
        }
        expect(try Data(contentsOf: partialFolder.appendingPathComponent("done")).count == 3)
        let failureFolder = try folder("failed-finalize")
        do {
            let writer = try NativeArtifactWriter(directory: failureFolder, destination: "done", cap: 3, deadline: deadline)
            expect(raw.withUnsafeBytes { writer.put($0.baseAddress!, count: 4) } == 0)
            refuses { _ = try writer.finish(encoderSucceeded: true) }
        }
        expect(try FileManager.default.contentsOfDirectory(atPath: failureFolder.path).isEmpty)
        let failedFinalizer = try folder("encoder-failure")
        do {
            let writer = try NativeArtifactWriter(directory: failedFinalizer, destination: "done", cap: 3, deadline: deadline)
            expect(raw.withUnsafeBytes { writer.put($0.baseAddress!, count: 3) } == 3)
            refuses { _ = try writer.finish(encoderSucceeded: false) }
        }
        expect(try FileManager.default.contentsOfDirectory(atPath: failedFinalizer.path).isEmpty)
        let directoryLink = root.appendingPathComponent("directory-link")
        try FileManager.default.createSymbolicLink(at: directoryLink, withDestinationURL: failedFinalizer)
        refuses { _ = try NativeArtifactWriter(directory: directoryLink, destination: "no-write", cap: 3, deadline: deadline) }
        let operationRoot = root.appendingPathComponent("operation")
        let axDirectory = try nativeChannelDirectory(operationRoot, channel: "ax")
        let captureDirectory = try nativeChannelDirectory(operationRoot, channel: "capture")
        expect(axDirectory.lastPathComponent == "ax" && captureDirectory.lastPathComponent == "capture")
        refuses { _ = try nativeChannelDirectory(operationRoot, channel: "capture") }
        refuses { _ = try nativeChannelDirectory(directoryLink, channel: "ax") }
        let sidecarFolder = try folder("sidecar")
        refuses { _ = try writeNativeSidecar(["count": 1], name: "metrics.json", admission: owner(["sidecar_bytes": 2]), directory: sidecarFolder) }
        expect(try FileManager.default.contentsOfDirectory(atPath: sidecarFolder.path).isEmpty)
        // Existing actual sample is borrowed/charged before insertion then encoded
        // by the real bounded codec. This is preservation, not fresh AX collection.
        let sample = try JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: args[3])))
        let sampleBudget = NativeJSON(try limits())
        let admittedSample = try sampleBudget.borrowed(sample)
        expect(sampleBudget.slots == 27143 && sampleBudget.stringBytes == 257900)
        let sampleFrame = try NativeJSONFrame(capacity: 524288, deadline: deadline)
        try sampleFrame.encode(admittedSample)
        let encodedSample = sampleFrame.bytes { Data($0) }
        try encodedSample.write(to: URL(fileURLWithPath: args[4]), options: .withoutOverwriting)
        let reconstructed = try JSONSerialization.jsonObject(with: encodedSample) as! NSDictionary
        expect(reconstructed.isEqual(to: sample as! [AnyHashable: Any]))
        print("{\"passed\":\(passed),\"sdk_acquisition\":false,\"sample_nodes\":76,\"sample_slots\":\(sampleBudget.slots),\"sample_string_bytes\":\(sampleBudget.stringBytes)}")
    }
}
