import Foundation
import Darwin
import CoreGraphics
import ImageIO
import UniformTypeIdentifiers

// Only caller-owned artifacts, never an arbitrary destination from UI data.
// Callback state is protected even if an encoder invokes it on another thread.
final class NativeArtifactWriter: @unchecked Sendable {
    private let lock = NSLock()
    private let directory: Int32
    private var file: Int32
    private let temporary: String
    private let destination: String
    private let cap: Int
    private let deadline: Double
    private let output: @Sendable (Int32, UnsafeRawPointer, Int) -> Int
    private var written = 0
    private var failed = false
    private var promoted = false

    init(directory url: URL, destination: String, cap: Int, deadline: Double,
         output: @escaping @Sendable (Int32, UnsafeRawPointer, Int) -> Int = { Darwin.write($0, $1, $2) }) throws {
        guard cap > 0, deadline.isFinite, !destination.isEmpty,
              !destination.contains("/"), !destination.utf8.contains(0), destination != ".", destination != ".."
        else { throw NativeAcquisitionError.invalidValue }
        self.cap = cap; self.deadline = deadline; self.destination = destination; self.output = output
        directory = open(url.path, O_RDONLY | O_DIRECTORY | O_CLOEXEC | O_NOFOLLOW)
        guard directory >= 0 else { throw NativeAcquisitionError.io }
        var info = stat()
        guard fstat(directory, &info) == 0, info.st_uid == geteuid() else {
            close(directory); throw NativeAcquisitionError.io
        }
        var existing = stat()
        guard fstatat(directory, destination, &existing, AT_SYMLINK_NOFOLLOW) != 0, errno == ENOENT else {
            close(directory); throw NativeAcquisitionError.io
        }
        temporary = ".native-\(UUID().uuidString).partial"
        file = openat(directory, temporary, O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW | O_CLOEXEC, mode_t(0o600))
        guard file >= 0 else { close(directory); throw NativeAcquisitionError.io }
    }
    deinit {
        if file >= 0 { close(file) }
        if !promoted { unlinkat(directory, temporary, 0) }
        close(directory)
    }
    func put(_ bytes: UnsafeRawPointer, count: Int) -> Int {
        lock.lock(); defer { lock.unlock() }
        guard !failed, !promoted, count >= 0, count <= cap - written,
              ProcessInfo.processInfo.systemUptime < deadline else { failed = true; return 0 }
        // Admission is for the complete callback chunk, before any of its bytes.
        var offset = 0
        while offset < count {
            guard ProcessInfo.processInfo.systemUptime < deadline else { failed = true; return 0 }
            let n = output(file, bytes.advanced(by: offset), count - offset)
            if n < 0 && errno == EINTR { continue }
            guard n > 0 else { failed = true; return 0 }
            offset += n; written += n
        }
        return count
    }
    func consumer() throws -> CGDataConsumer {
        let info = Unmanaged.passRetained(self).toOpaque()
        var callbacks = CGDataConsumerCallbacks(putBytes: { info, buffer, count in
            guard let info else { return 0 }
            return Unmanaged<NativeArtifactWriter>.fromOpaque(info).takeUnretainedValue().put(buffer, count: count)
        }, releaseConsumer: { info in
            if let info { Unmanaged<NativeArtifactWriter>.fromOpaque(info).release() }
        })
        guard let consumer = CGDataConsumer(info: info, cbks: &callbacks) else {
            Unmanaged<NativeArtifactWriter>.fromOpaque(info).release()
            throw NativeAcquisitionError.io
        }
        return consumer
    }
    func finish(encoderSucceeded: Bool) throws -> Int {
        lock.lock(); defer { lock.unlock() }
        guard encoderSucceeded, !failed, !promoted, written > 0,
              ProcessInfo.processInfo.systemUptime < deadline else { throw NativeAcquisitionError.io }
        // linkat fails if destination exists. Unlike rename(), it cannot replace
        // a pre-existing user's file or symlink in a racing destination.
        guard linkat(directory, temporary, directory, destination, 0) == 0 else { throw NativeAcquisitionError.io }
        guard unlinkat(directory, temporary, 0) == 0 else {
            // Published link is ours; do not remove an unrelated destination.
            throw NativeAcquisitionError.io
        }
        promoted = true
        close(file); file = -1
        return written
    }
}

func writeNativePNG(_ image: CGImage, admission: NativeAcquisition, directory: URL, name: String) throws -> Int {
    try admission.imageFootprint(width: image.width, height: image.height, rowBytes: image.bytesPerRow)
    guard image.bitsPerComponent == 8, image.bitsPerPixel == 32,
          image.colorSpace?.model == .rgb else { throw NativeAcquisitionError.invalidValue }
    let writer = try NativeArtifactWriter(directory: directory, destination: name,
        cap: admission.limits.png_bytes, deadline: admission.deadline)
    let consumer = try writer.consumer()
    guard let destination = CGImageDestinationCreateWithDataConsumer(consumer, UTType.png.identifier as CFString, 1, nil)
    else { throw NativeAcquisitionError.io }
    CGImageDestinationAddImage(destination, image, nil)
    return try writer.finish(encoderSucceeded: CGImageDestinationFinalize(destination))
}

func writeNativeSidecar(_ value: [String: Any], name: String, admission: NativeAcquisition, directory: URL) throws -> Int {
    let frame = try NativeJSONFrame(capacity: admission.limits.sidecar_bytes, deadline: admission.deadline)
    try frame.encode(value)
    let writer = try NativeArtifactWriter(directory: directory, destination: name,
        cap: admission.limits.sidecar_bytes, deadline: admission.deadline)
    let size = frame.bytes { writer.put($0.baseAddress!, count: $0.count) }
    return try writer.finish(encoderSucceeded: size == frame.count)
}

// The trusted per-operation container may be shared by AX evidence and capture.
// Each channel creates a new private subdirectory; repeated operations/overwrites
// refuse. No path component is sourced from UI data.
func nativeChannelDirectory(_ root: URL, channel: String) throws -> URL {
    guard channel == "ax" || channel == "capture" else { throw NativeAcquisitionError.invalidValue }
    if mkdir(root.path, mode_t(0o700)) != 0 && errno != EEXIST { throw NativeAcquisitionError.io }
    let fd = open(root.path, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC)
    guard fd >= 0 else { throw NativeAcquisitionError.io }
    defer { close(fd) }
    var info = stat()
    guard fstat(fd, &info) == 0, info.st_uid == geteuid(), info.st_mode & 0o077 == 0,
          mkdirat(fd, channel, mode_t(0o700)) == 0 else { throw NativeAcquisitionError.io }
    return root.appendingPathComponent(channel, isDirectory: true)
}
