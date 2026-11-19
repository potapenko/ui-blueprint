import Foundation
import AppKit
import CoreGraphics
import CryptoKit
import Darwin

// Explicit one-shot external capture; no AX reads, application mutation or permissions UI.
enum NativeWindowCapture {
    struct Refusal: Error { let code: String; let exit: Int32 }
    struct Options {
        let pid: Int32
        let window: UInt32
        let bundle: String
        let output: URL
        let request: String
        let deadlineMS: Int
        let expectedLaunch: Double?
        static func parse(_ arguments: [String]) throws -> Options {
            guard arguments.count <= 14, arguments.count % 2 == 0,
                  arguments.reduce(0, { $0 + $1.utf8.count }) <= 8192 else { throw invalid() }
            var values: [String: String] = [:]
            let allowed = Set(["--pid", "--window-id", "--expected-bundle", "--out", "--request-id", "--deadline-ms", "--expected-launch-time"])
            for index in stride(from: 0, to: arguments.count, by: 2) {
                let flag = arguments[index], value = arguments[index + 1]
                guard allowed.contains(flag), values[flag] == nil, !value.isEmpty,
                      !value.utf8.contains(0) else { throw invalid() }
                values[flag] = value
            }
            guard let rawPID = values["--pid"], rawPID.allSatisfy({ $0.isASCII && $0.isNumber }),
                  let pid = Int32(rawPID), pid > 0,
                  let rawWindow = values["--window-id"], rawWindow.allSatisfy({ $0.isASCII && $0.isNumber }),
                  let window = UInt32(rawWindow), window > 0,
                  let bundle = values["--expected-bundle"], bundle.utf8.count <= 256,
                  bundle.utf8.allSatisfy({ (65...90).contains($0) || (97...122).contains($0) || (48...57).contains($0) || [45,46,95].contains($0) }),
                  let path = values["--out"], path.hasPrefix("/"), path.utf8.count <= 4096,
                  let request = values["--request-id"], request.utf8.count <= 64,
                  request.utf8.allSatisfy({ (65...90).contains($0) || (97...122).contains($0) || (48...57).contains($0) || [45,46,95].contains($0) }),
                  let deadline = Int(values["--deadline-ms"] ?? "5000"), (100...10000).contains(deadline)
            else { throw invalid() }
            var launch: Double?
            if let value = values["--expected-launch-time"] {
                guard let number = Double(value), number.isFinite, number > 0 else { throw invalid() }
                launch = number
            }
            let output = URL(fileURLWithPath: path).standardizedFileURL
            let parent = output.deletingLastPathComponent().resolvingSymlinksInPath()
            let temp = FileManager.default.temporaryDirectory.resolvingSymlinksInPath().path
            guard parent.path == temp || parent.path.hasPrefix(temp + "/"),
                  output.lastPathComponent != ".", output.lastPathComponent != ".." else {
                throw Refusal(code: "destination_must_be_system_temp", exit: 2)
            }
            return Options(pid: pid, window: window, bundle: bundle,
                output: parent.appendingPathComponent(output.lastPathComponent, isDirectory: true),
                request: request, deadlineMS: deadline, expectedLaunch: launch)
        }
    }
    static func discoveryOptions(_ arguments: [String]) throws -> Options {
        guard arguments.filter({ $0 == "--list-windows" }).count == 1 else { throw invalid() }
        let rest = arguments.filter { $0 != "--list-windows" }
        guard rest.count % 2 == 0, rest.count <= 6 else { throw invalid() }
        for index in stride(from: 0, to: rest.count, by: 2) {
            guard ["--pid", "--expected-bundle", "--expected-launch-time"].contains(rest[index]) else { throw invalid() }
        }
        return try Options.parse(rest + ["--window-id", "1", "--out",
            FileManager.default.temporaryDirectory.appendingPathComponent("unused-window-list").path,
            "--request-id", "window-list"])
    }
    @MainActor static func listWindows(_ options: Options) throws -> [String: Any] {
        let process = try ProcessIdentity.read(options)
        guard let rows = CGWindowListCopyWindowInfo([.optionAll, .excludeDesktopElements], kCGNullWindowID) as? [[String: Any]] else {
            throw OwnedCaptureError.targetUnresolved
        }
        var windows: [[String: Any]] = []
        // Window-server inventory is an opaque public API return. Only exact-PID
        // numeric metadata is copied/output; never titles or other-owner content.
        for row in rows where row[kCGWindowOwnerPID as String] as? Int32 == process.pid {
            guard windows.count < 32, let window = row[kCGWindowNumber as String] as? UInt32,
                  let values = row[kCGWindowBounds as String] as? [String: Any],
                  let frame = CGRect(dictionaryRepresentation: values as CFDictionary),
                  [frame.minX, frame.minY, frame.width, frame.height].allSatisfy({ $0.isFinite }),
                  frame.width > 0, frame.height > 0 else { throw NativeAcquisitionError.limit }
            windows.append(["window_id": window, "frame": rect(frame),
                "layer": row[kCGWindowLayer as String] as? Int ?? 0,
                "on_screen": row[kCGWindowIsOnscreen as String] as? Bool ?? false])
        }
        guard try ProcessIdentity.read(options) == process else { throw OwnedCaptureError.staleTarget }
        return ["format_version": "1.0.0", "kind": "native_window_candidates", "source": "CGWindowListCopyWindowInfo",
            "process": ["pid": process.pid, "bundle_id": process.bundle, "launch_time": process.launch],
            "coordinate_space": ["kind": "screen", "units": "pt", "origin": "top_left"],
            "windows": windows, "capture_performed": false, "selection": "caller_selects_exact_window_id"]
    }
    struct ProcessIdentity: Equatable {
        let pid: Int32
        let bundle: String
        let launch: Double
        @MainActor static func read(_ options: Options) throws -> ProcessIdentity {
            guard let app = NSRunningApplication(processIdentifier: options.pid), !app.isTerminated,
                  app.bundleIdentifier == options.bundle,
                  let launch = app.launchDate?.timeIntervalSince1970, launch.isFinite,
                  options.expectedLaunch == nil || options.expectedLaunch == launch else {
                throw Refusal(code: "target_unresolved_or_stale_process", exit: 4)
            }
            return ProcessIdentity(pid: options.pid, bundle: options.bundle, launch: launch)
        }
    }
    static let help = """
    capture-window --pid PID --window-id CG_WINDOW_ID --expected-bundle BUNDLE --out NEW_SYSTEM_TEMP_DIRECTORY --request-id ID [--deadline-ms 100..10000] [--expected-launch-time SECONDS]
    capture-window --list-windows --pid PID --expected-bundle BUNDLE [--expected-launch-time SECONDS]
    Exact already-running window; lossless native window.png + capture.json. No shadow/cursor/audio/child windows, no focus/resize/permission prompt/retry. Default deadline5000ms; maximum4096x4096 and8388608 pixels. All image files remain, including partials on failure.
    """
    static func invalid() -> Refusal { Refusal(code: "invalid_capture_arguments", exit: 2) }
    static func limits() -> NativeAcquisitionLimits {
        NativeAcquisitionLimits(ax_windows: 32, array_page: 32, child_entries: 1280,
            value_utf8_bytes: 4096, action_names: 32, action_name_utf8_bytes: 256,
            batch_values: 8, batch_utf8_bytes: 16384, copied_utf8_bytes: 262144,
            response_slots: 65536, response_string_utf8_bytes: 1048576,
            image_width: 4096, image_height: 4096, image_pixels: 8388608,
            image_bytes: 67108864, png_bytes: 67108864, sidecar_bytes: 16384)
    }
    static func newDirectory(_ url: URL) throws {
        let parent = open(url.deletingLastPathComponent().path, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC)
        guard parent >= 0 else { throw Refusal(code: "invalid_destination", exit: 2) }
        defer { close(parent) }
        var info = stat()
        guard fstat(parent, &info) == 0, info.st_uid == geteuid(),
              mkdirat(parent, url.lastPathComponent, mode_t(0o700)) == 0 else {
            throw Refusal(code: "destination_exists_or_unavailable", exit: 2)
        }
    }
    static func rect(_ value: CGRect) -> [String: Double] {
        ["x": value.minX, "y": value.minY, "width": value.width, "height": value.height]
    }
    static func imageHash(_ url: URL, maxBytes: Int) throws -> String {
        let fd = open(url.path, O_RDONLY | O_NOFOLLOW | O_CLOEXEC)
        guard fd >= 0 else { throw NativeAcquisitionError.io }
        let file = FileHandle(fileDescriptor: fd, closeOnDealloc: true)
        defer { try? file.close() }
        var hash = SHA256(), count = 0
        while let chunk = try file.read(upToCount: 65536), !chunk.isEmpty {
            count += chunk.count
            guard count <= maxBytes else { throw NativeAcquisitionError.limit }
            hash.update(data: chunk)
        }
        return hash.finalize().map { String(format: "%02x", $0) }.joined()
    }
    @MainActor static func execute(_ options: Options, deadline: Double) async throws -> (metadata: [String: Any], geometry: CaptureGeometry, process: ProcessIdentity) {
        let started = ProcessInfo.processInfo.systemUptime
        let startedUTC = Date().timeIntervalSince1970
        let process = try ProcessIdentity.read(options)
        let before = try CaptureGeometry.read(windowID: options.window, pid: options.pid)
        guard before.frame.width > 0, before.frame.height > 0 else { throw invalid() }
        guard CGPreflightScreenCaptureAccess() else { throw OwnedCaptureError.permissionRequired }
        try newDirectory(options.output)
        let acquisition = try NativeAcquisition(limits(), deadline: deadline)
        let acquiredStart = ProcessInfo.processInfo.systemUptime
        // This command owns its process; no H01 parent or reusable session is created.
        // Existing parentOwned path skips the fixture-only global file lock and faults.
        let capture = try await CaptureLifecycle.capture(windowID: options.window, pid: options.pid,
            budget: min(2, deadline - acquiredStart), admission: .parentOwned, acquisition: acquisition)
        let acquiredEnd = ProcessInfo.processInfo.systemUptime
        guard try ProcessIdentity.read(options) == process else { throw OwnedCaptureError.staleTarget }
        try before.requireUnchanged(CaptureGeometry.read(windowID: options.window, pid: options.pid))
        let bytes = try writeNativePNG(capture.image, admission: acquisition, directory: options.output, name: "window.png")
        let hash = try imageHash(options.output.appendingPathComponent("window.png"), maxBytes: limits().png_bytes)
        guard try ProcessIdentity.read(options) == process else { throw OwnedCaptureError.staleTarget }
        try before.requireUnchanged(CaptureGeometry.read(windowID: options.window, pid: options.pid))
        try acquisition.check()
        let screenSpace: [String: Any] = ["units": "pt", "origin": "top_left", "kind": "screen"]
        let imageSpace: [String: Any] = ["units": "px", "origin": "top_left", "kind": "image"]
        var mapping: [String: Any] = ["status": "unknown", "reason": "full_window_source_mapping_not_established"]
        if let known = capture.mapping {
            mapping = ["status": "known", "source": "SCContentFilter.contentRect_and_pointPixelScale_with_CG_geometry_revalidation",
                "from": screenSpace, "to": imageSpace, "screen_to_image_affine": known.affine,
                "window_local_to_image_affine": [Double(capture.scale), 0, 0, Double(capture.scale), 0, 0]]
        }
        let metadata: [String: Any] = ["format_version": "1.0.0", "kind": "native_window_capture", "status": "captured",
            "request_id": options.request, "capture_id": UUID().uuidString,
            "source": "UIBlueprint.native-host-helper.ScreenCaptureKit",
            "process": ["pid": process.pid, "bundle_id": process.bundle, "launch_time": process.launch],
            "window_id": options.window,
            "timing": ["clock": "system_uptime_seconds", "request_start": started,
                "request_start_unix_seconds": startedUTC, "acquisition_start": acquiredStart,
                "acquisition_end": acquiredEnd, "validated_end": ProcessInfo.processInfo.systemUptime],
            "window_server_frame": rect(before.frame), "sck_window_frame": rect(capture.windowFrame),
            "filter_content_rect": rect(capture.filterRect), "window_coordinate_space": screenSpace,
            "point_pixel_scale": capture.scale,
            "image": ["path": options.output.appendingPathComponent("window.png").path,
                "format": "png", "lossless": true, "width_px": capture.image.width, "height_px": capture.image.height,
                "bytes": bytes, "sha256": hash, "resampled": false, "bits_per_component": capture.image.bitsPerComponent],
            "metadata_path": options.output.appendingPathComponent("capture.json").path,
            "convention": ["capture_kind": "window_isolated", "region": "full_window", "shadow": false,
                "cursor": false, "audio": false, "child_windows": false, "dynamic_range": "SDR"],
            "mapping": mapping,
            "association": ["layout_snapshot_modified": false, "layout_root_to_window": "consumer_verification_required",
                "presented_frame_atomicity": "unknown", "occlusion": "not_established"]]
        return (metadata, before, process)
    }
    @MainActor static func run(_ arguments: [String]) async -> Int32 {
        if arguments == ["--help"] { FileHandle.standardOutput.write(Data((help + "\n").utf8)); return 0 }
        do {
            if arguments.contains("--list-windows") {
                let options = try discoveryOptions(arguments)
                let watchdog = DispatchWorkItem { @Sendable in FileHandle.standardError.write(Data("capture_deadline\n".utf8)); _exit(4) }
                DispatchQueue.global().asyncAfter(deadline: .now() + 5, execute: watchdog)
                defer { watchdog.cancel() }
                let result = try listWindows(options)
                let data = try JSONSerialization.data(withJSONObject: result, options: [.sortedKeys])
                guard data.count + 1 <= 16384 else { throw NativeAcquisitionError.limit }
                FileHandle.standardOutput.write(data); FileHandle.standardOutput.write(Data([10]))
                return 0
            }
            let options = try Options.parse(arguments)
            let deadline = ProcessInfo.processInfo.systemUptime + Double(options.deadlineMS) / 1000
            let watchdog = DispatchWorkItem { @Sendable in
                FileHandle.standardError.write(Data("capture_deadline\n".utf8)); _exit(4)
            }
            DispatchQueue.global().asyncAfter(deadline: .now() + Double(options.deadlineMS) / 1000, execute: watchdog)
            defer { watchdog.cancel() }
            let captured = try await execute(options, deadline: deadline)
            let result = captured.metadata
            let data = try JSONSerialization.data(withJSONObject: result, options: [.sortedKeys])
            guard data.count + 1 <= 16384 else { throw NativeAcquisitionError.limit }
            // Publication is one complete response. Metadata accompanies the retained PNG.
            let admission = try NativeAcquisition(limits(), deadline: deadline)
            _ = try writeNativeSidecar(result, name: "capture.json", admission: admission, directory: options.output)
            do {
                guard try ProcessIdentity.read(options) == captured.process else { throw OwnedCaptureError.staleTarget }
                try captured.geometry.requireUnchanged(CaptureGeometry.read(windowID: options.window, pid: options.pid))
                try admission.check()
            } catch {
                // Only this request's non-image sidecar is retired on failed final validation.
                // PNG/staging image paths are always retained and never advertised as success.
                try? FileManager.default.removeItem(at: options.output.appendingPathComponent("capture.json"))
                throw error
            }
            FileHandle.standardOutput.write(data); FileHandle.standardOutput.write(Data([10]))
            return 0
        } catch {
            let failure: Refusal
            if let refusal = error as? Refusal { failure = refusal }
            else if let native = error as? NativeAcquisitionError {
                failure = Refusal(code: native == .limit ? "capture_limit" : native == .expired ? "capture_deadline" : "capture_io", exit: native == .limit ? 2 : native == .expired ? 4 : 1)
            } else { failure = Refusal(code: CaptureLifecycle.issue(error).code, exit: 4) }
            FileHandle.standardError.write(Data((failure.code + "\n").utf8)); return failure.exit
        }
    }
}
