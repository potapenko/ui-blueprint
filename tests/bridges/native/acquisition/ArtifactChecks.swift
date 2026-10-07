import Foundation
import CoreGraphics
import Darwin

// Focused real writer/ImageIO proof, without AX, ScreenCaptureKit or numeric suites.
@main struct ArtifactChecks {
    static func main() throws {
        guard CommandLine.arguments.count == 3 else { exit(2) }
        let profile = try JSONDecoder().decode(NativeAcquisitionLimits.self,
            from: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1])))
        let root = URL(fileURLWithPath: CommandLine.arguments[2], isDirectory: true)
        let deadline = ProcessInfo.processInfo.systemUptime + 20
        var checks = 0
        func expect(_ value: Bool) { precondition(value); checks += 1 }
        func refuses(_ body: () throws -> Void) {
            do { try body(); preconditionFailure("expected refusal") } catch { checks += 1 }
        }
        func folder(_ name: String) throws -> URL {
            let url = root.appendingPathComponent(name, isDirectory: true)
            try FileManager.default.createDirectory(at: url, withIntermediateDirectories: false,
                attributes: [.posixPermissions: 0o700])
            return url
        }
        func entries(_ directory: URL) throws -> [URL] {
            try FileManager.default.contentsOfDirectory(at: directory, includingPropertiesForKeys: nil)
        }
        let raw = Data(repeating: 128, count: 24)
        let image = CGImage(width: 3, height: 2, bitsPerComponent: 8, bitsPerPixel: 32, bytesPerRow: 12,
            space: CGColorSpaceCreateDeviceRGB(), bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedFirst.rawValue).union(.byteOrder32Little),
            provider: CGDataProvider(data: raw as CFData)!, decode: nil, shouldInterpolate: false, intent: .defaultIntent)!
        let success = try folder("png-success")
        let admission = try NativeAcquisition(profile, deadline: deadline)
        let size = try autoreleasepool { try writeNativePNG(image, admission: admission, directory: success, name: "capture.png") }
        let files = try entries(success)
        expect(files.count == 2 && size > 0)
        let final = success.appendingPathComponent("capture.png")
        let staging = files.first { $0.lastPathComponent != "capture.png" }!
        expect(try Data(contentsOf: staging) == Data(contentsOf: final))
        var a = stat(), b = stat()
        expect(stat(staging.path, &a) == 0 && stat(final.path, &b) == 0 && a.st_ino == b.st_ino && a.st_dev == b.st_dev)
        let saved = try Data(contentsOf: final)
        refuses { _ = try writeNativePNG(image, admission: admission, directory: success, name: "capture.png") }
        expect(try Data(contentsOf: final) == saved && entries(success).count == 2)
        refuses { _ = try writeNativePNG(image, admission: admission, directory: success, name: "misnamed") }
        // Refuse a persistent destination before opening it or creating an image.
        refuses { _ = try writeNativePNG(image, admission: admission, directory: URL(fileURLWithPath: "/Users"), name: "capture.png") }
        let overflow = try folder("png-overflow")
        var smallJSON = try JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1]))) as! [String: Int]
        smallJSON["png_bytes"] = size - 1
        let small = try JSONDecoder().decode(NativeAcquisitionLimits.self, from: JSONSerialization.data(withJSONObject: smallJSON))
        refuses { _ = try autoreleasepool { try writeNativePNG(image, admission: NativeAcquisition(small, deadline: deadline), directory: overflow, name: "capture.png") } }
        expect(try entries(overflow).count == 1 && !FileManager.default.fileExists(atPath: overflow.appendingPathComponent("capture.png").path))
        let failed = try folder("png-failed-finalizer")
        var imageFD: Int32 = -1
        let beforeFailure = (0..<256).filter { fcntl(Int32($0), F_GETFD) >= 0 }.count
        do {
            let writer = try NativeArtifactWriter(directory: failed, destination: "capture.png", cap: 24, deadline: deadline,
                output: { fd, bytes, count in Darwin.write(fd, bytes, count) })
            // Failed Finalize leaves only its unadvertised staging bytes.
            expect(raw.withUnsafeBytes { writer.put($0.baseAddress!, count: 24) } == 24)
            refuses { _ = try writer.finish(encoderSucceeded: false) }
        }
        expect(try entries(failed).count == 1 && !FileManager.default.fileExists(atPath: failed.appendingPathComponent("capture.png").path))
        expect((0..<256).filter { fcntl(Int32($0), F_GETFD) >= 0 }.count == beforeFailure)
        let expired = try folder("png-expired")
        do {
            let writer = try NativeArtifactWriter(directory: expired, destination: "capture.png", cap: 24,
                deadline: ProcessInfo.processInfo.systemUptime - 1)
            expect(raw.withUnsafeBytes { writer.put($0.baseAddress!, count: 24) } == 0)
            refuses { _ = try writer.finish(encoderSucceeded: true) }
        }
        expect(try entries(expired).count == 1 && !FileManager.default.fileExists(atPath: expired.appendingPathComponent("capture.png").path))
        let descriptor = try folder("png-descriptor")
        let openBefore = (0..<256).filter { fcntl(Int32($0), F_GETFD) >= 0 }.count
        do {
            let writer = try NativeArtifactWriter(directory: descriptor, destination: "capture.png", cap: 24, deadline: deadline)
            // Existing writer has no descriptor introspection API. Enumerate actual
            // newly opened descriptors in this bounded standalone test process.
            let opened = (0..<256).filter { fcntl(Int32($0), F_GETFD) >= 0 }
            imageFD = Int32(opened.max()!)
            expect(raw.withUnsafeBytes { writer.put($0.baseAddress!, count: 24) } == 24)
            expect(try writer.finish(encoderSucceeded: true) == 24)
        }
        expect(fcntl(imageFD, F_GETFD) == -1 && errno == EBADF)
        expect((0..<256).filter { fcntl(Int32($0), F_GETFD) >= 0 }.count == openBefore)
        let nonImage = try folder("non-image")
        do {
            let writer = try NativeArtifactWriter(directory: nonImage, destination: "metrics.json", cap: 3, deadline: deadline)
            expect(raw.withUnsafeBytes { writer.put($0.baseAddress!, count: 4) } == 0)
            refuses { _ = try writer.finish(encoderSucceeded: true) }
        }
        expect(try entries(nonImage).isEmpty)
        do {
            let writer = try NativeArtifactWriter(directory: nonImage, destination: "metrics.json", cap: 24, deadline: deadline)
            expect(raw.withUnsafeBytes { writer.put($0.baseAddress!, count: 24) } == 24)
            expect(try writer.finish(encoderSucceeded: true) == 24)
        }
        expect(try entries(nonImage).map(\.lastPathComponent) == ["metrics.json"])
        print("{\"checks\":\(checks),\"real_writer_imageio\":true,\"live_sdk\":false,\"images_retained\":true}")
    }
}
