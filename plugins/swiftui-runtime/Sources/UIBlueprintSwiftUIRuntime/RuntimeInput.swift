#if DEBUG
import Darwin
import Foundation

enum RuntimeInput {
    /// Remove the incoming name before inspecting it, including invalid files.
    /// Never follow a symlink or block opening a FIFO supplied as a request.
    static func claim(_ input: URL, in directory: URL) throws -> Data {
        let claimed = directory.appendingPathComponent(".claimed-" + UUID().uuidString)
        guard rename(input.path, claimed.path) == 0 else { throw RuntimeRefusal.invalidRequest }
        defer { unlink(claimed.path) }
        let fd = open(claimed.path, O_RDONLY | O_NOFOLLOW | O_NONBLOCK | O_CLOEXEC)
        guard fd >= 0 else { throw RuntimeRefusal.invalidRequest }
        defer { close(fd) }
        var info = stat()
        guard fstat(fd, &info) == 0, info.st_mode & S_IFMT == S_IFREG,
              info.st_uid == getuid(), info.st_size > 0, info.st_size <= 16_384 else {
            throw RuntimeRefusal.invalidRequest
        }
        let handle = FileHandle(fileDescriptor: fd, closeOnDealloc: false)
        guard let bytes = try handle.read(upToCount: 16_385), !bytes.isEmpty, bytes.count <= 16_384 else {
            throw RuntimeRefusal.invalidRequest
        }
        return bytes
    }
}
#endif
